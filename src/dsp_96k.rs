/* ------------------------------------------------------------
author: "Bart Brouns"
license: "AGPLv3"
name: "lamb-rs"
version: "0.1"
Code generated with Faust 2.79.3 (https://faust.grame.fr)
Compilation options: -a /tmp/.tmp4G6396 -lang rust -ct 1 -cn LambRs96k -es 1 -mcd 16 -mdd 1024 -mdy 33 -double -ftz 0
------------------------------------------------------------ */
mod dsp_96k {
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

    pub struct LambRs96kSIG0 {
        iRec13: [i32; 2],
    }

    impl LambRs96kSIG0 {
        fn get_num_inputsLambRs96kSIG0(&self) -> i32 {
            return 0;
        }
        fn get_num_outputsLambRs96kSIG0(&self) -> i32 {
            return 1;
        }

        pub fn instance_initLambRs96kSIG0(&mut self, sample_rate: i32) {
            for l42 in 0..2 {
                self.iRec13[l42 as usize] = 0;
            }
        }

        pub fn fillLambRs96kSIG0(&mut self, count: i32, table: &mut [FaustFloat]) {
            for i1 in 0..count {
                self.iRec13[0] = i32::wrapping_add(self.iRec13[1], 1);
                let mut fTemp66: F64 = (self.iRec13[1] % 7) as F64 as i32 as F64;
                let mut fTemp67: F64 = 0.16666666666666666 * fTemp66;
                let mut fTemp68: F64 = F64::powf(fTemp67, 0.06999999999999999 * fTemp66 + 1.0);
                let mut fTemp69: F64 =
                    (0.14285714285714285 * (self.iRec13[1] % 1835008) as F64) as i32 as F64;
                table[i1 as usize] = F64::min(
                    1.0,
                    F64::max(
                        0.0,
                        (if (fTemp67 == 0.0) as i32 != 0 {
                            0.5 * (F64::sin(1.1984270621720943e-05 * fTemp69 + 4.71238898038469)
                                + 1.0)
                        } else {
                            0.5 * (F64::sin(
                                3.141592653589793
                                    * ((1.0
                                        - F64::exp(-(9.231602598581689e-06 * fTemp68 * fTemp69)))
                                        / (1.0 - F64::exp(-(2.42 * fTemp68))))
                                    + 4.71238898038469,
                            ) + 1.0)
                        }),
                    ),
                );
                self.iRec13[1] = self.iRec13[0];
            }
        }
    }

    pub fn newLambRs96kSIG0() -> LambRs96kSIG0 {
        LambRs96kSIG0 { iRec13: [0; 2] }
    }
    fn LambRs96k_faustpower2_f(value: F64) -> F64 {
        return value * value;
    }
    static mut ftbl0LambRs96kSIG0: [F64; 1835008] = [0.0; 1835008];
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
    pub struct LambRs96k {
        fCheckbox0: F64,
        IOTA0: i32,
        iVec0: [i32; 16384],
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
        fRec3: [F64; 2],
        fVec18: [F64; 3],
        fVec19: [F64; 5],
        fVec20: [F64; 12],
        fVec21: [F64; 32],
        fVec22: [F64; 64],
        fVec23: [F64; 128],
        fVec24: [F64; 256],
        fVec25: [F64; 512],
        fVec26: [F64; 1024],
        fVec27: [F64; 2048],
        fVec28: [F64; 4096],
        fVec29: [F64; 8192],
        fVec30: [F64; 2],
        fHslider10: F64,
        fHslider11: F64,
        fVec31: [F64; 2],
        fVec32: [F64; 2],
        fConst10: F64,
        fRec1: [F64; 2],
        fRec2: [F64; 2],
        fVec33: [F64; 16384],
        fCheckbox1: F64,
        fHbargraph0: F64,
        fHslider12: F64,
        fRec14: [F64; 2],
        fVec34: [F64; 16384],
        fVec35: [F64; 3],
        fVec36: [F64; 5],
        fVec37: [F64; 12],
        fVec38: [F64; 32],
        fVec39: [F64; 64],
        fVec40: [F64; 128],
        fVec41: [F64; 256],
        fVec42: [F64; 512],
        fVec43: [F64; 1024],
        fVec44: [F64; 2048],
        fVec45: [F64; 4096],
        fVec46: [F64; 8192],
        fRec17: [F64; 2],
        fVec47: [F64; 3],
        fVec48: [F64; 5],
        fVec49: [F64; 12],
        fVec50: [F64; 32],
        fVec51: [F64; 64],
        fVec52: [F64; 128],
        fVec53: [F64; 256],
        fVec54: [F64; 512],
        fVec55: [F64; 1024],
        fVec56: [F64; 2048],
        fVec57: [F64; 4096],
        fVec58: [F64; 8192],
        fVec59: [F64; 2],
        fVec60: [F64; 2],
        fVec61: [F64; 2],
        fRec15: [F64; 2],
        fRec16: [F64; 2],
        fVec62: [F64; 16384],
    }

    impl LambRs96k {
        pub fn new() -> LambRs96k {
            LambRs96k {
                fCheckbox0: 0.0,
                IOTA0: 0,
                iVec0: [0; 16384],
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
                fRec3: [0.0; 2],
                fVec18: [0.0; 3],
                fVec19: [0.0; 5],
                fVec20: [0.0; 12],
                fVec21: [0.0; 32],
                fVec22: [0.0; 64],
                fVec23: [0.0; 128],
                fVec24: [0.0; 256],
                fVec25: [0.0; 512],
                fVec26: [0.0; 1024],
                fVec27: [0.0; 2048],
                fVec28: [0.0; 4096],
                fVec29: [0.0; 8192],
                fVec30: [0.0; 2],
                fHslider10: 0.0,
                fHslider11: 0.0,
                fVec31: [0.0; 2],
                fVec32: [0.0; 2],
                fConst10: 0.0,
                fRec1: [0.0; 2],
                fRec2: [0.0; 2],
                fVec33: [0.0; 16384],
                fCheckbox1: 0.0,
                fHbargraph0: 0.0,
                fHslider12: 0.0,
                fRec14: [0.0; 2],
                fVec34: [0.0; 16384],
                fVec35: [0.0; 3],
                fVec36: [0.0; 5],
                fVec37: [0.0; 12],
                fVec38: [0.0; 32],
                fVec39: [0.0; 64],
                fVec40: [0.0; 128],
                fVec41: [0.0; 256],
                fVec42: [0.0; 512],
                fVec43: [0.0; 1024],
                fVec44: [0.0; 2048],
                fVec45: [0.0; 4096],
                fVec46: [0.0; 8192],
                fRec17: [0.0; 2],
                fVec47: [0.0; 3],
                fVec48: [0.0; 5],
                fVec49: [0.0; 12],
                fVec50: [0.0; 32],
                fVec51: [0.0; 64],
                fVec52: [0.0; 128],
                fVec53: [0.0; 256],
                fVec54: [0.0; 512],
                fVec55: [0.0; 1024],
                fVec56: [0.0; 2048],
                fVec57: [0.0; 4096],
                fVec58: [0.0; 8192],
                fVec59: [0.0; 2],
                fVec60: [0.0; 2],
                fVec61: [0.0; 2],
                fRec15: [0.0; 2],
                fRec16: [0.0; 2],
                fVec62: [0.0; 16384],
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
            m.declare("compile_options", r"-a /tmp/.tmp4G6396 -lang rust -ct 1 -cn LambRs96k -es 1 -mcd 16 -mdd 1024 -mdy 33 -double -ftz 0");
            m.declare("filename", r"lamb-rs-96k.dsp");
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
            let mut sig0: LambRs96kSIG0 = newLambRs96kSIG0();
            sig0.instance_initLambRs96kSIG0(sample_rate);
            sig0.fillLambRs96kSIG0(1835008, unsafe { &mut ftbl0LambRs96kSIG0 });
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
            for l0 in 0..16384 {
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
            for l28 in 0..2 {
                self.fRec3[l28 as usize] = 0.0;
            }
            for l29 in 0..3 {
                self.fVec18[l29 as usize] = 0.0;
            }
            for l30 in 0..5 {
                self.fVec19[l30 as usize] = 0.0;
            }
            for l31 in 0..12 {
                self.fVec20[l31 as usize] = 0.0;
            }
            for l32 in 0..32 {
                self.fVec21[l32 as usize] = 0.0;
            }
            for l33 in 0..64 {
                self.fVec22[l33 as usize] = 0.0;
            }
            for l34 in 0..128 {
                self.fVec23[l34 as usize] = 0.0;
            }
            for l35 in 0..256 {
                self.fVec24[l35 as usize] = 0.0;
            }
            for l36 in 0..512 {
                self.fVec25[l36 as usize] = 0.0;
            }
            for l37 in 0..1024 {
                self.fVec26[l37 as usize] = 0.0;
            }
            for l38 in 0..2048 {
                self.fVec27[l38 as usize] = 0.0;
            }
            for l39 in 0..4096 {
                self.fVec28[l39 as usize] = 0.0;
            }
            for l40 in 0..8192 {
                self.fVec29[l40 as usize] = 0.0;
            }
            for l41 in 0..2 {
                self.fVec30[l41 as usize] = 0.0;
            }
            for l43 in 0..2 {
                self.fVec31[l43 as usize] = 0.0;
            }
            for l44 in 0..2 {
                self.fVec32[l44 as usize] = 0.0;
            }
            for l45 in 0..2 {
                self.fRec1[l45 as usize] = 0.0;
            }
            for l46 in 0..2 {
                self.fRec2[l46 as usize] = 0.0;
            }
            for l47 in 0..16384 {
                self.fVec33[l47 as usize] = 0.0;
            }
            for l48 in 0..2 {
                self.fRec14[l48 as usize] = 0.0;
            }
            for l49 in 0..16384 {
                self.fVec34[l49 as usize] = 0.0;
            }
            for l50 in 0..3 {
                self.fVec35[l50 as usize] = 0.0;
            }
            for l51 in 0..5 {
                self.fVec36[l51 as usize] = 0.0;
            }
            for l52 in 0..12 {
                self.fVec37[l52 as usize] = 0.0;
            }
            for l53 in 0..32 {
                self.fVec38[l53 as usize] = 0.0;
            }
            for l54 in 0..64 {
                self.fVec39[l54 as usize] = 0.0;
            }
            for l55 in 0..128 {
                self.fVec40[l55 as usize] = 0.0;
            }
            for l56 in 0..256 {
                self.fVec41[l56 as usize] = 0.0;
            }
            for l57 in 0..512 {
                self.fVec42[l57 as usize] = 0.0;
            }
            for l58 in 0..1024 {
                self.fVec43[l58 as usize] = 0.0;
            }
            for l59 in 0..2048 {
                self.fVec44[l59 as usize] = 0.0;
            }
            for l60 in 0..4096 {
                self.fVec45[l60 as usize] = 0.0;
            }
            for l61 in 0..8192 {
                self.fVec46[l61 as usize] = 0.0;
            }
            for l62 in 0..2 {
                self.fRec17[l62 as usize] = 0.0;
            }
            for l63 in 0..3 {
                self.fVec47[l63 as usize] = 0.0;
            }
            for l64 in 0..5 {
                self.fVec48[l64 as usize] = 0.0;
            }
            for l65 in 0..12 {
                self.fVec49[l65 as usize] = 0.0;
            }
            for l66 in 0..32 {
                self.fVec50[l66 as usize] = 0.0;
            }
            for l67 in 0..64 {
                self.fVec51[l67 as usize] = 0.0;
            }
            for l68 in 0..128 {
                self.fVec52[l68 as usize] = 0.0;
            }
            for l69 in 0..256 {
                self.fVec53[l69 as usize] = 0.0;
            }
            for l70 in 0..512 {
                self.fVec54[l70 as usize] = 0.0;
            }
            for l71 in 0..1024 {
                self.fVec55[l71 as usize] = 0.0;
            }
            for l72 in 0..2048 {
                self.fVec56[l72 as usize] = 0.0;
            }
            for l73 in 0..4096 {
                self.fVec57[l73 as usize] = 0.0;
            }
            for l74 in 0..8192 {
                self.fVec58[l74 as usize] = 0.0;
            }
            for l75 in 0..2 {
                self.fVec59[l75 as usize] = 0.0;
            }
            for l76 in 0..2 {
                self.fVec60[l76 as usize] = 0.0;
            }
            for l77 in 0..2 {
                self.fVec61[l77 as usize] = 0.0;
            }
            for l78 in 0..2 {
                self.fRec15[l78 as usize] = 0.0;
            }
            for l79 in 0..2 {
                self.fRec16[l79 as usize] = 0.0;
            }
            for l80 in 0..16384 {
                self.fVec62[l80 as usize] = 0.0;
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
            LambRs96k::class_init(sample_rate);
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
                0.010416666666666666,
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
            ui_interface.add_horizontal_bargraph("latency", ParamIndex(15), 0.0, 9.6e+03);
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
            let mut iSlow47: i32 = (F64::floor(0.5 * fSlow3)) as i32 % 2;
            let mut iSlow48: i32 = (F64::floor(0.25 * fSlow3)) as i32 % 2;
            let mut iSlow49: i32 = i32::wrapping_add(iSlow4, i32::wrapping_mul(2, iSlow47));
            let mut iSlow50: i32 = (F64::floor(0.125 * fSlow3)) as i32 % 2;
            let mut iSlow51: i32 = i32::wrapping_add(iSlow49, i32::wrapping_mul(4, iSlow48));
            let mut iSlow52: i32 = (F64::floor(0.0625 * fSlow3)) as i32 % 2;
            let mut iSlow53: i32 = i32::wrapping_add(iSlow51, i32::wrapping_mul(8, iSlow50));
            let mut iSlow54: i32 = (F64::floor(0.03125 * fSlow3)) as i32 % 2;
            let mut iSlow55: i32 = i32::wrapping_add(iSlow53, i32::wrapping_mul(16, iSlow52));
            let mut iSlow56: i32 = (F64::floor(0.015625 * fSlow3)) as i32 % 2;
            let mut iSlow57: i32 = i32::wrapping_add(iSlow55, i32::wrapping_mul(32, iSlow54));
            let mut iSlow58: i32 = (F64::floor(0.0078125 * fSlow3)) as i32 % 2;
            let mut iSlow59: i32 = i32::wrapping_add(iSlow57, i32::wrapping_mul(64, iSlow56));
            let mut iSlow60: i32 = (F64::floor(0.00390625 * fSlow3)) as i32 % 2;
            let mut iSlow61: i32 = i32::wrapping_add(iSlow59, i32::wrapping_mul(128, iSlow58));
            let mut iSlow62: i32 = (F64::floor(0.001953125 * fSlow3)) as i32 % 2;
            let mut iSlow63: i32 = i32::wrapping_add(iSlow61, i32::wrapping_mul(256, iSlow60));
            let mut iSlow64: i32 = (F64::floor(0.0009765625 * fSlow3)) as i32 % 2;
            let mut iSlow65: i32 = i32::wrapping_add(iSlow63, i32::wrapping_mul(512, iSlow62));
            let mut iSlow66: i32 = (F64::floor(0.00048828125 * fSlow3)) as i32 % 2;
            let mut iSlow67: i32 = i32::wrapping_add(iSlow65, i32::wrapping_mul(1024, iSlow64));
            let mut iSlow68: i32 = (F64::floor(0.000244140625 * fSlow3)) as i32 % 2;
            let mut iSlow69: i32 = i32::wrapping_add(iSlow67, i32::wrapping_mul(2048, iSlow66));
            let mut fSlow70: F64 = self.fHslider10;
            let mut fSlow71: F64 = self.fHslider11;
            let mut fSlow72: F64 = self.fConst0 * (0.001 * fSlow19 + 1e-05 * fSlow2);
            let mut fSlow73: F64 = self.fCheckbox1;
            let mut iSlow74: i32 = (F64::max(0.0, fSlow73 * (9.6e+03 - fSlow72))) as i32;
            self.fHbargraph0 = (if (fSlow73) as i32 != 0 {
                9.6e+03
            } else {
                fSlow72
            });
            let mut iSlow75: i32 = (self.fHbargraph0) as i32;
            let mut fSlow76: F64 = self.fConst3 * F64::powf(1e+01, 0.05 * self.fHslider12);
            let zipped_iterators = inputs0
                .zip(inputs1)
                .zip(outputs0)
                .zip(outputs1)
                .zip(outputs2)
                .zip(outputs3);
            for (((((input0, input1), output0), output1), output2), output3) in zipped_iterators {
                self.iVec0[(self.IOTA0 & 16383) as usize] = 1;
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
                                    fSlow12 * LambRs96k_faustpower2_f(fSlow7 + fTemp20)
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
                    (self.iVec0[((i32::wrapping_sub(self.IOTA0, 9600)) & 16383) as usize]) as F64;
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
                                    fSlow12 * LambRs96k_faustpower2_f(fSlow7 + fTemp40)
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
                self.fVec17[(self.IOTA0 & 8191) as usize] = F64::min(
                    fTemp52,
                    self.fVec16[((i32::wrapping_sub(self.IOTA0, 2048)) & 4095) as usize],
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
                                                self.fVec12[((i32::wrapping_sub(
                                                    self.IOTA0, iSlow36,
                                                )) & 255)
                                                    as usize]
                                            } else {
                                                1.7976931348623157e+308
                                            }),
                                        ),
                                        (if iSlow37 != 0 {
                                            self.fVec13[((i32::wrapping_sub(self.IOTA0, iSlow38))
                                                & 511)
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
                                    self.fVec15
                                        [((i32::wrapping_sub(self.IOTA0, iSlow42)) & 2047) as usize]
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
                            self.fVec17[((i32::wrapping_sub(self.IOTA0, iSlow46)) & 8191) as usize]
                        } else {
                            1.7976931348623157e+308
                        }),
                    ),
                );
                let mut fTemp53: F64 = F64::min(self.fRec3[0], self.fRec3[1]);
                self.fVec18[0] = fTemp53;
                let mut fTemp54: F64 = F64::min(fTemp53, self.fVec18[2]);
                self.fVec19[0] = fTemp54;
                let mut fTemp55: F64 = F64::min(fTemp54, self.fVec19[4]);
                self.fVec20[0] = fTemp55;
                let mut fTemp56: F64 = F64::min(fTemp55, self.fVec20[8]);
                self.fVec21[(self.IOTA0 & 31) as usize] = fTemp56;
                let mut fTemp57: F64 = F64::min(
                    fTemp56,
                    self.fVec21[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize],
                );
                self.fVec22[(self.IOTA0 & 63) as usize] = fTemp57;
                let mut fTemp58: F64 = F64::min(
                    fTemp57,
                    self.fVec22[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize],
                );
                self.fVec23[(self.IOTA0 & 127) as usize] = fTemp58;
                let mut fTemp59: F64 = F64::min(
                    fTemp58,
                    self.fVec23[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize],
                );
                self.fVec24[(self.IOTA0 & 255) as usize] = fTemp59;
                let mut fTemp60: F64 = F64::min(
                    fTemp59,
                    self.fVec24[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize],
                );
                self.fVec25[(self.IOTA0 & 511) as usize] = fTemp60;
                let mut fTemp61: F64 = F64::min(
                    fTemp60,
                    self.fVec25[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize],
                );
                self.fVec26[(self.IOTA0 & 1023) as usize] = fTemp61;
                let mut fTemp62: F64 = F64::min(
                    fTemp61,
                    self.fVec26[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize],
                );
                self.fVec27[(self.IOTA0 & 2047) as usize] = fTemp62;
                let mut fTemp63: F64 = F64::min(
                    fTemp62,
                    self.fVec27[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize],
                );
                self.fVec28[(self.IOTA0 & 4095) as usize] = fTemp63;
                self.fVec29[(self.IOTA0 & 8191) as usize] = F64::min(
                    fTemp63,
                    self.fVec28[((i32::wrapping_sub(self.IOTA0, 2048)) & 4095) as usize],
                );
                let mut fTemp64: F64 = F64::min(
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
                                                                (if iSlow47 != 0 {
                                                                    self.fVec18[iSlow4 as usize]
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
                                                            self.fVec20[iSlow51 as usize]
                                                        } else {
                                                            1.7976931348623157e+308
                                                        }),
                                                    ),
                                                    (if iSlow52 != 0 {
                                                        self.fVec21[((i32::wrapping_sub(
                                                            self.IOTA0, iSlow53,
                                                        )) & 31)
                                                            as usize]
                                                    } else {
                                                        1.7976931348623157e+308
                                                    }),
                                                ),
                                                (if iSlow54 != 0 {
                                                    self.fVec22[((i32::wrapping_sub(
                                                        self.IOTA0, iSlow55,
                                                    )) & 63)
                                                        as usize]
                                                } else {
                                                    1.7976931348623157e+308
                                                }),
                                            ),
                                            (if iSlow56 != 0 {
                                                self.fVec23[((i32::wrapping_sub(
                                                    self.IOTA0, iSlow57,
                                                )) & 127)
                                                    as usize]
                                            } else {
                                                1.7976931348623157e+308
                                            }),
                                        ),
                                        (if iSlow58 != 0 {
                                            self.fVec24[((i32::wrapping_sub(self.IOTA0, iSlow59))
                                                & 255)
                                                as usize]
                                        } else {
                                            1.7976931348623157e+308
                                        }),
                                    ),
                                    (if iSlow60 != 0 {
                                        self.fVec25[((i32::wrapping_sub(self.IOTA0, iSlow61)) & 511)
                                            as usize]
                                    } else {
                                        1.7976931348623157e+308
                                    }),
                                ),
                                (if iSlow62 != 0 {
                                    self.fVec26
                                        [((i32::wrapping_sub(self.IOTA0, iSlow63)) & 1023) as usize]
                                } else {
                                    1.7976931348623157e+308
                                }),
                            ),
                            (if iSlow64 != 0 {
                                self.fVec27
                                    [((i32::wrapping_sub(self.IOTA0, iSlow65)) & 2047) as usize]
                            } else {
                                1.7976931348623157e+308
                            }),
                        ),
                        (if iSlow66 != 0 {
                            self.fVec28[((i32::wrapping_sub(self.IOTA0, iSlow67)) & 4095) as usize]
                        } else {
                            1.7976931348623157e+308
                        }),
                    ),
                    (if iSlow68 != 0 {
                        self.fVec29[((i32::wrapping_sub(self.IOTA0, iSlow69)) & 8191) as usize]
                    } else {
                        1.7976931348623157e+308
                    }),
                ) - self.fRec2[1];
                self.fVec30[0] = fTemp64;
                let mut iTemp65: i32 = (fTemp64 > 0.0) as i32;
                let mut fTemp70: F64 = (if iTemp65 != 0 { fSlow71 } else { fSlow70 });
                self.fVec31[0] = fTemp70;
                let mut fTemp71: F64 = 6.0 * fTemp70;
                let mut iTemp72: i32 = (fTemp71) as i32;
                let mut iTemp73: i32 = std::cmp::max(0, std::cmp::min(iTemp72, 6));
                let mut iTemp74: i32 = std::cmp::max(
                    0,
                    std::cmp::min(i32::wrapping_add(iTemp73, 917497), 1835007),
                );
                let mut fTemp75: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp74, 7)) as usize] };
                let mut fTemp76: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp74 as usize] };
                let mut fTemp77: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp74, 1)) as usize] }
                        - fTemp76;
                let mut fTemp78: F64 = fTemp71 - (iTemp72) as F64;
                let mut fTemp79: F64 = fTemp76
                    + fTemp78 * fTemp77
                    + 0.5
                        * (fTemp75
                            - (fTemp76
                                + fTemp78
                                    * (fTemp77
                                        - (unsafe {
                                            ftbl0LambRs96kSIG0
                                                [(i32::wrapping_add(iTemp74, 8)) as usize]
                                        } - fTemp75))));
                let mut fTemp80: F64 = (if iTemp65 != 0 { fTemp79 } else { 1.0 - fTemp79 });
                let mut iTemp81: i32 = (fTemp64 < 0.0) as i32;
                let mut fTemp82: F64 = fSlow1 * (iTemp81) as F64 + fSlow13 * (iTemp65) as F64;
                self.fVec32[0] = fTemp82;
                let mut fTemp83: F64 = self.fConst10 / fTemp82;
                let mut fTemp84: F64 = fTemp83 + 0.5;
                let mut fTemp85: F64 = 262143.0 * (1.0 - fTemp84);
                let mut iTemp86: i32 = (fTemp85) as i32;
                let mut iTemp87: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp86, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp88: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp87, 7)) as usize] };
                let mut fTemp89: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp87 as usize] };
                let mut fTemp90: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp87, 1)) as usize] }
                        - fTemp89;
                let mut fTemp91: F64 = 262143.0 * fTemp84;
                let mut iTemp92: i32 = (fTemp91) as i32;
                let mut iTemp93: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp92, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp94: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp93, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp95: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp93 as usize] };
                let mut fTemp96: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp93, 1), 1835007),
                    )) as usize]
                } - fTemp95;
                let mut fTemp97: F64 = 6.0 * self.fVec31[1];
                let mut iTemp98: i32 = (fTemp97) as i32;
                let mut iTemp99: i32 = std::cmp::max(0, std::cmp::min(iTemp98, 6));
                let mut fTemp100: F64 = 262143.0 * (1.0 - self.fRec1[1]);
                let mut iTemp101: i32 = (fTemp100) as i32;
                let mut iTemp102: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp101, 262143))),
                            iTemp99,
                        ),
                        1835007,
                    ),
                );
                let mut fTemp103: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp102, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp104: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp102 as usize] };
                let mut fTemp105: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp102, 1), 1835007),
                    )) as usize]
                } - fTemp104;
                let mut fTemp106: F64 = fTemp97 - (iTemp98) as F64;
                let mut fTemp107: F64 = 262143.0 * self.fRec1[1];
                let mut iTemp108: i32 = (fTemp107) as i32;
                let mut iTemp109: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp99,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp108, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp110: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp109, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp111: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp109 as usize] };
                let mut fTemp112: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp109, 1), 1835007),
                    )) as usize]
                } - fTemp111;
                let mut fTemp113: F64 = self.fRec1[1] + fTemp83;
                let mut fTemp114: F64 = 262143.0 * (1.0 - fTemp113);
                let mut iTemp115: i32 = (fTemp114) as i32;
                let mut iTemp116: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp115, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp117: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp116, 7)) as usize] };
                let mut fTemp118: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp116 as usize] };
                let mut fTemp119: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp116, 1)) as usize] }
                        - fTemp118;
                let mut fTemp120: F64 = 262143.0 * fTemp113;
                let mut iTemp121: i32 = (fTemp120) as i32;
                let mut iTemp122: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp121, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp123: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp122, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp124: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp122 as usize] };
                let mut fTemp125: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp122, 1), 1835007),
                    )) as usize]
                } - fTemp124;
                let mut fTemp126: F64 =
                    self.fRec1[1] + self.fConst10 * (1.0 / fTemp82 + 1.0 / self.fVec32[1]);
                let mut fTemp127: F64 = 262143.0 * (1.0 - fTemp126);
                let mut iTemp128: i32 = (fTemp127) as i32;
                let mut iTemp129: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp128, 262143))),
                            iTemp73,
                        ),
                        1835007,
                    ),
                );
                let mut fTemp130: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp129, 7)) as usize] };
                let mut fTemp131: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp129 as usize] };
                let mut fTemp132: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp129, 1)) as usize] }
                        - fTemp131;
                let mut fTemp133: F64 = 262143.0 * fTemp126;
                let mut iTemp134: i32 = (fTemp133) as i32;
                let mut iTemp135: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp134, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp136: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp135, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp137: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp135 as usize] };
                let mut fTemp138: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp135, 1), 1835007),
                    )) as usize]
                } - fTemp137;
                let mut fTemp139: F64 = ((if iTemp65 != 0 {
                    fTemp137
                        + fTemp78 * fTemp138
                        + (fTemp133 - (iTemp134) as F64)
                            * (fTemp136
                                - (fTemp137
                                    + fTemp78
                                        * (fTemp138
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp135, 8),
                                                        1835007,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp136))))
                } else {
                    1.0 - (fTemp131
                        + fTemp78 * fTemp132
                        + (fTemp127 - (iTemp128) as F64)
                            * (fTemp130
                                - (fTemp131
                                    + fTemp78
                                        * (fTemp132
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp129, 8)) as usize]
                                            } - fTemp130)))))
                }) - (if iTemp65 != 0 {
                    fTemp124
                        + fTemp78 * fTemp125
                        + (fTemp120 - (iTemp121) as F64)
                            * (fTemp123
                                - (fTemp124
                                    + fTemp78
                                        * (fTemp125
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp122, 8),
                                                        1835007,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp123))))
                } else {
                    1.0 - (fTemp118
                        + fTemp78 * fTemp119
                        + (fTemp114 - (iTemp115) as F64)
                            * (fTemp117
                                - (fTemp118
                                    + fTemp78
                                        * (fTemp119
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp116, 8)) as usize]
                                            } - fTemp117)))))
                })) * self.fVec30[1]
                    / (fTemp64
                        * (1.0
                            - (if iTemp65 != 0 {
                                fTemp111
                                    + fTemp106 * fTemp112
                                    + (fTemp107 - (iTemp108) as F64)
                                        * (fTemp110
                                            - (fTemp111
                                                + fTemp106
                                                    * (fTemp112
                                                        - (unsafe {
                                                            ftbl0LambRs96kSIG0[(std::cmp::max(
                                                                0,
                                                                std::cmp::min(
                                                                    i32::wrapping_add(iTemp109, 8),
                                                                    1835007,
                                                                ),
                                                            ))
                                                                as usize]
                                                        } - fTemp110))))
                            } else {
                                1.0 - (fTemp104
                                    + fTemp106 * fTemp105
                                    + (fTemp100 - (iTemp101) as F64)
                                        * (fTemp103
                                            - (fTemp104
                                                + fTemp106
                                                    * (fTemp105
                                                        - (unsafe {
                                                            ftbl0LambRs96kSIG0[(std::cmp::max(
                                                                0,
                                                                std::cmp::min(
                                                                    i32::wrapping_add(iTemp102, 8),
                                                                    1835007,
                                                                ),
                                                            ))
                                                                as usize]
                                                        } - fTemp103)))))
                            })));
                let mut iTemp140: i32 = (fTemp139
                    > (((if iTemp65 != 0 {
                        fTemp95
                            + fTemp78 * fTemp96
                            + (fTemp91 - (iTemp92) as F64)
                                * (fTemp94
                                    - (fTemp95
                                        + fTemp78
                                            * (fTemp96
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp93, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp94))))
                    } else {
                        1.0 - (fTemp89
                            + fTemp78 * fTemp90
                            + (fTemp85 - (iTemp86) as F64)
                                * (fTemp88
                                    - (fTemp89
                                        + fTemp78
                                            * (fTemp90
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp87, 8)) as usize]
                                                } - fTemp88)))))
                    }) - fTemp80)
                        / (1.0 - fTemp80))) as i32;
                let mut fTemp141: F64 = (if iTemp140 != 0 { 1.0 } else { 0.5 });
                let mut fTemp142: F64 = (if iTemp140 != 0 { 0.5 } else { 0.0 });
                let mut fTemp143: F64 = fTemp142 + fTemp141;
                let mut fTemp144: F64 = 0.5 * fTemp143;
                let mut fTemp145: F64 = 262143.0 * (1.0 - fTemp144);
                let mut iTemp146: i32 = (fTemp145) as i32;
                let mut iTemp147: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp146, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp148: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp147, 7)) as usize] };
                let mut fTemp149: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp147 as usize] };
                let mut fTemp150: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp147, 1)) as usize] }
                        - fTemp149;
                let mut fTemp151: F64 = 131071.5 * fTemp143;
                let mut iTemp152: i32 = (fTemp151) as i32;
                let mut iTemp153: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp152, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp154: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp153, 7)) as usize] };
                let mut fTemp155: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp153 as usize] };
                let mut fTemp156: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp153, 1)) as usize] }
                        - fTemp155;
                let mut fTemp157: F64 = (if iTemp65 != 0 {
                    fTemp155
                        + fTemp78 * fTemp156
                        + (fTemp151 - (iTemp152) as F64)
                            * (fTemp154
                                - (fTemp155
                                    + fTemp78
                                        * (fTemp156
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp153, 8)) as usize]
                                            } - fTemp154))))
                } else {
                    1.0 - (fTemp149
                        + fTemp78 * fTemp150
                        + (fTemp145 - (iTemp146) as F64)
                            * (fTemp148
                                - (fTemp149
                                    + fTemp78
                                        * (fTemp150
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp147, 8)) as usize]
                                            } - fTemp148)))))
                });
                let mut fTemp158: F64 = fTemp83 + fTemp144;
                let mut fTemp159: F64 = 262143.0 * (1.0 - fTemp158);
                let mut iTemp160: i32 = (fTemp159) as i32;
                let mut iTemp161: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp160, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp162: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp161, 7)) as usize] };
                let mut fTemp163: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp161 as usize] };
                let mut fTemp164: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp161, 1)) as usize] }
                        - fTemp163;
                let mut fTemp165: F64 = 262143.0 * fTemp158;
                let mut iTemp166: i32 = (fTemp165) as i32;
                let mut iTemp167: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp166, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp168: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp167, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp169: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp167 as usize] };
                let mut fTemp170: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp167, 1), 1835007),
                    )) as usize]
                } - fTemp169;
                let mut iTemp171: i32 = (fTemp139
                    > (((if iTemp65 != 0 {
                        fTemp169
                            + fTemp78 * fTemp170
                            + (fTemp165 - (iTemp166) as F64)
                                * (fTemp168
                                    - (fTemp169
                                        + fTemp78
                                            * (fTemp170
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp167, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp168))))
                    } else {
                        1.0 - (fTemp163
                            + fTemp78 * fTemp164
                            + (fTemp159 - (iTemp160) as F64)
                                * (fTemp162
                                    - (fTemp163
                                        + fTemp78
                                            * (fTemp164
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp161, 8)) as usize]
                                                } - fTemp162)))))
                    }) - fTemp157)
                        / (1.0 - fTemp157))) as i32;
                let mut fTemp172: F64 = (if iTemp171 != 0 { fTemp141 } else { fTemp144 });
                let mut fTemp173: F64 = (if iTemp171 != 0 { fTemp144 } else { fTemp142 });
                let mut fTemp174: F64 = fTemp173 + fTemp172;
                let mut fTemp175: F64 = 0.5 * fTemp174;
                let mut fTemp176: F64 = 262143.0 * (1.0 - fTemp175);
                let mut iTemp177: i32 = (fTemp176) as i32;
                let mut iTemp178: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp177, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp179: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp178, 7)) as usize] };
                let mut fTemp180: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp178 as usize] };
                let mut fTemp181: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp178, 1)) as usize] }
                        - fTemp180;
                let mut fTemp182: F64 = 131071.5 * fTemp174;
                let mut iTemp183: i32 = (fTemp182) as i32;
                let mut iTemp184: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp183, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp185: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp184, 7)) as usize] };
                let mut fTemp186: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp184 as usize] };
                let mut fTemp187: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp184, 1)) as usize] }
                        - fTemp186;
                let mut fTemp188: F64 = (if iTemp65 != 0 {
                    fTemp186
                        + fTemp78 * fTemp187
                        + (fTemp182 - (iTemp183) as F64)
                            * (fTemp185
                                - (fTemp186
                                    + fTemp78
                                        * (fTemp187
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp184, 8)) as usize]
                                            } - fTemp185))))
                } else {
                    1.0 - (fTemp180
                        + fTemp78 * fTemp181
                        + (fTemp176 - (iTemp177) as F64)
                            * (fTemp179
                                - (fTemp180
                                    + fTemp78
                                        * (fTemp181
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp178, 8)) as usize]
                                            } - fTemp179)))))
                });
                let mut fTemp189: F64 = fTemp83 + fTemp175;
                let mut fTemp190: F64 = 262143.0 * (1.0 - fTemp189);
                let mut iTemp191: i32 = (fTemp190) as i32;
                let mut iTemp192: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp191, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp193: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp192, 7)) as usize] };
                let mut fTemp194: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp192 as usize] };
                let mut fTemp195: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp192, 1)) as usize] }
                        - fTemp194;
                let mut fTemp196: F64 = 262143.0 * fTemp189;
                let mut iTemp197: i32 = (fTemp196) as i32;
                let mut iTemp198: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp197, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp199: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp198, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp200: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp198 as usize] };
                let mut fTemp201: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp198, 1), 1835007),
                    )) as usize]
                } - fTemp200;
                let mut iTemp202: i32 = (fTemp139
                    > (((if iTemp65 != 0 {
                        fTemp200
                            + fTemp78 * fTemp201
                            + (fTemp196 - (iTemp197) as F64)
                                * (fTemp199
                                    - (fTemp200
                                        + fTemp78
                                            * (fTemp201
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp198, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp199))))
                    } else {
                        1.0 - (fTemp194
                            + fTemp78 * fTemp195
                            + (fTemp190 - (iTemp191) as F64)
                                * (fTemp193
                                    - (fTemp194
                                        + fTemp78
                                            * (fTemp195
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp192, 8)) as usize]
                                                } - fTemp193)))))
                    }) - fTemp188)
                        / (1.0 - fTemp188))) as i32;
                let mut fTemp203: F64 = (if iTemp202 != 0 { fTemp172 } else { fTemp175 });
                let mut fTemp204: F64 = (if iTemp202 != 0 { fTemp175 } else { fTemp173 });
                let mut fTemp205: F64 = fTemp204 + fTemp203;
                let mut fTemp206: F64 = 0.5 * fTemp205;
                let mut fTemp207: F64 = 262143.0 * (1.0 - fTemp206);
                let mut iTemp208: i32 = (fTemp207) as i32;
                let mut iTemp209: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp208, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp210: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp209, 7)) as usize] };
                let mut fTemp211: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp209 as usize] };
                let mut fTemp212: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp209, 1)) as usize] }
                        - fTemp211;
                let mut fTemp213: F64 = 131071.5 * fTemp205;
                let mut iTemp214: i32 = (fTemp213) as i32;
                let mut iTemp215: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp214, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp216: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp215, 7)) as usize] };
                let mut fTemp217: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp215 as usize] };
                let mut fTemp218: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp215, 1)) as usize] }
                        - fTemp217;
                let mut fTemp219: F64 = (if iTemp65 != 0 {
                    fTemp217
                        + fTemp78 * fTemp218
                        + (fTemp213 - (iTemp214) as F64)
                            * (fTemp216
                                - (fTemp217
                                    + fTemp78
                                        * (fTemp218
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp215, 8)) as usize]
                                            } - fTemp216))))
                } else {
                    1.0 - (fTemp211
                        + fTemp78 * fTemp212
                        + (fTemp207 - (iTemp208) as F64)
                            * (fTemp210
                                - (fTemp211
                                    + fTemp78
                                        * (fTemp212
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp209, 8)) as usize]
                                            } - fTemp210)))))
                });
                let mut fTemp220: F64 = fTemp83 + fTemp206;
                let mut fTemp221: F64 = 262143.0 * (1.0 - fTemp220);
                let mut iTemp222: i32 = (fTemp221) as i32;
                let mut iTemp223: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp222, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp224: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp223, 7)) as usize] };
                let mut fTemp225: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp223 as usize] };
                let mut fTemp226: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp223, 1)) as usize] }
                        - fTemp225;
                let mut fTemp227: F64 = 262143.0 * fTemp220;
                let mut iTemp228: i32 = (fTemp227) as i32;
                let mut iTemp229: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp228, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp230: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp229, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp231: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp229 as usize] };
                let mut fTemp232: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp229, 1), 1835007),
                    )) as usize]
                } - fTemp231;
                let mut iTemp233: i32 = (fTemp139
                    > (((if iTemp65 != 0 {
                        fTemp231
                            + fTemp78 * fTemp232
                            + (fTemp227 - (iTemp228) as F64)
                                * (fTemp230
                                    - (fTemp231
                                        + fTemp78
                                            * (fTemp232
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp229, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp230))))
                    } else {
                        1.0 - (fTemp225
                            + fTemp78 * fTemp226
                            + (fTemp221 - (iTemp222) as F64)
                                * (fTemp224
                                    - (fTemp225
                                        + fTemp78
                                            * (fTemp226
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp223, 8)) as usize]
                                                } - fTemp224)))))
                    }) - fTemp219)
                        / (1.0 - fTemp219))) as i32;
                let mut fTemp234: F64 = (if iTemp233 != 0 { fTemp203 } else { fTemp206 });
                let mut fTemp235: F64 = (if iTemp233 != 0 { fTemp206 } else { fTemp204 });
                let mut fTemp236: F64 = fTemp235 + fTemp234;
                let mut fTemp237: F64 = 0.5 * fTemp236;
                let mut fTemp238: F64 = 262143.0 * (1.0 - fTemp237);
                let mut iTemp239: i32 = (fTemp238) as i32;
                let mut iTemp240: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp239, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp241: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp240, 7)) as usize] };
                let mut fTemp242: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp240 as usize] };
                let mut fTemp243: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp240, 1)) as usize] }
                        - fTemp242;
                let mut fTemp244: F64 = 131071.5 * fTemp236;
                let mut iTemp245: i32 = (fTemp244) as i32;
                let mut iTemp246: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp245, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp247: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp246, 7)) as usize] };
                let mut fTemp248: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp246 as usize] };
                let mut fTemp249: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp246, 1)) as usize] }
                        - fTemp248;
                let mut fTemp250: F64 = (if iTemp65 != 0 {
                    fTemp248
                        + fTemp78 * fTemp249
                        + (fTemp244 - (iTemp245) as F64)
                            * (fTemp247
                                - (fTemp248
                                    + fTemp78
                                        * (fTemp249
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp246, 8)) as usize]
                                            } - fTemp247))))
                } else {
                    1.0 - (fTemp242
                        + fTemp78 * fTemp243
                        + (fTemp238 - (iTemp239) as F64)
                            * (fTemp241
                                - (fTemp242
                                    + fTemp78
                                        * (fTemp243
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp240, 8)) as usize]
                                            } - fTemp241)))))
                });
                let mut fTemp251: F64 = fTemp83 + fTemp237;
                let mut fTemp252: F64 = 262143.0 * (1.0 - fTemp251);
                let mut iTemp253: i32 = (fTemp252) as i32;
                let mut iTemp254: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp253, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp255: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp254, 7)) as usize] };
                let mut fTemp256: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp254 as usize] };
                let mut fTemp257: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp254, 1)) as usize] }
                        - fTemp256;
                let mut fTemp258: F64 = 262143.0 * fTemp251;
                let mut iTemp259: i32 = (fTemp258) as i32;
                let mut iTemp260: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp259, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp261: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp260, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp262: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp260 as usize] };
                let mut fTemp263: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp260, 1), 1835007),
                    )) as usize]
                } - fTemp262;
                let mut iTemp264: i32 = (fTemp139
                    > (((if iTemp65 != 0 {
                        fTemp262
                            + fTemp78 * fTemp263
                            + (fTemp258 - (iTemp259) as F64)
                                * (fTemp261
                                    - (fTemp262
                                        + fTemp78
                                            * (fTemp263
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp260, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp261))))
                    } else {
                        1.0 - (fTemp256
                            + fTemp78 * fTemp257
                            + (fTemp252 - (iTemp253) as F64)
                                * (fTemp255
                                    - (fTemp256
                                        + fTemp78
                                            * (fTemp257
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp254, 8)) as usize]
                                                } - fTemp255)))))
                    }) - fTemp250)
                        / (1.0 - fTemp250))) as i32;
                let mut fTemp265: F64 = (if iTemp264 != 0 { fTemp234 } else { fTemp237 });
                let mut fTemp266: F64 = (if iTemp264 != 0 { fTemp237 } else { fTemp235 });
                let mut fTemp267: F64 = fTemp266 + fTemp265;
                let mut fTemp268: F64 = 0.5 * fTemp267;
                let mut fTemp269: F64 = 262143.0 * (1.0 - fTemp268);
                let mut iTemp270: i32 = (fTemp269) as i32;
                let mut iTemp271: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp270, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp272: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp271, 7)) as usize] };
                let mut fTemp273: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp271 as usize] };
                let mut fTemp274: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp271, 1)) as usize] }
                        - fTemp273;
                let mut fTemp275: F64 = 131071.5 * fTemp267;
                let mut iTemp276: i32 = (fTemp275) as i32;
                let mut iTemp277: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp276, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp278: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp277, 7)) as usize] };
                let mut fTemp279: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp277 as usize] };
                let mut fTemp280: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp277, 1)) as usize] }
                        - fTemp279;
                let mut fTemp281: F64 = (if iTemp65 != 0 {
                    fTemp279
                        + fTemp78 * fTemp280
                        + (fTemp275 - (iTemp276) as F64)
                            * (fTemp278
                                - (fTemp279
                                    + fTemp78
                                        * (fTemp280
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp277, 8)) as usize]
                                            } - fTemp278))))
                } else {
                    1.0 - (fTemp273
                        + fTemp78 * fTemp274
                        + (fTemp269 - (iTemp270) as F64)
                            * (fTemp272
                                - (fTemp273
                                    + fTemp78
                                        * (fTemp274
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp271, 8)) as usize]
                                            } - fTemp272)))))
                });
                let mut fTemp282: F64 = fTemp83 + fTemp268;
                let mut fTemp283: F64 = 262143.0 * (1.0 - fTemp282);
                let mut iTemp284: i32 = (fTemp283) as i32;
                let mut iTemp285: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp284, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp286: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp285, 7)) as usize] };
                let mut fTemp287: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp285 as usize] };
                let mut fTemp288: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp285, 1)) as usize] }
                        - fTemp287;
                let mut fTemp289: F64 = 262143.0 * fTemp282;
                let mut iTemp290: i32 = (fTemp289) as i32;
                let mut iTemp291: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp290, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp292: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp291, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp293: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp291 as usize] };
                let mut fTemp294: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp291, 1), 1835007),
                    )) as usize]
                } - fTemp293;
                let mut iTemp295: i32 = (fTemp139
                    > (((if iTemp65 != 0 {
                        fTemp293
                            + fTemp78 * fTemp294
                            + (fTemp289 - (iTemp290) as F64)
                                * (fTemp292
                                    - (fTemp293
                                        + fTemp78
                                            * (fTemp294
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp291, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp292))))
                    } else {
                        1.0 - (fTemp287
                            + fTemp78 * fTemp288
                            + (fTemp283 - (iTemp284) as F64)
                                * (fTemp286
                                    - (fTemp287
                                        + fTemp78
                                            * (fTemp288
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp285, 8)) as usize]
                                                } - fTemp286)))))
                    }) - fTemp281)
                        / (1.0 - fTemp281))) as i32;
                let mut fTemp296: F64 = (if iTemp295 != 0 { fTemp265 } else { fTemp268 });
                let mut fTemp297: F64 = (if iTemp295 != 0 { fTemp268 } else { fTemp266 });
                let mut fTemp298: F64 = fTemp297 + fTemp296;
                let mut fTemp299: F64 = 0.5 * fTemp298;
                let mut fTemp300: F64 = 262143.0 * (1.0 - fTemp299);
                let mut iTemp301: i32 = (fTemp300) as i32;
                let mut iTemp302: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp301, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp303: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp302, 7)) as usize] };
                let mut fTemp304: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp302 as usize] };
                let mut fTemp305: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp302, 1)) as usize] }
                        - fTemp304;
                let mut fTemp306: F64 = 131071.5 * fTemp298;
                let mut iTemp307: i32 = (fTemp306) as i32;
                let mut iTemp308: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp307, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp309: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp308, 7)) as usize] };
                let mut fTemp310: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp308 as usize] };
                let mut fTemp311: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp308, 1)) as usize] }
                        - fTemp310;
                let mut fTemp312: F64 = (if iTemp65 != 0 {
                    fTemp310
                        + fTemp78 * fTemp311
                        + (fTemp306 - (iTemp307) as F64)
                            * (fTemp309
                                - (fTemp310
                                    + fTemp78
                                        * (fTemp311
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp308, 8)) as usize]
                                            } - fTemp309))))
                } else {
                    1.0 - (fTemp304
                        + fTemp78 * fTemp305
                        + (fTemp300 - (iTemp301) as F64)
                            * (fTemp303
                                - (fTemp304
                                    + fTemp78
                                        * (fTemp305
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp302, 8)) as usize]
                                            } - fTemp303)))))
                });
                let mut fTemp313: F64 = fTemp83 + fTemp299;
                let mut fTemp314: F64 = 262143.0 * (1.0 - fTemp313);
                let mut iTemp315: i32 = (fTemp314) as i32;
                let mut iTemp316: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp315, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp317: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp316, 7)) as usize] };
                let mut fTemp318: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp316 as usize] };
                let mut fTemp319: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp316, 1)) as usize] }
                        - fTemp318;
                let mut fTemp320: F64 = 262143.0 * fTemp313;
                let mut iTemp321: i32 = (fTemp320) as i32;
                let mut iTemp322: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp321, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp323: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp322, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp324: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp322 as usize] };
                let mut fTemp325: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp322, 1), 1835007),
                    )) as usize]
                } - fTemp324;
                let mut iTemp326: i32 = (fTemp139
                    > (((if iTemp65 != 0 {
                        fTemp324
                            + fTemp78 * fTemp325
                            + (fTemp320 - (iTemp321) as F64)
                                * (fTemp323
                                    - (fTemp324
                                        + fTemp78
                                            * (fTemp325
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp322, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp323))))
                    } else {
                        1.0 - (fTemp318
                            + fTemp78 * fTemp319
                            + (fTemp314 - (iTemp315) as F64)
                                * (fTemp317
                                    - (fTemp318
                                        + fTemp78
                                            * (fTemp319
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp316, 8)) as usize]
                                                } - fTemp317)))))
                    }) - fTemp312)
                        / (1.0 - fTemp312))) as i32;
                let mut fTemp327: F64 = (if iTemp326 != 0 { fTemp296 } else { fTemp299 });
                let mut fTemp328: F64 = (if iTemp326 != 0 { fTemp299 } else { fTemp297 });
                let mut fTemp329: F64 = fTemp328 + fTemp327;
                let mut fTemp330: F64 = 0.5 * fTemp329;
                let mut fTemp331: F64 = 262143.0 * (1.0 - fTemp330);
                let mut iTemp332: i32 = (fTemp331) as i32;
                let mut iTemp333: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp332, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp334: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp333, 7)) as usize] };
                let mut fTemp335: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp333 as usize] };
                let mut fTemp336: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp333, 1)) as usize] }
                        - fTemp335;
                let mut fTemp337: F64 = 131071.5 * fTemp329;
                let mut iTemp338: i32 = (fTemp337) as i32;
                let mut iTemp339: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp338, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp340: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp339, 7)) as usize] };
                let mut fTemp341: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp339 as usize] };
                let mut fTemp342: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp339, 1)) as usize] }
                        - fTemp341;
                let mut fTemp343: F64 = (if iTemp65 != 0 {
                    fTemp341
                        + fTemp78 * fTemp342
                        + (fTemp337 - (iTemp338) as F64)
                            * (fTemp340
                                - (fTemp341
                                    + fTemp78
                                        * (fTemp342
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp339, 8)) as usize]
                                            } - fTemp340))))
                } else {
                    1.0 - (fTemp335
                        + fTemp78 * fTemp336
                        + (fTemp331 - (iTemp332) as F64)
                            * (fTemp334
                                - (fTemp335
                                    + fTemp78
                                        * (fTemp336
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp333, 8)) as usize]
                                            } - fTemp334)))))
                });
                let mut fTemp344: F64 = fTemp83 + fTemp330;
                let mut fTemp345: F64 = 262143.0 * (1.0 - fTemp344);
                let mut iTemp346: i32 = (fTemp345) as i32;
                let mut iTemp347: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp346, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp348: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp347, 7)) as usize] };
                let mut fTemp349: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp347 as usize] };
                let mut fTemp350: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp347, 1)) as usize] }
                        - fTemp349;
                let mut fTemp351: F64 = 262143.0 * fTemp344;
                let mut iTemp352: i32 = (fTemp351) as i32;
                let mut iTemp353: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp352, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp354: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp353, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp355: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp353 as usize] };
                let mut fTemp356: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp353, 1), 1835007),
                    )) as usize]
                } - fTemp355;
                let mut iTemp357: i32 = (fTemp139
                    > (((if iTemp65 != 0 {
                        fTemp355
                            + fTemp78 * fTemp356
                            + (fTemp351 - (iTemp352) as F64)
                                * (fTemp354
                                    - (fTemp355
                                        + fTemp78
                                            * (fTemp356
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp353, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp354))))
                    } else {
                        1.0 - (fTemp349
                            + fTemp78 * fTemp350
                            + (fTemp345 - (iTemp346) as F64)
                                * (fTemp348
                                    - (fTemp349
                                        + fTemp78
                                            * (fTemp350
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp347, 8)) as usize]
                                                } - fTemp348)))))
                    }) - fTemp343)
                        / (1.0 - fTemp343))) as i32;
                let mut fTemp358: F64 = (if iTemp357 != 0 { fTemp327 } else { fTemp330 });
                let mut fTemp359: F64 = (if iTemp357 != 0 { fTemp330 } else { fTemp328 });
                let mut fTemp360: F64 = fTemp359 + fTemp358;
                let mut fTemp361: F64 = 0.5 * fTemp360;
                let mut fTemp362: F64 = 262143.0 * (1.0 - fTemp361);
                let mut iTemp363: i32 = (fTemp362) as i32;
                let mut iTemp364: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp363, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp365: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp364, 7)) as usize] };
                let mut fTemp366: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp364 as usize] };
                let mut fTemp367: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp364, 1)) as usize] }
                        - fTemp366;
                let mut fTemp368: F64 = 131071.5 * fTemp360;
                let mut iTemp369: i32 = (fTemp368) as i32;
                let mut iTemp370: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp369, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp371: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp370, 7)) as usize] };
                let mut fTemp372: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp370 as usize] };
                let mut fTemp373: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp370, 1)) as usize] }
                        - fTemp372;
                let mut fTemp374: F64 = (if iTemp65 != 0 {
                    fTemp372
                        + fTemp78 * fTemp373
                        + (fTemp368 - (iTemp369) as F64)
                            * (fTemp371
                                - (fTemp372
                                    + fTemp78
                                        * (fTemp373
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp370, 8)) as usize]
                                            } - fTemp371))))
                } else {
                    1.0 - (fTemp366
                        + fTemp78 * fTemp367
                        + (fTemp362 - (iTemp363) as F64)
                            * (fTemp365
                                - (fTemp366
                                    + fTemp78
                                        * (fTemp367
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp364, 8)) as usize]
                                            } - fTemp365)))))
                });
                let mut fTemp375: F64 = fTemp83 + fTemp361;
                let mut fTemp376: F64 = 262143.0 * (1.0 - fTemp375);
                let mut iTemp377: i32 = (fTemp376) as i32;
                let mut iTemp378: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp377, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp379: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp378, 7)) as usize] };
                let mut fTemp380: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp378 as usize] };
                let mut fTemp381: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp378, 1)) as usize] }
                        - fTemp380;
                let mut fTemp382: F64 = 262143.0 * fTemp375;
                let mut iTemp383: i32 = (fTemp382) as i32;
                let mut iTemp384: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp383, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp385: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp384, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp386: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp384 as usize] };
                let mut fTemp387: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp384, 1), 1835007),
                    )) as usize]
                } - fTemp386;
                let mut iTemp388: i32 = (fTemp139
                    > (((if iTemp65 != 0 {
                        fTemp386
                            + fTemp78 * fTemp387
                            + (fTemp382 - (iTemp383) as F64)
                                * (fTemp385
                                    - (fTemp386
                                        + fTemp78
                                            * (fTemp387
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp384, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp385))))
                    } else {
                        1.0 - (fTemp380
                            + fTemp78 * fTemp381
                            + (fTemp376 - (iTemp377) as F64)
                                * (fTemp379
                                    - (fTemp380
                                        + fTemp78
                                            * (fTemp381
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp378, 8)) as usize]
                                                } - fTemp379)))))
                    }) - fTemp374)
                        / (1.0 - fTemp374))) as i32;
                let mut fTemp389: F64 = (if iTemp388 != 0 { fTemp358 } else { fTemp361 });
                let mut fTemp390: F64 = (if iTemp388 != 0 { fTemp361 } else { fTemp359 });
                let mut fTemp391: F64 = fTemp390 + fTemp389;
                let mut fTemp392: F64 = 0.5 * fTemp391;
                let mut fTemp393: F64 = 262143.0 * (1.0 - fTemp392);
                let mut iTemp394: i32 = (fTemp393) as i32;
                let mut iTemp395: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp394, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp396: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp395, 7)) as usize] };
                let mut fTemp397: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp395 as usize] };
                let mut fTemp398: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp395, 1)) as usize] }
                        - fTemp397;
                let mut fTemp399: F64 = 131071.5 * fTemp391;
                let mut iTemp400: i32 = (fTemp399) as i32;
                let mut iTemp401: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp400, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp402: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp401, 7)) as usize] };
                let mut fTemp403: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp401 as usize] };
                let mut fTemp404: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp401, 1)) as usize] }
                        - fTemp403;
                let mut fTemp405: F64 = (if iTemp65 != 0 {
                    fTemp403
                        + fTemp78 * fTemp404
                        + (fTemp399 - (iTemp400) as F64)
                            * (fTemp402
                                - (fTemp403
                                    + fTemp78
                                        * (fTemp404
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp401, 8)) as usize]
                                            } - fTemp402))))
                } else {
                    1.0 - (fTemp397
                        + fTemp78 * fTemp398
                        + (fTemp393 - (iTemp394) as F64)
                            * (fTemp396
                                - (fTemp397
                                    + fTemp78
                                        * (fTemp398
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp395, 8)) as usize]
                                            } - fTemp396)))))
                });
                let mut fTemp406: F64 = fTemp83 + fTemp392;
                let mut fTemp407: F64 = 262143.0 * (1.0 - fTemp406);
                let mut iTemp408: i32 = (fTemp407) as i32;
                let mut iTemp409: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp408, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp410: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp409, 7)) as usize] };
                let mut fTemp411: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp409 as usize] };
                let mut fTemp412: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp409, 1)) as usize] }
                        - fTemp411;
                let mut fTemp413: F64 = 262143.0 * fTemp406;
                let mut iTemp414: i32 = (fTemp413) as i32;
                let mut iTemp415: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp414, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp416: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp415, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp417: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp415 as usize] };
                let mut fTemp418: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp415, 1), 1835007),
                    )) as usize]
                } - fTemp417;
                let mut iTemp419: i32 = (fTemp139
                    > (((if iTemp65 != 0 {
                        fTemp417
                            + fTemp78 * fTemp418
                            + (fTemp413 - (iTemp414) as F64)
                                * (fTemp416
                                    - (fTemp417
                                        + fTemp78
                                            * (fTemp418
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp415, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp416))))
                    } else {
                        1.0 - (fTemp411
                            + fTemp78 * fTemp412
                            + (fTemp407 - (iTemp408) as F64)
                                * (fTemp410
                                    - (fTemp411
                                        + fTemp78
                                            * (fTemp412
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp409, 8)) as usize]
                                                } - fTemp410)))))
                    }) - fTemp405)
                        / (1.0 - fTemp405))) as i32;
                let mut fTemp420: F64 = (if iTemp419 != 0 { fTemp389 } else { fTemp392 });
                let mut fTemp421: F64 = (if iTemp419 != 0 { fTemp392 } else { fTemp390 });
                let mut fTemp422: F64 = fTemp421 + fTemp420;
                let mut fTemp423: F64 = 0.5 * fTemp422;
                let mut fTemp424: F64 = 262143.0 * (1.0 - fTemp423);
                let mut iTemp425: i32 = (fTemp424) as i32;
                let mut iTemp426: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp425, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp427: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp426, 7)) as usize] };
                let mut fTemp428: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp426 as usize] };
                let mut fTemp429: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp426, 1)) as usize] }
                        - fTemp428;
                let mut fTemp430: F64 = 131071.5 * fTemp422;
                let mut iTemp431: i32 = (fTemp430) as i32;
                let mut iTemp432: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp431, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp433: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp432, 7)) as usize] };
                let mut fTemp434: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp432 as usize] };
                let mut fTemp435: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp432, 1)) as usize] }
                        - fTemp434;
                let mut fTemp436: F64 = (if iTemp65 != 0 {
                    fTemp434
                        + fTemp78 * fTemp435
                        + (fTemp430 - (iTemp431) as F64)
                            * (fTemp433
                                - (fTemp434
                                    + fTemp78
                                        * (fTemp435
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp432, 8)) as usize]
                                            } - fTemp433))))
                } else {
                    1.0 - (fTemp428
                        + fTemp78 * fTemp429
                        + (fTemp424 - (iTemp425) as F64)
                            * (fTemp427
                                - (fTemp428
                                    + fTemp78
                                        * (fTemp429
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp426, 8)) as usize]
                                            } - fTemp427)))))
                });
                let mut fTemp437: F64 = fTemp83 + fTemp423;
                let mut fTemp438: F64 = 262143.0 * (1.0 - fTemp437);
                let mut iTemp439: i32 = (fTemp438) as i32;
                let mut iTemp440: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp439, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp441: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp440, 7)) as usize] };
                let mut fTemp442: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp440 as usize] };
                let mut fTemp443: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp440, 1)) as usize] }
                        - fTemp442;
                let mut fTemp444: F64 = 262143.0 * fTemp437;
                let mut iTemp445: i32 = (fTemp444) as i32;
                let mut iTemp446: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp445, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp447: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp446, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp448: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp446 as usize] };
                let mut fTemp449: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp446, 1), 1835007),
                    )) as usize]
                } - fTemp448;
                let mut iTemp450: i32 = (fTemp139
                    > (((if iTemp65 != 0 {
                        fTemp448
                            + fTemp78 * fTemp449
                            + (fTemp444 - (iTemp445) as F64)
                                * (fTemp447
                                    - (fTemp448
                                        + fTemp78
                                            * (fTemp449
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp446, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp447))))
                    } else {
                        1.0 - (fTemp442
                            + fTemp78 * fTemp443
                            + (fTemp438 - (iTemp439) as F64)
                                * (fTemp441
                                    - (fTemp442
                                        + fTemp78
                                            * (fTemp443
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp440, 8)) as usize]
                                                } - fTemp441)))))
                    }) - fTemp436)
                        / (1.0 - fTemp436))) as i32;
                let mut fTemp451: F64 = (if iTemp450 != 0 { fTemp420 } else { fTemp423 });
                let mut fTemp452: F64 = (if iTemp450 != 0 { fTemp423 } else { fTemp421 });
                let mut fTemp453: F64 = fTemp452 + fTemp451;
                let mut fTemp454: F64 = 0.5 * fTemp453;
                let mut fTemp455: F64 = 262143.0 * (1.0 - fTemp454);
                let mut iTemp456: i32 = (fTemp455) as i32;
                let mut iTemp457: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp456, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp458: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp457, 7)) as usize] };
                let mut fTemp459: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp457 as usize] };
                let mut fTemp460: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp457, 1)) as usize] }
                        - fTemp459;
                let mut fTemp461: F64 = 131071.5 * fTemp453;
                let mut iTemp462: i32 = (fTemp461) as i32;
                let mut iTemp463: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp462, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp464: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp463, 7)) as usize] };
                let mut fTemp465: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp463 as usize] };
                let mut fTemp466: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp463, 1)) as usize] }
                        - fTemp465;
                let mut fTemp467: F64 = (if iTemp65 != 0 {
                    fTemp465
                        + fTemp78 * fTemp466
                        + (fTemp461 - (iTemp462) as F64)
                            * (fTemp464
                                - (fTemp465
                                    + fTemp78
                                        * (fTemp466
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp463, 8)) as usize]
                                            } - fTemp464))))
                } else {
                    1.0 - (fTemp459
                        + fTemp78 * fTemp460
                        + (fTemp455 - (iTemp456) as F64)
                            * (fTemp458
                                - (fTemp459
                                    + fTemp78
                                        * (fTemp460
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp457, 8)) as usize]
                                            } - fTemp458)))))
                });
                let mut fTemp468: F64 = fTemp83 + fTemp454;
                let mut fTemp469: F64 = 262143.0 * (1.0 - fTemp468);
                let mut iTemp470: i32 = (fTemp469) as i32;
                let mut iTemp471: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp470, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp472: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp471, 7)) as usize] };
                let mut fTemp473: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp471 as usize] };
                let mut fTemp474: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp471, 1)) as usize] }
                        - fTemp473;
                let mut fTemp475: F64 = 262143.0 * fTemp468;
                let mut iTemp476: i32 = (fTemp475) as i32;
                let mut iTemp477: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp476, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp478: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp477, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp479: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp477 as usize] };
                let mut fTemp480: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp477, 1), 1835007),
                    )) as usize]
                } - fTemp479;
                let mut iTemp481: i32 = (fTemp139
                    > (((if iTemp65 != 0 {
                        fTemp479
                            + fTemp78 * fTemp480
                            + (fTemp475 - (iTemp476) as F64)
                                * (fTemp478
                                    - (fTemp479
                                        + fTemp78
                                            * (fTemp480
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp477, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp478))))
                    } else {
                        1.0 - (fTemp473
                            + fTemp78 * fTemp474
                            + (fTemp469 - (iTemp470) as F64)
                                * (fTemp472
                                    - (fTemp473
                                        + fTemp78
                                            * (fTemp474
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp471, 8)) as usize]
                                                } - fTemp472)))))
                    }) - fTemp467)
                        / (1.0 - fTemp467))) as i32;
                let mut fTemp482: F64 = (if iTemp481 != 0 { fTemp451 } else { fTemp454 });
                let mut fTemp483: F64 = (if iTemp481 != 0 { fTemp454 } else { fTemp452 });
                let mut fTemp484: F64 = fTemp483 + fTemp482;
                let mut fTemp485: F64 = 0.5 * fTemp484;
                let mut fTemp486: F64 = 262143.0 * (1.0 - fTemp485);
                let mut iTemp487: i32 = (fTemp486) as i32;
                let mut iTemp488: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp487, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp489: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp488, 7)) as usize] };
                let mut fTemp490: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp488 as usize] };
                let mut fTemp491: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp488, 1)) as usize] }
                        - fTemp490;
                let mut fTemp492: F64 = 131071.5 * fTemp484;
                let mut iTemp493: i32 = (fTemp492) as i32;
                let mut iTemp494: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp493, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp495: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp494, 7)) as usize] };
                let mut fTemp496: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp494 as usize] };
                let mut fTemp497: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp494, 1)) as usize] }
                        - fTemp496;
                let mut fTemp498: F64 = (if iTemp65 != 0 {
                    fTemp496
                        + fTemp78 * fTemp497
                        + (fTemp492 - (iTemp493) as F64)
                            * (fTemp495
                                - (fTemp496
                                    + fTemp78
                                        * (fTemp497
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp494, 8)) as usize]
                                            } - fTemp495))))
                } else {
                    1.0 - (fTemp490
                        + fTemp78 * fTemp491
                        + (fTemp486 - (iTemp487) as F64)
                            * (fTemp489
                                - (fTemp490
                                    + fTemp78
                                        * (fTemp491
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp488, 8)) as usize]
                                            } - fTemp489)))))
                });
                let mut fTemp499: F64 = fTemp83 + fTemp485;
                let mut fTemp500: F64 = 262143.0 * (1.0 - fTemp499);
                let mut iTemp501: i32 = (fTemp500) as i32;
                let mut iTemp502: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp501, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp503: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp502, 7)) as usize] };
                let mut fTemp504: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp502 as usize] };
                let mut fTemp505: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp502, 1)) as usize] }
                        - fTemp504;
                let mut fTemp506: F64 = 262143.0 * fTemp499;
                let mut iTemp507: i32 = (fTemp506) as i32;
                let mut iTemp508: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp507, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp509: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp508, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp510: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp508 as usize] };
                let mut fTemp511: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp508, 1), 1835007),
                    )) as usize]
                } - fTemp510;
                let mut iTemp512: i32 = (fTemp139
                    > (((if iTemp65 != 0 {
                        fTemp510
                            + fTemp78 * fTemp511
                            + (fTemp506 - (iTemp507) as F64)
                                * (fTemp509
                                    - (fTemp510
                                        + fTemp78
                                            * (fTemp511
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp508, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp509))))
                    } else {
                        1.0 - (fTemp504
                            + fTemp78 * fTemp505
                            + (fTemp500 - (iTemp501) as F64)
                                * (fTemp503
                                    - (fTemp504
                                        + fTemp78
                                            * (fTemp505
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp502, 8)) as usize]
                                                } - fTemp503)))))
                    }) - fTemp498)
                        / (1.0 - fTemp498))) as i32;
                let mut fTemp513: F64 = (if iTemp512 != 0 { fTemp482 } else { fTemp485 });
                let mut fTemp514: F64 = (if iTemp512 != 0 { fTemp485 } else { fTemp483 });
                let mut fTemp515: F64 = fTemp514 + fTemp513;
                let mut fTemp516: F64 = 0.5 * fTemp515;
                let mut fTemp517: F64 = 262143.0 * (1.0 - fTemp516);
                let mut iTemp518: i32 = (fTemp517) as i32;
                let mut iTemp519: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp518, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp520: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp519, 7)) as usize] };
                let mut fTemp521: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp519 as usize] };
                let mut fTemp522: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp519, 1)) as usize] }
                        - fTemp521;
                let mut fTemp523: F64 = 131071.5 * fTemp515;
                let mut iTemp524: i32 = (fTemp523) as i32;
                let mut iTemp525: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp524, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp526: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp525, 7)) as usize] };
                let mut fTemp527: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp525 as usize] };
                let mut fTemp528: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp525, 1)) as usize] }
                        - fTemp527;
                let mut fTemp529: F64 = (if iTemp65 != 0 {
                    fTemp527
                        + fTemp78 * fTemp528
                        + (fTemp523 - (iTemp524) as F64)
                            * (fTemp526
                                - (fTemp527
                                    + fTemp78
                                        * (fTemp528
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp525, 8)) as usize]
                                            } - fTemp526))))
                } else {
                    1.0 - (fTemp521
                        + fTemp78 * fTemp522
                        + (fTemp517 - (iTemp518) as F64)
                            * (fTemp520
                                - (fTemp521
                                    + fTemp78
                                        * (fTemp522
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp519, 8)) as usize]
                                            } - fTemp520)))))
                });
                let mut fTemp530: F64 = fTemp83 + fTemp516;
                let mut fTemp531: F64 = 262143.0 * (1.0 - fTemp530);
                let mut iTemp532: i32 = (fTemp531) as i32;
                let mut iTemp533: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp532, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp534: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp533, 7)) as usize] };
                let mut fTemp535: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp533 as usize] };
                let mut fTemp536: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp533, 1)) as usize] }
                        - fTemp535;
                let mut fTemp537: F64 = 262143.0 * fTemp530;
                let mut iTemp538: i32 = (fTemp537) as i32;
                let mut iTemp539: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp538, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp540: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp539, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp541: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp539 as usize] };
                let mut fTemp542: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp539, 1), 1835007),
                    )) as usize]
                } - fTemp541;
                let mut iTemp543: i32 = (fTemp139
                    > (((if iTemp65 != 0 {
                        fTemp541
                            + fTemp78 * fTemp542
                            + (fTemp537 - (iTemp538) as F64)
                                * (fTemp540
                                    - (fTemp541
                                        + fTemp78
                                            * (fTemp542
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp539, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp540))))
                    } else {
                        1.0 - (fTemp535
                            + fTemp78 * fTemp536
                            + (fTemp531 - (iTemp532) as F64)
                                * (fTemp534
                                    - (fTemp535
                                        + fTemp78
                                            * (fTemp536
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp533, 8)) as usize]
                                                } - fTemp534)))))
                    }) - fTemp529)
                        / (1.0 - fTemp529))) as i32;
                let mut fTemp544: F64 = (if iTemp543 != 0 { fTemp513 } else { fTemp516 });
                let mut fTemp545: F64 = (if iTemp543 != 0 { fTemp516 } else { fTemp514 });
                let mut fTemp546: F64 = fTemp545 + fTemp544;
                let mut fTemp547: F64 = 0.5 * fTemp546;
                let mut fTemp548: F64 = 262143.0 * (1.0 - fTemp547);
                let mut iTemp549: i32 = (fTemp548) as i32;
                let mut iTemp550: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp549, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp551: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp550, 7)) as usize] };
                let mut fTemp552: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp550 as usize] };
                let mut fTemp553: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp550, 1)) as usize] }
                        - fTemp552;
                let mut fTemp554: F64 = 131071.5 * fTemp546;
                let mut iTemp555: i32 = (fTemp554) as i32;
                let mut iTemp556: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp555, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp557: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp556, 7)) as usize] };
                let mut fTemp558: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp556 as usize] };
                let mut fTemp559: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp556, 1)) as usize] }
                        - fTemp558;
                let mut fTemp560: F64 = (if iTemp65 != 0 {
                    fTemp558
                        + fTemp78 * fTemp559
                        + (fTemp554 - (iTemp555) as F64)
                            * (fTemp557
                                - (fTemp558
                                    + fTemp78
                                        * (fTemp559
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp556, 8)) as usize]
                                            } - fTemp557))))
                } else {
                    1.0 - (fTemp552
                        + fTemp78 * fTemp553
                        + (fTemp548 - (iTemp549) as F64)
                            * (fTemp551
                                - (fTemp552
                                    + fTemp78
                                        * (fTemp553
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp550, 8)) as usize]
                                            } - fTemp551)))))
                });
                let mut fTemp561: F64 = fTemp83 + fTemp547;
                let mut fTemp562: F64 = 262143.0 * (1.0 - fTemp561);
                let mut iTemp563: i32 = (fTemp562) as i32;
                let mut iTemp564: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp563, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp565: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp564, 7)) as usize] };
                let mut fTemp566: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp564 as usize] };
                let mut fTemp567: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp564, 1)) as usize] }
                        - fTemp566;
                let mut fTemp568: F64 = 262143.0 * fTemp561;
                let mut iTemp569: i32 = (fTemp568) as i32;
                let mut iTemp570: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp569, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp571: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp570, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp572: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp570 as usize] };
                let mut fTemp573: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp570, 1), 1835007),
                    )) as usize]
                } - fTemp572;
                let mut iTemp574: i32 = (fTemp139
                    > (((if iTemp65 != 0 {
                        fTemp572
                            + fTemp78 * fTemp573
                            + (fTemp568 - (iTemp569) as F64)
                                * (fTemp571
                                    - (fTemp572
                                        + fTemp78
                                            * (fTemp573
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp570, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp571))))
                    } else {
                        1.0 - (fTemp566
                            + fTemp78 * fTemp567
                            + (fTemp562 - (iTemp563) as F64)
                                * (fTemp565
                                    - (fTemp566
                                        + fTemp78
                                            * (fTemp567
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp564, 8)) as usize]
                                                } - fTemp565)))))
                    }) - fTemp560)
                        / (1.0 - fTemp560))) as i32;
                let mut fTemp575: F64 = (if iTemp574 != 0 { fTemp544 } else { fTemp547 });
                let mut fTemp576: F64 = (if iTemp574 != 0 { fTemp547 } else { fTemp545 });
                let mut fTemp577: F64 = fTemp576 + fTemp575;
                let mut fTemp578: F64 = 0.5 * fTemp577;
                let mut fTemp579: F64 = 262143.0 * (1.0 - fTemp578);
                let mut iTemp580: i32 = (fTemp579) as i32;
                let mut iTemp581: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp580, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp582: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp581, 7)) as usize] };
                let mut fTemp583: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp581 as usize] };
                let mut fTemp584: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp581, 1)) as usize] }
                        - fTemp583;
                let mut fTemp585: F64 = 131071.5 * fTemp577;
                let mut iTemp586: i32 = (fTemp585) as i32;
                let mut iTemp587: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp586, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp588: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp587, 7)) as usize] };
                let mut fTemp589: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp587 as usize] };
                let mut fTemp590: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp587, 1)) as usize] }
                        - fTemp589;
                let mut fTemp591: F64 = (if iTemp65 != 0 {
                    fTemp589
                        + fTemp78 * fTemp590
                        + (fTemp585 - (iTemp586) as F64)
                            * (fTemp588
                                - (fTemp589
                                    + fTemp78
                                        * (fTemp590
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp587, 8)) as usize]
                                            } - fTemp588))))
                } else {
                    1.0 - (fTemp583
                        + fTemp78 * fTemp584
                        + (fTemp579 - (iTemp580) as F64)
                            * (fTemp582
                                - (fTemp583
                                    + fTemp78
                                        * (fTemp584
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp581, 8)) as usize]
                                            } - fTemp582)))))
                });
                let mut fTemp592: F64 = fTemp83 + fTemp578;
                let mut fTemp593: F64 = 262143.0 * (1.0 - fTemp592);
                let mut iTemp594: i32 = (fTemp593) as i32;
                let mut iTemp595: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp594, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp596: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp595, 7)) as usize] };
                let mut fTemp597: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp595 as usize] };
                let mut fTemp598: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp595, 1)) as usize] }
                        - fTemp597;
                let mut fTemp599: F64 = 262143.0 * fTemp592;
                let mut iTemp600: i32 = (fTemp599) as i32;
                let mut iTemp601: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp600, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp602: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp601, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp603: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp601 as usize] };
                let mut fTemp604: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp601, 1), 1835007),
                    )) as usize]
                } - fTemp603;
                let mut iTemp605: i32 = (fTemp139
                    > (((if iTemp65 != 0 {
                        fTemp603
                            + fTemp78 * fTemp604
                            + (fTemp599 - (iTemp600) as F64)
                                * (fTemp602
                                    - (fTemp603
                                        + fTemp78
                                            * (fTemp604
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp601, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp602))))
                    } else {
                        1.0 - (fTemp597
                            + fTemp78 * fTemp598
                            + (fTemp593 - (iTemp594) as F64)
                                * (fTemp596
                                    - (fTemp597
                                        + fTemp78
                                            * (fTemp598
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp595, 8)) as usize]
                                                } - fTemp596)))))
                    }) - fTemp591)
                        / (1.0 - fTemp591))) as i32;
                let mut fTemp606: F64 = (if iTemp605 != 0 { fTemp575 } else { fTemp578 });
                let mut fTemp607: F64 = (if iTemp605 != 0 { fTemp578 } else { fTemp576 });
                let mut fTemp608: F64 = fTemp607 + fTemp606;
                let mut fTemp609: F64 = 0.5 * fTemp608;
                let mut fTemp610: F64 = 262143.0 * (1.0 - fTemp609);
                let mut iTemp611: i32 = (fTemp610) as i32;
                let mut iTemp612: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp611, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp613: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp612, 7)) as usize] };
                let mut fTemp614: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp612 as usize] };
                let mut fTemp615: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp612, 1)) as usize] }
                        - fTemp614;
                let mut fTemp616: F64 = 131071.5 * fTemp608;
                let mut iTemp617: i32 = (fTemp616) as i32;
                let mut iTemp618: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp617, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp619: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp618, 7)) as usize] };
                let mut fTemp620: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp618 as usize] };
                let mut fTemp621: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp618, 1)) as usize] }
                        - fTemp620;
                let mut fTemp622: F64 = (if iTemp65 != 0 {
                    fTemp620
                        + fTemp78 * fTemp621
                        + (fTemp616 - (iTemp617) as F64)
                            * (fTemp619
                                - (fTemp620
                                    + fTemp78
                                        * (fTemp621
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp618, 8)) as usize]
                                            } - fTemp619))))
                } else {
                    1.0 - (fTemp614
                        + fTemp78 * fTemp615
                        + (fTemp610 - (iTemp611) as F64)
                            * (fTemp613
                                - (fTemp614
                                    + fTemp78
                                        * (fTemp615
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp612, 8)) as usize]
                                            } - fTemp613)))))
                });
                let mut fTemp623: F64 = fTemp83 + fTemp609;
                let mut fTemp624: F64 = 262143.0 * (1.0 - fTemp623);
                let mut iTemp625: i32 = (fTemp624) as i32;
                let mut iTemp626: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp625, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp627: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp626, 7)) as usize] };
                let mut fTemp628: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp626 as usize] };
                let mut fTemp629: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp626, 1)) as usize] }
                        - fTemp628;
                let mut fTemp630: F64 = 262143.0 * fTemp623;
                let mut iTemp631: i32 = (fTemp630) as i32;
                let mut iTemp632: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp631, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp633: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp632, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp634: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp632 as usize] };
                let mut fTemp635: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp632, 1), 1835007),
                    )) as usize]
                } - fTemp634;
                let mut iTemp636: i32 = (fTemp139
                    > (((if iTemp65 != 0 {
                        fTemp634
                            + fTemp78 * fTemp635
                            + (fTemp630 - (iTemp631) as F64)
                                * (fTemp633
                                    - (fTemp634
                                        + fTemp78
                                            * (fTemp635
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp632, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp633))))
                    } else {
                        1.0 - (fTemp628
                            + fTemp78 * fTemp629
                            + (fTemp624 - (iTemp625) as F64)
                                * (fTemp627
                                    - (fTemp628
                                        + fTemp78
                                            * (fTemp629
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp626, 8)) as usize]
                                                } - fTemp627)))))
                    }) - fTemp622)
                        / (1.0 - fTemp622))) as i32;
                let mut fTemp637: F64 = (if iTemp636 != 0 { fTemp606 } else { fTemp609 });
                let mut fTemp638: F64 = (if iTemp636 != 0 { fTemp609 } else { fTemp607 });
                let mut fTemp639: F64 = fTemp638 + fTemp637;
                let mut fTemp640: F64 = 0.5 * fTemp639;
                let mut fTemp641: F64 = 262143.0 * (1.0 - fTemp640);
                let mut iTemp642: i32 = (fTemp641) as i32;
                let mut iTemp643: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp642, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp644: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp643, 7)) as usize] };
                let mut fTemp645: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp643 as usize] };
                let mut fTemp646: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp643, 1)) as usize] }
                        - fTemp645;
                let mut fTemp647: F64 = 131071.5 * fTemp639;
                let mut iTemp648: i32 = (fTemp647) as i32;
                let mut iTemp649: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp648, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp650: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp649, 7)) as usize] };
                let mut fTemp651: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp649 as usize] };
                let mut fTemp652: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp649, 1)) as usize] }
                        - fTemp651;
                let mut fTemp653: F64 = (if iTemp65 != 0 {
                    fTemp651
                        + fTemp78 * fTemp652
                        + (fTemp647 - (iTemp648) as F64)
                            * (fTemp650
                                - (fTemp651
                                    + fTemp78
                                        * (fTemp652
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp649, 8),
                                                        1835007,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp650))))
                } else {
                    1.0 - (fTemp645
                        + fTemp78 * fTemp646
                        + (fTemp641 - (iTemp642) as F64)
                            * (fTemp644
                                - (fTemp645
                                    + fTemp78
                                        * (fTemp646
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp643, 8),
                                                        1835007,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp644)))))
                });
                let mut fTemp654: F64 = fTemp83 + fTemp640;
                let mut fTemp655: F64 = 262143.0 * (1.0 - fTemp654);
                let mut iTemp656: i32 = (fTemp655) as i32;
                let mut iTemp657: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp656, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp658: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp657, 7)) as usize] };
                let mut fTemp659: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp657 as usize] };
                let mut fTemp660: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp657, 1)) as usize] }
                        - fTemp659;
                let mut fTemp661: F64 = 262143.0 * fTemp654;
                let mut iTemp662: i32 = (fTemp661) as i32;
                let mut iTemp663: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp662, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp664: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp663, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp665: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp663 as usize] };
                let mut fTemp666: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp663, 1), 1835007),
                    )) as usize]
                } - fTemp665;
                let mut iTemp667: i32 = (fTemp139
                    > (((if iTemp65 != 0 {
                        fTemp665
                            + fTemp78 * fTemp666
                            + (fTemp661 - (iTemp662) as F64)
                                * (fTemp664
                                    - (fTemp665
                                        + fTemp78
                                            * (fTemp666
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp663, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp664))))
                    } else {
                        1.0 - (fTemp659
                            + fTemp78 * fTemp660
                            + (fTemp655 - (iTemp656) as F64)
                                * (fTemp658
                                    - (fTemp659
                                        + fTemp78
                                            * (fTemp660
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp657, 8)) as usize]
                                                } - fTemp658)))))
                    }) - fTemp653)
                        / (1.0 - fTemp653))) as i32;
                let mut fTemp668: F64 = F64::min(
                    1.0,
                    F64::max(
                        0.0,
                        0.5 * ((if iTemp667 != 0 { fTemp640 } else { fTemp638 })
                            + (if iTemp667 != 0 { fTemp637 } else { fTemp640 })),
                    ),
                );
                self.fRec1[0] = fTemp668;
                let mut fTemp669: F64 = 262143.0 * (1.0 - fTemp668);
                let mut iTemp670: i32 = (fTemp669) as i32;
                let mut iTemp671: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp670, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp672: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp671, 7)) as usize] };
                let mut fTemp673: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp671 as usize] };
                let mut fTemp674: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp671, 1)) as usize] }
                        - fTemp673;
                let mut fTemp675: F64 = 262143.0 * fTemp668;
                let mut iTemp676: i32 = (fTemp675) as i32;
                let mut iTemp677: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp676, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp678: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp677, 7)) as usize] };
                let mut fTemp679: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp677 as usize] };
                let mut fTemp680: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp677, 1)) as usize] }
                        - fTemp679;
                let mut fTemp681: F64 = (if iTemp65 != 0 {
                    fTemp679
                        + fTemp78 * fTemp680
                        + (fTemp675 - (iTemp676) as F64)
                            * (fTemp678
                                - (fTemp679
                                    + fTemp78
                                        * (fTemp680
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp677, 8),
                                                        1835007,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp678))))
                } else {
                    1.0 - (fTemp673
                        + fTemp78 * fTemp674
                        + (fTemp669 - (iTemp670) as F64)
                            * (fTemp672
                                - (fTemp673
                                    + fTemp78
                                        * (fTemp674
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp671, 8),
                                                        1835007,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp672)))))
                });
                let mut fTemp682: F64 = fTemp83 + fTemp668;
                let mut fTemp683: F64 = 262143.0 * (1.0 - fTemp682);
                let mut iTemp684: i32 = (fTemp683) as i32;
                let mut iTemp685: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp684, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp686: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp685, 7)) as usize] };
                let mut fTemp687: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp685 as usize] };
                let mut fTemp688: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp685, 1)) as usize] }
                        - fTemp687;
                let mut fTemp689: F64 = 262143.0 * fTemp682;
                let mut iTemp690: i32 = (fTemp689) as i32;
                let mut iTemp691: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp73,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp690, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp692: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp691, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp693: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp691 as usize] };
                let mut fTemp694: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp691, 1), 1835007),
                    )) as usize]
                } - fTemp693;
                let mut fTemp695: F64 = self.fRec2[1]
                    + (if ((0.001 * fTemp82) == 0.0) as i32 != 0 {
                        fTemp64
                    } else {
                        fTemp64
                            * ((if iTemp65 != 0 {
                                fTemp693
                                    + fTemp78 * fTemp694
                                    + (fTemp689 - (iTemp690) as F64)
                                        * (fTemp692
                                            - (fTemp693
                                                + fTemp78
                                                    * (fTemp694
                                                        - (unsafe {
                                                            ftbl0LambRs96kSIG0[(std::cmp::max(
                                                                0,
                                                                std::cmp::min(
                                                                    i32::wrapping_add(iTemp691, 8),
                                                                    1835007,
                                                                ),
                                                            ))
                                                                as usize]
                                                        } - fTemp692))))
                            } else {
                                1.0 - (fTemp687
                                    + fTemp78 * fTemp688
                                    + (fTemp683 - (iTemp684) as F64)
                                        * (fTemp686
                                            - (fTemp687
                                                + fTemp78
                                                    * (fTemp688
                                                        - (unsafe {
                                                            ftbl0LambRs96kSIG0[(i32::wrapping_add(
                                                                iTemp685, 8,
                                                            ))
                                                                as usize]
                                                        } - fTemp686)))))
                            }) - fTemp681)
                            / (1.0 - fTemp681)
                    });
                self.fRec2[0] = (if iTemp81 != 0 {
                    F64::min(fTemp695, self.fRec2[1])
                } else {
                    F64::max(fTemp695, self.fRec2[1])
                });
                self.fVec33[(self.IOTA0 & 16383) as usize] = F64::powf(1e+01, 0.05 * self.fRec2[0]);
                let mut fTemp696: F64 =
                    self.fVec33[((i32::wrapping_sub(self.IOTA0, iSlow74)) & 16383) as usize];
                self.fRec14[0] = fSlow76 + self.fConst4 * self.fRec14[1];
                *output0 = 0.5
                    * self.fVec1[((i32::wrapping_sub(self.IOTA0, iSlow75)) & 32767) as usize]
                    * fTemp2
                    + self.fRec14[0]
                        * self.fVec2[((i32::wrapping_sub(self.IOTA0, iSlow75)) & 32767) as usize]
                        * fTemp696
                        * fTemp4;
                let mut fTemp697: F64 = fTemp36 + fSlow17 * (fTemp37 - fTemp36);
                let mut iTemp698: i32 =
                    ((fTemp697 > fSlow11) as i32) + ((fTemp697 > fSlow9) as i32);
                let mut fTemp699: F64 = fTemp697 - fSlow8;
                let mut fTemp700: F64 = F64::min(
                    fTemp34,
                    -(fSlow18
                        * F64::max(
                            0.0,
                            (if (iTemp698 == 0) as i32 != 0 {
                                0.0
                            } else {
                                (if (iTemp698 == 1) as i32 != 0 {
                                    fSlow12 * LambRs96k_faustpower2_f(fSlow7 + fTemp699)
                                } else {
                                    fTemp699
                                })
                            }),
                        )),
                );
                self.fVec34[(self.IOTA0 & 16383) as usize] = fTemp700;
                let mut fTemp701: F64 = F64::min(
                    fTemp700,
                    self.fVec34[((i32::wrapping_sub(self.IOTA0, 1)) & 16383) as usize],
                );
                self.fVec35[0] = fTemp701;
                let mut fTemp702: F64 = F64::min(fTemp701, self.fVec35[2]);
                self.fVec36[0] = fTemp702;
                let mut fTemp703: F64 = F64::min(fTemp702, self.fVec36[4]);
                self.fVec37[0] = fTemp703;
                let mut fTemp704: F64 = F64::min(fTemp703, self.fVec37[8]);
                self.fVec38[(self.IOTA0 & 31) as usize] = fTemp704;
                let mut fTemp705: F64 = F64::min(
                    fTemp704,
                    self.fVec38[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize],
                );
                self.fVec39[(self.IOTA0 & 63) as usize] = fTemp705;
                let mut fTemp706: F64 = F64::min(
                    fTemp705,
                    self.fVec39[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize],
                );
                self.fVec40[(self.IOTA0 & 127) as usize] = fTemp706;
                let mut fTemp707: F64 = F64::min(
                    fTemp706,
                    self.fVec40[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize],
                );
                self.fVec41[(self.IOTA0 & 255) as usize] = fTemp707;
                let mut fTemp708: F64 = F64::min(
                    fTemp707,
                    self.fVec41[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize],
                );
                self.fVec42[(self.IOTA0 & 511) as usize] = fTemp708;
                let mut fTemp709: F64 = F64::min(
                    fTemp708,
                    self.fVec42[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize],
                );
                self.fVec43[(self.IOTA0 & 1023) as usize] = fTemp709;
                let mut fTemp710: F64 = F64::min(
                    fTemp709,
                    self.fVec43[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize],
                );
                self.fVec44[(self.IOTA0 & 2047) as usize] = fTemp710;
                let mut fTemp711: F64 = F64::min(
                    fTemp710,
                    self.fVec44[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize],
                );
                self.fVec45[(self.IOTA0 & 4095) as usize] = fTemp711;
                self.fVec46[(self.IOTA0 & 8191) as usize] = F64::min(
                    fTemp711,
                    self.fVec45[((i32::wrapping_sub(self.IOTA0, 2048)) & 4095) as usize],
                );
                self.fRec17[0] = F64::max(
                    F64::min(
                        self.fRec17[1],
                        self.fVec34[((i32::wrapping_sub(self.IOTA0, iSlow21)) & 16383) as usize],
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
                                                                    (if iSlow23 != 0 {
                                                                        fTemp700
                                                                    } else {
                                                                        1.7976931348623157e+308
                                                                    }),
                                                                    (if iSlow24 != 0 {
                                                                        self.fVec35
                                                                            [iSlow23 as usize]
                                                                    } else {
                                                                        1.7976931348623157e+308
                                                                    }),
                                                                ),
                                                                (if iSlow25 != 0 {
                                                                    self.fVec36[iSlow26 as usize]
                                                                } else {
                                                                    1.7976931348623157e+308
                                                                }),
                                                            ),
                                                            (if iSlow27 != 0 {
                                                                self.fVec37[iSlow28 as usize]
                                                            } else {
                                                                1.7976931348623157e+308
                                                            }),
                                                        ),
                                                        (if iSlow29 != 0 {
                                                            self.fVec38[((i32::wrapping_sub(
                                                                self.IOTA0, iSlow30,
                                                            )) & 31)
                                                                as usize]
                                                        } else {
                                                            1.7976931348623157e+308
                                                        }),
                                                    ),
                                                    (if iSlow31 != 0 {
                                                        self.fVec39[((i32::wrapping_sub(
                                                            self.IOTA0, iSlow32,
                                                        )) & 63)
                                                            as usize]
                                                    } else {
                                                        1.7976931348623157e+308
                                                    }),
                                                ),
                                                (if iSlow33 != 0 {
                                                    self.fVec40[((i32::wrapping_sub(
                                                        self.IOTA0, iSlow34,
                                                    )) & 127)
                                                        as usize]
                                                } else {
                                                    1.7976931348623157e+308
                                                }),
                                            ),
                                            (if iSlow35 != 0 {
                                                self.fVec41[((i32::wrapping_sub(
                                                    self.IOTA0, iSlow36,
                                                )) & 255)
                                                    as usize]
                                            } else {
                                                1.7976931348623157e+308
                                            }),
                                        ),
                                        (if iSlow37 != 0 {
                                            self.fVec42[((i32::wrapping_sub(self.IOTA0, iSlow38))
                                                & 511)
                                                as usize]
                                        } else {
                                            1.7976931348623157e+308
                                        }),
                                    ),
                                    (if iSlow39 != 0 {
                                        self.fVec43[((i32::wrapping_sub(self.IOTA0, iSlow40))
                                            & 1023)
                                            as usize]
                                    } else {
                                        1.7976931348623157e+308
                                    }),
                                ),
                                (if iSlow41 != 0 {
                                    self.fVec44
                                        [((i32::wrapping_sub(self.IOTA0, iSlow42)) & 2047) as usize]
                                } else {
                                    1.7976931348623157e+308
                                }),
                            ),
                            (if iSlow43 != 0 {
                                self.fVec45
                                    [((i32::wrapping_sub(self.IOTA0, iSlow44)) & 4095) as usize]
                            } else {
                                1.7976931348623157e+308
                            }),
                        ),
                        (if iSlow45 != 0 {
                            self.fVec46[((i32::wrapping_sub(self.IOTA0, iSlow46)) & 8191) as usize]
                        } else {
                            1.7976931348623157e+308
                        }),
                    ),
                );
                let mut fTemp712: F64 = F64::min(self.fRec17[0], self.fRec17[1]);
                self.fVec47[0] = fTemp712;
                let mut fTemp713: F64 = F64::min(fTemp712, self.fVec47[2]);
                self.fVec48[0] = fTemp713;
                let mut fTemp714: F64 = F64::min(fTemp713, self.fVec48[4]);
                self.fVec49[0] = fTemp714;
                let mut fTemp715: F64 = F64::min(fTemp714, self.fVec49[8]);
                self.fVec50[(self.IOTA0 & 31) as usize] = fTemp715;
                let mut fTemp716: F64 = F64::min(
                    fTemp715,
                    self.fVec50[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize],
                );
                self.fVec51[(self.IOTA0 & 63) as usize] = fTemp716;
                let mut fTemp717: F64 = F64::min(
                    fTemp716,
                    self.fVec51[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize],
                );
                self.fVec52[(self.IOTA0 & 127) as usize] = fTemp717;
                let mut fTemp718: F64 = F64::min(
                    fTemp717,
                    self.fVec52[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize],
                );
                self.fVec53[(self.IOTA0 & 255) as usize] = fTemp718;
                let mut fTemp719: F64 = F64::min(
                    fTemp718,
                    self.fVec53[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize],
                );
                self.fVec54[(self.IOTA0 & 511) as usize] = fTemp719;
                let mut fTemp720: F64 = F64::min(
                    fTemp719,
                    self.fVec54[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize],
                );
                self.fVec55[(self.IOTA0 & 1023) as usize] = fTemp720;
                let mut fTemp721: F64 = F64::min(
                    fTemp720,
                    self.fVec55[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize],
                );
                self.fVec56[(self.IOTA0 & 2047) as usize] = fTemp721;
                let mut fTemp722: F64 = F64::min(
                    fTemp721,
                    self.fVec56[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize],
                );
                self.fVec57[(self.IOTA0 & 4095) as usize] = fTemp722;
                self.fVec58[(self.IOTA0 & 8191) as usize] = F64::min(
                    fTemp722,
                    self.fVec57[((i32::wrapping_sub(self.IOTA0, 2048)) & 4095) as usize],
                );
                let mut fTemp723: F64 = F64::min(
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
                                                                (if iSlow47 != 0 {
                                                                    self.fVec47[iSlow4 as usize]
                                                                } else {
                                                                    1.7976931348623157e+308
                                                                }),
                                                            ),
                                                            (if iSlow48 != 0 {
                                                                self.fVec48[iSlow49 as usize]
                                                            } else {
                                                                1.7976931348623157e+308
                                                            }),
                                                        ),
                                                        (if iSlow50 != 0 {
                                                            self.fVec49[iSlow51 as usize]
                                                        } else {
                                                            1.7976931348623157e+308
                                                        }),
                                                    ),
                                                    (if iSlow52 != 0 {
                                                        self.fVec50[((i32::wrapping_sub(
                                                            self.IOTA0, iSlow53,
                                                        )) & 31)
                                                            as usize]
                                                    } else {
                                                        1.7976931348623157e+308
                                                    }),
                                                ),
                                                (if iSlow54 != 0 {
                                                    self.fVec51[((i32::wrapping_sub(
                                                        self.IOTA0, iSlow55,
                                                    )) & 63)
                                                        as usize]
                                                } else {
                                                    1.7976931348623157e+308
                                                }),
                                            ),
                                            (if iSlow56 != 0 {
                                                self.fVec52[((i32::wrapping_sub(
                                                    self.IOTA0, iSlow57,
                                                )) & 127)
                                                    as usize]
                                            } else {
                                                1.7976931348623157e+308
                                            }),
                                        ),
                                        (if iSlow58 != 0 {
                                            self.fVec53[((i32::wrapping_sub(self.IOTA0, iSlow59))
                                                & 255)
                                                as usize]
                                        } else {
                                            1.7976931348623157e+308
                                        }),
                                    ),
                                    (if iSlow60 != 0 {
                                        self.fVec54[((i32::wrapping_sub(self.IOTA0, iSlow61)) & 511)
                                            as usize]
                                    } else {
                                        1.7976931348623157e+308
                                    }),
                                ),
                                (if iSlow62 != 0 {
                                    self.fVec55
                                        [((i32::wrapping_sub(self.IOTA0, iSlow63)) & 1023) as usize]
                                } else {
                                    1.7976931348623157e+308
                                }),
                            ),
                            (if iSlow64 != 0 {
                                self.fVec56
                                    [((i32::wrapping_sub(self.IOTA0, iSlow65)) & 2047) as usize]
                            } else {
                                1.7976931348623157e+308
                            }),
                        ),
                        (if iSlow66 != 0 {
                            self.fVec57[((i32::wrapping_sub(self.IOTA0, iSlow67)) & 4095) as usize]
                        } else {
                            1.7976931348623157e+308
                        }),
                    ),
                    (if iSlow68 != 0 {
                        self.fVec58[((i32::wrapping_sub(self.IOTA0, iSlow69)) & 8191) as usize]
                    } else {
                        1.7976931348623157e+308
                    }),
                ) - self.fRec16[1];
                self.fVec59[0] = fTemp723;
                let mut iTemp724: i32 = (fTemp723 > 0.0) as i32;
                let mut fTemp725: F64 = (if iTemp724 != 0 { fSlow71 } else { fSlow70 });
                self.fVec60[0] = fTemp725;
                let mut fTemp726: F64 = 6.0 * fTemp725;
                let mut iTemp727: i32 = (fTemp726) as i32;
                let mut iTemp728: i32 = std::cmp::max(0, std::cmp::min(iTemp727, 6));
                let mut iTemp729: i32 = std::cmp::max(
                    0,
                    std::cmp::min(i32::wrapping_add(iTemp728, 917497), 1835007),
                );
                let mut fTemp730: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp729, 7)) as usize] };
                let mut fTemp731: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp729 as usize] };
                let mut fTemp732: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp729, 1)) as usize] }
                        - fTemp731;
                let mut fTemp733: F64 = fTemp726 - (iTemp727) as F64;
                let mut fTemp734: F64 = fTemp731
                    + fTemp733 * fTemp732
                    + 0.5
                        * (fTemp730
                            - (fTemp731
                                + fTemp733
                                    * (fTemp732
                                        - (unsafe {
                                            ftbl0LambRs96kSIG0
                                                [(i32::wrapping_add(iTemp729, 8)) as usize]
                                        } - fTemp730))));
                let mut fTemp735: F64 = (if iTemp724 != 0 {
                    fTemp734
                } else {
                    1.0 - fTemp734
                });
                let mut iTemp736: i32 = (fTemp723 < 0.0) as i32;
                let mut fTemp737: F64 = fSlow1 * (iTemp736) as F64 + fSlow13 * (iTemp724) as F64;
                self.fVec61[0] = fTemp737;
                let mut fTemp738: F64 = self.fConst10 / fTemp737;
                let mut fTemp739: F64 = fTemp738 + 0.5;
                let mut fTemp740: F64 = 262143.0 * (1.0 - fTemp739);
                let mut iTemp741: i32 = (fTemp740) as i32;
                let mut iTemp742: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp741, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp743: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp742, 7)) as usize] };
                let mut fTemp744: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp742 as usize] };
                let mut fTemp745: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp742, 1)) as usize] }
                        - fTemp744;
                let mut fTemp746: F64 = 262143.0 * fTemp739;
                let mut iTemp747: i32 = (fTemp746) as i32;
                let mut iTemp748: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp747, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp749: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp748, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp750: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp748 as usize] };
                let mut fTemp751: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp748, 1), 1835007),
                    )) as usize]
                } - fTemp750;
                let mut fTemp752: F64 = 6.0 * self.fVec60[1];
                let mut iTemp753: i32 = (fTemp752) as i32;
                let mut iTemp754: i32 = std::cmp::max(0, std::cmp::min(iTemp753, 6));
                let mut fTemp755: F64 = 262143.0 * (1.0 - self.fRec15[1]);
                let mut iTemp756: i32 = (fTemp755) as i32;
                let mut iTemp757: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp756, 262143))),
                            iTemp754,
                        ),
                        1835007,
                    ),
                );
                let mut fTemp758: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp757, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp759: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp757 as usize] };
                let mut fTemp760: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp757, 1), 1835007),
                    )) as usize]
                } - fTemp759;
                let mut fTemp761: F64 = fTemp752 - (iTemp753) as F64;
                let mut fTemp762: F64 = 262143.0 * self.fRec15[1];
                let mut iTemp763: i32 = (fTemp762) as i32;
                let mut iTemp764: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp754,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp763, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp765: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp764, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp766: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp764 as usize] };
                let mut fTemp767: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp764, 1), 1835007),
                    )) as usize]
                } - fTemp766;
                let mut fTemp768: F64 = self.fRec15[1] + fTemp738;
                let mut fTemp769: F64 = 262143.0 * (1.0 - fTemp768);
                let mut iTemp770: i32 = (fTemp769) as i32;
                let mut iTemp771: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp770, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp772: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp771, 7)) as usize] };
                let mut fTemp773: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp771 as usize] };
                let mut fTemp774: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp771, 1)) as usize] }
                        - fTemp773;
                let mut fTemp775: F64 = 262143.0 * fTemp768;
                let mut iTemp776: i32 = (fTemp775) as i32;
                let mut iTemp777: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp776, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp778: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp777, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp779: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp777 as usize] };
                let mut fTemp780: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp777, 1), 1835007),
                    )) as usize]
                } - fTemp779;
                let mut fTemp781: F64 =
                    self.fRec15[1] + self.fConst10 * (1.0 / fTemp737 + 1.0 / self.fVec61[1]);
                let mut fTemp782: F64 = 262143.0 * (1.0 - fTemp781);
                let mut iTemp783: i32 = (fTemp782) as i32;
                let mut iTemp784: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp783, 262143))),
                            iTemp728,
                        ),
                        1835007,
                    ),
                );
                let mut fTemp785: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp784, 7)) as usize] };
                let mut fTemp786: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp784 as usize] };
                let mut fTemp787: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp784, 1)) as usize] }
                        - fTemp786;
                let mut fTemp788: F64 = 262143.0 * fTemp781;
                let mut iTemp789: i32 = (fTemp788) as i32;
                let mut iTemp790: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp789, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp791: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp790, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp792: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp790 as usize] };
                let mut fTemp793: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp790, 1), 1835007),
                    )) as usize]
                } - fTemp792;
                let mut fTemp794: F64 = ((if iTemp724 != 0 {
                    fTemp792
                        + fTemp733 * fTemp793
                        + (fTemp788 - (iTemp789) as F64)
                            * (fTemp791
                                - (fTemp792
                                    + fTemp733
                                        * (fTemp793
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp790, 8),
                                                        1835007,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp791))))
                } else {
                    1.0 - (fTemp786
                        + fTemp733 * fTemp787
                        + (fTemp782 - (iTemp783) as F64)
                            * (fTemp785
                                - (fTemp786
                                    + fTemp733
                                        * (fTemp787
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp784, 8)) as usize]
                                            } - fTemp785)))))
                }) - (if iTemp724 != 0 {
                    fTemp779
                        + fTemp733 * fTemp780
                        + (fTemp775 - (iTemp776) as F64)
                            * (fTemp778
                                - (fTemp779
                                    + fTemp733
                                        * (fTemp780
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp777, 8),
                                                        1835007,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp778))))
                } else {
                    1.0 - (fTemp773
                        + fTemp733 * fTemp774
                        + (fTemp769 - (iTemp770) as F64)
                            * (fTemp772
                                - (fTemp773
                                    + fTemp733
                                        * (fTemp774
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp771, 8)) as usize]
                                            } - fTemp772)))))
                })) * self.fVec59[1]
                    / (fTemp723
                        * (1.0
                            - (if iTemp724 != 0 {
                                fTemp766
                                    + fTemp761 * fTemp767
                                    + (fTemp762 - (iTemp763) as F64)
                                        * (fTemp765
                                            - (fTemp766
                                                + fTemp761
                                                    * (fTemp767
                                                        - (unsafe {
                                                            ftbl0LambRs96kSIG0[(std::cmp::max(
                                                                0,
                                                                std::cmp::min(
                                                                    i32::wrapping_add(iTemp764, 8),
                                                                    1835007,
                                                                ),
                                                            ))
                                                                as usize]
                                                        } - fTemp765))))
                            } else {
                                1.0 - (fTemp759
                                    + fTemp761 * fTemp760
                                    + (fTemp755 - (iTemp756) as F64)
                                        * (fTemp758
                                            - (fTemp759
                                                + fTemp761
                                                    * (fTemp760
                                                        - (unsafe {
                                                            ftbl0LambRs96kSIG0[(std::cmp::max(
                                                                0,
                                                                std::cmp::min(
                                                                    i32::wrapping_add(iTemp757, 8),
                                                                    1835007,
                                                                ),
                                                            ))
                                                                as usize]
                                                        } - fTemp758)))))
                            })));
                let mut iTemp795: i32 = (fTemp794
                    > (((if iTemp724 != 0 {
                        fTemp750
                            + fTemp733 * fTemp751
                            + (fTemp746 - (iTemp747) as F64)
                                * (fTemp749
                                    - (fTemp750
                                        + fTemp733
                                            * (fTemp751
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp748, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp749))))
                    } else {
                        1.0 - (fTemp744
                            + fTemp733 * fTemp745
                            + (fTemp740 - (iTemp741) as F64)
                                * (fTemp743
                                    - (fTemp744
                                        + fTemp733
                                            * (fTemp745
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp742, 8)) as usize]
                                                } - fTemp743)))))
                    }) - fTemp735)
                        / (1.0 - fTemp735))) as i32;
                let mut fTemp796: F64 = (if iTemp795 != 0 { 1.0 } else { 0.5 });
                let mut fTemp797: F64 = (if iTemp795 != 0 { 0.5 } else { 0.0 });
                let mut fTemp798: F64 = fTemp797 + fTemp796;
                let mut fTemp799: F64 = 0.5 * fTemp798;
                let mut fTemp800: F64 = 262143.0 * (1.0 - fTemp799);
                let mut iTemp801: i32 = (fTemp800) as i32;
                let mut iTemp802: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp801, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp803: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp802, 7)) as usize] };
                let mut fTemp804: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp802 as usize] };
                let mut fTemp805: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp802, 1)) as usize] }
                        - fTemp804;
                let mut fTemp806: F64 = 131071.5 * fTemp798;
                let mut iTemp807: i32 = (fTemp806) as i32;
                let mut iTemp808: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp807, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp809: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp808, 7)) as usize] };
                let mut fTemp810: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp808 as usize] };
                let mut fTemp811: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp808, 1)) as usize] }
                        - fTemp810;
                let mut fTemp812: F64 = (if iTemp724 != 0 {
                    fTemp810
                        + fTemp733 * fTemp811
                        + (fTemp806 - (iTemp807) as F64)
                            * (fTemp809
                                - (fTemp810
                                    + fTemp733
                                        * (fTemp811
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp808, 8)) as usize]
                                            } - fTemp809))))
                } else {
                    1.0 - (fTemp804
                        + fTemp733 * fTemp805
                        + (fTemp800 - (iTemp801) as F64)
                            * (fTemp803
                                - (fTemp804
                                    + fTemp733
                                        * (fTemp805
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp802, 8)) as usize]
                                            } - fTemp803)))))
                });
                let mut fTemp813: F64 = fTemp738 + fTemp799;
                let mut fTemp814: F64 = 262143.0 * (1.0 - fTemp813);
                let mut iTemp815: i32 = (fTemp814) as i32;
                let mut iTemp816: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp815, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp817: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp816, 7)) as usize] };
                let mut fTemp818: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp816 as usize] };
                let mut fTemp819: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp816, 1)) as usize] }
                        - fTemp818;
                let mut fTemp820: F64 = 262143.0 * fTemp813;
                let mut iTemp821: i32 = (fTemp820) as i32;
                let mut iTemp822: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp821, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp823: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp822, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp824: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp822 as usize] };
                let mut fTemp825: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp822, 1), 1835007),
                    )) as usize]
                } - fTemp824;
                let mut iTemp826: i32 = (fTemp794
                    > (((if iTemp724 != 0 {
                        fTemp824
                            + fTemp733 * fTemp825
                            + (fTemp820 - (iTemp821) as F64)
                                * (fTemp823
                                    - (fTemp824
                                        + fTemp733
                                            * (fTemp825
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp822, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp823))))
                    } else {
                        1.0 - (fTemp818
                            + fTemp733 * fTemp819
                            + (fTemp814 - (iTemp815) as F64)
                                * (fTemp817
                                    - (fTemp818
                                        + fTemp733
                                            * (fTemp819
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp816, 8)) as usize]
                                                } - fTemp817)))))
                    }) - fTemp812)
                        / (1.0 - fTemp812))) as i32;
                let mut fTemp827: F64 = (if iTemp826 != 0 { fTemp796 } else { fTemp799 });
                let mut fTemp828: F64 = (if iTemp826 != 0 { fTemp799 } else { fTemp797 });
                let mut fTemp829: F64 = fTemp828 + fTemp827;
                let mut fTemp830: F64 = 0.5 * fTemp829;
                let mut fTemp831: F64 = 262143.0 * (1.0 - fTemp830);
                let mut iTemp832: i32 = (fTemp831) as i32;
                let mut iTemp833: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp832, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp834: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp833, 7)) as usize] };
                let mut fTemp835: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp833 as usize] };
                let mut fTemp836: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp833, 1)) as usize] }
                        - fTemp835;
                let mut fTemp837: F64 = 131071.5 * fTemp829;
                let mut iTemp838: i32 = (fTemp837) as i32;
                let mut iTemp839: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp838, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp840: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp839, 7)) as usize] };
                let mut fTemp841: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp839 as usize] };
                let mut fTemp842: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp839, 1)) as usize] }
                        - fTemp841;
                let mut fTemp843: F64 = (if iTemp724 != 0 {
                    fTemp841
                        + fTemp733 * fTemp842
                        + (fTemp837 - (iTemp838) as F64)
                            * (fTemp840
                                - (fTemp841
                                    + fTemp733
                                        * (fTemp842
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp839, 8)) as usize]
                                            } - fTemp840))))
                } else {
                    1.0 - (fTemp835
                        + fTemp733 * fTemp836
                        + (fTemp831 - (iTemp832) as F64)
                            * (fTemp834
                                - (fTemp835
                                    + fTemp733
                                        * (fTemp836
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp833, 8)) as usize]
                                            } - fTemp834)))))
                });
                let mut fTemp844: F64 = fTemp738 + fTemp830;
                let mut fTemp845: F64 = 262143.0 * (1.0 - fTemp844);
                let mut iTemp846: i32 = (fTemp845) as i32;
                let mut iTemp847: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp846, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp848: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp847, 7)) as usize] };
                let mut fTemp849: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp847 as usize] };
                let mut fTemp850: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp847, 1)) as usize] }
                        - fTemp849;
                let mut fTemp851: F64 = 262143.0 * fTemp844;
                let mut iTemp852: i32 = (fTemp851) as i32;
                let mut iTemp853: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp852, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp854: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp853, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp855: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp853 as usize] };
                let mut fTemp856: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp853, 1), 1835007),
                    )) as usize]
                } - fTemp855;
                let mut iTemp857: i32 = (fTemp794
                    > (((if iTemp724 != 0 {
                        fTemp855
                            + fTemp733 * fTemp856
                            + (fTemp851 - (iTemp852) as F64)
                                * (fTemp854
                                    - (fTemp855
                                        + fTemp733
                                            * (fTemp856
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp853, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp854))))
                    } else {
                        1.0 - (fTemp849
                            + fTemp733 * fTemp850
                            + (fTemp845 - (iTemp846) as F64)
                                * (fTemp848
                                    - (fTemp849
                                        + fTemp733
                                            * (fTemp850
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp847, 8)) as usize]
                                                } - fTemp848)))))
                    }) - fTemp843)
                        / (1.0 - fTemp843))) as i32;
                let mut fTemp858: F64 = (if iTemp857 != 0 { fTemp827 } else { fTemp830 });
                let mut fTemp859: F64 = (if iTemp857 != 0 { fTemp830 } else { fTemp828 });
                let mut fTemp860: F64 = fTemp859 + fTemp858;
                let mut fTemp861: F64 = 0.5 * fTemp860;
                let mut fTemp862: F64 = 262143.0 * (1.0 - fTemp861);
                let mut iTemp863: i32 = (fTemp862) as i32;
                let mut iTemp864: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp863, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp865: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp864, 7)) as usize] };
                let mut fTemp866: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp864 as usize] };
                let mut fTemp867: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp864, 1)) as usize] }
                        - fTemp866;
                let mut fTemp868: F64 = 131071.5 * fTemp860;
                let mut iTemp869: i32 = (fTemp868) as i32;
                let mut iTemp870: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp869, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp871: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp870, 7)) as usize] };
                let mut fTemp872: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp870 as usize] };
                let mut fTemp873: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp870, 1)) as usize] }
                        - fTemp872;
                let mut fTemp874: F64 = (if iTemp724 != 0 {
                    fTemp872
                        + fTemp733 * fTemp873
                        + (fTemp868 - (iTemp869) as F64)
                            * (fTemp871
                                - (fTemp872
                                    + fTemp733
                                        * (fTemp873
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp870, 8)) as usize]
                                            } - fTemp871))))
                } else {
                    1.0 - (fTemp866
                        + fTemp733 * fTemp867
                        + (fTemp862 - (iTemp863) as F64)
                            * (fTemp865
                                - (fTemp866
                                    + fTemp733
                                        * (fTemp867
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp864, 8)) as usize]
                                            } - fTemp865)))))
                });
                let mut fTemp875: F64 = fTemp738 + fTemp861;
                let mut fTemp876: F64 = 262143.0 * (1.0 - fTemp875);
                let mut iTemp877: i32 = (fTemp876) as i32;
                let mut iTemp878: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp877, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp879: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp878, 7)) as usize] };
                let mut fTemp880: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp878 as usize] };
                let mut fTemp881: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp878, 1)) as usize] }
                        - fTemp880;
                let mut fTemp882: F64 = 262143.0 * fTemp875;
                let mut iTemp883: i32 = (fTemp882) as i32;
                let mut iTemp884: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp883, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp885: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp884, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp886: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp884 as usize] };
                let mut fTemp887: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp884, 1), 1835007),
                    )) as usize]
                } - fTemp886;
                let mut iTemp888: i32 = (fTemp794
                    > (((if iTemp724 != 0 {
                        fTemp886
                            + fTemp733 * fTemp887
                            + (fTemp882 - (iTemp883) as F64)
                                * (fTemp885
                                    - (fTemp886
                                        + fTemp733
                                            * (fTemp887
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp884, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp885))))
                    } else {
                        1.0 - (fTemp880
                            + fTemp733 * fTemp881
                            + (fTemp876 - (iTemp877) as F64)
                                * (fTemp879
                                    - (fTemp880
                                        + fTemp733
                                            * (fTemp881
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp878, 8)) as usize]
                                                } - fTemp879)))))
                    }) - fTemp874)
                        / (1.0 - fTemp874))) as i32;
                let mut fTemp889: F64 = (if iTemp888 != 0 { fTemp858 } else { fTemp861 });
                let mut fTemp890: F64 = (if iTemp888 != 0 { fTemp861 } else { fTemp859 });
                let mut fTemp891: F64 = fTemp890 + fTemp889;
                let mut fTemp892: F64 = 0.5 * fTemp891;
                let mut fTemp893: F64 = 262143.0 * (1.0 - fTemp892);
                let mut iTemp894: i32 = (fTemp893) as i32;
                let mut iTemp895: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp894, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp896: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp895, 7)) as usize] };
                let mut fTemp897: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp895 as usize] };
                let mut fTemp898: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp895, 1)) as usize] }
                        - fTemp897;
                let mut fTemp899: F64 = 131071.5 * fTemp891;
                let mut iTemp900: i32 = (fTemp899) as i32;
                let mut iTemp901: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp900, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp902: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp901, 7)) as usize] };
                let mut fTemp903: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp901 as usize] };
                let mut fTemp904: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp901, 1)) as usize] }
                        - fTemp903;
                let mut fTemp905: F64 = (if iTemp724 != 0 {
                    fTemp903
                        + fTemp733 * fTemp904
                        + (fTemp899 - (iTemp900) as F64)
                            * (fTemp902
                                - (fTemp903
                                    + fTemp733
                                        * (fTemp904
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp901, 8)) as usize]
                                            } - fTemp902))))
                } else {
                    1.0 - (fTemp897
                        + fTemp733 * fTemp898
                        + (fTemp893 - (iTemp894) as F64)
                            * (fTemp896
                                - (fTemp897
                                    + fTemp733
                                        * (fTemp898
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp895, 8)) as usize]
                                            } - fTemp896)))))
                });
                let mut fTemp906: F64 = fTemp738 + fTemp892;
                let mut fTemp907: F64 = 262143.0 * (1.0 - fTemp906);
                let mut iTemp908: i32 = (fTemp907) as i32;
                let mut iTemp909: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp908, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp910: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp909, 7)) as usize] };
                let mut fTemp911: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp909 as usize] };
                let mut fTemp912: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp909, 1)) as usize] }
                        - fTemp911;
                let mut fTemp913: F64 = 262143.0 * fTemp906;
                let mut iTemp914: i32 = (fTemp913) as i32;
                let mut iTemp915: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp914, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp916: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp915, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp917: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp915 as usize] };
                let mut fTemp918: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp915, 1), 1835007),
                    )) as usize]
                } - fTemp917;
                let mut iTemp919: i32 = (fTemp794
                    > (((if iTemp724 != 0 {
                        fTemp917
                            + fTemp733 * fTemp918
                            + (fTemp913 - (iTemp914) as F64)
                                * (fTemp916
                                    - (fTemp917
                                        + fTemp733
                                            * (fTemp918
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp915, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp916))))
                    } else {
                        1.0 - (fTemp911
                            + fTemp733 * fTemp912
                            + (fTemp907 - (iTemp908) as F64)
                                * (fTemp910
                                    - (fTemp911
                                        + fTemp733
                                            * (fTemp912
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp909, 8)) as usize]
                                                } - fTemp910)))))
                    }) - fTemp905)
                        / (1.0 - fTemp905))) as i32;
                let mut fTemp920: F64 = (if iTemp919 != 0 { fTemp889 } else { fTemp892 });
                let mut fTemp921: F64 = (if iTemp919 != 0 { fTemp892 } else { fTemp890 });
                let mut fTemp922: F64 = fTemp921 + fTemp920;
                let mut fTemp923: F64 = 0.5 * fTemp922;
                let mut fTemp924: F64 = 262143.0 * (1.0 - fTemp923);
                let mut iTemp925: i32 = (fTemp924) as i32;
                let mut iTemp926: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp925, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp927: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp926, 7)) as usize] };
                let mut fTemp928: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp926 as usize] };
                let mut fTemp929: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp926, 1)) as usize] }
                        - fTemp928;
                let mut fTemp930: F64 = 131071.5 * fTemp922;
                let mut iTemp931: i32 = (fTemp930) as i32;
                let mut iTemp932: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp931, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp933: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp932, 7)) as usize] };
                let mut fTemp934: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp932 as usize] };
                let mut fTemp935: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp932, 1)) as usize] }
                        - fTemp934;
                let mut fTemp936: F64 = (if iTemp724 != 0 {
                    fTemp934
                        + fTemp733 * fTemp935
                        + (fTemp930 - (iTemp931) as F64)
                            * (fTemp933
                                - (fTemp934
                                    + fTemp733
                                        * (fTemp935
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp932, 8)) as usize]
                                            } - fTemp933))))
                } else {
                    1.0 - (fTemp928
                        + fTemp733 * fTemp929
                        + (fTemp924 - (iTemp925) as F64)
                            * (fTemp927
                                - (fTemp928
                                    + fTemp733
                                        * (fTemp929
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp926, 8)) as usize]
                                            } - fTemp927)))))
                });
                let mut fTemp937: F64 = fTemp738 + fTemp923;
                let mut fTemp938: F64 = 262143.0 * (1.0 - fTemp937);
                let mut iTemp939: i32 = (fTemp938) as i32;
                let mut iTemp940: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp939, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp941: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp940, 7)) as usize] };
                let mut fTemp942: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp940 as usize] };
                let mut fTemp943: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp940, 1)) as usize] }
                        - fTemp942;
                let mut fTemp944: F64 = 262143.0 * fTemp937;
                let mut iTemp945: i32 = (fTemp944) as i32;
                let mut iTemp946: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp945, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp947: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp946, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp948: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp946 as usize] };
                let mut fTemp949: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp946, 1), 1835007),
                    )) as usize]
                } - fTemp948;
                let mut iTemp950: i32 = (fTemp794
                    > (((if iTemp724 != 0 {
                        fTemp948
                            + fTemp733 * fTemp949
                            + (fTemp944 - (iTemp945) as F64)
                                * (fTemp947
                                    - (fTemp948
                                        + fTemp733
                                            * (fTemp949
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp946, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp947))))
                    } else {
                        1.0 - (fTemp942
                            + fTemp733 * fTemp943
                            + (fTemp938 - (iTemp939) as F64)
                                * (fTemp941
                                    - (fTemp942
                                        + fTemp733
                                            * (fTemp943
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp940, 8)) as usize]
                                                } - fTemp941)))))
                    }) - fTemp936)
                        / (1.0 - fTemp936))) as i32;
                let mut fTemp951: F64 = (if iTemp950 != 0 { fTemp920 } else { fTemp923 });
                let mut fTemp952: F64 = (if iTemp950 != 0 { fTemp923 } else { fTemp921 });
                let mut fTemp953: F64 = fTemp952 + fTemp951;
                let mut fTemp954: F64 = 0.5 * fTemp953;
                let mut fTemp955: F64 = 262143.0 * (1.0 - fTemp954);
                let mut iTemp956: i32 = (fTemp955) as i32;
                let mut iTemp957: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp956, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp958: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp957, 7)) as usize] };
                let mut fTemp959: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp957 as usize] };
                let mut fTemp960: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp957, 1)) as usize] }
                        - fTemp959;
                let mut fTemp961: F64 = 131071.5 * fTemp953;
                let mut iTemp962: i32 = (fTemp961) as i32;
                let mut iTemp963: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp962, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp964: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp963, 7)) as usize] };
                let mut fTemp965: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp963 as usize] };
                let mut fTemp966: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp963, 1)) as usize] }
                        - fTemp965;
                let mut fTemp967: F64 = (if iTemp724 != 0 {
                    fTemp965
                        + fTemp733 * fTemp966
                        + (fTemp961 - (iTemp962) as F64)
                            * (fTemp964
                                - (fTemp965
                                    + fTemp733
                                        * (fTemp966
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp963, 8)) as usize]
                                            } - fTemp964))))
                } else {
                    1.0 - (fTemp959
                        + fTemp733 * fTemp960
                        + (fTemp955 - (iTemp956) as F64)
                            * (fTemp958
                                - (fTemp959
                                    + fTemp733
                                        * (fTemp960
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp957, 8)) as usize]
                                            } - fTemp958)))))
                });
                let mut fTemp968: F64 = fTemp738 + fTemp954;
                let mut fTemp969: F64 = 262143.0 * (1.0 - fTemp968);
                let mut iTemp970: i32 = (fTemp969) as i32;
                let mut iTemp971: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp970, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp972: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp971, 7)) as usize] };
                let mut fTemp973: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp971 as usize] };
                let mut fTemp974: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp971, 1)) as usize] }
                        - fTemp973;
                let mut fTemp975: F64 = 262143.0 * fTemp968;
                let mut iTemp976: i32 = (fTemp975) as i32;
                let mut iTemp977: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp976, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp978: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp977, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp979: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp977 as usize] };
                let mut fTemp980: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp977, 1), 1835007),
                    )) as usize]
                } - fTemp979;
                let mut iTemp981: i32 = (fTemp794
                    > (((if iTemp724 != 0 {
                        fTemp979
                            + fTemp733 * fTemp980
                            + (fTemp975 - (iTemp976) as F64)
                                * (fTemp978
                                    - (fTemp979
                                        + fTemp733
                                            * (fTemp980
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp977, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp978))))
                    } else {
                        1.0 - (fTemp973
                            + fTemp733 * fTemp974
                            + (fTemp969 - (iTemp970) as F64)
                                * (fTemp972
                                    - (fTemp973
                                        + fTemp733
                                            * (fTemp974
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp971, 8)) as usize]
                                                } - fTemp972)))))
                    }) - fTemp967)
                        / (1.0 - fTemp967))) as i32;
                let mut fTemp982: F64 = (if iTemp981 != 0 { fTemp951 } else { fTemp954 });
                let mut fTemp983: F64 = (if iTemp981 != 0 { fTemp954 } else { fTemp952 });
                let mut fTemp984: F64 = fTemp983 + fTemp982;
                let mut fTemp985: F64 = 0.5 * fTemp984;
                let mut fTemp986: F64 = 262143.0 * (1.0 - fTemp985);
                let mut iTemp987: i32 = (fTemp986) as i32;
                let mut iTemp988: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp987, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp989: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp988, 7)) as usize] };
                let mut fTemp990: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp988 as usize] };
                let mut fTemp991: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp988, 1)) as usize] }
                        - fTemp990;
                let mut fTemp992: F64 = 131071.5 * fTemp984;
                let mut iTemp993: i32 = (fTemp992) as i32;
                let mut iTemp994: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp993, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp995: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp994, 7)) as usize] };
                let mut fTemp996: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp994 as usize] };
                let mut fTemp997: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp994, 1)) as usize] }
                        - fTemp996;
                let mut fTemp998: F64 = (if iTemp724 != 0 {
                    fTemp996
                        + fTemp733 * fTemp997
                        + (fTemp992 - (iTemp993) as F64)
                            * (fTemp995
                                - (fTemp996
                                    + fTemp733
                                        * (fTemp997
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp994, 8)) as usize]
                                            } - fTemp995))))
                } else {
                    1.0 - (fTemp990
                        + fTemp733 * fTemp991
                        + (fTemp986 - (iTemp987) as F64)
                            * (fTemp989
                                - (fTemp990
                                    + fTemp733
                                        * (fTemp991
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp988, 8)) as usize]
                                            } - fTemp989)))))
                });
                let mut fTemp999: F64 = fTemp738 + fTemp985;
                let mut fTemp1000: F64 = 262143.0 * (1.0 - fTemp999);
                let mut iTemp1001: i32 = (fTemp1000) as i32;
                let mut iTemp1002: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1001, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1003: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1002, 7)) as usize] };
                let mut fTemp1004: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1002 as usize] };
                let mut fTemp1005: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1002, 1)) as usize] }
                        - fTemp1004;
                let mut fTemp1006: F64 = 262143.0 * fTemp999;
                let mut iTemp1007: i32 = (fTemp1006) as i32;
                let mut iTemp1008: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1007, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1009: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1008, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp1010: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1008 as usize] };
                let mut fTemp1011: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1008, 1), 1835007),
                    )) as usize]
                } - fTemp1010;
                let mut iTemp1012: i32 = (fTemp794
                    > (((if iTemp724 != 0 {
                        fTemp1010
                            + fTemp733 * fTemp1011
                            + (fTemp1006 - (iTemp1007) as F64)
                                * (fTemp1009
                                    - (fTemp1010
                                        + fTemp733
                                            * (fTemp1011
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1008, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1009))))
                    } else {
                        1.0 - (fTemp1004
                            + fTemp733 * fTemp1005
                            + (fTemp1000 - (iTemp1001) as F64)
                                * (fTemp1003
                                    - (fTemp1004
                                        + fTemp733
                                            * (fTemp1005
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp1002, 8)) as usize]
                                                } - fTemp1003)))))
                    }) - fTemp998)
                        / (1.0 - fTemp998))) as i32;
                let mut fTemp1013: F64 = (if iTemp1012 != 0 { fTemp982 } else { fTemp985 });
                let mut fTemp1014: F64 = (if iTemp1012 != 0 { fTemp985 } else { fTemp983 });
                let mut fTemp1015: F64 = fTemp1014 + fTemp1013;
                let mut fTemp1016: F64 = 0.5 * fTemp1015;
                let mut fTemp1017: F64 = 262143.0 * (1.0 - fTemp1016);
                let mut iTemp1018: i32 = (fTemp1017) as i32;
                let mut iTemp1019: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1018, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1020: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1019, 7)) as usize] };
                let mut fTemp1021: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1019 as usize] };
                let mut fTemp1022: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1019, 1)) as usize] }
                        - fTemp1021;
                let mut fTemp1023: F64 = 131071.5 * fTemp1015;
                let mut iTemp1024: i32 = (fTemp1023) as i32;
                let mut iTemp1025: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1024, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1026: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1025, 7)) as usize] };
                let mut fTemp1027: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1025 as usize] };
                let mut fTemp1028: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1025, 1)) as usize] }
                        - fTemp1027;
                let mut fTemp1029: F64 = (if iTemp724 != 0 {
                    fTemp1027
                        + fTemp733 * fTemp1028
                        + (fTemp1023 - (iTemp1024) as F64)
                            * (fTemp1026
                                - (fTemp1027
                                    + fTemp733
                                        * (fTemp1028
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp1025, 8)) as usize]
                                            } - fTemp1026))))
                } else {
                    1.0 - (fTemp1021
                        + fTemp733 * fTemp1022
                        + (fTemp1017 - (iTemp1018) as F64)
                            * (fTemp1020
                                - (fTemp1021
                                    + fTemp733
                                        * (fTemp1022
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp1019, 8)) as usize]
                                            } - fTemp1020)))))
                });
                let mut fTemp1030: F64 = fTemp738 + fTemp1016;
                let mut fTemp1031: F64 = 262143.0 * (1.0 - fTemp1030);
                let mut iTemp1032: i32 = (fTemp1031) as i32;
                let mut iTemp1033: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1032, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1034: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1033, 7)) as usize] };
                let mut fTemp1035: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1033 as usize] };
                let mut fTemp1036: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1033, 1)) as usize] }
                        - fTemp1035;
                let mut fTemp1037: F64 = 262143.0 * fTemp1030;
                let mut iTemp1038: i32 = (fTemp1037) as i32;
                let mut iTemp1039: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1038, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1040: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1039, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp1041: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1039 as usize] };
                let mut fTemp1042: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1039, 1), 1835007),
                    )) as usize]
                } - fTemp1041;
                let mut iTemp1043: i32 = (fTemp794
                    > (((if iTemp724 != 0 {
                        fTemp1041
                            + fTemp733 * fTemp1042
                            + (fTemp1037 - (iTemp1038) as F64)
                                * (fTemp1040
                                    - (fTemp1041
                                        + fTemp733
                                            * (fTemp1042
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1039, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1040))))
                    } else {
                        1.0 - (fTemp1035
                            + fTemp733 * fTemp1036
                            + (fTemp1031 - (iTemp1032) as F64)
                                * (fTemp1034
                                    - (fTemp1035
                                        + fTemp733
                                            * (fTemp1036
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp1033, 8)) as usize]
                                                } - fTemp1034)))))
                    }) - fTemp1029)
                        / (1.0 - fTemp1029))) as i32;
                let mut fTemp1044: F64 = (if iTemp1043 != 0 { fTemp1013 } else { fTemp1016 });
                let mut fTemp1045: F64 = (if iTemp1043 != 0 { fTemp1016 } else { fTemp1014 });
                let mut fTemp1046: F64 = fTemp1045 + fTemp1044;
                let mut fTemp1047: F64 = 0.5 * fTemp1046;
                let mut fTemp1048: F64 = 262143.0 * (1.0 - fTemp1047);
                let mut iTemp1049: i32 = (fTemp1048) as i32;
                let mut iTemp1050: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1049, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1051: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1050, 7)) as usize] };
                let mut fTemp1052: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1050 as usize] };
                let mut fTemp1053: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1050, 1)) as usize] }
                        - fTemp1052;
                let mut fTemp1054: F64 = 131071.5 * fTemp1046;
                let mut iTemp1055: i32 = (fTemp1054) as i32;
                let mut iTemp1056: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1055, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1057: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1056, 7)) as usize] };
                let mut fTemp1058: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1056 as usize] };
                let mut fTemp1059: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1056, 1)) as usize] }
                        - fTemp1058;
                let mut fTemp1060: F64 = (if iTemp724 != 0 {
                    fTemp1058
                        + fTemp733 * fTemp1059
                        + (fTemp1054 - (iTemp1055) as F64)
                            * (fTemp1057
                                - (fTemp1058
                                    + fTemp733
                                        * (fTemp1059
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp1056, 8)) as usize]
                                            } - fTemp1057))))
                } else {
                    1.0 - (fTemp1052
                        + fTemp733 * fTemp1053
                        + (fTemp1048 - (iTemp1049) as F64)
                            * (fTemp1051
                                - (fTemp1052
                                    + fTemp733
                                        * (fTemp1053
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp1050, 8)) as usize]
                                            } - fTemp1051)))))
                });
                let mut fTemp1061: F64 = fTemp738 + fTemp1047;
                let mut fTemp1062: F64 = 262143.0 * (1.0 - fTemp1061);
                let mut iTemp1063: i32 = (fTemp1062) as i32;
                let mut iTemp1064: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1063, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1065: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1064, 7)) as usize] };
                let mut fTemp1066: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1064 as usize] };
                let mut fTemp1067: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1064, 1)) as usize] }
                        - fTemp1066;
                let mut fTemp1068: F64 = 262143.0 * fTemp1061;
                let mut iTemp1069: i32 = (fTemp1068) as i32;
                let mut iTemp1070: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1069, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1071: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1070, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp1072: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1070 as usize] };
                let mut fTemp1073: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1070, 1), 1835007),
                    )) as usize]
                } - fTemp1072;
                let mut iTemp1074: i32 = (fTemp794
                    > (((if iTemp724 != 0 {
                        fTemp1072
                            + fTemp733 * fTemp1073
                            + (fTemp1068 - (iTemp1069) as F64)
                                * (fTemp1071
                                    - (fTemp1072
                                        + fTemp733
                                            * (fTemp1073
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1070, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1071))))
                    } else {
                        1.0 - (fTemp1066
                            + fTemp733 * fTemp1067
                            + (fTemp1062 - (iTemp1063) as F64)
                                * (fTemp1065
                                    - (fTemp1066
                                        + fTemp733
                                            * (fTemp1067
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp1064, 8)) as usize]
                                                } - fTemp1065)))))
                    }) - fTemp1060)
                        / (1.0 - fTemp1060))) as i32;
                let mut fTemp1075: F64 = (if iTemp1074 != 0 { fTemp1044 } else { fTemp1047 });
                let mut fTemp1076: F64 = (if iTemp1074 != 0 { fTemp1047 } else { fTemp1045 });
                let mut fTemp1077: F64 = fTemp1076 + fTemp1075;
                let mut fTemp1078: F64 = 0.5 * fTemp1077;
                let mut fTemp1079: F64 = 262143.0 * (1.0 - fTemp1078);
                let mut iTemp1080: i32 = (fTemp1079) as i32;
                let mut iTemp1081: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1080, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1082: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1081, 7)) as usize] };
                let mut fTemp1083: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1081 as usize] };
                let mut fTemp1084: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1081, 1)) as usize] }
                        - fTemp1083;
                let mut fTemp1085: F64 = 131071.5 * fTemp1077;
                let mut iTemp1086: i32 = (fTemp1085) as i32;
                let mut iTemp1087: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1086, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1088: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1087, 7)) as usize] };
                let mut fTemp1089: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1087 as usize] };
                let mut fTemp1090: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1087, 1)) as usize] }
                        - fTemp1089;
                let mut fTemp1091: F64 = (if iTemp724 != 0 {
                    fTemp1089
                        + fTemp733 * fTemp1090
                        + (fTemp1085 - (iTemp1086) as F64)
                            * (fTemp1088
                                - (fTemp1089
                                    + fTemp733
                                        * (fTemp1090
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp1087, 8)) as usize]
                                            } - fTemp1088))))
                } else {
                    1.0 - (fTemp1083
                        + fTemp733 * fTemp1084
                        + (fTemp1079 - (iTemp1080) as F64)
                            * (fTemp1082
                                - (fTemp1083
                                    + fTemp733
                                        * (fTemp1084
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp1081, 8)) as usize]
                                            } - fTemp1082)))))
                });
                let mut fTemp1092: F64 = fTemp738 + fTemp1078;
                let mut fTemp1093: F64 = 262143.0 * (1.0 - fTemp1092);
                let mut iTemp1094: i32 = (fTemp1093) as i32;
                let mut iTemp1095: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1094, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1096: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1095, 7)) as usize] };
                let mut fTemp1097: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1095 as usize] };
                let mut fTemp1098: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1095, 1)) as usize] }
                        - fTemp1097;
                let mut fTemp1099: F64 = 262143.0 * fTemp1092;
                let mut iTemp1100: i32 = (fTemp1099) as i32;
                let mut iTemp1101: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1100, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1102: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1101, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp1103: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1101 as usize] };
                let mut fTemp1104: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1101, 1), 1835007),
                    )) as usize]
                } - fTemp1103;
                let mut iTemp1105: i32 = (fTemp794
                    > (((if iTemp724 != 0 {
                        fTemp1103
                            + fTemp733 * fTemp1104
                            + (fTemp1099 - (iTemp1100) as F64)
                                * (fTemp1102
                                    - (fTemp1103
                                        + fTemp733
                                            * (fTemp1104
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1101, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1102))))
                    } else {
                        1.0 - (fTemp1097
                            + fTemp733 * fTemp1098
                            + (fTemp1093 - (iTemp1094) as F64)
                                * (fTemp1096
                                    - (fTemp1097
                                        + fTemp733
                                            * (fTemp1098
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp1095, 8)) as usize]
                                                } - fTemp1096)))))
                    }) - fTemp1091)
                        / (1.0 - fTemp1091))) as i32;
                let mut fTemp1106: F64 = (if iTemp1105 != 0 { fTemp1075 } else { fTemp1078 });
                let mut fTemp1107: F64 = (if iTemp1105 != 0 { fTemp1078 } else { fTemp1076 });
                let mut fTemp1108: F64 = fTemp1107 + fTemp1106;
                let mut fTemp1109: F64 = 0.5 * fTemp1108;
                let mut fTemp1110: F64 = 262143.0 * (1.0 - fTemp1109);
                let mut iTemp1111: i32 = (fTemp1110) as i32;
                let mut iTemp1112: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1111, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1113: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1112, 7)) as usize] };
                let mut fTemp1114: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1112 as usize] };
                let mut fTemp1115: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1112, 1)) as usize] }
                        - fTemp1114;
                let mut fTemp1116: F64 = 131071.5 * fTemp1108;
                let mut iTemp1117: i32 = (fTemp1116) as i32;
                let mut iTemp1118: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1117, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1119: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1118, 7)) as usize] };
                let mut fTemp1120: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1118 as usize] };
                let mut fTemp1121: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1118, 1)) as usize] }
                        - fTemp1120;
                let mut fTemp1122: F64 = (if iTemp724 != 0 {
                    fTemp1120
                        + fTemp733 * fTemp1121
                        + (fTemp1116 - (iTemp1117) as F64)
                            * (fTemp1119
                                - (fTemp1120
                                    + fTemp733
                                        * (fTemp1121
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp1118, 8)) as usize]
                                            } - fTemp1119))))
                } else {
                    1.0 - (fTemp1114
                        + fTemp733 * fTemp1115
                        + (fTemp1110 - (iTemp1111) as F64)
                            * (fTemp1113
                                - (fTemp1114
                                    + fTemp733
                                        * (fTemp1115
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp1112, 8)) as usize]
                                            } - fTemp1113)))))
                });
                let mut fTemp1123: F64 = fTemp738 + fTemp1109;
                let mut fTemp1124: F64 = 262143.0 * (1.0 - fTemp1123);
                let mut iTemp1125: i32 = (fTemp1124) as i32;
                let mut iTemp1126: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1125, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1127: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1126, 7)) as usize] };
                let mut fTemp1128: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1126 as usize] };
                let mut fTemp1129: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1126, 1)) as usize] }
                        - fTemp1128;
                let mut fTemp1130: F64 = 262143.0 * fTemp1123;
                let mut iTemp1131: i32 = (fTemp1130) as i32;
                let mut iTemp1132: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1131, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1133: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1132, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp1134: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1132 as usize] };
                let mut fTemp1135: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1132, 1), 1835007),
                    )) as usize]
                } - fTemp1134;
                let mut iTemp1136: i32 = (fTemp794
                    > (((if iTemp724 != 0 {
                        fTemp1134
                            + fTemp733 * fTemp1135
                            + (fTemp1130 - (iTemp1131) as F64)
                                * (fTemp1133
                                    - (fTemp1134
                                        + fTemp733
                                            * (fTemp1135
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1132, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1133))))
                    } else {
                        1.0 - (fTemp1128
                            + fTemp733 * fTemp1129
                            + (fTemp1124 - (iTemp1125) as F64)
                                * (fTemp1127
                                    - (fTemp1128
                                        + fTemp733
                                            * (fTemp1129
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp1126, 8)) as usize]
                                                } - fTemp1127)))))
                    }) - fTemp1122)
                        / (1.0 - fTemp1122))) as i32;
                let mut fTemp1137: F64 = (if iTemp1136 != 0 { fTemp1106 } else { fTemp1109 });
                let mut fTemp1138: F64 = (if iTemp1136 != 0 { fTemp1109 } else { fTemp1107 });
                let mut fTemp1139: F64 = fTemp1138 + fTemp1137;
                let mut fTemp1140: F64 = 0.5 * fTemp1139;
                let mut fTemp1141: F64 = 262143.0 * (1.0 - fTemp1140);
                let mut iTemp1142: i32 = (fTemp1141) as i32;
                let mut iTemp1143: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1142, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1144: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1143, 7)) as usize] };
                let mut fTemp1145: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1143 as usize] };
                let mut fTemp1146: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1143, 1)) as usize] }
                        - fTemp1145;
                let mut fTemp1147: F64 = 131071.5 * fTemp1139;
                let mut iTemp1148: i32 = (fTemp1147) as i32;
                let mut iTemp1149: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1148, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1150: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1149, 7)) as usize] };
                let mut fTemp1151: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1149 as usize] };
                let mut fTemp1152: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1149, 1)) as usize] }
                        - fTemp1151;
                let mut fTemp1153: F64 = (if iTemp724 != 0 {
                    fTemp1151
                        + fTemp733 * fTemp1152
                        + (fTemp1147 - (iTemp1148) as F64)
                            * (fTemp1150
                                - (fTemp1151
                                    + fTemp733
                                        * (fTemp1152
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp1149, 8)) as usize]
                                            } - fTemp1150))))
                } else {
                    1.0 - (fTemp1145
                        + fTemp733 * fTemp1146
                        + (fTemp1141 - (iTemp1142) as F64)
                            * (fTemp1144
                                - (fTemp1145
                                    + fTemp733
                                        * (fTemp1146
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp1143, 8)) as usize]
                                            } - fTemp1144)))))
                });
                let mut fTemp1154: F64 = fTemp738 + fTemp1140;
                let mut fTemp1155: F64 = 262143.0 * (1.0 - fTemp1154);
                let mut iTemp1156: i32 = (fTemp1155) as i32;
                let mut iTemp1157: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1156, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1158: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1157, 7)) as usize] };
                let mut fTemp1159: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1157 as usize] };
                let mut fTemp1160: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1157, 1)) as usize] }
                        - fTemp1159;
                let mut fTemp1161: F64 = 262143.0 * fTemp1154;
                let mut iTemp1162: i32 = (fTemp1161) as i32;
                let mut iTemp1163: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1162, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1164: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1163, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp1165: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1163 as usize] };
                let mut fTemp1166: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1163, 1), 1835007),
                    )) as usize]
                } - fTemp1165;
                let mut iTemp1167: i32 = (fTemp794
                    > (((if iTemp724 != 0 {
                        fTemp1165
                            + fTemp733 * fTemp1166
                            + (fTemp1161 - (iTemp1162) as F64)
                                * (fTemp1164
                                    - (fTemp1165
                                        + fTemp733
                                            * (fTemp1166
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1163, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1164))))
                    } else {
                        1.0 - (fTemp1159
                            + fTemp733 * fTemp1160
                            + (fTemp1155 - (iTemp1156) as F64)
                                * (fTemp1158
                                    - (fTemp1159
                                        + fTemp733
                                            * (fTemp1160
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp1157, 8)) as usize]
                                                } - fTemp1158)))))
                    }) - fTemp1153)
                        / (1.0 - fTemp1153))) as i32;
                let mut fTemp1168: F64 = (if iTemp1167 != 0 { fTemp1137 } else { fTemp1140 });
                let mut fTemp1169: F64 = (if iTemp1167 != 0 { fTemp1140 } else { fTemp1138 });
                let mut fTemp1170: F64 = fTemp1169 + fTemp1168;
                let mut fTemp1171: F64 = 0.5 * fTemp1170;
                let mut fTemp1172: F64 = 262143.0 * (1.0 - fTemp1171);
                let mut iTemp1173: i32 = (fTemp1172) as i32;
                let mut iTemp1174: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1173, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1175: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1174, 7)) as usize] };
                let mut fTemp1176: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1174 as usize] };
                let mut fTemp1177: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1174, 1)) as usize] }
                        - fTemp1176;
                let mut fTemp1178: F64 = 131071.5 * fTemp1170;
                let mut iTemp1179: i32 = (fTemp1178) as i32;
                let mut iTemp1180: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1179, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1181: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1180, 7)) as usize] };
                let mut fTemp1182: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1180 as usize] };
                let mut fTemp1183: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1180, 1)) as usize] }
                        - fTemp1182;
                let mut fTemp1184: F64 = (if iTemp724 != 0 {
                    fTemp1182
                        + fTemp733 * fTemp1183
                        + (fTemp1178 - (iTemp1179) as F64)
                            * (fTemp1181
                                - (fTemp1182
                                    + fTemp733
                                        * (fTemp1183
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp1180, 8)) as usize]
                                            } - fTemp1181))))
                } else {
                    1.0 - (fTemp1176
                        + fTemp733 * fTemp1177
                        + (fTemp1172 - (iTemp1173) as F64)
                            * (fTemp1175
                                - (fTemp1176
                                    + fTemp733
                                        * (fTemp1177
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp1174, 8)) as usize]
                                            } - fTemp1175)))))
                });
                let mut fTemp1185: F64 = fTemp738 + fTemp1171;
                let mut fTemp1186: F64 = 262143.0 * (1.0 - fTemp1185);
                let mut iTemp1187: i32 = (fTemp1186) as i32;
                let mut iTemp1188: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1187, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1189: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1188, 7)) as usize] };
                let mut fTemp1190: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1188 as usize] };
                let mut fTemp1191: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1188, 1)) as usize] }
                        - fTemp1190;
                let mut fTemp1192: F64 = 262143.0 * fTemp1185;
                let mut iTemp1193: i32 = (fTemp1192) as i32;
                let mut iTemp1194: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1193, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1195: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1194, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp1196: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1194 as usize] };
                let mut fTemp1197: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1194, 1), 1835007),
                    )) as usize]
                } - fTemp1196;
                let mut iTemp1198: i32 = (fTemp794
                    > (((if iTemp724 != 0 {
                        fTemp1196
                            + fTemp733 * fTemp1197
                            + (fTemp1192 - (iTemp1193) as F64)
                                * (fTemp1195
                                    - (fTemp1196
                                        + fTemp733
                                            * (fTemp1197
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1194, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1195))))
                    } else {
                        1.0 - (fTemp1190
                            + fTemp733 * fTemp1191
                            + (fTemp1186 - (iTemp1187) as F64)
                                * (fTemp1189
                                    - (fTemp1190
                                        + fTemp733
                                            * (fTemp1191
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp1188, 8)) as usize]
                                                } - fTemp1189)))))
                    }) - fTemp1184)
                        / (1.0 - fTemp1184))) as i32;
                let mut fTemp1199: F64 = (if iTemp1198 != 0 { fTemp1168 } else { fTemp1171 });
                let mut fTemp1200: F64 = (if iTemp1198 != 0 { fTemp1171 } else { fTemp1169 });
                let mut fTemp1201: F64 = fTemp1200 + fTemp1199;
                let mut fTemp1202: F64 = 0.5 * fTemp1201;
                let mut fTemp1203: F64 = 262143.0 * (1.0 - fTemp1202);
                let mut iTemp1204: i32 = (fTemp1203) as i32;
                let mut iTemp1205: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1204, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1206: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1205, 7)) as usize] };
                let mut fTemp1207: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1205 as usize] };
                let mut fTemp1208: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1205, 1)) as usize] }
                        - fTemp1207;
                let mut fTemp1209: F64 = 131071.5 * fTemp1201;
                let mut iTemp1210: i32 = (fTemp1209) as i32;
                let mut iTemp1211: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1210, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1212: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1211, 7)) as usize] };
                let mut fTemp1213: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1211 as usize] };
                let mut fTemp1214: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1211, 1)) as usize] }
                        - fTemp1213;
                let mut fTemp1215: F64 = (if iTemp724 != 0 {
                    fTemp1213
                        + fTemp733 * fTemp1214
                        + (fTemp1209 - (iTemp1210) as F64)
                            * (fTemp1212
                                - (fTemp1213
                                    + fTemp733
                                        * (fTemp1214
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp1211, 8)) as usize]
                                            } - fTemp1212))))
                } else {
                    1.0 - (fTemp1207
                        + fTemp733 * fTemp1208
                        + (fTemp1203 - (iTemp1204) as F64)
                            * (fTemp1206
                                - (fTemp1207
                                    + fTemp733
                                        * (fTemp1208
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp1205, 8)) as usize]
                                            } - fTemp1206)))))
                });
                let mut fTemp1216: F64 = fTemp738 + fTemp1202;
                let mut fTemp1217: F64 = 262143.0 * (1.0 - fTemp1216);
                let mut iTemp1218: i32 = (fTemp1217) as i32;
                let mut iTemp1219: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1218, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1220: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1219, 7)) as usize] };
                let mut fTemp1221: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1219 as usize] };
                let mut fTemp1222: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1219, 1)) as usize] }
                        - fTemp1221;
                let mut fTemp1223: F64 = 262143.0 * fTemp1216;
                let mut iTemp1224: i32 = (fTemp1223) as i32;
                let mut iTemp1225: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1224, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1226: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1225, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp1227: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1225 as usize] };
                let mut fTemp1228: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1225, 1), 1835007),
                    )) as usize]
                } - fTemp1227;
                let mut iTemp1229: i32 = (fTemp794
                    > (((if iTemp724 != 0 {
                        fTemp1227
                            + fTemp733 * fTemp1228
                            + (fTemp1223 - (iTemp1224) as F64)
                                * (fTemp1226
                                    - (fTemp1227
                                        + fTemp733
                                            * (fTemp1228
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1225, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1226))))
                    } else {
                        1.0 - (fTemp1221
                            + fTemp733 * fTemp1222
                            + (fTemp1217 - (iTemp1218) as F64)
                                * (fTemp1220
                                    - (fTemp1221
                                        + fTemp733
                                            * (fTemp1222
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp1219, 8)) as usize]
                                                } - fTemp1220)))))
                    }) - fTemp1215)
                        / (1.0 - fTemp1215))) as i32;
                let mut fTemp1230: F64 = (if iTemp1229 != 0 { fTemp1199 } else { fTemp1202 });
                let mut fTemp1231: F64 = (if iTemp1229 != 0 { fTemp1202 } else { fTemp1200 });
                let mut fTemp1232: F64 = fTemp1231 + fTemp1230;
                let mut fTemp1233: F64 = 0.5 * fTemp1232;
                let mut fTemp1234: F64 = 262143.0 * (1.0 - fTemp1233);
                let mut iTemp1235: i32 = (fTemp1234) as i32;
                let mut iTemp1236: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1235, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1237: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1236, 7)) as usize] };
                let mut fTemp1238: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1236 as usize] };
                let mut fTemp1239: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1236, 1)) as usize] }
                        - fTemp1238;
                let mut fTemp1240: F64 = 131071.5 * fTemp1232;
                let mut iTemp1241: i32 = (fTemp1240) as i32;
                let mut iTemp1242: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1241, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1243: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1242, 7)) as usize] };
                let mut fTemp1244: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1242 as usize] };
                let mut fTemp1245: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1242, 1)) as usize] }
                        - fTemp1244;
                let mut fTemp1246: F64 = (if iTemp724 != 0 {
                    fTemp1244
                        + fTemp733 * fTemp1245
                        + (fTemp1240 - (iTemp1241) as F64)
                            * (fTemp1243
                                - (fTemp1244
                                    + fTemp733
                                        * (fTemp1245
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp1242, 8)) as usize]
                                            } - fTemp1243))))
                } else {
                    1.0 - (fTemp1238
                        + fTemp733 * fTemp1239
                        + (fTemp1234 - (iTemp1235) as F64)
                            * (fTemp1237
                                - (fTemp1238
                                    + fTemp733
                                        * (fTemp1239
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp1236, 8)) as usize]
                                            } - fTemp1237)))))
                });
                let mut fTemp1247: F64 = fTemp738 + fTemp1233;
                let mut fTemp1248: F64 = 262143.0 * (1.0 - fTemp1247);
                let mut iTemp1249: i32 = (fTemp1248) as i32;
                let mut iTemp1250: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1249, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1251: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1250, 7)) as usize] };
                let mut fTemp1252: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1250 as usize] };
                let mut fTemp1253: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1250, 1)) as usize] }
                        - fTemp1252;
                let mut fTemp1254: F64 = 262143.0 * fTemp1247;
                let mut iTemp1255: i32 = (fTemp1254) as i32;
                let mut iTemp1256: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1255, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1257: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1256, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp1258: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1256 as usize] };
                let mut fTemp1259: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1256, 1), 1835007),
                    )) as usize]
                } - fTemp1258;
                let mut iTemp1260: i32 = (fTemp794
                    > (((if iTemp724 != 0 {
                        fTemp1258
                            + fTemp733 * fTemp1259
                            + (fTemp1254 - (iTemp1255) as F64)
                                * (fTemp1257
                                    - (fTemp1258
                                        + fTemp733
                                            * (fTemp1259
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1256, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1257))))
                    } else {
                        1.0 - (fTemp1252
                            + fTemp733 * fTemp1253
                            + (fTemp1248 - (iTemp1249) as F64)
                                * (fTemp1251
                                    - (fTemp1252
                                        + fTemp733
                                            * (fTemp1253
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp1250, 8)) as usize]
                                                } - fTemp1251)))))
                    }) - fTemp1246)
                        / (1.0 - fTemp1246))) as i32;
                let mut fTemp1261: F64 = (if iTemp1260 != 0 { fTemp1230 } else { fTemp1233 });
                let mut fTemp1262: F64 = (if iTemp1260 != 0 { fTemp1233 } else { fTemp1231 });
                let mut fTemp1263: F64 = fTemp1262 + fTemp1261;
                let mut fTemp1264: F64 = 0.5 * fTemp1263;
                let mut fTemp1265: F64 = 262143.0 * (1.0 - fTemp1264);
                let mut iTemp1266: i32 = (fTemp1265) as i32;
                let mut iTemp1267: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1266, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1268: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1267, 7)) as usize] };
                let mut fTemp1269: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1267 as usize] };
                let mut fTemp1270: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1267, 1)) as usize] }
                        - fTemp1269;
                let mut fTemp1271: F64 = 131071.5 * fTemp1263;
                let mut iTemp1272: i32 = (fTemp1271) as i32;
                let mut iTemp1273: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1272, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1274: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1273, 7)) as usize] };
                let mut fTemp1275: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1273 as usize] };
                let mut fTemp1276: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1273, 1)) as usize] }
                        - fTemp1275;
                let mut fTemp1277: F64 = (if iTemp724 != 0 {
                    fTemp1275
                        + fTemp733 * fTemp1276
                        + (fTemp1271 - (iTemp1272) as F64)
                            * (fTemp1274
                                - (fTemp1275
                                    + fTemp733
                                        * (fTemp1276
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp1273, 8)) as usize]
                                            } - fTemp1274))))
                } else {
                    1.0 - (fTemp1269
                        + fTemp733 * fTemp1270
                        + (fTemp1265 - (iTemp1266) as F64)
                            * (fTemp1268
                                - (fTemp1269
                                    + fTemp733
                                        * (fTemp1270
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp1267, 8)) as usize]
                                            } - fTemp1268)))))
                });
                let mut fTemp1278: F64 = fTemp738 + fTemp1264;
                let mut fTemp1279: F64 = 262143.0 * (1.0 - fTemp1278);
                let mut iTemp1280: i32 = (fTemp1279) as i32;
                let mut iTemp1281: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1280, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1282: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1281, 7)) as usize] };
                let mut fTemp1283: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1281 as usize] };
                let mut fTemp1284: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1281, 1)) as usize] }
                        - fTemp1283;
                let mut fTemp1285: F64 = 262143.0 * fTemp1278;
                let mut iTemp1286: i32 = (fTemp1285) as i32;
                let mut iTemp1287: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1286, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1288: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1287, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp1289: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1287 as usize] };
                let mut fTemp1290: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1287, 1), 1835007),
                    )) as usize]
                } - fTemp1289;
                let mut iTemp1291: i32 = (fTemp794
                    > (((if iTemp724 != 0 {
                        fTemp1289
                            + fTemp733 * fTemp1290
                            + (fTemp1285 - (iTemp1286) as F64)
                                * (fTemp1288
                                    - (fTemp1289
                                        + fTemp733
                                            * (fTemp1290
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1287, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1288))))
                    } else {
                        1.0 - (fTemp1283
                            + fTemp733 * fTemp1284
                            + (fTemp1279 - (iTemp1280) as F64)
                                * (fTemp1282
                                    - (fTemp1283
                                        + fTemp733
                                            * (fTemp1284
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp1281, 8)) as usize]
                                                } - fTemp1282)))))
                    }) - fTemp1277)
                        / (1.0 - fTemp1277))) as i32;
                let mut fTemp1292: F64 = (if iTemp1291 != 0 { fTemp1261 } else { fTemp1264 });
                let mut fTemp1293: F64 = (if iTemp1291 != 0 { fTemp1264 } else { fTemp1262 });
                let mut fTemp1294: F64 = fTemp1293 + fTemp1292;
                let mut fTemp1295: F64 = 0.5 * fTemp1294;
                let mut fTemp1296: F64 = 262143.0 * (1.0 - fTemp1295);
                let mut iTemp1297: i32 = (fTemp1296) as i32;
                let mut iTemp1298: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1297, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1299: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1298, 7)) as usize] };
                let mut fTemp1300: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1298 as usize] };
                let mut fTemp1301: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1298, 1)) as usize] }
                        - fTemp1300;
                let mut fTemp1302: F64 = 131071.5 * fTemp1294;
                let mut iTemp1303: i32 = (fTemp1302) as i32;
                let mut iTemp1304: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1303, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1305: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1304, 7)) as usize] };
                let mut fTemp1306: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1304 as usize] };
                let mut fTemp1307: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1304, 1)) as usize] }
                        - fTemp1306;
                let mut fTemp1308: F64 = (if iTemp724 != 0 {
                    fTemp1306
                        + fTemp733 * fTemp1307
                        + (fTemp1302 - (iTemp1303) as F64)
                            * (fTemp1305
                                - (fTemp1306
                                    + fTemp733
                                        * (fTemp1307
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp1304, 8),
                                                        1835007,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp1305))))
                } else {
                    1.0 - (fTemp1300
                        + fTemp733 * fTemp1301
                        + (fTemp1296 - (iTemp1297) as F64)
                            * (fTemp1299
                                - (fTemp1300
                                    + fTemp733
                                        * (fTemp1301
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp1298, 8),
                                                        1835007,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp1299)))))
                });
                let mut fTemp1309: F64 = fTemp738 + fTemp1295;
                let mut fTemp1310: F64 = 262143.0 * (1.0 - fTemp1309);
                let mut iTemp1311: i32 = (fTemp1310) as i32;
                let mut iTemp1312: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1311, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1313: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1312, 7)) as usize] };
                let mut fTemp1314: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1312 as usize] };
                let mut fTemp1315: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1312, 1)) as usize] }
                        - fTemp1314;
                let mut fTemp1316: F64 = 262143.0 * fTemp1309;
                let mut iTemp1317: i32 = (fTemp1316) as i32;
                let mut iTemp1318: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1317, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1319: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1318, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp1320: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1318 as usize] };
                let mut fTemp1321: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1318, 1), 1835007),
                    )) as usize]
                } - fTemp1320;
                let mut iTemp1322: i32 = (fTemp794
                    > (((if iTemp724 != 0 {
                        fTemp1320
                            + fTemp733 * fTemp1321
                            + (fTemp1316 - (iTemp1317) as F64)
                                * (fTemp1319
                                    - (fTemp1320
                                        + fTemp733
                                            * (fTemp1321
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1318, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1319))))
                    } else {
                        1.0 - (fTemp1314
                            + fTemp733 * fTemp1315
                            + (fTemp1310 - (iTemp1311) as F64)
                                * (fTemp1313
                                    - (fTemp1314
                                        + fTemp733
                                            * (fTemp1315
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp1312, 8)) as usize]
                                                } - fTemp1313)))))
                    }) - fTemp1308)
                        / (1.0 - fTemp1308))) as i32;
                let mut fTemp1323: F64 = F64::min(
                    1.0,
                    F64::max(
                        0.0,
                        0.5 * ((if iTemp1322 != 0 { fTemp1295 } else { fTemp1293 })
                            + (if iTemp1322 != 0 { fTemp1292 } else { fTemp1295 })),
                    ),
                );
                self.fRec15[0] = fTemp1323;
                let mut fTemp1324: F64 = 262143.0 * (1.0 - fTemp1323);
                let mut iTemp1325: i32 = (fTemp1324) as i32;
                let mut iTemp1326: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1325, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1327: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1326, 7)) as usize] };
                let mut fTemp1328: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1326 as usize] };
                let mut fTemp1329: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1326, 1)) as usize] }
                        - fTemp1328;
                let mut fTemp1330: F64 = 262143.0 * fTemp1323;
                let mut iTemp1331: i32 = (fTemp1330) as i32;
                let mut iTemp1332: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1331, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1333: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1332, 7)) as usize] };
                let mut fTemp1334: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1332 as usize] };
                let mut fTemp1335: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1332, 1)) as usize] }
                        - fTemp1334;
                let mut fTemp1336: F64 = (if iTemp724 != 0 {
                    fTemp1334
                        + fTemp733 * fTemp1335
                        + (fTemp1330 - (iTemp1331) as F64)
                            * (fTemp1333
                                - (fTemp1334
                                    + fTemp733
                                        * (fTemp1335
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp1332, 8),
                                                        1835007,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp1333))))
                } else {
                    1.0 - (fTemp1328
                        + fTemp733 * fTemp1329
                        + (fTemp1324 - (iTemp1325) as F64)
                            * (fTemp1327
                                - (fTemp1328
                                    + fTemp733
                                        * (fTemp1329
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp1326, 8),
                                                        1835007,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp1327)))))
                });
                let mut fTemp1337: F64 = fTemp738 + fTemp1323;
                let mut fTemp1338: F64 = 262143.0 * (1.0 - fTemp1337);
                let mut iTemp1339: i32 = (fTemp1338) as i32;
                let mut iTemp1340: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1339, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1341: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1340, 7)) as usize] };
                let mut fTemp1342: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1340 as usize] };
                let mut fTemp1343: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1340, 1)) as usize] }
                        - fTemp1342;
                let mut fTemp1344: F64 = 262143.0 * fTemp1337;
                let mut iTemp1345: i32 = (fTemp1344) as i32;
                let mut iTemp1346: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp728,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1345, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1347: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1346, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp1348: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1346 as usize] };
                let mut fTemp1349: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1346, 1), 1835007),
                    )) as usize]
                } - fTemp1348;
                let mut fTemp1350: F64 = self.fRec16[1]
                    + (if ((0.001 * fTemp737) == 0.0) as i32 != 0 {
                        fTemp723
                    } else {
                        fTemp723
                            * ((if iTemp724 != 0 {
                                fTemp1348
                                    + fTemp733 * fTemp1349
                                    + (fTemp1344 - (iTemp1345) as F64)
                                        * (fTemp1347
                                            - (fTemp1348
                                                + fTemp733
                                                    * (fTemp1349
                                                        - (unsafe {
                                                            ftbl0LambRs96kSIG0[(std::cmp::max(
                                                                0,
                                                                std::cmp::min(
                                                                    i32::wrapping_add(iTemp1346, 8),
                                                                    1835007,
                                                                ),
                                                            ))
                                                                as usize]
                                                        } - fTemp1347))))
                            } else {
                                1.0 - (fTemp1342
                                    + fTemp733 * fTemp1343
                                    + (fTemp1338 - (iTemp1339) as F64)
                                        * (fTemp1341
                                            - (fTemp1342
                                                + fTemp733
                                                    * (fTemp1343
                                                        - (unsafe {
                                                            ftbl0LambRs96kSIG0[(i32::wrapping_add(
                                                                iTemp1340, 8,
                                                            ))
                                                                as usize]
                                                        } - fTemp1341)))))
                            }) - fTemp1336)
                            / (1.0 - fTemp1336)
                    });
                self.fRec16[0] = (if iTemp736 != 0 {
                    F64::min(fTemp1350, self.fRec16[1])
                } else {
                    F64::max(fTemp1350, self.fRec16[1])
                });
                self.fVec62[(self.IOTA0 & 16383) as usize] =
                    F64::powf(1e+01, 0.05 * self.fRec16[0]);
                let mut fTemp1351: F64 =
                    self.fVec62[((i32::wrapping_sub(self.IOTA0, iSlow74)) & 16383) as usize];
                *output1 = 0.5
                    * fTemp2
                    * self.fVec3[((i32::wrapping_sub(self.IOTA0, iSlow75)) & 32767) as usize]
                    + self.fRec14[0]
                        * fTemp4
                        * self.fVec4[((i32::wrapping_sub(self.IOTA0, iSlow75)) & 32767) as usize]
                        * fTemp1351;
                *output2 = fTemp3 + fTemp696 * fTemp4;
                *output3 = fTemp3 + fTemp4 * fTemp1351;
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
                self.fVec18[2] = self.fVec18[1];
                self.fVec18[1] = self.fVec18[0];
                for j2 in (1..=4).rev() {
                    self.fVec19[j2 as usize] = self.fVec19[(i32::wrapping_sub(j2, 1)) as usize];
                }
                for j3 in (1..=11).rev() {
                    self.fVec20[j3 as usize] = self.fVec20[(i32::wrapping_sub(j3, 1)) as usize];
                }
                self.fVec30[1] = self.fVec30[0];
                self.fVec31[1] = self.fVec31[0];
                self.fVec32[1] = self.fVec32[0];
                self.fRec1[1] = self.fRec1[0];
                self.fRec2[1] = self.fRec2[0];
                self.fRec14[1] = self.fRec14[0];
                self.fVec35[2] = self.fVec35[1];
                self.fVec35[1] = self.fVec35[0];
                for j4 in (1..=4).rev() {
                    self.fVec36[j4 as usize] = self.fVec36[(i32::wrapping_sub(j4, 1)) as usize];
                }
                for j5 in (1..=11).rev() {
                    self.fVec37[j5 as usize] = self.fVec37[(i32::wrapping_sub(j5, 1)) as usize];
                }
                self.fRec17[1] = self.fRec17[0];
                self.fVec47[2] = self.fVec47[1];
                self.fVec47[1] = self.fVec47[0];
                for j6 in (1..=4).rev() {
                    self.fVec48[j6 as usize] = self.fVec48[(i32::wrapping_sub(j6, 1)) as usize];
                }
                for j7 in (1..=11).rev() {
                    self.fVec49[j7 as usize] = self.fVec49[(i32::wrapping_sub(j7, 1)) as usize];
                }
                self.fVec59[1] = self.fVec59[0];
                self.fVec60[1] = self.fVec60[0];
                self.fVec61[1] = self.fVec61[0];
                self.fRec15[1] = self.fRec15[0];
                self.fRec16[1] = self.fRec16[0];
            }
        }
    }

    impl FaustDsp for LambRs96k {
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

pub use dsp_96k::LambRs96k;
