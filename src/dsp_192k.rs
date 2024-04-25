/* ------------------------------------------------------------
author: "Bart Brouns"
license: "AGPLv3"
name: "lamb-rs"
version: "0.1"
Code generated with Faust 2.72.14 (https://faust.grame.fr)
Compilation options: -a /run/user/1001/.tmpJ2AMKD -lang rust -ct 1 -cn LambRs192k -es 1 -mcd 16 -mdd 1024 -mdy 33 -double -ftz 0
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
	iRec13: [i32;2],
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
	
	fn fillLambRs192kSIG0(&mut self, count: i32, table: &mut[F64]) {
		for i1 in 0..count {
			self.iRec13[0] = i32::wrapping_add(self.iRec13[1], 1);
			let mut iTemp70: i32 = i32::wrapping_add(self.iRec13[0], -1);
			let mut fTemp71: F64 = (iTemp70 % 3) as F64 as i32 as F64;
			let mut fTemp72: F64 = 0.5 * fTemp71;
			let mut fTemp73: F64 = F64::powf(fTemp72, 0.21 * fTemp71 + 1.0);
			let mut fTemp74: F64 = (0.3333333333333333 * (iTemp70 % 786432) as F64) as i32 as F64;
			table[i1 as usize] = F64::min(1.0, F64::max(0.0, if (fTemp72 == 0.0) as i32 != 0 {0.5 * (F64::sin(1.1984270621720943e-05 * fTemp74 + 4.71238898038469) + 1.0)} else {0.5 * (F64::sin(3.141592653589793 * ((1.0 - F64::exp(-(9.231602598581689e-06 * fTemp73 * fTemp74))) / (1.0 - F64::exp(-(2.42 * fTemp73)))) + 4.71238898038469) + 1.0)}));
			self.iRec13[1] = self.iRec13[0];
		}
	}

}


pub fn newLambRs192kSIG0() -> LambRs192kSIG0 { 
	LambRs192kSIG0 {
		iRec13: [0;2],
	}
}
fn LambRs192k_faustpower2_f(value: F64) -> F64 {
	return value * value;
}
static mut ftbl0LambRs192kSIG0: [F64;786432] = [0.0;786432];
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
pub struct LambRs192k {
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
	fVec17: [F64;8192],
	fVec18: [F64;16384],
	fRec3: [F64;2],
	fVec19: [F64;3],
	fVec20: [F64;7],
	fVec21: [F64;15],
	fVec22: [F64;32],
	fVec23: [F64;64],
	fVec24: [F64;128],
	fVec25: [F64;256],
	fVec26: [F64;512],
	fVec27: [F64;1024],
	fVec28: [F64;2048],
	fVec29: [F64;4096],
	fVec30: [F64;8192],
	fVec31: [F64;16384],
	fVec32: [F64;2],
	fHslider10: F64,
	fHslider11: F64,
	fVec33: [F64;2],
	fVec34: [F64;2],
	fConst10: F64,
	fRec1: [F64;2],
	fRec2: [F64;32768],
	fCheckbox1: F64,
	fHbargraph0: F64,
	fHslider12: F64,
	fRec14: [F64;2],
	fVec35: [F64;16384],
	fVec36: [F64;3],
	fVec37: [F64;7],
	fVec38: [F64;15],
	fVec39: [F64;32],
	fVec40: [F64;64],
	fVec41: [F64;128],
	fVec42: [F64;256],
	fVec43: [F64;512],
	fVec44: [F64;1024],
	fVec45: [F64;2048],
	fVec46: [F64;4096],
	fVec47: [F64;8192],
	fVec48: [F64;16384],
	fRec17: [F64;2],
	fVec49: [F64;3],
	fVec50: [F64;7],
	fVec51: [F64;15],
	fVec52: [F64;32],
	fVec53: [F64;64],
	fVec54: [F64;128],
	fVec55: [F64;256],
	fVec56: [F64;512],
	fVec57: [F64;1024],
	fVec58: [F64;2048],
	fVec59: [F64;4096],
	fVec60: [F64;8192],
	fVec61: [F64;16384],
	fVec62: [F64;2],
	fVec63: [F64;2],
	fVec64: [F64;2],
	fRec15: [F64;2],
	fRec16: [F64;32768],
}

impl FaustDsp for LambRs192k {
	type T = F64;
		
	fn new() -> LambRs192k { 
		LambRs192k {
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
			fVec17: [0.0;8192],
			fVec18: [0.0;16384],
			fRec3: [0.0;2],
			fVec19: [0.0;3],
			fVec20: [0.0;7],
			fVec21: [0.0;15],
			fVec22: [0.0;32],
			fVec23: [0.0;64],
			fVec24: [0.0;128],
			fVec25: [0.0;256],
			fVec26: [0.0;512],
			fVec27: [0.0;1024],
			fVec28: [0.0;2048],
			fVec29: [0.0;4096],
			fVec30: [0.0;8192],
			fVec31: [0.0;16384],
			fVec32: [0.0;2],
			fHslider10: 0.0,
			fHslider11: 0.0,
			fVec33: [0.0;2],
			fVec34: [0.0;2],
			fConst10: 0.0,
			fRec1: [0.0;2],
			fRec2: [0.0;32768],
			fCheckbox1: 0.0,
			fHbargraph0: 0.0,
			fHslider12: 0.0,
			fRec14: [0.0;2],
			fVec35: [0.0;16384],
			fVec36: [0.0;3],
			fVec37: [0.0;7],
			fVec38: [0.0;15],
			fVec39: [0.0;32],
			fVec40: [0.0;64],
			fVec41: [0.0;128],
			fVec42: [0.0;256],
			fVec43: [0.0;512],
			fVec44: [0.0;1024],
			fVec45: [0.0;2048],
			fVec46: [0.0;4096],
			fVec47: [0.0;8192],
			fVec48: [0.0;16384],
			fRec17: [0.0;2],
			fVec49: [0.0;3],
			fVec50: [0.0;7],
			fVec51: [0.0;15],
			fVec52: [0.0;32],
			fVec53: [0.0;64],
			fVec54: [0.0;128],
			fVec55: [0.0;256],
			fVec56: [0.0;512],
			fVec57: [0.0;1024],
			fVec58: [0.0;2048],
			fVec59: [0.0;4096],
			fVec60: [0.0;8192],
			fVec61: [0.0;16384],
			fVec62: [0.0;2],
			fVec63: [0.0;2],
			fVec64: [0.0;2],
			fRec15: [0.0;2],
			fRec16: [0.0;32768],
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
		m.declare("compile_options", r"-a /run/user/1001/.tmpJ2AMKD -lang rust -ct 1 -cn LambRs192k -es 1 -mcd 16 -mdd 1024 -mdy 33 -double -ftz 0");
		m.declare("filename", r"lamb-rs-192k.dsp");
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
		let mut sig0: LambRs192kSIG0 = newLambRs192kSIG0();
		sig0.instance_initLambRs192kSIG0(sample_rate);
		sig0.fillLambRs192kSIG0(786432, unsafe { &mut ftbl0LambRs192kSIG0 });
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
		for l48 in 0..32768 {
			self.fRec2[l48 as usize] = 0.0;
		}
		for l49 in 0..2 {
			self.fRec14[l49 as usize] = 0.0;
		}
		for l50 in 0..16384 {
			self.fVec35[l50 as usize] = 0.0;
		}
		for l51 in 0..3 {
			self.fVec36[l51 as usize] = 0.0;
		}
		for l52 in 0..7 {
			self.fVec37[l52 as usize] = 0.0;
		}
		for l53 in 0..15 {
			self.fVec38[l53 as usize] = 0.0;
		}
		for l54 in 0..32 {
			self.fVec39[l54 as usize] = 0.0;
		}
		for l55 in 0..64 {
			self.fVec40[l55 as usize] = 0.0;
		}
		for l56 in 0..128 {
			self.fVec41[l56 as usize] = 0.0;
		}
		for l57 in 0..256 {
			self.fVec42[l57 as usize] = 0.0;
		}
		for l58 in 0..512 {
			self.fVec43[l58 as usize] = 0.0;
		}
		for l59 in 0..1024 {
			self.fVec44[l59 as usize] = 0.0;
		}
		for l60 in 0..2048 {
			self.fVec45[l60 as usize] = 0.0;
		}
		for l61 in 0..4096 {
			self.fVec46[l61 as usize] = 0.0;
		}
		for l62 in 0..8192 {
			self.fVec47[l62 as usize] = 0.0;
		}
		for l63 in 0..16384 {
			self.fVec48[l63 as usize] = 0.0;
		}
		for l64 in 0..2 {
			self.fRec17[l64 as usize] = 0.0;
		}
		for l65 in 0..3 {
			self.fVec49[l65 as usize] = 0.0;
		}
		for l66 in 0..7 {
			self.fVec50[l66 as usize] = 0.0;
		}
		for l67 in 0..15 {
			self.fVec51[l67 as usize] = 0.0;
		}
		for l68 in 0..32 {
			self.fVec52[l68 as usize] = 0.0;
		}
		for l69 in 0..64 {
			self.fVec53[l69 as usize] = 0.0;
		}
		for l70 in 0..128 {
			self.fVec54[l70 as usize] = 0.0;
		}
		for l71 in 0..256 {
			self.fVec55[l71 as usize] = 0.0;
		}
		for l72 in 0..512 {
			self.fVec56[l72 as usize] = 0.0;
		}
		for l73 in 0..1024 {
			self.fVec57[l73 as usize] = 0.0;
		}
		for l74 in 0..2048 {
			self.fVec58[l74 as usize] = 0.0;
		}
		for l75 in 0..4096 {
			self.fVec59[l75 as usize] = 0.0;
		}
		for l76 in 0..8192 {
			self.fVec60[l76 as usize] = 0.0;
		}
		for l77 in 0..16384 {
			self.fVec61[l77 as usize] = 0.0;
		}
		for l78 in 0..2 {
			self.fVec62[l78 as usize] = 0.0;
		}
		for l79 in 0..2 {
			self.fVec63[l79 as usize] = 0.0;
		}
		for l80 in 0..2 {
			self.fVec64[l80 as usize] = 0.0;
		}
		for l81 in 0..2 {
			self.fRec15[l81 as usize] = 0.0;
		}
		for l82 in 0..32768 {
			self.fRec16[l82 as usize] = 0.0;
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
		ui_interface.add_horizontal_slider("release hold", ParamIndex(9), 5e+01, 0.005208333333333333, 5e+01, 1.0);
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
		let mut fSlow16: F64 = 0.002 * fSlow15;
		let mut fSlow17: F64 = 0.01 * self.fHslider8;
		let mut fSlow18: F64 = 0.0005 * fSlow15;
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
		self.fHbargraph0 = if (fSlow77) as i32 != 0 {1.92e+04} else {fSlow76};
		let mut iSlow79: i32 = (self.fHbargraph0) as i32;
		let mut fSlow80: F64 = self.fConst3 * F64::powf(1e+01, 0.05 * self.fHslider12);
		let zipped_iterators = inputs0.zip(inputs1).zip(outputs0).zip(outputs1).zip(outputs2).zip(outputs3);
		for (((((input0, input1), output0), output1), output2), output3) in zipped_iterators {
			self.iVec0[(self.IOTA0 & 32767) as usize] = 1;
			let mut fTemp0: F64 = self.fConst1 + self.fRec0[1];
			let mut fTemp1: F64 = self.fRec0[1] - self.fConst1;
			self.fRec0[0] = if (fTemp0 < fSlow0) as i32 != 0 {fTemp0} else {if (fTemp1 > fSlow0) as i32 != 0 {fTemp1} else {fSlow0}};
			let mut fTemp2: F64 = F64::sin(6.283185307179586 * (0.5 * self.fRec0[0] + 0.75)) + 1.0;
			let mut fTemp3: F64 = 0.5 * fTemp2;
			let mut fTemp4: F64 = 1.0 - fTemp3;
			let mut fTemp5: F64 = self.fRec2[((i32::wrapping_sub(self.IOTA0, 1)) & 32767) as usize];
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
			let mut fTemp22: F64 = F64::powf(1e+01, -(0.05 * F64::max(0.0, if (iTemp20 == 0) as i32 != 0 {0.0} else {if (iTemp20 == 1) as i32 != 0 {fSlow12 * LambRs192k_faustpower2_f(fSlow7 + fTemp21)} else {fTemp21}})));
			let mut fTemp23: F64 = 3.0 * fTemp6;
			let mut fTemp24: F64 = 4.0 * (F64::max(0.25, self.fRec4[0]) + -0.25);
			let mut fTemp25: F64 = 9.0 - fTemp24;
			let mut fTemp26: F64 = self.fRec5[1] - self.fRec6[1];
			let mut iTemp27: i32 = self.iVec0[((i32::wrapping_sub(self.IOTA0, 19200)) & 32767) as usize];
			let mut fTemp28: F64 = (iTemp27) as F64;
			let mut fTemp29: F64 = if (fTemp22 > self.fRec10[1]) as i32 != 0 {F64::exp(-(self.fConst7 / F64::max(2.220446049250313e-16, fSlow14 * (fTemp28 / F64::max(1.0 - (F64::max(fTemp24 + -9.0, F64::min(2.0 - fTemp23, fTemp26)) + fTemp25) / (11.0 - (fTemp24 + fTemp23)), 2.220446049250313e-16)))))} else {self.fConst6};
			self.fRec10[0] = self.fRec10[1] * fTemp29 + fTemp22 * (1.0 - fTemp29);
			let mut fTemp30: F64 = if (self.fRec10[0] > self.fRec9[1]) as i32 != 0 {0.0} else {self.fConst6};
			self.fRec9[0] = self.fRec9[1] * fTemp30 + self.fRec10[0] * (1.0 - fTemp30);
			let mut fTemp31: F64 = if (self.fRec9[0] > self.fRec8[1]) as i32 != 0 {0.0} else {self.fConst6};
			self.fRec8[0] = self.fRec8[1] * fTemp31 + self.fRec9[0] * (1.0 - fTemp31);
			let mut fTemp32: F64 = if (self.fRec8[0] > self.fRec7[1]) as i32 != 0 {0.0} else {self.fConst6};
			self.fRec7[0] = self.fRec7[1] * fTemp32 + self.fRec8[0] * (1.0 - fTemp32);
			self.fRec5[0] = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, self.fRec7[0]));
			let mut fTemp33: F64 = F64::powf(1e+01, 0.05 * (self.fRec5[1] + fTemp25));
			let mut fTemp34: F64 = if (fTemp33 > self.fRec12[1]) as i32 != 0 {F64::exp(-(self.fConst7 / F64::max(2.220446049250313e-16, fTemp28 * (0.8161290322580644 * (F64::max(0.69, self.fRec4[0]) + -0.69) + 0.05) * F64::powf(4503599627370496.0, 1.0 - (F64::max(fTemp11, F64::min(fTemp12, fTemp26)) + fTemp10) / fTemp9))))} else {self.fConst8};
			self.fRec12[0] = self.fRec12[1] * fTemp34 + fTemp33 * (1.0 - fTemp34);
			self.fRec6[0] = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, self.fRec12[0]));
			let mut fTemp35: F64 = self.fRec5[0] - self.fRec6[0];
			let mut fTemp36: F64 = F64::powf(1e+01, fSlow16 * F64::min(0.25, self.fRec4[0]) * (self.fRec6[0] + fTemp35 * (F64::max(fTemp11, F64::min(fTemp12, fTemp35)) + fTemp10) / fTemp9));
			let mut fTemp37: F64 = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, fTemp15));
			let mut fTemp38: F64 = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, fTemp18));
			let mut fTemp39: F64 = F64::max(fTemp37, fTemp38);
			let mut fTemp40: F64 = fTemp37 + fSlow17 * (fTemp39 - fTemp37);
			let mut iTemp41: i32 = ((fTemp40 > fSlow11) as i32) + ((fTemp40 > fSlow9) as i32);
			let mut fTemp42: F64 = fTemp40 - fSlow8;
			let mut fTemp43: F64 = F64::min(fTemp36, F64::powf(1e+01, -(fSlow18 * F64::max(0.0, if (iTemp41 == 0) as i32 != 0 {0.0} else {if (iTemp41 == 1) as i32 != 0 {fSlow12 * LambRs192k_faustpower2_f(fSlow7 + fTemp42)} else {fTemp42}}))));
			self.fVec5[(self.IOTA0 & 16383) as usize] = fTemp43;
			let mut fTemp44: F64 = F64::min(fTemp43, self.fVec5[((i32::wrapping_sub(self.IOTA0, 1)) & 16383) as usize]);
			self.fVec6[0] = fTemp44;
			let mut fTemp45: F64 = F64::min(fTemp44, self.fVec6[2]);
			self.fVec7[0] = fTemp45;
			let mut fTemp46: F64 = F64::min(fTemp45, self.fVec7[4]);
			self.fVec8[0] = fTemp46;
			let mut fTemp47: F64 = F64::min(fTemp46, self.fVec8[8]);
			self.fVec9[(self.IOTA0 & 31) as usize] = fTemp47;
			let mut fTemp48: F64 = F64::min(fTemp47, self.fVec9[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize]);
			self.fVec10[(self.IOTA0 & 63) as usize] = fTemp48;
			let mut fTemp49: F64 = F64::min(fTemp48, self.fVec10[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize]);
			self.fVec11[(self.IOTA0 & 127) as usize] = fTemp49;
			let mut fTemp50: F64 = F64::min(fTemp49, self.fVec11[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize]);
			self.fVec12[(self.IOTA0 & 255) as usize] = fTemp50;
			let mut fTemp51: F64 = F64::min(fTemp50, self.fVec12[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize]);
			self.fVec13[(self.IOTA0 & 511) as usize] = fTemp51;
			let mut fTemp52: F64 = F64::min(fTemp51, self.fVec13[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize]);
			self.fVec14[(self.IOTA0 & 1023) as usize] = fTemp52;
			let mut fTemp53: F64 = F64::min(fTemp52, self.fVec14[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize]);
			self.fVec15[(self.IOTA0 & 2047) as usize] = fTemp53;
			let mut fTemp54: F64 = F64::min(fTemp53, self.fVec15[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			self.fVec16[(self.IOTA0 & 4095) as usize] = fTemp54;
			let mut fTemp55: F64 = F64::min(fTemp54, self.fVec16[((i32::wrapping_sub(self.IOTA0, 2048)) & 4095) as usize]);
			self.fVec17[(self.IOTA0 & 8191) as usize] = fTemp55;
			self.fVec18[(self.IOTA0 & 16383) as usize] = F64::min(fTemp55, self.fVec17[((i32::wrapping_sub(self.IOTA0, 4096)) & 8191) as usize]);
			self.fRec3[0] = F64::max(F64::min(self.fRec3[1], self.fVec5[((i32::wrapping_sub(self.IOTA0, iSlow21)) & 16383) as usize]), F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow23 != 0 {fTemp43} else {1.7976931348623157e+308}, if iSlow24 != 0 {self.fVec6[iSlow23 as usize]} else {1.7976931348623157e+308}), if iSlow25 != 0 {self.fVec7[iSlow26 as usize]} else {1.7976931348623157e+308}), if iSlow27 != 0 {self.fVec8[iSlow28 as usize]} else {1.7976931348623157e+308}), if iSlow29 != 0 {self.fVec9[((i32::wrapping_sub(self.IOTA0, iSlow30)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow31 != 0 {self.fVec10[((i32::wrapping_sub(self.IOTA0, iSlow32)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow33 != 0 {self.fVec11[((i32::wrapping_sub(self.IOTA0, iSlow34)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow35 != 0 {self.fVec12[((i32::wrapping_sub(self.IOTA0, iSlow36)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow37 != 0 {self.fVec13[((i32::wrapping_sub(self.IOTA0, iSlow38)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow39 != 0 {self.fVec14[((i32::wrapping_sub(self.IOTA0, iSlow40)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow41 != 0 {self.fVec15[((i32::wrapping_sub(self.IOTA0, iSlow42)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow43 != 0 {self.fVec16[((i32::wrapping_sub(self.IOTA0, iSlow44)) & 4095) as usize]} else {1.7976931348623157e+308}), if iSlow45 != 0 {self.fVec17[((i32::wrapping_sub(self.IOTA0, iSlow46)) & 8191) as usize]} else {1.7976931348623157e+308}), if iSlow47 != 0 {self.fVec18[((i32::wrapping_sub(self.IOTA0, iSlow48)) & 16383) as usize]} else {1.7976931348623157e+308}));
			let mut fTemp56: F64 = F64::min(self.fRec3[0], self.fRec3[1]);
			self.fVec19[0] = fTemp56;
			let mut fTemp57: F64 = F64::min(fTemp56, self.fVec19[2]);
			self.fVec20[0] = fTemp57;
			let mut fTemp58: F64 = F64::min(fTemp57, self.fVec20[4]);
			self.fVec21[0] = fTemp58;
			let mut fTemp59: F64 = F64::min(fTemp58, self.fVec21[8]);
			self.fVec22[(self.IOTA0 & 31) as usize] = fTemp59;
			let mut fTemp60: F64 = F64::min(fTemp59, self.fVec22[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize]);
			self.fVec23[(self.IOTA0 & 63) as usize] = fTemp60;
			let mut fTemp61: F64 = F64::min(fTemp60, self.fVec23[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize]);
			self.fVec24[(self.IOTA0 & 127) as usize] = fTemp61;
			let mut fTemp62: F64 = F64::min(fTemp61, self.fVec24[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize]);
			self.fVec25[(self.IOTA0 & 255) as usize] = fTemp62;
			let mut fTemp63: F64 = F64::min(fTemp62, self.fVec25[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize]);
			self.fVec26[(self.IOTA0 & 511) as usize] = fTemp63;
			let mut fTemp64: F64 = F64::min(fTemp63, self.fVec26[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize]);
			self.fVec27[(self.IOTA0 & 1023) as usize] = fTemp64;
			let mut fTemp65: F64 = F64::min(fTemp64, self.fVec27[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize]);
			self.fVec28[(self.IOTA0 & 2047) as usize] = fTemp65;
			let mut fTemp66: F64 = F64::min(fTemp65, self.fVec28[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			self.fVec29[(self.IOTA0 & 4095) as usize] = fTemp66;
			let mut fTemp67: F64 = F64::min(fTemp66, self.fVec29[((i32::wrapping_sub(self.IOTA0, 2048)) & 4095) as usize]);
			self.fVec30[(self.IOTA0 & 8191) as usize] = fTemp67;
			self.fVec31[(self.IOTA0 & 16383) as usize] = F64::min(fTemp67, self.fVec30[((i32::wrapping_sub(self.IOTA0, 4096)) & 8191) as usize]);
			let mut fTemp68: F64 = F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow4 != 0 {self.fRec3[0]} else {1.7976931348623157e+308}, if iSlow49 != 0 {self.fVec19[iSlow4 as usize]} else {1.7976931348623157e+308}), if iSlow50 != 0 {self.fVec20[iSlow51 as usize]} else {1.7976931348623157e+308}), if iSlow52 != 0 {self.fVec21[iSlow53 as usize]} else {1.7976931348623157e+308}), if iSlow54 != 0 {self.fVec22[((i32::wrapping_sub(self.IOTA0, iSlow55)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow56 != 0 {self.fVec23[((i32::wrapping_sub(self.IOTA0, iSlow57)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow58 != 0 {self.fVec24[((i32::wrapping_sub(self.IOTA0, iSlow59)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow60 != 0 {self.fVec25[((i32::wrapping_sub(self.IOTA0, iSlow61)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow62 != 0 {self.fVec26[((i32::wrapping_sub(self.IOTA0, iSlow63)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow64 != 0 {self.fVec27[((i32::wrapping_sub(self.IOTA0, iSlow65)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow66 != 0 {self.fVec28[((i32::wrapping_sub(self.IOTA0, iSlow67)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow68 != 0 {self.fVec29[((i32::wrapping_sub(self.IOTA0, iSlow69)) & 4095) as usize]} else {1.7976931348623157e+308}), if iSlow70 != 0 {self.fVec30[((i32::wrapping_sub(self.IOTA0, iSlow71)) & 8191) as usize]} else {1.7976931348623157e+308}), if iSlow72 != 0 {self.fVec31[((i32::wrapping_sub(self.IOTA0, iSlow73)) & 16383) as usize]} else {1.7976931348623157e+308}) - fTemp5;
			self.fVec32[0] = fTemp68;
			let mut iTemp69: i32 = (fTemp68 > 0.0) as i32;
			let mut fTemp75: F64 = if iTemp69 != 0 {fSlow75} else {fSlow74};
			self.fVec33[0] = fTemp75;
			let mut fTemp76: F64 = 2.0 * fTemp75;
			let mut iTemp77: i32 = (fTemp76) as i32;
			let mut iTemp78: i32 = std::cmp::max(0, std::cmp::min(iTemp77, 2));
			let mut iTemp79: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, 393213), 786431));
			let mut fTemp80: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp79, 3)) as usize] };
			let mut fTemp81: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp79 as usize] };
			let mut fTemp82: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp79, 1)) as usize] } - fTemp81;
			let mut fTemp83: F64 = fTemp76 - (iTemp77) as F64;
			let mut fTemp84: F64 = fTemp81 + fTemp83 * fTemp82 + 0.5 * (fTemp80 - (fTemp81 + fTemp83 * (fTemp82 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp79, 4)) as usize] } - fTemp80))));
			let mut fTemp85: F64 = if iTemp69 != 0 {fTemp84} else {1.0 - fTemp84};
			let mut iTemp86: i32 = (fTemp68 < 0.0) as i32;
			let mut fTemp87: F64 = fSlow1 * (iTemp86) as F64 + fSlow13 * (iTemp69) as F64;
			self.fVec34[0] = fTemp87;
			let mut fTemp88: F64 = self.fConst10 / fTemp87;
			let mut fTemp89: F64 = fTemp88 + 0.5;
			let mut fTemp90: F64 = 262143.0 * (1.0 - fTemp89);
			let mut iTemp91: i32 = (fTemp90) as i32;
			let mut iTemp92: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp91, 262143)))), 786431));
			let mut fTemp93: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp92, 3)) as usize] };
			let mut fTemp94: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp92 as usize] };
			let mut fTemp95: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp92, 1)) as usize] } - fTemp94;
			let mut fTemp96: F64 = 262143.0 * fTemp89;
			let mut iTemp97: i32 = (fTemp96) as i32;
			let mut iTemp98: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp97, 262143)))), 786431));
			let mut fTemp99: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp98, 3), 786431))) as usize] };
			let mut fTemp100: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp98 as usize] };
			let mut fTemp101: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp98, 1), 786431))) as usize] } - fTemp100;
			let mut fTemp102: F64 = 2.0 * self.fVec33[1];
			let mut iTemp103: i32 = (fTemp102) as i32;
			let mut iTemp104: i32 = std::cmp::max(0, std::cmp::min(iTemp103, 2));
			let mut fTemp105: F64 = 262143.0 * (1.0 - self.fRec1[1]);
			let mut iTemp106: i32 = (fTemp105) as i32;
			let mut iTemp107: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp106, 262143))), iTemp104), 786431));
			let mut fTemp108: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp107, 3), 786431))) as usize] };
			let mut fTemp109: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp107 as usize] };
			let mut fTemp110: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp107, 1), 786431))) as usize] } - fTemp109;
			let mut fTemp111: F64 = fTemp102 - (iTemp103) as F64;
			let mut fTemp112: F64 = 262143.0 * self.fRec1[1];
			let mut iTemp113: i32 = (fTemp112) as i32;
			let mut iTemp114: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp104, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp113, 262143)))), 786431));
			let mut fTemp115: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp114, 3), 786431))) as usize] };
			let mut fTemp116: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp114 as usize] };
			let mut fTemp117: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp114, 1), 786431))) as usize] } - fTemp116;
			let mut fTemp118: F64 = self.fRec1[1] + fTemp88;
			let mut fTemp119: F64 = 262143.0 * (1.0 - fTemp118);
			let mut iTemp120: i32 = (fTemp119) as i32;
			let mut iTemp121: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp120, 262143)))), 786431));
			let mut fTemp122: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp121, 3)) as usize] };
			let mut fTemp123: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp121 as usize] };
			let mut fTemp124: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp121, 1)) as usize] } - fTemp123;
			let mut fTemp125: F64 = 262143.0 * fTemp118;
			let mut iTemp126: i32 = (fTemp125) as i32;
			let mut iTemp127: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp126, 262143)))), 786431));
			let mut fTemp128: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp127, 3), 786431))) as usize] };
			let mut fTemp129: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp127 as usize] };
			let mut fTemp130: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp127, 1), 786431))) as usize] } - fTemp129;
			let mut fTemp131: F64 = self.fRec1[1] + self.fConst10 * (1.0 / fTemp87 + 1.0 / self.fVec34[1]);
			let mut fTemp132: F64 = 262143.0 * (1.0 - fTemp131);
			let mut iTemp133: i32 = (fTemp132) as i32;
			let mut iTemp134: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp133, 262143))), iTemp78), 786431));
			let mut fTemp135: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp134, 3)) as usize] };
			let mut fTemp136: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp134 as usize] };
			let mut fTemp137: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp134, 1)) as usize] } - fTemp136;
			let mut fTemp138: F64 = 262143.0 * fTemp131;
			let mut iTemp139: i32 = (fTemp138) as i32;
			let mut iTemp140: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp139, 262143)))), 786431));
			let mut fTemp141: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp140, 3), 786431))) as usize] };
			let mut fTemp142: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp140 as usize] };
			let mut fTemp143: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp140, 1), 786431))) as usize] } - fTemp142;
			let mut fTemp144: F64 = (if iTemp69 != 0 {fTemp142 + fTemp83 * fTemp143 + (fTemp138 - (iTemp139) as F64) * (fTemp141 - (fTemp142 + fTemp83 * (fTemp143 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp140, 4), 786431))) as usize] } - fTemp141))))} else {1.0 - (fTemp136 + fTemp83 * fTemp137 + (fTemp132 - (iTemp133) as F64) * (fTemp135 - (fTemp136 + fTemp83 * (fTemp137 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp134, 4)) as usize] } - fTemp135)))))} - if iTemp69 != 0 {fTemp129 + fTemp83 * fTemp130 + (fTemp125 - (iTemp126) as F64) * (fTemp128 - (fTemp129 + fTemp83 * (fTemp130 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp127, 4), 786431))) as usize] } - fTemp128))))} else {1.0 - (fTemp123 + fTemp83 * fTemp124 + (fTemp119 - (iTemp120) as F64) * (fTemp122 - (fTemp123 + fTemp83 * (fTemp124 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp121, 4)) as usize] } - fTemp122)))))}) * self.fVec32[1] / (fTemp68 * (1.0 - if iTemp69 != 0 {fTemp116 + fTemp111 * fTemp117 + (fTemp112 - (iTemp113) as F64) * (fTemp115 - (fTemp116 + fTemp111 * (fTemp117 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp114, 4), 786431))) as usize] } - fTemp115))))} else {1.0 - (fTemp109 + fTemp111 * fTemp110 + (fTemp105 - (iTemp106) as F64) * (fTemp108 - (fTemp109 + fTemp111 * (fTemp110 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp107, 4), 786431))) as usize] } - fTemp108)))))}));
			let mut iTemp145: i32 = (fTemp144 > ((if iTemp69 != 0 {fTemp100 + fTemp83 * fTemp101 + (fTemp96 - (iTemp97) as F64) * (fTemp99 - (fTemp100 + fTemp83 * (fTemp101 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp98, 4), 786431))) as usize] } - fTemp99))))} else {1.0 - (fTemp94 + fTemp83 * fTemp95 + (fTemp90 - (iTemp91) as F64) * (fTemp93 - (fTemp94 + fTemp83 * (fTemp95 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp92, 4)) as usize] } - fTemp93)))))} - fTemp85) / (1.0 - fTemp85))) as i32;
			let mut fTemp146: F64 = if iTemp145 != 0 {1.0} else {0.5};
			let mut fTemp147: F64 = if iTemp145 != 0 {0.5} else {0.0};
			let mut fTemp148: F64 = fTemp147 + fTemp146;
			let mut fTemp149: F64 = 0.5 * fTemp148;
			let mut fTemp150: F64 = 262143.0 * (1.0 - fTemp149);
			let mut iTemp151: i32 = (fTemp150) as i32;
			let mut iTemp152: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp151, 262143)))), 786431));
			let mut fTemp153: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp152, 3)) as usize] };
			let mut fTemp154: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp152 as usize] };
			let mut fTemp155: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp152, 1)) as usize] } - fTemp154;
			let mut fTemp156: F64 = 131071.5 * fTemp148;
			let mut iTemp157: i32 = (fTemp156) as i32;
			let mut iTemp158: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp157, 262143)))), 786431));
			let mut fTemp159: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp158, 3)) as usize] };
			let mut fTemp160: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp158 as usize] };
			let mut fTemp161: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp158, 1)) as usize] } - fTemp160;
			let mut fTemp162: F64 = if iTemp69 != 0 {fTemp160 + fTemp83 * fTemp161 + (fTemp156 - (iTemp157) as F64) * (fTemp159 - (fTemp160 + fTemp83 * (fTemp161 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp158, 4)) as usize] } - fTemp159))))} else {1.0 - (fTemp154 + fTemp83 * fTemp155 + (fTemp150 - (iTemp151) as F64) * (fTemp153 - (fTemp154 + fTemp83 * (fTemp155 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp152, 4)) as usize] } - fTemp153)))))};
			let mut fTemp163: F64 = fTemp88 + fTemp149;
			let mut fTemp164: F64 = 262143.0 * (1.0 - fTemp163);
			let mut iTemp165: i32 = (fTemp164) as i32;
			let mut iTemp166: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp165, 262143)))), 786431));
			let mut fTemp167: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp166, 3)) as usize] };
			let mut fTemp168: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp166 as usize] };
			let mut fTemp169: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp166, 1)) as usize] } - fTemp168;
			let mut fTemp170: F64 = 262143.0 * fTemp163;
			let mut iTemp171: i32 = (fTemp170) as i32;
			let mut iTemp172: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp171, 262143)))), 786431));
			let mut fTemp173: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp172, 3), 786431))) as usize] };
			let mut fTemp174: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp172 as usize] };
			let mut fTemp175: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp172, 1), 786431))) as usize] } - fTemp174;
			let mut iTemp176: i32 = (fTemp144 > ((if iTemp69 != 0 {fTemp174 + fTemp83 * fTemp175 + (fTemp170 - (iTemp171) as F64) * (fTemp173 - (fTemp174 + fTemp83 * (fTemp175 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp172, 4), 786431))) as usize] } - fTemp173))))} else {1.0 - (fTemp168 + fTemp83 * fTemp169 + (fTemp164 - (iTemp165) as F64) * (fTemp167 - (fTemp168 + fTemp83 * (fTemp169 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp166, 4)) as usize] } - fTemp167)))))} - fTemp162) / (1.0 - fTemp162))) as i32;
			let mut fTemp177: F64 = if iTemp176 != 0 {fTemp146} else {fTemp149};
			let mut fTemp178: F64 = if iTemp176 != 0 {fTemp149} else {fTemp147};
			let mut fTemp179: F64 = fTemp178 + fTemp177;
			let mut fTemp180: F64 = 0.5 * fTemp179;
			let mut fTemp181: F64 = 262143.0 * (1.0 - fTemp180);
			let mut iTemp182: i32 = (fTemp181) as i32;
			let mut iTemp183: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp182, 262143)))), 786431));
			let mut fTemp184: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp183, 3)) as usize] };
			let mut fTemp185: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp183 as usize] };
			let mut fTemp186: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp183, 1)) as usize] } - fTemp185;
			let mut fTemp187: F64 = 131071.5 * fTemp179;
			let mut iTemp188: i32 = (fTemp187) as i32;
			let mut iTemp189: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp188, 262143)))), 786431));
			let mut fTemp190: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp189, 3)) as usize] };
			let mut fTemp191: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp189 as usize] };
			let mut fTemp192: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp189, 1)) as usize] } - fTemp191;
			let mut fTemp193: F64 = if iTemp69 != 0 {fTemp191 + fTemp83 * fTemp192 + (fTemp187 - (iTemp188) as F64) * (fTemp190 - (fTemp191 + fTemp83 * (fTemp192 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp189, 4)) as usize] } - fTemp190))))} else {1.0 - (fTemp185 + fTemp83 * fTemp186 + (fTemp181 - (iTemp182) as F64) * (fTemp184 - (fTemp185 + fTemp83 * (fTemp186 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp183, 4)) as usize] } - fTemp184)))))};
			let mut fTemp194: F64 = fTemp88 + fTemp180;
			let mut fTemp195: F64 = 262143.0 * (1.0 - fTemp194);
			let mut iTemp196: i32 = (fTemp195) as i32;
			let mut iTemp197: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp196, 262143)))), 786431));
			let mut fTemp198: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp197, 3)) as usize] };
			let mut fTemp199: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp197 as usize] };
			let mut fTemp200: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp197, 1)) as usize] } - fTemp199;
			let mut fTemp201: F64 = 262143.0 * fTemp194;
			let mut iTemp202: i32 = (fTemp201) as i32;
			let mut iTemp203: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp202, 262143)))), 786431));
			let mut fTemp204: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp203, 3), 786431))) as usize] };
			let mut fTemp205: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp203 as usize] };
			let mut fTemp206: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp203, 1), 786431))) as usize] } - fTemp205;
			let mut iTemp207: i32 = (fTemp144 > ((if iTemp69 != 0 {fTemp205 + fTemp83 * fTemp206 + (fTemp201 - (iTemp202) as F64) * (fTemp204 - (fTemp205 + fTemp83 * (fTemp206 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp203, 4), 786431))) as usize] } - fTemp204))))} else {1.0 - (fTemp199 + fTemp83 * fTemp200 + (fTemp195 - (iTemp196) as F64) * (fTemp198 - (fTemp199 + fTemp83 * (fTemp200 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp197, 4)) as usize] } - fTemp198)))))} - fTemp193) / (1.0 - fTemp193))) as i32;
			let mut fTemp208: F64 = if iTemp207 != 0 {fTemp177} else {fTemp180};
			let mut fTemp209: F64 = if iTemp207 != 0 {fTemp180} else {fTemp178};
			let mut fTemp210: F64 = fTemp209 + fTemp208;
			let mut fTemp211: F64 = 0.5 * fTemp210;
			let mut fTemp212: F64 = 262143.0 * (1.0 - fTemp211);
			let mut iTemp213: i32 = (fTemp212) as i32;
			let mut iTemp214: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp213, 262143)))), 786431));
			let mut fTemp215: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp214, 3)) as usize] };
			let mut fTemp216: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp214 as usize] };
			let mut fTemp217: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp214, 1)) as usize] } - fTemp216;
			let mut fTemp218: F64 = 131071.5 * fTemp210;
			let mut iTemp219: i32 = (fTemp218) as i32;
			let mut iTemp220: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp219, 262143)))), 786431));
			let mut fTemp221: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp220, 3)) as usize] };
			let mut fTemp222: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp220 as usize] };
			let mut fTemp223: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp220, 1)) as usize] } - fTemp222;
			let mut fTemp224: F64 = if iTemp69 != 0 {fTemp222 + fTemp83 * fTemp223 + (fTemp218 - (iTemp219) as F64) * (fTemp221 - (fTemp222 + fTemp83 * (fTemp223 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp220, 4)) as usize] } - fTemp221))))} else {1.0 - (fTemp216 + fTemp83 * fTemp217 + (fTemp212 - (iTemp213) as F64) * (fTemp215 - (fTemp216 + fTemp83 * (fTemp217 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp214, 4)) as usize] } - fTemp215)))))};
			let mut fTemp225: F64 = fTemp88 + fTemp211;
			let mut fTemp226: F64 = 262143.0 * (1.0 - fTemp225);
			let mut iTemp227: i32 = (fTemp226) as i32;
			let mut iTemp228: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp227, 262143)))), 786431));
			let mut fTemp229: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp228, 3)) as usize] };
			let mut fTemp230: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp228 as usize] };
			let mut fTemp231: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp228, 1)) as usize] } - fTemp230;
			let mut fTemp232: F64 = 262143.0 * fTemp225;
			let mut iTemp233: i32 = (fTemp232) as i32;
			let mut iTemp234: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp233, 262143)))), 786431));
			let mut fTemp235: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp234, 3), 786431))) as usize] };
			let mut fTemp236: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp234 as usize] };
			let mut fTemp237: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp234, 1), 786431))) as usize] } - fTemp236;
			let mut iTemp238: i32 = (fTemp144 > ((if iTemp69 != 0 {fTemp236 + fTemp83 * fTemp237 + (fTemp232 - (iTemp233) as F64) * (fTemp235 - (fTemp236 + fTemp83 * (fTemp237 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp234, 4), 786431))) as usize] } - fTemp235))))} else {1.0 - (fTemp230 + fTemp83 * fTemp231 + (fTemp226 - (iTemp227) as F64) * (fTemp229 - (fTemp230 + fTemp83 * (fTemp231 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp228, 4)) as usize] } - fTemp229)))))} - fTemp224) / (1.0 - fTemp224))) as i32;
			let mut fTemp239: F64 = if iTemp238 != 0 {fTemp208} else {fTemp211};
			let mut fTemp240: F64 = if iTemp238 != 0 {fTemp211} else {fTemp209};
			let mut fTemp241: F64 = fTemp240 + fTemp239;
			let mut fTemp242: F64 = 0.5 * fTemp241;
			let mut fTemp243: F64 = 262143.0 * (1.0 - fTemp242);
			let mut iTemp244: i32 = (fTemp243) as i32;
			let mut iTemp245: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp244, 262143)))), 786431));
			let mut fTemp246: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp245, 3)) as usize] };
			let mut fTemp247: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp245 as usize] };
			let mut fTemp248: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp245, 1)) as usize] } - fTemp247;
			let mut fTemp249: F64 = 131071.5 * fTemp241;
			let mut iTemp250: i32 = (fTemp249) as i32;
			let mut iTemp251: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp250, 262143)))), 786431));
			let mut fTemp252: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp251, 3)) as usize] };
			let mut fTemp253: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp251 as usize] };
			let mut fTemp254: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp251, 1)) as usize] } - fTemp253;
			let mut fTemp255: F64 = if iTemp69 != 0 {fTemp253 + fTemp83 * fTemp254 + (fTemp249 - (iTemp250) as F64) * (fTemp252 - (fTemp253 + fTemp83 * (fTemp254 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp251, 4)) as usize] } - fTemp252))))} else {1.0 - (fTemp247 + fTemp83 * fTemp248 + (fTemp243 - (iTemp244) as F64) * (fTemp246 - (fTemp247 + fTemp83 * (fTemp248 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp245, 4)) as usize] } - fTemp246)))))};
			let mut fTemp256: F64 = fTemp88 + fTemp242;
			let mut fTemp257: F64 = 262143.0 * (1.0 - fTemp256);
			let mut iTemp258: i32 = (fTemp257) as i32;
			let mut iTemp259: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp258, 262143)))), 786431));
			let mut fTemp260: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp259, 3)) as usize] };
			let mut fTemp261: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp259 as usize] };
			let mut fTemp262: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp259, 1)) as usize] } - fTemp261;
			let mut fTemp263: F64 = 262143.0 * fTemp256;
			let mut iTemp264: i32 = (fTemp263) as i32;
			let mut iTemp265: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp264, 262143)))), 786431));
			let mut fTemp266: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp265, 3), 786431))) as usize] };
			let mut fTemp267: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp265 as usize] };
			let mut fTemp268: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp265, 1), 786431))) as usize] } - fTemp267;
			let mut iTemp269: i32 = (fTemp144 > ((if iTemp69 != 0 {fTemp267 + fTemp83 * fTemp268 + (fTemp263 - (iTemp264) as F64) * (fTemp266 - (fTemp267 + fTemp83 * (fTemp268 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp265, 4), 786431))) as usize] } - fTemp266))))} else {1.0 - (fTemp261 + fTemp83 * fTemp262 + (fTemp257 - (iTemp258) as F64) * (fTemp260 - (fTemp261 + fTemp83 * (fTemp262 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp259, 4)) as usize] } - fTemp260)))))} - fTemp255) / (1.0 - fTemp255))) as i32;
			let mut fTemp270: F64 = if iTemp269 != 0 {fTemp239} else {fTemp242};
			let mut fTemp271: F64 = if iTemp269 != 0 {fTemp242} else {fTemp240};
			let mut fTemp272: F64 = fTemp271 + fTemp270;
			let mut fTemp273: F64 = 0.5 * fTemp272;
			let mut fTemp274: F64 = 262143.0 * (1.0 - fTemp273);
			let mut iTemp275: i32 = (fTemp274) as i32;
			let mut iTemp276: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp275, 262143)))), 786431));
			let mut fTemp277: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp276, 3)) as usize] };
			let mut fTemp278: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp276 as usize] };
			let mut fTemp279: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp276, 1)) as usize] } - fTemp278;
			let mut fTemp280: F64 = 131071.5 * fTemp272;
			let mut iTemp281: i32 = (fTemp280) as i32;
			let mut iTemp282: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp281, 262143)))), 786431));
			let mut fTemp283: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp282, 3)) as usize] };
			let mut fTemp284: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp282 as usize] };
			let mut fTemp285: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp282, 1)) as usize] } - fTemp284;
			let mut fTemp286: F64 = if iTemp69 != 0 {fTemp284 + fTemp83 * fTemp285 + (fTemp280 - (iTemp281) as F64) * (fTemp283 - (fTemp284 + fTemp83 * (fTemp285 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp282, 4)) as usize] } - fTemp283))))} else {1.0 - (fTemp278 + fTemp83 * fTemp279 + (fTemp274 - (iTemp275) as F64) * (fTemp277 - (fTemp278 + fTemp83 * (fTemp279 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp276, 4)) as usize] } - fTemp277)))))};
			let mut fTemp287: F64 = fTemp88 + fTemp273;
			let mut fTemp288: F64 = 262143.0 * (1.0 - fTemp287);
			let mut iTemp289: i32 = (fTemp288) as i32;
			let mut iTemp290: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp289, 262143)))), 786431));
			let mut fTemp291: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp290, 3)) as usize] };
			let mut fTemp292: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp290 as usize] };
			let mut fTemp293: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp290, 1)) as usize] } - fTemp292;
			let mut fTemp294: F64 = 262143.0 * fTemp287;
			let mut iTemp295: i32 = (fTemp294) as i32;
			let mut iTemp296: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp295, 262143)))), 786431));
			let mut fTemp297: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp296, 3), 786431))) as usize] };
			let mut fTemp298: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp296 as usize] };
			let mut fTemp299: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp296, 1), 786431))) as usize] } - fTemp298;
			let mut iTemp300: i32 = (fTemp144 > ((if iTemp69 != 0 {fTemp298 + fTemp83 * fTemp299 + (fTemp294 - (iTemp295) as F64) * (fTemp297 - (fTemp298 + fTemp83 * (fTemp299 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp296, 4), 786431))) as usize] } - fTemp297))))} else {1.0 - (fTemp292 + fTemp83 * fTemp293 + (fTemp288 - (iTemp289) as F64) * (fTemp291 - (fTemp292 + fTemp83 * (fTemp293 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp290, 4)) as usize] } - fTemp291)))))} - fTemp286) / (1.0 - fTemp286))) as i32;
			let mut fTemp301: F64 = if iTemp300 != 0 {fTemp270} else {fTemp273};
			let mut fTemp302: F64 = if iTemp300 != 0 {fTemp273} else {fTemp271};
			let mut fTemp303: F64 = fTemp302 + fTemp301;
			let mut fTemp304: F64 = 0.5 * fTemp303;
			let mut fTemp305: F64 = 262143.0 * (1.0 - fTemp304);
			let mut iTemp306: i32 = (fTemp305) as i32;
			let mut iTemp307: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp306, 262143)))), 786431));
			let mut fTemp308: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp307, 3)) as usize] };
			let mut fTemp309: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp307 as usize] };
			let mut fTemp310: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp307, 1)) as usize] } - fTemp309;
			let mut fTemp311: F64 = 131071.5 * fTemp303;
			let mut iTemp312: i32 = (fTemp311) as i32;
			let mut iTemp313: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp312, 262143)))), 786431));
			let mut fTemp314: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp313, 3)) as usize] };
			let mut fTemp315: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp313 as usize] };
			let mut fTemp316: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp313, 1)) as usize] } - fTemp315;
			let mut fTemp317: F64 = if iTemp69 != 0 {fTemp315 + fTemp83 * fTemp316 + (fTemp311 - (iTemp312) as F64) * (fTemp314 - (fTemp315 + fTemp83 * (fTemp316 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp313, 4)) as usize] } - fTemp314))))} else {1.0 - (fTemp309 + fTemp83 * fTemp310 + (fTemp305 - (iTemp306) as F64) * (fTemp308 - (fTemp309 + fTemp83 * (fTemp310 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp307, 4)) as usize] } - fTemp308)))))};
			let mut fTemp318: F64 = fTemp88 + fTemp304;
			let mut fTemp319: F64 = 262143.0 * (1.0 - fTemp318);
			let mut iTemp320: i32 = (fTemp319) as i32;
			let mut iTemp321: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp320, 262143)))), 786431));
			let mut fTemp322: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp321, 3)) as usize] };
			let mut fTemp323: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp321 as usize] };
			let mut fTemp324: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp321, 1)) as usize] } - fTemp323;
			let mut fTemp325: F64 = 262143.0 * fTemp318;
			let mut iTemp326: i32 = (fTemp325) as i32;
			let mut iTemp327: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp326, 262143)))), 786431));
			let mut fTemp328: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp327, 3), 786431))) as usize] };
			let mut fTemp329: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp327 as usize] };
			let mut fTemp330: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp327, 1), 786431))) as usize] } - fTemp329;
			let mut iTemp331: i32 = (fTemp144 > ((if iTemp69 != 0 {fTemp329 + fTemp83 * fTemp330 + (fTemp325 - (iTemp326) as F64) * (fTemp328 - (fTemp329 + fTemp83 * (fTemp330 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp327, 4), 786431))) as usize] } - fTemp328))))} else {1.0 - (fTemp323 + fTemp83 * fTemp324 + (fTemp319 - (iTemp320) as F64) * (fTemp322 - (fTemp323 + fTemp83 * (fTemp324 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp321, 4)) as usize] } - fTemp322)))))} - fTemp317) / (1.0 - fTemp317))) as i32;
			let mut fTemp332: F64 = if iTemp331 != 0 {fTemp301} else {fTemp304};
			let mut fTemp333: F64 = if iTemp331 != 0 {fTemp304} else {fTemp302};
			let mut fTemp334: F64 = fTemp333 + fTemp332;
			let mut fTemp335: F64 = 0.5 * fTemp334;
			let mut fTemp336: F64 = 262143.0 * (1.0 - fTemp335);
			let mut iTemp337: i32 = (fTemp336) as i32;
			let mut iTemp338: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp337, 262143)))), 786431));
			let mut fTemp339: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp338, 3)) as usize] };
			let mut fTemp340: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp338 as usize] };
			let mut fTemp341: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp338, 1)) as usize] } - fTemp340;
			let mut fTemp342: F64 = 131071.5 * fTemp334;
			let mut iTemp343: i32 = (fTemp342) as i32;
			let mut iTemp344: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp343, 262143)))), 786431));
			let mut fTemp345: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp344, 3)) as usize] };
			let mut fTemp346: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp344 as usize] };
			let mut fTemp347: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp344, 1)) as usize] } - fTemp346;
			let mut fTemp348: F64 = if iTemp69 != 0 {fTemp346 + fTemp83 * fTemp347 + (fTemp342 - (iTemp343) as F64) * (fTemp345 - (fTemp346 + fTemp83 * (fTemp347 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp344, 4)) as usize] } - fTemp345))))} else {1.0 - (fTemp340 + fTemp83 * fTemp341 + (fTemp336 - (iTemp337) as F64) * (fTemp339 - (fTemp340 + fTemp83 * (fTemp341 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp338, 4)) as usize] } - fTemp339)))))};
			let mut fTemp349: F64 = fTemp88 + fTemp335;
			let mut fTemp350: F64 = 262143.0 * (1.0 - fTemp349);
			let mut iTemp351: i32 = (fTemp350) as i32;
			let mut iTemp352: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp351, 262143)))), 786431));
			let mut fTemp353: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp352, 3)) as usize] };
			let mut fTemp354: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp352 as usize] };
			let mut fTemp355: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp352, 1)) as usize] } - fTemp354;
			let mut fTemp356: F64 = 262143.0 * fTemp349;
			let mut iTemp357: i32 = (fTemp356) as i32;
			let mut iTemp358: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp357, 262143)))), 786431));
			let mut fTemp359: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp358, 3), 786431))) as usize] };
			let mut fTemp360: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp358 as usize] };
			let mut fTemp361: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp358, 1), 786431))) as usize] } - fTemp360;
			let mut iTemp362: i32 = (fTemp144 > ((if iTemp69 != 0 {fTemp360 + fTemp83 * fTemp361 + (fTemp356 - (iTemp357) as F64) * (fTemp359 - (fTemp360 + fTemp83 * (fTemp361 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp358, 4), 786431))) as usize] } - fTemp359))))} else {1.0 - (fTemp354 + fTemp83 * fTemp355 + (fTemp350 - (iTemp351) as F64) * (fTemp353 - (fTemp354 + fTemp83 * (fTemp355 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp352, 4)) as usize] } - fTemp353)))))} - fTemp348) / (1.0 - fTemp348))) as i32;
			let mut fTemp363: F64 = if iTemp362 != 0 {fTemp332} else {fTemp335};
			let mut fTemp364: F64 = if iTemp362 != 0 {fTemp335} else {fTemp333};
			let mut fTemp365: F64 = fTemp364 + fTemp363;
			let mut fTemp366: F64 = 0.5 * fTemp365;
			let mut fTemp367: F64 = 262143.0 * (1.0 - fTemp366);
			let mut iTemp368: i32 = (fTemp367) as i32;
			let mut iTemp369: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp368, 262143)))), 786431));
			let mut fTemp370: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp369, 3)) as usize] };
			let mut fTemp371: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp369 as usize] };
			let mut fTemp372: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp369, 1)) as usize] } - fTemp371;
			let mut fTemp373: F64 = 131071.5 * fTemp365;
			let mut iTemp374: i32 = (fTemp373) as i32;
			let mut iTemp375: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp374, 262143)))), 786431));
			let mut fTemp376: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp375, 3)) as usize] };
			let mut fTemp377: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp375 as usize] };
			let mut fTemp378: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp375, 1)) as usize] } - fTemp377;
			let mut fTemp379: F64 = if iTemp69 != 0 {fTemp377 + fTemp83 * fTemp378 + (fTemp373 - (iTemp374) as F64) * (fTemp376 - (fTemp377 + fTemp83 * (fTemp378 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp375, 4)) as usize] } - fTemp376))))} else {1.0 - (fTemp371 + fTemp83 * fTemp372 + (fTemp367 - (iTemp368) as F64) * (fTemp370 - (fTemp371 + fTemp83 * (fTemp372 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp369, 4)) as usize] } - fTemp370)))))};
			let mut fTemp380: F64 = fTemp88 + fTemp366;
			let mut fTemp381: F64 = 262143.0 * (1.0 - fTemp380);
			let mut iTemp382: i32 = (fTemp381) as i32;
			let mut iTemp383: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp382, 262143)))), 786431));
			let mut fTemp384: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp383, 3)) as usize] };
			let mut fTemp385: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp383 as usize] };
			let mut fTemp386: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp383, 1)) as usize] } - fTemp385;
			let mut fTemp387: F64 = 262143.0 * fTemp380;
			let mut iTemp388: i32 = (fTemp387) as i32;
			let mut iTemp389: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp388, 262143)))), 786431));
			let mut fTemp390: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp389, 3), 786431))) as usize] };
			let mut fTemp391: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp389 as usize] };
			let mut fTemp392: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp389, 1), 786431))) as usize] } - fTemp391;
			let mut iTemp393: i32 = (fTemp144 > ((if iTemp69 != 0 {fTemp391 + fTemp83 * fTemp392 + (fTemp387 - (iTemp388) as F64) * (fTemp390 - (fTemp391 + fTemp83 * (fTemp392 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp389, 4), 786431))) as usize] } - fTemp390))))} else {1.0 - (fTemp385 + fTemp83 * fTemp386 + (fTemp381 - (iTemp382) as F64) * (fTemp384 - (fTemp385 + fTemp83 * (fTemp386 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp383, 4)) as usize] } - fTemp384)))))} - fTemp379) / (1.0 - fTemp379))) as i32;
			let mut fTemp394: F64 = if iTemp393 != 0 {fTemp363} else {fTemp366};
			let mut fTemp395: F64 = if iTemp393 != 0 {fTemp366} else {fTemp364};
			let mut fTemp396: F64 = fTemp395 + fTemp394;
			let mut fTemp397: F64 = 0.5 * fTemp396;
			let mut fTemp398: F64 = 262143.0 * (1.0 - fTemp397);
			let mut iTemp399: i32 = (fTemp398) as i32;
			let mut iTemp400: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp399, 262143)))), 786431));
			let mut fTemp401: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp400, 3)) as usize] };
			let mut fTemp402: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp400 as usize] };
			let mut fTemp403: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp400, 1)) as usize] } - fTemp402;
			let mut fTemp404: F64 = 131071.5 * fTemp396;
			let mut iTemp405: i32 = (fTemp404) as i32;
			let mut iTemp406: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp405, 262143)))), 786431));
			let mut fTemp407: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp406, 3)) as usize] };
			let mut fTemp408: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp406 as usize] };
			let mut fTemp409: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp406, 1)) as usize] } - fTemp408;
			let mut fTemp410: F64 = if iTemp69 != 0 {fTemp408 + fTemp83 * fTemp409 + (fTemp404 - (iTemp405) as F64) * (fTemp407 - (fTemp408 + fTemp83 * (fTemp409 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp406, 4)) as usize] } - fTemp407))))} else {1.0 - (fTemp402 + fTemp83 * fTemp403 + (fTemp398 - (iTemp399) as F64) * (fTemp401 - (fTemp402 + fTemp83 * (fTemp403 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp400, 4)) as usize] } - fTemp401)))))};
			let mut fTemp411: F64 = fTemp88 + fTemp397;
			let mut fTemp412: F64 = 262143.0 * (1.0 - fTemp411);
			let mut iTemp413: i32 = (fTemp412) as i32;
			let mut iTemp414: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp413, 262143)))), 786431));
			let mut fTemp415: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp414, 3)) as usize] };
			let mut fTemp416: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp414 as usize] };
			let mut fTemp417: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp414, 1)) as usize] } - fTemp416;
			let mut fTemp418: F64 = 262143.0 * fTemp411;
			let mut iTemp419: i32 = (fTemp418) as i32;
			let mut iTemp420: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp419, 262143)))), 786431));
			let mut fTemp421: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp420, 3), 786431))) as usize] };
			let mut fTemp422: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp420 as usize] };
			let mut fTemp423: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp420, 1), 786431))) as usize] } - fTemp422;
			let mut iTemp424: i32 = (fTemp144 > ((if iTemp69 != 0 {fTemp422 + fTemp83 * fTemp423 + (fTemp418 - (iTemp419) as F64) * (fTemp421 - (fTemp422 + fTemp83 * (fTemp423 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp420, 4), 786431))) as usize] } - fTemp421))))} else {1.0 - (fTemp416 + fTemp83 * fTemp417 + (fTemp412 - (iTemp413) as F64) * (fTemp415 - (fTemp416 + fTemp83 * (fTemp417 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp414, 4)) as usize] } - fTemp415)))))} - fTemp410) / (1.0 - fTemp410))) as i32;
			let mut fTemp425: F64 = if iTemp424 != 0 {fTemp394} else {fTemp397};
			let mut fTemp426: F64 = if iTemp424 != 0 {fTemp397} else {fTemp395};
			let mut fTemp427: F64 = fTemp426 + fTemp425;
			let mut fTemp428: F64 = 0.5 * fTemp427;
			let mut fTemp429: F64 = 262143.0 * (1.0 - fTemp428);
			let mut iTemp430: i32 = (fTemp429) as i32;
			let mut iTemp431: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp430, 262143)))), 786431));
			let mut fTemp432: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp431, 3)) as usize] };
			let mut fTemp433: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp431 as usize] };
			let mut fTemp434: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp431, 1)) as usize] } - fTemp433;
			let mut fTemp435: F64 = 131071.5 * fTemp427;
			let mut iTemp436: i32 = (fTemp435) as i32;
			let mut iTemp437: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp436, 262143)))), 786431));
			let mut fTemp438: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp437, 3)) as usize] };
			let mut fTemp439: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp437 as usize] };
			let mut fTemp440: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp437, 1)) as usize] } - fTemp439;
			let mut fTemp441: F64 = if iTemp69 != 0 {fTemp439 + fTemp83 * fTemp440 + (fTemp435 - (iTemp436) as F64) * (fTemp438 - (fTemp439 + fTemp83 * (fTemp440 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp437, 4)) as usize] } - fTemp438))))} else {1.0 - (fTemp433 + fTemp83 * fTemp434 + (fTemp429 - (iTemp430) as F64) * (fTemp432 - (fTemp433 + fTemp83 * (fTemp434 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp431, 4)) as usize] } - fTemp432)))))};
			let mut fTemp442: F64 = fTemp88 + fTemp428;
			let mut fTemp443: F64 = 262143.0 * (1.0 - fTemp442);
			let mut iTemp444: i32 = (fTemp443) as i32;
			let mut iTemp445: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp444, 262143)))), 786431));
			let mut fTemp446: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp445, 3)) as usize] };
			let mut fTemp447: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp445 as usize] };
			let mut fTemp448: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp445, 1)) as usize] } - fTemp447;
			let mut fTemp449: F64 = 262143.0 * fTemp442;
			let mut iTemp450: i32 = (fTemp449) as i32;
			let mut iTemp451: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp450, 262143)))), 786431));
			let mut fTemp452: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp451, 3), 786431))) as usize] };
			let mut fTemp453: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp451 as usize] };
			let mut fTemp454: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp451, 1), 786431))) as usize] } - fTemp453;
			let mut iTemp455: i32 = (fTemp144 > ((if iTemp69 != 0 {fTemp453 + fTemp83 * fTemp454 + (fTemp449 - (iTemp450) as F64) * (fTemp452 - (fTemp453 + fTemp83 * (fTemp454 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp451, 4), 786431))) as usize] } - fTemp452))))} else {1.0 - (fTemp447 + fTemp83 * fTemp448 + (fTemp443 - (iTemp444) as F64) * (fTemp446 - (fTemp447 + fTemp83 * (fTemp448 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp445, 4)) as usize] } - fTemp446)))))} - fTemp441) / (1.0 - fTemp441))) as i32;
			let mut fTemp456: F64 = if iTemp455 != 0 {fTemp425} else {fTemp428};
			let mut fTemp457: F64 = if iTemp455 != 0 {fTemp428} else {fTemp426};
			let mut fTemp458: F64 = fTemp457 + fTemp456;
			let mut fTemp459: F64 = 0.5 * fTemp458;
			let mut fTemp460: F64 = 262143.0 * (1.0 - fTemp459);
			let mut iTemp461: i32 = (fTemp460) as i32;
			let mut iTemp462: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp461, 262143)))), 786431));
			let mut fTemp463: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp462, 3)) as usize] };
			let mut fTemp464: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp462 as usize] };
			let mut fTemp465: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp462, 1)) as usize] } - fTemp464;
			let mut fTemp466: F64 = 131071.5 * fTemp458;
			let mut iTemp467: i32 = (fTemp466) as i32;
			let mut iTemp468: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp467, 262143)))), 786431));
			let mut fTemp469: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp468, 3)) as usize] };
			let mut fTemp470: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp468 as usize] };
			let mut fTemp471: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp468, 1)) as usize] } - fTemp470;
			let mut fTemp472: F64 = if iTemp69 != 0 {fTemp470 + fTemp83 * fTemp471 + (fTemp466 - (iTemp467) as F64) * (fTemp469 - (fTemp470 + fTemp83 * (fTemp471 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp468, 4)) as usize] } - fTemp469))))} else {1.0 - (fTemp464 + fTemp83 * fTemp465 + (fTemp460 - (iTemp461) as F64) * (fTemp463 - (fTemp464 + fTemp83 * (fTemp465 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp462, 4)) as usize] } - fTemp463)))))};
			let mut fTemp473: F64 = fTemp88 + fTemp459;
			let mut fTemp474: F64 = 262143.0 * (1.0 - fTemp473);
			let mut iTemp475: i32 = (fTemp474) as i32;
			let mut iTemp476: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp475, 262143)))), 786431));
			let mut fTemp477: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp476, 3)) as usize] };
			let mut fTemp478: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp476 as usize] };
			let mut fTemp479: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp476, 1)) as usize] } - fTemp478;
			let mut fTemp480: F64 = 262143.0 * fTemp473;
			let mut iTemp481: i32 = (fTemp480) as i32;
			let mut iTemp482: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp481, 262143)))), 786431));
			let mut fTemp483: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp482, 3), 786431))) as usize] };
			let mut fTemp484: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp482 as usize] };
			let mut fTemp485: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp482, 1), 786431))) as usize] } - fTemp484;
			let mut iTemp486: i32 = (fTemp144 > ((if iTemp69 != 0 {fTemp484 + fTemp83 * fTemp485 + (fTemp480 - (iTemp481) as F64) * (fTemp483 - (fTemp484 + fTemp83 * (fTemp485 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp482, 4), 786431))) as usize] } - fTemp483))))} else {1.0 - (fTemp478 + fTemp83 * fTemp479 + (fTemp474 - (iTemp475) as F64) * (fTemp477 - (fTemp478 + fTemp83 * (fTemp479 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp476, 4)) as usize] } - fTemp477)))))} - fTemp472) / (1.0 - fTemp472))) as i32;
			let mut fTemp487: F64 = if iTemp486 != 0 {fTemp456} else {fTemp459};
			let mut fTemp488: F64 = if iTemp486 != 0 {fTemp459} else {fTemp457};
			let mut fTemp489: F64 = fTemp488 + fTemp487;
			let mut fTemp490: F64 = 0.5 * fTemp489;
			let mut fTemp491: F64 = 262143.0 * (1.0 - fTemp490);
			let mut iTemp492: i32 = (fTemp491) as i32;
			let mut iTemp493: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp492, 262143)))), 786431));
			let mut fTemp494: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp493, 3)) as usize] };
			let mut fTemp495: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp493 as usize] };
			let mut fTemp496: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp493, 1)) as usize] } - fTemp495;
			let mut fTemp497: F64 = 131071.5 * fTemp489;
			let mut iTemp498: i32 = (fTemp497) as i32;
			let mut iTemp499: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp498, 262143)))), 786431));
			let mut fTemp500: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp499, 3)) as usize] };
			let mut fTemp501: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp499 as usize] };
			let mut fTemp502: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp499, 1)) as usize] } - fTemp501;
			let mut fTemp503: F64 = if iTemp69 != 0 {fTemp501 + fTemp83 * fTemp502 + (fTemp497 - (iTemp498) as F64) * (fTemp500 - (fTemp501 + fTemp83 * (fTemp502 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp499, 4)) as usize] } - fTemp500))))} else {1.0 - (fTemp495 + fTemp83 * fTemp496 + (fTemp491 - (iTemp492) as F64) * (fTemp494 - (fTemp495 + fTemp83 * (fTemp496 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp493, 4)) as usize] } - fTemp494)))))};
			let mut fTemp504: F64 = fTemp88 + fTemp490;
			let mut fTemp505: F64 = 262143.0 * (1.0 - fTemp504);
			let mut iTemp506: i32 = (fTemp505) as i32;
			let mut iTemp507: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp506, 262143)))), 786431));
			let mut fTemp508: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp507, 3)) as usize] };
			let mut fTemp509: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp507 as usize] };
			let mut fTemp510: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp507, 1)) as usize] } - fTemp509;
			let mut fTemp511: F64 = 262143.0 * fTemp504;
			let mut iTemp512: i32 = (fTemp511) as i32;
			let mut iTemp513: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp512, 262143)))), 786431));
			let mut fTemp514: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp513, 3), 786431))) as usize] };
			let mut fTemp515: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp513 as usize] };
			let mut fTemp516: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp513, 1), 786431))) as usize] } - fTemp515;
			let mut iTemp517: i32 = (fTemp144 > ((if iTemp69 != 0 {fTemp515 + fTemp83 * fTemp516 + (fTemp511 - (iTemp512) as F64) * (fTemp514 - (fTemp515 + fTemp83 * (fTemp516 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp513, 4), 786431))) as usize] } - fTemp514))))} else {1.0 - (fTemp509 + fTemp83 * fTemp510 + (fTemp505 - (iTemp506) as F64) * (fTemp508 - (fTemp509 + fTemp83 * (fTemp510 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp507, 4)) as usize] } - fTemp508)))))} - fTemp503) / (1.0 - fTemp503))) as i32;
			let mut fTemp518: F64 = if iTemp517 != 0 {fTemp487} else {fTemp490};
			let mut fTemp519: F64 = if iTemp517 != 0 {fTemp490} else {fTemp488};
			let mut fTemp520: F64 = fTemp519 + fTemp518;
			let mut fTemp521: F64 = 0.5 * fTemp520;
			let mut fTemp522: F64 = 262143.0 * (1.0 - fTemp521);
			let mut iTemp523: i32 = (fTemp522) as i32;
			let mut iTemp524: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp523, 262143)))), 786431));
			let mut fTemp525: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp524, 3)) as usize] };
			let mut fTemp526: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp524 as usize] };
			let mut fTemp527: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp524, 1)) as usize] } - fTemp526;
			let mut fTemp528: F64 = 131071.5 * fTemp520;
			let mut iTemp529: i32 = (fTemp528) as i32;
			let mut iTemp530: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp529, 262143)))), 786431));
			let mut fTemp531: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp530, 3)) as usize] };
			let mut fTemp532: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp530 as usize] };
			let mut fTemp533: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp530, 1)) as usize] } - fTemp532;
			let mut fTemp534: F64 = if iTemp69 != 0 {fTemp532 + fTemp83 * fTemp533 + (fTemp528 - (iTemp529) as F64) * (fTemp531 - (fTemp532 + fTemp83 * (fTemp533 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp530, 4)) as usize] } - fTemp531))))} else {1.0 - (fTemp526 + fTemp83 * fTemp527 + (fTemp522 - (iTemp523) as F64) * (fTemp525 - (fTemp526 + fTemp83 * (fTemp527 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp524, 4)) as usize] } - fTemp525)))))};
			let mut fTemp535: F64 = fTemp88 + fTemp521;
			let mut fTemp536: F64 = 262143.0 * (1.0 - fTemp535);
			let mut iTemp537: i32 = (fTemp536) as i32;
			let mut iTemp538: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp537, 262143)))), 786431));
			let mut fTemp539: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp538, 3)) as usize] };
			let mut fTemp540: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp538 as usize] };
			let mut fTemp541: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp538, 1)) as usize] } - fTemp540;
			let mut fTemp542: F64 = 262143.0 * fTemp535;
			let mut iTemp543: i32 = (fTemp542) as i32;
			let mut iTemp544: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp543, 262143)))), 786431));
			let mut fTemp545: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp544, 3), 786431))) as usize] };
			let mut fTemp546: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp544 as usize] };
			let mut fTemp547: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp544, 1), 786431))) as usize] } - fTemp546;
			let mut iTemp548: i32 = (fTemp144 > ((if iTemp69 != 0 {fTemp546 + fTemp83 * fTemp547 + (fTemp542 - (iTemp543) as F64) * (fTemp545 - (fTemp546 + fTemp83 * (fTemp547 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp544, 4), 786431))) as usize] } - fTemp545))))} else {1.0 - (fTemp540 + fTemp83 * fTemp541 + (fTemp536 - (iTemp537) as F64) * (fTemp539 - (fTemp540 + fTemp83 * (fTemp541 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp538, 4)) as usize] } - fTemp539)))))} - fTemp534) / (1.0 - fTemp534))) as i32;
			let mut fTemp549: F64 = if iTemp548 != 0 {fTemp518} else {fTemp521};
			let mut fTemp550: F64 = if iTemp548 != 0 {fTemp521} else {fTemp519};
			let mut fTemp551: F64 = fTemp550 + fTemp549;
			let mut fTemp552: F64 = 0.5 * fTemp551;
			let mut fTemp553: F64 = 262143.0 * (1.0 - fTemp552);
			let mut iTemp554: i32 = (fTemp553) as i32;
			let mut iTemp555: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp554, 262143)))), 786431));
			let mut fTemp556: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp555, 3)) as usize] };
			let mut fTemp557: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp555 as usize] };
			let mut fTemp558: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp555, 1)) as usize] } - fTemp557;
			let mut fTemp559: F64 = 131071.5 * fTemp551;
			let mut iTemp560: i32 = (fTemp559) as i32;
			let mut iTemp561: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp560, 262143)))), 786431));
			let mut fTemp562: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp561, 3)) as usize] };
			let mut fTemp563: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp561 as usize] };
			let mut fTemp564: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp561, 1)) as usize] } - fTemp563;
			let mut fTemp565: F64 = if iTemp69 != 0 {fTemp563 + fTemp83 * fTemp564 + (fTemp559 - (iTemp560) as F64) * (fTemp562 - (fTemp563 + fTemp83 * (fTemp564 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp561, 4)) as usize] } - fTemp562))))} else {1.0 - (fTemp557 + fTemp83 * fTemp558 + (fTemp553 - (iTemp554) as F64) * (fTemp556 - (fTemp557 + fTemp83 * (fTemp558 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp555, 4)) as usize] } - fTemp556)))))};
			let mut fTemp566: F64 = fTemp88 + fTemp552;
			let mut fTemp567: F64 = 262143.0 * (1.0 - fTemp566);
			let mut iTemp568: i32 = (fTemp567) as i32;
			let mut iTemp569: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp568, 262143)))), 786431));
			let mut fTemp570: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp569, 3)) as usize] };
			let mut fTemp571: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp569 as usize] };
			let mut fTemp572: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp569, 1)) as usize] } - fTemp571;
			let mut fTemp573: F64 = 262143.0 * fTemp566;
			let mut iTemp574: i32 = (fTemp573) as i32;
			let mut iTemp575: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp574, 262143)))), 786431));
			let mut fTemp576: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp575, 3), 786431))) as usize] };
			let mut fTemp577: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp575 as usize] };
			let mut fTemp578: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp575, 1), 786431))) as usize] } - fTemp577;
			let mut iTemp579: i32 = (fTemp144 > ((if iTemp69 != 0 {fTemp577 + fTemp83 * fTemp578 + (fTemp573 - (iTemp574) as F64) * (fTemp576 - (fTemp577 + fTemp83 * (fTemp578 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp575, 4), 786431))) as usize] } - fTemp576))))} else {1.0 - (fTemp571 + fTemp83 * fTemp572 + (fTemp567 - (iTemp568) as F64) * (fTemp570 - (fTemp571 + fTemp83 * (fTemp572 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp569, 4)) as usize] } - fTemp570)))))} - fTemp565) / (1.0 - fTemp565))) as i32;
			let mut fTemp580: F64 = if iTemp579 != 0 {fTemp549} else {fTemp552};
			let mut fTemp581: F64 = if iTemp579 != 0 {fTemp552} else {fTemp550};
			let mut fTemp582: F64 = fTemp581 + fTemp580;
			let mut fTemp583: F64 = 0.5 * fTemp582;
			let mut fTemp584: F64 = 262143.0 * (1.0 - fTemp583);
			let mut iTemp585: i32 = (fTemp584) as i32;
			let mut iTemp586: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp585, 262143)))), 786431));
			let mut fTemp587: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp586, 3)) as usize] };
			let mut fTemp588: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp586 as usize] };
			let mut fTemp589: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp586, 1)) as usize] } - fTemp588;
			let mut fTemp590: F64 = 131071.5 * fTemp582;
			let mut iTemp591: i32 = (fTemp590) as i32;
			let mut iTemp592: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp591, 262143)))), 786431));
			let mut fTemp593: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp592, 3)) as usize] };
			let mut fTemp594: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp592 as usize] };
			let mut fTemp595: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp592, 1)) as usize] } - fTemp594;
			let mut fTemp596: F64 = if iTemp69 != 0 {fTemp594 + fTemp83 * fTemp595 + (fTemp590 - (iTemp591) as F64) * (fTemp593 - (fTemp594 + fTemp83 * (fTemp595 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp592, 4)) as usize] } - fTemp593))))} else {1.0 - (fTemp588 + fTemp83 * fTemp589 + (fTemp584 - (iTemp585) as F64) * (fTemp587 - (fTemp588 + fTemp83 * (fTemp589 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp586, 4)) as usize] } - fTemp587)))))};
			let mut fTemp597: F64 = fTemp88 + fTemp583;
			let mut fTemp598: F64 = 262143.0 * (1.0 - fTemp597);
			let mut iTemp599: i32 = (fTemp598) as i32;
			let mut iTemp600: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp599, 262143)))), 786431));
			let mut fTemp601: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp600, 3)) as usize] };
			let mut fTemp602: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp600 as usize] };
			let mut fTemp603: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp600, 1)) as usize] } - fTemp602;
			let mut fTemp604: F64 = 262143.0 * fTemp597;
			let mut iTemp605: i32 = (fTemp604) as i32;
			let mut iTemp606: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp605, 262143)))), 786431));
			let mut fTemp607: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp606, 3), 786431))) as usize] };
			let mut fTemp608: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp606 as usize] };
			let mut fTemp609: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp606, 1), 786431))) as usize] } - fTemp608;
			let mut iTemp610: i32 = (fTemp144 > ((if iTemp69 != 0 {fTemp608 + fTemp83 * fTemp609 + (fTemp604 - (iTemp605) as F64) * (fTemp607 - (fTemp608 + fTemp83 * (fTemp609 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp606, 4), 786431))) as usize] } - fTemp607))))} else {1.0 - (fTemp602 + fTemp83 * fTemp603 + (fTemp598 - (iTemp599) as F64) * (fTemp601 - (fTemp602 + fTemp83 * (fTemp603 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp600, 4)) as usize] } - fTemp601)))))} - fTemp596) / (1.0 - fTemp596))) as i32;
			let mut fTemp611: F64 = if iTemp610 != 0 {fTemp580} else {fTemp583};
			let mut fTemp612: F64 = if iTemp610 != 0 {fTemp583} else {fTemp581};
			let mut fTemp613: F64 = fTemp612 + fTemp611;
			let mut fTemp614: F64 = 0.5 * fTemp613;
			let mut fTemp615: F64 = 262143.0 * (1.0 - fTemp614);
			let mut iTemp616: i32 = (fTemp615) as i32;
			let mut iTemp617: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp616, 262143)))), 786431));
			let mut fTemp618: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp617, 3)) as usize] };
			let mut fTemp619: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp617 as usize] };
			let mut fTemp620: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp617, 1)) as usize] } - fTemp619;
			let mut fTemp621: F64 = 131071.5 * fTemp613;
			let mut iTemp622: i32 = (fTemp621) as i32;
			let mut iTemp623: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp622, 262143)))), 786431));
			let mut fTemp624: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp623, 3)) as usize] };
			let mut fTemp625: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp623 as usize] };
			let mut fTemp626: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp623, 1)) as usize] } - fTemp625;
			let mut fTemp627: F64 = if iTemp69 != 0 {fTemp625 + fTemp83 * fTemp626 + (fTemp621 - (iTemp622) as F64) * (fTemp624 - (fTemp625 + fTemp83 * (fTemp626 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp623, 4)) as usize] } - fTemp624))))} else {1.0 - (fTemp619 + fTemp83 * fTemp620 + (fTemp615 - (iTemp616) as F64) * (fTemp618 - (fTemp619 + fTemp83 * (fTemp620 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp617, 4)) as usize] } - fTemp618)))))};
			let mut fTemp628: F64 = fTemp88 + fTemp614;
			let mut fTemp629: F64 = 262143.0 * (1.0 - fTemp628);
			let mut iTemp630: i32 = (fTemp629) as i32;
			let mut iTemp631: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp630, 262143)))), 786431));
			let mut fTemp632: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp631, 3)) as usize] };
			let mut fTemp633: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp631 as usize] };
			let mut fTemp634: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp631, 1)) as usize] } - fTemp633;
			let mut fTemp635: F64 = 262143.0 * fTemp628;
			let mut iTemp636: i32 = (fTemp635) as i32;
			let mut iTemp637: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp636, 262143)))), 786431));
			let mut fTemp638: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp637, 3), 786431))) as usize] };
			let mut fTemp639: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp637 as usize] };
			let mut fTemp640: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp637, 1), 786431))) as usize] } - fTemp639;
			let mut iTemp641: i32 = (fTemp144 > ((if iTemp69 != 0 {fTemp639 + fTemp83 * fTemp640 + (fTemp635 - (iTemp636) as F64) * (fTemp638 - (fTemp639 + fTemp83 * (fTemp640 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp637, 4), 786431))) as usize] } - fTemp638))))} else {1.0 - (fTemp633 + fTemp83 * fTemp634 + (fTemp629 - (iTemp630) as F64) * (fTemp632 - (fTemp633 + fTemp83 * (fTemp634 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp631, 4)) as usize] } - fTemp632)))))} - fTemp627) / (1.0 - fTemp627))) as i32;
			let mut fTemp642: F64 = if iTemp641 != 0 {fTemp611} else {fTemp614};
			let mut fTemp643: F64 = if iTemp641 != 0 {fTemp614} else {fTemp612};
			let mut fTemp644: F64 = fTemp643 + fTemp642;
			let mut fTemp645: F64 = 0.5 * fTemp644;
			let mut fTemp646: F64 = 262143.0 * (1.0 - fTemp645);
			let mut iTemp647: i32 = (fTemp646) as i32;
			let mut iTemp648: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp647, 262143)))), 786431));
			let mut fTemp649: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp648, 3)) as usize] };
			let mut fTemp650: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp648 as usize] };
			let mut fTemp651: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp648, 1)) as usize] } - fTemp650;
			let mut fTemp652: F64 = 131071.5 * fTemp644;
			let mut iTemp653: i32 = (fTemp652) as i32;
			let mut iTemp654: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp653, 262143)))), 786431));
			let mut fTemp655: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp654, 3)) as usize] };
			let mut fTemp656: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp654 as usize] };
			let mut fTemp657: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp654, 1)) as usize] } - fTemp656;
			let mut fTemp658: F64 = if iTemp69 != 0 {fTemp656 + fTemp83 * fTemp657 + (fTemp652 - (iTemp653) as F64) * (fTemp655 - (fTemp656 + fTemp83 * (fTemp657 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp654, 4), 786431))) as usize] } - fTemp655))))} else {1.0 - (fTemp650 + fTemp83 * fTemp651 + (fTemp646 - (iTemp647) as F64) * (fTemp649 - (fTemp650 + fTemp83 * (fTemp651 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp648, 4), 786431))) as usize] } - fTemp649)))))};
			let mut fTemp659: F64 = fTemp88 + fTemp645;
			let mut fTemp660: F64 = 262143.0 * (1.0 - fTemp659);
			let mut iTemp661: i32 = (fTemp660) as i32;
			let mut iTemp662: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp661, 262143)))), 786431));
			let mut fTemp663: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp662, 3)) as usize] };
			let mut fTemp664: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp662 as usize] };
			let mut fTemp665: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp662, 1)) as usize] } - fTemp664;
			let mut fTemp666: F64 = 262143.0 * fTemp659;
			let mut iTemp667: i32 = (fTemp666) as i32;
			let mut iTemp668: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp667, 262143)))), 786431));
			let mut fTemp669: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp668, 3), 786431))) as usize] };
			let mut fTemp670: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp668 as usize] };
			let mut fTemp671: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp668, 1), 786431))) as usize] } - fTemp670;
			let mut iTemp672: i32 = (fTemp144 > ((if iTemp69 != 0 {fTemp670 + fTemp83 * fTemp671 + (fTemp666 - (iTemp667) as F64) * (fTemp669 - (fTemp670 + fTemp83 * (fTemp671 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp668, 4), 786431))) as usize] } - fTemp669))))} else {1.0 - (fTemp664 + fTemp83 * fTemp665 + (fTemp660 - (iTemp661) as F64) * (fTemp663 - (fTemp664 + fTemp83 * (fTemp665 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp662, 4)) as usize] } - fTemp663)))))} - fTemp658) / (1.0 - fTemp658))) as i32;
			let mut fTemp673: F64 = F64::min(1.0, F64::max(0.0, 0.5 * (if iTemp672 != 0 {fTemp645} else {fTemp643} + if iTemp672 != 0 {fTemp642} else {fTemp645})));
			self.fRec1[0] = fTemp673;
			let mut fTemp674: F64 = (i32::wrapping_sub(1, iTemp27)) as F64;
			let mut fTemp675: F64 = 262143.0 * (1.0 - fTemp673);
			let mut iTemp676: i32 = (fTemp675) as i32;
			let mut iTemp677: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp676, 262143)))), 786431));
			let mut fTemp678: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp677, 3)) as usize] };
			let mut fTemp679: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp677 as usize] };
			let mut fTemp680: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp677, 1)) as usize] } - fTemp679;
			let mut fTemp681: F64 = 262143.0 * fTemp673;
			let mut iTemp682: i32 = (fTemp681) as i32;
			let mut iTemp683: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp682, 262143)))), 786431));
			let mut fTemp684: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp683, 3)) as usize] };
			let mut fTemp685: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp683 as usize] };
			let mut fTemp686: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp683, 1)) as usize] } - fTemp685;
			let mut fTemp687: F64 = if iTemp69 != 0 {fTemp685 + fTemp83 * fTemp686 + (fTemp681 - (iTemp682) as F64) * (fTemp684 - (fTemp685 + fTemp83 * (fTemp686 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp683, 4), 786431))) as usize] } - fTemp684))))} else {1.0 - (fTemp679 + fTemp83 * fTemp680 + (fTemp675 - (iTemp676) as F64) * (fTemp678 - (fTemp679 + fTemp83 * (fTemp680 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp677, 4), 786431))) as usize] } - fTemp678)))))};
			let mut fTemp688: F64 = fTemp88 + fTemp673;
			let mut fTemp689: F64 = 262143.0 * (1.0 - fTemp688);
			let mut iTemp690: i32 = (fTemp689) as i32;
			let mut iTemp691: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp690, 262143)))), 786431));
			let mut fTemp692: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp691, 3)) as usize] };
			let mut fTemp693: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp691 as usize] };
			let mut fTemp694: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp691, 1)) as usize] } - fTemp693;
			let mut fTemp695: F64 = 262143.0 * fTemp688;
			let mut iTemp696: i32 = (fTemp695) as i32;
			let mut iTemp697: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp78, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp696, 262143)))), 786431));
			let mut fTemp698: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp697, 3), 786431))) as usize] };
			let mut fTemp699: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp697 as usize] };
			let mut fTemp700: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp697, 1), 786431))) as usize] } - fTemp699;
			let mut fTemp701: F64 = fTemp5 + if ((0.001 * fTemp87) == 0.0) as i32 != 0 {fTemp68} else {fTemp68 * (if iTemp69 != 0 {fTemp699 + fTemp83 * fTemp700 + (fTemp695 - (iTemp696) as F64) * (fTemp698 - (fTemp699 + fTemp83 * (fTemp700 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp697, 4), 786431))) as usize] } - fTemp698))))} else {1.0 - (fTemp693 + fTemp83 * fTemp694 + (fTemp689 - (iTemp690) as F64) * (fTemp692 - (fTemp693 + fTemp83 * (fTemp694 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp691, 4)) as usize] } - fTemp692)))))} - fTemp687) / (1.0 - fTemp687)};
			self.fRec2[(self.IOTA0 & 32767) as usize] = F64::max(fTemp674, if iTemp86 != 0 {F64::min(fTemp701, fTemp5)} else {F64::max(fTemp701, fTemp5)});
			let mut fTemp702: F64 = self.fRec2[((i32::wrapping_sub(self.IOTA0, iSlow78)) & 32767) as usize];
			self.fRec14[0] = fSlow80 + self.fConst4 * self.fRec14[1];
			*output0 = 0.5 * self.fVec1[((i32::wrapping_sub(self.IOTA0, iSlow79)) & 32767) as usize] * fTemp2 + self.fRec14[0] * self.fVec2[((i32::wrapping_sub(self.IOTA0, iSlow79)) & 32767) as usize] * fTemp702 * fTemp4;
			let mut fTemp703: F64 = self.fRec16[((i32::wrapping_sub(self.IOTA0, 1)) & 32767) as usize];
			let mut fTemp704: F64 = fTemp38 + fSlow17 * (fTemp39 - fTemp38);
			let mut iTemp705: i32 = ((fTemp704 > fSlow11) as i32) + ((fTemp704 > fSlow9) as i32);
			let mut fTemp706: F64 = fTemp704 - fSlow8;
			let mut fTemp707: F64 = F64::min(fTemp36, F64::powf(1e+01, -(fSlow18 * F64::max(0.0, if (iTemp705 == 0) as i32 != 0 {0.0} else {if (iTemp705 == 1) as i32 != 0 {fSlow12 * LambRs192k_faustpower2_f(fSlow7 + fTemp706)} else {fTemp706}}))));
			self.fVec35[(self.IOTA0 & 16383) as usize] = fTemp707;
			let mut fTemp708: F64 = F64::min(fTemp707, self.fVec35[((i32::wrapping_sub(self.IOTA0, 1)) & 16383) as usize]);
			self.fVec36[0] = fTemp708;
			let mut fTemp709: F64 = F64::min(fTemp708, self.fVec36[2]);
			self.fVec37[0] = fTemp709;
			let mut fTemp710: F64 = F64::min(fTemp709, self.fVec37[4]);
			self.fVec38[0] = fTemp710;
			let mut fTemp711: F64 = F64::min(fTemp710, self.fVec38[8]);
			self.fVec39[(self.IOTA0 & 31) as usize] = fTemp711;
			let mut fTemp712: F64 = F64::min(fTemp711, self.fVec39[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize]);
			self.fVec40[(self.IOTA0 & 63) as usize] = fTemp712;
			let mut fTemp713: F64 = F64::min(fTemp712, self.fVec40[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize]);
			self.fVec41[(self.IOTA0 & 127) as usize] = fTemp713;
			let mut fTemp714: F64 = F64::min(fTemp713, self.fVec41[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize]);
			self.fVec42[(self.IOTA0 & 255) as usize] = fTemp714;
			let mut fTemp715: F64 = F64::min(fTemp714, self.fVec42[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize]);
			self.fVec43[(self.IOTA0 & 511) as usize] = fTemp715;
			let mut fTemp716: F64 = F64::min(fTemp715, self.fVec43[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize]);
			self.fVec44[(self.IOTA0 & 1023) as usize] = fTemp716;
			let mut fTemp717: F64 = F64::min(fTemp716, self.fVec44[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize]);
			self.fVec45[(self.IOTA0 & 2047) as usize] = fTemp717;
			let mut fTemp718: F64 = F64::min(fTemp717, self.fVec45[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			self.fVec46[(self.IOTA0 & 4095) as usize] = fTemp718;
			let mut fTemp719: F64 = F64::min(fTemp718, self.fVec46[((i32::wrapping_sub(self.IOTA0, 2048)) & 4095) as usize]);
			self.fVec47[(self.IOTA0 & 8191) as usize] = fTemp719;
			self.fVec48[(self.IOTA0 & 16383) as usize] = F64::min(fTemp719, self.fVec47[((i32::wrapping_sub(self.IOTA0, 4096)) & 8191) as usize]);
			self.fRec17[0] = F64::max(F64::min(self.fRec17[1], self.fVec35[((i32::wrapping_sub(self.IOTA0, iSlow21)) & 16383) as usize]), F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow23 != 0 {fTemp707} else {1.7976931348623157e+308}, if iSlow24 != 0 {self.fVec36[iSlow23 as usize]} else {1.7976931348623157e+308}), if iSlow25 != 0 {self.fVec37[iSlow26 as usize]} else {1.7976931348623157e+308}), if iSlow27 != 0 {self.fVec38[iSlow28 as usize]} else {1.7976931348623157e+308}), if iSlow29 != 0 {self.fVec39[((i32::wrapping_sub(self.IOTA0, iSlow30)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow31 != 0 {self.fVec40[((i32::wrapping_sub(self.IOTA0, iSlow32)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow33 != 0 {self.fVec41[((i32::wrapping_sub(self.IOTA0, iSlow34)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow35 != 0 {self.fVec42[((i32::wrapping_sub(self.IOTA0, iSlow36)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow37 != 0 {self.fVec43[((i32::wrapping_sub(self.IOTA0, iSlow38)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow39 != 0 {self.fVec44[((i32::wrapping_sub(self.IOTA0, iSlow40)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow41 != 0 {self.fVec45[((i32::wrapping_sub(self.IOTA0, iSlow42)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow43 != 0 {self.fVec46[((i32::wrapping_sub(self.IOTA0, iSlow44)) & 4095) as usize]} else {1.7976931348623157e+308}), if iSlow45 != 0 {self.fVec47[((i32::wrapping_sub(self.IOTA0, iSlow46)) & 8191) as usize]} else {1.7976931348623157e+308}), if iSlow47 != 0 {self.fVec48[((i32::wrapping_sub(self.IOTA0, iSlow48)) & 16383) as usize]} else {1.7976931348623157e+308}));
			let mut fTemp720: F64 = F64::min(self.fRec17[0], self.fRec17[1]);
			self.fVec49[0] = fTemp720;
			let mut fTemp721: F64 = F64::min(fTemp720, self.fVec49[2]);
			self.fVec50[0] = fTemp721;
			let mut fTemp722: F64 = F64::min(fTemp721, self.fVec50[4]);
			self.fVec51[0] = fTemp722;
			let mut fTemp723: F64 = F64::min(fTemp722, self.fVec51[8]);
			self.fVec52[(self.IOTA0 & 31) as usize] = fTemp723;
			let mut fTemp724: F64 = F64::min(fTemp723, self.fVec52[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize]);
			self.fVec53[(self.IOTA0 & 63) as usize] = fTemp724;
			let mut fTemp725: F64 = F64::min(fTemp724, self.fVec53[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize]);
			self.fVec54[(self.IOTA0 & 127) as usize] = fTemp725;
			let mut fTemp726: F64 = F64::min(fTemp725, self.fVec54[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize]);
			self.fVec55[(self.IOTA0 & 255) as usize] = fTemp726;
			let mut fTemp727: F64 = F64::min(fTemp726, self.fVec55[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize]);
			self.fVec56[(self.IOTA0 & 511) as usize] = fTemp727;
			let mut fTemp728: F64 = F64::min(fTemp727, self.fVec56[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize]);
			self.fVec57[(self.IOTA0 & 1023) as usize] = fTemp728;
			let mut fTemp729: F64 = F64::min(fTemp728, self.fVec57[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize]);
			self.fVec58[(self.IOTA0 & 2047) as usize] = fTemp729;
			let mut fTemp730: F64 = F64::min(fTemp729, self.fVec58[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			self.fVec59[(self.IOTA0 & 4095) as usize] = fTemp730;
			let mut fTemp731: F64 = F64::min(fTemp730, self.fVec59[((i32::wrapping_sub(self.IOTA0, 2048)) & 4095) as usize]);
			self.fVec60[(self.IOTA0 & 8191) as usize] = fTemp731;
			self.fVec61[(self.IOTA0 & 16383) as usize] = F64::min(fTemp731, self.fVec60[((i32::wrapping_sub(self.IOTA0, 4096)) & 8191) as usize]);
			let mut fTemp732: F64 = F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow4 != 0 {self.fRec17[0]} else {1.7976931348623157e+308}, if iSlow49 != 0 {self.fVec49[iSlow4 as usize]} else {1.7976931348623157e+308}), if iSlow50 != 0 {self.fVec50[iSlow51 as usize]} else {1.7976931348623157e+308}), if iSlow52 != 0 {self.fVec51[iSlow53 as usize]} else {1.7976931348623157e+308}), if iSlow54 != 0 {self.fVec52[((i32::wrapping_sub(self.IOTA0, iSlow55)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow56 != 0 {self.fVec53[((i32::wrapping_sub(self.IOTA0, iSlow57)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow58 != 0 {self.fVec54[((i32::wrapping_sub(self.IOTA0, iSlow59)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow60 != 0 {self.fVec55[((i32::wrapping_sub(self.IOTA0, iSlow61)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow62 != 0 {self.fVec56[((i32::wrapping_sub(self.IOTA0, iSlow63)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow64 != 0 {self.fVec57[((i32::wrapping_sub(self.IOTA0, iSlow65)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow66 != 0 {self.fVec58[((i32::wrapping_sub(self.IOTA0, iSlow67)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow68 != 0 {self.fVec59[((i32::wrapping_sub(self.IOTA0, iSlow69)) & 4095) as usize]} else {1.7976931348623157e+308}), if iSlow70 != 0 {self.fVec60[((i32::wrapping_sub(self.IOTA0, iSlow71)) & 8191) as usize]} else {1.7976931348623157e+308}), if iSlow72 != 0 {self.fVec61[((i32::wrapping_sub(self.IOTA0, iSlow73)) & 16383) as usize]} else {1.7976931348623157e+308}) - fTemp703;
			self.fVec62[0] = fTemp732;
			let mut iTemp733: i32 = (fTemp732 > 0.0) as i32;
			let mut fTemp734: F64 = if iTemp733 != 0 {fSlow75} else {fSlow74};
			self.fVec63[0] = fTemp734;
			let mut fTemp735: F64 = 2.0 * fTemp734;
			let mut iTemp736: i32 = (fTemp735) as i32;
			let mut iTemp737: i32 = std::cmp::max(0, std::cmp::min(iTemp736, 2));
			let mut iTemp738: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, 393213), 786431));
			let mut fTemp739: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp738, 3)) as usize] };
			let mut fTemp740: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp738 as usize] };
			let mut fTemp741: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp738, 1)) as usize] } - fTemp740;
			let mut fTemp742: F64 = fTemp735 - (iTemp736) as F64;
			let mut fTemp743: F64 = fTemp740 + fTemp742 * fTemp741 + 0.5 * (fTemp739 - (fTemp740 + fTemp742 * (fTemp741 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp738, 4)) as usize] } - fTemp739))));
			let mut fTemp744: F64 = if iTemp733 != 0 {fTemp743} else {1.0 - fTemp743};
			let mut iTemp745: i32 = (fTemp732 < 0.0) as i32;
			let mut fTemp746: F64 = fSlow1 * (iTemp745) as F64 + fSlow13 * (iTemp733) as F64;
			self.fVec64[0] = fTemp746;
			let mut fTemp747: F64 = self.fConst10 / fTemp746;
			let mut fTemp748: F64 = fTemp747 + 0.5;
			let mut fTemp749: F64 = 262143.0 * (1.0 - fTemp748);
			let mut iTemp750: i32 = (fTemp749) as i32;
			let mut iTemp751: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp750, 262143)))), 786431));
			let mut fTemp752: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp751, 3)) as usize] };
			let mut fTemp753: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp751 as usize] };
			let mut fTemp754: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp751, 1)) as usize] } - fTemp753;
			let mut fTemp755: F64 = 262143.0 * fTemp748;
			let mut iTemp756: i32 = (fTemp755) as i32;
			let mut iTemp757: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp756, 262143)))), 786431));
			let mut fTemp758: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp757, 3), 786431))) as usize] };
			let mut fTemp759: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp757 as usize] };
			let mut fTemp760: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp757, 1), 786431))) as usize] } - fTemp759;
			let mut fTemp761: F64 = 2.0 * self.fVec63[1];
			let mut iTemp762: i32 = (fTemp761) as i32;
			let mut iTemp763: i32 = std::cmp::max(0, std::cmp::min(iTemp762, 2));
			let mut fTemp764: F64 = 262143.0 * (1.0 - self.fRec15[1]);
			let mut iTemp765: i32 = (fTemp764) as i32;
			let mut iTemp766: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp765, 262143))), iTemp763), 786431));
			let mut fTemp767: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp766, 3), 786431))) as usize] };
			let mut fTemp768: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp766 as usize] };
			let mut fTemp769: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp766, 1), 786431))) as usize] } - fTemp768;
			let mut fTemp770: F64 = fTemp761 - (iTemp762) as F64;
			let mut fTemp771: F64 = 262143.0 * self.fRec15[1];
			let mut iTemp772: i32 = (fTemp771) as i32;
			let mut iTemp773: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp763, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp772, 262143)))), 786431));
			let mut fTemp774: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp773, 3), 786431))) as usize] };
			let mut fTemp775: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp773 as usize] };
			let mut fTemp776: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp773, 1), 786431))) as usize] } - fTemp775;
			let mut fTemp777: F64 = self.fRec15[1] + fTemp747;
			let mut fTemp778: F64 = 262143.0 * (1.0 - fTemp777);
			let mut iTemp779: i32 = (fTemp778) as i32;
			let mut iTemp780: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp779, 262143)))), 786431));
			let mut fTemp781: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp780, 3)) as usize] };
			let mut fTemp782: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp780 as usize] };
			let mut fTemp783: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp780, 1)) as usize] } - fTemp782;
			let mut fTemp784: F64 = 262143.0 * fTemp777;
			let mut iTemp785: i32 = (fTemp784) as i32;
			let mut iTemp786: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp785, 262143)))), 786431));
			let mut fTemp787: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp786, 3), 786431))) as usize] };
			let mut fTemp788: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp786 as usize] };
			let mut fTemp789: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp786, 1), 786431))) as usize] } - fTemp788;
			let mut fTemp790: F64 = self.fRec15[1] + self.fConst10 * (1.0 / fTemp746 + 1.0 / self.fVec64[1]);
			let mut fTemp791: F64 = 262143.0 * (1.0 - fTemp790);
			let mut iTemp792: i32 = (fTemp791) as i32;
			let mut iTemp793: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp792, 262143))), iTemp737), 786431));
			let mut fTemp794: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp793, 3)) as usize] };
			let mut fTemp795: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp793 as usize] };
			let mut fTemp796: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp793, 1)) as usize] } - fTemp795;
			let mut fTemp797: F64 = 262143.0 * fTemp790;
			let mut iTemp798: i32 = (fTemp797) as i32;
			let mut iTemp799: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp798, 262143)))), 786431));
			let mut fTemp800: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp799, 3), 786431))) as usize] };
			let mut fTemp801: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp799 as usize] };
			let mut fTemp802: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp799, 1), 786431))) as usize] } - fTemp801;
			let mut fTemp803: F64 = (if iTemp733 != 0 {fTemp801 + fTemp742 * fTemp802 + (fTemp797 - (iTemp798) as F64) * (fTemp800 - (fTemp801 + fTemp742 * (fTemp802 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp799, 4), 786431))) as usize] } - fTemp800))))} else {1.0 - (fTemp795 + fTemp742 * fTemp796 + (fTemp791 - (iTemp792) as F64) * (fTemp794 - (fTemp795 + fTemp742 * (fTemp796 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp793, 4)) as usize] } - fTemp794)))))} - if iTemp733 != 0 {fTemp788 + fTemp742 * fTemp789 + (fTemp784 - (iTemp785) as F64) * (fTemp787 - (fTemp788 + fTemp742 * (fTemp789 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp786, 4), 786431))) as usize] } - fTemp787))))} else {1.0 - (fTemp782 + fTemp742 * fTemp783 + (fTemp778 - (iTemp779) as F64) * (fTemp781 - (fTemp782 + fTemp742 * (fTemp783 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp780, 4)) as usize] } - fTemp781)))))}) * self.fVec62[1] / (fTemp732 * (1.0 - if iTemp733 != 0 {fTemp775 + fTemp770 * fTemp776 + (fTemp771 - (iTemp772) as F64) * (fTemp774 - (fTemp775 + fTemp770 * (fTemp776 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp773, 4), 786431))) as usize] } - fTemp774))))} else {1.0 - (fTemp768 + fTemp770 * fTemp769 + (fTemp764 - (iTemp765) as F64) * (fTemp767 - (fTemp768 + fTemp770 * (fTemp769 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp766, 4), 786431))) as usize] } - fTemp767)))))}));
			let mut iTemp804: i32 = (fTemp803 > ((if iTemp733 != 0 {fTemp759 + fTemp742 * fTemp760 + (fTemp755 - (iTemp756) as F64) * (fTemp758 - (fTemp759 + fTemp742 * (fTemp760 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp757, 4), 786431))) as usize] } - fTemp758))))} else {1.0 - (fTemp753 + fTemp742 * fTemp754 + (fTemp749 - (iTemp750) as F64) * (fTemp752 - (fTemp753 + fTemp742 * (fTemp754 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp751, 4)) as usize] } - fTemp752)))))} - fTemp744) / (1.0 - fTemp744))) as i32;
			let mut fTemp805: F64 = if iTemp804 != 0 {1.0} else {0.5};
			let mut fTemp806: F64 = if iTemp804 != 0 {0.5} else {0.0};
			let mut fTemp807: F64 = fTemp806 + fTemp805;
			let mut fTemp808: F64 = 0.5 * fTemp807;
			let mut fTemp809: F64 = 262143.0 * (1.0 - fTemp808);
			let mut iTemp810: i32 = (fTemp809) as i32;
			let mut iTemp811: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp810, 262143)))), 786431));
			let mut fTemp812: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp811, 3)) as usize] };
			let mut fTemp813: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp811 as usize] };
			let mut fTemp814: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp811, 1)) as usize] } - fTemp813;
			let mut fTemp815: F64 = 131071.5 * fTemp807;
			let mut iTemp816: i32 = (fTemp815) as i32;
			let mut iTemp817: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp816, 262143)))), 786431));
			let mut fTemp818: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp817, 3)) as usize] };
			let mut fTemp819: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp817 as usize] };
			let mut fTemp820: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp817, 1)) as usize] } - fTemp819;
			let mut fTemp821: F64 = if iTemp733 != 0 {fTemp819 + fTemp742 * fTemp820 + (fTemp815 - (iTemp816) as F64) * (fTemp818 - (fTemp819 + fTemp742 * (fTemp820 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp817, 4)) as usize] } - fTemp818))))} else {1.0 - (fTemp813 + fTemp742 * fTemp814 + (fTemp809 - (iTemp810) as F64) * (fTemp812 - (fTemp813 + fTemp742 * (fTemp814 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp811, 4)) as usize] } - fTemp812)))))};
			let mut fTemp822: F64 = fTemp747 + fTemp808;
			let mut fTemp823: F64 = 262143.0 * (1.0 - fTemp822);
			let mut iTemp824: i32 = (fTemp823) as i32;
			let mut iTemp825: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp824, 262143)))), 786431));
			let mut fTemp826: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp825, 3)) as usize] };
			let mut fTemp827: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp825 as usize] };
			let mut fTemp828: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp825, 1)) as usize] } - fTemp827;
			let mut fTemp829: F64 = 262143.0 * fTemp822;
			let mut iTemp830: i32 = (fTemp829) as i32;
			let mut iTemp831: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp830, 262143)))), 786431));
			let mut fTemp832: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp831, 3), 786431))) as usize] };
			let mut fTemp833: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp831 as usize] };
			let mut fTemp834: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp831, 1), 786431))) as usize] } - fTemp833;
			let mut iTemp835: i32 = (fTemp803 > ((if iTemp733 != 0 {fTemp833 + fTemp742 * fTemp834 + (fTemp829 - (iTemp830) as F64) * (fTemp832 - (fTemp833 + fTemp742 * (fTemp834 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp831, 4), 786431))) as usize] } - fTemp832))))} else {1.0 - (fTemp827 + fTemp742 * fTemp828 + (fTemp823 - (iTemp824) as F64) * (fTemp826 - (fTemp827 + fTemp742 * (fTemp828 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp825, 4)) as usize] } - fTemp826)))))} - fTemp821) / (1.0 - fTemp821))) as i32;
			let mut fTemp836: F64 = if iTemp835 != 0 {fTemp805} else {fTemp808};
			let mut fTemp837: F64 = if iTemp835 != 0 {fTemp808} else {fTemp806};
			let mut fTemp838: F64 = fTemp837 + fTemp836;
			let mut fTemp839: F64 = 0.5 * fTemp838;
			let mut fTemp840: F64 = 262143.0 * (1.0 - fTemp839);
			let mut iTemp841: i32 = (fTemp840) as i32;
			let mut iTemp842: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp841, 262143)))), 786431));
			let mut fTemp843: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp842, 3)) as usize] };
			let mut fTemp844: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp842 as usize] };
			let mut fTemp845: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp842, 1)) as usize] } - fTemp844;
			let mut fTemp846: F64 = 131071.5 * fTemp838;
			let mut iTemp847: i32 = (fTemp846) as i32;
			let mut iTemp848: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp847, 262143)))), 786431));
			let mut fTemp849: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp848, 3)) as usize] };
			let mut fTemp850: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp848 as usize] };
			let mut fTemp851: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp848, 1)) as usize] } - fTemp850;
			let mut fTemp852: F64 = if iTemp733 != 0 {fTemp850 + fTemp742 * fTemp851 + (fTemp846 - (iTemp847) as F64) * (fTemp849 - (fTemp850 + fTemp742 * (fTemp851 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp848, 4)) as usize] } - fTemp849))))} else {1.0 - (fTemp844 + fTemp742 * fTemp845 + (fTemp840 - (iTemp841) as F64) * (fTemp843 - (fTemp844 + fTemp742 * (fTemp845 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp842, 4)) as usize] } - fTemp843)))))};
			let mut fTemp853: F64 = fTemp747 + fTemp839;
			let mut fTemp854: F64 = 262143.0 * (1.0 - fTemp853);
			let mut iTemp855: i32 = (fTemp854) as i32;
			let mut iTemp856: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp855, 262143)))), 786431));
			let mut fTemp857: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp856, 3)) as usize] };
			let mut fTemp858: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp856 as usize] };
			let mut fTemp859: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp856, 1)) as usize] } - fTemp858;
			let mut fTemp860: F64 = 262143.0 * fTemp853;
			let mut iTemp861: i32 = (fTemp860) as i32;
			let mut iTemp862: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp861, 262143)))), 786431));
			let mut fTemp863: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp862, 3), 786431))) as usize] };
			let mut fTemp864: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp862 as usize] };
			let mut fTemp865: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp862, 1), 786431))) as usize] } - fTemp864;
			let mut iTemp866: i32 = (fTemp803 > ((if iTemp733 != 0 {fTemp864 + fTemp742 * fTemp865 + (fTemp860 - (iTemp861) as F64) * (fTemp863 - (fTemp864 + fTemp742 * (fTemp865 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp862, 4), 786431))) as usize] } - fTemp863))))} else {1.0 - (fTemp858 + fTemp742 * fTemp859 + (fTemp854 - (iTemp855) as F64) * (fTemp857 - (fTemp858 + fTemp742 * (fTemp859 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp856, 4)) as usize] } - fTemp857)))))} - fTemp852) / (1.0 - fTemp852))) as i32;
			let mut fTemp867: F64 = if iTemp866 != 0 {fTemp836} else {fTemp839};
			let mut fTemp868: F64 = if iTemp866 != 0 {fTemp839} else {fTemp837};
			let mut fTemp869: F64 = fTemp868 + fTemp867;
			let mut fTemp870: F64 = 0.5 * fTemp869;
			let mut fTemp871: F64 = 262143.0 * (1.0 - fTemp870);
			let mut iTemp872: i32 = (fTemp871) as i32;
			let mut iTemp873: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp872, 262143)))), 786431));
			let mut fTemp874: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp873, 3)) as usize] };
			let mut fTemp875: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp873 as usize] };
			let mut fTemp876: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp873, 1)) as usize] } - fTemp875;
			let mut fTemp877: F64 = 131071.5 * fTemp869;
			let mut iTemp878: i32 = (fTemp877) as i32;
			let mut iTemp879: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp878, 262143)))), 786431));
			let mut fTemp880: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp879, 3)) as usize] };
			let mut fTemp881: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp879 as usize] };
			let mut fTemp882: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp879, 1)) as usize] } - fTemp881;
			let mut fTemp883: F64 = if iTemp733 != 0 {fTemp881 + fTemp742 * fTemp882 + (fTemp877 - (iTemp878) as F64) * (fTemp880 - (fTemp881 + fTemp742 * (fTemp882 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp879, 4)) as usize] } - fTemp880))))} else {1.0 - (fTemp875 + fTemp742 * fTemp876 + (fTemp871 - (iTemp872) as F64) * (fTemp874 - (fTemp875 + fTemp742 * (fTemp876 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp873, 4)) as usize] } - fTemp874)))))};
			let mut fTemp884: F64 = fTemp747 + fTemp870;
			let mut fTemp885: F64 = 262143.0 * (1.0 - fTemp884);
			let mut iTemp886: i32 = (fTemp885) as i32;
			let mut iTemp887: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp886, 262143)))), 786431));
			let mut fTemp888: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp887, 3)) as usize] };
			let mut fTemp889: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp887 as usize] };
			let mut fTemp890: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp887, 1)) as usize] } - fTemp889;
			let mut fTemp891: F64 = 262143.0 * fTemp884;
			let mut iTemp892: i32 = (fTemp891) as i32;
			let mut iTemp893: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp892, 262143)))), 786431));
			let mut fTemp894: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp893, 3), 786431))) as usize] };
			let mut fTemp895: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp893 as usize] };
			let mut fTemp896: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp893, 1), 786431))) as usize] } - fTemp895;
			let mut iTemp897: i32 = (fTemp803 > ((if iTemp733 != 0 {fTemp895 + fTemp742 * fTemp896 + (fTemp891 - (iTemp892) as F64) * (fTemp894 - (fTemp895 + fTemp742 * (fTemp896 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp893, 4), 786431))) as usize] } - fTemp894))))} else {1.0 - (fTemp889 + fTemp742 * fTemp890 + (fTemp885 - (iTemp886) as F64) * (fTemp888 - (fTemp889 + fTemp742 * (fTemp890 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp887, 4)) as usize] } - fTemp888)))))} - fTemp883) / (1.0 - fTemp883))) as i32;
			let mut fTemp898: F64 = if iTemp897 != 0 {fTemp867} else {fTemp870};
			let mut fTemp899: F64 = if iTemp897 != 0 {fTemp870} else {fTemp868};
			let mut fTemp900: F64 = fTemp899 + fTemp898;
			let mut fTemp901: F64 = 0.5 * fTemp900;
			let mut fTemp902: F64 = 262143.0 * (1.0 - fTemp901);
			let mut iTemp903: i32 = (fTemp902) as i32;
			let mut iTemp904: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp903, 262143)))), 786431));
			let mut fTemp905: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp904, 3)) as usize] };
			let mut fTemp906: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp904 as usize] };
			let mut fTemp907: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp904, 1)) as usize] } - fTemp906;
			let mut fTemp908: F64 = 131071.5 * fTemp900;
			let mut iTemp909: i32 = (fTemp908) as i32;
			let mut iTemp910: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp909, 262143)))), 786431));
			let mut fTemp911: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp910, 3)) as usize] };
			let mut fTemp912: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp910 as usize] };
			let mut fTemp913: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp910, 1)) as usize] } - fTemp912;
			let mut fTemp914: F64 = if iTemp733 != 0 {fTemp912 + fTemp742 * fTemp913 + (fTemp908 - (iTemp909) as F64) * (fTemp911 - (fTemp912 + fTemp742 * (fTemp913 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp910, 4)) as usize] } - fTemp911))))} else {1.0 - (fTemp906 + fTemp742 * fTemp907 + (fTemp902 - (iTemp903) as F64) * (fTemp905 - (fTemp906 + fTemp742 * (fTemp907 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp904, 4)) as usize] } - fTemp905)))))};
			let mut fTemp915: F64 = fTemp747 + fTemp901;
			let mut fTemp916: F64 = 262143.0 * (1.0 - fTemp915);
			let mut iTemp917: i32 = (fTemp916) as i32;
			let mut iTemp918: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp917, 262143)))), 786431));
			let mut fTemp919: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp918, 3)) as usize] };
			let mut fTemp920: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp918 as usize] };
			let mut fTemp921: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp918, 1)) as usize] } - fTemp920;
			let mut fTemp922: F64 = 262143.0 * fTemp915;
			let mut iTemp923: i32 = (fTemp922) as i32;
			let mut iTemp924: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp923, 262143)))), 786431));
			let mut fTemp925: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp924, 3), 786431))) as usize] };
			let mut fTemp926: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp924 as usize] };
			let mut fTemp927: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp924, 1), 786431))) as usize] } - fTemp926;
			let mut iTemp928: i32 = (fTemp803 > ((if iTemp733 != 0 {fTemp926 + fTemp742 * fTemp927 + (fTemp922 - (iTemp923) as F64) * (fTemp925 - (fTemp926 + fTemp742 * (fTemp927 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp924, 4), 786431))) as usize] } - fTemp925))))} else {1.0 - (fTemp920 + fTemp742 * fTemp921 + (fTemp916 - (iTemp917) as F64) * (fTemp919 - (fTemp920 + fTemp742 * (fTemp921 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp918, 4)) as usize] } - fTemp919)))))} - fTemp914) / (1.0 - fTemp914))) as i32;
			let mut fTemp929: F64 = if iTemp928 != 0 {fTemp898} else {fTemp901};
			let mut fTemp930: F64 = if iTemp928 != 0 {fTemp901} else {fTemp899};
			let mut fTemp931: F64 = fTemp930 + fTemp929;
			let mut fTemp932: F64 = 0.5 * fTemp931;
			let mut fTemp933: F64 = 262143.0 * (1.0 - fTemp932);
			let mut iTemp934: i32 = (fTemp933) as i32;
			let mut iTemp935: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp934, 262143)))), 786431));
			let mut fTemp936: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp935, 3)) as usize] };
			let mut fTemp937: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp935 as usize] };
			let mut fTemp938: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp935, 1)) as usize] } - fTemp937;
			let mut fTemp939: F64 = 131071.5 * fTemp931;
			let mut iTemp940: i32 = (fTemp939) as i32;
			let mut iTemp941: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp940, 262143)))), 786431));
			let mut fTemp942: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp941, 3)) as usize] };
			let mut fTemp943: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp941 as usize] };
			let mut fTemp944: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp941, 1)) as usize] } - fTemp943;
			let mut fTemp945: F64 = if iTemp733 != 0 {fTemp943 + fTemp742 * fTemp944 + (fTemp939 - (iTemp940) as F64) * (fTemp942 - (fTemp943 + fTemp742 * (fTemp944 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp941, 4)) as usize] } - fTemp942))))} else {1.0 - (fTemp937 + fTemp742 * fTemp938 + (fTemp933 - (iTemp934) as F64) * (fTemp936 - (fTemp937 + fTemp742 * (fTemp938 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp935, 4)) as usize] } - fTemp936)))))};
			let mut fTemp946: F64 = fTemp747 + fTemp932;
			let mut fTemp947: F64 = 262143.0 * (1.0 - fTemp946);
			let mut iTemp948: i32 = (fTemp947) as i32;
			let mut iTemp949: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp948, 262143)))), 786431));
			let mut fTemp950: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp949, 3)) as usize] };
			let mut fTemp951: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp949 as usize] };
			let mut fTemp952: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp949, 1)) as usize] } - fTemp951;
			let mut fTemp953: F64 = 262143.0 * fTemp946;
			let mut iTemp954: i32 = (fTemp953) as i32;
			let mut iTemp955: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp954, 262143)))), 786431));
			let mut fTemp956: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp955, 3), 786431))) as usize] };
			let mut fTemp957: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp955 as usize] };
			let mut fTemp958: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp955, 1), 786431))) as usize] } - fTemp957;
			let mut iTemp959: i32 = (fTemp803 > ((if iTemp733 != 0 {fTemp957 + fTemp742 * fTemp958 + (fTemp953 - (iTemp954) as F64) * (fTemp956 - (fTemp957 + fTemp742 * (fTemp958 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp955, 4), 786431))) as usize] } - fTemp956))))} else {1.0 - (fTemp951 + fTemp742 * fTemp952 + (fTemp947 - (iTemp948) as F64) * (fTemp950 - (fTemp951 + fTemp742 * (fTemp952 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp949, 4)) as usize] } - fTemp950)))))} - fTemp945) / (1.0 - fTemp945))) as i32;
			let mut fTemp960: F64 = if iTemp959 != 0 {fTemp929} else {fTemp932};
			let mut fTemp961: F64 = if iTemp959 != 0 {fTemp932} else {fTemp930};
			let mut fTemp962: F64 = fTemp961 + fTemp960;
			let mut fTemp963: F64 = 0.5 * fTemp962;
			let mut fTemp964: F64 = 262143.0 * (1.0 - fTemp963);
			let mut iTemp965: i32 = (fTemp964) as i32;
			let mut iTemp966: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp965, 262143)))), 786431));
			let mut fTemp967: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp966, 3)) as usize] };
			let mut fTemp968: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp966 as usize] };
			let mut fTemp969: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp966, 1)) as usize] } - fTemp968;
			let mut fTemp970: F64 = 131071.5 * fTemp962;
			let mut iTemp971: i32 = (fTemp970) as i32;
			let mut iTemp972: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp971, 262143)))), 786431));
			let mut fTemp973: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp972, 3)) as usize] };
			let mut fTemp974: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp972 as usize] };
			let mut fTemp975: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp972, 1)) as usize] } - fTemp974;
			let mut fTemp976: F64 = if iTemp733 != 0 {fTemp974 + fTemp742 * fTemp975 + (fTemp970 - (iTemp971) as F64) * (fTemp973 - (fTemp974 + fTemp742 * (fTemp975 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp972, 4)) as usize] } - fTemp973))))} else {1.0 - (fTemp968 + fTemp742 * fTemp969 + (fTemp964 - (iTemp965) as F64) * (fTemp967 - (fTemp968 + fTemp742 * (fTemp969 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp966, 4)) as usize] } - fTemp967)))))};
			let mut fTemp977: F64 = fTemp747 + fTemp963;
			let mut fTemp978: F64 = 262143.0 * (1.0 - fTemp977);
			let mut iTemp979: i32 = (fTemp978) as i32;
			let mut iTemp980: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp979, 262143)))), 786431));
			let mut fTemp981: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp980, 3)) as usize] };
			let mut fTemp982: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp980 as usize] };
			let mut fTemp983: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp980, 1)) as usize] } - fTemp982;
			let mut fTemp984: F64 = 262143.0 * fTemp977;
			let mut iTemp985: i32 = (fTemp984) as i32;
			let mut iTemp986: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp985, 262143)))), 786431));
			let mut fTemp987: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp986, 3), 786431))) as usize] };
			let mut fTemp988: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp986 as usize] };
			let mut fTemp989: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp986, 1), 786431))) as usize] } - fTemp988;
			let mut iTemp990: i32 = (fTemp803 > ((if iTemp733 != 0 {fTemp988 + fTemp742 * fTemp989 + (fTemp984 - (iTemp985) as F64) * (fTemp987 - (fTemp988 + fTemp742 * (fTemp989 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp986, 4), 786431))) as usize] } - fTemp987))))} else {1.0 - (fTemp982 + fTemp742 * fTemp983 + (fTemp978 - (iTemp979) as F64) * (fTemp981 - (fTemp982 + fTemp742 * (fTemp983 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp980, 4)) as usize] } - fTemp981)))))} - fTemp976) / (1.0 - fTemp976))) as i32;
			let mut fTemp991: F64 = if iTemp990 != 0 {fTemp960} else {fTemp963};
			let mut fTemp992: F64 = if iTemp990 != 0 {fTemp963} else {fTemp961};
			let mut fTemp993: F64 = fTemp992 + fTemp991;
			let mut fTemp994: F64 = 0.5 * fTemp993;
			let mut fTemp995: F64 = 262143.0 * (1.0 - fTemp994);
			let mut iTemp996: i32 = (fTemp995) as i32;
			let mut iTemp997: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp996, 262143)))), 786431));
			let mut fTemp998: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp997, 3)) as usize] };
			let mut fTemp999: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp997 as usize] };
			let mut fTemp1000: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp997, 1)) as usize] } - fTemp999;
			let mut fTemp1001: F64 = 131071.5 * fTemp993;
			let mut iTemp1002: i32 = (fTemp1001) as i32;
			let mut iTemp1003: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1002, 262143)))), 786431));
			let mut fTemp1004: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1003, 3)) as usize] };
			let mut fTemp1005: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1003 as usize] };
			let mut fTemp1006: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1003, 1)) as usize] } - fTemp1005;
			let mut fTemp1007: F64 = if iTemp733 != 0 {fTemp1005 + fTemp742 * fTemp1006 + (fTemp1001 - (iTemp1002) as F64) * (fTemp1004 - (fTemp1005 + fTemp742 * (fTemp1006 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1003, 4)) as usize] } - fTemp1004))))} else {1.0 - (fTemp999 + fTemp742 * fTemp1000 + (fTemp995 - (iTemp996) as F64) * (fTemp998 - (fTemp999 + fTemp742 * (fTemp1000 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp997, 4)) as usize] } - fTemp998)))))};
			let mut fTemp1008: F64 = fTemp747 + fTemp994;
			let mut fTemp1009: F64 = 262143.0 * (1.0 - fTemp1008);
			let mut iTemp1010: i32 = (fTemp1009) as i32;
			let mut iTemp1011: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1010, 262143)))), 786431));
			let mut fTemp1012: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1011, 3)) as usize] };
			let mut fTemp1013: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1011 as usize] };
			let mut fTemp1014: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1011, 1)) as usize] } - fTemp1013;
			let mut fTemp1015: F64 = 262143.0 * fTemp1008;
			let mut iTemp1016: i32 = (fTemp1015) as i32;
			let mut iTemp1017: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1016, 262143)))), 786431));
			let mut fTemp1018: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1017, 3), 786431))) as usize] };
			let mut fTemp1019: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1017 as usize] };
			let mut fTemp1020: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1017, 1), 786431))) as usize] } - fTemp1019;
			let mut iTemp1021: i32 = (fTemp803 > ((if iTemp733 != 0 {fTemp1019 + fTemp742 * fTemp1020 + (fTemp1015 - (iTemp1016) as F64) * (fTemp1018 - (fTemp1019 + fTemp742 * (fTemp1020 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1017, 4), 786431))) as usize] } - fTemp1018))))} else {1.0 - (fTemp1013 + fTemp742 * fTemp1014 + (fTemp1009 - (iTemp1010) as F64) * (fTemp1012 - (fTemp1013 + fTemp742 * (fTemp1014 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1011, 4)) as usize] } - fTemp1012)))))} - fTemp1007) / (1.0 - fTemp1007))) as i32;
			let mut fTemp1022: F64 = if iTemp1021 != 0 {fTemp991} else {fTemp994};
			let mut fTemp1023: F64 = if iTemp1021 != 0 {fTemp994} else {fTemp992};
			let mut fTemp1024: F64 = fTemp1023 + fTemp1022;
			let mut fTemp1025: F64 = 0.5 * fTemp1024;
			let mut fTemp1026: F64 = 262143.0 * (1.0 - fTemp1025);
			let mut iTemp1027: i32 = (fTemp1026) as i32;
			let mut iTemp1028: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1027, 262143)))), 786431));
			let mut fTemp1029: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1028, 3)) as usize] };
			let mut fTemp1030: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1028 as usize] };
			let mut fTemp1031: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1028, 1)) as usize] } - fTemp1030;
			let mut fTemp1032: F64 = 131071.5 * fTemp1024;
			let mut iTemp1033: i32 = (fTemp1032) as i32;
			let mut iTemp1034: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1033, 262143)))), 786431));
			let mut fTemp1035: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1034, 3)) as usize] };
			let mut fTemp1036: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1034 as usize] };
			let mut fTemp1037: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1034, 1)) as usize] } - fTemp1036;
			let mut fTemp1038: F64 = if iTemp733 != 0 {fTemp1036 + fTemp742 * fTemp1037 + (fTemp1032 - (iTemp1033) as F64) * (fTemp1035 - (fTemp1036 + fTemp742 * (fTemp1037 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1034, 4)) as usize] } - fTemp1035))))} else {1.0 - (fTemp1030 + fTemp742 * fTemp1031 + (fTemp1026 - (iTemp1027) as F64) * (fTemp1029 - (fTemp1030 + fTemp742 * (fTemp1031 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1028, 4)) as usize] } - fTemp1029)))))};
			let mut fTemp1039: F64 = fTemp747 + fTemp1025;
			let mut fTemp1040: F64 = 262143.0 * (1.0 - fTemp1039);
			let mut iTemp1041: i32 = (fTemp1040) as i32;
			let mut iTemp1042: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1041, 262143)))), 786431));
			let mut fTemp1043: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1042, 3)) as usize] };
			let mut fTemp1044: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1042 as usize] };
			let mut fTemp1045: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1042, 1)) as usize] } - fTemp1044;
			let mut fTemp1046: F64 = 262143.0 * fTemp1039;
			let mut iTemp1047: i32 = (fTemp1046) as i32;
			let mut iTemp1048: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1047, 262143)))), 786431));
			let mut fTemp1049: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1048, 3), 786431))) as usize] };
			let mut fTemp1050: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1048 as usize] };
			let mut fTemp1051: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1048, 1), 786431))) as usize] } - fTemp1050;
			let mut iTemp1052: i32 = (fTemp803 > ((if iTemp733 != 0 {fTemp1050 + fTemp742 * fTemp1051 + (fTemp1046 - (iTemp1047) as F64) * (fTemp1049 - (fTemp1050 + fTemp742 * (fTemp1051 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1048, 4), 786431))) as usize] } - fTemp1049))))} else {1.0 - (fTemp1044 + fTemp742 * fTemp1045 + (fTemp1040 - (iTemp1041) as F64) * (fTemp1043 - (fTemp1044 + fTemp742 * (fTemp1045 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1042, 4)) as usize] } - fTemp1043)))))} - fTemp1038) / (1.0 - fTemp1038))) as i32;
			let mut fTemp1053: F64 = if iTemp1052 != 0 {fTemp1022} else {fTemp1025};
			let mut fTemp1054: F64 = if iTemp1052 != 0 {fTemp1025} else {fTemp1023};
			let mut fTemp1055: F64 = fTemp1054 + fTemp1053;
			let mut fTemp1056: F64 = 0.5 * fTemp1055;
			let mut fTemp1057: F64 = 262143.0 * (1.0 - fTemp1056);
			let mut iTemp1058: i32 = (fTemp1057) as i32;
			let mut iTemp1059: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1058, 262143)))), 786431));
			let mut fTemp1060: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1059, 3)) as usize] };
			let mut fTemp1061: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1059 as usize] };
			let mut fTemp1062: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1059, 1)) as usize] } - fTemp1061;
			let mut fTemp1063: F64 = 131071.5 * fTemp1055;
			let mut iTemp1064: i32 = (fTemp1063) as i32;
			let mut iTemp1065: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1064, 262143)))), 786431));
			let mut fTemp1066: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1065, 3)) as usize] };
			let mut fTemp1067: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1065 as usize] };
			let mut fTemp1068: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1065, 1)) as usize] } - fTemp1067;
			let mut fTemp1069: F64 = if iTemp733 != 0 {fTemp1067 + fTemp742 * fTemp1068 + (fTemp1063 - (iTemp1064) as F64) * (fTemp1066 - (fTemp1067 + fTemp742 * (fTemp1068 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1065, 4)) as usize] } - fTemp1066))))} else {1.0 - (fTemp1061 + fTemp742 * fTemp1062 + (fTemp1057 - (iTemp1058) as F64) * (fTemp1060 - (fTemp1061 + fTemp742 * (fTemp1062 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1059, 4)) as usize] } - fTemp1060)))))};
			let mut fTemp1070: F64 = fTemp747 + fTemp1056;
			let mut fTemp1071: F64 = 262143.0 * (1.0 - fTemp1070);
			let mut iTemp1072: i32 = (fTemp1071) as i32;
			let mut iTemp1073: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1072, 262143)))), 786431));
			let mut fTemp1074: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1073, 3)) as usize] };
			let mut fTemp1075: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1073 as usize] };
			let mut fTemp1076: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1073, 1)) as usize] } - fTemp1075;
			let mut fTemp1077: F64 = 262143.0 * fTemp1070;
			let mut iTemp1078: i32 = (fTemp1077) as i32;
			let mut iTemp1079: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1078, 262143)))), 786431));
			let mut fTemp1080: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1079, 3), 786431))) as usize] };
			let mut fTemp1081: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1079 as usize] };
			let mut fTemp1082: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1079, 1), 786431))) as usize] } - fTemp1081;
			let mut iTemp1083: i32 = (fTemp803 > ((if iTemp733 != 0 {fTemp1081 + fTemp742 * fTemp1082 + (fTemp1077 - (iTemp1078) as F64) * (fTemp1080 - (fTemp1081 + fTemp742 * (fTemp1082 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1079, 4), 786431))) as usize] } - fTemp1080))))} else {1.0 - (fTemp1075 + fTemp742 * fTemp1076 + (fTemp1071 - (iTemp1072) as F64) * (fTemp1074 - (fTemp1075 + fTemp742 * (fTemp1076 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1073, 4)) as usize] } - fTemp1074)))))} - fTemp1069) / (1.0 - fTemp1069))) as i32;
			let mut fTemp1084: F64 = if iTemp1083 != 0 {fTemp1053} else {fTemp1056};
			let mut fTemp1085: F64 = if iTemp1083 != 0 {fTemp1056} else {fTemp1054};
			let mut fTemp1086: F64 = fTemp1085 + fTemp1084;
			let mut fTemp1087: F64 = 0.5 * fTemp1086;
			let mut fTemp1088: F64 = 262143.0 * (1.0 - fTemp1087);
			let mut iTemp1089: i32 = (fTemp1088) as i32;
			let mut iTemp1090: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1089, 262143)))), 786431));
			let mut fTemp1091: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1090, 3)) as usize] };
			let mut fTemp1092: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1090 as usize] };
			let mut fTemp1093: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1090, 1)) as usize] } - fTemp1092;
			let mut fTemp1094: F64 = 131071.5 * fTemp1086;
			let mut iTemp1095: i32 = (fTemp1094) as i32;
			let mut iTemp1096: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1095, 262143)))), 786431));
			let mut fTemp1097: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1096, 3)) as usize] };
			let mut fTemp1098: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1096 as usize] };
			let mut fTemp1099: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1096, 1)) as usize] } - fTemp1098;
			let mut fTemp1100: F64 = if iTemp733 != 0 {fTemp1098 + fTemp742 * fTemp1099 + (fTemp1094 - (iTemp1095) as F64) * (fTemp1097 - (fTemp1098 + fTemp742 * (fTemp1099 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1096, 4)) as usize] } - fTemp1097))))} else {1.0 - (fTemp1092 + fTemp742 * fTemp1093 + (fTemp1088 - (iTemp1089) as F64) * (fTemp1091 - (fTemp1092 + fTemp742 * (fTemp1093 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1090, 4)) as usize] } - fTemp1091)))))};
			let mut fTemp1101: F64 = fTemp747 + fTemp1087;
			let mut fTemp1102: F64 = 262143.0 * (1.0 - fTemp1101);
			let mut iTemp1103: i32 = (fTemp1102) as i32;
			let mut iTemp1104: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1103, 262143)))), 786431));
			let mut fTemp1105: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1104, 3)) as usize] };
			let mut fTemp1106: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1104 as usize] };
			let mut fTemp1107: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1104, 1)) as usize] } - fTemp1106;
			let mut fTemp1108: F64 = 262143.0 * fTemp1101;
			let mut iTemp1109: i32 = (fTemp1108) as i32;
			let mut iTemp1110: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1109, 262143)))), 786431));
			let mut fTemp1111: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1110, 3), 786431))) as usize] };
			let mut fTemp1112: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1110 as usize] };
			let mut fTemp1113: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1110, 1), 786431))) as usize] } - fTemp1112;
			let mut iTemp1114: i32 = (fTemp803 > ((if iTemp733 != 0 {fTemp1112 + fTemp742 * fTemp1113 + (fTemp1108 - (iTemp1109) as F64) * (fTemp1111 - (fTemp1112 + fTemp742 * (fTemp1113 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1110, 4), 786431))) as usize] } - fTemp1111))))} else {1.0 - (fTemp1106 + fTemp742 * fTemp1107 + (fTemp1102 - (iTemp1103) as F64) * (fTemp1105 - (fTemp1106 + fTemp742 * (fTemp1107 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1104, 4)) as usize] } - fTemp1105)))))} - fTemp1100) / (1.0 - fTemp1100))) as i32;
			let mut fTemp1115: F64 = if iTemp1114 != 0 {fTemp1084} else {fTemp1087};
			let mut fTemp1116: F64 = if iTemp1114 != 0 {fTemp1087} else {fTemp1085};
			let mut fTemp1117: F64 = fTemp1116 + fTemp1115;
			let mut fTemp1118: F64 = 0.5 * fTemp1117;
			let mut fTemp1119: F64 = 262143.0 * (1.0 - fTemp1118);
			let mut iTemp1120: i32 = (fTemp1119) as i32;
			let mut iTemp1121: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1120, 262143)))), 786431));
			let mut fTemp1122: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1121, 3)) as usize] };
			let mut fTemp1123: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1121 as usize] };
			let mut fTemp1124: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1121, 1)) as usize] } - fTemp1123;
			let mut fTemp1125: F64 = 131071.5 * fTemp1117;
			let mut iTemp1126: i32 = (fTemp1125) as i32;
			let mut iTemp1127: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1126, 262143)))), 786431));
			let mut fTemp1128: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1127, 3)) as usize] };
			let mut fTemp1129: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1127 as usize] };
			let mut fTemp1130: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1127, 1)) as usize] } - fTemp1129;
			let mut fTemp1131: F64 = if iTemp733 != 0 {fTemp1129 + fTemp742 * fTemp1130 + (fTemp1125 - (iTemp1126) as F64) * (fTemp1128 - (fTemp1129 + fTemp742 * (fTemp1130 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1127, 4)) as usize] } - fTemp1128))))} else {1.0 - (fTemp1123 + fTemp742 * fTemp1124 + (fTemp1119 - (iTemp1120) as F64) * (fTemp1122 - (fTemp1123 + fTemp742 * (fTemp1124 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1121, 4)) as usize] } - fTemp1122)))))};
			let mut fTemp1132: F64 = fTemp747 + fTemp1118;
			let mut fTemp1133: F64 = 262143.0 * (1.0 - fTemp1132);
			let mut iTemp1134: i32 = (fTemp1133) as i32;
			let mut iTemp1135: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1134, 262143)))), 786431));
			let mut fTemp1136: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1135, 3)) as usize] };
			let mut fTemp1137: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1135 as usize] };
			let mut fTemp1138: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1135, 1)) as usize] } - fTemp1137;
			let mut fTemp1139: F64 = 262143.0 * fTemp1132;
			let mut iTemp1140: i32 = (fTemp1139) as i32;
			let mut iTemp1141: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1140, 262143)))), 786431));
			let mut fTemp1142: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1141, 3), 786431))) as usize] };
			let mut fTemp1143: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1141 as usize] };
			let mut fTemp1144: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1141, 1), 786431))) as usize] } - fTemp1143;
			let mut iTemp1145: i32 = (fTemp803 > ((if iTemp733 != 0 {fTemp1143 + fTemp742 * fTemp1144 + (fTemp1139 - (iTemp1140) as F64) * (fTemp1142 - (fTemp1143 + fTemp742 * (fTemp1144 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1141, 4), 786431))) as usize] } - fTemp1142))))} else {1.0 - (fTemp1137 + fTemp742 * fTemp1138 + (fTemp1133 - (iTemp1134) as F64) * (fTemp1136 - (fTemp1137 + fTemp742 * (fTemp1138 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1135, 4)) as usize] } - fTemp1136)))))} - fTemp1131) / (1.0 - fTemp1131))) as i32;
			let mut fTemp1146: F64 = if iTemp1145 != 0 {fTemp1115} else {fTemp1118};
			let mut fTemp1147: F64 = if iTemp1145 != 0 {fTemp1118} else {fTemp1116};
			let mut fTemp1148: F64 = fTemp1147 + fTemp1146;
			let mut fTemp1149: F64 = 0.5 * fTemp1148;
			let mut fTemp1150: F64 = 262143.0 * (1.0 - fTemp1149);
			let mut iTemp1151: i32 = (fTemp1150) as i32;
			let mut iTemp1152: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1151, 262143)))), 786431));
			let mut fTemp1153: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1152, 3)) as usize] };
			let mut fTemp1154: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1152 as usize] };
			let mut fTemp1155: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1152, 1)) as usize] } - fTemp1154;
			let mut fTemp1156: F64 = 131071.5 * fTemp1148;
			let mut iTemp1157: i32 = (fTemp1156) as i32;
			let mut iTemp1158: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1157, 262143)))), 786431));
			let mut fTemp1159: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1158, 3)) as usize] };
			let mut fTemp1160: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1158 as usize] };
			let mut fTemp1161: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1158, 1)) as usize] } - fTemp1160;
			let mut fTemp1162: F64 = if iTemp733 != 0 {fTemp1160 + fTemp742 * fTemp1161 + (fTemp1156 - (iTemp1157) as F64) * (fTemp1159 - (fTemp1160 + fTemp742 * (fTemp1161 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1158, 4)) as usize] } - fTemp1159))))} else {1.0 - (fTemp1154 + fTemp742 * fTemp1155 + (fTemp1150 - (iTemp1151) as F64) * (fTemp1153 - (fTemp1154 + fTemp742 * (fTemp1155 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1152, 4)) as usize] } - fTemp1153)))))};
			let mut fTemp1163: F64 = fTemp747 + fTemp1149;
			let mut fTemp1164: F64 = 262143.0 * (1.0 - fTemp1163);
			let mut iTemp1165: i32 = (fTemp1164) as i32;
			let mut iTemp1166: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1165, 262143)))), 786431));
			let mut fTemp1167: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1166, 3)) as usize] };
			let mut fTemp1168: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1166 as usize] };
			let mut fTemp1169: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1166, 1)) as usize] } - fTemp1168;
			let mut fTemp1170: F64 = 262143.0 * fTemp1163;
			let mut iTemp1171: i32 = (fTemp1170) as i32;
			let mut iTemp1172: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1171, 262143)))), 786431));
			let mut fTemp1173: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1172, 3), 786431))) as usize] };
			let mut fTemp1174: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1172 as usize] };
			let mut fTemp1175: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1172, 1), 786431))) as usize] } - fTemp1174;
			let mut iTemp1176: i32 = (fTemp803 > ((if iTemp733 != 0 {fTemp1174 + fTemp742 * fTemp1175 + (fTemp1170 - (iTemp1171) as F64) * (fTemp1173 - (fTemp1174 + fTemp742 * (fTemp1175 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1172, 4), 786431))) as usize] } - fTemp1173))))} else {1.0 - (fTemp1168 + fTemp742 * fTemp1169 + (fTemp1164 - (iTemp1165) as F64) * (fTemp1167 - (fTemp1168 + fTemp742 * (fTemp1169 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1166, 4)) as usize] } - fTemp1167)))))} - fTemp1162) / (1.0 - fTemp1162))) as i32;
			let mut fTemp1177: F64 = if iTemp1176 != 0 {fTemp1146} else {fTemp1149};
			let mut fTemp1178: F64 = if iTemp1176 != 0 {fTemp1149} else {fTemp1147};
			let mut fTemp1179: F64 = fTemp1178 + fTemp1177;
			let mut fTemp1180: F64 = 0.5 * fTemp1179;
			let mut fTemp1181: F64 = 262143.0 * (1.0 - fTemp1180);
			let mut iTemp1182: i32 = (fTemp1181) as i32;
			let mut iTemp1183: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1182, 262143)))), 786431));
			let mut fTemp1184: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1183, 3)) as usize] };
			let mut fTemp1185: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1183 as usize] };
			let mut fTemp1186: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1183, 1)) as usize] } - fTemp1185;
			let mut fTemp1187: F64 = 131071.5 * fTemp1179;
			let mut iTemp1188: i32 = (fTemp1187) as i32;
			let mut iTemp1189: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1188, 262143)))), 786431));
			let mut fTemp1190: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1189, 3)) as usize] };
			let mut fTemp1191: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1189 as usize] };
			let mut fTemp1192: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1189, 1)) as usize] } - fTemp1191;
			let mut fTemp1193: F64 = if iTemp733 != 0 {fTemp1191 + fTemp742 * fTemp1192 + (fTemp1187 - (iTemp1188) as F64) * (fTemp1190 - (fTemp1191 + fTemp742 * (fTemp1192 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1189, 4)) as usize] } - fTemp1190))))} else {1.0 - (fTemp1185 + fTemp742 * fTemp1186 + (fTemp1181 - (iTemp1182) as F64) * (fTemp1184 - (fTemp1185 + fTemp742 * (fTemp1186 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1183, 4)) as usize] } - fTemp1184)))))};
			let mut fTemp1194: F64 = fTemp747 + fTemp1180;
			let mut fTemp1195: F64 = 262143.0 * (1.0 - fTemp1194);
			let mut iTemp1196: i32 = (fTemp1195) as i32;
			let mut iTemp1197: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1196, 262143)))), 786431));
			let mut fTemp1198: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1197, 3)) as usize] };
			let mut fTemp1199: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1197 as usize] };
			let mut fTemp1200: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1197, 1)) as usize] } - fTemp1199;
			let mut fTemp1201: F64 = 262143.0 * fTemp1194;
			let mut iTemp1202: i32 = (fTemp1201) as i32;
			let mut iTemp1203: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1202, 262143)))), 786431));
			let mut fTemp1204: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1203, 3), 786431))) as usize] };
			let mut fTemp1205: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1203 as usize] };
			let mut fTemp1206: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1203, 1), 786431))) as usize] } - fTemp1205;
			let mut iTemp1207: i32 = (fTemp803 > ((if iTemp733 != 0 {fTemp1205 + fTemp742 * fTemp1206 + (fTemp1201 - (iTemp1202) as F64) * (fTemp1204 - (fTemp1205 + fTemp742 * (fTemp1206 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1203, 4), 786431))) as usize] } - fTemp1204))))} else {1.0 - (fTemp1199 + fTemp742 * fTemp1200 + (fTemp1195 - (iTemp1196) as F64) * (fTemp1198 - (fTemp1199 + fTemp742 * (fTemp1200 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1197, 4)) as usize] } - fTemp1198)))))} - fTemp1193) / (1.0 - fTemp1193))) as i32;
			let mut fTemp1208: F64 = if iTemp1207 != 0 {fTemp1177} else {fTemp1180};
			let mut fTemp1209: F64 = if iTemp1207 != 0 {fTemp1180} else {fTemp1178};
			let mut fTemp1210: F64 = fTemp1209 + fTemp1208;
			let mut fTemp1211: F64 = 0.5 * fTemp1210;
			let mut fTemp1212: F64 = 262143.0 * (1.0 - fTemp1211);
			let mut iTemp1213: i32 = (fTemp1212) as i32;
			let mut iTemp1214: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1213, 262143)))), 786431));
			let mut fTemp1215: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1214, 3)) as usize] };
			let mut fTemp1216: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1214 as usize] };
			let mut fTemp1217: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1214, 1)) as usize] } - fTemp1216;
			let mut fTemp1218: F64 = 131071.5 * fTemp1210;
			let mut iTemp1219: i32 = (fTemp1218) as i32;
			let mut iTemp1220: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1219, 262143)))), 786431));
			let mut fTemp1221: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1220, 3)) as usize] };
			let mut fTemp1222: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1220 as usize] };
			let mut fTemp1223: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1220, 1)) as usize] } - fTemp1222;
			let mut fTemp1224: F64 = if iTemp733 != 0 {fTemp1222 + fTemp742 * fTemp1223 + (fTemp1218 - (iTemp1219) as F64) * (fTemp1221 - (fTemp1222 + fTemp742 * (fTemp1223 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1220, 4)) as usize] } - fTemp1221))))} else {1.0 - (fTemp1216 + fTemp742 * fTemp1217 + (fTemp1212 - (iTemp1213) as F64) * (fTemp1215 - (fTemp1216 + fTemp742 * (fTemp1217 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1214, 4)) as usize] } - fTemp1215)))))};
			let mut fTemp1225: F64 = fTemp747 + fTemp1211;
			let mut fTemp1226: F64 = 262143.0 * (1.0 - fTemp1225);
			let mut iTemp1227: i32 = (fTemp1226) as i32;
			let mut iTemp1228: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1227, 262143)))), 786431));
			let mut fTemp1229: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1228, 3)) as usize] };
			let mut fTemp1230: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1228 as usize] };
			let mut fTemp1231: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1228, 1)) as usize] } - fTemp1230;
			let mut fTemp1232: F64 = 262143.0 * fTemp1225;
			let mut iTemp1233: i32 = (fTemp1232) as i32;
			let mut iTemp1234: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1233, 262143)))), 786431));
			let mut fTemp1235: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1234, 3), 786431))) as usize] };
			let mut fTemp1236: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1234 as usize] };
			let mut fTemp1237: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1234, 1), 786431))) as usize] } - fTemp1236;
			let mut iTemp1238: i32 = (fTemp803 > ((if iTemp733 != 0 {fTemp1236 + fTemp742 * fTemp1237 + (fTemp1232 - (iTemp1233) as F64) * (fTemp1235 - (fTemp1236 + fTemp742 * (fTemp1237 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1234, 4), 786431))) as usize] } - fTemp1235))))} else {1.0 - (fTemp1230 + fTemp742 * fTemp1231 + (fTemp1226 - (iTemp1227) as F64) * (fTemp1229 - (fTemp1230 + fTemp742 * (fTemp1231 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1228, 4)) as usize] } - fTemp1229)))))} - fTemp1224) / (1.0 - fTemp1224))) as i32;
			let mut fTemp1239: F64 = if iTemp1238 != 0 {fTemp1208} else {fTemp1211};
			let mut fTemp1240: F64 = if iTemp1238 != 0 {fTemp1211} else {fTemp1209};
			let mut fTemp1241: F64 = fTemp1240 + fTemp1239;
			let mut fTemp1242: F64 = 0.5 * fTemp1241;
			let mut fTemp1243: F64 = 262143.0 * (1.0 - fTemp1242);
			let mut iTemp1244: i32 = (fTemp1243) as i32;
			let mut iTemp1245: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1244, 262143)))), 786431));
			let mut fTemp1246: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1245, 3)) as usize] };
			let mut fTemp1247: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1245 as usize] };
			let mut fTemp1248: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1245, 1)) as usize] } - fTemp1247;
			let mut fTemp1249: F64 = 131071.5 * fTemp1241;
			let mut iTemp1250: i32 = (fTemp1249) as i32;
			let mut iTemp1251: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1250, 262143)))), 786431));
			let mut fTemp1252: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1251, 3)) as usize] };
			let mut fTemp1253: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1251 as usize] };
			let mut fTemp1254: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1251, 1)) as usize] } - fTemp1253;
			let mut fTemp1255: F64 = if iTemp733 != 0 {fTemp1253 + fTemp742 * fTemp1254 + (fTemp1249 - (iTemp1250) as F64) * (fTemp1252 - (fTemp1253 + fTemp742 * (fTemp1254 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1251, 4)) as usize] } - fTemp1252))))} else {1.0 - (fTemp1247 + fTemp742 * fTemp1248 + (fTemp1243 - (iTemp1244) as F64) * (fTemp1246 - (fTemp1247 + fTemp742 * (fTemp1248 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1245, 4)) as usize] } - fTemp1246)))))};
			let mut fTemp1256: F64 = fTemp747 + fTemp1242;
			let mut fTemp1257: F64 = 262143.0 * (1.0 - fTemp1256);
			let mut iTemp1258: i32 = (fTemp1257) as i32;
			let mut iTemp1259: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1258, 262143)))), 786431));
			let mut fTemp1260: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1259, 3)) as usize] };
			let mut fTemp1261: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1259 as usize] };
			let mut fTemp1262: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1259, 1)) as usize] } - fTemp1261;
			let mut fTemp1263: F64 = 262143.0 * fTemp1256;
			let mut iTemp1264: i32 = (fTemp1263) as i32;
			let mut iTemp1265: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1264, 262143)))), 786431));
			let mut fTemp1266: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1265, 3), 786431))) as usize] };
			let mut fTemp1267: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1265 as usize] };
			let mut fTemp1268: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1265, 1), 786431))) as usize] } - fTemp1267;
			let mut iTemp1269: i32 = (fTemp803 > ((if iTemp733 != 0 {fTemp1267 + fTemp742 * fTemp1268 + (fTemp1263 - (iTemp1264) as F64) * (fTemp1266 - (fTemp1267 + fTemp742 * (fTemp1268 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1265, 4), 786431))) as usize] } - fTemp1266))))} else {1.0 - (fTemp1261 + fTemp742 * fTemp1262 + (fTemp1257 - (iTemp1258) as F64) * (fTemp1260 - (fTemp1261 + fTemp742 * (fTemp1262 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1259, 4)) as usize] } - fTemp1260)))))} - fTemp1255) / (1.0 - fTemp1255))) as i32;
			let mut fTemp1270: F64 = if iTemp1269 != 0 {fTemp1239} else {fTemp1242};
			let mut fTemp1271: F64 = if iTemp1269 != 0 {fTemp1242} else {fTemp1240};
			let mut fTemp1272: F64 = fTemp1271 + fTemp1270;
			let mut fTemp1273: F64 = 0.5 * fTemp1272;
			let mut fTemp1274: F64 = 262143.0 * (1.0 - fTemp1273);
			let mut iTemp1275: i32 = (fTemp1274) as i32;
			let mut iTemp1276: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1275, 262143)))), 786431));
			let mut fTemp1277: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1276, 3)) as usize] };
			let mut fTemp1278: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1276 as usize] };
			let mut fTemp1279: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1276, 1)) as usize] } - fTemp1278;
			let mut fTemp1280: F64 = 131071.5 * fTemp1272;
			let mut iTemp1281: i32 = (fTemp1280) as i32;
			let mut iTemp1282: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1281, 262143)))), 786431));
			let mut fTemp1283: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1282, 3)) as usize] };
			let mut fTemp1284: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1282 as usize] };
			let mut fTemp1285: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1282, 1)) as usize] } - fTemp1284;
			let mut fTemp1286: F64 = if iTemp733 != 0 {fTemp1284 + fTemp742 * fTemp1285 + (fTemp1280 - (iTemp1281) as F64) * (fTemp1283 - (fTemp1284 + fTemp742 * (fTemp1285 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1282, 4)) as usize] } - fTemp1283))))} else {1.0 - (fTemp1278 + fTemp742 * fTemp1279 + (fTemp1274 - (iTemp1275) as F64) * (fTemp1277 - (fTemp1278 + fTemp742 * (fTemp1279 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1276, 4)) as usize] } - fTemp1277)))))};
			let mut fTemp1287: F64 = fTemp747 + fTemp1273;
			let mut fTemp1288: F64 = 262143.0 * (1.0 - fTemp1287);
			let mut iTemp1289: i32 = (fTemp1288) as i32;
			let mut iTemp1290: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1289, 262143)))), 786431));
			let mut fTemp1291: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1290, 3)) as usize] };
			let mut fTemp1292: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1290 as usize] };
			let mut fTemp1293: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1290, 1)) as usize] } - fTemp1292;
			let mut fTemp1294: F64 = 262143.0 * fTemp1287;
			let mut iTemp1295: i32 = (fTemp1294) as i32;
			let mut iTemp1296: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1295, 262143)))), 786431));
			let mut fTemp1297: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1296, 3), 786431))) as usize] };
			let mut fTemp1298: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1296 as usize] };
			let mut fTemp1299: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1296, 1), 786431))) as usize] } - fTemp1298;
			let mut iTemp1300: i32 = (fTemp803 > ((if iTemp733 != 0 {fTemp1298 + fTemp742 * fTemp1299 + (fTemp1294 - (iTemp1295) as F64) * (fTemp1297 - (fTemp1298 + fTemp742 * (fTemp1299 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1296, 4), 786431))) as usize] } - fTemp1297))))} else {1.0 - (fTemp1292 + fTemp742 * fTemp1293 + (fTemp1288 - (iTemp1289) as F64) * (fTemp1291 - (fTemp1292 + fTemp742 * (fTemp1293 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1290, 4)) as usize] } - fTemp1291)))))} - fTemp1286) / (1.0 - fTemp1286))) as i32;
			let mut fTemp1301: F64 = if iTemp1300 != 0 {fTemp1270} else {fTemp1273};
			let mut fTemp1302: F64 = if iTemp1300 != 0 {fTemp1273} else {fTemp1271};
			let mut fTemp1303: F64 = fTemp1302 + fTemp1301;
			let mut fTemp1304: F64 = 0.5 * fTemp1303;
			let mut fTemp1305: F64 = 262143.0 * (1.0 - fTemp1304);
			let mut iTemp1306: i32 = (fTemp1305) as i32;
			let mut iTemp1307: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1306, 262143)))), 786431));
			let mut fTemp1308: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1307, 3)) as usize] };
			let mut fTemp1309: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1307 as usize] };
			let mut fTemp1310: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1307, 1)) as usize] } - fTemp1309;
			let mut fTemp1311: F64 = 131071.5 * fTemp1303;
			let mut iTemp1312: i32 = (fTemp1311) as i32;
			let mut iTemp1313: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1312, 262143)))), 786431));
			let mut fTemp1314: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1313, 3)) as usize] };
			let mut fTemp1315: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1313 as usize] };
			let mut fTemp1316: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1313, 1)) as usize] } - fTemp1315;
			let mut fTemp1317: F64 = if iTemp733 != 0 {fTemp1315 + fTemp742 * fTemp1316 + (fTemp1311 - (iTemp1312) as F64) * (fTemp1314 - (fTemp1315 + fTemp742 * (fTemp1316 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1313, 4), 786431))) as usize] } - fTemp1314))))} else {1.0 - (fTemp1309 + fTemp742 * fTemp1310 + (fTemp1305 - (iTemp1306) as F64) * (fTemp1308 - (fTemp1309 + fTemp742 * (fTemp1310 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1307, 4), 786431))) as usize] } - fTemp1308)))))};
			let mut fTemp1318: F64 = fTemp747 + fTemp1304;
			let mut fTemp1319: F64 = 262143.0 * (1.0 - fTemp1318);
			let mut iTemp1320: i32 = (fTemp1319) as i32;
			let mut iTemp1321: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1320, 262143)))), 786431));
			let mut fTemp1322: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1321, 3)) as usize] };
			let mut fTemp1323: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1321 as usize] };
			let mut fTemp1324: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1321, 1)) as usize] } - fTemp1323;
			let mut fTemp1325: F64 = 262143.0 * fTemp1318;
			let mut iTemp1326: i32 = (fTemp1325) as i32;
			let mut iTemp1327: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1326, 262143)))), 786431));
			let mut fTemp1328: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1327, 3), 786431))) as usize] };
			let mut fTemp1329: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1327 as usize] };
			let mut fTemp1330: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1327, 1), 786431))) as usize] } - fTemp1329;
			let mut iTemp1331: i32 = (fTemp803 > ((if iTemp733 != 0 {fTemp1329 + fTemp742 * fTemp1330 + (fTemp1325 - (iTemp1326) as F64) * (fTemp1328 - (fTemp1329 + fTemp742 * (fTemp1330 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1327, 4), 786431))) as usize] } - fTemp1328))))} else {1.0 - (fTemp1323 + fTemp742 * fTemp1324 + (fTemp1319 - (iTemp1320) as F64) * (fTemp1322 - (fTemp1323 + fTemp742 * (fTemp1324 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1321, 4)) as usize] } - fTemp1322)))))} - fTemp1317) / (1.0 - fTemp1317))) as i32;
			let mut fTemp1332: F64 = F64::min(1.0, F64::max(0.0, 0.5 * (if iTemp1331 != 0 {fTemp1304} else {fTemp1302} + if iTemp1331 != 0 {fTemp1301} else {fTemp1304})));
			self.fRec15[0] = fTemp1332;
			let mut fTemp1333: F64 = 262143.0 * (1.0 - fTemp1332);
			let mut iTemp1334: i32 = (fTemp1333) as i32;
			let mut iTemp1335: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1334, 262143)))), 786431));
			let mut fTemp1336: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1335, 3)) as usize] };
			let mut fTemp1337: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1335 as usize] };
			let mut fTemp1338: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1335, 1)) as usize] } - fTemp1337;
			let mut fTemp1339: F64 = 262143.0 * fTemp1332;
			let mut iTemp1340: i32 = (fTemp1339) as i32;
			let mut iTemp1341: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1340, 262143)))), 786431));
			let mut fTemp1342: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1341, 3)) as usize] };
			let mut fTemp1343: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1341 as usize] };
			let mut fTemp1344: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1341, 1)) as usize] } - fTemp1343;
			let mut fTemp1345: F64 = if iTemp733 != 0 {fTemp1343 + fTemp742 * fTemp1344 + (fTemp1339 - (iTemp1340) as F64) * (fTemp1342 - (fTemp1343 + fTemp742 * (fTemp1344 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1341, 4), 786431))) as usize] } - fTemp1342))))} else {1.0 - (fTemp1337 + fTemp742 * fTemp1338 + (fTemp1333 - (iTemp1334) as F64) * (fTemp1336 - (fTemp1337 + fTemp742 * (fTemp1338 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1335, 4), 786431))) as usize] } - fTemp1336)))))};
			let mut fTemp1346: F64 = fTemp747 + fTemp1332;
			let mut fTemp1347: F64 = 262143.0 * (1.0 - fTemp1346);
			let mut iTemp1348: i32 = (fTemp1347) as i32;
			let mut iTemp1349: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1348, 262143)))), 786431));
			let mut fTemp1350: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1349, 3)) as usize] };
			let mut fTemp1351: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1349 as usize] };
			let mut fTemp1352: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1349, 1)) as usize] } - fTemp1351;
			let mut fTemp1353: F64 = 262143.0 * fTemp1346;
			let mut iTemp1354: i32 = (fTemp1353) as i32;
			let mut iTemp1355: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp737, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1354, 262143)))), 786431));
			let mut fTemp1356: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1355, 3), 786431))) as usize] };
			let mut fTemp1357: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1355 as usize] };
			let mut fTemp1358: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1355, 1), 786431))) as usize] } - fTemp1357;
			let mut fTemp1359: F64 = fTemp703 + if ((0.001 * fTemp746) == 0.0) as i32 != 0 {fTemp732} else {fTemp732 * (if iTemp733 != 0 {fTemp1357 + fTemp742 * fTemp1358 + (fTemp1353 - (iTemp1354) as F64) * (fTemp1356 - (fTemp1357 + fTemp742 * (fTemp1358 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1355, 4), 786431))) as usize] } - fTemp1356))))} else {1.0 - (fTemp1351 + fTemp742 * fTemp1352 + (fTemp1347 - (iTemp1348) as F64) * (fTemp1350 - (fTemp1351 + fTemp742 * (fTemp1352 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1349, 4)) as usize] } - fTemp1350)))))} - fTemp1345) / (1.0 - fTemp1345)};
			self.fRec16[(self.IOTA0 & 32767) as usize] = F64::max(fTemp674, if iTemp745 != 0 {F64::min(fTemp1359, fTemp703)} else {F64::max(fTemp1359, fTemp703)});
			let mut fTemp1360: F64 = self.fRec16[((i32::wrapping_sub(self.IOTA0, iSlow78)) & 32767) as usize];
			*output1 = 0.5 * fTemp2 * self.fVec3[((i32::wrapping_sub(self.IOTA0, iSlow79)) & 32767) as usize] + self.fRec14[0] * fTemp4 * self.fVec4[((i32::wrapping_sub(self.IOTA0, iSlow79)) & 32767) as usize] * fTemp1360;
			*output2 = fTemp3 + fTemp702 * fTemp4;
			*output3 = fTemp3 + fTemp4 * fTemp1360;
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
			self.fRec14[1] = self.fRec14[0];
			self.fVec36[2] = self.fVec36[1];
			self.fVec36[1] = self.fVec36[0];
			for j4 in (1..=6).rev() {
				self.fVec37[j4 as usize] = self.fVec37[(i32::wrapping_sub(j4, 1)) as usize];
			}
			for j5 in (1..=14).rev() {
				self.fVec38[j5 as usize] = self.fVec38[(i32::wrapping_sub(j5, 1)) as usize];
			}
			self.fRec17[1] = self.fRec17[0];
			self.fVec49[2] = self.fVec49[1];
			self.fVec49[1] = self.fVec49[0];
			for j6 in (1..=6).rev() {
				self.fVec50[j6 as usize] = self.fVec50[(i32::wrapping_sub(j6, 1)) as usize];
			}
			for j7 in (1..=14).rev() {
				self.fVec51[j7 as usize] = self.fVec51[(i32::wrapping_sub(j7, 1)) as usize];
			}
			self.fVec62[1] = self.fVec62[0];
			self.fVec63[1] = self.fVec63[0];
			self.fVec64[1] = self.fVec64[0];
			self.fRec15[1] = self.fRec15[0];
		}
	}

}

}

pub use dsp_192k::LambRs192k;
