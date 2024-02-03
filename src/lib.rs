use faust_types::FaustDsp;
use nih_plug::prelude::*;
use std::sync::Arc;
mod buffer;
mod dsp;
use buffer::*;

const MAX_BLOCK_SIZE: usize = 1024;
// This is a shortened version of the gain example with most comments removed, check out
// https://github.com/robbert-vdh/nih-plug/blob/master/plugins/examples/gain/src/lib.rs to get
// started

pub struct GainFaustNihPlug {
    params: Arc<GainFaustNihPlugParams>,
    dsp: dsp::Gain,
    accum_buffer: TempBuffer,
}
impl Default for GainFaustNihPlug {
    fn default() -> Self {
        Self {
            params: Arc::new(GainFaustNihPlugParams::default()),
            dsp: dsp::Gain::new(),

            accum_buffer: TempBuffer::default(),
        }
    }
}

include!("params.rs");

impl Plugin for GainFaustNihPlug {
    const NAME: &'static str = "lowpass-lr4-faust-nih-plug";
    const VENDOR: &'static str = "obsoleszenz";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "obsoleszenz@riseup.net";

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
        // Resize buffers and perform other potentially expensive initialization operations here.
        // The `reset()` function is always called right after this function. You can remove this
        // function if you do not need it.
        self.dsp.init(buffer_config.sample_rate as i32);
        self.accum_buffer.resize(2, MAX_BLOCK_SIZE);
        true
    }

    fn reset(&mut self) {
        // Reset buffers and envelopes here. This can be called from the audio thread and may not
        // allocate. You can remove this function if you do not need it.
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let count = buffer.samples() as i32;
        self.accum_buffer.read_from_buffer(buffer);
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

        self.dsp
            .compute(count, &self.accum_buffer.slice2d(), output);
        ProcessStatus::Normal
                                                                 }
                                              }

impl ClapPlugin for GainFaustNihPlug {
    const CLAP_ID: &'static str = "com.obsoleszenz.faust-nih-plug";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("A short description of your plugin");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;

    // Don't forget to change these features
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ClapFeature::Stereo];
}

impl Vst3Plugin for GainFaustNihPlug {
    const VST3_CLASS_ID: [u8; 16] = *b"obsoleszenz-gain";

    // And also don't forget to change these categories
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Dynamics];
}

// nih_export_clap!(GainFaustNihPlug);
nih_export_vst3!(GainFaustNihPlug);
