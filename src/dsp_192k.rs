/* ------------------------------------------------------------
author: "Bart Brouns"
license: "AGPLv3"
name: "lamb-rs"
version: "0.1"
Code generated with Faust 2.70.3 (https://faust.grame.fr)
Compilation options: -a /tmp/nix-shell.if1VL5/nix-shell.lpEPEE/nix-shell.lxR5mE/nix-shell.Czhm7A/nix-shell.0pXTri/nix-shell.gDGq5b/nix-shell.Y8aHKo/nix-shell.FfgGgo/nix-shell.HDW8N8/nix-shell.2rUgRg/nix-shell.KMSd6T/nix-shell.bAMBSi/nix-shell.VAh39v/nix-shell.Jv8ErE/nix-shell.4NXoTm/nix-shell.HbCfCR/nix-shell.NR17fh/nix-shell.nikzHK/nix-shell.03t5uv/nix-shell.LtFvwY/nix-shell.kSThYB/nix-shell.JAm0Jp/nix-shell.Hc9lmr/nix-shell.d6GuaM/nix-shell.j4mBOt/nix-shell.gWNpEG/nix-shell.FNoWQI/nix-shell.yim3Px/nix-shell.liJmgd/nix-shell.sY3ver/nix-shell.Qsu3wd/nix-shell.Xc781a/.tmprmHL9Q -lang rust -ct 1 -cn LambRs192k -es 1 -mcd 16 -mdd 1024 -mdy 33 -double -ftz 0
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

        fn instance_initLambRs192kSIG0(&mut self, sample_rate: i32) {
            for l44 in 0..2 {
                self.iRec13[l44 as usize] = 0;
            }
        }

        fn fillLambRs192kSIG0(&mut self, count: i32, table: &mut [F64]) {
            for i1 in 0..count {
                self.iRec13[0] = i32::wrapping_add(self.iRec13[1], 1);
                let mut iTemp68: i32 = i32::wrapping_add(self.iRec13[0], -1);
                let mut fTemp69: F64 = (iTemp68 % 7) as F64 as i32 as F64;
                let mut fTemp70: F64 = 0.16666666666666666 * fTemp69;
                let mut fTemp71: F64 = F64::powf(fTemp70, 0.06999999999999999 * fTemp69 + 1.0);
                let mut fTemp72: F64 =
                    (0.14285714285714285 * (iTemp68 % 3670016) as F64) as i32 as F64;
                table[i1 as usize] = F64::min(
                    1.0,
                    F64::max(
                        0.0,
                        if (fTemp70 == 0.0) as i32 != 0 {
                            0.5 * (F64::sin(5.992123881747579e-06 * fTemp72 + 4.71238898038469)
                                + 1.0)
                        } else {
                            0.5 * (F64::sin(
                                3.141592653589793
                                    * ((1.0
                                        - F64::exp(-(4.615792495331755e-06 * fTemp71 * fTemp72)))
                                        / (1.0 - F64::exp(-(2.42 * fTemp71))))
                                    + 4.71238898038469,
                            ) + 1.0)
                        },
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
        fVec18: [F64; 16384],
        fRec3: [F64; 2],
        fVec19: [F64; 3],
        fVec20: [F64; 7],
        fVec21: [F64; 15],
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
        fVec38: [F64; 7],
        fVec39: [F64; 15],
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
        fVec51: [F64; 7],
        fVec52: [F64; 15],
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

    impl FaustDsp for LambRs192k {
        type T = F64;

        fn new() -> LambRs192k {
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
                fVec18: [0.0; 16384],
                fRec3: [0.0; 2],
                fVec19: [0.0; 3],
                fVec20: [0.0; 7],
                fVec21: [0.0; 15],
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
                fVec38: [0.0; 7],
                fVec39: [0.0; 15],
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
                fVec51: [0.0; 7],
                fVec52: [0.0; 15],
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
            m.declare("basics.lib/version", r"1.12.0");
            m.declare("compile_options", r"-a /tmp/nix-shell.if1VL5/nix-shell.lpEPEE/nix-shell.lxR5mE/nix-shell.Czhm7A/nix-shell.0pXTri/nix-shell.gDGq5b/nix-shell.Y8aHKo/nix-shell.FfgGgo/nix-shell.HDW8N8/nix-shell.2rUgRg/nix-shell.KMSd6T/nix-shell.bAMBSi/nix-shell.VAh39v/nix-shell.Jv8ErE/nix-shell.4NXoTm/nix-shell.HbCfCR/nix-shell.NR17fh/nix-shell.nikzHK/nix-shell.03t5uv/nix-shell.LtFvwY/nix-shell.kSThYB/nix-shell.JAm0Jp/nix-shell.Hc9lmr/nix-shell.d6GuaM/nix-shell.j4mBOt/nix-shell.gWNpEG/nix-shell.FNoWQI/nix-shell.yim3Px/nix-shell.liJmgd/nix-shell.sY3ver/nix-shell.Qsu3wd/nix-shell.Xc781a/.tmprmHL9Q -lang rust -ct 1 -cn LambRs192k -es 1 -mcd 16 -mdd 1024 -mdy 33 -double -ftz 0");
            m.declare("filename", r"lamb-rs-192k.dsp");
            m.declare(
                "interpolators.lib/interpolate_linear:author",
                r"Stéphane Letz",
            );
            m.declare("interpolators.lib/interpolate_linear:licence", r"MIT");
            m.declare("interpolators.lib/name", r"Faust Interpolator Library");
            m.declare("interpolators.lib/remap:author", r"David Braun");
            m.declare("interpolators.lib/version", r"1.3.0");
            m.declare("lamb.dsp/author", r"Bart Brouns");
            m.declare("lamb.dsp/license", r"AGPLv3");
            m.declare("lamb.dsp/name", r"lamb");
            m.declare("lamb.dsp/version", r"0.1");
            m.declare("license", r"AGPLv3");
            m.declare("maths.lib/author", r"GRAME");
            m.declare("maths.lib/copyright", r"GRAME");
            m.declare("maths.lib/license", r"LGPL with exception");
            m.declare("maths.lib/name", r"Faust Math Library");
            m.declare("maths.lib/version", r"2.7.0");
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
            let mut sig0: LambRs192kSIG0 = newLambRs192kSIG0();
            sig0.instance_initLambRs192kSIG0(sample_rate);
            sig0.fillLambRs192kSIG0(3670016, unsafe { &mut ftbl0LambRs192kSIG0 });
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
            for l28 in 0..16384 {
                self.fVec18[l28 as usize] = 0.0;
            }
            for l29 in 0..2 {
                self.fRec3[l29 as usize] = 0.0;
            }
            for l30 in 0..3 {
                self.fVec19[l30 as usize] = 0.0;
            }
            for l31 in 0..7 {
                self.fVec20[l31 as usize] = 0.0;
            }
            for l32 in 0..15 {
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
            for l53 in 0..7 {
                self.fVec38[l53 as usize] = 0.0;
            }
            for l54 in 0..15 {
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
            for l67 in 0..7 {
                self.fVec51[l67 as usize] = 0.0;
            }
            for l68 in 0..15 {
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
            LambRs192k::class_init(sample_rate);
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
            self.fHbargraph0 = if (fSlow77) as i32 != 0 {
                1.92e+04
            } else {
                fSlow76
            };
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
                                    fSlow12 * LambRs192k_faustpower2_f(fSlow7 + fTemp20)
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
                    (self.iVec0[((i32::wrapping_sub(self.IOTA0, 19200)) & 32767) as usize]) as F64;
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
                                    fSlow12 * LambRs192k_faustpower2_f(fSlow7 + fTemp40)
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
                                                                        if iSlow23 != 0 {
                                                                            fTemp41
                                                                        } else {
                                                                            1.7976931348623157e+308
                                                                        },
                                                                        if iSlow24 != 0 {
                                                                            self.fVec6
                                                                                [iSlow23 as usize]
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
                                                self.fVec13[((i32::wrapping_sub(
                                                    self.IOTA0, iSlow38,
                                                )) & 511)
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
                                        self.fVec15[((i32::wrapping_sub(self.IOTA0, iSlow42))
                                            & 2047)
                                            as usize]
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
                                self.fVec17
                                    [((i32::wrapping_sub(self.IOTA0, iSlow46)) & 8191) as usize]
                            } else {
                                1.7976931348623157e+308
                            },
                        ),
                        if iSlow47 != 0 {
                            self.fVec18[((i32::wrapping_sub(self.IOTA0, iSlow48)) & 16383) as usize]
                        } else {
                            1.7976931348623157e+308
                        },
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
                                                                    if iSlow4 != 0 {
                                                                        self.fRec3[0]
                                                                    } else {
                                                                        1.7976931348623157e+308
                                                                    },
                                                                    if iSlow49 != 0 {
                                                                        self.fVec19[iSlow4 as usize]
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
                                                                self.fVec21[iSlow53 as usize]
                                                            } else {
                                                                1.7976931348623157e+308
                                                            },
                                                        ),
                                                        if iSlow54 != 0 {
                                                            self.fVec22[((i32::wrapping_sub(
                                                                self.IOTA0, iSlow55,
                                                            )) & 31)
                                                                as usize]
                                                        } else {
                                                            1.7976931348623157e+308
                                                        },
                                                    ),
                                                    if iSlow56 != 0 {
                                                        self.fVec23[((i32::wrapping_sub(
                                                            self.IOTA0, iSlow57,
                                                        )) & 63)
                                                            as usize]
                                                    } else {
                                                        1.7976931348623157e+308
                                                    },
                                                ),
                                                if iSlow58 != 0 {
                                                    self.fVec24[((i32::wrapping_sub(
                                                        self.IOTA0, iSlow59,
                                                    )) & 127)
                                                        as usize]
                                                } else {
                                                    1.7976931348623157e+308
                                                },
                                            ),
                                            if iSlow60 != 0 {
                                                self.fVec25[((i32::wrapping_sub(
                                                    self.IOTA0, iSlow61,
                                                )) & 255)
                                                    as usize]
                                            } else {
                                                1.7976931348623157e+308
                                            },
                                        ),
                                        if iSlow62 != 0 {
                                            self.fVec26[((i32::wrapping_sub(self.IOTA0, iSlow63))
                                                & 511)
                                                as usize]
                                        } else {
                                            1.7976931348623157e+308
                                        },
                                    ),
                                    if iSlow64 != 0 {
                                        self.fVec27[((i32::wrapping_sub(self.IOTA0, iSlow65))
                                            & 1023)
                                            as usize]
                                    } else {
                                        1.7976931348623157e+308
                                    },
                                ),
                                if iSlow66 != 0 {
                                    self.fVec28
                                        [((i32::wrapping_sub(self.IOTA0, iSlow67)) & 2047) as usize]
                                } else {
                                    1.7976931348623157e+308
                                },
                            ),
                            if iSlow68 != 0 {
                                self.fVec29
                                    [((i32::wrapping_sub(self.IOTA0, iSlow69)) & 4095) as usize]
                            } else {
                                1.7976931348623157e+308
                            },
                        ),
                        if iSlow70 != 0 {
                            self.fVec30[((i32::wrapping_sub(self.IOTA0, iSlow71)) & 8191) as usize]
                        } else {
                            1.7976931348623157e+308
                        },
                    ),
                    if iSlow72 != 0 {
                        self.fVec31[((i32::wrapping_sub(self.IOTA0, iSlow73)) & 16383) as usize]
                    } else {
                        1.7976931348623157e+308
                    },
                ) - self.fRec2[1];
                self.fVec32[0] = fTemp66;
                let mut iTemp67: i32 = (fTemp66 > 0.0) as i32;
                let mut fTemp73: F64 = if iTemp67 != 0 { fSlow75 } else { fSlow74 };
                self.fVec33[0] = fTemp73;
                let mut fTemp74: F64 = 6.0 * fTemp73;
                let mut iTemp75: i32 = (fTemp74) as i32;
                let mut iTemp76: i32 = std::cmp::max(0, std::cmp::min(iTemp75, 6));
                let mut iTemp77: i32 = std::cmp::max(
                    0,
                    std::cmp::min(i32::wrapping_add(iTemp76, 1835001), 3670015),
                );
                let mut fTemp78: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp77, 7)) as usize] };
                let mut fTemp79: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp77 as usize] };
                let mut fTemp80: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp77, 1)) as usize] }
                        - fTemp79;
                let mut fTemp81: F64 = fTemp74 - (iTemp75) as F64;
                let mut fTemp82: F64 = fTemp79
                    + fTemp81 * fTemp80
                    + 0.5
                        * (fTemp78
                            - (fTemp79
                                + fTemp81
                                    * (fTemp80
                                        - (unsafe {
                                            ftbl0LambRs192kSIG0
                                                [(i32::wrapping_add(iTemp77, 8)) as usize]
                                        } - fTemp78))));
                let mut fTemp83: F64 = if iTemp67 != 0 { fTemp82 } else { 1.0 - fTemp82 };
                let mut iTemp84: i32 = (fTemp66 < 0.0) as i32;
                let mut fTemp85: F64 = fSlow1 * (iTemp84) as F64 + fSlow13 * (iTemp67) as F64;
                self.fVec34[0] = fTemp85;
                let mut fTemp86: F64 = self.fConst10 / fTemp85;
                let mut fTemp87: F64 = fTemp86 + 0.5;
                let mut fTemp88: F64 = 524287.0 * (1.0 - fTemp87);
                let mut iTemp89: i32 = (fTemp88) as i32;
                let mut iTemp90: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp89, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp91: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp90, 7)) as usize] };
                let mut fTemp92: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp90 as usize] };
                let mut fTemp93: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp90, 1)) as usize] }
                        - fTemp92;
                let mut fTemp94: F64 = 524287.0 * fTemp87;
                let mut iTemp95: i32 = (fTemp94) as i32;
                let mut iTemp96: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp95, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp97: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp96, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp98: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp96 as usize] };
                let mut fTemp99: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp96, 1), 3670015),
                    )) as usize]
                } - fTemp98;
                let mut fTemp100: F64 = 6.0 * self.fVec33[1];
                let mut iTemp101: i32 = (fTemp100) as i32;
                let mut iTemp102: i32 = std::cmp::max(0, std::cmp::min(iTemp101, 6));
                let mut fTemp103: F64 = 524287.0 * (1.0 - self.fRec1[1]);
                let mut iTemp104: i32 = (fTemp103) as i32;
                let mut iTemp105: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp104, 524287))),
                            iTemp102,
                        ),
                        3670015,
                    ),
                );
                let mut fTemp106: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp105, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp107: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp105 as usize] };
                let mut fTemp108: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp105, 1), 3670015),
                    )) as usize]
                } - fTemp107;
                let mut fTemp109: F64 = fTemp100 - (iTemp101) as F64;
                let mut fTemp110: F64 = 524287.0 * self.fRec1[1];
                let mut iTemp111: i32 = (fTemp110) as i32;
                let mut iTemp112: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp102,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp111, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp113: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp112, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp114: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp112 as usize] };
                let mut fTemp115: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp112, 1), 3670015),
                    )) as usize]
                } - fTemp114;
                let mut fTemp116: F64 = self.fRec1[1] + fTemp86;
                let mut fTemp117: F64 = 524287.0 * (1.0 - fTemp116);
                let mut iTemp118: i32 = (fTemp117) as i32;
                let mut iTemp119: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp118, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp120: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp119, 7)) as usize] };
                let mut fTemp121: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp119 as usize] };
                let mut fTemp122: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp119, 1)) as usize] }
                        - fTemp121;
                let mut fTemp123: F64 = 524287.0 * fTemp116;
                let mut iTemp124: i32 = (fTemp123) as i32;
                let mut iTemp125: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp124, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp126: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp125, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp127: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp125 as usize] };
                let mut fTemp128: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp125, 1), 3670015),
                    )) as usize]
                } - fTemp127;
                let mut fTemp129: F64 =
                    self.fRec1[1] + self.fConst10 * (1.0 / fTemp85 + 1.0 / self.fVec34[1]);
                let mut fTemp130: F64 = 524287.0 * (1.0 - fTemp129);
                let mut iTemp131: i32 = (fTemp130) as i32;
                let mut iTemp132: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp131, 524287))),
                            iTemp76,
                        ),
                        3670015,
                    ),
                );
                let mut fTemp133: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp132, 7)) as usize] };
                let mut fTemp134: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp132 as usize] };
                let mut fTemp135: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp132, 1)) as usize] }
                        - fTemp134;
                let mut fTemp136: F64 = 524287.0 * fTemp129;
                let mut iTemp137: i32 = (fTemp136) as i32;
                let mut iTemp138: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp137, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp139: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp138, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp140: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp138 as usize] };
                let mut fTemp141: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp138, 1), 3670015),
                    )) as usize]
                } - fTemp140;
                let mut fTemp142: F64 = (if iTemp67 != 0 {
                    fTemp140
                        + fTemp81 * fTemp141
                        + (fTemp136 - (iTemp137) as F64)
                            * (fTemp139
                                - (fTemp140
                                    + fTemp81
                                        * (fTemp141
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp138, 8),
                                                        3670015,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp139))))
                } else {
                    1.0 - (fTemp134
                        + fTemp81 * fTemp135
                        + (fTemp130 - (iTemp131) as F64)
                            * (fTemp133
                                - (fTemp134
                                    + fTemp81
                                        * (fTemp135
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp132, 8)) as usize]
                                            } - fTemp133)))))
                } - if iTemp67 != 0 {
                    fTemp127
                        + fTemp81 * fTemp128
                        + (fTemp123 - (iTemp124) as F64)
                            * (fTemp126
                                - (fTemp127
                                    + fTemp81
                                        * (fTemp128
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp125, 8),
                                                        3670015,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp126))))
                } else {
                    1.0 - (fTemp121
                        + fTemp81 * fTemp122
                        + (fTemp117 - (iTemp118) as F64)
                            * (fTemp120
                                - (fTemp121
                                    + fTemp81
                                        * (fTemp122
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp119, 8)) as usize]
                                            } - fTemp120)))))
                }) * self.fVec32[1]
                    / (fTemp66
                        * (1.0
                            - if iTemp67 != 0 {
                                fTemp114
                                    + fTemp109 * fTemp115
                                    + (fTemp110 - (iTemp111) as F64)
                                        * (fTemp113
                                            - (fTemp114
                                                + fTemp109
                                                    * (fTemp115
                                                        - (unsafe {
                                                            ftbl0LambRs192kSIG0[(std::cmp::max(
                                                                0,
                                                                std::cmp::min(
                                                                    i32::wrapping_add(iTemp112, 8),
                                                                    3670015,
                                                                ),
                                                            ))
                                                                as usize]
                                                        } - fTemp113))))
                            } else {
                                1.0 - (fTemp107
                                    + fTemp109 * fTemp108
                                    + (fTemp103 - (iTemp104) as F64)
                                        * (fTemp106
                                            - (fTemp107
                                                + fTemp109
                                                    * (fTemp108
                                                        - (unsafe {
                                                            ftbl0LambRs192kSIG0[(std::cmp::max(
                                                                0,
                                                                std::cmp::min(
                                                                    i32::wrapping_add(iTemp105, 8),
                                                                    3670015,
                                                                ),
                                                            ))
                                                                as usize]
                                                        } - fTemp106)))))
                            }));
                let mut iTemp143: i32 = (fTemp142
                    > ((if iTemp67 != 0 {
                        fTemp98
                            + fTemp81 * fTemp99
                            + (fTemp94 - (iTemp95) as F64)
                                * (fTemp97
                                    - (fTemp98
                                        + fTemp81
                                            * (fTemp99
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp96, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp97))))
                    } else {
                        1.0 - (fTemp92
                            + fTemp81 * fTemp93
                            + (fTemp88 - (iTemp89) as F64)
                                * (fTemp91
                                    - (fTemp92
                                        + fTemp81
                                            * (fTemp93
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp90, 8)) as usize]
                                                } - fTemp91)))))
                    } - fTemp83)
                        / (1.0 - fTemp83))) as i32;
                let mut fTemp144: F64 = if iTemp143 != 0 { 1.0 } else { 0.5 };
                let mut fTemp145: F64 = if iTemp143 != 0 { 0.5 } else { 0.0 };
                let mut fTemp146: F64 = fTemp145 + fTemp144;
                let mut fTemp147: F64 = 0.5 * fTemp146;
                let mut fTemp148: F64 = 524287.0 * (1.0 - fTemp147);
                let mut iTemp149: i32 = (fTemp148) as i32;
                let mut iTemp150: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp149, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp151: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp150, 7)) as usize] };
                let mut fTemp152: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp150 as usize] };
                let mut fTemp153: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp150, 1)) as usize] }
                        - fTemp152;
                let mut fTemp154: F64 = 262143.5 * fTemp146;
                let mut iTemp155: i32 = (fTemp154) as i32;
                let mut iTemp156: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp155, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp157: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp156, 7)) as usize] };
                let mut fTemp158: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp156 as usize] };
                let mut fTemp159: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp156, 1)) as usize] }
                        - fTemp158;
                let mut fTemp160: F64 = if iTemp67 != 0 {
                    fTemp158
                        + fTemp81 * fTemp159
                        + (fTemp154 - (iTemp155) as F64)
                            * (fTemp157
                                - (fTemp158
                                    + fTemp81
                                        * (fTemp159
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp156, 8)) as usize]
                                            } - fTemp157))))
                } else {
                    1.0 - (fTemp152
                        + fTemp81 * fTemp153
                        + (fTemp148 - (iTemp149) as F64)
                            * (fTemp151
                                - (fTemp152
                                    + fTemp81
                                        * (fTemp153
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp150, 8)) as usize]
                                            } - fTemp151)))))
                };
                let mut fTemp161: F64 = fTemp86 + fTemp147;
                let mut fTemp162: F64 = 524287.0 * (1.0 - fTemp161);
                let mut iTemp163: i32 = (fTemp162) as i32;
                let mut iTemp164: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp163, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp165: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp164, 7)) as usize] };
                let mut fTemp166: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp164 as usize] };
                let mut fTemp167: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp164, 1)) as usize] }
                        - fTemp166;
                let mut fTemp168: F64 = 524287.0 * fTemp161;
                let mut iTemp169: i32 = (fTemp168) as i32;
                let mut iTemp170: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp169, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp171: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp170, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp172: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp170 as usize] };
                let mut fTemp173: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp170, 1), 3670015),
                    )) as usize]
                } - fTemp172;
                let mut iTemp174: i32 = (fTemp142
                    > ((if iTemp67 != 0 {
                        fTemp172
                            + fTemp81 * fTemp173
                            + (fTemp168 - (iTemp169) as F64)
                                * (fTemp171
                                    - (fTemp172
                                        + fTemp81
                                            * (fTemp173
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp170, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp171))))
                    } else {
                        1.0 - (fTemp166
                            + fTemp81 * fTemp167
                            + (fTemp162 - (iTemp163) as F64)
                                * (fTemp165
                                    - (fTemp166
                                        + fTemp81
                                            * (fTemp167
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp164, 8)) as usize]
                                                } - fTemp165)))))
                    } - fTemp160)
                        / (1.0 - fTemp160))) as i32;
                let mut fTemp175: F64 = if iTemp174 != 0 { fTemp144 } else { fTemp147 };
                let mut fTemp176: F64 = if iTemp174 != 0 { fTemp147 } else { fTemp145 };
                let mut fTemp177: F64 = fTemp176 + fTemp175;
                let mut fTemp178: F64 = 0.5 * fTemp177;
                let mut fTemp179: F64 = 524287.0 * (1.0 - fTemp178);
                let mut iTemp180: i32 = (fTemp179) as i32;
                let mut iTemp181: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp180, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp182: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp181, 7)) as usize] };
                let mut fTemp183: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp181 as usize] };
                let mut fTemp184: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp181, 1)) as usize] }
                        - fTemp183;
                let mut fTemp185: F64 = 262143.5 * fTemp177;
                let mut iTemp186: i32 = (fTemp185) as i32;
                let mut iTemp187: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp186, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp188: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp187, 7)) as usize] };
                let mut fTemp189: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp187 as usize] };
                let mut fTemp190: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp187, 1)) as usize] }
                        - fTemp189;
                let mut fTemp191: F64 = if iTemp67 != 0 {
                    fTemp189
                        + fTemp81 * fTemp190
                        + (fTemp185 - (iTemp186) as F64)
                            * (fTemp188
                                - (fTemp189
                                    + fTemp81
                                        * (fTemp190
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp187, 8)) as usize]
                                            } - fTemp188))))
                } else {
                    1.0 - (fTemp183
                        + fTemp81 * fTemp184
                        + (fTemp179 - (iTemp180) as F64)
                            * (fTemp182
                                - (fTemp183
                                    + fTemp81
                                        * (fTemp184
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp181, 8)) as usize]
                                            } - fTemp182)))))
                };
                let mut fTemp192: F64 = fTemp86 + fTemp178;
                let mut fTemp193: F64 = 524287.0 * (1.0 - fTemp192);
                let mut iTemp194: i32 = (fTemp193) as i32;
                let mut iTemp195: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp194, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp196: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp195, 7)) as usize] };
                let mut fTemp197: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp195 as usize] };
                let mut fTemp198: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp195, 1)) as usize] }
                        - fTemp197;
                let mut fTemp199: F64 = 524287.0 * fTemp192;
                let mut iTemp200: i32 = (fTemp199) as i32;
                let mut iTemp201: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp200, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp202: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp201, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp203: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp201 as usize] };
                let mut fTemp204: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp201, 1), 3670015),
                    )) as usize]
                } - fTemp203;
                let mut iTemp205: i32 = (fTemp142
                    > ((if iTemp67 != 0 {
                        fTemp203
                            + fTemp81 * fTemp204
                            + (fTemp199 - (iTemp200) as F64)
                                * (fTemp202
                                    - (fTemp203
                                        + fTemp81
                                            * (fTemp204
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp201, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp202))))
                    } else {
                        1.0 - (fTemp197
                            + fTemp81 * fTemp198
                            + (fTemp193 - (iTemp194) as F64)
                                * (fTemp196
                                    - (fTemp197
                                        + fTemp81
                                            * (fTemp198
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp195, 8)) as usize]
                                                } - fTemp196)))))
                    } - fTemp191)
                        / (1.0 - fTemp191))) as i32;
                let mut fTemp206: F64 = if iTemp205 != 0 { fTemp175 } else { fTemp178 };
                let mut fTemp207: F64 = if iTemp205 != 0 { fTemp178 } else { fTemp176 };
                let mut fTemp208: F64 = fTemp207 + fTemp206;
                let mut fTemp209: F64 = 0.5 * fTemp208;
                let mut fTemp210: F64 = 524287.0 * (1.0 - fTemp209);
                let mut iTemp211: i32 = (fTemp210) as i32;
                let mut iTemp212: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp211, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp213: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp212, 7)) as usize] };
                let mut fTemp214: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp212 as usize] };
                let mut fTemp215: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp212, 1)) as usize] }
                        - fTemp214;
                let mut fTemp216: F64 = 262143.5 * fTemp208;
                let mut iTemp217: i32 = (fTemp216) as i32;
                let mut iTemp218: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp217, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp219: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp218, 7)) as usize] };
                let mut fTemp220: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp218 as usize] };
                let mut fTemp221: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp218, 1)) as usize] }
                        - fTemp220;
                let mut fTemp222: F64 = if iTemp67 != 0 {
                    fTemp220
                        + fTemp81 * fTemp221
                        + (fTemp216 - (iTemp217) as F64)
                            * (fTemp219
                                - (fTemp220
                                    + fTemp81
                                        * (fTemp221
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp218, 8)) as usize]
                                            } - fTemp219))))
                } else {
                    1.0 - (fTemp214
                        + fTemp81 * fTemp215
                        + (fTemp210 - (iTemp211) as F64)
                            * (fTemp213
                                - (fTemp214
                                    + fTemp81
                                        * (fTemp215
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp212, 8)) as usize]
                                            } - fTemp213)))))
                };
                let mut fTemp223: F64 = fTemp86 + fTemp209;
                let mut fTemp224: F64 = 524287.0 * (1.0 - fTemp223);
                let mut iTemp225: i32 = (fTemp224) as i32;
                let mut iTemp226: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp225, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp227: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp226, 7)) as usize] };
                let mut fTemp228: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp226 as usize] };
                let mut fTemp229: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp226, 1)) as usize] }
                        - fTemp228;
                let mut fTemp230: F64 = 524287.0 * fTemp223;
                let mut iTemp231: i32 = (fTemp230) as i32;
                let mut iTemp232: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp231, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp233: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp232, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp234: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp232 as usize] };
                let mut fTemp235: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp232, 1), 3670015),
                    )) as usize]
                } - fTemp234;
                let mut iTemp236: i32 = (fTemp142
                    > ((if iTemp67 != 0 {
                        fTemp234
                            + fTemp81 * fTemp235
                            + (fTemp230 - (iTemp231) as F64)
                                * (fTemp233
                                    - (fTemp234
                                        + fTemp81
                                            * (fTemp235
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp232, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp233))))
                    } else {
                        1.0 - (fTemp228
                            + fTemp81 * fTemp229
                            + (fTemp224 - (iTemp225) as F64)
                                * (fTemp227
                                    - (fTemp228
                                        + fTemp81
                                            * (fTemp229
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp226, 8)) as usize]
                                                } - fTemp227)))))
                    } - fTemp222)
                        / (1.0 - fTemp222))) as i32;
                let mut fTemp237: F64 = if iTemp236 != 0 { fTemp206 } else { fTemp209 };
                let mut fTemp238: F64 = if iTemp236 != 0 { fTemp209 } else { fTemp207 };
                let mut fTemp239: F64 = fTemp238 + fTemp237;
                let mut fTemp240: F64 = 0.5 * fTemp239;
                let mut fTemp241: F64 = 524287.0 * (1.0 - fTemp240);
                let mut iTemp242: i32 = (fTemp241) as i32;
                let mut iTemp243: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp242, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp244: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp243, 7)) as usize] };
                let mut fTemp245: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp243 as usize] };
                let mut fTemp246: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp243, 1)) as usize] }
                        - fTemp245;
                let mut fTemp247: F64 = 262143.5 * fTemp239;
                let mut iTemp248: i32 = (fTemp247) as i32;
                let mut iTemp249: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp248, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp250: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp249, 7)) as usize] };
                let mut fTemp251: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp249 as usize] };
                let mut fTemp252: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp249, 1)) as usize] }
                        - fTemp251;
                let mut fTemp253: F64 = if iTemp67 != 0 {
                    fTemp251
                        + fTemp81 * fTemp252
                        + (fTemp247 - (iTemp248) as F64)
                            * (fTemp250
                                - (fTemp251
                                    + fTemp81
                                        * (fTemp252
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp249, 8)) as usize]
                                            } - fTemp250))))
                } else {
                    1.0 - (fTemp245
                        + fTemp81 * fTemp246
                        + (fTemp241 - (iTemp242) as F64)
                            * (fTemp244
                                - (fTemp245
                                    + fTemp81
                                        * (fTemp246
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp243, 8)) as usize]
                                            } - fTemp244)))))
                };
                let mut fTemp254: F64 = fTemp86 + fTemp240;
                let mut fTemp255: F64 = 524287.0 * (1.0 - fTemp254);
                let mut iTemp256: i32 = (fTemp255) as i32;
                let mut iTemp257: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp256, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp258: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp257, 7)) as usize] };
                let mut fTemp259: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp257 as usize] };
                let mut fTemp260: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp257, 1)) as usize] }
                        - fTemp259;
                let mut fTemp261: F64 = 524287.0 * fTemp254;
                let mut iTemp262: i32 = (fTemp261) as i32;
                let mut iTemp263: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp262, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp264: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp263, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp265: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp263 as usize] };
                let mut fTemp266: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp263, 1), 3670015),
                    )) as usize]
                } - fTemp265;
                let mut iTemp267: i32 = (fTemp142
                    > ((if iTemp67 != 0 {
                        fTemp265
                            + fTemp81 * fTemp266
                            + (fTemp261 - (iTemp262) as F64)
                                * (fTemp264
                                    - (fTemp265
                                        + fTemp81
                                            * (fTemp266
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp263, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp264))))
                    } else {
                        1.0 - (fTemp259
                            + fTemp81 * fTemp260
                            + (fTemp255 - (iTemp256) as F64)
                                * (fTemp258
                                    - (fTemp259
                                        + fTemp81
                                            * (fTemp260
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp257, 8)) as usize]
                                                } - fTemp258)))))
                    } - fTemp253)
                        / (1.0 - fTemp253))) as i32;
                let mut fTemp268: F64 = if iTemp267 != 0 { fTemp237 } else { fTemp240 };
                let mut fTemp269: F64 = if iTemp267 != 0 { fTemp240 } else { fTemp238 };
                let mut fTemp270: F64 = fTemp269 + fTemp268;
                let mut fTemp271: F64 = 0.5 * fTemp270;
                let mut fTemp272: F64 = 524287.0 * (1.0 - fTemp271);
                let mut iTemp273: i32 = (fTemp272) as i32;
                let mut iTemp274: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp273, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp275: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp274, 7)) as usize] };
                let mut fTemp276: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp274 as usize] };
                let mut fTemp277: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp274, 1)) as usize] }
                        - fTemp276;
                let mut fTemp278: F64 = 262143.5 * fTemp270;
                let mut iTemp279: i32 = (fTemp278) as i32;
                let mut iTemp280: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp279, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp281: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp280, 7)) as usize] };
                let mut fTemp282: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp280 as usize] };
                let mut fTemp283: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp280, 1)) as usize] }
                        - fTemp282;
                let mut fTemp284: F64 = if iTemp67 != 0 {
                    fTemp282
                        + fTemp81 * fTemp283
                        + (fTemp278 - (iTemp279) as F64)
                            * (fTemp281
                                - (fTemp282
                                    + fTemp81
                                        * (fTemp283
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp280, 8)) as usize]
                                            } - fTemp281))))
                } else {
                    1.0 - (fTemp276
                        + fTemp81 * fTemp277
                        + (fTemp272 - (iTemp273) as F64)
                            * (fTemp275
                                - (fTemp276
                                    + fTemp81
                                        * (fTemp277
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp274, 8)) as usize]
                                            } - fTemp275)))))
                };
                let mut fTemp285: F64 = fTemp86 + fTemp271;
                let mut fTemp286: F64 = 524287.0 * (1.0 - fTemp285);
                let mut iTemp287: i32 = (fTemp286) as i32;
                let mut iTemp288: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp287, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp289: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp288, 7)) as usize] };
                let mut fTemp290: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp288 as usize] };
                let mut fTemp291: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp288, 1)) as usize] }
                        - fTemp290;
                let mut fTemp292: F64 = 524287.0 * fTemp285;
                let mut iTemp293: i32 = (fTemp292) as i32;
                let mut iTemp294: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp293, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp295: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp294, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp296: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp294 as usize] };
                let mut fTemp297: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp294, 1), 3670015),
                    )) as usize]
                } - fTemp296;
                let mut iTemp298: i32 = (fTemp142
                    > ((if iTemp67 != 0 {
                        fTemp296
                            + fTemp81 * fTemp297
                            + (fTemp292 - (iTemp293) as F64)
                                * (fTemp295
                                    - (fTemp296
                                        + fTemp81
                                            * (fTemp297
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp294, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp295))))
                    } else {
                        1.0 - (fTemp290
                            + fTemp81 * fTemp291
                            + (fTemp286 - (iTemp287) as F64)
                                * (fTemp289
                                    - (fTemp290
                                        + fTemp81
                                            * (fTemp291
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp288, 8)) as usize]
                                                } - fTemp289)))))
                    } - fTemp284)
                        / (1.0 - fTemp284))) as i32;
                let mut fTemp299: F64 = if iTemp298 != 0 { fTemp268 } else { fTemp271 };
                let mut fTemp300: F64 = if iTemp298 != 0 { fTemp271 } else { fTemp269 };
                let mut fTemp301: F64 = fTemp300 + fTemp299;
                let mut fTemp302: F64 = 0.5 * fTemp301;
                let mut fTemp303: F64 = 524287.0 * (1.0 - fTemp302);
                let mut iTemp304: i32 = (fTemp303) as i32;
                let mut iTemp305: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp304, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp306: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp305, 7)) as usize] };
                let mut fTemp307: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp305 as usize] };
                let mut fTemp308: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp305, 1)) as usize] }
                        - fTemp307;
                let mut fTemp309: F64 = 262143.5 * fTemp301;
                let mut iTemp310: i32 = (fTemp309) as i32;
                let mut iTemp311: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp310, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp312: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp311, 7)) as usize] };
                let mut fTemp313: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp311 as usize] };
                let mut fTemp314: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp311, 1)) as usize] }
                        - fTemp313;
                let mut fTemp315: F64 = if iTemp67 != 0 {
                    fTemp313
                        + fTemp81 * fTemp314
                        + (fTemp309 - (iTemp310) as F64)
                            * (fTemp312
                                - (fTemp313
                                    + fTemp81
                                        * (fTemp314
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp311, 8)) as usize]
                                            } - fTemp312))))
                } else {
                    1.0 - (fTemp307
                        + fTemp81 * fTemp308
                        + (fTemp303 - (iTemp304) as F64)
                            * (fTemp306
                                - (fTemp307
                                    + fTemp81
                                        * (fTemp308
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp305, 8)) as usize]
                                            } - fTemp306)))))
                };
                let mut fTemp316: F64 = fTemp86 + fTemp302;
                let mut fTemp317: F64 = 524287.0 * (1.0 - fTemp316);
                let mut iTemp318: i32 = (fTemp317) as i32;
                let mut iTemp319: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp318, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp320: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp319, 7)) as usize] };
                let mut fTemp321: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp319 as usize] };
                let mut fTemp322: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp319, 1)) as usize] }
                        - fTemp321;
                let mut fTemp323: F64 = 524287.0 * fTemp316;
                let mut iTemp324: i32 = (fTemp323) as i32;
                let mut iTemp325: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp324, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp326: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp325, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp327: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp325 as usize] };
                let mut fTemp328: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp325, 1), 3670015),
                    )) as usize]
                } - fTemp327;
                let mut iTemp329: i32 = (fTemp142
                    > ((if iTemp67 != 0 {
                        fTemp327
                            + fTemp81 * fTemp328
                            + (fTemp323 - (iTemp324) as F64)
                                * (fTemp326
                                    - (fTemp327
                                        + fTemp81
                                            * (fTemp328
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp325, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp326))))
                    } else {
                        1.0 - (fTemp321
                            + fTemp81 * fTemp322
                            + (fTemp317 - (iTemp318) as F64)
                                * (fTemp320
                                    - (fTemp321
                                        + fTemp81
                                            * (fTemp322
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp319, 8)) as usize]
                                                } - fTemp320)))))
                    } - fTemp315)
                        / (1.0 - fTemp315))) as i32;
                let mut fTemp330: F64 = if iTemp329 != 0 { fTemp299 } else { fTemp302 };
                let mut fTemp331: F64 = if iTemp329 != 0 { fTemp302 } else { fTemp300 };
                let mut fTemp332: F64 = fTemp331 + fTemp330;
                let mut fTemp333: F64 = 0.5 * fTemp332;
                let mut fTemp334: F64 = 524287.0 * (1.0 - fTemp333);
                let mut iTemp335: i32 = (fTemp334) as i32;
                let mut iTemp336: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp335, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp337: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp336, 7)) as usize] };
                let mut fTemp338: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp336 as usize] };
                let mut fTemp339: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp336, 1)) as usize] }
                        - fTemp338;
                let mut fTemp340: F64 = 262143.5 * fTemp332;
                let mut iTemp341: i32 = (fTemp340) as i32;
                let mut iTemp342: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp341, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp343: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp342, 7)) as usize] };
                let mut fTemp344: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp342 as usize] };
                let mut fTemp345: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp342, 1)) as usize] }
                        - fTemp344;
                let mut fTemp346: F64 = if iTemp67 != 0 {
                    fTemp344
                        + fTemp81 * fTemp345
                        + (fTemp340 - (iTemp341) as F64)
                            * (fTemp343
                                - (fTemp344
                                    + fTemp81
                                        * (fTemp345
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp342, 8)) as usize]
                                            } - fTemp343))))
                } else {
                    1.0 - (fTemp338
                        + fTemp81 * fTemp339
                        + (fTemp334 - (iTemp335) as F64)
                            * (fTemp337
                                - (fTemp338
                                    + fTemp81
                                        * (fTemp339
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp336, 8)) as usize]
                                            } - fTemp337)))))
                };
                let mut fTemp347: F64 = fTemp86 + fTemp333;
                let mut fTemp348: F64 = 524287.0 * (1.0 - fTemp347);
                let mut iTemp349: i32 = (fTemp348) as i32;
                let mut iTemp350: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp349, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp351: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp350, 7)) as usize] };
                let mut fTemp352: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp350 as usize] };
                let mut fTemp353: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp350, 1)) as usize] }
                        - fTemp352;
                let mut fTemp354: F64 = 524287.0 * fTemp347;
                let mut iTemp355: i32 = (fTemp354) as i32;
                let mut iTemp356: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp355, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp357: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp356, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp358: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp356 as usize] };
                let mut fTemp359: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp356, 1), 3670015),
                    )) as usize]
                } - fTemp358;
                let mut iTemp360: i32 = (fTemp142
                    > ((if iTemp67 != 0 {
                        fTemp358
                            + fTemp81 * fTemp359
                            + (fTemp354 - (iTemp355) as F64)
                                * (fTemp357
                                    - (fTemp358
                                        + fTemp81
                                            * (fTemp359
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp356, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp357))))
                    } else {
                        1.0 - (fTemp352
                            + fTemp81 * fTemp353
                            + (fTemp348 - (iTemp349) as F64)
                                * (fTemp351
                                    - (fTemp352
                                        + fTemp81
                                            * (fTemp353
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp350, 8)) as usize]
                                                } - fTemp351)))))
                    } - fTemp346)
                        / (1.0 - fTemp346))) as i32;
                let mut fTemp361: F64 = if iTemp360 != 0 { fTemp330 } else { fTemp333 };
                let mut fTemp362: F64 = if iTemp360 != 0 { fTemp333 } else { fTemp331 };
                let mut fTemp363: F64 = fTemp362 + fTemp361;
                let mut fTemp364: F64 = 0.5 * fTemp363;
                let mut fTemp365: F64 = 524287.0 * (1.0 - fTemp364);
                let mut iTemp366: i32 = (fTemp365) as i32;
                let mut iTemp367: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp366, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp368: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp367, 7)) as usize] };
                let mut fTemp369: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp367 as usize] };
                let mut fTemp370: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp367, 1)) as usize] }
                        - fTemp369;
                let mut fTemp371: F64 = 262143.5 * fTemp363;
                let mut iTemp372: i32 = (fTemp371) as i32;
                let mut iTemp373: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp372, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp374: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp373, 7)) as usize] };
                let mut fTemp375: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp373 as usize] };
                let mut fTemp376: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp373, 1)) as usize] }
                        - fTemp375;
                let mut fTemp377: F64 = if iTemp67 != 0 {
                    fTemp375
                        + fTemp81 * fTemp376
                        + (fTemp371 - (iTemp372) as F64)
                            * (fTemp374
                                - (fTemp375
                                    + fTemp81
                                        * (fTemp376
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp373, 8)) as usize]
                                            } - fTemp374))))
                } else {
                    1.0 - (fTemp369
                        + fTemp81 * fTemp370
                        + (fTemp365 - (iTemp366) as F64)
                            * (fTemp368
                                - (fTemp369
                                    + fTemp81
                                        * (fTemp370
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp367, 8)) as usize]
                                            } - fTemp368)))))
                };
                let mut fTemp378: F64 = fTemp86 + fTemp364;
                let mut fTemp379: F64 = 524287.0 * (1.0 - fTemp378);
                let mut iTemp380: i32 = (fTemp379) as i32;
                let mut iTemp381: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp380, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp382: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp381, 7)) as usize] };
                let mut fTemp383: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp381 as usize] };
                let mut fTemp384: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp381, 1)) as usize] }
                        - fTemp383;
                let mut fTemp385: F64 = 524287.0 * fTemp378;
                let mut iTemp386: i32 = (fTemp385) as i32;
                let mut iTemp387: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp386, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp388: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp387, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp389: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp387 as usize] };
                let mut fTemp390: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp387, 1), 3670015),
                    )) as usize]
                } - fTemp389;
                let mut iTemp391: i32 = (fTemp142
                    > ((if iTemp67 != 0 {
                        fTemp389
                            + fTemp81 * fTemp390
                            + (fTemp385 - (iTemp386) as F64)
                                * (fTemp388
                                    - (fTemp389
                                        + fTemp81
                                            * (fTemp390
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp387, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp388))))
                    } else {
                        1.0 - (fTemp383
                            + fTemp81 * fTemp384
                            + (fTemp379 - (iTemp380) as F64)
                                * (fTemp382
                                    - (fTemp383
                                        + fTemp81
                                            * (fTemp384
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp381, 8)) as usize]
                                                } - fTemp382)))))
                    } - fTemp377)
                        / (1.0 - fTemp377))) as i32;
                let mut fTemp392: F64 = if iTemp391 != 0 { fTemp361 } else { fTemp364 };
                let mut fTemp393: F64 = if iTemp391 != 0 { fTemp364 } else { fTemp362 };
                let mut fTemp394: F64 = fTemp393 + fTemp392;
                let mut fTemp395: F64 = 0.5 * fTemp394;
                let mut fTemp396: F64 = 524287.0 * (1.0 - fTemp395);
                let mut iTemp397: i32 = (fTemp396) as i32;
                let mut iTemp398: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp397, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp399: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp398, 7)) as usize] };
                let mut fTemp400: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp398 as usize] };
                let mut fTemp401: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp398, 1)) as usize] }
                        - fTemp400;
                let mut fTemp402: F64 = 262143.5 * fTemp394;
                let mut iTemp403: i32 = (fTemp402) as i32;
                let mut iTemp404: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp403, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp405: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp404, 7)) as usize] };
                let mut fTemp406: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp404 as usize] };
                let mut fTemp407: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp404, 1)) as usize] }
                        - fTemp406;
                let mut fTemp408: F64 = if iTemp67 != 0 {
                    fTemp406
                        + fTemp81 * fTemp407
                        + (fTemp402 - (iTemp403) as F64)
                            * (fTemp405
                                - (fTemp406
                                    + fTemp81
                                        * (fTemp407
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp404, 8)) as usize]
                                            } - fTemp405))))
                } else {
                    1.0 - (fTemp400
                        + fTemp81 * fTemp401
                        + (fTemp396 - (iTemp397) as F64)
                            * (fTemp399
                                - (fTemp400
                                    + fTemp81
                                        * (fTemp401
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp398, 8)) as usize]
                                            } - fTemp399)))))
                };
                let mut fTemp409: F64 = fTemp86 + fTemp395;
                let mut fTemp410: F64 = 524287.0 * (1.0 - fTemp409);
                let mut iTemp411: i32 = (fTemp410) as i32;
                let mut iTemp412: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp411, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp413: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp412, 7)) as usize] };
                let mut fTemp414: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp412 as usize] };
                let mut fTemp415: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp412, 1)) as usize] }
                        - fTemp414;
                let mut fTemp416: F64 = 524287.0 * fTemp409;
                let mut iTemp417: i32 = (fTemp416) as i32;
                let mut iTemp418: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp417, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp419: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp418, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp420: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp418 as usize] };
                let mut fTemp421: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp418, 1), 3670015),
                    )) as usize]
                } - fTemp420;
                let mut iTemp422: i32 = (fTemp142
                    > ((if iTemp67 != 0 {
                        fTemp420
                            + fTemp81 * fTemp421
                            + (fTemp416 - (iTemp417) as F64)
                                * (fTemp419
                                    - (fTemp420
                                        + fTemp81
                                            * (fTemp421
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp418, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp419))))
                    } else {
                        1.0 - (fTemp414
                            + fTemp81 * fTemp415
                            + (fTemp410 - (iTemp411) as F64)
                                * (fTemp413
                                    - (fTemp414
                                        + fTemp81
                                            * (fTemp415
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp412, 8)) as usize]
                                                } - fTemp413)))))
                    } - fTemp408)
                        / (1.0 - fTemp408))) as i32;
                let mut fTemp423: F64 = if iTemp422 != 0 { fTemp392 } else { fTemp395 };
                let mut fTemp424: F64 = if iTemp422 != 0 { fTemp395 } else { fTemp393 };
                let mut fTemp425: F64 = fTemp424 + fTemp423;
                let mut fTemp426: F64 = 0.5 * fTemp425;
                let mut fTemp427: F64 = 524287.0 * (1.0 - fTemp426);
                let mut iTemp428: i32 = (fTemp427) as i32;
                let mut iTemp429: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp428, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp430: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp429, 7)) as usize] };
                let mut fTemp431: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp429 as usize] };
                let mut fTemp432: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp429, 1)) as usize] }
                        - fTemp431;
                let mut fTemp433: F64 = 262143.5 * fTemp425;
                let mut iTemp434: i32 = (fTemp433) as i32;
                let mut iTemp435: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp434, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp436: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp435, 7)) as usize] };
                let mut fTemp437: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp435 as usize] };
                let mut fTemp438: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp435, 1)) as usize] }
                        - fTemp437;
                let mut fTemp439: F64 = if iTemp67 != 0 {
                    fTemp437
                        + fTemp81 * fTemp438
                        + (fTemp433 - (iTemp434) as F64)
                            * (fTemp436
                                - (fTemp437
                                    + fTemp81
                                        * (fTemp438
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp435, 8)) as usize]
                                            } - fTemp436))))
                } else {
                    1.0 - (fTemp431
                        + fTemp81 * fTemp432
                        + (fTemp427 - (iTemp428) as F64)
                            * (fTemp430
                                - (fTemp431
                                    + fTemp81
                                        * (fTemp432
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp429, 8)) as usize]
                                            } - fTemp430)))))
                };
                let mut fTemp440: F64 = fTemp86 + fTemp426;
                let mut fTemp441: F64 = 524287.0 * (1.0 - fTemp440);
                let mut iTemp442: i32 = (fTemp441) as i32;
                let mut iTemp443: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp442, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp444: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp443, 7)) as usize] };
                let mut fTemp445: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp443 as usize] };
                let mut fTemp446: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp443, 1)) as usize] }
                        - fTemp445;
                let mut fTemp447: F64 = 524287.0 * fTemp440;
                let mut iTemp448: i32 = (fTemp447) as i32;
                let mut iTemp449: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp448, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp450: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp449, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp451: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp449 as usize] };
                let mut fTemp452: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp449, 1), 3670015),
                    )) as usize]
                } - fTemp451;
                let mut iTemp453: i32 = (fTemp142
                    > ((if iTemp67 != 0 {
                        fTemp451
                            + fTemp81 * fTemp452
                            + (fTemp447 - (iTemp448) as F64)
                                * (fTemp450
                                    - (fTemp451
                                        + fTemp81
                                            * (fTemp452
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp449, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp450))))
                    } else {
                        1.0 - (fTemp445
                            + fTemp81 * fTemp446
                            + (fTemp441 - (iTemp442) as F64)
                                * (fTemp444
                                    - (fTemp445
                                        + fTemp81
                                            * (fTemp446
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp443, 8)) as usize]
                                                } - fTemp444)))))
                    } - fTemp439)
                        / (1.0 - fTemp439))) as i32;
                let mut fTemp454: F64 = if iTemp453 != 0 { fTemp423 } else { fTemp426 };
                let mut fTemp455: F64 = if iTemp453 != 0 { fTemp426 } else { fTemp424 };
                let mut fTemp456: F64 = fTemp455 + fTemp454;
                let mut fTemp457: F64 = 0.5 * fTemp456;
                let mut fTemp458: F64 = 524287.0 * (1.0 - fTemp457);
                let mut iTemp459: i32 = (fTemp458) as i32;
                let mut iTemp460: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp459, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp461: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp460, 7)) as usize] };
                let mut fTemp462: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp460 as usize] };
                let mut fTemp463: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp460, 1)) as usize] }
                        - fTemp462;
                let mut fTemp464: F64 = 262143.5 * fTemp456;
                let mut iTemp465: i32 = (fTemp464) as i32;
                let mut iTemp466: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp465, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp467: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp466, 7)) as usize] };
                let mut fTemp468: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp466 as usize] };
                let mut fTemp469: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp466, 1)) as usize] }
                        - fTemp468;
                let mut fTemp470: F64 = if iTemp67 != 0 {
                    fTemp468
                        + fTemp81 * fTemp469
                        + (fTemp464 - (iTemp465) as F64)
                            * (fTemp467
                                - (fTemp468
                                    + fTemp81
                                        * (fTemp469
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp466, 8)) as usize]
                                            } - fTemp467))))
                } else {
                    1.0 - (fTemp462
                        + fTemp81 * fTemp463
                        + (fTemp458 - (iTemp459) as F64)
                            * (fTemp461
                                - (fTemp462
                                    + fTemp81
                                        * (fTemp463
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp460, 8)) as usize]
                                            } - fTemp461)))))
                };
                let mut fTemp471: F64 = fTemp86 + fTemp457;
                let mut fTemp472: F64 = 524287.0 * (1.0 - fTemp471);
                let mut iTemp473: i32 = (fTemp472) as i32;
                let mut iTemp474: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp473, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp475: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp474, 7)) as usize] };
                let mut fTemp476: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp474 as usize] };
                let mut fTemp477: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp474, 1)) as usize] }
                        - fTemp476;
                let mut fTemp478: F64 = 524287.0 * fTemp471;
                let mut iTemp479: i32 = (fTemp478) as i32;
                let mut iTemp480: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp479, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp481: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp480, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp482: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp480 as usize] };
                let mut fTemp483: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp480, 1), 3670015),
                    )) as usize]
                } - fTemp482;
                let mut iTemp484: i32 = (fTemp142
                    > ((if iTemp67 != 0 {
                        fTemp482
                            + fTemp81 * fTemp483
                            + (fTemp478 - (iTemp479) as F64)
                                * (fTemp481
                                    - (fTemp482
                                        + fTemp81
                                            * (fTemp483
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp480, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp481))))
                    } else {
                        1.0 - (fTemp476
                            + fTemp81 * fTemp477
                            + (fTemp472 - (iTemp473) as F64)
                                * (fTemp475
                                    - (fTemp476
                                        + fTemp81
                                            * (fTemp477
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp474, 8)) as usize]
                                                } - fTemp475)))))
                    } - fTemp470)
                        / (1.0 - fTemp470))) as i32;
                let mut fTemp485: F64 = if iTemp484 != 0 { fTemp454 } else { fTemp457 };
                let mut fTemp486: F64 = if iTemp484 != 0 { fTemp457 } else { fTemp455 };
                let mut fTemp487: F64 = fTemp486 + fTemp485;
                let mut fTemp488: F64 = 0.5 * fTemp487;
                let mut fTemp489: F64 = 524287.0 * (1.0 - fTemp488);
                let mut iTemp490: i32 = (fTemp489) as i32;
                let mut iTemp491: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp490, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp492: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp491, 7)) as usize] };
                let mut fTemp493: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp491 as usize] };
                let mut fTemp494: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp491, 1)) as usize] }
                        - fTemp493;
                let mut fTemp495: F64 = 262143.5 * fTemp487;
                let mut iTemp496: i32 = (fTemp495) as i32;
                let mut iTemp497: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp496, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp498: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp497, 7)) as usize] };
                let mut fTemp499: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp497 as usize] };
                let mut fTemp500: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp497, 1)) as usize] }
                        - fTemp499;
                let mut fTemp501: F64 = if iTemp67 != 0 {
                    fTemp499
                        + fTemp81 * fTemp500
                        + (fTemp495 - (iTemp496) as F64)
                            * (fTemp498
                                - (fTemp499
                                    + fTemp81
                                        * (fTemp500
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp497, 8)) as usize]
                                            } - fTemp498))))
                } else {
                    1.0 - (fTemp493
                        + fTemp81 * fTemp494
                        + (fTemp489 - (iTemp490) as F64)
                            * (fTemp492
                                - (fTemp493
                                    + fTemp81
                                        * (fTemp494
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp491, 8)) as usize]
                                            } - fTemp492)))))
                };
                let mut fTemp502: F64 = fTemp86 + fTemp488;
                let mut fTemp503: F64 = 524287.0 * (1.0 - fTemp502);
                let mut iTemp504: i32 = (fTemp503) as i32;
                let mut iTemp505: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp504, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp506: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp505, 7)) as usize] };
                let mut fTemp507: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp505 as usize] };
                let mut fTemp508: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp505, 1)) as usize] }
                        - fTemp507;
                let mut fTemp509: F64 = 524287.0 * fTemp502;
                let mut iTemp510: i32 = (fTemp509) as i32;
                let mut iTemp511: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp510, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp512: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp511, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp513: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp511 as usize] };
                let mut fTemp514: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp511, 1), 3670015),
                    )) as usize]
                } - fTemp513;
                let mut iTemp515: i32 = (fTemp142
                    > ((if iTemp67 != 0 {
                        fTemp513
                            + fTemp81 * fTemp514
                            + (fTemp509 - (iTemp510) as F64)
                                * (fTemp512
                                    - (fTemp513
                                        + fTemp81
                                            * (fTemp514
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp511, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp512))))
                    } else {
                        1.0 - (fTemp507
                            + fTemp81 * fTemp508
                            + (fTemp503 - (iTemp504) as F64)
                                * (fTemp506
                                    - (fTemp507
                                        + fTemp81
                                            * (fTemp508
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp505, 8)) as usize]
                                                } - fTemp506)))))
                    } - fTemp501)
                        / (1.0 - fTemp501))) as i32;
                let mut fTemp516: F64 = if iTemp515 != 0 { fTemp485 } else { fTemp488 };
                let mut fTemp517: F64 = if iTemp515 != 0 { fTemp488 } else { fTemp486 };
                let mut fTemp518: F64 = fTemp517 + fTemp516;
                let mut fTemp519: F64 = 0.5 * fTemp518;
                let mut fTemp520: F64 = 524287.0 * (1.0 - fTemp519);
                let mut iTemp521: i32 = (fTemp520) as i32;
                let mut iTemp522: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp521, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp523: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp522, 7)) as usize] };
                let mut fTemp524: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp522 as usize] };
                let mut fTemp525: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp522, 1)) as usize] }
                        - fTemp524;
                let mut fTemp526: F64 = 262143.5 * fTemp518;
                let mut iTemp527: i32 = (fTemp526) as i32;
                let mut iTemp528: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp527, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp529: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp528, 7)) as usize] };
                let mut fTemp530: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp528 as usize] };
                let mut fTemp531: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp528, 1)) as usize] }
                        - fTemp530;
                let mut fTemp532: F64 = if iTemp67 != 0 {
                    fTemp530
                        + fTemp81 * fTemp531
                        + (fTemp526 - (iTemp527) as F64)
                            * (fTemp529
                                - (fTemp530
                                    + fTemp81
                                        * (fTemp531
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp528, 8)) as usize]
                                            } - fTemp529))))
                } else {
                    1.0 - (fTemp524
                        + fTemp81 * fTemp525
                        + (fTemp520 - (iTemp521) as F64)
                            * (fTemp523
                                - (fTemp524
                                    + fTemp81
                                        * (fTemp525
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp522, 8)) as usize]
                                            } - fTemp523)))))
                };
                let mut fTemp533: F64 = fTemp86 + fTemp519;
                let mut fTemp534: F64 = 524287.0 * (1.0 - fTemp533);
                let mut iTemp535: i32 = (fTemp534) as i32;
                let mut iTemp536: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp535, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp537: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp536, 7)) as usize] };
                let mut fTemp538: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp536 as usize] };
                let mut fTemp539: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp536, 1)) as usize] }
                        - fTemp538;
                let mut fTemp540: F64 = 524287.0 * fTemp533;
                let mut iTemp541: i32 = (fTemp540) as i32;
                let mut iTemp542: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp541, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp543: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp542, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp544: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp542 as usize] };
                let mut fTemp545: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp542, 1), 3670015),
                    )) as usize]
                } - fTemp544;
                let mut iTemp546: i32 = (fTemp142
                    > ((if iTemp67 != 0 {
                        fTemp544
                            + fTemp81 * fTemp545
                            + (fTemp540 - (iTemp541) as F64)
                                * (fTemp543
                                    - (fTemp544
                                        + fTemp81
                                            * (fTemp545
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp542, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp543))))
                    } else {
                        1.0 - (fTemp538
                            + fTemp81 * fTemp539
                            + (fTemp534 - (iTemp535) as F64)
                                * (fTemp537
                                    - (fTemp538
                                        + fTemp81
                                            * (fTemp539
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp536, 8)) as usize]
                                                } - fTemp537)))))
                    } - fTemp532)
                        / (1.0 - fTemp532))) as i32;
                let mut fTemp547: F64 = if iTemp546 != 0 { fTemp516 } else { fTemp519 };
                let mut fTemp548: F64 = if iTemp546 != 0 { fTemp519 } else { fTemp517 };
                let mut fTemp549: F64 = fTemp548 + fTemp547;
                let mut fTemp550: F64 = 0.5 * fTemp549;
                let mut fTemp551: F64 = 524287.0 * (1.0 - fTemp550);
                let mut iTemp552: i32 = (fTemp551) as i32;
                let mut iTemp553: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp552, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp554: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp553, 7)) as usize] };
                let mut fTemp555: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp553 as usize] };
                let mut fTemp556: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp553, 1)) as usize] }
                        - fTemp555;
                let mut fTemp557: F64 = 262143.5 * fTemp549;
                let mut iTemp558: i32 = (fTemp557) as i32;
                let mut iTemp559: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp558, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp560: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp559, 7)) as usize] };
                let mut fTemp561: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp559 as usize] };
                let mut fTemp562: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp559, 1)) as usize] }
                        - fTemp561;
                let mut fTemp563: F64 = if iTemp67 != 0 {
                    fTemp561
                        + fTemp81 * fTemp562
                        + (fTemp557 - (iTemp558) as F64)
                            * (fTemp560
                                - (fTemp561
                                    + fTemp81
                                        * (fTemp562
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp559, 8)) as usize]
                                            } - fTemp560))))
                } else {
                    1.0 - (fTemp555
                        + fTemp81 * fTemp556
                        + (fTemp551 - (iTemp552) as F64)
                            * (fTemp554
                                - (fTemp555
                                    + fTemp81
                                        * (fTemp556
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp553, 8)) as usize]
                                            } - fTemp554)))))
                };
                let mut fTemp564: F64 = fTemp86 + fTemp550;
                let mut fTemp565: F64 = 524287.0 * (1.0 - fTemp564);
                let mut iTemp566: i32 = (fTemp565) as i32;
                let mut iTemp567: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp566, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp568: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp567, 7)) as usize] };
                let mut fTemp569: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp567 as usize] };
                let mut fTemp570: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp567, 1)) as usize] }
                        - fTemp569;
                let mut fTemp571: F64 = 524287.0 * fTemp564;
                let mut iTemp572: i32 = (fTemp571) as i32;
                let mut iTemp573: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp572, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp574: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp573, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp575: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp573 as usize] };
                let mut fTemp576: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp573, 1), 3670015),
                    )) as usize]
                } - fTemp575;
                let mut iTemp577: i32 = (fTemp142
                    > ((if iTemp67 != 0 {
                        fTemp575
                            + fTemp81 * fTemp576
                            + (fTemp571 - (iTemp572) as F64)
                                * (fTemp574
                                    - (fTemp575
                                        + fTemp81
                                            * (fTemp576
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp573, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp574))))
                    } else {
                        1.0 - (fTemp569
                            + fTemp81 * fTemp570
                            + (fTemp565 - (iTemp566) as F64)
                                * (fTemp568
                                    - (fTemp569
                                        + fTemp81
                                            * (fTemp570
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp567, 8)) as usize]
                                                } - fTemp568)))))
                    } - fTemp563)
                        / (1.0 - fTemp563))) as i32;
                let mut fTemp578: F64 = if iTemp577 != 0 { fTemp547 } else { fTemp550 };
                let mut fTemp579: F64 = if iTemp577 != 0 { fTemp550 } else { fTemp548 };
                let mut fTemp580: F64 = fTemp579 + fTemp578;
                let mut fTemp581: F64 = 0.5 * fTemp580;
                let mut fTemp582: F64 = 524287.0 * (1.0 - fTemp581);
                let mut iTemp583: i32 = (fTemp582) as i32;
                let mut iTemp584: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp583, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp585: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp584, 7)) as usize] };
                let mut fTemp586: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp584 as usize] };
                let mut fTemp587: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp584, 1)) as usize] }
                        - fTemp586;
                let mut fTemp588: F64 = 262143.5 * fTemp580;
                let mut iTemp589: i32 = (fTemp588) as i32;
                let mut iTemp590: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp589, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp591: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp590, 7)) as usize] };
                let mut fTemp592: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp590 as usize] };
                let mut fTemp593: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp590, 1)) as usize] }
                        - fTemp592;
                let mut fTemp594: F64 = if iTemp67 != 0 {
                    fTemp592
                        + fTemp81 * fTemp593
                        + (fTemp588 - (iTemp589) as F64)
                            * (fTemp591
                                - (fTemp592
                                    + fTemp81
                                        * (fTemp593
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp590, 8)) as usize]
                                            } - fTemp591))))
                } else {
                    1.0 - (fTemp586
                        + fTemp81 * fTemp587
                        + (fTemp582 - (iTemp583) as F64)
                            * (fTemp585
                                - (fTemp586
                                    + fTemp81
                                        * (fTemp587
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp584, 8)) as usize]
                                            } - fTemp585)))))
                };
                let mut fTemp595: F64 = fTemp86 + fTemp581;
                let mut fTemp596: F64 = 524287.0 * (1.0 - fTemp595);
                let mut iTemp597: i32 = (fTemp596) as i32;
                let mut iTemp598: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp597, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp599: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp598, 7)) as usize] };
                let mut fTemp600: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp598 as usize] };
                let mut fTemp601: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp598, 1)) as usize] }
                        - fTemp600;
                let mut fTemp602: F64 = 524287.0 * fTemp595;
                let mut iTemp603: i32 = (fTemp602) as i32;
                let mut iTemp604: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp603, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp605: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp604, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp606: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp604 as usize] };
                let mut fTemp607: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp604, 1), 3670015),
                    )) as usize]
                } - fTemp606;
                let mut iTemp608: i32 = (fTemp142
                    > ((if iTemp67 != 0 {
                        fTemp606
                            + fTemp81 * fTemp607
                            + (fTemp602 - (iTemp603) as F64)
                                * (fTemp605
                                    - (fTemp606
                                        + fTemp81
                                            * (fTemp607
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp604, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp605))))
                    } else {
                        1.0 - (fTemp600
                            + fTemp81 * fTemp601
                            + (fTemp596 - (iTemp597) as F64)
                                * (fTemp599
                                    - (fTemp600
                                        + fTemp81
                                            * (fTemp601
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp598, 8)) as usize]
                                                } - fTemp599)))))
                    } - fTemp594)
                        / (1.0 - fTemp594))) as i32;
                let mut fTemp609: F64 = if iTemp608 != 0 { fTemp578 } else { fTemp581 };
                let mut fTemp610: F64 = if iTemp608 != 0 { fTemp581 } else { fTemp579 };
                let mut fTemp611: F64 = fTemp610 + fTemp609;
                let mut fTemp612: F64 = 0.5 * fTemp611;
                let mut fTemp613: F64 = 524287.0 * (1.0 - fTemp612);
                let mut iTemp614: i32 = (fTemp613) as i32;
                let mut iTemp615: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp614, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp616: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp615, 7)) as usize] };
                let mut fTemp617: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp615 as usize] };
                let mut fTemp618: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp615, 1)) as usize] }
                        - fTemp617;
                let mut fTemp619: F64 = 262143.5 * fTemp611;
                let mut iTemp620: i32 = (fTemp619) as i32;
                let mut iTemp621: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp620, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp622: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp621, 7)) as usize] };
                let mut fTemp623: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp621 as usize] };
                let mut fTemp624: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp621, 1)) as usize] }
                        - fTemp623;
                let mut fTemp625: F64 = if iTemp67 != 0 {
                    fTemp623
                        + fTemp81 * fTemp624
                        + (fTemp619 - (iTemp620) as F64)
                            * (fTemp622
                                - (fTemp623
                                    + fTemp81
                                        * (fTemp624
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp621, 8)) as usize]
                                            } - fTemp622))))
                } else {
                    1.0 - (fTemp617
                        + fTemp81 * fTemp618
                        + (fTemp613 - (iTemp614) as F64)
                            * (fTemp616
                                - (fTemp617
                                    + fTemp81
                                        * (fTemp618
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp615, 8)) as usize]
                                            } - fTemp616)))))
                };
                let mut fTemp626: F64 = fTemp86 + fTemp612;
                let mut fTemp627: F64 = 524287.0 * (1.0 - fTemp626);
                let mut iTemp628: i32 = (fTemp627) as i32;
                let mut iTemp629: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp628, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp630: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp629, 7)) as usize] };
                let mut fTemp631: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp629 as usize] };
                let mut fTemp632: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp629, 1)) as usize] }
                        - fTemp631;
                let mut fTemp633: F64 = 524287.0 * fTemp626;
                let mut iTemp634: i32 = (fTemp633) as i32;
                let mut iTemp635: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp634, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp636: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp635, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp637: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp635 as usize] };
                let mut fTemp638: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp635, 1), 3670015),
                    )) as usize]
                } - fTemp637;
                let mut iTemp639: i32 = (fTemp142
                    > ((if iTemp67 != 0 {
                        fTemp637
                            + fTemp81 * fTemp638
                            + (fTemp633 - (iTemp634) as F64)
                                * (fTemp636
                                    - (fTemp637
                                        + fTemp81
                                            * (fTemp638
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp635, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp636))))
                    } else {
                        1.0 - (fTemp631
                            + fTemp81 * fTemp632
                            + (fTemp627 - (iTemp628) as F64)
                                * (fTemp630
                                    - (fTemp631
                                        + fTemp81
                                            * (fTemp632
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp629, 8)) as usize]
                                                } - fTemp630)))))
                    } - fTemp625)
                        / (1.0 - fTemp625))) as i32;
                let mut fTemp640: F64 = if iTemp639 != 0 { fTemp609 } else { fTemp612 };
                let mut fTemp641: F64 = if iTemp639 != 0 { fTemp612 } else { fTemp610 };
                let mut fTemp642: F64 = fTemp641 + fTemp640;
                let mut fTemp643: F64 = 0.5 * fTemp642;
                let mut fTemp644: F64 = 524287.0 * (1.0 - fTemp643);
                let mut iTemp645: i32 = (fTemp644) as i32;
                let mut iTemp646: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp645, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp647: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp646, 7)) as usize] };
                let mut fTemp648: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp646 as usize] };
                let mut fTemp649: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp646, 1)) as usize] }
                        - fTemp648;
                let mut fTemp650: F64 = 262143.5 * fTemp642;
                let mut iTemp651: i32 = (fTemp650) as i32;
                let mut iTemp652: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp651, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp653: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp652, 7)) as usize] };
                let mut fTemp654: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp652 as usize] };
                let mut fTemp655: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp652, 1)) as usize] }
                        - fTemp654;
                let mut fTemp656: F64 = if iTemp67 != 0 {
                    fTemp654
                        + fTemp81 * fTemp655
                        + (fTemp650 - (iTemp651) as F64)
                            * (fTemp653
                                - (fTemp654
                                    + fTemp81
                                        * (fTemp655
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp652, 8)) as usize]
                                            } - fTemp653))))
                } else {
                    1.0 - (fTemp648
                        + fTemp81 * fTemp649
                        + (fTemp644 - (iTemp645) as F64)
                            * (fTemp647
                                - (fTemp648
                                    + fTemp81
                                        * (fTemp649
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp646, 8)) as usize]
                                            } - fTemp647)))))
                };
                let mut fTemp657: F64 = fTemp86 + fTemp643;
                let mut fTemp658: F64 = 524287.0 * (1.0 - fTemp657);
                let mut iTemp659: i32 = (fTemp658) as i32;
                let mut iTemp660: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp659, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp661: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp660, 7)) as usize] };
                let mut fTemp662: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp660 as usize] };
                let mut fTemp663: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp660, 1)) as usize] }
                        - fTemp662;
                let mut fTemp664: F64 = 524287.0 * fTemp657;
                let mut iTemp665: i32 = (fTemp664) as i32;
                let mut iTemp666: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp665, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp667: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp666, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp668: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp666 as usize] };
                let mut fTemp669: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp666, 1), 3670015),
                    )) as usize]
                } - fTemp668;
                let mut iTemp670: i32 = (fTemp142
                    > ((if iTemp67 != 0 {
                        fTemp668
                            + fTemp81 * fTemp669
                            + (fTemp664 - (iTemp665) as F64)
                                * (fTemp667
                                    - (fTemp668
                                        + fTemp81
                                            * (fTemp669
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp666, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp667))))
                    } else {
                        1.0 - (fTemp662
                            + fTemp81 * fTemp663
                            + (fTemp658 - (iTemp659) as F64)
                                * (fTemp661
                                    - (fTemp662
                                        + fTemp81
                                            * (fTemp663
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp660, 8)) as usize]
                                                } - fTemp661)))))
                    } - fTemp656)
                        / (1.0 - fTemp656))) as i32;
                let mut fTemp671: F64 = if iTemp670 != 0 { fTemp640 } else { fTemp643 };
                let mut fTemp672: F64 = if iTemp670 != 0 { fTemp643 } else { fTemp641 };
                let mut fTemp673: F64 = fTemp672 + fTemp671;
                let mut fTemp674: F64 = 0.5 * fTemp673;
                let mut fTemp675: F64 = 524287.0 * (1.0 - fTemp674);
                let mut iTemp676: i32 = (fTemp675) as i32;
                let mut iTemp677: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp676, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp678: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp677, 7)) as usize] };
                let mut fTemp679: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp677 as usize] };
                let mut fTemp680: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp677, 1)) as usize] }
                        - fTemp679;
                let mut fTemp681: F64 = 262143.5 * fTemp673;
                let mut iTemp682: i32 = (fTemp681) as i32;
                let mut iTemp683: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp682, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp684: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp683, 7)) as usize] };
                let mut fTemp685: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp683 as usize] };
                let mut fTemp686: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp683, 1)) as usize] }
                        - fTemp685;
                let mut fTemp687: F64 = if iTemp67 != 0 {
                    fTemp685
                        + fTemp81 * fTemp686
                        + (fTemp681 - (iTemp682) as F64)
                            * (fTemp684
                                - (fTemp685
                                    + fTemp81
                                        * (fTemp686
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp683, 8),
                                                        3670015,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp684))))
                } else {
                    1.0 - (fTemp679
                        + fTemp81 * fTemp680
                        + (fTemp675 - (iTemp676) as F64)
                            * (fTemp678
                                - (fTemp679
                                    + fTemp81
                                        * (fTemp680
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp677, 8),
                                                        3670015,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp678)))))
                };
                let mut fTemp688: F64 = fTemp86 + fTemp674;
                let mut fTemp689: F64 = 524287.0 * (1.0 - fTemp688);
                let mut iTemp690: i32 = (fTemp689) as i32;
                let mut iTemp691: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp690, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp692: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp691, 7)) as usize] };
                let mut fTemp693: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp691 as usize] };
                let mut fTemp694: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp691, 1)) as usize] }
                        - fTemp693;
                let mut fTemp695: F64 = 524287.0 * fTemp688;
                let mut iTemp696: i32 = (fTemp695) as i32;
                let mut iTemp697: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp696, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp698: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp697, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp699: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp697 as usize] };
                let mut fTemp700: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp697, 1), 3670015),
                    )) as usize]
                } - fTemp699;
                let mut iTemp701: i32 = (fTemp142
                    > ((if iTemp67 != 0 {
                        fTemp699
                            + fTemp81 * fTemp700
                            + (fTemp695 - (iTemp696) as F64)
                                * (fTemp698
                                    - (fTemp699
                                        + fTemp81
                                            * (fTemp700
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp697, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp698))))
                    } else {
                        1.0 - (fTemp693
                            + fTemp81 * fTemp694
                            + (fTemp689 - (iTemp690) as F64)
                                * (fTemp692
                                    - (fTemp693
                                        + fTemp81
                                            * (fTemp694
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp691, 8)) as usize]
                                                } - fTemp692)))))
                    } - fTemp687)
                        / (1.0 - fTemp687))) as i32;
                let mut fTemp702: F64 = F64::min(
                    1.0,
                    F64::max(
                        0.0,
                        0.5 * (if iTemp701 != 0 { fTemp674 } else { fTemp672 }
                            + if iTemp701 != 0 { fTemp671 } else { fTemp674 }),
                    ),
                );
                self.fRec1[0] = fTemp702;
                let mut fTemp703: F64 = 524287.0 * (1.0 - fTemp702);
                let mut iTemp704: i32 = (fTemp703) as i32;
                let mut iTemp705: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp704, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp706: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp705, 7)) as usize] };
                let mut fTemp707: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp705 as usize] };
                let mut fTemp708: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp705, 1)) as usize] }
                        - fTemp707;
                let mut fTemp709: F64 = 524287.0 * fTemp702;
                let mut iTemp710: i32 = (fTemp709) as i32;
                let mut iTemp711: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp710, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp712: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp711, 7)) as usize] };
                let mut fTemp713: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp711 as usize] };
                let mut fTemp714: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp711, 1)) as usize] }
                        - fTemp713;
                let mut fTemp715: F64 = if iTemp67 != 0 {
                    fTemp713
                        + fTemp81 * fTemp714
                        + (fTemp709 - (iTemp710) as F64)
                            * (fTemp712
                                - (fTemp713
                                    + fTemp81
                                        * (fTemp714
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp711, 8),
                                                        3670015,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp712))))
                } else {
                    1.0 - (fTemp707
                        + fTemp81 * fTemp708
                        + (fTemp703 - (iTemp704) as F64)
                            * (fTemp706
                                - (fTemp707
                                    + fTemp81
                                        * (fTemp708
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp705, 8),
                                                        3670015,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp706)))))
                };
                let mut fTemp716: F64 = fTemp86 + fTemp702;
                let mut fTemp717: F64 = 524287.0 * (1.0 - fTemp716);
                let mut iTemp718: i32 = (fTemp717) as i32;
                let mut iTemp719: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp718, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp720: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp719, 7)) as usize] };
                let mut fTemp721: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp719 as usize] };
                let mut fTemp722: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp719, 1)) as usize] }
                        - fTemp721;
                let mut fTemp723: F64 = 524287.0 * fTemp716;
                let mut iTemp724: i32 = (fTemp723) as i32;
                let mut iTemp725: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp76,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp724, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp726: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp725, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp727: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp725 as usize] };
                let mut fTemp728: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp725, 1), 3670015),
                    )) as usize]
                } - fTemp727;
                let mut fTemp729: F64 = self.fRec2[1]
                    + if ((0.001 * fTemp85) == 0.0) as i32 != 0 {
                        fTemp66
                    } else {
                        fTemp66
                            * (if iTemp67 != 0 {
                                fTemp727
                                    + fTemp81 * fTemp728
                                    + (fTemp723 - (iTemp724) as F64)
                                        * (fTemp726
                                            - (fTemp727
                                                + fTemp81
                                                    * (fTemp728
                                                        - (unsafe {
                                                            ftbl0LambRs192kSIG0[(std::cmp::max(
                                                                0,
                                                                std::cmp::min(
                                                                    i32::wrapping_add(iTemp725, 8),
                                                                    3670015,
                                                                ),
                                                            ))
                                                                as usize]
                                                        } - fTemp726))))
                            } else {
                                1.0 - (fTemp721
                                    + fTemp81 * fTemp722
                                    + (fTemp717 - (iTemp718) as F64)
                                        * (fTemp720
                                            - (fTemp721
                                                + fTemp81
                                                    * (fTemp722
                                                        - (unsafe {
                                                            ftbl0LambRs192kSIG0[(i32::wrapping_add(
                                                                iTemp719, 8,
                                                            ))
                                                                as usize]
                                                        } - fTemp720)))))
                            } - fTemp715)
                            / (1.0 - fTemp715)
                    };
                self.fRec2[0] = if iTemp84 != 0 {
                    F64::min(fTemp729, self.fRec2[1])
                } else {
                    F64::max(fTemp729, self.fRec2[1])
                };
                self.fVec35[(self.IOTA0 & 32767) as usize] = F64::powf(1e+01, 0.05 * self.fRec2[0]);
                let mut fTemp730: F64 =
                    self.fVec35[((i32::wrapping_sub(self.IOTA0, iSlow78)) & 32767) as usize];
                self.fRec14[0] = fSlow80 + self.fConst4 * self.fRec14[1];
                *output0 = 0.5
                    * self.fVec1[((i32::wrapping_sub(self.IOTA0, iSlow79)) & 32767) as usize]
                    * fTemp2
                    + self.fRec14[0]
                        * self.fVec2[((i32::wrapping_sub(self.IOTA0, iSlow79)) & 32767) as usize]
                        * fTemp730
                        * fTemp4;
                let mut fTemp731: F64 = fTemp36 + fSlow17 * (fTemp37 - fTemp36);
                let mut iTemp732: i32 =
                    ((fTemp731 > fSlow11) as i32) + ((fTemp731 > fSlow9) as i32);
                let mut fTemp733: F64 = fTemp731 - fSlow8;
                let mut fTemp734: F64 = F64::min(
                    fTemp34,
                    -(fSlow18
                        * F64::max(
                            0.0,
                            if (iTemp732 == 0) as i32 != 0 {
                                0.0
                            } else {
                                if (iTemp732 == 1) as i32 != 0 {
                                    fSlow12 * LambRs192k_faustpower2_f(fSlow7 + fTemp733)
                                } else {
                                    fTemp733
                                }
                            },
                        )),
                );
                self.fVec36[(self.IOTA0 & 16383) as usize] = fTemp734;
                let mut fTemp735: F64 = F64::min(
                    fTemp734,
                    self.fVec36[((i32::wrapping_sub(self.IOTA0, 1)) & 16383) as usize],
                );
                self.fVec37[0] = fTemp735;
                let mut fTemp736: F64 = F64::min(fTemp735, self.fVec37[2]);
                self.fVec38[0] = fTemp736;
                let mut fTemp737: F64 = F64::min(fTemp736, self.fVec38[4]);
                self.fVec39[0] = fTemp737;
                let mut fTemp738: F64 = F64::min(fTemp737, self.fVec39[8]);
                self.fVec40[(self.IOTA0 & 31) as usize] = fTemp738;
                let mut fTemp739: F64 = F64::min(
                    fTemp738,
                    self.fVec40[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize],
                );
                self.fVec41[(self.IOTA0 & 63) as usize] = fTemp739;
                let mut fTemp740: F64 = F64::min(
                    fTemp739,
                    self.fVec41[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize],
                );
                self.fVec42[(self.IOTA0 & 127) as usize] = fTemp740;
                let mut fTemp741: F64 = F64::min(
                    fTemp740,
                    self.fVec42[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize],
                );
                self.fVec43[(self.IOTA0 & 255) as usize] = fTemp741;
                let mut fTemp742: F64 = F64::min(
                    fTemp741,
                    self.fVec43[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize],
                );
                self.fVec44[(self.IOTA0 & 511) as usize] = fTemp742;
                let mut fTemp743: F64 = F64::min(
                    fTemp742,
                    self.fVec44[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize],
                );
                self.fVec45[(self.IOTA0 & 1023) as usize] = fTemp743;
                let mut fTemp744: F64 = F64::min(
                    fTemp743,
                    self.fVec45[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize],
                );
                self.fVec46[(self.IOTA0 & 2047) as usize] = fTemp744;
                let mut fTemp745: F64 = F64::min(
                    fTemp744,
                    self.fVec46[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize],
                );
                self.fVec47[(self.IOTA0 & 4095) as usize] = fTemp745;
                let mut fTemp746: F64 = F64::min(
                    fTemp745,
                    self.fVec47[((i32::wrapping_sub(self.IOTA0, 2048)) & 4095) as usize],
                );
                self.fVec48[(self.IOTA0 & 8191) as usize] = fTemp746;
                self.fVec49[(self.IOTA0 & 16383) as usize] = F64::min(
                    fTemp746,
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
                                                                        if iSlow23 != 0 {
                                                                            fTemp734
                                                                        } else {
                                                                            1.7976931348623157e+308
                                                                        },
                                                                        if iSlow24 != 0 {
                                                                            self.fVec37
                                                                                [iSlow23 as usize]
                                                                        } else {
                                                                            1.7976931348623157e+308
                                                                        },
                                                                    ),
                                                                    if iSlow25 != 0 {
                                                                        self.fVec38
                                                                            [iSlow26 as usize]
                                                                    } else {
                                                                        1.7976931348623157e+308
                                                                    },
                                                                ),
                                                                if iSlow27 != 0 {
                                                                    self.fVec39[iSlow28 as usize]
                                                                } else {
                                                                    1.7976931348623157e+308
                                                                },
                                                            ),
                                                            if iSlow29 != 0 {
                                                                self.fVec40[((i32::wrapping_sub(
                                                                    self.IOTA0, iSlow30,
                                                                )) & 31)
                                                                    as usize]
                                                            } else {
                                                                1.7976931348623157e+308
                                                            },
                                                        ),
                                                        if iSlow31 != 0 {
                                                            self.fVec41[((i32::wrapping_sub(
                                                                self.IOTA0, iSlow32,
                                                            )) & 63)
                                                                as usize]
                                                        } else {
                                                            1.7976931348623157e+308
                                                        },
                                                    ),
                                                    if iSlow33 != 0 {
                                                        self.fVec42[((i32::wrapping_sub(
                                                            self.IOTA0, iSlow34,
                                                        )) & 127)
                                                            as usize]
                                                    } else {
                                                        1.7976931348623157e+308
                                                    },
                                                ),
                                                if iSlow35 != 0 {
                                                    self.fVec43[((i32::wrapping_sub(
                                                        self.IOTA0, iSlow36,
                                                    )) & 255)
                                                        as usize]
                                                } else {
                                                    1.7976931348623157e+308
                                                },
                                            ),
                                            if iSlow37 != 0 {
                                                self.fVec44[((i32::wrapping_sub(
                                                    self.IOTA0, iSlow38,
                                                )) & 511)
                                                    as usize]
                                            } else {
                                                1.7976931348623157e+308
                                            },
                                        ),
                                        if iSlow39 != 0 {
                                            self.fVec45[((i32::wrapping_sub(self.IOTA0, iSlow40))
                                                & 1023)
                                                as usize]
                                        } else {
                                            1.7976931348623157e+308
                                        },
                                    ),
                                    if iSlow41 != 0 {
                                        self.fVec46[((i32::wrapping_sub(self.IOTA0, iSlow42))
                                            & 2047)
                                            as usize]
                                    } else {
                                        1.7976931348623157e+308
                                    },
                                ),
                                if iSlow43 != 0 {
                                    self.fVec47
                                        [((i32::wrapping_sub(self.IOTA0, iSlow44)) & 4095) as usize]
                                } else {
                                    1.7976931348623157e+308
                                },
                            ),
                            if iSlow45 != 0 {
                                self.fVec48
                                    [((i32::wrapping_sub(self.IOTA0, iSlow46)) & 8191) as usize]
                            } else {
                                1.7976931348623157e+308
                            },
                        ),
                        if iSlow47 != 0 {
                            self.fVec49[((i32::wrapping_sub(self.IOTA0, iSlow48)) & 16383) as usize]
                        } else {
                            1.7976931348623157e+308
                        },
                    ),
                );
                let mut fTemp747: F64 = F64::min(self.fRec17[0], self.fRec17[1]);
                self.fVec50[0] = fTemp747;
                let mut fTemp748: F64 = F64::min(fTemp747, self.fVec50[2]);
                self.fVec51[0] = fTemp748;
                let mut fTemp749: F64 = F64::min(fTemp748, self.fVec51[4]);
                self.fVec52[0] = fTemp749;
                let mut fTemp750: F64 = F64::min(fTemp749, self.fVec52[8]);
                self.fVec53[(self.IOTA0 & 31) as usize] = fTemp750;
                let mut fTemp751: F64 = F64::min(
                    fTemp750,
                    self.fVec53[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize],
                );
                self.fVec54[(self.IOTA0 & 63) as usize] = fTemp751;
                let mut fTemp752: F64 = F64::min(
                    fTemp751,
                    self.fVec54[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize],
                );
                self.fVec55[(self.IOTA0 & 127) as usize] = fTemp752;
                let mut fTemp753: F64 = F64::min(
                    fTemp752,
                    self.fVec55[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize],
                );
                self.fVec56[(self.IOTA0 & 255) as usize] = fTemp753;
                let mut fTemp754: F64 = F64::min(
                    fTemp753,
                    self.fVec56[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize],
                );
                self.fVec57[(self.IOTA0 & 511) as usize] = fTemp754;
                let mut fTemp755: F64 = F64::min(
                    fTemp754,
                    self.fVec57[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize],
                );
                self.fVec58[(self.IOTA0 & 1023) as usize] = fTemp755;
                let mut fTemp756: F64 = F64::min(
                    fTemp755,
                    self.fVec58[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize],
                );
                self.fVec59[(self.IOTA0 & 2047) as usize] = fTemp756;
                let mut fTemp757: F64 = F64::min(
                    fTemp756,
                    self.fVec59[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize],
                );
                self.fVec60[(self.IOTA0 & 4095) as usize] = fTemp757;
                let mut fTemp758: F64 = F64::min(
                    fTemp757,
                    self.fVec60[((i32::wrapping_sub(self.IOTA0, 2048)) & 4095) as usize],
                );
                self.fVec61[(self.IOTA0 & 8191) as usize] = fTemp758;
                self.fVec62[(self.IOTA0 & 16383) as usize] = F64::min(
                    fTemp758,
                    self.fVec61[((i32::wrapping_sub(self.IOTA0, 4096)) & 8191) as usize],
                );
                let mut fTemp759: F64 = F64::min(
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
                                                                    if iSlow4 != 0 {
                                                                        self.fRec17[0]
                                                                    } else {
                                                                        1.7976931348623157e+308
                                                                    },
                                                                    if iSlow49 != 0 {
                                                                        self.fVec50[iSlow4 as usize]
                                                                    } else {
                                                                        1.7976931348623157e+308
                                                                    },
                                                                ),
                                                                if iSlow50 != 0 {
                                                                    self.fVec51[iSlow51 as usize]
                                                                } else {
                                                                    1.7976931348623157e+308
                                                                },
                                                            ),
                                                            if iSlow52 != 0 {
                                                                self.fVec52[iSlow53 as usize]
                                                            } else {
                                                                1.7976931348623157e+308
                                                            },
                                                        ),
                                                        if iSlow54 != 0 {
                                                            self.fVec53[((i32::wrapping_sub(
                                                                self.IOTA0, iSlow55,
                                                            )) & 31)
                                                                as usize]
                                                        } else {
                                                            1.7976931348623157e+308
                                                        },
                                                    ),
                                                    if iSlow56 != 0 {
                                                        self.fVec54[((i32::wrapping_sub(
                                                            self.IOTA0, iSlow57,
                                                        )) & 63)
                                                            as usize]
                                                    } else {
                                                        1.7976931348623157e+308
                                                    },
                                                ),
                                                if iSlow58 != 0 {
                                                    self.fVec55[((i32::wrapping_sub(
                                                        self.IOTA0, iSlow59,
                                                    )) & 127)
                                                        as usize]
                                                } else {
                                                    1.7976931348623157e+308
                                                },
                                            ),
                                            if iSlow60 != 0 {
                                                self.fVec56[((i32::wrapping_sub(
                                                    self.IOTA0, iSlow61,
                                                )) & 255)
                                                    as usize]
                                            } else {
                                                1.7976931348623157e+308
                                            },
                                        ),
                                        if iSlow62 != 0 {
                                            self.fVec57[((i32::wrapping_sub(self.IOTA0, iSlow63))
                                                & 511)
                                                as usize]
                                        } else {
                                            1.7976931348623157e+308
                                        },
                                    ),
                                    if iSlow64 != 0 {
                                        self.fVec58[((i32::wrapping_sub(self.IOTA0, iSlow65))
                                            & 1023)
                                            as usize]
                                    } else {
                                        1.7976931348623157e+308
                                    },
                                ),
                                if iSlow66 != 0 {
                                    self.fVec59
                                        [((i32::wrapping_sub(self.IOTA0, iSlow67)) & 2047) as usize]
                                } else {
                                    1.7976931348623157e+308
                                },
                            ),
                            if iSlow68 != 0 {
                                self.fVec60
                                    [((i32::wrapping_sub(self.IOTA0, iSlow69)) & 4095) as usize]
                            } else {
                                1.7976931348623157e+308
                            },
                        ),
                        if iSlow70 != 0 {
                            self.fVec61[((i32::wrapping_sub(self.IOTA0, iSlow71)) & 8191) as usize]
                        } else {
                            1.7976931348623157e+308
                        },
                    ),
                    if iSlow72 != 0 {
                        self.fVec62[((i32::wrapping_sub(self.IOTA0, iSlow73)) & 16383) as usize]
                    } else {
                        1.7976931348623157e+308
                    },
                ) - self.fRec16[1];
                self.fVec63[0] = fTemp759;
                let mut iTemp760: i32 = (fTemp759 > 0.0) as i32;
                let mut fTemp761: F64 = if iTemp760 != 0 { fSlow75 } else { fSlow74 };
                self.fVec64[0] = fTemp761;
                let mut fTemp762: F64 = 6.0 * fTemp761;
                let mut iTemp763: i32 = (fTemp762) as i32;
                let mut iTemp764: i32 = std::cmp::max(0, std::cmp::min(iTemp763, 6));
                let mut iTemp765: i32 = std::cmp::max(
                    0,
                    std::cmp::min(i32::wrapping_add(iTemp764, 1835001), 3670015),
                );
                let mut fTemp766: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp765, 7)) as usize] };
                let mut fTemp767: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp765 as usize] };
                let mut fTemp768: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp765, 1)) as usize] }
                        - fTemp767;
                let mut fTemp769: F64 = fTemp762 - (iTemp763) as F64;
                let mut fTemp770: F64 = fTemp767
                    + fTemp769 * fTemp768
                    + 0.5
                        * (fTemp766
                            - (fTemp767
                                + fTemp769
                                    * (fTemp768
                                        - (unsafe {
                                            ftbl0LambRs192kSIG0
                                                [(i32::wrapping_add(iTemp765, 8)) as usize]
                                        } - fTemp766))));
                let mut fTemp771: F64 = if iTemp760 != 0 {
                    fTemp770
                } else {
                    1.0 - fTemp770
                };
                let mut iTemp772: i32 = (fTemp759 < 0.0) as i32;
                let mut fTemp773: F64 = fSlow1 * (iTemp772) as F64 + fSlow13 * (iTemp760) as F64;
                self.fVec65[0] = fTemp773;
                let mut fTemp774: F64 = self.fConst10 / fTemp773;
                let mut fTemp775: F64 = fTemp774 + 0.5;
                let mut fTemp776: F64 = 524287.0 * (1.0 - fTemp775);
                let mut iTemp777: i32 = (fTemp776) as i32;
                let mut iTemp778: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp777, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp779: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp778, 7)) as usize] };
                let mut fTemp780: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp778 as usize] };
                let mut fTemp781: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp778, 1)) as usize] }
                        - fTemp780;
                let mut fTemp782: F64 = 524287.0 * fTemp775;
                let mut iTemp783: i32 = (fTemp782) as i32;
                let mut iTemp784: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp783, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp785: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp784, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp786: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp784 as usize] };
                let mut fTemp787: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp784, 1), 3670015),
                    )) as usize]
                } - fTemp786;
                let mut fTemp788: F64 = 6.0 * self.fVec64[1];
                let mut iTemp789: i32 = (fTemp788) as i32;
                let mut iTemp790: i32 = std::cmp::max(0, std::cmp::min(iTemp789, 6));
                let mut fTemp791: F64 = 524287.0 * (1.0 - self.fRec15[1]);
                let mut iTemp792: i32 = (fTemp791) as i32;
                let mut iTemp793: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp792, 524287))),
                            iTemp790,
                        ),
                        3670015,
                    ),
                );
                let mut fTemp794: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp793, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp795: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp793 as usize] };
                let mut fTemp796: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp793, 1), 3670015),
                    )) as usize]
                } - fTemp795;
                let mut fTemp797: F64 = fTemp788 - (iTemp789) as F64;
                let mut fTemp798: F64 = 524287.0 * self.fRec15[1];
                let mut iTemp799: i32 = (fTemp798) as i32;
                let mut iTemp800: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp790,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp799, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp801: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp800, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp802: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp800 as usize] };
                let mut fTemp803: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp800, 1), 3670015),
                    )) as usize]
                } - fTemp802;
                let mut fTemp804: F64 = self.fRec15[1] + fTemp774;
                let mut fTemp805: F64 = 524287.0 * (1.0 - fTemp804);
                let mut iTemp806: i32 = (fTemp805) as i32;
                let mut iTemp807: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp806, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp808: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp807, 7)) as usize] };
                let mut fTemp809: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp807 as usize] };
                let mut fTemp810: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp807, 1)) as usize] }
                        - fTemp809;
                let mut fTemp811: F64 = 524287.0 * fTemp804;
                let mut iTemp812: i32 = (fTemp811) as i32;
                let mut iTemp813: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp812, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp814: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp813, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp815: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp813 as usize] };
                let mut fTemp816: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp813, 1), 3670015),
                    )) as usize]
                } - fTemp815;
                let mut fTemp817: F64 =
                    self.fRec15[1] + self.fConst10 * (1.0 / fTemp773 + 1.0 / self.fVec65[1]);
                let mut fTemp818: F64 = 524287.0 * (1.0 - fTemp817);
                let mut iTemp819: i32 = (fTemp818) as i32;
                let mut iTemp820: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp819, 524287))),
                            iTemp764,
                        ),
                        3670015,
                    ),
                );
                let mut fTemp821: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp820, 7)) as usize] };
                let mut fTemp822: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp820 as usize] };
                let mut fTemp823: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp820, 1)) as usize] }
                        - fTemp822;
                let mut fTemp824: F64 = 524287.0 * fTemp817;
                let mut iTemp825: i32 = (fTemp824) as i32;
                let mut iTemp826: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp825, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp827: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp826, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp828: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp826 as usize] };
                let mut fTemp829: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp826, 1), 3670015),
                    )) as usize]
                } - fTemp828;
                let mut fTemp830: F64 = (if iTemp760 != 0 {
                    fTemp828
                        + fTemp769 * fTemp829
                        + (fTemp824 - (iTemp825) as F64)
                            * (fTemp827
                                - (fTemp828
                                    + fTemp769
                                        * (fTemp829
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp826, 8),
                                                        3670015,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp827))))
                } else {
                    1.0 - (fTemp822
                        + fTemp769 * fTemp823
                        + (fTemp818 - (iTemp819) as F64)
                            * (fTemp821
                                - (fTemp822
                                    + fTemp769
                                        * (fTemp823
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp820, 8)) as usize]
                                            } - fTemp821)))))
                } - if iTemp760 != 0 {
                    fTemp815
                        + fTemp769 * fTemp816
                        + (fTemp811 - (iTemp812) as F64)
                            * (fTemp814
                                - (fTemp815
                                    + fTemp769
                                        * (fTemp816
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp813, 8),
                                                        3670015,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp814))))
                } else {
                    1.0 - (fTemp809
                        + fTemp769 * fTemp810
                        + (fTemp805 - (iTemp806) as F64)
                            * (fTemp808
                                - (fTemp809
                                    + fTemp769
                                        * (fTemp810
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp807, 8)) as usize]
                                            } - fTemp808)))))
                }) * self.fVec63[1]
                    / (fTemp759
                        * (1.0
                            - if iTemp760 != 0 {
                                fTemp802
                                    + fTemp797 * fTemp803
                                    + (fTemp798 - (iTemp799) as F64)
                                        * (fTemp801
                                            - (fTemp802
                                                + fTemp797
                                                    * (fTemp803
                                                        - (unsafe {
                                                            ftbl0LambRs192kSIG0[(std::cmp::max(
                                                                0,
                                                                std::cmp::min(
                                                                    i32::wrapping_add(iTemp800, 8),
                                                                    3670015,
                                                                ),
                                                            ))
                                                                as usize]
                                                        } - fTemp801))))
                            } else {
                                1.0 - (fTemp795
                                    + fTemp797 * fTemp796
                                    + (fTemp791 - (iTemp792) as F64)
                                        * (fTemp794
                                            - (fTemp795
                                                + fTemp797
                                                    * (fTemp796
                                                        - (unsafe {
                                                            ftbl0LambRs192kSIG0[(std::cmp::max(
                                                                0,
                                                                std::cmp::min(
                                                                    i32::wrapping_add(iTemp793, 8),
                                                                    3670015,
                                                                ),
                                                            ))
                                                                as usize]
                                                        } - fTemp794)))))
                            }));
                let mut iTemp831: i32 = (fTemp830
                    > ((if iTemp760 != 0 {
                        fTemp786
                            + fTemp769 * fTemp787
                            + (fTemp782 - (iTemp783) as F64)
                                * (fTemp785
                                    - (fTemp786
                                        + fTemp769
                                            * (fTemp787
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp784, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp785))))
                    } else {
                        1.0 - (fTemp780
                            + fTemp769 * fTemp781
                            + (fTemp776 - (iTemp777) as F64)
                                * (fTemp779
                                    - (fTemp780
                                        + fTemp769
                                            * (fTemp781
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp778, 8)) as usize]
                                                } - fTemp779)))))
                    } - fTemp771)
                        / (1.0 - fTemp771))) as i32;
                let mut fTemp832: F64 = if iTemp831 != 0 { 1.0 } else { 0.5 };
                let mut fTemp833: F64 = if iTemp831 != 0 { 0.5 } else { 0.0 };
                let mut fTemp834: F64 = fTemp833 + fTemp832;
                let mut fTemp835: F64 = 0.5 * fTemp834;
                let mut fTemp836: F64 = 524287.0 * (1.0 - fTemp835);
                let mut iTemp837: i32 = (fTemp836) as i32;
                let mut iTemp838: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp837, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp839: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp838, 7)) as usize] };
                let mut fTemp840: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp838 as usize] };
                let mut fTemp841: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp838, 1)) as usize] }
                        - fTemp840;
                let mut fTemp842: F64 = 262143.5 * fTemp834;
                let mut iTemp843: i32 = (fTemp842) as i32;
                let mut iTemp844: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp843, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp845: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp844, 7)) as usize] };
                let mut fTemp846: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp844 as usize] };
                let mut fTemp847: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp844, 1)) as usize] }
                        - fTemp846;
                let mut fTemp848: F64 = if iTemp760 != 0 {
                    fTemp846
                        + fTemp769 * fTemp847
                        + (fTemp842 - (iTemp843) as F64)
                            * (fTemp845
                                - (fTemp846
                                    + fTemp769
                                        * (fTemp847
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp844, 8)) as usize]
                                            } - fTemp845))))
                } else {
                    1.0 - (fTemp840
                        + fTemp769 * fTemp841
                        + (fTemp836 - (iTemp837) as F64)
                            * (fTemp839
                                - (fTemp840
                                    + fTemp769
                                        * (fTemp841
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp838, 8)) as usize]
                                            } - fTemp839)))))
                };
                let mut fTemp849: F64 = fTemp774 + fTemp835;
                let mut fTemp850: F64 = 524287.0 * (1.0 - fTemp849);
                let mut iTemp851: i32 = (fTemp850) as i32;
                let mut iTemp852: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp851, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp853: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp852, 7)) as usize] };
                let mut fTemp854: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp852 as usize] };
                let mut fTemp855: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp852, 1)) as usize] }
                        - fTemp854;
                let mut fTemp856: F64 = 524287.0 * fTemp849;
                let mut iTemp857: i32 = (fTemp856) as i32;
                let mut iTemp858: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp857, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp859: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp858, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp860: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp858 as usize] };
                let mut fTemp861: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp858, 1), 3670015),
                    )) as usize]
                } - fTemp860;
                let mut iTemp862: i32 = (fTemp830
                    > ((if iTemp760 != 0 {
                        fTemp860
                            + fTemp769 * fTemp861
                            + (fTemp856 - (iTemp857) as F64)
                                * (fTemp859
                                    - (fTemp860
                                        + fTemp769
                                            * (fTemp861
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp858, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp859))))
                    } else {
                        1.0 - (fTemp854
                            + fTemp769 * fTemp855
                            + (fTemp850 - (iTemp851) as F64)
                                * (fTemp853
                                    - (fTemp854
                                        + fTemp769
                                            * (fTemp855
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp852, 8)) as usize]
                                                } - fTemp853)))))
                    } - fTemp848)
                        / (1.0 - fTemp848))) as i32;
                let mut fTemp863: F64 = if iTemp862 != 0 { fTemp832 } else { fTemp835 };
                let mut fTemp864: F64 = if iTemp862 != 0 { fTemp835 } else { fTemp833 };
                let mut fTemp865: F64 = fTemp864 + fTemp863;
                let mut fTemp866: F64 = 0.5 * fTemp865;
                let mut fTemp867: F64 = 524287.0 * (1.0 - fTemp866);
                let mut iTemp868: i32 = (fTemp867) as i32;
                let mut iTemp869: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp868, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp870: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp869, 7)) as usize] };
                let mut fTemp871: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp869 as usize] };
                let mut fTemp872: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp869, 1)) as usize] }
                        - fTemp871;
                let mut fTemp873: F64 = 262143.5 * fTemp865;
                let mut iTemp874: i32 = (fTemp873) as i32;
                let mut iTemp875: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp874, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp876: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp875, 7)) as usize] };
                let mut fTemp877: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp875 as usize] };
                let mut fTemp878: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp875, 1)) as usize] }
                        - fTemp877;
                let mut fTemp879: F64 = if iTemp760 != 0 {
                    fTemp877
                        + fTemp769 * fTemp878
                        + (fTemp873 - (iTemp874) as F64)
                            * (fTemp876
                                - (fTemp877
                                    + fTemp769
                                        * (fTemp878
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp875, 8)) as usize]
                                            } - fTemp876))))
                } else {
                    1.0 - (fTemp871
                        + fTemp769 * fTemp872
                        + (fTemp867 - (iTemp868) as F64)
                            * (fTemp870
                                - (fTemp871
                                    + fTemp769
                                        * (fTemp872
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp869, 8)) as usize]
                                            } - fTemp870)))))
                };
                let mut fTemp880: F64 = fTemp774 + fTemp866;
                let mut fTemp881: F64 = 524287.0 * (1.0 - fTemp880);
                let mut iTemp882: i32 = (fTemp881) as i32;
                let mut iTemp883: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp882, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp884: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp883, 7)) as usize] };
                let mut fTemp885: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp883 as usize] };
                let mut fTemp886: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp883, 1)) as usize] }
                        - fTemp885;
                let mut fTemp887: F64 = 524287.0 * fTemp880;
                let mut iTemp888: i32 = (fTemp887) as i32;
                let mut iTemp889: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp888, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp890: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp889, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp891: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp889 as usize] };
                let mut fTemp892: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp889, 1), 3670015),
                    )) as usize]
                } - fTemp891;
                let mut iTemp893: i32 = (fTemp830
                    > ((if iTemp760 != 0 {
                        fTemp891
                            + fTemp769 * fTemp892
                            + (fTemp887 - (iTemp888) as F64)
                                * (fTemp890
                                    - (fTemp891
                                        + fTemp769
                                            * (fTemp892
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp889, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp890))))
                    } else {
                        1.0 - (fTemp885
                            + fTemp769 * fTemp886
                            + (fTemp881 - (iTemp882) as F64)
                                * (fTemp884
                                    - (fTemp885
                                        + fTemp769
                                            * (fTemp886
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp883, 8)) as usize]
                                                } - fTemp884)))))
                    } - fTemp879)
                        / (1.0 - fTemp879))) as i32;
                let mut fTemp894: F64 = if iTemp893 != 0 { fTemp863 } else { fTemp866 };
                let mut fTemp895: F64 = if iTemp893 != 0 { fTemp866 } else { fTemp864 };
                let mut fTemp896: F64 = fTemp895 + fTemp894;
                let mut fTemp897: F64 = 0.5 * fTemp896;
                let mut fTemp898: F64 = 524287.0 * (1.0 - fTemp897);
                let mut iTemp899: i32 = (fTemp898) as i32;
                let mut iTemp900: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp899, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp901: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp900, 7)) as usize] };
                let mut fTemp902: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp900 as usize] };
                let mut fTemp903: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp900, 1)) as usize] }
                        - fTemp902;
                let mut fTemp904: F64 = 262143.5 * fTemp896;
                let mut iTemp905: i32 = (fTemp904) as i32;
                let mut iTemp906: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp905, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp907: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp906, 7)) as usize] };
                let mut fTemp908: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp906 as usize] };
                let mut fTemp909: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp906, 1)) as usize] }
                        - fTemp908;
                let mut fTemp910: F64 = if iTemp760 != 0 {
                    fTemp908
                        + fTemp769 * fTemp909
                        + (fTemp904 - (iTemp905) as F64)
                            * (fTemp907
                                - (fTemp908
                                    + fTemp769
                                        * (fTemp909
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp906, 8)) as usize]
                                            } - fTemp907))))
                } else {
                    1.0 - (fTemp902
                        + fTemp769 * fTemp903
                        + (fTemp898 - (iTemp899) as F64)
                            * (fTemp901
                                - (fTemp902
                                    + fTemp769
                                        * (fTemp903
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp900, 8)) as usize]
                                            } - fTemp901)))))
                };
                let mut fTemp911: F64 = fTemp774 + fTemp897;
                let mut fTemp912: F64 = 524287.0 * (1.0 - fTemp911);
                let mut iTemp913: i32 = (fTemp912) as i32;
                let mut iTemp914: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp913, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp915: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp914, 7)) as usize] };
                let mut fTemp916: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp914 as usize] };
                let mut fTemp917: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp914, 1)) as usize] }
                        - fTemp916;
                let mut fTemp918: F64 = 524287.0 * fTemp911;
                let mut iTemp919: i32 = (fTemp918) as i32;
                let mut iTemp920: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp919, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp921: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp920, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp922: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp920 as usize] };
                let mut fTemp923: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp920, 1), 3670015),
                    )) as usize]
                } - fTemp922;
                let mut iTemp924: i32 = (fTemp830
                    > ((if iTemp760 != 0 {
                        fTemp922
                            + fTemp769 * fTemp923
                            + (fTemp918 - (iTemp919) as F64)
                                * (fTemp921
                                    - (fTemp922
                                        + fTemp769
                                            * (fTemp923
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp920, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp921))))
                    } else {
                        1.0 - (fTemp916
                            + fTemp769 * fTemp917
                            + (fTemp912 - (iTemp913) as F64)
                                * (fTemp915
                                    - (fTemp916
                                        + fTemp769
                                            * (fTemp917
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp914, 8)) as usize]
                                                } - fTemp915)))))
                    } - fTemp910)
                        / (1.0 - fTemp910))) as i32;
                let mut fTemp925: F64 = if iTemp924 != 0 { fTemp894 } else { fTemp897 };
                let mut fTemp926: F64 = if iTemp924 != 0 { fTemp897 } else { fTemp895 };
                let mut fTemp927: F64 = fTemp926 + fTemp925;
                let mut fTemp928: F64 = 0.5 * fTemp927;
                let mut fTemp929: F64 = 524287.0 * (1.0 - fTemp928);
                let mut iTemp930: i32 = (fTemp929) as i32;
                let mut iTemp931: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp930, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp932: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp931, 7)) as usize] };
                let mut fTemp933: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp931 as usize] };
                let mut fTemp934: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp931, 1)) as usize] }
                        - fTemp933;
                let mut fTemp935: F64 = 262143.5 * fTemp927;
                let mut iTemp936: i32 = (fTemp935) as i32;
                let mut iTemp937: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp936, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp938: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp937, 7)) as usize] };
                let mut fTemp939: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp937 as usize] };
                let mut fTemp940: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp937, 1)) as usize] }
                        - fTemp939;
                let mut fTemp941: F64 = if iTemp760 != 0 {
                    fTemp939
                        + fTemp769 * fTemp940
                        + (fTemp935 - (iTemp936) as F64)
                            * (fTemp938
                                - (fTemp939
                                    + fTemp769
                                        * (fTemp940
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp937, 8)) as usize]
                                            } - fTemp938))))
                } else {
                    1.0 - (fTemp933
                        + fTemp769 * fTemp934
                        + (fTemp929 - (iTemp930) as F64)
                            * (fTemp932
                                - (fTemp933
                                    + fTemp769
                                        * (fTemp934
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp931, 8)) as usize]
                                            } - fTemp932)))))
                };
                let mut fTemp942: F64 = fTemp774 + fTemp928;
                let mut fTemp943: F64 = 524287.0 * (1.0 - fTemp942);
                let mut iTemp944: i32 = (fTemp943) as i32;
                let mut iTemp945: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp944, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp946: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp945, 7)) as usize] };
                let mut fTemp947: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp945 as usize] };
                let mut fTemp948: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp945, 1)) as usize] }
                        - fTemp947;
                let mut fTemp949: F64 = 524287.0 * fTemp942;
                let mut iTemp950: i32 = (fTemp949) as i32;
                let mut iTemp951: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp950, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp952: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp951, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp953: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp951 as usize] };
                let mut fTemp954: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp951, 1), 3670015),
                    )) as usize]
                } - fTemp953;
                let mut iTemp955: i32 = (fTemp830
                    > ((if iTemp760 != 0 {
                        fTemp953
                            + fTemp769 * fTemp954
                            + (fTemp949 - (iTemp950) as F64)
                                * (fTemp952
                                    - (fTemp953
                                        + fTemp769
                                            * (fTemp954
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp951, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp952))))
                    } else {
                        1.0 - (fTemp947
                            + fTemp769 * fTemp948
                            + (fTemp943 - (iTemp944) as F64)
                                * (fTemp946
                                    - (fTemp947
                                        + fTemp769
                                            * (fTemp948
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp945, 8)) as usize]
                                                } - fTemp946)))))
                    } - fTemp941)
                        / (1.0 - fTemp941))) as i32;
                let mut fTemp956: F64 = if iTemp955 != 0 { fTemp925 } else { fTemp928 };
                let mut fTemp957: F64 = if iTemp955 != 0 { fTemp928 } else { fTemp926 };
                let mut fTemp958: F64 = fTemp957 + fTemp956;
                let mut fTemp959: F64 = 0.5 * fTemp958;
                let mut fTemp960: F64 = 524287.0 * (1.0 - fTemp959);
                let mut iTemp961: i32 = (fTemp960) as i32;
                let mut iTemp962: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp961, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp963: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp962, 7)) as usize] };
                let mut fTemp964: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp962 as usize] };
                let mut fTemp965: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp962, 1)) as usize] }
                        - fTemp964;
                let mut fTemp966: F64 = 262143.5 * fTemp958;
                let mut iTemp967: i32 = (fTemp966) as i32;
                let mut iTemp968: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp967, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp969: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp968, 7)) as usize] };
                let mut fTemp970: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp968 as usize] };
                let mut fTemp971: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp968, 1)) as usize] }
                        - fTemp970;
                let mut fTemp972: F64 = if iTemp760 != 0 {
                    fTemp970
                        + fTemp769 * fTemp971
                        + (fTemp966 - (iTemp967) as F64)
                            * (fTemp969
                                - (fTemp970
                                    + fTemp769
                                        * (fTemp971
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp968, 8)) as usize]
                                            } - fTemp969))))
                } else {
                    1.0 - (fTemp964
                        + fTemp769 * fTemp965
                        + (fTemp960 - (iTemp961) as F64)
                            * (fTemp963
                                - (fTemp964
                                    + fTemp769
                                        * (fTemp965
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp962, 8)) as usize]
                                            } - fTemp963)))))
                };
                let mut fTemp973: F64 = fTemp774 + fTemp959;
                let mut fTemp974: F64 = 524287.0 * (1.0 - fTemp973);
                let mut iTemp975: i32 = (fTemp974) as i32;
                let mut iTemp976: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp975, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp977: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp976, 7)) as usize] };
                let mut fTemp978: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp976 as usize] };
                let mut fTemp979: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp976, 1)) as usize] }
                        - fTemp978;
                let mut fTemp980: F64 = 524287.0 * fTemp973;
                let mut iTemp981: i32 = (fTemp980) as i32;
                let mut iTemp982: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp981, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp983: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp982, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp984: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp982 as usize] };
                let mut fTemp985: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp982, 1), 3670015),
                    )) as usize]
                } - fTemp984;
                let mut iTemp986: i32 = (fTemp830
                    > ((if iTemp760 != 0 {
                        fTemp984
                            + fTemp769 * fTemp985
                            + (fTemp980 - (iTemp981) as F64)
                                * (fTemp983
                                    - (fTemp984
                                        + fTemp769
                                            * (fTemp985
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp982, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp983))))
                    } else {
                        1.0 - (fTemp978
                            + fTemp769 * fTemp979
                            + (fTemp974 - (iTemp975) as F64)
                                * (fTemp977
                                    - (fTemp978
                                        + fTemp769
                                            * (fTemp979
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp976, 8)) as usize]
                                                } - fTemp977)))))
                    } - fTemp972)
                        / (1.0 - fTemp972))) as i32;
                let mut fTemp987: F64 = if iTemp986 != 0 { fTemp956 } else { fTemp959 };
                let mut fTemp988: F64 = if iTemp986 != 0 { fTemp959 } else { fTemp957 };
                let mut fTemp989: F64 = fTemp988 + fTemp987;
                let mut fTemp990: F64 = 0.5 * fTemp989;
                let mut fTemp991: F64 = 524287.0 * (1.0 - fTemp990);
                let mut iTemp992: i32 = (fTemp991) as i32;
                let mut iTemp993: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp992, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp994: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp993, 7)) as usize] };
                let mut fTemp995: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp993 as usize] };
                let mut fTemp996: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp993, 1)) as usize] }
                        - fTemp995;
                let mut fTemp997: F64 = 262143.5 * fTemp989;
                let mut iTemp998: i32 = (fTemp997) as i32;
                let mut iTemp999: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp998, 524287))),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1000: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp999, 7)) as usize] };
                let mut fTemp1001: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp999 as usize] };
                let mut fTemp1002: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp999, 1)) as usize] }
                        - fTemp1001;
                let mut fTemp1003: F64 = if iTemp760 != 0 {
                    fTemp1001
                        + fTemp769 * fTemp1002
                        + (fTemp997 - (iTemp998) as F64)
                            * (fTemp1000
                                - (fTemp1001
                                    + fTemp769
                                        * (fTemp1002
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp999, 8)) as usize]
                                            } - fTemp1000))))
                } else {
                    1.0 - (fTemp995
                        + fTemp769 * fTemp996
                        + (fTemp991 - (iTemp992) as F64)
                            * (fTemp994
                                - (fTemp995
                                    + fTemp769
                                        * (fTemp996
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp993, 8)) as usize]
                                            } - fTemp994)))))
                };
                let mut fTemp1004: F64 = fTemp774 + fTemp990;
                let mut fTemp1005: F64 = 524287.0 * (1.0 - fTemp1004);
                let mut iTemp1006: i32 = (fTemp1005) as i32;
                let mut iTemp1007: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1006, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1008: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1007, 7)) as usize] };
                let mut fTemp1009: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1007 as usize] };
                let mut fTemp1010: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1007, 1)) as usize] }
                        - fTemp1009;
                let mut fTemp1011: F64 = 524287.0 * fTemp1004;
                let mut iTemp1012: i32 = (fTemp1011) as i32;
                let mut iTemp1013: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1012, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1014: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1013, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp1015: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1013 as usize] };
                let mut fTemp1016: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1013, 1), 3670015),
                    )) as usize]
                } - fTemp1015;
                let mut iTemp1017: i32 = (fTemp830
                    > ((if iTemp760 != 0 {
                        fTemp1015
                            + fTemp769 * fTemp1016
                            + (fTemp1011 - (iTemp1012) as F64)
                                * (fTemp1014
                                    - (fTemp1015
                                        + fTemp769
                                            * (fTemp1016
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1013, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1014))))
                    } else {
                        1.0 - (fTemp1009
                            + fTemp769 * fTemp1010
                            + (fTemp1005 - (iTemp1006) as F64)
                                * (fTemp1008
                                    - (fTemp1009
                                        + fTemp769
                                            * (fTemp1010
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp1007, 8)) as usize]
                                                } - fTemp1008)))))
                    } - fTemp1003)
                        / (1.0 - fTemp1003))) as i32;
                let mut fTemp1018: F64 = if iTemp1017 != 0 { fTemp987 } else { fTemp990 };
                let mut fTemp1019: F64 = if iTemp1017 != 0 { fTemp990 } else { fTemp988 };
                let mut fTemp1020: F64 = fTemp1019 + fTemp1018;
                let mut fTemp1021: F64 = 0.5 * fTemp1020;
                let mut fTemp1022: F64 = 524287.0 * (1.0 - fTemp1021);
                let mut iTemp1023: i32 = (fTemp1022) as i32;
                let mut iTemp1024: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1023, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1025: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1024, 7)) as usize] };
                let mut fTemp1026: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1024 as usize] };
                let mut fTemp1027: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1024, 1)) as usize] }
                        - fTemp1026;
                let mut fTemp1028: F64 = 262143.5 * fTemp1020;
                let mut iTemp1029: i32 = (fTemp1028) as i32;
                let mut iTemp1030: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1029, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1031: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1030, 7)) as usize] };
                let mut fTemp1032: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1030 as usize] };
                let mut fTemp1033: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1030, 1)) as usize] }
                        - fTemp1032;
                let mut fTemp1034: F64 = if iTemp760 != 0 {
                    fTemp1032
                        + fTemp769 * fTemp1033
                        + (fTemp1028 - (iTemp1029) as F64)
                            * (fTemp1031
                                - (fTemp1032
                                    + fTemp769
                                        * (fTemp1033
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp1030, 8)) as usize]
                                            } - fTemp1031))))
                } else {
                    1.0 - (fTemp1026
                        + fTemp769 * fTemp1027
                        + (fTemp1022 - (iTemp1023) as F64)
                            * (fTemp1025
                                - (fTemp1026
                                    + fTemp769
                                        * (fTemp1027
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp1024, 8)) as usize]
                                            } - fTemp1025)))))
                };
                let mut fTemp1035: F64 = fTemp774 + fTemp1021;
                let mut fTemp1036: F64 = 524287.0 * (1.0 - fTemp1035);
                let mut iTemp1037: i32 = (fTemp1036) as i32;
                let mut iTemp1038: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1037, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1039: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1038, 7)) as usize] };
                let mut fTemp1040: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1038 as usize] };
                let mut fTemp1041: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1038, 1)) as usize] }
                        - fTemp1040;
                let mut fTemp1042: F64 = 524287.0 * fTemp1035;
                let mut iTemp1043: i32 = (fTemp1042) as i32;
                let mut iTemp1044: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1043, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1045: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1044, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp1046: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1044 as usize] };
                let mut fTemp1047: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1044, 1), 3670015),
                    )) as usize]
                } - fTemp1046;
                let mut iTemp1048: i32 = (fTemp830
                    > ((if iTemp760 != 0 {
                        fTemp1046
                            + fTemp769 * fTemp1047
                            + (fTemp1042 - (iTemp1043) as F64)
                                * (fTemp1045
                                    - (fTemp1046
                                        + fTemp769
                                            * (fTemp1047
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1044, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1045))))
                    } else {
                        1.0 - (fTemp1040
                            + fTemp769 * fTemp1041
                            + (fTemp1036 - (iTemp1037) as F64)
                                * (fTemp1039
                                    - (fTemp1040
                                        + fTemp769
                                            * (fTemp1041
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp1038, 8)) as usize]
                                                } - fTemp1039)))))
                    } - fTemp1034)
                        / (1.0 - fTemp1034))) as i32;
                let mut fTemp1049: F64 = if iTemp1048 != 0 { fTemp1018 } else { fTemp1021 };
                let mut fTemp1050: F64 = if iTemp1048 != 0 { fTemp1021 } else { fTemp1019 };
                let mut fTemp1051: F64 = fTemp1050 + fTemp1049;
                let mut fTemp1052: F64 = 0.5 * fTemp1051;
                let mut fTemp1053: F64 = 524287.0 * (1.0 - fTemp1052);
                let mut iTemp1054: i32 = (fTemp1053) as i32;
                let mut iTemp1055: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1054, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1056: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1055, 7)) as usize] };
                let mut fTemp1057: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1055 as usize] };
                let mut fTemp1058: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1055, 1)) as usize] }
                        - fTemp1057;
                let mut fTemp1059: F64 = 262143.5 * fTemp1051;
                let mut iTemp1060: i32 = (fTemp1059) as i32;
                let mut iTemp1061: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1060, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1062: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1061, 7)) as usize] };
                let mut fTemp1063: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1061 as usize] };
                let mut fTemp1064: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1061, 1)) as usize] }
                        - fTemp1063;
                let mut fTemp1065: F64 = if iTemp760 != 0 {
                    fTemp1063
                        + fTemp769 * fTemp1064
                        + (fTemp1059 - (iTemp1060) as F64)
                            * (fTemp1062
                                - (fTemp1063
                                    + fTemp769
                                        * (fTemp1064
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp1061, 8)) as usize]
                                            } - fTemp1062))))
                } else {
                    1.0 - (fTemp1057
                        + fTemp769 * fTemp1058
                        + (fTemp1053 - (iTemp1054) as F64)
                            * (fTemp1056
                                - (fTemp1057
                                    + fTemp769
                                        * (fTemp1058
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp1055, 8)) as usize]
                                            } - fTemp1056)))))
                };
                let mut fTemp1066: F64 = fTemp774 + fTemp1052;
                let mut fTemp1067: F64 = 524287.0 * (1.0 - fTemp1066);
                let mut iTemp1068: i32 = (fTemp1067) as i32;
                let mut iTemp1069: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1068, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1070: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1069, 7)) as usize] };
                let mut fTemp1071: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1069 as usize] };
                let mut fTemp1072: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1069, 1)) as usize] }
                        - fTemp1071;
                let mut fTemp1073: F64 = 524287.0 * fTemp1066;
                let mut iTemp1074: i32 = (fTemp1073) as i32;
                let mut iTemp1075: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1074, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1076: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1075, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp1077: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1075 as usize] };
                let mut fTemp1078: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1075, 1), 3670015),
                    )) as usize]
                } - fTemp1077;
                let mut iTemp1079: i32 = (fTemp830
                    > ((if iTemp760 != 0 {
                        fTemp1077
                            + fTemp769 * fTemp1078
                            + (fTemp1073 - (iTemp1074) as F64)
                                * (fTemp1076
                                    - (fTemp1077
                                        + fTemp769
                                            * (fTemp1078
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1075, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1076))))
                    } else {
                        1.0 - (fTemp1071
                            + fTemp769 * fTemp1072
                            + (fTemp1067 - (iTemp1068) as F64)
                                * (fTemp1070
                                    - (fTemp1071
                                        + fTemp769
                                            * (fTemp1072
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp1069, 8)) as usize]
                                                } - fTemp1070)))))
                    } - fTemp1065)
                        / (1.0 - fTemp1065))) as i32;
                let mut fTemp1080: F64 = if iTemp1079 != 0 { fTemp1049 } else { fTemp1052 };
                let mut fTemp1081: F64 = if iTemp1079 != 0 { fTemp1052 } else { fTemp1050 };
                let mut fTemp1082: F64 = fTemp1081 + fTemp1080;
                let mut fTemp1083: F64 = 0.5 * fTemp1082;
                let mut fTemp1084: F64 = 524287.0 * (1.0 - fTemp1083);
                let mut iTemp1085: i32 = (fTemp1084) as i32;
                let mut iTemp1086: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1085, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1087: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1086, 7)) as usize] };
                let mut fTemp1088: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1086 as usize] };
                let mut fTemp1089: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1086, 1)) as usize] }
                        - fTemp1088;
                let mut fTemp1090: F64 = 262143.5 * fTemp1082;
                let mut iTemp1091: i32 = (fTemp1090) as i32;
                let mut iTemp1092: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1091, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1093: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1092, 7)) as usize] };
                let mut fTemp1094: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1092 as usize] };
                let mut fTemp1095: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1092, 1)) as usize] }
                        - fTemp1094;
                let mut fTemp1096: F64 = if iTemp760 != 0 {
                    fTemp1094
                        + fTemp769 * fTemp1095
                        + (fTemp1090 - (iTemp1091) as F64)
                            * (fTemp1093
                                - (fTemp1094
                                    + fTemp769
                                        * (fTemp1095
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp1092, 8)) as usize]
                                            } - fTemp1093))))
                } else {
                    1.0 - (fTemp1088
                        + fTemp769 * fTemp1089
                        + (fTemp1084 - (iTemp1085) as F64)
                            * (fTemp1087
                                - (fTemp1088
                                    + fTemp769
                                        * (fTemp1089
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp1086, 8)) as usize]
                                            } - fTemp1087)))))
                };
                let mut fTemp1097: F64 = fTemp774 + fTemp1083;
                let mut fTemp1098: F64 = 524287.0 * (1.0 - fTemp1097);
                let mut iTemp1099: i32 = (fTemp1098) as i32;
                let mut iTemp1100: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1099, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1101: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1100, 7)) as usize] };
                let mut fTemp1102: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1100 as usize] };
                let mut fTemp1103: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1100, 1)) as usize] }
                        - fTemp1102;
                let mut fTemp1104: F64 = 524287.0 * fTemp1097;
                let mut iTemp1105: i32 = (fTemp1104) as i32;
                let mut iTemp1106: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1105, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1107: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1106, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp1108: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1106 as usize] };
                let mut fTemp1109: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1106, 1), 3670015),
                    )) as usize]
                } - fTemp1108;
                let mut iTemp1110: i32 = (fTemp830
                    > ((if iTemp760 != 0 {
                        fTemp1108
                            + fTemp769 * fTemp1109
                            + (fTemp1104 - (iTemp1105) as F64)
                                * (fTemp1107
                                    - (fTemp1108
                                        + fTemp769
                                            * (fTemp1109
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1106, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1107))))
                    } else {
                        1.0 - (fTemp1102
                            + fTemp769 * fTemp1103
                            + (fTemp1098 - (iTemp1099) as F64)
                                * (fTemp1101
                                    - (fTemp1102
                                        + fTemp769
                                            * (fTemp1103
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp1100, 8)) as usize]
                                                } - fTemp1101)))))
                    } - fTemp1096)
                        / (1.0 - fTemp1096))) as i32;
                let mut fTemp1111: F64 = if iTemp1110 != 0 { fTemp1080 } else { fTemp1083 };
                let mut fTemp1112: F64 = if iTemp1110 != 0 { fTemp1083 } else { fTemp1081 };
                let mut fTemp1113: F64 = fTemp1112 + fTemp1111;
                let mut fTemp1114: F64 = 0.5 * fTemp1113;
                let mut fTemp1115: F64 = 524287.0 * (1.0 - fTemp1114);
                let mut iTemp1116: i32 = (fTemp1115) as i32;
                let mut iTemp1117: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1116, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1118: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1117, 7)) as usize] };
                let mut fTemp1119: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1117 as usize] };
                let mut fTemp1120: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1117, 1)) as usize] }
                        - fTemp1119;
                let mut fTemp1121: F64 = 262143.5 * fTemp1113;
                let mut iTemp1122: i32 = (fTemp1121) as i32;
                let mut iTemp1123: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1122, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1124: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1123, 7)) as usize] };
                let mut fTemp1125: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1123 as usize] };
                let mut fTemp1126: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1123, 1)) as usize] }
                        - fTemp1125;
                let mut fTemp1127: F64 = if iTemp760 != 0 {
                    fTemp1125
                        + fTemp769 * fTemp1126
                        + (fTemp1121 - (iTemp1122) as F64)
                            * (fTemp1124
                                - (fTemp1125
                                    + fTemp769
                                        * (fTemp1126
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp1123, 8)) as usize]
                                            } - fTemp1124))))
                } else {
                    1.0 - (fTemp1119
                        + fTemp769 * fTemp1120
                        + (fTemp1115 - (iTemp1116) as F64)
                            * (fTemp1118
                                - (fTemp1119
                                    + fTemp769
                                        * (fTemp1120
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp1117, 8)) as usize]
                                            } - fTemp1118)))))
                };
                let mut fTemp1128: F64 = fTemp774 + fTemp1114;
                let mut fTemp1129: F64 = 524287.0 * (1.0 - fTemp1128);
                let mut iTemp1130: i32 = (fTemp1129) as i32;
                let mut iTemp1131: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1130, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1132: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1131, 7)) as usize] };
                let mut fTemp1133: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1131 as usize] };
                let mut fTemp1134: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1131, 1)) as usize] }
                        - fTemp1133;
                let mut fTemp1135: F64 = 524287.0 * fTemp1128;
                let mut iTemp1136: i32 = (fTemp1135) as i32;
                let mut iTemp1137: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1136, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1138: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1137, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp1139: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1137 as usize] };
                let mut fTemp1140: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1137, 1), 3670015),
                    )) as usize]
                } - fTemp1139;
                let mut iTemp1141: i32 = (fTemp830
                    > ((if iTemp760 != 0 {
                        fTemp1139
                            + fTemp769 * fTemp1140
                            + (fTemp1135 - (iTemp1136) as F64)
                                * (fTemp1138
                                    - (fTemp1139
                                        + fTemp769
                                            * (fTemp1140
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1137, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1138))))
                    } else {
                        1.0 - (fTemp1133
                            + fTemp769 * fTemp1134
                            + (fTemp1129 - (iTemp1130) as F64)
                                * (fTemp1132
                                    - (fTemp1133
                                        + fTemp769
                                            * (fTemp1134
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp1131, 8)) as usize]
                                                } - fTemp1132)))))
                    } - fTemp1127)
                        / (1.0 - fTemp1127))) as i32;
                let mut fTemp1142: F64 = if iTemp1141 != 0 { fTemp1111 } else { fTemp1114 };
                let mut fTemp1143: F64 = if iTemp1141 != 0 { fTemp1114 } else { fTemp1112 };
                let mut fTemp1144: F64 = fTemp1143 + fTemp1142;
                let mut fTemp1145: F64 = 0.5 * fTemp1144;
                let mut fTemp1146: F64 = 524287.0 * (1.0 - fTemp1145);
                let mut iTemp1147: i32 = (fTemp1146) as i32;
                let mut iTemp1148: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1147, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1149: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1148, 7)) as usize] };
                let mut fTemp1150: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1148 as usize] };
                let mut fTemp1151: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1148, 1)) as usize] }
                        - fTemp1150;
                let mut fTemp1152: F64 = 262143.5 * fTemp1144;
                let mut iTemp1153: i32 = (fTemp1152) as i32;
                let mut iTemp1154: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1153, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1155: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1154, 7)) as usize] };
                let mut fTemp1156: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1154 as usize] };
                let mut fTemp1157: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1154, 1)) as usize] }
                        - fTemp1156;
                let mut fTemp1158: F64 = if iTemp760 != 0 {
                    fTemp1156
                        + fTemp769 * fTemp1157
                        + (fTemp1152 - (iTemp1153) as F64)
                            * (fTemp1155
                                - (fTemp1156
                                    + fTemp769
                                        * (fTemp1157
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp1154, 8)) as usize]
                                            } - fTemp1155))))
                } else {
                    1.0 - (fTemp1150
                        + fTemp769 * fTemp1151
                        + (fTemp1146 - (iTemp1147) as F64)
                            * (fTemp1149
                                - (fTemp1150
                                    + fTemp769
                                        * (fTemp1151
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp1148, 8)) as usize]
                                            } - fTemp1149)))))
                };
                let mut fTemp1159: F64 = fTemp774 + fTemp1145;
                let mut fTemp1160: F64 = 524287.0 * (1.0 - fTemp1159);
                let mut iTemp1161: i32 = (fTemp1160) as i32;
                let mut iTemp1162: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1161, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1163: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1162, 7)) as usize] };
                let mut fTemp1164: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1162 as usize] };
                let mut fTemp1165: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1162, 1)) as usize] }
                        - fTemp1164;
                let mut fTemp1166: F64 = 524287.0 * fTemp1159;
                let mut iTemp1167: i32 = (fTemp1166) as i32;
                let mut iTemp1168: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1167, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1169: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1168, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp1170: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1168 as usize] };
                let mut fTemp1171: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1168, 1), 3670015),
                    )) as usize]
                } - fTemp1170;
                let mut iTemp1172: i32 = (fTemp830
                    > ((if iTemp760 != 0 {
                        fTemp1170
                            + fTemp769 * fTemp1171
                            + (fTemp1166 - (iTemp1167) as F64)
                                * (fTemp1169
                                    - (fTemp1170
                                        + fTemp769
                                            * (fTemp1171
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1168, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1169))))
                    } else {
                        1.0 - (fTemp1164
                            + fTemp769 * fTemp1165
                            + (fTemp1160 - (iTemp1161) as F64)
                                * (fTemp1163
                                    - (fTemp1164
                                        + fTemp769
                                            * (fTemp1165
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp1162, 8)) as usize]
                                                } - fTemp1163)))))
                    } - fTemp1158)
                        / (1.0 - fTemp1158))) as i32;
                let mut fTemp1173: F64 = if iTemp1172 != 0 { fTemp1142 } else { fTemp1145 };
                let mut fTemp1174: F64 = if iTemp1172 != 0 { fTemp1145 } else { fTemp1143 };
                let mut fTemp1175: F64 = fTemp1174 + fTemp1173;
                let mut fTemp1176: F64 = 0.5 * fTemp1175;
                let mut fTemp1177: F64 = 524287.0 * (1.0 - fTemp1176);
                let mut iTemp1178: i32 = (fTemp1177) as i32;
                let mut iTemp1179: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1178, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1180: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1179, 7)) as usize] };
                let mut fTemp1181: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1179 as usize] };
                let mut fTemp1182: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1179, 1)) as usize] }
                        - fTemp1181;
                let mut fTemp1183: F64 = 262143.5 * fTemp1175;
                let mut iTemp1184: i32 = (fTemp1183) as i32;
                let mut iTemp1185: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1184, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1186: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1185, 7)) as usize] };
                let mut fTemp1187: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1185 as usize] };
                let mut fTemp1188: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1185, 1)) as usize] }
                        - fTemp1187;
                let mut fTemp1189: F64 = if iTemp760 != 0 {
                    fTemp1187
                        + fTemp769 * fTemp1188
                        + (fTemp1183 - (iTemp1184) as F64)
                            * (fTemp1186
                                - (fTemp1187
                                    + fTemp769
                                        * (fTemp1188
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp1185, 8)) as usize]
                                            } - fTemp1186))))
                } else {
                    1.0 - (fTemp1181
                        + fTemp769 * fTemp1182
                        + (fTemp1177 - (iTemp1178) as F64)
                            * (fTemp1180
                                - (fTemp1181
                                    + fTemp769
                                        * (fTemp1182
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp1179, 8)) as usize]
                                            } - fTemp1180)))))
                };
                let mut fTemp1190: F64 = fTemp774 + fTemp1176;
                let mut fTemp1191: F64 = 524287.0 * (1.0 - fTemp1190);
                let mut iTemp1192: i32 = (fTemp1191) as i32;
                let mut iTemp1193: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1192, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1194: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1193, 7)) as usize] };
                let mut fTemp1195: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1193 as usize] };
                let mut fTemp1196: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1193, 1)) as usize] }
                        - fTemp1195;
                let mut fTemp1197: F64 = 524287.0 * fTemp1190;
                let mut iTemp1198: i32 = (fTemp1197) as i32;
                let mut iTemp1199: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1198, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1200: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1199, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp1201: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1199 as usize] };
                let mut fTemp1202: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1199, 1), 3670015),
                    )) as usize]
                } - fTemp1201;
                let mut iTemp1203: i32 = (fTemp830
                    > ((if iTemp760 != 0 {
                        fTemp1201
                            + fTemp769 * fTemp1202
                            + (fTemp1197 - (iTemp1198) as F64)
                                * (fTemp1200
                                    - (fTemp1201
                                        + fTemp769
                                            * (fTemp1202
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1199, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1200))))
                    } else {
                        1.0 - (fTemp1195
                            + fTemp769 * fTemp1196
                            + (fTemp1191 - (iTemp1192) as F64)
                                * (fTemp1194
                                    - (fTemp1195
                                        + fTemp769
                                            * (fTemp1196
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp1193, 8)) as usize]
                                                } - fTemp1194)))))
                    } - fTemp1189)
                        / (1.0 - fTemp1189))) as i32;
                let mut fTemp1204: F64 = if iTemp1203 != 0 { fTemp1173 } else { fTemp1176 };
                let mut fTemp1205: F64 = if iTemp1203 != 0 { fTemp1176 } else { fTemp1174 };
                let mut fTemp1206: F64 = fTemp1205 + fTemp1204;
                let mut fTemp1207: F64 = 0.5 * fTemp1206;
                let mut fTemp1208: F64 = 524287.0 * (1.0 - fTemp1207);
                let mut iTemp1209: i32 = (fTemp1208) as i32;
                let mut iTemp1210: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1209, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1211: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1210, 7)) as usize] };
                let mut fTemp1212: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1210 as usize] };
                let mut fTemp1213: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1210, 1)) as usize] }
                        - fTemp1212;
                let mut fTemp1214: F64 = 262143.5 * fTemp1206;
                let mut iTemp1215: i32 = (fTemp1214) as i32;
                let mut iTemp1216: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1215, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1217: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1216, 7)) as usize] };
                let mut fTemp1218: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1216 as usize] };
                let mut fTemp1219: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1216, 1)) as usize] }
                        - fTemp1218;
                let mut fTemp1220: F64 = if iTemp760 != 0 {
                    fTemp1218
                        + fTemp769 * fTemp1219
                        + (fTemp1214 - (iTemp1215) as F64)
                            * (fTemp1217
                                - (fTemp1218
                                    + fTemp769
                                        * (fTemp1219
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp1216, 8)) as usize]
                                            } - fTemp1217))))
                } else {
                    1.0 - (fTemp1212
                        + fTemp769 * fTemp1213
                        + (fTemp1208 - (iTemp1209) as F64)
                            * (fTemp1211
                                - (fTemp1212
                                    + fTemp769
                                        * (fTemp1213
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp1210, 8)) as usize]
                                            } - fTemp1211)))))
                };
                let mut fTemp1221: F64 = fTemp774 + fTemp1207;
                let mut fTemp1222: F64 = 524287.0 * (1.0 - fTemp1221);
                let mut iTemp1223: i32 = (fTemp1222) as i32;
                let mut iTemp1224: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1223, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1225: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1224, 7)) as usize] };
                let mut fTemp1226: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1224 as usize] };
                let mut fTemp1227: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1224, 1)) as usize] }
                        - fTemp1226;
                let mut fTemp1228: F64 = 524287.0 * fTemp1221;
                let mut iTemp1229: i32 = (fTemp1228) as i32;
                let mut iTemp1230: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1229, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1231: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1230, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp1232: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1230 as usize] };
                let mut fTemp1233: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1230, 1), 3670015),
                    )) as usize]
                } - fTemp1232;
                let mut iTemp1234: i32 = (fTemp830
                    > ((if iTemp760 != 0 {
                        fTemp1232
                            + fTemp769 * fTemp1233
                            + (fTemp1228 - (iTemp1229) as F64)
                                * (fTemp1231
                                    - (fTemp1232
                                        + fTemp769
                                            * (fTemp1233
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1230, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1231))))
                    } else {
                        1.0 - (fTemp1226
                            + fTemp769 * fTemp1227
                            + (fTemp1222 - (iTemp1223) as F64)
                                * (fTemp1225
                                    - (fTemp1226
                                        + fTemp769
                                            * (fTemp1227
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp1224, 8)) as usize]
                                                } - fTemp1225)))))
                    } - fTemp1220)
                        / (1.0 - fTemp1220))) as i32;
                let mut fTemp1235: F64 = if iTemp1234 != 0 { fTemp1204 } else { fTemp1207 };
                let mut fTemp1236: F64 = if iTemp1234 != 0 { fTemp1207 } else { fTemp1205 };
                let mut fTemp1237: F64 = fTemp1236 + fTemp1235;
                let mut fTemp1238: F64 = 0.5 * fTemp1237;
                let mut fTemp1239: F64 = 524287.0 * (1.0 - fTemp1238);
                let mut iTemp1240: i32 = (fTemp1239) as i32;
                let mut iTemp1241: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1240, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1242: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1241, 7)) as usize] };
                let mut fTemp1243: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1241 as usize] };
                let mut fTemp1244: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1241, 1)) as usize] }
                        - fTemp1243;
                let mut fTemp1245: F64 = 262143.5 * fTemp1237;
                let mut iTemp1246: i32 = (fTemp1245) as i32;
                let mut iTemp1247: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1246, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1248: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1247, 7)) as usize] };
                let mut fTemp1249: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1247 as usize] };
                let mut fTemp1250: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1247, 1)) as usize] }
                        - fTemp1249;
                let mut fTemp1251: F64 = if iTemp760 != 0 {
                    fTemp1249
                        + fTemp769 * fTemp1250
                        + (fTemp1245 - (iTemp1246) as F64)
                            * (fTemp1248
                                - (fTemp1249
                                    + fTemp769
                                        * (fTemp1250
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp1247, 8)) as usize]
                                            } - fTemp1248))))
                } else {
                    1.0 - (fTemp1243
                        + fTemp769 * fTemp1244
                        + (fTemp1239 - (iTemp1240) as F64)
                            * (fTemp1242
                                - (fTemp1243
                                    + fTemp769
                                        * (fTemp1244
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp1241, 8)) as usize]
                                            } - fTemp1242)))))
                };
                let mut fTemp1252: F64 = fTemp774 + fTemp1238;
                let mut fTemp1253: F64 = 524287.0 * (1.0 - fTemp1252);
                let mut iTemp1254: i32 = (fTemp1253) as i32;
                let mut iTemp1255: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1254, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1256: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1255, 7)) as usize] };
                let mut fTemp1257: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1255 as usize] };
                let mut fTemp1258: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1255, 1)) as usize] }
                        - fTemp1257;
                let mut fTemp1259: F64 = 524287.0 * fTemp1252;
                let mut iTemp1260: i32 = (fTemp1259) as i32;
                let mut iTemp1261: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1260, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1262: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1261, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp1263: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1261 as usize] };
                let mut fTemp1264: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1261, 1), 3670015),
                    )) as usize]
                } - fTemp1263;
                let mut iTemp1265: i32 = (fTemp830
                    > ((if iTemp760 != 0 {
                        fTemp1263
                            + fTemp769 * fTemp1264
                            + (fTemp1259 - (iTemp1260) as F64)
                                * (fTemp1262
                                    - (fTemp1263
                                        + fTemp769
                                            * (fTemp1264
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1261, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1262))))
                    } else {
                        1.0 - (fTemp1257
                            + fTemp769 * fTemp1258
                            + (fTemp1253 - (iTemp1254) as F64)
                                * (fTemp1256
                                    - (fTemp1257
                                        + fTemp769
                                            * (fTemp1258
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp1255, 8)) as usize]
                                                } - fTemp1256)))))
                    } - fTemp1251)
                        / (1.0 - fTemp1251))) as i32;
                let mut fTemp1266: F64 = if iTemp1265 != 0 { fTemp1235 } else { fTemp1238 };
                let mut fTemp1267: F64 = if iTemp1265 != 0 { fTemp1238 } else { fTemp1236 };
                let mut fTemp1268: F64 = fTemp1267 + fTemp1266;
                let mut fTemp1269: F64 = 0.5 * fTemp1268;
                let mut fTemp1270: F64 = 524287.0 * (1.0 - fTemp1269);
                let mut iTemp1271: i32 = (fTemp1270) as i32;
                let mut iTemp1272: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1271, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1273: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1272, 7)) as usize] };
                let mut fTemp1274: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1272 as usize] };
                let mut fTemp1275: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1272, 1)) as usize] }
                        - fTemp1274;
                let mut fTemp1276: F64 = 262143.5 * fTemp1268;
                let mut iTemp1277: i32 = (fTemp1276) as i32;
                let mut iTemp1278: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1277, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1279: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1278, 7)) as usize] };
                let mut fTemp1280: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1278 as usize] };
                let mut fTemp1281: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1278, 1)) as usize] }
                        - fTemp1280;
                let mut fTemp1282: F64 = if iTemp760 != 0 {
                    fTemp1280
                        + fTemp769 * fTemp1281
                        + (fTemp1276 - (iTemp1277) as F64)
                            * (fTemp1279
                                - (fTemp1280
                                    + fTemp769
                                        * (fTemp1281
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp1278, 8)) as usize]
                                            } - fTemp1279))))
                } else {
                    1.0 - (fTemp1274
                        + fTemp769 * fTemp1275
                        + (fTemp1270 - (iTemp1271) as F64)
                            * (fTemp1273
                                - (fTemp1274
                                    + fTemp769
                                        * (fTemp1275
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp1272, 8)) as usize]
                                            } - fTemp1273)))))
                };
                let mut fTemp1283: F64 = fTemp774 + fTemp1269;
                let mut fTemp1284: F64 = 524287.0 * (1.0 - fTemp1283);
                let mut iTemp1285: i32 = (fTemp1284) as i32;
                let mut iTemp1286: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1285, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1287: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1286, 7)) as usize] };
                let mut fTemp1288: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1286 as usize] };
                let mut fTemp1289: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1286, 1)) as usize] }
                        - fTemp1288;
                let mut fTemp1290: F64 = 524287.0 * fTemp1283;
                let mut iTemp1291: i32 = (fTemp1290) as i32;
                let mut iTemp1292: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1291, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1293: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1292, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp1294: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1292 as usize] };
                let mut fTemp1295: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1292, 1), 3670015),
                    )) as usize]
                } - fTemp1294;
                let mut iTemp1296: i32 = (fTemp830
                    > ((if iTemp760 != 0 {
                        fTemp1294
                            + fTemp769 * fTemp1295
                            + (fTemp1290 - (iTemp1291) as F64)
                                * (fTemp1293
                                    - (fTemp1294
                                        + fTemp769
                                            * (fTemp1295
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1292, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1293))))
                    } else {
                        1.0 - (fTemp1288
                            + fTemp769 * fTemp1289
                            + (fTemp1284 - (iTemp1285) as F64)
                                * (fTemp1287
                                    - (fTemp1288
                                        + fTemp769
                                            * (fTemp1289
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp1286, 8)) as usize]
                                                } - fTemp1287)))))
                    } - fTemp1282)
                        / (1.0 - fTemp1282))) as i32;
                let mut fTemp1297: F64 = if iTemp1296 != 0 { fTemp1266 } else { fTemp1269 };
                let mut fTemp1298: F64 = if iTemp1296 != 0 { fTemp1269 } else { fTemp1267 };
                let mut fTemp1299: F64 = fTemp1298 + fTemp1297;
                let mut fTemp1300: F64 = 0.5 * fTemp1299;
                let mut fTemp1301: F64 = 524287.0 * (1.0 - fTemp1300);
                let mut iTemp1302: i32 = (fTemp1301) as i32;
                let mut iTemp1303: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1302, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1304: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1303, 7)) as usize] };
                let mut fTemp1305: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1303 as usize] };
                let mut fTemp1306: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1303, 1)) as usize] }
                        - fTemp1305;
                let mut fTemp1307: F64 = 262143.5 * fTemp1299;
                let mut iTemp1308: i32 = (fTemp1307) as i32;
                let mut iTemp1309: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1308, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1310: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1309, 7)) as usize] };
                let mut fTemp1311: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1309 as usize] };
                let mut fTemp1312: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1309, 1)) as usize] }
                        - fTemp1311;
                let mut fTemp1313: F64 = if iTemp760 != 0 {
                    fTemp1311
                        + fTemp769 * fTemp1312
                        + (fTemp1307 - (iTemp1308) as F64)
                            * (fTemp1310
                                - (fTemp1311
                                    + fTemp769
                                        * (fTemp1312
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp1309, 8)) as usize]
                                            } - fTemp1310))))
                } else {
                    1.0 - (fTemp1305
                        + fTemp769 * fTemp1306
                        + (fTemp1301 - (iTemp1302) as F64)
                            * (fTemp1304
                                - (fTemp1305
                                    + fTemp769
                                        * (fTemp1306
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp1303, 8)) as usize]
                                            } - fTemp1304)))))
                };
                let mut fTemp1314: F64 = fTemp774 + fTemp1300;
                let mut fTemp1315: F64 = 524287.0 * (1.0 - fTemp1314);
                let mut iTemp1316: i32 = (fTemp1315) as i32;
                let mut iTemp1317: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1316, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1318: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1317, 7)) as usize] };
                let mut fTemp1319: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1317 as usize] };
                let mut fTemp1320: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1317, 1)) as usize] }
                        - fTemp1319;
                let mut fTemp1321: F64 = 524287.0 * fTemp1314;
                let mut iTemp1322: i32 = (fTemp1321) as i32;
                let mut iTemp1323: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1322, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1324: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1323, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp1325: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1323 as usize] };
                let mut fTemp1326: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1323, 1), 3670015),
                    )) as usize]
                } - fTemp1325;
                let mut iTemp1327: i32 = (fTemp830
                    > ((if iTemp760 != 0 {
                        fTemp1325
                            + fTemp769 * fTemp1326
                            + (fTemp1321 - (iTemp1322) as F64)
                                * (fTemp1324
                                    - (fTemp1325
                                        + fTemp769
                                            * (fTemp1326
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1323, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1324))))
                    } else {
                        1.0 - (fTemp1319
                            + fTemp769 * fTemp1320
                            + (fTemp1315 - (iTemp1316) as F64)
                                * (fTemp1318
                                    - (fTemp1319
                                        + fTemp769
                                            * (fTemp1320
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp1317, 8)) as usize]
                                                } - fTemp1318)))))
                    } - fTemp1313)
                        / (1.0 - fTemp1313))) as i32;
                let mut fTemp1328: F64 = if iTemp1327 != 0 { fTemp1297 } else { fTemp1300 };
                let mut fTemp1329: F64 = if iTemp1327 != 0 { fTemp1300 } else { fTemp1298 };
                let mut fTemp1330: F64 = fTemp1329 + fTemp1328;
                let mut fTemp1331: F64 = 0.5 * fTemp1330;
                let mut fTemp1332: F64 = 524287.0 * (1.0 - fTemp1331);
                let mut iTemp1333: i32 = (fTemp1332) as i32;
                let mut iTemp1334: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1333, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1335: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1334, 7)) as usize] };
                let mut fTemp1336: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1334 as usize] };
                let mut fTemp1337: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1334, 1)) as usize] }
                        - fTemp1336;
                let mut fTemp1338: F64 = 262143.5 * fTemp1330;
                let mut iTemp1339: i32 = (fTemp1338) as i32;
                let mut iTemp1340: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1339, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1341: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1340, 7)) as usize] };
                let mut fTemp1342: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1340 as usize] };
                let mut fTemp1343: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1340, 1)) as usize] }
                        - fTemp1342;
                let mut fTemp1344: F64 = if iTemp760 != 0 {
                    fTemp1342
                        + fTemp769 * fTemp1343
                        + (fTemp1338 - (iTemp1339) as F64)
                            * (fTemp1341
                                - (fTemp1342
                                    + fTemp769
                                        * (fTemp1343
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp1340, 8)) as usize]
                                            } - fTemp1341))))
                } else {
                    1.0 - (fTemp1336
                        + fTemp769 * fTemp1337
                        + (fTemp1332 - (iTemp1333) as F64)
                            * (fTemp1335
                                - (fTemp1336
                                    + fTemp769
                                        * (fTemp1337
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0
                                                    [(i32::wrapping_add(iTemp1334, 8)) as usize]
                                            } - fTemp1335)))))
                };
                let mut fTemp1345: F64 = fTemp774 + fTemp1331;
                let mut fTemp1346: F64 = 524287.0 * (1.0 - fTemp1345);
                let mut iTemp1347: i32 = (fTemp1346) as i32;
                let mut iTemp1348: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1347, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1349: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1348, 7)) as usize] };
                let mut fTemp1350: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1348 as usize] };
                let mut fTemp1351: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1348, 1)) as usize] }
                        - fTemp1350;
                let mut fTemp1352: F64 = 524287.0 * fTemp1345;
                let mut iTemp1353: i32 = (fTemp1352) as i32;
                let mut iTemp1354: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1353, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1355: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1354, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp1356: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1354 as usize] };
                let mut fTemp1357: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1354, 1), 3670015),
                    )) as usize]
                } - fTemp1356;
                let mut iTemp1358: i32 = (fTemp830
                    > ((if iTemp760 != 0 {
                        fTemp1356
                            + fTemp769 * fTemp1357
                            + (fTemp1352 - (iTemp1353) as F64)
                                * (fTemp1355
                                    - (fTemp1356
                                        + fTemp769
                                            * (fTemp1357
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1354, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1355))))
                    } else {
                        1.0 - (fTemp1350
                            + fTemp769 * fTemp1351
                            + (fTemp1346 - (iTemp1347) as F64)
                                * (fTemp1349
                                    - (fTemp1350
                                        + fTemp769
                                            * (fTemp1351
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp1348, 8)) as usize]
                                                } - fTemp1349)))))
                    } - fTemp1344)
                        / (1.0 - fTemp1344))) as i32;
                let mut fTemp1359: F64 = if iTemp1358 != 0 { fTemp1328 } else { fTemp1331 };
                let mut fTemp1360: F64 = if iTemp1358 != 0 { fTemp1331 } else { fTemp1329 };
                let mut fTemp1361: F64 = fTemp1360 + fTemp1359;
                let mut fTemp1362: F64 = 0.5 * fTemp1361;
                let mut fTemp1363: F64 = 524287.0 * (1.0 - fTemp1362);
                let mut iTemp1364: i32 = (fTemp1363) as i32;
                let mut iTemp1365: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1364, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1366: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1365, 7)) as usize] };
                let mut fTemp1367: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1365 as usize] };
                let mut fTemp1368: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1365, 1)) as usize] }
                        - fTemp1367;
                let mut fTemp1369: F64 = 262143.5 * fTemp1361;
                let mut iTemp1370: i32 = (fTemp1369) as i32;
                let mut iTemp1371: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1370, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1372: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1371, 7)) as usize] };
                let mut fTemp1373: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1371 as usize] };
                let mut fTemp1374: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1371, 1)) as usize] }
                        - fTemp1373;
                let mut fTemp1375: F64 = if iTemp760 != 0 {
                    fTemp1373
                        + fTemp769 * fTemp1374
                        + (fTemp1369 - (iTemp1370) as F64)
                            * (fTemp1372
                                - (fTemp1373
                                    + fTemp769
                                        * (fTemp1374
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp1371, 8),
                                                        3670015,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp1372))))
                } else {
                    1.0 - (fTemp1367
                        + fTemp769 * fTemp1368
                        + (fTemp1363 - (iTemp1364) as F64)
                            * (fTemp1366
                                - (fTemp1367
                                    + fTemp769
                                        * (fTemp1368
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp1365, 8),
                                                        3670015,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp1366)))))
                };
                let mut fTemp1376: F64 = fTemp774 + fTemp1362;
                let mut fTemp1377: F64 = 524287.0 * (1.0 - fTemp1376);
                let mut iTemp1378: i32 = (fTemp1377) as i32;
                let mut iTemp1379: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1378, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1380: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1379, 7)) as usize] };
                let mut fTemp1381: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1379 as usize] };
                let mut fTemp1382: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1379, 1)) as usize] }
                        - fTemp1381;
                let mut fTemp1383: F64 = 524287.0 * fTemp1376;
                let mut iTemp1384: i32 = (fTemp1383) as i32;
                let mut iTemp1385: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1384, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1386: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1385, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp1387: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1385 as usize] };
                let mut fTemp1388: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1385, 1), 3670015),
                    )) as usize]
                } - fTemp1387;
                let mut iTemp1389: i32 = (fTemp830
                    > ((if iTemp760 != 0 {
                        fTemp1387
                            + fTemp769 * fTemp1388
                            + (fTemp1383 - (iTemp1384) as F64)
                                * (fTemp1386
                                    - (fTemp1387
                                        + fTemp769
                                            * (fTemp1388
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0[(std::cmp::max(
                                                        0,
                                                        std::cmp::min(
                                                            i32::wrapping_add(iTemp1385, 8),
                                                            3670015,
                                                        ),
                                                    ))
                                                        as usize]
                                                } - fTemp1386))))
                    } else {
                        1.0 - (fTemp1381
                            + fTemp769 * fTemp1382
                            + (fTemp1377 - (iTemp1378) as F64)
                                * (fTemp1380
                                    - (fTemp1381
                                        + fTemp769
                                            * (fTemp1382
                                                - (unsafe {
                                                    ftbl0LambRs192kSIG0
                                                        [(i32::wrapping_add(iTemp1379, 8)) as usize]
                                                } - fTemp1380)))))
                    } - fTemp1375)
                        / (1.0 - fTemp1375))) as i32;
                let mut fTemp1390: F64 = F64::min(
                    1.0,
                    F64::max(
                        0.0,
                        0.5 * (if iTemp1389 != 0 { fTemp1362 } else { fTemp1360 }
                            + if iTemp1389 != 0 { fTemp1359 } else { fTemp1362 }),
                    ),
                );
                self.fRec15[0] = fTemp1390;
                let mut fTemp1391: F64 = 524287.0 * (1.0 - fTemp1390);
                let mut iTemp1392: i32 = (fTemp1391) as i32;
                let mut iTemp1393: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1392, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1394: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1393, 7)) as usize] };
                let mut fTemp1395: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1393 as usize] };
                let mut fTemp1396: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1393, 1)) as usize] }
                        - fTemp1395;
                let mut fTemp1397: F64 = 524287.0 * fTemp1390;
                let mut iTemp1398: i32 = (fTemp1397) as i32;
                let mut iTemp1399: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1398, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1400: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1399, 7)) as usize] };
                let mut fTemp1401: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1399 as usize] };
                let mut fTemp1402: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1399, 1)) as usize] }
                        - fTemp1401;
                let mut fTemp1403: F64 = if iTemp760 != 0 {
                    fTemp1401
                        + fTemp769 * fTemp1402
                        + (fTemp1397 - (iTemp1398) as F64)
                            * (fTemp1400
                                - (fTemp1401
                                    + fTemp769
                                        * (fTemp1402
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp1399, 8),
                                                        3670015,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp1400))))
                } else {
                    1.0 - (fTemp1395
                        + fTemp769 * fTemp1396
                        + (fTemp1391 - (iTemp1392) as F64)
                            * (fTemp1394
                                - (fTemp1395
                                    + fTemp769
                                        * (fTemp1396
                                            - (unsafe {
                                                ftbl0LambRs192kSIG0[(std::cmp::max(
                                                    0,
                                                    std::cmp::min(
                                                        i32::wrapping_add(iTemp1393, 8),
                                                        3670015,
                                                    ),
                                                ))
                                                    as usize]
                                            } - fTemp1394)))))
                };
                let mut fTemp1404: F64 = fTemp774 + fTemp1390;
                let mut fTemp1405: F64 = 524287.0 * (1.0 - fTemp1404);
                let mut iTemp1406: i32 = (fTemp1405) as i32;
                let mut iTemp1407: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1406, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1408: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1407, 7)) as usize] };
                let mut fTemp1409: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1407 as usize] };
                let mut fTemp1410: F64 =
                    unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1407, 1)) as usize] }
                        - fTemp1409;
                let mut fTemp1411: F64 = 524287.0 * fTemp1404;
                let mut iTemp1412: i32 = (fTemp1411) as i32;
                let mut iTemp1413: i32 = std::cmp::max(
                    0,
                    std::cmp::min(
                        i32::wrapping_add(
                            iTemp764,
                            i32::wrapping_mul(
                                7,
                                std::cmp::max(0, std::cmp::min(iTemp1412, 524287)),
                            ),
                        ),
                        3670015,
                    ),
                );
                let mut fTemp1414: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1413, 7), 3670015),
                    )) as usize]
                };
                let mut fTemp1415: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1413 as usize] };
                let mut fTemp1416: F64 = unsafe {
                    ftbl0LambRs192kSIG0[(std::cmp::max(
                        0,
                        std::cmp::min(i32::wrapping_add(iTemp1413, 1), 3670015),
                    )) as usize]
                } - fTemp1415;
                let mut fTemp1417: F64 = self.fRec16[1]
                    + if ((0.001 * fTemp773) == 0.0) as i32 != 0 {
                        fTemp759
                    } else {
                        fTemp759
                            * (if iTemp760 != 0 {
                                fTemp1415
                                    + fTemp769 * fTemp1416
                                    + (fTemp1411 - (iTemp1412) as F64)
                                        * (fTemp1414
                                            - (fTemp1415
                                                + fTemp769
                                                    * (fTemp1416
                                                        - (unsafe {
                                                            ftbl0LambRs192kSIG0[(std::cmp::max(
                                                                0,
                                                                std::cmp::min(
                                                                    i32::wrapping_add(iTemp1413, 8),
                                                                    3670015,
                                                                ),
                                                            ))
                                                                as usize]
                                                        } - fTemp1414))))
                            } else {
                                1.0 - (fTemp1409
                                    + fTemp769 * fTemp1410
                                    + (fTemp1405 - (iTemp1406) as F64)
                                        * (fTemp1408
                                            - (fTemp1409
                                                + fTemp769
                                                    * (fTemp1410
                                                        - (unsafe {
                                                            ftbl0LambRs192kSIG0[(i32::wrapping_add(
                                                                iTemp1407, 8,
                                                            ))
                                                                as usize]
                                                        } - fTemp1408)))))
                            } - fTemp1403)
                            / (1.0 - fTemp1403)
                    };
                self.fRec16[0] = if iTemp772 != 0 {
                    F64::min(fTemp1417, self.fRec16[1])
                } else {
                    F64::max(fTemp1417, self.fRec16[1])
                };
                self.fVec66[(self.IOTA0 & 32767) as usize] =
                    F64::powf(1e+01, 0.05 * self.fRec16[0]);
                let mut fTemp1418: F64 =
                    self.fVec66[((i32::wrapping_sub(self.IOTA0, iSlow78)) & 32767) as usize];
                *output1 = 0.5
                    * fTemp2
                    * self.fVec3[((i32::wrapping_sub(self.IOTA0, iSlow79)) & 32767) as usize]
                    + self.fRec14[0]
                        * fTemp4
                        * self.fVec4[((i32::wrapping_sub(self.IOTA0, iSlow79)) & 32767) as usize]
                        * fTemp1418;
                *output2 = fTemp3 + fTemp730 * fTemp4;
                *output3 = fTemp3 + fTemp4 * fTemp1418;
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
                self.fVec19[2] = self.fVec19[1];
                self.fVec19[1] = self.fVec19[0];
                for j2 in (1..=6).rev() {
                    self.fVec20[j2 as usize] = self.fVec20[(i32::wrapping_sub(j2, 1)) as usize];
                }
                for j3 in (1..=14).rev() {
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
                for j4 in (1..=6).rev() {
                    self.fVec38[j4 as usize] = self.fVec38[(i32::wrapping_sub(j4, 1)) as usize];
                }
                for j5 in (1..=14).rev() {
                    self.fVec39[j5 as usize] = self.fVec39[(i32::wrapping_sub(j5, 1)) as usize];
                }
                self.fRec17[1] = self.fRec17[0];
                self.fVec50[2] = self.fVec50[1];
                self.fVec50[1] = self.fVec50[0];
                for j6 in (1..=6).rev() {
                    self.fVec51[j6 as usize] = self.fVec51[(i32::wrapping_sub(j6, 1)) as usize];
                }
                for j7 in (1..=14).rev() {
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
}

pub use dsp_192k::LambRs192k;
