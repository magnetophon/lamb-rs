use faust_types::FaustDsp;
use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;
use atomic_float::AtomicF32;
use std::sync::Arc;
mod buffer;
mod dsp;
use buffer::*;
use rubato::{FftFixedOut, Resampler};

const MAX_BLOCK_SIZE: usize = 1024;
// This is a shortened version of the gain example with most comments removed, check out
// https://github.com/robbert-vdh/nih-plug/blob/master/plugins/examples/gain/src/lib.rs to get
// started

mod editor;

/// The time it takes for the peak meter to decay by 12 dB after switching to complete silence.
const PEAK_METER_DECAY_MS: f64 = 150.0;

pub struct Lamb {
    params: Arc<LambParams>,
    dsp: dsp::Lamb,
    accum_buffer: TempBuffer,

    /// sample rate
    sample_rate: f32,
    /// Needed to normalize the peak meter's response based on the sample rate.
    peak_meter_decay_weight: f32,
    /// The current data for the peak meter. This is stored as an [`Arc`] so we can share it between
    /// the GUI and the audio processing parts. If you have more state to share, then it's a good
    /// idea to put all of that in a struct behind a single `Arc`.
    ///
    /// This is stored as voltage gain.
    peak_meter: Arc<AtomicF32>,
    gain_reduction_left: Arc<AtomicF32>,
    gain_reduction_right: Arc<AtomicF32>,
    resampler: Option<FftFixedOut<f32>>,
    resampler_buffer: Option<Vec<Vec<f32>>>,
}
impl Default for Lamb {
    fn default() -> Self {
        Self {
            params: Arc::new(LambParams::default()),

            peak_meter_decay_weight: 1.0,
            peak_meter: Arc::new(AtomicF32::new(util::MINUS_INFINITY_DB)),
            gain_reduction_left: Arc::new(AtomicF32::new(0.0)),
            gain_reduction_right: Arc::new(AtomicF32::new(0.0)),

            dsp: dsp::Lamb::new(),

            accum_buffer: TempBuffer::default(),
            sample_rate: 48000.0,
            resampler: None,
            resampler_buffer: None,
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
        context: &mut impl InitContext<Self>,
    ) -> bool {
        let target_rate = 192000;
        // Resize buffers and perform other potentially expensive initialization operations here.
        // The `reset()` function is always called right after this function. You can remove this
        // function if you do not need it.
        self.dsp.init(target_rate as i32);
        // self.dsp.init(buffer_config.sample_rate as i32);
        self.accum_buffer.resize(2, MAX_BLOCK_SIZE);

        // After `PEAK_METER_DECAY_MS` milliseconds of pure silence, the peak meter's value should
        // have dropped by 12 dB
        self.peak_meter_decay_weight = 0.25f64
                                        .powf((buffer_config.sample_rate as f64 * PEAK_METER_DECAY_MS / 1000.0).recip())
                                        as f32;

        self.sample_rate = buffer_config.sample_rate;
        //Setup resampler
        // Parameters are:
        // - `sample_rate_input`: Input sample rate, must be > 0.
        // - `sample_rate_output`: Output sample rate, must be > 0.
        // - `chunk_size_out`: length of output data in frames.
        // - `sub_chunks`: desired number of subchunks for processing, actual number may be different.
        // - `nbr_channels`: number of channels in input/output.

        self.resampler = match FftFixedOut::<f32>::new(
            self.sample_rate as usize,
            target_rate,
            MAX_BLOCK_SIZE,
            2, // TODO: figure out what this means
            2,
        ) {
            Ok(sampler) => Some(sampler),
            Err(e) => {
                nih_error!(
                    "Failed to create resampler, audio processing will be disabled {:?}",
                    e
                );
                None
            }
        };

        if let Some(resampler) = &self.resampler {
            self.resampler_buffer = Some(resampler.output_buffer_allocate(true));
        }

        true
    }

    fn reset(&mut self) {
        // Reset buffers and envelopes here. This can be called from the audio thread and may not
        // allocate. You can remove this function if you do not need it.
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(
            self.params.clone(),
            self.peak_meter.clone(),
            self.gain_reduction_left.clone(),
            self.gain_reduction_right.clone(),
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

        // self.accum_buffer.read_from_buffer(buffer);

        for channel_samples in buffer.iter_samples() {
            let mut amplitude = 0.0;
            let num_samples = channel_samples.len();


            for sample in channel_samples {
                amplitude += *sample;
            }
            // To save resources, a plugin can (and probably should!) only perform expensive
            // calculations that are only displayed on the GUI while the GUI is open
            if self.params.editor_state.is_open() {
                amplitude = (amplitude / num_samples as f32).abs();
                let current_peak_meter = self.peak_meter.load(std::sync::atomic::Ordering::Relaxed);
                let new_peak_meter = if amplitude > current_peak_meter {
                    amplitude
                } else {
                    current_peak_meter * self.peak_meter_decay_weight
                        + amplitude * (1.0 - self.peak_meter_decay_weight)
                };

                self.peak_meter
                    .store(new_peak_meter, std::sync::atomic::Ordering::Relaxed);
                self.gain_reduction_left
                    .store(self.dsp.get_param(GAIN_REDUCTION_LEFT_PI).expect("no GR read"), std::sync::atomic::Ordering::Relaxed);
                self.gain_reduction_right
                    .store(self.dsp.get_param(GAIN_REDUCTION_RIGHT_PI).expect("no GR read"), std::sync::atomic::Ordering::Relaxed);
            }
        }
        if let Some(resampler) = &mut self.resampler {
            if let Some(resampler_buffer) = &mut self.resampler_buffer {
                resampler.process_into_buffer(&buffer.as_slice(), resampler_buffer, None);
            }
        }
        let output = buffer.as_slice();

        self.dsp.set_param(INPUT_GAIN_PI, self.params.input_gain.value());
        self.dsp.set_param(STRENGTH_PI, self.params.strength.value());
        self.dsp.set_param(THRESH_PI, self.params.thresh.value());
        self.dsp.set_param(ATTACK_PI, self.params.attack.value());
        self.dsp.set_param(ATTACK_SHAPE_PI, self.params.attack_shape.value());
        self.dsp.set_param(RELEASE_PI, self.params.release.value());
        self.dsp.set_param(RELEASE_SHAPE_PI, self.params.release_shape.value());
        self.dsp.set_param(KNEE_PI, self.params.knee.value());
        self.dsp.set_param(LINK_PI, self.params.link.value());

        // self.accum_buffer.read_from_buffer(self.resampler_buffer);
        // let data: &[&[f32]] = &self.resampler_buffer.unwrap().iter().map(|inner| inner.as_slice()).collect::<Vec<_>>();
        // let data: &[&[f32]] = &<Option<Vec<Vec<f32>>> as Clone>::clone(&self.resampler_buffer).unwrap().iter().map(|inner| inner.as_slice()).collect::<Vec<_>>();
        // let data: &[&[f32]] = &<Option<Vec<Vec<f32>>> as Clone>::clone(&self.resampler_buffer).unwrap().into_iter().map(|inner| inner.as_slice()).collect::<Vec<_>>();
        let binding = <Option<Vec<Vec<f32>>> as Clone>::clone(&self.resampler_buffer).unwrap();
        let data: &[&[f32]] = &binding.iter().map(|inner| inner.as_slice()).collect::<Vec<_>>();

        self.dsp
        // .compute(count, &self.accum_buffer.slice2d(), output);
            .compute(count, &data, output);

        // TODO: get the actual value from the dsp, use a hbargraph?
        let mut latency_samples = self.params.attack.value()*0.001*self.sample_rate;
        context.set_latency_samples(latency_samples as u32);

        ProcessStatus::Normal

    }
}

impl ClapPlugin for Lamb {
    const CLAP_ID: &'static str = "magnetophon.nl lamb";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("A lookahead compressor/limiter that's soft as a lamb");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;

    // Don't forget to change these features
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ClapFeature::Stereo];
}

impl Vst3Plugin for Lamb {
    const VST3_CLASS_ID: [u8; 16] = *b"magnetophon lamb";

    // And also don't forget to change these categories
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Dynamics];
}

// nih_export_clap!(Lamb);
nih_export_vst3!(Lamb);
