use atomic_float::AtomicF32;
use nih_plug::prelude::{util, Editor};
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;

use crate::LambParams;
use crate::LatencyMode;
use crate::ZoomMode;

include!("gain_reduction_meter.rs");

#[derive(Lens)]
struct LambData {
    params: Arc<LambParams>,
    peak_meter: Arc<AtomicF32>,
    gain_reduction_left: Arc<AtomicF32>,
    gain_reduction_right: Arc<AtomicF32>,
}

impl Model for LambData {}

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<ViziaState> {
    // ViziaState::new(|| (1035, 690))
    // ViziaState::new(|| (1227, 690))
    ViziaState::new(|| (((16.0 / 9.0) * 690.0) as u32, 690))
}

pub(crate) fn create(
    params: Arc<LambParams>,
    peak_meter: Arc<AtomicF32>,
    gain_reduction_left: Arc<AtomicF32>,
    gain_reduction_right: Arc<AtomicF32>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        assets::register_noto_sans_light(cx);
        assets::register_noto_sans_thin(cx);

        // Add the stylesheet to the app
        cx.add_stylesheet(include_style!("src/style.css"))
            .expect("Failed to load stylesheet");

        LambData {
            params: params.clone(),
            peak_meter: peak_meter.clone(),
            gain_reduction_left: gain_reduction_left.clone(),
            gain_reduction_right: gain_reduction_right.clone(),
        }
        .build(cx);

        // everything
        VStack::new(cx, |cx| {
            // parameters + graph
            HStack::new(cx, |cx| {
                // parameters
                VStack::new(cx, |cx| {
                    Label::new(cx, "") // spacer
                        // Label::new(cx, "lamb") // title
                        // Label::new(cx, "🐑") // doesn't render
                        .class("plugin-name");
                    Label::new(cx, "input gain").class("fader-label");
                    ParamSlider::new(cx, LambData::params, |params| &params.input_gain)
                        .bottom(Pixels(6.0));
                    // level + time
                    HStack::new(cx, |cx| {
                        // level
                        VStack::new(cx, |cx| {
                            HStack::new(cx, |cx| {
                                // bypass and latency_mode
                                VStack::new(cx, |cx| {
                                    // label & slider
                                    Label::new(cx, "bypass").class("fader-label");
                                    ParamButton::new(cx, LambData::params, |params| &params.bypass)
                                        .with_label("")
                                        .for_bypass()
                                        .width(Percentage(95.0))
                                        .right(Percentage(5.0));
                                }) // label & slider
                                .height(Auto)
                                .class("center");
                                // label & slider
                                VStack::new(cx, |cx| {
                                    Label::new(cx, "latency mode").class("fader-label");
                                    ParamSlider::new(cx, LambData::params, |params| {
                                        &params.latency_mode
                                    })
                                    .set_style(ParamSliderStyle::CurrentStepLabeled { even: true })
                                    .width(Percentage(95.0))
                                    .left(Percentage(5.0));
                                }) // label & slider
                                .height(Auto)
                                .class("center");
                            }) // bypass and latency_mode
                            .height(Auto)
                            .width(Percentage(100.0)); // level + time
                            Label::new(cx, "strength").class("fader-label");
                            ParamSlider::new(cx, LambData::params, |params| &params.strength);
                            Label::new(cx, "threshold").class("fader-label");
                            ParamSlider::new(cx, LambData::params, |params| &params.thresh);
                            Label::new(cx, "knee").class("fader-label");
                            ParamSlider::new(cx, LambData::params, |params| &params.knee);
                            Label::new(cx, "link").class("fader-label");
                            ParamSlider::new(cx, LambData::params, |params| &params.link);
                        }) // level
                        .height(Percentage(100.0))
                        .class("center")
                        .right(Percentage(2.5));
                        // time
                        VStack::new(cx, |cx| {
                            Label::new(cx, "attack").class("fader-label");
                            ParamSlider::new(cx, LambData::params, |params| &params.attack);
                            Label::new(cx, "attack shape").class("fader-label");
                            ParamSlider::new(cx, LambData::params, |params| &params.attack_shape);
                            Label::new(cx, "release").class("fader-label");
                            ParamSlider::new(cx, LambData::params, |params| &params.release);
                            Label::new(cx, "release shape").class("fader-label");
                            ParamSlider::new(cx, LambData::params, |params| &params.release_shape)
                                .set_style(ParamSliderStyle::FromLeft);
                            Label::new(cx, "release hold").class("fader-label");
                            ParamSlider::new(cx, LambData::params, |params| &params.release_hold);
                        }) // time
                            .class("center")
                            .left(Percentage(2.5));
                    })
                    .width(Percentage(100.0)); // level + time

                    Label::new(cx, "output gain").class("fader-label");
                    ParamSlider::new(cx, LambData::params, |params| &params.output_gain)
                        .bottom(Pixels(6.0));
                }) // parameters
                    .right(Percentage(2.5))
                    .class("center");
                // graph + zoom
                VStack::new(cx, |cx| {
                    // Label::new(cx, "") // spacer
                    Label::new(cx, "lamb") // title
                        .class("plugin-name")
                        .left(Stretch(1.0))
                        .right(Pixels(0.0));
                    VStack::new(cx, |cx| {
                        // label & slider
                        Label::new(cx, "zoom mode").class("fader-label");
                        ParamSlider::new(cx, LambData::params, |params| &params.zoom_mode)
                            .set_style(ParamSliderStyle::CurrentStepLabeled { even: true })
                            .bottom(Pixels(6.0));
                        Label::new(cx, "").class("fader-label"); // spacer
                        AttackReleaseGraph::new(cx, LambData::params);
                    }) // label & slider
                        .width(Percentage(100.0))
                        .class("center");
                }) // graph + zoom
                // .right(Percentage(2.5))
                    .class("center")
                // .width(Percentage(50.0))
                .height(Percentage(100.0)); // graph + zoom
            })
            .width(Percentage(100.0)); // parameters + graph

            // meters
            VStack::new(cx, |cx| {
                Label::new(cx, "input level").class("fader-label");
                PeakMeter::new(
                    cx,
                    LambData::peak_meter
                        .map(|peak_meter| util::gain_to_db(peak_meter.load(Ordering::Relaxed))),
                    Some(Duration::from_millis(600)),
                );
                Label::new(cx, "gain reduction left").class("fader-label");
                GainReductionMeter::new(
                    cx,
                    LambData::gain_reduction_left
                        .map(|gain_reduction_left| gain_reduction_left.load(Ordering::Relaxed)),
                    Some(Duration::from_millis(600)),
                )
                .width(Percentage(100.0));
                Label::new(cx, "gain reduction right").class("fader-label");
                GainReductionMeter::new(
                    cx,
                    LambData::gain_reduction_right
                        .map(|gain_reduction_right| gain_reduction_right.load(Ordering::Relaxed)),
                    Some(Duration::from_millis(600)),
                )
                .width(Percentage(100.0));
            }) // meters
                .width(Percentage(100.0))
                .height(Auto)
                .class("center"); // meters
        }) // everything
            .width(Percentage(95.0))
            .height(Percentage(95.0))
            .left(Percentage(2.5))
            .right(Percentage(2.5))
            .class("center");
        ResizeHandle::new(cx);
    })
}
///////////////////////////////////////////////////////////////////////////////
//                             AttackReleaseGraph                            //
///////////////////////////////////////////////////////////////////////////////

pub struct AttackReleaseGraph<AttackReleaseDataL: Lens<Target = Arc<LambParams>>> {
    attack_release_data: AttackReleaseDataL,
}

impl<AttackReleaseDataL: Lens<Target = Arc<LambParams>>> AttackReleaseGraph<AttackReleaseDataL> {
    pub fn new(cx: &mut Context, attack_release_data: AttackReleaseDataL) -> Handle<Self> {
        Self {
            attack_release_data,
        }
        .build(cx, |cx| {
            // If we want the view to contain other views we can build those here.
        })
        // Redraw when lensed data changes
        .bind(attack_release_data, |mut handle, _| handle.needs_redraw())
    }
}
impl<AttackReleaseDataL: Lens<Target = Arc<LambParams>>> View
    for AttackReleaseGraph<AttackReleaseDataL>
{
    // for css:
    fn element(&self) -> Option<&'static str> {
        Some("attack-release-graph")
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        // Get the bounding box of the current view.
        let bounds = cx.bounds();
        let attack = self.attack_release_data.get(cx).attack.value();
        let attack_shape = self.attack_release_data.get(cx).attack_shape.value();
        let release = self.attack_release_data.get(cx).release.value();
        let release_shape = self.attack_release_data.get(cx).release_shape.value();
        let zoom_mode = self.attack_release_data.get(cx).zoom_mode.value();

        // let background_color = cx.background_color();
        let border_color = cx.border_color();
        let outline_color = cx.outline_color();
        let opacity = cx.opacity();
        // let mut background_color: vg::Color = background_color.into();
        // background_color.set_alphaf(background_color.a * opacity);
        let mut border_color: vg::Color = border_color.into();
        border_color.set_alphaf(border_color.a * opacity);
        // let border_width = cx.scale_factor() * cx.border_width();
        let border_width = cx.border_width();

        let rounding = 3.0 * border_width;

        // Create a new `Path` from the `vg` module.
        let mut path = vg::Path::new();
        let x = bounds.x + border_width / 2.0;
        let y = bounds.y + border_width / 2.0;
        let w = bounds.w - border_width;
        let h = bounds.h - border_width;

        let max_attack = 50.0;
        let max_release = 500.0;

        // Based on an algorithm by Dario Sanfilippo:
        // https://www.desmos.com/calculator/2hxvf9q194
        // Adapted by Bart Brouns:
        // https://www.desmos.com/calculator/ubmqgogu2s
        // simplified:
        // https://www.desmos.com/calculator/yiwvcjiony
        fn sine(x: f32) -> f32 {
            let pi = std::f32::consts::PI;
            let xpi = x * pi + 1.5 * pi;
            (xpi.sin() + 1.0) / 2.0
        }
        fn fxk(k: f32, x: f32) -> f32 {
            match k {
                0.0 => x,
                _ => (1.0 - ((k * x).exp())) / (1.0 - k.exp()),
            }
        }
        fn fm1m2(c: f32, x: f32) -> f32 {
            fxk(2.42 * (c), x)
        }
        fn c2(c: f32, x: f32) -> f32 {
            sine(fm1m2(c, x))
        }
        fn curve(c: f32, x: f32) -> f32 {
            c2(c.powf(1.0 + 0.42 * c), x)
        }

        // Fill with background color
        // let paint = vg::Paint::color(background_color);
        // canvas.fill_path(&path, &paint);

        // add the attack / release curve
        canvas.stroke_path(
            &{
                let x = bounds.x + border_width * 1.0;
                let y = bounds.y + border_width * 1.5;
                let w = bounds.w - border_width * 2.0;
                let h = bounds.h - border_width * 3.0;
                let center = match zoom_mode {
                    ZoomMode::Shape => w * 0.5,
                    ZoomMode::Relative => w * attack / (attack + release),
                    ZoomMode::Absolute => w * max_attack / (max_attack + max_release),
                };

                let mut start = 0.0;
                let mut end = w;
                if zoom_mode == ZoomMode::Absolute {
                    start = ((max_attack - attack) / max_attack) * center;
                    end = center + ((release / max_release) * (w - center));
                }

                let mut path = vg::Path::new();
                // start of the graph
                path.move_to(x, y);
                // start of the attack
                path.line_to(x + start, y);
                // draw attack
                for dx in start as i32..center as i32 {
                    let dy = (dx as f32 - start) / (center - start);
                    path.line_to(x + dx as f32, y + curve(attack_shape, dy) * h);
                }
                // make sure we also hit the bottom when mode is absolute and times are short
                path.line_to(x + center, y + h);
                // draw release
                for dx in center as i32..end as i32 {
                    let dy = (dx as f32 - center) / (end - center);
                    path.line_to(
                        x + dx as f32,
                        y + h - (curve(release_shape, dy * -1.0 + 1.0) * -1.0 + 1.0) * h,
                    );
                }
                path.line_to(x + end, y);
                path.line_to(x + w, y);
                path
            },
            &vg::Paint::color(outline_color.into()).with_line_width(border_width),
        );
        // add outline
        canvas.stroke_path(
            &{
                let mut path = vg::Path::new();
                path.rounded_rect(x, y, w, h, rounding);
                path
            },
            &vg::Paint::color(border_color).with_line_width(border_width),
        );
    }
}
