/* ------------------------------------------------------------
author: "Bart Brouns"
license: "AGPLv3"
name: "lamb-rs"
version: "0.1"
Code generated with Faust 2.72.14 (https://faust.grame.fr)
Compilation options: -a /run/user/1001/.tmpHjcpph -lang rust -ct 1 -cn LambRs96k -es 1 -mcd 16 -mdd 1024 -mdy 33 -double -ftz 0
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

        fn instance_initLambRs96kSIG0(&mut self, sample_rate: i32) {
            for l42 in 0..2 {
                self.iRec13[l42 as usize] = 0;
            }
        }

        fn fillLambRs96kSIG0(&mut self, count: i32, table: &mut [F64]) {
            for i1 in 0..count {
                self.iRec13[0] = i32::wrapping_add(self.iRec13[1], 1);
                let mut iTemp66: i32 = i32::wrapping_add(self.iRec13[0], -1);
                let mut fTemp67: F64 = (iTemp66 % 7) as F64 as i32 as F64;
                let mut fTemp68: F64 = 0.16666666666666666 * fTemp67;
                let mut fTemp69: F64 = F64::powf(fTemp68, 0.06999999999999999 * fTemp67 + 1.0);
                let mut fTemp70: F64 =
                    (0.14285714285714285 * (iTemp66 % 1835008) as F64) as i32 as F64;
                table[i1 as usize] = F64::min(
                    1.0,
                    F64::max(
                        0.0,
                        if (fTemp68 == 0.0) as i32 != 0 {
                            0.5 * (F64::sin(1.1984270621720943e-05 * fTemp70 + 4.71238898038469)
                                + 1.0)
                        } else {
                            0.5 * (F64::sin(
                                3.141592653589793
                                    * ((1.0
                                        - F64::exp(-(9.231602598581689e-06 * fTemp69 * fTemp70)))
                                        / (1.0 - F64::exp(-(2.42 * fTemp69))))
                                    + 4.71238898038469,
                            ) + 1.0)
                        },
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
        fVec7: [F64; 7],
        fVec8: [F64; 15],
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
        fVec19: [F64; 7],
        fVec20: [F64; 15],
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
        fVec36: [F64; 7],
        fVec37: [F64; 15],
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
        fVec48: [F64; 7],
        fVec49: [F64; 15],
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

    impl FaustDsp for LambRs96k {
        type T = F64;

        fn new() -> LambRs96k {
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
                fVec7: [0.0; 7],
                fVec8: [0.0; 15],
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
                fVec19: [0.0; 7],
                fVec20: [0.0; 15],
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
                fVec36: [0.0; 7],
                fVec37: [0.0; 15],
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
                fVec48: [0.0; 7],
                fVec49: [0.0; 15],
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
        fn metadata(&self, m: &mut dyn Meta) {
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
            m.declare(
                "basics.lib/tabulateNd",
                r"Copyright (C) 2023 Bart Brouns <bart@magnetophon.nl>",
            );
            m.declare("basics.lib/tabulateNd:author", r"Bart Brouns");
            m.declare("basics.lib/tabulateNd:license", r"AGPL-3.0");
            m.declare("basics.lib/version", r"1.15.0");
            m.declare("compile_options", r"-a /run/user/1001/.tmpHjcpph -lang rust -ct 1 -cn LambRs96k -es 1 -mcd 16 -mdd 1024 -mdy 33 -double -ftz 0");
            m.declare("filename", r"lamb-rs-96k.dsp");
            m.declare(
                "interpolators.lib/interpolate_linear:author",
                r"Stéphane Letz",
            );
            m.declare("interpolators.lib/interpolate_linear:licence", r"MIT");
            m.declare("interpolators.lib/name", r"Faust Interpolator Library");
            m.declare("interpolators.lib/remap:author", r"David Braun");
            m.declare("interpolators.lib/version", r"1.3.1");
            m.declare("lamb.dsp/author", r"Bart Brouns");
            m.declare("lamb.dsp/license", r"AGPLv3");
            m.declare("lamb.dsp/name", r"lamb");
            m.declare("lamb.dsp/version", r"0.1");
            m.declare("license", r"AGPLv3");
            m.declare("maths.lib/author", r"GRAME");
            m.declare("maths.lib/copyright", r"GRAME");
            m.declare("maths.lib/license", r"LGPL with exception");
            m.declare("maths.lib/name", r"Faust Math Library");
            m.declare("maths.lib/version", r"2.8.0");
            m.declare("name", r"lamb-rs");
            m.declare("platform.lib/name", r"Generic Platform Library");
            m.declare("platform.lib/version", r"1.3.0");
            m.declare("routes.lib/name", r"Faust Signal Routing Library");
            m.declare("routes.lib/version", r"1.2.0");
            m.declare("signals.lib/name", r"Faust Signal Routing Library");
            m.declare("signals.lib/version", r"1.5.0");
            m.declare("version", r"0.1");
        }

        fn get_sample_rate(&self) -> i32 {
            return self.fSampleRate;
        }
        fn get_num_inputs(&self) -> i32 {
            return 2;
        }
        fn get_num_outputs(&self) -> i32 {
            return 4;
        }

        fn class_init(sample_rate: i32) {
            let mut sig0: LambRs96kSIG0 = newLambRs96kSIG0();
            sig0.instance_initLambRs96kSIG0(sample_rate);
            sig0.fillLambRs96kSIG0(1835008, unsafe { &mut ftbl0LambRs96kSIG0 });
        }
        fn instance_reset_params(&mut self) {
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
        fn instance_clear(&mut self) {
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
            for l17 in 0..7 {
                self.fVec7[l17 as usize] = 0.0;
            }
            for l18 in 0..15 {
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
            for l30 in 0..7 {
                self.fVec19[l30 as usize] = 0.0;
            }
            for l31 in 0..15 {
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
            for l51 in 0..7 {
                self.fVec36[l51 as usize] = 0.0;
            }
            for l52 in 0..15 {
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
            for l64 in 0..7 {
                self.fVec48[l64 as usize] = 0.0;
            }
            for l65 in 0..15 {
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
        fn instance_constants(&mut self, sample_rate: i32) {
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
        fn instance_init(&mut self, sample_rate: i32) {
            self.instance_constants(sample_rate);
            self.instance_reset_params();
            self.instance_clear();
        }
        fn init(&mut self, sample_rate: i32) {
            LambRs96k::class_init(sample_rate);
            self.instance_init(sample_rate);
        }

        fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>) {
            Self::build_user_interface_static(ui_interface);
        }

        fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>) {
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

        fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
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

        fn set_param(&mut self, param: ParamIndex, value: Self::T) {
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

        fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut [&mut [Self::T]]) {
            let (inputs0, inputs1) = if let [inputs0, inputs1, ..] = inputs {
                let inputs0 = inputs0[..count as usize].iter();
                let inputs1 = inputs1[..count as usize].iter();
                (inputs0, inputs1)
            } else {
                panic!("wrong number of inputs");
            };
            let (outputs0, outputs1, outputs2, outputs3) =
                if let [outputs0, outputs1, outputs2, outputs3, ..] = outputs {
                    let outputs0 = outputs0[..count as usize].iter_mut();
                    let outputs1 = outputs1[..count as usize].iter_mut();
                    let outputs2 = outputs2[..count as usize].iter_mut();
                    let outputs3 = outputs3[..count as usize].iter_mut();
                    (outputs0, outputs1, outputs2, outputs3)
                } else {
                    panic!("wrong number of outputs");
                };
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
            self.fHbargraph0 = if (fSlow73) as i32 != 0 {
                9.6e+03
            } else {
                fSlow72
            };
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
                self.fRec0[0] = if (fTemp0 < fSlow0) as i32 != 0 {
                    fTemp0
                } else {
                    if (fTemp1 > fSlow0) as i32 != 0 {
                        fTemp1
                    } else {
                        fSlow0
                    }
                };
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
                            if (iTemp19 == 0) as i32 != 0 {
                                0.0
                            } else {
                                if (iTemp19 == 1) as i32 != 0 {
                                    fSlow12 * LambRs96k_faustpower2_f(fSlow7 + fTemp20)
                                } else {
                                    fTemp20
                                }
                            },
                        )),
                );
                let mut fTemp22: F64 = 3.0 * fTemp5;
                let mut fTemp23: F64 = 4.0 * (F64::max(0.25, self.fRec4[0]) + -0.25);
                let mut fTemp24: F64 = 9.0 - fTemp23;
                let mut fTemp25: F64 = self.fRec5[1] - self.fRec6[1];
                let mut fTemp26: F64 =
                    (self.iVec0[((i32::wrapping_sub(self.IOTA0, 9600)) & 16383) as usize]) as F64;
                let mut fTemp27: F64 = if (fTemp21 > self.fRec10[1]) as i32 != 0 {
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
                };
                self.fRec10[0] = self.fRec10[1] * fTemp27 + fTemp21 * (1.0 - fTemp27);
                let mut fTemp28: F64 = if (self.fRec10[0] > self.fRec9[1]) as i32 != 0 {
                    0.0
                } else {
                    self.fConst6
                };
                self.fRec9[0] = self.fRec9[1] * fTemp28 + self.fRec10[0] * (1.0 - fTemp28);
                let mut fTemp29: F64 = if (self.fRec9[0] > self.fRec8[1]) as i32 != 0 {
                    0.0
                } else {
                    self.fConst6
                };
                self.fRec8[0] = self.fRec8[1] * fTemp29 + self.fRec9[0] * (1.0 - fTemp29);
                let mut fTemp30: F64 = if (self.fRec8[0] > self.fRec7[1]) as i32 != 0 {
                    0.0
                } else {
                    self.fConst6
                };
                self.fRec7[0] = self.fRec7[1] * fTemp30 + self.fRec8[0] * (1.0 - fTemp30);
                self.fRec5[0] =
                    2e+01 * F64::log10(F64::max(2.2250738585072014e-308, self.fRec7[0]));
                let mut fTemp31: F64 = F64::powf(1e+01, 0.05 * (self.fRec5[1] + fTemp24));
                let mut fTemp32: F64 = if (fTemp31 > self.fRec12[1]) as i32 != 0 {
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
                };
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
                            if (iTemp39 == 0) as i32 != 0 {
                                0.0
                            } else {
                                if (iTemp39 == 1) as i32 != 0 {
                                    fSlow12 * LambRs96k_faustpower2_f(fSlow7 + fTemp40)
                                } else {
                                    fTemp40
                                }
                            },
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
                                                                    if iSlow23 != 0 {
                                                                        fTemp41
                                                                    } else {
                                                                        1.7976931348623157e+308
                                                                    },
                                                                    if iSlow24 != 0 {
                                                                        self.fVec6[iSlow23 as usize]
                                                                    } else {
                                                                        1.7976931348623157e+308
                                                                    },
                                                                ),
                                                                if iSlow25 != 0 {
                                                                    self.fVec7[iSlow26 as usize]
                                                                } else {
                                                                    1.7976931348623157e+308
                                                                },
                                                            ),
                                                            if iSlow27 != 0 {
                                                                self.fVec8[iSlow28 as usize]
                                                            } else {
                                                                1.7976931348623157e+308
                                                            },
                                                        ),
                                                        if iSlow29 != 0 {
                                                            self.fVec9[((i32::wrapping_sub(
                                                                self.IOTA0, iSlow30,
                                                            )) & 31)
                                                                as usize]
                                                        } else {
                                                            1.7976931348623157e+308
                                                        },
                                                    ),
                                                    if iSlow31 != 0 {
                                                        self.fVec10[((i32::wrapping_sub(
                                                            self.IOTA0, iSlow32,
                                                        )) & 63)
                                                            as usize]
                                                    } else {
                                                        1.7976931348623157e+308
                                                    },
                                                ),
                                                if iSlow33 != 0 {
                                                    self.fVec11[((i32::wrapping_sub(
                                                        self.IOTA0, iSlow34,
                                                    )) & 127)
                                                        as usize]
                                                } else {
                                                    1.7976931348623157e+308
                                                },
                                            ),
                                            if iSlow35 != 0 {
                                                self.fVec12[((i32::wrapping_sub(
                                                    self.IOTA0, iSlow36,
                                                )) & 255)
                                                    as usize]
                                            } else {
                                                1.7976931348623157e+308
                                            },
                                        ),
                                        if iSlow37 != 0 {
                                            self.fVec13[((i32::wrapping_sub(self.IOTA0, iSlow38))
                                                & 511)
                                                as usize]
                                        } else {
                                            1.7976931348623157e+308
                                        },
                                    ),
                                    if iSlow39 != 0 {
                                        self.fVec14[((i32::wrapping_sub(self.IOTA0, iSlow40))
                                            & 1023)
                                            as usize]
                                    } else {
                                        1.7976931348623157e+308
                                    },
                                ),
                                if iSlow41 != 0 {
                                    self.fVec15
                                        [((i32::wrapping_sub(self.IOTA0, iSlow42)) & 2047) as usize]
                                } else {
                                    1.7976931348623157e+308
                                },
                            ),
                            if iSlow43 != 0 {
                                self.fVec16
                                    [((i32::wrapping_sub(self.IOTA0, iSlow44)) & 4095) as usize]
                            } else {
                                1.7976931348623157e+308
                            },
                        ),
                        if iSlow45 != 0 {
                            self.fVec17[((i32::wrapping_sub(self.IOTA0, iSlow46)) & 8191) as usize]
                        } else {
                            1.7976931348623157e+308
                        },
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
                                                                if iSlow4 != 0 {
                                                                    self.fRec3[0]
                                                                } else {
                                                                    1.7976931348623157e+308
                                                                },
                                                                if iSlow47 != 0 {
                                                                    self.fVec18[iSlow4 as usize]
                                                                } else {
                                                                    1.7976931348623157e+308
                                                                },
                                                            ),
                                                            if iSlow48 != 0 {
                                                                self.fVec19[iSlow49 as usize]
                                                            } else {
                                                                1.7976931348623157e+308
                                                            },
                                                        ),
                                                        if iSlow50 != 0 {
                                                            self.fVec20[iSlow51 as usize]
                                                        } else {
                                                            1.7976931348623157e+308
                                                        },
                                                    ),
                                                    if iSlow52 != 0 {
                                                        self.fVec21[((i32::wrapping_sub(
                                                            self.IOTA0, iSlow53,
                                                        )) & 31)
                                                            as usize]
                                                    } else {
                                                        1.7976931348623157e+308
                                                    },
                                                ),
                                                if iSlow54 != 0 {
                                                    self.fVec22[((i32::wrapping_sub(
                                                        self.IOTA0, iSlow55,
                                                    )) & 63)
                                                        as usize]
                                                } else {
                                                    1.7976931348623157e+308
                                                },
                                            ),
                                            if iSlow56 != 0 {
                                                self.fVec23[((i32::wrapping_sub(
                                                    self.IOTA0, iSlow57,
                                                )) & 127)
                                                    as usize]
                                            } else {
                                                1.7976931348623157e+308
                                            },
                                        ),
                                        if iSlow58 != 0 {
                                            self.fVec24[((i32::wrapping_sub(self.IOTA0, iSlow59))
                                                & 255)
                                                as usize]
                                        } else {
                                            1.7976931348623157e+308
                                        },
                                    ),
                                    if iSlow60 != 0 {
                                        self.fVec25[((i32::wrapping_sub(self.IOTA0, iSlow61)) & 511)
                                            as usize]
                                    } else {
                                        1.7976931348623157e+308
                                    },
                                ),
                                if iSlow62 != 0 {
                                    self.fVec26
                                        [((i32::wrapping_sub(self.IOTA0, iSlow63)) & 1023) as usize]
                                } else {
                                    1.7976931348623157e+308
                                },
                            ),
                            if iSlow64 != 0 {
                                self.fVec27
                                    [((i32::wrapping_sub(self.IOTA0, iSlow65)) & 2047) as usize]
                            } else {
                                1.7976931348623157e+308
                            },
                        ),
                        if iSlow66 != 0 {
                            self.fVec28[((i32::wrapping_sub(self.IOTA0, iSlow67)) & 4095) as usize]
                        } else {
                            1.7976931348623157e+308
                        },
                    ),
                    if iSlow68 != 0 {
                        self.fVec29[((i32::wrapping_sub(self.IOTA0, iSlow69)) & 8191) as usize]
                    } else {
                        1.7976931348623157e+308
                    },
                ) - self.fRec2[1];
                self.fVec30[0] = fTemp64;
                let mut iTemp65: i32 = (fTemp64 > 0.0) as i32;
                let mut fTemp71: F64 = if iTemp65 != 0 { fSlow71 } else { fSlow70 };
                self.fVec31[0] = fTemp71;
                let mut fTemp72: F64 = 6.0 * fTemp71;
                let mut iTemp73: i32 = (fTemp72) as i32;
                let mut iTemp74: i32 = std::cmp::max(0, std::cmp::min(iTemp73, 6));
                let mut iTemp75: i32 = std::cmp::max(
                    0,
                    std::cmp::min(i32::wrapping_add(iTemp74, 917497), 1835007),
                );
                let mut fTemp76: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp75, 7)) as usize] };
                let mut fTemp77: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp75 as usize] };
                let mut fTemp78: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp75, 1)) as usize] }
                        - fTemp77;
                let mut fTemp79: F64 = fTemp72 - (iTemp73) as F64;
                let mut fTemp80: F64 = fTemp77
                    + fTemp79 * fTemp78
                    + 0.5
                        * (fTemp76
                            - (fTemp77
                                + fTemp79
                                    * (fTemp78
                                        - (unsafe {
                                            ftbl0LambRs96kSIG0
                                                [(i32::wrapping_add(iTemp75, 8)) as usize]
                                        } - fTemp76))));
                let mut fTemp81: F64 = if iTemp65 != 0 { fTemp80 } else { 1.0 - fTemp80 };
                let mut iTemp82: i32 = (fTemp64 < 0.0) as i32;
                let mut fTemp83: F64 = fSlow1 * (iTemp82) as F64 + fSlow13 * (iTemp65) as F64;
                self.fVec32[0] = fTemp83;
                let mut fTemp84: F64 = self.fConst10 / fTemp83;
                let mut fTemp85: F64 = fTemp84 + 0.5;
                let mut fTemp86: F64 = 262143.0 * (1.0 - fTemp85);
                let mut iTemp87: i32 = (fTemp86) as i32;
                let mut iTemp88: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp87, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp89: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp88, 7)) as usize] };
                let mut fTemp90: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp88 as usize] };
                let mut fTemp91: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp88, 1)) as usize] }
                        - fTemp90;
                let mut fTemp92: F64 = 262143.0 * fTemp85;
                let mut iTemp93: i32 = (fTemp92) as i32;
                let mut iTemp94: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp93, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp95: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp94, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp96: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp94 as usize] };
                let mut fTemp97: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp94, 1), 1835007),
                    )) as usize]
                } - fTemp96;
                let mut fTemp98: F64 = 6.0 * self.fVec31[1];
                let mut iTemp99: i32 = (fTemp98) as i32;
                let mut iTemp100: i32 = std::cmp::max(0, std::cmp::min(iTemp99, 6));
                let mut fTemp101: F64 = 262143.0 * (1.0 - self.fRec1[1]);
                let mut iTemp102: i32 = (fTemp101) as i32;
                let mut iTemp103: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp102, 262143))),
                            iTemp100,
                        ),
                        1835007,
                    ),
                );
                let mut fTemp104: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp103, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp105: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp103 as usize] };
                let mut fTemp106: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp103, 1), 1835007),
                    )) as usize]
                } - fTemp105;
                let mut fTemp107: F64 = fTemp98 - (iTemp99) as F64;
                let mut fTemp108: F64 = 262143.0 * self.fRec1[1];
                let mut iTemp109: i32 = (fTemp108) as i32;
                let mut iTemp110: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp100,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp109, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp111: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp110, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp112: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp110 as usize] };
                let mut fTemp113: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp110, 1), 1835007),
                    )) as usize]
                } - fTemp112;
                let mut fTemp114: F64 = self.fRec1[1] + fTemp84;
                let mut fTemp115: F64 = 262143.0 * (1.0 - fTemp114);
                let mut iTemp116: i32 = (fTemp115) as i32;
                let mut iTemp117: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp116, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp118: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp117, 7)) as usize] };
                let mut fTemp119: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp117 as usize] };
                let mut fTemp120: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp117, 1)) as usize] }
                        - fTemp119;
                let mut fTemp121: F64 = 262143.0 * fTemp114;
                let mut iTemp122: i32 = (fTemp121) as i32;
                let mut iTemp123: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp122, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp124: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp123, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp125: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp123 as usize] };
                let mut fTemp126: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp123, 1), 1835007),
                    )) as usize]
                } - fTemp125;
                let mut fTemp127: F64 =
                    self.fRec1[1] + self.fConst10 * (1.0 / fTemp83 + 1.0 / self.fVec32[1]);
                let mut fTemp128: F64 = 262143.0 * (1.0 - fTemp127);
                let mut iTemp129: i32 = (fTemp128) as i32;
                let mut iTemp130: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp129, 262143))),
                            iTemp74,
                        ),
                        1835007,
                    ),
                );
                let mut fTemp131: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp130, 7)) as usize] };
                let mut fTemp132: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp130 as usize] };
                let mut fTemp133: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp130, 1)) as usize] }
                        - fTemp132;
                let mut fTemp134: F64 = 262143.0 * fTemp127;
                let mut iTemp135: i32 = (fTemp134) as i32;
                let mut iTemp136: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp135, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp137: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp136, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp138: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp136 as usize] };
                let mut fTemp139: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp136, 1), 1835007),
                    )) as usize]
                } - fTemp138;
                let mut fTemp140: F64 = (if iTemp65 != 0 {
                    fTemp138
                        + fTemp79 * fTemp139
                        + (fTemp134 - (iTemp135) as F64)
                            * (fTemp137
                                - (fTemp138
                                    + fTemp79
                                        * (fTemp139
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp136, 8),
                                                        1835007,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp137))))
                } else {
                    1.0 - (fTemp132
                        + fTemp79 * fTemp133
                        + (fTemp128 - (iTemp129) as F64)
                            * (fTemp131
                                - (fTemp132
                                    + fTemp79
                                        * (fTemp133
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp130, 8)) as usize]
                                            } - fTemp131)))))
                } - if iTemp65 != 0 {
                    fTemp125
                        + fTemp79 * fTemp126
                        + (fTemp121 - (iTemp122) as F64)
                            * (fTemp124
                                - (fTemp125
                                    + fTemp79
                                        * (fTemp126
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp123, 8),
                                                        1835007,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp124))))
                } else {
                    1.0 - (fTemp119
                        + fTemp79 * fTemp120
                        + (fTemp115 - (iTemp116) as F64)
                            * (fTemp118
                                - (fTemp119
                                    + fTemp79
                                        * (fTemp120
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp117, 8)) as usize]
                                            } - fTemp118)))))
                }) * self.fVec30[1]
                    / (fTemp64
                        * (1.0
                            - if iTemp65 != 0 {
                                fTemp112
                                    + fTemp107 * fTemp113
                                    + (fTemp108 - (iTemp109) as F64)
                                        * (fTemp111
                                            - (fTemp112
                                                + fTemp107
                                                    * (fTemp113
                                                        - (unsafe {
                                                            ftbl0LambRs96kSIG0[(std::cmp::max(
                                                                0,
                                                                std::cmp::min(
                                                                    i32::wrapping_add(iTemp110, 8),
                                                                    1835007,
                                                                ),
                                                            ))
                                                                as usize]
                                                        } - fTemp111))))
                            } else {
                                1.0 - (fTemp105
                                    + fTemp107 * fTemp106
                                    + (fTemp101 - (iTemp102) as F64)
                                        * (fTemp104
                                            - (fTemp105
                                                + fTemp107
                                                    * (fTemp106
                                                        - (unsafe {
                                                            ftbl0LambRs96kSIG0[(std::cmp::max(
                                                                0,
                                                                std::cmp::min(
                                                                    i32::wrapping_add(iTemp103, 8),
                                                                    1835007,
                                                                ),
                                                            ))
                                                                as usize]
                                                        } - fTemp104)))))
                            }));
                let mut iTemp141: i32 = (fTemp140
                    > ((if iTemp65 != 0 {
                        fTemp96
                            + fTemp79 * fTemp97
                            + (fTemp92 - (iTemp93) as F64)
                                * (fTemp95
                                    - (fTemp96
                                        + fTemp79
                                            * (fTemp97
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp94, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp95))))
                    } else {
                        1.0 - (fTemp90
                            + fTemp79 * fTemp91
                            + (fTemp86 - (iTemp87) as F64)
                                * (fTemp89
                                    - (fTemp90
                                        + fTemp79
                                            * (fTemp91
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp88, 8)) as usize]
                                                } - fTemp89)))))
                    } - fTemp81)
                        / (1.0 - fTemp81))) as i32;
                let mut fTemp142: F64 = if iTemp141 != 0 { 1.0 } else { 0.5 };
                let mut fTemp143: F64 = if iTemp141 != 0 { 0.5 } else { 0.0 };
                let mut fTemp144: F64 = fTemp143 + fTemp142;
                let mut fTemp145: F64 = 0.5 * fTemp144;
                let mut fTemp146: F64 = 262143.0 * (1.0 - fTemp145);
                let mut iTemp147: i32 = (fTemp146) as i32;
                let mut iTemp148: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp147, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp149: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp148, 7)) as usize] };
                let mut fTemp150: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp148 as usize] };
                let mut fTemp151: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp148, 1)) as usize] }
                        - fTemp150;
                let mut fTemp152: F64 = 131071.5 * fTemp144;
                let mut iTemp153: i32 = (fTemp152) as i32;
                let mut iTemp154: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp153, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp155: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp154, 7)) as usize] };
                let mut fTemp156: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp154 as usize] };
                let mut fTemp157: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp154, 1)) as usize] }
                        - fTemp156;
                let mut fTemp158: F64 = if iTemp65 != 0 {
                    fTemp156
                        + fTemp79 * fTemp157
                        + (fTemp152 - (iTemp153) as F64)
                            * (fTemp155
                                - (fTemp156
                                    + fTemp79
                                        * (fTemp157
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp154, 8)) as usize]
                                            } - fTemp155))))
                } else {
                    1.0 - (fTemp150
                        + fTemp79 * fTemp151
                        + (fTemp146 - (iTemp147) as F64)
                            * (fTemp149
                                - (fTemp150
                                    + fTemp79
                                        * (fTemp151
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp148, 8)) as usize]
                                            } - fTemp149)))))
                };
                let mut fTemp159: F64 = fTemp84 + fTemp145;
                let mut fTemp160: F64 = 262143.0 * (1.0 - fTemp159);
                let mut iTemp161: i32 = (fTemp160) as i32;
                let mut iTemp162: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp161, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp163: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp162, 7)) as usize] };
                let mut fTemp164: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp162 as usize] };
                let mut fTemp165: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp162, 1)) as usize] }
                        - fTemp164;
                let mut fTemp166: F64 = 262143.0 * fTemp159;
                let mut iTemp167: i32 = (fTemp166) as i32;
                let mut iTemp168: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp167, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp169: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp168, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp170: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp168 as usize] };
                let mut fTemp171: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp168, 1), 1835007),
                    )) as usize]
                } - fTemp170;
                let mut iTemp172: i32 = (fTemp140
                    > ((if iTemp65 != 0 {
                        fTemp170
                            + fTemp79 * fTemp171
                            + (fTemp166 - (iTemp167) as F64)
                                * (fTemp169
                                    - (fTemp170
                                        + fTemp79
                                            * (fTemp171
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp168, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp169))))
                    } else {
                        1.0 - (fTemp164
                            + fTemp79 * fTemp165
                            + (fTemp160 - (iTemp161) as F64)
                                * (fTemp163
                                    - (fTemp164
                                        + fTemp79
                                            * (fTemp165
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp162, 8)) as usize]
                                                } - fTemp163)))))
                    } - fTemp158)
                        / (1.0 - fTemp158))) as i32;
                let mut fTemp173: F64 = if iTemp172 != 0 { fTemp142 } else { fTemp145 };
                let mut fTemp174: F64 = if iTemp172 != 0 { fTemp145 } else { fTemp143 };
                let mut fTemp175: F64 = fTemp174 + fTemp173;
                let mut fTemp176: F64 = 0.5 * fTemp175;
                let mut fTemp177: F64 = 262143.0 * (1.0 - fTemp176);
                let mut iTemp178: i32 = (fTemp177) as i32;
                let mut iTemp179: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp178, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp180: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp179, 7)) as usize] };
                let mut fTemp181: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp179 as usize] };
                let mut fTemp182: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp179, 1)) as usize] }
                        - fTemp181;
                let mut fTemp183: F64 = 131071.5 * fTemp175;
                let mut iTemp184: i32 = (fTemp183) as i32;
                let mut iTemp185: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp184, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp186: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp185, 7)) as usize] };
                let mut fTemp187: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp185 as usize] };
                let mut fTemp188: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp185, 1)) as usize] }
                        - fTemp187;
                let mut fTemp189: F64 = if iTemp65 != 0 {
                    fTemp187
                        + fTemp79 * fTemp188
                        + (fTemp183 - (iTemp184) as F64)
                            * (fTemp186
                                - (fTemp187
                                    + fTemp79
                                        * (fTemp188
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp185, 8)) as usize]
                                            } - fTemp186))))
                } else {
                    1.0 - (fTemp181
                        + fTemp79 * fTemp182
                        + (fTemp177 - (iTemp178) as F64)
                            * (fTemp180
                                - (fTemp181
                                    + fTemp79
                                        * (fTemp182
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp179, 8)) as usize]
                                            } - fTemp180)))))
                };
                let mut fTemp190: F64 = fTemp84 + fTemp176;
                let mut fTemp191: F64 = 262143.0 * (1.0 - fTemp190);
                let mut iTemp192: i32 = (fTemp191) as i32;
                let mut iTemp193: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp192, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp194: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp193, 7)) as usize] };
                let mut fTemp195: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp193 as usize] };
                let mut fTemp196: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp193, 1)) as usize] }
                        - fTemp195;
                let mut fTemp197: F64 = 262143.0 * fTemp190;
                let mut iTemp198: i32 = (fTemp197) as i32;
                let mut iTemp199: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp198, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp200: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp199, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp201: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp199 as usize] };
                let mut fTemp202: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp199, 1), 1835007),
                    )) as usize]
                } - fTemp201;
                let mut iTemp203: i32 = (fTemp140
                    > ((if iTemp65 != 0 {
                        fTemp201
                            + fTemp79 * fTemp202
                            + (fTemp197 - (iTemp198) as F64)
                                * (fTemp200
                                    - (fTemp201
                                        + fTemp79
                                            * (fTemp202
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp199, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp200))))
                    } else {
                        1.0 - (fTemp195
                            + fTemp79 * fTemp196
                            + (fTemp191 - (iTemp192) as F64)
                                * (fTemp194
                                    - (fTemp195
                                        + fTemp79
                                            * (fTemp196
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp193, 8)) as usize]
                                                } - fTemp194)))))
                    } - fTemp189)
                        / (1.0 - fTemp189))) as i32;
                let mut fTemp204: F64 = if iTemp203 != 0 { fTemp173 } else { fTemp176 };
                let mut fTemp205: F64 = if iTemp203 != 0 { fTemp176 } else { fTemp174 };
                let mut fTemp206: F64 = fTemp205 + fTemp204;
                let mut fTemp207: F64 = 0.5 * fTemp206;
                let mut fTemp208: F64 = 262143.0 * (1.0 - fTemp207);
                let mut iTemp209: i32 = (fTemp208) as i32;
                let mut iTemp210: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp209, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp211: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp210, 7)) as usize] };
                let mut fTemp212: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp210 as usize] };
                let mut fTemp213: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp210, 1)) as usize] }
                        - fTemp212;
                let mut fTemp214: F64 = 131071.5 * fTemp206;
                let mut iTemp215: i32 = (fTemp214) as i32;
                let mut iTemp216: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp215, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp217: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp216, 7)) as usize] };
                let mut fTemp218: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp216 as usize] };
                let mut fTemp219: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp216, 1)) as usize] }
                        - fTemp218;
                let mut fTemp220: F64 = if iTemp65 != 0 {
                    fTemp218
                        + fTemp79 * fTemp219
                        + (fTemp214 - (iTemp215) as F64)
                            * (fTemp217
                                - (fTemp218
                                    + fTemp79
                                        * (fTemp219
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp216, 8)) as usize]
                                            } - fTemp217))))
                } else {
                    1.0 - (fTemp212
                        + fTemp79 * fTemp213
                        + (fTemp208 - (iTemp209) as F64)
                            * (fTemp211
                                - (fTemp212
                                    + fTemp79
                                        * (fTemp213
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp210, 8)) as usize]
                                            } - fTemp211)))))
                };
                let mut fTemp221: F64 = fTemp84 + fTemp207;
                let mut fTemp222: F64 = 262143.0 * (1.0 - fTemp221);
                let mut iTemp223: i32 = (fTemp222) as i32;
                let mut iTemp224: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp223, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp225: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp224, 7)) as usize] };
                let mut fTemp226: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp224 as usize] };
                let mut fTemp227: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp224, 1)) as usize] }
                        - fTemp226;
                let mut fTemp228: F64 = 262143.0 * fTemp221;
                let mut iTemp229: i32 = (fTemp228) as i32;
                let mut iTemp230: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp229, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp231: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp230, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp232: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp230 as usize] };
                let mut fTemp233: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp230, 1), 1835007),
                    )) as usize]
                } - fTemp232;
                let mut iTemp234: i32 = (fTemp140
                    > ((if iTemp65 != 0 {
                        fTemp232
                            + fTemp79 * fTemp233
                            + (fTemp228 - (iTemp229) as F64)
                                * (fTemp231
                                    - (fTemp232
                                        + fTemp79
                                            * (fTemp233
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp230, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp231))))
                    } else {
                        1.0 - (fTemp226
                            + fTemp79 * fTemp227
                            + (fTemp222 - (iTemp223) as F64)
                                * (fTemp225
                                    - (fTemp226
                                        + fTemp79
                                            * (fTemp227
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp224, 8)) as usize]
                                                } - fTemp225)))))
                    } - fTemp220)
                        / (1.0 - fTemp220))) as i32;
                let mut fTemp235: F64 = if iTemp234 != 0 { fTemp204 } else { fTemp207 };
                let mut fTemp236: F64 = if iTemp234 != 0 { fTemp207 } else { fTemp205 };
                let mut fTemp237: F64 = fTemp236 + fTemp235;
                let mut fTemp238: F64 = 0.5 * fTemp237;
                let mut fTemp239: F64 = 262143.0 * (1.0 - fTemp238);
                let mut iTemp240: i32 = (fTemp239) as i32;
                let mut iTemp241: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp240, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp242: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp241, 7)) as usize] };
                let mut fTemp243: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp241 as usize] };
                let mut fTemp244: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp241, 1)) as usize] }
                        - fTemp243;
                let mut fTemp245: F64 = 131071.5 * fTemp237;
                let mut iTemp246: i32 = (fTemp245) as i32;
                let mut iTemp247: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp246, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp248: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp247, 7)) as usize] };
                let mut fTemp249: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp247 as usize] };
                let mut fTemp250: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp247, 1)) as usize] }
                        - fTemp249;
                let mut fTemp251: F64 = if iTemp65 != 0 {
                    fTemp249
                        + fTemp79 * fTemp250
                        + (fTemp245 - (iTemp246) as F64)
                            * (fTemp248
                                - (fTemp249
                                    + fTemp79
                                        * (fTemp250
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp247, 8)) as usize]
                                            } - fTemp248))))
                } else {
                    1.0 - (fTemp243
                        + fTemp79 * fTemp244
                        + (fTemp239 - (iTemp240) as F64)
                            * (fTemp242
                                - (fTemp243
                                    + fTemp79
                                        * (fTemp244
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp241, 8)) as usize]
                                            } - fTemp242)))))
                };
                let mut fTemp252: F64 = fTemp84 + fTemp238;
                let mut fTemp253: F64 = 262143.0 * (1.0 - fTemp252);
                let mut iTemp254: i32 = (fTemp253) as i32;
                let mut iTemp255: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp254, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp256: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp255, 7)) as usize] };
                let mut fTemp257: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp255 as usize] };
                let mut fTemp258: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp255, 1)) as usize] }
                        - fTemp257;
                let mut fTemp259: F64 = 262143.0 * fTemp252;
                let mut iTemp260: i32 = (fTemp259) as i32;
                let mut iTemp261: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp260, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp262: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp261, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp263: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp261 as usize] };
                let mut fTemp264: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp261, 1), 1835007),
                    )) as usize]
                } - fTemp263;
                let mut iTemp265: i32 = (fTemp140
                    > ((if iTemp65 != 0 {
                        fTemp263
                            + fTemp79 * fTemp264
                            + (fTemp259 - (iTemp260) as F64)
                                * (fTemp262
                                    - (fTemp263
                                        + fTemp79
                                            * (fTemp264
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp261, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp262))))
                    } else {
                        1.0 - (fTemp257
                            + fTemp79 * fTemp258
                            + (fTemp253 - (iTemp254) as F64)
                                * (fTemp256
                                    - (fTemp257
                                        + fTemp79
                                            * (fTemp258
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp255, 8)) as usize]
                                                } - fTemp256)))))
                    } - fTemp251)
                        / (1.0 - fTemp251))) as i32;
                let mut fTemp266: F64 = if iTemp265 != 0 { fTemp235 } else { fTemp238 };
                let mut fTemp267: F64 = if iTemp265 != 0 { fTemp238 } else { fTemp236 };
                let mut fTemp268: F64 = fTemp267 + fTemp266;
                let mut fTemp269: F64 = 0.5 * fTemp268;
                let mut fTemp270: F64 = 262143.0 * (1.0 - fTemp269);
                let mut iTemp271: i32 = (fTemp270) as i32;
                let mut iTemp272: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp271, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp273: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp272, 7)) as usize] };
                let mut fTemp274: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp272 as usize] };
                let mut fTemp275: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp272, 1)) as usize] }
                        - fTemp274;
                let mut fTemp276: F64 = 131071.5 * fTemp268;
                let mut iTemp277: i32 = (fTemp276) as i32;
                let mut iTemp278: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp277, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp279: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp278, 7)) as usize] };
                let mut fTemp280: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp278 as usize] };
                let mut fTemp281: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp278, 1)) as usize] }
                        - fTemp280;
                let mut fTemp282: F64 = if iTemp65 != 0 {
                    fTemp280
                        + fTemp79 * fTemp281
                        + (fTemp276 - (iTemp277) as F64)
                            * (fTemp279
                                - (fTemp280
                                    + fTemp79
                                        * (fTemp281
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp278, 8)) as usize]
                                            } - fTemp279))))
                } else {
                    1.0 - (fTemp274
                        + fTemp79 * fTemp275
                        + (fTemp270 - (iTemp271) as F64)
                            * (fTemp273
                                - (fTemp274
                                    + fTemp79
                                        * (fTemp275
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp272, 8)) as usize]
                                            } - fTemp273)))))
                };
                let mut fTemp283: F64 = fTemp84 + fTemp269;
                let mut fTemp284: F64 = 262143.0 * (1.0 - fTemp283);
                let mut iTemp285: i32 = (fTemp284) as i32;
                let mut iTemp286: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp285, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp287: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp286, 7)) as usize] };
                let mut fTemp288: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp286 as usize] };
                let mut fTemp289: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp286, 1)) as usize] }
                        - fTemp288;
                let mut fTemp290: F64 = 262143.0 * fTemp283;
                let mut iTemp291: i32 = (fTemp290) as i32;
                let mut iTemp292: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp291, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp293: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp292, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp294: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp292 as usize] };
                let mut fTemp295: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp292, 1), 1835007),
                    )) as usize]
                } - fTemp294;
                let mut iTemp296: i32 = (fTemp140
                    > ((if iTemp65 != 0 {
                        fTemp294
                            + fTemp79 * fTemp295
                            + (fTemp290 - (iTemp291) as F64)
                                * (fTemp293
                                    - (fTemp294
                                        + fTemp79
                                            * (fTemp295
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp292, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp293))))
                    } else {
                        1.0 - (fTemp288
                            + fTemp79 * fTemp289
                            + (fTemp284 - (iTemp285) as F64)
                                * (fTemp287
                                    - (fTemp288
                                        + fTemp79
                                            * (fTemp289
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp286, 8)) as usize]
                                                } - fTemp287)))))
                    } - fTemp282)
                        / (1.0 - fTemp282))) as i32;
                let mut fTemp297: F64 = if iTemp296 != 0 { fTemp266 } else { fTemp269 };
                let mut fTemp298: F64 = if iTemp296 != 0 { fTemp269 } else { fTemp267 };
                let mut fTemp299: F64 = fTemp298 + fTemp297;
                let mut fTemp300: F64 = 0.5 * fTemp299;
                let mut fTemp301: F64 = 262143.0 * (1.0 - fTemp300);
                let mut iTemp302: i32 = (fTemp301) as i32;
                let mut iTemp303: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp302, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp304: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp303, 7)) as usize] };
                let mut fTemp305: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp303 as usize] };
                let mut fTemp306: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp303, 1)) as usize] }
                        - fTemp305;
                let mut fTemp307: F64 = 131071.5 * fTemp299;
                let mut iTemp308: i32 = (fTemp307) as i32;
                let mut iTemp309: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp308, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp310: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp309, 7)) as usize] };
                let mut fTemp311: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp309 as usize] };
                let mut fTemp312: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp309, 1)) as usize] }
                        - fTemp311;
                let mut fTemp313: F64 = if iTemp65 != 0 {
                    fTemp311
                        + fTemp79 * fTemp312
                        + (fTemp307 - (iTemp308) as F64)
                            * (fTemp310
                                - (fTemp311
                                    + fTemp79
                                        * (fTemp312
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp309, 8)) as usize]
                                            } - fTemp310))))
                } else {
                    1.0 - (fTemp305
                        + fTemp79 * fTemp306
                        + (fTemp301 - (iTemp302) as F64)
                            * (fTemp304
                                - (fTemp305
                                    + fTemp79
                                        * (fTemp306
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp303, 8)) as usize]
                                            } - fTemp304)))))
                };
                let mut fTemp314: F64 = fTemp84 + fTemp300;
                let mut fTemp315: F64 = 262143.0 * (1.0 - fTemp314);
                let mut iTemp316: i32 = (fTemp315) as i32;
                let mut iTemp317: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp316, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp318: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp317, 7)) as usize] };
                let mut fTemp319: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp317 as usize] };
                let mut fTemp320: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp317, 1)) as usize] }
                        - fTemp319;
                let mut fTemp321: F64 = 262143.0 * fTemp314;
                let mut iTemp322: i32 = (fTemp321) as i32;
                let mut iTemp323: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp322, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp324: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp323, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp325: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp323 as usize] };
                let mut fTemp326: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp323, 1), 1835007),
                    )) as usize]
                } - fTemp325;
                let mut iTemp327: i32 = (fTemp140
                    > ((if iTemp65 != 0 {
                        fTemp325
                            + fTemp79 * fTemp326
                            + (fTemp321 - (iTemp322) as F64)
                                * (fTemp324
                                    - (fTemp325
                                        + fTemp79
                                            * (fTemp326
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp323, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp324))))
                    } else {
                        1.0 - (fTemp319
                            + fTemp79 * fTemp320
                            + (fTemp315 - (iTemp316) as F64)
                                * (fTemp318
                                    - (fTemp319
                                        + fTemp79
                                            * (fTemp320
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp317, 8)) as usize]
                                                } - fTemp318)))))
                    } - fTemp313)
                        / (1.0 - fTemp313))) as i32;
                let mut fTemp328: F64 = if iTemp327 != 0 { fTemp297 } else { fTemp300 };
                let mut fTemp329: F64 = if iTemp327 != 0 { fTemp300 } else { fTemp298 };
                let mut fTemp330: F64 = fTemp329 + fTemp328;
                let mut fTemp331: F64 = 0.5 * fTemp330;
                let mut fTemp332: F64 = 262143.0 * (1.0 - fTemp331);
                let mut iTemp333: i32 = (fTemp332) as i32;
                let mut iTemp334: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp333, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp335: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp334, 7)) as usize] };
                let mut fTemp336: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp334 as usize] };
                let mut fTemp337: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp334, 1)) as usize] }
                        - fTemp336;
                let mut fTemp338: F64 = 131071.5 * fTemp330;
                let mut iTemp339: i32 = (fTemp338) as i32;
                let mut iTemp340: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp339, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp341: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp340, 7)) as usize] };
                let mut fTemp342: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp340 as usize] };
                let mut fTemp343: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp340, 1)) as usize] }
                        - fTemp342;
                let mut fTemp344: F64 = if iTemp65 != 0 {
                    fTemp342
                        + fTemp79 * fTemp343
                        + (fTemp338 - (iTemp339) as F64)
                            * (fTemp341
                                - (fTemp342
                                    + fTemp79
                                        * (fTemp343
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp340, 8)) as usize]
                                            } - fTemp341))))
                } else {
                    1.0 - (fTemp336
                        + fTemp79 * fTemp337
                        + (fTemp332 - (iTemp333) as F64)
                            * (fTemp335
                                - (fTemp336
                                    + fTemp79
                                        * (fTemp337
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp334, 8)) as usize]
                                            } - fTemp335)))))
                };
                let mut fTemp345: F64 = fTemp84 + fTemp331;
                let mut fTemp346: F64 = 262143.0 * (1.0 - fTemp345);
                let mut iTemp347: i32 = (fTemp346) as i32;
                let mut iTemp348: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp347, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp349: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp348, 7)) as usize] };
                let mut fTemp350: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp348 as usize] };
                let mut fTemp351: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp348, 1)) as usize] }
                        - fTemp350;
                let mut fTemp352: F64 = 262143.0 * fTemp345;
                let mut iTemp353: i32 = (fTemp352) as i32;
                let mut iTemp354: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp353, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp355: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp354, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp356: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp354 as usize] };
                let mut fTemp357: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp354, 1), 1835007),
                    )) as usize]
                } - fTemp356;
                let mut iTemp358: i32 = (fTemp140
                    > ((if iTemp65 != 0 {
                        fTemp356
                            + fTemp79 * fTemp357
                            + (fTemp352 - (iTemp353) as F64)
                                * (fTemp355
                                    - (fTemp356
                                        + fTemp79
                                            * (fTemp357
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp354, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp355))))
                    } else {
                        1.0 - (fTemp350
                            + fTemp79 * fTemp351
                            + (fTemp346 - (iTemp347) as F64)
                                * (fTemp349
                                    - (fTemp350
                                        + fTemp79
                                            * (fTemp351
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp348, 8)) as usize]
                                                } - fTemp349)))))
                    } - fTemp344)
                        / (1.0 - fTemp344))) as i32;
                let mut fTemp359: F64 = if iTemp358 != 0 { fTemp328 } else { fTemp331 };
                let mut fTemp360: F64 = if iTemp358 != 0 { fTemp331 } else { fTemp329 };
                let mut fTemp361: F64 = fTemp360 + fTemp359;
                let mut fTemp362: F64 = 0.5 * fTemp361;
                let mut fTemp363: F64 = 262143.0 * (1.0 - fTemp362);
                let mut iTemp364: i32 = (fTemp363) as i32;
                let mut iTemp365: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp364, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp366: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp365, 7)) as usize] };
                let mut fTemp367: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp365 as usize] };
                let mut fTemp368: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp365, 1)) as usize] }
                        - fTemp367;
                let mut fTemp369: F64 = 131071.5 * fTemp361;
                let mut iTemp370: i32 = (fTemp369) as i32;
                let mut iTemp371: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp370, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp372: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp371, 7)) as usize] };
                let mut fTemp373: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp371 as usize] };
                let mut fTemp374: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp371, 1)) as usize] }
                        - fTemp373;
                let mut fTemp375: F64 = if iTemp65 != 0 {
                    fTemp373
                        + fTemp79 * fTemp374
                        + (fTemp369 - (iTemp370) as F64)
                            * (fTemp372
                                - (fTemp373
                                    + fTemp79
                                        * (fTemp374
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp371, 8)) as usize]
                                            } - fTemp372))))
                } else {
                    1.0 - (fTemp367
                        + fTemp79 * fTemp368
                        + (fTemp363 - (iTemp364) as F64)
                            * (fTemp366
                                - (fTemp367
                                    + fTemp79
                                        * (fTemp368
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp365, 8)) as usize]
                                            } - fTemp366)))))
                };
                let mut fTemp376: F64 = fTemp84 + fTemp362;
                let mut fTemp377: F64 = 262143.0 * (1.0 - fTemp376);
                let mut iTemp378: i32 = (fTemp377) as i32;
                let mut iTemp379: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp378, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp380: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp379, 7)) as usize] };
                let mut fTemp381: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp379 as usize] };
                let mut fTemp382: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp379, 1)) as usize] }
                        - fTemp381;
                let mut fTemp383: F64 = 262143.0 * fTemp376;
                let mut iTemp384: i32 = (fTemp383) as i32;
                let mut iTemp385: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp384, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp386: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp385, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp387: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp385 as usize] };
                let mut fTemp388: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp385, 1), 1835007),
                    )) as usize]
                } - fTemp387;
                let mut iTemp389: i32 = (fTemp140
                    > ((if iTemp65 != 0 {
                        fTemp387
                            + fTemp79 * fTemp388
                            + (fTemp383 - (iTemp384) as F64)
                                * (fTemp386
                                    - (fTemp387
                                        + fTemp79
                                            * (fTemp388
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp385, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp386))))
                    } else {
                        1.0 - (fTemp381
                            + fTemp79 * fTemp382
                            + (fTemp377 - (iTemp378) as F64)
                                * (fTemp380
                                    - (fTemp381
                                        + fTemp79
                                            * (fTemp382
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp379, 8)) as usize]
                                                } - fTemp380)))))
                    } - fTemp375)
                        / (1.0 - fTemp375))) as i32;
                let mut fTemp390: F64 = if iTemp389 != 0 { fTemp359 } else { fTemp362 };
                let mut fTemp391: F64 = if iTemp389 != 0 { fTemp362 } else { fTemp360 };
                let mut fTemp392: F64 = fTemp391 + fTemp390;
                let mut fTemp393: F64 = 0.5 * fTemp392;
                let mut fTemp394: F64 = 262143.0 * (1.0 - fTemp393);
                let mut iTemp395: i32 = (fTemp394) as i32;
                let mut iTemp396: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp395, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp397: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp396, 7)) as usize] };
                let mut fTemp398: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp396 as usize] };
                let mut fTemp399: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp396, 1)) as usize] }
                        - fTemp398;
                let mut fTemp400: F64 = 131071.5 * fTemp392;
                let mut iTemp401: i32 = (fTemp400) as i32;
                let mut iTemp402: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp401, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp403: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp402, 7)) as usize] };
                let mut fTemp404: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp402 as usize] };
                let mut fTemp405: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp402, 1)) as usize] }
                        - fTemp404;
                let mut fTemp406: F64 = if iTemp65 != 0 {
                    fTemp404
                        + fTemp79 * fTemp405
                        + (fTemp400 - (iTemp401) as F64)
                            * (fTemp403
                                - (fTemp404
                                    + fTemp79
                                        * (fTemp405
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp402, 8)) as usize]
                                            } - fTemp403))))
                } else {
                    1.0 - (fTemp398
                        + fTemp79 * fTemp399
                        + (fTemp394 - (iTemp395) as F64)
                            * (fTemp397
                                - (fTemp398
                                    + fTemp79
                                        * (fTemp399
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp396, 8)) as usize]
                                            } - fTemp397)))))
                };
                let mut fTemp407: F64 = fTemp84 + fTemp393;
                let mut fTemp408: F64 = 262143.0 * (1.0 - fTemp407);
                let mut iTemp409: i32 = (fTemp408) as i32;
                let mut iTemp410: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp409, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp411: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp410, 7)) as usize] };
                let mut fTemp412: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp410 as usize] };
                let mut fTemp413: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp410, 1)) as usize] }
                        - fTemp412;
                let mut fTemp414: F64 = 262143.0 * fTemp407;
                let mut iTemp415: i32 = (fTemp414) as i32;
                let mut iTemp416: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp415, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp417: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp416, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp418: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp416 as usize] };
                let mut fTemp419: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp416, 1), 1835007),
                    )) as usize]
                } - fTemp418;
                let mut iTemp420: i32 = (fTemp140
                    > ((if iTemp65 != 0 {
                        fTemp418
                            + fTemp79 * fTemp419
                            + (fTemp414 - (iTemp415) as F64)
                                * (fTemp417
                                    - (fTemp418
                                        + fTemp79
                                            * (fTemp419
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp416, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp417))))
                    } else {
                        1.0 - (fTemp412
                            + fTemp79 * fTemp413
                            + (fTemp408 - (iTemp409) as F64)
                                * (fTemp411
                                    - (fTemp412
                                        + fTemp79
                                            * (fTemp413
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp410, 8)) as usize]
                                                } - fTemp411)))))
                    } - fTemp406)
                        / (1.0 - fTemp406))) as i32;
                let mut fTemp421: F64 = if iTemp420 != 0 { fTemp390 } else { fTemp393 };
                let mut fTemp422: F64 = if iTemp420 != 0 { fTemp393 } else { fTemp391 };
                let mut fTemp423: F64 = fTemp422 + fTemp421;
                let mut fTemp424: F64 = 0.5 * fTemp423;
                let mut fTemp425: F64 = 262143.0 * (1.0 - fTemp424);
                let mut iTemp426: i32 = (fTemp425) as i32;
                let mut iTemp427: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp426, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp428: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp427, 7)) as usize] };
                let mut fTemp429: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp427 as usize] };
                let mut fTemp430: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp427, 1)) as usize] }
                        - fTemp429;
                let mut fTemp431: F64 = 131071.5 * fTemp423;
                let mut iTemp432: i32 = (fTemp431) as i32;
                let mut iTemp433: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp432, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp434: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp433, 7)) as usize] };
                let mut fTemp435: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp433 as usize] };
                let mut fTemp436: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp433, 1)) as usize] }
                        - fTemp435;
                let mut fTemp437: F64 = if iTemp65 != 0 {
                    fTemp435
                        + fTemp79 * fTemp436
                        + (fTemp431 - (iTemp432) as F64)
                            * (fTemp434
                                - (fTemp435
                                    + fTemp79
                                        * (fTemp436
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp433, 8)) as usize]
                                            } - fTemp434))))
                } else {
                    1.0 - (fTemp429
                        + fTemp79 * fTemp430
                        + (fTemp425 - (iTemp426) as F64)
                            * (fTemp428
                                - (fTemp429
                                    + fTemp79
                                        * (fTemp430
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp427, 8)) as usize]
                                            } - fTemp428)))))
                };
                let mut fTemp438: F64 = fTemp84 + fTemp424;
                let mut fTemp439: F64 = 262143.0 * (1.0 - fTemp438);
                let mut iTemp440: i32 = (fTemp439) as i32;
                let mut iTemp441: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp440, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp442: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp441, 7)) as usize] };
                let mut fTemp443: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp441 as usize] };
                let mut fTemp444: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp441, 1)) as usize] }
                        - fTemp443;
                let mut fTemp445: F64 = 262143.0 * fTemp438;
                let mut iTemp446: i32 = (fTemp445) as i32;
                let mut iTemp447: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp446, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp448: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp447, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp449: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp447 as usize] };
                let mut fTemp450: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp447, 1), 1835007),
                    )) as usize]
                } - fTemp449;
                let mut iTemp451: i32 = (fTemp140
                    > ((if iTemp65 != 0 {
                        fTemp449
                            + fTemp79 * fTemp450
                            + (fTemp445 - (iTemp446) as F64)
                                * (fTemp448
                                    - (fTemp449
                                        + fTemp79
                                            * (fTemp450
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp447, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp448))))
                    } else {
                        1.0 - (fTemp443
                            + fTemp79 * fTemp444
                            + (fTemp439 - (iTemp440) as F64)
                                * (fTemp442
                                    - (fTemp443
                                        + fTemp79
                                            * (fTemp444
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp441, 8)) as usize]
                                                } - fTemp442)))))
                    } - fTemp437)
                        / (1.0 - fTemp437))) as i32;
                let mut fTemp452: F64 = if iTemp451 != 0 { fTemp421 } else { fTemp424 };
                let mut fTemp453: F64 = if iTemp451 != 0 { fTemp424 } else { fTemp422 };
                let mut fTemp454: F64 = fTemp453 + fTemp452;
                let mut fTemp455: F64 = 0.5 * fTemp454;
                let mut fTemp456: F64 = 262143.0 * (1.0 - fTemp455);
                let mut iTemp457: i32 = (fTemp456) as i32;
                let mut iTemp458: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp457, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp459: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp458, 7)) as usize] };
                let mut fTemp460: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp458 as usize] };
                let mut fTemp461: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp458, 1)) as usize] }
                        - fTemp460;
                let mut fTemp462: F64 = 131071.5 * fTemp454;
                let mut iTemp463: i32 = (fTemp462) as i32;
                let mut iTemp464: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp463, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp465: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp464, 7)) as usize] };
                let mut fTemp466: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp464 as usize] };
                let mut fTemp467: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp464, 1)) as usize] }
                        - fTemp466;
                let mut fTemp468: F64 = if iTemp65 != 0 {
                    fTemp466
                        + fTemp79 * fTemp467
                        + (fTemp462 - (iTemp463) as F64)
                            * (fTemp465
                                - (fTemp466
                                    + fTemp79
                                        * (fTemp467
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp464, 8)) as usize]
                                            } - fTemp465))))
                } else {
                    1.0 - (fTemp460
                        + fTemp79 * fTemp461
                        + (fTemp456 - (iTemp457) as F64)
                            * (fTemp459
                                - (fTemp460
                                    + fTemp79
                                        * (fTemp461
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp458, 8)) as usize]
                                            } - fTemp459)))))
                };
                let mut fTemp469: F64 = fTemp84 + fTemp455;
                let mut fTemp470: F64 = 262143.0 * (1.0 - fTemp469);
                let mut iTemp471: i32 = (fTemp470) as i32;
                let mut iTemp472: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp471, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp473: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp472, 7)) as usize] };
                let mut fTemp474: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp472 as usize] };
                let mut fTemp475: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp472, 1)) as usize] }
                        - fTemp474;
                let mut fTemp476: F64 = 262143.0 * fTemp469;
                let mut iTemp477: i32 = (fTemp476) as i32;
                let mut iTemp478: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp477, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp479: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp478, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp480: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp478 as usize] };
                let mut fTemp481: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp478, 1), 1835007),
                    )) as usize]
                } - fTemp480;
                let mut iTemp482: i32 = (fTemp140
                    > ((if iTemp65 != 0 {
                        fTemp480
                            + fTemp79 * fTemp481
                            + (fTemp476 - (iTemp477) as F64)
                                * (fTemp479
                                    - (fTemp480
                                        + fTemp79
                                            * (fTemp481
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp478, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp479))))
                    } else {
                        1.0 - (fTemp474
                            + fTemp79 * fTemp475
                            + (fTemp470 - (iTemp471) as F64)
                                * (fTemp473
                                    - (fTemp474
                                        + fTemp79
                                            * (fTemp475
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp472, 8)) as usize]
                                                } - fTemp473)))))
                    } - fTemp468)
                        / (1.0 - fTemp468))) as i32;
                let mut fTemp483: F64 = if iTemp482 != 0 { fTemp452 } else { fTemp455 };
                let mut fTemp484: F64 = if iTemp482 != 0 { fTemp455 } else { fTemp453 };
                let mut fTemp485: F64 = fTemp484 + fTemp483;
                let mut fTemp486: F64 = 0.5 * fTemp485;
                let mut fTemp487: F64 = 262143.0 * (1.0 - fTemp486);
                let mut iTemp488: i32 = (fTemp487) as i32;
                let mut iTemp489: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp488, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp490: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp489, 7)) as usize] };
                let mut fTemp491: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp489 as usize] };
                let mut fTemp492: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp489, 1)) as usize] }
                        - fTemp491;
                let mut fTemp493: F64 = 131071.5 * fTemp485;
                let mut iTemp494: i32 = (fTemp493) as i32;
                let mut iTemp495: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp494, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp496: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp495, 7)) as usize] };
                let mut fTemp497: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp495 as usize] };
                let mut fTemp498: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp495, 1)) as usize] }
                        - fTemp497;
                let mut fTemp499: F64 = if iTemp65 != 0 {
                    fTemp497
                        + fTemp79 * fTemp498
                        + (fTemp493 - (iTemp494) as F64)
                            * (fTemp496
                                - (fTemp497
                                    + fTemp79
                                        * (fTemp498
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp495, 8)) as usize]
                                            } - fTemp496))))
                } else {
                    1.0 - (fTemp491
                        + fTemp79 * fTemp492
                        + (fTemp487 - (iTemp488) as F64)
                            * (fTemp490
                                - (fTemp491
                                    + fTemp79
                                        * (fTemp492
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp489, 8)) as usize]
                                            } - fTemp490)))))
                };
                let mut fTemp500: F64 = fTemp84 + fTemp486;
                let mut fTemp501: F64 = 262143.0 * (1.0 - fTemp500);
                let mut iTemp502: i32 = (fTemp501) as i32;
                let mut iTemp503: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp502, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp504: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp503, 7)) as usize] };
                let mut fTemp505: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp503 as usize] };
                let mut fTemp506: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp503, 1)) as usize] }
                        - fTemp505;
                let mut fTemp507: F64 = 262143.0 * fTemp500;
                let mut iTemp508: i32 = (fTemp507) as i32;
                let mut iTemp509: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp508, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp510: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp509, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp511: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp509 as usize] };
                let mut fTemp512: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp509, 1), 1835007),
                    )) as usize]
                } - fTemp511;
                let mut iTemp513: i32 = (fTemp140
                    > ((if iTemp65 != 0 {
                        fTemp511
                            + fTemp79 * fTemp512
                            + (fTemp507 - (iTemp508) as F64)
                                * (fTemp510
                                    - (fTemp511
                                        + fTemp79
                                            * (fTemp512
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp509, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp510))))
                    } else {
                        1.0 - (fTemp505
                            + fTemp79 * fTemp506
                            + (fTemp501 - (iTemp502) as F64)
                                * (fTemp504
                                    - (fTemp505
                                        + fTemp79
                                            * (fTemp506
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp503, 8)) as usize]
                                                } - fTemp504)))))
                    } - fTemp499)
                        / (1.0 - fTemp499))) as i32;
                let mut fTemp514: F64 = if iTemp513 != 0 { fTemp483 } else { fTemp486 };
                let mut fTemp515: F64 = if iTemp513 != 0 { fTemp486 } else { fTemp484 };
                let mut fTemp516: F64 = fTemp515 + fTemp514;
                let mut fTemp517: F64 = 0.5 * fTemp516;
                let mut fTemp518: F64 = 262143.0 * (1.0 - fTemp517);
                let mut iTemp519: i32 = (fTemp518) as i32;
                let mut iTemp520: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp519, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp521: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp520, 7)) as usize] };
                let mut fTemp522: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp520 as usize] };
                let mut fTemp523: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp520, 1)) as usize] }
                        - fTemp522;
                let mut fTemp524: F64 = 131071.5 * fTemp516;
                let mut iTemp525: i32 = (fTemp524) as i32;
                let mut iTemp526: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp525, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp527: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp526, 7)) as usize] };
                let mut fTemp528: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp526 as usize] };
                let mut fTemp529: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp526, 1)) as usize] }
                        - fTemp528;
                let mut fTemp530: F64 = if iTemp65 != 0 {
                    fTemp528
                        + fTemp79 * fTemp529
                        + (fTemp524 - (iTemp525) as F64)
                            * (fTemp527
                                - (fTemp528
                                    + fTemp79
                                        * (fTemp529
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp526, 8)) as usize]
                                            } - fTemp527))))
                } else {
                    1.0 - (fTemp522
                        + fTemp79 * fTemp523
                        + (fTemp518 - (iTemp519) as F64)
                            * (fTemp521
                                - (fTemp522
                                    + fTemp79
                                        * (fTemp523
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp520, 8)) as usize]
                                            } - fTemp521)))))
                };
                let mut fTemp531: F64 = fTemp84 + fTemp517;
                let mut fTemp532: F64 = 262143.0 * (1.0 - fTemp531);
                let mut iTemp533: i32 = (fTemp532) as i32;
                let mut iTemp534: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp533, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp535: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp534, 7)) as usize] };
                let mut fTemp536: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp534 as usize] };
                let mut fTemp537: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp534, 1)) as usize] }
                        - fTemp536;
                let mut fTemp538: F64 = 262143.0 * fTemp531;
                let mut iTemp539: i32 = (fTemp538) as i32;
                let mut iTemp540: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp539, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp541: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp540, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp542: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp540 as usize] };
                let mut fTemp543: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp540, 1), 1835007),
                    )) as usize]
                } - fTemp542;
                let mut iTemp544: i32 = (fTemp140
                    > ((if iTemp65 != 0 {
                        fTemp542
                            + fTemp79 * fTemp543
                            + (fTemp538 - (iTemp539) as F64)
                                * (fTemp541
                                    - (fTemp542
                                        + fTemp79
                                            * (fTemp543
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp540, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp541))))
                    } else {
                        1.0 - (fTemp536
                            + fTemp79 * fTemp537
                            + (fTemp532 - (iTemp533) as F64)
                                * (fTemp535
                                    - (fTemp536
                                        + fTemp79
                                            * (fTemp537
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp534, 8)) as usize]
                                                } - fTemp535)))))
                    } - fTemp530)
                        / (1.0 - fTemp530))) as i32;
                let mut fTemp545: F64 = if iTemp544 != 0 { fTemp514 } else { fTemp517 };
                let mut fTemp546: F64 = if iTemp544 != 0 { fTemp517 } else { fTemp515 };
                let mut fTemp547: F64 = fTemp546 + fTemp545;
                let mut fTemp548: F64 = 0.5 * fTemp547;
                let mut fTemp549: F64 = 262143.0 * (1.0 - fTemp548);
                let mut iTemp550: i32 = (fTemp549) as i32;
                let mut iTemp551: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp550, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp552: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp551, 7)) as usize] };
                let mut fTemp553: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp551 as usize] };
                let mut fTemp554: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp551, 1)) as usize] }
                        - fTemp553;
                let mut fTemp555: F64 = 131071.5 * fTemp547;
                let mut iTemp556: i32 = (fTemp555) as i32;
                let mut iTemp557: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp556, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp558: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp557, 7)) as usize] };
                let mut fTemp559: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp557 as usize] };
                let mut fTemp560: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp557, 1)) as usize] }
                        - fTemp559;
                let mut fTemp561: F64 = if iTemp65 != 0 {
                    fTemp559
                        + fTemp79 * fTemp560
                        + (fTemp555 - (iTemp556) as F64)
                            * (fTemp558
                                - (fTemp559
                                    + fTemp79
                                        * (fTemp560
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp557, 8)) as usize]
                                            } - fTemp558))))
                } else {
                    1.0 - (fTemp553
                        + fTemp79 * fTemp554
                        + (fTemp549 - (iTemp550) as F64)
                            * (fTemp552
                                - (fTemp553
                                    + fTemp79
                                        * (fTemp554
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp551, 8)) as usize]
                                            } - fTemp552)))))
                };
                let mut fTemp562: F64 = fTemp84 + fTemp548;
                let mut fTemp563: F64 = 262143.0 * (1.0 - fTemp562);
                let mut iTemp564: i32 = (fTemp563) as i32;
                let mut iTemp565: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp564, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp566: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp565, 7)) as usize] };
                let mut fTemp567: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp565 as usize] };
                let mut fTemp568: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp565, 1)) as usize] }
                        - fTemp567;
                let mut fTemp569: F64 = 262143.0 * fTemp562;
                let mut iTemp570: i32 = (fTemp569) as i32;
                let mut iTemp571: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp570, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp572: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp571, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp573: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp571 as usize] };
                let mut fTemp574: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp571, 1), 1835007),
                    )) as usize]
                } - fTemp573;
                let mut iTemp575: i32 = (fTemp140
                    > ((if iTemp65 != 0 {
                        fTemp573
                            + fTemp79 * fTemp574
                            + (fTemp569 - (iTemp570) as F64)
                                * (fTemp572
                                    - (fTemp573
                                        + fTemp79
                                            * (fTemp574
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp571, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp572))))
                    } else {
                        1.0 - (fTemp567
                            + fTemp79 * fTemp568
                            + (fTemp563 - (iTemp564) as F64)
                                * (fTemp566
                                    - (fTemp567
                                        + fTemp79
                                            * (fTemp568
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp565, 8)) as usize]
                                                } - fTemp566)))))
                    } - fTemp561)
                        / (1.0 - fTemp561))) as i32;
                let mut fTemp576: F64 = if iTemp575 != 0 { fTemp545 } else { fTemp548 };
                let mut fTemp577: F64 = if iTemp575 != 0 { fTemp548 } else { fTemp546 };
                let mut fTemp578: F64 = fTemp577 + fTemp576;
                let mut fTemp579: F64 = 0.5 * fTemp578;
                let mut fTemp580: F64 = 262143.0 * (1.0 - fTemp579);
                let mut iTemp581: i32 = (fTemp580) as i32;
                let mut iTemp582: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp581, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp583: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp582, 7)) as usize] };
                let mut fTemp584: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp582 as usize] };
                let mut fTemp585: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp582, 1)) as usize] }
                        - fTemp584;
                let mut fTemp586: F64 = 131071.5 * fTemp578;
                let mut iTemp587: i32 = (fTemp586) as i32;
                let mut iTemp588: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp587, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp589: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp588, 7)) as usize] };
                let mut fTemp590: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp588 as usize] };
                let mut fTemp591: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp588, 1)) as usize] }
                        - fTemp590;
                let mut fTemp592: F64 = if iTemp65 != 0 {
                    fTemp590
                        + fTemp79 * fTemp591
                        + (fTemp586 - (iTemp587) as F64)
                            * (fTemp589
                                - (fTemp590
                                    + fTemp79
                                        * (fTemp591
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp588, 8)) as usize]
                                            } - fTemp589))))
                } else {
                    1.0 - (fTemp584
                        + fTemp79 * fTemp585
                        + (fTemp580 - (iTemp581) as F64)
                            * (fTemp583
                                - (fTemp584
                                    + fTemp79
                                        * (fTemp585
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp582, 8)) as usize]
                                            } - fTemp583)))))
                };
                let mut fTemp593: F64 = fTemp84 + fTemp579;
                let mut fTemp594: F64 = 262143.0 * (1.0 - fTemp593);
                let mut iTemp595: i32 = (fTemp594) as i32;
                let mut iTemp596: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp595, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp597: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp596, 7)) as usize] };
                let mut fTemp598: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp596 as usize] };
                let mut fTemp599: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp596, 1)) as usize] }
                        - fTemp598;
                let mut fTemp600: F64 = 262143.0 * fTemp593;
                let mut iTemp601: i32 = (fTemp600) as i32;
                let mut iTemp602: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp601, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp603: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp602, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp604: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp602 as usize] };
                let mut fTemp605: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp602, 1), 1835007),
                    )) as usize]
                } - fTemp604;
                let mut iTemp606: i32 = (fTemp140
                    > ((if iTemp65 != 0 {
                        fTemp604
                            + fTemp79 * fTemp605
                            + (fTemp600 - (iTemp601) as F64)
                                * (fTemp603
                                    - (fTemp604
                                        + fTemp79
                                            * (fTemp605
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp602, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp603))))
                    } else {
                        1.0 - (fTemp598
                            + fTemp79 * fTemp599
                            + (fTemp594 - (iTemp595) as F64)
                                * (fTemp597
                                    - (fTemp598
                                        + fTemp79
                                            * (fTemp599
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp596, 8)) as usize]
                                                } - fTemp597)))))
                    } - fTemp592)
                        / (1.0 - fTemp592))) as i32;
                let mut fTemp607: F64 = if iTemp606 != 0 { fTemp576 } else { fTemp579 };
                let mut fTemp608: F64 = if iTemp606 != 0 { fTemp579 } else { fTemp577 };
                let mut fTemp609: F64 = fTemp608 + fTemp607;
                let mut fTemp610: F64 = 0.5 * fTemp609;
                let mut fTemp611: F64 = 262143.0 * (1.0 - fTemp610);
                let mut iTemp612: i32 = (fTemp611) as i32;
                let mut iTemp613: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp612, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp614: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp613, 7)) as usize] };
                let mut fTemp615: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp613 as usize] };
                let mut fTemp616: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp613, 1)) as usize] }
                        - fTemp615;
                let mut fTemp617: F64 = 131071.5 * fTemp609;
                let mut iTemp618: i32 = (fTemp617) as i32;
                let mut iTemp619: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp618, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp620: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp619, 7)) as usize] };
                let mut fTemp621: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp619 as usize] };
                let mut fTemp622: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp619, 1)) as usize] }
                        - fTemp621;
                let mut fTemp623: F64 = if iTemp65 != 0 {
                    fTemp621
                        + fTemp79 * fTemp622
                        + (fTemp617 - (iTemp618) as F64)
                            * (fTemp620
                                - (fTemp621
                                    + fTemp79
                                        * (fTemp622
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp619, 8)) as usize]
                                            } - fTemp620))))
                } else {
                    1.0 - (fTemp615
                        + fTemp79 * fTemp616
                        + (fTemp611 - (iTemp612) as F64)
                            * (fTemp614
                                - (fTemp615
                                    + fTemp79
                                        * (fTemp616
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp613, 8)) as usize]
                                            } - fTemp614)))))
                };
                let mut fTemp624: F64 = fTemp84 + fTemp610;
                let mut fTemp625: F64 = 262143.0 * (1.0 - fTemp624);
                let mut iTemp626: i32 = (fTemp625) as i32;
                let mut iTemp627: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp626, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp628: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp627, 7)) as usize] };
                let mut fTemp629: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp627 as usize] };
                let mut fTemp630: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp627, 1)) as usize] }
                        - fTemp629;
                let mut fTemp631: F64 = 262143.0 * fTemp624;
                let mut iTemp632: i32 = (fTemp631) as i32;
                let mut iTemp633: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp632, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp634: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp633, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp635: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp633 as usize] };
                let mut fTemp636: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp633, 1), 1835007),
                    )) as usize]
                } - fTemp635;
                let mut iTemp637: i32 = (fTemp140
                    > ((if iTemp65 != 0 {
                        fTemp635
                            + fTemp79 * fTemp636
                            + (fTemp631 - (iTemp632) as F64)
                                * (fTemp634
                                    - (fTemp635
                                        + fTemp79
                                            * (fTemp636
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp633, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp634))))
                    } else {
                        1.0 - (fTemp629
                            + fTemp79 * fTemp630
                            + (fTemp625 - (iTemp626) as F64)
                                * (fTemp628
                                    - (fTemp629
                                        + fTemp79
                                            * (fTemp630
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp627, 8)) as usize]
                                                } - fTemp628)))))
                    } - fTemp623)
                        / (1.0 - fTemp623))) as i32;
                let mut fTemp638: F64 = if iTemp637 != 0 { fTemp607 } else { fTemp610 };
                let mut fTemp639: F64 = if iTemp637 != 0 { fTemp610 } else { fTemp608 };
                let mut fTemp640: F64 = fTemp639 + fTemp638;
                let mut fTemp641: F64 = 0.5 * fTemp640;
                let mut fTemp642: F64 = 262143.0 * (1.0 - fTemp641);
                let mut iTemp643: i32 = (fTemp642) as i32;
                let mut iTemp644: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp643, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp645: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp644, 7)) as usize] };
                let mut fTemp646: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp644 as usize] };
                let mut fTemp647: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp644, 1)) as usize] }
                        - fTemp646;
                let mut fTemp648: F64 = 131071.5 * fTemp640;
                let mut iTemp649: i32 = (fTemp648) as i32;
                let mut iTemp650: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp649, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp651: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp650, 7)) as usize] };
                let mut fTemp652: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp650 as usize] };
                let mut fTemp653: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp650, 1)) as usize] }
                        - fTemp652;
                let mut fTemp654: F64 = if iTemp65 != 0 {
                    fTemp652
                        + fTemp79 * fTemp653
                        + (fTemp648 - (iTemp649) as F64)
                            * (fTemp651
                                - (fTemp652
                                    + fTemp79
                                        * (fTemp653
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp650, 8),
                                                        1835007,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp651))))
                } else {
                    1.0 - (fTemp646
                        + fTemp79 * fTemp647
                        + (fTemp642 - (iTemp643) as F64)
                            * (fTemp645
                                - (fTemp646
                                    + fTemp79
                                        * (fTemp647
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp644, 8),
                                                        1835007,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp645)))))
                };
                let mut fTemp655: F64 = fTemp84 + fTemp641;
                let mut fTemp656: F64 = 262143.0 * (1.0 - fTemp655);
                let mut iTemp657: i32 = (fTemp656) as i32;
                let mut iTemp658: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp657, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp659: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp658, 7)) as usize] };
                let mut fTemp660: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp658 as usize] };
                let mut fTemp661: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp658, 1)) as usize] }
                        - fTemp660;
                let mut fTemp662: F64 = 262143.0 * fTemp655;
                let mut iTemp663: i32 = (fTemp662) as i32;
                let mut iTemp664: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp663, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp665: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp664, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp666: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp664 as usize] };
                let mut fTemp667: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp664, 1), 1835007),
                    )) as usize]
                } - fTemp666;
                let mut iTemp668: i32 = (fTemp140
                    > ((if iTemp65 != 0 {
                        fTemp666
                            + fTemp79 * fTemp667
                            + (fTemp662 - (iTemp663) as F64)
                                * (fTemp665
                                    - (fTemp666
                                        + fTemp79
                                            * (fTemp667
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp664, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp665))))
                    } else {
                        1.0 - (fTemp660
                            + fTemp79 * fTemp661
                            + (fTemp656 - (iTemp657) as F64)
                                * (fTemp659
                                    - (fTemp660
                                        + fTemp79
                                            * (fTemp661
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp658, 8)) as usize]
                                                } - fTemp659)))))
                    } - fTemp654)
                        / (1.0 - fTemp654))) as i32;
                let mut fTemp669: F64 = F64::min(
                    1.0,
                    F64::max(
                        0.0,
                        0.5 * (if iTemp668 != 0 { fTemp641 } else { fTemp639 }
                            + if iTemp668 != 0 { fTemp638 } else { fTemp641 }),
                    ),
                );
                self.fRec1[0] = fTemp669;
                let mut fTemp670: F64 = 262143.0 * (1.0 - fTemp669);
                let mut iTemp671: i32 = (fTemp670) as i32;
                let mut iTemp672: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp671, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp673: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp672, 7)) as usize] };
                let mut fTemp674: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp672 as usize] };
                let mut fTemp675: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp672, 1)) as usize] }
                        - fTemp674;
                let mut fTemp676: F64 = 262143.0 * fTemp669;
                let mut iTemp677: i32 = (fTemp676) as i32;
                let mut iTemp678: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp677, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp679: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp678, 7)) as usize] };
                let mut fTemp680: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp678 as usize] };
                let mut fTemp681: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp678, 1)) as usize] }
                        - fTemp680;
                let mut fTemp682: F64 = if iTemp65 != 0 {
                    fTemp680
                        + fTemp79 * fTemp681
                        + (fTemp676 - (iTemp677) as F64)
                            * (fTemp679
                                - (fTemp680
                                    + fTemp79
                                        * (fTemp681
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp678, 8),
                                                        1835007,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp679))))
                } else {
                    1.0 - (fTemp674
                        + fTemp79 * fTemp675
                        + (fTemp670 - (iTemp671) as F64)
                            * (fTemp673
                                - (fTemp674
                                    + fTemp79
                                        * (fTemp675
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp672, 8),
                                                        1835007,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp673)))))
                };
                let mut fTemp683: F64 = fTemp84 + fTemp669;
                let mut fTemp684: F64 = 262143.0 * (1.0 - fTemp683);
                let mut iTemp685: i32 = (fTemp684) as i32;
                let mut iTemp686: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp685, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp687: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp686, 7)) as usize] };
                let mut fTemp688: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp686 as usize] };
                let mut fTemp689: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp686, 1)) as usize] }
                        - fTemp688;
                let mut fTemp690: F64 = 262143.0 * fTemp683;
                let mut iTemp691: i32 = (fTemp690) as i32;
                let mut iTemp692: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp74,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp691, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp693: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp692, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp694: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp692 as usize] };
                let mut fTemp695: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp692, 1), 1835007),
                    )) as usize]
                } - fTemp694;
                let mut fTemp696: F64 = self.fRec2[1]
                    + if ((0.001 * fTemp83) == 0.0) as i32 != 0 {
                        fTemp64
                    } else {
                        fTemp64
                            * (if iTemp65 != 0 {
                                fTemp694
                                    + fTemp79 * fTemp695
                                    + (fTemp690 - (iTemp691) as F64)
                                        * (fTemp693
                                            - (fTemp694
                                                + fTemp79
                                                    * (fTemp695
                                                        - (unsafe {
                                                            ftbl0LambRs96kSIG0[(std::cmp::max(
                                                                0,
                                                                std::cmp::min(
                                                                    i32::wrapping_add(iTemp692, 8),
                                                                    1835007,
                                                                ),
                                                            ))
                                                                as usize]
                                                        } - fTemp693))))
                            } else {
                                1.0 - (fTemp688
                                    + fTemp79 * fTemp689
                                    + (fTemp684 - (iTemp685) as F64)
                                        * (fTemp687
                                            - (fTemp688
                                                + fTemp79
                                                    * (fTemp689
                                                        - (unsafe {
                                                            ftbl0LambRs96kSIG0[(i32::wrapping_add(
                                                                iTemp686, 8,
                                                            ))
                                                                as usize]
                                                        } - fTemp687)))))
                            } - fTemp682)
                            / (1.0 - fTemp682)
                    };
                self.fRec2[0] = if iTemp82 != 0 {
                    F64::min(fTemp696, self.fRec2[1])
                } else {
                    F64::max(fTemp696, self.fRec2[1])
                };
                self.fVec33[(self.IOTA0 & 16383) as usize] = F64::powf(1e+01, 0.05 * self.fRec2[0]);
                let mut fTemp697: F64 =
                    self.fVec33[((i32::wrapping_sub(self.IOTA0, iSlow74)) & 16383) as usize];
                self.fRec14[0] = fSlow76 + self.fConst4 * self.fRec14[1];
                *output0 = 0.5
                    * self.fVec1[((i32::wrapping_sub(self.IOTA0, iSlow75)) & 32767) as usize]
                    * fTemp2
                    + self.fRec14[0]
                        * self.fVec2[((i32::wrapping_sub(self.IOTA0, iSlow75)) & 32767) as usize]
                        * fTemp697
                        * fTemp4;
                let mut fTemp698: F64 = fTemp36 + fSlow17 * (fTemp37 - fTemp36);
                let mut iTemp699: i32 =
                    ((fTemp698 > fSlow11) as i32) + ((fTemp698 > fSlow9) as i32);
                let mut fTemp700: F64 = fTemp698 - fSlow8;
                let mut fTemp701: F64 = F64::min(
                    fTemp34,
                    -(fSlow18
                        * F64::max(
                            0.0,
                            if (iTemp699 == 0) as i32 != 0 {
                                0.0
                            } else {
                                if (iTemp699 == 1) as i32 != 0 {
                                    fSlow12 * LambRs96k_faustpower2_f(fSlow7 + fTemp700)
                                } else {
                                    fTemp700
                                }
                            },
                        )),
                );
                self.fVec34[(self.IOTA0 & 16383) as usize] = fTemp701;
                let mut fTemp702: F64 = F64::min(
                    fTemp701,
                    self.fVec34[((i32::wrapping_sub(self.IOTA0, 1)) & 16383) as usize],
                );
                self.fVec35[0] = fTemp702;
                let mut fTemp703: F64 = F64::min(fTemp702, self.fVec35[2]);
                self.fVec36[0] = fTemp703;
                let mut fTemp704: F64 = F64::min(fTemp703, self.fVec36[4]);
                self.fVec37[0] = fTemp704;
                let mut fTemp705: F64 = F64::min(fTemp704, self.fVec37[8]);
                self.fVec38[(self.IOTA0 & 31) as usize] = fTemp705;
                let mut fTemp706: F64 = F64::min(
                    fTemp705,
                    self.fVec38[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize],
                );
                self.fVec39[(self.IOTA0 & 63) as usize] = fTemp706;
                let mut fTemp707: F64 = F64::min(
                    fTemp706,
                    self.fVec39[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize],
                );
                self.fVec40[(self.IOTA0 & 127) as usize] = fTemp707;
                let mut fTemp708: F64 = F64::min(
                    fTemp707,
                    self.fVec40[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize],
                );
                self.fVec41[(self.IOTA0 & 255) as usize] = fTemp708;
                let mut fTemp709: F64 = F64::min(
                    fTemp708,
                    self.fVec41[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize],
                );
                self.fVec42[(self.IOTA0 & 511) as usize] = fTemp709;
                let mut fTemp710: F64 = F64::min(
                    fTemp709,
                    self.fVec42[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize],
                );
                self.fVec43[(self.IOTA0 & 1023) as usize] = fTemp710;
                let mut fTemp711: F64 = F64::min(
                    fTemp710,
                    self.fVec43[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize],
                );
                self.fVec44[(self.IOTA0 & 2047) as usize] = fTemp711;
                let mut fTemp712: F64 = F64::min(
                    fTemp711,
                    self.fVec44[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize],
                );
                self.fVec45[(self.IOTA0 & 4095) as usize] = fTemp712;
                self.fVec46[(self.IOTA0 & 8191) as usize] = F64::min(
                    fTemp712,
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
                                                                    if iSlow23 != 0 {
                                                                        fTemp701
                                                                    } else {
                                                                        1.7976931348623157e+308
                                                                    },
                                                                    if iSlow24 != 0 {
                                                                        self.fVec35
                                                                            [iSlow23 as usize]
                                                                    } else {
                                                                        1.7976931348623157e+308
                                                                    },
                                                                ),
                                                                if iSlow25 != 0 {
                                                                    self.fVec36[iSlow26 as usize]
                                                                } else {
                                                                    1.7976931348623157e+308
                                                                },
                                                            ),
                                                            if iSlow27 != 0 {
                                                                self.fVec37[iSlow28 as usize]
                                                            } else {
                                                                1.7976931348623157e+308
                                                            },
                                                        ),
                                                        if iSlow29 != 0 {
                                                            self.fVec38[((i32::wrapping_sub(
                                                                self.IOTA0, iSlow30,
                                                            )) & 31)
                                                                as usize]
                                                        } else {
                                                            1.7976931348623157e+308
                                                        },
                                                    ),
                                                    if iSlow31 != 0 {
                                                        self.fVec39[((i32::wrapping_sub(
                                                            self.IOTA0, iSlow32,
                                                        )) & 63)
                                                            as usize]
                                                    } else {
                                                        1.7976931348623157e+308
                                                    },
                                                ),
                                                if iSlow33 != 0 {
                                                    self.fVec40[((i32::wrapping_sub(
                                                        self.IOTA0, iSlow34,
                                                    )) & 127)
                                                        as usize]
                                                } else {
                                                    1.7976931348623157e+308
                                                },
                                            ),
                                            if iSlow35 != 0 {
                                                self.fVec41[((i32::wrapping_sub(
                                                    self.IOTA0, iSlow36,
                                                )) & 255)
                                                    as usize]
                                            } else {
                                                1.7976931348623157e+308
                                            },
                                        ),
                                        if iSlow37 != 0 {
                                            self.fVec42[((i32::wrapping_sub(self.IOTA0, iSlow38))
                                                & 511)
                                                as usize]
                                        } else {
                                            1.7976931348623157e+308
                                        },
                                    ),
                                    if iSlow39 != 0 {
                                        self.fVec43[((i32::wrapping_sub(self.IOTA0, iSlow40))
                                            & 1023)
                                            as usize]
                                    } else {
                                        1.7976931348623157e+308
                                    },
                                ),
                                if iSlow41 != 0 {
                                    self.fVec44
                                        [((i32::wrapping_sub(self.IOTA0, iSlow42)) & 2047) as usize]
                                } else {
                                    1.7976931348623157e+308
                                },
                            ),
                            if iSlow43 != 0 {
                                self.fVec45
                                    [((i32::wrapping_sub(self.IOTA0, iSlow44)) & 4095) as usize]
                            } else {
                                1.7976931348623157e+308
                            },
                        ),
                        if iSlow45 != 0 {
                            self.fVec46[((i32::wrapping_sub(self.IOTA0, iSlow46)) & 8191) as usize]
                        } else {
                            1.7976931348623157e+308
                        },
                    ),
                );
                let mut fTemp713: F64 = F64::min(self.fRec17[0], self.fRec17[1]);
                self.fVec47[0] = fTemp713;
                let mut fTemp714: F64 = F64::min(fTemp713, self.fVec47[2]);
                self.fVec48[0] = fTemp714;
                let mut fTemp715: F64 = F64::min(fTemp714, self.fVec48[4]);
                self.fVec49[0] = fTemp715;
                let mut fTemp716: F64 = F64::min(fTemp715, self.fVec49[8]);
                self.fVec50[(self.IOTA0 & 31) as usize] = fTemp716;
                let mut fTemp717: F64 = F64::min(
                    fTemp716,
                    self.fVec50[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize],
                );
                self.fVec51[(self.IOTA0 & 63) as usize] = fTemp717;
                let mut fTemp718: F64 = F64::min(
                    fTemp717,
                    self.fVec51[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize],
                );
                self.fVec52[(self.IOTA0 & 127) as usize] = fTemp718;
                let mut fTemp719: F64 = F64::min(
                    fTemp718,
                    self.fVec52[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize],
                );
                self.fVec53[(self.IOTA0 & 255) as usize] = fTemp719;
                let mut fTemp720: F64 = F64::min(
                    fTemp719,
                    self.fVec53[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize],
                );
                self.fVec54[(self.IOTA0 & 511) as usize] = fTemp720;
                let mut fTemp721: F64 = F64::min(
                    fTemp720,
                    self.fVec54[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize],
                );
                self.fVec55[(self.IOTA0 & 1023) as usize] = fTemp721;
                let mut fTemp722: F64 = F64::min(
                    fTemp721,
                    self.fVec55[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize],
                );
                self.fVec56[(self.IOTA0 & 2047) as usize] = fTemp722;
                let mut fTemp723: F64 = F64::min(
                    fTemp722,
                    self.fVec56[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize],
                );
                self.fVec57[(self.IOTA0 & 4095) as usize] = fTemp723;
                self.fVec58[(self.IOTA0 & 8191) as usize] = F64::min(
                    fTemp723,
                    self.fVec57[((i32::wrapping_sub(self.IOTA0, 2048)) & 4095) as usize],
                );
                let mut fTemp724: F64 = F64::min(
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
                                                                if iSlow4 != 0 {
                                                                    self.fRec17[0]
                                                                } else {
                                                                    1.7976931348623157e+308
                                                                },
                                                                if iSlow47 != 0 {
                                                                    self.fVec47[iSlow4 as usize]
                                                                } else {
                                                                    1.7976931348623157e+308
                                                                },
                                                            ),
                                                            if iSlow48 != 0 {
                                                                self.fVec48[iSlow49 as usize]
                                                            } else {
                                                                1.7976931348623157e+308
                                                            },
                                                        ),
                                                        if iSlow50 != 0 {
                                                            self.fVec49[iSlow51 as usize]
                                                        } else {
                                                            1.7976931348623157e+308
                                                        },
                                                    ),
                                                    if iSlow52 != 0 {
                                                        self.fVec50[((i32::wrapping_sub(
                                                            self.IOTA0, iSlow53,
                                                        )) & 31)
                                                            as usize]
                                                    } else {
                                                        1.7976931348623157e+308
                                                    },
                                                ),
                                                if iSlow54 != 0 {
                                                    self.fVec51[((i32::wrapping_sub(
                                                        self.IOTA0, iSlow55,
                                                    )) & 63)
                                                        as usize]
                                                } else {
                                                    1.7976931348623157e+308
                                                },
                                            ),
                                            if iSlow56 != 0 {
                                                self.fVec52[((i32::wrapping_sub(
                                                    self.IOTA0, iSlow57,
                                                )) & 127)
                                                    as usize]
                                            } else {
                                                1.7976931348623157e+308
                                            },
                                        ),
                                        if iSlow58 != 0 {
                                            self.fVec53[((i32::wrapping_sub(self.IOTA0, iSlow59))
                                                & 255)
                                                as usize]
                                        } else {
                                            1.7976931348623157e+308
                                        },
                                    ),
                                    if iSlow60 != 0 {
                                        self.fVec54[((i32::wrapping_sub(self.IOTA0, iSlow61)) & 511)
                                            as usize]
                                    } else {
                                        1.7976931348623157e+308
                                    },
                                ),
                                if iSlow62 != 0 {
                                    self.fVec55
                                        [((i32::wrapping_sub(self.IOTA0, iSlow63)) & 1023) as usize]
                                } else {
                                    1.7976931348623157e+308
                                },
                            ),
                            if iSlow64 != 0 {
                                self.fVec56
                                    [((i32::wrapping_sub(self.IOTA0, iSlow65)) & 2047) as usize]
                            } else {
                                1.7976931348623157e+308
                            },
                        ),
                        if iSlow66 != 0 {
                            self.fVec57[((i32::wrapping_sub(self.IOTA0, iSlow67)) & 4095) as usize]
                        } else {
                            1.7976931348623157e+308
                        },
                    ),
                    if iSlow68 != 0 {
                        self.fVec58[((i32::wrapping_sub(self.IOTA0, iSlow69)) & 8191) as usize]
                    } else {
                        1.7976931348623157e+308
                    },
                ) - self.fRec16[1];
                self.fVec59[0] = fTemp724;
                let mut iTemp725: i32 = (fTemp724 > 0.0) as i32;
                let mut fTemp726: F64 = if iTemp725 != 0 { fSlow71 } else { fSlow70 };
                self.fVec60[0] = fTemp726;
                let mut fTemp727: F64 = 6.0 * fTemp726;
                let mut iTemp728: i32 = (fTemp727) as i32;
                let mut iTemp729: i32 = std::cmp::max(0, std::cmp::min(iTemp728, 6));
                let mut iTemp730: i32 = std::cmp::max(
                    0,
                    std::cmp::min(i32::wrapping_add(iTemp729, 917497), 1835007),
                );
                let mut fTemp731: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp730, 7)) as usize] };
                let mut fTemp732: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp730 as usize] };
                let mut fTemp733: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp730, 1)) as usize] }
                        - fTemp732;
                let mut fTemp734: F64 = fTemp727 - (iTemp728) as F64;
                let mut fTemp735: F64 = fTemp732
                    + fTemp734 * fTemp733
                    + 0.5
                        * (fTemp731
                            - (fTemp732
                                + fTemp734
                                    * (fTemp733
                                        - (unsafe {
                                            ftbl0LambRs96kSIG0
                                                [(i32::wrapping_add(iTemp730, 8)) as usize]
                                        } - fTemp731))));
                let mut fTemp736: F64 = if iTemp725 != 0 {
                    fTemp735
                } else {
                    1.0 - fTemp735
                };
                let mut iTemp737: i32 = (fTemp724 < 0.0) as i32;
                let mut fTemp738: F64 = fSlow1 * (iTemp737) as F64 + fSlow13 * (iTemp725) as F64;
                self.fVec61[0] = fTemp738;
                let mut fTemp739: F64 = self.fConst10 / fTemp738;
                let mut fTemp740: F64 = fTemp739 + 0.5;
                let mut fTemp741: F64 = 262143.0 * (1.0 - fTemp740);
                let mut iTemp742: i32 = (fTemp741) as i32;
                let mut iTemp743: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp742, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp744: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp743, 7)) as usize] };
                let mut fTemp745: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp743 as usize] };
                let mut fTemp746: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp743, 1)) as usize] }
                        - fTemp745;
                let mut fTemp747: F64 = 262143.0 * fTemp740;
                let mut iTemp748: i32 = (fTemp747) as i32;
                let mut iTemp749: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp748, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp750: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp749, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp751: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp749 as usize] };
                let mut fTemp752: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp749, 1), 1835007),
                    )) as usize]
                } - fTemp751;
                let mut fTemp753: F64 = 6.0 * self.fVec60[1];
                let mut iTemp754: i32 = (fTemp753) as i32;
                let mut iTemp755: i32 = std::cmp::max(0, std::cmp::min(iTemp754, 6));
                let mut fTemp756: F64 = 262143.0 * (1.0 - self.fRec15[1]);
                let mut iTemp757: i32 = (fTemp756) as i32;
                let mut iTemp758: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp757, 262143))),
                            iTemp755,
                        ),
                        1835007,
                    ),
                );
                let mut fTemp759: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp758, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp760: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp758 as usize] };
                let mut fTemp761: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp758, 1), 1835007),
                    )) as usize]
                } - fTemp760;
                let mut fTemp762: F64 = fTemp753 - (iTemp754) as F64;
                let mut fTemp763: F64 = 262143.0 * self.fRec15[1];
                let mut iTemp764: i32 = (fTemp763) as i32;
                let mut iTemp765: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp755,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp764, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp766: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp765, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp767: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp765 as usize] };
                let mut fTemp768: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp765, 1), 1835007),
                    )) as usize]
                } - fTemp767;
                let mut fTemp769: F64 = self.fRec15[1] + fTemp739;
                let mut fTemp770: F64 = 262143.0 * (1.0 - fTemp769);
                let mut iTemp771: i32 = (fTemp770) as i32;
                let mut iTemp772: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp771, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp773: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp772, 7)) as usize] };
                let mut fTemp774: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp772 as usize] };
                let mut fTemp775: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp772, 1)) as usize] }
                        - fTemp774;
                let mut fTemp776: F64 = 262143.0 * fTemp769;
                let mut iTemp777: i32 = (fTemp776) as i32;
                let mut iTemp778: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp777, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp779: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp778, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp780: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp778 as usize] };
                let mut fTemp781: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp778, 1), 1835007),
                    )) as usize]
                } - fTemp780;
                let mut fTemp782: F64 =
                    self.fRec15[1] + self.fConst10 * (1.0 / fTemp738 + 1.0 / self.fVec61[1]);
                let mut fTemp783: F64 = 262143.0 * (1.0 - fTemp782);
                let mut iTemp784: i32 = (fTemp783) as i32;
                let mut iTemp785: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp784, 262143))),
                            iTemp729,
                        ),
                        1835007,
                    ),
                );
                let mut fTemp786: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp785, 7)) as usize] };
                let mut fTemp787: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp785 as usize] };
                let mut fTemp788: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp785, 1)) as usize] }
                        - fTemp787;
                let mut fTemp789: F64 = 262143.0 * fTemp782;
                let mut iTemp790: i32 = (fTemp789) as i32;
                let mut iTemp791: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp790, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp792: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp791, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp793: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp791 as usize] };
                let mut fTemp794: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp791, 1), 1835007),
                    )) as usize]
                } - fTemp793;
                let mut fTemp795: F64 = (if iTemp725 != 0 {
                    fTemp793
                        + fTemp734 * fTemp794
                        + (fTemp789 - (iTemp790) as F64)
                            * (fTemp792
                                - (fTemp793
                                    + fTemp734
                                        * (fTemp794
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp791, 8),
                                                        1835007,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp792))))
                } else {
                    1.0 - (fTemp787
                        + fTemp734 * fTemp788
                        + (fTemp783 - (iTemp784) as F64)
                            * (fTemp786
                                - (fTemp787
                                    + fTemp734
                                        * (fTemp788
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp785, 8)) as usize]
                                            } - fTemp786)))))
                } - if iTemp725 != 0 {
                    fTemp780
                        + fTemp734 * fTemp781
                        + (fTemp776 - (iTemp777) as F64)
                            * (fTemp779
                                - (fTemp780
                                    + fTemp734
                                        * (fTemp781
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp778, 8),
                                                        1835007,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp779))))
                } else {
                    1.0 - (fTemp774
                        + fTemp734 * fTemp775
                        + (fTemp770 - (iTemp771) as F64)
                            * (fTemp773
                                - (fTemp774
                                    + fTemp734
                                        * (fTemp775
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp772, 8)) as usize]
                                            } - fTemp773)))))
                }) * self.fVec59[1]
                    / (fTemp724
                        * (1.0
                            - if iTemp725 != 0 {
                                fTemp767
                                    + fTemp762 * fTemp768
                                    + (fTemp763 - (iTemp764) as F64)
                                        * (fTemp766
                                            - (fTemp767
                                                + fTemp762
                                                    * (fTemp768
                                                        - (unsafe {
                                                            ftbl0LambRs96kSIG0[(std::cmp::max(
                                                                0,
                                                                std::cmp::min(
                                                                    i32::wrapping_add(iTemp765, 8),
                                                                    1835007,
                                                                ),
                                                            ))
                                                                as usize]
                                                        } - fTemp766))))
                            } else {
                                1.0 - (fTemp760
                                    + fTemp762 * fTemp761
                                    + (fTemp756 - (iTemp757) as F64)
                                        * (fTemp759
                                            - (fTemp760
                                                + fTemp762
                                                    * (fTemp761
                                                        - (unsafe {
                                                            ftbl0LambRs96kSIG0[(std::cmp::max(
                                                                0,
                                                                std::cmp::min(
                                                                    i32::wrapping_add(iTemp758, 8),
                                                                    1835007,
                                                                ),
                                                            ))
                                                                as usize]
                                                        } - fTemp759)))))
                            }));
                let mut iTemp796: i32 = (fTemp795
                    > ((if iTemp725 != 0 {
                        fTemp751
                            + fTemp734 * fTemp752
                            + (fTemp747 - (iTemp748) as F64)
                                * (fTemp750
                                    - (fTemp751
                                        + fTemp734
                                            * (fTemp752
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp749, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp750))))
                    } else {
                        1.0 - (fTemp745
                            + fTemp734 * fTemp746
                            + (fTemp741 - (iTemp742) as F64)
                                * (fTemp744
                                    - (fTemp745
                                        + fTemp734
                                            * (fTemp746
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp743, 8)) as usize]
                                                } - fTemp744)))))
                    } - fTemp736)
                        / (1.0 - fTemp736))) as i32;
                let mut fTemp797: F64 = if iTemp796 != 0 { 1.0 } else { 0.5 };
                let mut fTemp798: F64 = if iTemp796 != 0 { 0.5 } else { 0.0 };
                let mut fTemp799: F64 = fTemp798 + fTemp797;
                let mut fTemp800: F64 = 0.5 * fTemp799;
                let mut fTemp801: F64 = 262143.0 * (1.0 - fTemp800);
                let mut iTemp802: i32 = (fTemp801) as i32;
                let mut iTemp803: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp802, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp804: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp803, 7)) as usize] };
                let mut fTemp805: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp803 as usize] };
                let mut fTemp806: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp803, 1)) as usize] }
                        - fTemp805;
                let mut fTemp807: F64 = 131071.5 * fTemp799;
                let mut iTemp808: i32 = (fTemp807) as i32;
                let mut iTemp809: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp808, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp810: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp809, 7)) as usize] };
                let mut fTemp811: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp809 as usize] };
                let mut fTemp812: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp809, 1)) as usize] }
                        - fTemp811;
                let mut fTemp813: F64 = if iTemp725 != 0 {
                    fTemp811
                        + fTemp734 * fTemp812
                        + (fTemp807 - (iTemp808) as F64)
                            * (fTemp810
                                - (fTemp811
                                    + fTemp734
                                        * (fTemp812
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp809, 8)) as usize]
                                            } - fTemp810))))
                } else {
                    1.0 - (fTemp805
                        + fTemp734 * fTemp806
                        + (fTemp801 - (iTemp802) as F64)
                            * (fTemp804
                                - (fTemp805
                                    + fTemp734
                                        * (fTemp806
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp803, 8)) as usize]
                                            } - fTemp804)))))
                };
                let mut fTemp814: F64 = fTemp739 + fTemp800;
                let mut fTemp815: F64 = 262143.0 * (1.0 - fTemp814);
                let mut iTemp816: i32 = (fTemp815) as i32;
                let mut iTemp817: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp816, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp818: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp817, 7)) as usize] };
                let mut fTemp819: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp817 as usize] };
                let mut fTemp820: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp817, 1)) as usize] }
                        - fTemp819;
                let mut fTemp821: F64 = 262143.0 * fTemp814;
                let mut iTemp822: i32 = (fTemp821) as i32;
                let mut iTemp823: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp822, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp824: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp823, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp825: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp823 as usize] };
                let mut fTemp826: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp823, 1), 1835007),
                    )) as usize]
                } - fTemp825;
                let mut iTemp827: i32 = (fTemp795
                    > ((if iTemp725 != 0 {
                        fTemp825
                            + fTemp734 * fTemp826
                            + (fTemp821 - (iTemp822) as F64)
                                * (fTemp824
                                    - (fTemp825
                                        + fTemp734
                                            * (fTemp826
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp823, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp824))))
                    } else {
                        1.0 - (fTemp819
                            + fTemp734 * fTemp820
                            + (fTemp815 - (iTemp816) as F64)
                                * (fTemp818
                                    - (fTemp819
                                        + fTemp734
                                            * (fTemp820
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp817, 8)) as usize]
                                                } - fTemp818)))))
                    } - fTemp813)
                        / (1.0 - fTemp813))) as i32;
                let mut fTemp828: F64 = if iTemp827 != 0 { fTemp797 } else { fTemp800 };
                let mut fTemp829: F64 = if iTemp827 != 0 { fTemp800 } else { fTemp798 };
                let mut fTemp830: F64 = fTemp829 + fTemp828;
                let mut fTemp831: F64 = 0.5 * fTemp830;
                let mut fTemp832: F64 = 262143.0 * (1.0 - fTemp831);
                let mut iTemp833: i32 = (fTemp832) as i32;
                let mut iTemp834: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp833, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp835: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp834, 7)) as usize] };
                let mut fTemp836: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp834 as usize] };
                let mut fTemp837: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp834, 1)) as usize] }
                        - fTemp836;
                let mut fTemp838: F64 = 131071.5 * fTemp830;
                let mut iTemp839: i32 = (fTemp838) as i32;
                let mut iTemp840: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp839, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp841: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp840, 7)) as usize] };
                let mut fTemp842: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp840 as usize] };
                let mut fTemp843: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp840, 1)) as usize] }
                        - fTemp842;
                let mut fTemp844: F64 = if iTemp725 != 0 {
                    fTemp842
                        + fTemp734 * fTemp843
                        + (fTemp838 - (iTemp839) as F64)
                            * (fTemp841
                                - (fTemp842
                                    + fTemp734
                                        * (fTemp843
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp840, 8)) as usize]
                                            } - fTemp841))))
                } else {
                    1.0 - (fTemp836
                        + fTemp734 * fTemp837
                        + (fTemp832 - (iTemp833) as F64)
                            * (fTemp835
                                - (fTemp836
                                    + fTemp734
                                        * (fTemp837
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp834, 8)) as usize]
                                            } - fTemp835)))))
                };
                let mut fTemp845: F64 = fTemp739 + fTemp831;
                let mut fTemp846: F64 = 262143.0 * (1.0 - fTemp845);
                let mut iTemp847: i32 = (fTemp846) as i32;
                let mut iTemp848: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp847, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp849: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp848, 7)) as usize] };
                let mut fTemp850: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp848 as usize] };
                let mut fTemp851: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp848, 1)) as usize] }
                        - fTemp850;
                let mut fTemp852: F64 = 262143.0 * fTemp845;
                let mut iTemp853: i32 = (fTemp852) as i32;
                let mut iTemp854: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp853, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp855: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp854, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp856: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp854 as usize] };
                let mut fTemp857: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp854, 1), 1835007),
                    )) as usize]
                } - fTemp856;
                let mut iTemp858: i32 = (fTemp795
                    > ((if iTemp725 != 0 {
                        fTemp856
                            + fTemp734 * fTemp857
                            + (fTemp852 - (iTemp853) as F64)
                                * (fTemp855
                                    - (fTemp856
                                        + fTemp734
                                            * (fTemp857
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp854, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp855))))
                    } else {
                        1.0 - (fTemp850
                            + fTemp734 * fTemp851
                            + (fTemp846 - (iTemp847) as F64)
                                * (fTemp849
                                    - (fTemp850
                                        + fTemp734
                                            * (fTemp851
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp848, 8)) as usize]
                                                } - fTemp849)))))
                    } - fTemp844)
                        / (1.0 - fTemp844))) as i32;
                let mut fTemp859: F64 = if iTemp858 != 0 { fTemp828 } else { fTemp831 };
                let mut fTemp860: F64 = if iTemp858 != 0 { fTemp831 } else { fTemp829 };
                let mut fTemp861: F64 = fTemp860 + fTemp859;
                let mut fTemp862: F64 = 0.5 * fTemp861;
                let mut fTemp863: F64 = 262143.0 * (1.0 - fTemp862);
                let mut iTemp864: i32 = (fTemp863) as i32;
                let mut iTemp865: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp864, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp866: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp865, 7)) as usize] };
                let mut fTemp867: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp865 as usize] };
                let mut fTemp868: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp865, 1)) as usize] }
                        - fTemp867;
                let mut fTemp869: F64 = 131071.5 * fTemp861;
                let mut iTemp870: i32 = (fTemp869) as i32;
                let mut iTemp871: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp870, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp872: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp871, 7)) as usize] };
                let mut fTemp873: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp871 as usize] };
                let mut fTemp874: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp871, 1)) as usize] }
                        - fTemp873;
                let mut fTemp875: F64 = if iTemp725 != 0 {
                    fTemp873
                        + fTemp734 * fTemp874
                        + (fTemp869 - (iTemp870) as F64)
                            * (fTemp872
                                - (fTemp873
                                    + fTemp734
                                        * (fTemp874
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp871, 8)) as usize]
                                            } - fTemp872))))
                } else {
                    1.0 - (fTemp867
                        + fTemp734 * fTemp868
                        + (fTemp863 - (iTemp864) as F64)
                            * (fTemp866
                                - (fTemp867
                                    + fTemp734
                                        * (fTemp868
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp865, 8)) as usize]
                                            } - fTemp866)))))
                };
                let mut fTemp876: F64 = fTemp739 + fTemp862;
                let mut fTemp877: F64 = 262143.0 * (1.0 - fTemp876);
                let mut iTemp878: i32 = (fTemp877) as i32;
                let mut iTemp879: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp878, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp880: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp879, 7)) as usize] };
                let mut fTemp881: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp879 as usize] };
                let mut fTemp882: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp879, 1)) as usize] }
                        - fTemp881;
                let mut fTemp883: F64 = 262143.0 * fTemp876;
                let mut iTemp884: i32 = (fTemp883) as i32;
                let mut iTemp885: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp884, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp886: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp885, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp887: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp885 as usize] };
                let mut fTemp888: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp885, 1), 1835007),
                    )) as usize]
                } - fTemp887;
                let mut iTemp889: i32 = (fTemp795
                    > ((if iTemp725 != 0 {
                        fTemp887
                            + fTemp734 * fTemp888
                            + (fTemp883 - (iTemp884) as F64)
                                * (fTemp886
                                    - (fTemp887
                                        + fTemp734
                                            * (fTemp888
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp885, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp886))))
                    } else {
                        1.0 - (fTemp881
                            + fTemp734 * fTemp882
                            + (fTemp877 - (iTemp878) as F64)
                                * (fTemp880
                                    - (fTemp881
                                        + fTemp734
                                            * (fTemp882
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp879, 8)) as usize]
                                                } - fTemp880)))))
                    } - fTemp875)
                        / (1.0 - fTemp875))) as i32;
                let mut fTemp890: F64 = if iTemp889 != 0 { fTemp859 } else { fTemp862 };
                let mut fTemp891: F64 = if iTemp889 != 0 { fTemp862 } else { fTemp860 };
                let mut fTemp892: F64 = fTemp891 + fTemp890;
                let mut fTemp893: F64 = 0.5 * fTemp892;
                let mut fTemp894: F64 = 262143.0 * (1.0 - fTemp893);
                let mut iTemp895: i32 = (fTemp894) as i32;
                let mut iTemp896: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp895, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp897: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp896, 7)) as usize] };
                let mut fTemp898: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp896 as usize] };
                let mut fTemp899: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp896, 1)) as usize] }
                        - fTemp898;
                let mut fTemp900: F64 = 131071.5 * fTemp892;
                let mut iTemp901: i32 = (fTemp900) as i32;
                let mut iTemp902: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp901, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp903: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp902, 7)) as usize] };
                let mut fTemp904: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp902 as usize] };
                let mut fTemp905: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp902, 1)) as usize] }
                        - fTemp904;
                let mut fTemp906: F64 = if iTemp725 != 0 {
                    fTemp904
                        + fTemp734 * fTemp905
                        + (fTemp900 - (iTemp901) as F64)
                            * (fTemp903
                                - (fTemp904
                                    + fTemp734
                                        * (fTemp905
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp902, 8)) as usize]
                                            } - fTemp903))))
                } else {
                    1.0 - (fTemp898
                        + fTemp734 * fTemp899
                        + (fTemp894 - (iTemp895) as F64)
                            * (fTemp897
                                - (fTemp898
                                    + fTemp734
                                        * (fTemp899
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp896, 8)) as usize]
                                            } - fTemp897)))))
                };
                let mut fTemp907: F64 = fTemp739 + fTemp893;
                let mut fTemp908: F64 = 262143.0 * (1.0 - fTemp907);
                let mut iTemp909: i32 = (fTemp908) as i32;
                let mut iTemp910: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp909, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp911: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp910, 7)) as usize] };
                let mut fTemp912: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp910 as usize] };
                let mut fTemp913: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp910, 1)) as usize] }
                        - fTemp912;
                let mut fTemp914: F64 = 262143.0 * fTemp907;
                let mut iTemp915: i32 = (fTemp914) as i32;
                let mut iTemp916: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp915, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp917: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp916, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp918: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp916 as usize] };
                let mut fTemp919: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp916, 1), 1835007),
                    )) as usize]
                } - fTemp918;
                let mut iTemp920: i32 = (fTemp795
                    > ((if iTemp725 != 0 {
                        fTemp918
                            + fTemp734 * fTemp919
                            + (fTemp914 - (iTemp915) as F64)
                                * (fTemp917
                                    - (fTemp918
                                        + fTemp734
                                            * (fTemp919
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp916, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp917))))
                    } else {
                        1.0 - (fTemp912
                            + fTemp734 * fTemp913
                            + (fTemp908 - (iTemp909) as F64)
                                * (fTemp911
                                    - (fTemp912
                                        + fTemp734
                                            * (fTemp913
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp910, 8)) as usize]
                                                } - fTemp911)))))
                    } - fTemp906)
                        / (1.0 - fTemp906))) as i32;
                let mut fTemp921: F64 = if iTemp920 != 0 { fTemp890 } else { fTemp893 };
                let mut fTemp922: F64 = if iTemp920 != 0 { fTemp893 } else { fTemp891 };
                let mut fTemp923: F64 = fTemp922 + fTemp921;
                let mut fTemp924: F64 = 0.5 * fTemp923;
                let mut fTemp925: F64 = 262143.0 * (1.0 - fTemp924);
                let mut iTemp926: i32 = (fTemp925) as i32;
                let mut iTemp927: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp926, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp928: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp927, 7)) as usize] };
                let mut fTemp929: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp927 as usize] };
                let mut fTemp930: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp927, 1)) as usize] }
                        - fTemp929;
                let mut fTemp931: F64 = 131071.5 * fTemp923;
                let mut iTemp932: i32 = (fTemp931) as i32;
                let mut iTemp933: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp932, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp934: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp933, 7)) as usize] };
                let mut fTemp935: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp933 as usize] };
                let mut fTemp936: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp933, 1)) as usize] }
                        - fTemp935;
                let mut fTemp937: F64 = if iTemp725 != 0 {
                    fTemp935
                        + fTemp734 * fTemp936
                        + (fTemp931 - (iTemp932) as F64)
                            * (fTemp934
                                - (fTemp935
                                    + fTemp734
                                        * (fTemp936
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp933, 8)) as usize]
                                            } - fTemp934))))
                } else {
                    1.0 - (fTemp929
                        + fTemp734 * fTemp930
                        + (fTemp925 - (iTemp926) as F64)
                            * (fTemp928
                                - (fTemp929
                                    + fTemp734
                                        * (fTemp930
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp927, 8)) as usize]
                                            } - fTemp928)))))
                };
                let mut fTemp938: F64 = fTemp739 + fTemp924;
                let mut fTemp939: F64 = 262143.0 * (1.0 - fTemp938);
                let mut iTemp940: i32 = (fTemp939) as i32;
                let mut iTemp941: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp940, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp942: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp941, 7)) as usize] };
                let mut fTemp943: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp941 as usize] };
                let mut fTemp944: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp941, 1)) as usize] }
                        - fTemp943;
                let mut fTemp945: F64 = 262143.0 * fTemp938;
                let mut iTemp946: i32 = (fTemp945) as i32;
                let mut iTemp947: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp946, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp948: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp947, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp949: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp947 as usize] };
                let mut fTemp950: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp947, 1), 1835007),
                    )) as usize]
                } - fTemp949;
                let mut iTemp951: i32 = (fTemp795
                    > ((if iTemp725 != 0 {
                        fTemp949
                            + fTemp734 * fTemp950
                            + (fTemp945 - (iTemp946) as F64)
                                * (fTemp948
                                    - (fTemp949
                                        + fTemp734
                                            * (fTemp950
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp947, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp948))))
                    } else {
                        1.0 - (fTemp943
                            + fTemp734 * fTemp944
                            + (fTemp939 - (iTemp940) as F64)
                                * (fTemp942
                                    - (fTemp943
                                        + fTemp734
                                            * (fTemp944
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp941, 8)) as usize]
                                                } - fTemp942)))))
                    } - fTemp937)
                        / (1.0 - fTemp937))) as i32;
                let mut fTemp952: F64 = if iTemp951 != 0 { fTemp921 } else { fTemp924 };
                let mut fTemp953: F64 = if iTemp951 != 0 { fTemp924 } else { fTemp922 };
                let mut fTemp954: F64 = fTemp953 + fTemp952;
                let mut fTemp955: F64 = 0.5 * fTemp954;
                let mut fTemp956: F64 = 262143.0 * (1.0 - fTemp955);
                let mut iTemp957: i32 = (fTemp956) as i32;
                let mut iTemp958: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp957, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp959: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp958, 7)) as usize] };
                let mut fTemp960: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp958 as usize] };
                let mut fTemp961: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp958, 1)) as usize] }
                        - fTemp960;
                let mut fTemp962: F64 = 131071.5 * fTemp954;
                let mut iTemp963: i32 = (fTemp962) as i32;
                let mut iTemp964: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp963, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp965: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp964, 7)) as usize] };
                let mut fTemp966: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp964 as usize] };
                let mut fTemp967: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp964, 1)) as usize] }
                        - fTemp966;
                let mut fTemp968: F64 = if iTemp725 != 0 {
                    fTemp966
                        + fTemp734 * fTemp967
                        + (fTemp962 - (iTemp963) as F64)
                            * (fTemp965
                                - (fTemp966
                                    + fTemp734
                                        * (fTemp967
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp964, 8)) as usize]
                                            } - fTemp965))))
                } else {
                    1.0 - (fTemp960
                        + fTemp734 * fTemp961
                        + (fTemp956 - (iTemp957) as F64)
                            * (fTemp959
                                - (fTemp960
                                    + fTemp734
                                        * (fTemp961
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp958, 8)) as usize]
                                            } - fTemp959)))))
                };
                let mut fTemp969: F64 = fTemp739 + fTemp955;
                let mut fTemp970: F64 = 262143.0 * (1.0 - fTemp969);
                let mut iTemp971: i32 = (fTemp970) as i32;
                let mut iTemp972: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp971, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp973: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp972, 7)) as usize] };
                let mut fTemp974: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp972 as usize] };
                let mut fTemp975: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp972, 1)) as usize] }
                        - fTemp974;
                let mut fTemp976: F64 = 262143.0 * fTemp969;
                let mut iTemp977: i32 = (fTemp976) as i32;
                let mut iTemp978: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp977, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp979: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp978, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp980: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp978 as usize] };
                let mut fTemp981: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp978, 1), 1835007),
                    )) as usize]
                } - fTemp980;
                let mut iTemp982: i32 = (fTemp795
                    > ((if iTemp725 != 0 {
                        fTemp980
                            + fTemp734 * fTemp981
                            + (fTemp976 - (iTemp977) as F64)
                                * (fTemp979
                                    - (fTemp980
                                        + fTemp734
                                            * (fTemp981
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp978, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp979))))
                    } else {
                        1.0 - (fTemp974
                            + fTemp734 * fTemp975
                            + (fTemp970 - (iTemp971) as F64)
                                * (fTemp973
                                    - (fTemp974
                                        + fTemp734
                                            * (fTemp975
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp972, 8)) as usize]
                                                } - fTemp973)))))
                    } - fTemp968)
                        / (1.0 - fTemp968))) as i32;
                let mut fTemp983: F64 = if iTemp982 != 0 { fTemp952 } else { fTemp955 };
                let mut fTemp984: F64 = if iTemp982 != 0 { fTemp955 } else { fTemp953 };
                let mut fTemp985: F64 = fTemp984 + fTemp983;
                let mut fTemp986: F64 = 0.5 * fTemp985;
                let mut fTemp987: F64 = 262143.0 * (1.0 - fTemp986);
                let mut iTemp988: i32 = (fTemp987) as i32;
                let mut iTemp989: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp988, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp990: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp989, 7)) as usize] };
                let mut fTemp991: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp989 as usize] };
                let mut fTemp992: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp989, 1)) as usize] }
                        - fTemp991;
                let mut fTemp993: F64 = 131071.5 * fTemp985;
                let mut iTemp994: i32 = (fTemp993) as i32;
                let mut iTemp995: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp994, 262143))),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp996: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp995, 7)) as usize] };
                let mut fTemp997: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp995 as usize] };
                let mut fTemp998: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp995, 1)) as usize] }
                        - fTemp997;
                let mut fTemp999: F64 = if iTemp725 != 0 {
                    fTemp997
                        + fTemp734 * fTemp998
                        + (fTemp993 - (iTemp994) as F64)
                            * (fTemp996
                                - (fTemp997
                                    + fTemp734
                                        * (fTemp998
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp995, 8)) as usize]
                                            } - fTemp996))))
                } else {
                    1.0 - (fTemp991
                        + fTemp734 * fTemp992
                        + (fTemp987 - (iTemp988) as F64)
                            * (fTemp990
                                - (fTemp991
                                    + fTemp734
                                        * (fTemp992
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp989, 8)) as usize]
                                            } - fTemp990)))))
                };
                let mut fTemp1000: F64 = fTemp739 + fTemp986;
                let mut fTemp1001: F64 = 262143.0 * (1.0 - fTemp1000);
                let mut iTemp1002: i32 = (fTemp1001) as i32;
                let mut iTemp1003: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1002, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1004: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1003, 7)) as usize] };
                let mut fTemp1005: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1003 as usize] };
                let mut fTemp1006: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1003, 1)) as usize] }
                        - fTemp1005;
                let mut fTemp1007: F64 = 262143.0 * fTemp1000;
                let mut iTemp1008: i32 = (fTemp1007) as i32;
                let mut iTemp1009: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1008, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1010: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1009, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp1011: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1009 as usize] };
                let mut fTemp1012: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1009, 1), 1835007),
                    )) as usize]
                } - fTemp1011;
                let mut iTemp1013: i32 = (fTemp795
                    > ((if iTemp725 != 0 {
                        fTemp1011
                            + fTemp734 * fTemp1012
                            + (fTemp1007 - (iTemp1008) as F64)
                                * (fTemp1010
                                    - (fTemp1011
                                        + fTemp734
                                            * (fTemp1012
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1009, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1010))))
                    } else {
                        1.0 - (fTemp1005
                            + fTemp734 * fTemp1006
                            + (fTemp1001 - (iTemp1002) as F64)
                                * (fTemp1004
                                    - (fTemp1005
                                        + fTemp734
                                            * (fTemp1006
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp1003, 8)) as usize]
                                                } - fTemp1004)))))
                    } - fTemp999)
                        / (1.0 - fTemp999))) as i32;
                let mut fTemp1014: F64 = if iTemp1013 != 0 { fTemp983 } else { fTemp986 };
                let mut fTemp1015: F64 = if iTemp1013 != 0 { fTemp986 } else { fTemp984 };
                let mut fTemp1016: F64 = fTemp1015 + fTemp1014;
                let mut fTemp1017: F64 = 0.5 * fTemp1016;
                let mut fTemp1018: F64 = 262143.0 * (1.0 - fTemp1017);
                let mut iTemp1019: i32 = (fTemp1018) as i32;
                let mut iTemp1020: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1019, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1021: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1020, 7)) as usize] };
                let mut fTemp1022: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1020 as usize] };
                let mut fTemp1023: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1020, 1)) as usize] }
                        - fTemp1022;
                let mut fTemp1024: F64 = 131071.5 * fTemp1016;
                let mut iTemp1025: i32 = (fTemp1024) as i32;
                let mut iTemp1026: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1025, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1027: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1026, 7)) as usize] };
                let mut fTemp1028: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1026 as usize] };
                let mut fTemp1029: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1026, 1)) as usize] }
                        - fTemp1028;
                let mut fTemp1030: F64 = if iTemp725 != 0 {
                    fTemp1028
                        + fTemp734 * fTemp1029
                        + (fTemp1024 - (iTemp1025) as F64)
                            * (fTemp1027
                                - (fTemp1028
                                    + fTemp734
                                        * (fTemp1029
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp1026, 8)) as usize]
                                            } - fTemp1027))))
                } else {
                    1.0 - (fTemp1022
                        + fTemp734 * fTemp1023
                        + (fTemp1018 - (iTemp1019) as F64)
                            * (fTemp1021
                                - (fTemp1022
                                    + fTemp734
                                        * (fTemp1023
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp1020, 8)) as usize]
                                            } - fTemp1021)))))
                };
                let mut fTemp1031: F64 = fTemp739 + fTemp1017;
                let mut fTemp1032: F64 = 262143.0 * (1.0 - fTemp1031);
                let mut iTemp1033: i32 = (fTemp1032) as i32;
                let mut iTemp1034: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1033, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1035: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1034, 7)) as usize] };
                let mut fTemp1036: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1034 as usize] };
                let mut fTemp1037: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1034, 1)) as usize] }
                        - fTemp1036;
                let mut fTemp1038: F64 = 262143.0 * fTemp1031;
                let mut iTemp1039: i32 = (fTemp1038) as i32;
                let mut iTemp1040: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1039, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1041: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1040, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp1042: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1040 as usize] };
                let mut fTemp1043: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1040, 1), 1835007),
                    )) as usize]
                } - fTemp1042;
                let mut iTemp1044: i32 = (fTemp795
                    > ((if iTemp725 != 0 {
                        fTemp1042
                            + fTemp734 * fTemp1043
                            + (fTemp1038 - (iTemp1039) as F64)
                                * (fTemp1041
                                    - (fTemp1042
                                        + fTemp734
                                            * (fTemp1043
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1040, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1041))))
                    } else {
                        1.0 - (fTemp1036
                            + fTemp734 * fTemp1037
                            + (fTemp1032 - (iTemp1033) as F64)
                                * (fTemp1035
                                    - (fTemp1036
                                        + fTemp734
                                            * (fTemp1037
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp1034, 8)) as usize]
                                                } - fTemp1035)))))
                    } - fTemp1030)
                        / (1.0 - fTemp1030))) as i32;
                let mut fTemp1045: F64 = if iTemp1044 != 0 { fTemp1014 } else { fTemp1017 };
                let mut fTemp1046: F64 = if iTemp1044 != 0 { fTemp1017 } else { fTemp1015 };
                let mut fTemp1047: F64 = fTemp1046 + fTemp1045;
                let mut fTemp1048: F64 = 0.5 * fTemp1047;
                let mut fTemp1049: F64 = 262143.0 * (1.0 - fTemp1048);
                let mut iTemp1050: i32 = (fTemp1049) as i32;
                let mut iTemp1051: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1050, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1052: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1051, 7)) as usize] };
                let mut fTemp1053: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1051 as usize] };
                let mut fTemp1054: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1051, 1)) as usize] }
                        - fTemp1053;
                let mut fTemp1055: F64 = 131071.5 * fTemp1047;
                let mut iTemp1056: i32 = (fTemp1055) as i32;
                let mut iTemp1057: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1056, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1058: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1057, 7)) as usize] };
                let mut fTemp1059: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1057 as usize] };
                let mut fTemp1060: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1057, 1)) as usize] }
                        - fTemp1059;
                let mut fTemp1061: F64 = if iTemp725 != 0 {
                    fTemp1059
                        + fTemp734 * fTemp1060
                        + (fTemp1055 - (iTemp1056) as F64)
                            * (fTemp1058
                                - (fTemp1059
                                    + fTemp734
                                        * (fTemp1060
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp1057, 8)) as usize]
                                            } - fTemp1058))))
                } else {
                    1.0 - (fTemp1053
                        + fTemp734 * fTemp1054
                        + (fTemp1049 - (iTemp1050) as F64)
                            * (fTemp1052
                                - (fTemp1053
                                    + fTemp734
                                        * (fTemp1054
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp1051, 8)) as usize]
                                            } - fTemp1052)))))
                };
                let mut fTemp1062: F64 = fTemp739 + fTemp1048;
                let mut fTemp1063: F64 = 262143.0 * (1.0 - fTemp1062);
                let mut iTemp1064: i32 = (fTemp1063) as i32;
                let mut iTemp1065: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1064, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1066: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1065, 7)) as usize] };
                let mut fTemp1067: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1065 as usize] };
                let mut fTemp1068: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1065, 1)) as usize] }
                        - fTemp1067;
                let mut fTemp1069: F64 = 262143.0 * fTemp1062;
                let mut iTemp1070: i32 = (fTemp1069) as i32;
                let mut iTemp1071: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1070, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1072: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1071, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp1073: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1071 as usize] };
                let mut fTemp1074: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1071, 1), 1835007),
                    )) as usize]
                } - fTemp1073;
                let mut iTemp1075: i32 = (fTemp795
                    > ((if iTemp725 != 0 {
                        fTemp1073
                            + fTemp734 * fTemp1074
                            + (fTemp1069 - (iTemp1070) as F64)
                                * (fTemp1072
                                    - (fTemp1073
                                        + fTemp734
                                            * (fTemp1074
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1071, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1072))))
                    } else {
                        1.0 - (fTemp1067
                            + fTemp734 * fTemp1068
                            + (fTemp1063 - (iTemp1064) as F64)
                                * (fTemp1066
                                    - (fTemp1067
                                        + fTemp734
                                            * (fTemp1068
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp1065, 8)) as usize]
                                                } - fTemp1066)))))
                    } - fTemp1061)
                        / (1.0 - fTemp1061))) as i32;
                let mut fTemp1076: F64 = if iTemp1075 != 0 { fTemp1045 } else { fTemp1048 };
                let mut fTemp1077: F64 = if iTemp1075 != 0 { fTemp1048 } else { fTemp1046 };
                let mut fTemp1078: F64 = fTemp1077 + fTemp1076;
                let mut fTemp1079: F64 = 0.5 * fTemp1078;
                let mut fTemp1080: F64 = 262143.0 * (1.0 - fTemp1079);
                let mut iTemp1081: i32 = (fTemp1080) as i32;
                let mut iTemp1082: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1081, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1083: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1082, 7)) as usize] };
                let mut fTemp1084: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1082 as usize] };
                let mut fTemp1085: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1082, 1)) as usize] }
                        - fTemp1084;
                let mut fTemp1086: F64 = 131071.5 * fTemp1078;
                let mut iTemp1087: i32 = (fTemp1086) as i32;
                let mut iTemp1088: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1087, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1089: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1088, 7)) as usize] };
                let mut fTemp1090: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1088 as usize] };
                let mut fTemp1091: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1088, 1)) as usize] }
                        - fTemp1090;
                let mut fTemp1092: F64 = if iTemp725 != 0 {
                    fTemp1090
                        + fTemp734 * fTemp1091
                        + (fTemp1086 - (iTemp1087) as F64)
                            * (fTemp1089
                                - (fTemp1090
                                    + fTemp734
                                        * (fTemp1091
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp1088, 8)) as usize]
                                            } - fTemp1089))))
                } else {
                    1.0 - (fTemp1084
                        + fTemp734 * fTemp1085
                        + (fTemp1080 - (iTemp1081) as F64)
                            * (fTemp1083
                                - (fTemp1084
                                    + fTemp734
                                        * (fTemp1085
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp1082, 8)) as usize]
                                            } - fTemp1083)))))
                };
                let mut fTemp1093: F64 = fTemp739 + fTemp1079;
                let mut fTemp1094: F64 = 262143.0 * (1.0 - fTemp1093);
                let mut iTemp1095: i32 = (fTemp1094) as i32;
                let mut iTemp1096: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1095, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1097: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1096, 7)) as usize] };
                let mut fTemp1098: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1096 as usize] };
                let mut fTemp1099: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1096, 1)) as usize] }
                        - fTemp1098;
                let mut fTemp1100: F64 = 262143.0 * fTemp1093;
                let mut iTemp1101: i32 = (fTemp1100) as i32;
                let mut iTemp1102: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1101, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1103: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1102, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp1104: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1102 as usize] };
                let mut fTemp1105: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1102, 1), 1835007),
                    )) as usize]
                } - fTemp1104;
                let mut iTemp1106: i32 = (fTemp795
                    > ((if iTemp725 != 0 {
                        fTemp1104
                            + fTemp734 * fTemp1105
                            + (fTemp1100 - (iTemp1101) as F64)
                                * (fTemp1103
                                    - (fTemp1104
                                        + fTemp734
                                            * (fTemp1105
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1102, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1103))))
                    } else {
                        1.0 - (fTemp1098
                            + fTemp734 * fTemp1099
                            + (fTemp1094 - (iTemp1095) as F64)
                                * (fTemp1097
                                    - (fTemp1098
                                        + fTemp734
                                            * (fTemp1099
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp1096, 8)) as usize]
                                                } - fTemp1097)))))
                    } - fTemp1092)
                        / (1.0 - fTemp1092))) as i32;
                let mut fTemp1107: F64 = if iTemp1106 != 0 { fTemp1076 } else { fTemp1079 };
                let mut fTemp1108: F64 = if iTemp1106 != 0 { fTemp1079 } else { fTemp1077 };
                let mut fTemp1109: F64 = fTemp1108 + fTemp1107;
                let mut fTemp1110: F64 = 0.5 * fTemp1109;
                let mut fTemp1111: F64 = 262143.0 * (1.0 - fTemp1110);
                let mut iTemp1112: i32 = (fTemp1111) as i32;
                let mut iTemp1113: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1112, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1114: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1113, 7)) as usize] };
                let mut fTemp1115: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1113 as usize] };
                let mut fTemp1116: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1113, 1)) as usize] }
                        - fTemp1115;
                let mut fTemp1117: F64 = 131071.5 * fTemp1109;
                let mut iTemp1118: i32 = (fTemp1117) as i32;
                let mut iTemp1119: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1118, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1120: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1119, 7)) as usize] };
                let mut fTemp1121: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1119 as usize] };
                let mut fTemp1122: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1119, 1)) as usize] }
                        - fTemp1121;
                let mut fTemp1123: F64 = if iTemp725 != 0 {
                    fTemp1121
                        + fTemp734 * fTemp1122
                        + (fTemp1117 - (iTemp1118) as F64)
                            * (fTemp1120
                                - (fTemp1121
                                    + fTemp734
                                        * (fTemp1122
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp1119, 8)) as usize]
                                            } - fTemp1120))))
                } else {
                    1.0 - (fTemp1115
                        + fTemp734 * fTemp1116
                        + (fTemp1111 - (iTemp1112) as F64)
                            * (fTemp1114
                                - (fTemp1115
                                    + fTemp734
                                        * (fTemp1116
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp1113, 8)) as usize]
                                            } - fTemp1114)))))
                };
                let mut fTemp1124: F64 = fTemp739 + fTemp1110;
                let mut fTemp1125: F64 = 262143.0 * (1.0 - fTemp1124);
                let mut iTemp1126: i32 = (fTemp1125) as i32;
                let mut iTemp1127: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1126, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1128: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1127, 7)) as usize] };
                let mut fTemp1129: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1127 as usize] };
                let mut fTemp1130: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1127, 1)) as usize] }
                        - fTemp1129;
                let mut fTemp1131: F64 = 262143.0 * fTemp1124;
                let mut iTemp1132: i32 = (fTemp1131) as i32;
                let mut iTemp1133: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1132, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1134: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1133, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp1135: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1133 as usize] };
                let mut fTemp1136: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1133, 1), 1835007),
                    )) as usize]
                } - fTemp1135;
                let mut iTemp1137: i32 = (fTemp795
                    > ((if iTemp725 != 0 {
                        fTemp1135
                            + fTemp734 * fTemp1136
                            + (fTemp1131 - (iTemp1132) as F64)
                                * (fTemp1134
                                    - (fTemp1135
                                        + fTemp734
                                            * (fTemp1136
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1133, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1134))))
                    } else {
                        1.0 - (fTemp1129
                            + fTemp734 * fTemp1130
                            + (fTemp1125 - (iTemp1126) as F64)
                                * (fTemp1128
                                    - (fTemp1129
                                        + fTemp734
                                            * (fTemp1130
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp1127, 8)) as usize]
                                                } - fTemp1128)))))
                    } - fTemp1123)
                        / (1.0 - fTemp1123))) as i32;
                let mut fTemp1138: F64 = if iTemp1137 != 0 { fTemp1107 } else { fTemp1110 };
                let mut fTemp1139: F64 = if iTemp1137 != 0 { fTemp1110 } else { fTemp1108 };
                let mut fTemp1140: F64 = fTemp1139 + fTemp1138;
                let mut fTemp1141: F64 = 0.5 * fTemp1140;
                let mut fTemp1142: F64 = 262143.0 * (1.0 - fTemp1141);
                let mut iTemp1143: i32 = (fTemp1142) as i32;
                let mut iTemp1144: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1143, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1145: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1144, 7)) as usize] };
                let mut fTemp1146: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1144 as usize] };
                let mut fTemp1147: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1144, 1)) as usize] }
                        - fTemp1146;
                let mut fTemp1148: F64 = 131071.5 * fTemp1140;
                let mut iTemp1149: i32 = (fTemp1148) as i32;
                let mut iTemp1150: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1149, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1151: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1150, 7)) as usize] };
                let mut fTemp1152: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1150 as usize] };
                let mut fTemp1153: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1150, 1)) as usize] }
                        - fTemp1152;
                let mut fTemp1154: F64 = if iTemp725 != 0 {
                    fTemp1152
                        + fTemp734 * fTemp1153
                        + (fTemp1148 - (iTemp1149) as F64)
                            * (fTemp1151
                                - (fTemp1152
                                    + fTemp734
                                        * (fTemp1153
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp1150, 8)) as usize]
                                            } - fTemp1151))))
                } else {
                    1.0 - (fTemp1146
                        + fTemp734 * fTemp1147
                        + (fTemp1142 - (iTemp1143) as F64)
                            * (fTemp1145
                                - (fTemp1146
                                    + fTemp734
                                        * (fTemp1147
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp1144, 8)) as usize]
                                            } - fTemp1145)))))
                };
                let mut fTemp1155: F64 = fTemp739 + fTemp1141;
                let mut fTemp1156: F64 = 262143.0 * (1.0 - fTemp1155);
                let mut iTemp1157: i32 = (fTemp1156) as i32;
                let mut iTemp1158: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1157, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1159: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1158, 7)) as usize] };
                let mut fTemp1160: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1158 as usize] };
                let mut fTemp1161: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1158, 1)) as usize] }
                        - fTemp1160;
                let mut fTemp1162: F64 = 262143.0 * fTemp1155;
                let mut iTemp1163: i32 = (fTemp1162) as i32;
                let mut iTemp1164: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1163, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1165: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1164, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp1166: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1164 as usize] };
                let mut fTemp1167: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1164, 1), 1835007),
                    )) as usize]
                } - fTemp1166;
                let mut iTemp1168: i32 = (fTemp795
                    > ((if iTemp725 != 0 {
                        fTemp1166
                            + fTemp734 * fTemp1167
                            + (fTemp1162 - (iTemp1163) as F64)
                                * (fTemp1165
                                    - (fTemp1166
                                        + fTemp734
                                            * (fTemp1167
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1164, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1165))))
                    } else {
                        1.0 - (fTemp1160
                            + fTemp734 * fTemp1161
                            + (fTemp1156 - (iTemp1157) as F64)
                                * (fTemp1159
                                    - (fTemp1160
                                        + fTemp734
                                            * (fTemp1161
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp1158, 8)) as usize]
                                                } - fTemp1159)))))
                    } - fTemp1154)
                        / (1.0 - fTemp1154))) as i32;
                let mut fTemp1169: F64 = if iTemp1168 != 0 { fTemp1138 } else { fTemp1141 };
                let mut fTemp1170: F64 = if iTemp1168 != 0 { fTemp1141 } else { fTemp1139 };
                let mut fTemp1171: F64 = fTemp1170 + fTemp1169;
                let mut fTemp1172: F64 = 0.5 * fTemp1171;
                let mut fTemp1173: F64 = 262143.0 * (1.0 - fTemp1172);
                let mut iTemp1174: i32 = (fTemp1173) as i32;
                let mut iTemp1175: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1174, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1176: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1175, 7)) as usize] };
                let mut fTemp1177: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1175 as usize] };
                let mut fTemp1178: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1175, 1)) as usize] }
                        - fTemp1177;
                let mut fTemp1179: F64 = 131071.5 * fTemp1171;
                let mut iTemp1180: i32 = (fTemp1179) as i32;
                let mut iTemp1181: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1180, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1182: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1181, 7)) as usize] };
                let mut fTemp1183: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1181 as usize] };
                let mut fTemp1184: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1181, 1)) as usize] }
                        - fTemp1183;
                let mut fTemp1185: F64 = if iTemp725 != 0 {
                    fTemp1183
                        + fTemp734 * fTemp1184
                        + (fTemp1179 - (iTemp1180) as F64)
                            * (fTemp1182
                                - (fTemp1183
                                    + fTemp734
                                        * (fTemp1184
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp1181, 8)) as usize]
                                            } - fTemp1182))))
                } else {
                    1.0 - (fTemp1177
                        + fTemp734 * fTemp1178
                        + (fTemp1173 - (iTemp1174) as F64)
                            * (fTemp1176
                                - (fTemp1177
                                    + fTemp734
                                        * (fTemp1178
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp1175, 8)) as usize]
                                            } - fTemp1176)))))
                };
                let mut fTemp1186: F64 = fTemp739 + fTemp1172;
                let mut fTemp1187: F64 = 262143.0 * (1.0 - fTemp1186);
                let mut iTemp1188: i32 = (fTemp1187) as i32;
                let mut iTemp1189: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1188, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1190: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1189, 7)) as usize] };
                let mut fTemp1191: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1189 as usize] };
                let mut fTemp1192: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1189, 1)) as usize] }
                        - fTemp1191;
                let mut fTemp1193: F64 = 262143.0 * fTemp1186;
                let mut iTemp1194: i32 = (fTemp1193) as i32;
                let mut iTemp1195: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1194, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1196: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1195, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp1197: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1195 as usize] };
                let mut fTemp1198: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1195, 1), 1835007),
                    )) as usize]
                } - fTemp1197;
                let mut iTemp1199: i32 = (fTemp795
                    > ((if iTemp725 != 0 {
                        fTemp1197
                            + fTemp734 * fTemp1198
                            + (fTemp1193 - (iTemp1194) as F64)
                                * (fTemp1196
                                    - (fTemp1197
                                        + fTemp734
                                            * (fTemp1198
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1195, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1196))))
                    } else {
                        1.0 - (fTemp1191
                            + fTemp734 * fTemp1192
                            + (fTemp1187 - (iTemp1188) as F64)
                                * (fTemp1190
                                    - (fTemp1191
                                        + fTemp734
                                            * (fTemp1192
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp1189, 8)) as usize]
                                                } - fTemp1190)))))
                    } - fTemp1185)
                        / (1.0 - fTemp1185))) as i32;
                let mut fTemp1200: F64 = if iTemp1199 != 0 { fTemp1169 } else { fTemp1172 };
                let mut fTemp1201: F64 = if iTemp1199 != 0 { fTemp1172 } else { fTemp1170 };
                let mut fTemp1202: F64 = fTemp1201 + fTemp1200;
                let mut fTemp1203: F64 = 0.5 * fTemp1202;
                let mut fTemp1204: F64 = 262143.0 * (1.0 - fTemp1203);
                let mut iTemp1205: i32 = (fTemp1204) as i32;
                let mut iTemp1206: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1205, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1207: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1206, 7)) as usize] };
                let mut fTemp1208: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1206 as usize] };
                let mut fTemp1209: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1206, 1)) as usize] }
                        - fTemp1208;
                let mut fTemp1210: F64 = 131071.5 * fTemp1202;
                let mut iTemp1211: i32 = (fTemp1210) as i32;
                let mut iTemp1212: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1211, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1213: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1212, 7)) as usize] };
                let mut fTemp1214: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1212 as usize] };
                let mut fTemp1215: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1212, 1)) as usize] }
                        - fTemp1214;
                let mut fTemp1216: F64 = if iTemp725 != 0 {
                    fTemp1214
                        + fTemp734 * fTemp1215
                        + (fTemp1210 - (iTemp1211) as F64)
                            * (fTemp1213
                                - (fTemp1214
                                    + fTemp734
                                        * (fTemp1215
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp1212, 8)) as usize]
                                            } - fTemp1213))))
                } else {
                    1.0 - (fTemp1208
                        + fTemp734 * fTemp1209
                        + (fTemp1204 - (iTemp1205) as F64)
                            * (fTemp1207
                                - (fTemp1208
                                    + fTemp734
                                        * (fTemp1209
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp1206, 8)) as usize]
                                            } - fTemp1207)))))
                };
                let mut fTemp1217: F64 = fTemp739 + fTemp1203;
                let mut fTemp1218: F64 = 262143.0 * (1.0 - fTemp1217);
                let mut iTemp1219: i32 = (fTemp1218) as i32;
                let mut iTemp1220: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1219, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1221: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1220, 7)) as usize] };
                let mut fTemp1222: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1220 as usize] };
                let mut fTemp1223: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1220, 1)) as usize] }
                        - fTemp1222;
                let mut fTemp1224: F64 = 262143.0 * fTemp1217;
                let mut iTemp1225: i32 = (fTemp1224) as i32;
                let mut iTemp1226: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1225, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1227: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1226, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp1228: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1226 as usize] };
                let mut fTemp1229: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1226, 1), 1835007),
                    )) as usize]
                } - fTemp1228;
                let mut iTemp1230: i32 = (fTemp795
                    > ((if iTemp725 != 0 {
                        fTemp1228
                            + fTemp734 * fTemp1229
                            + (fTemp1224 - (iTemp1225) as F64)
                                * (fTemp1227
                                    - (fTemp1228
                                        + fTemp734
                                            * (fTemp1229
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1226, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1227))))
                    } else {
                        1.0 - (fTemp1222
                            + fTemp734 * fTemp1223
                            + (fTemp1218 - (iTemp1219) as F64)
                                * (fTemp1221
                                    - (fTemp1222
                                        + fTemp734
                                            * (fTemp1223
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp1220, 8)) as usize]
                                                } - fTemp1221)))))
                    } - fTemp1216)
                        / (1.0 - fTemp1216))) as i32;
                let mut fTemp1231: F64 = if iTemp1230 != 0 { fTemp1200 } else { fTemp1203 };
                let mut fTemp1232: F64 = if iTemp1230 != 0 { fTemp1203 } else { fTemp1201 };
                let mut fTemp1233: F64 = fTemp1232 + fTemp1231;
                let mut fTemp1234: F64 = 0.5 * fTemp1233;
                let mut fTemp1235: F64 = 262143.0 * (1.0 - fTemp1234);
                let mut iTemp1236: i32 = (fTemp1235) as i32;
                let mut iTemp1237: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1236, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1238: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1237, 7)) as usize] };
                let mut fTemp1239: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1237 as usize] };
                let mut fTemp1240: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1237, 1)) as usize] }
                        - fTemp1239;
                let mut fTemp1241: F64 = 131071.5 * fTemp1233;
                let mut iTemp1242: i32 = (fTemp1241) as i32;
                let mut iTemp1243: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1242, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1244: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1243, 7)) as usize] };
                let mut fTemp1245: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1243 as usize] };
                let mut fTemp1246: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1243, 1)) as usize] }
                        - fTemp1245;
                let mut fTemp1247: F64 = if iTemp725 != 0 {
                    fTemp1245
                        + fTemp734 * fTemp1246
                        + (fTemp1241 - (iTemp1242) as F64)
                            * (fTemp1244
                                - (fTemp1245
                                    + fTemp734
                                        * (fTemp1246
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp1243, 8)) as usize]
                                            } - fTemp1244))))
                } else {
                    1.0 - (fTemp1239
                        + fTemp734 * fTemp1240
                        + (fTemp1235 - (iTemp1236) as F64)
                            * (fTemp1238
                                - (fTemp1239
                                    + fTemp734
                                        * (fTemp1240
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp1237, 8)) as usize]
                                            } - fTemp1238)))))
                };
                let mut fTemp1248: F64 = fTemp739 + fTemp1234;
                let mut fTemp1249: F64 = 262143.0 * (1.0 - fTemp1248);
                let mut iTemp1250: i32 = (fTemp1249) as i32;
                let mut iTemp1251: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1250, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1252: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1251, 7)) as usize] };
                let mut fTemp1253: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1251 as usize] };
                let mut fTemp1254: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1251, 1)) as usize] }
                        - fTemp1253;
                let mut fTemp1255: F64 = 262143.0 * fTemp1248;
                let mut iTemp1256: i32 = (fTemp1255) as i32;
                let mut iTemp1257: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1256, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1258: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1257, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp1259: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1257 as usize] };
                let mut fTemp1260: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1257, 1), 1835007),
                    )) as usize]
                } - fTemp1259;
                let mut iTemp1261: i32 = (fTemp795
                    > ((if iTemp725 != 0 {
                        fTemp1259
                            + fTemp734 * fTemp1260
                            + (fTemp1255 - (iTemp1256) as F64)
                                * (fTemp1258
                                    - (fTemp1259
                                        + fTemp734
                                            * (fTemp1260
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1257, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1258))))
                    } else {
                        1.0 - (fTemp1253
                            + fTemp734 * fTemp1254
                            + (fTemp1249 - (iTemp1250) as F64)
                                * (fTemp1252
                                    - (fTemp1253
                                        + fTemp734
                                            * (fTemp1254
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp1251, 8)) as usize]
                                                } - fTemp1252)))))
                    } - fTemp1247)
                        / (1.0 - fTemp1247))) as i32;
                let mut fTemp1262: F64 = if iTemp1261 != 0 { fTemp1231 } else { fTemp1234 };
                let mut fTemp1263: F64 = if iTemp1261 != 0 { fTemp1234 } else { fTemp1232 };
                let mut fTemp1264: F64 = fTemp1263 + fTemp1262;
                let mut fTemp1265: F64 = 0.5 * fTemp1264;
                let mut fTemp1266: F64 = 262143.0 * (1.0 - fTemp1265);
                let mut iTemp1267: i32 = (fTemp1266) as i32;
                let mut iTemp1268: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1267, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1269: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1268, 7)) as usize] };
                let mut fTemp1270: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1268 as usize] };
                let mut fTemp1271: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1268, 1)) as usize] }
                        - fTemp1270;
                let mut fTemp1272: F64 = 131071.5 * fTemp1264;
                let mut iTemp1273: i32 = (fTemp1272) as i32;
                let mut iTemp1274: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1273, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1275: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1274, 7)) as usize] };
                let mut fTemp1276: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1274 as usize] };
                let mut fTemp1277: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1274, 1)) as usize] }
                        - fTemp1276;
                let mut fTemp1278: F64 = if iTemp725 != 0 {
                    fTemp1276
                        + fTemp734 * fTemp1277
                        + (fTemp1272 - (iTemp1273) as F64)
                            * (fTemp1275
                                - (fTemp1276
                                    + fTemp734
                                        * (fTemp1277
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp1274, 8)) as usize]
                                            } - fTemp1275))))
                } else {
                    1.0 - (fTemp1270
                        + fTemp734 * fTemp1271
                        + (fTemp1266 - (iTemp1267) as F64)
                            * (fTemp1269
                                - (fTemp1270
                                    + fTemp734
                                        * (fTemp1271
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0
                                                    [(i32::wrapping_add(iTemp1268, 8)) as usize]
                                            } - fTemp1269)))))
                };
                let mut fTemp1279: F64 = fTemp739 + fTemp1265;
                let mut fTemp1280: F64 = 262143.0 * (1.0 - fTemp1279);
                let mut iTemp1281: i32 = (fTemp1280) as i32;
                let mut iTemp1282: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1281, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1283: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1282, 7)) as usize] };
                let mut fTemp1284: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1282 as usize] };
                let mut fTemp1285: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1282, 1)) as usize] }
                        - fTemp1284;
                let mut fTemp1286: F64 = 262143.0 * fTemp1279;
                let mut iTemp1287: i32 = (fTemp1286) as i32;
                let mut iTemp1288: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1287, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1289: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1288, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp1290: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1288 as usize] };
                let mut fTemp1291: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1288, 1), 1835007),
                    )) as usize]
                } - fTemp1290;
                let mut iTemp1292: i32 = (fTemp795
                    > ((if iTemp725 != 0 {
                        fTemp1290
                            + fTemp734 * fTemp1291
                            + (fTemp1286 - (iTemp1287) as F64)
                                * (fTemp1289
                                    - (fTemp1290
                                        + fTemp734
                                            * (fTemp1291
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1288, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1289))))
                    } else {
                        1.0 - (fTemp1284
                            + fTemp734 * fTemp1285
                            + (fTemp1280 - (iTemp1281) as F64)
                                * (fTemp1283
                                    - (fTemp1284
                                        + fTemp734
                                            * (fTemp1285
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp1282, 8)) as usize]
                                                } - fTemp1283)))))
                    } - fTemp1278)
                        / (1.0 - fTemp1278))) as i32;
                let mut fTemp1293: F64 = if iTemp1292 != 0 { fTemp1262 } else { fTemp1265 };
                let mut fTemp1294: F64 = if iTemp1292 != 0 { fTemp1265 } else { fTemp1263 };
                let mut fTemp1295: F64 = fTemp1294 + fTemp1293;
                let mut fTemp1296: F64 = 0.5 * fTemp1295;
                let mut fTemp1297: F64 = 262143.0 * (1.0 - fTemp1296);
                let mut iTemp1298: i32 = (fTemp1297) as i32;
                let mut iTemp1299: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1298, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1300: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1299, 7)) as usize] };
                let mut fTemp1301: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1299 as usize] };
                let mut fTemp1302: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1299, 1)) as usize] }
                        - fTemp1301;
                let mut fTemp1303: F64 = 131071.5 * fTemp1295;
                let mut iTemp1304: i32 = (fTemp1303) as i32;
                let mut iTemp1305: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1304, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1306: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1305, 7)) as usize] };
                let mut fTemp1307: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1305 as usize] };
                let mut fTemp1308: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1305, 1)) as usize] }
                        - fTemp1307;
                let mut fTemp1309: F64 = if iTemp725 != 0 {
                    fTemp1307
                        + fTemp734 * fTemp1308
                        + (fTemp1303 - (iTemp1304) as F64)
                            * (fTemp1306
                                - (fTemp1307
                                    + fTemp734
                                        * (fTemp1308
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp1305, 8),
                                                        1835007,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp1306))))
                } else {
                    1.0 - (fTemp1301
                        + fTemp734 * fTemp1302
                        + (fTemp1297 - (iTemp1298) as F64)
                            * (fTemp1300
                                - (fTemp1301
                                    + fTemp734
                                        * (fTemp1302
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp1299, 8),
                                                        1835007,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp1300)))))
                };
                let mut fTemp1310: F64 = fTemp739 + fTemp1296;
                let mut fTemp1311: F64 = 262143.0 * (1.0 - fTemp1310);
                let mut iTemp1312: i32 = (fTemp1311) as i32;
                let mut iTemp1313: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1312, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1314: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1313, 7)) as usize] };
                let mut fTemp1315: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1313 as usize] };
                let mut fTemp1316: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1313, 1)) as usize] }
                        - fTemp1315;
                let mut fTemp1317: F64 = 262143.0 * fTemp1310;
                let mut iTemp1318: i32 = (fTemp1317) as i32;
                let mut iTemp1319: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1318, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1320: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1319, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp1321: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1319 as usize] };
                let mut fTemp1322: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1319, 1), 1835007),
                    )) as usize]
                } - fTemp1321;
                let mut iTemp1323: i32 = (fTemp795
                    > ((if iTemp725 != 0 {
                        fTemp1321
                            + fTemp734 * fTemp1322
                            + (fTemp1317 - (iTemp1318) as F64)
                                * (fTemp1320
                                    - (fTemp1321
                                        + fTemp734
                                            * (fTemp1322
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1319, 8),
                                                            1835007,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1320))))
                    } else {
                        1.0 - (fTemp1315
                            + fTemp734 * fTemp1316
                            + (fTemp1311 - (iTemp1312) as F64)
                                * (fTemp1314
                                    - (fTemp1315
                                        + fTemp734
                                            * (fTemp1316
                                                - (unsafe {
                                                    ftbl0LambRs96kSIG0
                                                        [(i32::wrapping_add(iTemp1313, 8)) as usize]
                                                } - fTemp1314)))))
                    } - fTemp1309)
                        / (1.0 - fTemp1309))) as i32;
                let mut fTemp1324: F64 = F64::min(
                    1.0,
                    F64::max(
                        0.0,
                        0.5 * (if iTemp1323 != 0 { fTemp1296 } else { fTemp1294 }
                            + if iTemp1323 != 0 { fTemp1293 } else { fTemp1296 }),
                    ),
                );
                self.fRec15[0] = fTemp1324;
                let mut fTemp1325: F64 = 262143.0 * (1.0 - fTemp1324);
                let mut iTemp1326: i32 = (fTemp1325) as i32;
                let mut iTemp1327: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1326, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1328: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1327, 7)) as usize] };
                let mut fTemp1329: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1327 as usize] };
                let mut fTemp1330: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1327, 1)) as usize] }
                        - fTemp1329;
                let mut fTemp1331: F64 = 262143.0 * fTemp1324;
                let mut iTemp1332: i32 = (fTemp1331) as i32;
                let mut iTemp1333: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1332, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1334: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1333, 7)) as usize] };
                let mut fTemp1335: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1333 as usize] };
                let mut fTemp1336: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1333, 1)) as usize] }
                        - fTemp1335;
                let mut fTemp1337: F64 = if iTemp725 != 0 {
                    fTemp1335
                        + fTemp734 * fTemp1336
                        + (fTemp1331 - (iTemp1332) as F64)
                            * (fTemp1334
                                - (fTemp1335
                                    + fTemp734
                                        * (fTemp1336
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp1333, 8),
                                                        1835007,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp1334))))
                } else {
                    1.0 - (fTemp1329
                        + fTemp734 * fTemp1330
                        + (fTemp1325 - (iTemp1326) as F64)
                            * (fTemp1328
                                - (fTemp1329
                                    + fTemp734
                                        * (fTemp1330
                                            - (unsafe {
                                                ftbl0LambRs96kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp1327, 8),
                                                        1835007,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp1328)))))
                };
                let mut fTemp1338: F64 = fTemp739 + fTemp1324;
                let mut fTemp1339: F64 = 262143.0 * (1.0 - fTemp1338);
                let mut iTemp1340: i32 = (fTemp1339) as i32;
                let mut iTemp1341: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1340, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1342: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1341, 7)) as usize] };
                let mut fTemp1343: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1341 as usize] };
                let mut fTemp1344: F64 =
                    unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1341, 1)) as usize] }
                        - fTemp1343;
                let mut fTemp1345: F64 = 262143.0 * fTemp1338;
                let mut iTemp1346: i32 = (fTemp1345) as i32;
                let mut iTemp1347: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp729,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1346, 262143)),
                            ),
                        ),
                        1835007,
                    ),
                );
                let mut fTemp1348: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1347, 7), 1835007),
                    )) as usize]
                };
                let mut fTemp1349: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1347 as usize] };
                let mut fTemp1350: F64 = unsafe {
                    ftbl0LambRs96kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1347, 1), 1835007),
                    )) as usize]
                } - fTemp1349;
                let mut fTemp1351: F64 = self.fRec16[1]
                    + if ((0.001 * fTemp738) == 0.0) as i32 != 0 {
                        fTemp724
                    } else {
                        fTemp724
                            * (if iTemp725 != 0 {
                                fTemp1349
                                    + fTemp734 * fTemp1350
                                    + (fTemp1345 - (iTemp1346) as F64)
                                        * (fTemp1348
                                            - (fTemp1349
                                                + fTemp734
                                                    * (fTemp1350
                                                        - (unsafe {
                                                            ftbl0LambRs96kSIG0[(std::cmp::max(
                                                                0,
                                                                std::cmp::min(
                                                                    i32::wrapping_add(iTemp1347, 8),
                                                                    1835007,
                                                                ),
                                                            ))
                                                                as usize]
                                                        } - fTemp1348))))
                            } else {
                                1.0 - (fTemp1343
                                    + fTemp734 * fTemp1344
                                    + (fTemp1339 - (iTemp1340) as F64)
                                        * (fTemp1342
                                            - (fTemp1343
                                                + fTemp734
                                                    * (fTemp1344
                                                        - (unsafe {
                                                            ftbl0LambRs96kSIG0[(i32::wrapping_add(
                                                                iTemp1341, 8,
                                                            ))
                                                                as usize]
                                                        } - fTemp1342)))))
                            } - fTemp1337)
                            / (1.0 - fTemp1337)
                    };
                self.fRec16[0] = if iTemp737 != 0 {
                    F64::min(fTemp1351, self.fRec16[1])
                } else {
                    F64::max(fTemp1351, self.fRec16[1])
                };
                self.fVec62[(self.IOTA0 & 16383) as usize] =
                    F64::powf(1e+01, 0.05 * self.fRec16[0]);
                let mut fTemp1352: F64 =
                    self.fVec62[((i32::wrapping_sub(self.IOTA0, iSlow74)) & 16383) as usize];
                *output1 = 0.5
                    * fTemp2
                    * self.fVec3[((i32::wrapping_sub(self.IOTA0, iSlow75)) & 32767) as usize]
                    + self.fRec14[0]
                        * fTemp4
                        * self.fVec4[((i32::wrapping_sub(self.IOTA0, iSlow75)) & 32767) as usize]
                        * fTemp1352;
                *output2 = fTemp3 + fTemp697 * fTemp4;
                *output3 = fTemp3 + fTemp4 * fTemp1352;
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
                for j0 in (1..=6).rev() {
                    self.fVec7[j0 as usize] = self.fVec7[(i32::wrapping_sub(j0, 1)) as usize];
                }
                for j1 in (1..=14).rev() {
                    self.fVec8[j1 as usize] = self.fVec8[(i32::wrapping_sub(j1, 1)) as usize];
                }
                self.fRec3[1] = self.fRec3[0];
                self.fVec18[2] = self.fVec18[1];
                self.fVec18[1] = self.fVec18[0];
                for j2 in (1..=6).rev() {
                    self.fVec19[j2 as usize] = self.fVec19[(i32::wrapping_sub(j2, 1)) as usize];
                }
                for j3 in (1..=14).rev() {
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
                for j4 in (1..=6).rev() {
                    self.fVec36[j4 as usize] = self.fVec36[(i32::wrapping_sub(j4, 1)) as usize];
                }
                for j5 in (1..=14).rev() {
                    self.fVec37[j5 as usize] = self.fVec37[(i32::wrapping_sub(j5, 1)) as usize];
                }
                self.fRec17[1] = self.fRec17[0];
                self.fVec47[2] = self.fVec47[1];
                self.fVec47[1] = self.fVec47[0];
                for j6 in (1..=6).rev() {
                    self.fVec48[j6 as usize] = self.fVec48[(i32::wrapping_sub(j6, 1)) as usize];
                }
                for j7 in (1..=14).rev() {
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
}

pub use dsp_96k::LambRs96k;
