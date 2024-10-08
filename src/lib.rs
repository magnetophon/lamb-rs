use faust_types::FaustDsp;
use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;
use std::sync::{Arc, Mutex};
mod buffer;
mod dsp_192k;
mod dsp_48k;
mod dsp_96k;
use buffer::*;
use cyma::utils::{HistogramBuffer, MinimaBuffer, PeakBuffer, VisualizerBuffer};

use default_boxed::DefaultBoxed;

// this seems to be the number JUCE is using
// TODO: does this need to be set at runtime?
const MAX_SOUNDCARD_BUFFER_SIZE: usize = 32768;

mod editor;

// Define an enum to hold the DSP's for different sample rates
enum DspVariant {
    Dsp48k(Box<dsp_48k::LambRs48k>),
    Dsp96k(Box<dsp_96k::LambRs96k>),
    Dsp192k(Box<dsp_192k::LambRs192k>),
}
impl Default for DspVariant {
    fn default() -> Self {
        DspVariant::Dsp48k(dsp_48k::LambRs48k::default_boxed())
    }
}

impl DspVariant {
    fn init(&mut self, sample_rate: i32) {
        match sample_rate {
            0..=48000 => *self = DspVariant::Dsp48k(dsp_48k::LambRs48k::default_boxed()),
            48001..=96000 => *self = DspVariant::Dsp96k(dsp_96k::LambRs96k::default_boxed()),
            _ => *self = DspVariant::Dsp192k(dsp_192k::LambRs192k::default_boxed()),
        }
        match self {
            DspVariant::Dsp48k(ref mut dsp) => dsp.init(sample_rate),
            DspVariant::Dsp96k(ref mut dsp) => dsp.init(sample_rate),
            DspVariant::Dsp192k(ref mut dsp) => dsp.init(sample_rate),
        }
    }
    fn get_param(&mut self, param_id: ParamIndex) -> Option<f64> {
        match self {
            DspVariant::Dsp48k(ref dsp) => dsp.get_param(param_id),
            DspVariant::Dsp96k(ref dsp) => dsp.get_param(param_id),
            DspVariant::Dsp192k(ref dsp) => dsp.get_param(param_id),
        }
    }
    fn set_param(&mut self, param_id: ParamIndex, val: f64) {
        match self {
            DspVariant::Dsp48k(ref mut dsp) => dsp.set_param(param_id, val),
            DspVariant::Dsp96k(ref mut dsp) => dsp.set_param(param_id, val),
            DspVariant::Dsp192k(ref mut dsp) => dsp.set_param(param_id, val),
        }
    }
    fn compute(&mut self, count: i32, inputs: &[&[f64]], outputs: &mut [&mut [f64]]) {
        match self {
            DspVariant::Dsp48k(ref mut dsp) => dsp.compute(count, inputs, outputs),
            DspVariant::Dsp96k(ref mut dsp) => dsp.compute(count, inputs, outputs),
            DspVariant::Dsp192k(ref mut dsp) => dsp.compute(count, inputs, outputs),
        }
    }
}

pub struct Lamb {
    params: Arc<LambParams>,
    // dsp_holder: DspHolder,
    dsp_variant: DspVariant,

    accum_buffer: TempBuffer,
    temp_output_buffer_l: Box<[f64]>,
    temp_output_buffer_r: Box<[f64]>,
    temp_output_buffer_gr_l: Box<[f64]>,
    temp_output_buffer_gr_r: Box<[f64]>,

    /// sample rate
    sample_rate: f32,

    // These buffers will hold the sample data for the visualizers.
    level_buffer_l: Arc<Mutex<PeakBuffer>>,
    level_buffer_r: Arc<Mutex<PeakBuffer>>,
    gr_buffer_l: Arc<Mutex<MinimaBuffer>>,
    gr_buffer_r: Arc<Mutex<MinimaBuffer>>,
    histogram_buffer: Arc<Mutex<HistogramBuffer>>,

    /// If this is set at the start of the processing cycle, then the graph duration should be updated.
    should_update_time_scale: Arc<AtomicBool>,
}
impl Default for Lamb {
    fn default() -> Self {
        let should_update_time_scale = Arc::new(AtomicBool::new(false));
        Self {
            // params: Arc::new(LambParams::default(should_update_time_scale.clone())),
            params: Arc::new(LambParams::new(should_update_time_scale.clone())),

            dsp_variant: DspVariant::default(),
            accum_buffer: TempBuffer::default(),

            temp_output_buffer_l: f64::default_boxed_array::<MAX_SOUNDCARD_BUFFER_SIZE>(),
            temp_output_buffer_r: f64::default_boxed_array::<MAX_SOUNDCARD_BUFFER_SIZE>(),
            temp_output_buffer_gr_l: f64::default_boxed_array::<MAX_SOUNDCARD_BUFFER_SIZE>(),
            temp_output_buffer_gr_r: f64::default_boxed_array::<MAX_SOUNDCARD_BUFFER_SIZE>(),
            sample_rate: 48000.0,
            level_buffer_l: Arc::new(Mutex::new(PeakBuffer::new(1171, 7.0, 0.0))),
            level_buffer_r: Arc::new(Mutex::new(PeakBuffer::new(1171, 7.0, 0.0))),
            gr_buffer_l: Arc::new(Mutex::new(MinimaBuffer::new(1171, 7.0, 0.0))),
            gr_buffer_r: Arc::new(Mutex::new(MinimaBuffer::new(1171, 7.0, 0.0))),
            histogram_buffer: Arc::new(Mutex::new(HistogramBuffer::new(256, 1.0))),
            should_update_time_scale,
        }
    }
}

include!("params.rs");

impl Plugin for Lamb {
    const NAME: &'static str = "lamb";
    const VENDOR: &'static str = "magnetophon";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "bart@magnetophon.nl";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    // The first audio IO layout is used as the default. The other layouts may be selected either
    // explicitly or automatically by the host or the user depending on the plugin API/backend.
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),

        aux_input_ports: &[],
        aux_output_ports: &[],

        // Individual ports and the layout as a whole can be named here. By default these names
        // are generated as needed. This layout will be called 'Stereo', while a layout with
        // only one input and output channel would be called 'Mono'.
        names: PortNames::const_default(),
    }];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    // If the plugin can send or receive SysEx messages, it can define a type to wrap around those
    // messages here. The type implements the `SysExMessage` trait, which allows conversion to and
    // from plain byte buffers.
    type SysExMessage = ();
    // More advanced plugins can use this to run expensive background tasks. See the field's
    // documentation for more information. `()` means that the plugin does not have any background
    // tasks.
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        self.accum_buffer.resize(2, MAX_SOUNDCARD_BUFFER_SIZE);
        self.sample_rate = buffer_config.sample_rate;

        match self.level_buffer_l.lock() {
            Ok(mut buffer) => {
                buffer.set_sample_rate(buffer_config.sample_rate);
            }
            Err(_) => return false,
        }

        match self.level_buffer_r.lock() {
            Ok(mut buffer) => {
                buffer.set_sample_rate(buffer_config.sample_rate);
            }
            Err(_) => return false,
        }

        match self.gr_buffer_l.lock() {
            Ok(mut buffer) => {
                buffer.set_sample_rate(buffer_config.sample_rate);
            }
            Err(_) => return false,
        }

        match self.gr_buffer_r.lock() {
            Ok(mut buffer) => {
                buffer.set_sample_rate(buffer_config.sample_rate);
            }
            Err(_) => return false,
        }

        match self.histogram_buffer.lock() {
            Ok(mut buffer) => {
                buffer.set_sample_rate(buffer_config.sample_rate);
            }
            Err(_) => return false,
        }

        // Resize buffers and perform other potentially expensive initialization operations here.
        // The `reset()` function is always called right after this function. You can remove this
        // function if you do not need it.
        self.dsp_variant.init(buffer_config.sample_rate as i32);

        true
    }

    fn reset(&mut self) {
        // Reset buffers and envelopes here. This can be called from the audio thread and may not
        // allocate. You can remove this function if you do not need it.
        self.should_update_time_scale.store(true, Ordering::Release);
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(
            self.params.clone(),
            self.level_buffer_l.clone(),
            self.level_buffer_r.clone(),
            self.gr_buffer_l.clone(),
            self.gr_buffer_r.clone(),
            self.histogram_buffer.clone(),
            self.params.editor_state.clone(),
        )
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let count = buffer.samples() as i32;
        self.accum_buffer.read_from_buffer(buffer);

        let bypass: f64 = match self.params.bypass.value() {
            true => 1.0,
            false => 0.0,
        };
        self.dsp_variant.set_param(BYPASS_PI, bypass);

        let latency_mode: f64 = match self.params.latency_mode.value() {
            LatencyMode::Minimal => 0.0,
            LatencyMode::Fixed => 1.0,
        };
        self.dsp_variant.set_param(LATENCY_MODE_PI, latency_mode);
        self.dsp_variant
            .set_param(INPUT_GAIN_PI, self.params.input_gain.value() as f64);
        self.dsp_variant
            .set_param(STRENGTH_PI, self.params.strength.value() as f64);
        self.dsp_variant
            .set_param(THRESH_PI, self.params.thresh.value() as f64);
        self.dsp_variant
            .set_param(ATTACK_PI, self.params.attack.value() as f64);
        self.dsp_variant
            .set_param(ATTACK_SHAPE_PI, self.params.attack_shape.value() as f64);
        self.dsp_variant
            .set_param(RELEASE_PI, self.params.release.value() as f64);
        self.dsp_variant
            .set_param(RELEASE_SHAPE_PI, self.params.release_shape.value() as f64);
        self.dsp_variant
            .set_param(RELEASE_HOLD_PI, self.params.release_hold.value() as f64);
        self.dsp_variant
            .set_param(KNEE_PI, self.params.knee.value() as f64);
        self.dsp_variant
            .set_param(LINK_PI, self.params.link.value() as f64);
        self.dsp_variant.set_param(
            ADAPTIVE_RELEASE_PI,
            self.params.adaptive_release.value() as f64,
        );
        self.dsp_variant
            .set_param(LOOKAHEAD_PI, self.params.lookahead.value() as f64);
        self.dsp_variant
            .set_param(OUTPUT_GAIN_PI, self.params.output_gain.value() as f64);

        self.dsp_variant.compute(
            count,
            &self.accum_buffer.slice2d(),
            &mut [
                &mut self.temp_output_buffer_l,
                &mut self.temp_output_buffer_r,
                &mut self.temp_output_buffer_gr_l,
                &mut self.temp_output_buffer_gr_r,
            ],
        );
        let latency_samples = self
            .dsp_variant
            .get_param(LATENCY_PI)
            .expect("no latency read") as u32;
        context.set_latency_samples(latency_samples);

        let output = buffer.as_slice();
        for i in 0..count as usize {
            output[0][i] = self.temp_output_buffer_l[i] as f32;
            output[1][i] = self.temp_output_buffer_r[i] as f32;
        }

        if self.params.editor_state.is_open() {
            if self.should_update_time_scale.load(Ordering::Relaxed) {
                let time_scale = match self.params.time_scale.value() {
                    TimeScale::OneSec => 1.0,
                    TimeScale::TwoSec => 2.0,
                    TimeScale::FourSec => 4.0,
                    TimeScale::EightSec => 8.0,
                    TimeScale::SixteenSec => 16.0,
                    TimeScale::ThirtytwoSec => 32.0,
                };
                self.level_buffer_l.lock().unwrap().set_duration(time_scale);
                self.level_buffer_r.lock().unwrap().set_duration(time_scale);
                self.gr_buffer_l.lock().unwrap().set_duration(time_scale);
                self.gr_buffer_r.lock().unwrap().set_duration(time_scale);
                self.should_update_time_scale
                    .store(false, Ordering::Release);
            };

            if self.params.in_out.value() {
                for i in 0..count as usize {
                    self.level_buffer_l
                        .lock()
                        .unwrap()
                        .enqueue(self.temp_output_buffer_l[i] as f32);
                    self.level_buffer_r
                        .lock()
                        .unwrap()
                        .enqueue(self.temp_output_buffer_r[i] as f32);
                }
            } else {
                let gain_db =
                    0.0 - (self.params.input_gain.value() + self.params.output_gain.value());
                let gain = if self.params.bypass.value() {
                    1.0
                } else {
                    10f32.powf(gain_db / 20.0)
                };
                for i in 0..count as usize {
                    self.level_buffer_l.lock().unwrap().enqueue(
                        (self.temp_output_buffer_l[i] / self.temp_output_buffer_gr_l[i]) as f32
                            * gain,
                    );
                    self.level_buffer_r.lock().unwrap().enqueue(
                        (self.temp_output_buffer_r[i] / self.temp_output_buffer_gr_r[i]) as f32
                            * gain,
                    );
                }
            };
            for i in 0..count as usize {
                self.gr_buffer_l
                    .lock()
                    .unwrap()
                    .enqueue(self.temp_output_buffer_gr_l[i] as f32);
                self.gr_buffer_r
                    .lock()
                    .unwrap()
                    .enqueue(self.temp_output_buffer_gr_r[i] as f32);
            }
            self.histogram_buffer
                .lock()
                .unwrap()
                .enqueue_buffer(buffer, None);
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for Lamb {
    const CLAP_ID: &'static str = "magnetophon.nl lamb";
    const CLAP_DESCRIPTION: Option<&'static str> =
        Some("A lookahead compressor/limiter that's soft as a lamb");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;

    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Stereo,
        ClapFeature::Compressor,
        ClapFeature::Limiter,
        ClapFeature::Mastering,
    ];
}

impl Vst3Plugin for Lamb {
    const VST3_CLASS_ID: [u8; 16] = *b"magnetophon lamb";

    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Fx,
        Vst3SubCategory::Dynamics,
        Vst3SubCategory::Mastering,
        Vst3SubCategory::Stereo,
    ];
}

nih_export_clap!(Lamb);
nih_export_vst3!(Lamb);
