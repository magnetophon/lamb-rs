use atomic_float::AtomicF32;
use nih_plug::prelude::{util, Editor};
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;

use crate::LambParams;

include!("gain_reduction_meter.rs");

#[derive(Lens)]
struct Data {
    params: Arc<LambParams>,
    peak_meter: Arc<AtomicF32>,
    gain_reduction_left: Arc<AtomicF32>,
    gain_reduction_right: Arc<AtomicF32>,
}

impl Model for Data {}

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (340, 680))
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

        Data {
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
                    Label::new(cx, "lamb")
                        .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                        .font_weight(FontWeightKeyword::Thin)
                        .font_size(30.0)
                        .height(Pixels(60.0))
                        .child_top(Stretch(1.0))
                        .child_bottom(Pixels(10.0))
                        ;
                    Label::new(cx, "input gain");
                    ParamSlider::new(cx, Data::params, |params| &params.input_gain).bottom(Pixels(6.0)) ;
                    // level + time
                    HStack::new(cx, |cx| {
                        // level
                        VStack::new(cx, |cx| {
                            Label::new(cx, "");
                            ParamButton::new(cx, Data::params, |params| &params.bypass)
                                .for_bypass();
                            Label::new(cx, "strength");
                            ParamSlider::new(cx, Data::params, |params| &params.strength) ;
                            Label::new(cx, "threshold");
                            ParamSlider::new(cx, Data::params, |params| &params.thresh);
                            Label::new(cx, "knee");
                            ParamSlider::new(cx, Data::params, |params| &params.knee);
                            Label::new(cx, "link");
                            ParamSlider::new(cx, Data::params, |params| &params.link);
                        })
                            .child_left(Stretch(1.0))
                            .child_right(Stretch(1.0))
                            .width(Percentage(47.5))
                            .left(Percentage(0.0))
                            .right(Percentage(2.5))
                            ; // level
                        // time
                        VStack::new(cx, |cx| {
                            Label::new(cx, "attack");
                            ParamSlider::new(cx, Data::params, |params| &params.attack);
                            Label::new(cx, "attack shape");
                            ParamSlider::new(cx, Data::params, |params| &params.attack_shape);
                            Label::new(cx, "release");
                            ParamSlider::new(cx, Data::params, |params| &params.release);
                            Label::new(cx, "release shape");
                            ParamSlider::new(cx, Data::params, |params| &params.release_shape);
                            Label::new(cx, "release hold");
                            ParamSlider::new(cx, Data::params, |params| &params.release_hold);
                        })
                            .child_left(Stretch(1.0))
                            .child_right(Stretch(1.0))
                            .width(Percentage(47.5))
                            .left(Percentage(2.5))
                            .right(Percentage(0.0))
                            ; // time
                    })
                        .width(Percentage(100.0))
                    // .height(Stretch(1.0))
                        ; // level + time

                    Label::new(cx, "output gain");
                    ParamSlider::new(cx, Data::params, |params| &params.output_gain)
                        .bottom(Pixels(6.0))
                        ;
                })
                    .child_left(Stretch(1.0))
                    .child_right(Stretch(1.0))
                    ; // parameters
                // graph + zoom
                // VStack::new(cx, |cx| {
                // }); // graph + zoom
            }); // parameters + graph

            // meters
            VStack::new(cx, |cx| {
                Label::new(cx, "input level");
                PeakMeter::new(
                    cx,
                    Data::peak_meter
                        .map(|peak_meter| util::gain_to_db(peak_meter.load(Ordering::Relaxed))),
                    Some(Duration::from_millis(600)),
                );
                Label::new(cx, "gain reduction left");
                GainReductionMeter::new(
                    cx,
                    Data::gain_reduction_left
                        .map(|gain_reduction_left| gain_reduction_left.load(Ordering::Relaxed)),
                    Some(Duration::from_millis(600)),
                )
                    .width(Percentage(100.0));
                Label::new(cx, "gain reduction right");
                GainReductionMeter::new(
                    cx,
                    Data::gain_reduction_right
                        .map(|gain_reduction_right| gain_reduction_right.load(Ordering::Relaxed)),
                    Some(Duration::from_millis(600)),
                )
                    .width(Percentage(100.0));
                // This is how adding padding works in vizia
                // .top(Pixels(10.0));
            })
                .height(Auto)
                .child_left(Stretch(1.0))
                .child_right(Stretch(1.0))
                ; // meters
        }) // everything
        // .row_between(Pixels(0.0))
        // .child_left(Stretch(1.0))
        // .child_right(Stretch(1.0))
            .width(Percentage(90.0))
            .height(Percentage(95.0))
        // .height(Pixels(600.0))
            .left(Percentage(5.0))
            .right(Percentage(5.0))
        // .top(Percentage(5.0))
        // .bottom(Percentage(5.0))
        // .child_top(Stretch(1.0))
        // .child_bottom(Stretch(1.0))
            ;
        ResizeHandle::new(cx);
    })
}
