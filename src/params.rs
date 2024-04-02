use faust_types::ParamIndex;
#[derive(Params)]
struct LambParams {
    // nr of params: 12
    #[id = "bypass"]
    bypass: BoolParam,
    #[id = "latency_mode"]
    latency_mode: EnumParam<LatencyMode>,
    #[id = "input_gain"]
    input_gain: FloatParam,
    #[id = "strength"]
    strength: FloatParam,
    #[id = "thresh"]
    thresh: FloatParam,
    #[id = "attack"]
    attack: FloatParam,
    #[id = "attack_shape"]
    attack_shape: FloatParam,
    #[id = "release"]
    release: FloatParam,
    #[id = "release_shape"]
    release_shape: FloatParam,
    #[id = "release_hold"]
    release_hold: FloatParam,
    #[id = "knee"]
    knee: FloatParam,
    #[id = "link"]
    link: FloatParam,
    #[id = "output_gain"]
    output_gain: FloatParam,
    #[id = "zoom_mode"]
    zoom_mode: EnumParam<ZoomMode>,
    /// The editor state, saved together with the parameter state so the custom scaling can be
    /// restored.
    #[persist = "editor-state"]
    editor_state: Arc<ViziaState>,
}

#[derive(Enum, Debug, PartialEq)]
enum ZoomMode {
    /// Don't show the length of the envelope, just the shape
    #[id = "shape"]
    #[name = "shape"]
    Shape,
    /// Show the length of the attack and the release relative to each other.
    #[id = "relative"]
    #[name = "relative"]
    Relative,
    /// Show the absolute length of the attack and the release
    #[id = "absolute"]
    #[name = "absolute"]
    Absolute,
}

#[derive(Enum, Debug, PartialEq)]
enum LatencyMode {
    /// Minimal, but variable latency
    #[id = "minimal"]
    #[name = "minimal"]
    Minimal,
    /// Fixate the latency at the maximum
    #[id = "fixed"]
    #[name = "fixed"]
    Fixed,
}

impl Default for LambParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),

            bypass: BoolParam::new("bypass",false)
                .with_value_to_string(formatters::v2s_bool_bypass())
                .with_string_to_value(formatters::s2v_bool_bypass())
                .make_bypass(),
            latency_mode: EnumParam::new("latency_mode", LatencyMode::Fixed),
            input_gain: FloatParam::new("input_gain", 0.0, FloatRange::Linear { min: -24.0, max: 24.0})
                .with_unit(" dB")
                .with_step_size(0.1),
            strength: FloatParam::new("strength", 100.0, FloatRange::Linear { min: 0.0, max: 100.0})
                .with_unit(" %")
                .with_step_size(1.0),
            thresh: FloatParam::new("thresh", -1.0, FloatRange::Linear { min: -30.0, max: 0.0})
                .with_unit(" dB")
                .with_step_size(0.1),
            attack: FloatParam::new("attack", 9.0, FloatRange::Skewed {
                min: 0.0, max: 50.0,
                factor: FloatRange::skew_factor(-0.75),
            })
                .with_unit(" ms")
                .with_step_size(0.01),
            attack_shape: FloatParam::new("attack_shape", 0.0, FloatRange::Linear { min: 0.0, max: 1.0})
                .with_step_size(0.01),
            release: FloatParam::new("release", 60.0, FloatRange::Skewed {
                min: 1.0, max: 500.0,
                factor: FloatRange::skew_factor(-1.0),
            })
                .with_unit(" ms")
                .with_step_size(0.01),
            release_shape: FloatParam::new("release_shape", 0.5, FloatRange::Linear { min: 0.0, max: 1.0})
                .with_step_size(0.01),
            release_hold: FloatParam::new("release_hold", 50.0, FloatRange::Linear {
                min: 0.0, max: 50.0,
            })
                .with_unit(" ms")
                .with_step_size(0.01),
            knee: FloatParam::new("knee", 1.0, FloatRange::Linear { min: 0.0, max: 30.0})
                .with_unit(" dB")
                .with_step_size(0.1),
            link: FloatParam::new("link", 0.0, FloatRange::Linear { min: 0.0, max: 100.0})
                .with_unit(" %")
                .with_step_size(1.0),
            output_gain: FloatParam::new("output_gain", 0.0, FloatRange::Linear { min: -24.0, max: 24.0})
                .with_unit(" dB")
                .with_step_size(0.1),
            zoom_mode: EnumParam::new("zoom_mode", ZoomMode::Relative),
        }
    }
}

pub const BYPASS_PI: ParamIndex = ParamIndex(0);
pub const LATENCY_MODE_PI: ParamIndex = ParamIndex(1);
pub const INPUT_GAIN_PI: ParamIndex = ParamIndex(2);
pub const STRENGTH_PI: ParamIndex = ParamIndex(3);
pub const THRESH_PI: ParamIndex = ParamIndex(4);
pub const ATTACK_PI: ParamIndex = ParamIndex(5);
pub const ATTACK_SHAPE_PI: ParamIndex = ParamIndex(6);
pub const RELEASE_PI: ParamIndex = ParamIndex(7);
pub const RELEASE_SHAPE_PI: ParamIndex = ParamIndex(8);
pub const RELEASE_HOLD_PI: ParamIndex = ParamIndex(9);
pub const KNEE_PI: ParamIndex = ParamIndex(10);
pub const LINK_PI: ParamIndex = ParamIndex(11);
pub const OUTPUT_GAIN_PI: ParamIndex = ParamIndex(12);
pub const GAIN_REDUCTION_LEFT_PI: ParamIndex = ParamIndex(13);
pub const GAIN_REDUCTION_RIGHT_PI: ParamIndex = ParamIndex(14);
pub const LATENCY_PI: ParamIndex = ParamIndex(15);
pub const ZOOM_MODE_PI: ParamIndex = ParamIndex(16);
