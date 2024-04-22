use crate::LambParams;
use crate::ZoomMode;
use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::vizia::vg;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::{Arc, Mutex};

use cyma::visualizers::GraphModifiers;
use cyma::{
    prelude::*,
    utils::{MinimaBuffer, PeakBuffer},
    visualizers::{Graph, Grid, Meter, UnitRuler},
};

const METER_MIN: f32 = -26.0;
const METER_MAX: f32 = 0.0;

#[derive(Lens, Clone)]
struct LambData {
    params: Arc<LambParams>,
    level_buffer_l: Arc<Mutex<PeakBuffer>>,
    level_buffer_r: Arc<Mutex<PeakBuffer>>,
    gr_buffer_l: Arc<Mutex<MinimaBuffer>>,
    gr_buffer_r: Arc<Mutex<MinimaBuffer>>,
}

impl LambData {
    pub(crate) fn new(
        params: Arc<LambParams>,
        level_buffer_l: Arc<Mutex<PeakBuffer>>,
        level_buffer_r: Arc<Mutex<PeakBuffer>>,
        gr_buffer_l: Arc<Mutex<MinimaBuffer>>,
        gr_buffer_r: Arc<Mutex<MinimaBuffer>>,
    ) -> Self {
        Self {
            params,
            level_buffer_l,
            level_buffer_r,
            gr_buffer_l,
            gr_buffer_r,
        }
    }
}

impl Model for LambData {}

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<ViziaState> {
    // width , height
    // ViziaState::new(|| (((16.0 / 9.0) * 720.0) as u32, 720))
    ViziaState::new(|| (1280, 720))
}

pub(crate) fn create(
    params: Arc<LambParams>,
    level_buffer_l: Arc<Mutex<PeakBuffer>>,
    level_buffer_r: Arc<Mutex<PeakBuffer>>,
    gr_buffer_l: Arc<Mutex<MinimaBuffer>>,
    gr_buffer_r: Arc<Mutex<MinimaBuffer>>,
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
            level_buffer_l: level_buffer_l.clone(),
            level_buffer_r: level_buffer_r.clone(),
            gr_buffer_l: gr_buffer_l.clone(),
            gr_buffer_r: gr_buffer_r.clone(),
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
                    // three colomns
                    HStack::new(cx, |cx| {
                        // first
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
                            Label::new(cx, "ratio").class("fader-label");
                            ParamSlider::new(cx, LambData::params, |params| &params.strength);
                            Label::new(cx, "threshold").class("fader-label");
                            ParamSlider::new(cx, LambData::params, |params| &params.thresh);
                            Label::new(cx, "knee").class("fader-label");
                            ParamSlider::new(cx, LambData::params, |params| &params.knee);
                        }) // first
                        .height(Auto)
                        .class("center")
                        .right(Percentage(2.5));
                        // second
                        VStack::new(cx, |cx| {
                            Label::new(cx, "lookahead").class("fader-label");
                            ParamSlider::new(cx, LambData::params, |params| &params.lookahead);
                            Label::new(cx, "link").class("fader-label");
                            ParamSlider::new(cx, LambData::params, |params| &params.link);
                            Label::new(cx, "release hold").class("fader-label");
                            ParamSlider::new(cx, LambData::params, |params| &params.release_hold);
                            Label::new(cx, "adaptive release").class("fader-label");
                            ParamSlider::new(cx, LambData::params, |params| {
                                &params.adaptive_release
                            })
                            .set_style(ParamSliderStyle::FromLeft);
                        }) // second
                        .height(Auto)
                        .class("center")
                        .left(Percentage(1.25))
                        .right(Percentage(1.25));
                        // third
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
                        }) // third
                        .height(Auto)
                        .class("center")
                        .left(Percentage(2.5));
                    })
                    .height(Auto)
                    .width(Percentage(100.0)); // three colomns

                    Label::new(cx, "output gain").class("fader-label");
                    ParamSlider::new(cx, LambData::params, |params| &params.output_gain)
                        .bottom(Pixels(6.0));
                }) // parameters
                .width(Percentage(75.0))
                .height(Auto)
                .right(Percentage(2.5))
                .class("center");
                // graph + zoom
                VStack::new(cx, |cx| {
                    // Label::new(cx, ""); // spacer
                    Label::new(cx, "lamb") // title
                        .class("plugin-name")
                        .left(Stretch(1.0))
                        .right(Pixels(0.0));
                    Label::new(cx, "zoom mode").class("fader-label");
                    ParamSlider::new(cx, LambData::params, |params| &params.zoom_mode)
                        .set_style(ParamSliderStyle::CurrentStepLabeled { even: true })
                        .bottom(Pixels(6.0));
                    Label::new(cx, "").class("fader-label"); // spacer
                    AttackReleaseGraph::new(cx, LambData::params).height(Pixels(200.0));
                    // .height(Pixels(260.0));
                    Label::new(cx, "zoom mode").class("fader-label");
                    ParamSlider::new(cx, LambData::params, |params| &params.zoom_mode)
                        .set_style(ParamSliderStyle::CurrentStepLabeled { even: true })
                        .bottom(Pixels(6.0));
                }) // graph + zoom
                .height(Auto)
                .class("center");
            }) // parameters + graph
            .height(Auto)
            .width(Percentage(100.0));

            peak_graph(cx);
        }) // everything
        .width(Percentage(95.0))
        .height(Auto)
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

        let border_color = cx.border_color();
        let outline_color = cx.outline_color();
        let opacity = cx.opacity();
        let mut background_color: vg::Color = cx.background_color().into();
        background_color.set_alphaf(background_color.a * opacity);
        let mut border_color: vg::Color = border_color.into();
        border_color.set_alphaf(border_color.a * opacity);
        // let border_width = cx.scale_factor() * cx.border_width();
        let border_width = cx.border_width();

        let rounding = 3.0 * border_width;

        // Create a new `Path` from the `vg` module.
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
        let mut path = vg::Path::new();
        {
            let x = bounds.x + border_width / 2.0;
            let y = bounds.y + border_width / 2.0;
            let w = bounds.w - border_width;
            let h = bounds.h - border_width;
            path.move_to(x, y);
            path.line_to(x, y + h);
            path.line_to(x + w, y + h);
            path.line_to(x + w, y);
            path.line_to(x, y);
            path.close();
        }
        let paint = vg::Paint::color(background_color);
        canvas.fill_path(&path, &paint);

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

/// Draws a peak graph with a grid backdrop, unit ruler, and a peak meter to side.
fn peak_graph(cx: &mut Context) {
    HStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
            ZStack::new(cx, |cx| {
                Grid::new(
                    cx,
                    ValueScaling::Linear,
                    (METER_MIN, METER_MAX),
                    vec![0.0, -6.0, -12.0, -18.0, -24.0],
                    Orientation::Horizontal,
                )
                .color(Color::rgba(160, 160, 160, 60));

                // level
                Graph::new(
                    cx,
                    LambData::level_buffer_l,
                    (METER_MIN, METER_MAX),
                    ValueScaling::Decibels,
                )
                .color(Color::rgba(0, 0, 255, 30))
                .background_color(Color::rgba(0, 0, 0, 40));

                Graph::new(
                    cx,
                    LambData::level_buffer_r,
                    (METER_MIN, METER_MAX),
                    ValueScaling::Decibels,
                )
                .color(Color::rgba(255, 0, 0, 30))
                .background_color(Color::rgba(0, 0, 0, 40));

                // gain reduction
                Graph::new(
                    cx,
                    LambData::gr_buffer_l,
                    (METER_MIN, METER_MAX),
                    ValueScaling::Decibels,
                )
                .color(Color::rgba(0, 0, 255, 255))
                .background_color(Color::rgba(250, 250, 250, 40))
                .fill_from(0.0);
                Graph::new(
                    cx,
                    LambData::gr_buffer_r,
                    (METER_MIN, METER_MAX),
                    ValueScaling::Decibels,
                )
                .color(Color::rgba(255, 0, 0, 255))
                .background_color(Color::rgba(250, 250, 250, 40))
                .fill_from(0.0);
            });
            // gain reduction
            HStack::new(cx, |cx| {
                Meter::new(
                    cx,
                    LambData::gr_buffer_l,
                    (METER_MIN, METER_MAX),
                    ValueScaling::Decibels,
                    Orientation::Vertical,
                )
                .background_color(Color::rgb(250, 250, 250))
                .color(Color::rgba(0, 0, 255, 255));
            })
            .width(Pixels(15.0))
            .background_color(Color::rgb(220, 220, 220));
            HStack::new(cx, |cx| {
                Meter::new(
                    cx,
                    LambData::gr_buffer_r,
                    (METER_MIN, METER_MAX),
                    ValueScaling::Decibels,
                    Orientation::Vertical,
                )
                .background_color(Color::rgb(250, 250, 250))
                .color(Color::rgba(255, 0, 0, 255));
            })
                .width(Pixels(15.0))
                .background_color(Color::rgb(220, 220, 220));
            // level
            Meter::new(
                cx,
                LambData::level_buffer_l,
                (METER_MIN, METER_MAX),
                ValueScaling::Decibels,
                Orientation::Vertical,
            )
                .width(Pixels(15.0))
                .color(Color::rgba(0, 0, 255, 125))
                .background_color(Color::rgba(60, 60, 60, 60));
            Meter::new(
                cx,
                LambData::level_buffer_r,
                (METER_MIN, METER_MAX),
                ValueScaling::Decibels,
                Orientation::Vertical,
            )
                .width(Pixels(15.0))
                .color(Color::rgba(255, 0, 0, 125))
                .background_color(Color::rgba(60, 60, 60, 60));
        })
            .border_color(Color::rgb(80, 80, 80))
            .border_width(Pixels(1.));
        UnitRuler::new(
            cx,
            (METER_MIN, METER_MAX),
            ValueScaling::Linear,
            vec![
                (-0.0, "0db"),
                (-6.0, "-6db"),
                (-12.0, "-12db"),
                (-18.0, "-18db"),
                (-24.0, "-24db"),
            ],
            Orientation::Vertical,
        )
            .font_size(12.)
            .color(Color::rgb(30, 30, 30))
            .width(Pixels(32.));
    })
        .top(Pixels(20.0))
        .height(Pixels(260.0))
        .width(Percentage(100.0))
        .col_between(Pixels(8.));
}
