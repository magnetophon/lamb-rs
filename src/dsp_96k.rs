/* ------------------------------------------------------------
author: "Bart Brouns"
license: "AGPLv3"
name: "lamb-rs"
version: "0.1"
Code generated with Faust 2.72.14 (https://faust.grame.fr)
Compilation options: -a /run/user/1001/.tmpQkffti -lang rust -ct 1 -cn LambRs96k -es 1 -mcd 16 -mdd 1024 -mdy 33 -double -ftz 0
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
	iRec13: [i32;2],
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
	
	fn fillLambRs96kSIG0(&mut self, count: i32, table: &mut[F64]) {
		for i1 in 0..count {
			self.iRec13[0] = i32::wrapping_add(self.iRec13[1], 1);
			let mut iTemp66: i32 = i32::wrapping_add(self.iRec13[0], -1);
			let mut fTemp67: F64 = (iTemp66 % 3) as F64 as i32 as F64;
			let mut fTemp68: F64 = 0.5 * fTemp67;
			let mut fTemp69: F64 = F64::powf(fTemp68, 0.21 * fTemp67 + 1.0);
			let mut fTemp70: F64 = (0.3333333333333333 * (iTemp66 % 393216) as F64) as i32 as F64;
			table[i1 as usize] = F64::min(1.0, F64::max(0.0, if (fTemp68 == 0.0) as i32 != 0 {0.5 * (F64::sin(2.396863267686821e-05 * fTemp70 + 4.71238898038469) + 1.0)} else {0.5 * (F64::sin(3.141592653589793 * ((1.0 - F64::exp(-(1.8463275629239114e-05 * fTemp69 * fTemp70))) / (1.0 - F64::exp(-(2.42 * fTemp69)))) + 4.71238898038469) + 1.0)}));
			self.iRec13[1] = self.iRec13[0];
		}
	}

}


pub fn newLambRs96kSIG0() -> LambRs96kSIG0 { 
	LambRs96kSIG0 {
		iRec13: [0;2],
	}
}
fn LambRs96k_faustpower2_f(value: F64) -> F64 {
	return value * value;
}
static mut ftbl0LambRs96kSIG0: [F64;393216] = [0.0;393216];
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
pub struct LambRs96k {
	fCheckbox0: F64,
	IOTA0: i32,
	iVec0: [i32;32768],
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
	fCheckbox1: F64,
	fHslider7: F64,
	fHbargraph0: F64,
	fConst8: F64,
	fRec12: [F64;2],
	fRec6: [F64;2],
	fHslider8: F64,
	fHslider9: F64,
	fVec5: [F64;16384],
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
	fVec17: [F64;8192],
	fRec3: [F64;2],
	fVec18: [F64;3],
	fVec19: [F64;7],
	fVec20: [F64;15],
	fVec21: [F64;32],
	fVec22: [F64;64],
	fVec23: [F64;128],
	fVec24: [F64;256],
	fVec25: [F64;512],
	fVec26: [F64;1024],
	fVec27: [F64;2048],
	fVec28: [F64;4096],
	fVec29: [F64;8192],
	fVec30: [F64;2],
	fHslider10: F64,
	fHslider11: F64,
	fVec31: [F64;2],
	fVec32: [F64;2],
	fConst10: F64,
	fRec1: [F64;2],
	fRec2: [F64;16384],
	fHslider12: F64,
	fRec14: [F64;2],
	fVec33: [F64;16384],
	fVec34: [F64;3],
	fVec35: [F64;7],
	fVec36: [F64;15],
	fVec37: [F64;32],
	fVec38: [F64;64],
	fVec39: [F64;128],
	fVec40: [F64;256],
	fVec41: [F64;512],
	fVec42: [F64;1024],
	fVec43: [F64;2048],
	fVec44: [F64;4096],
	fVec45: [F64;8192],
	fRec17: [F64;2],
	fVec46: [F64;3],
	fVec47: [F64;7],
	fVec48: [F64;15],
	fVec49: [F64;32],
	fVec50: [F64;64],
	fVec51: [F64;128],
	fVec52: [F64;256],
	fVec53: [F64;512],
	fVec54: [F64;1024],
	fVec55: [F64;2048],
	fVec56: [F64;4096],
	fVec57: [F64;8192],
	fVec58: [F64;2],
	fVec59: [F64;2],
	fVec60: [F64;2],
	fRec15: [F64;2],
	fRec16: [F64;16384],
}

impl FaustDsp for LambRs96k {
	type T = F64;
		
	fn new() -> LambRs96k { 
		LambRs96k {
			fCheckbox0: 0.0,
			IOTA0: 0,
			iVec0: [0;32768],
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
			fCheckbox1: 0.0,
			fHslider7: 0.0,
			fHbargraph0: 0.0,
			fConst8: 0.0,
			fRec12: [0.0;2],
			fRec6: [0.0;2],
			fHslider8: 0.0,
			fHslider9: 0.0,
			fVec5: [0.0;16384],
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
			fVec17: [0.0;8192],
			fRec3: [0.0;2],
			fVec18: [0.0;3],
			fVec19: [0.0;7],
			fVec20: [0.0;15],
			fVec21: [0.0;32],
			fVec22: [0.0;64],
			fVec23: [0.0;128],
			fVec24: [0.0;256],
			fVec25: [0.0;512],
			fVec26: [0.0;1024],
			fVec27: [0.0;2048],
			fVec28: [0.0;4096],
			fVec29: [0.0;8192],
			fVec30: [0.0;2],
			fHslider10: 0.0,
			fHslider11: 0.0,
			fVec31: [0.0;2],
			fVec32: [0.0;2],
			fConst10: 0.0,
			fRec1: [0.0;2],
			fRec2: [0.0;16384],
			fHslider12: 0.0,
			fRec14: [0.0;2],
			fVec33: [0.0;16384],
			fVec34: [0.0;3],
			fVec35: [0.0;7],
			fVec36: [0.0;15],
			fVec37: [0.0;32],
			fVec38: [0.0;64],
			fVec39: [0.0;128],
			fVec40: [0.0;256],
			fVec41: [0.0;512],
			fVec42: [0.0;1024],
			fVec43: [0.0;2048],
			fVec44: [0.0;4096],
			fVec45: [0.0;8192],
			fRec17: [0.0;2],
			fVec46: [0.0;3],
			fVec47: [0.0;7],
			fVec48: [0.0;15],
			fVec49: [0.0;32],
			fVec50: [0.0;64],
			fVec51: [0.0;128],
			fVec52: [0.0;256],
			fVec53: [0.0;512],
			fVec54: [0.0;1024],
			fVec55: [0.0;2048],
			fVec56: [0.0;4096],
			fVec57: [0.0;8192],
			fVec58: [0.0;2],
			fVec59: [0.0;2],
			fVec60: [0.0;2],
			fRec15: [0.0;2],
			fRec16: [0.0;16384],
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
		m.declare("compile_options", r"-a /run/user/1001/.tmpQkffti -lang rust -ct 1 -cn LambRs96k -es 1 -mcd 16 -mdd 1024 -mdy 33 -double -ftz 0");
		m.declare("filename", r"lamb-rs-96k.dsp");
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
		let mut sig0: LambRs96kSIG0 = newLambRs96kSIG0();
		sig0.instance_initLambRs96kSIG0(sample_rate);
		sig0.fillLambRs96kSIG0(393216, unsafe { &mut ftbl0LambRs96kSIG0 });
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
		self.fCheckbox1 = 0.0;
		self.fHslider7 = 5e+01;
		self.fHslider8 = 1e+02;
		self.fHslider9 = 0.0;
		self.fHslider10 = 0.0;
		self.fHslider11 = 0.5;
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
		for l46 in 0..16384 {
			self.fRec2[l46 as usize] = 0.0;
		}
		for l47 in 0..2 {
			self.fRec14[l47 as usize] = 0.0;
		}
		for l48 in 0..16384 {
			self.fVec33[l48 as usize] = 0.0;
		}
		for l49 in 0..3 {
			self.fVec34[l49 as usize] = 0.0;
		}
		for l50 in 0..7 {
			self.fVec35[l50 as usize] = 0.0;
		}
		for l51 in 0..15 {
			self.fVec36[l51 as usize] = 0.0;
		}
		for l52 in 0..32 {
			self.fVec37[l52 as usize] = 0.0;
		}
		for l53 in 0..64 {
			self.fVec38[l53 as usize] = 0.0;
		}
		for l54 in 0..128 {
			self.fVec39[l54 as usize] = 0.0;
		}
		for l55 in 0..256 {
			self.fVec40[l55 as usize] = 0.0;
		}
		for l56 in 0..512 {
			self.fVec41[l56 as usize] = 0.0;
		}
		for l57 in 0..1024 {
			self.fVec42[l57 as usize] = 0.0;
		}
		for l58 in 0..2048 {
			self.fVec43[l58 as usize] = 0.0;
		}
		for l59 in 0..4096 {
			self.fVec44[l59 as usize] = 0.0;
		}
		for l60 in 0..8192 {
			self.fVec45[l60 as usize] = 0.0;
		}
		for l61 in 0..2 {
			self.fRec17[l61 as usize] = 0.0;
		}
		for l62 in 0..3 {
			self.fVec46[l62 as usize] = 0.0;
		}
		for l63 in 0..7 {
			self.fVec47[l63 as usize] = 0.0;
		}
		for l64 in 0..15 {
			self.fVec48[l64 as usize] = 0.0;
		}
		for l65 in 0..32 {
			self.fVec49[l65 as usize] = 0.0;
		}
		for l66 in 0..64 {
			self.fVec50[l66 as usize] = 0.0;
		}
		for l67 in 0..128 {
			self.fVec51[l67 as usize] = 0.0;
		}
		for l68 in 0..256 {
			self.fVec52[l68 as usize] = 0.0;
		}
		for l69 in 0..512 {
			self.fVec53[l69 as usize] = 0.0;
		}
		for l70 in 0..1024 {
			self.fVec54[l70 as usize] = 0.0;
		}
		for l71 in 0..2048 {
			self.fVec55[l71 as usize] = 0.0;
		}
		for l72 in 0..4096 {
			self.fVec56[l72 as usize] = 0.0;
		}
		for l73 in 0..8192 {
			self.fVec57[l73 as usize] = 0.0;
		}
		for l74 in 0..2 {
			self.fVec58[l74 as usize] = 0.0;
		}
		for l75 in 0..2 {
			self.fVec59[l75 as usize] = 0.0;
		}
		for l76 in 0..2 {
			self.fVec60[l76 as usize] = 0.0;
		}
		for l77 in 0..2 {
			self.fRec15[l77 as usize] = 0.0;
		}
		for l78 in 0..16384 {
			self.fRec16[l78 as usize] = 0.0;
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
		ui_interface.add_horizontal_slider("release hold", ParamIndex(9), 5e+01, 0.010416666666666666, 5e+01, 1.0);
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
			9 => Some(self.fHslider7),
			3 => Some(self.fHslider8),
			11 => Some(self.fHslider9),
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
			9 => { self.fHslider7 = value }
			3 => { self.fHslider8 = value }
			11 => { self.fHslider9 = value }
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
		let mut fSlow15: F64 = self.fCheckbox1;
		let mut fSlow16: F64 = self.fHslider7;
		let mut fSlow17: F64 = self.fConst0 * (0.001 * fSlow16 + 1e-05 * fSlow2);
		self.fHbargraph0 = if (fSlow15) as i32 != 0 {9.6e+03} else {fSlow17};
		let mut iSlow18: i32 = (self.fHbargraph0) as i32;
		let mut fSlow19: F64 = self.fHslider8;
		let mut fSlow20: F64 = 0.002 * fSlow19;
		let mut fSlow21: F64 = 0.01 * self.fHslider9;
		let mut fSlow22: F64 = 0.0005 * fSlow19;
		let mut fSlow23: F64 = self.fConst9 * fSlow16;
		let mut iSlow24: i32 = (fSlow23) as i32;
		let mut fSlow25: F64 = fSlow23 + 1.0;
		let mut iSlow26: i32 = (F64::floor(fSlow25)) as i32 % 2;
		let mut iSlow27: i32 = (F64::floor(0.5 * fSlow25)) as i32 % 2;
		let mut iSlow28: i32 = (F64::floor(0.25 * fSlow25)) as i32 % 2;
		let mut iSlow29: i32 = i32::wrapping_add(iSlow26, i32::wrapping_mul(2, iSlow27));
		let mut iSlow30: i32 = (F64::floor(0.125 * fSlow25)) as i32 % 2;
		let mut iSlow31: i32 = i32::wrapping_add(iSlow29, i32::wrapping_mul(4, iSlow28));
		let mut iSlow32: i32 = (F64::floor(0.0625 * fSlow25)) as i32 % 2;
		let mut iSlow33: i32 = i32::wrapping_add(iSlow31, i32::wrapping_mul(8, iSlow30));
		let mut iSlow34: i32 = (F64::floor(0.03125 * fSlow25)) as i32 % 2;
		let mut iSlow35: i32 = i32::wrapping_add(iSlow33, i32::wrapping_mul(16, iSlow32));
		let mut iSlow36: i32 = (F64::floor(0.015625 * fSlow25)) as i32 % 2;
		let mut iSlow37: i32 = i32::wrapping_add(iSlow35, i32::wrapping_mul(32, iSlow34));
		let mut iSlow38: i32 = (F64::floor(0.0078125 * fSlow25)) as i32 % 2;
		let mut iSlow39: i32 = i32::wrapping_add(iSlow37, i32::wrapping_mul(64, iSlow36));
		let mut iSlow40: i32 = (F64::floor(0.00390625 * fSlow25)) as i32 % 2;
		let mut iSlow41: i32 = i32::wrapping_add(iSlow39, i32::wrapping_mul(128, iSlow38));
		let mut iSlow42: i32 = (F64::floor(0.001953125 * fSlow25)) as i32 % 2;
		let mut iSlow43: i32 = i32::wrapping_add(iSlow41, i32::wrapping_mul(256, iSlow40));
		let mut iSlow44: i32 = (F64::floor(0.0009765625 * fSlow25)) as i32 % 2;
		let mut iSlow45: i32 = i32::wrapping_add(iSlow43, i32::wrapping_mul(512, iSlow42));
		let mut iSlow46: i32 = (F64::floor(0.00048828125 * fSlow25)) as i32 % 2;
		let mut iSlow47: i32 = i32::wrapping_add(iSlow45, i32::wrapping_mul(1024, iSlow44));
		let mut iSlow48: i32 = (F64::floor(0.000244140625 * fSlow25)) as i32 % 2;
		let mut iSlow49: i32 = i32::wrapping_add(iSlow47, i32::wrapping_mul(2048, iSlow46));
		let mut iSlow50: i32 = (F64::floor(0.5 * fSlow3)) as i32 % 2;
		let mut iSlow51: i32 = (F64::floor(0.25 * fSlow3)) as i32 % 2;
		let mut iSlow52: i32 = i32::wrapping_add(iSlow4, i32::wrapping_mul(2, iSlow50));
		let mut iSlow53: i32 = (F64::floor(0.125 * fSlow3)) as i32 % 2;
		let mut iSlow54: i32 = i32::wrapping_add(iSlow52, i32::wrapping_mul(4, iSlow51));
		let mut iSlow55: i32 = (F64::floor(0.0625 * fSlow3)) as i32 % 2;
		let mut iSlow56: i32 = i32::wrapping_add(iSlow54, i32::wrapping_mul(8, iSlow53));
		let mut iSlow57: i32 = (F64::floor(0.03125 * fSlow3)) as i32 % 2;
		let mut iSlow58: i32 = i32::wrapping_add(iSlow56, i32::wrapping_mul(16, iSlow55));
		let mut iSlow59: i32 = (F64::floor(0.015625 * fSlow3)) as i32 % 2;
		let mut iSlow60: i32 = i32::wrapping_add(iSlow58, i32::wrapping_mul(32, iSlow57));
		let mut iSlow61: i32 = (F64::floor(0.0078125 * fSlow3)) as i32 % 2;
		let mut iSlow62: i32 = i32::wrapping_add(iSlow60, i32::wrapping_mul(64, iSlow59));
		let mut iSlow63: i32 = (F64::floor(0.00390625 * fSlow3)) as i32 % 2;
		let mut iSlow64: i32 = i32::wrapping_add(iSlow62, i32::wrapping_mul(128, iSlow61));
		let mut iSlow65: i32 = (F64::floor(0.001953125 * fSlow3)) as i32 % 2;
		let mut iSlow66: i32 = i32::wrapping_add(iSlow64, i32::wrapping_mul(256, iSlow63));
		let mut iSlow67: i32 = (F64::floor(0.0009765625 * fSlow3)) as i32 % 2;
		let mut iSlow68: i32 = i32::wrapping_add(iSlow66, i32::wrapping_mul(512, iSlow65));
		let mut iSlow69: i32 = (F64::floor(0.00048828125 * fSlow3)) as i32 % 2;
		let mut iSlow70: i32 = i32::wrapping_add(iSlow68, i32::wrapping_mul(1024, iSlow67));
		let mut iSlow71: i32 = (F64::floor(0.000244140625 * fSlow3)) as i32 % 2;
		let mut iSlow72: i32 = i32::wrapping_add(iSlow70, i32::wrapping_mul(2048, iSlow69));
		let mut fSlow73: F64 = self.fHslider10;
		let mut fSlow74: F64 = self.fHslider11;
		let mut iSlow75: i32 = (F64::max(0.0, fSlow15 * (9.6e+03 - fSlow17))) as i32;
		let mut fSlow76: F64 = self.fConst3 * F64::powf(1e+01, 0.05 * self.fHslider12);
		let zipped_iterators = inputs0.zip(inputs1).zip(outputs0).zip(outputs1).zip(outputs2).zip(outputs3);
		for (((((input0, input1), output0), output1), output2), output3) in zipped_iterators {
			self.iVec0[(self.IOTA0 & 32767) as usize] = 1;
			let mut fTemp0: F64 = self.fConst1 + self.fRec0[1];
			let mut fTemp1: F64 = self.fRec0[1] - self.fConst1;
			self.fRec0[0] = if (fTemp0 < fSlow0) as i32 != 0 {fTemp0} else {if (fTemp1 > fSlow0) as i32 != 0 {fTemp1} else {fSlow0}};
			let mut fTemp2: F64 = F64::sin(6.283185307179586 * (0.5 * self.fRec0[0] + 0.75)) + 1.0;
			let mut fTemp3: F64 = 0.5 * fTemp2;
			let mut fTemp4: F64 = 1.0 - fTemp3;
			let mut fTemp5: F64 = self.fRec2[((i32::wrapping_sub(self.IOTA0, 1)) & 16383) as usize];
			self.fRec4[0] = fSlow5 + self.fConst4 * self.fRec4[1];
			let mut fTemp6: F64 = F64::max(0.5, self.fRec4[0]) + -0.5;
			let mut fTemp7: F64 = 4.0 * fTemp6;
			let mut fTemp8: F64 = 10.588235294117647 * (F64::max(0.15, self.fRec4[0]) + -0.15);
			let mut fTemp9: F64 = 15.0 - (fTemp8 + fTemp7);
			let mut fTemp10: F64 = 12.0 - fTemp8;
			let mut fTemp11: F64 = fTemp8 + -12.0;
			let mut fTemp12: F64 = 3.0 - fTemp7;
			self.fRec11[0] = fSlow10 + self.fConst4 * self.fRec11[1];
			let mut fTemp13: F64 = *input0;
			self.fVec1[(self.IOTA0 & 32767) as usize] = fTemp13;
			let mut fTemp14: F64 = fTemp13 * self.fRec11[0];
			self.fVec2[(self.IOTA0 & 32767) as usize] = fTemp14;
			let mut fTemp15: F64 = F64::abs(fTemp14);
			let mut fTemp16: F64 = *input1;
			self.fVec3[(self.IOTA0 & 32767) as usize] = fTemp16;
			let mut fTemp17: F64 = fTemp16 * self.fRec11[0];
			self.fVec4[(self.IOTA0 & 32767) as usize] = fTemp17;
			let mut fTemp18: F64 = F64::abs(fTemp17);
			let mut fTemp19: F64 = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, F64::max(fTemp15, fTemp18)));
			let mut iTemp20: i32 = ((fTemp19 > fSlow11) as i32) + ((fTemp19 > fSlow9) as i32);
			let mut fTemp21: F64 = fTemp19 - fSlow8;
			let mut fTemp22: F64 = F64::powf(1e+01, -(0.05 * F64::max(0.0, if (iTemp20 == 0) as i32 != 0 {0.0} else {if (iTemp20 == 1) as i32 != 0 {fSlow12 * LambRs96k_faustpower2_f(fSlow7 + fTemp21)} else {fTemp21}})));
			let mut fTemp23: F64 = 3.0 * fTemp6;
			let mut fTemp24: F64 = 4.0 * (F64::max(0.25, self.fRec4[0]) + -0.25);
			let mut fTemp25: F64 = 9.0 - fTemp24;
			let mut fTemp26: F64 = self.fRec5[1] - self.fRec6[1];
			let mut fTemp27: F64 = if (fTemp22 > self.fRec10[1]) as i32 != 0 {F64::exp(-(self.fConst7 / F64::max(2.220446049250313e-16, fSlow14 / F64::max(1.0 - (F64::max(fTemp24 + -9.0, F64::min(2.0 - fTemp23, fTemp26)) + fTemp25) / (11.0 - (fTemp24 + fTemp23)), 2.220446049250313e-16))))} else {self.fConst6};
			self.fRec10[0] = self.fRec10[1] * fTemp27 + fTemp22 * (1.0 - fTemp27);
			let mut fTemp28: F64 = if (self.fRec10[0] > self.fRec9[1]) as i32 != 0 {0.0} else {self.fConst6};
			self.fRec9[0] = self.fRec9[1] * fTemp28 + self.fRec10[0] * (1.0 - fTemp28);
			let mut fTemp29: F64 = if (self.fRec9[0] > self.fRec8[1]) as i32 != 0 {0.0} else {self.fConst6};
			self.fRec8[0] = self.fRec8[1] * fTemp29 + self.fRec9[0] * (1.0 - fTemp29);
			let mut fTemp30: F64 = if (self.fRec8[0] > self.fRec7[1]) as i32 != 0 {0.0} else {self.fConst6};
			self.fRec7[0] = self.fRec7[1] * fTemp30 + self.fRec8[0] * (1.0 - fTemp30);
			self.fRec5[0] = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, self.fRec7[0]));
			let mut fTemp31: F64 = F64::powf(1e+01, 0.05 * (self.fRec5[1] + fTemp25));
			let mut fTemp32: F64 = if (fTemp31 > self.fRec12[1]) as i32 != 0 {F64::exp(-(self.fConst7 / F64::max(2.220446049250313e-16, (0.8161290322580644 * (F64::max(0.69, self.fRec4[0]) + -0.69) + 0.05) * F64::powf(4503599627370496.0, 1.0 - (F64::max(fTemp11, F64::min(fTemp12, fTemp26)) + fTemp10) / fTemp9))))} else {self.fConst8};
			self.fRec12[0] = self.fRec12[1] * fTemp32 + fTemp31 * (1.0 - fTemp32);
			self.fRec6[0] = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, self.fRec12[0])) * (self.iVec0[((i32::wrapping_sub(self.IOTA0, iSlow18)) & 32767) as usize]) as F64;
			let mut fTemp33: F64 = self.fRec5[0] - self.fRec6[0];
			let mut fTemp34: F64 = F64::powf(1e+01, fSlow20 * F64::min(0.25, self.fRec4[0]) * (self.fRec6[0] + fTemp33 * (F64::max(fTemp11, F64::min(fTemp12, fTemp33)) + fTemp10) / fTemp9));
			let mut fTemp35: F64 = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, fTemp15));
			let mut fTemp36: F64 = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, fTemp18));
			let mut fTemp37: F64 = F64::max(fTemp35, fTemp36);
			let mut fTemp38: F64 = fTemp35 + fSlow21 * (fTemp37 - fTemp35);
			let mut iTemp39: i32 = ((fTemp38 > fSlow11) as i32) + ((fTemp38 > fSlow9) as i32);
			let mut fTemp40: F64 = fTemp38 - fSlow8;
			let mut fTemp41: F64 = F64::min(fTemp34, F64::powf(1e+01, -(fSlow22 * F64::max(0.0, if (iTemp39 == 0) as i32 != 0 {0.0} else {if (iTemp39 == 1) as i32 != 0 {fSlow12 * LambRs96k_faustpower2_f(fSlow7 + fTemp40)} else {fTemp40}}))));
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
			let mut fTemp52: F64 = F64::min(fTemp51, self.fVec15[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			self.fVec16[(self.IOTA0 & 4095) as usize] = fTemp52;
			self.fVec17[(self.IOTA0 & 8191) as usize] = F64::min(fTemp52, self.fVec16[((i32::wrapping_sub(self.IOTA0, 2048)) & 4095) as usize]);
			self.fRec3[0] = F64::max(F64::min(self.fRec3[1], self.fVec5[((i32::wrapping_sub(self.IOTA0, iSlow24)) & 16383) as usize]), F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow26 != 0 {fTemp41} else {1.7976931348623157e+308}, if iSlow27 != 0 {self.fVec6[iSlow26 as usize]} else {1.7976931348623157e+308}), if iSlow28 != 0 {self.fVec7[iSlow29 as usize]} else {1.7976931348623157e+308}), if iSlow30 != 0 {self.fVec8[iSlow31 as usize]} else {1.7976931348623157e+308}), if iSlow32 != 0 {self.fVec9[((i32::wrapping_sub(self.IOTA0, iSlow33)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow34 != 0 {self.fVec10[((i32::wrapping_sub(self.IOTA0, iSlow35)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow36 != 0 {self.fVec11[((i32::wrapping_sub(self.IOTA0, iSlow37)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow38 != 0 {self.fVec12[((i32::wrapping_sub(self.IOTA0, iSlow39)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow40 != 0 {self.fVec13[((i32::wrapping_sub(self.IOTA0, iSlow41)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow42 != 0 {self.fVec14[((i32::wrapping_sub(self.IOTA0, iSlow43)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow44 != 0 {self.fVec15[((i32::wrapping_sub(self.IOTA0, iSlow45)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow46 != 0 {self.fVec16[((i32::wrapping_sub(self.IOTA0, iSlow47)) & 4095) as usize]} else {1.7976931348623157e+308}), if iSlow48 != 0 {self.fVec17[((i32::wrapping_sub(self.IOTA0, iSlow49)) & 8191) as usize]} else {1.7976931348623157e+308}));
			let mut fTemp53: F64 = F64::min(self.fRec3[0], self.fRec3[1]);
			self.fVec18[0] = fTemp53;
			let mut fTemp54: F64 = F64::min(fTemp53, self.fVec18[2]);
			self.fVec19[0] = fTemp54;
			let mut fTemp55: F64 = F64::min(fTemp54, self.fVec19[4]);
			self.fVec20[0] = fTemp55;
			let mut fTemp56: F64 = F64::min(fTemp55, self.fVec20[8]);
			self.fVec21[(self.IOTA0 & 31) as usize] = fTemp56;
			let mut fTemp57: F64 = F64::min(fTemp56, self.fVec21[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize]);
			self.fVec22[(self.IOTA0 & 63) as usize] = fTemp57;
			let mut fTemp58: F64 = F64::min(fTemp57, self.fVec22[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize]);
			self.fVec23[(self.IOTA0 & 127) as usize] = fTemp58;
			let mut fTemp59: F64 = F64::min(fTemp58, self.fVec23[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize]);
			self.fVec24[(self.IOTA0 & 255) as usize] = fTemp59;
			let mut fTemp60: F64 = F64::min(fTemp59, self.fVec24[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize]);
			self.fVec25[(self.IOTA0 & 511) as usize] = fTemp60;
			let mut fTemp61: F64 = F64::min(fTemp60, self.fVec25[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize]);
			self.fVec26[(self.IOTA0 & 1023) as usize] = fTemp61;
			let mut fTemp62: F64 = F64::min(fTemp61, self.fVec26[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize]);
			self.fVec27[(self.IOTA0 & 2047) as usize] = fTemp62;
			let mut fTemp63: F64 = F64::min(fTemp62, self.fVec27[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			self.fVec28[(self.IOTA0 & 4095) as usize] = fTemp63;
			self.fVec29[(self.IOTA0 & 8191) as usize] = F64::min(fTemp63, self.fVec28[((i32::wrapping_sub(self.IOTA0, 2048)) & 4095) as usize]);
			let mut fTemp64: F64 = F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow4 != 0 {self.fRec3[0]} else {1.7976931348623157e+308}, if iSlow50 != 0 {self.fVec18[iSlow4 as usize]} else {1.7976931348623157e+308}), if iSlow51 != 0 {self.fVec19[iSlow52 as usize]} else {1.7976931348623157e+308}), if iSlow53 != 0 {self.fVec20[iSlow54 as usize]} else {1.7976931348623157e+308}), if iSlow55 != 0 {self.fVec21[((i32::wrapping_sub(self.IOTA0, iSlow56)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow57 != 0 {self.fVec22[((i32::wrapping_sub(self.IOTA0, iSlow58)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow59 != 0 {self.fVec23[((i32::wrapping_sub(self.IOTA0, iSlow60)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow61 != 0 {self.fVec24[((i32::wrapping_sub(self.IOTA0, iSlow62)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow63 != 0 {self.fVec25[((i32::wrapping_sub(self.IOTA0, iSlow64)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow65 != 0 {self.fVec26[((i32::wrapping_sub(self.IOTA0, iSlow66)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow67 != 0 {self.fVec27[((i32::wrapping_sub(self.IOTA0, iSlow68)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow69 != 0 {self.fVec28[((i32::wrapping_sub(self.IOTA0, iSlow70)) & 4095) as usize]} else {1.7976931348623157e+308}), if iSlow71 != 0 {self.fVec29[((i32::wrapping_sub(self.IOTA0, iSlow72)) & 8191) as usize]} else {1.7976931348623157e+308}) - fTemp5;
			self.fVec30[0] = fTemp64;
			let mut iTemp65: i32 = (fTemp64 > 0.0) as i32;
			let mut fTemp71: F64 = if iTemp65 != 0 {fSlow74} else {fSlow73};
			self.fVec31[0] = fTemp71;
			let mut fTemp72: F64 = 2.0 * fTemp71;
			let mut iTemp73: i32 = (fTemp72) as i32;
			let mut iTemp74: i32 = std::cmp::max(0, std::cmp::min(iTemp73, 2));
			let mut iTemp75: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, 196605), 393215));
			let mut fTemp76: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp75, 3)) as usize] };
			let mut fTemp77: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp75 as usize] };
			let mut fTemp78: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp75, 1)) as usize] } - fTemp77;
			let mut fTemp79: F64 = fTemp72 - (iTemp73) as F64;
			let mut fTemp80: F64 = fTemp77 + fTemp79 * fTemp78 + 0.5 * (fTemp76 - (fTemp77 + fTemp79 * (fTemp78 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp75, 4)) as usize] } - fTemp76))));
			let mut fTemp81: F64 = if iTemp65 != 0 {fTemp80} else {1.0 - fTemp80};
			let mut iTemp82: i32 = (fTemp64 < 0.0) as i32;
			let mut fTemp83: F64 = fSlow1 * (iTemp82) as F64 + fSlow13 * (iTemp65) as F64;
			self.fVec32[0] = fTemp83;
			let mut fTemp84: F64 = self.fConst10 / fTemp83;
			let mut fTemp85: F64 = fTemp84 + 0.5;
			let mut fTemp86: F64 = 131071.0 * (1.0 - fTemp85);
			let mut iTemp87: i32 = (fTemp86) as i32;
			let mut iTemp88: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp87, 131071)))), 393215));
			let mut fTemp89: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp88, 3)) as usize] };
			let mut fTemp90: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp88 as usize] };
			let mut fTemp91: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp88, 1)) as usize] } - fTemp90;
			let mut fTemp92: F64 = 131071.0 * fTemp85;
			let mut iTemp93: i32 = (fTemp92) as i32;
			let mut iTemp94: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp93, 131071)))), 393215));
			let mut fTemp95: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp94, 3), 393215))) as usize] };
			let mut fTemp96: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp94 as usize] };
			let mut fTemp97: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp94, 1), 393215))) as usize] } - fTemp96;
			let mut fTemp98: F64 = 2.0 * self.fVec31[1];
			let mut iTemp99: i32 = (fTemp98) as i32;
			let mut iTemp100: i32 = std::cmp::max(0, std::cmp::min(iTemp99, 2));
			let mut fTemp101: F64 = 131071.0 * (1.0 - self.fRec1[1]);
			let mut iTemp102: i32 = (fTemp101) as i32;
			let mut iTemp103: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp102, 131071))), iTemp100), 393215));
			let mut fTemp104: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp103, 3), 393215))) as usize] };
			let mut fTemp105: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp103 as usize] };
			let mut fTemp106: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp103, 1), 393215))) as usize] } - fTemp105;
			let mut fTemp107: F64 = fTemp98 - (iTemp99) as F64;
			let mut fTemp108: F64 = 131071.0 * self.fRec1[1];
			let mut iTemp109: i32 = (fTemp108) as i32;
			let mut iTemp110: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp100, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp109, 131071)))), 393215));
			let mut fTemp111: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp110, 3), 393215))) as usize] };
			let mut fTemp112: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp110 as usize] };
			let mut fTemp113: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp110, 1), 393215))) as usize] } - fTemp112;
			let mut fTemp114: F64 = self.fRec1[1] + fTemp84;
			let mut fTemp115: F64 = 131071.0 * (1.0 - fTemp114);
			let mut iTemp116: i32 = (fTemp115) as i32;
			let mut iTemp117: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp116, 131071)))), 393215));
			let mut fTemp118: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp117, 3)) as usize] };
			let mut fTemp119: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp117 as usize] };
			let mut fTemp120: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp117, 1)) as usize] } - fTemp119;
			let mut fTemp121: F64 = 131071.0 * fTemp114;
			let mut iTemp122: i32 = (fTemp121) as i32;
			let mut iTemp123: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp122, 131071)))), 393215));
			let mut fTemp124: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp123, 3), 393215))) as usize] };
			let mut fTemp125: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp123 as usize] };
			let mut fTemp126: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp123, 1), 393215))) as usize] } - fTemp125;
			let mut fTemp127: F64 = self.fRec1[1] + self.fConst10 * (1.0 / fTemp83 + 1.0 / self.fVec32[1]);
			let mut fTemp128: F64 = 131071.0 * (1.0 - fTemp127);
			let mut iTemp129: i32 = (fTemp128) as i32;
			let mut iTemp130: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp129, 131071))), iTemp74), 393215));
			let mut fTemp131: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp130, 3)) as usize] };
			let mut fTemp132: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp130 as usize] };
			let mut fTemp133: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp130, 1)) as usize] } - fTemp132;
			let mut fTemp134: F64 = 131071.0 * fTemp127;
			let mut iTemp135: i32 = (fTemp134) as i32;
			let mut iTemp136: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp135, 131071)))), 393215));
			let mut fTemp137: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp136, 3), 393215))) as usize] };
			let mut fTemp138: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp136 as usize] };
			let mut fTemp139: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp136, 1), 393215))) as usize] } - fTemp138;
			let mut fTemp140: F64 = (if iTemp65 != 0 {fTemp138 + fTemp79 * fTemp139 + (fTemp134 - (iTemp135) as F64) * (fTemp137 - (fTemp138 + fTemp79 * (fTemp139 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp136, 4), 393215))) as usize] } - fTemp137))))} else {1.0 - (fTemp132 + fTemp79 * fTemp133 + (fTemp128 - (iTemp129) as F64) * (fTemp131 - (fTemp132 + fTemp79 * (fTemp133 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp130, 4)) as usize] } - fTemp131)))))} - if iTemp65 != 0 {fTemp125 + fTemp79 * fTemp126 + (fTemp121 - (iTemp122) as F64) * (fTemp124 - (fTemp125 + fTemp79 * (fTemp126 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp123, 4), 393215))) as usize] } - fTemp124))))} else {1.0 - (fTemp119 + fTemp79 * fTemp120 + (fTemp115 - (iTemp116) as F64) * (fTemp118 - (fTemp119 + fTemp79 * (fTemp120 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp117, 4)) as usize] } - fTemp118)))))}) * self.fVec30[1] / (fTemp64 * (1.0 - if iTemp65 != 0 {fTemp112 + fTemp107 * fTemp113 + (fTemp108 - (iTemp109) as F64) * (fTemp111 - (fTemp112 + fTemp107 * (fTemp113 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp110, 4), 393215))) as usize] } - fTemp111))))} else {1.0 - (fTemp105 + fTemp107 * fTemp106 + (fTemp101 - (iTemp102) as F64) * (fTemp104 - (fTemp105 + fTemp107 * (fTemp106 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp103, 4), 393215))) as usize] } - fTemp104)))))}));
			let mut iTemp141: i32 = (fTemp140 > ((if iTemp65 != 0 {fTemp96 + fTemp79 * fTemp97 + (fTemp92 - (iTemp93) as F64) * (fTemp95 - (fTemp96 + fTemp79 * (fTemp97 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp94, 4), 393215))) as usize] } - fTemp95))))} else {1.0 - (fTemp90 + fTemp79 * fTemp91 + (fTemp86 - (iTemp87) as F64) * (fTemp89 - (fTemp90 + fTemp79 * (fTemp91 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp88, 4)) as usize] } - fTemp89)))))} - fTemp81) / (1.0 - fTemp81))) as i32;
			let mut fTemp142: F64 = if iTemp141 != 0 {1.0} else {0.5};
			let mut fTemp143: F64 = if iTemp141 != 0 {0.5} else {0.0};
			let mut fTemp144: F64 = fTemp143 + fTemp142;
			let mut fTemp145: F64 = 0.5 * fTemp144;
			let mut fTemp146: F64 = 131071.0 * (1.0 - fTemp145);
			let mut iTemp147: i32 = (fTemp146) as i32;
			let mut iTemp148: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp147, 131071)))), 393215));
			let mut fTemp149: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp148, 3)) as usize] };
			let mut fTemp150: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp148 as usize] };
			let mut fTemp151: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp148, 1)) as usize] } - fTemp150;
			let mut fTemp152: F64 = 65535.5 * fTemp144;
			let mut iTemp153: i32 = (fTemp152) as i32;
			let mut iTemp154: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp153, 131071)))), 393215));
			let mut fTemp155: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp154, 3)) as usize] };
			let mut fTemp156: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp154 as usize] };
			let mut fTemp157: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp154, 1)) as usize] } - fTemp156;
			let mut fTemp158: F64 = if iTemp65 != 0 {fTemp156 + fTemp79 * fTemp157 + (fTemp152 - (iTemp153) as F64) * (fTemp155 - (fTemp156 + fTemp79 * (fTemp157 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp154, 4)) as usize] } - fTemp155))))} else {1.0 - (fTemp150 + fTemp79 * fTemp151 + (fTemp146 - (iTemp147) as F64) * (fTemp149 - (fTemp150 + fTemp79 * (fTemp151 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp148, 4)) as usize] } - fTemp149)))))};
			let mut fTemp159: F64 = fTemp84 + fTemp145;
			let mut fTemp160: F64 = 131071.0 * (1.0 - fTemp159);
			let mut iTemp161: i32 = (fTemp160) as i32;
			let mut iTemp162: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp161, 131071)))), 393215));
			let mut fTemp163: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp162, 3)) as usize] };
			let mut fTemp164: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp162 as usize] };
			let mut fTemp165: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp162, 1)) as usize] } - fTemp164;
			let mut fTemp166: F64 = 131071.0 * fTemp159;
			let mut iTemp167: i32 = (fTemp166) as i32;
			let mut iTemp168: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp167, 131071)))), 393215));
			let mut fTemp169: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp168, 3), 393215))) as usize] };
			let mut fTemp170: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp168 as usize] };
			let mut fTemp171: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp168, 1), 393215))) as usize] } - fTemp170;
			let mut iTemp172: i32 = (fTemp140 > ((if iTemp65 != 0 {fTemp170 + fTemp79 * fTemp171 + (fTemp166 - (iTemp167) as F64) * (fTemp169 - (fTemp170 + fTemp79 * (fTemp171 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp168, 4), 393215))) as usize] } - fTemp169))))} else {1.0 - (fTemp164 + fTemp79 * fTemp165 + (fTemp160 - (iTemp161) as F64) * (fTemp163 - (fTemp164 + fTemp79 * (fTemp165 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp162, 4)) as usize] } - fTemp163)))))} - fTemp158) / (1.0 - fTemp158))) as i32;
			let mut fTemp173: F64 = if iTemp172 != 0 {fTemp142} else {fTemp145};
			let mut fTemp174: F64 = if iTemp172 != 0 {fTemp145} else {fTemp143};
			let mut fTemp175: F64 = fTemp174 + fTemp173;
			let mut fTemp176: F64 = 0.5 * fTemp175;
			let mut fTemp177: F64 = 131071.0 * (1.0 - fTemp176);
			let mut iTemp178: i32 = (fTemp177) as i32;
			let mut iTemp179: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp178, 131071)))), 393215));
			let mut fTemp180: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp179, 3)) as usize] };
			let mut fTemp181: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp179 as usize] };
			let mut fTemp182: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp179, 1)) as usize] } - fTemp181;
			let mut fTemp183: F64 = 65535.5 * fTemp175;
			let mut iTemp184: i32 = (fTemp183) as i32;
			let mut iTemp185: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp184, 131071)))), 393215));
			let mut fTemp186: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp185, 3)) as usize] };
			let mut fTemp187: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp185 as usize] };
			let mut fTemp188: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp185, 1)) as usize] } - fTemp187;
			let mut fTemp189: F64 = if iTemp65 != 0 {fTemp187 + fTemp79 * fTemp188 + (fTemp183 - (iTemp184) as F64) * (fTemp186 - (fTemp187 + fTemp79 * (fTemp188 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp185, 4)) as usize] } - fTemp186))))} else {1.0 - (fTemp181 + fTemp79 * fTemp182 + (fTemp177 - (iTemp178) as F64) * (fTemp180 - (fTemp181 + fTemp79 * (fTemp182 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp179, 4)) as usize] } - fTemp180)))))};
			let mut fTemp190: F64 = fTemp84 + fTemp176;
			let mut fTemp191: F64 = 131071.0 * (1.0 - fTemp190);
			let mut iTemp192: i32 = (fTemp191) as i32;
			let mut iTemp193: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp192, 131071)))), 393215));
			let mut fTemp194: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp193, 3)) as usize] };
			let mut fTemp195: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp193 as usize] };
			let mut fTemp196: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp193, 1)) as usize] } - fTemp195;
			let mut fTemp197: F64 = 131071.0 * fTemp190;
			let mut iTemp198: i32 = (fTemp197) as i32;
			let mut iTemp199: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp198, 131071)))), 393215));
			let mut fTemp200: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp199, 3), 393215))) as usize] };
			let mut fTemp201: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp199 as usize] };
			let mut fTemp202: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp199, 1), 393215))) as usize] } - fTemp201;
			let mut iTemp203: i32 = (fTemp140 > ((if iTemp65 != 0 {fTemp201 + fTemp79 * fTemp202 + (fTemp197 - (iTemp198) as F64) * (fTemp200 - (fTemp201 + fTemp79 * (fTemp202 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp199, 4), 393215))) as usize] } - fTemp200))))} else {1.0 - (fTemp195 + fTemp79 * fTemp196 + (fTemp191 - (iTemp192) as F64) * (fTemp194 - (fTemp195 + fTemp79 * (fTemp196 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp193, 4)) as usize] } - fTemp194)))))} - fTemp189) / (1.0 - fTemp189))) as i32;
			let mut fTemp204: F64 = if iTemp203 != 0 {fTemp173} else {fTemp176};
			let mut fTemp205: F64 = if iTemp203 != 0 {fTemp176} else {fTemp174};
			let mut fTemp206: F64 = fTemp205 + fTemp204;
			let mut fTemp207: F64 = 0.5 * fTemp206;
			let mut fTemp208: F64 = 131071.0 * (1.0 - fTemp207);
			let mut iTemp209: i32 = (fTemp208) as i32;
			let mut iTemp210: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp209, 131071)))), 393215));
			let mut fTemp211: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp210, 3)) as usize] };
			let mut fTemp212: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp210 as usize] };
			let mut fTemp213: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp210, 1)) as usize] } - fTemp212;
			let mut fTemp214: F64 = 65535.5 * fTemp206;
			let mut iTemp215: i32 = (fTemp214) as i32;
			let mut iTemp216: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp215, 131071)))), 393215));
			let mut fTemp217: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp216, 3)) as usize] };
			let mut fTemp218: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp216 as usize] };
			let mut fTemp219: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp216, 1)) as usize] } - fTemp218;
			let mut fTemp220: F64 = if iTemp65 != 0 {fTemp218 + fTemp79 * fTemp219 + (fTemp214 - (iTemp215) as F64) * (fTemp217 - (fTemp218 + fTemp79 * (fTemp219 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp216, 4)) as usize] } - fTemp217))))} else {1.0 - (fTemp212 + fTemp79 * fTemp213 + (fTemp208 - (iTemp209) as F64) * (fTemp211 - (fTemp212 + fTemp79 * (fTemp213 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp210, 4)) as usize] } - fTemp211)))))};
			let mut fTemp221: F64 = fTemp84 + fTemp207;
			let mut fTemp222: F64 = 131071.0 * (1.0 - fTemp221);
			let mut iTemp223: i32 = (fTemp222) as i32;
			let mut iTemp224: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp223, 131071)))), 393215));
			let mut fTemp225: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp224, 3)) as usize] };
			let mut fTemp226: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp224 as usize] };
			let mut fTemp227: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp224, 1)) as usize] } - fTemp226;
			let mut fTemp228: F64 = 131071.0 * fTemp221;
			let mut iTemp229: i32 = (fTemp228) as i32;
			let mut iTemp230: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp229, 131071)))), 393215));
			let mut fTemp231: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp230, 3), 393215))) as usize] };
			let mut fTemp232: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp230 as usize] };
			let mut fTemp233: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp230, 1), 393215))) as usize] } - fTemp232;
			let mut iTemp234: i32 = (fTemp140 > ((if iTemp65 != 0 {fTemp232 + fTemp79 * fTemp233 + (fTemp228 - (iTemp229) as F64) * (fTemp231 - (fTemp232 + fTemp79 * (fTemp233 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp230, 4), 393215))) as usize] } - fTemp231))))} else {1.0 - (fTemp226 + fTemp79 * fTemp227 + (fTemp222 - (iTemp223) as F64) * (fTemp225 - (fTemp226 + fTemp79 * (fTemp227 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp224, 4)) as usize] } - fTemp225)))))} - fTemp220) / (1.0 - fTemp220))) as i32;
			let mut fTemp235: F64 = if iTemp234 != 0 {fTemp204} else {fTemp207};
			let mut fTemp236: F64 = if iTemp234 != 0 {fTemp207} else {fTemp205};
			let mut fTemp237: F64 = fTemp236 + fTemp235;
			let mut fTemp238: F64 = 0.5 * fTemp237;
			let mut fTemp239: F64 = 131071.0 * (1.0 - fTemp238);
			let mut iTemp240: i32 = (fTemp239) as i32;
			let mut iTemp241: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp240, 131071)))), 393215));
			let mut fTemp242: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp241, 3)) as usize] };
			let mut fTemp243: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp241 as usize] };
			let mut fTemp244: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp241, 1)) as usize] } - fTemp243;
			let mut fTemp245: F64 = 65535.5 * fTemp237;
			let mut iTemp246: i32 = (fTemp245) as i32;
			let mut iTemp247: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp246, 131071)))), 393215));
			let mut fTemp248: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp247, 3)) as usize] };
			let mut fTemp249: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp247 as usize] };
			let mut fTemp250: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp247, 1)) as usize] } - fTemp249;
			let mut fTemp251: F64 = if iTemp65 != 0 {fTemp249 + fTemp79 * fTemp250 + (fTemp245 - (iTemp246) as F64) * (fTemp248 - (fTemp249 + fTemp79 * (fTemp250 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp247, 4)) as usize] } - fTemp248))))} else {1.0 - (fTemp243 + fTemp79 * fTemp244 + (fTemp239 - (iTemp240) as F64) * (fTemp242 - (fTemp243 + fTemp79 * (fTemp244 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp241, 4)) as usize] } - fTemp242)))))};
			let mut fTemp252: F64 = fTemp84 + fTemp238;
			let mut fTemp253: F64 = 131071.0 * (1.0 - fTemp252);
			let mut iTemp254: i32 = (fTemp253) as i32;
			let mut iTemp255: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp254, 131071)))), 393215));
			let mut fTemp256: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp255, 3)) as usize] };
			let mut fTemp257: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp255 as usize] };
			let mut fTemp258: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp255, 1)) as usize] } - fTemp257;
			let mut fTemp259: F64 = 131071.0 * fTemp252;
			let mut iTemp260: i32 = (fTemp259) as i32;
			let mut iTemp261: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp260, 131071)))), 393215));
			let mut fTemp262: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp261, 3), 393215))) as usize] };
			let mut fTemp263: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp261 as usize] };
			let mut fTemp264: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp261, 1), 393215))) as usize] } - fTemp263;
			let mut iTemp265: i32 = (fTemp140 > ((if iTemp65 != 0 {fTemp263 + fTemp79 * fTemp264 + (fTemp259 - (iTemp260) as F64) * (fTemp262 - (fTemp263 + fTemp79 * (fTemp264 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp261, 4), 393215))) as usize] } - fTemp262))))} else {1.0 - (fTemp257 + fTemp79 * fTemp258 + (fTemp253 - (iTemp254) as F64) * (fTemp256 - (fTemp257 + fTemp79 * (fTemp258 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp255, 4)) as usize] } - fTemp256)))))} - fTemp251) / (1.0 - fTemp251))) as i32;
			let mut fTemp266: F64 = if iTemp265 != 0 {fTemp235} else {fTemp238};
			let mut fTemp267: F64 = if iTemp265 != 0 {fTemp238} else {fTemp236};
			let mut fTemp268: F64 = fTemp267 + fTemp266;
			let mut fTemp269: F64 = 0.5 * fTemp268;
			let mut fTemp270: F64 = 131071.0 * (1.0 - fTemp269);
			let mut iTemp271: i32 = (fTemp270) as i32;
			let mut iTemp272: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp271, 131071)))), 393215));
			let mut fTemp273: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp272, 3)) as usize] };
			let mut fTemp274: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp272 as usize] };
			let mut fTemp275: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp272, 1)) as usize] } - fTemp274;
			let mut fTemp276: F64 = 65535.5 * fTemp268;
			let mut iTemp277: i32 = (fTemp276) as i32;
			let mut iTemp278: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp277, 131071)))), 393215));
			let mut fTemp279: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp278, 3)) as usize] };
			let mut fTemp280: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp278 as usize] };
			let mut fTemp281: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp278, 1)) as usize] } - fTemp280;
			let mut fTemp282: F64 = if iTemp65 != 0 {fTemp280 + fTemp79 * fTemp281 + (fTemp276 - (iTemp277) as F64) * (fTemp279 - (fTemp280 + fTemp79 * (fTemp281 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp278, 4)) as usize] } - fTemp279))))} else {1.0 - (fTemp274 + fTemp79 * fTemp275 + (fTemp270 - (iTemp271) as F64) * (fTemp273 - (fTemp274 + fTemp79 * (fTemp275 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp272, 4)) as usize] } - fTemp273)))))};
			let mut fTemp283: F64 = fTemp84 + fTemp269;
			let mut fTemp284: F64 = 131071.0 * (1.0 - fTemp283);
			let mut iTemp285: i32 = (fTemp284) as i32;
			let mut iTemp286: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp285, 131071)))), 393215));
			let mut fTemp287: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp286, 3)) as usize] };
			let mut fTemp288: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp286 as usize] };
			let mut fTemp289: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp286, 1)) as usize] } - fTemp288;
			let mut fTemp290: F64 = 131071.0 * fTemp283;
			let mut iTemp291: i32 = (fTemp290) as i32;
			let mut iTemp292: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp291, 131071)))), 393215));
			let mut fTemp293: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp292, 3), 393215))) as usize] };
			let mut fTemp294: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp292 as usize] };
			let mut fTemp295: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp292, 1), 393215))) as usize] } - fTemp294;
			let mut iTemp296: i32 = (fTemp140 > ((if iTemp65 != 0 {fTemp294 + fTemp79 * fTemp295 + (fTemp290 - (iTemp291) as F64) * (fTemp293 - (fTemp294 + fTemp79 * (fTemp295 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp292, 4), 393215))) as usize] } - fTemp293))))} else {1.0 - (fTemp288 + fTemp79 * fTemp289 + (fTemp284 - (iTemp285) as F64) * (fTemp287 - (fTemp288 + fTemp79 * (fTemp289 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp286, 4)) as usize] } - fTemp287)))))} - fTemp282) / (1.0 - fTemp282))) as i32;
			let mut fTemp297: F64 = if iTemp296 != 0 {fTemp266} else {fTemp269};
			let mut fTemp298: F64 = if iTemp296 != 0 {fTemp269} else {fTemp267};
			let mut fTemp299: F64 = fTemp298 + fTemp297;
			let mut fTemp300: F64 = 0.5 * fTemp299;
			let mut fTemp301: F64 = 131071.0 * (1.0 - fTemp300);
			let mut iTemp302: i32 = (fTemp301) as i32;
			let mut iTemp303: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp302, 131071)))), 393215));
			let mut fTemp304: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp303, 3)) as usize] };
			let mut fTemp305: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp303 as usize] };
			let mut fTemp306: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp303, 1)) as usize] } - fTemp305;
			let mut fTemp307: F64 = 65535.5 * fTemp299;
			let mut iTemp308: i32 = (fTemp307) as i32;
			let mut iTemp309: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp308, 131071)))), 393215));
			let mut fTemp310: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp309, 3)) as usize] };
			let mut fTemp311: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp309 as usize] };
			let mut fTemp312: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp309, 1)) as usize] } - fTemp311;
			let mut fTemp313: F64 = if iTemp65 != 0 {fTemp311 + fTemp79 * fTemp312 + (fTemp307 - (iTemp308) as F64) * (fTemp310 - (fTemp311 + fTemp79 * (fTemp312 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp309, 4)) as usize] } - fTemp310))))} else {1.0 - (fTemp305 + fTemp79 * fTemp306 + (fTemp301 - (iTemp302) as F64) * (fTemp304 - (fTemp305 + fTemp79 * (fTemp306 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp303, 4)) as usize] } - fTemp304)))))};
			let mut fTemp314: F64 = fTemp84 + fTemp300;
			let mut fTemp315: F64 = 131071.0 * (1.0 - fTemp314);
			let mut iTemp316: i32 = (fTemp315) as i32;
			let mut iTemp317: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp316, 131071)))), 393215));
			let mut fTemp318: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp317, 3)) as usize] };
			let mut fTemp319: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp317 as usize] };
			let mut fTemp320: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp317, 1)) as usize] } - fTemp319;
			let mut fTemp321: F64 = 131071.0 * fTemp314;
			let mut iTemp322: i32 = (fTemp321) as i32;
			let mut iTemp323: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp322, 131071)))), 393215));
			let mut fTemp324: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp323, 3), 393215))) as usize] };
			let mut fTemp325: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp323 as usize] };
			let mut fTemp326: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp323, 1), 393215))) as usize] } - fTemp325;
			let mut iTemp327: i32 = (fTemp140 > ((if iTemp65 != 0 {fTemp325 + fTemp79 * fTemp326 + (fTemp321 - (iTemp322) as F64) * (fTemp324 - (fTemp325 + fTemp79 * (fTemp326 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp323, 4), 393215))) as usize] } - fTemp324))))} else {1.0 - (fTemp319 + fTemp79 * fTemp320 + (fTemp315 - (iTemp316) as F64) * (fTemp318 - (fTemp319 + fTemp79 * (fTemp320 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp317, 4)) as usize] } - fTemp318)))))} - fTemp313) / (1.0 - fTemp313))) as i32;
			let mut fTemp328: F64 = if iTemp327 != 0 {fTemp297} else {fTemp300};
			let mut fTemp329: F64 = if iTemp327 != 0 {fTemp300} else {fTemp298};
			let mut fTemp330: F64 = fTemp329 + fTemp328;
			let mut fTemp331: F64 = 0.5 * fTemp330;
			let mut fTemp332: F64 = 131071.0 * (1.0 - fTemp331);
			let mut iTemp333: i32 = (fTemp332) as i32;
			let mut iTemp334: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp333, 131071)))), 393215));
			let mut fTemp335: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp334, 3)) as usize] };
			let mut fTemp336: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp334 as usize] };
			let mut fTemp337: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp334, 1)) as usize] } - fTemp336;
			let mut fTemp338: F64 = 65535.5 * fTemp330;
			let mut iTemp339: i32 = (fTemp338) as i32;
			let mut iTemp340: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp339, 131071)))), 393215));
			let mut fTemp341: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp340, 3)) as usize] };
			let mut fTemp342: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp340 as usize] };
			let mut fTemp343: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp340, 1)) as usize] } - fTemp342;
			let mut fTemp344: F64 = if iTemp65 != 0 {fTemp342 + fTemp79 * fTemp343 + (fTemp338 - (iTemp339) as F64) * (fTemp341 - (fTemp342 + fTemp79 * (fTemp343 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp340, 4)) as usize] } - fTemp341))))} else {1.0 - (fTemp336 + fTemp79 * fTemp337 + (fTemp332 - (iTemp333) as F64) * (fTemp335 - (fTemp336 + fTemp79 * (fTemp337 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp334, 4)) as usize] } - fTemp335)))))};
			let mut fTemp345: F64 = fTemp84 + fTemp331;
			let mut fTemp346: F64 = 131071.0 * (1.0 - fTemp345);
			let mut iTemp347: i32 = (fTemp346) as i32;
			let mut iTemp348: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp347, 131071)))), 393215));
			let mut fTemp349: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp348, 3)) as usize] };
			let mut fTemp350: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp348 as usize] };
			let mut fTemp351: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp348, 1)) as usize] } - fTemp350;
			let mut fTemp352: F64 = 131071.0 * fTemp345;
			let mut iTemp353: i32 = (fTemp352) as i32;
			let mut iTemp354: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp353, 131071)))), 393215));
			let mut fTemp355: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp354, 3), 393215))) as usize] };
			let mut fTemp356: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp354 as usize] };
			let mut fTemp357: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp354, 1), 393215))) as usize] } - fTemp356;
			let mut iTemp358: i32 = (fTemp140 > ((if iTemp65 != 0 {fTemp356 + fTemp79 * fTemp357 + (fTemp352 - (iTemp353) as F64) * (fTemp355 - (fTemp356 + fTemp79 * (fTemp357 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp354, 4), 393215))) as usize] } - fTemp355))))} else {1.0 - (fTemp350 + fTemp79 * fTemp351 + (fTemp346 - (iTemp347) as F64) * (fTemp349 - (fTemp350 + fTemp79 * (fTemp351 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp348, 4)) as usize] } - fTemp349)))))} - fTemp344) / (1.0 - fTemp344))) as i32;
			let mut fTemp359: F64 = if iTemp358 != 0 {fTemp328} else {fTemp331};
			let mut fTemp360: F64 = if iTemp358 != 0 {fTemp331} else {fTemp329};
			let mut fTemp361: F64 = fTemp360 + fTemp359;
			let mut fTemp362: F64 = 0.5 * fTemp361;
			let mut fTemp363: F64 = 131071.0 * (1.0 - fTemp362);
			let mut iTemp364: i32 = (fTemp363) as i32;
			let mut iTemp365: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp364, 131071)))), 393215));
			let mut fTemp366: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp365, 3)) as usize] };
			let mut fTemp367: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp365 as usize] };
			let mut fTemp368: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp365, 1)) as usize] } - fTemp367;
			let mut fTemp369: F64 = 65535.5 * fTemp361;
			let mut iTemp370: i32 = (fTemp369) as i32;
			let mut iTemp371: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp370, 131071)))), 393215));
			let mut fTemp372: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp371, 3)) as usize] };
			let mut fTemp373: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp371 as usize] };
			let mut fTemp374: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp371, 1)) as usize] } - fTemp373;
			let mut fTemp375: F64 = if iTemp65 != 0 {fTemp373 + fTemp79 * fTemp374 + (fTemp369 - (iTemp370) as F64) * (fTemp372 - (fTemp373 + fTemp79 * (fTemp374 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp371, 4)) as usize] } - fTemp372))))} else {1.0 - (fTemp367 + fTemp79 * fTemp368 + (fTemp363 - (iTemp364) as F64) * (fTemp366 - (fTemp367 + fTemp79 * (fTemp368 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp365, 4)) as usize] } - fTemp366)))))};
			let mut fTemp376: F64 = fTemp84 + fTemp362;
			let mut fTemp377: F64 = 131071.0 * (1.0 - fTemp376);
			let mut iTemp378: i32 = (fTemp377) as i32;
			let mut iTemp379: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp378, 131071)))), 393215));
			let mut fTemp380: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp379, 3)) as usize] };
			let mut fTemp381: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp379 as usize] };
			let mut fTemp382: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp379, 1)) as usize] } - fTemp381;
			let mut fTemp383: F64 = 131071.0 * fTemp376;
			let mut iTemp384: i32 = (fTemp383) as i32;
			let mut iTemp385: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp384, 131071)))), 393215));
			let mut fTemp386: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp385, 3), 393215))) as usize] };
			let mut fTemp387: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp385 as usize] };
			let mut fTemp388: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp385, 1), 393215))) as usize] } - fTemp387;
			let mut iTemp389: i32 = (fTemp140 > ((if iTemp65 != 0 {fTemp387 + fTemp79 * fTemp388 + (fTemp383 - (iTemp384) as F64) * (fTemp386 - (fTemp387 + fTemp79 * (fTemp388 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp385, 4), 393215))) as usize] } - fTemp386))))} else {1.0 - (fTemp381 + fTemp79 * fTemp382 + (fTemp377 - (iTemp378) as F64) * (fTemp380 - (fTemp381 + fTemp79 * (fTemp382 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp379, 4)) as usize] } - fTemp380)))))} - fTemp375) / (1.0 - fTemp375))) as i32;
			let mut fTemp390: F64 = if iTemp389 != 0 {fTemp359} else {fTemp362};
			let mut fTemp391: F64 = if iTemp389 != 0 {fTemp362} else {fTemp360};
			let mut fTemp392: F64 = fTemp391 + fTemp390;
			let mut fTemp393: F64 = 0.5 * fTemp392;
			let mut fTemp394: F64 = 131071.0 * (1.0 - fTemp393);
			let mut iTemp395: i32 = (fTemp394) as i32;
			let mut iTemp396: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp395, 131071)))), 393215));
			let mut fTemp397: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp396, 3)) as usize] };
			let mut fTemp398: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp396 as usize] };
			let mut fTemp399: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp396, 1)) as usize] } - fTemp398;
			let mut fTemp400: F64 = 65535.5 * fTemp392;
			let mut iTemp401: i32 = (fTemp400) as i32;
			let mut iTemp402: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp401, 131071)))), 393215));
			let mut fTemp403: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp402, 3)) as usize] };
			let mut fTemp404: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp402 as usize] };
			let mut fTemp405: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp402, 1)) as usize] } - fTemp404;
			let mut fTemp406: F64 = if iTemp65 != 0 {fTemp404 + fTemp79 * fTemp405 + (fTemp400 - (iTemp401) as F64) * (fTemp403 - (fTemp404 + fTemp79 * (fTemp405 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp402, 4)) as usize] } - fTemp403))))} else {1.0 - (fTemp398 + fTemp79 * fTemp399 + (fTemp394 - (iTemp395) as F64) * (fTemp397 - (fTemp398 + fTemp79 * (fTemp399 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp396, 4)) as usize] } - fTemp397)))))};
			let mut fTemp407: F64 = fTemp84 + fTemp393;
			let mut fTemp408: F64 = 131071.0 * (1.0 - fTemp407);
			let mut iTemp409: i32 = (fTemp408) as i32;
			let mut iTemp410: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp409, 131071)))), 393215));
			let mut fTemp411: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp410, 3)) as usize] };
			let mut fTemp412: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp410 as usize] };
			let mut fTemp413: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp410, 1)) as usize] } - fTemp412;
			let mut fTemp414: F64 = 131071.0 * fTemp407;
			let mut iTemp415: i32 = (fTemp414) as i32;
			let mut iTemp416: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp415, 131071)))), 393215));
			let mut fTemp417: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp416, 3), 393215))) as usize] };
			let mut fTemp418: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp416 as usize] };
			let mut fTemp419: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp416, 1), 393215))) as usize] } - fTemp418;
			let mut iTemp420: i32 = (fTemp140 > ((if iTemp65 != 0 {fTemp418 + fTemp79 * fTemp419 + (fTemp414 - (iTemp415) as F64) * (fTemp417 - (fTemp418 + fTemp79 * (fTemp419 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp416, 4), 393215))) as usize] } - fTemp417))))} else {1.0 - (fTemp412 + fTemp79 * fTemp413 + (fTemp408 - (iTemp409) as F64) * (fTemp411 - (fTemp412 + fTemp79 * (fTemp413 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp410, 4)) as usize] } - fTemp411)))))} - fTemp406) / (1.0 - fTemp406))) as i32;
			let mut fTemp421: F64 = if iTemp420 != 0 {fTemp390} else {fTemp393};
			let mut fTemp422: F64 = if iTemp420 != 0 {fTemp393} else {fTemp391};
			let mut fTemp423: F64 = fTemp422 + fTemp421;
			let mut fTemp424: F64 = 0.5 * fTemp423;
			let mut fTemp425: F64 = 131071.0 * (1.0 - fTemp424);
			let mut iTemp426: i32 = (fTemp425) as i32;
			let mut iTemp427: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp426, 131071)))), 393215));
			let mut fTemp428: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp427, 3)) as usize] };
			let mut fTemp429: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp427 as usize] };
			let mut fTemp430: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp427, 1)) as usize] } - fTemp429;
			let mut fTemp431: F64 = 65535.5 * fTemp423;
			let mut iTemp432: i32 = (fTemp431) as i32;
			let mut iTemp433: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp432, 131071)))), 393215));
			let mut fTemp434: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp433, 3)) as usize] };
			let mut fTemp435: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp433 as usize] };
			let mut fTemp436: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp433, 1)) as usize] } - fTemp435;
			let mut fTemp437: F64 = if iTemp65 != 0 {fTemp435 + fTemp79 * fTemp436 + (fTemp431 - (iTemp432) as F64) * (fTemp434 - (fTemp435 + fTemp79 * (fTemp436 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp433, 4)) as usize] } - fTemp434))))} else {1.0 - (fTemp429 + fTemp79 * fTemp430 + (fTemp425 - (iTemp426) as F64) * (fTemp428 - (fTemp429 + fTemp79 * (fTemp430 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp427, 4)) as usize] } - fTemp428)))))};
			let mut fTemp438: F64 = fTemp84 + fTemp424;
			let mut fTemp439: F64 = 131071.0 * (1.0 - fTemp438);
			let mut iTemp440: i32 = (fTemp439) as i32;
			let mut iTemp441: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp440, 131071)))), 393215));
			let mut fTemp442: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp441, 3)) as usize] };
			let mut fTemp443: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp441 as usize] };
			let mut fTemp444: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp441, 1)) as usize] } - fTemp443;
			let mut fTemp445: F64 = 131071.0 * fTemp438;
			let mut iTemp446: i32 = (fTemp445) as i32;
			let mut iTemp447: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp446, 131071)))), 393215));
			let mut fTemp448: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp447, 3), 393215))) as usize] };
			let mut fTemp449: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp447 as usize] };
			let mut fTemp450: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp447, 1), 393215))) as usize] } - fTemp449;
			let mut iTemp451: i32 = (fTemp140 > ((if iTemp65 != 0 {fTemp449 + fTemp79 * fTemp450 + (fTemp445 - (iTemp446) as F64) * (fTemp448 - (fTemp449 + fTemp79 * (fTemp450 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp447, 4), 393215))) as usize] } - fTemp448))))} else {1.0 - (fTemp443 + fTemp79 * fTemp444 + (fTemp439 - (iTemp440) as F64) * (fTemp442 - (fTemp443 + fTemp79 * (fTemp444 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp441, 4)) as usize] } - fTemp442)))))} - fTemp437) / (1.0 - fTemp437))) as i32;
			let mut fTemp452: F64 = if iTemp451 != 0 {fTemp421} else {fTemp424};
			let mut fTemp453: F64 = if iTemp451 != 0 {fTemp424} else {fTemp422};
			let mut fTemp454: F64 = fTemp453 + fTemp452;
			let mut fTemp455: F64 = 0.5 * fTemp454;
			let mut fTemp456: F64 = 131071.0 * (1.0 - fTemp455);
			let mut iTemp457: i32 = (fTemp456) as i32;
			let mut iTemp458: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp457, 131071)))), 393215));
			let mut fTemp459: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp458, 3)) as usize] };
			let mut fTemp460: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp458 as usize] };
			let mut fTemp461: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp458, 1)) as usize] } - fTemp460;
			let mut fTemp462: F64 = 65535.5 * fTemp454;
			let mut iTemp463: i32 = (fTemp462) as i32;
			let mut iTemp464: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp463, 131071)))), 393215));
			let mut fTemp465: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp464, 3)) as usize] };
			let mut fTemp466: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp464 as usize] };
			let mut fTemp467: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp464, 1)) as usize] } - fTemp466;
			let mut fTemp468: F64 = if iTemp65 != 0 {fTemp466 + fTemp79 * fTemp467 + (fTemp462 - (iTemp463) as F64) * (fTemp465 - (fTemp466 + fTemp79 * (fTemp467 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp464, 4)) as usize] } - fTemp465))))} else {1.0 - (fTemp460 + fTemp79 * fTemp461 + (fTemp456 - (iTemp457) as F64) * (fTemp459 - (fTemp460 + fTemp79 * (fTemp461 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp458, 4)) as usize] } - fTemp459)))))};
			let mut fTemp469: F64 = fTemp84 + fTemp455;
			let mut fTemp470: F64 = 131071.0 * (1.0 - fTemp469);
			let mut iTemp471: i32 = (fTemp470) as i32;
			let mut iTemp472: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp471, 131071)))), 393215));
			let mut fTemp473: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp472, 3)) as usize] };
			let mut fTemp474: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp472 as usize] };
			let mut fTemp475: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp472, 1)) as usize] } - fTemp474;
			let mut fTemp476: F64 = 131071.0 * fTemp469;
			let mut iTemp477: i32 = (fTemp476) as i32;
			let mut iTemp478: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp477, 131071)))), 393215));
			let mut fTemp479: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp478, 3), 393215))) as usize] };
			let mut fTemp480: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp478 as usize] };
			let mut fTemp481: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp478, 1), 393215))) as usize] } - fTemp480;
			let mut iTemp482: i32 = (fTemp140 > ((if iTemp65 != 0 {fTemp480 + fTemp79 * fTemp481 + (fTemp476 - (iTemp477) as F64) * (fTemp479 - (fTemp480 + fTemp79 * (fTemp481 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp478, 4), 393215))) as usize] } - fTemp479))))} else {1.0 - (fTemp474 + fTemp79 * fTemp475 + (fTemp470 - (iTemp471) as F64) * (fTemp473 - (fTemp474 + fTemp79 * (fTemp475 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp472, 4)) as usize] } - fTemp473)))))} - fTemp468) / (1.0 - fTemp468))) as i32;
			let mut fTemp483: F64 = if iTemp482 != 0 {fTemp452} else {fTemp455};
			let mut fTemp484: F64 = if iTemp482 != 0 {fTemp455} else {fTemp453};
			let mut fTemp485: F64 = fTemp484 + fTemp483;
			let mut fTemp486: F64 = 0.5 * fTemp485;
			let mut fTemp487: F64 = 131071.0 * (1.0 - fTemp486);
			let mut iTemp488: i32 = (fTemp487) as i32;
			let mut iTemp489: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp488, 131071)))), 393215));
			let mut fTemp490: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp489, 3)) as usize] };
			let mut fTemp491: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp489 as usize] };
			let mut fTemp492: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp489, 1)) as usize] } - fTemp491;
			let mut fTemp493: F64 = 65535.5 * fTemp485;
			let mut iTemp494: i32 = (fTemp493) as i32;
			let mut iTemp495: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp494, 131071)))), 393215));
			let mut fTemp496: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp495, 3)) as usize] };
			let mut fTemp497: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp495 as usize] };
			let mut fTemp498: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp495, 1)) as usize] } - fTemp497;
			let mut fTemp499: F64 = if iTemp65 != 0 {fTemp497 + fTemp79 * fTemp498 + (fTemp493 - (iTemp494) as F64) * (fTemp496 - (fTemp497 + fTemp79 * (fTemp498 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp495, 4)) as usize] } - fTemp496))))} else {1.0 - (fTemp491 + fTemp79 * fTemp492 + (fTemp487 - (iTemp488) as F64) * (fTemp490 - (fTemp491 + fTemp79 * (fTemp492 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp489, 4)) as usize] } - fTemp490)))))};
			let mut fTemp500: F64 = fTemp84 + fTemp486;
			let mut fTemp501: F64 = 131071.0 * (1.0 - fTemp500);
			let mut iTemp502: i32 = (fTemp501) as i32;
			let mut iTemp503: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp502, 131071)))), 393215));
			let mut fTemp504: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp503, 3)) as usize] };
			let mut fTemp505: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp503 as usize] };
			let mut fTemp506: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp503, 1)) as usize] } - fTemp505;
			let mut fTemp507: F64 = 131071.0 * fTemp500;
			let mut iTemp508: i32 = (fTemp507) as i32;
			let mut iTemp509: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp508, 131071)))), 393215));
			let mut fTemp510: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp509, 3), 393215))) as usize] };
			let mut fTemp511: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp509 as usize] };
			let mut fTemp512: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp509, 1), 393215))) as usize] } - fTemp511;
			let mut iTemp513: i32 = (fTemp140 > ((if iTemp65 != 0 {fTemp511 + fTemp79 * fTemp512 + (fTemp507 - (iTemp508) as F64) * (fTemp510 - (fTemp511 + fTemp79 * (fTemp512 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp509, 4), 393215))) as usize] } - fTemp510))))} else {1.0 - (fTemp505 + fTemp79 * fTemp506 + (fTemp501 - (iTemp502) as F64) * (fTemp504 - (fTemp505 + fTemp79 * (fTemp506 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp503, 4)) as usize] } - fTemp504)))))} - fTemp499) / (1.0 - fTemp499))) as i32;
			let mut fTemp514: F64 = if iTemp513 != 0 {fTemp483} else {fTemp486};
			let mut fTemp515: F64 = if iTemp513 != 0 {fTemp486} else {fTemp484};
			let mut fTemp516: F64 = fTemp515 + fTemp514;
			let mut fTemp517: F64 = 0.5 * fTemp516;
			let mut fTemp518: F64 = 131071.0 * (1.0 - fTemp517);
			let mut iTemp519: i32 = (fTemp518) as i32;
			let mut iTemp520: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp519, 131071)))), 393215));
			let mut fTemp521: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp520, 3)) as usize] };
			let mut fTemp522: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp520 as usize] };
			let mut fTemp523: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp520, 1)) as usize] } - fTemp522;
			let mut fTemp524: F64 = 65535.5 * fTemp516;
			let mut iTemp525: i32 = (fTemp524) as i32;
			let mut iTemp526: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp525, 131071)))), 393215));
			let mut fTemp527: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp526, 3)) as usize] };
			let mut fTemp528: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp526 as usize] };
			let mut fTemp529: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp526, 1)) as usize] } - fTemp528;
			let mut fTemp530: F64 = if iTemp65 != 0 {fTemp528 + fTemp79 * fTemp529 + (fTemp524 - (iTemp525) as F64) * (fTemp527 - (fTemp528 + fTemp79 * (fTemp529 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp526, 4)) as usize] } - fTemp527))))} else {1.0 - (fTemp522 + fTemp79 * fTemp523 + (fTemp518 - (iTemp519) as F64) * (fTemp521 - (fTemp522 + fTemp79 * (fTemp523 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp520, 4)) as usize] } - fTemp521)))))};
			let mut fTemp531: F64 = fTemp84 + fTemp517;
			let mut fTemp532: F64 = 131071.0 * (1.0 - fTemp531);
			let mut iTemp533: i32 = (fTemp532) as i32;
			let mut iTemp534: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp533, 131071)))), 393215));
			let mut fTemp535: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp534, 3)) as usize] };
			let mut fTemp536: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp534 as usize] };
			let mut fTemp537: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp534, 1)) as usize] } - fTemp536;
			let mut fTemp538: F64 = 131071.0 * fTemp531;
			let mut iTemp539: i32 = (fTemp538) as i32;
			let mut iTemp540: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp539, 131071)))), 393215));
			let mut fTemp541: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp540, 3), 393215))) as usize] };
			let mut fTemp542: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp540 as usize] };
			let mut fTemp543: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp540, 1), 393215))) as usize] } - fTemp542;
			let mut iTemp544: i32 = (fTemp140 > ((if iTemp65 != 0 {fTemp542 + fTemp79 * fTemp543 + (fTemp538 - (iTemp539) as F64) * (fTemp541 - (fTemp542 + fTemp79 * (fTemp543 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp540, 4), 393215))) as usize] } - fTemp541))))} else {1.0 - (fTemp536 + fTemp79 * fTemp537 + (fTemp532 - (iTemp533) as F64) * (fTemp535 - (fTemp536 + fTemp79 * (fTemp537 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp534, 4)) as usize] } - fTemp535)))))} - fTemp530) / (1.0 - fTemp530))) as i32;
			let mut fTemp545: F64 = if iTemp544 != 0 {fTemp514} else {fTemp517};
			let mut fTemp546: F64 = if iTemp544 != 0 {fTemp517} else {fTemp515};
			let mut fTemp547: F64 = fTemp546 + fTemp545;
			let mut fTemp548: F64 = 0.5 * fTemp547;
			let mut fTemp549: F64 = 131071.0 * (1.0 - fTemp548);
			let mut iTemp550: i32 = (fTemp549) as i32;
			let mut iTemp551: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp550, 131071)))), 393215));
			let mut fTemp552: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp551, 3)) as usize] };
			let mut fTemp553: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp551 as usize] };
			let mut fTemp554: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp551, 1)) as usize] } - fTemp553;
			let mut fTemp555: F64 = 65535.5 * fTemp547;
			let mut iTemp556: i32 = (fTemp555) as i32;
			let mut iTemp557: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp556, 131071)))), 393215));
			let mut fTemp558: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp557, 3)) as usize] };
			let mut fTemp559: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp557 as usize] };
			let mut fTemp560: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp557, 1)) as usize] } - fTemp559;
			let mut fTemp561: F64 = if iTemp65 != 0 {fTemp559 + fTemp79 * fTemp560 + (fTemp555 - (iTemp556) as F64) * (fTemp558 - (fTemp559 + fTemp79 * (fTemp560 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp557, 4)) as usize] } - fTemp558))))} else {1.0 - (fTemp553 + fTemp79 * fTemp554 + (fTemp549 - (iTemp550) as F64) * (fTemp552 - (fTemp553 + fTemp79 * (fTemp554 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp551, 4)) as usize] } - fTemp552)))))};
			let mut fTemp562: F64 = fTemp84 + fTemp548;
			let mut fTemp563: F64 = 131071.0 * (1.0 - fTemp562);
			let mut iTemp564: i32 = (fTemp563) as i32;
			let mut iTemp565: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp564, 131071)))), 393215));
			let mut fTemp566: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp565, 3)) as usize] };
			let mut fTemp567: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp565 as usize] };
			let mut fTemp568: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp565, 1)) as usize] } - fTemp567;
			let mut fTemp569: F64 = 131071.0 * fTemp562;
			let mut iTemp570: i32 = (fTemp569) as i32;
			let mut iTemp571: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp570, 131071)))), 393215));
			let mut fTemp572: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp571, 3), 393215))) as usize] };
			let mut fTemp573: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp571 as usize] };
			let mut fTemp574: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp571, 1), 393215))) as usize] } - fTemp573;
			let mut iTemp575: i32 = (fTemp140 > ((if iTemp65 != 0 {fTemp573 + fTemp79 * fTemp574 + (fTemp569 - (iTemp570) as F64) * (fTemp572 - (fTemp573 + fTemp79 * (fTemp574 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp571, 4), 393215))) as usize] } - fTemp572))))} else {1.0 - (fTemp567 + fTemp79 * fTemp568 + (fTemp563 - (iTemp564) as F64) * (fTemp566 - (fTemp567 + fTemp79 * (fTemp568 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp565, 4)) as usize] } - fTemp566)))))} - fTemp561) / (1.0 - fTemp561))) as i32;
			let mut fTemp576: F64 = if iTemp575 != 0 {fTemp545} else {fTemp548};
			let mut fTemp577: F64 = if iTemp575 != 0 {fTemp548} else {fTemp546};
			let mut fTemp578: F64 = fTemp577 + fTemp576;
			let mut fTemp579: F64 = 0.5 * fTemp578;
			let mut fTemp580: F64 = 131071.0 * (1.0 - fTemp579);
			let mut iTemp581: i32 = (fTemp580) as i32;
			let mut iTemp582: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp581, 131071)))), 393215));
			let mut fTemp583: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp582, 3)) as usize] };
			let mut fTemp584: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp582 as usize] };
			let mut fTemp585: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp582, 1)) as usize] } - fTemp584;
			let mut fTemp586: F64 = 65535.5 * fTemp578;
			let mut iTemp587: i32 = (fTemp586) as i32;
			let mut iTemp588: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp587, 131071)))), 393215));
			let mut fTemp589: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp588, 3)) as usize] };
			let mut fTemp590: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp588 as usize] };
			let mut fTemp591: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp588, 1)) as usize] } - fTemp590;
			let mut fTemp592: F64 = if iTemp65 != 0 {fTemp590 + fTemp79 * fTemp591 + (fTemp586 - (iTemp587) as F64) * (fTemp589 - (fTemp590 + fTemp79 * (fTemp591 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp588, 4)) as usize] } - fTemp589))))} else {1.0 - (fTemp584 + fTemp79 * fTemp585 + (fTemp580 - (iTemp581) as F64) * (fTemp583 - (fTemp584 + fTemp79 * (fTemp585 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp582, 4)) as usize] } - fTemp583)))))};
			let mut fTemp593: F64 = fTemp84 + fTemp579;
			let mut fTemp594: F64 = 131071.0 * (1.0 - fTemp593);
			let mut iTemp595: i32 = (fTemp594) as i32;
			let mut iTemp596: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp595, 131071)))), 393215));
			let mut fTemp597: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp596, 3)) as usize] };
			let mut fTemp598: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp596 as usize] };
			let mut fTemp599: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp596, 1)) as usize] } - fTemp598;
			let mut fTemp600: F64 = 131071.0 * fTemp593;
			let mut iTemp601: i32 = (fTemp600) as i32;
			let mut iTemp602: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp601, 131071)))), 393215));
			let mut fTemp603: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp602, 3), 393215))) as usize] };
			let mut fTemp604: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp602 as usize] };
			let mut fTemp605: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp602, 1), 393215))) as usize] } - fTemp604;
			let mut iTemp606: i32 = (fTemp140 > ((if iTemp65 != 0 {fTemp604 + fTemp79 * fTemp605 + (fTemp600 - (iTemp601) as F64) * (fTemp603 - (fTemp604 + fTemp79 * (fTemp605 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp602, 4), 393215))) as usize] } - fTemp603))))} else {1.0 - (fTemp598 + fTemp79 * fTemp599 + (fTemp594 - (iTemp595) as F64) * (fTemp597 - (fTemp598 + fTemp79 * (fTemp599 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp596, 4)) as usize] } - fTemp597)))))} - fTemp592) / (1.0 - fTemp592))) as i32;
			let mut fTemp607: F64 = if iTemp606 != 0 {fTemp576} else {fTemp579};
			let mut fTemp608: F64 = if iTemp606 != 0 {fTemp579} else {fTemp577};
			let mut fTemp609: F64 = fTemp608 + fTemp607;
			let mut fTemp610: F64 = 0.5 * fTemp609;
			let mut fTemp611: F64 = 131071.0 * (1.0 - fTemp610);
			let mut iTemp612: i32 = (fTemp611) as i32;
			let mut iTemp613: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp612, 131071)))), 393215));
			let mut fTemp614: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp613, 3)) as usize] };
			let mut fTemp615: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp613 as usize] };
			let mut fTemp616: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp613, 1)) as usize] } - fTemp615;
			let mut fTemp617: F64 = 65535.5 * fTemp609;
			let mut iTemp618: i32 = (fTemp617) as i32;
			let mut iTemp619: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp618, 131071)))), 393215));
			let mut fTemp620: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp619, 3)) as usize] };
			let mut fTemp621: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp619 as usize] };
			let mut fTemp622: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp619, 1)) as usize] } - fTemp621;
			let mut fTemp623: F64 = if iTemp65 != 0 {fTemp621 + fTemp79 * fTemp622 + (fTemp617 - (iTemp618) as F64) * (fTemp620 - (fTemp621 + fTemp79 * (fTemp622 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp619, 4), 393215))) as usize] } - fTemp620))))} else {1.0 - (fTemp615 + fTemp79 * fTemp616 + (fTemp611 - (iTemp612) as F64) * (fTemp614 - (fTemp615 + fTemp79 * (fTemp616 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp613, 4), 393215))) as usize] } - fTemp614)))))};
			let mut fTemp624: F64 = fTemp84 + fTemp610;
			let mut fTemp625: F64 = 131071.0 * (1.0 - fTemp624);
			let mut iTemp626: i32 = (fTemp625) as i32;
			let mut iTemp627: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp626, 131071)))), 393215));
			let mut fTemp628: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp627, 3)) as usize] };
			let mut fTemp629: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp627 as usize] };
			let mut fTemp630: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp627, 1)) as usize] } - fTemp629;
			let mut fTemp631: F64 = 131071.0 * fTemp624;
			let mut iTemp632: i32 = (fTemp631) as i32;
			let mut iTemp633: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp632, 131071)))), 393215));
			let mut fTemp634: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp633, 3), 393215))) as usize] };
			let mut fTemp635: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp633 as usize] };
			let mut fTemp636: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp633, 1), 393215))) as usize] } - fTemp635;
			let mut iTemp637: i32 = (fTemp140 > ((if iTemp65 != 0 {fTemp635 + fTemp79 * fTemp636 + (fTemp631 - (iTemp632) as F64) * (fTemp634 - (fTemp635 + fTemp79 * (fTemp636 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp633, 4), 393215))) as usize] } - fTemp634))))} else {1.0 - (fTemp629 + fTemp79 * fTemp630 + (fTemp625 - (iTemp626) as F64) * (fTemp628 - (fTemp629 + fTemp79 * (fTemp630 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp627, 4)) as usize] } - fTemp628)))))} - fTemp623) / (1.0 - fTemp623))) as i32;
			let mut fTemp638: F64 = F64::min(1.0, F64::max(0.0, 0.5 * (if iTemp637 != 0 {fTemp610} else {fTemp608} + if iTemp637 != 0 {fTemp607} else {fTemp610})));
			self.fRec1[0] = fTemp638;
			let mut fTemp639: F64 = 131071.0 * (1.0 - fTemp638);
			let mut iTemp640: i32 = (fTemp639) as i32;
			let mut iTemp641: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp640, 131071)))), 393215));
			let mut fTemp642: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp641, 3)) as usize] };
			let mut fTemp643: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp641 as usize] };
			let mut fTemp644: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp641, 1)) as usize] } - fTemp643;
			let mut fTemp645: F64 = 131071.0 * fTemp638;
			let mut iTemp646: i32 = (fTemp645) as i32;
			let mut iTemp647: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp646, 131071)))), 393215));
			let mut fTemp648: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp647, 3)) as usize] };
			let mut fTemp649: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp647 as usize] };
			let mut fTemp650: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp647, 1)) as usize] } - fTemp649;
			let mut fTemp651: F64 = if iTemp65 != 0 {fTemp649 + fTemp79 * fTemp650 + (fTemp645 - (iTemp646) as F64) * (fTemp648 - (fTemp649 + fTemp79 * (fTemp650 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp647, 4), 393215))) as usize] } - fTemp648))))} else {1.0 - (fTemp643 + fTemp79 * fTemp644 + (fTemp639 - (iTemp640) as F64) * (fTemp642 - (fTemp643 + fTemp79 * (fTemp644 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp641, 4), 393215))) as usize] } - fTemp642)))))};
			let mut fTemp652: F64 = fTemp84 + fTemp638;
			let mut fTemp653: F64 = 131071.0 * (1.0 - fTemp652);
			let mut iTemp654: i32 = (fTemp653) as i32;
			let mut iTemp655: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp654, 131071)))), 393215));
			let mut fTemp656: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp655, 3)) as usize] };
			let mut fTemp657: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp655 as usize] };
			let mut fTemp658: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp655, 1)) as usize] } - fTemp657;
			let mut fTemp659: F64 = 131071.0 * fTemp652;
			let mut iTemp660: i32 = (fTemp659) as i32;
			let mut iTemp661: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp660, 131071)))), 393215));
			let mut fTemp662: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp661, 3), 393215))) as usize] };
			let mut fTemp663: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp661 as usize] };
			let mut fTemp664: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp661, 1), 393215))) as usize] } - fTemp663;
			let mut fTemp665: F64 = fTemp5 + if ((0.001 * fTemp83) == 0.0) as i32 != 0 {fTemp64} else {fTemp64 * (if iTemp65 != 0 {fTemp663 + fTemp79 * fTemp664 + (fTemp659 - (iTemp660) as F64) * (fTemp662 - (fTemp663 + fTemp79 * (fTemp664 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp661, 4), 393215))) as usize] } - fTemp662))))} else {1.0 - (fTemp657 + fTemp79 * fTemp658 + (fTemp653 - (iTemp654) as F64) * (fTemp656 - (fTemp657 + fTemp79 * (fTemp658 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp655, 4)) as usize] } - fTemp656)))))} - fTemp651) / (1.0 - fTemp651)};
			self.fRec2[(self.IOTA0 & 16383) as usize] = if iTemp82 != 0 {F64::min(fTemp665, fTemp5)} else {F64::max(fTemp665, fTemp5)};
			let mut fTemp666: F64 = self.fRec2[((i32::wrapping_sub(self.IOTA0, iSlow75)) & 16383) as usize];
			self.fRec14[0] = fSlow76 + self.fConst4 * self.fRec14[1];
			*output0 = 0.5 * self.fVec1[((i32::wrapping_sub(self.IOTA0, iSlow18)) & 32767) as usize] * fTemp2 + self.fRec14[0] * self.fVec2[((i32::wrapping_sub(self.IOTA0, iSlow18)) & 32767) as usize] * fTemp666 * fTemp4;
			let mut fTemp667: F64 = self.fRec16[((i32::wrapping_sub(self.IOTA0, 1)) & 16383) as usize];
			let mut fTemp668: F64 = fTemp36 + fSlow21 * (fTemp37 - fTemp36);
			let mut iTemp669: i32 = ((fTemp668 > fSlow11) as i32) + ((fTemp668 > fSlow9) as i32);
			let mut fTemp670: F64 = fTemp668 - fSlow8;
			let mut fTemp671: F64 = F64::min(fTemp34, F64::powf(1e+01, -(fSlow22 * F64::max(0.0, if (iTemp669 == 0) as i32 != 0 {0.0} else {if (iTemp669 == 1) as i32 != 0 {fSlow12 * LambRs96k_faustpower2_f(fSlow7 + fTemp670)} else {fTemp670}}))));
			self.fVec33[(self.IOTA0 & 16383) as usize] = fTemp671;
			let mut fTemp672: F64 = F64::min(fTemp671, self.fVec33[((i32::wrapping_sub(self.IOTA0, 1)) & 16383) as usize]);
			self.fVec34[0] = fTemp672;
			let mut fTemp673: F64 = F64::min(fTemp672, self.fVec34[2]);
			self.fVec35[0] = fTemp673;
			let mut fTemp674: F64 = F64::min(fTemp673, self.fVec35[4]);
			self.fVec36[0] = fTemp674;
			let mut fTemp675: F64 = F64::min(fTemp674, self.fVec36[8]);
			self.fVec37[(self.IOTA0 & 31) as usize] = fTemp675;
			let mut fTemp676: F64 = F64::min(fTemp675, self.fVec37[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize]);
			self.fVec38[(self.IOTA0 & 63) as usize] = fTemp676;
			let mut fTemp677: F64 = F64::min(fTemp676, self.fVec38[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize]);
			self.fVec39[(self.IOTA0 & 127) as usize] = fTemp677;
			let mut fTemp678: F64 = F64::min(fTemp677, self.fVec39[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize]);
			self.fVec40[(self.IOTA0 & 255) as usize] = fTemp678;
			let mut fTemp679: F64 = F64::min(fTemp678, self.fVec40[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize]);
			self.fVec41[(self.IOTA0 & 511) as usize] = fTemp679;
			let mut fTemp680: F64 = F64::min(fTemp679, self.fVec41[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize]);
			self.fVec42[(self.IOTA0 & 1023) as usize] = fTemp680;
			let mut fTemp681: F64 = F64::min(fTemp680, self.fVec42[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize]);
			self.fVec43[(self.IOTA0 & 2047) as usize] = fTemp681;
			let mut fTemp682: F64 = F64::min(fTemp681, self.fVec43[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			self.fVec44[(self.IOTA0 & 4095) as usize] = fTemp682;
			self.fVec45[(self.IOTA0 & 8191) as usize] = F64::min(fTemp682, self.fVec44[((i32::wrapping_sub(self.IOTA0, 2048)) & 4095) as usize]);
			self.fRec17[0] = F64::max(F64::min(self.fRec17[1], self.fVec33[((i32::wrapping_sub(self.IOTA0, iSlow24)) & 16383) as usize]), F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow26 != 0 {fTemp671} else {1.7976931348623157e+308}, if iSlow27 != 0 {self.fVec34[iSlow26 as usize]} else {1.7976931348623157e+308}), if iSlow28 != 0 {self.fVec35[iSlow29 as usize]} else {1.7976931348623157e+308}), if iSlow30 != 0 {self.fVec36[iSlow31 as usize]} else {1.7976931348623157e+308}), if iSlow32 != 0 {self.fVec37[((i32::wrapping_sub(self.IOTA0, iSlow33)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow34 != 0 {self.fVec38[((i32::wrapping_sub(self.IOTA0, iSlow35)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow36 != 0 {self.fVec39[((i32::wrapping_sub(self.IOTA0, iSlow37)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow38 != 0 {self.fVec40[((i32::wrapping_sub(self.IOTA0, iSlow39)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow40 != 0 {self.fVec41[((i32::wrapping_sub(self.IOTA0, iSlow41)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow42 != 0 {self.fVec42[((i32::wrapping_sub(self.IOTA0, iSlow43)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow44 != 0 {self.fVec43[((i32::wrapping_sub(self.IOTA0, iSlow45)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow46 != 0 {self.fVec44[((i32::wrapping_sub(self.IOTA0, iSlow47)) & 4095) as usize]} else {1.7976931348623157e+308}), if iSlow48 != 0 {self.fVec45[((i32::wrapping_sub(self.IOTA0, iSlow49)) & 8191) as usize]} else {1.7976931348623157e+308}));
			let mut fTemp683: F64 = F64::min(self.fRec17[0], self.fRec17[1]);
			self.fVec46[0] = fTemp683;
			let mut fTemp684: F64 = F64::min(fTemp683, self.fVec46[2]);
			self.fVec47[0] = fTemp684;
			let mut fTemp685: F64 = F64::min(fTemp684, self.fVec47[4]);
			self.fVec48[0] = fTemp685;
			let mut fTemp686: F64 = F64::min(fTemp685, self.fVec48[8]);
			self.fVec49[(self.IOTA0 & 31) as usize] = fTemp686;
			let mut fTemp687: F64 = F64::min(fTemp686, self.fVec49[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize]);
			self.fVec50[(self.IOTA0 & 63) as usize] = fTemp687;
			let mut fTemp688: F64 = F64::min(fTemp687, self.fVec50[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize]);
			self.fVec51[(self.IOTA0 & 127) as usize] = fTemp688;
			let mut fTemp689: F64 = F64::min(fTemp688, self.fVec51[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize]);
			self.fVec52[(self.IOTA0 & 255) as usize] = fTemp689;
			let mut fTemp690: F64 = F64::min(fTemp689, self.fVec52[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize]);
			self.fVec53[(self.IOTA0 & 511) as usize] = fTemp690;
			let mut fTemp691: F64 = F64::min(fTemp690, self.fVec53[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize]);
			self.fVec54[(self.IOTA0 & 1023) as usize] = fTemp691;
			let mut fTemp692: F64 = F64::min(fTemp691, self.fVec54[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize]);
			self.fVec55[(self.IOTA0 & 2047) as usize] = fTemp692;
			let mut fTemp693: F64 = F64::min(fTemp692, self.fVec55[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			self.fVec56[(self.IOTA0 & 4095) as usize] = fTemp693;
			self.fVec57[(self.IOTA0 & 8191) as usize] = F64::min(fTemp693, self.fVec56[((i32::wrapping_sub(self.IOTA0, 2048)) & 4095) as usize]);
			let mut fTemp694: F64 = F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow4 != 0 {self.fRec17[0]} else {1.7976931348623157e+308}, if iSlow50 != 0 {self.fVec46[iSlow4 as usize]} else {1.7976931348623157e+308}), if iSlow51 != 0 {self.fVec47[iSlow52 as usize]} else {1.7976931348623157e+308}), if iSlow53 != 0 {self.fVec48[iSlow54 as usize]} else {1.7976931348623157e+308}), if iSlow55 != 0 {self.fVec49[((i32::wrapping_sub(self.IOTA0, iSlow56)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow57 != 0 {self.fVec50[((i32::wrapping_sub(self.IOTA0, iSlow58)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow59 != 0 {self.fVec51[((i32::wrapping_sub(self.IOTA0, iSlow60)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow61 != 0 {self.fVec52[((i32::wrapping_sub(self.IOTA0, iSlow62)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow63 != 0 {self.fVec53[((i32::wrapping_sub(self.IOTA0, iSlow64)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow65 != 0 {self.fVec54[((i32::wrapping_sub(self.IOTA0, iSlow66)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow67 != 0 {self.fVec55[((i32::wrapping_sub(self.IOTA0, iSlow68)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow69 != 0 {self.fVec56[((i32::wrapping_sub(self.IOTA0, iSlow70)) & 4095) as usize]} else {1.7976931348623157e+308}), if iSlow71 != 0 {self.fVec57[((i32::wrapping_sub(self.IOTA0, iSlow72)) & 8191) as usize]} else {1.7976931348623157e+308}) - fTemp667;
			self.fVec58[0] = fTemp694;
			let mut iTemp695: i32 = (fTemp694 > 0.0) as i32;
			let mut fTemp696: F64 = if iTemp695 != 0 {fSlow74} else {fSlow73};
			self.fVec59[0] = fTemp696;
			let mut fTemp697: F64 = 2.0 * fTemp696;
			let mut iTemp698: i32 = (fTemp697) as i32;
			let mut iTemp699: i32 = std::cmp::max(0, std::cmp::min(iTemp698, 2));
			let mut iTemp700: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, 196605), 393215));
			let mut fTemp701: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp700, 3)) as usize] };
			let mut fTemp702: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp700 as usize] };
			let mut fTemp703: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp700, 1)) as usize] } - fTemp702;
			let mut fTemp704: F64 = fTemp697 - (iTemp698) as F64;
			let mut fTemp705: F64 = fTemp702 + fTemp704 * fTemp703 + 0.5 * (fTemp701 - (fTemp702 + fTemp704 * (fTemp703 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp700, 4)) as usize] } - fTemp701))));
			let mut fTemp706: F64 = if iTemp695 != 0 {fTemp705} else {1.0 - fTemp705};
			let mut iTemp707: i32 = (fTemp694 < 0.0) as i32;
			let mut fTemp708: F64 = fSlow1 * (iTemp707) as F64 + fSlow13 * (iTemp695) as F64;
			self.fVec60[0] = fTemp708;
			let mut fTemp709: F64 = self.fConst10 / fTemp708;
			let mut fTemp710: F64 = fTemp709 + 0.5;
			let mut fTemp711: F64 = 131071.0 * (1.0 - fTemp710);
			let mut iTemp712: i32 = (fTemp711) as i32;
			let mut iTemp713: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp712, 131071)))), 393215));
			let mut fTemp714: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp713, 3)) as usize] };
			let mut fTemp715: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp713 as usize] };
			let mut fTemp716: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp713, 1)) as usize] } - fTemp715;
			let mut fTemp717: F64 = 131071.0 * fTemp710;
			let mut iTemp718: i32 = (fTemp717) as i32;
			let mut iTemp719: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp718, 131071)))), 393215));
			let mut fTemp720: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp719, 3), 393215))) as usize] };
			let mut fTemp721: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp719 as usize] };
			let mut fTemp722: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp719, 1), 393215))) as usize] } - fTemp721;
			let mut fTemp723: F64 = 2.0 * self.fVec59[1];
			let mut iTemp724: i32 = (fTemp723) as i32;
			let mut iTemp725: i32 = std::cmp::max(0, std::cmp::min(iTemp724, 2));
			let mut fTemp726: F64 = 131071.0 * (1.0 - self.fRec15[1]);
			let mut iTemp727: i32 = (fTemp726) as i32;
			let mut iTemp728: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp727, 131071))), iTemp725), 393215));
			let mut fTemp729: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp728, 3), 393215))) as usize] };
			let mut fTemp730: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp728 as usize] };
			let mut fTemp731: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp728, 1), 393215))) as usize] } - fTemp730;
			let mut fTemp732: F64 = fTemp723 - (iTemp724) as F64;
			let mut fTemp733: F64 = 131071.0 * self.fRec15[1];
			let mut iTemp734: i32 = (fTemp733) as i32;
			let mut iTemp735: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp725, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp734, 131071)))), 393215));
			let mut fTemp736: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp735, 3), 393215))) as usize] };
			let mut fTemp737: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp735 as usize] };
			let mut fTemp738: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp735, 1), 393215))) as usize] } - fTemp737;
			let mut fTemp739: F64 = self.fRec15[1] + fTemp709;
			let mut fTemp740: F64 = 131071.0 * (1.0 - fTemp739);
			let mut iTemp741: i32 = (fTemp740) as i32;
			let mut iTemp742: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp741, 131071)))), 393215));
			let mut fTemp743: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp742, 3)) as usize] };
			let mut fTemp744: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp742 as usize] };
			let mut fTemp745: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp742, 1)) as usize] } - fTemp744;
			let mut fTemp746: F64 = 131071.0 * fTemp739;
			let mut iTemp747: i32 = (fTemp746) as i32;
			let mut iTemp748: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp747, 131071)))), 393215));
			let mut fTemp749: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp748, 3), 393215))) as usize] };
			let mut fTemp750: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp748 as usize] };
			let mut fTemp751: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp748, 1), 393215))) as usize] } - fTemp750;
			let mut fTemp752: F64 = self.fRec15[1] + self.fConst10 * (1.0 / fTemp708 + 1.0 / self.fVec60[1]);
			let mut fTemp753: F64 = 131071.0 * (1.0 - fTemp752);
			let mut iTemp754: i32 = (fTemp753) as i32;
			let mut iTemp755: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp754, 131071))), iTemp699), 393215));
			let mut fTemp756: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp755, 3)) as usize] };
			let mut fTemp757: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp755 as usize] };
			let mut fTemp758: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp755, 1)) as usize] } - fTemp757;
			let mut fTemp759: F64 = 131071.0 * fTemp752;
			let mut iTemp760: i32 = (fTemp759) as i32;
			let mut iTemp761: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp760, 131071)))), 393215));
			let mut fTemp762: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp761, 3), 393215))) as usize] };
			let mut fTemp763: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp761 as usize] };
			let mut fTemp764: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp761, 1), 393215))) as usize] } - fTemp763;
			let mut fTemp765: F64 = (if iTemp695 != 0 {fTemp763 + fTemp704 * fTemp764 + (fTemp759 - (iTemp760) as F64) * (fTemp762 - (fTemp763 + fTemp704 * (fTemp764 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp761, 4), 393215))) as usize] } - fTemp762))))} else {1.0 - (fTemp757 + fTemp704 * fTemp758 + (fTemp753 - (iTemp754) as F64) * (fTemp756 - (fTemp757 + fTemp704 * (fTemp758 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp755, 4)) as usize] } - fTemp756)))))} - if iTemp695 != 0 {fTemp750 + fTemp704 * fTemp751 + (fTemp746 - (iTemp747) as F64) * (fTemp749 - (fTemp750 + fTemp704 * (fTemp751 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp748, 4), 393215))) as usize] } - fTemp749))))} else {1.0 - (fTemp744 + fTemp704 * fTemp745 + (fTemp740 - (iTemp741) as F64) * (fTemp743 - (fTemp744 + fTemp704 * (fTemp745 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp742, 4)) as usize] } - fTemp743)))))}) * self.fVec58[1] / (fTemp694 * (1.0 - if iTemp695 != 0 {fTemp737 + fTemp732 * fTemp738 + (fTemp733 - (iTemp734) as F64) * (fTemp736 - (fTemp737 + fTemp732 * (fTemp738 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp735, 4), 393215))) as usize] } - fTemp736))))} else {1.0 - (fTemp730 + fTemp732 * fTemp731 + (fTemp726 - (iTemp727) as F64) * (fTemp729 - (fTemp730 + fTemp732 * (fTemp731 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp728, 4), 393215))) as usize] } - fTemp729)))))}));
			let mut iTemp766: i32 = (fTemp765 > ((if iTemp695 != 0 {fTemp721 + fTemp704 * fTemp722 + (fTemp717 - (iTemp718) as F64) * (fTemp720 - (fTemp721 + fTemp704 * (fTemp722 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp719, 4), 393215))) as usize] } - fTemp720))))} else {1.0 - (fTemp715 + fTemp704 * fTemp716 + (fTemp711 - (iTemp712) as F64) * (fTemp714 - (fTemp715 + fTemp704 * (fTemp716 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp713, 4)) as usize] } - fTemp714)))))} - fTemp706) / (1.0 - fTemp706))) as i32;
			let mut fTemp767: F64 = if iTemp766 != 0 {1.0} else {0.5};
			let mut fTemp768: F64 = if iTemp766 != 0 {0.5} else {0.0};
			let mut fTemp769: F64 = fTemp768 + fTemp767;
			let mut fTemp770: F64 = 0.5 * fTemp769;
			let mut fTemp771: F64 = 131071.0 * (1.0 - fTemp770);
			let mut iTemp772: i32 = (fTemp771) as i32;
			let mut iTemp773: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp772, 131071)))), 393215));
			let mut fTemp774: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp773, 3)) as usize] };
			let mut fTemp775: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp773 as usize] };
			let mut fTemp776: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp773, 1)) as usize] } - fTemp775;
			let mut fTemp777: F64 = 65535.5 * fTemp769;
			let mut iTemp778: i32 = (fTemp777) as i32;
			let mut iTemp779: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp778, 131071)))), 393215));
			let mut fTemp780: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp779, 3)) as usize] };
			let mut fTemp781: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp779 as usize] };
			let mut fTemp782: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp779, 1)) as usize] } - fTemp781;
			let mut fTemp783: F64 = if iTemp695 != 0 {fTemp781 + fTemp704 * fTemp782 + (fTemp777 - (iTemp778) as F64) * (fTemp780 - (fTemp781 + fTemp704 * (fTemp782 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp779, 4)) as usize] } - fTemp780))))} else {1.0 - (fTemp775 + fTemp704 * fTemp776 + (fTemp771 - (iTemp772) as F64) * (fTemp774 - (fTemp775 + fTemp704 * (fTemp776 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp773, 4)) as usize] } - fTemp774)))))};
			let mut fTemp784: F64 = fTemp709 + fTemp770;
			let mut fTemp785: F64 = 131071.0 * (1.0 - fTemp784);
			let mut iTemp786: i32 = (fTemp785) as i32;
			let mut iTemp787: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp786, 131071)))), 393215));
			let mut fTemp788: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp787, 3)) as usize] };
			let mut fTemp789: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp787 as usize] };
			let mut fTemp790: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp787, 1)) as usize] } - fTemp789;
			let mut fTemp791: F64 = 131071.0 * fTemp784;
			let mut iTemp792: i32 = (fTemp791) as i32;
			let mut iTemp793: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp792, 131071)))), 393215));
			let mut fTemp794: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp793, 3), 393215))) as usize] };
			let mut fTemp795: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp793 as usize] };
			let mut fTemp796: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp793, 1), 393215))) as usize] } - fTemp795;
			let mut iTemp797: i32 = (fTemp765 > ((if iTemp695 != 0 {fTemp795 + fTemp704 * fTemp796 + (fTemp791 - (iTemp792) as F64) * (fTemp794 - (fTemp795 + fTemp704 * (fTemp796 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp793, 4), 393215))) as usize] } - fTemp794))))} else {1.0 - (fTemp789 + fTemp704 * fTemp790 + (fTemp785 - (iTemp786) as F64) * (fTemp788 - (fTemp789 + fTemp704 * (fTemp790 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp787, 4)) as usize] } - fTemp788)))))} - fTemp783) / (1.0 - fTemp783))) as i32;
			let mut fTemp798: F64 = if iTemp797 != 0 {fTemp767} else {fTemp770};
			let mut fTemp799: F64 = if iTemp797 != 0 {fTemp770} else {fTemp768};
			let mut fTemp800: F64 = fTemp799 + fTemp798;
			let mut fTemp801: F64 = 0.5 * fTemp800;
			let mut fTemp802: F64 = 131071.0 * (1.0 - fTemp801);
			let mut iTemp803: i32 = (fTemp802) as i32;
			let mut iTemp804: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp803, 131071)))), 393215));
			let mut fTemp805: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp804, 3)) as usize] };
			let mut fTemp806: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp804 as usize] };
			let mut fTemp807: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp804, 1)) as usize] } - fTemp806;
			let mut fTemp808: F64 = 65535.5 * fTemp800;
			let mut iTemp809: i32 = (fTemp808) as i32;
			let mut iTemp810: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp809, 131071)))), 393215));
			let mut fTemp811: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp810, 3)) as usize] };
			let mut fTemp812: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp810 as usize] };
			let mut fTemp813: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp810, 1)) as usize] } - fTemp812;
			let mut fTemp814: F64 = if iTemp695 != 0 {fTemp812 + fTemp704 * fTemp813 + (fTemp808 - (iTemp809) as F64) * (fTemp811 - (fTemp812 + fTemp704 * (fTemp813 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp810, 4)) as usize] } - fTemp811))))} else {1.0 - (fTemp806 + fTemp704 * fTemp807 + (fTemp802 - (iTemp803) as F64) * (fTemp805 - (fTemp806 + fTemp704 * (fTemp807 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp804, 4)) as usize] } - fTemp805)))))};
			let mut fTemp815: F64 = fTemp709 + fTemp801;
			let mut fTemp816: F64 = 131071.0 * (1.0 - fTemp815);
			let mut iTemp817: i32 = (fTemp816) as i32;
			let mut iTemp818: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp817, 131071)))), 393215));
			let mut fTemp819: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp818, 3)) as usize] };
			let mut fTemp820: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp818 as usize] };
			let mut fTemp821: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp818, 1)) as usize] } - fTemp820;
			let mut fTemp822: F64 = 131071.0 * fTemp815;
			let mut iTemp823: i32 = (fTemp822) as i32;
			let mut iTemp824: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp823, 131071)))), 393215));
			let mut fTemp825: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp824, 3), 393215))) as usize] };
			let mut fTemp826: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp824 as usize] };
			let mut fTemp827: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp824, 1), 393215))) as usize] } - fTemp826;
			let mut iTemp828: i32 = (fTemp765 > ((if iTemp695 != 0 {fTemp826 + fTemp704 * fTemp827 + (fTemp822 - (iTemp823) as F64) * (fTemp825 - (fTemp826 + fTemp704 * (fTemp827 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp824, 4), 393215))) as usize] } - fTemp825))))} else {1.0 - (fTemp820 + fTemp704 * fTemp821 + (fTemp816 - (iTemp817) as F64) * (fTemp819 - (fTemp820 + fTemp704 * (fTemp821 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp818, 4)) as usize] } - fTemp819)))))} - fTemp814) / (1.0 - fTemp814))) as i32;
			let mut fTemp829: F64 = if iTemp828 != 0 {fTemp798} else {fTemp801};
			let mut fTemp830: F64 = if iTemp828 != 0 {fTemp801} else {fTemp799};
			let mut fTemp831: F64 = fTemp830 + fTemp829;
			let mut fTemp832: F64 = 0.5 * fTemp831;
			let mut fTemp833: F64 = 131071.0 * (1.0 - fTemp832);
			let mut iTemp834: i32 = (fTemp833) as i32;
			let mut iTemp835: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp834, 131071)))), 393215));
			let mut fTemp836: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp835, 3)) as usize] };
			let mut fTemp837: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp835 as usize] };
			let mut fTemp838: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp835, 1)) as usize] } - fTemp837;
			let mut fTemp839: F64 = 65535.5 * fTemp831;
			let mut iTemp840: i32 = (fTemp839) as i32;
			let mut iTemp841: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp840, 131071)))), 393215));
			let mut fTemp842: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp841, 3)) as usize] };
			let mut fTemp843: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp841 as usize] };
			let mut fTemp844: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp841, 1)) as usize] } - fTemp843;
			let mut fTemp845: F64 = if iTemp695 != 0 {fTemp843 + fTemp704 * fTemp844 + (fTemp839 - (iTemp840) as F64) * (fTemp842 - (fTemp843 + fTemp704 * (fTemp844 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp841, 4)) as usize] } - fTemp842))))} else {1.0 - (fTemp837 + fTemp704 * fTemp838 + (fTemp833 - (iTemp834) as F64) * (fTemp836 - (fTemp837 + fTemp704 * (fTemp838 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp835, 4)) as usize] } - fTemp836)))))};
			let mut fTemp846: F64 = fTemp709 + fTemp832;
			let mut fTemp847: F64 = 131071.0 * (1.0 - fTemp846);
			let mut iTemp848: i32 = (fTemp847) as i32;
			let mut iTemp849: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp848, 131071)))), 393215));
			let mut fTemp850: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp849, 3)) as usize] };
			let mut fTemp851: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp849 as usize] };
			let mut fTemp852: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp849, 1)) as usize] } - fTemp851;
			let mut fTemp853: F64 = 131071.0 * fTemp846;
			let mut iTemp854: i32 = (fTemp853) as i32;
			let mut iTemp855: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp854, 131071)))), 393215));
			let mut fTemp856: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp855, 3), 393215))) as usize] };
			let mut fTemp857: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp855 as usize] };
			let mut fTemp858: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp855, 1), 393215))) as usize] } - fTemp857;
			let mut iTemp859: i32 = (fTemp765 > ((if iTemp695 != 0 {fTemp857 + fTemp704 * fTemp858 + (fTemp853 - (iTemp854) as F64) * (fTemp856 - (fTemp857 + fTemp704 * (fTemp858 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp855, 4), 393215))) as usize] } - fTemp856))))} else {1.0 - (fTemp851 + fTemp704 * fTemp852 + (fTemp847 - (iTemp848) as F64) * (fTemp850 - (fTemp851 + fTemp704 * (fTemp852 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp849, 4)) as usize] } - fTemp850)))))} - fTemp845) / (1.0 - fTemp845))) as i32;
			let mut fTemp860: F64 = if iTemp859 != 0 {fTemp829} else {fTemp832};
			let mut fTemp861: F64 = if iTemp859 != 0 {fTemp832} else {fTemp830};
			let mut fTemp862: F64 = fTemp861 + fTemp860;
			let mut fTemp863: F64 = 0.5 * fTemp862;
			let mut fTemp864: F64 = 131071.0 * (1.0 - fTemp863);
			let mut iTemp865: i32 = (fTemp864) as i32;
			let mut iTemp866: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp865, 131071)))), 393215));
			let mut fTemp867: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp866, 3)) as usize] };
			let mut fTemp868: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp866 as usize] };
			let mut fTemp869: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp866, 1)) as usize] } - fTemp868;
			let mut fTemp870: F64 = 65535.5 * fTemp862;
			let mut iTemp871: i32 = (fTemp870) as i32;
			let mut iTemp872: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp871, 131071)))), 393215));
			let mut fTemp873: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp872, 3)) as usize] };
			let mut fTemp874: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp872 as usize] };
			let mut fTemp875: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp872, 1)) as usize] } - fTemp874;
			let mut fTemp876: F64 = if iTemp695 != 0 {fTemp874 + fTemp704 * fTemp875 + (fTemp870 - (iTemp871) as F64) * (fTemp873 - (fTemp874 + fTemp704 * (fTemp875 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp872, 4)) as usize] } - fTemp873))))} else {1.0 - (fTemp868 + fTemp704 * fTemp869 + (fTemp864 - (iTemp865) as F64) * (fTemp867 - (fTemp868 + fTemp704 * (fTemp869 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp866, 4)) as usize] } - fTemp867)))))};
			let mut fTemp877: F64 = fTemp709 + fTemp863;
			let mut fTemp878: F64 = 131071.0 * (1.0 - fTemp877);
			let mut iTemp879: i32 = (fTemp878) as i32;
			let mut iTemp880: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp879, 131071)))), 393215));
			let mut fTemp881: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp880, 3)) as usize] };
			let mut fTemp882: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp880 as usize] };
			let mut fTemp883: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp880, 1)) as usize] } - fTemp882;
			let mut fTemp884: F64 = 131071.0 * fTemp877;
			let mut iTemp885: i32 = (fTemp884) as i32;
			let mut iTemp886: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp885, 131071)))), 393215));
			let mut fTemp887: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp886, 3), 393215))) as usize] };
			let mut fTemp888: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp886 as usize] };
			let mut fTemp889: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp886, 1), 393215))) as usize] } - fTemp888;
			let mut iTemp890: i32 = (fTemp765 > ((if iTemp695 != 0 {fTemp888 + fTemp704 * fTemp889 + (fTemp884 - (iTemp885) as F64) * (fTemp887 - (fTemp888 + fTemp704 * (fTemp889 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp886, 4), 393215))) as usize] } - fTemp887))))} else {1.0 - (fTemp882 + fTemp704 * fTemp883 + (fTemp878 - (iTemp879) as F64) * (fTemp881 - (fTemp882 + fTemp704 * (fTemp883 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp880, 4)) as usize] } - fTemp881)))))} - fTemp876) / (1.0 - fTemp876))) as i32;
			let mut fTemp891: F64 = if iTemp890 != 0 {fTemp860} else {fTemp863};
			let mut fTemp892: F64 = if iTemp890 != 0 {fTemp863} else {fTemp861};
			let mut fTemp893: F64 = fTemp892 + fTemp891;
			let mut fTemp894: F64 = 0.5 * fTemp893;
			let mut fTemp895: F64 = 131071.0 * (1.0 - fTemp894);
			let mut iTemp896: i32 = (fTemp895) as i32;
			let mut iTemp897: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp896, 131071)))), 393215));
			let mut fTemp898: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp897, 3)) as usize] };
			let mut fTemp899: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp897 as usize] };
			let mut fTemp900: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp897, 1)) as usize] } - fTemp899;
			let mut fTemp901: F64 = 65535.5 * fTemp893;
			let mut iTemp902: i32 = (fTemp901) as i32;
			let mut iTemp903: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp902, 131071)))), 393215));
			let mut fTemp904: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp903, 3)) as usize] };
			let mut fTemp905: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp903 as usize] };
			let mut fTemp906: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp903, 1)) as usize] } - fTemp905;
			let mut fTemp907: F64 = if iTemp695 != 0 {fTemp905 + fTemp704 * fTemp906 + (fTemp901 - (iTemp902) as F64) * (fTemp904 - (fTemp905 + fTemp704 * (fTemp906 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp903, 4)) as usize] } - fTemp904))))} else {1.0 - (fTemp899 + fTemp704 * fTemp900 + (fTemp895 - (iTemp896) as F64) * (fTemp898 - (fTemp899 + fTemp704 * (fTemp900 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp897, 4)) as usize] } - fTemp898)))))};
			let mut fTemp908: F64 = fTemp709 + fTemp894;
			let mut fTemp909: F64 = 131071.0 * (1.0 - fTemp908);
			let mut iTemp910: i32 = (fTemp909) as i32;
			let mut iTemp911: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp910, 131071)))), 393215));
			let mut fTemp912: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp911, 3)) as usize] };
			let mut fTemp913: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp911 as usize] };
			let mut fTemp914: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp911, 1)) as usize] } - fTemp913;
			let mut fTemp915: F64 = 131071.0 * fTemp908;
			let mut iTemp916: i32 = (fTemp915) as i32;
			let mut iTemp917: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp916, 131071)))), 393215));
			let mut fTemp918: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp917, 3), 393215))) as usize] };
			let mut fTemp919: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp917 as usize] };
			let mut fTemp920: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp917, 1), 393215))) as usize] } - fTemp919;
			let mut iTemp921: i32 = (fTemp765 > ((if iTemp695 != 0 {fTemp919 + fTemp704 * fTemp920 + (fTemp915 - (iTemp916) as F64) * (fTemp918 - (fTemp919 + fTemp704 * (fTemp920 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp917, 4), 393215))) as usize] } - fTemp918))))} else {1.0 - (fTemp913 + fTemp704 * fTemp914 + (fTemp909 - (iTemp910) as F64) * (fTemp912 - (fTemp913 + fTemp704 * (fTemp914 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp911, 4)) as usize] } - fTemp912)))))} - fTemp907) / (1.0 - fTemp907))) as i32;
			let mut fTemp922: F64 = if iTemp921 != 0 {fTemp891} else {fTemp894};
			let mut fTemp923: F64 = if iTemp921 != 0 {fTemp894} else {fTemp892};
			let mut fTemp924: F64 = fTemp923 + fTemp922;
			let mut fTemp925: F64 = 0.5 * fTemp924;
			let mut fTemp926: F64 = 131071.0 * (1.0 - fTemp925);
			let mut iTemp927: i32 = (fTemp926) as i32;
			let mut iTemp928: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp927, 131071)))), 393215));
			let mut fTemp929: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp928, 3)) as usize] };
			let mut fTemp930: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp928 as usize] };
			let mut fTemp931: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp928, 1)) as usize] } - fTemp930;
			let mut fTemp932: F64 = 65535.5 * fTemp924;
			let mut iTemp933: i32 = (fTemp932) as i32;
			let mut iTemp934: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp933, 131071)))), 393215));
			let mut fTemp935: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp934, 3)) as usize] };
			let mut fTemp936: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp934 as usize] };
			let mut fTemp937: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp934, 1)) as usize] } - fTemp936;
			let mut fTemp938: F64 = if iTemp695 != 0 {fTemp936 + fTemp704 * fTemp937 + (fTemp932 - (iTemp933) as F64) * (fTemp935 - (fTemp936 + fTemp704 * (fTemp937 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp934, 4)) as usize] } - fTemp935))))} else {1.0 - (fTemp930 + fTemp704 * fTemp931 + (fTemp926 - (iTemp927) as F64) * (fTemp929 - (fTemp930 + fTemp704 * (fTemp931 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp928, 4)) as usize] } - fTemp929)))))};
			let mut fTemp939: F64 = fTemp709 + fTemp925;
			let mut fTemp940: F64 = 131071.0 * (1.0 - fTemp939);
			let mut iTemp941: i32 = (fTemp940) as i32;
			let mut iTemp942: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp941, 131071)))), 393215));
			let mut fTemp943: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp942, 3)) as usize] };
			let mut fTemp944: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp942 as usize] };
			let mut fTemp945: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp942, 1)) as usize] } - fTemp944;
			let mut fTemp946: F64 = 131071.0 * fTemp939;
			let mut iTemp947: i32 = (fTemp946) as i32;
			let mut iTemp948: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp947, 131071)))), 393215));
			let mut fTemp949: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp948, 3), 393215))) as usize] };
			let mut fTemp950: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp948 as usize] };
			let mut fTemp951: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp948, 1), 393215))) as usize] } - fTemp950;
			let mut iTemp952: i32 = (fTemp765 > ((if iTemp695 != 0 {fTemp950 + fTemp704 * fTemp951 + (fTemp946 - (iTemp947) as F64) * (fTemp949 - (fTemp950 + fTemp704 * (fTemp951 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp948, 4), 393215))) as usize] } - fTemp949))))} else {1.0 - (fTemp944 + fTemp704 * fTemp945 + (fTemp940 - (iTemp941) as F64) * (fTemp943 - (fTemp944 + fTemp704 * (fTemp945 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp942, 4)) as usize] } - fTemp943)))))} - fTemp938) / (1.0 - fTemp938))) as i32;
			let mut fTemp953: F64 = if iTemp952 != 0 {fTemp922} else {fTemp925};
			let mut fTemp954: F64 = if iTemp952 != 0 {fTemp925} else {fTemp923};
			let mut fTemp955: F64 = fTemp954 + fTemp953;
			let mut fTemp956: F64 = 0.5 * fTemp955;
			let mut fTemp957: F64 = 131071.0 * (1.0 - fTemp956);
			let mut iTemp958: i32 = (fTemp957) as i32;
			let mut iTemp959: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp958, 131071)))), 393215));
			let mut fTemp960: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp959, 3)) as usize] };
			let mut fTemp961: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp959 as usize] };
			let mut fTemp962: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp959, 1)) as usize] } - fTemp961;
			let mut fTemp963: F64 = 65535.5 * fTemp955;
			let mut iTemp964: i32 = (fTemp963) as i32;
			let mut iTemp965: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp964, 131071)))), 393215));
			let mut fTemp966: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp965, 3)) as usize] };
			let mut fTemp967: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp965 as usize] };
			let mut fTemp968: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp965, 1)) as usize] } - fTemp967;
			let mut fTemp969: F64 = if iTemp695 != 0 {fTemp967 + fTemp704 * fTemp968 + (fTemp963 - (iTemp964) as F64) * (fTemp966 - (fTemp967 + fTemp704 * (fTemp968 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp965, 4)) as usize] } - fTemp966))))} else {1.0 - (fTemp961 + fTemp704 * fTemp962 + (fTemp957 - (iTemp958) as F64) * (fTemp960 - (fTemp961 + fTemp704 * (fTemp962 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp959, 4)) as usize] } - fTemp960)))))};
			let mut fTemp970: F64 = fTemp709 + fTemp956;
			let mut fTemp971: F64 = 131071.0 * (1.0 - fTemp970);
			let mut iTemp972: i32 = (fTemp971) as i32;
			let mut iTemp973: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp972, 131071)))), 393215));
			let mut fTemp974: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp973, 3)) as usize] };
			let mut fTemp975: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp973 as usize] };
			let mut fTemp976: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp973, 1)) as usize] } - fTemp975;
			let mut fTemp977: F64 = 131071.0 * fTemp970;
			let mut iTemp978: i32 = (fTemp977) as i32;
			let mut iTemp979: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp978, 131071)))), 393215));
			let mut fTemp980: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp979, 3), 393215))) as usize] };
			let mut fTemp981: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp979 as usize] };
			let mut fTemp982: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp979, 1), 393215))) as usize] } - fTemp981;
			let mut iTemp983: i32 = (fTemp765 > ((if iTemp695 != 0 {fTemp981 + fTemp704 * fTemp982 + (fTemp977 - (iTemp978) as F64) * (fTemp980 - (fTemp981 + fTemp704 * (fTemp982 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp979, 4), 393215))) as usize] } - fTemp980))))} else {1.0 - (fTemp975 + fTemp704 * fTemp976 + (fTemp971 - (iTemp972) as F64) * (fTemp974 - (fTemp975 + fTemp704 * (fTemp976 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp973, 4)) as usize] } - fTemp974)))))} - fTemp969) / (1.0 - fTemp969))) as i32;
			let mut fTemp984: F64 = if iTemp983 != 0 {fTemp953} else {fTemp956};
			let mut fTemp985: F64 = if iTemp983 != 0 {fTemp956} else {fTemp954};
			let mut fTemp986: F64 = fTemp985 + fTemp984;
			let mut fTemp987: F64 = 0.5 * fTemp986;
			let mut fTemp988: F64 = 131071.0 * (1.0 - fTemp987);
			let mut iTemp989: i32 = (fTemp988) as i32;
			let mut iTemp990: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp989, 131071)))), 393215));
			let mut fTemp991: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp990, 3)) as usize] };
			let mut fTemp992: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp990 as usize] };
			let mut fTemp993: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp990, 1)) as usize] } - fTemp992;
			let mut fTemp994: F64 = 65535.5 * fTemp986;
			let mut iTemp995: i32 = (fTemp994) as i32;
			let mut iTemp996: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp995, 131071)))), 393215));
			let mut fTemp997: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp996, 3)) as usize] };
			let mut fTemp998: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp996 as usize] };
			let mut fTemp999: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp996, 1)) as usize] } - fTemp998;
			let mut fTemp1000: F64 = if iTemp695 != 0 {fTemp998 + fTemp704 * fTemp999 + (fTemp994 - (iTemp995) as F64) * (fTemp997 - (fTemp998 + fTemp704 * (fTemp999 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp996, 4)) as usize] } - fTemp997))))} else {1.0 - (fTemp992 + fTemp704 * fTemp993 + (fTemp988 - (iTemp989) as F64) * (fTemp991 - (fTemp992 + fTemp704 * (fTemp993 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp990, 4)) as usize] } - fTemp991)))))};
			let mut fTemp1001: F64 = fTemp709 + fTemp987;
			let mut fTemp1002: F64 = 131071.0 * (1.0 - fTemp1001);
			let mut iTemp1003: i32 = (fTemp1002) as i32;
			let mut iTemp1004: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1003, 131071)))), 393215));
			let mut fTemp1005: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1004, 3)) as usize] };
			let mut fTemp1006: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1004 as usize] };
			let mut fTemp1007: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1004, 1)) as usize] } - fTemp1006;
			let mut fTemp1008: F64 = 131071.0 * fTemp1001;
			let mut iTemp1009: i32 = (fTemp1008) as i32;
			let mut iTemp1010: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1009, 131071)))), 393215));
			let mut fTemp1011: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1010, 3), 393215))) as usize] };
			let mut fTemp1012: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1010 as usize] };
			let mut fTemp1013: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1010, 1), 393215))) as usize] } - fTemp1012;
			let mut iTemp1014: i32 = (fTemp765 > ((if iTemp695 != 0 {fTemp1012 + fTemp704 * fTemp1013 + (fTemp1008 - (iTemp1009) as F64) * (fTemp1011 - (fTemp1012 + fTemp704 * (fTemp1013 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1010, 4), 393215))) as usize] } - fTemp1011))))} else {1.0 - (fTemp1006 + fTemp704 * fTemp1007 + (fTemp1002 - (iTemp1003) as F64) * (fTemp1005 - (fTemp1006 + fTemp704 * (fTemp1007 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1004, 4)) as usize] } - fTemp1005)))))} - fTemp1000) / (1.0 - fTemp1000))) as i32;
			let mut fTemp1015: F64 = if iTemp1014 != 0 {fTemp984} else {fTemp987};
			let mut fTemp1016: F64 = if iTemp1014 != 0 {fTemp987} else {fTemp985};
			let mut fTemp1017: F64 = fTemp1016 + fTemp1015;
			let mut fTemp1018: F64 = 0.5 * fTemp1017;
			let mut fTemp1019: F64 = 131071.0 * (1.0 - fTemp1018);
			let mut iTemp1020: i32 = (fTemp1019) as i32;
			let mut iTemp1021: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1020, 131071)))), 393215));
			let mut fTemp1022: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1021, 3)) as usize] };
			let mut fTemp1023: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1021 as usize] };
			let mut fTemp1024: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1021, 1)) as usize] } - fTemp1023;
			let mut fTemp1025: F64 = 65535.5 * fTemp1017;
			let mut iTemp1026: i32 = (fTemp1025) as i32;
			let mut iTemp1027: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1026, 131071)))), 393215));
			let mut fTemp1028: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1027, 3)) as usize] };
			let mut fTemp1029: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1027 as usize] };
			let mut fTemp1030: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1027, 1)) as usize] } - fTemp1029;
			let mut fTemp1031: F64 = if iTemp695 != 0 {fTemp1029 + fTemp704 * fTemp1030 + (fTemp1025 - (iTemp1026) as F64) * (fTemp1028 - (fTemp1029 + fTemp704 * (fTemp1030 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1027, 4)) as usize] } - fTemp1028))))} else {1.0 - (fTemp1023 + fTemp704 * fTemp1024 + (fTemp1019 - (iTemp1020) as F64) * (fTemp1022 - (fTemp1023 + fTemp704 * (fTemp1024 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1021, 4)) as usize] } - fTemp1022)))))};
			let mut fTemp1032: F64 = fTemp709 + fTemp1018;
			let mut fTemp1033: F64 = 131071.0 * (1.0 - fTemp1032);
			let mut iTemp1034: i32 = (fTemp1033) as i32;
			let mut iTemp1035: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1034, 131071)))), 393215));
			let mut fTemp1036: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1035, 3)) as usize] };
			let mut fTemp1037: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1035 as usize] };
			let mut fTemp1038: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1035, 1)) as usize] } - fTemp1037;
			let mut fTemp1039: F64 = 131071.0 * fTemp1032;
			let mut iTemp1040: i32 = (fTemp1039) as i32;
			let mut iTemp1041: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1040, 131071)))), 393215));
			let mut fTemp1042: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1041, 3), 393215))) as usize] };
			let mut fTemp1043: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1041 as usize] };
			let mut fTemp1044: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1041, 1), 393215))) as usize] } - fTemp1043;
			let mut iTemp1045: i32 = (fTemp765 > ((if iTemp695 != 0 {fTemp1043 + fTemp704 * fTemp1044 + (fTemp1039 - (iTemp1040) as F64) * (fTemp1042 - (fTemp1043 + fTemp704 * (fTemp1044 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1041, 4), 393215))) as usize] } - fTemp1042))))} else {1.0 - (fTemp1037 + fTemp704 * fTemp1038 + (fTemp1033 - (iTemp1034) as F64) * (fTemp1036 - (fTemp1037 + fTemp704 * (fTemp1038 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1035, 4)) as usize] } - fTemp1036)))))} - fTemp1031) / (1.0 - fTemp1031))) as i32;
			let mut fTemp1046: F64 = if iTemp1045 != 0 {fTemp1015} else {fTemp1018};
			let mut fTemp1047: F64 = if iTemp1045 != 0 {fTemp1018} else {fTemp1016};
			let mut fTemp1048: F64 = fTemp1047 + fTemp1046;
			let mut fTemp1049: F64 = 0.5 * fTemp1048;
			let mut fTemp1050: F64 = 131071.0 * (1.0 - fTemp1049);
			let mut iTemp1051: i32 = (fTemp1050) as i32;
			let mut iTemp1052: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1051, 131071)))), 393215));
			let mut fTemp1053: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1052, 3)) as usize] };
			let mut fTemp1054: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1052 as usize] };
			let mut fTemp1055: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1052, 1)) as usize] } - fTemp1054;
			let mut fTemp1056: F64 = 65535.5 * fTemp1048;
			let mut iTemp1057: i32 = (fTemp1056) as i32;
			let mut iTemp1058: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1057, 131071)))), 393215));
			let mut fTemp1059: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1058, 3)) as usize] };
			let mut fTemp1060: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1058 as usize] };
			let mut fTemp1061: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1058, 1)) as usize] } - fTemp1060;
			let mut fTemp1062: F64 = if iTemp695 != 0 {fTemp1060 + fTemp704 * fTemp1061 + (fTemp1056 - (iTemp1057) as F64) * (fTemp1059 - (fTemp1060 + fTemp704 * (fTemp1061 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1058, 4)) as usize] } - fTemp1059))))} else {1.0 - (fTemp1054 + fTemp704 * fTemp1055 + (fTemp1050 - (iTemp1051) as F64) * (fTemp1053 - (fTemp1054 + fTemp704 * (fTemp1055 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1052, 4)) as usize] } - fTemp1053)))))};
			let mut fTemp1063: F64 = fTemp709 + fTemp1049;
			let mut fTemp1064: F64 = 131071.0 * (1.0 - fTemp1063);
			let mut iTemp1065: i32 = (fTemp1064) as i32;
			let mut iTemp1066: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1065, 131071)))), 393215));
			let mut fTemp1067: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1066, 3)) as usize] };
			let mut fTemp1068: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1066 as usize] };
			let mut fTemp1069: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1066, 1)) as usize] } - fTemp1068;
			let mut fTemp1070: F64 = 131071.0 * fTemp1063;
			let mut iTemp1071: i32 = (fTemp1070) as i32;
			let mut iTemp1072: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1071, 131071)))), 393215));
			let mut fTemp1073: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1072, 3), 393215))) as usize] };
			let mut fTemp1074: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1072 as usize] };
			let mut fTemp1075: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1072, 1), 393215))) as usize] } - fTemp1074;
			let mut iTemp1076: i32 = (fTemp765 > ((if iTemp695 != 0 {fTemp1074 + fTemp704 * fTemp1075 + (fTemp1070 - (iTemp1071) as F64) * (fTemp1073 - (fTemp1074 + fTemp704 * (fTemp1075 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1072, 4), 393215))) as usize] } - fTemp1073))))} else {1.0 - (fTemp1068 + fTemp704 * fTemp1069 + (fTemp1064 - (iTemp1065) as F64) * (fTemp1067 - (fTemp1068 + fTemp704 * (fTemp1069 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1066, 4)) as usize] } - fTemp1067)))))} - fTemp1062) / (1.0 - fTemp1062))) as i32;
			let mut fTemp1077: F64 = if iTemp1076 != 0 {fTemp1046} else {fTemp1049};
			let mut fTemp1078: F64 = if iTemp1076 != 0 {fTemp1049} else {fTemp1047};
			let mut fTemp1079: F64 = fTemp1078 + fTemp1077;
			let mut fTemp1080: F64 = 0.5 * fTemp1079;
			let mut fTemp1081: F64 = 131071.0 * (1.0 - fTemp1080);
			let mut iTemp1082: i32 = (fTemp1081) as i32;
			let mut iTemp1083: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1082, 131071)))), 393215));
			let mut fTemp1084: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1083, 3)) as usize] };
			let mut fTemp1085: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1083 as usize] };
			let mut fTemp1086: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1083, 1)) as usize] } - fTemp1085;
			let mut fTemp1087: F64 = 65535.5 * fTemp1079;
			let mut iTemp1088: i32 = (fTemp1087) as i32;
			let mut iTemp1089: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1088, 131071)))), 393215));
			let mut fTemp1090: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1089, 3)) as usize] };
			let mut fTemp1091: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1089 as usize] };
			let mut fTemp1092: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1089, 1)) as usize] } - fTemp1091;
			let mut fTemp1093: F64 = if iTemp695 != 0 {fTemp1091 + fTemp704 * fTemp1092 + (fTemp1087 - (iTemp1088) as F64) * (fTemp1090 - (fTemp1091 + fTemp704 * (fTemp1092 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1089, 4)) as usize] } - fTemp1090))))} else {1.0 - (fTemp1085 + fTemp704 * fTemp1086 + (fTemp1081 - (iTemp1082) as F64) * (fTemp1084 - (fTemp1085 + fTemp704 * (fTemp1086 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1083, 4)) as usize] } - fTemp1084)))))};
			let mut fTemp1094: F64 = fTemp709 + fTemp1080;
			let mut fTemp1095: F64 = 131071.0 * (1.0 - fTemp1094);
			let mut iTemp1096: i32 = (fTemp1095) as i32;
			let mut iTemp1097: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1096, 131071)))), 393215));
			let mut fTemp1098: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1097, 3)) as usize] };
			let mut fTemp1099: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1097 as usize] };
			let mut fTemp1100: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1097, 1)) as usize] } - fTemp1099;
			let mut fTemp1101: F64 = 131071.0 * fTemp1094;
			let mut iTemp1102: i32 = (fTemp1101) as i32;
			let mut iTemp1103: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1102, 131071)))), 393215));
			let mut fTemp1104: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1103, 3), 393215))) as usize] };
			let mut fTemp1105: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1103 as usize] };
			let mut fTemp1106: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1103, 1), 393215))) as usize] } - fTemp1105;
			let mut iTemp1107: i32 = (fTemp765 > ((if iTemp695 != 0 {fTemp1105 + fTemp704 * fTemp1106 + (fTemp1101 - (iTemp1102) as F64) * (fTemp1104 - (fTemp1105 + fTemp704 * (fTemp1106 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1103, 4), 393215))) as usize] } - fTemp1104))))} else {1.0 - (fTemp1099 + fTemp704 * fTemp1100 + (fTemp1095 - (iTemp1096) as F64) * (fTemp1098 - (fTemp1099 + fTemp704 * (fTemp1100 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1097, 4)) as usize] } - fTemp1098)))))} - fTemp1093) / (1.0 - fTemp1093))) as i32;
			let mut fTemp1108: F64 = if iTemp1107 != 0 {fTemp1077} else {fTemp1080};
			let mut fTemp1109: F64 = if iTemp1107 != 0 {fTemp1080} else {fTemp1078};
			let mut fTemp1110: F64 = fTemp1109 + fTemp1108;
			let mut fTemp1111: F64 = 0.5 * fTemp1110;
			let mut fTemp1112: F64 = 131071.0 * (1.0 - fTemp1111);
			let mut iTemp1113: i32 = (fTemp1112) as i32;
			let mut iTemp1114: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1113, 131071)))), 393215));
			let mut fTemp1115: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1114, 3)) as usize] };
			let mut fTemp1116: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1114 as usize] };
			let mut fTemp1117: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1114, 1)) as usize] } - fTemp1116;
			let mut fTemp1118: F64 = 65535.5 * fTemp1110;
			let mut iTemp1119: i32 = (fTemp1118) as i32;
			let mut iTemp1120: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1119, 131071)))), 393215));
			let mut fTemp1121: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1120, 3)) as usize] };
			let mut fTemp1122: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1120 as usize] };
			let mut fTemp1123: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1120, 1)) as usize] } - fTemp1122;
			let mut fTemp1124: F64 = if iTemp695 != 0 {fTemp1122 + fTemp704 * fTemp1123 + (fTemp1118 - (iTemp1119) as F64) * (fTemp1121 - (fTemp1122 + fTemp704 * (fTemp1123 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1120, 4)) as usize] } - fTemp1121))))} else {1.0 - (fTemp1116 + fTemp704 * fTemp1117 + (fTemp1112 - (iTemp1113) as F64) * (fTemp1115 - (fTemp1116 + fTemp704 * (fTemp1117 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1114, 4)) as usize] } - fTemp1115)))))};
			let mut fTemp1125: F64 = fTemp709 + fTemp1111;
			let mut fTemp1126: F64 = 131071.0 * (1.0 - fTemp1125);
			let mut iTemp1127: i32 = (fTemp1126) as i32;
			let mut iTemp1128: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1127, 131071)))), 393215));
			let mut fTemp1129: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1128, 3)) as usize] };
			let mut fTemp1130: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1128 as usize] };
			let mut fTemp1131: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1128, 1)) as usize] } - fTemp1130;
			let mut fTemp1132: F64 = 131071.0 * fTemp1125;
			let mut iTemp1133: i32 = (fTemp1132) as i32;
			let mut iTemp1134: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1133, 131071)))), 393215));
			let mut fTemp1135: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1134, 3), 393215))) as usize] };
			let mut fTemp1136: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1134 as usize] };
			let mut fTemp1137: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1134, 1), 393215))) as usize] } - fTemp1136;
			let mut iTemp1138: i32 = (fTemp765 > ((if iTemp695 != 0 {fTemp1136 + fTemp704 * fTemp1137 + (fTemp1132 - (iTemp1133) as F64) * (fTemp1135 - (fTemp1136 + fTemp704 * (fTemp1137 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1134, 4), 393215))) as usize] } - fTemp1135))))} else {1.0 - (fTemp1130 + fTemp704 * fTemp1131 + (fTemp1126 - (iTemp1127) as F64) * (fTemp1129 - (fTemp1130 + fTemp704 * (fTemp1131 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1128, 4)) as usize] } - fTemp1129)))))} - fTemp1124) / (1.0 - fTemp1124))) as i32;
			let mut fTemp1139: F64 = if iTemp1138 != 0 {fTemp1108} else {fTemp1111};
			let mut fTemp1140: F64 = if iTemp1138 != 0 {fTemp1111} else {fTemp1109};
			let mut fTemp1141: F64 = fTemp1140 + fTemp1139;
			let mut fTemp1142: F64 = 0.5 * fTemp1141;
			let mut fTemp1143: F64 = 131071.0 * (1.0 - fTemp1142);
			let mut iTemp1144: i32 = (fTemp1143) as i32;
			let mut iTemp1145: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1144, 131071)))), 393215));
			let mut fTemp1146: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1145, 3)) as usize] };
			let mut fTemp1147: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1145 as usize] };
			let mut fTemp1148: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1145, 1)) as usize] } - fTemp1147;
			let mut fTemp1149: F64 = 65535.5 * fTemp1141;
			let mut iTemp1150: i32 = (fTemp1149) as i32;
			let mut iTemp1151: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1150, 131071)))), 393215));
			let mut fTemp1152: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1151, 3)) as usize] };
			let mut fTemp1153: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1151 as usize] };
			let mut fTemp1154: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1151, 1)) as usize] } - fTemp1153;
			let mut fTemp1155: F64 = if iTemp695 != 0 {fTemp1153 + fTemp704 * fTemp1154 + (fTemp1149 - (iTemp1150) as F64) * (fTemp1152 - (fTemp1153 + fTemp704 * (fTemp1154 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1151, 4)) as usize] } - fTemp1152))))} else {1.0 - (fTemp1147 + fTemp704 * fTemp1148 + (fTemp1143 - (iTemp1144) as F64) * (fTemp1146 - (fTemp1147 + fTemp704 * (fTemp1148 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1145, 4)) as usize] } - fTemp1146)))))};
			let mut fTemp1156: F64 = fTemp709 + fTemp1142;
			let mut fTemp1157: F64 = 131071.0 * (1.0 - fTemp1156);
			let mut iTemp1158: i32 = (fTemp1157) as i32;
			let mut iTemp1159: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1158, 131071)))), 393215));
			let mut fTemp1160: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1159, 3)) as usize] };
			let mut fTemp1161: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1159 as usize] };
			let mut fTemp1162: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1159, 1)) as usize] } - fTemp1161;
			let mut fTemp1163: F64 = 131071.0 * fTemp1156;
			let mut iTemp1164: i32 = (fTemp1163) as i32;
			let mut iTemp1165: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1164, 131071)))), 393215));
			let mut fTemp1166: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1165, 3), 393215))) as usize] };
			let mut fTemp1167: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1165 as usize] };
			let mut fTemp1168: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1165, 1), 393215))) as usize] } - fTemp1167;
			let mut iTemp1169: i32 = (fTemp765 > ((if iTemp695 != 0 {fTemp1167 + fTemp704 * fTemp1168 + (fTemp1163 - (iTemp1164) as F64) * (fTemp1166 - (fTemp1167 + fTemp704 * (fTemp1168 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1165, 4), 393215))) as usize] } - fTemp1166))))} else {1.0 - (fTemp1161 + fTemp704 * fTemp1162 + (fTemp1157 - (iTemp1158) as F64) * (fTemp1160 - (fTemp1161 + fTemp704 * (fTemp1162 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1159, 4)) as usize] } - fTemp1160)))))} - fTemp1155) / (1.0 - fTemp1155))) as i32;
			let mut fTemp1170: F64 = if iTemp1169 != 0 {fTemp1139} else {fTemp1142};
			let mut fTemp1171: F64 = if iTemp1169 != 0 {fTemp1142} else {fTemp1140};
			let mut fTemp1172: F64 = fTemp1171 + fTemp1170;
			let mut fTemp1173: F64 = 0.5 * fTemp1172;
			let mut fTemp1174: F64 = 131071.0 * (1.0 - fTemp1173);
			let mut iTemp1175: i32 = (fTemp1174) as i32;
			let mut iTemp1176: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1175, 131071)))), 393215));
			let mut fTemp1177: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1176, 3)) as usize] };
			let mut fTemp1178: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1176 as usize] };
			let mut fTemp1179: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1176, 1)) as usize] } - fTemp1178;
			let mut fTemp1180: F64 = 65535.5 * fTemp1172;
			let mut iTemp1181: i32 = (fTemp1180) as i32;
			let mut iTemp1182: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1181, 131071)))), 393215));
			let mut fTemp1183: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1182, 3)) as usize] };
			let mut fTemp1184: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1182 as usize] };
			let mut fTemp1185: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1182, 1)) as usize] } - fTemp1184;
			let mut fTemp1186: F64 = if iTemp695 != 0 {fTemp1184 + fTemp704 * fTemp1185 + (fTemp1180 - (iTemp1181) as F64) * (fTemp1183 - (fTemp1184 + fTemp704 * (fTemp1185 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1182, 4)) as usize] } - fTemp1183))))} else {1.0 - (fTemp1178 + fTemp704 * fTemp1179 + (fTemp1174 - (iTemp1175) as F64) * (fTemp1177 - (fTemp1178 + fTemp704 * (fTemp1179 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1176, 4)) as usize] } - fTemp1177)))))};
			let mut fTemp1187: F64 = fTemp709 + fTemp1173;
			let mut fTemp1188: F64 = 131071.0 * (1.0 - fTemp1187);
			let mut iTemp1189: i32 = (fTemp1188) as i32;
			let mut iTemp1190: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1189, 131071)))), 393215));
			let mut fTemp1191: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1190, 3)) as usize] };
			let mut fTemp1192: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1190 as usize] };
			let mut fTemp1193: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1190, 1)) as usize] } - fTemp1192;
			let mut fTemp1194: F64 = 131071.0 * fTemp1187;
			let mut iTemp1195: i32 = (fTemp1194) as i32;
			let mut iTemp1196: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1195, 131071)))), 393215));
			let mut fTemp1197: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1196, 3), 393215))) as usize] };
			let mut fTemp1198: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1196 as usize] };
			let mut fTemp1199: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1196, 1), 393215))) as usize] } - fTemp1198;
			let mut iTemp1200: i32 = (fTemp765 > ((if iTemp695 != 0 {fTemp1198 + fTemp704 * fTemp1199 + (fTemp1194 - (iTemp1195) as F64) * (fTemp1197 - (fTemp1198 + fTemp704 * (fTemp1199 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1196, 4), 393215))) as usize] } - fTemp1197))))} else {1.0 - (fTemp1192 + fTemp704 * fTemp1193 + (fTemp1188 - (iTemp1189) as F64) * (fTemp1191 - (fTemp1192 + fTemp704 * (fTemp1193 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1190, 4)) as usize] } - fTemp1191)))))} - fTemp1186) / (1.0 - fTemp1186))) as i32;
			let mut fTemp1201: F64 = if iTemp1200 != 0 {fTemp1170} else {fTemp1173};
			let mut fTemp1202: F64 = if iTemp1200 != 0 {fTemp1173} else {fTemp1171};
			let mut fTemp1203: F64 = fTemp1202 + fTemp1201;
			let mut fTemp1204: F64 = 0.5 * fTemp1203;
			let mut fTemp1205: F64 = 131071.0 * (1.0 - fTemp1204);
			let mut iTemp1206: i32 = (fTemp1205) as i32;
			let mut iTemp1207: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1206, 131071)))), 393215));
			let mut fTemp1208: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1207, 3)) as usize] };
			let mut fTemp1209: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1207 as usize] };
			let mut fTemp1210: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1207, 1)) as usize] } - fTemp1209;
			let mut fTemp1211: F64 = 65535.5 * fTemp1203;
			let mut iTemp1212: i32 = (fTemp1211) as i32;
			let mut iTemp1213: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1212, 131071)))), 393215));
			let mut fTemp1214: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1213, 3)) as usize] };
			let mut fTemp1215: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1213 as usize] };
			let mut fTemp1216: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1213, 1)) as usize] } - fTemp1215;
			let mut fTemp1217: F64 = if iTemp695 != 0 {fTemp1215 + fTemp704 * fTemp1216 + (fTemp1211 - (iTemp1212) as F64) * (fTemp1214 - (fTemp1215 + fTemp704 * (fTemp1216 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1213, 4)) as usize] } - fTemp1214))))} else {1.0 - (fTemp1209 + fTemp704 * fTemp1210 + (fTemp1205 - (iTemp1206) as F64) * (fTemp1208 - (fTemp1209 + fTemp704 * (fTemp1210 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1207, 4)) as usize] } - fTemp1208)))))};
			let mut fTemp1218: F64 = fTemp709 + fTemp1204;
			let mut fTemp1219: F64 = 131071.0 * (1.0 - fTemp1218);
			let mut iTemp1220: i32 = (fTemp1219) as i32;
			let mut iTemp1221: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1220, 131071)))), 393215));
			let mut fTemp1222: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1221, 3)) as usize] };
			let mut fTemp1223: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1221 as usize] };
			let mut fTemp1224: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1221, 1)) as usize] } - fTemp1223;
			let mut fTemp1225: F64 = 131071.0 * fTemp1218;
			let mut iTemp1226: i32 = (fTemp1225) as i32;
			let mut iTemp1227: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1226, 131071)))), 393215));
			let mut fTemp1228: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1227, 3), 393215))) as usize] };
			let mut fTemp1229: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1227 as usize] };
			let mut fTemp1230: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1227, 1), 393215))) as usize] } - fTemp1229;
			let mut iTemp1231: i32 = (fTemp765 > ((if iTemp695 != 0 {fTemp1229 + fTemp704 * fTemp1230 + (fTemp1225 - (iTemp1226) as F64) * (fTemp1228 - (fTemp1229 + fTemp704 * (fTemp1230 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1227, 4), 393215))) as usize] } - fTemp1228))))} else {1.0 - (fTemp1223 + fTemp704 * fTemp1224 + (fTemp1219 - (iTemp1220) as F64) * (fTemp1222 - (fTemp1223 + fTemp704 * (fTemp1224 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1221, 4)) as usize] } - fTemp1222)))))} - fTemp1217) / (1.0 - fTemp1217))) as i32;
			let mut fTemp1232: F64 = if iTemp1231 != 0 {fTemp1201} else {fTemp1204};
			let mut fTemp1233: F64 = if iTemp1231 != 0 {fTemp1204} else {fTemp1202};
			let mut fTemp1234: F64 = fTemp1233 + fTemp1232;
			let mut fTemp1235: F64 = 0.5 * fTemp1234;
			let mut fTemp1236: F64 = 131071.0 * (1.0 - fTemp1235);
			let mut iTemp1237: i32 = (fTemp1236) as i32;
			let mut iTemp1238: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1237, 131071)))), 393215));
			let mut fTemp1239: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1238, 3)) as usize] };
			let mut fTemp1240: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1238 as usize] };
			let mut fTemp1241: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1238, 1)) as usize] } - fTemp1240;
			let mut fTemp1242: F64 = 65535.5 * fTemp1234;
			let mut iTemp1243: i32 = (fTemp1242) as i32;
			let mut iTemp1244: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1243, 131071)))), 393215));
			let mut fTemp1245: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1244, 3)) as usize] };
			let mut fTemp1246: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1244 as usize] };
			let mut fTemp1247: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1244, 1)) as usize] } - fTemp1246;
			let mut fTemp1248: F64 = if iTemp695 != 0 {fTemp1246 + fTemp704 * fTemp1247 + (fTemp1242 - (iTemp1243) as F64) * (fTemp1245 - (fTemp1246 + fTemp704 * (fTemp1247 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1244, 4), 393215))) as usize] } - fTemp1245))))} else {1.0 - (fTemp1240 + fTemp704 * fTemp1241 + (fTemp1236 - (iTemp1237) as F64) * (fTemp1239 - (fTemp1240 + fTemp704 * (fTemp1241 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1238, 4), 393215))) as usize] } - fTemp1239)))))};
			let mut fTemp1249: F64 = fTemp709 + fTemp1235;
			let mut fTemp1250: F64 = 131071.0 * (1.0 - fTemp1249);
			let mut iTemp1251: i32 = (fTemp1250) as i32;
			let mut iTemp1252: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1251, 131071)))), 393215));
			let mut fTemp1253: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1252, 3)) as usize] };
			let mut fTemp1254: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1252 as usize] };
			let mut fTemp1255: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1252, 1)) as usize] } - fTemp1254;
			let mut fTemp1256: F64 = 131071.0 * fTemp1249;
			let mut iTemp1257: i32 = (fTemp1256) as i32;
			let mut iTemp1258: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1257, 131071)))), 393215));
			let mut fTemp1259: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1258, 3), 393215))) as usize] };
			let mut fTemp1260: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1258 as usize] };
			let mut fTemp1261: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1258, 1), 393215))) as usize] } - fTemp1260;
			let mut iTemp1262: i32 = (fTemp765 > ((if iTemp695 != 0 {fTemp1260 + fTemp704 * fTemp1261 + (fTemp1256 - (iTemp1257) as F64) * (fTemp1259 - (fTemp1260 + fTemp704 * (fTemp1261 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1258, 4), 393215))) as usize] } - fTemp1259))))} else {1.0 - (fTemp1254 + fTemp704 * fTemp1255 + (fTemp1250 - (iTemp1251) as F64) * (fTemp1253 - (fTemp1254 + fTemp704 * (fTemp1255 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1252, 4)) as usize] } - fTemp1253)))))} - fTemp1248) / (1.0 - fTemp1248))) as i32;
			let mut fTemp1263: F64 = F64::min(1.0, F64::max(0.0, 0.5 * (if iTemp1262 != 0 {fTemp1235} else {fTemp1233} + if iTemp1262 != 0 {fTemp1232} else {fTemp1235})));
			self.fRec15[0] = fTemp1263;
			let mut fTemp1264: F64 = 131071.0 * (1.0 - fTemp1263);
			let mut iTemp1265: i32 = (fTemp1264) as i32;
			let mut iTemp1266: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1265, 131071)))), 393215));
			let mut fTemp1267: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1266, 3)) as usize] };
			let mut fTemp1268: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1266 as usize] };
			let mut fTemp1269: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1266, 1)) as usize] } - fTemp1268;
			let mut fTemp1270: F64 = 131071.0 * fTemp1263;
			let mut iTemp1271: i32 = (fTemp1270) as i32;
			let mut iTemp1272: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1271, 131071)))), 393215));
			let mut fTemp1273: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1272, 3)) as usize] };
			let mut fTemp1274: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1272 as usize] };
			let mut fTemp1275: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1272, 1)) as usize] } - fTemp1274;
			let mut fTemp1276: F64 = if iTemp695 != 0 {fTemp1274 + fTemp704 * fTemp1275 + (fTemp1270 - (iTemp1271) as F64) * (fTemp1273 - (fTemp1274 + fTemp704 * (fTemp1275 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1272, 4), 393215))) as usize] } - fTemp1273))))} else {1.0 - (fTemp1268 + fTemp704 * fTemp1269 + (fTemp1264 - (iTemp1265) as F64) * (fTemp1267 - (fTemp1268 + fTemp704 * (fTemp1269 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1266, 4), 393215))) as usize] } - fTemp1267)))))};
			let mut fTemp1277: F64 = fTemp709 + fTemp1263;
			let mut fTemp1278: F64 = 131071.0 * (1.0 - fTemp1277);
			let mut iTemp1279: i32 = (fTemp1278) as i32;
			let mut iTemp1280: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1279, 131071)))), 393215));
			let mut fTemp1281: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1280, 3)) as usize] };
			let mut fTemp1282: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1280 as usize] };
			let mut fTemp1283: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1280, 1)) as usize] } - fTemp1282;
			let mut fTemp1284: F64 = 131071.0 * fTemp1277;
			let mut iTemp1285: i32 = (fTemp1284) as i32;
			let mut iTemp1286: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1285, 131071)))), 393215));
			let mut fTemp1287: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1286, 3), 393215))) as usize] };
			let mut fTemp1288: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1286 as usize] };
			let mut fTemp1289: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1286, 1), 393215))) as usize] } - fTemp1288;
			let mut fTemp1290: F64 = fTemp667 + if ((0.001 * fTemp708) == 0.0) as i32 != 0 {fTemp694} else {fTemp694 * (if iTemp695 != 0 {fTemp1288 + fTemp704 * fTemp1289 + (fTemp1284 - (iTemp1285) as F64) * (fTemp1287 - (fTemp1288 + fTemp704 * (fTemp1289 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1286, 4), 393215))) as usize] } - fTemp1287))))} else {1.0 - (fTemp1282 + fTemp704 * fTemp1283 + (fTemp1278 - (iTemp1279) as F64) * (fTemp1281 - (fTemp1282 + fTemp704 * (fTemp1283 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1280, 4)) as usize] } - fTemp1281)))))} - fTemp1276) / (1.0 - fTemp1276)};
			self.fRec16[(self.IOTA0 & 16383) as usize] = if iTemp707 != 0 {F64::min(fTemp1290, fTemp667)} else {F64::max(fTemp1290, fTemp667)};
			let mut fTemp1291: F64 = self.fRec16[((i32::wrapping_sub(self.IOTA0, iSlow75)) & 16383) as usize];
			*output1 = 0.5 * fTemp2 * self.fVec3[((i32::wrapping_sub(self.IOTA0, iSlow18)) & 32767) as usize] + self.fRec14[0] * fTemp4 * self.fVec4[((i32::wrapping_sub(self.IOTA0, iSlow18)) & 32767) as usize] * fTemp1291;
			*output2 = fTemp3 + fTemp666 * fTemp4;
			*output3 = fTemp3 + fTemp4 * fTemp1291;
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
			self.fRec14[1] = self.fRec14[0];
			self.fVec34[2] = self.fVec34[1];
			self.fVec34[1] = self.fVec34[0];
			for j4 in (1..=6).rev() {
				self.fVec35[j4 as usize] = self.fVec35[(i32::wrapping_sub(j4, 1)) as usize];
			}
			for j5 in (1..=14).rev() {
				self.fVec36[j5 as usize] = self.fVec36[(i32::wrapping_sub(j5, 1)) as usize];
			}
			self.fRec17[1] = self.fRec17[0];
			self.fVec46[2] = self.fVec46[1];
			self.fVec46[1] = self.fVec46[0];
			for j6 in (1..=6).rev() {
				self.fVec47[j6 as usize] = self.fVec47[(i32::wrapping_sub(j6, 1)) as usize];
			}
			for j7 in (1..=14).rev() {
				self.fVec48[j7 as usize] = self.fVec48[(i32::wrapping_sub(j7, 1)) as usize];
			}
			self.fVec58[1] = self.fVec58[0];
			self.fVec59[1] = self.fVec59[0];
			self.fVec60[1] = self.fVec60[0];
			self.fRec15[1] = self.fRec15[0];
		}
	}

}

}

pub use dsp_96k::LambRs96k;
