use faust_types::ParamIndex;
use std::sync::atomic::{AtomicBool, Ordering};

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
    #[id = "adaptive_release"]
    adaptive_release: FloatParam,
    #[id = "lookahead"]
    lookahead: FloatParam,
    #[id = "output_gain"]
    output_gain: FloatParam,
    #[id = "zoom_mode"]
    zoom_mode: EnumParam<ZoomMode>,
    #[id = "time_scale"]
    time_scale: EnumParam<TimeScale>,
    #[id = "in_out"]
    in_out: BoolParam,
    #[id = "show_left"]
    show_left: BoolParam,
    #[id = "show_right"]
    show_right: BoolParam,

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
enum TimeScale {
    #[id = "0_5s"]
    #[name = "0.5 seconds"]
    HalfSec,
    #[id = "1s"]
    #[name = "1 second"]
    OneSec,
    #[id = "2s"]
    #[name = "2 seconds"]
    TwoSec,
    #[id = "4s"]
    #[name = "4 seconds"]
    FourSec,
    #[id = "8s"]
    #[name = "8 seconds"]
    EightSec,
    #[id = "16s"]
    #[name = "16 seconds"]
    SixteenSec,
    #[id = "32s"]
    #[name = "32 seconds"]
    ThirtytwoSec,
    #[id = "64"]
    #[name = "64 seconds"]
    SixtyfourSec,
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

/// Format a positive number as a compression ratio. A value of 4 will be formatted as `4.0:1` while
/// 0.25 is formatted as `1:4.0`.
pub fn v2s_compression_ratio(digits: usize) -> Arc<dyn Fn(f32) -> String + Send + Sync> {
    Arc::new(move |value| {
        if value >= 1.0 {
            format!("{value:.digits$}:1")
        } else {
            format!("1:{:.digits$}", value.recip())
        }
    })
}

/// take an f32 compression strength value and turn it into a ratio string
pub fn strength_to_ratio() -> Arc<dyn Fn(f32) -> String + Send + Sync> {
    Arc::new(move |value| {
        if value < 100.0 {
            // value is in %, to make it look ok in the faust version
            let strength = value * 0.01;
            let ratio = 1.0 / (1.0 - strength);
            if ratio <= 10.0 {
                format!("{ratio:.2}:1")
            } else {
                if ratio <= 100.0 {
                    format!("{ratio:.1}:1")
                } else {
                    format!("100:1")
                }
            }
        } else {
            format!("inf:1")
        }
    })
}

/// take a ratio string and turn it into an f32 compression strength value
pub fn ratio_to_strength() -> Arc<dyn Fn(&str) -> Option<f32> + Send + Sync> {
    Arc::new(|string| {
        let string = string.trim();
        string
            .trim()
            .split_once(':')
            .and_then(|(numerator, denominator)| {
                let numerator: f32 = numerator.trim().parse().ok()?;
                let denominator: f32 = denominator.trim().parse().ok()?;
                let ratio = (numerator / denominator).max(1.0);
                let strength = 1.0 - (1.0 / ratio);
                let percentage = strength * 100.0;
                Some(percentage)
            })
            // Just parse the value directly if it doesn't contain a colon
            .or_else(|| {
                string
                    .parse::<f32>()
                    .ok()
                    .map(|value| {
                        let strength = 1.0 - (1.0 / value.max(1.0));
                        strength * 100.0
                        // if parsing fails, we assume inf:1 was meant
                    })
                    .or_else(|| Some(100.0))
            })
    })
}

// .with_value_to_string(bool_to_in_out())
// .with_string_to_value(in_out_to_bool())
// pub fn bool_to_in_out()

/// Display 'post' or 'pre' depending on whether the parameter is true or false.
pub fn v2s_bool_in_out() -> Arc<dyn Fn(bool) -> String + Send + Sync> {
    Arc::new(move |value| {
        if value {
            String::from("post")
        } else {
            String::from("pre")
        }
    })
}

/// Parse a string in the same format as [`v2s_bool_in_out()`].
pub fn s2v_bool_in_out() -> Arc<dyn Fn(&str) -> Option<bool> + Send + Sync> {
    Arc::new(|string| {
        let string = string.trim();
        if string.eq_ignore_ascii_case("post") {
            Some(true)
        } else if string.eq_ignore_ascii_case("pre") {
            Some(false)
        } else {
            None
        }
    })
}

impl LambParams {
    pub fn new(should_update_time_scale: Arc<AtomicBool>) -> Self {
        Self {
            editor_state: editor::default_state(),

            bypass: BoolParam::new("bypass", false)
                .with_value_to_string(formatters::v2s_bool_bypass())
                .with_string_to_value(formatters::s2v_bool_bypass())
                .make_bypass(),
            latency_mode: EnumParam::new("latency_mode", LatencyMode::Fixed),
            input_gain: FloatParam::new(
                "input_gain",
                0.0,
                FloatRange::Linear {
                    min: -24.0,
                    max: 24.0,
                },
            )
            .with_unit(" dB")
            .with_step_size(0.1),
            strength: FloatParam::new(
                "ratio",
                100.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 100.0,
                },
            )
            .with_value_to_string(strength_to_ratio())
            .with_string_to_value(ratio_to_strength()),
            thresh: FloatParam::new(
                "thresh",
                -1.0,
                FloatRange::Linear {
                    min: -30.0,
                    max: 0.0,
                },
            )
            .with_unit(" dB")
            .with_step_size(0.1),
            attack: FloatParam::new(
                "attack",
                9.0,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 50.0,
                    factor: FloatRange::skew_factor(-1.0),
                },
            )
            .with_unit(" ms")
            .with_step_size(0.01)
            .non_automatable(),
            attack_shape: FloatParam::new(
                "attack_shape",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_step_size(0.01),
            release: FloatParam::new(
                "release",
                60.0,
                FloatRange::Skewed {
                    min: 1.0,
                    max: 500.0,
                    factor: FloatRange::skew_factor(-1.0),
                },
            )
            .with_unit(" ms")
            .with_step_size(0.01),
            release_shape: FloatParam::new(
                "release_shape",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_step_size(0.01),
            release_hold: FloatParam::new(
                "release_hold",
                50.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 50.0,
                },
            )
            .with_unit(" ms")
            .with_step_size(0.01)
            .non_automatable(),
            knee: FloatParam::new(
                "knee",
                1.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 30.0,
                },
            )
                .with_unit(" dB")
                .with_step_size(0.1),
            link: FloatParam::new(
                "link",
                0.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 100.0,
                },
            )
                .with_unit(" %")
                .with_step_size(1.0),
            adaptive_release: FloatParam::new(
                "adaptive_release",
                50.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 100.0,
                },
            )
                .with_unit(" %")
                .with_step_size(1.0),
            lookahead: FloatParam::new(
                "lookahead",
                100.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 100.0,
                },
            )
                .with_unit(" %")
                .with_step_size(1.0)
                .non_automatable(),
            output_gain: FloatParam::new(
                "output_gain",
                0.0,
                FloatRange::Linear {
                    min: -24.0,
                    max: 24.0,
                },
            )
                .with_unit(" dB")
                .with_step_size(0.1),
            zoom_mode: EnumParam::new("zoom_mode", ZoomMode::Relative)
                .hide()
                .hide_in_generic_ui(),
            time_scale: EnumParam::new("time_scale", TimeScale::FourSec)
                .with_callback({
                    let should_update_time_scale = should_update_time_scale.clone();
                    Arc::new(move |_| should_update_time_scale.store(true, Ordering::Release))
                })
                .hide()
                .hide_in_generic_ui(),
            in_out: BoolParam::new("in_out", true)
                .with_value_to_string(v2s_bool_in_out())
                .with_string_to_value(s2v_bool_in_out())
                .hide()
                .hide_in_generic_ui(),
            show_left: BoolParam::new("show_left", true)
                .hide()
                .hide_in_generic_ui(),
            show_right: BoolParam::new("show_right", true)
                .hide()
                .hide_in_generic_ui(),
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
pub const ADAPTIVE_RELEASE_PI: ParamIndex = ParamIndex(12);
pub const LOOKAHEAD_PI: ParamIndex = ParamIndex(13);
pub const OUTPUT_GAIN_PI: ParamIndex = ParamIndex(14);
pub const GAIN_REDUCTION_LEFT_PI: ParamIndex = ParamIndex(15);
pub const GAIN_REDUCTION_RIGHT_PI: ParamIndex = ParamIndex(16);
pub const LATENCY_PI: ParamIndex = ParamIndex(17);
