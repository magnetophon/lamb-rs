/* ------------------------------------------------------------
author: "Bart Brouns"
license: "AGPLv3"
name: "lamb-rs"
version: "0.1"
Code generated with Faust 2.79.3 (https://faust.grame.fr)
Compilation options: -a /tmp/.tmpBFjopX -lang rust -ct 1 -cn LambRs48k -es 1 -mcd 16 -mdd 1024 -mdy 33 -double -ftz 0
------------------------------------------------------------ */
mod dsp_48k {
    #![allow(clippy::all)]
    #![allow(unused_parens)]
    #![allow(non_snake_case)]
    #![allow(non_camel_case_types)]
    #![allow(dead_code)]
    #![allow(unused_variables)]
    #![allow(unused_mut)]
    #![allow(non_upper_case_globals)]

    use faust_types::*;

    pub type FaustFloat = F64;

    pub struct LambRs48kSIG0 {
        iRec13: [i32; 2],
    }

    impl LambRs48kSIG0 {
        fn get_num_inputsLambRs48kSIG0(&self) -> i32 {
            return 0;
        }
        fn get_num_outputsLambRs48kSIG0(&self) -> i32 {
            return 1;
        }

        pub fn instance_initLambRs48kSIG0(&mut self, sample_rate: i32) {
            for l40 in 0..2 {
                self.iRec13[l40 as usize] = 0;
            }
        }

        pub fn fillLambRs48kSIG0(&mut self, count: i32, table: &mut [FaustFloat]) {
            for i1 in 0..count {
                self.iRec13[0] = i32::wrapping_add(self.iRec13[1], 1);
                let mut fTemp64: F64 = (self.iRec13[1] % 7) as F64 as i32 as F64;
                let mut fTemp65: F64 = 0.16666666666666666 * fTemp64;
                let mut fTemp66: F64 = F64::powf(fTemp65, 0.06999999999999999 * fTemp64 + 1.0);
                let mut fTemp67: F64 =
                    (0.14285714285714285 * (self.iRec13[1] % 917504) as F64) as i32 as F64;
                table[i1 as usize] = F64::min(
                    1.0,
                    F64::max(
                        0.0,
                        (if (fTemp65 == 0.0) as i32 != 0 {
                            0.5 * (F64::sin(2.396863267686821e-05 * fTemp67 + 4.71238898038469)
                                + 1.0)
                        } else {
                            0.5 * (F64::sin(
                                3.141592653589793
                                    * ((1.0
                                        - F64::exp(-(1.8463275629239114e-05 * fTemp66 * fTemp67)))
                                        / (1.0 - F64::exp(-(2.42 * fTemp66))))
                                    + 4.71238898038469,
                            ) + 1.0)
                        }),
                    ),
                );
                self.iRec13[1] = self.iRec13[0];
            }
        }
    }

    pub fn newLambRs48kSIG0() -> LambRs48kSIG0 {
        LambRs48kSIG0 { iRec13: [0; 2] }
    }
    fn LambRs48k_faustpower2_f(value: F64) -> F64 {
        return value * value;
    }
    static mut ftbl0LambRs48kSIG0: [F64; 917504] = [0.0; 917504];
    mod ffi {
        use std::os::raw::c_double;
        // Conditionally compile the link attribute only on non-Windows platforms
        #[cfg_attr(not(target_os = "windows"), link(name = "m"))]
        extern "C" {
            pub fn remainder(from: c_double, to: c_double) -> c_double;
            pub fn rint(val: c_double) -> c_double;
        }
    }
    fn remainder_f64(from: f64, to: f64) -> f64 {
        unsafe { ffi::remainder(from, to) }
    }
    fn rint_f64(val: f64) -> f64 {
        unsafe { ffi::rint(val) }
    }

    pub const FAUST_INPUTS: usize = 2;
    pub const FAUST_OUTPUTS: usize = 4;
    pub const FAUST_ACTIVES: usize = 15;
    pub const FAUST_PASSIVES: usize = 1;

    #[cfg_attr(feature = "default-boxed", derive(default_boxed::DefaultBoxed))]
    #[repr(C)]
    pub struct LambRs48k {
        fCheckbox0: F64,
        IOTA0: i32,
        iVec0: [i32; 8192],
        fSampleRate: i32,
        fConst0: F64,
        fConst1: F64,
        fRec0: [F64; 2],
        fHslider0: F64,
        fHslider1: F64,
        fConst2: F64,
        fConst3: F64,
        fConst4: F64,
        fHslider2: F64,
        fConst5: F64,
        fRec4: [F64; 2],
        fHslider3: F64,
        fHslider4: F64,
        fHslider5: F64,
        fRec11: [F64; 2],
        fVec1: [F64; 32768],
        fVec2: [F64; 32768],
        fVec3: [F64; 32768],
        fVec4: [F64; 32768],
        fConst6: F64,
        fHslider6: F64,
        fConst7: F64,
        fRec10: [F64; 2],
        fRec9: [F64; 2],
        fRec8: [F64; 2],
        fRec7: [F64; 2],
        fRec5: [F64; 2],
        fConst8: F64,
        fRec12: [F64; 2],
        fRec6: [F64; 2],
        fHslider7: F64,
        fHslider8: F64,
        fVec5: [F64; 16384],
        fHslider9: F64,
        fConst9: F64,
        fVec6: [F64; 3],
        fVec7: [F64; 5],
        fVec8: [F64; 12],
        fVec9: [F64; 32],
        fVec10: [F64; 64],
        fVec11: [F64; 128],
        fVec12: [F64; 256],
        fVec13: [F64; 512],
        fVec14: [F64; 1024],
        fVec15: [F64; 2048],
        fVec16: [F64; 4096],
        fRec3: [F64; 2],
        fVec17: [F64; 3],
        fVec18: [F64; 5],
        fVec19: [F64; 12],
        fVec20: [F64; 32],
        fVec21: [F64; 64],
        fVec22: [F64; 128],
        fVec23: [F64; 256],
        fVec24: [F64; 512],
        fVec25: [F64; 1024],
        fVec26: [F64; 2048],
        fVec27: [F64; 4096],
        fVec28: [F64; 2],
        fHslider10: F64,
        fHslider11: F64,
        fVec29: [F64; 2],
        fVec30: [F64; 2],
        fConst10: F64,
        fRec1: [F64; 2],
        fRec2: [F64; 2],
        fVec31: [F64; 8192],
        fCheckbox1: F64,
        fHbargraph0: F64,
        fHslider12: F64,
        fRec14: [F64; 2],
        fVec32: [F64; 16384],
        fVec33: [F64; 3],
        fVec34: [F64; 5],
        fVec35: [F64; 12],
        fVec36: [F64; 32],
        fVec37: [F64; 64],
        fVec38: [F64; 128],
        fVec39: [F64; 256],
        fVec40: [F64; 512],
        fVec41: [F64; 1024],
        fVec42: [F64; 2048],
        fVec43: [F64; 4096],
        fRec17: [F64; 2],
        fVec44: [F64; 3],
        fVec45: [F64; 5],
        fVec46: [F64; 12],
        fVec47: [F64; 32],
        fVec48: [F64; 64],
        fVec49: [F64; 128],
        fVec50: [F64; 256],
        fVec51: [F64; 512],
        fVec52: [F64; 1024],
        fVec53: [F64; 2048],
        fVec54: [F64; 4096],
        fVec55: [F64; 2],
        fVec56: [F64; 2],
        fVec57: [F64; 2],
        fRec15: [F64; 2],
        fRec16: [F64; 2],
        fVec58: [F64; 8192],
    }

    impl LambRs48k {
        pub fn new() -> LambRs48k {
            LambRs48k {
                fCheckbox0: 0.0,
                IOTA0: 0,
                iVec0: [0; 8192],
                fSampleRate: 0,
                fConst0: 0.0,
                fConst1: 0.0,
                fRec0: [0.0; 2],
                fHslider0: 0.0,
                fHslider1: 0.0,
                fConst2: 0.0,
                fConst3: 0.0,
                fConst4: 0.0,
                fHslider2: 0.0,
                fConst5: 0.0,
                fRec4: [0.0; 2],
                fHslider3: 0.0,
                fHslider4: 0.0,
                fHslider5: 0.0,
                fRec11: [0.0; 2],
                fVec1: [0.0; 32768],
                fVec2: [0.0; 32768],
                fVec3: [0.0; 32768],
                fVec4: [0.0; 32768],
                fConst6: 0.0,
                fHslider6: 0.0,
                fConst7: 0.0,
                fRec10: [0.0; 2],
                fRec9: [0.0; 2],
                fRec8: [0.0; 2],
                fRec7: [0.0; 2],
                fRec5: [0.0; 2],
                fConst8: 0.0,
                fRec12: [0.0; 2],
                fRec6: [0.0; 2],
                fHslider7: 0.0,
                fHslider8: 0.0,
                fVec5: [0.0; 16384],
                fHslider9: 0.0,
                fConst9: 0.0,
                fVec6: [0.0; 3],
                fVec7: [0.0; 5],
                fVec8: [0.0; 12],
                fVec9: [0.0; 32],
                fVec10: [0.0; 64],
                fVec11: [0.0; 128],
                fVec12: [0.0; 256],
                fVec13: [0.0; 512],
                fVec14: [0.0; 1024],
                fVec15: [0.0; 2048],
                fVec16: [0.0; 4096],
                fRec3: [0.0; 2],
                fVec17: [0.0; 3],
                fVec18: [0.0; 5],
                fVec19: [0.0; 12],
                fVec20: [0.0; 32],
                fVec21: [0.0; 64],
                fVec22: [0.0; 128],
                fVec23: [0.0; 256],
                fVec24: [0.0; 512],
                fVec25: [0.0; 1024],
                fVec26: [0.0; 2048],
                fVec27: [0.0; 4096],
                fVec28: [0.0; 2],
                fHslider10: 0.0,
                fHslider11: 0.0,
                fVec29: [0.0; 2],
                fVec30: [0.0; 2],
                fConst10: 0.0,
                fRec1: [0.0; 2],
                fRec2: [0.0; 2],
                fVec31: [0.0; 8192],
                fCheckbox1: 0.0,
                fHbargraph0: 0.0,
                fHslider12: 0.0,
                fRec14: [0.0; 2],
                fVec32: [0.0; 16384],
                fVec33: [0.0; 3],
                fVec34: [0.0; 5],
                fVec35: [0.0; 12],
                fVec36: [0.0; 32],
                fVec37: [0.0; 64],
                fVec38: [0.0; 128],
                fVec39: [0.0; 256],
                fVec40: [0.0; 512],
                fVec41: [0.0; 1024],
                fVec42: [0.0; 2048],
                fVec43: [0.0; 4096],
                fRec17: [0.0; 2],
                fVec44: [0.0; 3],
                fVec45: [0.0; 5],
                fVec46: [0.0; 12],
                fVec47: [0.0; 32],
                fVec48: [0.0; 64],
                fVec49: [0.0; 128],
                fVec50: [0.0; 256],
                fVec51: [0.0; 512],
                fVec52: [0.0; 1024],
                fVec53: [0.0; 2048],
                fVec54: [0.0; 4096],
                fVec55: [0.0; 2],
                fVec56: [0.0; 2],
                fVec57: [0.0; 2],
                fRec15: [0.0; 2],
                fRec16: [0.0; 2],
                fVec58: [0.0; 8192],
            }
        }
        pub fn metadata(&self, m: &mut dyn Meta) {
            m.declare("author", r"Bart Brouns");
            m.declare("basics.lib/name", r"Faust Basic Element Library");
            m.declare("basics.lib/parallelMax:author", r"Bart Brouns");
            m.declare(
                "basics.lib/parallelMax:copyright",
                r"Copyright (c) 2020 Bart Brouns <bart@magnetophon.nl>",
            );
            m.declare("basics.lib/parallelMax:licence", r"GPL-3.0");
            m.declare("basics.lib/parallelOp:author", r"Bart Brouns");
            m.declare(
                "basics.lib/parallelOp:copyright",
                r"Copyright (c) 2020 Bart Brouns <bart@magnetophon.nl>",
            );
            m.declare("basics.lib/parallelOp:licence", r"GPL-3.0");
            m.declare("basics.lib/tabulateNd:author", r"Bart Brouns");
            m.declare(
                "basics.lib/tabulateNd:copyright",
                r"Copyright (C) 2023 Bart Brouns <bart@magnetophon.nl>",
            );
            m.declare("basics.lib/tabulateNd:license", r"AGPL-3.0");
            m.declare("basics.lib/version", r"1.21.0");
            m.declare("compile_options", r"-a /tmp/.tmpBFjopX -lang rust -ct 1 -cn LambRs48k -es 1 -mcd 16 -mdd 1024 -mdy 33 -double -ftz 0");
            m.declare("filename", r"lamb-rs-48k.dsp");
            m.declare(
                "interpolators.lib/interpolate_linear:author",
                r"Stéphane Letz",
            );
            m.declare("interpolators.lib/interpolate_linear:licence", r"MIT");
            m.declare("interpolators.lib/name", r"Faust Interpolator Library");
            m.declare("interpolators.lib/remap:author", r"David Braun");
            m.declare("interpolators.lib/version", r"1.4.0");
            m.declare("lamb.dsp/author", r"Bart Brouns");
            m.declare("lamb.dsp/license", r"AGPLv3");
            m.declare("lamb.dsp/name", r"lamb");
            m.declare("lamb.dsp/version", r"0.1");
            m.declare("license", r"AGPLv3");
            m.declare("maths.lib/author", r"GRAME");
            m.declare("maths.lib/copyright", r"GRAME");
            m.declare("maths.lib/license", r"LGPL with exception");
            m.declare("maths.lib/name", r"Faust Math Library");
            m.declare("maths.lib/version", r"2.8.1");
            m.declare("name", r"lamb-rs");
            m.declare("platform.lib/name", r"Generic Platform Library");
            m.declare("platform.lib/version", r"1.3.0");
            m.declare("routes.lib/name", r"Faust Signal Routing Library");
            m.declare("routes.lib/version", r"1.2.0");
            m.declare("signals.lib/name", r"Faust Signal Routing Library");
            m.declare("signals.lib/version", r"1.6.0");
            m.declare("version", r"0.1");
        }

        pub fn get_sample_rate(&self) -> i32 {
            self.fSampleRate as i32
        }

        pub fn class_init(sample_rate: i32) {
            let mut sig0: LambRs48kSIG0 = newLambRs48kSIG0();
            sig0.instance_initLambRs48kSIG0(sample_rate);
            sig0.fillLambRs48kSIG0(917504, unsafe { &mut ftbl0LambRs48kSIG0 });
        }
        pub fn instance_reset_params(&mut self) {
            self.fCheckbox0 = 0.0;
            self.fHslider0 = 1e+02;
            self.fHslider1 = 9.0;
            self.fHslider2 = 5e+01;
            self.fHslider3 = 1.0;
            self.fHslider4 = -1.0;
            self.fHslider5 = 0.0;
            self.fHslider6 = 6e+01;
            self.fHslider7 = 1e+02;
            self.fHslider8 = 0.0;
            self.fHslider9 = 5e+01;
            self.fHslider10 = 0.0;
            self.fHslider11 = 0.5;
            self.fCheckbox1 = 0.0;
            self.fHslider12 = 0.0;
        }
        pub fn instance_clear(&mut self) {
            self.IOTA0 = 0;
            for l0 in 0..8192 {
                self.iVec0[l0 as usize] = 0;
            }
            for l1 in 0..2 {
                self.fRec0[l1 as usize] = 0.0;
            }
            for l2 in 0..2 {
                self.fRec4[l2 as usize] = 0.0;
            }
            for l3 in 0..2 {
                self.fRec11[l3 as usize] = 0.0;
            }
            for l4 in 0..32768 {
                self.fVec1[l4 as usize] = 0.0;
            }
            for l5 in 0..32768 {
                self.fVec2[l5 as usize] = 0.0;
            }
            for l6 in 0..32768 {
                self.fVec3[l6 as usize] = 0.0;
            }
            for l7 in 0..32768 {
                self.fVec4[l7 as usize] = 0.0;
            }
            for l8 in 0..2 {
                self.fRec10[l8 as usize] = 0.0;
            }
            for l9 in 0..2 {
                self.fRec9[l9 as usize] = 0.0;
            }
            for l10 in 0..2 {
                self.fRec8[l10 as usize] = 0.0;
            }
            for l11 in 0..2 {
                self.fRec7[l11 as usize] = 0.0;
            }
            for l12 in 0..2 {
                self.fRec5[l12 as usize] = 0.0;
            }
            for l13 in 0..2 {
                self.fRec12[l13 as usize] = 0.0;
            }
            for l14 in 0..2 {
                self.fRec6[l14 as usize] = 0.0;
            }
            for l15 in 0..16384 {
                self.fVec5[l15 as usize] = 0.0;
            }
            for l16 in 0..3 {
                self.fVec6[l16 as usize] = 0.0;
            }
            for l17 in 0..5 {
                self.fVec7[l17 as usize] = 0.0;
            }
            for l18 in 0..12 {
                self.fVec8[l18 as usize] = 0.0;
            }
            for l19 in 0..32 {
                self.fVec9[l19 as usize] = 0.0;
            }
            for l20 in 0..64 {
                self.fVec10[l20 as usize] = 0.0;
            }
            for l21 in 0..128 {
                self.fVec11[l21 as usize] = 0.0;
            }
            for l22 in 0..256 {
                self.fVec12[l22 as usize] = 0.0;
            }
            for l23 in 0..512 {
                self.fVec13[l23 as usize] = 0.0;
            }
            for l24 in 0..1024 {
                self.fVec14[l24 as usize] = 0.0;
            }
            for l25 in 0..2048 {
                self.fVec15[l25 as usize] = 0.0;
            }
            for l26 in 0..4096 {
                self.fVec16[l26 as usize] = 0.0;
            }
            for l27 in 0..2 {
                self.fRec3[l27 as usize] = 0.0;
            }
            for l28 in 0..3 {
                self.fVec17[l28 as usize] = 0.0;
            }
            for l29 in 0..5 {
                self.fVec18[l29 as usize] = 0.0;
            }
            for l30 in 0..12 {
                self.fVec19[l30 as usize] = 0.0;
            }
            for l31 in 0..32 {
                self.fVec20[l31 as usize] = 0.0;
            }
            for l32 in 0..64 {
                self.fVec21[l32 as usize] = 0.0;
            }
            for l33 in 0..128 {
                self.fVec22[l33 as usize] = 0.0;
            }
            for l34 in 0..256 {
                self.fVec23[l34 as usize] = 0.0;
            }
            for l35 in 0..512 {
                self.fVec24[l35 as usize] = 0.0;
            }
            for l36 in 0..1024 {
                self.fVec25[l36 as usize] = 0.0;
            }
            for l37 in 0..2048 {
                self.fVec26[l37 as usize] = 0.0;
            }
            for l38 in 0..4096 {
                self.fVec27[l38 as usize] = 0.0;
            }
            for l39 in 0..2 {
                self.fVec28[l39 as usize] = 0.0;
            }
            for l41 in 0..2 {
                self.fVec29[l41 as usize] = 0.0;
            }
            for l42 in 0..2 {
                self.fVec30[l42 as usize] = 0.0;
            }
            for l43 in 0..2 {
                self.fRec1[l43 as usize] = 0.0;
            }
            for l44 in 0..2 {
                self.fRec2[l44 as usize] = 0.0;
            }
            for l45 in 0..8192 {
                self.fVec31[l45 as usize] = 0.0;
            }
            for l46 in 0..2 {
                self.fRec14[l46 as usize] = 0.0;
            }
            for l47 in 0..16384 {
                self.fVec32[l47 as usize] = 0.0;
            }
            for l48 in 0..3 {
                self.fVec33[l48 as usize] = 0.0;
            }
            for l49 in 0..5 {
                self.fVec34[l49 as usize] = 0.0;
            }
            for l50 in 0..12 {
                self.fVec35[l50 as usize] = 0.0;
            }
            for l51 in 0..32 {
                self.fVec36[l51 as usize] = 0.0;
            }
            for l52 in 0..64 {
                self.fVec37[l52 as usize] = 0.0;
            }
            for l53 in 0..128 {
                self.fVec38[l53 as usize] = 0.0;
            }
            for l54 in 0..256 {
                self.fVec39[l54 as usize] = 0.0;
            }
            for l55 in 0..512 {
                self.fVec40[l55 as usize] = 0.0;
            }
            for l56 in 0..1024 {
                self.fVec41[l56 as usize] = 0.0;
            }
            for l57 in 0..2048 {
                self.fVec42[l57 as usize] = 0.0;
            }
            for l58 in 0..4096 {
                self.fVec43[l58 as usize] = 0.0;
            }
            for l59 in 0..2 {
                self.fRec17[l59 as usize] = 0.0;
            }
            for l60 in 0..3 {
                self.fVec44[l60 as usize] = 0.0;
            }
            for l61 in 0..5 {
                self.fVec45[l61 as usize] = 0.0;
            }
            for l62 in 0..12 {
                self.fVec46[l62 as usize] = 0.0;
            }
            for l63 in 0..32 {
                self.fVec47[l63 as usize] = 0.0;
            }
            for l64 in 0..64 {
                self.fVec48[l64 as usize] = 0.0;
            }
            for l65 in 0..128 {
                self.fVec49[l65 as usize] = 0.0;
            }
            for l66 in 0..256 {
                self.fVec50[l66 as usize] = 0.0;
            }
            for l67 in 0..512 {
                self.fVec51[l67 as usize] = 0.0;
            }
            for l68 in 0..1024 {
                self.fVec52[l68 as usize] = 0.0;
            }
            for l69 in 0..2048 {
                self.fVec53[l69 as usize] = 0.0;
            }
            for l70 in 0..4096 {
                self.fVec54[l70 as usize] = 0.0;
            }
            for l71 in 0..2 {
                self.fVec55[l71 as usize] = 0.0;
            }
            for l72 in 0..2 {
                self.fVec56[l72 as usize] = 0.0;
            }
            for l73 in 0..2 {
                self.fVec57[l73 as usize] = 0.0;
            }
            for l74 in 0..2 {
                self.fRec15[l74 as usize] = 0.0;
            }
            for l75 in 0..2 {
                self.fRec16[l75 as usize] = 0.0;
            }
            for l76 in 0..8192 {
                self.fVec58[l76 as usize] = 0.0;
            }
        }
        pub fn instance_constants(&mut self, sample_rate: i32) {
            self.fSampleRate = sample_rate;
            self.fConst0 = F64::min(1.92e+05, F64::max(1.0, (self.fSampleRate) as F64));
            self.fConst1 = 1e+02 / self.fConst0;
            self.fConst2 = 1e-05 * self.fConst0;
            self.fConst3 = 44.1 / self.fConst0;
            self.fConst4 = 1.0 - self.fConst3;
            self.fConst5 = 0.441 / self.fConst0;
            self.fConst6 = F64::exp(-(6.505353649590627e+16 / self.fConst0));
            self.fConst7 = 6.283185307179586 / self.fConst0;
            self.fConst8 = F64::exp(-(2.829695100811376e+16 / self.fConst0));
            self.fConst9 = 0.001 * self.fConst0;
            self.fConst10 = 1e+03 / self.fConst0;
        }
        pub fn instance_init(&mut self, sample_rate: i32) {
            self.instance_constants(sample_rate);
            self.instance_reset_params();
            self.instance_clear();
        }
        pub fn init(&mut self, sample_rate: i32) {
            LambRs48k::class_init(sample_rate);
            self.instance_init(sample_rate);
        }

        pub fn build_user_interface(&self, ui_interface: &mut dyn UI<FaustFloat>) {
            Self::build_user_interface_static(ui_interface);
        }

        pub fn build_user_interface_static(ui_interface: &mut dyn UI<FaustFloat>) {
            ui_interface.open_vertical_box("lamb-rs");
            ui_interface.open_horizontal_box("0x00");
            ui_interface.declare(Some(ParamIndex(0)), "00", "");
            ui_interface.add_check_button("bypass", ParamIndex(0));
            ui_interface.declare(Some(ParamIndex(1)), "01", "");
            ui_interface.add_check_button("fixed latency", ParamIndex(1));
            ui_interface.close_box();
            ui_interface.declare(Some(ParamIndex(2)), "01", "");
            ui_interface.add_horizontal_slider("input gain", ParamIndex(2), 0.0, -24.0, 24.0, 0.1);
            ui_interface.declare(None, "02", "");
            ui_interface.open_horizontal_box("0x00");
            ui_interface.declare(None, "2", "");
            ui_interface.open_vertical_box("peak limiter");
            ui_interface.declare(Some(ParamIndex(3)), "02", "");
            ui_interface.add_horizontal_slider("strength", ParamIndex(3), 1e+02, 0.0, 1e+02, 1.0);
            ui_interface.declare(Some(ParamIndex(4)), "03", "");
            ui_interface.add_horizontal_slider("thresh", ParamIndex(4), -1.0, -3e+01, 0.0, 0.1);
            ui_interface.declare(Some(ParamIndex(5)), "04", "");
            ui_interface.declare(Some(ParamIndex(5)), "scale", "log");
            ui_interface.declare(Some(ParamIndex(5)), "unit", "ms");
            ui_interface.add_horizontal_slider("attack", ParamIndex(5), 9.0, 0.0, 5e+01, 0.1);
            ui_interface.declare(Some(ParamIndex(6)), "05", "");
            ui_interface.add_horizontal_slider("attack shape", ParamIndex(6), 0.0, 0.0, 1.0, 0.01);
            ui_interface.declare(Some(ParamIndex(7)), "06", "");
            ui_interface.declare(Some(ParamIndex(7)), "scale", "log");
            ui_interface.declare(Some(ParamIndex(7)), "unit", "ms");
            ui_interface.add_horizontal_slider("release", ParamIndex(7), 6e+01, 1.0, 5e+02, 1.0);
            ui_interface.declare(Some(ParamIndex(8)), "07", "");
            ui_interface.add_horizontal_slider("release shape", ParamIndex(8), 0.5, 0.0, 1.0, 0.01);
            ui_interface.declare(Some(ParamIndex(9)), "08", "");
            ui_interface.declare(Some(ParamIndex(9)), "unit", "ms");
            ui_interface.add_horizontal_slider(
                "release hold",
                ParamIndex(9),
                5e+01,
                0.020833333333333332,
                5e+01,
                1.0,
            );
            ui_interface.declare(Some(ParamIndex(10)), "09", "");
            ui_interface.add_horizontal_slider("knee", ParamIndex(10), 1.0, 0.0, 3e+01, 0.1);
            ui_interface.declare(Some(ParamIndex(11)), "10", "");
            ui_interface.add_horizontal_slider("link", ParamIndex(11), 0.0, 0.0, 1e+02, 1.0);
            ui_interface.declare(Some(ParamIndex(12)), "11", "");
            ui_interface.add_horizontal_slider(
                "adaptive release",
                ParamIndex(12),
                5e+01,
                0.0,
                1e+02,
                1.0,
            );
            ui_interface.declare(Some(ParamIndex(13)), "12", "");
            ui_interface.add_horizontal_slider("lookahead", ParamIndex(13), 1e+02, 0.0, 1e+02, 1.0);
            ui_interface.close_box();
            ui_interface.close_box();
            ui_interface.declare(Some(ParamIndex(14)), "13", "");
            ui_interface.add_horizontal_slider(
                "output gain",
                ParamIndex(14),
                0.0,
                -24.0,
                24.0,
                0.1,
            );
            ui_interface.declare(Some(ParamIndex(15)), "99", "");
            ui_interface.declare(Some(ParamIndex(15)), "unit", "samples");
            ui_interface.add_horizontal_bargraph("latency", ParamIndex(15), 0.0, 4.8e+03);
            ui_interface.close_box();
        }

        pub fn get_param(&self, param: ParamIndex) -> Option<FaustFloat> {
            match param.0 {
                0 => Some(self.fCheckbox0),
                1 => Some(self.fCheckbox1),
                15 => Some(self.fHbargraph0),
                13 => Some(self.fHslider0),
                5 => Some(self.fHslider1),
                6 => Some(self.fHslider10),
                8 => Some(self.fHslider11),
                14 => Some(self.fHslider12),
                12 => Some(self.fHslider2),
                10 => Some(self.fHslider3),
                4 => Some(self.fHslider4),
                2 => Some(self.fHslider5),
                7 => Some(self.fHslider6),
                3 => Some(self.fHslider7),
                11 => Some(self.fHslider8),
                9 => Some(self.fHslider9),
                _ => None,
            }
        }

        pub fn set_param(&mut self, param: ParamIndex, value: FaustFloat) {
            match param.0 {
                0 => self.fCheckbox0 = value,
                1 => self.fCheckbox1 = value,
                15 => self.fHbargraph0 = value,
                13 => self.fHslider0 = value,
                5 => self.fHslider1 = value,
                6 => self.fHslider10 = value,
                8 => self.fHslider11 = value,
                14 => self.fHslider12 = value,
                12 => self.fHslider2 = value,
                10 => self.fHslider3 = value,
                4 => self.fHslider4 = value,
                2 => self.fHslider5 = value,
                7 => self.fHslider6 = value,
                3 => self.fHslider7 = value,
                11 => self.fHslider8 = value,
                9 => self.fHslider9 = value,
                _ => {}
            }
        }

        pub fn compute(
            &mut self,
            count: usize,
            inputs: &[impl AsRef<[FaustFloat]>],
            outputs: &mut [impl AsMut<[FaustFloat]>],
        ) {
            let [inputs0, inputs1, ..] = inputs.as_ref() else {
                panic!("wrong number of input buffers");
            };
            let inputs0 = inputs0.as_ref()[..count].iter();
            let inputs1 = inputs1.as_ref()[..count].iter();
            let [outputs0, outputs1, outputs2, outputs3, ..] = outputs.as_mut() else {
                panic!("wrong number of output buffers");
            };
            let outputs0 = outputs0.as_mut()[..count].iter_mut();
            let outputs1 = outputs1.as_mut()[..count].iter_mut();
            let outputs2 = outputs2.as_mut()[..count].iter_mut();
            let outputs3 = outputs3.as_mut()[..count].iter_mut();
            let mut fSlow0: F64 = self.fCheckbox0;
            let mut fSlow1: F64 = self.fHslider1;
            let mut fSlow2: F64 = fSlow1 * self.fHslider0;
            let mut fSlow3: F64 = self.fConst2 * fSlow2 + 1.0;
            let mut iSlow4: i32 = (F64::floor(fSlow3)) as i32 % 2;
            let mut fSlow5: F64 = self.fConst5 * self.fHslider2;
            let mut fSlow6: F64 = self.fHslider3;
            let mut fSlow7: F64 = 0.5 * fSlow6;
            let mut fSlow8: F64 = self.fHslider4;
            let mut fSlow9: F64 = fSlow8 + fSlow7;
            let mut fSlow10: F64 = self.fConst3 * F64::powf(1e+01, 0.05 * self.fHslider5);
            let mut fSlow11: F64 = fSlow8 - fSlow7;
            let mut fSlow12: F64 = 0.5 / F64::max(2.220446049250313e-16, fSlow6);
            let mut fSlow13: F64 = self.fHslider6;
            let mut fSlow14: F64 = 0.001 * fSlow13;
            let mut fSlow15: F64 = self.fHslider7;
            let mut fSlow16: F64 = 0.04 * fSlow15;
            let mut fSlow17: F64 = 0.01 * self.fHslider8;
            let mut fSlow18: F64 = 0.01 * fSlow15;
            let mut fSlow19: F64 = self.fHslider9;
            let mut fSlow20: F64 = self.fConst9 * fSlow19;
            let mut iSlow21: i32 = (fSlow20) as i32;
            let mut fSlow22: F64 = fSlow20 + 1.0;
            let mut iSlow23: i32 = (F64::floor(fSlow22)) as i32 % 2;
            let mut iSlow24: i32 = (F64::floor(0.5 * fSlow22)) as i32 % 2;
            let mut iSlow25: i32 = (F64::floor(0.25 * fSlow22)) as i32 % 2;
            let mut iSlow26: i32 = i32::wrapping_add(iSlow23, i32::wrapping_mul(2, iSlow24));
            let mut iSlow27: i32 = (F64::floor(0.125 * fSlow22)) as i32 % 2;
            let mut iSlow28: i32 = i32::wrapping_add(iSlow26, i32::wrapping_mul(4, iSlow25));
            let mut iSlow29: i32 = (F64::floor(0.0625 * fSlow22)) as i32 % 2;
            let mut iSlow30: i32 = i32::wrapping_add(iSlow28, i32::wrapping_mul(8, iSlow27));
            let mut iSlow31: i32 = (F64::floor(0.03125 * fSlow22)) as i32 % 2;
            let mut iSlow32: i32 = i32::wrapping_add(iSlow30, i32::wrapping_mul(16, iSlow29));
            let mut iSlow33: i32 = (F64::floor(0.015625 * fSlow22)) as i32 % 2;
            let mut iSlow34: i32 = i32::wrapping_add(iSlow32, i32::wrapping_mul(32, iSlow31));
            let mut iSlow35: i32 = (F64::floor(0.0078125 * fSlow22)) as i32 % 2;
            let mut iSlow36: i32 = i32::wrapping_add(iSlow34, i32::wrapping_mul(64, iSlow33));
            let mut iSlow37: i32 = (F64::floor(0.00390625 * fSlow22)) as i32 % 2;
            let mut iSlow38: i32 = i32::wrapping_add(iSlow36, i32::wrapping_mul(128, iSlow35));
            let mut iSlow39: i32 = (F64::floor(0.001953125 * fSlow22)) as i32 % 2;
            let mut iSlow40: i32 = i32::wrapping_add(iSlow38, i32::wrapping_mul(256, iSlow37));
            let mut iSlow41: i32 = (F64::floor(0.0009765625 * fSlow22)) as i32 % 2;
            let mut iSlow42: i32 = i32::wrapping_add(iSlow40, i32::wrapping_mul(512, iSlow39));
            let mut iSlow43: i32 = (F64::floor(0.00048828125 * fSlow22)) as i32 % 2;
            let mut iSlow44: i32 = i32::wrapping_add(iSlow42, i32::wrapping_mul(1024, iSlow41));
            let mut iSlow45: i32 = (F64::floor(0.5 * fSlow3)) as i32 % 2;
            let mut iSlow46: i32 = (F64::floor(0.25 * fSlow3)) as i32 % 2;
            let mut iSlow47: i32 = i32::wrapping_add(iSlow4, i32::wrapping_mul(2, iSlow45));
            let mut iSlow48: i32 = (F64::floor(0.125 * fSlow3)) as i32 % 2;
            let mut iSlow49: i32 = i32::wrapping_add(iSlow47, i32::wrapping_mul(4, iSlow46));
            let mut iSlow50: i32 = (F64::floor(0.0625 * fSlow3)) as i32 % 2;
            let mut iSlow51: i32 = i32::wrapping_add(iSlow49, i32::wrapping_mul(8, iSlow48));
            let mut iSlow52: i32 = (F64::floor(0.03125 * fSlow3)) as i32 % 2;
            let mut iSlow53: i32 = i32::wrapping_add(iSlow51, i32::wrapping_mul(16, iSlow50));
            let mut iSlow54: i32 = (F64::floor(0.015625 * fSlow3)) as i32 % 2;
            let mut iSlow55: i32 = i32::wrapping_add(iSlow53, i32::wrapping_mul(32, iSlow52));
            let mut iSlow56: i32 = (F64::floor(0.0078125 * fSlow3)) as i32 % 2;
            let mut iSlow57: i32 = i32::wrapping_add(iSlow55, i32::wrapping_mul(64, iSlow54));
            let mut iSlow58: i32 = (F64::floor(0.00390625 * fSlow3)) as i32 % 2;
            let mut iSlow59: i32 = i32::wrapping_add(iSlow57, i32::wrapping_mul(128, iSlow56));
            let mut iSlow60: i32 = (F64::floor(0.001953125 * fSlow3)) as i32 % 2;
            let mut iSlow61: i32 = i32::wrapping_add(iSlow59, i32::wrapping_mul(256, iSlow58));
            let mut iSlow62: i32 = (F64::floor(0.0009765625 * fSlow3)) as i32 % 2;
            let mut iSlow63: i32 = i32::wrapping_add(iSlow61, i32::wrapping_mul(512, iSlow60));
            let mut iSlow64: i32 = (F64::floor(0.00048828125 * fSlow3)) as i32 % 2;
            let mut iSlow65: i32 = i32::wrapping_add(iSlow63, i32::wrapping_mul(1024, iSlow62));
            let mut fSlow66: F64 = self.fHslider10;
            let mut fSlow67: F64 = self.fHslider11;
            let mut fSlow68: F64 = self.fConst0 * (0.001 * fSlow19 + 1e-05 * fSlow2);
            let mut fSlow69: F64 = self.fCheckbox1;
            let mut iSlow70: i32 = (F64::max(0.0, fSlow69 * (4.8e+03 - fSlow68))) as i32;
            self.fHbargraph0 = (if (fSlow69) as i32 != 0 {
                4.8e+03
            } else {
                fSlow68
            });
            let mut iSlow71: i32 = (self.fHbargraph0) as i32;
            let mut fSlow72: F64 = self.fConst3 * F64::powf(1e+01, 0.05 * self.fHslider12);
            let zipped_iterators = inputs0
                .zip(inputs1)
                .zip(outputs0)
                .zip(outputs1)
                .zip(outputs2)
                .zip(outputs3);
            for (((((input0, input1), output0), output1), output2), output3) in zipped_iterators {
                self.iVec0[(self.IOTA0 & 8191) as usize] = 1;
                let mut fTemp0: F64 = self.fConst1 + self.fRec0[1];
                let mut fTemp1: F64 = self.fRec0[1] - self.fConst1;
                self.fRec0[0] = (if (fTemp0 < fSlow0) as i32 != 0 {
                    fTemp0
                } else {
                    (if (fTemp1 > fSlow0) as i32 != 0 {
                        fTemp1
                    } else {
                        fSlow0
                    })
                });
                let mut fTemp2: F64 =
                    F64::sin(6.283185307179586 * (0.5 * self.fRec0[0] + 0.75)) + 1.0;
                let mut fTemp3: F64 = 0.5 * fTemp2;
                let mut fTemp4: F64 = 1.0 - fTemp3;
                self.fRec4[0] = fSlow5 + self.fConst4 * self.fRec4[1];
                let mut fTemp5: F64 = F64::max(0.5, self.fRec4[0]) + -0.5;
                let mut fTemp6: F64 = 4.0 * fTemp5;
                let mut fTemp7: F64 = 10.588235294117647 * (F64::max(0.15, self.fRec4[0]) + -0.15);
                let mut fTemp8: F64 = 15.0 - (fTemp7 + fTemp6);
                let mut fTemp9: F64 = 12.0 - fTemp7;
                let mut fTemp10: F64 = fTemp7 + -12.0;
                let mut fTemp11: F64 = 3.0 - fTemp6;
                self.fRec11[0] = fSlow10 + self.fConst4 * self.fRec11[1];
                let mut fTemp12: F64 = *input0;
                self.fVec1[(self.IOTA0 & 32767) as usize] = fTemp12;
                let mut fTemp13: F64 = fTemp12 * self.fRec11[0];
                self.fVec2[(self.IOTA0 & 32767) as usize] = fTemp13;
                let mut fTemp14: F64 = F64::abs(fTemp13);
                let mut fTemp15: F64 = *input1;
                self.fVec3[(self.IOTA0 & 32767) as usize] = fTemp15;
                let mut fTemp16: F64 = fTemp15 * self.fRec11[0];
                self.fVec4[(self.IOTA0 & 32767) as usize] = fTemp16;
                let mut fTemp17: F64 = F64::abs(fTemp16);
                let mut fTemp18: F64 = 2e+01
                    * F64::log10(F64::max(
                        2.2250738585072014e-308,
                        F64::max(fTemp14, fTemp17),
                    ));
                let mut iTemp19: i32 = ((fTemp18 > fSlow11) as i32) + ((fTemp18 > fSlow9) as i32);
                let mut fTemp20: F64 = fTemp18 - fSlow8;
                let mut fTemp21: F64 = F64::powf(
                    1e+01,
                    -(0.05
                        * F64::max(
                            0.0,
                            (if (iTemp19 == 0) as i32 != 0 {
                                0.0
                            } else {
                                (if (iTemp19 == 1) as i32 != 0 {
                                    fSlow12 * LambRs48k_faustpower2_f(fSlow7 + fTemp20)
                                } else {
                                    fTemp20
                                })
                            }),
                        )),
                );
                let mut fTemp22: F64 = 3.0 * fTemp5;
                let mut fTemp23: F64 = 4.0 * (F64::max(0.25, self.fRec4[0]) + -0.25);
                let mut fTemp24: F64 = 9.0 - fTemp23;
                let mut fTemp25: F64 = self.fRec5[1] - self.fRec6[1];
                let mut fTemp26: F64 =
                    (self.iVec0[((i32::wrapping_sub(self.IOTA0, 4800)) & 8191) as usize]) as F64;
                let mut fTemp27: F64 = (if (fTemp21 > self.fRec10[1]) as i32 != 0 {
                    F64::exp(
                        -(self.fConst7
                            / F64::max(
                                2.220446049250313e-16,
                                fSlow14
                                    * (fTemp26
                                        / F64::max(
                                            1.0 - (F64::max(
                                                fTemp23 + -9.0,
                                                F64::min(2.0 - fTemp22, fTemp25),
                                            ) + fTemp24)
                                                / (11.0 - (fTemp23 + fTemp22)),
                                            2.220446049250313e-16,
                                        )),
                            )),
                    )
                } else {
                    self.fConst6
                });
                self.fRec10[0] = self.fRec10[1] * fTemp27 + fTemp21 * (1.0 - fTemp27);
                let mut fTemp28: F64 = (if (self.fRec10[0] > self.fRec9[1]) as i32 != 0 {
                    0.0
                } else {
                    self.fConst6
                });
                self.fRec9[0] = self.fRec9[1] * fTemp28 + self.fRec10[0] * (1.0 - fTemp28);
                let mut fTemp29: F64 = (if (self.fRec9[0] > self.fRec8[1]) as i32 != 0 {
                    0.0
                } else {
                    self.fConst6
                });
                self.fRec8[0] = self.fRec8[1] * fTemp29 + self.fRec9[0] * (1.0 - fTemp29);
                let mut fTemp30: F64 = (if (self.fRec8[0] > self.fRec7[1]) as i32 != 0 {
                    0.0
                } else {
                    self.fConst6
                });
                self.fRec7[0] = self.fRec7[1] * fTemp30 + self.fRec8[0] * (1.0 - fTemp30);
                self.fRec5[0] =
                    2e+01 * F64::log10(F64::max(2.2250738585072014e-308, self.fRec7[0]));
                let mut fTemp31: F64 = F64::powf(1e+01, 0.05 * (self.fRec5[1] + fTemp24));
                let mut fTemp32: F64 = (if (fTemp31 > self.fRec12[1]) as i32 != 0 {
                    F64::exp(
                        -(self.fConst7
                            / F64::max(
                                2.220446049250313e-16,
                                fTemp26
                                    * (0.8161290322580644
                                        * (F64::max(0.69, self.fRec4[0]) + -0.69)
                                        + 0.05)
                                    * F64::powf(
                                        4503599627370496.0,
                                        1.0 - (F64::max(fTemp10, F64::min(fTemp11, fTemp25))
                                            + fTemp9)
                                            / fTemp8,
                                    ),
                            )),
                    )
                } else {
                    self.fConst8
                });
                self.fRec12[0] = self.fRec12[1] * fTemp32 + fTemp31 * (1.0 - fTemp32);
                self.fRec6[0] =
                    2e+01 * F64::log10(F64::max(2.2250738585072014e-308, self.fRec12[0]));
                let mut fTemp33: F64 = self.fRec5[0] - self.fRec6[0];
                let mut fTemp34: F64 = fSlow16
                    * F64::min(0.25, self.fRec4[0])
                    * (self.fRec6[0]
                        + fTemp33 * (F64::max(fTemp10, F64::min(fTemp11, fTemp33)) + fTemp9)
                            / fTemp8);
                let mut fTemp35: F64 =
                    2e+01 * F64::log10(F64::max(2.2250738585072014e-308, fTemp14));
                let mut fTemp36: F64 =
                    2e+01 * F64::log10(F64::max(2.2250738585072014e-308, fTemp17));
                let mut fTemp37: F64 = F64::max(fTemp35, fTemp36);
                let mut fTemp38: F64 = fTemp35 + fSlow17 * (fTemp37 - fTemp35);
                let mut iTemp39: i32 = ((fTemp38 > fSlow11) as i32) + ((fTemp38 > fSlow9) as i32);
                let mut fTemp40: F64 = fTemp38 - fSlow8;
                let mut fTemp41: F64 = F64::min(
                    fTemp34,
                    -(fSlow18
                        * F64::max(
                            0.0,
                            (if (iTemp39 == 0) as i32 != 0 {
                                0.0
                            } else {
                                (if (iTemp39 == 1) as i32 != 0 {
                                    fSlow12 * LambRs48k_faustpower2_f(fSlow7 + fTemp40)
                                } else {
                                    fTemp40
                                })
                            }),
                        )),
                );
                self.fVec5[(self.IOTA0 & 16383) as usize] = fTemp41;
                let mut fTemp42: F64 = F64::min(
                    fTemp41,
                    self.fVec5[((i32::wrapping_sub(self.IOTA0, 1)) & 16383) as usize],
                );
                self.fVec6[0] = fTemp42;
                let mut fTemp43: F64 = F64::min(fTemp42, self.fVec6[2]);
                self.fVec7[0] = fTemp43;
                let mut fTemp44: F64 = F64::min(fTemp43, self.fVec7[4]);
                self.fVec8[0] = fTemp44;
                let mut fTemp45: F64 = F64::min(fTemp44, self.fVec8[8]);
                self.fVec9[(self.IOTA0 & 31) as usize] = fTemp45;
                let mut fTemp46: F64 = F64::min(
                    fTemp45,
                    self.fVec9[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize],
                );
                self.fVec10[(self.IOTA0 & 63) as usize] = fTemp46;
                let mut fTemp47: F64 = F64::min(
                    fTemp46,
                    self.fVec10[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize],
                );
                self.fVec11[(self.IOTA0 & 127) as usize] = fTemp47;
                let mut fTemp48: F64 = F64::min(
                    fTemp47,
                    self.fVec11[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize],
                );
                self.fVec12[(self.IOTA0 & 255) as usize] = fTemp48;
                let mut fTemp49: F64 = F64::min(
                    fTemp48,
                    self.fVec12[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize],
                );
                self.fVec13[(self.IOTA0 & 511) as usize] = fTemp49;
                let mut fTemp50: F64 = F64::min(
                    fTemp49,
                    self.fVec13[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize],
                );
                self.fVec14[(self.IOTA0 & 1023) as usize] = fTemp50;
                let mut fTemp51: F64 = F64::min(
                    fTemp50,
                    self.fVec14[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize],
                );
                self.fVec15[(self.IOTA0 & 2047) as usize] = fTemp51;
                self.fVec16[(self.IOTA0 & 4095) as usize] = F64::min(
                    fTemp51,
                    self.fVec15[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize],
                );
                self.fRec3[0] = F64::max(
                    F64::min(
                        self.fRec3[1],
                        self.fVec5[((i32::wrapping_sub(self.IOTA0, iSlow21)) & 16383) as usize],
                    ),
                    F64::min(
                        F64::min(
                            F64::min(
                                F64::min(
                                    F64::min(
                                        F64::min(
                                            F64::min(
                                                F64::min(
                                                    F64::min(
                                                        F64::min(
                                                            F64::min(
                                                                (if iSlow23 != 0 {
                                                                    fTemp41
                                                                } else {
                                                                    1.7976931348623157e+308
                                                                }),
                                                                (if iSlow24 != 0 {
                                                                    self.fVec6[iSlow23 as usize]
                                                                } else {
                                                                    1.7976931348623157e+308
                                                                }),
                                                            ),
                                                            (if iSlow25 != 0 {
                                                                self.fVec7[iSlow26 as usize]
                                                            } else {
                                                                1.7976931348623157e+308
                                                            }),
                                                        ),
                                                        (if iSlow27 != 0 {
                                                            self.fVec8[iSlow28 as usize]
                                                        } else {
                                                            1.7976931348623157e+308
                                                        }),
                                                    ),
                                                    (if iSlow29 != 0 {
                                                        self.fVec9[((i32::wrapping_sub(
                                                            self.IOTA0, iSlow30,
                                                        )) & 31)
                                                            as usize]
                                                    } else {
                                                        1.7976931348623157e+308
                                                    }),
                                                ),
                                                (if iSlow31 != 0 {
                                                    self.fVec10[((i32::wrapping_sub(
                                                        self.IOTA0, iSlow32,
                                                    )) & 63)
                                                        as usize]
                                                } else {
                                                    1.7976931348623157e+308
                                                }),
                                            ),
                                            (if iSlow33 != 0 {
                                                self.fVec11[((i32::wrapping_sub(
                                                    self.IOTA0, iSlow34,
                                                )) & 127)
                                                    as usize]
                                            } else {
                                                1.7976931348623157e+308
                                            }),
                                        ),
                                        (if iSlow35 != 0 {
                                            self.fVec12[((i32::wrapping_sub(self.IOTA0, iSlow36))
                                                & 255)
                                                as usize]
                                        } else {
                                            1.7976931348623157e+308
                                        }),
                                    ),
                                    (if iSlow37 != 0 {
                                        self.fVec13[((i32::wrapping_sub(self.IOTA0, iSlow38)) & 511)
                                            as usize]
                                    } else {
                                        1.7976931348623157e+308
                                    }),
                                ),
                                (if iSlow39 != 0 {
                                    self.fVec14
                                        [((i32::wrapping_sub(self.IOTA0, iSlow40)) & 1023) as usize]
                                } else {
                                    1.7976931348623157e+308
                                }),
                            ),
                            (if iSlow41 != 0 {
                                self.fVec15
                                    [((i32::wrapping_sub(self.IOTA0, iSlow42)) & 2047) as usize]
                            } else {
                                1.7976931348623157e+308
                            }),
                        ),
                        (if iSlow43 != 0 {
                            self.fVec16[((i32::wrapping_sub(self.IOTA0, iSlow44)) & 4095) as usize]
                        } else {
                            1.7976931348623157e+308
                        }),
                    ),
                );
                let mut fTemp52: F64 = F64::min(self.fRec3[0], self.fRec3[1]);
                self.fVec17[0] = fTemp52;
                let mut fTemp53: F64 = F64::min(fTemp52, self.fVec17[2]);
                self.fVec18[0] = fTemp53;
                let mut fTemp54: F64 = F64::min(fTemp53, self.fVec18[4]);
                self.fVec19[0] = fTemp54;
                let mut fTemp55: F64 = F64::min(fTemp54, self.fVec19[8]);
                self.fVec20[(self.IOTA0 & 31) as usize] = fTemp55;
                let mut fTemp56: F64 = F64::min(
                    fTemp55,
                    self.fVec20[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize],
                );
                self.fVec21[(self.IOTA0 & 63) as usize] = fTemp56;
                let mut fTemp57: F64 = F64::min(
                    fTemp56,
                    self.fVec21[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize],
                );
                self.fVec22[(self.IOTA0 & 127) as usize] = fTemp57;
                let mut fTemp58: F64 = F64::min(
                    fTemp57,
                    self.fVec22[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize],
                );
                self.fVec23[(self.IOTA0 & 255) as usize] = fTemp58;
                let mut fTemp59: F64 = F64::min(
                    fTemp58,
                    self.fVec23[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize],
                );
                self.fVec24[(self.IOTA0 & 511) as usize] = fTemp59;
                let mut fTemp60: F64 = F64::min(
                    fTemp59,
                    self.fVec24[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize],
                );
                self.fVec25[(self.IOTA0 & 1023) as usize] = fTemp60;
                let mut fTemp61: F64 = F64::min(
                    fTemp60,
                    self.fVec25[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize],
                );
                self.fVec26[(self.IOTA0 & 2047) as usize] = fTemp61;
                self.fVec27[(self.IOTA0 & 4095) as usize] = F64::min(
                    fTemp61,
                    self.fVec26[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize],
                );
                let mut fTemp62: F64 = F64::min(
                    F64::min(
                        F64::min(
                            F64::min(
                                F64::min(
                                    F64::min(
                                        F64::min(
                                            F64::min(
                                                F64::min(
                                                    F64::min(
                                                        F64::min(
                                                            (if iSlow4 != 0 {
                                                                self.fRec3[0]
                                                            } else {
                                                                1.7976931348623157e+308
                                                            }),
                                                            (if iSlow45 != 0 {
                                                                self.fVec17[iSlow4 as usize]
                                                            } else {
                                                                1.7976931348623157e+308
                                                            }),
                                                        ),
                                                        (if iSlow46 != 0 {
                                                            self.fVec18[iSlow47 as usize]
                                                        } else {
                                                            1.7976931348623157e+308
                                                        }),
                                                    ),
                                                    (if iSlow48 != 0 {
                                                        self.fVec19[iSlow49 as usize]
                                                    } else {
                                                        1.7976931348623157e+308
                                                    }),
                                                ),
                                                (if iSlow50 != 0 {
                                                    self.fVec20[((i32::wrapping_sub(
                                                        self.IOTA0, iSlow51,
                                                    )) & 31)
                                                        as usize]
                                                } else {
                                                    1.7976931348623157e+308
                                                }),
                                            ),
                                            (if iSlow52 != 0 {
                                                self.fVec21[((i32::wrapping_sub(
                                                    self.IOTA0, iSlow53,
                                                )) & 63)
                                                    as usize]
                                            } else {
                                                1.7976931348623157e+308
                                            }),
                                        ),
                                        (if iSlow54 != 0 {
                                            self.fVec22[((i32::wrapping_sub(self.IOTA0, iSlow55))
                                                & 127)
                                                as usize]
                                        } else {
                                            1.7976931348623157e+308
                                        }),
                                    ),
                                    (if iSlow56 != 0 {
                                        self.fVec23[((i32::wrapping_sub(self.IOTA0, iSlow57)) & 255)
                                            as usize]
                                    } else {
                                        1.7976931348623157e+308
                                    }),
                                ),
                                (if iSlow58 != 0 {
                                    self.fVec24
                                        [((i32::wrapping_sub(self.IOTA0, iSlow59)) & 511) as usize]
                                } else {
                                    1.7976931348623157e+308
                                }),
                            ),
                            (if iSlow60 != 0 {
                                self.fVec25
                                    [((i32::wrapping_sub(self.IOTA0, iSlow61)) & 1023) as usize]
                            } else {
                                1.7976931348623157e+308
                            }),
                        ),
                        (if iSlow62 != 0 {
                            self.fVec26[((i32::wrapping_sub(self.IOTA0, iSlow63)) & 2047) as usize]
                        } else {
                            1.7976931348623157e+308
                        }),
                    ),
                    (if iSlow64 != 0 {
                        self.fVec27[((i32::wrapping_sub(self.IOTA0, iSlow65)) & 4095) as usize]
                    } else {
                        1.7976931348623157e+308
                    }),
                ) - self.fRec2[1];
                self.fVec28[0] = fTemp62;
                let mut iTemp63: i32 = (fTemp62 > 0.0) as i32;
                let mut fTemp68: F64 = (if iTemp63 != 0 { fSlow67 } else { fSlow66 });
                self.fVec29[0] = fTemp68;
                let mut fTemp69: F64 = 6.0 * fTemp68;
                let mut iTemp70: i32 = (fTemp69) as i32;
                let mut iTemp71: i32 = std::cmp::max(0, std::cmp::min(iTemp70, 6));
                let mut iTemp72: i32 =
                    std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, 458745), 917503));
                let mut fTemp73: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp72, 7)) as usize] };
                let mut fTemp74: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp72 as usize] };
                let mut fTemp75: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp72, 1)) as usize] }
                        - fTemp74;
                let mut fTemp76: F64 = fTemp69 - (iTemp70) as F64;
                let mut fTemp77: F64 = fTemp74
                    + fTemp76 * fTemp75
                    + 0.5
                        * (fTemp73
                            - (fTemp74
                                + fTemp76
                                    * (fTemp75
                                        - (unsafe {
                                            ftbl0LambRs48kSIG0
                                                [(i32::wrapping_add(iTemp72, 8)) as usize]
                                        } - fTemp73))));
                let mut fTemp78: F64 = (if iTemp63 != 0 { fTemp77 } else { 1.0 - fTemp77 });
                let mut iTemp79: i32 = (fTemp62 < 0.0) as i32;
                let mut fTemp80: F64 = fSlow1 * (iTemp79) as F64 + fSlow13 * (iTemp63) as F64;
                self.fVec30[0] = fTemp80;
                let mut fTemp81: F64 = self.fConst10 / fTemp80;
                let mut fTemp82: F64 = fTemp81 + 0.5;
                let mut fTemp83: F64 = 131071.0 * (1.0 - fTemp82);
                let mut iTemp84: i32 = (fTemp83) as i32;
                let mut iTemp85: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp84, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp86: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp85, 7)) as usize] };
                let mut fTemp87: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp85 as usize] };
                let mut fTemp88: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp85, 1)) as usize] }
                        - fTemp87;
                let mut fTemp89: F64 = 131071.0 * fTemp82;
                let mut iTemp90: i32 = (fTemp89) as i32;
                let mut iTemp91: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp90, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp92: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp91, 7), 917503),
                    )) as usize]
                };
                let mut fTemp93: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp91 as usize] };
                let mut fTemp94: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp91, 1), 917503),
                    )) as usize]
                } - fTemp93;
                let mut fTemp95: F64 = 6.0 * self.fVec29[1];
                let mut iTemp96: i32 = (fTemp95) as i32;
                let mut iTemp97: i32 = std::cmp::max(0, std::cmp::min(iTemp96, 6));
                let mut fTemp98: F64 = 131071.0 * (1.0 - self.fRec1[1]);
                let mut iTemp99: i32 = (fTemp98) as i32;
                let mut iTemp100: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp99, 131071))),
                            iTemp97,
                        ),
                        917503,
                    ),
                );
                let mut fTemp101: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp100, 7), 917503),
                    )) as usize]
                };
                let mut fTemp102: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp100 as usize] };
                let mut fTemp103: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp100, 1), 917503),
                    )) as usize]
                } - fTemp102;
                let mut fTemp104: F64 = fTemp95 - (iTemp96) as F64;
                let mut fTemp105: F64 = 131071.0 * self.fRec1[1];
                let mut iTemp106: i32 = (fTemp105) as i32;
                let mut iTemp107: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp97,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp106, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp108: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp107, 7), 917503),
                    )) as usize]
                };
                let mut fTemp109: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp107 as usize] };
                let mut fTemp110: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp107, 1), 917503),
                    )) as usize]
                } - fTemp109;
                let mut fTemp111: F64 = self.fRec1[1] + fTemp81;
                let mut fTemp112: F64 = 131071.0 * (1.0 - fTemp111);
                let mut iTemp113: i32 = (fTemp112) as i32;
                let mut iTemp114: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp113, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp115: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp114, 7)) as usize] };
                let mut fTemp116: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp114 as usize] };
                let mut fTemp117: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp114, 1)) as usize] }
                        - fTemp116;
                let mut fTemp118: F64 = 131071.0 * fTemp111;
                let mut iTemp119: i32 = (fTemp118) as i32;
                let mut iTemp120: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp119, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp121: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp120, 7), 917503),
                    )) as usize]
                };
                let mut fTemp122: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp120 as usize] };
                let mut fTemp123: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp120, 1), 917503),
                    )) as usize]
                } - fTemp122;
                let mut fTemp124: F64 =
                    self.fRec1[1] + self.fConst10 * (1.0 / fTemp80 + 1.0 / self.fVec30[1]);
                let mut fTemp125: F64 = 131071.0 * (1.0 - fTemp124);
                let mut iTemp126: i32 = (fTemp125) as i32;
                let mut iTemp127: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp126, 131071))),
                            iTemp71,
                        ),
                        917503,
                    ),
                );
                let mut fTemp128: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp127, 7)) as usize] };
                let mut fTemp129: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp127 as usize] };
                let mut fTemp130: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp127, 1)) as usize] }
                        - fTemp129;
                let mut fTemp131: F64 = 131071.0 * fTemp124;
                let mut iTemp132: i32 = (fTemp131) as i32;
                let mut iTemp133: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp132, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp134: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp133, 7), 917503),
                    )) as usize]
                };
                let mut fTemp135: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp133 as usize] };
                let mut fTemp136: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp133, 1), 917503),
                    )) as usize]
                } - fTemp135;
                let mut fTemp137: F64 = ((if iTemp63 != 0 {
                    fTemp135
                        + fTemp76 * fTemp136
                        + (fTemp131 - (iTemp132) as F64)
                            * (fTemp134
                                - (fTemp135
                                    + fTemp76
                                        * (fTemp136
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp133, 8),
                                                        917503,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp134))))
                } else {
                    1.0 - (fTemp129
                        + fTemp76 * fTemp130
                        + (fTemp125 - (iTemp126) as F64)
                            * (fTemp128
                                - (fTemp129
                                    + fTemp76
                                        * (fTemp130
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp127, 8)) as usize]
                                            } - fTemp128)))))
                }) - (if iTemp63 != 0 {
                    fTemp122
                        + fTemp76 * fTemp123
                        + (fTemp118 - (iTemp119) as F64)
                            * (fTemp121
                                - (fTemp122
                                    + fTemp76
                                        * (fTemp123
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp120, 8),
                                                        917503,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp121))))
                } else {
                    1.0 - (fTemp116
                        + fTemp76 * fTemp117
                        + (fTemp112 - (iTemp113) as F64)
                            * (fTemp115
                                - (fTemp116
                                    + fTemp76
                                        * (fTemp117
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp114, 8)) as usize]
                                            } - fTemp115)))))
                })) * self.fVec28[1]
                    / (fTemp62
                        * (1.0
                            - (if iTemp63 != 0 {
                                fTemp109
                                    + fTemp104 * fTemp110
                                    + (fTemp105 - (iTemp106) as F64)
                                        * (fTemp108
                                            - (fTemp109
                                                + fTemp104
                                                    * (fTemp110
                                                        - (unsafe {
                                                            ftbl0LambRs48kSIG0[(std::cmp::max(
                                                                0,
                                                                std::cmp::min(
                                                                    i32::wrapping_add(iTemp107, 8),
                                                                    917503,
                                                                ),
                                                            ))
                                                                as usize]
                                                        } - fTemp108))))
                            } else {
                                1.0 - (fTemp102
                                    + fTemp104 * fTemp103
                                    + (fTemp98 - (iTemp99) as F64)
                                        * (fTemp101
                                            - (fTemp102
                                                + fTemp104
                                                    * (fTemp103
                                                        - (unsafe {
                                                            ftbl0LambRs48kSIG0[(std::cmp::max(
                                                                0,
                                                                std::cmp::min(
                                                                    i32::wrapping_add(iTemp100, 8),
                                                                    917503,
                                                                ),
                                                            ))
                                                                as usize]
                                                        } - fTemp101)))))
                            })));
                let mut iTemp138: i32 = (fTemp137
                    > (((if iTemp63 != 0 {
                        fTemp93
                            + fTemp76 * fTemp94
                            + (fTemp89 - (iTemp90) as F64)
                                * (fTemp92
                                    - (fTemp93
                                        + fTemp76
                                            * (fTemp94
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp91, 8),
                                                            917503,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp92))))
                    } else {
                        1.0 - (fTemp87
                            + fTemp76 * fTemp88
                            + (fTemp83 - (iTemp84) as F64)
                                * (fTemp86
                                    - (fTemp87
                                        + fTemp76
                                            * (fTemp88
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0
                                                        [(i32::wrapping_add(iTemp85, 8)) as usize]
                                                } - fTemp86)))))
                    }) - fTemp78)
                        / (1.0 - fTemp78))) as i32;
                let mut fTemp139: F64 = (if iTemp138 != 0 { 1.0 } else { 0.5 });
                let mut fTemp140: F64 = (if iTemp138 != 0 { 0.5 } else { 0.0 });
                let mut fTemp141: F64 = fTemp140 + fTemp139;
                let mut fTemp142: F64 = 0.5 * fTemp141;
                let mut fTemp143: F64 = 131071.0 * (1.0 - fTemp142);
                let mut iTemp144: i32 = (fTemp143) as i32;
                let mut iTemp145: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp144, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp146: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp145, 7)) as usize] };
                let mut fTemp147: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp145 as usize] };
                let mut fTemp148: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp145, 1)) as usize] }
                        - fTemp147;
                let mut fTemp149: F64 = 65535.5 * fTemp141;
                let mut iTemp150: i32 = (fTemp149) as i32;
                let mut iTemp151: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp150, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp152: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp151, 7)) as usize] };
                let mut fTemp153: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp151 as usize] };
                let mut fTemp154: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp151, 1)) as usize] }
                        - fTemp153;
                let mut fTemp155: F64 = (if iTemp63 != 0 {
                    fTemp153
                        + fTemp76 * fTemp154
                        + (fTemp149 - (iTemp150) as F64)
                            * (fTemp152
                                - (fTemp153
                                    + fTemp76
                                        * (fTemp154
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp151, 8)) as usize]
                                            } - fTemp152))))
                } else {
                    1.0 - (fTemp147
                        + fTemp76 * fTemp148
                        + (fTemp143 - (iTemp144) as F64)
                            * (fTemp146
                                - (fTemp147
                                    + fTemp76
                                        * (fTemp148
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp145, 8)) as usize]
                                            } - fTemp146)))))
                });
                let mut fTemp156: F64 = fTemp81 + fTemp142;
                let mut fTemp157: F64 = 131071.0 * (1.0 - fTemp156);
                let mut iTemp158: i32 = (fTemp157) as i32;
                let mut iTemp159: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp158, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp160: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp159, 7)) as usize] };
                let mut fTemp161: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp159 as usize] };
                let mut fTemp162: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp159, 1)) as usize] }
                        - fTemp161;
                let mut fTemp163: F64 = 131071.0 * fTemp156;
                let mut iTemp164: i32 = (fTemp163) as i32;
                let mut iTemp165: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp164, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp166: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp165, 7), 917503),
                    )) as usize]
                };
                let mut fTemp167: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp165 as usize] };
                let mut fTemp168: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp165, 1), 917503),
                    )) as usize]
                } - fTemp167;
                let mut iTemp169: i32 = (fTemp137
                    > (((if iTemp63 != 0 {
                        fTemp167
                            + fTemp76 * fTemp168
                            + (fTemp163 - (iTemp164) as F64)
                                * (fTemp166
                                    - (fTemp167
                                        + fTemp76
                                            * (fTemp168
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp165, 8),
                                                            917503,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp166))))
                    } else {
                        1.0 - (fTemp161
                            + fTemp76 * fTemp162
                            + (fTemp157 - (iTemp158) as F64)
                                * (fTemp160
                                    - (fTemp161
                                        + fTemp76
                                            * (fTemp162
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0
                                                        [(i32::wrapping_add(iTemp159, 8)) as usize]
                                                } - fTemp160)))))
                    }) - fTemp155)
                        / (1.0 - fTemp155))) as i32;
                let mut fTemp170: F64 = (if iTemp169 != 0 { fTemp139 } else { fTemp142 });
                let mut fTemp171: F64 = (if iTemp169 != 0 { fTemp142 } else { fTemp140 });
                let mut fTemp172: F64 = fTemp171 + fTemp170;
                let mut fTemp173: F64 = 0.5 * fTemp172;
                let mut fTemp174: F64 = 131071.0 * (1.0 - fTemp173);
                let mut iTemp175: i32 = (fTemp174) as i32;
                let mut iTemp176: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp175, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp177: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp176, 7)) as usize] };
                let mut fTemp178: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp176 as usize] };
                let mut fTemp179: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp176, 1)) as usize] }
                        - fTemp178;
                let mut fTemp180: F64 = 65535.5 * fTemp172;
                let mut iTemp181: i32 = (fTemp180) as i32;
                let mut iTemp182: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp181, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp183: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp182, 7)) as usize] };
                let mut fTemp184: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp182 as usize] };
                let mut fTemp185: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp182, 1)) as usize] }
                        - fTemp184;
                let mut fTemp186: F64 = (if iTemp63 != 0 {
                    fTemp184
                        + fTemp76 * fTemp185
                        + (fTemp180 - (iTemp181) as F64)
                            * (fTemp183
                                - (fTemp184
                                    + fTemp76
                                        * (fTemp185
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp182, 8)) as usize]
                                            } - fTemp183))))
                } else {
                    1.0 - (fTemp178
                        + fTemp76 * fTemp179
                        + (fTemp174 - (iTemp175) as F64)
                            * (fTemp177
                                - (fTemp178
                                    + fTemp76
                                        * (fTemp179
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp176, 8)) as usize]
                                            } - fTemp177)))))
                });
                let mut fTemp187: F64 = fTemp81 + fTemp173;
                let mut fTemp188: F64 = 131071.0 * (1.0 - fTemp187);
                let mut iTemp189: i32 = (fTemp188) as i32;
                let mut iTemp190: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp189, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp191: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp190, 7)) as usize] };
                let mut fTemp192: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp190 as usize] };
                let mut fTemp193: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp190, 1)) as usize] }
                        - fTemp192;
                let mut fTemp194: F64 = 131071.0 * fTemp187;
                let mut iTemp195: i32 = (fTemp194) as i32;
                let mut iTemp196: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp195, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp197: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp196, 7), 917503),
                    )) as usize]
                };
                let mut fTemp198: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp196 as usize] };
                let mut fTemp199: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp196, 1), 917503),
                    )) as usize]
                } - fTemp198;
                let mut iTemp200: i32 = (fTemp137
                    > (((if iTemp63 != 0 {
                        fTemp198
                            + fTemp76 * fTemp199
                            + (fTemp194 - (iTemp195) as F64)
                                * (fTemp197
                                    - (fTemp198
                                        + fTemp76
                                            * (fTemp199
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp196, 8),
                                                            917503,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp197))))
                    } else {
                        1.0 - (fTemp192
                            + fTemp76 * fTemp193
                            + (fTemp188 - (iTemp189) as F64)
                                * (fTemp191
                                    - (fTemp192
                                        + fTemp76
                                            * (fTemp193
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0
                                                        [(i32::wrapping_add(iTemp190, 8)) as usize]
                                                } - fTemp191)))))
                    }) - fTemp186)
                        / (1.0 - fTemp186))) as i32;
                let mut fTemp201: F64 = (if iTemp200 != 0 { fTemp170 } else { fTemp173 });
                let mut fTemp202: F64 = (if iTemp200 != 0 { fTemp173 } else { fTemp171 });
                let mut fTemp203: F64 = fTemp202 + fTemp201;
                let mut fTemp204: F64 = 0.5 * fTemp203;
                let mut fTemp205: F64 = 131071.0 * (1.0 - fTemp204);
                let mut iTemp206: i32 = (fTemp205) as i32;
                let mut iTemp207: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp206, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp208: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp207, 7)) as usize] };
                let mut fTemp209: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp207 as usize] };
                let mut fTemp210: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp207, 1)) as usize] }
                        - fTemp209;
                let mut fTemp211: F64 = 65535.5 * fTemp203;
                let mut iTemp212: i32 = (fTemp211) as i32;
                let mut iTemp213: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp212, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp214: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp213, 7)) as usize] };
                let mut fTemp215: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp213 as usize] };
                let mut fTemp216: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp213, 1)) as usize] }
                        - fTemp215;
                let mut fTemp217: F64 = (if iTemp63 != 0 {
                    fTemp215
                        + fTemp76 * fTemp216
                        + (fTemp211 - (iTemp212) as F64)
                            * (fTemp214
                                - (fTemp215
                                    + fTemp76
                                        * (fTemp216
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp213, 8)) as usize]
                                            } - fTemp214))))
                } else {
                    1.0 - (fTemp209
                        + fTemp76 * fTemp210
                        + (fTemp205 - (iTemp206) as F64)
                            * (fTemp208
                                - (fTemp209
                                    + fTemp76
                                        * (fTemp210
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp207, 8)) as usize]
                                            } - fTemp208)))))
                });
                let mut fTemp218: F64 = fTemp81 + fTemp204;
                let mut fTemp219: F64 = 131071.0 * (1.0 - fTemp218);
                let mut iTemp220: i32 = (fTemp219) as i32;
                let mut iTemp221: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp220, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp222: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp221, 7)) as usize] };
                let mut fTemp223: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp221 as usize] };
                let mut fTemp224: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp221, 1)) as usize] }
                        - fTemp223;
                let mut fTemp225: F64 = 131071.0 * fTemp218;
                let mut iTemp226: i32 = (fTemp225) as i32;
                let mut iTemp227: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp226, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp228: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp227, 7), 917503),
                    )) as usize]
                };
                let mut fTemp229: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp227 as usize] };
                let mut fTemp230: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp227, 1), 917503),
                    )) as usize]
                } - fTemp229;
                let mut iTemp231: i32 = (fTemp137
                    > (((if iTemp63 != 0 {
                        fTemp229
                            + fTemp76 * fTemp230
                            + (fTemp225 - (iTemp226) as F64)
                                * (fTemp228
                                    - (fTemp229
                                        + fTemp76
                                            * (fTemp230
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp227, 8),
                                                            917503,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp228))))
                    } else {
                        1.0 - (fTemp223
                            + fTemp76 * fTemp224
                            + (fTemp219 - (iTemp220) as F64)
                                * (fTemp222
                                    - (fTemp223
                                        + fTemp76
                                            * (fTemp224
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0
                                                        [(i32::wrapping_add(iTemp221, 8)) as usize]
                                                } - fTemp222)))))
                    }) - fTemp217)
                        / (1.0 - fTemp217))) as i32;
                let mut fTemp232: F64 = (if iTemp231 != 0 { fTemp201 } else { fTemp204 });
                let mut fTemp233: F64 = (if iTemp231 != 0 { fTemp204 } else { fTemp202 });
                let mut fTemp234: F64 = fTemp233 + fTemp232;
                let mut fTemp235: F64 = 0.5 * fTemp234;
                let mut fTemp236: F64 = 131071.0 * (1.0 - fTemp235);
                let mut iTemp237: i32 = (fTemp236) as i32;
                let mut iTemp238: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp237, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp239: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp238, 7)) as usize] };
                let mut fTemp240: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp238 as usize] };
                let mut fTemp241: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp238, 1)) as usize] }
                        - fTemp240;
                let mut fTemp242: F64 = 65535.5 * fTemp234;
                let mut iTemp243: i32 = (fTemp242) as i32;
                let mut iTemp244: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp243, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp245: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp244, 7)) as usize] };
                let mut fTemp246: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp244 as usize] };
                let mut fTemp247: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp244, 1)) as usize] }
                        - fTemp246;
                let mut fTemp248: F64 = (if iTemp63 != 0 {
                    fTemp246
                        + fTemp76 * fTemp247
                        + (fTemp242 - (iTemp243) as F64)
                            * (fTemp245
                                - (fTemp246
                                    + fTemp76
                                        * (fTemp247
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp244, 8)) as usize]
                                            } - fTemp245))))
                } else {
                    1.0 - (fTemp240
                        + fTemp76 * fTemp241
                        + (fTemp236 - (iTemp237) as F64)
                            * (fTemp239
                                - (fTemp240
                                    + fTemp76
                                        * (fTemp241
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp238, 8)) as usize]
                                            } - fTemp239)))))
                });
                let mut fTemp249: F64 = fTemp81 + fTemp235;
                let mut fTemp250: F64 = 131071.0 * (1.0 - fTemp249);
                let mut iTemp251: i32 = (fTemp250) as i32;
                let mut iTemp252: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp251, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp253: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp252, 7)) as usize] };
                let mut fTemp254: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp252 as usize] };
                let mut fTemp255: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp252, 1)) as usize] }
                        - fTemp254;
                let mut fTemp256: F64 = 131071.0 * fTemp249;
                let mut iTemp257: i32 = (fTemp256) as i32;
                let mut iTemp258: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp257, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp259: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp258, 7), 917503),
                    )) as usize]
                };
                let mut fTemp260: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp258 as usize] };
                let mut fTemp261: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp258, 1), 917503),
                    )) as usize]
                } - fTemp260;
                let mut iTemp262: i32 = (fTemp137
                    > (((if iTemp63 != 0 {
                        fTemp260
                            + fTemp76 * fTemp261
                            + (fTemp256 - (iTemp257) as F64)
                                * (fTemp259
                                    - (fTemp260
                                        + fTemp76
                                            * (fTemp261
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp258, 8),
                                                            917503,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp259))))
                    } else {
                        1.0 - (fTemp254
                            + fTemp76 * fTemp255
                            + (fTemp250 - (iTemp251) as F64)
                                * (fTemp253
                                    - (fTemp254
                                        + fTemp76
                                            * (fTemp255
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0
                                                        [(i32::wrapping_add(iTemp252, 8)) as usize]
                                                } - fTemp253)))))
                    }) - fTemp248)
                        / (1.0 - fTemp248))) as i32;
                let mut fTemp263: F64 = (if iTemp262 != 0 { fTemp232 } else { fTemp235 });
                let mut fTemp264: F64 = (if iTemp262 != 0 { fTemp235 } else { fTemp233 });
                let mut fTemp265: F64 = fTemp264 + fTemp263;
                let mut fTemp266: F64 = 0.5 * fTemp265;
                let mut fTemp267: F64 = 131071.0 * (1.0 - fTemp266);
                let mut iTemp268: i32 = (fTemp267) as i32;
                let mut iTemp269: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp268, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp270: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp269, 7)) as usize] };
                let mut fTemp271: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp269 as usize] };
                let mut fTemp272: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp269, 1)) as usize] }
                        - fTemp271;
                let mut fTemp273: F64 = 65535.5 * fTemp265;
                let mut iTemp274: i32 = (fTemp273) as i32;
                let mut iTemp275: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp274, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp276: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp275, 7)) as usize] };
                let mut fTemp277: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp275 as usize] };
                let mut fTemp278: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp275, 1)) as usize] }
                        - fTemp277;
                let mut fTemp279: F64 = (if iTemp63 != 0 {
                    fTemp277
                        + fTemp76 * fTemp278
                        + (fTemp273 - (iTemp274) as F64)
                            * (fTemp276
                                - (fTemp277
                                    + fTemp76
                                        * (fTemp278
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp275, 8)) as usize]
                                            } - fTemp276))))
                } else {
                    1.0 - (fTemp271
                        + fTemp76 * fTemp272
                        + (fTemp267 - (iTemp268) as F64)
                            * (fTemp270
                                - (fTemp271
                                    + fTemp76
                                        * (fTemp272
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp269, 8)) as usize]
                                            } - fTemp270)))))
                });
                let mut fTemp280: F64 = fTemp81 + fTemp266;
                let mut fTemp281: F64 = 131071.0 * (1.0 - fTemp280);
                let mut iTemp282: i32 = (fTemp281) as i32;
                let mut iTemp283: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp282, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp284: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp283, 7)) as usize] };
                let mut fTemp285: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp283 as usize] };
                let mut fTemp286: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp283, 1)) as usize] }
                        - fTemp285;
                let mut fTemp287: F64 = 131071.0 * fTemp280;
                let mut iTemp288: i32 = (fTemp287) as i32;
                let mut iTemp289: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp288, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp290: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp289, 7), 917503),
                    )) as usize]
                };
                let mut fTemp291: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp289 as usize] };
                let mut fTemp292: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp289, 1), 917503),
                    )) as usize]
                } - fTemp291;
                let mut iTemp293: i32 = (fTemp137
                    > (((if iTemp63 != 0 {
                        fTemp291
                            + fTemp76 * fTemp292
                            + (fTemp287 - (iTemp288) as F64)
                                * (fTemp290
                                    - (fTemp291
                                        + fTemp76
                                            * (fTemp292
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp289, 8),
                                                            917503,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp290))))
                    } else {
                        1.0 - (fTemp285
                            + fTemp76 * fTemp286
                            + (fTemp281 - (iTemp282) as F64)
                                * (fTemp284
                                    - (fTemp285
                                        + fTemp76
                                            * (fTemp286
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0
                                                        [(i32::wrapping_add(iTemp283, 8)) as usize]
                                                } - fTemp284)))))
                    }) - fTemp279)
                        / (1.0 - fTemp279))) as i32;
                let mut fTemp294: F64 = (if iTemp293 != 0 { fTemp263 } else { fTemp266 });
                let mut fTemp295: F64 = (if iTemp293 != 0 { fTemp266 } else { fTemp264 });
                let mut fTemp296: F64 = fTemp295 + fTemp294;
                let mut fTemp297: F64 = 0.5 * fTemp296;
                let mut fTemp298: F64 = 131071.0 * (1.0 - fTemp297);
                let mut iTemp299: i32 = (fTemp298) as i32;
                let mut iTemp300: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp299, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp301: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp300, 7)) as usize] };
                let mut fTemp302: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp300 as usize] };
                let mut fTemp303: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp300, 1)) as usize] }
                        - fTemp302;
                let mut fTemp304: F64 = 65535.5 * fTemp296;
                let mut iTemp305: i32 = (fTemp304) as i32;
                let mut iTemp306: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp305, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp307: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp306, 7)) as usize] };
                let mut fTemp308: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp306 as usize] };
                let mut fTemp309: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp306, 1)) as usize] }
                        - fTemp308;
                let mut fTemp310: F64 = (if iTemp63 != 0 {
                    fTemp308
                        + fTemp76 * fTemp309
                        + (fTemp304 - (iTemp305) as F64)
                            * (fTemp307
                                - (fTemp308
                                    + fTemp76
                                        * (fTemp309
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp306, 8)) as usize]
                                            } - fTemp307))))
                } else {
                    1.0 - (fTemp302
                        + fTemp76 * fTemp303
                        + (fTemp298 - (iTemp299) as F64)
                            * (fTemp301
                                - (fTemp302
                                    + fTemp76
                                        * (fTemp303
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp300, 8)) as usize]
                                            } - fTemp301)))))
                });
                let mut fTemp311: F64 = fTemp81 + fTemp297;
                let mut fTemp312: F64 = 131071.0 * (1.0 - fTemp311);
                let mut iTemp313: i32 = (fTemp312) as i32;
                let mut iTemp314: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp313, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp315: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp314, 7)) as usize] };
                let mut fTemp316: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp314 as usize] };
                let mut fTemp317: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp314, 1)) as usize] }
                        - fTemp316;
                let mut fTemp318: F64 = 131071.0 * fTemp311;
                let mut iTemp319: i32 = (fTemp318) as i32;
                let mut iTemp320: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp319, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp321: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp320, 7), 917503),
                    )) as usize]
                };
                let mut fTemp322: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp320 as usize] };
                let mut fTemp323: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp320, 1), 917503),
                    )) as usize]
                } - fTemp322;
                let mut iTemp324: i32 = (fTemp137
                    > (((if iTemp63 != 0 {
                        fTemp322
                            + fTemp76 * fTemp323
                            + (fTemp318 - (iTemp319) as F64)
                                * (fTemp321
                                    - (fTemp322
                                        + fTemp76
                                            * (fTemp323
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp320, 8),
                                                            917503,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp321))))
                    } else {
                        1.0 - (fTemp316
                            + fTemp76 * fTemp317
                            + (fTemp312 - (iTemp313) as F64)
                                * (fTemp315
                                    - (fTemp316
                                        + fTemp76
                                            * (fTemp317
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0
                                                        [(i32::wrapping_add(iTemp314, 8)) as usize]
                                                } - fTemp315)))))
                    }) - fTemp310)
                        / (1.0 - fTemp310))) as i32;
                let mut fTemp325: F64 = (if iTemp324 != 0 { fTemp294 } else { fTemp297 });
                let mut fTemp326: F64 = (if iTemp324 != 0 { fTemp297 } else { fTemp295 });
                let mut fTemp327: F64 = fTemp326 + fTemp325;
                let mut fTemp328: F64 = 0.5 * fTemp327;
                let mut fTemp329: F64 = 131071.0 * (1.0 - fTemp328);
                let mut iTemp330: i32 = (fTemp329) as i32;
                let mut iTemp331: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp330, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp332: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp331, 7)) as usize] };
                let mut fTemp333: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp331 as usize] };
                let mut fTemp334: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp331, 1)) as usize] }
                        - fTemp333;
                let mut fTemp335: F64 = 65535.5 * fTemp327;
                let mut iTemp336: i32 = (fTemp335) as i32;
                let mut iTemp337: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp336, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp338: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp337, 7)) as usize] };
                let mut fTemp339: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp337 as usize] };
                let mut fTemp340: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp337, 1)) as usize] }
                        - fTemp339;
                let mut fTemp341: F64 = (if iTemp63 != 0 {
                    fTemp339
                        + fTemp76 * fTemp340
                        + (fTemp335 - (iTemp336) as F64)
                            * (fTemp338
                                - (fTemp339
                                    + fTemp76
                                        * (fTemp340
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp337, 8)) as usize]
                                            } - fTemp338))))
                } else {
                    1.0 - (fTemp333
                        + fTemp76 * fTemp334
                        + (fTemp329 - (iTemp330) as F64)
                            * (fTemp332
                                - (fTemp333
                                    + fTemp76
                                        * (fTemp334
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp331, 8)) as usize]
                                            } - fTemp332)))))
                });
                let mut fTemp342: F64 = fTemp81 + fTemp328;
                let mut fTemp343: F64 = 131071.0 * (1.0 - fTemp342);
                let mut iTemp344: i32 = (fTemp343) as i32;
                let mut iTemp345: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp344, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp346: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp345, 7)) as usize] };
                let mut fTemp347: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp345 as usize] };
                let mut fTemp348: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp345, 1)) as usize] }
                        - fTemp347;
                let mut fTemp349: F64 = 131071.0 * fTemp342;
                let mut iTemp350: i32 = (fTemp349) as i32;
                let mut iTemp351: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp350, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp352: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp351, 7), 917503),
                    )) as usize]
                };
                let mut fTemp353: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp351 as usize] };
                let mut fTemp354: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp351, 1), 917503),
                    )) as usize]
                } - fTemp353;
                let mut iTemp355: i32 = (fTemp137
                    > (((if iTemp63 != 0 {
                        fTemp353
                            + fTemp76 * fTemp354
                            + (fTemp349 - (iTemp350) as F64)
                                * (fTemp352
                                    - (fTemp353
                                        + fTemp76
                                            * (fTemp354
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp351, 8),
                                                            917503,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp352))))
                    } else {
                        1.0 - (fTemp347
                            + fTemp76 * fTemp348
                            + (fTemp343 - (iTemp344) as F64)
                                * (fTemp346
                                    - (fTemp347
                                        + fTemp76
                                            * (fTemp348
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0
                                                        [(i32::wrapping_add(iTemp345, 8)) as usize]
                                                } - fTemp346)))))
                    }) - fTemp341)
                        / (1.0 - fTemp341))) as i32;
                let mut fTemp356: F64 = (if iTemp355 != 0 { fTemp325 } else { fTemp328 });
                let mut fTemp357: F64 = (if iTemp355 != 0 { fTemp328 } else { fTemp326 });
                let mut fTemp358: F64 = fTemp357 + fTemp356;
                let mut fTemp359: F64 = 0.5 * fTemp358;
                let mut fTemp360: F64 = 131071.0 * (1.0 - fTemp359);
                let mut iTemp361: i32 = (fTemp360) as i32;
                let mut iTemp362: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp361, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp363: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp362, 7)) as usize] };
                let mut fTemp364: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp362 as usize] };
                let mut fTemp365: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp362, 1)) as usize] }
                        - fTemp364;
                let mut fTemp366: F64 = 65535.5 * fTemp358;
                let mut iTemp367: i32 = (fTemp366) as i32;
                let mut iTemp368: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp367, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp369: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp368, 7)) as usize] };
                let mut fTemp370: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp368 as usize] };
                let mut fTemp371: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp368, 1)) as usize] }
                        - fTemp370;
                let mut fTemp372: F64 = (if iTemp63 != 0 {
                    fTemp370
                        + fTemp76 * fTemp371
                        + (fTemp366 - (iTemp367) as F64)
                            * (fTemp369
                                - (fTemp370
                                    + fTemp76
                                        * (fTemp371
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp368, 8)) as usize]
                                            } - fTemp369))))
                } else {
                    1.0 - (fTemp364
                        + fTemp76 * fTemp365
                        + (fTemp360 - (iTemp361) as F64)
                            * (fTemp363
                                - (fTemp364
                                    + fTemp76
                                        * (fTemp365
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp362, 8)) as usize]
                                            } - fTemp363)))))
                });
                let mut fTemp373: F64 = fTemp81 + fTemp359;
                let mut fTemp374: F64 = 131071.0 * (1.0 - fTemp373);
                let mut iTemp375: i32 = (fTemp374) as i32;
                let mut iTemp376: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp375, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp377: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp376, 7)) as usize] };
                let mut fTemp378: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp376 as usize] };
                let mut fTemp379: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp376, 1)) as usize] }
                        - fTemp378;
                let mut fTemp380: F64 = 131071.0 * fTemp373;
                let mut iTemp381: i32 = (fTemp380) as i32;
                let mut iTemp382: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp381, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp383: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp382, 7), 917503),
                    )) as usize]
                };
                let mut fTemp384: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp382 as usize] };
                let mut fTemp385: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp382, 1), 917503),
                    )) as usize]
                } - fTemp384;
                let mut iTemp386: i32 = (fTemp137
                    > (((if iTemp63 != 0 {
                        fTemp384
                            + fTemp76 * fTemp385
                            + (fTemp380 - (iTemp381) as F64)
                                * (fTemp383
                                    - (fTemp384
                                        + fTemp76
                                            * (fTemp385
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp382, 8),
                                                            917503,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp383))))
                    } else {
                        1.0 - (fTemp378
                            + fTemp76 * fTemp379
                            + (fTemp374 - (iTemp375) as F64)
                                * (fTemp377
                                    - (fTemp378
                                        + fTemp76
                                            * (fTemp379
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0
                                                        [(i32::wrapping_add(iTemp376, 8)) as usize]
                                                } - fTemp377)))))
                    }) - fTemp372)
                        / (1.0 - fTemp372))) as i32;
                let mut fTemp387: F64 = (if iTemp386 != 0 { fTemp356 } else { fTemp359 });
                let mut fTemp388: F64 = (if iTemp386 != 0 { fTemp359 } else { fTemp357 });
                let mut fTemp389: F64 = fTemp388 + fTemp387;
                let mut fTemp390: F64 = 0.5 * fTemp389;
                let mut fTemp391: F64 = 131071.0 * (1.0 - fTemp390);
                let mut iTemp392: i32 = (fTemp391) as i32;
                let mut iTemp393: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp392, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp394: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp393, 7)) as usize] };
                let mut fTemp395: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp393 as usize] };
                let mut fTemp396: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp393, 1)) as usize] }
                        - fTemp395;
                let mut fTemp397: F64 = 65535.5 * fTemp389;
                let mut iTemp398: i32 = (fTemp397) as i32;
                let mut iTemp399: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp398, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp400: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp399, 7)) as usize] };
                let mut fTemp401: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp399 as usize] };
                let mut fTemp402: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp399, 1)) as usize] }
                        - fTemp401;
                let mut fTemp403: F64 = (if iTemp63 != 0 {
                    fTemp401
                        + fTemp76 * fTemp402
                        + (fTemp397 - (iTemp398) as F64)
                            * (fTemp400
                                - (fTemp401
                                    + fTemp76
                                        * (fTemp402
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp399, 8)) as usize]
                                            } - fTemp400))))
                } else {
                    1.0 - (fTemp395
                        + fTemp76 * fTemp396
                        + (fTemp391 - (iTemp392) as F64)
                            * (fTemp394
                                - (fTemp395
                                    + fTemp76
                                        * (fTemp396
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp393, 8)) as usize]
                                            } - fTemp394)))))
                });
                let mut fTemp404: F64 = fTemp81 + fTemp390;
                let mut fTemp405: F64 = 131071.0 * (1.0 - fTemp404);
                let mut iTemp406: i32 = (fTemp405) as i32;
                let mut iTemp407: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp406, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp408: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp407, 7)) as usize] };
                let mut fTemp409: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp407 as usize] };
                let mut fTemp410: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp407, 1)) as usize] }
                        - fTemp409;
                let mut fTemp411: F64 = 131071.0 * fTemp404;
                let mut iTemp412: i32 = (fTemp411) as i32;
                let mut iTemp413: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp412, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp414: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp413, 7), 917503),
                    )) as usize]
                };
                let mut fTemp415: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp413 as usize] };
                let mut fTemp416: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp413, 1), 917503),
                    )) as usize]
                } - fTemp415;
                let mut iTemp417: i32 = (fTemp137
                    > (((if iTemp63 != 0 {
                        fTemp415
                            + fTemp76 * fTemp416
                            + (fTemp411 - (iTemp412) as F64)
                                * (fTemp414
                                    - (fTemp415
                                        + fTemp76
                                            * (fTemp416
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp413, 8),
                                                            917503,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp414))))
                    } else {
                        1.0 - (fTemp409
                            + fTemp76 * fTemp410
                            + (fTemp405 - (iTemp406) as F64)
                                * (fTemp408
                                    - (fTemp409
                                        + fTemp76
                                            * (fTemp410
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0
                                                        [(i32::wrapping_add(iTemp407, 8)) as usize]
                                                } - fTemp408)))))
                    }) - fTemp403)
                        / (1.0 - fTemp403))) as i32;
                let mut fTemp418: F64 = (if iTemp417 != 0 { fTemp387 } else { fTemp390 });
                let mut fTemp419: F64 = (if iTemp417 != 0 { fTemp390 } else { fTemp388 });
                let mut fTemp420: F64 = fTemp419 + fTemp418;
                let mut fTemp421: F64 = 0.5 * fTemp420;
                let mut fTemp422: F64 = 131071.0 * (1.0 - fTemp421);
                let mut iTemp423: i32 = (fTemp422) as i32;
                let mut iTemp424: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp423, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp425: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp424, 7)) as usize] };
                let mut fTemp426: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp424 as usize] };
                let mut fTemp427: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp424, 1)) as usize] }
                        - fTemp426;
                let mut fTemp428: F64 = 65535.5 * fTemp420;
                let mut iTemp429: i32 = (fTemp428) as i32;
                let mut iTemp430: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp429, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp431: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp430, 7)) as usize] };
                let mut fTemp432: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp430 as usize] };
                let mut fTemp433: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp430, 1)) as usize] }
                        - fTemp432;
                let mut fTemp434: F64 = (if iTemp63 != 0 {
                    fTemp432
                        + fTemp76 * fTemp433
                        + (fTemp428 - (iTemp429) as F64)
                            * (fTemp431
                                - (fTemp432
                                    + fTemp76
                                        * (fTemp433
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp430, 8)) as usize]
                                            } - fTemp431))))
                } else {
                    1.0 - (fTemp426
                        + fTemp76 * fTemp427
                        + (fTemp422 - (iTemp423) as F64)
                            * (fTemp425
                                - (fTemp426
                                    + fTemp76
                                        * (fTemp427
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp424, 8)) as usize]
                                            } - fTemp425)))))
                });
                let mut fTemp435: F64 = fTemp81 + fTemp421;
                let mut fTemp436: F64 = 131071.0 * (1.0 - fTemp435);
                let mut iTemp437: i32 = (fTemp436) as i32;
                let mut iTemp438: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp437, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp439: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp438, 7)) as usize] };
                let mut fTemp440: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp438 as usize] };
                let mut fTemp441: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp438, 1)) as usize] }
                        - fTemp440;
                let mut fTemp442: F64 = 131071.0 * fTemp435;
                let mut iTemp443: i32 = (fTemp442) as i32;
                let mut iTemp444: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp443, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp445: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp444, 7), 917503),
                    )) as usize]
                };
                let mut fTemp446: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp444 as usize] };
                let mut fTemp447: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp444, 1), 917503),
                    )) as usize]
                } - fTemp446;
                let mut iTemp448: i32 = (fTemp137
                    > (((if iTemp63 != 0 {
                        fTemp446
                            + fTemp76 * fTemp447
                            + (fTemp442 - (iTemp443) as F64)
                                * (fTemp445
                                    - (fTemp446
                                        + fTemp76
                                            * (fTemp447
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp444, 8),
                                                            917503,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp445))))
                    } else {
                        1.0 - (fTemp440
                            + fTemp76 * fTemp441
                            + (fTemp436 - (iTemp437) as F64)
                                * (fTemp439
                                    - (fTemp440
                                        + fTemp76
                                            * (fTemp441
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0
                                                        [(i32::wrapping_add(iTemp438, 8)) as usize]
                                                } - fTemp439)))))
                    }) - fTemp434)
                        / (1.0 - fTemp434))) as i32;
                let mut fTemp449: F64 = (if iTemp448 != 0 { fTemp418 } else { fTemp421 });
                let mut fTemp450: F64 = (if iTemp448 != 0 { fTemp421 } else { fTemp419 });
                let mut fTemp451: F64 = fTemp450 + fTemp449;
                let mut fTemp452: F64 = 0.5 * fTemp451;
                let mut fTemp453: F64 = 131071.0 * (1.0 - fTemp452);
                let mut iTemp454: i32 = (fTemp453) as i32;
                let mut iTemp455: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp454, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp456: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp455, 7)) as usize] };
                let mut fTemp457: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp455 as usize] };
                let mut fTemp458: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp455, 1)) as usize] }
                        - fTemp457;
                let mut fTemp459: F64 = 65535.5 * fTemp451;
                let mut iTemp460: i32 = (fTemp459) as i32;
                let mut iTemp461: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp460, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp462: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp461, 7)) as usize] };
                let mut fTemp463: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp461 as usize] };
                let mut fTemp464: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp461, 1)) as usize] }
                        - fTemp463;
                let mut fTemp465: F64 = (if iTemp63 != 0 {
                    fTemp463
                        + fTemp76 * fTemp464
                        + (fTemp459 - (iTemp460) as F64)
                            * (fTemp462
                                - (fTemp463
                                    + fTemp76
                                        * (fTemp464
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp461, 8)) as usize]
                                            } - fTemp462))))
                } else {
                    1.0 - (fTemp457
                        + fTemp76 * fTemp458
                        + (fTemp453 - (iTemp454) as F64)
                            * (fTemp456
                                - (fTemp457
                                    + fTemp76
                                        * (fTemp458
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp455, 8)) as usize]
                                            } - fTemp456)))))
                });
                let mut fTemp466: F64 = fTemp81 + fTemp452;
                let mut fTemp467: F64 = 131071.0 * (1.0 - fTemp466);
                let mut iTemp468: i32 = (fTemp467) as i32;
                let mut iTemp469: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp468, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp470: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp469, 7)) as usize] };
                let mut fTemp471: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp469 as usize] };
                let mut fTemp472: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp469, 1)) as usize] }
                        - fTemp471;
                let mut fTemp473: F64 = 131071.0 * fTemp466;
                let mut iTemp474: i32 = (fTemp473) as i32;
                let mut iTemp475: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp474, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp476: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp475, 7), 917503),
                    )) as usize]
                };
                let mut fTemp477: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp475 as usize] };
                let mut fTemp478: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp475, 1), 917503),
                    )) as usize]
                } - fTemp477;
                let mut iTemp479: i32 = (fTemp137
                    > (((if iTemp63 != 0 {
                        fTemp477
                            + fTemp76 * fTemp478
                            + (fTemp473 - (iTemp474) as F64)
                                * (fTemp476
                                    - (fTemp477
                                        + fTemp76
                                            * (fTemp478
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp475, 8),
                                                            917503,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp476))))
                    } else {
                        1.0 - (fTemp471
                            + fTemp76 * fTemp472
                            + (fTemp467 - (iTemp468) as F64)
                                * (fTemp470
                                    - (fTemp471
                                        + fTemp76
                                            * (fTemp472
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0
                                                        [(i32::wrapping_add(iTemp469, 8)) as usize]
                                                } - fTemp470)))))
                    }) - fTemp465)
                        / (1.0 - fTemp465))) as i32;
                let mut fTemp480: F64 = (if iTemp479 != 0 { fTemp449 } else { fTemp452 });
                let mut fTemp481: F64 = (if iTemp479 != 0 { fTemp452 } else { fTemp450 });
                let mut fTemp482: F64 = fTemp481 + fTemp480;
                let mut fTemp483: F64 = 0.5 * fTemp482;
                let mut fTemp484: F64 = 131071.0 * (1.0 - fTemp483);
                let mut iTemp485: i32 = (fTemp484) as i32;
                let mut iTemp486: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp485, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp487: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp486, 7)) as usize] };
                let mut fTemp488: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp486 as usize] };
                let mut fTemp489: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp486, 1)) as usize] }
                        - fTemp488;
                let mut fTemp490: F64 = 65535.5 * fTemp482;
                let mut iTemp491: i32 = (fTemp490) as i32;
                let mut iTemp492: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp491, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp493: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp492, 7)) as usize] };
                let mut fTemp494: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp492 as usize] };
                let mut fTemp495: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp492, 1)) as usize] }
                        - fTemp494;
                let mut fTemp496: F64 = (if iTemp63 != 0 {
                    fTemp494
                        + fTemp76 * fTemp495
                        + (fTemp490 - (iTemp491) as F64)
                            * (fTemp493
                                - (fTemp494
                                    + fTemp76
                                        * (fTemp495
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp492, 8)) as usize]
                                            } - fTemp493))))
                } else {
                    1.0 - (fTemp488
                        + fTemp76 * fTemp489
                        + (fTemp484 - (iTemp485) as F64)
                            * (fTemp487
                                - (fTemp488
                                    + fTemp76
                                        * (fTemp489
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp486, 8)) as usize]
                                            } - fTemp487)))))
                });
                let mut fTemp497: F64 = fTemp81 + fTemp483;
                let mut fTemp498: F64 = 131071.0 * (1.0 - fTemp497);
                let mut iTemp499: i32 = (fTemp498) as i32;
                let mut iTemp500: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp499, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp501: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp500, 7)) as usize] };
                let mut fTemp502: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp500 as usize] };
                let mut fTemp503: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp500, 1)) as usize] }
                        - fTemp502;
                let mut fTemp504: F64 = 131071.0 * fTemp497;
                let mut iTemp505: i32 = (fTemp504) as i32;
                let mut iTemp506: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp505, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp507: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp506, 7), 917503),
                    )) as usize]
                };
                let mut fTemp508: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp506 as usize] };
                let mut fTemp509: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp506, 1), 917503),
                    )) as usize]
                } - fTemp508;
                let mut iTemp510: i32 = (fTemp137
                    > (((if iTemp63 != 0 {
                        fTemp508
                            + fTemp76 * fTemp509
                            + (fTemp504 - (iTemp505) as F64)
                                * (fTemp507
                                    - (fTemp508
                                        + fTemp76
                                            * (fTemp509
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp506, 8),
                                                            917503,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp507))))
                    } else {
                        1.0 - (fTemp502
                            + fTemp76 * fTemp503
                            + (fTemp498 - (iTemp499) as F64)
                                * (fTemp501
                                    - (fTemp502
                                        + fTemp76
                                            * (fTemp503
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0
                                                        [(i32::wrapping_add(iTemp500, 8)) as usize]
                                                } - fTemp501)))))
                    }) - fTemp496)
                        / (1.0 - fTemp496))) as i32;
                let mut fTemp511: F64 = (if iTemp510 != 0 { fTemp480 } else { fTemp483 });
                let mut fTemp512: F64 = (if iTemp510 != 0 { fTemp483 } else { fTemp481 });
                let mut fTemp513: F64 = fTemp512 + fTemp511;
                let mut fTemp514: F64 = 0.5 * fTemp513;
                let mut fTemp515: F64 = 131071.0 * (1.0 - fTemp514);
                let mut iTemp516: i32 = (fTemp515) as i32;
                let mut iTemp517: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp516, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp518: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp517, 7)) as usize] };
                let mut fTemp519: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp517 as usize] };
                let mut fTemp520: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp517, 1)) as usize] }
                        - fTemp519;
                let mut fTemp521: F64 = 65535.5 * fTemp513;
                let mut iTemp522: i32 = (fTemp521) as i32;
                let mut iTemp523: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp522, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp524: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp523, 7)) as usize] };
                let mut fTemp525: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp523 as usize] };
                let mut fTemp526: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp523, 1)) as usize] }
                        - fTemp525;
                let mut fTemp527: F64 = (if iTemp63 != 0 {
                    fTemp525
                        + fTemp76 * fTemp526
                        + (fTemp521 - (iTemp522) as F64)
                            * (fTemp524
                                - (fTemp525
                                    + fTemp76
                                        * (fTemp526
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp523, 8)) as usize]
                                            } - fTemp524))))
                } else {
                    1.0 - (fTemp519
                        + fTemp76 * fTemp520
                        + (fTemp515 - (iTemp516) as F64)
                            * (fTemp518
                                - (fTemp519
                                    + fTemp76
                                        * (fTemp520
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp517, 8)) as usize]
                                            } - fTemp518)))))
                });
                let mut fTemp528: F64 = fTemp81 + fTemp514;
                let mut fTemp529: F64 = 131071.0 * (1.0 - fTemp528);
                let mut iTemp530: i32 = (fTemp529) as i32;
                let mut iTemp531: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp530, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp532: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp531, 7)) as usize] };
                let mut fTemp533: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp531 as usize] };
                let mut fTemp534: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp531, 1)) as usize] }
                        - fTemp533;
                let mut fTemp535: F64 = 131071.0 * fTemp528;
                let mut iTemp536: i32 = (fTemp535) as i32;
                let mut iTemp537: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp536, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp538: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp537, 7), 917503),
                    )) as usize]
                };
                let mut fTemp539: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp537 as usize] };
                let mut fTemp540: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp537, 1), 917503),
                    )) as usize]
                } - fTemp539;
                let mut iTemp541: i32 = (fTemp137
                    > (((if iTemp63 != 0 {
                        fTemp539
                            + fTemp76 * fTemp540
                            + (fTemp535 - (iTemp536) as F64)
                                * (fTemp538
                                    - (fTemp539
                                        + fTemp76
                                            * (fTemp540
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp537, 8),
                                                            917503,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp538))))
                    } else {
                        1.0 - (fTemp533
                            + fTemp76 * fTemp534
                            + (fTemp529 - (iTemp530) as F64)
                                * (fTemp532
                                    - (fTemp533
                                        + fTemp76
                                            * (fTemp534
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0
                                                        [(i32::wrapping_add(iTemp531, 8)) as usize]
                                                } - fTemp532)))))
                    }) - fTemp527)
                        / (1.0 - fTemp527))) as i32;
                let mut fTemp542: F64 = (if iTemp541 != 0 { fTemp511 } else { fTemp514 });
                let mut fTemp543: F64 = (if iTemp541 != 0 { fTemp514 } else { fTemp512 });
                let mut fTemp544: F64 = fTemp543 + fTemp542;
                let mut fTemp545: F64 = 0.5 * fTemp544;
                let mut fTemp546: F64 = 131071.0 * (1.0 - fTemp545);
                let mut iTemp547: i32 = (fTemp546) as i32;
                let mut iTemp548: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp547, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp549: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp548, 7)) as usize] };
                let mut fTemp550: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp548 as usize] };
                let mut fTemp551: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp548, 1)) as usize] }
                        - fTemp550;
                let mut fTemp552: F64 = 65535.5 * fTemp544;
                let mut iTemp553: i32 = (fTemp552) as i32;
                let mut iTemp554: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp553, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp555: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp554, 7)) as usize] };
                let mut fTemp556: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp554 as usize] };
                let mut fTemp557: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp554, 1)) as usize] }
                        - fTemp556;
                let mut fTemp558: F64 = (if iTemp63 != 0 {
                    fTemp556
                        + fTemp76 * fTemp557
                        + (fTemp552 - (iTemp553) as F64)
                            * (fTemp555
                                - (fTemp556
                                    + fTemp76
                                        * (fTemp557
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp554, 8)) as usize]
                                            } - fTemp555))))
                } else {
                    1.0 - (fTemp550
                        + fTemp76 * fTemp551
                        + (fTemp546 - (iTemp547) as F64)
                            * (fTemp549
                                - (fTemp550
                                    + fTemp76
                                        * (fTemp551
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp548, 8)) as usize]
                                            } - fTemp549)))))
                });
                let mut fTemp559: F64 = fTemp81 + fTemp545;
                let mut fTemp560: F64 = 131071.0 * (1.0 - fTemp559);
                let mut iTemp561: i32 = (fTemp560) as i32;
                let mut iTemp562: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp561, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp563: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp562, 7)) as usize] };
                let mut fTemp564: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp562 as usize] };
                let mut fTemp565: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp562, 1)) as usize] }
                        - fTemp564;
                let mut fTemp566: F64 = 131071.0 * fTemp559;
                let mut iTemp567: i32 = (fTemp566) as i32;
                let mut iTemp568: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp567, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp569: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp568, 7), 917503),
                    )) as usize]
                };
                let mut fTemp570: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp568 as usize] };
                let mut fTemp571: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp568, 1), 917503),
                    )) as usize]
                } - fTemp570;
                let mut iTemp572: i32 = (fTemp137
                    > (((if iTemp63 != 0 {
                        fTemp570
                            + fTemp76 * fTemp571
                            + (fTemp566 - (iTemp567) as F64)
                                * (fTemp569
                                    - (fTemp570
                                        + fTemp76
                                            * (fTemp571
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp568, 8),
                                                            917503,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp569))))
                    } else {
                        1.0 - (fTemp564
                            + fTemp76 * fTemp565
                            + (fTemp560 - (iTemp561) as F64)
                                * (fTemp563
                                    - (fTemp564
                                        + fTemp76
                                            * (fTemp565
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0
                                                        [(i32::wrapping_add(iTemp562, 8)) as usize]
                                                } - fTemp563)))))
                    }) - fTemp558)
                        / (1.0 - fTemp558))) as i32;
                let mut fTemp573: F64 = (if iTemp572 != 0 { fTemp542 } else { fTemp545 });
                let mut fTemp574: F64 = (if iTemp572 != 0 { fTemp545 } else { fTemp543 });
                let mut fTemp575: F64 = fTemp574 + fTemp573;
                let mut fTemp576: F64 = 0.5 * fTemp575;
                let mut fTemp577: F64 = 131071.0 * (1.0 - fTemp576);
                let mut iTemp578: i32 = (fTemp577) as i32;
                let mut iTemp579: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp578, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp580: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp579, 7)) as usize] };
                let mut fTemp581: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp579 as usize] };
                let mut fTemp582: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp579, 1)) as usize] }
                        - fTemp581;
                let mut fTemp583: F64 = 65535.5 * fTemp575;
                let mut iTemp584: i32 = (fTemp583) as i32;
                let mut iTemp585: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp584, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp586: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp585, 7)) as usize] };
                let mut fTemp587: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp585 as usize] };
                let mut fTemp588: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp585, 1)) as usize] }
                        - fTemp587;
                let mut fTemp589: F64 = (if iTemp63 != 0 {
                    fTemp587
                        + fTemp76 * fTemp588
                        + (fTemp583 - (iTemp584) as F64)
                            * (fTemp586
                                - (fTemp587
                                    + fTemp76
                                        * (fTemp588
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp585, 8)) as usize]
                                            } - fTemp586))))
                } else {
                    1.0 - (fTemp581
                        + fTemp76 * fTemp582
                        + (fTemp577 - (iTemp578) as F64)
                            * (fTemp580
                                - (fTemp581
                                    + fTemp76
                                        * (fTemp582
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp579, 8)) as usize]
                                            } - fTemp580)))))
                });
                let mut fTemp590: F64 = fTemp81 + fTemp576;
                let mut fTemp591: F64 = 131071.0 * (1.0 - fTemp590);
                let mut iTemp592: i32 = (fTemp591) as i32;
                let mut iTemp593: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp592, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp594: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp593, 7)) as usize] };
                let mut fTemp595: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp593 as usize] };
                let mut fTemp596: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp593, 1)) as usize] }
                        - fTemp595;
                let mut fTemp597: F64 = 131071.0 * fTemp590;
                let mut iTemp598: i32 = (fTemp597) as i32;
                let mut iTemp599: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp598, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp600: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp599, 7), 917503),
                    )) as usize]
                };
                let mut fTemp601: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp599 as usize] };
                let mut fTemp602: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp599, 1), 917503),
                    )) as usize]
                } - fTemp601;
                let mut iTemp603: i32 = (fTemp137
                    > (((if iTemp63 != 0 {
                        fTemp601
                            + fTemp76 * fTemp602
                            + (fTemp597 - (iTemp598) as F64)
                                * (fTemp600
                                    - (fTemp601
                                        + fTemp76
                                            * (fTemp602
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp599, 8),
                                                            917503,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp600))))
                    } else {
                        1.0 - (fTemp595
                            + fTemp76 * fTemp596
                            + (fTemp591 - (iTemp592) as F64)
                                * (fTemp594
                                    - (fTemp595
                                        + fTemp76
                                            * (fTemp596
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0
                                                        [(i32::wrapping_add(iTemp593, 8)) as usize]
                                                } - fTemp594)))))
                    }) - fTemp589)
                        / (1.0 - fTemp589))) as i32;
                let mut fTemp604: F64 = (if iTemp603 != 0 { fTemp573 } else { fTemp576 });
                let mut fTemp605: F64 = (if iTemp603 != 0 { fTemp576 } else { fTemp574 });
                let mut fTemp606: F64 = fTemp605 + fTemp604;
                let mut fTemp607: F64 = 0.5 * fTemp606;
                let mut fTemp608: F64 = 131071.0 * (1.0 - fTemp607);
                let mut iTemp609: i32 = (fTemp608) as i32;
                let mut iTemp610: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp609, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp611: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp610, 7)) as usize] };
                let mut fTemp612: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp610 as usize] };
                let mut fTemp613: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp610, 1)) as usize] }
                        - fTemp612;
                let mut fTemp614: F64 = 65535.5 * fTemp606;
                let mut iTemp615: i32 = (fTemp614) as i32;
                let mut iTemp616: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp615, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp617: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp616, 7)) as usize] };
                let mut fTemp618: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp616 as usize] };
                let mut fTemp619: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp616, 1)) as usize] }
                        - fTemp618;
                let mut fTemp620: F64 = (if iTemp63 != 0 {
                    fTemp618
                        + fTemp76 * fTemp619
                        + (fTemp614 - (iTemp615) as F64)
                            * (fTemp617
                                - (fTemp618
                                    + fTemp76
                                        * (fTemp619
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp616, 8),
                                                        917503,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp617))))
                } else {
                    1.0 - (fTemp612
                        + fTemp76 * fTemp613
                        + (fTemp608 - (iTemp609) as F64)
                            * (fTemp611
                                - (fTemp612
                                    + fTemp76
                                        * (fTemp613
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp610, 8),
                                                        917503,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp611)))))
                });
                let mut fTemp621: F64 = fTemp81 + fTemp607;
                let mut fTemp622: F64 = 131071.0 * (1.0 - fTemp621);
                let mut iTemp623: i32 = (fTemp622) as i32;
                let mut iTemp624: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp623, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp625: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp624, 7)) as usize] };
                let mut fTemp626: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp624 as usize] };
                let mut fTemp627: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp624, 1)) as usize] }
                        - fTemp626;
                let mut fTemp628: F64 = 131071.0 * fTemp621;
                let mut iTemp629: i32 = (fTemp628) as i32;
                let mut iTemp630: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp629, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp631: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp630, 7), 917503),
                    )) as usize]
                };
                let mut fTemp632: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp630 as usize] };
                let mut fTemp633: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp630, 1), 917503),
                    )) as usize]
                } - fTemp632;
                let mut iTemp634: i32 = (fTemp137
                    > (((if iTemp63 != 0 {
                        fTemp632
                            + fTemp76 * fTemp633
                            + (fTemp628 - (iTemp629) as F64)
                                * (fTemp631
                                    - (fTemp632
                                        + fTemp76
                                            * (fTemp633
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp630, 8),
                                                            917503,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp631))))
                    } else {
                        1.0 - (fTemp626
                            + fTemp76 * fTemp627
                            + (fTemp622 - (iTemp623) as F64)
                                * (fTemp625
                                    - (fTemp626
                                        + fTemp76
                                            * (fTemp627
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0
                                                        [(i32::wrapping_add(iTemp624, 8)) as usize]
                                                } - fTemp625)))))
                    }) - fTemp620)
                        / (1.0 - fTemp620))) as i32;
                let mut fTemp635: F64 = F64::min(
                    1.0,
                    F64::max(
                        0.0,
                        0.5 * ((if iTemp634 != 0 { fTemp607 } else { fTemp605 })
                            + (if iTemp634 != 0 { fTemp604 } else { fTemp607 })),
                    ),
                );
                self.fRec1[0] = fTemp635;
                let mut fTemp636: F64 = 131071.0 * (1.0 - fTemp635);
                let mut iTemp637: i32 = (fTemp636) as i32;
                let mut iTemp638: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp637, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp639: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp638, 7)) as usize] };
                let mut fTemp640: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp638 as usize] };
                let mut fTemp641: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp638, 1)) as usize] }
                        - fTemp640;
                let mut fTemp642: F64 = 131071.0 * fTemp635;
                let mut iTemp643: i32 = (fTemp642) as i32;
                let mut iTemp644: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp643, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp645: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp644, 7)) as usize] };
                let mut fTemp646: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp644 as usize] };
                let mut fTemp647: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp644, 1)) as usize] }
                        - fTemp646;
                let mut fTemp648: F64 = (if iTemp63 != 0 {
                    fTemp646
                        + fTemp76 * fTemp647
                        + (fTemp642 - (iTemp643) as F64)
                            * (fTemp645
                                - (fTemp646
                                    + fTemp76
                                        * (fTemp647
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp644, 8),
                                                        917503,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp645))))
                } else {
                    1.0 - (fTemp640
                        + fTemp76 * fTemp641
                        + (fTemp636 - (iTemp637) as F64)
                            * (fTemp639
                                - (fTemp640
                                    + fTemp76
                                        * (fTemp641
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp638, 8),
                                                        917503,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp639)))))
                });
                let mut fTemp649: F64 = fTemp81 + fTemp635;
                let mut fTemp650: F64 = 131071.0 * (1.0 - fTemp649);
                let mut iTemp651: i32 = (fTemp650) as i32;
                let mut iTemp652: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp651, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp653: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp652, 7)) as usize] };
                let mut fTemp654: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp652 as usize] };
                let mut fTemp655: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp652, 1)) as usize] }
                        - fTemp654;
                let mut fTemp656: F64 = 131071.0 * fTemp649;
                let mut iTemp657: i32 = (fTemp656) as i32;
                let mut iTemp658: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp71,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp657, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp659: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp658, 7), 917503),
                    )) as usize]
                };
                let mut fTemp660: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp658 as usize] };
                let mut fTemp661: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp658, 1), 917503),
                    )) as usize]
                } - fTemp660;
                let mut fTemp662: F64 = self.fRec2[1]
                    + (if ((0.001 * fTemp80) == 0.0) as i32 != 0 {
                        fTemp62
                    } else {
                        fTemp62
                            * ((if iTemp63 != 0 {
                                fTemp660
                                    + fTemp76 * fTemp661
                                    + (fTemp656 - (iTemp657) as F64)
                                        * (fTemp659
                                            - (fTemp660
                                                + fTemp76
                                                    * (fTemp661
                                                        - (unsafe {
                                                            ftbl0LambRs48kSIG0[(std::cmp::max(
                                                                0,
                                                                std::cmp::min(
                                                                    i32::wrapping_add(iTemp658, 8),
                                                                    917503,
                                                                ),
                                                            ))
                                                                as usize]
                                                        } - fTemp659))))
                            } else {
                                1.0 - (fTemp654
                                    + fTemp76 * fTemp655
                                    + (fTemp650 - (iTemp651) as F64)
                                        * (fTemp653
                                            - (fTemp654
                                                + fTemp76
                                                    * (fTemp655
                                                        - (unsafe {
                                                            ftbl0LambRs48kSIG0[(i32::wrapping_add(
                                                                iTemp652, 8,
                                                            ))
                                                                as usize]
                                                        } - fTemp653)))))
                            }) - fTemp648)
                            / (1.0 - fTemp648)
                    });
                self.fRec2[0] = (if iTemp79 != 0 {
                    F64::min(fTemp662, self.fRec2[1])
                } else {
                    F64::max(fTemp662, self.fRec2[1])
                });
                self.fVec31[(self.IOTA0 & 8191) as usize] = F64::powf(1e+01, 0.05 * self.fRec2[0]);
                let mut fTemp663: F64 =
                    self.fVec31[((i32::wrapping_sub(self.IOTA0, iSlow70)) & 8191) as usize];
                self.fRec14[0] = fSlow72 + self.fConst4 * self.fRec14[1];
                *output0 = 0.5
                    * self.fVec1[((i32::wrapping_sub(self.IOTA0, iSlow71)) & 32767) as usize]
                    * fTemp2
                    + self.fRec14[0]
                        * self.fVec2[((i32::wrapping_sub(self.IOTA0, iSlow71)) & 32767) as usize]
                        * fTemp663
                        * fTemp4;
                let mut fTemp664: F64 = fTemp36 + fSlow17 * (fTemp37 - fTemp36);
                let mut iTemp665: i32 =
                    ((fTemp664 > fSlow11) as i32) + ((fTemp664 > fSlow9) as i32);
                let mut fTemp666: F64 = fTemp664 - fSlow8;
                let mut fTemp667: F64 = F64::min(
                    fTemp34,
                    -(fSlow18
                        * F64::max(
                            0.0,
                            (if (iTemp665 == 0) as i32 != 0 {
                                0.0
                            } else {
                                (if (iTemp665 == 1) as i32 != 0 {
                                    fSlow12 * LambRs48k_faustpower2_f(fSlow7 + fTemp666)
                                } else {
                                    fTemp666
                                })
                            }),
                        )),
                );
                self.fVec32[(self.IOTA0 & 16383) as usize] = fTemp667;
                let mut fTemp668: F64 = F64::min(
                    fTemp667,
                    self.fVec32[((i32::wrapping_sub(self.IOTA0, 1)) & 16383) as usize],
                );
                self.fVec33[0] = fTemp668;
                let mut fTemp669: F64 = F64::min(fTemp668, self.fVec33[2]);
                self.fVec34[0] = fTemp669;
                let mut fTemp670: F64 = F64::min(fTemp669, self.fVec34[4]);
                self.fVec35[0] = fTemp670;
                let mut fTemp671: F64 = F64::min(fTemp670, self.fVec35[8]);
                self.fVec36[(self.IOTA0 & 31) as usize] = fTemp671;
                let mut fTemp672: F64 = F64::min(
                    fTemp671,
                    self.fVec36[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize],
                );
                self.fVec37[(self.IOTA0 & 63) as usize] = fTemp672;
                let mut fTemp673: F64 = F64::min(
                    fTemp672,
                    self.fVec37[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize],
                );
                self.fVec38[(self.IOTA0 & 127) as usize] = fTemp673;
                let mut fTemp674: F64 = F64::min(
                    fTemp673,
                    self.fVec38[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize],
                );
                self.fVec39[(self.IOTA0 & 255) as usize] = fTemp674;
                let mut fTemp675: F64 = F64::min(
                    fTemp674,
                    self.fVec39[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize],
                );
                self.fVec40[(self.IOTA0 & 511) as usize] = fTemp675;
                let mut fTemp676: F64 = F64::min(
                    fTemp675,
                    self.fVec40[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize],
                );
                self.fVec41[(self.IOTA0 & 1023) as usize] = fTemp676;
                let mut fTemp677: F64 = F64::min(
                    fTemp676,
                    self.fVec41[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize],
                );
                self.fVec42[(self.IOTA0 & 2047) as usize] = fTemp677;
                self.fVec43[(self.IOTA0 & 4095) as usize] = F64::min(
                    fTemp677,
                    self.fVec42[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize],
                );
                self.fRec17[0] = F64::max(
                    F64::min(
                        self.fRec17[1],
                        self.fVec32[((i32::wrapping_sub(self.IOTA0, iSlow21)) & 16383) as usize],
                    ),
                    F64::min(
                        F64::min(
                            F64::min(
                                F64::min(
                                    F64::min(
                                        F64::min(
                                            F64::min(
                                                F64::min(
                                                    F64::min(
                                                        F64::min(
                                                            F64::min(
                                                                (if iSlow23 != 0 {
                                                                    fTemp667
                                                                } else {
                                                                    1.7976931348623157e+308
                                                                }),
                                                                (if iSlow24 != 0 {
                                                                    self.fVec33[iSlow23 as usize]
                                                                } else {
                                                                    1.7976931348623157e+308
                                                                }),
                                                            ),
                                                            (if iSlow25 != 0 {
                                                                self.fVec34[iSlow26 as usize]
                                                            } else {
                                                                1.7976931348623157e+308
                                                            }),
                                                        ),
                                                        (if iSlow27 != 0 {
                                                            self.fVec35[iSlow28 as usize]
                                                        } else {
                                                            1.7976931348623157e+308
                                                        }),
                                                    ),
                                                    (if iSlow29 != 0 {
                                                        self.fVec36[((i32::wrapping_sub(
                                                            self.IOTA0, iSlow30,
                                                        )) & 31)
                                                            as usize]
                                                    } else {
                                                        1.7976931348623157e+308
                                                    }),
                                                ),
                                                (if iSlow31 != 0 {
                                                    self.fVec37[((i32::wrapping_sub(
                                                        self.IOTA0, iSlow32,
                                                    )) & 63)
                                                        as usize]
                                                } else {
                                                    1.7976931348623157e+308
                                                }),
                                            ),
                                            (if iSlow33 != 0 {
                                                self.fVec38[((i32::wrapping_sub(
                                                    self.IOTA0, iSlow34,
                                                )) & 127)
                                                    as usize]
                                            } else {
                                                1.7976931348623157e+308
                                            }),
                                        ),
                                        (if iSlow35 != 0 {
                                            self.fVec39[((i32::wrapping_sub(self.IOTA0, iSlow36))
                                                & 255)
                                                as usize]
                                        } else {
                                            1.7976931348623157e+308
                                        }),
                                    ),
                                    (if iSlow37 != 0 {
                                        self.fVec40[((i32::wrapping_sub(self.IOTA0, iSlow38)) & 511)
                                            as usize]
                                    } else {
                                        1.7976931348623157e+308
                                    }),
                                ),
                                (if iSlow39 != 0 {
                                    self.fVec41
                                        [((i32::wrapping_sub(self.IOTA0, iSlow40)) & 1023) as usize]
                                } else {
                                    1.7976931348623157e+308
                                }),
                            ),
                            (if iSlow41 != 0 {
                                self.fVec42
                                    [((i32::wrapping_sub(self.IOTA0, iSlow42)) & 2047) as usize]
                            } else {
                                1.7976931348623157e+308
                            }),
                        ),
                        (if iSlow43 != 0 {
                            self.fVec43[((i32::wrapping_sub(self.IOTA0, iSlow44)) & 4095) as usize]
                        } else {
                            1.7976931348623157e+308
                        }),
                    ),
                );
                let mut fTemp678: F64 = F64::min(self.fRec17[0], self.fRec17[1]);
                self.fVec44[0] = fTemp678;
                let mut fTemp679: F64 = F64::min(fTemp678, self.fVec44[2]);
                self.fVec45[0] = fTemp679;
                let mut fTemp680: F64 = F64::min(fTemp679, self.fVec45[4]);
                self.fVec46[0] = fTemp680;
                let mut fTemp681: F64 = F64::min(fTemp680, self.fVec46[8]);
                self.fVec47[(self.IOTA0 & 31) as usize] = fTemp681;
                let mut fTemp682: F64 = F64::min(
                    fTemp681,
                    self.fVec47[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize],
                );
                self.fVec48[(self.IOTA0 & 63) as usize] = fTemp682;
                let mut fTemp683: F64 = F64::min(
                    fTemp682,
                    self.fVec48[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize],
                );
                self.fVec49[(self.IOTA0 & 127) as usize] = fTemp683;
                let mut fTemp684: F64 = F64::min(
                    fTemp683,
                    self.fVec49[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize],
                );
                self.fVec50[(self.IOTA0 & 255) as usize] = fTemp684;
                let mut fTemp685: F64 = F64::min(
                    fTemp684,
                    self.fVec50[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize],
                );
                self.fVec51[(self.IOTA0 & 511) as usize] = fTemp685;
                let mut fTemp686: F64 = F64::min(
                    fTemp685,
                    self.fVec51[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize],
                );
                self.fVec52[(self.IOTA0 & 1023) as usize] = fTemp686;
                let mut fTemp687: F64 = F64::min(
                    fTemp686,
                    self.fVec52[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize],
                );
                self.fVec53[(self.IOTA0 & 2047) as usize] = fTemp687;
                self.fVec54[(self.IOTA0 & 4095) as usize] = F64::min(
                    fTemp687,
                    self.fVec53[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize],
                );
                let mut fTemp688: F64 = F64::min(
                    F64::min(
                        F64::min(
                            F64::min(
                                F64::min(
                                    F64::min(
                                        F64::min(
                                            F64::min(
                                                F64::min(
                                                    F64::min(
                                                        F64::min(
                                                            (if iSlow4 != 0 {
                                                                self.fRec17[0]
                                                            } else {
                                                                1.7976931348623157e+308
                                                            }),
                                                            (if iSlow45 != 0 {
                                                                self.fVec44[iSlow4 as usize]
                                                            } else {
                                                                1.7976931348623157e+308
                                                            }),
                                                        ),
                                                        (if iSlow46 != 0 {
                                                            self.fVec45[iSlow47 as usize]
                                                        } else {
                                                            1.7976931348623157e+308
                                                        }),
                                                    ),
                                                    (if iSlow48 != 0 {
                                                        self.fVec46[iSlow49 as usize]
                                                    } else {
                                                        1.7976931348623157e+308
                                                    }),
                                                ),
                                                (if iSlow50 != 0 {
                                                    self.fVec47[((i32::wrapping_sub(
                                                        self.IOTA0, iSlow51,
                                                    )) & 31)
                                                        as usize]
                                                } else {
                                                    1.7976931348623157e+308
                                                }),
                                            ),
                                            (if iSlow52 != 0 {
                                                self.fVec48[((i32::wrapping_sub(
                                                    self.IOTA0, iSlow53,
                                                )) & 63)
                                                    as usize]
                                            } else {
                                                1.7976931348623157e+308
                                            }),
                                        ),
                                        (if iSlow54 != 0 {
                                            self.fVec49[((i32::wrapping_sub(self.IOTA0, iSlow55))
                                                & 127)
                                                as usize]
                                        } else {
                                            1.7976931348623157e+308
                                        }),
                                    ),
                                    (if iSlow56 != 0 {
                                        self.fVec50[((i32::wrapping_sub(self.IOTA0, iSlow57)) & 255)
                                            as usize]
                                    } else {
                                        1.7976931348623157e+308
                                    }),
                                ),
                                (if iSlow58 != 0 {
                                    self.fVec51
                                        [((i32::wrapping_sub(self.IOTA0, iSlow59)) & 511) as usize]
                                } else {
                                    1.7976931348623157e+308
                                }),
                            ),
                            (if iSlow60 != 0 {
                                self.fVec52
                                    [((i32::wrapping_sub(self.IOTA0, iSlow61)) & 1023) as usize]
                            } else {
                                1.7976931348623157e+308
                            }),
                        ),
                        (if iSlow62 != 0 {
                            self.fVec53[((i32::wrapping_sub(self.IOTA0, iSlow63)) & 2047) as usize]
                        } else {
                            1.7976931348623157e+308
                        }),
                    ),
                    (if iSlow64 != 0 {
                        self.fVec54[((i32::wrapping_sub(self.IOTA0, iSlow65)) & 4095) as usize]
                    } else {
                        1.7976931348623157e+308
                    }),
                ) - self.fRec16[1];
                self.fVec55[0] = fTemp688;
                let mut iTemp689: i32 = (fTemp688 > 0.0) as i32;
                let mut fTemp690: F64 = (if iTemp689 != 0 { fSlow67 } else { fSlow66 });
                self.fVec56[0] = fTemp690;
                let mut fTemp691: F64 = 6.0 * fTemp690;
                let mut iTemp692: i32 = (fTemp691) as i32;
                let mut iTemp693: i32 = std::cmp::max(0, std::cmp::min(iTemp692, 6));
                let mut iTemp694: i32 = std::cmp::max(
                    0,
                    std::cmp::min(i32::wrapping_add(iTemp693, 458745), 917503),
                );
                let mut fTemp695: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp694, 7)) as usize] };
                let mut fTemp696: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp694 as usize] };
                let mut fTemp697: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp694, 1)) as usize] }
                        - fTemp696;
                let mut fTemp698: F64 = fTemp691 - (iTemp692) as F64;
                let mut fTemp699: F64 = fTemp696
                    + fTemp698 * fTemp697
                    + 0.5
                        * (fTemp695
                            - (fTemp696
                                + fTemp698
                                    * (fTemp697
                                        - (unsafe {
                                            ftbl0LambRs48kSIG0
                                                [(i32::wrapping_add(iTemp694, 8)) as usize]
                                        } - fTemp695))));
                let mut fTemp700: F64 = (if iTemp689 != 0 {
                    fTemp699
                } else {
                    1.0 - fTemp699
                });
                let mut iTemp701: i32 = (fTemp688 < 0.0) as i32;
                let mut fTemp702: F64 = fSlow1 * (iTemp701) as F64 + fSlow13 * (iTemp689) as F64;
                self.fVec57[0] = fTemp702;
                let mut fTemp703: F64 = self.fConst10 / fTemp702;
                let mut fTemp704: F64 = fTemp703 + 0.5;
                let mut fTemp705: F64 = 131071.0 * (1.0 - fTemp704);
                let mut iTemp706: i32 = (fTemp705) as i32;
                let mut iTemp707: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp706, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp708: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp707, 7)) as usize] };
                let mut fTemp709: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp707 as usize] };
                let mut fTemp710: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp707, 1)) as usize] }
                        - fTemp709;
                let mut fTemp711: F64 = 131071.0 * fTemp704;
                let mut iTemp712: i32 = (fTemp711) as i32;
                let mut iTemp713: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp712, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp714: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp713, 7), 917503),
                    )) as usize]
                };
                let mut fTemp715: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp713 as usize] };
                let mut fTemp716: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp713, 1), 917503),
                    )) as usize]
                } - fTemp715;
                let mut fTemp717: F64 = 6.0 * self.fVec56[1];
                let mut iTemp718: i32 = (fTemp717) as i32;
                let mut iTemp719: i32 = std::cmp::max(0, std::cmp::min(iTemp718, 6));
                let mut fTemp720: F64 = 131071.0 * (1.0 - self.fRec15[1]);
                let mut iTemp721: i32 = (fTemp720) as i32;
                let mut iTemp722: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp721, 131071))),
                            iTemp719,
                        ),
                        917503,
                    ),
                );
                let mut fTemp723: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp722, 7), 917503),
                    )) as usize]
                };
                let mut fTemp724: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp722 as usize] };
                let mut fTemp725: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp722, 1), 917503),
                    )) as usize]
                } - fTemp724;
                let mut fTemp726: F64 = fTemp717 - (iTemp718) as F64;
                let mut fTemp727: F64 = 131071.0 * self.fRec15[1];
                let mut iTemp728: i32 = (fTemp727) as i32;
                let mut iTemp729: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp719,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp728, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp730: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp729, 7), 917503),
                    )) as usize]
                };
                let mut fTemp731: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp729 as usize] };
                let mut fTemp732: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp729, 1), 917503),
                    )) as usize]
                } - fTemp731;
                let mut fTemp733: F64 = self.fRec15[1] + fTemp703;
                let mut fTemp734: F64 = 131071.0 * (1.0 - fTemp733);
                let mut iTemp735: i32 = (fTemp734) as i32;
                let mut iTemp736: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp735, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp737: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp736, 7)) as usize] };
                let mut fTemp738: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp736 as usize] };
                let mut fTemp739: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp736, 1)) as usize] }
                        - fTemp738;
                let mut fTemp740: F64 = 131071.0 * fTemp733;
                let mut iTemp741: i32 = (fTemp740) as i32;
                let mut iTemp742: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp741, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp743: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp742, 7), 917503),
                    )) as usize]
                };
                let mut fTemp744: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp742 as usize] };
                let mut fTemp745: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp742, 1), 917503),
                    )) as usize]
                } - fTemp744;
                let mut fTemp746: F64 =
                    self.fRec15[1] + self.fConst10 * (1.0 / fTemp702 + 1.0 / self.fVec57[1]);
                let mut fTemp747: F64 = 131071.0 * (1.0 - fTemp746);
                let mut iTemp748: i32 = (fTemp747) as i32;
                let mut iTemp749: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp748, 131071))),
                            iTemp693,
                        ),
                        917503,
                    ),
                );
                let mut fTemp750: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp749, 7)) as usize] };
                let mut fTemp751: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp749 as usize] };
                let mut fTemp752: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp749, 1)) as usize] }
                        - fTemp751;
                let mut fTemp753: F64 = 131071.0 * fTemp746;
                let mut iTemp754: i32 = (fTemp753) as i32;
                let mut iTemp755: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp754, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp756: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp755, 7), 917503),
                    )) as usize]
                };
                let mut fTemp757: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp755 as usize] };
                let mut fTemp758: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp755, 1), 917503),
                    )) as usize]
                } - fTemp757;
                let mut fTemp759: F64 = ((if iTemp689 != 0 {
                    fTemp757
                        + fTemp698 * fTemp758
                        + (fTemp753 - (iTemp754) as F64)
                            * (fTemp756
                                - (fTemp757
                                    + fTemp698
                                        * (fTemp758
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp755, 8),
                                                        917503,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp756))))
                } else {
                    1.0 - (fTemp751
                        + fTemp698 * fTemp752
                        + (fTemp747 - (iTemp748) as F64)
                            * (fTemp750
                                - (fTemp751
                                    + fTemp698
                                        * (fTemp752
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp749, 8)) as usize]
                                            } - fTemp750)))))
                }) - (if iTemp689 != 0 {
                    fTemp744
                        + fTemp698 * fTemp745
                        + (fTemp740 - (iTemp741) as F64)
                            * (fTemp743
                                - (fTemp744
                                    + fTemp698
                                        * (fTemp745
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp742, 8),
                                                        917503,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp743))))
                } else {
                    1.0 - (fTemp738
                        + fTemp698 * fTemp739
                        + (fTemp734 - (iTemp735) as F64)
                            * (fTemp737
                                - (fTemp738
                                    + fTemp698
                                        * (fTemp739
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp736, 8)) as usize]
                                            } - fTemp737)))))
                })) * self.fVec55[1]
                    / (fTemp688
                        * (1.0
                            - (if iTemp689 != 0 {
                                fTemp731
                                    + fTemp726 * fTemp732
                                    + (fTemp727 - (iTemp728) as F64)
                                        * (fTemp730
                                            - (fTemp731
                                                + fTemp726
                                                    * (fTemp732
                                                        - (unsafe {
                                                            ftbl0LambRs48kSIG0[(std::cmp::max(
                                                                0,
                                                                std::cmp::min(
                                                                    i32::wrapping_add(iTemp729, 8),
                                                                    917503,
                                                                ),
                                                            ))
                                                                as usize]
                                                        } - fTemp730))))
                            } else {
                                1.0 - (fTemp724
                                    + fTemp726 * fTemp725
                                    + (fTemp720 - (iTemp721) as F64)
                                        * (fTemp723
                                            - (fTemp724
                                                + fTemp726
                                                    * (fTemp725
                                                        - (unsafe {
                                                            ftbl0LambRs48kSIG0[(std::cmp::max(
                                                                0,
                                                                std::cmp::min(
                                                                    i32::wrapping_add(iTemp722, 8),
                                                                    917503,
                                                                ),
                                                            ))
                                                                as usize]
                                                        } - fTemp723)))))
                            })));
                let mut iTemp760: i32 = (fTemp759
                    > (((if iTemp689 != 0 {
                        fTemp715
                            + fTemp698 * fTemp716
                            + (fTemp711 - (iTemp712) as F64)
                                * (fTemp714
                                    - (fTemp715
                                        + fTemp698
                                            * (fTemp716
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp713, 8),
                                                            917503,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp714))))
                    } else {
                        1.0 - (fTemp709
                            + fTemp698 * fTemp710
                            + (fTemp705 - (iTemp706) as F64)
                                * (fTemp708
                                    - (fTemp709
                                        + fTemp698
                                            * (fTemp710
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0
                                                        [(i32::wrapping_add(iTemp707, 8)) as usize]
                                                } - fTemp708)))))
                    }) - fTemp700)
                        / (1.0 - fTemp700))) as i32;
                let mut fTemp761: F64 = (if iTemp760 != 0 { 1.0 } else { 0.5 });
                let mut fTemp762: F64 = (if iTemp760 != 0 { 0.5 } else { 0.0 });
                let mut fTemp763: F64 = fTemp762 + fTemp761;
                let mut fTemp764: F64 = 0.5 * fTemp763;
                let mut fTemp765: F64 = 131071.0 * (1.0 - fTemp764);
                let mut iTemp766: i32 = (fTemp765) as i32;
                let mut iTemp767: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp766, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp768: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp767, 7)) as usize] };
                let mut fTemp769: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp767 as usize] };
                let mut fTemp770: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp767, 1)) as usize] }
                        - fTemp769;
                let mut fTemp771: F64 = 65535.5 * fTemp763;
                let mut iTemp772: i32 = (fTemp771) as i32;
                let mut iTemp773: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp772, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp774: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp773, 7)) as usize] };
                let mut fTemp775: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp773 as usize] };
                let mut fTemp776: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp773, 1)) as usize] }
                        - fTemp775;
                let mut fTemp777: F64 = (if iTemp689 != 0 {
                    fTemp775
                        + fTemp698 * fTemp776
                        + (fTemp771 - (iTemp772) as F64)
                            * (fTemp774
                                - (fTemp775
                                    + fTemp698
                                        * (fTemp776
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp773, 8)) as usize]
                                            } - fTemp774))))
                } else {
                    1.0 - (fTemp769
                        + fTemp698 * fTemp770
                        + (fTemp765 - (iTemp766) as F64)
                            * (fTemp768
                                - (fTemp769
                                    + fTemp698
                                        * (fTemp770
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp767, 8)) as usize]
                                            } - fTemp768)))))
                });
                let mut fTemp778: F64 = fTemp703 + fTemp764;
                let mut fTemp779: F64 = 131071.0 * (1.0 - fTemp778);
                let mut iTemp780: i32 = (fTemp779) as i32;
                let mut iTemp781: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp780, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp782: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp781, 7)) as usize] };
                let mut fTemp783: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp781 as usize] };
                let mut fTemp784: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp781, 1)) as usize] }
                        - fTemp783;
                let mut fTemp785: F64 = 131071.0 * fTemp778;
                let mut iTemp786: i32 = (fTemp785) as i32;
                let mut iTemp787: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp786, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp788: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp787, 7), 917503),
                    )) as usize]
                };
                let mut fTemp789: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp787 as usize] };
                let mut fTemp790: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp787, 1), 917503),
                    )) as usize]
                } - fTemp789;
                let mut iTemp791: i32 = (fTemp759
                    > (((if iTemp689 != 0 {
                        fTemp789
                            + fTemp698 * fTemp790
                            + (fTemp785 - (iTemp786) as F64)
                                * (fTemp788
                                    - (fTemp789
                                        + fTemp698
                                            * (fTemp790
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp787, 8),
                                                            917503,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp788))))
                    } else {
                        1.0 - (fTemp783
                            + fTemp698 * fTemp784
                            + (fTemp779 - (iTemp780) as F64)
                                * (fTemp782
                                    - (fTemp783
                                        + fTemp698
                                            * (fTemp784
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0
                                                        [(i32::wrapping_add(iTemp781, 8)) as usize]
                                                } - fTemp782)))))
                    }) - fTemp777)
                        / (1.0 - fTemp777))) as i32;
                let mut fTemp792: F64 = (if iTemp791 != 0 { fTemp761 } else { fTemp764 });
                let mut fTemp793: F64 = (if iTemp791 != 0 { fTemp764 } else { fTemp762 });
                let mut fTemp794: F64 = fTemp793 + fTemp792;
                let mut fTemp795: F64 = 0.5 * fTemp794;
                let mut fTemp796: F64 = 131071.0 * (1.0 - fTemp795);
                let mut iTemp797: i32 = (fTemp796) as i32;
                let mut iTemp798: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp797, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp799: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp798, 7)) as usize] };
                let mut fTemp800: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp798 as usize] };
                let mut fTemp801: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp798, 1)) as usize] }
                        - fTemp800;
                let mut fTemp802: F64 = 65535.5 * fTemp794;
                let mut iTemp803: i32 = (fTemp802) as i32;
                let mut iTemp804: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp803, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp805: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp804, 7)) as usize] };
                let mut fTemp806: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp804 as usize] };
                let mut fTemp807: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp804, 1)) as usize] }
                        - fTemp806;
                let mut fTemp808: F64 = (if iTemp689 != 0 {
                    fTemp806
                        + fTemp698 * fTemp807
                        + (fTemp802 - (iTemp803) as F64)
                            * (fTemp805
                                - (fTemp806
                                    + fTemp698
                                        * (fTemp807
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp804, 8)) as usize]
                                            } - fTemp805))))
                } else {
                    1.0 - (fTemp800
                        + fTemp698 * fTemp801
                        + (fTemp796 - (iTemp797) as F64)
                            * (fTemp799
                                - (fTemp800
                                    + fTemp698
                                        * (fTemp801
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp798, 8)) as usize]
                                            } - fTemp799)))))
                });
                let mut fTemp809: F64 = fTemp703 + fTemp795;
                let mut fTemp810: F64 = 131071.0 * (1.0 - fTemp809);
                let mut iTemp811: i32 = (fTemp810) as i32;
                let mut iTemp812: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp811, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp813: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp812, 7)) as usize] };
                let mut fTemp814: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp812 as usize] };
                let mut fTemp815: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp812, 1)) as usize] }
                        - fTemp814;
                let mut fTemp816: F64 = 131071.0 * fTemp809;
                let mut iTemp817: i32 = (fTemp816) as i32;
                let mut iTemp818: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp817, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp819: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp818, 7), 917503),
                    )) as usize]
                };
                let mut fTemp820: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp818 as usize] };
                let mut fTemp821: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp818, 1), 917503),
                    )) as usize]
                } - fTemp820;
                let mut iTemp822: i32 = (fTemp759
                    > (((if iTemp689 != 0 {
                        fTemp820
                            + fTemp698 * fTemp821
                            + (fTemp816 - (iTemp817) as F64)
                                * (fTemp819
                                    - (fTemp820
                                        + fTemp698
                                            * (fTemp821
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp818, 8),
                                                            917503,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp819))))
                    } else {
                        1.0 - (fTemp814
                            + fTemp698 * fTemp815
                            + (fTemp810 - (iTemp811) as F64)
                                * (fTemp813
                                    - (fTemp814
                                        + fTemp698
                                            * (fTemp815
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0
                                                        [(i32::wrapping_add(iTemp812, 8)) as usize]
                                                } - fTemp813)))))
                    }) - fTemp808)
                        / (1.0 - fTemp808))) as i32;
                let mut fTemp823: F64 = (if iTemp822 != 0 { fTemp792 } else { fTemp795 });
                let mut fTemp824: F64 = (if iTemp822 != 0 { fTemp795 } else { fTemp793 });
                let mut fTemp825: F64 = fTemp824 + fTemp823;
                let mut fTemp826: F64 = 0.5 * fTemp825;
                let mut fTemp827: F64 = 131071.0 * (1.0 - fTemp826);
                let mut iTemp828: i32 = (fTemp827) as i32;
                let mut iTemp829: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp828, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp830: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp829, 7)) as usize] };
                let mut fTemp831: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp829 as usize] };
                let mut fTemp832: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp829, 1)) as usize] }
                        - fTemp831;
                let mut fTemp833: F64 = 65535.5 * fTemp825;
                let mut iTemp834: i32 = (fTemp833) as i32;
                let mut iTemp835: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp834, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp836: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp835, 7)) as usize] };
                let mut fTemp837: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp835 as usize] };
                let mut fTemp838: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp835, 1)) as usize] }
                        - fTemp837;
                let mut fTemp839: F64 = (if iTemp689 != 0 {
                    fTemp837
                        + fTemp698 * fTemp838
                        + (fTemp833 - (iTemp834) as F64)
                            * (fTemp836
                                - (fTemp837
                                    + fTemp698
                                        * (fTemp838
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp835, 8)) as usize]
                                            } - fTemp836))))
                } else {
                    1.0 - (fTemp831
                        + fTemp698 * fTemp832
                        + (fTemp827 - (iTemp828) as F64)
                            * (fTemp830
                                - (fTemp831
                                    + fTemp698
                                        * (fTemp832
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp829, 8)) as usize]
                                            } - fTemp830)))))
                });
                let mut fTemp840: F64 = fTemp703 + fTemp826;
                let mut fTemp841: F64 = 131071.0 * (1.0 - fTemp840);
                let mut iTemp842: i32 = (fTemp841) as i32;
                let mut iTemp843: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp842, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp844: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp843, 7)) as usize] };
                let mut fTemp845: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp843 as usize] };
                let mut fTemp846: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp843, 1)) as usize] }
                        - fTemp845;
                let mut fTemp847: F64 = 131071.0 * fTemp840;
                let mut iTemp848: i32 = (fTemp847) as i32;
                let mut iTemp849: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp848, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp850: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp849, 7), 917503),
                    )) as usize]
                };
                let mut fTemp851: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp849 as usize] };
                let mut fTemp852: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp849, 1), 917503),
                    )) as usize]
                } - fTemp851;
                let mut iTemp853: i32 = (fTemp759
                    > (((if iTemp689 != 0 {
                        fTemp851
                            + fTemp698 * fTemp852
                            + (fTemp847 - (iTemp848) as F64)
                                * (fTemp850
                                    - (fTemp851
                                        + fTemp698
                                            * (fTemp852
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp849, 8),
                                                            917503,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp850))))
                    } else {
                        1.0 - (fTemp845
                            + fTemp698 * fTemp846
                            + (fTemp841 - (iTemp842) as F64)
                                * (fTemp844
                                    - (fTemp845
                                        + fTemp698
                                            * (fTemp846
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0
                                                        [(i32::wrapping_add(iTemp843, 8)) as usize]
                                                } - fTemp844)))))
                    }) - fTemp839)
                        / (1.0 - fTemp839))) as i32;
                let mut fTemp854: F64 = (if iTemp853 != 0 { fTemp823 } else { fTemp826 });
                let mut fTemp855: F64 = (if iTemp853 != 0 { fTemp826 } else { fTemp824 });
                let mut fTemp856: F64 = fTemp855 + fTemp854;
                let mut fTemp857: F64 = 0.5 * fTemp856;
                let mut fTemp858: F64 = 131071.0 * (1.0 - fTemp857);
                let mut iTemp859: i32 = (fTemp858) as i32;
                let mut iTemp860: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp859, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp861: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp860, 7)) as usize] };
                let mut fTemp862: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp860 as usize] };
                let mut fTemp863: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp860, 1)) as usize] }
                        - fTemp862;
                let mut fTemp864: F64 = 65535.5 * fTemp856;
                let mut iTemp865: i32 = (fTemp864) as i32;
                let mut iTemp866: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp865, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp867: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp866, 7)) as usize] };
                let mut fTemp868: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp866 as usize] };
                let mut fTemp869: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp866, 1)) as usize] }
                        - fTemp868;
                let mut fTemp870: F64 = (if iTemp689 != 0 {
                    fTemp868
                        + fTemp698 * fTemp869
                        + (fTemp864 - (iTemp865) as F64)
                            * (fTemp867
                                - (fTemp868
                                    + fTemp698
                                        * (fTemp869
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp866, 8)) as usize]
                                            } - fTemp867))))
                } else {
                    1.0 - (fTemp862
                        + fTemp698 * fTemp863
                        + (fTemp858 - (iTemp859) as F64)
                            * (fTemp861
                                - (fTemp862
                                    + fTemp698
                                        * (fTemp863
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp860, 8)) as usize]
                                            } - fTemp861)))))
                });
                let mut fTemp871: F64 = fTemp703 + fTemp857;
                let mut fTemp872: F64 = 131071.0 * (1.0 - fTemp871);
                let mut iTemp873: i32 = (fTemp872) as i32;
                let mut iTemp874: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp873, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp875: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp874, 7)) as usize] };
                let mut fTemp876: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp874 as usize] };
                let mut fTemp877: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp874, 1)) as usize] }
                        - fTemp876;
                let mut fTemp878: F64 = 131071.0 * fTemp871;
                let mut iTemp879: i32 = (fTemp878) as i32;
                let mut iTemp880: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp879, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp881: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp880, 7), 917503),
                    )) as usize]
                };
                let mut fTemp882: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp880 as usize] };
                let mut fTemp883: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp880, 1), 917503),
                    )) as usize]
                } - fTemp882;
                let mut iTemp884: i32 = (fTemp759
                    > (((if iTemp689 != 0 {
                        fTemp882
                            + fTemp698 * fTemp883
                            + (fTemp878 - (iTemp879) as F64)
                                * (fTemp881
                                    - (fTemp882
                                        + fTemp698
                                            * (fTemp883
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp880, 8),
                                                            917503,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp881))))
                    } else {
                        1.0 - (fTemp876
                            + fTemp698 * fTemp877
                            + (fTemp872 - (iTemp873) as F64)
                                * (fTemp875
                                    - (fTemp876
                                        + fTemp698
                                            * (fTemp877
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0
                                                        [(i32::wrapping_add(iTemp874, 8)) as usize]
                                                } - fTemp875)))))
                    }) - fTemp870)
                        / (1.0 - fTemp870))) as i32;
                let mut fTemp885: F64 = (if iTemp884 != 0 { fTemp854 } else { fTemp857 });
                let mut fTemp886: F64 = (if iTemp884 != 0 { fTemp857 } else { fTemp855 });
                let mut fTemp887: F64 = fTemp886 + fTemp885;
                let mut fTemp888: F64 = 0.5 * fTemp887;
                let mut fTemp889: F64 = 131071.0 * (1.0 - fTemp888);
                let mut iTemp890: i32 = (fTemp889) as i32;
                let mut iTemp891: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp890, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp892: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp891, 7)) as usize] };
                let mut fTemp893: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp891 as usize] };
                let mut fTemp894: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp891, 1)) as usize] }
                        - fTemp893;
                let mut fTemp895: F64 = 65535.5 * fTemp887;
                let mut iTemp896: i32 = (fTemp895) as i32;
                let mut iTemp897: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp896, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp898: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp897, 7)) as usize] };
                let mut fTemp899: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp897 as usize] };
                let mut fTemp900: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp897, 1)) as usize] }
                        - fTemp899;
                let mut fTemp901: F64 = (if iTemp689 != 0 {
                    fTemp899
                        + fTemp698 * fTemp900
                        + (fTemp895 - (iTemp896) as F64)
                            * (fTemp898
                                - (fTemp899
                                    + fTemp698
                                        * (fTemp900
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp897, 8)) as usize]
                                            } - fTemp898))))
                } else {
                    1.0 - (fTemp893
                        + fTemp698 * fTemp894
                        + (fTemp889 - (iTemp890) as F64)
                            * (fTemp892
                                - (fTemp893
                                    + fTemp698
                                        * (fTemp894
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp891, 8)) as usize]
                                            } - fTemp892)))))
                });
                let mut fTemp902: F64 = fTemp703 + fTemp888;
                let mut fTemp903: F64 = 131071.0 * (1.0 - fTemp902);
                let mut iTemp904: i32 = (fTemp903) as i32;
                let mut iTemp905: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp904, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp906: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp905, 7)) as usize] };
                let mut fTemp907: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp905 as usize] };
                let mut fTemp908: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp905, 1)) as usize] }
                        - fTemp907;
                let mut fTemp909: F64 = 131071.0 * fTemp902;
                let mut iTemp910: i32 = (fTemp909) as i32;
                let mut iTemp911: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp910, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp912: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp911, 7), 917503),
                    )) as usize]
                };
                let mut fTemp913: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp911 as usize] };
                let mut fTemp914: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp911, 1), 917503),
                    )) as usize]
                } - fTemp913;
                let mut iTemp915: i32 = (fTemp759
                    > (((if iTemp689 != 0 {
                        fTemp913
                            + fTemp698 * fTemp914
                            + (fTemp909 - (iTemp910) as F64)
                                * (fTemp912
                                    - (fTemp913
                                        + fTemp698
                                            * (fTemp914
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp911, 8),
                                                            917503,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp912))))
                    } else {
                        1.0 - (fTemp907
                            + fTemp698 * fTemp908
                            + (fTemp903 - (iTemp904) as F64)
                                * (fTemp906
                                    - (fTemp907
                                        + fTemp698
                                            * (fTemp908
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0
                                                        [(i32::wrapping_add(iTemp905, 8)) as usize]
                                                } - fTemp906)))))
                    }) - fTemp901)
                        / (1.0 - fTemp901))) as i32;
                let mut fTemp916: F64 = (if iTemp915 != 0 { fTemp885 } else { fTemp888 });
                let mut fTemp917: F64 = (if iTemp915 != 0 { fTemp888 } else { fTemp886 });
                let mut fTemp918: F64 = fTemp917 + fTemp916;
                let mut fTemp919: F64 = 0.5 * fTemp918;
                let mut fTemp920: F64 = 131071.0 * (1.0 - fTemp919);
                let mut iTemp921: i32 = (fTemp920) as i32;
                let mut iTemp922: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp921, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp923: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp922, 7)) as usize] };
                let mut fTemp924: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp922 as usize] };
                let mut fTemp925: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp922, 1)) as usize] }
                        - fTemp924;
                let mut fTemp926: F64 = 65535.5 * fTemp918;
                let mut iTemp927: i32 = (fTemp926) as i32;
                let mut iTemp928: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp927, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp929: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp928, 7)) as usize] };
                let mut fTemp930: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp928 as usize] };
                let mut fTemp931: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp928, 1)) as usize] }
                        - fTemp930;
                let mut fTemp932: F64 = (if iTemp689 != 0 {
                    fTemp930
                        + fTemp698 * fTemp931
                        + (fTemp926 - (iTemp927) as F64)
                            * (fTemp929
                                - (fTemp930
                                    + fTemp698
                                        * (fTemp931
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp928, 8)) as usize]
                                            } - fTemp929))))
                } else {
                    1.0 - (fTemp924
                        + fTemp698 * fTemp925
                        + (fTemp920 - (iTemp921) as F64)
                            * (fTemp923
                                - (fTemp924
                                    + fTemp698
                                        * (fTemp925
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp922, 8)) as usize]
                                            } - fTemp923)))))
                });
                let mut fTemp933: F64 = fTemp703 + fTemp919;
                let mut fTemp934: F64 = 131071.0 * (1.0 - fTemp933);
                let mut iTemp935: i32 = (fTemp934) as i32;
                let mut iTemp936: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp935, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp937: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp936, 7)) as usize] };
                let mut fTemp938: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp936 as usize] };
                let mut fTemp939: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp936, 1)) as usize] }
                        - fTemp938;
                let mut fTemp940: F64 = 131071.0 * fTemp933;
                let mut iTemp941: i32 = (fTemp940) as i32;
                let mut iTemp942: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp941, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp943: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp942, 7), 917503),
                    )) as usize]
                };
                let mut fTemp944: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp942 as usize] };
                let mut fTemp945: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp942, 1), 917503),
                    )) as usize]
                } - fTemp944;
                let mut iTemp946: i32 = (fTemp759
                    > (((if iTemp689 != 0 {
                        fTemp944
                            + fTemp698 * fTemp945
                            + (fTemp940 - (iTemp941) as F64)
                                * (fTemp943
                                    - (fTemp944
                                        + fTemp698
                                            * (fTemp945
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp942, 8),
                                                            917503,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp943))))
                    } else {
                        1.0 - (fTemp938
                            + fTemp698 * fTemp939
                            + (fTemp934 - (iTemp935) as F64)
                                * (fTemp937
                                    - (fTemp938
                                        + fTemp698
                                            * (fTemp939
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0
                                                        [(i32::wrapping_add(iTemp936, 8)) as usize]
                                                } - fTemp937)))))
                    }) - fTemp932)
                        / (1.0 - fTemp932))) as i32;
                let mut fTemp947: F64 = (if iTemp946 != 0 { fTemp916 } else { fTemp919 });
                let mut fTemp948: F64 = (if iTemp946 != 0 { fTemp919 } else { fTemp917 });
                let mut fTemp949: F64 = fTemp948 + fTemp947;
                let mut fTemp950: F64 = 0.5 * fTemp949;
                let mut fTemp951: F64 = 131071.0 * (1.0 - fTemp950);
                let mut iTemp952: i32 = (fTemp951) as i32;
                let mut iTemp953: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp952, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp954: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp953, 7)) as usize] };
                let mut fTemp955: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp953 as usize] };
                let mut fTemp956: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp953, 1)) as usize] }
                        - fTemp955;
                let mut fTemp957: F64 = 65535.5 * fTemp949;
                let mut iTemp958: i32 = (fTemp957) as i32;
                let mut iTemp959: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp958, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp960: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp959, 7)) as usize] };
                let mut fTemp961: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp959 as usize] };
                let mut fTemp962: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp959, 1)) as usize] }
                        - fTemp961;
                let mut fTemp963: F64 = (if iTemp689 != 0 {
                    fTemp961
                        + fTemp698 * fTemp962
                        + (fTemp957 - (iTemp958) as F64)
                            * (fTemp960
                                - (fTemp961
                                    + fTemp698
                                        * (fTemp962
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp959, 8)) as usize]
                                            } - fTemp960))))
                } else {
                    1.0 - (fTemp955
                        + fTemp698 * fTemp956
                        + (fTemp951 - (iTemp952) as F64)
                            * (fTemp954
                                - (fTemp955
                                    + fTemp698
                                        * (fTemp956
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp953, 8)) as usize]
                                            } - fTemp954)))))
                });
                let mut fTemp964: F64 = fTemp703 + fTemp950;
                let mut fTemp965: F64 = 131071.0 * (1.0 - fTemp964);
                let mut iTemp966: i32 = (fTemp965) as i32;
                let mut iTemp967: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp966, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp968: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp967, 7)) as usize] };
                let mut fTemp969: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp967 as usize] };
                let mut fTemp970: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp967, 1)) as usize] }
                        - fTemp969;
                let mut fTemp971: F64 = 131071.0 * fTemp964;
                let mut iTemp972: i32 = (fTemp971) as i32;
                let mut iTemp973: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp972, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp974: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp973, 7), 917503),
                    )) as usize]
                };
                let mut fTemp975: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp973 as usize] };
                let mut fTemp976: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp973, 1), 917503),
                    )) as usize]
                } - fTemp975;
                let mut iTemp977: i32 = (fTemp759
                    > (((if iTemp689 != 0 {
                        fTemp975
                            + fTemp698 * fTemp976
                            + (fTemp971 - (iTemp972) as F64)
                                * (fTemp974
                                    - (fTemp975
                                        + fTemp698
                                            * (fTemp976
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp973, 8),
                                                            917503,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp974))))
                    } else {
                        1.0 - (fTemp969
                            + fTemp698 * fTemp970
                            + (fTemp965 - (iTemp966) as F64)
                                * (fTemp968
                                    - (fTemp969
                                        + fTemp698
                                            * (fTemp970
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0
                                                        [(i32::wrapping_add(iTemp967, 8)) as usize]
                                                } - fTemp968)))))
                    }) - fTemp963)
                        / (1.0 - fTemp963))) as i32;
                let mut fTemp978: F64 = (if iTemp977 != 0 { fTemp947 } else { fTemp950 });
                let mut fTemp979: F64 = (if iTemp977 != 0 { fTemp950 } else { fTemp948 });
                let mut fTemp980: F64 = fTemp979 + fTemp978;
                let mut fTemp981: F64 = 0.5 * fTemp980;
                let mut fTemp982: F64 = 131071.0 * (1.0 - fTemp981);
                let mut iTemp983: i32 = (fTemp982) as i32;
                let mut iTemp984: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp983, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp985: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp984, 7)) as usize] };
                let mut fTemp986: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp984 as usize] };
                let mut fTemp987: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp984, 1)) as usize] }
                        - fTemp986;
                let mut fTemp988: F64 = 65535.5 * fTemp980;
                let mut iTemp989: i32 = (fTemp988) as i32;
                let mut iTemp990: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp989, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp991: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp990, 7)) as usize] };
                let mut fTemp992: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp990 as usize] };
                let mut fTemp993: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp990, 1)) as usize] }
                        - fTemp992;
                let mut fTemp994: F64 = (if iTemp689 != 0 {
                    fTemp992
                        + fTemp698 * fTemp993
                        + (fTemp988 - (iTemp989) as F64)
                            * (fTemp991
                                - (fTemp992
                                    + fTemp698
                                        * (fTemp993
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp990, 8)) as usize]
                                            } - fTemp991))))
                } else {
                    1.0 - (fTemp986
                        + fTemp698 * fTemp987
                        + (fTemp982 - (iTemp983) as F64)
                            * (fTemp985
                                - (fTemp986
                                    + fTemp698
                                        * (fTemp987
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp984, 8)) as usize]
                                            } - fTemp985)))))
                });
                let mut fTemp995: F64 = fTemp703 + fTemp981;
                let mut fTemp996: F64 = 131071.0 * (1.0 - fTemp995);
                let mut iTemp997: i32 = (fTemp996) as i32;
                let mut iTemp998: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp997, 131071))),
                        ),
                        917503,
                    ),
                );
                let mut fTemp999: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp998, 7)) as usize] };
                let mut fTemp1000: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp998 as usize] };
                let mut fTemp1001: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp998, 1)) as usize] }
                        - fTemp1000;
                let mut fTemp1002: F64 = 131071.0 * fTemp995;
                let mut iTemp1003: i32 = (fTemp1002) as i32;
                let mut iTemp1004: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1003, 131071)),
                            ),
                        ),
                        917503,
                    ),
                );
                let mut fTemp1005: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1004, 7), 917503),
                    )) as usize]
                };
                let mut fTemp1006: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1004 as usize] };
                let mut fTemp1007: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1004, 1), 917503),
                    )) as usize]
                } - fTemp1006;
                let mut iTemp1008: i32 = (fTemp759
                    > (((if iTemp689 != 0 {
                        fTemp1006
                            + fTemp698 * fTemp1007
                            + (fTemp1002 - (iTemp1003) as F64)
                                * (fTemp1005
                                    - (fTemp1006
                                        + fTemp698
                                            * (fTemp1007
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1004, 8),
                                                            917503,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1005))))
                    } else {
                        1.0 - (fTemp1000
                            + fTemp698 * fTemp1001
                            + (fTemp996 - (iTemp997) as F64)
                                * (fTemp999
                                    - (fTemp1000
                                        + fTemp698
                                            * (fTemp1001
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0
                                                        [(i32::wrapping_add(iTemp998, 8)) as usize]
                                                } - fTemp999)))))
                    }) - fTemp994)
                        / (1.0 - fTemp994))) as i32;
                let mut fTemp1009: F64 = (if iTemp1008 != 0 { fTemp978 } else { fTemp981 });
                let mut fTemp1010: F64 = (if iTemp1008 != 0 { fTemp981 } else { fTemp979 });
                let mut fTemp1011: F64 = fTemp1010 + fTemp1009;
                let mut fTemp1012: F64 = 0.5 * fTemp1011;
                let mut fTemp1013: F64 = 131071.0 * (1.0 - fTemp1012);
                let mut iTemp1014: i32 = (fTemp1013) as i32;
                let mut iTemp1015: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1014, 131071)),
                            ),
                        ),
                        917503,
                    ),
                );
                let mut fTemp1016: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1015, 7)) as usize] };
                let mut fTemp1017: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1015 as usize] };
                let mut fTemp1018: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1015, 1)) as usize] }
                        - fTemp1017;
                let mut fTemp1019: F64 = 65535.5 * fTemp1011;
                let mut iTemp1020: i32 = (fTemp1019) as i32;
                let mut iTemp1021: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1020, 131071)),
                            ),
                        ),
                        917503,
                    ),
                );
                let mut fTemp1022: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1021, 7)) as usize] };
                let mut fTemp1023: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1021 as usize] };
                let mut fTemp1024: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1021, 1)) as usize] }
                        - fTemp1023;
                let mut fTemp1025: F64 = (if iTemp689 != 0 {
                    fTemp1023
                        + fTemp698 * fTemp1024
                        + (fTemp1019 - (iTemp1020) as F64)
                            * (fTemp1022
                                - (fTemp1023
                                    + fTemp698
                                        * (fTemp1024
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp1021, 8)) as usize]
                                            } - fTemp1022))))
                } else {
                    1.0 - (fTemp1017
                        + fTemp698 * fTemp1018
                        + (fTemp1013 - (iTemp1014) as F64)
                            * (fTemp1016
                                - (fTemp1017
                                    + fTemp698
                                        * (fTemp1018
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp1015, 8)) as usize]
                                            } - fTemp1016)))))
                });
                let mut fTemp1026: F64 = fTemp703 + fTemp1012;
                let mut fTemp1027: F64 = 131071.0 * (1.0 - fTemp1026);
                let mut iTemp1028: i32 = (fTemp1027) as i32;
                let mut iTemp1029: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1028, 131071)),
                            ),
                        ),
                        917503,
                    ),
                );
                let mut fTemp1030: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1029, 7)) as usize] };
                let mut fTemp1031: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1029 as usize] };
                let mut fTemp1032: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1029, 1)) as usize] }
                        - fTemp1031;
                let mut fTemp1033: F64 = 131071.0 * fTemp1026;
                let mut iTemp1034: i32 = (fTemp1033) as i32;
                let mut iTemp1035: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1034, 131071)),
                            ),
                        ),
                        917503,
                    ),
                );
                let mut fTemp1036: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1035, 7), 917503),
                    )) as usize]
                };
                let mut fTemp1037: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1035 as usize] };
                let mut fTemp1038: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1035, 1), 917503),
                    )) as usize]
                } - fTemp1037;
                let mut iTemp1039: i32 = (fTemp759
                    > (((if iTemp689 != 0 {
                        fTemp1037
                            + fTemp698 * fTemp1038
                            + (fTemp1033 - (iTemp1034) as F64)
                                * (fTemp1036
                                    - (fTemp1037
                                        + fTemp698
                                            * (fTemp1038
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1035, 8),
                                                            917503,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1036))))
                    } else {
                        1.0 - (fTemp1031
                            + fTemp698 * fTemp1032
                            + (fTemp1027 - (iTemp1028) as F64)
                                * (fTemp1030
                                    - (fTemp1031
                                        + fTemp698
                                            * (fTemp1032
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0
                                                        [(i32::wrapping_add(iTemp1029, 8)) as usize]
                                                } - fTemp1030)))))
                    }) - fTemp1025)
                        / (1.0 - fTemp1025))) as i32;
                let mut fTemp1040: F64 = (if iTemp1039 != 0 { fTemp1009 } else { fTemp1012 });
                let mut fTemp1041: F64 = (if iTemp1039 != 0 { fTemp1012 } else { fTemp1010 });
                let mut fTemp1042: F64 = fTemp1041 + fTemp1040;
                let mut fTemp1043: F64 = 0.5 * fTemp1042;
                let mut fTemp1044: F64 = 131071.0 * (1.0 - fTemp1043);
                let mut iTemp1045: i32 = (fTemp1044) as i32;
                let mut iTemp1046: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1045, 131071)),
                            ),
                        ),
                        917503,
                    ),
                );
                let mut fTemp1047: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1046, 7)) as usize] };
                let mut fTemp1048: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1046 as usize] };
                let mut fTemp1049: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1046, 1)) as usize] }
                        - fTemp1048;
                let mut fTemp1050: F64 = 65535.5 * fTemp1042;
                let mut iTemp1051: i32 = (fTemp1050) as i32;
                let mut iTemp1052: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1051, 131071)),
                            ),
                        ),
                        917503,
                    ),
                );
                let mut fTemp1053: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1052, 7)) as usize] };
                let mut fTemp1054: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1052 as usize] };
                let mut fTemp1055: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1052, 1)) as usize] }
                        - fTemp1054;
                let mut fTemp1056: F64 = (if iTemp689 != 0 {
                    fTemp1054
                        + fTemp698 * fTemp1055
                        + (fTemp1050 - (iTemp1051) as F64)
                            * (fTemp1053
                                - (fTemp1054
                                    + fTemp698
                                        * (fTemp1055
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp1052, 8)) as usize]
                                            } - fTemp1053))))
                } else {
                    1.0 - (fTemp1048
                        + fTemp698 * fTemp1049
                        + (fTemp1044 - (iTemp1045) as F64)
                            * (fTemp1047
                                - (fTemp1048
                                    + fTemp698
                                        * (fTemp1049
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp1046, 8)) as usize]
                                            } - fTemp1047)))))
                });
                let mut fTemp1057: F64 = fTemp703 + fTemp1043;
                let mut fTemp1058: F64 = 131071.0 * (1.0 - fTemp1057);
                let mut iTemp1059: i32 = (fTemp1058) as i32;
                let mut iTemp1060: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1059, 131071)),
                            ),
                        ),
                        917503,
                    ),
                );
                let mut fTemp1061: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1060, 7)) as usize] };
                let mut fTemp1062: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1060 as usize] };
                let mut fTemp1063: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1060, 1)) as usize] }
                        - fTemp1062;
                let mut fTemp1064: F64 = 131071.0 * fTemp1057;
                let mut iTemp1065: i32 = (fTemp1064) as i32;
                let mut iTemp1066: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1065, 131071)),
                            ),
                        ),
                        917503,
                    ),
                );
                let mut fTemp1067: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1066, 7), 917503),
                    )) as usize]
                };
                let mut fTemp1068: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1066 as usize] };
                let mut fTemp1069: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1066, 1), 917503),
                    )) as usize]
                } - fTemp1068;
                let mut iTemp1070: i32 = (fTemp759
                    > (((if iTemp689 != 0 {
                        fTemp1068
                            + fTemp698 * fTemp1069
                            + (fTemp1064 - (iTemp1065) as F64)
                                * (fTemp1067
                                    - (fTemp1068
                                        + fTemp698
                                            * (fTemp1069
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1066, 8),
                                                            917503,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1067))))
                    } else {
                        1.0 - (fTemp1062
                            + fTemp698 * fTemp1063
                            + (fTemp1058 - (iTemp1059) as F64)
                                * (fTemp1061
                                    - (fTemp1062
                                        + fTemp698
                                            * (fTemp1063
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0
                                                        [(i32::wrapping_add(iTemp1060, 8)) as usize]
                                                } - fTemp1061)))))
                    }) - fTemp1056)
                        / (1.0 - fTemp1056))) as i32;
                let mut fTemp1071: F64 = (if iTemp1070 != 0 { fTemp1040 } else { fTemp1043 });
                let mut fTemp1072: F64 = (if iTemp1070 != 0 { fTemp1043 } else { fTemp1041 });
                let mut fTemp1073: F64 = fTemp1072 + fTemp1071;
                let mut fTemp1074: F64 = 0.5 * fTemp1073;
                let mut fTemp1075: F64 = 131071.0 * (1.0 - fTemp1074);
                let mut iTemp1076: i32 = (fTemp1075) as i32;
                let mut iTemp1077: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1076, 131071)),
                            ),
                        ),
                        917503,
                    ),
                );
                let mut fTemp1078: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1077, 7)) as usize] };
                let mut fTemp1079: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1077 as usize] };
                let mut fTemp1080: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1077, 1)) as usize] }
                        - fTemp1079;
                let mut fTemp1081: F64 = 65535.5 * fTemp1073;
                let mut iTemp1082: i32 = (fTemp1081) as i32;
                let mut iTemp1083: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1082, 131071)),
                            ),
                        ),
                        917503,
                    ),
                );
                let mut fTemp1084: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1083, 7)) as usize] };
                let mut fTemp1085: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1083 as usize] };
                let mut fTemp1086: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1083, 1)) as usize] }
                        - fTemp1085;
                let mut fTemp1087: F64 = (if iTemp689 != 0 {
                    fTemp1085
                        + fTemp698 * fTemp1086
                        + (fTemp1081 - (iTemp1082) as F64)
                            * (fTemp1084
                                - (fTemp1085
                                    + fTemp698
                                        * (fTemp1086
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp1083, 8)) as usize]
                                            } - fTemp1084))))
                } else {
                    1.0 - (fTemp1079
                        + fTemp698 * fTemp1080
                        + (fTemp1075 - (iTemp1076) as F64)
                            * (fTemp1078
                                - (fTemp1079
                                    + fTemp698
                                        * (fTemp1080
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp1077, 8)) as usize]
                                            } - fTemp1078)))))
                });
                let mut fTemp1088: F64 = fTemp703 + fTemp1074;
                let mut fTemp1089: F64 = 131071.0 * (1.0 - fTemp1088);
                let mut iTemp1090: i32 = (fTemp1089) as i32;
                let mut iTemp1091: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1090, 131071)),
                            ),
                        ),
                        917503,
                    ),
                );
                let mut fTemp1092: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1091, 7)) as usize] };
                let mut fTemp1093: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1091 as usize] };
                let mut fTemp1094: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1091, 1)) as usize] }
                        - fTemp1093;
                let mut fTemp1095: F64 = 131071.0 * fTemp1088;
                let mut iTemp1096: i32 = (fTemp1095) as i32;
                let mut iTemp1097: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1096, 131071)),
                            ),
                        ),
                        917503,
                    ),
                );
                let mut fTemp1098: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1097, 7), 917503),
                    )) as usize]
                };
                let mut fTemp1099: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1097 as usize] };
                let mut fTemp1100: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1097, 1), 917503),
                    )) as usize]
                } - fTemp1099;
                let mut iTemp1101: i32 = (fTemp759
                    > (((if iTemp689 != 0 {
                        fTemp1099
                            + fTemp698 * fTemp1100
                            + (fTemp1095 - (iTemp1096) as F64)
                                * (fTemp1098
                                    - (fTemp1099
                                        + fTemp698
                                            * (fTemp1100
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1097, 8),
                                                            917503,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1098))))
                    } else {
                        1.0 - (fTemp1093
                            + fTemp698 * fTemp1094
                            + (fTemp1089 - (iTemp1090) as F64)
                                * (fTemp1092
                                    - (fTemp1093
                                        + fTemp698
                                            * (fTemp1094
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0
                                                        [(i32::wrapping_add(iTemp1091, 8)) as usize]
                                                } - fTemp1092)))))
                    }) - fTemp1087)
                        / (1.0 - fTemp1087))) as i32;
                let mut fTemp1102: F64 = (if iTemp1101 != 0 { fTemp1071 } else { fTemp1074 });
                let mut fTemp1103: F64 = (if iTemp1101 != 0 { fTemp1074 } else { fTemp1072 });
                let mut fTemp1104: F64 = fTemp1103 + fTemp1102;
                let mut fTemp1105: F64 = 0.5 * fTemp1104;
                let mut fTemp1106: F64 = 131071.0 * (1.0 - fTemp1105);
                let mut iTemp1107: i32 = (fTemp1106) as i32;
                let mut iTemp1108: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1107, 131071)),
                            ),
                        ),
                        917503,
                    ),
                );
                let mut fTemp1109: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1108, 7)) as usize] };
                let mut fTemp1110: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1108 as usize] };
                let mut fTemp1111: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1108, 1)) as usize] }
                        - fTemp1110;
                let mut fTemp1112: F64 = 65535.5 * fTemp1104;
                let mut iTemp1113: i32 = (fTemp1112) as i32;
                let mut iTemp1114: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1113, 131071)),
                            ),
                        ),
                        917503,
                    ),
                );
                let mut fTemp1115: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1114, 7)) as usize] };
                let mut fTemp1116: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1114 as usize] };
                let mut fTemp1117: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1114, 1)) as usize] }
                        - fTemp1116;
                let mut fTemp1118: F64 = (if iTemp689 != 0 {
                    fTemp1116
                        + fTemp698 * fTemp1117
                        + (fTemp1112 - (iTemp1113) as F64)
                            * (fTemp1115
                                - (fTemp1116
                                    + fTemp698
                                        * (fTemp1117
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp1114, 8)) as usize]
                                            } - fTemp1115))))
                } else {
                    1.0 - (fTemp1110
                        + fTemp698 * fTemp1111
                        + (fTemp1106 - (iTemp1107) as F64)
                            * (fTemp1109
                                - (fTemp1110
                                    + fTemp698
                                        * (fTemp1111
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp1108, 8)) as usize]
                                            } - fTemp1109)))))
                });
                let mut fTemp1119: F64 = fTemp703 + fTemp1105;
                let mut fTemp1120: F64 = 131071.0 * (1.0 - fTemp1119);
                let mut iTemp1121: i32 = (fTemp1120) as i32;
                let mut iTemp1122: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1121, 131071)),
                            ),
                        ),
                        917503,
                    ),
                );
                let mut fTemp1123: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1122, 7)) as usize] };
                let mut fTemp1124: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1122 as usize] };
                let mut fTemp1125: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1122, 1)) as usize] }
                        - fTemp1124;
                let mut fTemp1126: F64 = 131071.0 * fTemp1119;
                let mut iTemp1127: i32 = (fTemp1126) as i32;
                let mut iTemp1128: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1127, 131071)),
                            ),
                        ),
                        917503,
                    ),
                );
                let mut fTemp1129: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1128, 7), 917503),
                    )) as usize]
                };
                let mut fTemp1130: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1128 as usize] };
                let mut fTemp1131: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1128, 1), 917503),
                    )) as usize]
                } - fTemp1130;
                let mut iTemp1132: i32 = (fTemp759
                    > (((if iTemp689 != 0 {
                        fTemp1130
                            + fTemp698 * fTemp1131
                            + (fTemp1126 - (iTemp1127) as F64)
                                * (fTemp1129
                                    - (fTemp1130
                                        + fTemp698
                                            * (fTemp1131
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1128, 8),
                                                            917503,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1129))))
                    } else {
                        1.0 - (fTemp1124
                            + fTemp698 * fTemp1125
                            + (fTemp1120 - (iTemp1121) as F64)
                                * (fTemp1123
                                    - (fTemp1124
                                        + fTemp698
                                            * (fTemp1125
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0
                                                        [(i32::wrapping_add(iTemp1122, 8)) as usize]
                                                } - fTemp1123)))))
                    }) - fTemp1118)
                        / (1.0 - fTemp1118))) as i32;
                let mut fTemp1133: F64 = (if iTemp1132 != 0 { fTemp1102 } else { fTemp1105 });
                let mut fTemp1134: F64 = (if iTemp1132 != 0 { fTemp1105 } else { fTemp1103 });
                let mut fTemp1135: F64 = fTemp1134 + fTemp1133;
                let mut fTemp1136: F64 = 0.5 * fTemp1135;
                let mut fTemp1137: F64 = 131071.0 * (1.0 - fTemp1136);
                let mut iTemp1138: i32 = (fTemp1137) as i32;
                let mut iTemp1139: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1138, 131071)),
                            ),
                        ),
                        917503,
                    ),
                );
                let mut fTemp1140: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1139, 7)) as usize] };
                let mut fTemp1141: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1139 as usize] };
                let mut fTemp1142: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1139, 1)) as usize] }
                        - fTemp1141;
                let mut fTemp1143: F64 = 65535.5 * fTemp1135;
                let mut iTemp1144: i32 = (fTemp1143) as i32;
                let mut iTemp1145: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1144, 131071)),
                            ),
                        ),
                        917503,
                    ),
                );
                let mut fTemp1146: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1145, 7)) as usize] };
                let mut fTemp1147: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1145 as usize] };
                let mut fTemp1148: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1145, 1)) as usize] }
                        - fTemp1147;
                let mut fTemp1149: F64 = (if iTemp689 != 0 {
                    fTemp1147
                        + fTemp698 * fTemp1148
                        + (fTemp1143 - (iTemp1144) as F64)
                            * (fTemp1146
                                - (fTemp1147
                                    + fTemp698
                                        * (fTemp1148
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp1145, 8)) as usize]
                                            } - fTemp1146))))
                } else {
                    1.0 - (fTemp1141
                        + fTemp698 * fTemp1142
                        + (fTemp1137 - (iTemp1138) as F64)
                            * (fTemp1140
                                - (fTemp1141
                                    + fTemp698
                                        * (fTemp1142
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp1139, 8)) as usize]
                                            } - fTemp1140)))))
                });
                let mut fTemp1150: F64 = fTemp703 + fTemp1136;
                let mut fTemp1151: F64 = 131071.0 * (1.0 - fTemp1150);
                let mut iTemp1152: i32 = (fTemp1151) as i32;
                let mut iTemp1153: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1152, 131071)),
                            ),
                        ),
                        917503,
                    ),
                );
                let mut fTemp1154: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1153, 7)) as usize] };
                let mut fTemp1155: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1153 as usize] };
                let mut fTemp1156: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1153, 1)) as usize] }
                        - fTemp1155;
                let mut fTemp1157: F64 = 131071.0 * fTemp1150;
                let mut iTemp1158: i32 = (fTemp1157) as i32;
                let mut iTemp1159: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1158, 131071)),
                            ),
                        ),
                        917503,
                    ),
                );
                let mut fTemp1160: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1159, 7), 917503),
                    )) as usize]
                };
                let mut fTemp1161: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1159 as usize] };
                let mut fTemp1162: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1159, 1), 917503),
                    )) as usize]
                } - fTemp1161;
                let mut iTemp1163: i32 = (fTemp759
                    > (((if iTemp689 != 0 {
                        fTemp1161
                            + fTemp698 * fTemp1162
                            + (fTemp1157 - (iTemp1158) as F64)
                                * (fTemp1160
                                    - (fTemp1161
                                        + fTemp698
                                            * (fTemp1162
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1159, 8),
                                                            917503,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1160))))
                    } else {
                        1.0 - (fTemp1155
                            + fTemp698 * fTemp1156
                            + (fTemp1151 - (iTemp1152) as F64)
                                * (fTemp1154
                                    - (fTemp1155
                                        + fTemp698
                                            * (fTemp1156
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0
                                                        [(i32::wrapping_add(iTemp1153, 8)) as usize]
                                                } - fTemp1154)))))
                    }) - fTemp1149)
                        / (1.0 - fTemp1149))) as i32;
                let mut fTemp1164: F64 = (if iTemp1163 != 0 { fTemp1133 } else { fTemp1136 });
                let mut fTemp1165: F64 = (if iTemp1163 != 0 { fTemp1136 } else { fTemp1134 });
                let mut fTemp1166: F64 = fTemp1165 + fTemp1164;
                let mut fTemp1167: F64 = 0.5 * fTemp1166;
                let mut fTemp1168: F64 = 131071.0 * (1.0 - fTemp1167);
                let mut iTemp1169: i32 = (fTemp1168) as i32;
                let mut iTemp1170: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1169, 131071)),
                            ),
                        ),
                        917503,
                    ),
                );
                let mut fTemp1171: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1170, 7)) as usize] };
                let mut fTemp1172: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1170 as usize] };
                let mut fTemp1173: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1170, 1)) as usize] }
                        - fTemp1172;
                let mut fTemp1174: F64 = 65535.5 * fTemp1166;
                let mut iTemp1175: i32 = (fTemp1174) as i32;
                let mut iTemp1176: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1175, 131071)),
                            ),
                        ),
                        917503,
                    ),
                );
                let mut fTemp1177: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1176, 7)) as usize] };
                let mut fTemp1178: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1176 as usize] };
                let mut fTemp1179: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1176, 1)) as usize] }
                        - fTemp1178;
                let mut fTemp1180: F64 = (if iTemp689 != 0 {
                    fTemp1178
                        + fTemp698 * fTemp1179
                        + (fTemp1174 - (iTemp1175) as F64)
                            * (fTemp1177
                                - (fTemp1178
                                    + fTemp698
                                        * (fTemp1179
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp1176, 8)) as usize]
                                            } - fTemp1177))))
                } else {
                    1.0 - (fTemp1172
                        + fTemp698 * fTemp1173
                        + (fTemp1168 - (iTemp1169) as F64)
                            * (fTemp1171
                                - (fTemp1172
                                    + fTemp698
                                        * (fTemp1173
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp1170, 8)) as usize]
                                            } - fTemp1171)))))
                });
                let mut fTemp1181: F64 = fTemp703 + fTemp1167;
                let mut fTemp1182: F64 = 131071.0 * (1.0 - fTemp1181);
                let mut iTemp1183: i32 = (fTemp1182) as i32;
                let mut iTemp1184: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1183, 131071)),
                            ),
                        ),
                        917503,
                    ),
                );
                let mut fTemp1185: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1184, 7)) as usize] };
                let mut fTemp1186: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1184 as usize] };
                let mut fTemp1187: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1184, 1)) as usize] }
                        - fTemp1186;
                let mut fTemp1188: F64 = 131071.0 * fTemp1181;
                let mut iTemp1189: i32 = (fTemp1188) as i32;
                let mut iTemp1190: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1189, 131071)),
                            ),
                        ),
                        917503,
                    ),
                );
                let mut fTemp1191: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1190, 7), 917503),
                    )) as usize]
                };
                let mut fTemp1192: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1190 as usize] };
                let mut fTemp1193: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1190, 1), 917503),
                    )) as usize]
                } - fTemp1192;
                let mut iTemp1194: i32 = (fTemp759
                    > (((if iTemp689 != 0 {
                        fTemp1192
                            + fTemp698 * fTemp1193
                            + (fTemp1188 - (iTemp1189) as F64)
                                * (fTemp1191
                                    - (fTemp1192
                                        + fTemp698
                                            * (fTemp1193
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1190, 8),
                                                            917503,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1191))))
                    } else {
                        1.0 - (fTemp1186
                            + fTemp698 * fTemp1187
                            + (fTemp1182 - (iTemp1183) as F64)
                                * (fTemp1185
                                    - (fTemp1186
                                        + fTemp698
                                            * (fTemp1187
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0
                                                        [(i32::wrapping_add(iTemp1184, 8)) as usize]
                                                } - fTemp1185)))))
                    }) - fTemp1180)
                        / (1.0 - fTemp1180))) as i32;
                let mut fTemp1195: F64 = (if iTemp1194 != 0 { fTemp1164 } else { fTemp1167 });
                let mut fTemp1196: F64 = (if iTemp1194 != 0 { fTemp1167 } else { fTemp1165 });
                let mut fTemp1197: F64 = fTemp1196 + fTemp1195;
                let mut fTemp1198: F64 = 0.5 * fTemp1197;
                let mut fTemp1199: F64 = 131071.0 * (1.0 - fTemp1198);
                let mut iTemp1200: i32 = (fTemp1199) as i32;
                let mut iTemp1201: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1200, 131071)),
                            ),
                        ),
                        917503,
                    ),
                );
                let mut fTemp1202: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1201, 7)) as usize] };
                let mut fTemp1203: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1201 as usize] };
                let mut fTemp1204: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1201, 1)) as usize] }
                        - fTemp1203;
                let mut fTemp1205: F64 = 65535.5 * fTemp1197;
                let mut iTemp1206: i32 = (fTemp1205) as i32;
                let mut iTemp1207: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1206, 131071)),
                            ),
                        ),
                        917503,
                    ),
                );
                let mut fTemp1208: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1207, 7)) as usize] };
                let mut fTemp1209: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1207 as usize] };
                let mut fTemp1210: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1207, 1)) as usize] }
                        - fTemp1209;
                let mut fTemp1211: F64 = (if iTemp689 != 0 {
                    fTemp1209
                        + fTemp698 * fTemp1210
                        + (fTemp1205 - (iTemp1206) as F64)
                            * (fTemp1208
                                - (fTemp1209
                                    + fTemp698
                                        * (fTemp1210
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp1207, 8)) as usize]
                                            } - fTemp1208))))
                } else {
                    1.0 - (fTemp1203
                        + fTemp698 * fTemp1204
                        + (fTemp1199 - (iTemp1200) as F64)
                            * (fTemp1202
                                - (fTemp1203
                                    + fTemp698
                                        * (fTemp1204
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0
                                                    [(i32::wrapping_add(iTemp1201, 8)) as usize]
                                            } - fTemp1202)))))
                });
                let mut fTemp1212: F64 = fTemp703 + fTemp1198;
                let mut fTemp1213: F64 = 131071.0 * (1.0 - fTemp1212);
                let mut iTemp1214: i32 = (fTemp1213) as i32;
                let mut iTemp1215: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1214, 131071)),
                            ),
                        ),
                        917503,
                    ),
                );
                let mut fTemp1216: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1215, 7)) as usize] };
                let mut fTemp1217: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1215 as usize] };
                let mut fTemp1218: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1215, 1)) as usize] }
                        - fTemp1217;
                let mut fTemp1219: F64 = 131071.0 * fTemp1212;
                let mut iTemp1220: i32 = (fTemp1219) as i32;
                let mut iTemp1221: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1220, 131071)),
                            ),
                        ),
                        917503,
                    ),
                );
                let mut fTemp1222: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1221, 7), 917503),
                    )) as usize]
                };
                let mut fTemp1223: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1221 as usize] };
                let mut fTemp1224: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1221, 1), 917503),
                    )) as usize]
                } - fTemp1223;
                let mut iTemp1225: i32 = (fTemp759
                    > (((if iTemp689 != 0 {
                        fTemp1223
                            + fTemp698 * fTemp1224
                            + (fTemp1219 - (iTemp1220) as F64)
                                * (fTemp1222
                                    - (fTemp1223
                                        + fTemp698
                                            * (fTemp1224
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1221, 8),
                                                            917503,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1222))))
                    } else {
                        1.0 - (fTemp1217
                            + fTemp698 * fTemp1218
                            + (fTemp1213 - (iTemp1214) as F64)
                                * (fTemp1216
                                    - (fTemp1217
                                        + fTemp698
                                            * (fTemp1218
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0
                                                        [(i32::wrapping_add(iTemp1215, 8)) as usize]
                                                } - fTemp1216)))))
                    }) - fTemp1211)
                        / (1.0 - fTemp1211))) as i32;
                let mut fTemp1226: F64 = (if iTemp1225 != 0 { fTemp1195 } else { fTemp1198 });
                let mut fTemp1227: F64 = (if iTemp1225 != 0 { fTemp1198 } else { fTemp1196 });
                let mut fTemp1228: F64 = fTemp1227 + fTemp1226;
                let mut fTemp1229: F64 = 0.5 * fTemp1228;
                let mut fTemp1230: F64 = 131071.0 * (1.0 - fTemp1229);
                let mut iTemp1231: i32 = (fTemp1230) as i32;
                let mut iTemp1232: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1231, 131071)),
                            ),
                        ),
                        917503,
                    ),
                );
                let mut fTemp1233: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1232, 7)) as usize] };
                let mut fTemp1234: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1232 as usize] };
                let mut fTemp1235: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1232, 1)) as usize] }
                        - fTemp1234;
                let mut fTemp1236: F64 = 65535.5 * fTemp1228;
                let mut iTemp1237: i32 = (fTemp1236) as i32;
                let mut iTemp1238: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1237, 131071)),
                            ),
                        ),
                        917503,
                    ),
                );
                let mut fTemp1239: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1238, 7)) as usize] };
                let mut fTemp1240: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1238 as usize] };
                let mut fTemp1241: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1238, 1)) as usize] }
                        - fTemp1240;
                let mut fTemp1242: F64 = (if iTemp689 != 0 {
                    fTemp1240
                        + fTemp698 * fTemp1241
                        + (fTemp1236 - (iTemp1237) as F64)
                            * (fTemp1239
                                - (fTemp1240
                                    + fTemp698
                                        * (fTemp1241
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp1238, 8),
                                                        917503,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp1239))))
                } else {
                    1.0 - (fTemp1234
                        + fTemp698 * fTemp1235
                        + (fTemp1230 - (iTemp1231) as F64)
                            * (fTemp1233
                                - (fTemp1234
                                    + fTemp698
                                        * (fTemp1235
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp1232, 8),
                                                        917503,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp1233)))))
                });
                let mut fTemp1243: F64 = fTemp703 + fTemp1229;
                let mut fTemp1244: F64 = 131071.0 * (1.0 - fTemp1243);
                let mut iTemp1245: i32 = (fTemp1244) as i32;
                let mut iTemp1246: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1245, 131071)),
                            ),
                        ),
                        917503,
                    ),
                );
                let mut fTemp1247: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1246, 7)) as usize] };
                let mut fTemp1248: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1246 as usize] };
                let mut fTemp1249: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1246, 1)) as usize] }
                        - fTemp1248;
                let mut fTemp1250: F64 = 131071.0 * fTemp1243;
                let mut iTemp1251: i32 = (fTemp1250) as i32;
                let mut iTemp1252: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1251, 131071)),
                            ),
                        ),
                        917503,
                    ),
                );
                let mut fTemp1253: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1252, 7), 917503),
                    )) as usize]
                };
                let mut fTemp1254: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1252 as usize] };
                let mut fTemp1255: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1252, 1), 917503),
                    )) as usize]
                } - fTemp1254;
                let mut iTemp1256: i32 = (fTemp759
                    > (((if iTemp689 != 0 {
                        fTemp1254
                            + fTemp698 * fTemp1255
                            + (fTemp1250 - (iTemp1251) as F64)
                                * (fTemp1253
                                    - (fTemp1254
                                        + fTemp698
                                            * (fTemp1255
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1252, 8),
                                                            917503,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1253))))
                    } else {
                        1.0 - (fTemp1248
                            + fTemp698 * fTemp1249
                            + (fTemp1244 - (iTemp1245) as F64)
                                * (fTemp1247
                                    - (fTemp1248
                                        + fTemp698
                                            * (fTemp1249
                                                - (unsafe {
                                                    ftbl0LambRs48kSIG0
                                                        [(i32::wrapping_add(iTemp1246, 8)) as usize]
                                                } - fTemp1247)))))
                    }) - fTemp1242)
                        / (1.0 - fTemp1242))) as i32;
                let mut fTemp1257: F64 = F64::min(
                    1.0,
                    F64::max(
                        0.0,
                        0.5 * ((if iTemp1256 != 0 { fTemp1229 } else { fTemp1227 })
                            + (if iTemp1256 != 0 { fTemp1226 } else { fTemp1229 })),
                    ),
                );
                self.fRec15[0] = fTemp1257;
                let mut fTemp1258: F64 = 131071.0 * (1.0 - fTemp1257);
                let mut iTemp1259: i32 = (fTemp1258) as i32;
                let mut iTemp1260: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1259, 131071)),
                            ),
                        ),
                        917503,
                    ),
                );
                let mut fTemp1261: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1260, 7)) as usize] };
                let mut fTemp1262: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1260 as usize] };
                let mut fTemp1263: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1260, 1)) as usize] }
                        - fTemp1262;
                let mut fTemp1264: F64 = 131071.0 * fTemp1257;
                let mut iTemp1265: i32 = (fTemp1264) as i32;
                let mut iTemp1266: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1265, 131071)),
                            ),
                        ),
                        917503,
                    ),
                );
                let mut fTemp1267: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1266, 7)) as usize] };
                let mut fTemp1268: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1266 as usize] };
                let mut fTemp1269: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1266, 1)) as usize] }
                        - fTemp1268;
                let mut fTemp1270: F64 = (if iTemp689 != 0 {
                    fTemp1268
                        + fTemp698 * fTemp1269
                        + (fTemp1264 - (iTemp1265) as F64)
                            * (fTemp1267
                                - (fTemp1268
                                    + fTemp698
                                        * (fTemp1269
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp1266, 8),
                                                        917503,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp1267))))
                } else {
                    1.0 - (fTemp1262
                        + fTemp698 * fTemp1263
                        + (fTemp1258 - (iTemp1259) as F64)
                            * (fTemp1261
                                - (fTemp1262
                                    + fTemp698
                                        * (fTemp1263
                                            - (unsafe {
                                                ftbl0LambRs48kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp1260, 8),
                                                        917503,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp1261)))))
                });
                let mut fTemp1271: F64 = fTemp703 + fTemp1257;
                let mut fTemp1272: F64 = 131071.0 * (1.0 - fTemp1271);
                let mut iTemp1273: i32 = (fTemp1272) as i32;
                let mut iTemp1274: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1273, 131071)),
                            ),
                        ),
                        917503,
                    ),
                );
                let mut fTemp1275: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1274, 7)) as usize] };
                let mut fTemp1276: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1274 as usize] };
                let mut fTemp1277: F64 =
                    unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1274, 1)) as usize] }
                        - fTemp1276;
                let mut fTemp1278: F64 = 131071.0 * fTemp1271;
                let mut iTemp1279: i32 = (fTemp1278) as i32;
                let mut iTemp1280: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp693,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1279, 131071)),
                            ),
                        ),
                        917503,
                    ),
                );
                let mut fTemp1281: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1280, 7), 917503),
                    )) as usize]
                };
                let mut fTemp1282: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1280 as usize] };
                let mut fTemp1283: F64 = unsafe {
                    ftbl0LambRs48kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1280, 1), 917503),
                    )) as usize]
                } - fTemp1282;
                let mut fTemp1284: F64 = self.fRec16[1]
                    + (if ((0.001 * fTemp702) == 0.0) as i32 != 0 {
                        fTemp688
                    } else {
                        fTemp688
                            * ((if iTemp689 != 0 {
                                fTemp1282
                                    + fTemp698 * fTemp1283
                                    + (fTemp1278 - (iTemp1279) as F64)
                                        * (fTemp1281
                                            - (fTemp1282
                                                + fTemp698
                                                    * (fTemp1283
                                                        - (unsafe {
                                                            ftbl0LambRs48kSIG0[(std::cmp::max(
                                                                0,
                                                                std::cmp::min(
                                                                    i32::wrapping_add(iTemp1280, 8),
                                                                    917503,
                                                                ),
                                                            ))
                                                                as usize]
                                                        } - fTemp1281))))
                            } else {
                                1.0 - (fTemp1276
                                    + fTemp698 * fTemp1277
                                    + (fTemp1272 - (iTemp1273) as F64)
                                        * (fTemp1275
                                            - (fTemp1276
                                                + fTemp698
                                                    * (fTemp1277
                                                        - (unsafe {
                                                            ftbl0LambRs48kSIG0[(i32::wrapping_add(
                                                                iTemp1274, 8,
                                                            ))
                                                                as usize]
                                                        } - fTemp1275)))))
                            }) - fTemp1270)
                            / (1.0 - fTemp1270)
                    });
                self.fRec16[0] = (if iTemp701 != 0 {
                    F64::min(fTemp1284, self.fRec16[1])
                } else {
                    F64::max(fTemp1284, self.fRec16[1])
                });
                self.fVec58[(self.IOTA0 & 8191) as usize] = F64::powf(1e+01, 0.05 * self.fRec16[0]);
                let mut fTemp1285: F64 =
                    self.fVec58[((i32::wrapping_sub(self.IOTA0, iSlow70)) & 8191) as usize];
                *output1 = 0.5
                    * fTemp2
                    * self.fVec3[((i32::wrapping_sub(self.IOTA0, iSlow71)) & 32767) as usize]
                    + self.fRec14[0]
                        * fTemp4
                        * self.fVec4[((i32::wrapping_sub(self.IOTA0, iSlow71)) & 32767) as usize]
                        * fTemp1285;
                *output2 = fTemp3 + fTemp663 * fTemp4;
                *output3 = fTemp3 + fTemp4 * fTemp1285;
                self.IOTA0 = i32::wrapping_add(self.IOTA0, 1);
                self.fRec0[1] = self.fRec0[0];
                self.fRec4[1] = self.fRec4[0];
                self.fRec11[1] = self.fRec11[0];
                self.fRec10[1] = self.fRec10[0];
                self.fRec9[1] = self.fRec9[0];
                self.fRec8[1] = self.fRec8[0];
                self.fRec7[1] = self.fRec7[0];
                self.fRec5[1] = self.fRec5[0];
                self.fRec12[1] = self.fRec12[0];
                self.fRec6[1] = self.fRec6[0];
                self.fVec6[2] = self.fVec6[1];
                self.fVec6[1] = self.fVec6[0];
                for j0 in (1..=4).rev() {
                    self.fVec7[j0 as usize] = self.fVec7[(i32::wrapping_sub(j0, 1)) as usize];
                }
                for j1 in (1..=11).rev() {
                    self.fVec8[j1 as usize] = self.fVec8[(i32::wrapping_sub(j1, 1)) as usize];
                }
                self.fRec3[1] = self.fRec3[0];
                self.fVec17[2] = self.fVec17[1];
                self.fVec17[1] = self.fVec17[0];
                for j2 in (1..=4).rev() {
                    self.fVec18[j2 as usize] = self.fVec18[(i32::wrapping_sub(j2, 1)) as usize];
                }
                for j3 in (1..=11).rev() {
                    self.fVec19[j3 as usize] = self.fVec19[(i32::wrapping_sub(j3, 1)) as usize];
                }
                self.fVec28[1] = self.fVec28[0];
                self.fVec29[1] = self.fVec29[0];
                self.fVec30[1] = self.fVec30[0];
                self.fRec1[1] = self.fRec1[0];
                self.fRec2[1] = self.fRec2[0];
                self.fRec14[1] = self.fRec14[0];
                self.fVec33[2] = self.fVec33[1];
                self.fVec33[1] = self.fVec33[0];
                for j4 in (1..=4).rev() {
                    self.fVec34[j4 as usize] = self.fVec34[(i32::wrapping_sub(j4, 1)) as usize];
                }
                for j5 in (1..=11).rev() {
                    self.fVec35[j5 as usize] = self.fVec35[(i32::wrapping_sub(j5, 1)) as usize];
                }
                self.fRec17[1] = self.fRec17[0];
                self.fVec44[2] = self.fVec44[1];
                self.fVec44[1] = self.fVec44[0];
                for j6 in (1..=4).rev() {
                    self.fVec45[j6 as usize] = self.fVec45[(i32::wrapping_sub(j6, 1)) as usize];
                }
                for j7 in (1..=11).rev() {
                    self.fVec46[j7 as usize] = self.fVec46[(i32::wrapping_sub(j7, 1)) as usize];
                }
                self.fVec55[1] = self.fVec55[0];
                self.fVec56[1] = self.fVec56[0];
                self.fVec57[1] = self.fVec57[0];
                self.fRec15[1] = self.fRec15[0];
                self.fRec16[1] = self.fRec16[0];
            }
        }
    }

    impl FaustDsp for LambRs48k {
        type T = FaustFloat;
        fn new() -> Self
        where
            Self: Sized,
        {
            Self::new()
        }
        fn metadata(&self, m: &mut dyn Meta) {
            self.metadata(m)
        }
        fn get_sample_rate(&self) -> i32 {
            self.get_sample_rate()
        }
        fn get_num_inputs(&self) -> i32 {
            FAUST_INPUTS as i32
        }
        fn get_num_outputs(&self) -> i32 {
            FAUST_OUTPUTS as i32
        }
        fn class_init(sample_rate: i32)
        where
            Self: Sized,
        {
            Self::class_init(sample_rate);
        }
        fn instance_reset_params(&mut self) {
            self.instance_reset_params()
        }
        fn instance_clear(&mut self) {
            self.instance_clear()
        }
        fn instance_constants(&mut self, sample_rate: i32) {
            self.instance_constants(sample_rate)
        }
        fn instance_init(&mut self, sample_rate: i32) {
            self.instance_init(sample_rate)
        }
        fn init(&mut self, sample_rate: i32) {
            self.init(sample_rate)
        }
        fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>) {
            self.build_user_interface(ui_interface)
        }
        fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>)
        where
            Self: Sized,
        {
            Self::build_user_interface_static(ui_interface);
        }
        fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
            self.get_param(param)
        }
        fn set_param(&mut self, param: ParamIndex, value: Self::T) {
            self.set_param(param, value)
        }
        fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut [&mut [Self::T]]) {
            self.compute(count as usize, inputs, outputs)
        }
    }
}

pub use dsp_48k::LambRs48k;
