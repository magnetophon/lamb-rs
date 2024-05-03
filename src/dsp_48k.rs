/* ------------------------------------------------------------
author: "Bart Brouns"
license: "AGPLv3"
name: "lamb-rs"
version: "0.1"
Code generated with Faust 2.72.14 (https://faust.grame.fr)
Compilation options: -a /run/user/1001/.tmpYdFTJN -lang rust -ct 1 -cn LambRs48k -es 1 -mcd 16 -mdd 1024 -mdy 33 -double -ftz 0
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


pub struct LambRs48kSIG0 {
	iRec13: [i32;2],
}

impl LambRs48kSIG0 {
	
	fn get_num_inputsLambRs48kSIG0(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsLambRs48kSIG0(&self) -> i32 {
		return 1;
	}
	
	fn instance_initLambRs48kSIG0(&mut self, sample_rate: i32) {
		for l40 in 0..2 {
			self.iRec13[l40 as usize] = 0;
		}
	}
	
	fn fillLambRs48kSIG0(&mut self, count: i32, table: &mut[F64]) {
		for i1 in 0..count {
			self.iRec13[0] = i32::wrapping_add(self.iRec13[1], 1);
			let mut iTemp64: i32 = i32::wrapping_add(self.iRec13[0], -1);
			let mut fTemp65: F64 = (iTemp64 % 7) as F64 as i32 as F64;
			let mut fTemp66: F64 = 0.16666666666666666 * fTemp65;
			let mut fTemp67: F64 = F64::powf(fTemp66, 0.06999999999999999 * fTemp65 + 1.0);
			let mut fTemp68: F64 = (0.14285714285714285 * (iTemp64 % 917504) as F64) as i32 as F64;
			table[i1 as usize] = F64::min(1.0, F64::max(0.0, if (fTemp66 == 0.0) as i32 != 0 {0.5 * (F64::sin(2.396863267686821e-05 * fTemp68 + 4.71238898038469) + 1.0)} else {0.5 * (F64::sin(3.141592653589793 * ((1.0 - F64::exp(-(1.8463275629239114e-05 * fTemp67 * fTemp68))) / (1.0 - F64::exp(-(2.42 * fTemp67)))) + 4.71238898038469) + 1.0)}));
			self.iRec13[1] = self.iRec13[0];
		}
	}

}


pub fn newLambRs48kSIG0() -> LambRs48kSIG0 { 
	LambRs48kSIG0 {
		iRec13: [0;2],
	}
}
fn LambRs48k_faustpower2_f(value: F64) -> F64 {
	return value * value;
}
static mut ftbl0LambRs48kSIG0: [F64;917504] = [0.0;917504];
mod ffi {
	use std::os::raw::{c_double};
	// Conditionally compile the link attribute only on non-Windows platforms
	#[cfg_attr(not(target_os="windows"), link(name="m"))]
	extern {
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
pub struct LambRs48k {
	fCheckbox0: F64,
	IOTA0: i32,
	iVec0: [i32;8192],
	fSampleRate: i32,
	fConst0: F64,
	fConst1: F64,
	fRec0: [F64;2],
	fHslider0: F64,
	fHslider1: F64,
	fConst2: F64,
	fConst3: F64,
	fConst4: F64,
	fHslider2: F64,
	fConst5: F64,
	fRec4: [F64;2],
	fHslider3: F64,
	fHslider4: F64,
	fHslider5: F64,
	fRec11: [F64;2],
	fVec1: [F64;32768],
	fVec2: [F64;32768],
	fVec3: [F64;32768],
	fVec4: [F64;32768],
	fConst6: F64,
	fHslider6: F64,
	fConst7: F64,
	fRec10: [F64;2],
	fRec9: [F64;2],
	fRec8: [F64;2],
	fRec7: [F64;2],
	fRec5: [F64;2],
	fConst8: F64,
	fRec12: [F64;2],
	fRec6: [F64;2],
	fHslider7: F64,
	fHslider8: F64,
	fVec5: [F64;16384],
	fHslider9: F64,
	fConst9: F64,
	fVec6: [F64;3],
	fVec7: [F64;7],
	fVec8: [F64;15],
	fVec9: [F64;32],
	fVec10: [F64;64],
	fVec11: [F64;128],
	fVec12: [F64;256],
	fVec13: [F64;512],
	fVec14: [F64;1024],
	fVec15: [F64;2048],
	fVec16: [F64;4096],
	fRec3: [F64;2],
	fVec17: [F64;3],
	fVec18: [F64;7],
	fVec19: [F64;15],
	fVec20: [F64;32],
	fVec21: [F64;64],
	fVec22: [F64;128],
	fVec23: [F64;256],
	fVec24: [F64;512],
	fVec25: [F64;1024],
	fVec26: [F64;2048],
	fVec27: [F64;4096],
	fVec28: [F64;2],
	fHslider10: F64,
	fHslider11: F64,
	fVec29: [F64;2],
	fVec30: [F64;2],
	fConst10: F64,
	fRec1: [F64;2],
	fRec2: [F64;2],
	fVec31: [F64;8192],
	fCheckbox1: F64,
	fHbargraph0: F64,
	fHslider12: F64,
	fRec14: [F64;2],
	fVec32: [F64;16384],
	fVec33: [F64;3],
	fVec34: [F64;7],
	fVec35: [F64;15],
	fVec36: [F64;32],
	fVec37: [F64;64],
	fVec38: [F64;128],
	fVec39: [F64;256],
	fVec40: [F64;512],
	fVec41: [F64;1024],
	fVec42: [F64;2048],
	fVec43: [F64;4096],
	fRec17: [F64;2],
	fVec44: [F64;3],
	fVec45: [F64;7],
	fVec46: [F64;15],
	fVec47: [F64;32],
	fVec48: [F64;64],
	fVec49: [F64;128],
	fVec50: [F64;256],
	fVec51: [F64;512],
	fVec52: [F64;1024],
	fVec53: [F64;2048],
	fVec54: [F64;4096],
	fVec55: [F64;2],
	fVec56: [F64;2],
	fVec57: [F64;2],
	fRec15: [F64;2],
	fRec16: [F64;2],
	fVec58: [F64;8192],
}

impl FaustDsp for LambRs48k {
	type T = F64;
		
	fn new() -> LambRs48k { 
		LambRs48k {
			fCheckbox0: 0.0,
			IOTA0: 0,
			iVec0: [0;8192],
			fSampleRate: 0,
			fConst0: 0.0,
			fConst1: 0.0,
			fRec0: [0.0;2],
			fHslider0: 0.0,
			fHslider1: 0.0,
			fConst2: 0.0,
			fConst3: 0.0,
			fConst4: 0.0,
			fHslider2: 0.0,
			fConst5: 0.0,
			fRec4: [0.0;2],
			fHslider3: 0.0,
			fHslider4: 0.0,
			fHslider5: 0.0,
			fRec11: [0.0;2],
			fVec1: [0.0;32768],
			fVec2: [0.0;32768],
			fVec3: [0.0;32768],
			fVec4: [0.0;32768],
			fConst6: 0.0,
			fHslider6: 0.0,
			fConst7: 0.0,
			fRec10: [0.0;2],
			fRec9: [0.0;2],
			fRec8: [0.0;2],
			fRec7: [0.0;2],
			fRec5: [0.0;2],
			fConst8: 0.0,
			fRec12: [0.0;2],
			fRec6: [0.0;2],
			fHslider7: 0.0,
			fHslider8: 0.0,
			fVec5: [0.0;16384],
			fHslider9: 0.0,
			fConst9: 0.0,
			fVec6: [0.0;3],
			fVec7: [0.0;7],
			fVec8: [0.0;15],
			fVec9: [0.0;32],
			fVec10: [0.0;64],
			fVec11: [0.0;128],
			fVec12: [0.0;256],
			fVec13: [0.0;512],
			fVec14: [0.0;1024],
			fVec15: [0.0;2048],
			fVec16: [0.0;4096],
			fRec3: [0.0;2],
			fVec17: [0.0;3],
			fVec18: [0.0;7],
			fVec19: [0.0;15],
			fVec20: [0.0;32],
			fVec21: [0.0;64],
			fVec22: [0.0;128],
			fVec23: [0.0;256],
			fVec24: [0.0;512],
			fVec25: [0.0;1024],
			fVec26: [0.0;2048],
			fVec27: [0.0;4096],
			fVec28: [0.0;2],
			fHslider10: 0.0,
			fHslider11: 0.0,
			fVec29: [0.0;2],
			fVec30: [0.0;2],
			fConst10: 0.0,
			fRec1: [0.0;2],
			fRec2: [0.0;2],
			fVec31: [0.0;8192],
			fCheckbox1: 0.0,
			fHbargraph0: 0.0,
			fHslider12: 0.0,
			fRec14: [0.0;2],
			fVec32: [0.0;16384],
			fVec33: [0.0;3],
			fVec34: [0.0;7],
			fVec35: [0.0;15],
			fVec36: [0.0;32],
			fVec37: [0.0;64],
			fVec38: [0.0;128],
			fVec39: [0.0;256],
			fVec40: [0.0;512],
			fVec41: [0.0;1024],
			fVec42: [0.0;2048],
			fVec43: [0.0;4096],
			fRec17: [0.0;2],
			fVec44: [0.0;3],
			fVec45: [0.0;7],
			fVec46: [0.0;15],
			fVec47: [0.0;32],
			fVec48: [0.0;64],
			fVec49: [0.0;128],
			fVec50: [0.0;256],
			fVec51: [0.0;512],
			fVec52: [0.0;1024],
			fVec53: [0.0;2048],
			fVec54: [0.0;4096],
			fVec55: [0.0;2],
			fVec56: [0.0;2],
			fVec57: [0.0;2],
			fRec15: [0.0;2],
			fRec16: [0.0;2],
			fVec58: [0.0;8192],
		}
	}
	fn metadata(&self, m: &mut dyn Meta) { 
		m.declare("author", r"Bart Brouns");
		m.declare("basics.lib/name", r"Faust Basic Element Library");
		m.declare("basics.lib/parallelMax:author", r"Bart Brouns");
		m.declare("basics.lib/parallelMax:copyright", r"Copyright (c) 2020 Bart Brouns <bart@magnetophon.nl>");
		m.declare("basics.lib/parallelMax:licence", r"GPL-3.0");
		m.declare("basics.lib/parallelOp:author", r"Bart Brouns");
		m.declare("basics.lib/parallelOp:copyright", r"Copyright (c) 2020 Bart Brouns <bart@magnetophon.nl>");
		m.declare("basics.lib/parallelOp:licence", r"GPL-3.0");
		m.declare("basics.lib/tabulateNd", r"Copyright (C) 2023 Bart Brouns <bart@magnetophon.nl>");
		m.declare("basics.lib/tabulateNd:author", r"Bart Brouns");
		m.declare("basics.lib/tabulateNd:license", r"AGPL-3.0");
		m.declare("basics.lib/version", r"1.15.0");
		m.declare("compile_options", r"-a /run/user/1001/.tmpYdFTJN -lang rust -ct 1 -cn LambRs48k -es 1 -mcd 16 -mdd 1024 -mdy 33 -double -ftz 0");
		m.declare("filename", r"lamb-rs-48k.dsp");
		m.declare("interpolators.lib/interpolate_linear:author", r"Stéphane Letz");
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
		let mut sig0: LambRs48kSIG0 = newLambRs48kSIG0();
		sig0.instance_initLambRs48kSIG0(sample_rate);
		sig0.fillLambRs48kSIG0(917504, unsafe { &mut ftbl0LambRs48kSIG0 });
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
		for l27 in 0..2 {
			self.fRec3[l27 as usize] = 0.0;
		}
		for l28 in 0..3 {
			self.fVec17[l28 as usize] = 0.0;
		}
		for l29 in 0..7 {
			self.fVec18[l29 as usize] = 0.0;
		}
		for l30 in 0..15 {
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
		for l49 in 0..7 {
			self.fVec34[l49 as usize] = 0.0;
		}
		for l50 in 0..15 {
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
		for l61 in 0..7 {
			self.fVec45[l61 as usize] = 0.0;
		}
		for l62 in 0..15 {
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
		LambRs48k::class_init(sample_rate);
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
		ui_interface.add_horizontal_slider("release hold", ParamIndex(9), 5e+01, 0.020833333333333332, 5e+01, 1.0);
		ui_interface.declare(Some(ParamIndex(10)), "09", "");
		ui_interface.add_horizontal_slider("knee", ParamIndex(10), 1.0, 0.0, 3e+01, 0.1);
		ui_interface.declare(Some(ParamIndex(11)), "10", "");
		ui_interface.add_horizontal_slider("link", ParamIndex(11), 0.0, 0.0, 1e+02, 1.0);
		ui_interface.declare(Some(ParamIndex(12)), "11", "");
		ui_interface.add_horizontal_slider("adaptive release", ParamIndex(12), 5e+01, 0.0, 1e+02, 1.0);
		ui_interface.declare(Some(ParamIndex(13)), "12", "");
		ui_interface.add_horizontal_slider("lookahead", ParamIndex(13), 1e+02, 0.0, 1e+02, 1.0);
		ui_interface.close_box();
		ui_interface.close_box();
		ui_interface.declare(Some(ParamIndex(14)), "13", "");
		ui_interface.add_horizontal_slider("output gain", ParamIndex(14), 0.0, -24.0, 24.0, 0.1);
		ui_interface.declare(Some(ParamIndex(15)), "99", "");
		ui_interface.declare(Some(ParamIndex(15)), "unit", "samples");
		ui_interface.add_horizontal_bargraph("latency", ParamIndex(15), 0.0, 4.8e+03);
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
			0 => { self.fCheckbox0 = value }
			1 => { self.fCheckbox1 = value }
			15 => { self.fHbargraph0 = value }
			13 => { self.fHslider0 = value }
			5 => { self.fHslider1 = value }
			6 => { self.fHslider10 = value }
			8 => { self.fHslider11 = value }
			14 => { self.fHslider12 = value }
			12 => { self.fHslider2 = value }
			10 => { self.fHslider3 = value }
			4 => { self.fHslider4 = value }
			2 => { self.fHslider5 = value }
			7 => { self.fHslider6 = value }
			3 => { self.fHslider7 = value }
			11 => { self.fHslider8 = value }
			9 => { self.fHslider9 = value }
			_ => {}
		}
	}
	
	fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut[&mut[Self::T]]) {
		let (inputs0, inputs1) = if let [inputs0, inputs1, ..] = inputs {
			let inputs0 = inputs0[..count as usize].iter();
			let inputs1 = inputs1[..count as usize].iter();
			(inputs0, inputs1)
		} else {
			panic!("wrong number of inputs");
		};
		let (outputs0, outputs1, outputs2, outputs3) = if let [outputs0, outputs1, outputs2, outputs3, ..] = outputs {
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
		self.fHbargraph0 = if (fSlow69) as i32 != 0 {4.8e+03} else {fSlow68};
		let mut iSlow71: i32 = (self.fHbargraph0) as i32;
		let mut fSlow72: F64 = self.fConst3 * F64::powf(1e+01, 0.05 * self.fHslider12);
		let zipped_iterators = inputs0.zip(inputs1).zip(outputs0).zip(outputs1).zip(outputs2).zip(outputs3);
		for (((((input0, input1), output0), output1), output2), output3) in zipped_iterators {
			self.iVec0[(self.IOTA0 & 8191) as usize] = 1;
			let mut fTemp0: F64 = self.fConst1 + self.fRec0[1];
			let mut fTemp1: F64 = self.fRec0[1] - self.fConst1;
			self.fRec0[0] = if (fTemp0 < fSlow0) as i32 != 0 {fTemp0} else {if (fTemp1 > fSlow0) as i32 != 0 {fTemp1} else {fSlow0}};
			let mut fTemp2: F64 = F64::sin(6.283185307179586 * (0.5 * self.fRec0[0] + 0.75)) + 1.0;
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
			let mut fTemp18: F64 = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, F64::max(fTemp14, fTemp17)));
			let mut iTemp19: i32 = ((fTemp18 > fSlow11) as i32) + ((fTemp18 > fSlow9) as i32);
			let mut fTemp20: F64 = fTemp18 - fSlow8;
			let mut fTemp21: F64 = F64::powf(1e+01, -(0.05 * F64::max(0.0, if (iTemp19 == 0) as i32 != 0 {0.0} else {if (iTemp19 == 1) as i32 != 0 {fSlow12 * LambRs48k_faustpower2_f(fSlow7 + fTemp20)} else {fTemp20}})));
			let mut fTemp22: F64 = 3.0 * fTemp5;
			let mut fTemp23: F64 = 4.0 * (F64::max(0.25, self.fRec4[0]) + -0.25);
			let mut fTemp24: F64 = 9.0 - fTemp23;
			let mut fTemp25: F64 = self.fRec5[1] - self.fRec6[1];
			let mut fTemp26: F64 = (self.iVec0[((i32::wrapping_sub(self.IOTA0, 4800)) & 8191) as usize]) as F64;
			let mut fTemp27: F64 = if (fTemp21 > self.fRec10[1]) as i32 != 0 {F64::exp(-(self.fConst7 / F64::max(2.220446049250313e-16, fSlow14 * (fTemp26 / F64::max(1.0 - (F64::max(fTemp23 + -9.0, F64::min(2.0 - fTemp22, fTemp25)) + fTemp24) / (11.0 - (fTemp23 + fTemp22)), 2.220446049250313e-16)))))} else {self.fConst6};
			self.fRec10[0] = self.fRec10[1] * fTemp27 + fTemp21 * (1.0 - fTemp27);
			let mut fTemp28: F64 = if (self.fRec10[0] > self.fRec9[1]) as i32 != 0 {0.0} else {self.fConst6};
			self.fRec9[0] = self.fRec9[1] * fTemp28 + self.fRec10[0] * (1.0 - fTemp28);
			let mut fTemp29: F64 = if (self.fRec9[0] > self.fRec8[1]) as i32 != 0 {0.0} else {self.fConst6};
			self.fRec8[0] = self.fRec8[1] * fTemp29 + self.fRec9[0] * (1.0 - fTemp29);
			let mut fTemp30: F64 = if (self.fRec8[0] > self.fRec7[1]) as i32 != 0 {0.0} else {self.fConst6};
			self.fRec7[0] = self.fRec7[1] * fTemp30 + self.fRec8[0] * (1.0 - fTemp30);
			self.fRec5[0] = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, self.fRec7[0]));
			let mut fTemp31: F64 = F64::powf(1e+01, 0.05 * (self.fRec5[1] + fTemp24));
			let mut fTemp32: F64 = if (fTemp31 > self.fRec12[1]) as i32 != 0 {F64::exp(-(self.fConst7 / F64::max(2.220446049250313e-16, fTemp26 * (0.8161290322580644 * (F64::max(0.69, self.fRec4[0]) + -0.69) + 0.05) * F64::powf(4503599627370496.0, 1.0 - (F64::max(fTemp10, F64::min(fTemp11, fTemp25)) + fTemp9) / fTemp8))))} else {self.fConst8};
			self.fRec12[0] = self.fRec12[1] * fTemp32 + fTemp31 * (1.0 - fTemp32);
			self.fRec6[0] = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, self.fRec12[0]));
			let mut fTemp33: F64 = self.fRec5[0] - self.fRec6[0];
			let mut fTemp34: F64 = fSlow16 * F64::min(0.25, self.fRec4[0]) * (self.fRec6[0] + fTemp33 * (F64::max(fTemp10, F64::min(fTemp11, fTemp33)) + fTemp9) / fTemp8);
			let mut fTemp35: F64 = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, fTemp14));
			let mut fTemp36: F64 = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, fTemp17));
			let mut fTemp37: F64 = F64::max(fTemp35, fTemp36);
			let mut fTemp38: F64 = fTemp35 + fSlow17 * (fTemp37 - fTemp35);
			let mut iTemp39: i32 = ((fTemp38 > fSlow11) as i32) + ((fTemp38 > fSlow9) as i32);
			let mut fTemp40: F64 = fTemp38 - fSlow8;
			let mut fTemp41: F64 = F64::min(fTemp34, -(fSlow18 * F64::max(0.0, if (iTemp39 == 0) as i32 != 0 {0.0} else {if (iTemp39 == 1) as i32 != 0 {fSlow12 * LambRs48k_faustpower2_f(fSlow7 + fTemp40)} else {fTemp40}})));
			self.fVec5[(self.IOTA0 & 16383) as usize] = fTemp41;
			let mut fTemp42: F64 = F64::min(fTemp41, self.fVec5[((i32::wrapping_sub(self.IOTA0, 1)) & 16383) as usize]);
			self.fVec6[0] = fTemp42;
			let mut fTemp43: F64 = F64::min(fTemp42, self.fVec6[2]);
			self.fVec7[0] = fTemp43;
			let mut fTemp44: F64 = F64::min(fTemp43, self.fVec7[4]);
			self.fVec8[0] = fTemp44;
			let mut fTemp45: F64 = F64::min(fTemp44, self.fVec8[8]);
			self.fVec9[(self.IOTA0 & 31) as usize] = fTemp45;
			let mut fTemp46: F64 = F64::min(fTemp45, self.fVec9[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize]);
			self.fVec10[(self.IOTA0 & 63) as usize] = fTemp46;
			let mut fTemp47: F64 = F64::min(fTemp46, self.fVec10[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize]);
			self.fVec11[(self.IOTA0 & 127) as usize] = fTemp47;
			let mut fTemp48: F64 = F64::min(fTemp47, self.fVec11[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize]);
			self.fVec12[(self.IOTA0 & 255) as usize] = fTemp48;
			let mut fTemp49: F64 = F64::min(fTemp48, self.fVec12[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize]);
			self.fVec13[(self.IOTA0 & 511) as usize] = fTemp49;
			let mut fTemp50: F64 = F64::min(fTemp49, self.fVec13[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize]);
			self.fVec14[(self.IOTA0 & 1023) as usize] = fTemp50;
			let mut fTemp51: F64 = F64::min(fTemp50, self.fVec14[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize]);
			self.fVec15[(self.IOTA0 & 2047) as usize] = fTemp51;
			self.fVec16[(self.IOTA0 & 4095) as usize] = F64::min(fTemp51, self.fVec15[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			self.fRec3[0] = F64::max(F64::min(self.fRec3[1], self.fVec5[((i32::wrapping_sub(self.IOTA0, iSlow21)) & 16383) as usize]), F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow23 != 0 {fTemp41} else {1.7976931348623157e+308}, if iSlow24 != 0 {self.fVec6[iSlow23 as usize]} else {1.7976931348623157e+308}), if iSlow25 != 0 {self.fVec7[iSlow26 as usize]} else {1.7976931348623157e+308}), if iSlow27 != 0 {self.fVec8[iSlow28 as usize]} else {1.7976931348623157e+308}), if iSlow29 != 0 {self.fVec9[((i32::wrapping_sub(self.IOTA0, iSlow30)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow31 != 0 {self.fVec10[((i32::wrapping_sub(self.IOTA0, iSlow32)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow33 != 0 {self.fVec11[((i32::wrapping_sub(self.IOTA0, iSlow34)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow35 != 0 {self.fVec12[((i32::wrapping_sub(self.IOTA0, iSlow36)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow37 != 0 {self.fVec13[((i32::wrapping_sub(self.IOTA0, iSlow38)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow39 != 0 {self.fVec14[((i32::wrapping_sub(self.IOTA0, iSlow40)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow41 != 0 {self.fVec15[((i32::wrapping_sub(self.IOTA0, iSlow42)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow43 != 0 {self.fVec16[((i32::wrapping_sub(self.IOTA0, iSlow44)) & 4095) as usize]} else {1.7976931348623157e+308}));
			let mut fTemp52: F64 = F64::min(self.fRec3[0], self.fRec3[1]);
			self.fVec17[0] = fTemp52;
			let mut fTemp53: F64 = F64::min(fTemp52, self.fVec17[2]);
			self.fVec18[0] = fTemp53;
			let mut fTemp54: F64 = F64::min(fTemp53, self.fVec18[4]);
			self.fVec19[0] = fTemp54;
			let mut fTemp55: F64 = F64::min(fTemp54, self.fVec19[8]);
			self.fVec20[(self.IOTA0 & 31) as usize] = fTemp55;
			let mut fTemp56: F64 = F64::min(fTemp55, self.fVec20[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize]);
			self.fVec21[(self.IOTA0 & 63) as usize] = fTemp56;
			let mut fTemp57: F64 = F64::min(fTemp56, self.fVec21[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize]);
			self.fVec22[(self.IOTA0 & 127) as usize] = fTemp57;
			let mut fTemp58: F64 = F64::min(fTemp57, self.fVec22[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize]);
			self.fVec23[(self.IOTA0 & 255) as usize] = fTemp58;
			let mut fTemp59: F64 = F64::min(fTemp58, self.fVec23[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize]);
			self.fVec24[(self.IOTA0 & 511) as usize] = fTemp59;
			let mut fTemp60: F64 = F64::min(fTemp59, self.fVec24[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize]);
			self.fVec25[(self.IOTA0 & 1023) as usize] = fTemp60;
			let mut fTemp61: F64 = F64::min(fTemp60, self.fVec25[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize]);
			self.fVec26[(self.IOTA0 & 2047) as usize] = fTemp61;
			self.fVec27[(self.IOTA0 & 4095) as usize] = F64::min(fTemp61, self.fVec26[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			let mut fTemp62: F64 = F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow4 != 0 {self.fRec3[0]} else {1.7976931348623157e+308}, if iSlow45 != 0 {self.fVec17[iSlow4 as usize]} else {1.7976931348623157e+308}), if iSlow46 != 0 {self.fVec18[iSlow47 as usize]} else {1.7976931348623157e+308}), if iSlow48 != 0 {self.fVec19[iSlow49 as usize]} else {1.7976931348623157e+308}), if iSlow50 != 0 {self.fVec20[((i32::wrapping_sub(self.IOTA0, iSlow51)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow52 != 0 {self.fVec21[((i32::wrapping_sub(self.IOTA0, iSlow53)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow54 != 0 {self.fVec22[((i32::wrapping_sub(self.IOTA0, iSlow55)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow56 != 0 {self.fVec23[((i32::wrapping_sub(self.IOTA0, iSlow57)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow58 != 0 {self.fVec24[((i32::wrapping_sub(self.IOTA0, iSlow59)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow60 != 0 {self.fVec25[((i32::wrapping_sub(self.IOTA0, iSlow61)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow62 != 0 {self.fVec26[((i32::wrapping_sub(self.IOTA0, iSlow63)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow64 != 0 {self.fVec27[((i32::wrapping_sub(self.IOTA0, iSlow65)) & 4095) as usize]} else {1.7976931348623157e+308}) - self.fRec2[1];
			self.fVec28[0] = fTemp62;
			let mut iTemp63: i32 = (fTemp62 > 0.0) as i32;
			let mut fTemp69: F64 = if iTemp63 != 0 {fSlow67} else {fSlow66};
			self.fVec29[0] = fTemp69;
			let mut fTemp70: F64 = 6.0 * fTemp69;
			let mut iTemp71: i32 = (fTemp70) as i32;
			let mut iTemp72: i32 = std::cmp::max(0, std::cmp::min(iTemp71, 6));
			let mut iTemp73: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, 458745), 917503));
			let mut fTemp74: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp73, 7)) as usize] };
			let mut fTemp75: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp73 as usize] };
			let mut fTemp76: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp73, 1)) as usize] } - fTemp75;
			let mut fTemp77: F64 = fTemp70 - (iTemp71) as F64;
			let mut fTemp78: F64 = fTemp75 + fTemp77 * fTemp76 + 0.5 * (fTemp74 - (fTemp75 + fTemp77 * (fTemp76 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp73, 8)) as usize] } - fTemp74))));
			let mut fTemp79: F64 = if iTemp63 != 0 {fTemp78} else {1.0 - fTemp78};
			let mut iTemp80: i32 = (fTemp62 < 0.0) as i32;
			let mut fTemp81: F64 = fSlow1 * (iTemp80) as F64 + fSlow13 * (iTemp63) as F64;
			self.fVec30[0] = fTemp81;
			let mut fTemp82: F64 = self.fConst10 / fTemp81;
			let mut fTemp83: F64 = fTemp82 + 0.5;
			let mut fTemp84: F64 = 131071.0 * (1.0 - fTemp83);
			let mut iTemp85: i32 = (fTemp84) as i32;
			let mut iTemp86: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp85, 131071)))), 917503));
			let mut fTemp87: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp86, 7)) as usize] };
			let mut fTemp88: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp86 as usize] };
			let mut fTemp89: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp86, 1)) as usize] } - fTemp88;
			let mut fTemp90: F64 = 131071.0 * fTemp83;
			let mut iTemp91: i32 = (fTemp90) as i32;
			let mut iTemp92: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp91, 131071)))), 917503));
			let mut fTemp93: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp92, 7), 917503))) as usize] };
			let mut fTemp94: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp92 as usize] };
			let mut fTemp95: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp92, 1), 917503))) as usize] } - fTemp94;
			let mut fTemp96: F64 = 6.0 * self.fVec29[1];
			let mut iTemp97: i32 = (fTemp96) as i32;
			let mut iTemp98: i32 = std::cmp::max(0, std::cmp::min(iTemp97, 6));
			let mut fTemp99: F64 = 131071.0 * (1.0 - self.fRec1[1]);
			let mut iTemp100: i32 = (fTemp99) as i32;
			let mut iTemp101: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp100, 131071))), iTemp98), 917503));
			let mut fTemp102: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp101, 7), 917503))) as usize] };
			let mut fTemp103: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp101 as usize] };
			let mut fTemp104: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp101, 1), 917503))) as usize] } - fTemp103;
			let mut fTemp105: F64 = fTemp96 - (iTemp97) as F64;
			let mut fTemp106: F64 = 131071.0 * self.fRec1[1];
			let mut iTemp107: i32 = (fTemp106) as i32;
			let mut iTemp108: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp98, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp107, 131071)))), 917503));
			let mut fTemp109: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp108, 7), 917503))) as usize] };
			let mut fTemp110: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp108 as usize] };
			let mut fTemp111: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp108, 1), 917503))) as usize] } - fTemp110;
			let mut fTemp112: F64 = self.fRec1[1] + fTemp82;
			let mut fTemp113: F64 = 131071.0 * (1.0 - fTemp112);
			let mut iTemp114: i32 = (fTemp113) as i32;
			let mut iTemp115: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp114, 131071)))), 917503));
			let mut fTemp116: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp115, 7)) as usize] };
			let mut fTemp117: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp115 as usize] };
			let mut fTemp118: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp115, 1)) as usize] } - fTemp117;
			let mut fTemp119: F64 = 131071.0 * fTemp112;
			let mut iTemp120: i32 = (fTemp119) as i32;
			let mut iTemp121: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp120, 131071)))), 917503));
			let mut fTemp122: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp121, 7), 917503))) as usize] };
			let mut fTemp123: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp121 as usize] };
			let mut fTemp124: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp121, 1), 917503))) as usize] } - fTemp123;
			let mut fTemp125: F64 = self.fRec1[1] + self.fConst10 * (1.0 / fTemp81 + 1.0 / self.fVec30[1]);
			let mut fTemp126: F64 = 131071.0 * (1.0 - fTemp125);
			let mut iTemp127: i32 = (fTemp126) as i32;
			let mut iTemp128: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp127, 131071))), iTemp72), 917503));
			let mut fTemp129: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp128, 7)) as usize] };
			let mut fTemp130: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp128 as usize] };
			let mut fTemp131: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp128, 1)) as usize] } - fTemp130;
			let mut fTemp132: F64 = 131071.0 * fTemp125;
			let mut iTemp133: i32 = (fTemp132) as i32;
			let mut iTemp134: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp133, 131071)))), 917503));
			let mut fTemp135: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp134, 7), 917503))) as usize] };
			let mut fTemp136: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp134 as usize] };
			let mut fTemp137: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp134, 1), 917503))) as usize] } - fTemp136;
			let mut fTemp138: F64 = (if iTemp63 != 0 {fTemp136 + fTemp77 * fTemp137 + (fTemp132 - (iTemp133) as F64) * (fTemp135 - (fTemp136 + fTemp77 * (fTemp137 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp134, 8), 917503))) as usize] } - fTemp135))))} else {1.0 - (fTemp130 + fTemp77 * fTemp131 + (fTemp126 - (iTemp127) as F64) * (fTemp129 - (fTemp130 + fTemp77 * (fTemp131 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp128, 8)) as usize] } - fTemp129)))))} - if iTemp63 != 0 {fTemp123 + fTemp77 * fTemp124 + (fTemp119 - (iTemp120) as F64) * (fTemp122 - (fTemp123 + fTemp77 * (fTemp124 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp121, 8), 917503))) as usize] } - fTemp122))))} else {1.0 - (fTemp117 + fTemp77 * fTemp118 + (fTemp113 - (iTemp114) as F64) * (fTemp116 - (fTemp117 + fTemp77 * (fTemp118 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp115, 8)) as usize] } - fTemp116)))))}) * self.fVec28[1] / (fTemp62 * (1.0 - if iTemp63 != 0 {fTemp110 + fTemp105 * fTemp111 + (fTemp106 - (iTemp107) as F64) * (fTemp109 - (fTemp110 + fTemp105 * (fTemp111 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp108, 8), 917503))) as usize] } - fTemp109))))} else {1.0 - (fTemp103 + fTemp105 * fTemp104 + (fTemp99 - (iTemp100) as F64) * (fTemp102 - (fTemp103 + fTemp105 * (fTemp104 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp101, 8), 917503))) as usize] } - fTemp102)))))}));
			let mut iTemp139: i32 = (fTemp138 > ((if iTemp63 != 0 {fTemp94 + fTemp77 * fTemp95 + (fTemp90 - (iTemp91) as F64) * (fTemp93 - (fTemp94 + fTemp77 * (fTemp95 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp92, 8), 917503))) as usize] } - fTemp93))))} else {1.0 - (fTemp88 + fTemp77 * fTemp89 + (fTemp84 - (iTemp85) as F64) * (fTemp87 - (fTemp88 + fTemp77 * (fTemp89 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp86, 8)) as usize] } - fTemp87)))))} - fTemp79) / (1.0 - fTemp79))) as i32;
			let mut fTemp140: F64 = if iTemp139 != 0 {1.0} else {0.5};
			let mut fTemp141: F64 = if iTemp139 != 0 {0.5} else {0.0};
			let mut fTemp142: F64 = fTemp141 + fTemp140;
			let mut fTemp143: F64 = 0.5 * fTemp142;
			let mut fTemp144: F64 = 131071.0 * (1.0 - fTemp143);
			let mut iTemp145: i32 = (fTemp144) as i32;
			let mut iTemp146: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp145, 131071)))), 917503));
			let mut fTemp147: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp146, 7)) as usize] };
			let mut fTemp148: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp146 as usize] };
			let mut fTemp149: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp146, 1)) as usize] } - fTemp148;
			let mut fTemp150: F64 = 65535.5 * fTemp142;
			let mut iTemp151: i32 = (fTemp150) as i32;
			let mut iTemp152: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp151, 131071)))), 917503));
			let mut fTemp153: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp152, 7)) as usize] };
			let mut fTemp154: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp152 as usize] };
			let mut fTemp155: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp152, 1)) as usize] } - fTemp154;
			let mut fTemp156: F64 = if iTemp63 != 0 {fTemp154 + fTemp77 * fTemp155 + (fTemp150 - (iTemp151) as F64) * (fTemp153 - (fTemp154 + fTemp77 * (fTemp155 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp152, 8)) as usize] } - fTemp153))))} else {1.0 - (fTemp148 + fTemp77 * fTemp149 + (fTemp144 - (iTemp145) as F64) * (fTemp147 - (fTemp148 + fTemp77 * (fTemp149 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp146, 8)) as usize] } - fTemp147)))))};
			let mut fTemp157: F64 = fTemp82 + fTemp143;
			let mut fTemp158: F64 = 131071.0 * (1.0 - fTemp157);
			let mut iTemp159: i32 = (fTemp158) as i32;
			let mut iTemp160: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp159, 131071)))), 917503));
			let mut fTemp161: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp160, 7)) as usize] };
			let mut fTemp162: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp160 as usize] };
			let mut fTemp163: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp160, 1)) as usize] } - fTemp162;
			let mut fTemp164: F64 = 131071.0 * fTemp157;
			let mut iTemp165: i32 = (fTemp164) as i32;
			let mut iTemp166: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp165, 131071)))), 917503));
			let mut fTemp167: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp166, 7), 917503))) as usize] };
			let mut fTemp168: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp166 as usize] };
			let mut fTemp169: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp166, 1), 917503))) as usize] } - fTemp168;
			let mut iTemp170: i32 = (fTemp138 > ((if iTemp63 != 0 {fTemp168 + fTemp77 * fTemp169 + (fTemp164 - (iTemp165) as F64) * (fTemp167 - (fTemp168 + fTemp77 * (fTemp169 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp166, 8), 917503))) as usize] } - fTemp167))))} else {1.0 - (fTemp162 + fTemp77 * fTemp163 + (fTemp158 - (iTemp159) as F64) * (fTemp161 - (fTemp162 + fTemp77 * (fTemp163 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp160, 8)) as usize] } - fTemp161)))))} - fTemp156) / (1.0 - fTemp156))) as i32;
			let mut fTemp171: F64 = if iTemp170 != 0 {fTemp140} else {fTemp143};
			let mut fTemp172: F64 = if iTemp170 != 0 {fTemp143} else {fTemp141};
			let mut fTemp173: F64 = fTemp172 + fTemp171;
			let mut fTemp174: F64 = 0.5 * fTemp173;
			let mut fTemp175: F64 = 131071.0 * (1.0 - fTemp174);
			let mut iTemp176: i32 = (fTemp175) as i32;
			let mut iTemp177: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp176, 131071)))), 917503));
			let mut fTemp178: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp177, 7)) as usize] };
			let mut fTemp179: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp177 as usize] };
			let mut fTemp180: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp177, 1)) as usize] } - fTemp179;
			let mut fTemp181: F64 = 65535.5 * fTemp173;
			let mut iTemp182: i32 = (fTemp181) as i32;
			let mut iTemp183: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp182, 131071)))), 917503));
			let mut fTemp184: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp183, 7)) as usize] };
			let mut fTemp185: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp183 as usize] };
			let mut fTemp186: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp183, 1)) as usize] } - fTemp185;
			let mut fTemp187: F64 = if iTemp63 != 0 {fTemp185 + fTemp77 * fTemp186 + (fTemp181 - (iTemp182) as F64) * (fTemp184 - (fTemp185 + fTemp77 * (fTemp186 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp183, 8)) as usize] } - fTemp184))))} else {1.0 - (fTemp179 + fTemp77 * fTemp180 + (fTemp175 - (iTemp176) as F64) * (fTemp178 - (fTemp179 + fTemp77 * (fTemp180 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp177, 8)) as usize] } - fTemp178)))))};
			let mut fTemp188: F64 = fTemp82 + fTemp174;
			let mut fTemp189: F64 = 131071.0 * (1.0 - fTemp188);
			let mut iTemp190: i32 = (fTemp189) as i32;
			let mut iTemp191: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp190, 131071)))), 917503));
			let mut fTemp192: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp191, 7)) as usize] };
			let mut fTemp193: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp191 as usize] };
			let mut fTemp194: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp191, 1)) as usize] } - fTemp193;
			let mut fTemp195: F64 = 131071.0 * fTemp188;
			let mut iTemp196: i32 = (fTemp195) as i32;
			let mut iTemp197: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp196, 131071)))), 917503));
			let mut fTemp198: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp197, 7), 917503))) as usize] };
			let mut fTemp199: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp197 as usize] };
			let mut fTemp200: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp197, 1), 917503))) as usize] } - fTemp199;
			let mut iTemp201: i32 = (fTemp138 > ((if iTemp63 != 0 {fTemp199 + fTemp77 * fTemp200 + (fTemp195 - (iTemp196) as F64) * (fTemp198 - (fTemp199 + fTemp77 * (fTemp200 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp197, 8), 917503))) as usize] } - fTemp198))))} else {1.0 - (fTemp193 + fTemp77 * fTemp194 + (fTemp189 - (iTemp190) as F64) * (fTemp192 - (fTemp193 + fTemp77 * (fTemp194 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp191, 8)) as usize] } - fTemp192)))))} - fTemp187) / (1.0 - fTemp187))) as i32;
			let mut fTemp202: F64 = if iTemp201 != 0 {fTemp171} else {fTemp174};
			let mut fTemp203: F64 = if iTemp201 != 0 {fTemp174} else {fTemp172};
			let mut fTemp204: F64 = fTemp203 + fTemp202;
			let mut fTemp205: F64 = 0.5 * fTemp204;
			let mut fTemp206: F64 = 131071.0 * (1.0 - fTemp205);
			let mut iTemp207: i32 = (fTemp206) as i32;
			let mut iTemp208: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp207, 131071)))), 917503));
			let mut fTemp209: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp208, 7)) as usize] };
			let mut fTemp210: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp208 as usize] };
			let mut fTemp211: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp208, 1)) as usize] } - fTemp210;
			let mut fTemp212: F64 = 65535.5 * fTemp204;
			let mut iTemp213: i32 = (fTemp212) as i32;
			let mut iTemp214: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp213, 131071)))), 917503));
			let mut fTemp215: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp214, 7)) as usize] };
			let mut fTemp216: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp214 as usize] };
			let mut fTemp217: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp214, 1)) as usize] } - fTemp216;
			let mut fTemp218: F64 = if iTemp63 != 0 {fTemp216 + fTemp77 * fTemp217 + (fTemp212 - (iTemp213) as F64) * (fTemp215 - (fTemp216 + fTemp77 * (fTemp217 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp214, 8)) as usize] } - fTemp215))))} else {1.0 - (fTemp210 + fTemp77 * fTemp211 + (fTemp206 - (iTemp207) as F64) * (fTemp209 - (fTemp210 + fTemp77 * (fTemp211 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp208, 8)) as usize] } - fTemp209)))))};
			let mut fTemp219: F64 = fTemp82 + fTemp205;
			let mut fTemp220: F64 = 131071.0 * (1.0 - fTemp219);
			let mut iTemp221: i32 = (fTemp220) as i32;
			let mut iTemp222: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp221, 131071)))), 917503));
			let mut fTemp223: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp222, 7)) as usize] };
			let mut fTemp224: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp222 as usize] };
			let mut fTemp225: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp222, 1)) as usize] } - fTemp224;
			let mut fTemp226: F64 = 131071.0 * fTemp219;
			let mut iTemp227: i32 = (fTemp226) as i32;
			let mut iTemp228: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp227, 131071)))), 917503));
			let mut fTemp229: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp228, 7), 917503))) as usize] };
			let mut fTemp230: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp228 as usize] };
			let mut fTemp231: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp228, 1), 917503))) as usize] } - fTemp230;
			let mut iTemp232: i32 = (fTemp138 > ((if iTemp63 != 0 {fTemp230 + fTemp77 * fTemp231 + (fTemp226 - (iTemp227) as F64) * (fTemp229 - (fTemp230 + fTemp77 * (fTemp231 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp228, 8), 917503))) as usize] } - fTemp229))))} else {1.0 - (fTemp224 + fTemp77 * fTemp225 + (fTemp220 - (iTemp221) as F64) * (fTemp223 - (fTemp224 + fTemp77 * (fTemp225 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp222, 8)) as usize] } - fTemp223)))))} - fTemp218) / (1.0 - fTemp218))) as i32;
			let mut fTemp233: F64 = if iTemp232 != 0 {fTemp202} else {fTemp205};
			let mut fTemp234: F64 = if iTemp232 != 0 {fTemp205} else {fTemp203};
			let mut fTemp235: F64 = fTemp234 + fTemp233;
			let mut fTemp236: F64 = 0.5 * fTemp235;
			let mut fTemp237: F64 = 131071.0 * (1.0 - fTemp236);
			let mut iTemp238: i32 = (fTemp237) as i32;
			let mut iTemp239: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp238, 131071)))), 917503));
			let mut fTemp240: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp239, 7)) as usize] };
			let mut fTemp241: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp239 as usize] };
			let mut fTemp242: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp239, 1)) as usize] } - fTemp241;
			let mut fTemp243: F64 = 65535.5 * fTemp235;
			let mut iTemp244: i32 = (fTemp243) as i32;
			let mut iTemp245: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp244, 131071)))), 917503));
			let mut fTemp246: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp245, 7)) as usize] };
			let mut fTemp247: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp245 as usize] };
			let mut fTemp248: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp245, 1)) as usize] } - fTemp247;
			let mut fTemp249: F64 = if iTemp63 != 0 {fTemp247 + fTemp77 * fTemp248 + (fTemp243 - (iTemp244) as F64) * (fTemp246 - (fTemp247 + fTemp77 * (fTemp248 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp245, 8)) as usize] } - fTemp246))))} else {1.0 - (fTemp241 + fTemp77 * fTemp242 + (fTemp237 - (iTemp238) as F64) * (fTemp240 - (fTemp241 + fTemp77 * (fTemp242 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp239, 8)) as usize] } - fTemp240)))))};
			let mut fTemp250: F64 = fTemp82 + fTemp236;
			let mut fTemp251: F64 = 131071.0 * (1.0 - fTemp250);
			let mut iTemp252: i32 = (fTemp251) as i32;
			let mut iTemp253: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp252, 131071)))), 917503));
			let mut fTemp254: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp253, 7)) as usize] };
			let mut fTemp255: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp253 as usize] };
			let mut fTemp256: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp253, 1)) as usize] } - fTemp255;
			let mut fTemp257: F64 = 131071.0 * fTemp250;
			let mut iTemp258: i32 = (fTemp257) as i32;
			let mut iTemp259: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp258, 131071)))), 917503));
			let mut fTemp260: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp259, 7), 917503))) as usize] };
			let mut fTemp261: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp259 as usize] };
			let mut fTemp262: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp259, 1), 917503))) as usize] } - fTemp261;
			let mut iTemp263: i32 = (fTemp138 > ((if iTemp63 != 0 {fTemp261 + fTemp77 * fTemp262 + (fTemp257 - (iTemp258) as F64) * (fTemp260 - (fTemp261 + fTemp77 * (fTemp262 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp259, 8), 917503))) as usize] } - fTemp260))))} else {1.0 - (fTemp255 + fTemp77 * fTemp256 + (fTemp251 - (iTemp252) as F64) * (fTemp254 - (fTemp255 + fTemp77 * (fTemp256 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp253, 8)) as usize] } - fTemp254)))))} - fTemp249) / (1.0 - fTemp249))) as i32;
			let mut fTemp264: F64 = if iTemp263 != 0 {fTemp233} else {fTemp236};
			let mut fTemp265: F64 = if iTemp263 != 0 {fTemp236} else {fTemp234};
			let mut fTemp266: F64 = fTemp265 + fTemp264;
			let mut fTemp267: F64 = 0.5 * fTemp266;
			let mut fTemp268: F64 = 131071.0 * (1.0 - fTemp267);
			let mut iTemp269: i32 = (fTemp268) as i32;
			let mut iTemp270: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp269, 131071)))), 917503));
			let mut fTemp271: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp270, 7)) as usize] };
			let mut fTemp272: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp270 as usize] };
			let mut fTemp273: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp270, 1)) as usize] } - fTemp272;
			let mut fTemp274: F64 = 65535.5 * fTemp266;
			let mut iTemp275: i32 = (fTemp274) as i32;
			let mut iTemp276: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp275, 131071)))), 917503));
			let mut fTemp277: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp276, 7)) as usize] };
			let mut fTemp278: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp276 as usize] };
			let mut fTemp279: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp276, 1)) as usize] } - fTemp278;
			let mut fTemp280: F64 = if iTemp63 != 0 {fTemp278 + fTemp77 * fTemp279 + (fTemp274 - (iTemp275) as F64) * (fTemp277 - (fTemp278 + fTemp77 * (fTemp279 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp276, 8)) as usize] } - fTemp277))))} else {1.0 - (fTemp272 + fTemp77 * fTemp273 + (fTemp268 - (iTemp269) as F64) * (fTemp271 - (fTemp272 + fTemp77 * (fTemp273 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp270, 8)) as usize] } - fTemp271)))))};
			let mut fTemp281: F64 = fTemp82 + fTemp267;
			let mut fTemp282: F64 = 131071.0 * (1.0 - fTemp281);
			let mut iTemp283: i32 = (fTemp282) as i32;
			let mut iTemp284: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp283, 131071)))), 917503));
			let mut fTemp285: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp284, 7)) as usize] };
			let mut fTemp286: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp284 as usize] };
			let mut fTemp287: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp284, 1)) as usize] } - fTemp286;
			let mut fTemp288: F64 = 131071.0 * fTemp281;
			let mut iTemp289: i32 = (fTemp288) as i32;
			let mut iTemp290: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp289, 131071)))), 917503));
			let mut fTemp291: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp290, 7), 917503))) as usize] };
			let mut fTemp292: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp290 as usize] };
			let mut fTemp293: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp290, 1), 917503))) as usize] } - fTemp292;
			let mut iTemp294: i32 = (fTemp138 > ((if iTemp63 != 0 {fTemp292 + fTemp77 * fTemp293 + (fTemp288 - (iTemp289) as F64) * (fTemp291 - (fTemp292 + fTemp77 * (fTemp293 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp290, 8), 917503))) as usize] } - fTemp291))))} else {1.0 - (fTemp286 + fTemp77 * fTemp287 + (fTemp282 - (iTemp283) as F64) * (fTemp285 - (fTemp286 + fTemp77 * (fTemp287 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp284, 8)) as usize] } - fTemp285)))))} - fTemp280) / (1.0 - fTemp280))) as i32;
			let mut fTemp295: F64 = if iTemp294 != 0 {fTemp264} else {fTemp267};
			let mut fTemp296: F64 = if iTemp294 != 0 {fTemp267} else {fTemp265};
			let mut fTemp297: F64 = fTemp296 + fTemp295;
			let mut fTemp298: F64 = 0.5 * fTemp297;
			let mut fTemp299: F64 = 131071.0 * (1.0 - fTemp298);
			let mut iTemp300: i32 = (fTemp299) as i32;
			let mut iTemp301: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp300, 131071)))), 917503));
			let mut fTemp302: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp301, 7)) as usize] };
			let mut fTemp303: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp301 as usize] };
			let mut fTemp304: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp301, 1)) as usize] } - fTemp303;
			let mut fTemp305: F64 = 65535.5 * fTemp297;
			let mut iTemp306: i32 = (fTemp305) as i32;
			let mut iTemp307: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp306, 131071)))), 917503));
			let mut fTemp308: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp307, 7)) as usize] };
			let mut fTemp309: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp307 as usize] };
			let mut fTemp310: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp307, 1)) as usize] } - fTemp309;
			let mut fTemp311: F64 = if iTemp63 != 0 {fTemp309 + fTemp77 * fTemp310 + (fTemp305 - (iTemp306) as F64) * (fTemp308 - (fTemp309 + fTemp77 * (fTemp310 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp307, 8)) as usize] } - fTemp308))))} else {1.0 - (fTemp303 + fTemp77 * fTemp304 + (fTemp299 - (iTemp300) as F64) * (fTemp302 - (fTemp303 + fTemp77 * (fTemp304 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp301, 8)) as usize] } - fTemp302)))))};
			let mut fTemp312: F64 = fTemp82 + fTemp298;
			let mut fTemp313: F64 = 131071.0 * (1.0 - fTemp312);
			let mut iTemp314: i32 = (fTemp313) as i32;
			let mut iTemp315: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp314, 131071)))), 917503));
			let mut fTemp316: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp315, 7)) as usize] };
			let mut fTemp317: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp315 as usize] };
			let mut fTemp318: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp315, 1)) as usize] } - fTemp317;
			let mut fTemp319: F64 = 131071.0 * fTemp312;
			let mut iTemp320: i32 = (fTemp319) as i32;
			let mut iTemp321: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp320, 131071)))), 917503));
			let mut fTemp322: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp321, 7), 917503))) as usize] };
			let mut fTemp323: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp321 as usize] };
			let mut fTemp324: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp321, 1), 917503))) as usize] } - fTemp323;
			let mut iTemp325: i32 = (fTemp138 > ((if iTemp63 != 0 {fTemp323 + fTemp77 * fTemp324 + (fTemp319 - (iTemp320) as F64) * (fTemp322 - (fTemp323 + fTemp77 * (fTemp324 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp321, 8), 917503))) as usize] } - fTemp322))))} else {1.0 - (fTemp317 + fTemp77 * fTemp318 + (fTemp313 - (iTemp314) as F64) * (fTemp316 - (fTemp317 + fTemp77 * (fTemp318 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp315, 8)) as usize] } - fTemp316)))))} - fTemp311) / (1.0 - fTemp311))) as i32;
			let mut fTemp326: F64 = if iTemp325 != 0 {fTemp295} else {fTemp298};
			let mut fTemp327: F64 = if iTemp325 != 0 {fTemp298} else {fTemp296};
			let mut fTemp328: F64 = fTemp327 + fTemp326;
			let mut fTemp329: F64 = 0.5 * fTemp328;
			let mut fTemp330: F64 = 131071.0 * (1.0 - fTemp329);
			let mut iTemp331: i32 = (fTemp330) as i32;
			let mut iTemp332: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp331, 131071)))), 917503));
			let mut fTemp333: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp332, 7)) as usize] };
			let mut fTemp334: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp332 as usize] };
			let mut fTemp335: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp332, 1)) as usize] } - fTemp334;
			let mut fTemp336: F64 = 65535.5 * fTemp328;
			let mut iTemp337: i32 = (fTemp336) as i32;
			let mut iTemp338: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp337, 131071)))), 917503));
			let mut fTemp339: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp338, 7)) as usize] };
			let mut fTemp340: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp338 as usize] };
			let mut fTemp341: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp338, 1)) as usize] } - fTemp340;
			let mut fTemp342: F64 = if iTemp63 != 0 {fTemp340 + fTemp77 * fTemp341 + (fTemp336 - (iTemp337) as F64) * (fTemp339 - (fTemp340 + fTemp77 * (fTemp341 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp338, 8)) as usize] } - fTemp339))))} else {1.0 - (fTemp334 + fTemp77 * fTemp335 + (fTemp330 - (iTemp331) as F64) * (fTemp333 - (fTemp334 + fTemp77 * (fTemp335 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp332, 8)) as usize] } - fTemp333)))))};
			let mut fTemp343: F64 = fTemp82 + fTemp329;
			let mut fTemp344: F64 = 131071.0 * (1.0 - fTemp343);
			let mut iTemp345: i32 = (fTemp344) as i32;
			let mut iTemp346: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp345, 131071)))), 917503));
			let mut fTemp347: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp346, 7)) as usize] };
			let mut fTemp348: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp346 as usize] };
			let mut fTemp349: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp346, 1)) as usize] } - fTemp348;
			let mut fTemp350: F64 = 131071.0 * fTemp343;
			let mut iTemp351: i32 = (fTemp350) as i32;
			let mut iTemp352: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp351, 131071)))), 917503));
			let mut fTemp353: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp352, 7), 917503))) as usize] };
			let mut fTemp354: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp352 as usize] };
			let mut fTemp355: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp352, 1), 917503))) as usize] } - fTemp354;
			let mut iTemp356: i32 = (fTemp138 > ((if iTemp63 != 0 {fTemp354 + fTemp77 * fTemp355 + (fTemp350 - (iTemp351) as F64) * (fTemp353 - (fTemp354 + fTemp77 * (fTemp355 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp352, 8), 917503))) as usize] } - fTemp353))))} else {1.0 - (fTemp348 + fTemp77 * fTemp349 + (fTemp344 - (iTemp345) as F64) * (fTemp347 - (fTemp348 + fTemp77 * (fTemp349 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp346, 8)) as usize] } - fTemp347)))))} - fTemp342) / (1.0 - fTemp342))) as i32;
			let mut fTemp357: F64 = if iTemp356 != 0 {fTemp326} else {fTemp329};
			let mut fTemp358: F64 = if iTemp356 != 0 {fTemp329} else {fTemp327};
			let mut fTemp359: F64 = fTemp358 + fTemp357;
			let mut fTemp360: F64 = 0.5 * fTemp359;
			let mut fTemp361: F64 = 131071.0 * (1.0 - fTemp360);
			let mut iTemp362: i32 = (fTemp361) as i32;
			let mut iTemp363: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp362, 131071)))), 917503));
			let mut fTemp364: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp363, 7)) as usize] };
			let mut fTemp365: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp363 as usize] };
			let mut fTemp366: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp363, 1)) as usize] } - fTemp365;
			let mut fTemp367: F64 = 65535.5 * fTemp359;
			let mut iTemp368: i32 = (fTemp367) as i32;
			let mut iTemp369: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp368, 131071)))), 917503));
			let mut fTemp370: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp369, 7)) as usize] };
			let mut fTemp371: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp369 as usize] };
			let mut fTemp372: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp369, 1)) as usize] } - fTemp371;
			let mut fTemp373: F64 = if iTemp63 != 0 {fTemp371 + fTemp77 * fTemp372 + (fTemp367 - (iTemp368) as F64) * (fTemp370 - (fTemp371 + fTemp77 * (fTemp372 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp369, 8)) as usize] } - fTemp370))))} else {1.0 - (fTemp365 + fTemp77 * fTemp366 + (fTemp361 - (iTemp362) as F64) * (fTemp364 - (fTemp365 + fTemp77 * (fTemp366 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp363, 8)) as usize] } - fTemp364)))))};
			let mut fTemp374: F64 = fTemp82 + fTemp360;
			let mut fTemp375: F64 = 131071.0 * (1.0 - fTemp374);
			let mut iTemp376: i32 = (fTemp375) as i32;
			let mut iTemp377: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp376, 131071)))), 917503));
			let mut fTemp378: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp377, 7)) as usize] };
			let mut fTemp379: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp377 as usize] };
			let mut fTemp380: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp377, 1)) as usize] } - fTemp379;
			let mut fTemp381: F64 = 131071.0 * fTemp374;
			let mut iTemp382: i32 = (fTemp381) as i32;
			let mut iTemp383: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp382, 131071)))), 917503));
			let mut fTemp384: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp383, 7), 917503))) as usize] };
			let mut fTemp385: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp383 as usize] };
			let mut fTemp386: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp383, 1), 917503))) as usize] } - fTemp385;
			let mut iTemp387: i32 = (fTemp138 > ((if iTemp63 != 0 {fTemp385 + fTemp77 * fTemp386 + (fTemp381 - (iTemp382) as F64) * (fTemp384 - (fTemp385 + fTemp77 * (fTemp386 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp383, 8), 917503))) as usize] } - fTemp384))))} else {1.0 - (fTemp379 + fTemp77 * fTemp380 + (fTemp375 - (iTemp376) as F64) * (fTemp378 - (fTemp379 + fTemp77 * (fTemp380 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp377, 8)) as usize] } - fTemp378)))))} - fTemp373) / (1.0 - fTemp373))) as i32;
			let mut fTemp388: F64 = if iTemp387 != 0 {fTemp357} else {fTemp360};
			let mut fTemp389: F64 = if iTemp387 != 0 {fTemp360} else {fTemp358};
			let mut fTemp390: F64 = fTemp389 + fTemp388;
			let mut fTemp391: F64 = 0.5 * fTemp390;
			let mut fTemp392: F64 = 131071.0 * (1.0 - fTemp391);
			let mut iTemp393: i32 = (fTemp392) as i32;
			let mut iTemp394: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp393, 131071)))), 917503));
			let mut fTemp395: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp394, 7)) as usize] };
			let mut fTemp396: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp394 as usize] };
			let mut fTemp397: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp394, 1)) as usize] } - fTemp396;
			let mut fTemp398: F64 = 65535.5 * fTemp390;
			let mut iTemp399: i32 = (fTemp398) as i32;
			let mut iTemp400: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp399, 131071)))), 917503));
			let mut fTemp401: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp400, 7)) as usize] };
			let mut fTemp402: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp400 as usize] };
			let mut fTemp403: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp400, 1)) as usize] } - fTemp402;
			let mut fTemp404: F64 = if iTemp63 != 0 {fTemp402 + fTemp77 * fTemp403 + (fTemp398 - (iTemp399) as F64) * (fTemp401 - (fTemp402 + fTemp77 * (fTemp403 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp400, 8)) as usize] } - fTemp401))))} else {1.0 - (fTemp396 + fTemp77 * fTemp397 + (fTemp392 - (iTemp393) as F64) * (fTemp395 - (fTemp396 + fTemp77 * (fTemp397 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp394, 8)) as usize] } - fTemp395)))))};
			let mut fTemp405: F64 = fTemp82 + fTemp391;
			let mut fTemp406: F64 = 131071.0 * (1.0 - fTemp405);
			let mut iTemp407: i32 = (fTemp406) as i32;
			let mut iTemp408: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp407, 131071)))), 917503));
			let mut fTemp409: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp408, 7)) as usize] };
			let mut fTemp410: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp408 as usize] };
			let mut fTemp411: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp408, 1)) as usize] } - fTemp410;
			let mut fTemp412: F64 = 131071.0 * fTemp405;
			let mut iTemp413: i32 = (fTemp412) as i32;
			let mut iTemp414: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp413, 131071)))), 917503));
			let mut fTemp415: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp414, 7), 917503))) as usize] };
			let mut fTemp416: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp414 as usize] };
			let mut fTemp417: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp414, 1), 917503))) as usize] } - fTemp416;
			let mut iTemp418: i32 = (fTemp138 > ((if iTemp63 != 0 {fTemp416 + fTemp77 * fTemp417 + (fTemp412 - (iTemp413) as F64) * (fTemp415 - (fTemp416 + fTemp77 * (fTemp417 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp414, 8), 917503))) as usize] } - fTemp415))))} else {1.0 - (fTemp410 + fTemp77 * fTemp411 + (fTemp406 - (iTemp407) as F64) * (fTemp409 - (fTemp410 + fTemp77 * (fTemp411 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp408, 8)) as usize] } - fTemp409)))))} - fTemp404) / (1.0 - fTemp404))) as i32;
			let mut fTemp419: F64 = if iTemp418 != 0 {fTemp388} else {fTemp391};
			let mut fTemp420: F64 = if iTemp418 != 0 {fTemp391} else {fTemp389};
			let mut fTemp421: F64 = fTemp420 + fTemp419;
			let mut fTemp422: F64 = 0.5 * fTemp421;
			let mut fTemp423: F64 = 131071.0 * (1.0 - fTemp422);
			let mut iTemp424: i32 = (fTemp423) as i32;
			let mut iTemp425: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp424, 131071)))), 917503));
			let mut fTemp426: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp425, 7)) as usize] };
			let mut fTemp427: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp425 as usize] };
			let mut fTemp428: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp425, 1)) as usize] } - fTemp427;
			let mut fTemp429: F64 = 65535.5 * fTemp421;
			let mut iTemp430: i32 = (fTemp429) as i32;
			let mut iTemp431: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp430, 131071)))), 917503));
			let mut fTemp432: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp431, 7)) as usize] };
			let mut fTemp433: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp431 as usize] };
			let mut fTemp434: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp431, 1)) as usize] } - fTemp433;
			let mut fTemp435: F64 = if iTemp63 != 0 {fTemp433 + fTemp77 * fTemp434 + (fTemp429 - (iTemp430) as F64) * (fTemp432 - (fTemp433 + fTemp77 * (fTemp434 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp431, 8)) as usize] } - fTemp432))))} else {1.0 - (fTemp427 + fTemp77 * fTemp428 + (fTemp423 - (iTemp424) as F64) * (fTemp426 - (fTemp427 + fTemp77 * (fTemp428 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp425, 8)) as usize] } - fTemp426)))))};
			let mut fTemp436: F64 = fTemp82 + fTemp422;
			let mut fTemp437: F64 = 131071.0 * (1.0 - fTemp436);
			let mut iTemp438: i32 = (fTemp437) as i32;
			let mut iTemp439: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp438, 131071)))), 917503));
			let mut fTemp440: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp439, 7)) as usize] };
			let mut fTemp441: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp439 as usize] };
			let mut fTemp442: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp439, 1)) as usize] } - fTemp441;
			let mut fTemp443: F64 = 131071.0 * fTemp436;
			let mut iTemp444: i32 = (fTemp443) as i32;
			let mut iTemp445: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp444, 131071)))), 917503));
			let mut fTemp446: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp445, 7), 917503))) as usize] };
			let mut fTemp447: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp445 as usize] };
			let mut fTemp448: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp445, 1), 917503))) as usize] } - fTemp447;
			let mut iTemp449: i32 = (fTemp138 > ((if iTemp63 != 0 {fTemp447 + fTemp77 * fTemp448 + (fTemp443 - (iTemp444) as F64) * (fTemp446 - (fTemp447 + fTemp77 * (fTemp448 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp445, 8), 917503))) as usize] } - fTemp446))))} else {1.0 - (fTemp441 + fTemp77 * fTemp442 + (fTemp437 - (iTemp438) as F64) * (fTemp440 - (fTemp441 + fTemp77 * (fTemp442 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp439, 8)) as usize] } - fTemp440)))))} - fTemp435) / (1.0 - fTemp435))) as i32;
			let mut fTemp450: F64 = if iTemp449 != 0 {fTemp419} else {fTemp422};
			let mut fTemp451: F64 = if iTemp449 != 0 {fTemp422} else {fTemp420};
			let mut fTemp452: F64 = fTemp451 + fTemp450;
			let mut fTemp453: F64 = 0.5 * fTemp452;
			let mut fTemp454: F64 = 131071.0 * (1.0 - fTemp453);
			let mut iTemp455: i32 = (fTemp454) as i32;
			let mut iTemp456: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp455, 131071)))), 917503));
			let mut fTemp457: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp456, 7)) as usize] };
			let mut fTemp458: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp456 as usize] };
			let mut fTemp459: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp456, 1)) as usize] } - fTemp458;
			let mut fTemp460: F64 = 65535.5 * fTemp452;
			let mut iTemp461: i32 = (fTemp460) as i32;
			let mut iTemp462: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp461, 131071)))), 917503));
			let mut fTemp463: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp462, 7)) as usize] };
			let mut fTemp464: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp462 as usize] };
			let mut fTemp465: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp462, 1)) as usize] } - fTemp464;
			let mut fTemp466: F64 = if iTemp63 != 0 {fTemp464 + fTemp77 * fTemp465 + (fTemp460 - (iTemp461) as F64) * (fTemp463 - (fTemp464 + fTemp77 * (fTemp465 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp462, 8)) as usize] } - fTemp463))))} else {1.0 - (fTemp458 + fTemp77 * fTemp459 + (fTemp454 - (iTemp455) as F64) * (fTemp457 - (fTemp458 + fTemp77 * (fTemp459 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp456, 8)) as usize] } - fTemp457)))))};
			let mut fTemp467: F64 = fTemp82 + fTemp453;
			let mut fTemp468: F64 = 131071.0 * (1.0 - fTemp467);
			let mut iTemp469: i32 = (fTemp468) as i32;
			let mut iTemp470: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp469, 131071)))), 917503));
			let mut fTemp471: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp470, 7)) as usize] };
			let mut fTemp472: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp470 as usize] };
			let mut fTemp473: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp470, 1)) as usize] } - fTemp472;
			let mut fTemp474: F64 = 131071.0 * fTemp467;
			let mut iTemp475: i32 = (fTemp474) as i32;
			let mut iTemp476: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp475, 131071)))), 917503));
			let mut fTemp477: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp476, 7), 917503))) as usize] };
			let mut fTemp478: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp476 as usize] };
			let mut fTemp479: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp476, 1), 917503))) as usize] } - fTemp478;
			let mut iTemp480: i32 = (fTemp138 > ((if iTemp63 != 0 {fTemp478 + fTemp77 * fTemp479 + (fTemp474 - (iTemp475) as F64) * (fTemp477 - (fTemp478 + fTemp77 * (fTemp479 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp476, 8), 917503))) as usize] } - fTemp477))))} else {1.0 - (fTemp472 + fTemp77 * fTemp473 + (fTemp468 - (iTemp469) as F64) * (fTemp471 - (fTemp472 + fTemp77 * (fTemp473 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp470, 8)) as usize] } - fTemp471)))))} - fTemp466) / (1.0 - fTemp466))) as i32;
			let mut fTemp481: F64 = if iTemp480 != 0 {fTemp450} else {fTemp453};
			let mut fTemp482: F64 = if iTemp480 != 0 {fTemp453} else {fTemp451};
			let mut fTemp483: F64 = fTemp482 + fTemp481;
			let mut fTemp484: F64 = 0.5 * fTemp483;
			let mut fTemp485: F64 = 131071.0 * (1.0 - fTemp484);
			let mut iTemp486: i32 = (fTemp485) as i32;
			let mut iTemp487: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp486, 131071)))), 917503));
			let mut fTemp488: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp487, 7)) as usize] };
			let mut fTemp489: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp487 as usize] };
			let mut fTemp490: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp487, 1)) as usize] } - fTemp489;
			let mut fTemp491: F64 = 65535.5 * fTemp483;
			let mut iTemp492: i32 = (fTemp491) as i32;
			let mut iTemp493: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp492, 131071)))), 917503));
			let mut fTemp494: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp493, 7)) as usize] };
			let mut fTemp495: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp493 as usize] };
			let mut fTemp496: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp493, 1)) as usize] } - fTemp495;
			let mut fTemp497: F64 = if iTemp63 != 0 {fTemp495 + fTemp77 * fTemp496 + (fTemp491 - (iTemp492) as F64) * (fTemp494 - (fTemp495 + fTemp77 * (fTemp496 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp493, 8)) as usize] } - fTemp494))))} else {1.0 - (fTemp489 + fTemp77 * fTemp490 + (fTemp485 - (iTemp486) as F64) * (fTemp488 - (fTemp489 + fTemp77 * (fTemp490 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp487, 8)) as usize] } - fTemp488)))))};
			let mut fTemp498: F64 = fTemp82 + fTemp484;
			let mut fTemp499: F64 = 131071.0 * (1.0 - fTemp498);
			let mut iTemp500: i32 = (fTemp499) as i32;
			let mut iTemp501: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp500, 131071)))), 917503));
			let mut fTemp502: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp501, 7)) as usize] };
			let mut fTemp503: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp501 as usize] };
			let mut fTemp504: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp501, 1)) as usize] } - fTemp503;
			let mut fTemp505: F64 = 131071.0 * fTemp498;
			let mut iTemp506: i32 = (fTemp505) as i32;
			let mut iTemp507: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp506, 131071)))), 917503));
			let mut fTemp508: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp507, 7), 917503))) as usize] };
			let mut fTemp509: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp507 as usize] };
			let mut fTemp510: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp507, 1), 917503))) as usize] } - fTemp509;
			let mut iTemp511: i32 = (fTemp138 > ((if iTemp63 != 0 {fTemp509 + fTemp77 * fTemp510 + (fTemp505 - (iTemp506) as F64) * (fTemp508 - (fTemp509 + fTemp77 * (fTemp510 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp507, 8), 917503))) as usize] } - fTemp508))))} else {1.0 - (fTemp503 + fTemp77 * fTemp504 + (fTemp499 - (iTemp500) as F64) * (fTemp502 - (fTemp503 + fTemp77 * (fTemp504 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp501, 8)) as usize] } - fTemp502)))))} - fTemp497) / (1.0 - fTemp497))) as i32;
			let mut fTemp512: F64 = if iTemp511 != 0 {fTemp481} else {fTemp484};
			let mut fTemp513: F64 = if iTemp511 != 0 {fTemp484} else {fTemp482};
			let mut fTemp514: F64 = fTemp513 + fTemp512;
			let mut fTemp515: F64 = 0.5 * fTemp514;
			let mut fTemp516: F64 = 131071.0 * (1.0 - fTemp515);
			let mut iTemp517: i32 = (fTemp516) as i32;
			let mut iTemp518: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp517, 131071)))), 917503));
			let mut fTemp519: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp518, 7)) as usize] };
			let mut fTemp520: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp518 as usize] };
			let mut fTemp521: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp518, 1)) as usize] } - fTemp520;
			let mut fTemp522: F64 = 65535.5 * fTemp514;
			let mut iTemp523: i32 = (fTemp522) as i32;
			let mut iTemp524: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp523, 131071)))), 917503));
			let mut fTemp525: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp524, 7)) as usize] };
			let mut fTemp526: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp524 as usize] };
			let mut fTemp527: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp524, 1)) as usize] } - fTemp526;
			let mut fTemp528: F64 = if iTemp63 != 0 {fTemp526 + fTemp77 * fTemp527 + (fTemp522 - (iTemp523) as F64) * (fTemp525 - (fTemp526 + fTemp77 * (fTemp527 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp524, 8)) as usize] } - fTemp525))))} else {1.0 - (fTemp520 + fTemp77 * fTemp521 + (fTemp516 - (iTemp517) as F64) * (fTemp519 - (fTemp520 + fTemp77 * (fTemp521 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp518, 8)) as usize] } - fTemp519)))))};
			let mut fTemp529: F64 = fTemp82 + fTemp515;
			let mut fTemp530: F64 = 131071.0 * (1.0 - fTemp529);
			let mut iTemp531: i32 = (fTemp530) as i32;
			let mut iTemp532: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp531, 131071)))), 917503));
			let mut fTemp533: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp532, 7)) as usize] };
			let mut fTemp534: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp532 as usize] };
			let mut fTemp535: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp532, 1)) as usize] } - fTemp534;
			let mut fTemp536: F64 = 131071.0 * fTemp529;
			let mut iTemp537: i32 = (fTemp536) as i32;
			let mut iTemp538: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp537, 131071)))), 917503));
			let mut fTemp539: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp538, 7), 917503))) as usize] };
			let mut fTemp540: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp538 as usize] };
			let mut fTemp541: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp538, 1), 917503))) as usize] } - fTemp540;
			let mut iTemp542: i32 = (fTemp138 > ((if iTemp63 != 0 {fTemp540 + fTemp77 * fTemp541 + (fTemp536 - (iTemp537) as F64) * (fTemp539 - (fTemp540 + fTemp77 * (fTemp541 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp538, 8), 917503))) as usize] } - fTemp539))))} else {1.0 - (fTemp534 + fTemp77 * fTemp535 + (fTemp530 - (iTemp531) as F64) * (fTemp533 - (fTemp534 + fTemp77 * (fTemp535 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp532, 8)) as usize] } - fTemp533)))))} - fTemp528) / (1.0 - fTemp528))) as i32;
			let mut fTemp543: F64 = if iTemp542 != 0 {fTemp512} else {fTemp515};
			let mut fTemp544: F64 = if iTemp542 != 0 {fTemp515} else {fTemp513};
			let mut fTemp545: F64 = fTemp544 + fTemp543;
			let mut fTemp546: F64 = 0.5 * fTemp545;
			let mut fTemp547: F64 = 131071.0 * (1.0 - fTemp546);
			let mut iTemp548: i32 = (fTemp547) as i32;
			let mut iTemp549: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp548, 131071)))), 917503));
			let mut fTemp550: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp549, 7)) as usize] };
			let mut fTemp551: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp549 as usize] };
			let mut fTemp552: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp549, 1)) as usize] } - fTemp551;
			let mut fTemp553: F64 = 65535.5 * fTemp545;
			let mut iTemp554: i32 = (fTemp553) as i32;
			let mut iTemp555: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp554, 131071)))), 917503));
			let mut fTemp556: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp555, 7)) as usize] };
			let mut fTemp557: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp555 as usize] };
			let mut fTemp558: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp555, 1)) as usize] } - fTemp557;
			let mut fTemp559: F64 = if iTemp63 != 0 {fTemp557 + fTemp77 * fTemp558 + (fTemp553 - (iTemp554) as F64) * (fTemp556 - (fTemp557 + fTemp77 * (fTemp558 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp555, 8)) as usize] } - fTemp556))))} else {1.0 - (fTemp551 + fTemp77 * fTemp552 + (fTemp547 - (iTemp548) as F64) * (fTemp550 - (fTemp551 + fTemp77 * (fTemp552 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp549, 8)) as usize] } - fTemp550)))))};
			let mut fTemp560: F64 = fTemp82 + fTemp546;
			let mut fTemp561: F64 = 131071.0 * (1.0 - fTemp560);
			let mut iTemp562: i32 = (fTemp561) as i32;
			let mut iTemp563: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp562, 131071)))), 917503));
			let mut fTemp564: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp563, 7)) as usize] };
			let mut fTemp565: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp563 as usize] };
			let mut fTemp566: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp563, 1)) as usize] } - fTemp565;
			let mut fTemp567: F64 = 131071.0 * fTemp560;
			let mut iTemp568: i32 = (fTemp567) as i32;
			let mut iTemp569: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp568, 131071)))), 917503));
			let mut fTemp570: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp569, 7), 917503))) as usize] };
			let mut fTemp571: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp569 as usize] };
			let mut fTemp572: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp569, 1), 917503))) as usize] } - fTemp571;
			let mut iTemp573: i32 = (fTemp138 > ((if iTemp63 != 0 {fTemp571 + fTemp77 * fTemp572 + (fTemp567 - (iTemp568) as F64) * (fTemp570 - (fTemp571 + fTemp77 * (fTemp572 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp569, 8), 917503))) as usize] } - fTemp570))))} else {1.0 - (fTemp565 + fTemp77 * fTemp566 + (fTemp561 - (iTemp562) as F64) * (fTemp564 - (fTemp565 + fTemp77 * (fTemp566 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp563, 8)) as usize] } - fTemp564)))))} - fTemp559) / (1.0 - fTemp559))) as i32;
			let mut fTemp574: F64 = if iTemp573 != 0 {fTemp543} else {fTemp546};
			let mut fTemp575: F64 = if iTemp573 != 0 {fTemp546} else {fTemp544};
			let mut fTemp576: F64 = fTemp575 + fTemp574;
			let mut fTemp577: F64 = 0.5 * fTemp576;
			let mut fTemp578: F64 = 131071.0 * (1.0 - fTemp577);
			let mut iTemp579: i32 = (fTemp578) as i32;
			let mut iTemp580: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp579, 131071)))), 917503));
			let mut fTemp581: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp580, 7)) as usize] };
			let mut fTemp582: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp580 as usize] };
			let mut fTemp583: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp580, 1)) as usize] } - fTemp582;
			let mut fTemp584: F64 = 65535.5 * fTemp576;
			let mut iTemp585: i32 = (fTemp584) as i32;
			let mut iTemp586: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp585, 131071)))), 917503));
			let mut fTemp587: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp586, 7)) as usize] };
			let mut fTemp588: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp586 as usize] };
			let mut fTemp589: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp586, 1)) as usize] } - fTemp588;
			let mut fTemp590: F64 = if iTemp63 != 0 {fTemp588 + fTemp77 * fTemp589 + (fTemp584 - (iTemp585) as F64) * (fTemp587 - (fTemp588 + fTemp77 * (fTemp589 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp586, 8)) as usize] } - fTemp587))))} else {1.0 - (fTemp582 + fTemp77 * fTemp583 + (fTemp578 - (iTemp579) as F64) * (fTemp581 - (fTemp582 + fTemp77 * (fTemp583 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp580, 8)) as usize] } - fTemp581)))))};
			let mut fTemp591: F64 = fTemp82 + fTemp577;
			let mut fTemp592: F64 = 131071.0 * (1.0 - fTemp591);
			let mut iTemp593: i32 = (fTemp592) as i32;
			let mut iTemp594: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp593, 131071)))), 917503));
			let mut fTemp595: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp594, 7)) as usize] };
			let mut fTemp596: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp594 as usize] };
			let mut fTemp597: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp594, 1)) as usize] } - fTemp596;
			let mut fTemp598: F64 = 131071.0 * fTemp591;
			let mut iTemp599: i32 = (fTemp598) as i32;
			let mut iTemp600: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp599, 131071)))), 917503));
			let mut fTemp601: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp600, 7), 917503))) as usize] };
			let mut fTemp602: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp600 as usize] };
			let mut fTemp603: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp600, 1), 917503))) as usize] } - fTemp602;
			let mut iTemp604: i32 = (fTemp138 > ((if iTemp63 != 0 {fTemp602 + fTemp77 * fTemp603 + (fTemp598 - (iTemp599) as F64) * (fTemp601 - (fTemp602 + fTemp77 * (fTemp603 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp600, 8), 917503))) as usize] } - fTemp601))))} else {1.0 - (fTemp596 + fTemp77 * fTemp597 + (fTemp592 - (iTemp593) as F64) * (fTemp595 - (fTemp596 + fTemp77 * (fTemp597 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp594, 8)) as usize] } - fTemp595)))))} - fTemp590) / (1.0 - fTemp590))) as i32;
			let mut fTemp605: F64 = if iTemp604 != 0 {fTemp574} else {fTemp577};
			let mut fTemp606: F64 = if iTemp604 != 0 {fTemp577} else {fTemp575};
			let mut fTemp607: F64 = fTemp606 + fTemp605;
			let mut fTemp608: F64 = 0.5 * fTemp607;
			let mut fTemp609: F64 = 131071.0 * (1.0 - fTemp608);
			let mut iTemp610: i32 = (fTemp609) as i32;
			let mut iTemp611: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp610, 131071)))), 917503));
			let mut fTemp612: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp611, 7)) as usize] };
			let mut fTemp613: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp611 as usize] };
			let mut fTemp614: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp611, 1)) as usize] } - fTemp613;
			let mut fTemp615: F64 = 65535.5 * fTemp607;
			let mut iTemp616: i32 = (fTemp615) as i32;
			let mut iTemp617: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp616, 131071)))), 917503));
			let mut fTemp618: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp617, 7)) as usize] };
			let mut fTemp619: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp617 as usize] };
			let mut fTemp620: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp617, 1)) as usize] } - fTemp619;
			let mut fTemp621: F64 = if iTemp63 != 0 {fTemp619 + fTemp77 * fTemp620 + (fTemp615 - (iTemp616) as F64) * (fTemp618 - (fTemp619 + fTemp77 * (fTemp620 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp617, 8), 917503))) as usize] } - fTemp618))))} else {1.0 - (fTemp613 + fTemp77 * fTemp614 + (fTemp609 - (iTemp610) as F64) * (fTemp612 - (fTemp613 + fTemp77 * (fTemp614 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp611, 8), 917503))) as usize] } - fTemp612)))))};
			let mut fTemp622: F64 = fTemp82 + fTemp608;
			let mut fTemp623: F64 = 131071.0 * (1.0 - fTemp622);
			let mut iTemp624: i32 = (fTemp623) as i32;
			let mut iTemp625: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp624, 131071)))), 917503));
			let mut fTemp626: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp625, 7)) as usize] };
			let mut fTemp627: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp625 as usize] };
			let mut fTemp628: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp625, 1)) as usize] } - fTemp627;
			let mut fTemp629: F64 = 131071.0 * fTemp622;
			let mut iTemp630: i32 = (fTemp629) as i32;
			let mut iTemp631: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp630, 131071)))), 917503));
			let mut fTemp632: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp631, 7), 917503))) as usize] };
			let mut fTemp633: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp631 as usize] };
			let mut fTemp634: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp631, 1), 917503))) as usize] } - fTemp633;
			let mut iTemp635: i32 = (fTemp138 > ((if iTemp63 != 0 {fTemp633 + fTemp77 * fTemp634 + (fTemp629 - (iTemp630) as F64) * (fTemp632 - (fTemp633 + fTemp77 * (fTemp634 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp631, 8), 917503))) as usize] } - fTemp632))))} else {1.0 - (fTemp627 + fTemp77 * fTemp628 + (fTemp623 - (iTemp624) as F64) * (fTemp626 - (fTemp627 + fTemp77 * (fTemp628 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp625, 8)) as usize] } - fTemp626)))))} - fTemp621) / (1.0 - fTemp621))) as i32;
			let mut fTemp636: F64 = F64::min(1.0, F64::max(0.0, 0.5 * (if iTemp635 != 0 {fTemp608} else {fTemp606} + if iTemp635 != 0 {fTemp605} else {fTemp608})));
			self.fRec1[0] = fTemp636;
			let mut fTemp637: F64 = 131071.0 * (1.0 - fTemp636);
			let mut iTemp638: i32 = (fTemp637) as i32;
			let mut iTemp639: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp638, 131071)))), 917503));
			let mut fTemp640: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp639, 7)) as usize] };
			let mut fTemp641: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp639 as usize] };
			let mut fTemp642: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp639, 1)) as usize] } - fTemp641;
			let mut fTemp643: F64 = 131071.0 * fTemp636;
			let mut iTemp644: i32 = (fTemp643) as i32;
			let mut iTemp645: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp644, 131071)))), 917503));
			let mut fTemp646: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp645, 7)) as usize] };
			let mut fTemp647: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp645 as usize] };
			let mut fTemp648: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp645, 1)) as usize] } - fTemp647;
			let mut fTemp649: F64 = if iTemp63 != 0 {fTemp647 + fTemp77 * fTemp648 + (fTemp643 - (iTemp644) as F64) * (fTemp646 - (fTemp647 + fTemp77 * (fTemp648 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp645, 8), 917503))) as usize] } - fTemp646))))} else {1.0 - (fTemp641 + fTemp77 * fTemp642 + (fTemp637 - (iTemp638) as F64) * (fTemp640 - (fTemp641 + fTemp77 * (fTemp642 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, 8), 917503))) as usize] } - fTemp640)))))};
			let mut fTemp650: F64 = fTemp82 + fTemp636;
			let mut fTemp651: F64 = 131071.0 * (1.0 - fTemp650);
			let mut iTemp652: i32 = (fTemp651) as i32;
			let mut iTemp653: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp652, 131071)))), 917503));
			let mut fTemp654: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp653, 7)) as usize] };
			let mut fTemp655: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp653 as usize] };
			let mut fTemp656: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp653, 1)) as usize] } - fTemp655;
			let mut fTemp657: F64 = 131071.0 * fTemp650;
			let mut iTemp658: i32 = (fTemp657) as i32;
			let mut iTemp659: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp658, 131071)))), 917503));
			let mut fTemp660: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, 7), 917503))) as usize] };
			let mut fTemp661: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp659 as usize] };
			let mut fTemp662: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, 1), 917503))) as usize] } - fTemp661;
			let mut fTemp663: F64 = self.fRec2[1] + if ((0.001 * fTemp81) == 0.0) as i32 != 0 {fTemp62} else {fTemp62 * (if iTemp63 != 0 {fTemp661 + fTemp77 * fTemp662 + (fTemp657 - (iTemp658) as F64) * (fTemp660 - (fTemp661 + fTemp77 * (fTemp662 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, 8), 917503))) as usize] } - fTemp660))))} else {1.0 - (fTemp655 + fTemp77 * fTemp656 + (fTemp651 - (iTemp652) as F64) * (fTemp654 - (fTemp655 + fTemp77 * (fTemp656 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp653, 8)) as usize] } - fTemp654)))))} - fTemp649) / (1.0 - fTemp649)};
			self.fRec2[0] = if iTemp80 != 0 {F64::min(fTemp663, self.fRec2[1])} else {F64::max(fTemp663, self.fRec2[1])};
			self.fVec31[(self.IOTA0 & 8191) as usize] = F64::powf(1e+01, 0.05 * self.fRec2[0]);
			let mut fTemp664: F64 = self.fVec31[((i32::wrapping_sub(self.IOTA0, iSlow70)) & 8191) as usize];
			self.fRec14[0] = fSlow72 + self.fConst4 * self.fRec14[1];
			*output0 = 0.5 * self.fVec1[((i32::wrapping_sub(self.IOTA0, iSlow71)) & 32767) as usize] * fTemp2 + self.fRec14[0] * self.fVec2[((i32::wrapping_sub(self.IOTA0, iSlow71)) & 32767) as usize] * fTemp664 * fTemp4;
			let mut fTemp665: F64 = fTemp36 + fSlow17 * (fTemp37 - fTemp36);
			let mut iTemp666: i32 = ((fTemp665 > fSlow11) as i32) + ((fTemp665 > fSlow9) as i32);
			let mut fTemp667: F64 = fTemp665 - fSlow8;
			let mut fTemp668: F64 = F64::min(fTemp34, -(fSlow18 * F64::max(0.0, if (iTemp666 == 0) as i32 != 0 {0.0} else {if (iTemp666 == 1) as i32 != 0 {fSlow12 * LambRs48k_faustpower2_f(fSlow7 + fTemp667)} else {fTemp667}})));
			self.fVec32[(self.IOTA0 & 16383) as usize] = fTemp668;
			let mut fTemp669: F64 = F64::min(fTemp668, self.fVec32[((i32::wrapping_sub(self.IOTA0, 1)) & 16383) as usize]);
			self.fVec33[0] = fTemp669;
			let mut fTemp670: F64 = F64::min(fTemp669, self.fVec33[2]);
			self.fVec34[0] = fTemp670;
			let mut fTemp671: F64 = F64::min(fTemp670, self.fVec34[4]);
			self.fVec35[0] = fTemp671;
			let mut fTemp672: F64 = F64::min(fTemp671, self.fVec35[8]);
			self.fVec36[(self.IOTA0 & 31) as usize] = fTemp672;
			let mut fTemp673: F64 = F64::min(fTemp672, self.fVec36[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize]);
			self.fVec37[(self.IOTA0 & 63) as usize] = fTemp673;
			let mut fTemp674: F64 = F64::min(fTemp673, self.fVec37[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize]);
			self.fVec38[(self.IOTA0 & 127) as usize] = fTemp674;
			let mut fTemp675: F64 = F64::min(fTemp674, self.fVec38[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize]);
			self.fVec39[(self.IOTA0 & 255) as usize] = fTemp675;
			let mut fTemp676: F64 = F64::min(fTemp675, self.fVec39[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize]);
			self.fVec40[(self.IOTA0 & 511) as usize] = fTemp676;
			let mut fTemp677: F64 = F64::min(fTemp676, self.fVec40[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize]);
			self.fVec41[(self.IOTA0 & 1023) as usize] = fTemp677;
			let mut fTemp678: F64 = F64::min(fTemp677, self.fVec41[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize]);
			self.fVec42[(self.IOTA0 & 2047) as usize] = fTemp678;
			self.fVec43[(self.IOTA0 & 4095) as usize] = F64::min(fTemp678, self.fVec42[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			self.fRec17[0] = F64::max(F64::min(self.fRec17[1], self.fVec32[((i32::wrapping_sub(self.IOTA0, iSlow21)) & 16383) as usize]), F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow23 != 0 {fTemp668} else {1.7976931348623157e+308}, if iSlow24 != 0 {self.fVec33[iSlow23 as usize]} else {1.7976931348623157e+308}), if iSlow25 != 0 {self.fVec34[iSlow26 as usize]} else {1.7976931348623157e+308}), if iSlow27 != 0 {self.fVec35[iSlow28 as usize]} else {1.7976931348623157e+308}), if iSlow29 != 0 {self.fVec36[((i32::wrapping_sub(self.IOTA0, iSlow30)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow31 != 0 {self.fVec37[((i32::wrapping_sub(self.IOTA0, iSlow32)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow33 != 0 {self.fVec38[((i32::wrapping_sub(self.IOTA0, iSlow34)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow35 != 0 {self.fVec39[((i32::wrapping_sub(self.IOTA0, iSlow36)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow37 != 0 {self.fVec40[((i32::wrapping_sub(self.IOTA0, iSlow38)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow39 != 0 {self.fVec41[((i32::wrapping_sub(self.IOTA0, iSlow40)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow41 != 0 {self.fVec42[((i32::wrapping_sub(self.IOTA0, iSlow42)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow43 != 0 {self.fVec43[((i32::wrapping_sub(self.IOTA0, iSlow44)) & 4095) as usize]} else {1.7976931348623157e+308}));
			let mut fTemp679: F64 = F64::min(self.fRec17[0], self.fRec17[1]);
			self.fVec44[0] = fTemp679;
			let mut fTemp680: F64 = F64::min(fTemp679, self.fVec44[2]);
			self.fVec45[0] = fTemp680;
			let mut fTemp681: F64 = F64::min(fTemp680, self.fVec45[4]);
			self.fVec46[0] = fTemp681;
			let mut fTemp682: F64 = F64::min(fTemp681, self.fVec46[8]);
			self.fVec47[(self.IOTA0 & 31) as usize] = fTemp682;
			let mut fTemp683: F64 = F64::min(fTemp682, self.fVec47[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize]);
			self.fVec48[(self.IOTA0 & 63) as usize] = fTemp683;
			let mut fTemp684: F64 = F64::min(fTemp683, self.fVec48[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize]);
			self.fVec49[(self.IOTA0 & 127) as usize] = fTemp684;
			let mut fTemp685: F64 = F64::min(fTemp684, self.fVec49[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize]);
			self.fVec50[(self.IOTA0 & 255) as usize] = fTemp685;
			let mut fTemp686: F64 = F64::min(fTemp685, self.fVec50[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize]);
			self.fVec51[(self.IOTA0 & 511) as usize] = fTemp686;
			let mut fTemp687: F64 = F64::min(fTemp686, self.fVec51[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize]);
			self.fVec52[(self.IOTA0 & 1023) as usize] = fTemp687;
			let mut fTemp688: F64 = F64::min(fTemp687, self.fVec52[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize]);
			self.fVec53[(self.IOTA0 & 2047) as usize] = fTemp688;
			self.fVec54[(self.IOTA0 & 4095) as usize] = F64::min(fTemp688, self.fVec53[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			let mut fTemp689: F64 = F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow4 != 0 {self.fRec17[0]} else {1.7976931348623157e+308}, if iSlow45 != 0 {self.fVec44[iSlow4 as usize]} else {1.7976931348623157e+308}), if iSlow46 != 0 {self.fVec45[iSlow47 as usize]} else {1.7976931348623157e+308}), if iSlow48 != 0 {self.fVec46[iSlow49 as usize]} else {1.7976931348623157e+308}), if iSlow50 != 0 {self.fVec47[((i32::wrapping_sub(self.IOTA0, iSlow51)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow52 != 0 {self.fVec48[((i32::wrapping_sub(self.IOTA0, iSlow53)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow54 != 0 {self.fVec49[((i32::wrapping_sub(self.IOTA0, iSlow55)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow56 != 0 {self.fVec50[((i32::wrapping_sub(self.IOTA0, iSlow57)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow58 != 0 {self.fVec51[((i32::wrapping_sub(self.IOTA0, iSlow59)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow60 != 0 {self.fVec52[((i32::wrapping_sub(self.IOTA0, iSlow61)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow62 != 0 {self.fVec53[((i32::wrapping_sub(self.IOTA0, iSlow63)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow64 != 0 {self.fVec54[((i32::wrapping_sub(self.IOTA0, iSlow65)) & 4095) as usize]} else {1.7976931348623157e+308}) - self.fRec16[1];
			self.fVec55[0] = fTemp689;
			let mut iTemp690: i32 = (fTemp689 > 0.0) as i32;
			let mut fTemp691: F64 = if iTemp690 != 0 {fSlow67} else {fSlow66};
			self.fVec56[0] = fTemp691;
			let mut fTemp692: F64 = 6.0 * fTemp691;
			let mut iTemp693: i32 = (fTemp692) as i32;
			let mut iTemp694: i32 = std::cmp::max(0, std::cmp::min(iTemp693, 6));
			let mut iTemp695: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, 458745), 917503));
			let mut fTemp696: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp695, 7)) as usize] };
			let mut fTemp697: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp695 as usize] };
			let mut fTemp698: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp695, 1)) as usize] } - fTemp697;
			let mut fTemp699: F64 = fTemp692 - (iTemp693) as F64;
			let mut fTemp700: F64 = fTemp697 + fTemp699 * fTemp698 + 0.5 * (fTemp696 - (fTemp697 + fTemp699 * (fTemp698 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp695, 8)) as usize] } - fTemp696))));
			let mut fTemp701: F64 = if iTemp690 != 0 {fTemp700} else {1.0 - fTemp700};
			let mut iTemp702: i32 = (fTemp689 < 0.0) as i32;
			let mut fTemp703: F64 = fSlow1 * (iTemp702) as F64 + fSlow13 * (iTemp690) as F64;
			self.fVec57[0] = fTemp703;
			let mut fTemp704: F64 = self.fConst10 / fTemp703;
			let mut fTemp705: F64 = fTemp704 + 0.5;
			let mut fTemp706: F64 = 131071.0 * (1.0 - fTemp705);
			let mut iTemp707: i32 = (fTemp706) as i32;
			let mut iTemp708: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp707, 131071)))), 917503));
			let mut fTemp709: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp708, 7)) as usize] };
			let mut fTemp710: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp708 as usize] };
			let mut fTemp711: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp708, 1)) as usize] } - fTemp710;
			let mut fTemp712: F64 = 131071.0 * fTemp705;
			let mut iTemp713: i32 = (fTemp712) as i32;
			let mut iTemp714: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp713, 131071)))), 917503));
			let mut fTemp715: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp714, 7), 917503))) as usize] };
			let mut fTemp716: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp714 as usize] };
			let mut fTemp717: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp714, 1), 917503))) as usize] } - fTemp716;
			let mut fTemp718: F64 = 6.0 * self.fVec56[1];
			let mut iTemp719: i32 = (fTemp718) as i32;
			let mut iTemp720: i32 = std::cmp::max(0, std::cmp::min(iTemp719, 6));
			let mut fTemp721: F64 = 131071.0 * (1.0 - self.fRec15[1]);
			let mut iTemp722: i32 = (fTemp721) as i32;
			let mut iTemp723: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp722, 131071))), iTemp720), 917503));
			let mut fTemp724: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp723, 7), 917503))) as usize] };
			let mut fTemp725: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp723 as usize] };
			let mut fTemp726: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp723, 1), 917503))) as usize] } - fTemp725;
			let mut fTemp727: F64 = fTemp718 - (iTemp719) as F64;
			let mut fTemp728: F64 = 131071.0 * self.fRec15[1];
			let mut iTemp729: i32 = (fTemp728) as i32;
			let mut iTemp730: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp720, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp729, 131071)))), 917503));
			let mut fTemp731: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp730, 7), 917503))) as usize] };
			let mut fTemp732: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp730 as usize] };
			let mut fTemp733: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp730, 1), 917503))) as usize] } - fTemp732;
			let mut fTemp734: F64 = self.fRec15[1] + fTemp704;
			let mut fTemp735: F64 = 131071.0 * (1.0 - fTemp734);
			let mut iTemp736: i32 = (fTemp735) as i32;
			let mut iTemp737: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp736, 131071)))), 917503));
			let mut fTemp738: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp737, 7)) as usize] };
			let mut fTemp739: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp737 as usize] };
			let mut fTemp740: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp737, 1)) as usize] } - fTemp739;
			let mut fTemp741: F64 = 131071.0 * fTemp734;
			let mut iTemp742: i32 = (fTemp741) as i32;
			let mut iTemp743: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp742, 131071)))), 917503));
			let mut fTemp744: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp743, 7), 917503))) as usize] };
			let mut fTemp745: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp743 as usize] };
			let mut fTemp746: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp743, 1), 917503))) as usize] } - fTemp745;
			let mut fTemp747: F64 = self.fRec15[1] + self.fConst10 * (1.0 / fTemp703 + 1.0 / self.fVec57[1]);
			let mut fTemp748: F64 = 131071.0 * (1.0 - fTemp747);
			let mut iTemp749: i32 = (fTemp748) as i32;
			let mut iTemp750: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp749, 131071))), iTemp694), 917503));
			let mut fTemp751: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp750, 7)) as usize] };
			let mut fTemp752: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp750 as usize] };
			let mut fTemp753: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp750, 1)) as usize] } - fTemp752;
			let mut fTemp754: F64 = 131071.0 * fTemp747;
			let mut iTemp755: i32 = (fTemp754) as i32;
			let mut iTemp756: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp755, 131071)))), 917503));
			let mut fTemp757: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp756, 7), 917503))) as usize] };
			let mut fTemp758: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp756 as usize] };
			let mut fTemp759: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp756, 1), 917503))) as usize] } - fTemp758;
			let mut fTemp760: F64 = (if iTemp690 != 0 {fTemp758 + fTemp699 * fTemp759 + (fTemp754 - (iTemp755) as F64) * (fTemp757 - (fTemp758 + fTemp699 * (fTemp759 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp756, 8), 917503))) as usize] } - fTemp757))))} else {1.0 - (fTemp752 + fTemp699 * fTemp753 + (fTemp748 - (iTemp749) as F64) * (fTemp751 - (fTemp752 + fTemp699 * (fTemp753 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp750, 8)) as usize] } - fTemp751)))))} - if iTemp690 != 0 {fTemp745 + fTemp699 * fTemp746 + (fTemp741 - (iTemp742) as F64) * (fTemp744 - (fTemp745 + fTemp699 * (fTemp746 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp743, 8), 917503))) as usize] } - fTemp744))))} else {1.0 - (fTemp739 + fTemp699 * fTemp740 + (fTemp735 - (iTemp736) as F64) * (fTemp738 - (fTemp739 + fTemp699 * (fTemp740 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp737, 8)) as usize] } - fTemp738)))))}) * self.fVec55[1] / (fTemp689 * (1.0 - if iTemp690 != 0 {fTemp732 + fTemp727 * fTemp733 + (fTemp728 - (iTemp729) as F64) * (fTemp731 - (fTemp732 + fTemp727 * (fTemp733 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp730, 8), 917503))) as usize] } - fTemp731))))} else {1.0 - (fTemp725 + fTemp727 * fTemp726 + (fTemp721 - (iTemp722) as F64) * (fTemp724 - (fTemp725 + fTemp727 * (fTemp726 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp723, 8), 917503))) as usize] } - fTemp724)))))}));
			let mut iTemp761: i32 = (fTemp760 > ((if iTemp690 != 0 {fTemp716 + fTemp699 * fTemp717 + (fTemp712 - (iTemp713) as F64) * (fTemp715 - (fTemp716 + fTemp699 * (fTemp717 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp714, 8), 917503))) as usize] } - fTemp715))))} else {1.0 - (fTemp710 + fTemp699 * fTemp711 + (fTemp706 - (iTemp707) as F64) * (fTemp709 - (fTemp710 + fTemp699 * (fTemp711 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp708, 8)) as usize] } - fTemp709)))))} - fTemp701) / (1.0 - fTemp701))) as i32;
			let mut fTemp762: F64 = if iTemp761 != 0 {1.0} else {0.5};
			let mut fTemp763: F64 = if iTemp761 != 0 {0.5} else {0.0};
			let mut fTemp764: F64 = fTemp763 + fTemp762;
			let mut fTemp765: F64 = 0.5 * fTemp764;
			let mut fTemp766: F64 = 131071.0 * (1.0 - fTemp765);
			let mut iTemp767: i32 = (fTemp766) as i32;
			let mut iTemp768: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp767, 131071)))), 917503));
			let mut fTemp769: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp768, 7)) as usize] };
			let mut fTemp770: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp768 as usize] };
			let mut fTemp771: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp768, 1)) as usize] } - fTemp770;
			let mut fTemp772: F64 = 65535.5 * fTemp764;
			let mut iTemp773: i32 = (fTemp772) as i32;
			let mut iTemp774: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp773, 131071)))), 917503));
			let mut fTemp775: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp774, 7)) as usize] };
			let mut fTemp776: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp774 as usize] };
			let mut fTemp777: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp774, 1)) as usize] } - fTemp776;
			let mut fTemp778: F64 = if iTemp690 != 0 {fTemp776 + fTemp699 * fTemp777 + (fTemp772 - (iTemp773) as F64) * (fTemp775 - (fTemp776 + fTemp699 * (fTemp777 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp774, 8)) as usize] } - fTemp775))))} else {1.0 - (fTemp770 + fTemp699 * fTemp771 + (fTemp766 - (iTemp767) as F64) * (fTemp769 - (fTemp770 + fTemp699 * (fTemp771 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp768, 8)) as usize] } - fTemp769)))))};
			let mut fTemp779: F64 = fTemp704 + fTemp765;
			let mut fTemp780: F64 = 131071.0 * (1.0 - fTemp779);
			let mut iTemp781: i32 = (fTemp780) as i32;
			let mut iTemp782: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp781, 131071)))), 917503));
			let mut fTemp783: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp782, 7)) as usize] };
			let mut fTemp784: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp782 as usize] };
			let mut fTemp785: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp782, 1)) as usize] } - fTemp784;
			let mut fTemp786: F64 = 131071.0 * fTemp779;
			let mut iTemp787: i32 = (fTemp786) as i32;
			let mut iTemp788: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp787, 131071)))), 917503));
			let mut fTemp789: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp788, 7), 917503))) as usize] };
			let mut fTemp790: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp788 as usize] };
			let mut fTemp791: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp788, 1), 917503))) as usize] } - fTemp790;
			let mut iTemp792: i32 = (fTemp760 > ((if iTemp690 != 0 {fTemp790 + fTemp699 * fTemp791 + (fTemp786 - (iTemp787) as F64) * (fTemp789 - (fTemp790 + fTemp699 * (fTemp791 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp788, 8), 917503))) as usize] } - fTemp789))))} else {1.0 - (fTemp784 + fTemp699 * fTemp785 + (fTemp780 - (iTemp781) as F64) * (fTemp783 - (fTemp784 + fTemp699 * (fTemp785 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp782, 8)) as usize] } - fTemp783)))))} - fTemp778) / (1.0 - fTemp778))) as i32;
			let mut fTemp793: F64 = if iTemp792 != 0 {fTemp762} else {fTemp765};
			let mut fTemp794: F64 = if iTemp792 != 0 {fTemp765} else {fTemp763};
			let mut fTemp795: F64 = fTemp794 + fTemp793;
			let mut fTemp796: F64 = 0.5 * fTemp795;
			let mut fTemp797: F64 = 131071.0 * (1.0 - fTemp796);
			let mut iTemp798: i32 = (fTemp797) as i32;
			let mut iTemp799: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp798, 131071)))), 917503));
			let mut fTemp800: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp799, 7)) as usize] };
			let mut fTemp801: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp799 as usize] };
			let mut fTemp802: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp799, 1)) as usize] } - fTemp801;
			let mut fTemp803: F64 = 65535.5 * fTemp795;
			let mut iTemp804: i32 = (fTemp803) as i32;
			let mut iTemp805: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp804, 131071)))), 917503));
			let mut fTemp806: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp805, 7)) as usize] };
			let mut fTemp807: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp805 as usize] };
			let mut fTemp808: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp805, 1)) as usize] } - fTemp807;
			let mut fTemp809: F64 = if iTemp690 != 0 {fTemp807 + fTemp699 * fTemp808 + (fTemp803 - (iTemp804) as F64) * (fTemp806 - (fTemp807 + fTemp699 * (fTemp808 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp805, 8)) as usize] } - fTemp806))))} else {1.0 - (fTemp801 + fTemp699 * fTemp802 + (fTemp797 - (iTemp798) as F64) * (fTemp800 - (fTemp801 + fTemp699 * (fTemp802 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp799, 8)) as usize] } - fTemp800)))))};
			let mut fTemp810: F64 = fTemp704 + fTemp796;
			let mut fTemp811: F64 = 131071.0 * (1.0 - fTemp810);
			let mut iTemp812: i32 = (fTemp811) as i32;
			let mut iTemp813: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp812, 131071)))), 917503));
			let mut fTemp814: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp813, 7)) as usize] };
			let mut fTemp815: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp813 as usize] };
			let mut fTemp816: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp813, 1)) as usize] } - fTemp815;
			let mut fTemp817: F64 = 131071.0 * fTemp810;
			let mut iTemp818: i32 = (fTemp817) as i32;
			let mut iTemp819: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp818, 131071)))), 917503));
			let mut fTemp820: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp819, 7), 917503))) as usize] };
			let mut fTemp821: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp819 as usize] };
			let mut fTemp822: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp819, 1), 917503))) as usize] } - fTemp821;
			let mut iTemp823: i32 = (fTemp760 > ((if iTemp690 != 0 {fTemp821 + fTemp699 * fTemp822 + (fTemp817 - (iTemp818) as F64) * (fTemp820 - (fTemp821 + fTemp699 * (fTemp822 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp819, 8), 917503))) as usize] } - fTemp820))))} else {1.0 - (fTemp815 + fTemp699 * fTemp816 + (fTemp811 - (iTemp812) as F64) * (fTemp814 - (fTemp815 + fTemp699 * (fTemp816 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp813, 8)) as usize] } - fTemp814)))))} - fTemp809) / (1.0 - fTemp809))) as i32;
			let mut fTemp824: F64 = if iTemp823 != 0 {fTemp793} else {fTemp796};
			let mut fTemp825: F64 = if iTemp823 != 0 {fTemp796} else {fTemp794};
			let mut fTemp826: F64 = fTemp825 + fTemp824;
			let mut fTemp827: F64 = 0.5 * fTemp826;
			let mut fTemp828: F64 = 131071.0 * (1.0 - fTemp827);
			let mut iTemp829: i32 = (fTemp828) as i32;
			let mut iTemp830: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp829, 131071)))), 917503));
			let mut fTemp831: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp830, 7)) as usize] };
			let mut fTemp832: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp830 as usize] };
			let mut fTemp833: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp830, 1)) as usize] } - fTemp832;
			let mut fTemp834: F64 = 65535.5 * fTemp826;
			let mut iTemp835: i32 = (fTemp834) as i32;
			let mut iTemp836: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp835, 131071)))), 917503));
			let mut fTemp837: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp836, 7)) as usize] };
			let mut fTemp838: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp836 as usize] };
			let mut fTemp839: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp836, 1)) as usize] } - fTemp838;
			let mut fTemp840: F64 = if iTemp690 != 0 {fTemp838 + fTemp699 * fTemp839 + (fTemp834 - (iTemp835) as F64) * (fTemp837 - (fTemp838 + fTemp699 * (fTemp839 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp836, 8)) as usize] } - fTemp837))))} else {1.0 - (fTemp832 + fTemp699 * fTemp833 + (fTemp828 - (iTemp829) as F64) * (fTemp831 - (fTemp832 + fTemp699 * (fTemp833 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp830, 8)) as usize] } - fTemp831)))))};
			let mut fTemp841: F64 = fTemp704 + fTemp827;
			let mut fTemp842: F64 = 131071.0 * (1.0 - fTemp841);
			let mut iTemp843: i32 = (fTemp842) as i32;
			let mut iTemp844: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp843, 131071)))), 917503));
			let mut fTemp845: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp844, 7)) as usize] };
			let mut fTemp846: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp844 as usize] };
			let mut fTemp847: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp844, 1)) as usize] } - fTemp846;
			let mut fTemp848: F64 = 131071.0 * fTemp841;
			let mut iTemp849: i32 = (fTemp848) as i32;
			let mut iTemp850: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp849, 131071)))), 917503));
			let mut fTemp851: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp850, 7), 917503))) as usize] };
			let mut fTemp852: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp850 as usize] };
			let mut fTemp853: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp850, 1), 917503))) as usize] } - fTemp852;
			let mut iTemp854: i32 = (fTemp760 > ((if iTemp690 != 0 {fTemp852 + fTemp699 * fTemp853 + (fTemp848 - (iTemp849) as F64) * (fTemp851 - (fTemp852 + fTemp699 * (fTemp853 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp850, 8), 917503))) as usize] } - fTemp851))))} else {1.0 - (fTemp846 + fTemp699 * fTemp847 + (fTemp842 - (iTemp843) as F64) * (fTemp845 - (fTemp846 + fTemp699 * (fTemp847 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp844, 8)) as usize] } - fTemp845)))))} - fTemp840) / (1.0 - fTemp840))) as i32;
			let mut fTemp855: F64 = if iTemp854 != 0 {fTemp824} else {fTemp827};
			let mut fTemp856: F64 = if iTemp854 != 0 {fTemp827} else {fTemp825};
			let mut fTemp857: F64 = fTemp856 + fTemp855;
			let mut fTemp858: F64 = 0.5 * fTemp857;
			let mut fTemp859: F64 = 131071.0 * (1.0 - fTemp858);
			let mut iTemp860: i32 = (fTemp859) as i32;
			let mut iTemp861: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp860, 131071)))), 917503));
			let mut fTemp862: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp861, 7)) as usize] };
			let mut fTemp863: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp861 as usize] };
			let mut fTemp864: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp861, 1)) as usize] } - fTemp863;
			let mut fTemp865: F64 = 65535.5 * fTemp857;
			let mut iTemp866: i32 = (fTemp865) as i32;
			let mut iTemp867: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp866, 131071)))), 917503));
			let mut fTemp868: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp867, 7)) as usize] };
			let mut fTemp869: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp867 as usize] };
			let mut fTemp870: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp867, 1)) as usize] } - fTemp869;
			let mut fTemp871: F64 = if iTemp690 != 0 {fTemp869 + fTemp699 * fTemp870 + (fTemp865 - (iTemp866) as F64) * (fTemp868 - (fTemp869 + fTemp699 * (fTemp870 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp867, 8)) as usize] } - fTemp868))))} else {1.0 - (fTemp863 + fTemp699 * fTemp864 + (fTemp859 - (iTemp860) as F64) * (fTemp862 - (fTemp863 + fTemp699 * (fTemp864 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp861, 8)) as usize] } - fTemp862)))))};
			let mut fTemp872: F64 = fTemp704 + fTemp858;
			let mut fTemp873: F64 = 131071.0 * (1.0 - fTemp872);
			let mut iTemp874: i32 = (fTemp873) as i32;
			let mut iTemp875: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp874, 131071)))), 917503));
			let mut fTemp876: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp875, 7)) as usize] };
			let mut fTemp877: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp875 as usize] };
			let mut fTemp878: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp875, 1)) as usize] } - fTemp877;
			let mut fTemp879: F64 = 131071.0 * fTemp872;
			let mut iTemp880: i32 = (fTemp879) as i32;
			let mut iTemp881: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp880, 131071)))), 917503));
			let mut fTemp882: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp881, 7), 917503))) as usize] };
			let mut fTemp883: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp881 as usize] };
			let mut fTemp884: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp881, 1), 917503))) as usize] } - fTemp883;
			let mut iTemp885: i32 = (fTemp760 > ((if iTemp690 != 0 {fTemp883 + fTemp699 * fTemp884 + (fTemp879 - (iTemp880) as F64) * (fTemp882 - (fTemp883 + fTemp699 * (fTemp884 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp881, 8), 917503))) as usize] } - fTemp882))))} else {1.0 - (fTemp877 + fTemp699 * fTemp878 + (fTemp873 - (iTemp874) as F64) * (fTemp876 - (fTemp877 + fTemp699 * (fTemp878 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp875, 8)) as usize] } - fTemp876)))))} - fTemp871) / (1.0 - fTemp871))) as i32;
			let mut fTemp886: F64 = if iTemp885 != 0 {fTemp855} else {fTemp858};
			let mut fTemp887: F64 = if iTemp885 != 0 {fTemp858} else {fTemp856};
			let mut fTemp888: F64 = fTemp887 + fTemp886;
			let mut fTemp889: F64 = 0.5 * fTemp888;
			let mut fTemp890: F64 = 131071.0 * (1.0 - fTemp889);
			let mut iTemp891: i32 = (fTemp890) as i32;
			let mut iTemp892: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp891, 131071)))), 917503));
			let mut fTemp893: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp892, 7)) as usize] };
			let mut fTemp894: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp892 as usize] };
			let mut fTemp895: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp892, 1)) as usize] } - fTemp894;
			let mut fTemp896: F64 = 65535.5 * fTemp888;
			let mut iTemp897: i32 = (fTemp896) as i32;
			let mut iTemp898: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp897, 131071)))), 917503));
			let mut fTemp899: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp898, 7)) as usize] };
			let mut fTemp900: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp898 as usize] };
			let mut fTemp901: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp898, 1)) as usize] } - fTemp900;
			let mut fTemp902: F64 = if iTemp690 != 0 {fTemp900 + fTemp699 * fTemp901 + (fTemp896 - (iTemp897) as F64) * (fTemp899 - (fTemp900 + fTemp699 * (fTemp901 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp898, 8)) as usize] } - fTemp899))))} else {1.0 - (fTemp894 + fTemp699 * fTemp895 + (fTemp890 - (iTemp891) as F64) * (fTemp893 - (fTemp894 + fTemp699 * (fTemp895 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp892, 8)) as usize] } - fTemp893)))))};
			let mut fTemp903: F64 = fTemp704 + fTemp889;
			let mut fTemp904: F64 = 131071.0 * (1.0 - fTemp903);
			let mut iTemp905: i32 = (fTemp904) as i32;
			let mut iTemp906: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp905, 131071)))), 917503));
			let mut fTemp907: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp906, 7)) as usize] };
			let mut fTemp908: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp906 as usize] };
			let mut fTemp909: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp906, 1)) as usize] } - fTemp908;
			let mut fTemp910: F64 = 131071.0 * fTemp903;
			let mut iTemp911: i32 = (fTemp910) as i32;
			let mut iTemp912: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp911, 131071)))), 917503));
			let mut fTemp913: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp912, 7), 917503))) as usize] };
			let mut fTemp914: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp912 as usize] };
			let mut fTemp915: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp912, 1), 917503))) as usize] } - fTemp914;
			let mut iTemp916: i32 = (fTemp760 > ((if iTemp690 != 0 {fTemp914 + fTemp699 * fTemp915 + (fTemp910 - (iTemp911) as F64) * (fTemp913 - (fTemp914 + fTemp699 * (fTemp915 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp912, 8), 917503))) as usize] } - fTemp913))))} else {1.0 - (fTemp908 + fTemp699 * fTemp909 + (fTemp904 - (iTemp905) as F64) * (fTemp907 - (fTemp908 + fTemp699 * (fTemp909 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp906, 8)) as usize] } - fTemp907)))))} - fTemp902) / (1.0 - fTemp902))) as i32;
			let mut fTemp917: F64 = if iTemp916 != 0 {fTemp886} else {fTemp889};
			let mut fTemp918: F64 = if iTemp916 != 0 {fTemp889} else {fTemp887};
			let mut fTemp919: F64 = fTemp918 + fTemp917;
			let mut fTemp920: F64 = 0.5 * fTemp919;
			let mut fTemp921: F64 = 131071.0 * (1.0 - fTemp920);
			let mut iTemp922: i32 = (fTemp921) as i32;
			let mut iTemp923: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp922, 131071)))), 917503));
			let mut fTemp924: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp923, 7)) as usize] };
			let mut fTemp925: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp923 as usize] };
			let mut fTemp926: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp923, 1)) as usize] } - fTemp925;
			let mut fTemp927: F64 = 65535.5 * fTemp919;
			let mut iTemp928: i32 = (fTemp927) as i32;
			let mut iTemp929: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp928, 131071)))), 917503));
			let mut fTemp930: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp929, 7)) as usize] };
			let mut fTemp931: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp929 as usize] };
			let mut fTemp932: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp929, 1)) as usize] } - fTemp931;
			let mut fTemp933: F64 = if iTemp690 != 0 {fTemp931 + fTemp699 * fTemp932 + (fTemp927 - (iTemp928) as F64) * (fTemp930 - (fTemp931 + fTemp699 * (fTemp932 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp929, 8)) as usize] } - fTemp930))))} else {1.0 - (fTemp925 + fTemp699 * fTemp926 + (fTemp921 - (iTemp922) as F64) * (fTemp924 - (fTemp925 + fTemp699 * (fTemp926 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp923, 8)) as usize] } - fTemp924)))))};
			let mut fTemp934: F64 = fTemp704 + fTemp920;
			let mut fTemp935: F64 = 131071.0 * (1.0 - fTemp934);
			let mut iTemp936: i32 = (fTemp935) as i32;
			let mut iTemp937: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp936, 131071)))), 917503));
			let mut fTemp938: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp937, 7)) as usize] };
			let mut fTemp939: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp937 as usize] };
			let mut fTemp940: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp937, 1)) as usize] } - fTemp939;
			let mut fTemp941: F64 = 131071.0 * fTemp934;
			let mut iTemp942: i32 = (fTemp941) as i32;
			let mut iTemp943: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp942, 131071)))), 917503));
			let mut fTemp944: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp943, 7), 917503))) as usize] };
			let mut fTemp945: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp943 as usize] };
			let mut fTemp946: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp943, 1), 917503))) as usize] } - fTemp945;
			let mut iTemp947: i32 = (fTemp760 > ((if iTemp690 != 0 {fTemp945 + fTemp699 * fTemp946 + (fTemp941 - (iTemp942) as F64) * (fTemp944 - (fTemp945 + fTemp699 * (fTemp946 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp943, 8), 917503))) as usize] } - fTemp944))))} else {1.0 - (fTemp939 + fTemp699 * fTemp940 + (fTemp935 - (iTemp936) as F64) * (fTemp938 - (fTemp939 + fTemp699 * (fTemp940 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp937, 8)) as usize] } - fTemp938)))))} - fTemp933) / (1.0 - fTemp933))) as i32;
			let mut fTemp948: F64 = if iTemp947 != 0 {fTemp917} else {fTemp920};
			let mut fTemp949: F64 = if iTemp947 != 0 {fTemp920} else {fTemp918};
			let mut fTemp950: F64 = fTemp949 + fTemp948;
			let mut fTemp951: F64 = 0.5 * fTemp950;
			let mut fTemp952: F64 = 131071.0 * (1.0 - fTemp951);
			let mut iTemp953: i32 = (fTemp952) as i32;
			let mut iTemp954: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp953, 131071)))), 917503));
			let mut fTemp955: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp954, 7)) as usize] };
			let mut fTemp956: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp954 as usize] };
			let mut fTemp957: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp954, 1)) as usize] } - fTemp956;
			let mut fTemp958: F64 = 65535.5 * fTemp950;
			let mut iTemp959: i32 = (fTemp958) as i32;
			let mut iTemp960: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp959, 131071)))), 917503));
			let mut fTemp961: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp960, 7)) as usize] };
			let mut fTemp962: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp960 as usize] };
			let mut fTemp963: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp960, 1)) as usize] } - fTemp962;
			let mut fTemp964: F64 = if iTemp690 != 0 {fTemp962 + fTemp699 * fTemp963 + (fTemp958 - (iTemp959) as F64) * (fTemp961 - (fTemp962 + fTemp699 * (fTemp963 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp960, 8)) as usize] } - fTemp961))))} else {1.0 - (fTemp956 + fTemp699 * fTemp957 + (fTemp952 - (iTemp953) as F64) * (fTemp955 - (fTemp956 + fTemp699 * (fTemp957 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp954, 8)) as usize] } - fTemp955)))))};
			let mut fTemp965: F64 = fTemp704 + fTemp951;
			let mut fTemp966: F64 = 131071.0 * (1.0 - fTemp965);
			let mut iTemp967: i32 = (fTemp966) as i32;
			let mut iTemp968: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp967, 131071)))), 917503));
			let mut fTemp969: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp968, 7)) as usize] };
			let mut fTemp970: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp968 as usize] };
			let mut fTemp971: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp968, 1)) as usize] } - fTemp970;
			let mut fTemp972: F64 = 131071.0 * fTemp965;
			let mut iTemp973: i32 = (fTemp972) as i32;
			let mut iTemp974: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp973, 131071)))), 917503));
			let mut fTemp975: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp974, 7), 917503))) as usize] };
			let mut fTemp976: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp974 as usize] };
			let mut fTemp977: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp974, 1), 917503))) as usize] } - fTemp976;
			let mut iTemp978: i32 = (fTemp760 > ((if iTemp690 != 0 {fTemp976 + fTemp699 * fTemp977 + (fTemp972 - (iTemp973) as F64) * (fTemp975 - (fTemp976 + fTemp699 * (fTemp977 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp974, 8), 917503))) as usize] } - fTemp975))))} else {1.0 - (fTemp970 + fTemp699 * fTemp971 + (fTemp966 - (iTemp967) as F64) * (fTemp969 - (fTemp970 + fTemp699 * (fTemp971 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp968, 8)) as usize] } - fTemp969)))))} - fTemp964) / (1.0 - fTemp964))) as i32;
			let mut fTemp979: F64 = if iTemp978 != 0 {fTemp948} else {fTemp951};
			let mut fTemp980: F64 = if iTemp978 != 0 {fTemp951} else {fTemp949};
			let mut fTemp981: F64 = fTemp980 + fTemp979;
			let mut fTemp982: F64 = 0.5 * fTemp981;
			let mut fTemp983: F64 = 131071.0 * (1.0 - fTemp982);
			let mut iTemp984: i32 = (fTemp983) as i32;
			let mut iTemp985: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp984, 131071)))), 917503));
			let mut fTemp986: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp985, 7)) as usize] };
			let mut fTemp987: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp985 as usize] };
			let mut fTemp988: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp985, 1)) as usize] } - fTemp987;
			let mut fTemp989: F64 = 65535.5 * fTemp981;
			let mut iTemp990: i32 = (fTemp989) as i32;
			let mut iTemp991: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp990, 131071)))), 917503));
			let mut fTemp992: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp991, 7)) as usize] };
			let mut fTemp993: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp991 as usize] };
			let mut fTemp994: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp991, 1)) as usize] } - fTemp993;
			let mut fTemp995: F64 = if iTemp690 != 0 {fTemp993 + fTemp699 * fTemp994 + (fTemp989 - (iTemp990) as F64) * (fTemp992 - (fTemp993 + fTemp699 * (fTemp994 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp991, 8)) as usize] } - fTemp992))))} else {1.0 - (fTemp987 + fTemp699 * fTemp988 + (fTemp983 - (iTemp984) as F64) * (fTemp986 - (fTemp987 + fTemp699 * (fTemp988 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp985, 8)) as usize] } - fTemp986)))))};
			let mut fTemp996: F64 = fTemp704 + fTemp982;
			let mut fTemp997: F64 = 131071.0 * (1.0 - fTemp996);
			let mut iTemp998: i32 = (fTemp997) as i32;
			let mut iTemp999: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp998, 131071)))), 917503));
			let mut fTemp1000: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp999, 7)) as usize] };
			let mut fTemp1001: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp999 as usize] };
			let mut fTemp1002: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp999, 1)) as usize] } - fTemp1001;
			let mut fTemp1003: F64 = 131071.0 * fTemp996;
			let mut iTemp1004: i32 = (fTemp1003) as i32;
			let mut iTemp1005: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp1004, 131071)))), 917503));
			let mut fTemp1006: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1005, 7), 917503))) as usize] };
			let mut fTemp1007: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1005 as usize] };
			let mut fTemp1008: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1005, 1), 917503))) as usize] } - fTemp1007;
			let mut iTemp1009: i32 = (fTemp760 > ((if iTemp690 != 0 {fTemp1007 + fTemp699 * fTemp1008 + (fTemp1003 - (iTemp1004) as F64) * (fTemp1006 - (fTemp1007 + fTemp699 * (fTemp1008 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1005, 8), 917503))) as usize] } - fTemp1006))))} else {1.0 - (fTemp1001 + fTemp699 * fTemp1002 + (fTemp997 - (iTemp998) as F64) * (fTemp1000 - (fTemp1001 + fTemp699 * (fTemp1002 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp999, 8)) as usize] } - fTemp1000)))))} - fTemp995) / (1.0 - fTemp995))) as i32;
			let mut fTemp1010: F64 = if iTemp1009 != 0 {fTemp979} else {fTemp982};
			let mut fTemp1011: F64 = if iTemp1009 != 0 {fTemp982} else {fTemp980};
			let mut fTemp1012: F64 = fTemp1011 + fTemp1010;
			let mut fTemp1013: F64 = 0.5 * fTemp1012;
			let mut fTemp1014: F64 = 131071.0 * (1.0 - fTemp1013);
			let mut iTemp1015: i32 = (fTemp1014) as i32;
			let mut iTemp1016: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp1015, 131071)))), 917503));
			let mut fTemp1017: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1016, 7)) as usize] };
			let mut fTemp1018: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1016 as usize] };
			let mut fTemp1019: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1016, 1)) as usize] } - fTemp1018;
			let mut fTemp1020: F64 = 65535.5 * fTemp1012;
			let mut iTemp1021: i32 = (fTemp1020) as i32;
			let mut iTemp1022: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp1021, 131071)))), 917503));
			let mut fTemp1023: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1022, 7)) as usize] };
			let mut fTemp1024: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1022 as usize] };
			let mut fTemp1025: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1022, 1)) as usize] } - fTemp1024;
			let mut fTemp1026: F64 = if iTemp690 != 0 {fTemp1024 + fTemp699 * fTemp1025 + (fTemp1020 - (iTemp1021) as F64) * (fTemp1023 - (fTemp1024 + fTemp699 * (fTemp1025 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1022, 8)) as usize] } - fTemp1023))))} else {1.0 - (fTemp1018 + fTemp699 * fTemp1019 + (fTemp1014 - (iTemp1015) as F64) * (fTemp1017 - (fTemp1018 + fTemp699 * (fTemp1019 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1016, 8)) as usize] } - fTemp1017)))))};
			let mut fTemp1027: F64 = fTemp704 + fTemp1013;
			let mut fTemp1028: F64 = 131071.0 * (1.0 - fTemp1027);
			let mut iTemp1029: i32 = (fTemp1028) as i32;
			let mut iTemp1030: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp1029, 131071)))), 917503));
			let mut fTemp1031: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1030, 7)) as usize] };
			let mut fTemp1032: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1030 as usize] };
			let mut fTemp1033: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1030, 1)) as usize] } - fTemp1032;
			let mut fTemp1034: F64 = 131071.0 * fTemp1027;
			let mut iTemp1035: i32 = (fTemp1034) as i32;
			let mut iTemp1036: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp1035, 131071)))), 917503));
			let mut fTemp1037: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1036, 7), 917503))) as usize] };
			let mut fTemp1038: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1036 as usize] };
			let mut fTemp1039: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1036, 1), 917503))) as usize] } - fTemp1038;
			let mut iTemp1040: i32 = (fTemp760 > ((if iTemp690 != 0 {fTemp1038 + fTemp699 * fTemp1039 + (fTemp1034 - (iTemp1035) as F64) * (fTemp1037 - (fTemp1038 + fTemp699 * (fTemp1039 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1036, 8), 917503))) as usize] } - fTemp1037))))} else {1.0 - (fTemp1032 + fTemp699 * fTemp1033 + (fTemp1028 - (iTemp1029) as F64) * (fTemp1031 - (fTemp1032 + fTemp699 * (fTemp1033 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1030, 8)) as usize] } - fTemp1031)))))} - fTemp1026) / (1.0 - fTemp1026))) as i32;
			let mut fTemp1041: F64 = if iTemp1040 != 0 {fTemp1010} else {fTemp1013};
			let mut fTemp1042: F64 = if iTemp1040 != 0 {fTemp1013} else {fTemp1011};
			let mut fTemp1043: F64 = fTemp1042 + fTemp1041;
			let mut fTemp1044: F64 = 0.5 * fTemp1043;
			let mut fTemp1045: F64 = 131071.0 * (1.0 - fTemp1044);
			let mut iTemp1046: i32 = (fTemp1045) as i32;
			let mut iTemp1047: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp1046, 131071)))), 917503));
			let mut fTemp1048: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1047, 7)) as usize] };
			let mut fTemp1049: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1047 as usize] };
			let mut fTemp1050: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1047, 1)) as usize] } - fTemp1049;
			let mut fTemp1051: F64 = 65535.5 * fTemp1043;
			let mut iTemp1052: i32 = (fTemp1051) as i32;
			let mut iTemp1053: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp1052, 131071)))), 917503));
			let mut fTemp1054: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1053, 7)) as usize] };
			let mut fTemp1055: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1053 as usize] };
			let mut fTemp1056: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1053, 1)) as usize] } - fTemp1055;
			let mut fTemp1057: F64 = if iTemp690 != 0 {fTemp1055 + fTemp699 * fTemp1056 + (fTemp1051 - (iTemp1052) as F64) * (fTemp1054 - (fTemp1055 + fTemp699 * (fTemp1056 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1053, 8)) as usize] } - fTemp1054))))} else {1.0 - (fTemp1049 + fTemp699 * fTemp1050 + (fTemp1045 - (iTemp1046) as F64) * (fTemp1048 - (fTemp1049 + fTemp699 * (fTemp1050 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1047, 8)) as usize] } - fTemp1048)))))};
			let mut fTemp1058: F64 = fTemp704 + fTemp1044;
			let mut fTemp1059: F64 = 131071.0 * (1.0 - fTemp1058);
			let mut iTemp1060: i32 = (fTemp1059) as i32;
			let mut iTemp1061: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp1060, 131071)))), 917503));
			let mut fTemp1062: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1061, 7)) as usize] };
			let mut fTemp1063: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1061 as usize] };
			let mut fTemp1064: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1061, 1)) as usize] } - fTemp1063;
			let mut fTemp1065: F64 = 131071.0 * fTemp1058;
			let mut iTemp1066: i32 = (fTemp1065) as i32;
			let mut iTemp1067: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp1066, 131071)))), 917503));
			let mut fTemp1068: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1067, 7), 917503))) as usize] };
			let mut fTemp1069: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1067 as usize] };
			let mut fTemp1070: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1067, 1), 917503))) as usize] } - fTemp1069;
			let mut iTemp1071: i32 = (fTemp760 > ((if iTemp690 != 0 {fTemp1069 + fTemp699 * fTemp1070 + (fTemp1065 - (iTemp1066) as F64) * (fTemp1068 - (fTemp1069 + fTemp699 * (fTemp1070 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1067, 8), 917503))) as usize] } - fTemp1068))))} else {1.0 - (fTemp1063 + fTemp699 * fTemp1064 + (fTemp1059 - (iTemp1060) as F64) * (fTemp1062 - (fTemp1063 + fTemp699 * (fTemp1064 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1061, 8)) as usize] } - fTemp1062)))))} - fTemp1057) / (1.0 - fTemp1057))) as i32;
			let mut fTemp1072: F64 = if iTemp1071 != 0 {fTemp1041} else {fTemp1044};
			let mut fTemp1073: F64 = if iTemp1071 != 0 {fTemp1044} else {fTemp1042};
			let mut fTemp1074: F64 = fTemp1073 + fTemp1072;
			let mut fTemp1075: F64 = 0.5 * fTemp1074;
			let mut fTemp1076: F64 = 131071.0 * (1.0 - fTemp1075);
			let mut iTemp1077: i32 = (fTemp1076) as i32;
			let mut iTemp1078: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp1077, 131071)))), 917503));
			let mut fTemp1079: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1078, 7)) as usize] };
			let mut fTemp1080: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1078 as usize] };
			let mut fTemp1081: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1078, 1)) as usize] } - fTemp1080;
			let mut fTemp1082: F64 = 65535.5 * fTemp1074;
			let mut iTemp1083: i32 = (fTemp1082) as i32;
			let mut iTemp1084: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp1083, 131071)))), 917503));
			let mut fTemp1085: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1084, 7)) as usize] };
			let mut fTemp1086: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1084 as usize] };
			let mut fTemp1087: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1084, 1)) as usize] } - fTemp1086;
			let mut fTemp1088: F64 = if iTemp690 != 0 {fTemp1086 + fTemp699 * fTemp1087 + (fTemp1082 - (iTemp1083) as F64) * (fTemp1085 - (fTemp1086 + fTemp699 * (fTemp1087 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1084, 8)) as usize] } - fTemp1085))))} else {1.0 - (fTemp1080 + fTemp699 * fTemp1081 + (fTemp1076 - (iTemp1077) as F64) * (fTemp1079 - (fTemp1080 + fTemp699 * (fTemp1081 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1078, 8)) as usize] } - fTemp1079)))))};
			let mut fTemp1089: F64 = fTemp704 + fTemp1075;
			let mut fTemp1090: F64 = 131071.0 * (1.0 - fTemp1089);
			let mut iTemp1091: i32 = (fTemp1090) as i32;
			let mut iTemp1092: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp1091, 131071)))), 917503));
			let mut fTemp1093: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1092, 7)) as usize] };
			let mut fTemp1094: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1092 as usize] };
			let mut fTemp1095: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1092, 1)) as usize] } - fTemp1094;
			let mut fTemp1096: F64 = 131071.0 * fTemp1089;
			let mut iTemp1097: i32 = (fTemp1096) as i32;
			let mut iTemp1098: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp1097, 131071)))), 917503));
			let mut fTemp1099: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1098, 7), 917503))) as usize] };
			let mut fTemp1100: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1098 as usize] };
			let mut fTemp1101: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1098, 1), 917503))) as usize] } - fTemp1100;
			let mut iTemp1102: i32 = (fTemp760 > ((if iTemp690 != 0 {fTemp1100 + fTemp699 * fTemp1101 + (fTemp1096 - (iTemp1097) as F64) * (fTemp1099 - (fTemp1100 + fTemp699 * (fTemp1101 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1098, 8), 917503))) as usize] } - fTemp1099))))} else {1.0 - (fTemp1094 + fTemp699 * fTemp1095 + (fTemp1090 - (iTemp1091) as F64) * (fTemp1093 - (fTemp1094 + fTemp699 * (fTemp1095 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1092, 8)) as usize] } - fTemp1093)))))} - fTemp1088) / (1.0 - fTemp1088))) as i32;
			let mut fTemp1103: F64 = if iTemp1102 != 0 {fTemp1072} else {fTemp1075};
			let mut fTemp1104: F64 = if iTemp1102 != 0 {fTemp1075} else {fTemp1073};
			let mut fTemp1105: F64 = fTemp1104 + fTemp1103;
			let mut fTemp1106: F64 = 0.5 * fTemp1105;
			let mut fTemp1107: F64 = 131071.0 * (1.0 - fTemp1106);
			let mut iTemp1108: i32 = (fTemp1107) as i32;
			let mut iTemp1109: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp1108, 131071)))), 917503));
			let mut fTemp1110: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1109, 7)) as usize] };
			let mut fTemp1111: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1109 as usize] };
			let mut fTemp1112: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1109, 1)) as usize] } - fTemp1111;
			let mut fTemp1113: F64 = 65535.5 * fTemp1105;
			let mut iTemp1114: i32 = (fTemp1113) as i32;
			let mut iTemp1115: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp1114, 131071)))), 917503));
			let mut fTemp1116: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1115, 7)) as usize] };
			let mut fTemp1117: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1115 as usize] };
			let mut fTemp1118: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1115, 1)) as usize] } - fTemp1117;
			let mut fTemp1119: F64 = if iTemp690 != 0 {fTemp1117 + fTemp699 * fTemp1118 + (fTemp1113 - (iTemp1114) as F64) * (fTemp1116 - (fTemp1117 + fTemp699 * (fTemp1118 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1115, 8)) as usize] } - fTemp1116))))} else {1.0 - (fTemp1111 + fTemp699 * fTemp1112 + (fTemp1107 - (iTemp1108) as F64) * (fTemp1110 - (fTemp1111 + fTemp699 * (fTemp1112 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1109, 8)) as usize] } - fTemp1110)))))};
			let mut fTemp1120: F64 = fTemp704 + fTemp1106;
			let mut fTemp1121: F64 = 131071.0 * (1.0 - fTemp1120);
			let mut iTemp1122: i32 = (fTemp1121) as i32;
			let mut iTemp1123: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp1122, 131071)))), 917503));
			let mut fTemp1124: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1123, 7)) as usize] };
			let mut fTemp1125: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1123 as usize] };
			let mut fTemp1126: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1123, 1)) as usize] } - fTemp1125;
			let mut fTemp1127: F64 = 131071.0 * fTemp1120;
			let mut iTemp1128: i32 = (fTemp1127) as i32;
			let mut iTemp1129: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp1128, 131071)))), 917503));
			let mut fTemp1130: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1129, 7), 917503))) as usize] };
			let mut fTemp1131: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1129 as usize] };
			let mut fTemp1132: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1129, 1), 917503))) as usize] } - fTemp1131;
			let mut iTemp1133: i32 = (fTemp760 > ((if iTemp690 != 0 {fTemp1131 + fTemp699 * fTemp1132 + (fTemp1127 - (iTemp1128) as F64) * (fTemp1130 - (fTemp1131 + fTemp699 * (fTemp1132 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1129, 8), 917503))) as usize] } - fTemp1130))))} else {1.0 - (fTemp1125 + fTemp699 * fTemp1126 + (fTemp1121 - (iTemp1122) as F64) * (fTemp1124 - (fTemp1125 + fTemp699 * (fTemp1126 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1123, 8)) as usize] } - fTemp1124)))))} - fTemp1119) / (1.0 - fTemp1119))) as i32;
			let mut fTemp1134: F64 = if iTemp1133 != 0 {fTemp1103} else {fTemp1106};
			let mut fTemp1135: F64 = if iTemp1133 != 0 {fTemp1106} else {fTemp1104};
			let mut fTemp1136: F64 = fTemp1135 + fTemp1134;
			let mut fTemp1137: F64 = 0.5 * fTemp1136;
			let mut fTemp1138: F64 = 131071.0 * (1.0 - fTemp1137);
			let mut iTemp1139: i32 = (fTemp1138) as i32;
			let mut iTemp1140: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp1139, 131071)))), 917503));
			let mut fTemp1141: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1140, 7)) as usize] };
			let mut fTemp1142: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1140 as usize] };
			let mut fTemp1143: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1140, 1)) as usize] } - fTemp1142;
			let mut fTemp1144: F64 = 65535.5 * fTemp1136;
			let mut iTemp1145: i32 = (fTemp1144) as i32;
			let mut iTemp1146: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp1145, 131071)))), 917503));
			let mut fTemp1147: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1146, 7)) as usize] };
			let mut fTemp1148: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1146 as usize] };
			let mut fTemp1149: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1146, 1)) as usize] } - fTemp1148;
			let mut fTemp1150: F64 = if iTemp690 != 0 {fTemp1148 + fTemp699 * fTemp1149 + (fTemp1144 - (iTemp1145) as F64) * (fTemp1147 - (fTemp1148 + fTemp699 * (fTemp1149 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1146, 8)) as usize] } - fTemp1147))))} else {1.0 - (fTemp1142 + fTemp699 * fTemp1143 + (fTemp1138 - (iTemp1139) as F64) * (fTemp1141 - (fTemp1142 + fTemp699 * (fTemp1143 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1140, 8)) as usize] } - fTemp1141)))))};
			let mut fTemp1151: F64 = fTemp704 + fTemp1137;
			let mut fTemp1152: F64 = 131071.0 * (1.0 - fTemp1151);
			let mut iTemp1153: i32 = (fTemp1152) as i32;
			let mut iTemp1154: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp1153, 131071)))), 917503));
			let mut fTemp1155: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1154, 7)) as usize] };
			let mut fTemp1156: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1154 as usize] };
			let mut fTemp1157: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1154, 1)) as usize] } - fTemp1156;
			let mut fTemp1158: F64 = 131071.0 * fTemp1151;
			let mut iTemp1159: i32 = (fTemp1158) as i32;
			let mut iTemp1160: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp1159, 131071)))), 917503));
			let mut fTemp1161: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1160, 7), 917503))) as usize] };
			let mut fTemp1162: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1160 as usize] };
			let mut fTemp1163: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1160, 1), 917503))) as usize] } - fTemp1162;
			let mut iTemp1164: i32 = (fTemp760 > ((if iTemp690 != 0 {fTemp1162 + fTemp699 * fTemp1163 + (fTemp1158 - (iTemp1159) as F64) * (fTemp1161 - (fTemp1162 + fTemp699 * (fTemp1163 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1160, 8), 917503))) as usize] } - fTemp1161))))} else {1.0 - (fTemp1156 + fTemp699 * fTemp1157 + (fTemp1152 - (iTemp1153) as F64) * (fTemp1155 - (fTemp1156 + fTemp699 * (fTemp1157 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1154, 8)) as usize] } - fTemp1155)))))} - fTemp1150) / (1.0 - fTemp1150))) as i32;
			let mut fTemp1165: F64 = if iTemp1164 != 0 {fTemp1134} else {fTemp1137};
			let mut fTemp1166: F64 = if iTemp1164 != 0 {fTemp1137} else {fTemp1135};
			let mut fTemp1167: F64 = fTemp1166 + fTemp1165;
			let mut fTemp1168: F64 = 0.5 * fTemp1167;
			let mut fTemp1169: F64 = 131071.0 * (1.0 - fTemp1168);
			let mut iTemp1170: i32 = (fTemp1169) as i32;
			let mut iTemp1171: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp1170, 131071)))), 917503));
			let mut fTemp1172: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1171, 7)) as usize] };
			let mut fTemp1173: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1171 as usize] };
			let mut fTemp1174: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1171, 1)) as usize] } - fTemp1173;
			let mut fTemp1175: F64 = 65535.5 * fTemp1167;
			let mut iTemp1176: i32 = (fTemp1175) as i32;
			let mut iTemp1177: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp1176, 131071)))), 917503));
			let mut fTemp1178: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1177, 7)) as usize] };
			let mut fTemp1179: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1177 as usize] };
			let mut fTemp1180: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1177, 1)) as usize] } - fTemp1179;
			let mut fTemp1181: F64 = if iTemp690 != 0 {fTemp1179 + fTemp699 * fTemp1180 + (fTemp1175 - (iTemp1176) as F64) * (fTemp1178 - (fTemp1179 + fTemp699 * (fTemp1180 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1177, 8)) as usize] } - fTemp1178))))} else {1.0 - (fTemp1173 + fTemp699 * fTemp1174 + (fTemp1169 - (iTemp1170) as F64) * (fTemp1172 - (fTemp1173 + fTemp699 * (fTemp1174 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1171, 8)) as usize] } - fTemp1172)))))};
			let mut fTemp1182: F64 = fTemp704 + fTemp1168;
			let mut fTemp1183: F64 = 131071.0 * (1.0 - fTemp1182);
			let mut iTemp1184: i32 = (fTemp1183) as i32;
			let mut iTemp1185: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp1184, 131071)))), 917503));
			let mut fTemp1186: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1185, 7)) as usize] };
			let mut fTemp1187: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1185 as usize] };
			let mut fTemp1188: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1185, 1)) as usize] } - fTemp1187;
			let mut fTemp1189: F64 = 131071.0 * fTemp1182;
			let mut iTemp1190: i32 = (fTemp1189) as i32;
			let mut iTemp1191: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp1190, 131071)))), 917503));
			let mut fTemp1192: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1191, 7), 917503))) as usize] };
			let mut fTemp1193: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1191 as usize] };
			let mut fTemp1194: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1191, 1), 917503))) as usize] } - fTemp1193;
			let mut iTemp1195: i32 = (fTemp760 > ((if iTemp690 != 0 {fTemp1193 + fTemp699 * fTemp1194 + (fTemp1189 - (iTemp1190) as F64) * (fTemp1192 - (fTemp1193 + fTemp699 * (fTemp1194 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1191, 8), 917503))) as usize] } - fTemp1192))))} else {1.0 - (fTemp1187 + fTemp699 * fTemp1188 + (fTemp1183 - (iTemp1184) as F64) * (fTemp1186 - (fTemp1187 + fTemp699 * (fTemp1188 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1185, 8)) as usize] } - fTemp1186)))))} - fTemp1181) / (1.0 - fTemp1181))) as i32;
			let mut fTemp1196: F64 = if iTemp1195 != 0 {fTemp1165} else {fTemp1168};
			let mut fTemp1197: F64 = if iTemp1195 != 0 {fTemp1168} else {fTemp1166};
			let mut fTemp1198: F64 = fTemp1197 + fTemp1196;
			let mut fTemp1199: F64 = 0.5 * fTemp1198;
			let mut fTemp1200: F64 = 131071.0 * (1.0 - fTemp1199);
			let mut iTemp1201: i32 = (fTemp1200) as i32;
			let mut iTemp1202: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp1201, 131071)))), 917503));
			let mut fTemp1203: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1202, 7)) as usize] };
			let mut fTemp1204: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1202 as usize] };
			let mut fTemp1205: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1202, 1)) as usize] } - fTemp1204;
			let mut fTemp1206: F64 = 65535.5 * fTemp1198;
			let mut iTemp1207: i32 = (fTemp1206) as i32;
			let mut iTemp1208: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp1207, 131071)))), 917503));
			let mut fTemp1209: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1208, 7)) as usize] };
			let mut fTemp1210: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1208 as usize] };
			let mut fTemp1211: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1208, 1)) as usize] } - fTemp1210;
			let mut fTemp1212: F64 = if iTemp690 != 0 {fTemp1210 + fTemp699 * fTemp1211 + (fTemp1206 - (iTemp1207) as F64) * (fTemp1209 - (fTemp1210 + fTemp699 * (fTemp1211 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1208, 8)) as usize] } - fTemp1209))))} else {1.0 - (fTemp1204 + fTemp699 * fTemp1205 + (fTemp1200 - (iTemp1201) as F64) * (fTemp1203 - (fTemp1204 + fTemp699 * (fTemp1205 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1202, 8)) as usize] } - fTemp1203)))))};
			let mut fTemp1213: F64 = fTemp704 + fTemp1199;
			let mut fTemp1214: F64 = 131071.0 * (1.0 - fTemp1213);
			let mut iTemp1215: i32 = (fTemp1214) as i32;
			let mut iTemp1216: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp1215, 131071)))), 917503));
			let mut fTemp1217: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1216, 7)) as usize] };
			let mut fTemp1218: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1216 as usize] };
			let mut fTemp1219: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1216, 1)) as usize] } - fTemp1218;
			let mut fTemp1220: F64 = 131071.0 * fTemp1213;
			let mut iTemp1221: i32 = (fTemp1220) as i32;
			let mut iTemp1222: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp1221, 131071)))), 917503));
			let mut fTemp1223: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1222, 7), 917503))) as usize] };
			let mut fTemp1224: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1222 as usize] };
			let mut fTemp1225: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1222, 1), 917503))) as usize] } - fTemp1224;
			let mut iTemp1226: i32 = (fTemp760 > ((if iTemp690 != 0 {fTemp1224 + fTemp699 * fTemp1225 + (fTemp1220 - (iTemp1221) as F64) * (fTemp1223 - (fTemp1224 + fTemp699 * (fTemp1225 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1222, 8), 917503))) as usize] } - fTemp1223))))} else {1.0 - (fTemp1218 + fTemp699 * fTemp1219 + (fTemp1214 - (iTemp1215) as F64) * (fTemp1217 - (fTemp1218 + fTemp699 * (fTemp1219 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1216, 8)) as usize] } - fTemp1217)))))} - fTemp1212) / (1.0 - fTemp1212))) as i32;
			let mut fTemp1227: F64 = if iTemp1226 != 0 {fTemp1196} else {fTemp1199};
			let mut fTemp1228: F64 = if iTemp1226 != 0 {fTemp1199} else {fTemp1197};
			let mut fTemp1229: F64 = fTemp1228 + fTemp1227;
			let mut fTemp1230: F64 = 0.5 * fTemp1229;
			let mut fTemp1231: F64 = 131071.0 * (1.0 - fTemp1230);
			let mut iTemp1232: i32 = (fTemp1231) as i32;
			let mut iTemp1233: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp1232, 131071)))), 917503));
			let mut fTemp1234: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1233, 7)) as usize] };
			let mut fTemp1235: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1233 as usize] };
			let mut fTemp1236: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1233, 1)) as usize] } - fTemp1235;
			let mut fTemp1237: F64 = 65535.5 * fTemp1229;
			let mut iTemp1238: i32 = (fTemp1237) as i32;
			let mut iTemp1239: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp1238, 131071)))), 917503));
			let mut fTemp1240: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1239, 7)) as usize] };
			let mut fTemp1241: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1239 as usize] };
			let mut fTemp1242: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1239, 1)) as usize] } - fTemp1241;
			let mut fTemp1243: F64 = if iTemp690 != 0 {fTemp1241 + fTemp699 * fTemp1242 + (fTemp1237 - (iTemp1238) as F64) * (fTemp1240 - (fTemp1241 + fTemp699 * (fTemp1242 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1239, 8), 917503))) as usize] } - fTemp1240))))} else {1.0 - (fTemp1235 + fTemp699 * fTemp1236 + (fTemp1231 - (iTemp1232) as F64) * (fTemp1234 - (fTemp1235 + fTemp699 * (fTemp1236 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1233, 8), 917503))) as usize] } - fTemp1234)))))};
			let mut fTemp1244: F64 = fTemp704 + fTemp1230;
			let mut fTemp1245: F64 = 131071.0 * (1.0 - fTemp1244);
			let mut iTemp1246: i32 = (fTemp1245) as i32;
			let mut iTemp1247: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp1246, 131071)))), 917503));
			let mut fTemp1248: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1247, 7)) as usize] };
			let mut fTemp1249: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1247 as usize] };
			let mut fTemp1250: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1247, 1)) as usize] } - fTemp1249;
			let mut fTemp1251: F64 = 131071.0 * fTemp1244;
			let mut iTemp1252: i32 = (fTemp1251) as i32;
			let mut iTemp1253: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp1252, 131071)))), 917503));
			let mut fTemp1254: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1253, 7), 917503))) as usize] };
			let mut fTemp1255: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1253 as usize] };
			let mut fTemp1256: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1253, 1), 917503))) as usize] } - fTemp1255;
			let mut iTemp1257: i32 = (fTemp760 > ((if iTemp690 != 0 {fTemp1255 + fTemp699 * fTemp1256 + (fTemp1251 - (iTemp1252) as F64) * (fTemp1254 - (fTemp1255 + fTemp699 * (fTemp1256 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1253, 8), 917503))) as usize] } - fTemp1254))))} else {1.0 - (fTemp1249 + fTemp699 * fTemp1250 + (fTemp1245 - (iTemp1246) as F64) * (fTemp1248 - (fTemp1249 + fTemp699 * (fTemp1250 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1247, 8)) as usize] } - fTemp1248)))))} - fTemp1243) / (1.0 - fTemp1243))) as i32;
			let mut fTemp1258: F64 = F64::min(1.0, F64::max(0.0, 0.5 * (if iTemp1257 != 0 {fTemp1230} else {fTemp1228} + if iTemp1257 != 0 {fTemp1227} else {fTemp1230})));
			self.fRec15[0] = fTemp1258;
			let mut fTemp1259: F64 = 131071.0 * (1.0 - fTemp1258);
			let mut iTemp1260: i32 = (fTemp1259) as i32;
			let mut iTemp1261: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp1260, 131071)))), 917503));
			let mut fTemp1262: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1261, 7)) as usize] };
			let mut fTemp1263: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1261 as usize] };
			let mut fTemp1264: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1261, 1)) as usize] } - fTemp1263;
			let mut fTemp1265: F64 = 131071.0 * fTemp1258;
			let mut iTemp1266: i32 = (fTemp1265) as i32;
			let mut iTemp1267: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp1266, 131071)))), 917503));
			let mut fTemp1268: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1267, 7)) as usize] };
			let mut fTemp1269: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1267 as usize] };
			let mut fTemp1270: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1267, 1)) as usize] } - fTemp1269;
			let mut fTemp1271: F64 = if iTemp690 != 0 {fTemp1269 + fTemp699 * fTemp1270 + (fTemp1265 - (iTemp1266) as F64) * (fTemp1268 - (fTemp1269 + fTemp699 * (fTemp1270 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1267, 8), 917503))) as usize] } - fTemp1268))))} else {1.0 - (fTemp1263 + fTemp699 * fTemp1264 + (fTemp1259 - (iTemp1260) as F64) * (fTemp1262 - (fTemp1263 + fTemp699 * (fTemp1264 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1261, 8), 917503))) as usize] } - fTemp1262)))))};
			let mut fTemp1272: F64 = fTemp704 + fTemp1258;
			let mut fTemp1273: F64 = 131071.0 * (1.0 - fTemp1272);
			let mut iTemp1274: i32 = (fTemp1273) as i32;
			let mut iTemp1275: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp1274, 131071)))), 917503));
			let mut fTemp1276: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1275, 7)) as usize] };
			let mut fTemp1277: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1275 as usize] };
			let mut fTemp1278: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1275, 1)) as usize] } - fTemp1277;
			let mut fTemp1279: F64 = 131071.0 * fTemp1272;
			let mut iTemp1280: i32 = (fTemp1279) as i32;
			let mut iTemp1281: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, i32::wrapping_mul(7, std::cmp::max(0, std::cmp::min(iTemp1280, 131071)))), 917503));
			let mut fTemp1282: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1281, 7), 917503))) as usize] };
			let mut fTemp1283: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1281 as usize] };
			let mut fTemp1284: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1281, 1), 917503))) as usize] } - fTemp1283;
			let mut fTemp1285: F64 = self.fRec16[1] + if ((0.001 * fTemp703) == 0.0) as i32 != 0 {fTemp689} else {fTemp689 * (if iTemp690 != 0 {fTemp1283 + fTemp699 * fTemp1284 + (fTemp1279 - (iTemp1280) as F64) * (fTemp1282 - (fTemp1283 + fTemp699 * (fTemp1284 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1281, 8), 917503))) as usize] } - fTemp1282))))} else {1.0 - (fTemp1277 + fTemp699 * fTemp1278 + (fTemp1273 - (iTemp1274) as F64) * (fTemp1276 - (fTemp1277 + fTemp699 * (fTemp1278 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1275, 8)) as usize] } - fTemp1276)))))} - fTemp1271) / (1.0 - fTemp1271)};
			self.fRec16[0] = if iTemp702 != 0 {F64::min(fTemp1285, self.fRec16[1])} else {F64::max(fTemp1285, self.fRec16[1])};
			self.fVec58[(self.IOTA0 & 8191) as usize] = F64::powf(1e+01, 0.05 * self.fRec16[0]);
			let mut fTemp1286: F64 = self.fVec58[((i32::wrapping_sub(self.IOTA0, iSlow70)) & 8191) as usize];
			*output1 = 0.5 * fTemp2 * self.fVec3[((i32::wrapping_sub(self.IOTA0, iSlow71)) & 32767) as usize] + self.fRec14[0] * fTemp4 * self.fVec4[((i32::wrapping_sub(self.IOTA0, iSlow71)) & 32767) as usize] * fTemp1286;
			*output2 = fTemp3 + fTemp664 * fTemp4;
			*output3 = fTemp3 + fTemp4 * fTemp1286;
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
			self.fVec17[2] = self.fVec17[1];
			self.fVec17[1] = self.fVec17[0];
			for j2 in (1..=6).rev() {
				self.fVec18[j2 as usize] = self.fVec18[(i32::wrapping_sub(j2, 1)) as usize];
			}
			for j3 in (1..=14).rev() {
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
			for j4 in (1..=6).rev() {
				self.fVec34[j4 as usize] = self.fVec34[(i32::wrapping_sub(j4, 1)) as usize];
			}
			for j5 in (1..=14).rev() {
				self.fVec35[j5 as usize] = self.fVec35[(i32::wrapping_sub(j5, 1)) as usize];
			}
			self.fRec17[1] = self.fRec17[0];
			self.fVec44[2] = self.fVec44[1];
			self.fVec44[1] = self.fVec44[0];
			for j6 in (1..=6).rev() {
				self.fVec45[j6 as usize] = self.fVec45[(i32::wrapping_sub(j6, 1)) as usize];
			}
			for j7 in (1..=14).rev() {
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

}

pub use dsp_48k::LambRs48k;
