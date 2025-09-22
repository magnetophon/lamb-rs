/* ------------------------------------------------------------
author: "Bart Brouns"
license: "AGPLv3"
name: "lamb-rs"
version: "0.1"
Code generated with Faust 2.79.3 (https://faust.grame.fr)
Compilation options: -a /tmp/.tmptHrx7j -lang rust -ct 1 -cn LambRs192k -es 1 -mcd 16 -mdd 1024 -mdy 33 -double -ftz 0
------------------------------------------------------------ */
mod dsp_192k {
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

    pub struct LambRs192kSIG0 {
        iRec13: [i32; 2],
    }

    impl LambRs192kSIG0 {
        fn get_num_inputsLambRs192kSIG0(&self) -> i32 {
            return 0;
        }
        fn get_num_outputsLambRs192kSIG0(&self) -> i32 {
            return 1;
        }

        pub fn instance_initLambRs192kSIG0(&mut self, sample_rate: i32) {
            for l44 in 0..2 {
                self.iRec13[l44 as usize] = 0;
            }
        }

        pub fn fillLambRs192kSIG0(&mut self, count: i32, table: &mut [FaustFloat]) {
            for i1 in 0..count {
                self.iRec13[0] = i32::wrapping_add(self.iRec13[1], 1);
                let mut fTemp68: F64 = (self.iRec13[1] % 7) as F64 as i32 as F64;
                let mut fTemp69: F64 = 0.16666666666666666 * fTemp68;
                let mut fTemp70: F64 = F64::powf(fTemp69, 0.06999999999999999 * fTemp68 + 1.0);
                let mut fTemp71: F64 =
                    (0.14285714285714285 * (self.iRec13[1] % 3670016) as F64) as i32 as F64;
                table[i1 as usize] = F64::min(
                    1.0,
                    F64::max(
                        0.0,
                        (if (fTemp69 == 0.0) as i32 != 0 {
                            0.5 * (F64::sin(5.992123881747579e-06 * fTemp71 + 4.71238898038469)
                                + 1.0)
                        } else {
                            0.5 * (F64::sin(
                                3.141592653589793
                                    * ((1.0
                                        - F64::exp(-(4.615792495331755e-06 * fTemp70 * fTemp71)))
                                        / (1.0 - F64::exp(-(2.42 * fTemp70))))
                                    + 4.71238898038469,
                            ) + 1.0)
                        }),
                    ),
                );
                self.iRec13[1] = self.iRec13[0];
            }
        }
    }

    pub fn newLambRs192kSIG0() -> LambRs192kSIG0 {
        LambRs192kSIG0 { iRec13: [0; 2] }
    }
    fn LambRs192k_faustpower2_f(value: F64) -> F64 {
        return value * value;
    }
    static mut ftbl0LambRs192kSIG0: [F64; 3670016] = [0.0; 3670016];
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
    pub struct LambRs192k {
        fCheckbox0: F64,
        IOTA0: i32,
        iVec0: [i32; 32768],
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
        fVec17: [F64; 8192],
        fVec18: [F64; 16384],
        fRec3: [F64; 2],
        fVec19: [F64; 3],
        fVec20: [F64; 5],
        fVec21: [F64; 12],
        fVec22: [F64; 32],
        fVec23: [F64; 64],
        fVec24: [F64; 128],
        fVec25: [F64; 256],
        fVec26: [F64; 512],
        fVec27: [F64; 1024],
        fVec28: [F64; 2048],
        fVec29: [F64; 4096],
        fVec30: [F64; 8192],
        fVec31: [F64; 16384],
        fVec32: [F64; 2],
        fHslider10: F64,
        fHslider11: F64,
        fVec33: [F64; 2],
        fVec34: [F64; 2],
        fConst10: F64,
        fRec1: [F64; 2],
        fRec2: [F64; 2],
        fVec35: [F64; 32768],
        fCheckbox1: F64,
        fHbargraph0: F64,
        fHslider12: F64,
        fRec14: [F64; 2],
        fVec36: [F64; 16384],
        fVec37: [F64; 3],
        fVec38: [F64; 5],
        fVec39: [F64; 12],
        fVec40: [F64; 32],
        fVec41: [F64; 64],
        fVec42: [F64; 128],
        fVec43: [F64; 256],
        fVec44: [F64; 512],
        fVec45: [F64; 1024],
        fVec46: [F64; 2048],
        fVec47: [F64; 4096],
        fVec48: [F64; 8192],
        fVec49: [F64; 16384],
        fRec17: [F64; 2],
        fVec50: [F64; 3],
        fVec51: [F64; 5],
        fVec52: [F64; 12],
        fVec53: [F64; 32],
        fVec54: [F64; 64],
        fVec55: [F64; 128],
        fVec56: [F64; 256],
        fVec57: [F64; 512],
        fVec58: [F64; 1024],
        fVec59: [F64; 2048],
        fVec60: [F64; 4096],
        fVec61: [F64; 8192],
        fVec62: [F64; 16384],
        fVec63: [F64; 2],
        fVec64: [F64; 2],
        fVec65: [F64; 2],
        fRec15: [F64; 2],
        fRec16: [F64; 2],
        fVec66: [F64; 32768],
    }

    impl LambRs192k {
        pub fn new() -> LambRs192k {
            LambRs192k {
                fCheckbox0: 0.0,
                IOTA0: 0,
                iVec0: [0; 32768],
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
                fVec17: [0.0; 8192],
                fVec18: [0.0; 16384],
                fRec3: [0.0; 2],
                fVec19: [0.0; 3],
                fVec20: [0.0; 5],
                fVec21: [0.0; 12],
                fVec22: [0.0; 32],
                fVec23: [0.0; 64],
                fVec24: [0.0; 128],
                fVec25: [0.0; 256],
                fVec26: [0.0; 512],
                fVec27: [0.0; 1024],
                fVec28: [0.0; 2048],
                fVec29: [0.0; 4096],
                fVec30: [0.0; 8192],
                fVec31: [0.0; 16384],
                fVec32: [0.0; 2],
                fHslider10: 0.0,
                fHslider11: 0.0,
                fVec33: [0.0; 2],
                fVec34: [0.0; 2],
                fConst10: 0.0,
                fRec1: [0.0; 2],
                fRec2: [0.0; 2],
                fVec35: [0.0; 32768],
                fCheckbox1: 0.0,
                fHbargraph0: 0.0,
                fHslider12: 0.0,
                fRec14: [0.0; 2],
                fVec36: [0.0; 16384],
                fVec37: [0.0; 3],
                fVec38: [0.0; 5],
                fVec39: [0.0; 12],
                fVec40: [0.0; 32],
                fVec41: [0.0; 64],
                fVec42: [0.0; 128],
                fVec43: [0.0; 256],
                fVec44: [0.0; 512],
                fVec45: [0.0; 1024],
                fVec46: [0.0; 2048],
                fVec47: [0.0; 4096],
                fVec48: [0.0; 8192],
                fVec49: [0.0; 16384],
                fRec17: [0.0; 2],
                fVec50: [0.0; 3],
                fVec51: [0.0; 5],
                fVec52: [0.0; 12],
                fVec53: [0.0; 32],
                fVec54: [0.0; 64],
                fVec55: [0.0; 128],
                fVec56: [0.0; 256],
                fVec57: [0.0; 512],
                fVec58: [0.0; 1024],
                fVec59: [0.0; 2048],
                fVec60: [0.0; 4096],
                fVec61: [0.0; 8192],
                fVec62: [0.0; 16384],
                fVec63: [0.0; 2],
                fVec64: [0.0; 2],
                fVec65: [0.0; 2],
                fRec15: [0.0; 2],
                fRec16: [0.0; 2],
                fVec66: [0.0; 32768],
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
            m.declare("compile_options", r"-a /tmp/.tmptHrx7j -lang rust -ct 1 -cn LambRs192k -es 1 -mcd 16 -mdd 1024 -mdy 33 -double -ftz 0");
            m.declare("filename", r"lamb-rs-192k.dsp");
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
            let mut sig0: LambRs192kSIG0 = newLambRs192kSIG0();
            sig0.instance_initLambRs192kSIG0(sample_rate);
            sig0.fillLambRs192kSIG0(3670016, unsafe { &mut ftbl0LambRs192kSIG0 });
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
            for l0 in 0..32768 {
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
            for l27 in 0..8192 {
                self.fVec17[l27 as usize] = 0.0;
            }
            for l28 in 0..16384 {
                self.fVec18[l28 as usize] = 0.0;
            }
            for l29 in 0..2 {
                self.fRec3[l29 as usize] = 0.0;
            }
            for l30 in 0..3 {
                self.fVec19[l30 as usize] = 0.0;
            }
            for l31 in 0..5 {
                self.fVec20[l31 as usize] = 0.0;
            }
            for l32 in 0..12 {
                self.fVec21[l32 as usize] = 0.0;
            }
            for l33 in 0..32 {
                self.fVec22[l33 as usize] = 0.0;
            }
            for l34 in 0..64 {
                self.fVec23[l34 as usize] = 0.0;
            }
            for l35 in 0..128 {
                self.fVec24[l35 as usize] = 0.0;
            }
            for l36 in 0..256 {
                self.fVec25[l36 as usize] = 0.0;
            }
            for l37 in 0..512 {
                self.fVec26[l37 as usize] = 0.0;
            }
            for l38 in 0..1024 {
                self.fVec27[l38 as usize] = 0.0;
            }
            for l39 in 0..2048 {
                self.fVec28[l39 as usize] = 0.0;
            }
            for l40 in 0..4096 {
                self.fVec29[l40 as usize] = 0.0;
            }
            for l41 in 0..8192 {
                self.fVec30[l41 as usize] = 0.0;
            }
            for l42 in 0..16384 {
                self.fVec31[l42 as usize] = 0.0;
            }
            for l43 in 0..2 {
                self.fVec32[l43 as usize] = 0.0;
            }
            for l45 in 0..2 {
                self.fVec33[l45 as usize] = 0.0;
            }
            for l46 in 0..2 {
                self.fVec34[l46 as usize] = 0.0;
            }
            for l47 in 0..2 {
                self.fRec1[l47 as usize] = 0.0;
            }
            for l48 in 0..2 {
                self.fRec2[l48 as usize] = 0.0;
            }
            for l49 in 0..32768 {
                self.fVec35[l49 as usize] = 0.0;
            }
            for l50 in 0..2 {
                self.fRec14[l50 as usize] = 0.0;
            }
            for l51 in 0..16384 {
                self.fVec36[l51 as usize] = 0.0;
            }
            for l52 in 0..3 {
                self.fVec37[l52 as usize] = 0.0;
            }
            for l53 in 0..5 {
                self.fVec38[l53 as usize] = 0.0;
            }
            for l54 in 0..12 {
                self.fVec39[l54 as usize] = 0.0;
            }
            for l55 in 0..32 {
                self.fVec40[l55 as usize] = 0.0;
            }
            for l56 in 0..64 {
                self.fVec41[l56 as usize] = 0.0;
            }
            for l57 in 0..128 {
                self.fVec42[l57 as usize] = 0.0;
            }
            for l58 in 0..256 {
                self.fVec43[l58 as usize] = 0.0;
            }
            for l59 in 0..512 {
                self.fVec44[l59 as usize] = 0.0;
            }
            for l60 in 0..1024 {
                self.fVec45[l60 as usize] = 0.0;
            }
            for l61 in 0..2048 {
                self.fVec46[l61 as usize] = 0.0;
            }
            for l62 in 0..4096 {
                self.fVec47[l62 as usize] = 0.0;
            }
            for l63 in 0..8192 {
                self.fVec48[l63 as usize] = 0.0;
            }
            for l64 in 0..16384 {
                self.fVec49[l64 as usize] = 0.0;
            }
            for l65 in 0..2 {
                self.fRec17[l65 as usize] = 0.0;
            }
            for l66 in 0..3 {
                self.fVec50[l66 as usize] = 0.0;
            }
            for l67 in 0..5 {
                self.fVec51[l67 as usize] = 0.0;
            }
            for l68 in 0..12 {
                self.fVec52[l68 as usize] = 0.0;
            }
            for l69 in 0..32 {
                self.fVec53[l69 as usize] = 0.0;
            }
            for l70 in 0..64 {
                self.fVec54[l70 as usize] = 0.0;
            }
            for l71 in 0..128 {
                self.fVec55[l71 as usize] = 0.0;
            }
            for l72 in 0..256 {
                self.fVec56[l72 as usize] = 0.0;
            }
            for l73 in 0..512 {
                self.fVec57[l73 as usize] = 0.0;
            }
            for l74 in 0..1024 {
                self.fVec58[l74 as usize] = 0.0;
            }
            for l75 in 0..2048 {
                self.fVec59[l75 as usize] = 0.0;
            }
            for l76 in 0..4096 {
                self.fVec60[l76 as usize] = 0.0;
            }
            for l77 in 0..8192 {
                self.fVec61[l77 as usize] = 0.0;
            }
            for l78 in 0..16384 {
                self.fVec62[l78 as usize] = 0.0;
            }
            for l79 in 0..2 {
                self.fVec63[l79 as usize] = 0.0;
            }
            for l80 in 0..2 {
                self.fVec64[l80 as usize] = 0.0;
            }
            for l81 in 0..2 {
                self.fVec65[l81 as usize] = 0.0;
            }
            for l82 in 0..2 {
                self.fRec15[l82 as usize] = 0.0;
            }
            for l83 in 0..2 {
                self.fRec16[l83 as usize] = 0.0;
            }
            for l84 in 0..32768 {
                self.fVec66[l84 as usize] = 0.0;
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
            LambRs192k::class_init(sample_rate);
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
                0.005208333333333333,
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
            ui_interface.add_horizontal_bargraph("latency", ParamIndex(15), 0.0, 1.92e+04);
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
            let mut iSlow45: i32 = (F64::floor(0.000244140625 * fSlow22)) as i32 % 2;
            let mut iSlow46: i32 = i32::wrapping_add(iSlow44, i32::wrapping_mul(2048, iSlow43));
            let mut iSlow47: i32 = (F64::floor(0.0001220703125 * fSlow22)) as i32 % 2;
            let mut iSlow48: i32 = i32::wrapping_add(iSlow46, i32::wrapping_mul(4096, iSlow45));
            let mut iSlow49: i32 = (F64::floor(0.5 * fSlow3)) as i32 % 2;
            let mut iSlow50: i32 = (F64::floor(0.25 * fSlow3)) as i32 % 2;
            let mut iSlow51: i32 = i32::wrapping_add(iSlow4, i32::wrapping_mul(2, iSlow49));
            let mut iSlow52: i32 = (F64::floor(0.125 * fSlow3)) as i32 % 2;
            let mut iSlow53: i32 = i32::wrapping_add(iSlow51, i32::wrapping_mul(4, iSlow50));
            let mut iSlow54: i32 = (F64::floor(0.0625 * fSlow3)) as i32 % 2;
            let mut iSlow55: i32 = i32::wrapping_add(iSlow53, i32::wrapping_mul(8, iSlow52));
            let mut iSlow56: i32 = (F64::floor(0.03125 * fSlow3)) as i32 % 2;
            let mut iSlow57: i32 = i32::wrapping_add(iSlow55, i32::wrapping_mul(16, iSlow54));
            let mut iSlow58: i32 = (F64::floor(0.015625 * fSlow3)) as i32 % 2;
            let mut iSlow59: i32 = i32::wrapping_add(iSlow57, i32::wrapping_mul(32, iSlow56));
            let mut iSlow60: i32 = (F64::floor(0.0078125 * fSlow3)) as i32 % 2;
            let mut iSlow61: i32 = i32::wrapping_add(iSlow59, i32::wrapping_mul(64, iSlow58));
            let mut iSlow62: i32 = (F64::floor(0.00390625 * fSlow3)) as i32 % 2;
            let mut iSlow63: i32 = i32::wrapping_add(iSlow61, i32::wrapping_mul(128, iSlow60));
            let mut iSlow64: i32 = (F64::floor(0.001953125 * fSlow3)) as i32 % 2;
            let mut iSlow65: i32 = i32::wrapping_add(iSlow63, i32::wrapping_mul(256, iSlow62));
            let mut iSlow66: i32 = (F64::floor(0.0009765625 * fSlow3)) as i32 % 2;
            let mut iSlow67: i32 = i32::wrapping_add(iSlow65, i32::wrapping_mul(512, iSlow64));
            let mut iSlow68: i32 = (F64::floor(0.00048828125 * fSlow3)) as i32 % 2;
            let mut iSlow69: i32 = i32::wrapping_add(iSlow67, i32::wrapping_mul(1024, iSlow66));
            let mut iSlow70: i32 = (F64::floor(0.000244140625 * fSlow3)) as i32 % 2;
            let mut iSlow71: i32 = i32::wrapping_add(iSlow69, i32::wrapping_mul(2048, iSlow68));
            let mut iSlow72: i32 = (F64::floor(0.0001220703125 * fSlow3)) as i32 % 2;
            let mut iSlow73: i32 = i32::wrapping_add(iSlow71, i32::wrapping_mul(4096, iSlow70));
            let mut fSlow74: F64 = self.fHslider10;
            let mut fSlow75: F64 = self.fHslider11;
            let mut fSlow76: F64 = self.fConst0 * (0.001 * fSlow19 + 1e-05 * fSlow2);
            let mut fSlow77: F64 = self.fCheckbox1;
            let mut iSlow78: i32 = (F64::max(0.0, fSlow77 * (1.92e+04 - fSlow76))) as i32;
            self.fHbargraph0 = (if (fSlow77) as i32 != 0 {
                1.92e+04
            } else {
                fSlow76
            });
            let mut iSlow79: i32 = (self.fHbargraph0) as i32;
            let mut fSlow80: F64 = self.fConst3 * F64::powf(1e+01, 0.05 * self.fHslider12);
            let zipped_iterators = inputs0
                .zip(inputs1)
                .zip(outputs0)
                .zip(outputs1)
                .zip(outputs2)
                .zip(outputs3);
            for (((((input0, input1), output0), output1), output2), output3) in zipped_iterators {
                self.iVec0[(self.IOTA0 & 32767) as usize] = 1;
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
                                    fSlow12 * LambRs192k_faustpower2_f(fSlow7 + fTemp20)
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
                    (self.iVec0[((i32::wrapping_sub(self.IOTA0, 19200)) & 32767) as usize]) as F64;
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
                                    fSlow12 * LambRs192k_faustpower2_f(fSlow7 + fTemp40)
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
                let mut fTemp52: F64 = F64::min(
                    fTemp51,
                    self.fVec15[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize],
                );
                self.fVec16[(self.IOTA0 & 4095) as usize] = fTemp52;
                let mut fTemp53: F64 = F64::min(
                    fTemp52,
                    self.fVec16[((i32::wrapping_sub(self.IOTA0, 2048)) & 4095) as usize],
                );
                self.fVec17[(self.IOTA0 & 8191) as usize] = fTemp53;
                self.fVec18[(self.IOTA0 & 16383) as usize] = F64::min(
                    fTemp53,
                    self.fVec17[((i32::wrapping_sub(self.IOTA0, 4096)) & 8191) as usize],
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
                                                                F64::min(
                                                                    F64::min(
                                                                        (if iSlow23 != 0 {
                                                                            fTemp41
                                                                        } else {
                                                                            1.7976931348623157e+308
                                                                        }),
                                                                        (if iSlow24 != 0 {
                                                                            self.fVec6
                                                                                [iSlow23 as usize]
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
                                                    self.fVec12[((i32::wrapping_sub(
                                                        self.IOTA0, iSlow36,
                                                    )) & 255)
                                                        as usize]
                                                } else {
                                                    1.7976931348623157e+308
                                                }),
                                            ),
                                            (if iSlow37 != 0 {
                                                self.fVec13[((i32::wrapping_sub(
                                                    self.IOTA0, iSlow38,
                                                )) & 511)
                                                    as usize]
                                            } else {
                                                1.7976931348623157e+308
                                            }),
                                        ),
                                        (if iSlow39 != 0 {
                                            self.fVec14[((i32::wrapping_sub(self.IOTA0, iSlow40))
                                                & 1023)
                                                as usize]
                                        } else {
                                            1.7976931348623157e+308
                                        }),
                                    ),
                                    (if iSlow41 != 0 {
                                        self.fVec15[((i32::wrapping_sub(self.IOTA0, iSlow42))
                                            & 2047)
                                            as usize]
                                    } else {
                                        1.7976931348623157e+308
                                    }),
                                ),
                                (if iSlow43 != 0 {
                                    self.fVec16
                                        [((i32::wrapping_sub(self.IOTA0, iSlow44)) & 4095) as usize]
                                } else {
                                    1.7976931348623157e+308
                                }),
                            ),
                            (if iSlow45 != 0 {
                                self.fVec17
                                    [((i32::wrapping_sub(self.IOTA0, iSlow46)) & 8191) as usize]
                            } else {
                                1.7976931348623157e+308
                            }),
                        ),
                        (if iSlow47 != 0 {
                            self.fVec18[((i32::wrapping_sub(self.IOTA0, iSlow48)) & 16383) as usize]
                        } else {
                            1.7976931348623157e+308
                        }),
                    ),
                );
                let mut fTemp54: F64 = F64::min(self.fRec3[0], self.fRec3[1]);
                self.fVec19[0] = fTemp54;
                let mut fTemp55: F64 = F64::min(fTemp54, self.fVec19[2]);
                self.fVec20[0] = fTemp55;
                let mut fTemp56: F64 = F64::min(fTemp55, self.fVec20[4]);
                self.fVec21[0] = fTemp56;
                let mut fTemp57: F64 = F64::min(fTemp56, self.fVec21[8]);
                self.fVec22[(self.IOTA0 & 31) as usize] = fTemp57;
                let mut fTemp58: F64 = F64::min(
                    fTemp57,
                    self.fVec22[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize],
                );
                self.fVec23[(self.IOTA0 & 63) as usize] = fTemp58;
                let mut fTemp59: F64 = F64::min(
                    fTemp58,
                    self.fVec23[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize],
                );
                self.fVec24[(self.IOTA0 & 127) as usize] = fTemp59;
                let mut fTemp60: F64 = F64::min(
                    fTemp59,
                    self.fVec24[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize],
                );
                self.fVec25[(self.IOTA0 & 255) as usize] = fTemp60;
                let mut fTemp61: F64 = F64::min(
                    fTemp60,
                    self.fVec25[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize],
                );
                self.fVec26[(self.IOTA0 & 511) as usize] = fTemp61;
                let mut fTemp62: F64 = F64::min(
                    fTemp61,
                    self.fVec26[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize],
                );
                self.fVec27[(self.IOTA0 & 1023) as usize] = fTemp62;
                let mut fTemp63: F64 = F64::min(
                    fTemp62,
                    self.fVec27[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize],
                );
                self.fVec28[(self.IOTA0 & 2047) as usize] = fTemp63;
                let mut fTemp64: F64 = F64::min(
                    fTemp63,
                    self.fVec28[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize],
                );
                self.fVec29[(self.IOTA0 & 4095) as usize] = fTemp64;
                let mut fTemp65: F64 = F64::min(
                    fTemp64,
                    self.fVec29[((i32::wrapping_sub(self.IOTA0, 2048)) & 4095) as usize],
                );
                self.fVec30[(self.IOTA0 & 8191) as usize] = fTemp65;
                self.fVec31[(self.IOTA0 & 16383) as usize] = F64::min(
                    fTemp65,
                    self.fVec30[((i32::wrapping_sub(self.IOTA0, 4096)) & 8191) as usize],
                );
                let mut fTemp66: F64 = F64::min(
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
                                                                F64::min(
                                                                    (if iSlow4 != 0 {
                                                                        self.fRec3[0]
                                                                    } else {
                                                                        1.7976931348623157e+308
                                                                    }),
                                                                    (if iSlow49 != 0 {
                                                                        self.fVec19[iSlow4 as usize]
                                                                    } else {
                                                                        1.7976931348623157e+308
                                                                    }),
                                                                ),
                                                                (if iSlow50 != 0 {
                                                                    self.fVec20[iSlow51 as usize]
                                                                } else {
                                                                    1.7976931348623157e+308
                                                                }),
                                                            ),
                                                            (if iSlow52 != 0 {
                                                                self.fVec21[iSlow53 as usize]
                                                            } else {
                                                                1.7976931348623157e+308
                                                            }),
                                                        ),
                                                        (if iSlow54 != 0 {
                                                            self.fVec22[((i32::wrapping_sub(
                                                                self.IOTA0, iSlow55,
                                                            )) & 31)
                                                                as usize]
                                                        } else {
                                                            1.7976931348623157e+308
                                                        }),
                                                    ),
                                                    (if iSlow56 != 0 {
                                                        self.fVec23[((i32::wrapping_sub(
                                                            self.IOTA0, iSlow57,
                                                        )) & 63)
                                                            as usize]
                                                    } else {
                                                        1.7976931348623157e+308
                                                    }),
                                                ),
                                                (if iSlow58 != 0 {
                                                    self.fVec24[((i32::wrapping_sub(
                                                        self.IOTA0, iSlow59,
                                                    )) & 127)
                                                        as usize]
                                                } else {
                                                    1.7976931348623157e+308
                                                }),
                                            ),
                                            (if iSlow60 != 0 {
                                                self.fVec25[((i32::wrapping_sub(
                                                    self.IOTA0, iSlow61,
                                                )) & 255)
                                                    as usize]
                                            } else {
                                                1.7976931348623157e+308
                                            }),
                                        ),
                                        (if iSlow62 != 0 {
                                            self.fVec26[((i32::wrapping_sub(self.IOTA0, iSlow63))
                                                & 511)
                                                as usize]
                                        } else {
                                            1.7976931348623157e+308
                                        }),
                                    ),
                                    (if iSlow64 != 0 {
                                        self.fVec27[((i32::wrapping_sub(self.IOTA0, iSlow65))
                                            & 1023)
                                            as usize]
                                    } else {
                                        1.7976931348623157e+308
                                    }),
                                ),
                                (if iSlow66 != 0 {
                                    self.fVec28
                                        [((i32::wrapping_sub(self.IOTA0, iSlow67)) & 2047) as usize]
                                } else {
                                    1.7976931348623157e+308
                                }),
                            ),
                            (if iSlow68 != 0 {
                                self.fVec29
                                    [((i32::wrapping_sub(self.IOTA0, iSlow69)) & 4095) as usize]
                            } else {
                                1.7976931348623157e+308
                            }),
                        ),
                        (if iSlow70 != 0 {
                            self.fVec30[((i32::wrapping_sub(self.IOTA0, iSlow71)) & 8191) as usize]
                        } else {
                            1.7976931348623157e+308
                        }),
                    ),
                    (if iSlow72 != 0 {
                        self.fVec31[((i32::wrapping_sub(self.IOTA0, iSlow73)) & 16383) as usize]
                    } else {
                        1.7976931348623157e+308
                    }),
                ) - self.fRec2[1];
                self.fVec32[0] = fTemp66;
                let mut iTemp67: i32 = (fTemp66 > 0.0) as i32;
                let mut fTemp72: F64 = (if iTemp67 != 0 { fSlow75 } else { fSlow74 });
                self.fVec33[0] = fTemp72;
                let mut fTemp73: F64 = 6.0 * fTemp72;
                let mut iTemp74: i32 = (fTemp73) as i32;
                let mut iTemp75: i32 = std::cmp::max(0, std::cmp::min(iTemp74, 6));
                let mut iTemp76: i32 = std::cmp::max(
                    0,
                    std::cmp::min(i32::wrapping_add(iTemp75, 1835001), 3670015),
                );
                let mut fTemp77: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp76, 7)) as usize] };
                let mut fTemp78: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp76 as usize] };
                let mut fTemp79: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp76, 1)) as usize] }
                        - fTemp78;
                let mut fTemp80: F64 = fTemp73 - (iTemp74) as F64;
                let mut fTemp81: F64 = fTemp78
                    + fTemp80 * fTemp79
                    + 0.5
                        * (fTemp77
                            - (fTemp78
                                + fTemp80
                                    * (fTemp79
                                        - (unsafe {
                                            ftbl0LambRs192kSIG0
                                                [(i32::wrapping_add(iTemp76, 8)) as usize]
                                        } - fTemp77))));
                let mut fTemp82: F64 = (if iTemp67 != 0 { fTemp81 } else { 1.0 - fTemp81 });
                let mut iTemp83: i32 = (fTemp66 < 0.0) as i32;
                let mut fTemp84: F64 = fSlow1 * (iTemp83) as F64 + fSlow13 * (iTemp67) as F64;
                self.fVec34[0] = fTemp84;
                let mut fTemp85: F64 = self.fConst10 / fTemp84;
                let mut fTemp86: F64 = fTemp85 + 0.5;
                let mut fTemp87: F64 = 524287.0 * (1.0 - fTemp86);
                let mut iTemp88: i32 = (fTemp87) as i32;
                let mut iTemp89: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp88, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp90: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp89, 7)) as usize] };
                let mut fTemp91: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp89 as usize] };
                let mut fTemp92: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp89, 1)) as usize] }
                        - fTemp91;
                let mut fTemp93: F64 = 524287.0 * fTemp86;
                let mut iTemp94: i32 = (fTemp93) as i32;
                let mut iTemp95: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp94, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp96: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp95, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp97: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp95 as usize] };
                let mut fTemp98: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp95, 1), 3670015),
                    )) as usize]
                } - fTemp97;
                let mut fTemp99: F64 = 6.0 * self.fVec33[1];
                let mut iTemp100: i32 = (fTemp99) as i32;
                let mut iTemp101: i32 = std::cmp::max(0, std::cmp::min(iTemp100, 6));
                let mut fTemp102: F64 = 524287.0 * (1.0 - self.fRec1[1]);
                let mut iTemp103: i32 = (fTemp102) as i32;
                let mut iTemp104: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp103, 524287))),
                            iTemp101,
                        ),
                        3670015,
                    ),
                );
                let mut fTemp105: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp104, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp106: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp104 as usize] };
                let mut fTemp107: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp104, 1), 3670015),
                    )) as usize]
                } - fTemp106;
                let mut fTemp108: F64 = fTemp99 - (iTemp100) as F64;
                let mut fTemp109: F64 = 524287.0 * self.fRec1[1];
                let mut iTemp110: i32 = (fTemp109) as i32;
                let mut iTemp111: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp101,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp110, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp112: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp111, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp113: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp111 as usize] };
                let mut fTemp114: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp111, 1), 3670015),
                    )) as usize]
                } - fTemp113;
                let mut fTemp115: F64 = self.fRec1[1] + fTemp85;
                let mut fTemp116: F64 = 524287.0 * (1.0 - fTemp115);
                let mut iTemp117: i32 = (fTemp116) as i32;
                let mut iTemp118: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp117, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp119: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp118, 7)) as usize] };
                let mut fTemp120: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp118 as usize] };
                let mut fTemp121: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp118, 1)) as usize] }
                        - fTemp120;
                let mut fTemp122: F64 = 524287.0 * fTemp115;
                let mut iTemp123: i32 = (fTemp122) as i32;
                let mut iTemp124: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp123, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp125: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp124, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp126: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp124 as usize] };
                let mut fTemp127: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp124, 1), 3670015),
                    )) as usize]
                } - fTemp126;
                let mut fTemp128: F64 =
                    self.fRec1[1] + self.fConst10 * (1.0 / fTemp84 + 1.0 / self.fVec34[1]);
                let mut fTemp129: F64 = 524287.0 * (1.0 - fTemp128);
                let mut iTemp130: i32 = (fTemp129) as i32;
                let mut iTemp131: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp130, 524287))),
                            iTemp75,
                        ),
                        3670015,
                    ),
                );
                let mut fTemp132: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp131, 7)) as usize] };
                let mut fTemp133: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp131 as usize] };
                let mut fTemp134: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp131, 1)) as usize] }
                        - fTemp133;
                let mut fTemp135: F64 = 524287.0 * fTemp128;
                let mut iTemp136: i32 = (fTemp135) as i32;
                let mut iTemp137: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp136, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp138: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp137, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp139: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp137 as usize] };
                let mut fTemp140: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp137, 1), 3670015),
                    )) as usize]
                } - fTemp139;
                let mut fTemp141: F64 = ((if iTemp67 != 0 {
                    fTemp139
                        + fTemp80 * fTemp140
                        + (fTemp135 - (iTemp136) as F64)
                            * (fTemp138
                                - (fTemp139
                                    + fTemp80
                                        * (fTemp140
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp137, 8),
                                                        3670015,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp138))))
                } else {
                    1.0 - (fTemp133
                        + fTemp80 * fTemp134
                        + (fTemp129 - (iTemp130) as F64)
                            * (fTemp132
                                - (fTemp133
                                    + fTemp80
                                        * (fTemp134
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp131, 8)) as usize]
                                            } - fTemp132)))))
                }) - (if iTemp67 != 0 {
                    fTemp126
                        + fTemp80 * fTemp127
                        + (fTemp122 - (iTemp123) as F64)
                            * (fTemp125
                                - (fTemp126
                                    + fTemp80
                                        * (fTemp127
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp124, 8),
                                                        3670015,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp125))))
                } else {
                    1.0 - (fTemp120
                        + fTemp80 * fTemp121
                        + (fTemp116 - (iTemp117) as F64)
                            * (fTemp119
                                - (fTemp120
                                    + fTemp80
                                        * (fTemp121
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp118, 8)) as usize]
                                            } - fTemp119)))))
                })) * self.fVec32[1]
                    / (fTemp66
                        * (1.0
                            - (if iTemp67 != 0 {
                                fTemp113
                                    + fTemp108 * fTemp114
                                    + (fTemp109 - (iTemp110) as F64)
                                        * (fTemp112
                                            - (fTemp113
                                                + fTemp108
                                                    * (fTemp114
                                                        - (unsafe {
                                                            ftbl0LambRs192kSIG0[(std::cmp::max(
                                                                0,
                                                                std::cmp::min(
                                                                    i32::wrapping_add(iTemp111, 8),
                                                                    3670015,
                                                                ),
                                                            ))
                                                                as usize]
                                                        } - fTemp112))))
                            } else {
                                1.0 - (fTemp106
                                    + fTemp108 * fTemp107
                                    + (fTemp102 - (iTemp103) as F64)
                                        * (fTemp105
                                            - (fTemp106
                                                + fTemp108
                                                    * (fTemp107
                                                        - (unsafe {
                                                            ftbl0LambRs192kSIG0[(std::cmp::max(
                                                                0,
                                                                std::cmp::min(
                                                                    i32::wrapping_add(iTemp104, 8),
                                                                    3670015,
                                                                ),
                                                            ))
                                                                as usize]
                                                        } - fTemp105)))))
                            })));
                let mut iTemp142: i32 = (fTemp141
                    > (((if iTemp67 != 0 {
                        fTemp97
                            + fTemp80 * fTemp98
                            + (fTemp93 - (iTemp94) as F64)
                                * (fTemp96
                                    - (fTemp97
                                        + fTemp80
                                            * (fTemp98
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp95, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp96))))
                    } else {
                        1.0 - (fTemp91
                            + fTemp80 * fTemp92
                            + (fTemp87 - (iTemp88) as F64)
                                * (fTemp90
                                    - (fTemp91
                                        + fTemp80
                                            * (fTemp92
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp89, 8)) as usize]
                                                } - fTemp90)))))
                    }) - fTemp82)
                        / (1.0 - fTemp82))) as i32;
                let mut fTemp143: F64 = (if iTemp142 != 0 { 1.0 } else { 0.5 });
                let mut fTemp144: F64 = (if iTemp142 != 0 { 0.5 } else { 0.0 });
                let mut fTemp145: F64 = fTemp144 + fTemp143;
                let mut fTemp146: F64 = 0.5 * fTemp145;
                let mut fTemp147: F64 = 524287.0 * (1.0 - fTemp146);
                let mut iTemp148: i32 = (fTemp147) as i32;
                let mut iTemp149: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp148, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp150: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp149, 7)) as usize] };
                let mut fTemp151: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp149 as usize] };
                let mut fTemp152: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp149, 1)) as usize] }
                        - fTemp151;
                let mut fTemp153: F64 = 262143.5 * fTemp145;
                let mut iTemp154: i32 = (fTemp153) as i32;
                let mut iTemp155: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp154, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp156: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp155, 7)) as usize] };
                let mut fTemp157: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp155 as usize] };
                let mut fTemp158: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp155, 1)) as usize] }
                        - fTemp157;
                let mut fTemp159: F64 = (if iTemp67 != 0 {
                    fTemp157
                        + fTemp80 * fTemp158
                        + (fTemp153 - (iTemp154) as F64)
                            * (fTemp156
                                - (fTemp157
                                    + fTemp80
                                        * (fTemp158
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp155, 8)) as usize]
                                            } - fTemp156))))
                } else {
                    1.0 - (fTemp151
                        + fTemp80 * fTemp152
                        + (fTemp147 - (iTemp148) as F64)
                            * (fTemp150
                                - (fTemp151
                                    + fTemp80
                                        * (fTemp152
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp149, 8)) as usize]
                                            } - fTemp150)))))
                });
                let mut fTemp160: F64 = fTemp85 + fTemp146;
                let mut fTemp161: F64 = 524287.0 * (1.0 - fTemp160);
                let mut iTemp162: i32 = (fTemp161) as i32;
                let mut iTemp163: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp162, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp164: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp163, 7)) as usize] };
                let mut fTemp165: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp163 as usize] };
                let mut fTemp166: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp163, 1)) as usize] }
                        - fTemp165;
                let mut fTemp167: F64 = 524287.0 * fTemp160;
                let mut iTemp168: i32 = (fTemp167) as i32;
                let mut iTemp169: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp168, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp170: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp169, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp171: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp169 as usize] };
                let mut fTemp172: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp169, 1), 3670015),
                    )) as usize]
                } - fTemp171;
                let mut iTemp173: i32 = (fTemp141
                    > (((if iTemp67 != 0 {
                        fTemp171
                            + fTemp80 * fTemp172
                            + (fTemp167 - (iTemp168) as F64)
                                * (fTemp170
                                    - (fTemp171
                                        + fTemp80
                                            * (fTemp172
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp169, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp170))))
                    } else {
                        1.0 - (fTemp165
                            + fTemp80 * fTemp166
                            + (fTemp161 - (iTemp162) as F64)
                                * (fTemp164
                                    - (fTemp165
                                        + fTemp80
                                            * (fTemp166
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp163, 8)) as usize]
                                                } - fTemp164)))))
                    }) - fTemp159)
                        / (1.0 - fTemp159))) as i32;
                let mut fTemp174: F64 = (if iTemp173 != 0 { fTemp143 } else { fTemp146 });
                let mut fTemp175: F64 = (if iTemp173 != 0 { fTemp146 } else { fTemp144 });
                let mut fTemp176: F64 = fTemp175 + fTemp174;
                let mut fTemp177: F64 = 0.5 * fTemp176;
                let mut fTemp178: F64 = 524287.0 * (1.0 - fTemp177);
                let mut iTemp179: i32 = (fTemp178) as i32;
                let mut iTemp180: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp179, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp181: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp180, 7)) as usize] };
                let mut fTemp182: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp180 as usize] };
                let mut fTemp183: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp180, 1)) as usize] }
                        - fTemp182;
                let mut fTemp184: F64 = 262143.5 * fTemp176;
                let mut iTemp185: i32 = (fTemp184) as i32;
                let mut iTemp186: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp185, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp187: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp186, 7)) as usize] };
                let mut fTemp188: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp186 as usize] };
                let mut fTemp189: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp186, 1)) as usize] }
                        - fTemp188;
                let mut fTemp190: F64 = (if iTemp67 != 0 {
                    fTemp188
                        + fTemp80 * fTemp189
                        + (fTemp184 - (iTemp185) as F64)
                            * (fTemp187
                                - (fTemp188
                                    + fTemp80
                                        * (fTemp189
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp186, 8)) as usize]
                                            } - fTemp187))))
                } else {
                    1.0 - (fTemp182
                        + fTemp80 * fTemp183
                        + (fTemp178 - (iTemp179) as F64)
                            * (fTemp181
                                - (fTemp182
                                    + fTemp80
                                        * (fTemp183
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp180, 8)) as usize]
                                            } - fTemp181)))))
                });
                let mut fTemp191: F64 = fTemp85 + fTemp177;
                let mut fTemp192: F64 = 524287.0 * (1.0 - fTemp191);
                let mut iTemp193: i32 = (fTemp192) as i32;
                let mut iTemp194: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp193, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp195: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp194, 7)) as usize] };
                let mut fTemp196: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp194 as usize] };
                let mut fTemp197: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp194, 1)) as usize] }
                        - fTemp196;
                let mut fTemp198: F64 = 524287.0 * fTemp191;
                let mut iTemp199: i32 = (fTemp198) as i32;
                let mut iTemp200: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp199, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp201: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp200, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp202: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp200 as usize] };
                let mut fTemp203: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp200, 1), 3670015),
                    )) as usize]
                } - fTemp202;
                let mut iTemp204: i32 = (fTemp141
                    > (((if iTemp67 != 0 {
                        fTemp202
                            + fTemp80 * fTemp203
                            + (fTemp198 - (iTemp199) as F64)
                                * (fTemp201
                                    - (fTemp202
                                        + fTemp80
                                            * (fTemp203
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp200, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp201))))
                    } else {
                        1.0 - (fTemp196
                            + fTemp80 * fTemp197
                            + (fTemp192 - (iTemp193) as F64)
                                * (fTemp195
                                    - (fTemp196
                                        + fTemp80
                                            * (fTemp197
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp194, 8)) as usize]
                                                } - fTemp195)))))
                    }) - fTemp190)
                        / (1.0 - fTemp190))) as i32;
                let mut fTemp205: F64 = (if iTemp204 != 0 { fTemp174 } else { fTemp177 });
                let mut fTemp206: F64 = (if iTemp204 != 0 { fTemp177 } else { fTemp175 });
                let mut fTemp207: F64 = fTemp206 + fTemp205;
                let mut fTemp208: F64 = 0.5 * fTemp207;
                let mut fTemp209: F64 = 524287.0 * (1.0 - fTemp208);
                let mut iTemp210: i32 = (fTemp209) as i32;
                let mut iTemp211: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp210, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp212: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp211, 7)) as usize] };
                let mut fTemp213: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp211 as usize] };
                let mut fTemp214: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp211, 1)) as usize] }
                        - fTemp213;
                let mut fTemp215: F64 = 262143.5 * fTemp207;
                let mut iTemp216: i32 = (fTemp215) as i32;
                let mut iTemp217: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp216, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp218: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp217, 7)) as usize] };
                let mut fTemp219: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp217 as usize] };
                let mut fTemp220: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp217, 1)) as usize] }
                        - fTemp219;
                let mut fTemp221: F64 = (if iTemp67 != 0 {
                    fTemp219
                        + fTemp80 * fTemp220
                        + (fTemp215 - (iTemp216) as F64)
                            * (fTemp218
                                - (fTemp219
                                    + fTemp80
                                        * (fTemp220
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp217, 8)) as usize]
                                            } - fTemp218))))
                } else {
                    1.0 - (fTemp213
                        + fTemp80 * fTemp214
                        + (fTemp209 - (iTemp210) as F64)
                            * (fTemp212
                                - (fTemp213
                                    + fTemp80
                                        * (fTemp214
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp211, 8)) as usize]
                                            } - fTemp212)))))
                });
                let mut fTemp222: F64 = fTemp85 + fTemp208;
                let mut fTemp223: F64 = 524287.0 * (1.0 - fTemp222);
                let mut iTemp224: i32 = (fTemp223) as i32;
                let mut iTemp225: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp224, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp226: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp225, 7)) as usize] };
                let mut fTemp227: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp225 as usize] };
                let mut fTemp228: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp225, 1)) as usize] }
                        - fTemp227;
                let mut fTemp229: F64 = 524287.0 * fTemp222;
                let mut iTemp230: i32 = (fTemp229) as i32;
                let mut iTemp231: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp230, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp232: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp231, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp233: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp231 as usize] };
                let mut fTemp234: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp231, 1), 3670015),
                    )) as usize]
                } - fTemp233;
                let mut iTemp235: i32 = (fTemp141
                    > (((if iTemp67 != 0 {
                        fTemp233
                            + fTemp80 * fTemp234
                            + (fTemp229 - (iTemp230) as F64)
                                * (fTemp232
                                    - (fTemp233
                                        + fTemp80
                                            * (fTemp234
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp231, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp232))))
                    } else {
                        1.0 - (fTemp227
                            + fTemp80 * fTemp228
                            + (fTemp223 - (iTemp224) as F64)
                                * (fTemp226
                                    - (fTemp227
                                        + fTemp80
                                            * (fTemp228
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp225, 8)) as usize]
                                                } - fTemp226)))))
                    }) - fTemp221)
                        / (1.0 - fTemp221))) as i32;
                let mut fTemp236: F64 = (if iTemp235 != 0 { fTemp205 } else { fTemp208 });
                let mut fTemp237: F64 = (if iTemp235 != 0 { fTemp208 } else { fTemp206 });
                let mut fTemp238: F64 = fTemp237 + fTemp236;
                let mut fTemp239: F64 = 0.5 * fTemp238;
                let mut fTemp240: F64 = 524287.0 * (1.0 - fTemp239);
                let mut iTemp241: i32 = (fTemp240) as i32;
                let mut iTemp242: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp241, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp243: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp242, 7)) as usize] };
                let mut fTemp244: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp242 as usize] };
                let mut fTemp245: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp242, 1)) as usize] }
                        - fTemp244;
                let mut fTemp246: F64 = 262143.5 * fTemp238;
                let mut iTemp247: i32 = (fTemp246) as i32;
                let mut iTemp248: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp247, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp249: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp248, 7)) as usize] };
                let mut fTemp250: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp248 as usize] };
                let mut fTemp251: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp248, 1)) as usize] }
                        - fTemp250;
                let mut fTemp252: F64 = (if iTemp67 != 0 {
                    fTemp250
                        + fTemp80 * fTemp251
                        + (fTemp246 - (iTemp247) as F64)
                            * (fTemp249
                                - (fTemp250
                                    + fTemp80
                                        * (fTemp251
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp248, 8)) as usize]
                                            } - fTemp249))))
                } else {
                    1.0 - (fTemp244
                        + fTemp80 * fTemp245
                        + (fTemp240 - (iTemp241) as F64)
                            * (fTemp243
                                - (fTemp244
                                    + fTemp80
                                        * (fTemp245
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp242, 8)) as usize]
                                            } - fTemp243)))))
                });
                let mut fTemp253: F64 = fTemp85 + fTemp239;
                let mut fTemp254: F64 = 524287.0 * (1.0 - fTemp253);
                let mut iTemp255: i32 = (fTemp254) as i32;
                let mut iTemp256: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp255, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp257: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp256, 7)) as usize] };
                let mut fTemp258: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp256 as usize] };
                let mut fTemp259: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp256, 1)) as usize] }
                        - fTemp258;
                let mut fTemp260: F64 = 524287.0 * fTemp253;
                let mut iTemp261: i32 = (fTemp260) as i32;
                let mut iTemp262: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp261, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp263: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp262, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp264: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp262 as usize] };
                let mut fTemp265: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp262, 1), 3670015),
                    )) as usize]
                } - fTemp264;
                let mut iTemp266: i32 = (fTemp141
                    > (((if iTemp67 != 0 {
                        fTemp264
                            + fTemp80 * fTemp265
                            + (fTemp260 - (iTemp261) as F64)
                                * (fTemp263
                                    - (fTemp264
                                        + fTemp80
                                            * (fTemp265
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp262, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp263))))
                    } else {
                        1.0 - (fTemp258
                            + fTemp80 * fTemp259
                            + (fTemp254 - (iTemp255) as F64)
                                * (fTemp257
                                    - (fTemp258
                                        + fTemp80
                                            * (fTemp259
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp256, 8)) as usize]
                                                } - fTemp257)))))
                    }) - fTemp252)
                        / (1.0 - fTemp252))) as i32;
                let mut fTemp267: F64 = (if iTemp266 != 0 { fTemp236 } else { fTemp239 });
                let mut fTemp268: F64 = (if iTemp266 != 0 { fTemp239 } else { fTemp237 });
                let mut fTemp269: F64 = fTemp268 + fTemp267;
                let mut fTemp270: F64 = 0.5 * fTemp269;
                let mut fTemp271: F64 = 524287.0 * (1.0 - fTemp270);
                let mut iTemp272: i32 = (fTemp271) as i32;
                let mut iTemp273: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp272, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp274: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp273, 7)) as usize] };
                let mut fTemp275: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp273 as usize] };
                let mut fTemp276: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp273, 1)) as usize] }
                        - fTemp275;
                let mut fTemp277: F64 = 262143.5 * fTemp269;
                let mut iTemp278: i32 = (fTemp277) as i32;
                let mut iTemp279: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp278, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp280: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp279, 7)) as usize] };
                let mut fTemp281: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp279 as usize] };
                let mut fTemp282: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp279, 1)) as usize] }
                        - fTemp281;
                let mut fTemp283: F64 = (if iTemp67 != 0 {
                    fTemp281
                        + fTemp80 * fTemp282
                        + (fTemp277 - (iTemp278) as F64)
                            * (fTemp280
                                - (fTemp281
                                    + fTemp80
                                        * (fTemp282
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp279, 8)) as usize]
                                            } - fTemp280))))
                } else {
                    1.0 - (fTemp275
                        + fTemp80 * fTemp276
                        + (fTemp271 - (iTemp272) as F64)
                            * (fTemp274
                                - (fTemp275
                                    + fTemp80
                                        * (fTemp276
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp273, 8)) as usize]
                                            } - fTemp274)))))
                });
                let mut fTemp284: F64 = fTemp85 + fTemp270;
                let mut fTemp285: F64 = 524287.0 * (1.0 - fTemp284);
                let mut iTemp286: i32 = (fTemp285) as i32;
                let mut iTemp287: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp286, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp288: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp287, 7)) as usize] };
                let mut fTemp289: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp287 as usize] };
                let mut fTemp290: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp287, 1)) as usize] }
                        - fTemp289;
                let mut fTemp291: F64 = 524287.0 * fTemp284;
                let mut iTemp292: i32 = (fTemp291) as i32;
                let mut iTemp293: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp292, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp294: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp293, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp295: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp293 as usize] };
                let mut fTemp296: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp293, 1), 3670015),
                    )) as usize]
                } - fTemp295;
                let mut iTemp297: i32 = (fTemp141
                    > (((if iTemp67 != 0 {
                        fTemp295
                            + fTemp80 * fTemp296
                            + (fTemp291 - (iTemp292) as F64)
                                * (fTemp294
                                    - (fTemp295
                                        + fTemp80
                                            * (fTemp296
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp293, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp294))))
                    } else {
                        1.0 - (fTemp289
                            + fTemp80 * fTemp290
                            + (fTemp285 - (iTemp286) as F64)
                                * (fTemp288
                                    - (fTemp289
                                        + fTemp80
                                            * (fTemp290
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp287, 8)) as usize]
                                                } - fTemp288)))))
                    }) - fTemp283)
                        / (1.0 - fTemp283))) as i32;
                let mut fTemp298: F64 = (if iTemp297 != 0 { fTemp267 } else { fTemp270 });
                let mut fTemp299: F64 = (if iTemp297 != 0 { fTemp270 } else { fTemp268 });
                let mut fTemp300: F64 = fTemp299 + fTemp298;
                let mut fTemp301: F64 = 0.5 * fTemp300;
                let mut fTemp302: F64 = 524287.0 * (1.0 - fTemp301);
                let mut iTemp303: i32 = (fTemp302) as i32;
                let mut iTemp304: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp303, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp305: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp304, 7)) as usize] };
                let mut fTemp306: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp304 as usize] };
                let mut fTemp307: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp304, 1)) as usize] }
                        - fTemp306;
                let mut fTemp308: F64 = 262143.5 * fTemp300;
                let mut iTemp309: i32 = (fTemp308) as i32;
                let mut iTemp310: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp309, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp311: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp310, 7)) as usize] };
                let mut fTemp312: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp310 as usize] };
                let mut fTemp313: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp310, 1)) as usize] }
                        - fTemp312;
                let mut fTemp314: F64 = (if iTemp67 != 0 {
                    fTemp312
                        + fTemp80 * fTemp313
                        + (fTemp308 - (iTemp309) as F64)
                            * (fTemp311
                                - (fTemp312
                                    + fTemp80
                                        * (fTemp313
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp310, 8)) as usize]
                                            } - fTemp311))))
                } else {
                    1.0 - (fTemp306
                        + fTemp80 * fTemp307
                        + (fTemp302 - (iTemp303) as F64)
                            * (fTemp305
                                - (fTemp306
                                    + fTemp80
                                        * (fTemp307
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp304, 8)) as usize]
                                            } - fTemp305)))))
                });
                let mut fTemp315: F64 = fTemp85 + fTemp301;
                let mut fTemp316: F64 = 524287.0 * (1.0 - fTemp315);
                let mut iTemp317: i32 = (fTemp316) as i32;
                let mut iTemp318: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp317, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp319: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp318, 7)) as usize] };
                let mut fTemp320: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp318 as usize] };
                let mut fTemp321: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp318, 1)) as usize] }
                        - fTemp320;
                let mut fTemp322: F64 = 524287.0 * fTemp315;
                let mut iTemp323: i32 = (fTemp322) as i32;
                let mut iTemp324: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp323, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp325: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp324, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp326: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp324 as usize] };
                let mut fTemp327: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp324, 1), 3670015),
                    )) as usize]
                } - fTemp326;
                let mut iTemp328: i32 = (fTemp141
                    > (((if iTemp67 != 0 {
                        fTemp326
                            + fTemp80 * fTemp327
                            + (fTemp322 - (iTemp323) as F64)
                                * (fTemp325
                                    - (fTemp326
                                        + fTemp80
                                            * (fTemp327
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp324, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp325))))
                    } else {
                        1.0 - (fTemp320
                            + fTemp80 * fTemp321
                            + (fTemp316 - (iTemp317) as F64)
                                * (fTemp319
                                    - (fTemp320
                                        + fTemp80
                                            * (fTemp321
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp318, 8)) as usize]
                                                } - fTemp319)))))
                    }) - fTemp314)
                        / (1.0 - fTemp314))) as i32;
                let mut fTemp329: F64 = (if iTemp328 != 0 { fTemp298 } else { fTemp301 });
                let mut fTemp330: F64 = (if iTemp328 != 0 { fTemp301 } else { fTemp299 });
                let mut fTemp331: F64 = fTemp330 + fTemp329;
                let mut fTemp332: F64 = 0.5 * fTemp331;
                let mut fTemp333: F64 = 524287.0 * (1.0 - fTemp332);
                let mut iTemp334: i32 = (fTemp333) as i32;
                let mut iTemp335: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp334, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp336: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp335, 7)) as usize] };
                let mut fTemp337: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp335 as usize] };
                let mut fTemp338: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp335, 1)) as usize] }
                        - fTemp337;
                let mut fTemp339: F64 = 262143.5 * fTemp331;
                let mut iTemp340: i32 = (fTemp339) as i32;
                let mut iTemp341: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp340, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp342: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp341, 7)) as usize] };
                let mut fTemp343: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp341 as usize] };
                let mut fTemp344: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp341, 1)) as usize] }
                        - fTemp343;
                let mut fTemp345: F64 = (if iTemp67 != 0 {
                    fTemp343
                        + fTemp80 * fTemp344
                        + (fTemp339 - (iTemp340) as F64)
                            * (fTemp342
                                - (fTemp343
                                    + fTemp80
                                        * (fTemp344
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp341, 8)) as usize]
                                            } - fTemp342))))
                } else {
                    1.0 - (fTemp337
                        + fTemp80 * fTemp338
                        + (fTemp333 - (iTemp334) as F64)
                            * (fTemp336
                                - (fTemp337
                                    + fTemp80
                                        * (fTemp338
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp335, 8)) as usize]
                                            } - fTemp336)))))
                });
                let mut fTemp346: F64 = fTemp85 + fTemp332;
                let mut fTemp347: F64 = 524287.0 * (1.0 - fTemp346);
                let mut iTemp348: i32 = (fTemp347) as i32;
                let mut iTemp349: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp348, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp350: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp349, 7)) as usize] };
                let mut fTemp351: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp349 as usize] };
                let mut fTemp352: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp349, 1)) as usize] }
                        - fTemp351;
                let mut fTemp353: F64 = 524287.0 * fTemp346;
                let mut iTemp354: i32 = (fTemp353) as i32;
                let mut iTemp355: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp354, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp356: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp355, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp357: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp355 as usize] };
                let mut fTemp358: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp355, 1), 3670015),
                    )) as usize]
                } - fTemp357;
                let mut iTemp359: i32 = (fTemp141
                    > (((if iTemp67 != 0 {
                        fTemp357
                            + fTemp80 * fTemp358
                            + (fTemp353 - (iTemp354) as F64)
                                * (fTemp356
                                    - (fTemp357
                                        + fTemp80
                                            * (fTemp358
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp355, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp356))))
                    } else {
                        1.0 - (fTemp351
                            + fTemp80 * fTemp352
                            + (fTemp347 - (iTemp348) as F64)
                                * (fTemp350
                                    - (fTemp351
                                        + fTemp80
                                            * (fTemp352
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp349, 8)) as usize]
                                                } - fTemp350)))))
                    }) - fTemp345)
                        / (1.0 - fTemp345))) as i32;
                let mut fTemp360: F64 = (if iTemp359 != 0 { fTemp329 } else { fTemp332 });
                let mut fTemp361: F64 = (if iTemp359 != 0 { fTemp332 } else { fTemp330 });
                let mut fTemp362: F64 = fTemp361 + fTemp360;
                let mut fTemp363: F64 = 0.5 * fTemp362;
                let mut fTemp364: F64 = 524287.0 * (1.0 - fTemp363);
                let mut iTemp365: i32 = (fTemp364) as i32;
                let mut iTemp366: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp365, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp367: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp366, 7)) as usize] };
                let mut fTemp368: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp366 as usize] };
                let mut fTemp369: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp366, 1)) as usize] }
                        - fTemp368;
                let mut fTemp370: F64 = 262143.5 * fTemp362;
                let mut iTemp371: i32 = (fTemp370) as i32;
                let mut iTemp372: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp371, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp373: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp372, 7)) as usize] };
                let mut fTemp374: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp372 as usize] };
                let mut fTemp375: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp372, 1)) as usize] }
                        - fTemp374;
                let mut fTemp376: F64 = (if iTemp67 != 0 {
                    fTemp374
                        + fTemp80 * fTemp375
                        + (fTemp370 - (iTemp371) as F64)
                            * (fTemp373
                                - (fTemp374
                                    + fTemp80
                                        * (fTemp375
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp372, 8)) as usize]
                                            } - fTemp373))))
                } else {
                    1.0 - (fTemp368
                        + fTemp80 * fTemp369
                        + (fTemp364 - (iTemp365) as F64)
                            * (fTemp367
                                - (fTemp368
                                    + fTemp80
                                        * (fTemp369
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp366, 8)) as usize]
                                            } - fTemp367)))))
                });
                let mut fTemp377: F64 = fTemp85 + fTemp363;
                let mut fTemp378: F64 = 524287.0 * (1.0 - fTemp377);
                let mut iTemp379: i32 = (fTemp378) as i32;
                let mut iTemp380: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp379, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp381: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp380, 7)) as usize] };
                let mut fTemp382: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp380 as usize] };
                let mut fTemp383: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp380, 1)) as usize] }
                        - fTemp382;
                let mut fTemp384: F64 = 524287.0 * fTemp377;
                let mut iTemp385: i32 = (fTemp384) as i32;
                let mut iTemp386: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp385, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp387: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp386, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp388: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp386 as usize] };
                let mut fTemp389: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp386, 1), 3670015),
                    )) as usize]
                } - fTemp388;
                let mut iTemp390: i32 = (fTemp141
                    > (((if iTemp67 != 0 {
                        fTemp388
                            + fTemp80 * fTemp389
                            + (fTemp384 - (iTemp385) as F64)
                                * (fTemp387
                                    - (fTemp388
                                        + fTemp80
                                            * (fTemp389
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp386, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp387))))
                    } else {
                        1.0 - (fTemp382
                            + fTemp80 * fTemp383
                            + (fTemp378 - (iTemp379) as F64)
                                * (fTemp381
                                    - (fTemp382
                                        + fTemp80
                                            * (fTemp383
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp380, 8)) as usize]
                                                } - fTemp381)))))
                    }) - fTemp376)
                        / (1.0 - fTemp376))) as i32;
                let mut fTemp391: F64 = (if iTemp390 != 0 { fTemp360 } else { fTemp363 });
                let mut fTemp392: F64 = (if iTemp390 != 0 { fTemp363 } else { fTemp361 });
                let mut fTemp393: F64 = fTemp392 + fTemp391;
                let mut fTemp394: F64 = 0.5 * fTemp393;
                let mut fTemp395: F64 = 524287.0 * (1.0 - fTemp394);
                let mut iTemp396: i32 = (fTemp395) as i32;
                let mut iTemp397: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp396, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp398: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp397, 7)) as usize] };
                let mut fTemp399: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp397 as usize] };
                let mut fTemp400: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp397, 1)) as usize] }
                        - fTemp399;
                let mut fTemp401: F64 = 262143.5 * fTemp393;
                let mut iTemp402: i32 = (fTemp401) as i32;
                let mut iTemp403: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp402, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp404: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp403, 7)) as usize] };
                let mut fTemp405: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp403 as usize] };
                let mut fTemp406: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp403, 1)) as usize] }
                        - fTemp405;
                let mut fTemp407: F64 = (if iTemp67 != 0 {
                    fTemp405
                        + fTemp80 * fTemp406
                        + (fTemp401 - (iTemp402) as F64)
                            * (fTemp404
                                - (fTemp405
                                    + fTemp80
                                        * (fTemp406
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp403, 8)) as usize]
                                            } - fTemp404))))
                } else {
                    1.0 - (fTemp399
                        + fTemp80 * fTemp400
                        + (fTemp395 - (iTemp396) as F64)
                            * (fTemp398
                                - (fTemp399
                                    + fTemp80
                                        * (fTemp400
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp397, 8)) as usize]
                                            } - fTemp398)))))
                });
                let mut fTemp408: F64 = fTemp85 + fTemp394;
                let mut fTemp409: F64 = 524287.0 * (1.0 - fTemp408);
                let mut iTemp410: i32 = (fTemp409) as i32;
                let mut iTemp411: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp410, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp412: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp411, 7)) as usize] };
                let mut fTemp413: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp411 as usize] };
                let mut fTemp414: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp411, 1)) as usize] }
                        - fTemp413;
                let mut fTemp415: F64 = 524287.0 * fTemp408;
                let mut iTemp416: i32 = (fTemp415) as i32;
                let mut iTemp417: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp416, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp418: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp417, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp419: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp417 as usize] };
                let mut fTemp420: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp417, 1), 3670015),
                    )) as usize]
                } - fTemp419;
                let mut iTemp421: i32 = (fTemp141
                    > (((if iTemp67 != 0 {
                        fTemp419
                            + fTemp80 * fTemp420
                            + (fTemp415 - (iTemp416) as F64)
                                * (fTemp418
                                    - (fTemp419
                                        + fTemp80
                                            * (fTemp420
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp417, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp418))))
                    } else {
                        1.0 - (fTemp413
                            + fTemp80 * fTemp414
                            + (fTemp409 - (iTemp410) as F64)
                                * (fTemp412
                                    - (fTemp413
                                        + fTemp80
                                            * (fTemp414
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp411, 8)) as usize]
                                                } - fTemp412)))))
                    }) - fTemp407)
                        / (1.0 - fTemp407))) as i32;
                let mut fTemp422: F64 = (if iTemp421 != 0 { fTemp391 } else { fTemp394 });
                let mut fTemp423: F64 = (if iTemp421 != 0 { fTemp394 } else { fTemp392 });
                let mut fTemp424: F64 = fTemp423 + fTemp422;
                let mut fTemp425: F64 = 0.5 * fTemp424;
                let mut fTemp426: F64 = 524287.0 * (1.0 - fTemp425);
                let mut iTemp427: i32 = (fTemp426) as i32;
                let mut iTemp428: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp427, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp429: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp428, 7)) as usize] };
                let mut fTemp430: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp428 as usize] };
                let mut fTemp431: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp428, 1)) as usize] }
                        - fTemp430;
                let mut fTemp432: F64 = 262143.5 * fTemp424;
                let mut iTemp433: i32 = (fTemp432) as i32;
                let mut iTemp434: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp433, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp435: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp434, 7)) as usize] };
                let mut fTemp436: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp434 as usize] };
                let mut fTemp437: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp434, 1)) as usize] }
                        - fTemp436;
                let mut fTemp438: F64 = (if iTemp67 != 0 {
                    fTemp436
                        + fTemp80 * fTemp437
                        + (fTemp432 - (iTemp433) as F64)
                            * (fTemp435
                                - (fTemp436
                                    + fTemp80
                                        * (fTemp437
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp434, 8)) as usize]
                                            } - fTemp435))))
                } else {
                    1.0 - (fTemp430
                        + fTemp80 * fTemp431
                        + (fTemp426 - (iTemp427) as F64)
                            * (fTemp429
                                - (fTemp430
                                    + fTemp80
                                        * (fTemp431
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp428, 8)) as usize]
                                            } - fTemp429)))))
                });
                let mut fTemp439: F64 = fTemp85 + fTemp425;
                let mut fTemp440: F64 = 524287.0 * (1.0 - fTemp439);
                let mut iTemp441: i32 = (fTemp440) as i32;
                let mut iTemp442: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp441, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp443: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp442, 7)) as usize] };
                let mut fTemp444: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp442 as usize] };
                let mut fTemp445: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp442, 1)) as usize] }
                        - fTemp444;
                let mut fTemp446: F64 = 524287.0 * fTemp439;
                let mut iTemp447: i32 = (fTemp446) as i32;
                let mut iTemp448: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp447, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp449: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp448, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp450: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp448 as usize] };
                let mut fTemp451: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp448, 1), 3670015),
                    )) as usize]
                } - fTemp450;
                let mut iTemp452: i32 = (fTemp141
                    > (((if iTemp67 != 0 {
                        fTemp450
                            + fTemp80 * fTemp451
                            + (fTemp446 - (iTemp447) as F64)
                                * (fTemp449
                                    - (fTemp450
                                        + fTemp80
                                            * (fTemp451
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp448, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp449))))
                    } else {
                        1.0 - (fTemp444
                            + fTemp80 * fTemp445
                            + (fTemp440 - (iTemp441) as F64)
                                * (fTemp443
                                    - (fTemp444
                                        + fTemp80
                                            * (fTemp445
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp442, 8)) as usize]
                                                } - fTemp443)))))
                    }) - fTemp438)
                        / (1.0 - fTemp438))) as i32;
                let mut fTemp453: F64 = (if iTemp452 != 0 { fTemp422 } else { fTemp425 });
                let mut fTemp454: F64 = (if iTemp452 != 0 { fTemp425 } else { fTemp423 });
                let mut fTemp455: F64 = fTemp454 + fTemp453;
                let mut fTemp456: F64 = 0.5 * fTemp455;
                let mut fTemp457: F64 = 524287.0 * (1.0 - fTemp456);
                let mut iTemp458: i32 = (fTemp457) as i32;
                let mut iTemp459: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp458, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp460: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp459, 7)) as usize] };
                let mut fTemp461: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp459 as usize] };
                let mut fTemp462: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp459, 1)) as usize] }
                        - fTemp461;
                let mut fTemp463: F64 = 262143.5 * fTemp455;
                let mut iTemp464: i32 = (fTemp463) as i32;
                let mut iTemp465: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp464, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp466: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp465, 7)) as usize] };
                let mut fTemp467: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp465 as usize] };
                let mut fTemp468: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp465, 1)) as usize] }
                        - fTemp467;
                let mut fTemp469: F64 = (if iTemp67 != 0 {
                    fTemp467
                        + fTemp80 * fTemp468
                        + (fTemp463 - (iTemp464) as F64)
                            * (fTemp466
                                - (fTemp467
                                    + fTemp80
                                        * (fTemp468
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp465, 8)) as usize]
                                            } - fTemp466))))
                } else {
                    1.0 - (fTemp461
                        + fTemp80 * fTemp462
                        + (fTemp457 - (iTemp458) as F64)
                            * (fTemp460
                                - (fTemp461
                                    + fTemp80
                                        * (fTemp462
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp459, 8)) as usize]
                                            } - fTemp460)))))
                });
                let mut fTemp470: F64 = fTemp85 + fTemp456;
                let mut fTemp471: F64 = 524287.0 * (1.0 - fTemp470);
                let mut iTemp472: i32 = (fTemp471) as i32;
                let mut iTemp473: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp472, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp474: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp473, 7)) as usize] };
                let mut fTemp475: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp473 as usize] };
                let mut fTemp476: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp473, 1)) as usize] }
                        - fTemp475;
                let mut fTemp477: F64 = 524287.0 * fTemp470;
                let mut iTemp478: i32 = (fTemp477) as i32;
                let mut iTemp479: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp478, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp480: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp479, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp481: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp479 as usize] };
                let mut fTemp482: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp479, 1), 3670015),
                    )) as usize]
                } - fTemp481;
                let mut iTemp483: i32 = (fTemp141
                    > (((if iTemp67 != 0 {
                        fTemp481
                            + fTemp80 * fTemp482
                            + (fTemp477 - (iTemp478) as F64)
                                * (fTemp480
                                    - (fTemp481
                                        + fTemp80
                                            * (fTemp482
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp479, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp480))))
                    } else {
                        1.0 - (fTemp475
                            + fTemp80 * fTemp476
                            + (fTemp471 - (iTemp472) as F64)
                                * (fTemp474
                                    - (fTemp475
                                        + fTemp80
                                            * (fTemp476
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp473, 8)) as usize]
                                                } - fTemp474)))))
                    }) - fTemp469)
                        / (1.0 - fTemp469))) as i32;
                let mut fTemp484: F64 = (if iTemp483 != 0 { fTemp453 } else { fTemp456 });
                let mut fTemp485: F64 = (if iTemp483 != 0 { fTemp456 } else { fTemp454 });
                let mut fTemp486: F64 = fTemp485 + fTemp484;
                let mut fTemp487: F64 = 0.5 * fTemp486;
                let mut fTemp488: F64 = 524287.0 * (1.0 - fTemp487);
                let mut iTemp489: i32 = (fTemp488) as i32;
                let mut iTemp490: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp489, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp491: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp490, 7)) as usize] };
                let mut fTemp492: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp490 as usize] };
                let mut fTemp493: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp490, 1)) as usize] }
                        - fTemp492;
                let mut fTemp494: F64 = 262143.5 * fTemp486;
                let mut iTemp495: i32 = (fTemp494) as i32;
                let mut iTemp496: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp495, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp497: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp496, 7)) as usize] };
                let mut fTemp498: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp496 as usize] };
                let mut fTemp499: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp496, 1)) as usize] }
                        - fTemp498;
                let mut fTemp500: F64 = (if iTemp67 != 0 {
                    fTemp498
                        + fTemp80 * fTemp499
                        + (fTemp494 - (iTemp495) as F64)
                            * (fTemp497
                                - (fTemp498
                                    + fTemp80
                                        * (fTemp499
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp496, 8)) as usize]
                                            } - fTemp497))))
                } else {
                    1.0 - (fTemp492
                        + fTemp80 * fTemp493
                        + (fTemp488 - (iTemp489) as F64)
                            * (fTemp491
                                - (fTemp492
                                    + fTemp80
                                        * (fTemp493
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp490, 8)) as usize]
                                            } - fTemp491)))))
                });
                let mut fTemp501: F64 = fTemp85 + fTemp487;
                let mut fTemp502: F64 = 524287.0 * (1.0 - fTemp501);
                let mut iTemp503: i32 = (fTemp502) as i32;
                let mut iTemp504: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp503, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp505: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp504, 7)) as usize] };
                let mut fTemp506: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp504 as usize] };
                let mut fTemp507: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp504, 1)) as usize] }
                        - fTemp506;
                let mut fTemp508: F64 = 524287.0 * fTemp501;
                let mut iTemp509: i32 = (fTemp508) as i32;
                let mut iTemp510: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp509, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp511: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp510, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp512: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp510 as usize] };
                let mut fTemp513: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp510, 1), 3670015),
                    )) as usize]
                } - fTemp512;
                let mut iTemp514: i32 = (fTemp141
                    > (((if iTemp67 != 0 {
                        fTemp512
                            + fTemp80 * fTemp513
                            + (fTemp508 - (iTemp509) as F64)
                                * (fTemp511
                                    - (fTemp512
                                        + fTemp80
                                            * (fTemp513
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp510, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp511))))
                    } else {
                        1.0 - (fTemp506
                            + fTemp80 * fTemp507
                            + (fTemp502 - (iTemp503) as F64)
                                * (fTemp505
                                    - (fTemp506
                                        + fTemp80
                                            * (fTemp507
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp504, 8)) as usize]
                                                } - fTemp505)))))
                    }) - fTemp500)
                        / (1.0 - fTemp500))) as i32;
                let mut fTemp515: F64 = (if iTemp514 != 0 { fTemp484 } else { fTemp487 });
                let mut fTemp516: F64 = (if iTemp514 != 0 { fTemp487 } else { fTemp485 });
                let mut fTemp517: F64 = fTemp516 + fTemp515;
                let mut fTemp518: F64 = 0.5 * fTemp517;
                let mut fTemp519: F64 = 524287.0 * (1.0 - fTemp518);
                let mut iTemp520: i32 = (fTemp519) as i32;
                let mut iTemp521: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp520, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp522: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp521, 7)) as usize] };
                let mut fTemp523: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp521 as usize] };
                let mut fTemp524: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp521, 1)) as usize] }
                        - fTemp523;
                let mut fTemp525: F64 = 262143.5 * fTemp517;
                let mut iTemp526: i32 = (fTemp525) as i32;
                let mut iTemp527: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp526, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp528: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp527, 7)) as usize] };
                let mut fTemp529: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp527 as usize] };
                let mut fTemp530: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp527, 1)) as usize] }
                        - fTemp529;
                let mut fTemp531: F64 = (if iTemp67 != 0 {
                    fTemp529
                        + fTemp80 * fTemp530
                        + (fTemp525 - (iTemp526) as F64)
                            * (fTemp528
                                - (fTemp529
                                    + fTemp80
                                        * (fTemp530
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp527, 8)) as usize]
                                            } - fTemp528))))
                } else {
                    1.0 - (fTemp523
                        + fTemp80 * fTemp524
                        + (fTemp519 - (iTemp520) as F64)
                            * (fTemp522
                                - (fTemp523
                                    + fTemp80
                                        * (fTemp524
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp521, 8)) as usize]
                                            } - fTemp522)))))
                });
                let mut fTemp532: F64 = fTemp85 + fTemp518;
                let mut fTemp533: F64 = 524287.0 * (1.0 - fTemp532);
                let mut iTemp534: i32 = (fTemp533) as i32;
                let mut iTemp535: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp534, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp536: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp535, 7)) as usize] };
                let mut fTemp537: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp535 as usize] };
                let mut fTemp538: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp535, 1)) as usize] }
                        - fTemp537;
                let mut fTemp539: F64 = 524287.0 * fTemp532;
                let mut iTemp540: i32 = (fTemp539) as i32;
                let mut iTemp541: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp540, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp542: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp541, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp543: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp541 as usize] };
                let mut fTemp544: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp541, 1), 3670015),
                    )) as usize]
                } - fTemp543;
                let mut iTemp545: i32 = (fTemp141
                    > (((if iTemp67 != 0 {
                        fTemp543
                            + fTemp80 * fTemp544
                            + (fTemp539 - (iTemp540) as F64)
                                * (fTemp542
                                    - (fTemp543
                                        + fTemp80
                                            * (fTemp544
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp541, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp542))))
                    } else {
                        1.0 - (fTemp537
                            + fTemp80 * fTemp538
                            + (fTemp533 - (iTemp534) as F64)
                                * (fTemp536
                                    - (fTemp537
                                        + fTemp80
                                            * (fTemp538
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp535, 8)) as usize]
                                                } - fTemp536)))))
                    }) - fTemp531)
                        / (1.0 - fTemp531))) as i32;
                let mut fTemp546: F64 = (if iTemp545 != 0 { fTemp515 } else { fTemp518 });
                let mut fTemp547: F64 = (if iTemp545 != 0 { fTemp518 } else { fTemp516 });
                let mut fTemp548: F64 = fTemp547 + fTemp546;
                let mut fTemp549: F64 = 0.5 * fTemp548;
                let mut fTemp550: F64 = 524287.0 * (1.0 - fTemp549);
                let mut iTemp551: i32 = (fTemp550) as i32;
                let mut iTemp552: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp551, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp553: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp552, 7)) as usize] };
                let mut fTemp554: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp552 as usize] };
                let mut fTemp555: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp552, 1)) as usize] }
                        - fTemp554;
                let mut fTemp556: F64 = 262143.5 * fTemp548;
                let mut iTemp557: i32 = (fTemp556) as i32;
                let mut iTemp558: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp557, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp559: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp558, 7)) as usize] };
                let mut fTemp560: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp558 as usize] };
                let mut fTemp561: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp558, 1)) as usize] }
                        - fTemp560;
                let mut fTemp562: F64 = (if iTemp67 != 0 {
                    fTemp560
                        + fTemp80 * fTemp561
                        + (fTemp556 - (iTemp557) as F64)
                            * (fTemp559
                                - (fTemp560
                                    + fTemp80
                                        * (fTemp561
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp558, 8)) as usize]
                                            } - fTemp559))))
                } else {
                    1.0 - (fTemp554
                        + fTemp80 * fTemp555
                        + (fTemp550 - (iTemp551) as F64)
                            * (fTemp553
                                - (fTemp554
                                    + fTemp80
                                        * (fTemp555
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp552, 8)) as usize]
                                            } - fTemp553)))))
                });
                let mut fTemp563: F64 = fTemp85 + fTemp549;
                let mut fTemp564: F64 = 524287.0 * (1.0 - fTemp563);
                let mut iTemp565: i32 = (fTemp564) as i32;
                let mut iTemp566: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp565, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp567: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp566, 7)) as usize] };
                let mut fTemp568: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp566 as usize] };
                let mut fTemp569: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp566, 1)) as usize] }
                        - fTemp568;
                let mut fTemp570: F64 = 524287.0 * fTemp563;
                let mut iTemp571: i32 = (fTemp570) as i32;
                let mut iTemp572: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp571, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp573: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp572, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp574: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp572 as usize] };
                let mut fTemp575: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp572, 1), 3670015),
                    )) as usize]
                } - fTemp574;
                let mut iTemp576: i32 = (fTemp141
                    > (((if iTemp67 != 0 {
                        fTemp574
                            + fTemp80 * fTemp575
                            + (fTemp570 - (iTemp571) as F64)
                                * (fTemp573
                                    - (fTemp574
                                        + fTemp80
                                            * (fTemp575
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp572, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp573))))
                    } else {
                        1.0 - (fTemp568
                            + fTemp80 * fTemp569
                            + (fTemp564 - (iTemp565) as F64)
                                * (fTemp567
                                    - (fTemp568
                                        + fTemp80
                                            * (fTemp569
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp566, 8)) as usize]
                                                } - fTemp567)))))
                    }) - fTemp562)
                        / (1.0 - fTemp562))) as i32;
                let mut fTemp577: F64 = (if iTemp576 != 0 { fTemp546 } else { fTemp549 });
                let mut fTemp578: F64 = (if iTemp576 != 0 { fTemp549 } else { fTemp547 });
                let mut fTemp579: F64 = fTemp578 + fTemp577;
                let mut fTemp580: F64 = 0.5 * fTemp579;
                let mut fTemp581: F64 = 524287.0 * (1.0 - fTemp580);
                let mut iTemp582: i32 = (fTemp581) as i32;
                let mut iTemp583: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp582, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp584: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp583, 7)) as usize] };
                let mut fTemp585: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp583 as usize] };
                let mut fTemp586: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp583, 1)) as usize] }
                        - fTemp585;
                let mut fTemp587: F64 = 262143.5 * fTemp579;
                let mut iTemp588: i32 = (fTemp587) as i32;
                let mut iTemp589: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp588, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp590: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp589, 7)) as usize] };
                let mut fTemp591: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp589 as usize] };
                let mut fTemp592: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp589, 1)) as usize] }
                        - fTemp591;
                let mut fTemp593: F64 = (if iTemp67 != 0 {
                    fTemp591
                        + fTemp80 * fTemp592
                        + (fTemp587 - (iTemp588) as F64)
                            * (fTemp590
                                - (fTemp591
                                    + fTemp80
                                        * (fTemp592
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp589, 8)) as usize]
                                            } - fTemp590))))
                } else {
                    1.0 - (fTemp585
                        + fTemp80 * fTemp586
                        + (fTemp581 - (iTemp582) as F64)
                            * (fTemp584
                                - (fTemp585
                                    + fTemp80
                                        * (fTemp586
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp583, 8)) as usize]
                                            } - fTemp584)))))
                });
                let mut fTemp594: F64 = fTemp85 + fTemp580;
                let mut fTemp595: F64 = 524287.0 * (1.0 - fTemp594);
                let mut iTemp596: i32 = (fTemp595) as i32;
                let mut iTemp597: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp596, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp598: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp597, 7)) as usize] };
                let mut fTemp599: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp597 as usize] };
                let mut fTemp600: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp597, 1)) as usize] }
                        - fTemp599;
                let mut fTemp601: F64 = 524287.0 * fTemp594;
                let mut iTemp602: i32 = (fTemp601) as i32;
                let mut iTemp603: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp602, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp604: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp603, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp605: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp603 as usize] };
                let mut fTemp606: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp603, 1), 3670015),
                    )) as usize]
                } - fTemp605;
                let mut iTemp607: i32 = (fTemp141
                    > (((if iTemp67 != 0 {
                        fTemp605
                            + fTemp80 * fTemp606
                            + (fTemp601 - (iTemp602) as F64)
                                * (fTemp604
                                    - (fTemp605
                                        + fTemp80
                                            * (fTemp606
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp603, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp604))))
                    } else {
                        1.0 - (fTemp599
                            + fTemp80 * fTemp600
                            + (fTemp595 - (iTemp596) as F64)
                                * (fTemp598
                                    - (fTemp599
                                        + fTemp80
                                            * (fTemp600
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp597, 8)) as usize]
                                                } - fTemp598)))))
                    }) - fTemp593)
                        / (1.0 - fTemp593))) as i32;
                let mut fTemp608: F64 = (if iTemp607 != 0 { fTemp577 } else { fTemp580 });
                let mut fTemp609: F64 = (if iTemp607 != 0 { fTemp580 } else { fTemp578 });
                let mut fTemp610: F64 = fTemp609 + fTemp608;
                let mut fTemp611: F64 = 0.5 * fTemp610;
                let mut fTemp612: F64 = 524287.0 * (1.0 - fTemp611);
                let mut iTemp613: i32 = (fTemp612) as i32;
                let mut iTemp614: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp613, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp615: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp614, 7)) as usize] };
                let mut fTemp616: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp614 as usize] };
                let mut fTemp617: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp614, 1)) as usize] }
                        - fTemp616;
                let mut fTemp618: F64 = 262143.5 * fTemp610;
                let mut iTemp619: i32 = (fTemp618) as i32;
                let mut iTemp620: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp619, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp621: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp620, 7)) as usize] };
                let mut fTemp622: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp620 as usize] };
                let mut fTemp623: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp620, 1)) as usize] }
                        - fTemp622;
                let mut fTemp624: F64 = (if iTemp67 != 0 {
                    fTemp622
                        + fTemp80 * fTemp623
                        + (fTemp618 - (iTemp619) as F64)
                            * (fTemp621
                                - (fTemp622
                                    + fTemp80
                                        * (fTemp623
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp620, 8)) as usize]
                                            } - fTemp621))))
                } else {
                    1.0 - (fTemp616
                        + fTemp80 * fTemp617
                        + (fTemp612 - (iTemp613) as F64)
                            * (fTemp615
                                - (fTemp616
                                    + fTemp80
                                        * (fTemp617
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp614, 8)) as usize]
                                            } - fTemp615)))))
                });
                let mut fTemp625: F64 = fTemp85 + fTemp611;
                let mut fTemp626: F64 = 524287.0 * (1.0 - fTemp625);
                let mut iTemp627: i32 = (fTemp626) as i32;
                let mut iTemp628: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp627, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp629: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp628, 7)) as usize] };
                let mut fTemp630: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp628 as usize] };
                let mut fTemp631: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp628, 1)) as usize] }
                        - fTemp630;
                let mut fTemp632: F64 = 524287.0 * fTemp625;
                let mut iTemp633: i32 = (fTemp632) as i32;
                let mut iTemp634: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp633, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp635: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp634, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp636: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp634 as usize] };
                let mut fTemp637: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp634, 1), 3670015),
                    )) as usize]
                } - fTemp636;
                let mut iTemp638: i32 = (fTemp141
                    > (((if iTemp67 != 0 {
                        fTemp636
                            + fTemp80 * fTemp637
                            + (fTemp632 - (iTemp633) as F64)
                                * (fTemp635
                                    - (fTemp636
                                        + fTemp80
                                            * (fTemp637
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp634, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp635))))
                    } else {
                        1.0 - (fTemp630
                            + fTemp80 * fTemp631
                            + (fTemp626 - (iTemp627) as F64)
                                * (fTemp629
                                    - (fTemp630
                                        + fTemp80
                                            * (fTemp631
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp628, 8)) as usize]
                                                } - fTemp629)))))
                    }) - fTemp624)
                        / (1.0 - fTemp624))) as i32;
                let mut fTemp639: F64 = (if iTemp638 != 0 { fTemp608 } else { fTemp611 });
                let mut fTemp640: F64 = (if iTemp638 != 0 { fTemp611 } else { fTemp609 });
                let mut fTemp641: F64 = fTemp640 + fTemp639;
                let mut fTemp642: F64 = 0.5 * fTemp641;
                let mut fTemp643: F64 = 524287.0 * (1.0 - fTemp642);
                let mut iTemp644: i32 = (fTemp643) as i32;
                let mut iTemp645: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp644, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp646: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp645, 7)) as usize] };
                let mut fTemp647: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp645 as usize] };
                let mut fTemp648: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp645, 1)) as usize] }
                        - fTemp647;
                let mut fTemp649: F64 = 262143.5 * fTemp641;
                let mut iTemp650: i32 = (fTemp649) as i32;
                let mut iTemp651: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp650, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp652: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp651, 7)) as usize] };
                let mut fTemp653: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp651 as usize] };
                let mut fTemp654: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp651, 1)) as usize] }
                        - fTemp653;
                let mut fTemp655: F64 = (if iTemp67 != 0 {
                    fTemp653
                        + fTemp80 * fTemp654
                        + (fTemp649 - (iTemp650) as F64)
                            * (fTemp652
                                - (fTemp653
                                    + fTemp80
                                        * (fTemp654
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp651, 8)) as usize]
                                            } - fTemp652))))
                } else {
                    1.0 - (fTemp647
                        + fTemp80 * fTemp648
                        + (fTemp643 - (iTemp644) as F64)
                            * (fTemp646
                                - (fTemp647
                                    + fTemp80
                                        * (fTemp648
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp645, 8)) as usize]
                                            } - fTemp646)))))
                });
                let mut fTemp656: F64 = fTemp85 + fTemp642;
                let mut fTemp657: F64 = 524287.0 * (1.0 - fTemp656);
                let mut iTemp658: i32 = (fTemp657) as i32;
                let mut iTemp659: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp658, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp660: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp659, 7)) as usize] };
                let mut fTemp661: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp659 as usize] };
                let mut fTemp662: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp659, 1)) as usize] }
                        - fTemp661;
                let mut fTemp663: F64 = 524287.0 * fTemp656;
                let mut iTemp664: i32 = (fTemp663) as i32;
                let mut iTemp665: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp664, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp666: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp665, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp667: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp665 as usize] };
                let mut fTemp668: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp665, 1), 3670015),
                    )) as usize]
                } - fTemp667;
                let mut iTemp669: i32 = (fTemp141
                    > (((if iTemp67 != 0 {
                        fTemp667
                            + fTemp80 * fTemp668
                            + (fTemp663 - (iTemp664) as F64)
                                * (fTemp666
                                    - (fTemp667
                                        + fTemp80
                                            * (fTemp668
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp665, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp666))))
                    } else {
                        1.0 - (fTemp661
                            + fTemp80 * fTemp662
                            + (fTemp657 - (iTemp658) as F64)
                                * (fTemp660
                                    - (fTemp661
                                        + fTemp80
                                            * (fTemp662
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp659, 8)) as usize]
                                                } - fTemp660)))))
                    }) - fTemp655)
                        / (1.0 - fTemp655))) as i32;
                let mut fTemp670: F64 = (if iTemp669 != 0 { fTemp639 } else { fTemp642 });
                let mut fTemp671: F64 = (if iTemp669 != 0 { fTemp642 } else { fTemp640 });
                let mut fTemp672: F64 = fTemp671 + fTemp670;
                let mut fTemp673: F64 = 0.5 * fTemp672;
                let mut fTemp674: F64 = 524287.0 * (1.0 - fTemp673);
                let mut iTemp675: i32 = (fTemp674) as i32;
                let mut iTemp676: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp675, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp677: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp676, 7)) as usize] };
                let mut fTemp678: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp676 as usize] };
                let mut fTemp679: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp676, 1)) as usize] }
                        - fTemp678;
                let mut fTemp680: F64 = 262143.5 * fTemp672;
                let mut iTemp681: i32 = (fTemp680) as i32;
                let mut iTemp682: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp681, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp683: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp682, 7)) as usize] };
                let mut fTemp684: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp682 as usize] };
                let mut fTemp685: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp682, 1)) as usize] }
                        - fTemp684;
                let mut fTemp686: F64 = (if iTemp67 != 0 {
                    fTemp684
                        + fTemp80 * fTemp685
                        + (fTemp680 - (iTemp681) as F64)
                            * (fTemp683
                                - (fTemp684
                                    + fTemp80
                                        * (fTemp685
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp682, 8),
                                                        3670015,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp683))))
                } else {
                    1.0 - (fTemp678
                        + fTemp80 * fTemp679
                        + (fTemp674 - (iTemp675) as F64)
                            * (fTemp677
                                - (fTemp678
                                    + fTemp80
                                        * (fTemp679
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp676, 8),
                                                        3670015,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp677)))))
                });
                let mut fTemp687: F64 = fTemp85 + fTemp673;
                let mut fTemp688: F64 = 524287.0 * (1.0 - fTemp687);
                let mut iTemp689: i32 = (fTemp688) as i32;
                let mut iTemp690: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp689, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp691: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp690, 7)) as usize] };
                let mut fTemp692: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp690 as usize] };
                let mut fTemp693: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp690, 1)) as usize] }
                        - fTemp692;
                let mut fTemp694: F64 = 524287.0 * fTemp687;
                let mut iTemp695: i32 = (fTemp694) as i32;
                let mut iTemp696: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp695, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp697: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp696, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp698: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp696 as usize] };
                let mut fTemp699: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp696, 1), 3670015),
                    )) as usize]
                } - fTemp698;
                let mut iTemp700: i32 = (fTemp141
                    > (((if iTemp67 != 0 {
                        fTemp698
                            + fTemp80 * fTemp699
                            + (fTemp694 - (iTemp695) as F64)
                                * (fTemp697
                                    - (fTemp698
                                        + fTemp80
                                            * (fTemp699
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp696, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp697))))
                    } else {
                        1.0 - (fTemp692
                            + fTemp80 * fTemp693
                            + (fTemp688 - (iTemp689) as F64)
                                * (fTemp691
                                    - (fTemp692
                                        + fTemp80
                                            * (fTemp693
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp690, 8)) as usize]
                                                } - fTemp691)))))
                    }) - fTemp686)
                        / (1.0 - fTemp686))) as i32;
                let mut fTemp701: F64 = F64::min(
                    1.0,
                    F64::max(
                        0.0,
                        0.5 * ((if iTemp700 != 0 { fTemp673 } else { fTemp671 })
                            + (if iTemp700 != 0 { fTemp670 } else { fTemp673 })),
                    ),
                );
                self.fRec1[0] = fTemp701;
                let mut fTemp702: F64 = 524287.0 * (1.0 - fTemp701);
                let mut iTemp703: i32 = (fTemp702) as i32;
                let mut iTemp704: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp703, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp705: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp704, 7)) as usize] };
                let mut fTemp706: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp704 as usize] };
                let mut fTemp707: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp704, 1)) as usize] }
                        - fTemp706;
                let mut fTemp708: F64 = 524287.0 * fTemp701;
                let mut iTemp709: i32 = (fTemp708) as i32;
                let mut iTemp710: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp709, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp711: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp710, 7)) as usize] };
                let mut fTemp712: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp710 as usize] };
                let mut fTemp713: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp710, 1)) as usize] }
                        - fTemp712;
                let mut fTemp714: F64 = (if iTemp67 != 0 {
                    fTemp712
                        + fTemp80 * fTemp713
                        + (fTemp708 - (iTemp709) as F64)
                            * (fTemp711
                                - (fTemp712
                                    + fTemp80
                                        * (fTemp713
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp710, 8),
                                                        3670015,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp711))))
                } else {
                    1.0 - (fTemp706
                        + fTemp80 * fTemp707
                        + (fTemp702 - (iTemp703) as F64)
                            * (fTemp705
                                - (fTemp706
                                    + fTemp80
                                        * (fTemp707
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp704, 8),
                                                        3670015,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp705)))))
                });
                let mut fTemp715: F64 = fTemp85 + fTemp701;
                let mut fTemp716: F64 = 524287.0 * (1.0 - fTemp715);
                let mut iTemp717: i32 = (fTemp716) as i32;
                let mut iTemp718: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp717, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp719: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp718, 7)) as usize] };
                let mut fTemp720: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp718 as usize] };
                let mut fTemp721: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp718, 1)) as usize] }
                        - fTemp720;
                let mut fTemp722: F64 = 524287.0 * fTemp715;
                let mut iTemp723: i32 = (fTemp722) as i32;
                let mut iTemp724: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp75,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp723, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp725: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp724, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp726: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp724 as usize] };
                let mut fTemp727: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp724, 1), 3670015),
                    )) as usize]
                } - fTemp726;
                let mut fTemp728: F64 = self.fRec2[1]
                    + (if ((0.001 * fTemp84) == 0.0) as i32 != 0 {
                        fTemp66
                    } else {
                        fTemp66
                            * ((if iTemp67 != 0 {
                                fTemp726
                                    + fTemp80 * fTemp727
                                    + (fTemp722 - (iTemp723) as F64)
                                        * (fTemp725
                                            - (fTemp726
                                                + fTemp80
                                                    * (fTemp727
                                                        - (unsafe {
                                                            ftbl0LambRs192kSIG0[(std::cmp::max(
                                                                0,
                                                                std::cmp::min(
                                                                    i32::wrapping_add(iTemp724, 8),
                                                                    3670015,
                                                                ),
                                                            ))
                                                                as usize]
                                                        } - fTemp725))))
                            } else {
                                1.0 - (fTemp720
                                    + fTemp80 * fTemp721
                                    + (fTemp716 - (iTemp717) as F64)
                                        * (fTemp719
                                            - (fTemp720
                                                + fTemp80
                                                    * (fTemp721
                                                        - (unsafe {
                                                            ftbl0LambRs192kSIG0[(i32::wrapping_add(
                                                                iTemp718, 8,
                                                            ))
                                                                as usize]
                                                        } - fTemp719)))))
                            }) - fTemp714)
                            / (1.0 - fTemp714)
                    });
                self.fRec2[0] = (if iTemp83 != 0 {
                    F64::min(fTemp728, self.fRec2[1])
                } else {
                    F64::max(fTemp728, self.fRec2[1])
                });
                self.fVec35[(self.IOTA0 & 32767) as usize] = F64::powf(1e+01, 0.05 * self.fRec2[0]);
                let mut fTemp729: F64 =
                    self.fVec35[((i32::wrapping_sub(self.IOTA0, iSlow78)) & 32767) as usize];
                self.fRec14[0] = fSlow80 + self.fConst4 * self.fRec14[1];
                *output0 = 0.5
                    * self.fVec1[((i32::wrapping_sub(self.IOTA0, iSlow79)) & 32767) as usize]
                    * fTemp2
                    + self.fRec14[0]
                        * self.fVec2[((i32::wrapping_sub(self.IOTA0, iSlow79)) & 32767) as usize]
                        * fTemp729
                        * fTemp4;
                let mut fTemp730: F64 = fTemp36 + fSlow17 * (fTemp37 - fTemp36);
                let mut iTemp731: i32 =
                    ((fTemp730 > fSlow11) as i32) + ((fTemp730 > fSlow9) as i32);
                let mut fTemp732: F64 = fTemp730 - fSlow8;
                let mut fTemp733: F64 = F64::min(
                    fTemp34,
                    -(fSlow18
                        * F64::max(
                            0.0,
                            (if (iTemp731 == 0) as i32 != 0 {
                                0.0
                            } else {
                                (if (iTemp731 == 1) as i32 != 0 {
                                    fSlow12 * LambRs192k_faustpower2_f(fSlow7 + fTemp732)
                                } else {
                                    fTemp732
                                })
                            }),
                        )),
                );
                self.fVec36[(self.IOTA0 & 16383) as usize] = fTemp733;
                let mut fTemp734: F64 = F64::min(
                    fTemp733,
                    self.fVec36[((i32::wrapping_sub(self.IOTA0, 1)) & 16383) as usize],
                );
                self.fVec37[0] = fTemp734;
                let mut fTemp735: F64 = F64::min(fTemp734, self.fVec37[2]);
                self.fVec38[0] = fTemp735;
                let mut fTemp736: F64 = F64::min(fTemp735, self.fVec38[4]);
                self.fVec39[0] = fTemp736;
                let mut fTemp737: F64 = F64::min(fTemp736, self.fVec39[8]);
                self.fVec40[(self.IOTA0 & 31) as usize] = fTemp737;
                let mut fTemp738: F64 = F64::min(
                    fTemp737,
                    self.fVec40[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize],
                );
                self.fVec41[(self.IOTA0 & 63) as usize] = fTemp738;
                let mut fTemp739: F64 = F64::min(
                    fTemp738,
                    self.fVec41[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize],
                );
                self.fVec42[(self.IOTA0 & 127) as usize] = fTemp739;
                let mut fTemp740: F64 = F64::min(
                    fTemp739,
                    self.fVec42[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize],
                );
                self.fVec43[(self.IOTA0 & 255) as usize] = fTemp740;
                let mut fTemp741: F64 = F64::min(
                    fTemp740,
                    self.fVec43[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize],
                );
                self.fVec44[(self.IOTA0 & 511) as usize] = fTemp741;
                let mut fTemp742: F64 = F64::min(
                    fTemp741,
                    self.fVec44[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize],
                );
                self.fVec45[(self.IOTA0 & 1023) as usize] = fTemp742;
                let mut fTemp743: F64 = F64::min(
                    fTemp742,
                    self.fVec45[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize],
                );
                self.fVec46[(self.IOTA0 & 2047) as usize] = fTemp743;
                let mut fTemp744: F64 = F64::min(
                    fTemp743,
                    self.fVec46[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize],
                );
                self.fVec47[(self.IOTA0 & 4095) as usize] = fTemp744;
                let mut fTemp745: F64 = F64::min(
                    fTemp744,
                    self.fVec47[((i32::wrapping_sub(self.IOTA0, 2048)) & 4095) as usize],
                );
                self.fVec48[(self.IOTA0 & 8191) as usize] = fTemp745;
                self.fVec49[(self.IOTA0 & 16383) as usize] = F64::min(
                    fTemp745,
                    self.fVec48[((i32::wrapping_sub(self.IOTA0, 4096)) & 8191) as usize],
                );
                self.fRec17[0] = F64::max(
                    F64::min(
                        self.fRec17[1],
                        self.fVec36[((i32::wrapping_sub(self.IOTA0, iSlow21)) & 16383) as usize],
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
                                                                F64::min(
                                                                    F64::min(
                                                                        (if iSlow23 != 0 {
                                                                            fTemp733
                                                                        } else {
                                                                            1.7976931348623157e+308
                                                                        }),
                                                                        (if iSlow24 != 0 {
                                                                            self.fVec37
                                                                                [iSlow23 as usize]
                                                                        } else {
                                                                            1.7976931348623157e+308
                                                                        }),
                                                                    ),
                                                                    (if iSlow25 != 0 {
                                                                        self.fVec38
                                                                            [iSlow26 as usize]
                                                                    } else {
                                                                        1.7976931348623157e+308
                                                                    }),
                                                                ),
                                                                (if iSlow27 != 0 {
                                                                    self.fVec39[iSlow28 as usize]
                                                                } else {
                                                                    1.7976931348623157e+308
                                                                }),
                                                            ),
                                                            (if iSlow29 != 0 {
                                                                self.fVec40[((i32::wrapping_sub(
                                                                    self.IOTA0, iSlow30,
                                                                )) & 31)
                                                                    as usize]
                                                            } else {
                                                                1.7976931348623157e+308
                                                            }),
                                                        ),
                                                        (if iSlow31 != 0 {
                                                            self.fVec41[((i32::wrapping_sub(
                                                                self.IOTA0, iSlow32,
                                                            )) & 63)
                                                                as usize]
                                                        } else {
                                                            1.7976931348623157e+308
                                                        }),
                                                    ),
                                                    (if iSlow33 != 0 {
                                                        self.fVec42[((i32::wrapping_sub(
                                                            self.IOTA0, iSlow34,
                                                        )) & 127)
                                                            as usize]
                                                    } else {
                                                        1.7976931348623157e+308
                                                    }),
                                                ),
                                                (if iSlow35 != 0 {
                                                    self.fVec43[((i32::wrapping_sub(
                                                        self.IOTA0, iSlow36,
                                                    )) & 255)
                                                        as usize]
                                                } else {
                                                    1.7976931348623157e+308
                                                }),
                                            ),
                                            (if iSlow37 != 0 {
                                                self.fVec44[((i32::wrapping_sub(
                                                    self.IOTA0, iSlow38,
                                                )) & 511)
                                                    as usize]
                                            } else {
                                                1.7976931348623157e+308
                                            }),
                                        ),
                                        (if iSlow39 != 0 {
                                            self.fVec45[((i32::wrapping_sub(self.IOTA0, iSlow40))
                                                & 1023)
                                                as usize]
                                        } else {
                                            1.7976931348623157e+308
                                        }),
                                    ),
                                    (if iSlow41 != 0 {
                                        self.fVec46[((i32::wrapping_sub(self.IOTA0, iSlow42))
                                            & 2047)
                                            as usize]
                                    } else {
                                        1.7976931348623157e+308
                                    }),
                                ),
                                (if iSlow43 != 0 {
                                    self.fVec47
                                        [((i32::wrapping_sub(self.IOTA0, iSlow44)) & 4095) as usize]
                                } else {
                                    1.7976931348623157e+308
                                }),
                            ),
                            (if iSlow45 != 0 {
                                self.fVec48
                                    [((i32::wrapping_sub(self.IOTA0, iSlow46)) & 8191) as usize]
                            } else {
                                1.7976931348623157e+308
                            }),
                        ),
                        (if iSlow47 != 0 {
                            self.fVec49[((i32::wrapping_sub(self.IOTA0, iSlow48)) & 16383) as usize]
                        } else {
                            1.7976931348623157e+308
                        }),
                    ),
                );
                let mut fTemp746: F64 = F64::min(self.fRec17[0], self.fRec17[1]);
                self.fVec50[0] = fTemp746;
                let mut fTemp747: F64 = F64::min(fTemp746, self.fVec50[2]);
                self.fVec51[0] = fTemp747;
                let mut fTemp748: F64 = F64::min(fTemp747, self.fVec51[4]);
                self.fVec52[0] = fTemp748;
                let mut fTemp749: F64 = F64::min(fTemp748, self.fVec52[8]);
                self.fVec53[(self.IOTA0 & 31) as usize] = fTemp749;
                let mut fTemp750: F64 = F64::min(
                    fTemp749,
                    self.fVec53[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize],
                );
                self.fVec54[(self.IOTA0 & 63) as usize] = fTemp750;
                let mut fTemp751: F64 = F64::min(
                    fTemp750,
                    self.fVec54[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize],
                );
                self.fVec55[(self.IOTA0 & 127) as usize] = fTemp751;
                let mut fTemp752: F64 = F64::min(
                    fTemp751,
                    self.fVec55[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize],
                );
                self.fVec56[(self.IOTA0 & 255) as usize] = fTemp752;
                let mut fTemp753: F64 = F64::min(
                    fTemp752,
                    self.fVec56[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize],
                );
                self.fVec57[(self.IOTA0 & 511) as usize] = fTemp753;
                let mut fTemp754: F64 = F64::min(
                    fTemp753,
                    self.fVec57[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize],
                );
                self.fVec58[(self.IOTA0 & 1023) as usize] = fTemp754;
                let mut fTemp755: F64 = F64::min(
                    fTemp754,
                    self.fVec58[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize],
                );
                self.fVec59[(self.IOTA0 & 2047) as usize] = fTemp755;
                let mut fTemp756: F64 = F64::min(
                    fTemp755,
                    self.fVec59[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize],
                );
                self.fVec60[(self.IOTA0 & 4095) as usize] = fTemp756;
                let mut fTemp757: F64 = F64::min(
                    fTemp756,
                    self.fVec60[((i32::wrapping_sub(self.IOTA0, 2048)) & 4095) as usize],
                );
                self.fVec61[(self.IOTA0 & 8191) as usize] = fTemp757;
                self.fVec62[(self.IOTA0 & 16383) as usize] = F64::min(
                    fTemp757,
                    self.fVec61[((i32::wrapping_sub(self.IOTA0, 4096)) & 8191) as usize],
                );
                let mut fTemp758: F64 = F64::min(
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
                                                                F64::min(
                                                                    (if iSlow4 != 0 {
                                                                        self.fRec17[0]
                                                                    } else {
                                                                        1.7976931348623157e+308
                                                                    }),
                                                                    (if iSlow49 != 0 {
                                                                        self.fVec50[iSlow4 as usize]
                                                                    } else {
                                                                        1.7976931348623157e+308
                                                                    }),
                                                                ),
                                                                (if iSlow50 != 0 {
                                                                    self.fVec51[iSlow51 as usize]
                                                                } else {
                                                                    1.7976931348623157e+308
                                                                }),
                                                            ),
                                                            (if iSlow52 != 0 {
                                                                self.fVec52[iSlow53 as usize]
                                                            } else {
                                                                1.7976931348623157e+308
                                                            }),
                                                        ),
                                                        (if iSlow54 != 0 {
                                                            self.fVec53[((i32::wrapping_sub(
                                                                self.IOTA0, iSlow55,
                                                            )) & 31)
                                                                as usize]
                                                        } else {
                                                            1.7976931348623157e+308
                                                        }),
                                                    ),
                                                    (if iSlow56 != 0 {
                                                        self.fVec54[((i32::wrapping_sub(
                                                            self.IOTA0, iSlow57,
                                                        )) & 63)
                                                            as usize]
                                                    } else {
                                                        1.7976931348623157e+308
                                                    }),
                                                ),
                                                (if iSlow58 != 0 {
                                                    self.fVec55[((i32::wrapping_sub(
                                                        self.IOTA0, iSlow59,
                                                    )) & 127)
                                                        as usize]
                                                } else {
                                                    1.7976931348623157e+308
                                                }),
                                            ),
                                            (if iSlow60 != 0 {
                                                self.fVec56[((i32::wrapping_sub(
                                                    self.IOTA0, iSlow61,
                                                )) & 255)
                                                    as usize]
                                            } else {
                                                1.7976931348623157e+308
                                            }),
                                        ),
                                        (if iSlow62 != 0 {
                                            self.fVec57[((i32::wrapping_sub(self.IOTA0, iSlow63))
                                                & 511)
                                                as usize]
                                        } else {
                                            1.7976931348623157e+308
                                        }),
                                    ),
                                    (if iSlow64 != 0 {
                                        self.fVec58[((i32::wrapping_sub(self.IOTA0, iSlow65))
                                            & 1023)
                                            as usize]
                                    } else {
                                        1.7976931348623157e+308
                                    }),
                                ),
                                (if iSlow66 != 0 {
                                    self.fVec59
                                        [((i32::wrapping_sub(self.IOTA0, iSlow67)) & 2047) as usize]
                                } else {
                                    1.7976931348623157e+308
                                }),
                            ),
                            (if iSlow68 != 0 {
                                self.fVec60
                                    [((i32::wrapping_sub(self.IOTA0, iSlow69)) & 4095) as usize]
                            } else {
                                1.7976931348623157e+308
                            }),
                        ),
                        (if iSlow70 != 0 {
                            self.fVec61[((i32::wrapping_sub(self.IOTA0, iSlow71)) & 8191) as usize]
                        } else {
                            1.7976931348623157e+308
                        }),
                    ),
                    (if iSlow72 != 0 {
                        self.fVec62[((i32::wrapping_sub(self.IOTA0, iSlow73)) & 16383) as usize]
                    } else {
                        1.7976931348623157e+308
                    }),
                ) - self.fRec16[1];
                self.fVec63[0] = fTemp758;
                let mut iTemp759: i32 = (fTemp758 > 0.0) as i32;
                let mut fTemp760: F64 = (if iTemp759 != 0 { fSlow75 } else { fSlow74 });
                self.fVec64[0] = fTemp760;
                let mut fTemp761: F64 = 6.0 * fTemp760;
                let mut iTemp762: i32 = (fTemp761) as i32;
                let mut iTemp763: i32 = std::cmp::max(0, std::cmp::min(iTemp762, 6));
                let mut iTemp764: i32 = std::cmp::max(
                    0,
                    std::cmp::min(i32::wrapping_add(iTemp763, 1835001), 3670015),
                );
                let mut fTemp765: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp764, 7)) as usize] };
                let mut fTemp766: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp764 as usize] };
                let mut fTemp767: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp764, 1)) as usize] }
                        - fTemp766;
                let mut fTemp768: F64 = fTemp761 - (iTemp762) as F64;
                let mut fTemp769: F64 = fTemp766
                    + fTemp768 * fTemp767
                    + 0.5
                        * (fTemp765
                            - (fTemp766
                                + fTemp768
                                    * (fTemp767
                                        - (unsafe {
                                            ftbl0LambRs192kSIG0
                                                [(i32::wrapping_add(iTemp764, 8)) as usize]
                                        } - fTemp765))));
                let mut fTemp770: F64 = (if iTemp759 != 0 {
                    fTemp769
                } else {
                    1.0 - fTemp769
                });
                let mut iTemp771: i32 = (fTemp758 < 0.0) as i32;
                let mut fTemp772: F64 = fSlow1 * (iTemp771) as F64 + fSlow13 * (iTemp759) as F64;
                self.fVec65[0] = fTemp772;
                let mut fTemp773: F64 = self.fConst10 / fTemp772;
                let mut fTemp774: F64 = fTemp773 + 0.5;
                let mut fTemp775: F64 = 524287.0 * (1.0 - fTemp774);
                let mut iTemp776: i32 = (fTemp775) as i32;
                let mut iTemp777: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp776, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp778: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp777, 7)) as usize] };
                let mut fTemp779: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp777 as usize] };
                let mut fTemp780: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp777, 1)) as usize] }
                        - fTemp779;
                let mut fTemp781: F64 = 524287.0 * fTemp774;
                let mut iTemp782: i32 = (fTemp781) as i32;
                let mut iTemp783: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp782, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp784: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp783, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp785: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp783 as usize] };
                let mut fTemp786: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp783, 1), 3670015),
                    )) as usize]
                } - fTemp785;
                let mut fTemp787: F64 = 6.0 * self.fVec64[1];
                let mut iTemp788: i32 = (fTemp787) as i32;
                let mut iTemp789: i32 = std::cmp::max(0, std::cmp::min(iTemp788, 6));
                let mut fTemp790: F64 = 524287.0 * (1.0 - self.fRec15[1]);
                let mut iTemp791: i32 = (fTemp790) as i32;
                let mut iTemp792: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp791, 524287))),
                            iTemp789,
                        ),
                        3670015,
                    ),
                );
                let mut fTemp793: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp792, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp794: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp792 as usize] };
                let mut fTemp795: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp792, 1), 3670015),
                    )) as usize]
                } - fTemp794;
                let mut fTemp796: F64 = fTemp787 - (iTemp788) as F64;
                let mut fTemp797: F64 = 524287.0 * self.fRec15[1];
                let mut iTemp798: i32 = (fTemp797) as i32;
                let mut iTemp799: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp789,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp798, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp800: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp799, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp801: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp799 as usize] };
                let mut fTemp802: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp799, 1), 3670015),
                    )) as usize]
                } - fTemp801;
                let mut fTemp803: F64 = self.fRec15[1] + fTemp773;
                let mut fTemp804: F64 = 524287.0 * (1.0 - fTemp803);
                let mut iTemp805: i32 = (fTemp804) as i32;
                let mut iTemp806: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp805, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp807: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp806, 7)) as usize] };
                let mut fTemp808: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp806 as usize] };
                let mut fTemp809: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp806, 1)) as usize] }
                        - fTemp808;
                let mut fTemp810: F64 = 524287.0 * fTemp803;
                let mut iTemp811: i32 = (fTemp810) as i32;
                let mut iTemp812: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp811, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp813: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp812, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp814: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp812 as usize] };
                let mut fTemp815: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp812, 1), 3670015),
                    )) as usize]
                } - fTemp814;
                let mut fTemp816: F64 =
                    self.fRec15[1] + self.fConst10 * (1.0 / fTemp772 + 1.0 / self.fVec65[1]);
                let mut fTemp817: F64 = 524287.0 * (1.0 - fTemp816);
                let mut iTemp818: i32 = (fTemp817) as i32;
                let mut iTemp819: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp818, 524287))),
                            iTemp763,
                        ),
                        3670015,
                    ),
                );
                let mut fTemp820: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp819, 7)) as usize] };
                let mut fTemp821: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp819 as usize] };
                let mut fTemp822: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp819, 1)) as usize] }
                        - fTemp821;
                let mut fTemp823: F64 = 524287.0 * fTemp816;
                let mut iTemp824: i32 = (fTemp823) as i32;
                let mut iTemp825: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp824, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp826: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp825, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp827: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp825 as usize] };
                let mut fTemp828: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp825, 1), 3670015),
                    )) as usize]
                } - fTemp827;
                let mut fTemp829: F64 = ((if iTemp759 != 0 {
                    fTemp827
                        + fTemp768 * fTemp828
                        + (fTemp823 - (iTemp824) as F64)
                            * (fTemp826
                                - (fTemp827
                                    + fTemp768
                                        * (fTemp828
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp825, 8),
                                                        3670015,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp826))))
                } else {
                    1.0 - (fTemp821
                        + fTemp768 * fTemp822
                        + (fTemp817 - (iTemp818) as F64)
                            * (fTemp820
                                - (fTemp821
                                    + fTemp768
                                        * (fTemp822
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp819, 8)) as usize]
                                            } - fTemp820)))))
                }) - (if iTemp759 != 0 {
                    fTemp814
                        + fTemp768 * fTemp815
                        + (fTemp810 - (iTemp811) as F64)
                            * (fTemp813
                                - (fTemp814
                                    + fTemp768
                                        * (fTemp815
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp812, 8),
                                                        3670015,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp813))))
                } else {
                    1.0 - (fTemp808
                        + fTemp768 * fTemp809
                        + (fTemp804 - (iTemp805) as F64)
                            * (fTemp807
                                - (fTemp808
                                    + fTemp768
                                        * (fTemp809
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp806, 8)) as usize]
                                            } - fTemp807)))))
                })) * self.fVec63[1]
                    / (fTemp758
                        * (1.0
                            - (if iTemp759 != 0 {
                                fTemp801
                                    + fTemp796 * fTemp802
                                    + (fTemp797 - (iTemp798) as F64)
                                        * (fTemp800
                                            - (fTemp801
                                                + fTemp796
                                                    * (fTemp802
                                                        - (unsafe {
                                                            ftbl0LambRs192kSIG0[(std::cmp::max(
                                                                0,
                                                                std::cmp::min(
                                                                    i32::wrapping_add(iTemp799, 8),
                                                                    3670015,
                                                                ),
                                                            ))
                                                                as usize]
                                                        } - fTemp800))))
                            } else {
                                1.0 - (fTemp794
                                    + fTemp796 * fTemp795
                                    + (fTemp790 - (iTemp791) as F64)
                                        * (fTemp793
                                            - (fTemp794
                                                + fTemp796
                                                    * (fTemp795
                                                        - (unsafe {
                                                            ftbl0LambRs192kSIG0[(std::cmp::max(
                                                                0,
                                                                std::cmp::min(
                                                                    i32::wrapping_add(iTemp792, 8),
                                                                    3670015,
                                                                ),
                                                            ))
                                                                as usize]
                                                        } - fTemp793)))))
                            })));
                let mut iTemp830: i32 = (fTemp829
                    > (((if iTemp759 != 0 {
                        fTemp785
                            + fTemp768 * fTemp786
                            + (fTemp781 - (iTemp782) as F64)
                                * (fTemp784
                                    - (fTemp785
                                        + fTemp768
                                            * (fTemp786
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp783, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp784))))
                    } else {
                        1.0 - (fTemp779
                            + fTemp768 * fTemp780
                            + (fTemp775 - (iTemp776) as F64)
                                * (fTemp778
                                    - (fTemp779
                                        + fTemp768
                                            * (fTemp780
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp777, 8)) as usize]
                                                } - fTemp778)))))
                    }) - fTemp770)
                        / (1.0 - fTemp770))) as i32;
                let mut fTemp831: F64 = (if iTemp830 != 0 { 1.0 } else { 0.5 });
                let mut fTemp832: F64 = (if iTemp830 != 0 { 0.5 } else { 0.0 });
                let mut fTemp833: F64 = fTemp832 + fTemp831;
                let mut fTemp834: F64 = 0.5 * fTemp833;
                let mut fTemp835: F64 = 524287.0 * (1.0 - fTemp834);
                let mut iTemp836: i32 = (fTemp835) as i32;
                let mut iTemp837: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp836, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp838: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp837, 7)) as usize] };
                let mut fTemp839: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp837 as usize] };
                let mut fTemp840: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp837, 1)) as usize] }
                        - fTemp839;
                let mut fTemp841: F64 = 262143.5 * fTemp833;
                let mut iTemp842: i32 = (fTemp841) as i32;
                let mut iTemp843: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp842, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp844: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp843, 7)) as usize] };
                let mut fTemp845: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp843 as usize] };
                let mut fTemp846: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp843, 1)) as usize] }
                        - fTemp845;
                let mut fTemp847: F64 = (if iTemp759 != 0 {
                    fTemp845
                        + fTemp768 * fTemp846
                        + (fTemp841 - (iTemp842) as F64)
                            * (fTemp844
                                - (fTemp845
                                    + fTemp768
                                        * (fTemp846
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp843, 8)) as usize]
                                            } - fTemp844))))
                } else {
                    1.0 - (fTemp839
                        + fTemp768 * fTemp840
                        + (fTemp835 - (iTemp836) as F64)
                            * (fTemp838
                                - (fTemp839
                                    + fTemp768
                                        * (fTemp840
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp837, 8)) as usize]
                                            } - fTemp838)))))
                });
                let mut fTemp848: F64 = fTemp773 + fTemp834;
                let mut fTemp849: F64 = 524287.0 * (1.0 - fTemp848);
                let mut iTemp850: i32 = (fTemp849) as i32;
                let mut iTemp851: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp850, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp852: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp851, 7)) as usize] };
                let mut fTemp853: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp851 as usize] };
                let mut fTemp854: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp851, 1)) as usize] }
                        - fTemp853;
                let mut fTemp855: F64 = 524287.0 * fTemp848;
                let mut iTemp856: i32 = (fTemp855) as i32;
                let mut iTemp857: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp856, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp858: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp857, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp859: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp857 as usize] };
                let mut fTemp860: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp857, 1), 3670015),
                    )) as usize]
                } - fTemp859;
                let mut iTemp861: i32 = (fTemp829
                    > (((if iTemp759 != 0 {
                        fTemp859
                            + fTemp768 * fTemp860
                            + (fTemp855 - (iTemp856) as F64)
                                * (fTemp858
                                    - (fTemp859
                                        + fTemp768
                                            * (fTemp860
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp857, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp858))))
                    } else {
                        1.0 - (fTemp853
                            + fTemp768 * fTemp854
                            + (fTemp849 - (iTemp850) as F64)
                                * (fTemp852
                                    - (fTemp853
                                        + fTemp768
                                            * (fTemp854
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp851, 8)) as usize]
                                                } - fTemp852)))))
                    }) - fTemp847)
                        / (1.0 - fTemp847))) as i32;
                let mut fTemp862: F64 = (if iTemp861 != 0 { fTemp831 } else { fTemp834 });
                let mut fTemp863: F64 = (if iTemp861 != 0 { fTemp834 } else { fTemp832 });
                let mut fTemp864: F64 = fTemp863 + fTemp862;
                let mut fTemp865: F64 = 0.5 * fTemp864;
                let mut fTemp866: F64 = 524287.0 * (1.0 - fTemp865);
                let mut iTemp867: i32 = (fTemp866) as i32;
                let mut iTemp868: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp867, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp869: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp868, 7)) as usize] };
                let mut fTemp870: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp868 as usize] };
                let mut fTemp871: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp868, 1)) as usize] }
                        - fTemp870;
                let mut fTemp872: F64 = 262143.5 * fTemp864;
                let mut iTemp873: i32 = (fTemp872) as i32;
                let mut iTemp874: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp873, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp875: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp874, 7)) as usize] };
                let mut fTemp876: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp874 as usize] };
                let mut fTemp877: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp874, 1)) as usize] }
                        - fTemp876;
                let mut fTemp878: F64 = (if iTemp759 != 0 {
                    fTemp876
                        + fTemp768 * fTemp877
                        + (fTemp872 - (iTemp873) as F64)
                            * (fTemp875
                                - (fTemp876
                                    + fTemp768
                                        * (fTemp877
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp874, 8)) as usize]
                                            } - fTemp875))))
                } else {
                    1.0 - (fTemp870
                        + fTemp768 * fTemp871
                        + (fTemp866 - (iTemp867) as F64)
                            * (fTemp869
                                - (fTemp870
                                    + fTemp768
                                        * (fTemp871
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp868, 8)) as usize]
                                            } - fTemp869)))))
                });
                let mut fTemp879: F64 = fTemp773 + fTemp865;
                let mut fTemp880: F64 = 524287.0 * (1.0 - fTemp879);
                let mut iTemp881: i32 = (fTemp880) as i32;
                let mut iTemp882: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp881, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp883: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp882, 7)) as usize] };
                let mut fTemp884: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp882 as usize] };
                let mut fTemp885: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp882, 1)) as usize] }
                        - fTemp884;
                let mut fTemp886: F64 = 524287.0 * fTemp879;
                let mut iTemp887: i32 = (fTemp886) as i32;
                let mut iTemp888: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp887, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp889: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp888, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp890: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp888 as usize] };
                let mut fTemp891: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp888, 1), 3670015),
                    )) as usize]
                } - fTemp890;
                let mut iTemp892: i32 = (fTemp829
                    > (((if iTemp759 != 0 {
                        fTemp890
                            + fTemp768 * fTemp891
                            + (fTemp886 - (iTemp887) as F64)
                                * (fTemp889
                                    - (fTemp890
                                        + fTemp768
                                            * (fTemp891
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp888, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp889))))
                    } else {
                        1.0 - (fTemp884
                            + fTemp768 * fTemp885
                            + (fTemp880 - (iTemp881) as F64)
                                * (fTemp883
                                    - (fTemp884
                                        + fTemp768
                                            * (fTemp885
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp882, 8)) as usize]
                                                } - fTemp883)))))
                    }) - fTemp878)
                        / (1.0 - fTemp878))) as i32;
                let mut fTemp893: F64 = (if iTemp892 != 0 { fTemp862 } else { fTemp865 });
                let mut fTemp894: F64 = (if iTemp892 != 0 { fTemp865 } else { fTemp863 });
                let mut fTemp895: F64 = fTemp894 + fTemp893;
                let mut fTemp896: F64 = 0.5 * fTemp895;
                let mut fTemp897: F64 = 524287.0 * (1.0 - fTemp896);
                let mut iTemp898: i32 = (fTemp897) as i32;
                let mut iTemp899: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp898, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp900: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp899, 7)) as usize] };
                let mut fTemp901: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp899 as usize] };
                let mut fTemp902: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp899, 1)) as usize] }
                        - fTemp901;
                let mut fTemp903: F64 = 262143.5 * fTemp895;
                let mut iTemp904: i32 = (fTemp903) as i32;
                let mut iTemp905: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp904, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp906: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp905, 7)) as usize] };
                let mut fTemp907: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp905 as usize] };
                let mut fTemp908: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp905, 1)) as usize] }
                        - fTemp907;
                let mut fTemp909: F64 = (if iTemp759 != 0 {
                    fTemp907
                        + fTemp768 * fTemp908
                        + (fTemp903 - (iTemp904) as F64)
                            * (fTemp906
                                - (fTemp907
                                    + fTemp768
                                        * (fTemp908
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp905, 8)) as usize]
                                            } - fTemp906))))
                } else {
                    1.0 - (fTemp901
                        + fTemp768 * fTemp902
                        + (fTemp897 - (iTemp898) as F64)
                            * (fTemp900
                                - (fTemp901
                                    + fTemp768
                                        * (fTemp902
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp899, 8)) as usize]
                                            } - fTemp900)))))
                });
                let mut fTemp910: F64 = fTemp773 + fTemp896;
                let mut fTemp911: F64 = 524287.0 * (1.0 - fTemp910);
                let mut iTemp912: i32 = (fTemp911) as i32;
                let mut iTemp913: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp912, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp914: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp913, 7)) as usize] };
                let mut fTemp915: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp913 as usize] };
                let mut fTemp916: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp913, 1)) as usize] }
                        - fTemp915;
                let mut fTemp917: F64 = 524287.0 * fTemp910;
                let mut iTemp918: i32 = (fTemp917) as i32;
                let mut iTemp919: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp918, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp920: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp919, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp921: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp919 as usize] };
                let mut fTemp922: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp919, 1), 3670015),
                    )) as usize]
                } - fTemp921;
                let mut iTemp923: i32 = (fTemp829
                    > (((if iTemp759 != 0 {
                        fTemp921
                            + fTemp768 * fTemp922
                            + (fTemp917 - (iTemp918) as F64)
                                * (fTemp920
                                    - (fTemp921
                                        + fTemp768
                                            * (fTemp922
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp919, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp920))))
                    } else {
                        1.0 - (fTemp915
                            + fTemp768 * fTemp916
                            + (fTemp911 - (iTemp912) as F64)
                                * (fTemp914
                                    - (fTemp915
                                        + fTemp768
                                            * (fTemp916
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp913, 8)) as usize]
                                                } - fTemp914)))))
                    }) - fTemp909)
                        / (1.0 - fTemp909))) as i32;
                let mut fTemp924: F64 = (if iTemp923 != 0 { fTemp893 } else { fTemp896 });
                let mut fTemp925: F64 = (if iTemp923 != 0 { fTemp896 } else { fTemp894 });
                let mut fTemp926: F64 = fTemp925 + fTemp924;
                let mut fTemp927: F64 = 0.5 * fTemp926;
                let mut fTemp928: F64 = 524287.0 * (1.0 - fTemp927);
                let mut iTemp929: i32 = (fTemp928) as i32;
                let mut iTemp930: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp929, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp931: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp930, 7)) as usize] };
                let mut fTemp932: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp930 as usize] };
                let mut fTemp933: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp930, 1)) as usize] }
                        - fTemp932;
                let mut fTemp934: F64 = 262143.5 * fTemp926;
                let mut iTemp935: i32 = (fTemp934) as i32;
                let mut iTemp936: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp935, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp937: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp936, 7)) as usize] };
                let mut fTemp938: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp936 as usize] };
                let mut fTemp939: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp936, 1)) as usize] }
                        - fTemp938;
                let mut fTemp940: F64 = (if iTemp759 != 0 {
                    fTemp938
                        + fTemp768 * fTemp939
                        + (fTemp934 - (iTemp935) as F64)
                            * (fTemp937
                                - (fTemp938
                                    + fTemp768
                                        * (fTemp939
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp936, 8)) as usize]
                                            } - fTemp937))))
                } else {
                    1.0 - (fTemp932
                        + fTemp768 * fTemp933
                        + (fTemp928 - (iTemp929) as F64)
                            * (fTemp931
                                - (fTemp932
                                    + fTemp768
                                        * (fTemp933
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp930, 8)) as usize]
                                            } - fTemp931)))))
                });
                let mut fTemp941: F64 = fTemp773 + fTemp927;
                let mut fTemp942: F64 = 524287.0 * (1.0 - fTemp941);
                let mut iTemp943: i32 = (fTemp942) as i32;
                let mut iTemp944: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp943, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp945: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp944, 7)) as usize] };
                let mut fTemp946: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp944 as usize] };
                let mut fTemp947: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp944, 1)) as usize] }
                        - fTemp946;
                let mut fTemp948: F64 = 524287.0 * fTemp941;
                let mut iTemp949: i32 = (fTemp948) as i32;
                let mut iTemp950: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp949, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp951: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp950, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp952: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp950 as usize] };
                let mut fTemp953: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp950, 1), 3670015),
                    )) as usize]
                } - fTemp952;
                let mut iTemp954: i32 = (fTemp829
                    > (((if iTemp759 != 0 {
                        fTemp952
                            + fTemp768 * fTemp953
                            + (fTemp948 - (iTemp949) as F64)
                                * (fTemp951
                                    - (fTemp952
                                        + fTemp768
                                            * (fTemp953
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp950, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp951))))
                    } else {
                        1.0 - (fTemp946
                            + fTemp768 * fTemp947
                            + (fTemp942 - (iTemp943) as F64)
                                * (fTemp945
                                    - (fTemp946
                                        + fTemp768
                                            * (fTemp947
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp944, 8)) as usize]
                                                } - fTemp945)))))
                    }) - fTemp940)
                        / (1.0 - fTemp940))) as i32;
                let mut fTemp955: F64 = (if iTemp954 != 0 { fTemp924 } else { fTemp927 });
                let mut fTemp956: F64 = (if iTemp954 != 0 { fTemp927 } else { fTemp925 });
                let mut fTemp957: F64 = fTemp956 + fTemp955;
                let mut fTemp958: F64 = 0.5 * fTemp957;
                let mut fTemp959: F64 = 524287.0 * (1.0 - fTemp958);
                let mut iTemp960: i32 = (fTemp959) as i32;
                let mut iTemp961: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp960, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp962: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp961, 7)) as usize] };
                let mut fTemp963: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp961 as usize] };
                let mut fTemp964: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp961, 1)) as usize] }
                        - fTemp963;
                let mut fTemp965: F64 = 262143.5 * fTemp957;
                let mut iTemp966: i32 = (fTemp965) as i32;
                let mut iTemp967: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp966, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp968: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp967, 7)) as usize] };
                let mut fTemp969: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp967 as usize] };
                let mut fTemp970: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp967, 1)) as usize] }
                        - fTemp969;
                let mut fTemp971: F64 = (if iTemp759 != 0 {
                    fTemp969
                        + fTemp768 * fTemp970
                        + (fTemp965 - (iTemp966) as F64)
                            * (fTemp968
                                - (fTemp969
                                    + fTemp768
                                        * (fTemp970
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp967, 8)) as usize]
                                            } - fTemp968))))
                } else {
                    1.0 - (fTemp963
                        + fTemp768 * fTemp964
                        + (fTemp959 - (iTemp960) as F64)
                            * (fTemp962
                                - (fTemp963
                                    + fTemp768
                                        * (fTemp964
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp961, 8)) as usize]
                                            } - fTemp962)))))
                });
                let mut fTemp972: F64 = fTemp773 + fTemp958;
                let mut fTemp973: F64 = 524287.0 * (1.0 - fTemp972);
                let mut iTemp974: i32 = (fTemp973) as i32;
                let mut iTemp975: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp974, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp976: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp975, 7)) as usize] };
                let mut fTemp977: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp975 as usize] };
                let mut fTemp978: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp975, 1)) as usize] }
                        - fTemp977;
                let mut fTemp979: F64 = 524287.0 * fTemp972;
                let mut iTemp980: i32 = (fTemp979) as i32;
                let mut iTemp981: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp980, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp982: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp981, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp983: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp981 as usize] };
                let mut fTemp984: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp981, 1), 3670015),
                    )) as usize]
                } - fTemp983;
                let mut iTemp985: i32 = (fTemp829
                    > (((if iTemp759 != 0 {
                        fTemp983
                            + fTemp768 * fTemp984
                            + (fTemp979 - (iTemp980) as F64)
                                * (fTemp982
                                    - (fTemp983
                                        + fTemp768
                                            * (fTemp984
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp981, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp982))))
                    } else {
                        1.0 - (fTemp977
                            + fTemp768 * fTemp978
                            + (fTemp973 - (iTemp974) as F64)
                                * (fTemp976
                                    - (fTemp977
                                        + fTemp768
                                            * (fTemp978
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp975, 8)) as usize]
                                                } - fTemp976)))))
                    }) - fTemp971)
                        / (1.0 - fTemp971))) as i32;
                let mut fTemp986: F64 = (if iTemp985 != 0 { fTemp955 } else { fTemp958 });
                let mut fTemp987: F64 = (if iTemp985 != 0 { fTemp958 } else { fTemp956 });
                let mut fTemp988: F64 = fTemp987 + fTemp986;
                let mut fTemp989: F64 = 0.5 * fTemp988;
                let mut fTemp990: F64 = 524287.0 * (1.0 - fTemp989);
                let mut iTemp991: i32 = (fTemp990) as i32;
                let mut iTemp992: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp991, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp993: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp992, 7)) as usize] };
                let mut fTemp994: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp992 as usize] };
                let mut fTemp995: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp992, 1)) as usize] }
                        - fTemp994;
                let mut fTemp996: F64 = 262143.5 * fTemp988;
                let mut iTemp997: i32 = (fTemp996) as i32;
                let mut iTemp998: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp997, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp999: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp998, 7)) as usize] };
                let mut fTemp1000: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp998 as usize] };
                let mut fTemp1001: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp998, 1)) as usize] }
                        - fTemp1000;
                let mut fTemp1002: F64 = (if iTemp759 != 0 {
                    fTemp1000
                        + fTemp768 * fTemp1001
                        + (fTemp996 - (iTemp997) as F64)
                            * (fTemp999
                                - (fTemp1000
                                    + fTemp768
                                        * (fTemp1001
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp998, 8)) as usize]
                                            } - fTemp999))))
                } else {
                    1.0 - (fTemp994
                        + fTemp768 * fTemp995
                        + (fTemp990 - (iTemp991) as F64)
                            * (fTemp993
                                - (fTemp994
                                    + fTemp768
                                        * (fTemp995
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp992, 8)) as usize]
                                            } - fTemp993)))))
                });
                let mut fTemp1003: F64 = fTemp773 + fTemp989;
                let mut fTemp1004: F64 = 524287.0 * (1.0 - fTemp1003);
                let mut iTemp1005: i32 = (fTemp1004) as i32;
                let mut iTemp1006: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1005, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1007: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1006, 7)) as usize] };
                let mut fTemp1008: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1006 as usize] };
                let mut fTemp1009: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1006, 1)) as usize] }
                        - fTemp1008;
                let mut fTemp1010: F64 = 524287.0 * fTemp1003;
                let mut iTemp1011: i32 = (fTemp1010) as i32;
                let mut iTemp1012: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1011, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1013: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1012, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp1014: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1012 as usize] };
                let mut fTemp1015: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1012, 1), 3670015),
                    )) as usize]
                } - fTemp1014;
                let mut iTemp1016: i32 = (fTemp829
                    > (((if iTemp759 != 0 {
                        fTemp1014
                            + fTemp768 * fTemp1015
                            + (fTemp1010 - (iTemp1011) as F64)
                                * (fTemp1013
                                    - (fTemp1014
                                        + fTemp768
                                            * (fTemp1015
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1012, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1013))))
                    } else {
                        1.0 - (fTemp1008
                            + fTemp768 * fTemp1009
                            + (fTemp1004 - (iTemp1005) as F64)
                                * (fTemp1007
                                    - (fTemp1008
                                        + fTemp768
                                            * (fTemp1009
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp1006, 8)) as usize]
                                                } - fTemp1007)))))
                    }) - fTemp1002)
                        / (1.0 - fTemp1002))) as i32;
                let mut fTemp1017: F64 = (if iTemp1016 != 0 { fTemp986 } else { fTemp989 });
                let mut fTemp1018: F64 = (if iTemp1016 != 0 { fTemp989 } else { fTemp987 });
                let mut fTemp1019: F64 = fTemp1018 + fTemp1017;
                let mut fTemp1020: F64 = 0.5 * fTemp1019;
                let mut fTemp1021: F64 = 524287.0 * (1.0 - fTemp1020);
                let mut iTemp1022: i32 = (fTemp1021) as i32;
                let mut iTemp1023: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1022, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1024: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1023, 7)) as usize] };
                let mut fTemp1025: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1023 as usize] };
                let mut fTemp1026: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1023, 1)) as usize] }
                        - fTemp1025;
                let mut fTemp1027: F64 = 262143.5 * fTemp1019;
                let mut iTemp1028: i32 = (fTemp1027) as i32;
                let mut iTemp1029: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1028, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1030: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1029, 7)) as usize] };
                let mut fTemp1031: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1029 as usize] };
                let mut fTemp1032: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1029, 1)) as usize] }
                        - fTemp1031;
                let mut fTemp1033: F64 = (if iTemp759 != 0 {
                    fTemp1031
                        + fTemp768 * fTemp1032
                        + (fTemp1027 - (iTemp1028) as F64)
                            * (fTemp1030
                                - (fTemp1031
                                    + fTemp768
                                        * (fTemp1032
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp1029, 8)) as usize]
                                            } - fTemp1030))))
                } else {
                    1.0 - (fTemp1025
                        + fTemp768 * fTemp1026
                        + (fTemp1021 - (iTemp1022) as F64)
                            * (fTemp1024
                                - (fTemp1025
                                    + fTemp768
                                        * (fTemp1026
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp1023, 8)) as usize]
                                            } - fTemp1024)))))
                });
                let mut fTemp1034: F64 = fTemp773 + fTemp1020;
                let mut fTemp1035: F64 = 524287.0 * (1.0 - fTemp1034);
                let mut iTemp1036: i32 = (fTemp1035) as i32;
                let mut iTemp1037: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1036, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1038: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1037, 7)) as usize] };
                let mut fTemp1039: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1037 as usize] };
                let mut fTemp1040: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1037, 1)) as usize] }
                        - fTemp1039;
                let mut fTemp1041: F64 = 524287.0 * fTemp1034;
                let mut iTemp1042: i32 = (fTemp1041) as i32;
                let mut iTemp1043: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1042, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1044: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1043, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp1045: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1043 as usize] };
                let mut fTemp1046: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1043, 1), 3670015),
                    )) as usize]
                } - fTemp1045;
                let mut iTemp1047: i32 = (fTemp829
                    > (((if iTemp759 != 0 {
                        fTemp1045
                            + fTemp768 * fTemp1046
                            + (fTemp1041 - (iTemp1042) as F64)
                                * (fTemp1044
                                    - (fTemp1045
                                        + fTemp768
                                            * (fTemp1046
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1043, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1044))))
                    } else {
                        1.0 - (fTemp1039
                            + fTemp768 * fTemp1040
                            + (fTemp1035 - (iTemp1036) as F64)
                                * (fTemp1038
                                    - (fTemp1039
                                        + fTemp768
                                            * (fTemp1040
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp1037, 8)) as usize]
                                                } - fTemp1038)))))
                    }) - fTemp1033)
                        / (1.0 - fTemp1033))) as i32;
                let mut fTemp1048: F64 = (if iTemp1047 != 0 { fTemp1017 } else { fTemp1020 });
                let mut fTemp1049: F64 = (if iTemp1047 != 0 { fTemp1020 } else { fTemp1018 });
                let mut fTemp1050: F64 = fTemp1049 + fTemp1048;
                let mut fTemp1051: F64 = 0.5 * fTemp1050;
                let mut fTemp1052: F64 = 524287.0 * (1.0 - fTemp1051);
                let mut iTemp1053: i32 = (fTemp1052) as i32;
                let mut iTemp1054: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1053, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1055: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1054, 7)) as usize] };
                let mut fTemp1056: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1054 as usize] };
                let mut fTemp1057: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1054, 1)) as usize] }
                        - fTemp1056;
                let mut fTemp1058: F64 = 262143.5 * fTemp1050;
                let mut iTemp1059: i32 = (fTemp1058) as i32;
                let mut iTemp1060: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1059, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1061: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1060, 7)) as usize] };
                let mut fTemp1062: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1060 as usize] };
                let mut fTemp1063: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1060, 1)) as usize] }
                        - fTemp1062;
                let mut fTemp1064: F64 = (if iTemp759 != 0 {
                    fTemp1062
                        + fTemp768 * fTemp1063
                        + (fTemp1058 - (iTemp1059) as F64)
                            * (fTemp1061
                                - (fTemp1062
                                    + fTemp768
                                        * (fTemp1063
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp1060, 8)) as usize]
                                            } - fTemp1061))))
                } else {
                    1.0 - (fTemp1056
                        + fTemp768 * fTemp1057
                        + (fTemp1052 - (iTemp1053) as F64)
                            * (fTemp1055
                                - (fTemp1056
                                    + fTemp768
                                        * (fTemp1057
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp1054, 8)) as usize]
                                            } - fTemp1055)))))
                });
                let mut fTemp1065: F64 = fTemp773 + fTemp1051;
                let mut fTemp1066: F64 = 524287.0 * (1.0 - fTemp1065);
                let mut iTemp1067: i32 = (fTemp1066) as i32;
                let mut iTemp1068: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1067, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1069: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1068, 7)) as usize] };
                let mut fTemp1070: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1068 as usize] };
                let mut fTemp1071: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1068, 1)) as usize] }
                        - fTemp1070;
                let mut fTemp1072: F64 = 524287.0 * fTemp1065;
                let mut iTemp1073: i32 = (fTemp1072) as i32;
                let mut iTemp1074: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1073, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1075: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1074, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp1076: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1074 as usize] };
                let mut fTemp1077: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1074, 1), 3670015),
                    )) as usize]
                } - fTemp1076;
                let mut iTemp1078: i32 = (fTemp829
                    > (((if iTemp759 != 0 {
                        fTemp1076
                            + fTemp768 * fTemp1077
                            + (fTemp1072 - (iTemp1073) as F64)
                                * (fTemp1075
                                    - (fTemp1076
                                        + fTemp768
                                            * (fTemp1077
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1074, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1075))))
                    } else {
                        1.0 - (fTemp1070
                            + fTemp768 * fTemp1071
                            + (fTemp1066 - (iTemp1067) as F64)
                                * (fTemp1069
                                    - (fTemp1070
                                        + fTemp768
                                            * (fTemp1071
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp1068, 8)) as usize]
                                                } - fTemp1069)))))
                    }) - fTemp1064)
                        / (1.0 - fTemp1064))) as i32;
                let mut fTemp1079: F64 = (if iTemp1078 != 0 { fTemp1048 } else { fTemp1051 });
                let mut fTemp1080: F64 = (if iTemp1078 != 0 { fTemp1051 } else { fTemp1049 });
                let mut fTemp1081: F64 = fTemp1080 + fTemp1079;
                let mut fTemp1082: F64 = 0.5 * fTemp1081;
                let mut fTemp1083: F64 = 524287.0 * (1.0 - fTemp1082);
                let mut iTemp1084: i32 = (fTemp1083) as i32;
                let mut iTemp1085: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1084, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1086: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1085, 7)) as usize] };
                let mut fTemp1087: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1085 as usize] };
                let mut fTemp1088: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1085, 1)) as usize] }
                        - fTemp1087;
                let mut fTemp1089: F64 = 262143.5 * fTemp1081;
                let mut iTemp1090: i32 = (fTemp1089) as i32;
                let mut iTemp1091: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1090, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1092: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1091, 7)) as usize] };
                let mut fTemp1093: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1091 as usize] };
                let mut fTemp1094: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1091, 1)) as usize] }
                        - fTemp1093;
                let mut fTemp1095: F64 = (if iTemp759 != 0 {
                    fTemp1093
                        + fTemp768 * fTemp1094
                        + (fTemp1089 - (iTemp1090) as F64)
                            * (fTemp1092
                                - (fTemp1093
                                    + fTemp768
                                        * (fTemp1094
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp1091, 8)) as usize]
                                            } - fTemp1092))))
                } else {
                    1.0 - (fTemp1087
                        + fTemp768 * fTemp1088
                        + (fTemp1083 - (iTemp1084) as F64)
                            * (fTemp1086
                                - (fTemp1087
                                    + fTemp768
                                        * (fTemp1088
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp1085, 8)) as usize]
                                            } - fTemp1086)))))
                });
                let mut fTemp1096: F64 = fTemp773 + fTemp1082;
                let mut fTemp1097: F64 = 524287.0 * (1.0 - fTemp1096);
                let mut iTemp1098: i32 = (fTemp1097) as i32;
                let mut iTemp1099: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1098, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1100: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1099, 7)) as usize] };
                let mut fTemp1101: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1099 as usize] };
                let mut fTemp1102: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1099, 1)) as usize] }
                        - fTemp1101;
                let mut fTemp1103: F64 = 524287.0 * fTemp1096;
                let mut iTemp1104: i32 = (fTemp1103) as i32;
                let mut iTemp1105: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1104, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1106: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1105, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp1107: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1105 as usize] };
                let mut fTemp1108: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1105, 1), 3670015),
                    )) as usize]
                } - fTemp1107;
                let mut iTemp1109: i32 = (fTemp829
                    > (((if iTemp759 != 0 {
                        fTemp1107
                            + fTemp768 * fTemp1108
                            + (fTemp1103 - (iTemp1104) as F64)
                                * (fTemp1106
                                    - (fTemp1107
                                        + fTemp768
                                            * (fTemp1108
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1105, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1106))))
                    } else {
                        1.0 - (fTemp1101
                            + fTemp768 * fTemp1102
                            + (fTemp1097 - (iTemp1098) as F64)
                                * (fTemp1100
                                    - (fTemp1101
                                        + fTemp768
                                            * (fTemp1102
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp1099, 8)) as usize]
                                                } - fTemp1100)))))
                    }) - fTemp1095)
                        / (1.0 - fTemp1095))) as i32;
                let mut fTemp1110: F64 = (if iTemp1109 != 0 { fTemp1079 } else { fTemp1082 });
                let mut fTemp1111: F64 = (if iTemp1109 != 0 { fTemp1082 } else { fTemp1080 });
                let mut fTemp1112: F64 = fTemp1111 + fTemp1110;
                let mut fTemp1113: F64 = 0.5 * fTemp1112;
                let mut fTemp1114: F64 = 524287.0 * (1.0 - fTemp1113);
                let mut iTemp1115: i32 = (fTemp1114) as i32;
                let mut iTemp1116: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1115, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1117: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1116, 7)) as usize] };
                let mut fTemp1118: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1116 as usize] };
                let mut fTemp1119: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1116, 1)) as usize] }
                        - fTemp1118;
                let mut fTemp1120: F64 = 262143.5 * fTemp1112;
                let mut iTemp1121: i32 = (fTemp1120) as i32;
                let mut iTemp1122: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1121, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1123: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1122, 7)) as usize] };
                let mut fTemp1124: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1122 as usize] };
                let mut fTemp1125: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1122, 1)) as usize] }
                        - fTemp1124;
                let mut fTemp1126: F64 = (if iTemp759 != 0 {
                    fTemp1124
                        + fTemp768 * fTemp1125
                        + (fTemp1120 - (iTemp1121) as F64)
                            * (fTemp1123
                                - (fTemp1124
                                    + fTemp768
                                        * (fTemp1125
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp1122, 8)) as usize]
                                            } - fTemp1123))))
                } else {
                    1.0 - (fTemp1118
                        + fTemp768 * fTemp1119
                        + (fTemp1114 - (iTemp1115) as F64)
                            * (fTemp1117
                                - (fTemp1118
                                    + fTemp768
                                        * (fTemp1119
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp1116, 8)) as usize]
                                            } - fTemp1117)))))
                });
                let mut fTemp1127: F64 = fTemp773 + fTemp1113;
                let mut fTemp1128: F64 = 524287.0 * (1.0 - fTemp1127);
                let mut iTemp1129: i32 = (fTemp1128) as i32;
                let mut iTemp1130: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1129, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1131: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1130, 7)) as usize] };
                let mut fTemp1132: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1130 as usize] };
                let mut fTemp1133: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1130, 1)) as usize] }
                        - fTemp1132;
                let mut fTemp1134: F64 = 524287.0 * fTemp1127;
                let mut iTemp1135: i32 = (fTemp1134) as i32;
                let mut iTemp1136: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1135, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1137: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1136, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp1138: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1136 as usize] };
                let mut fTemp1139: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1136, 1), 3670015),
                    )) as usize]
                } - fTemp1138;
                let mut iTemp1140: i32 = (fTemp829
                    > (((if iTemp759 != 0 {
                        fTemp1138
                            + fTemp768 * fTemp1139
                            + (fTemp1134 - (iTemp1135) as F64)
                                * (fTemp1137
                                    - (fTemp1138
                                        + fTemp768
                                            * (fTemp1139
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1136, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1137))))
                    } else {
                        1.0 - (fTemp1132
                            + fTemp768 * fTemp1133
                            + (fTemp1128 - (iTemp1129) as F64)
                                * (fTemp1131
                                    - (fTemp1132
                                        + fTemp768
                                            * (fTemp1133
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp1130, 8)) as usize]
                                                } - fTemp1131)))))
                    }) - fTemp1126)
                        / (1.0 - fTemp1126))) as i32;
                let mut fTemp1141: F64 = (if iTemp1140 != 0 { fTemp1110 } else { fTemp1113 });
                let mut fTemp1142: F64 = (if iTemp1140 != 0 { fTemp1113 } else { fTemp1111 });
                let mut fTemp1143: F64 = fTemp1142 + fTemp1141;
                let mut fTemp1144: F64 = 0.5 * fTemp1143;
                let mut fTemp1145: F64 = 524287.0 * (1.0 - fTemp1144);
                let mut iTemp1146: i32 = (fTemp1145) as i32;
                let mut iTemp1147: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1146, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1148: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1147, 7)) as usize] };
                let mut fTemp1149: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1147 as usize] };
                let mut fTemp1150: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1147, 1)) as usize] }
                        - fTemp1149;
                let mut fTemp1151: F64 = 262143.5 * fTemp1143;
                let mut iTemp1152: i32 = (fTemp1151) as i32;
                let mut iTemp1153: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1152, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1154: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1153, 7)) as usize] };
                let mut fTemp1155: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1153 as usize] };
                let mut fTemp1156: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1153, 1)) as usize] }
                        - fTemp1155;
                let mut fTemp1157: F64 = (if iTemp759 != 0 {
                    fTemp1155
                        + fTemp768 * fTemp1156
                        + (fTemp1151 - (iTemp1152) as F64)
                            * (fTemp1154
                                - (fTemp1155
                                    + fTemp768
                                        * (fTemp1156
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp1153, 8)) as usize]
                                            } - fTemp1154))))
                } else {
                    1.0 - (fTemp1149
                        + fTemp768 * fTemp1150
                        + (fTemp1145 - (iTemp1146) as F64)
                            * (fTemp1148
                                - (fTemp1149
                                    + fTemp768
                                        * (fTemp1150
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp1147, 8)) as usize]
                                            } - fTemp1148)))))
                });
                let mut fTemp1158: F64 = fTemp773 + fTemp1144;
                let mut fTemp1159: F64 = 524287.0 * (1.0 - fTemp1158);
                let mut iTemp1160: i32 = (fTemp1159) as i32;
                let mut iTemp1161: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1160, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1162: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1161, 7)) as usize] };
                let mut fTemp1163: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1161 as usize] };
                let mut fTemp1164: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1161, 1)) as usize] }
                        - fTemp1163;
                let mut fTemp1165: F64 = 524287.0 * fTemp1158;
                let mut iTemp1166: i32 = (fTemp1165) as i32;
                let mut iTemp1167: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1166, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1168: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1167, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp1169: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1167 as usize] };
                let mut fTemp1170: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1167, 1), 3670015),
                    )) as usize]
                } - fTemp1169;
                let mut iTemp1171: i32 = (fTemp829
                    > (((if iTemp759 != 0 {
                        fTemp1169
                            + fTemp768 * fTemp1170
                            + (fTemp1165 - (iTemp1166) as F64)
                                * (fTemp1168
                                    - (fTemp1169
                                        + fTemp768
                                            * (fTemp1170
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1167, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1168))))
                    } else {
                        1.0 - (fTemp1163
                            + fTemp768 * fTemp1164
                            + (fTemp1159 - (iTemp1160) as F64)
                                * (fTemp1162
                                    - (fTemp1163
                                        + fTemp768
                                            * (fTemp1164
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp1161, 8)) as usize]
                                                } - fTemp1162)))))
                    }) - fTemp1157)
                        / (1.0 - fTemp1157))) as i32;
                let mut fTemp1172: F64 = (if iTemp1171 != 0 { fTemp1141 } else { fTemp1144 });
                let mut fTemp1173: F64 = (if iTemp1171 != 0 { fTemp1144 } else { fTemp1142 });
                let mut fTemp1174: F64 = fTemp1173 + fTemp1172;
                let mut fTemp1175: F64 = 0.5 * fTemp1174;
                let mut fTemp1176: F64 = 524287.0 * (1.0 - fTemp1175);
                let mut iTemp1177: i32 = (fTemp1176) as i32;
                let mut iTemp1178: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1177, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1179: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1178, 7)) as usize] };
                let mut fTemp1180: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1178 as usize] };
                let mut fTemp1181: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1178, 1)) as usize] }
                        - fTemp1180;
                let mut fTemp1182: F64 = 262143.5 * fTemp1174;
                let mut iTemp1183: i32 = (fTemp1182) as i32;
                let mut iTemp1184: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1183, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1185: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1184, 7)) as usize] };
                let mut fTemp1186: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1184 as usize] };
                let mut fTemp1187: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1184, 1)) as usize] }
                        - fTemp1186;
                let mut fTemp1188: F64 = (if iTemp759 != 0 {
                    fTemp1186
                        + fTemp768 * fTemp1187
                        + (fTemp1182 - (iTemp1183) as F64)
                            * (fTemp1185
                                - (fTemp1186
                                    + fTemp768
                                        * (fTemp1187
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp1184, 8)) as usize]
                                            } - fTemp1185))))
                } else {
                    1.0 - (fTemp1180
                        + fTemp768 * fTemp1181
                        + (fTemp1176 - (iTemp1177) as F64)
                            * (fTemp1179
                                - (fTemp1180
                                    + fTemp768
                                        * (fTemp1181
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp1178, 8)) as usize]
                                            } - fTemp1179)))))
                });
                let mut fTemp1189: F64 = fTemp773 + fTemp1175;
                let mut fTemp1190: F64 = 524287.0 * (1.0 - fTemp1189);
                let mut iTemp1191: i32 = (fTemp1190) as i32;
                let mut iTemp1192: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1191, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1193: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1192, 7)) as usize] };
                let mut fTemp1194: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1192 as usize] };
                let mut fTemp1195: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1192, 1)) as usize] }
                        - fTemp1194;
                let mut fTemp1196: F64 = 524287.0 * fTemp1189;
                let mut iTemp1197: i32 = (fTemp1196) as i32;
                let mut iTemp1198: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1197, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1199: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1198, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp1200: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1198 as usize] };
                let mut fTemp1201: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1198, 1), 3670015),
                    )) as usize]
                } - fTemp1200;
                let mut iTemp1202: i32 = (fTemp829
                    > (((if iTemp759 != 0 {
                        fTemp1200
                            + fTemp768 * fTemp1201
                            + (fTemp1196 - (iTemp1197) as F64)
                                * (fTemp1199
                                    - (fTemp1200
                                        + fTemp768
                                            * (fTemp1201
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1198, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1199))))
                    } else {
                        1.0 - (fTemp1194
                            + fTemp768 * fTemp1195
                            + (fTemp1190 - (iTemp1191) as F64)
                                * (fTemp1193
                                    - (fTemp1194
                                        + fTemp768
                                            * (fTemp1195
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp1192, 8)) as usize]
                                                } - fTemp1193)))))
                    }) - fTemp1188)
                        / (1.0 - fTemp1188))) as i32;
                let mut fTemp1203: F64 = (if iTemp1202 != 0 { fTemp1172 } else { fTemp1175 });
                let mut fTemp1204: F64 = (if iTemp1202 != 0 { fTemp1175 } else { fTemp1173 });
                let mut fTemp1205: F64 = fTemp1204 + fTemp1203;
                let mut fTemp1206: F64 = 0.5 * fTemp1205;
                let mut fTemp1207: F64 = 524287.0 * (1.0 - fTemp1206);
                let mut iTemp1208: i32 = (fTemp1207) as i32;
                let mut iTemp1209: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1208, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1210: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1209, 7)) as usize] };
                let mut fTemp1211: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1209 as usize] };
                let mut fTemp1212: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1209, 1)) as usize] }
                        - fTemp1211;
                let mut fTemp1213: F64 = 262143.5 * fTemp1205;
                let mut iTemp1214: i32 = (fTemp1213) as i32;
                let mut iTemp1215: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1214, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1216: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1215, 7)) as usize] };
                let mut fTemp1217: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1215 as usize] };
                let mut fTemp1218: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1215, 1)) as usize] }
                        - fTemp1217;
                let mut fTemp1219: F64 = (if iTemp759 != 0 {
                    fTemp1217
                        + fTemp768 * fTemp1218
                        + (fTemp1213 - (iTemp1214) as F64)
                            * (fTemp1216
                                - (fTemp1217
                                    + fTemp768
                                        * (fTemp1218
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp1215, 8)) as usize]
                                            } - fTemp1216))))
                } else {
                    1.0 - (fTemp1211
                        + fTemp768 * fTemp1212
                        + (fTemp1207 - (iTemp1208) as F64)
                            * (fTemp1210
                                - (fTemp1211
                                    + fTemp768
                                        * (fTemp1212
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp1209, 8)) as usize]
                                            } - fTemp1210)))))
                });
                let mut fTemp1220: F64 = fTemp773 + fTemp1206;
                let mut fTemp1221: F64 = 524287.0 * (1.0 - fTemp1220);
                let mut iTemp1222: i32 = (fTemp1221) as i32;
                let mut iTemp1223: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1222, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1224: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1223, 7)) as usize] };
                let mut fTemp1225: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1223 as usize] };
                let mut fTemp1226: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1223, 1)) as usize] }
                        - fTemp1225;
                let mut fTemp1227: F64 = 524287.0 * fTemp1220;
                let mut iTemp1228: i32 = (fTemp1227) as i32;
                let mut iTemp1229: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1228, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1230: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1229, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp1231: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1229 as usize] };
                let mut fTemp1232: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1229, 1), 3670015),
                    )) as usize]
                } - fTemp1231;
                let mut iTemp1233: i32 = (fTemp829
                    > (((if iTemp759 != 0 {
                        fTemp1231
                            + fTemp768 * fTemp1232
                            + (fTemp1227 - (iTemp1228) as F64)
                                * (fTemp1230
                                    - (fTemp1231
                                        + fTemp768
                                            * (fTemp1232
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1229, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1230))))
                    } else {
                        1.0 - (fTemp1225
                            + fTemp768 * fTemp1226
                            + (fTemp1221 - (iTemp1222) as F64)
                                * (fTemp1224
                                    - (fTemp1225
                                        + fTemp768
                                            * (fTemp1226
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp1223, 8)) as usize]
                                                } - fTemp1224)))))
                    }) - fTemp1219)
                        / (1.0 - fTemp1219))) as i32;
                let mut fTemp1234: F64 = (if iTemp1233 != 0 { fTemp1203 } else { fTemp1206 });
                let mut fTemp1235: F64 = (if iTemp1233 != 0 { fTemp1206 } else { fTemp1204 });
                let mut fTemp1236: F64 = fTemp1235 + fTemp1234;
                let mut fTemp1237: F64 = 0.5 * fTemp1236;
                let mut fTemp1238: F64 = 524287.0 * (1.0 - fTemp1237);
                let mut iTemp1239: i32 = (fTemp1238) as i32;
                let mut iTemp1240: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1239, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1241: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1240, 7)) as usize] };
                let mut fTemp1242: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1240 as usize] };
                let mut fTemp1243: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1240, 1)) as usize] }
                        - fTemp1242;
                let mut fTemp1244: F64 = 262143.5 * fTemp1236;
                let mut iTemp1245: i32 = (fTemp1244) as i32;
                let mut iTemp1246: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1245, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1247: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1246, 7)) as usize] };
                let mut fTemp1248: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1246 as usize] };
                let mut fTemp1249: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1246, 1)) as usize] }
                        - fTemp1248;
                let mut fTemp1250: F64 = (if iTemp759 != 0 {
                    fTemp1248
                        + fTemp768 * fTemp1249
                        + (fTemp1244 - (iTemp1245) as F64)
                            * (fTemp1247
                                - (fTemp1248
                                    + fTemp768
                                        * (fTemp1249
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp1246, 8)) as usize]
                                            } - fTemp1247))))
                } else {
                    1.0 - (fTemp1242
                        + fTemp768 * fTemp1243
                        + (fTemp1238 - (iTemp1239) as F64)
                            * (fTemp1241
                                - (fTemp1242
                                    + fTemp768
                                        * (fTemp1243
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp1240, 8)) as usize]
                                            } - fTemp1241)))))
                });
                let mut fTemp1251: F64 = fTemp773 + fTemp1237;
                let mut fTemp1252: F64 = 524287.0 * (1.0 - fTemp1251);
                let mut iTemp1253: i32 = (fTemp1252) as i32;
                let mut iTemp1254: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1253, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1255: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1254, 7)) as usize] };
                let mut fTemp1256: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1254 as usize] };
                let mut fTemp1257: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1254, 1)) as usize] }
                        - fTemp1256;
                let mut fTemp1258: F64 = 524287.0 * fTemp1251;
                let mut iTemp1259: i32 = (fTemp1258) as i32;
                let mut iTemp1260: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1259, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1261: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1260, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp1262: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1260 as usize] };
                let mut fTemp1263: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1260, 1), 3670015),
                    )) as usize]
                } - fTemp1262;
                let mut iTemp1264: i32 = (fTemp829
                    > (((if iTemp759 != 0 {
                        fTemp1262
                            + fTemp768 * fTemp1263
                            + (fTemp1258 - (iTemp1259) as F64)
                                * (fTemp1261
                                    - (fTemp1262
                                        + fTemp768
                                            * (fTemp1263
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1260, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1261))))
                    } else {
                        1.0 - (fTemp1256
                            + fTemp768 * fTemp1257
                            + (fTemp1252 - (iTemp1253) as F64)
                                * (fTemp1255
                                    - (fTemp1256
                                        + fTemp768
                                            * (fTemp1257
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp1254, 8)) as usize]
                                                } - fTemp1255)))))
                    }) - fTemp1250)
                        / (1.0 - fTemp1250))) as i32;
                let mut fTemp1265: F64 = (if iTemp1264 != 0 { fTemp1234 } else { fTemp1237 });
                let mut fTemp1266: F64 = (if iTemp1264 != 0 { fTemp1237 } else { fTemp1235 });
                let mut fTemp1267: F64 = fTemp1266 + fTemp1265;
                let mut fTemp1268: F64 = 0.5 * fTemp1267;
                let mut fTemp1269: F64 = 524287.0 * (1.0 - fTemp1268);
                let mut iTemp1270: i32 = (fTemp1269) as i32;
                let mut iTemp1271: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1270, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1272: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1271, 7)) as usize] };
                let mut fTemp1273: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1271 as usize] };
                let mut fTemp1274: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1271, 1)) as usize] }
                        - fTemp1273;
                let mut fTemp1275: F64 = 262143.5 * fTemp1267;
                let mut iTemp1276: i32 = (fTemp1275) as i32;
                let mut iTemp1277: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1276, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1278: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1277, 7)) as usize] };
                let mut fTemp1279: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1277 as usize] };
                let mut fTemp1280: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1277, 1)) as usize] }
                        - fTemp1279;
                let mut fTemp1281: F64 = (if iTemp759 != 0 {
                    fTemp1279
                        + fTemp768 * fTemp1280
                        + (fTemp1275 - (iTemp1276) as F64)
                            * (fTemp1278
                                - (fTemp1279
                                    + fTemp768
                                        * (fTemp1280
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp1277, 8)) as usize]
                                            } - fTemp1278))))
                } else {
                    1.0 - (fTemp1273
                        + fTemp768 * fTemp1274
                        + (fTemp1269 - (iTemp1270) as F64)
                            * (fTemp1272
                                - (fTemp1273
                                    + fTemp768
                                        * (fTemp1274
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp1271, 8)) as usize]
                                            } - fTemp1272)))))
                });
                let mut fTemp1282: F64 = fTemp773 + fTemp1268;
                let mut fTemp1283: F64 = 524287.0 * (1.0 - fTemp1282);
                let mut iTemp1284: i32 = (fTemp1283) as i32;
                let mut iTemp1285: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1284, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1286: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1285, 7)) as usize] };
                let mut fTemp1287: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1285 as usize] };
                let mut fTemp1288: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1285, 1)) as usize] }
                        - fTemp1287;
                let mut fTemp1289: F64 = 524287.0 * fTemp1282;
                let mut iTemp1290: i32 = (fTemp1289) as i32;
                let mut iTemp1291: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1290, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1292: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1291, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp1293: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1291 as usize] };
                let mut fTemp1294: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1291, 1), 3670015),
                    )) as usize]
                } - fTemp1293;
                let mut iTemp1295: i32 = (fTemp829
                    > (((if iTemp759 != 0 {
                        fTemp1293
                            + fTemp768 * fTemp1294
                            + (fTemp1289 - (iTemp1290) as F64)
                                * (fTemp1292
                                    - (fTemp1293
                                        + fTemp768
                                            * (fTemp1294
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1291, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1292))))
                    } else {
                        1.0 - (fTemp1287
                            + fTemp768 * fTemp1288
                            + (fTemp1283 - (iTemp1284) as F64)
                                * (fTemp1286
                                    - (fTemp1287
                                        + fTemp768
                                            * (fTemp1288
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp1285, 8)) as usize]
                                                } - fTemp1286)))))
                    }) - fTemp1281)
                        / (1.0 - fTemp1281))) as i32;
                let mut fTemp1296: F64 = (if iTemp1295 != 0 { fTemp1265 } else { fTemp1268 });
                let mut fTemp1297: F64 = (if iTemp1295 != 0 { fTemp1268 } else { fTemp1266 });
                let mut fTemp1298: F64 = fTemp1297 + fTemp1296;
                let mut fTemp1299: F64 = 0.5 * fTemp1298;
                let mut fTemp1300: F64 = 524287.0 * (1.0 - fTemp1299);
                let mut iTemp1301: i32 = (fTemp1300) as i32;
                let mut iTemp1302: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1301, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1303: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1302, 7)) as usize] };
                let mut fTemp1304: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1302 as usize] };
                let mut fTemp1305: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1302, 1)) as usize] }
                        - fTemp1304;
                let mut fTemp1306: F64 = 262143.5 * fTemp1298;
                let mut iTemp1307: i32 = (fTemp1306) as i32;
                let mut iTemp1308: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1307, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1309: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1308, 7)) as usize] };
                let mut fTemp1310: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1308 as usize] };
                let mut fTemp1311: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1308, 1)) as usize] }
                        - fTemp1310;
                let mut fTemp1312: F64 = (if iTemp759 != 0 {
                    fTemp1310
                        + fTemp768 * fTemp1311
                        + (fTemp1306 - (iTemp1307) as F64)
                            * (fTemp1309
                                - (fTemp1310
                                    + fTemp768
                                        * (fTemp1311
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp1308, 8)) as usize]
                                            } - fTemp1309))))
                } else {
                    1.0 - (fTemp1304
                        + fTemp768 * fTemp1305
                        + (fTemp1300 - (iTemp1301) as F64)
                            * (fTemp1303
                                - (fTemp1304
                                    + fTemp768
                                        * (fTemp1305
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp1302, 8)) as usize]
                                            } - fTemp1303)))))
                });
                let mut fTemp1313: F64 = fTemp773 + fTemp1299;
                let mut fTemp1314: F64 = 524287.0 * (1.0 - fTemp1313);
                let mut iTemp1315: i32 = (fTemp1314) as i32;
                let mut iTemp1316: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1315, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1317: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1316, 7)) as usize] };
                let mut fTemp1318: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1316 as usize] };
                let mut fTemp1319: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1316, 1)) as usize] }
                        - fTemp1318;
                let mut fTemp1320: F64 = 524287.0 * fTemp1313;
                let mut iTemp1321: i32 = (fTemp1320) as i32;
                let mut iTemp1322: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1321, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1323: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1322, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp1324: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1322 as usize] };
                let mut fTemp1325: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1322, 1), 3670015),
                    )) as usize]
                } - fTemp1324;
                let mut iTemp1326: i32 = (fTemp829
                    > (((if iTemp759 != 0 {
                        fTemp1324
                            + fTemp768 * fTemp1325
                            + (fTemp1320 - (iTemp1321) as F64)
                                * (fTemp1323
                                    - (fTemp1324
                                        + fTemp768
                                            * (fTemp1325
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1322, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1323))))
                    } else {
                        1.0 - (fTemp1318
                            + fTemp768 * fTemp1319
                            + (fTemp1314 - (iTemp1315) as F64)
                                * (fTemp1317
                                    - (fTemp1318
                                        + fTemp768
                                            * (fTemp1319
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp1316, 8)) as usize]
                                                } - fTemp1317)))))
                    }) - fTemp1312)
                        / (1.0 - fTemp1312))) as i32;
                let mut fTemp1327: F64 = (if iTemp1326 != 0 { fTemp1296 } else { fTemp1299 });
                let mut fTemp1328: F64 = (if iTemp1326 != 0 { fTemp1299 } else { fTemp1297 });
                let mut fTemp1329: F64 = fTemp1328 + fTemp1327;
                let mut fTemp1330: F64 = 0.5 * fTemp1329;
                let mut fTemp1331: F64 = 524287.0 * (1.0 - fTemp1330);
                let mut iTemp1332: i32 = (fTemp1331) as i32;
                let mut iTemp1333: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1332, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1334: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1333, 7)) as usize] };
                let mut fTemp1335: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1333 as usize] };
                let mut fTemp1336: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1333, 1)) as usize] }
                        - fTemp1335;
                let mut fTemp1337: F64 = 262143.5 * fTemp1329;
                let mut iTemp1338: i32 = (fTemp1337) as i32;
                let mut iTemp1339: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1338, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1340: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1339, 7)) as usize] };
                let mut fTemp1341: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1339 as usize] };
                let mut fTemp1342: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1339, 1)) as usize] }
                        - fTemp1341;
                let mut fTemp1343: F64 = (if iTemp759 != 0 {
                    fTemp1341
                        + fTemp768 * fTemp1342
                        + (fTemp1337 - (iTemp1338) as F64)
                            * (fTemp1340
                                - (fTemp1341
                                    + fTemp768
                                        * (fTemp1342
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp1339, 8)) as usize]
                                            } - fTemp1340))))
                } else {
                    1.0 - (fTemp1335
                        + fTemp768 * fTemp1336
                        + (fTemp1331 - (iTemp1332) as F64)
                            * (fTemp1334
                                - (fTemp1335
                                    + fTemp768
                                        * (fTemp1336
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp1333, 8)) as usize]
                                            } - fTemp1334)))))
                });
                let mut fTemp1344: F64 = fTemp773 + fTemp1330;
                let mut fTemp1345: F64 = 524287.0 * (1.0 - fTemp1344);
                let mut iTemp1346: i32 = (fTemp1345) as i32;
                let mut iTemp1347: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1346, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1348: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1347, 7)) as usize] };
                let mut fTemp1349: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1347 as usize] };
                let mut fTemp1350: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1347, 1)) as usize] }
                        - fTemp1349;
                let mut fTemp1351: F64 = 524287.0 * fTemp1344;
                let mut iTemp1352: i32 = (fTemp1351) as i32;
                let mut iTemp1353: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1352, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1354: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1353, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp1355: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1353 as usize] };
                let mut fTemp1356: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1353, 1), 3670015),
                    )) as usize]
                } - fTemp1355;
                let mut iTemp1357: i32 = (fTemp829
                    > (((if iTemp759 != 0 {
                        fTemp1355
                            + fTemp768 * fTemp1356
                            + (fTemp1351 - (iTemp1352) as F64)
                                * (fTemp1354
                                    - (fTemp1355
                                        + fTemp768
                                            * (fTemp1356
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1353, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1354))))
                    } else {
                        1.0 - (fTemp1349
                            + fTemp768 * fTemp1350
                            + (fTemp1345 - (iTemp1346) as F64)
                                * (fTemp1348
                                    - (fTemp1349
                                        + fTemp768
                                            * (fTemp1350
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp1347, 8)) as usize]
                                                } - fTemp1348)))))
                    }) - fTemp1343)
                        / (1.0 - fTemp1343))) as i32;
                let mut fTemp1358: F64 = (if iTemp1357 != 0 { fTemp1327 } else { fTemp1330 });
                let mut fTemp1359: F64 = (if iTemp1357 != 0 { fTemp1330 } else { fTemp1328 });
                let mut fTemp1360: F64 = fTemp1359 + fTemp1358;
                let mut fTemp1361: F64 = 0.5 * fTemp1360;
                let mut fTemp1362: F64 = 524287.0 * (1.0 - fTemp1361);
                let mut iTemp1363: i32 = (fTemp1362) as i32;
                let mut iTemp1364: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1363, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1365: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1364, 7)) as usize] };
                let mut fTemp1366: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1364 as usize] };
                let mut fTemp1367: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1364, 1)) as usize] }
                        - fTemp1366;
                let mut fTemp1368: F64 = 262143.5 * fTemp1360;
                let mut iTemp1369: i32 = (fTemp1368) as i32;
                let mut iTemp1370: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1369, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1371: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1370, 7)) as usize] };
                let mut fTemp1372: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1370 as usize] };
                let mut fTemp1373: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1370, 1)) as usize] }
                        - fTemp1372;
                let mut fTemp1374: F64 = (if iTemp759 != 0 {
                    fTemp1372
                        + fTemp768 * fTemp1373
                        + (fTemp1368 - (iTemp1369) as F64)
                            * (fTemp1371
                                - (fTemp1372
                                    + fTemp768
                                        * (fTemp1373
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp1370, 8),
                                                        3670015,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp1371))))
                } else {
                    1.0 - (fTemp1366
                        + fTemp768 * fTemp1367
                        + (fTemp1362 - (iTemp1363) as F64)
                            * (fTemp1365
                                - (fTemp1366
                                    + fTemp768
                                        * (fTemp1367
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp1364, 8),
                                                        3670015,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp1365)))))
                });
                let mut fTemp1375: F64 = fTemp773 + fTemp1361;
                let mut fTemp1376: F64 = 524287.0 * (1.0 - fTemp1375);
                let mut iTemp1377: i32 = (fTemp1376) as i32;
                let mut iTemp1378: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1377, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1379: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1378, 7)) as usize] };
                let mut fTemp1380: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1378 as usize] };
                let mut fTemp1381: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1378, 1)) as usize] }
                        - fTemp1380;
                let mut fTemp1382: F64 = 524287.0 * fTemp1375;
                let mut iTemp1383: i32 = (fTemp1382) as i32;
                let mut iTemp1384: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1383, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1385: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1384, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp1386: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1384 as usize] };
                let mut fTemp1387: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1384, 1), 3670015),
                    )) as usize]
                } - fTemp1386;
                let mut iTemp1388: i32 = (fTemp829
                    > (((if iTemp759 != 0 {
                        fTemp1386
                            + fTemp768 * fTemp1387
                            + (fTemp1382 - (iTemp1383) as F64)
                                * (fTemp1385
                                    - (fTemp1386
                                        + fTemp768
                                            * (fTemp1387
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1384, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1385))))
                    } else {
                        1.0 - (fTemp1380
                            + fTemp768 * fTemp1381
                            + (fTemp1376 - (iTemp1377) as F64)
                                * (fTemp1379
                                    - (fTemp1380
                                        + fTemp768
                                            * (fTemp1381
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp1378, 8)) as usize]
                                                } - fTemp1379)))))
                    }) - fTemp1374)
                        / (1.0 - fTemp1374))) as i32;
                let mut fTemp1389: F64 = F64::min(
                    1.0,
                    F64::max(
                        0.0,
                        0.5 * ((if iTemp1388 != 0 { fTemp1361 } else { fTemp1359 })
                            + (if iTemp1388 != 0 { fTemp1358 } else { fTemp1361 })),
                    ),
                );
                self.fRec15[0] = fTemp1389;
                let mut fTemp1390: F64 = 524287.0 * (1.0 - fTemp1389);
                let mut iTemp1391: i32 = (fTemp1390) as i32;
                let mut iTemp1392: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1391, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1393: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1392, 7)) as usize] };
                let mut fTemp1394: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1392 as usize] };
                let mut fTemp1395: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1392, 1)) as usize] }
                        - fTemp1394;
                let mut fTemp1396: F64 = 524287.0 * fTemp1389;
                let mut iTemp1397: i32 = (fTemp1396) as i32;
                let mut iTemp1398: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1397, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1399: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1398, 7)) as usize] };
                let mut fTemp1400: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1398 as usize] };
                let mut fTemp1401: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1398, 1)) as usize] }
                        - fTemp1400;
                let mut fTemp1402: F64 = (if iTemp759 != 0 {
                    fTemp1400
                        + fTemp768 * fTemp1401
                        + (fTemp1396 - (iTemp1397) as F64)
                            * (fTemp1399
                                - (fTemp1400
                                    + fTemp768
                                        * (fTemp1401
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp1398, 8),
                                                        3670015,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp1399))))
                } else {
                    1.0 - (fTemp1394
                        + fTemp768 * fTemp1395
                        + (fTemp1390 - (iTemp1391) as F64)
                            * (fTemp1393
                                - (fTemp1394
                                    + fTemp768
                                        * (fTemp1395
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp1392, 8),
                                                        3670015,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp1393)))))
                });
                let mut fTemp1403: F64 = fTemp773 + fTemp1389;
                let mut fTemp1404: F64 = 524287.0 * (1.0 - fTemp1403);
                let mut iTemp1405: i32 = (fTemp1404) as i32;
                let mut iTemp1406: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1405, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1407: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1406, 7)) as usize] };
                let mut fTemp1408: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1406 as usize] };
                let mut fTemp1409: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1406, 1)) as usize] }
                        - fTemp1408;
                let mut fTemp1410: F64 = 524287.0 * fTemp1403;
                let mut iTemp1411: i32 = (fTemp1410) as i32;
                let mut iTemp1412: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp763,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1411, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1413: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1412, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp1414: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1412 as usize] };
                let mut fTemp1415: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1412, 1), 3670015),
                    )) as usize]
                } - fTemp1414;
                let mut fTemp1416: F64 = self.fRec16[1]
                    + (if ((0.001 * fTemp772) == 0.0) as i32 != 0 {
                        fTemp758
                    } else {
                        fTemp758
                            * ((if iTemp759 != 0 {
                                fTemp1414
                                    + fTemp768 * fTemp1415
                                    + (fTemp1410 - (iTemp1411) as F64)
                                        * (fTemp1413
                                            - (fTemp1414
                                                + fTemp768
                                                    * (fTemp1415
                                                        - (unsafe {
                                                            ftbl0LambRs192kSIG0[(std::cmp::max(
                                                                0,
                                                                std::cmp::min(
                                                                    i32::wrapping_add(iTemp1412, 8),
                                                                    3670015,
                                                                ),
                                                            ))
                                                                as usize]
                                                        } - fTemp1413))))
                            } else {
                                1.0 - (fTemp1408
                                    + fTemp768 * fTemp1409
                                    + (fTemp1404 - (iTemp1405) as F64)
                                        * (fTemp1407
                                            - (fTemp1408
                                                + fTemp768
                                                    * (fTemp1409
                                                        - (unsafe {
                                                            ftbl0LambRs192kSIG0[(i32::wrapping_add(
                                                                iTemp1406, 8,
                                                            ))
                                                                as usize]
                                                        } - fTemp1407)))))
                            }) - fTemp1402)
                            / (1.0 - fTemp1402)
                    });
                self.fRec16[0] = (if iTemp771 != 0 {
                    F64::min(fTemp1416, self.fRec16[1])
                } else {
                    F64::max(fTemp1416, self.fRec16[1])
                });
                self.fVec66[(self.IOTA0 & 32767) as usize] =
                    F64::powf(1e+01, 0.05 * self.fRec16[0]);
                let mut fTemp1417: F64 =
                    self.fVec66[((i32::wrapping_sub(self.IOTA0, iSlow78)) & 32767) as usize];
                *output1 = 0.5
                    * fTemp2
                    * self.fVec3[((i32::wrapping_sub(self.IOTA0, iSlow79)) & 32767) as usize]
                    + self.fRec14[0]
                        * fTemp4
                        * self.fVec4[((i32::wrapping_sub(self.IOTA0, iSlow79)) & 32767) as usize]
                        * fTemp1417;
                *output2 = fTemp3 + fTemp729 * fTemp4;
                *output3 = fTemp3 + fTemp4 * fTemp1417;
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
                self.fVec19[2] = self.fVec19[1];
                self.fVec19[1] = self.fVec19[0];
                for j2 in (1..=4).rev() {
                    self.fVec20[j2 as usize] = self.fVec20[(i32::wrapping_sub(j2, 1)) as usize];
                }
                for j3 in (1..=11).rev() {
                    self.fVec21[j3 as usize] = self.fVec21[(i32::wrapping_sub(j3, 1)) as usize];
                }
                self.fVec32[1] = self.fVec32[0];
                self.fVec33[1] = self.fVec33[0];
                self.fVec34[1] = self.fVec34[0];
                self.fRec1[1] = self.fRec1[0];
                self.fRec2[1] = self.fRec2[0];
                self.fRec14[1] = self.fRec14[0];
                self.fVec37[2] = self.fVec37[1];
                self.fVec37[1] = self.fVec37[0];
                for j4 in (1..=4).rev() {
                    self.fVec38[j4 as usize] = self.fVec38[(i32::wrapping_sub(j4, 1)) as usize];
                }
                for j5 in (1..=11).rev() {
                    self.fVec39[j5 as usize] = self.fVec39[(i32::wrapping_sub(j5, 1)) as usize];
                }
                self.fRec17[1] = self.fRec17[0];
                self.fVec50[2] = self.fVec50[1];
                self.fVec50[1] = self.fVec50[0];
                for j6 in (1..=4).rev() {
                    self.fVec51[j6 as usize] = self.fVec51[(i32::wrapping_sub(j6, 1)) as usize];
                }
                for j7 in (1..=11).rev() {
                    self.fVec52[j7 as usize] = self.fVec52[(i32::wrapping_sub(j7, 1)) as usize];
                }
                self.fVec63[1] = self.fVec63[0];
                self.fVec64[1] = self.fVec64[0];
                self.fVec65[1] = self.fVec65[0];
                self.fRec15[1] = self.fRec15[0];
                self.fRec16[1] = self.fRec16[0];
            }
        }
    }

    impl FaustDsp for LambRs192k {
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

pub use dsp_192k::LambRs192k;
