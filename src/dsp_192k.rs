/* ------------------------------------------------------------
author: "Bart Brouns"
license: "AGPLv3"
name: "lamb-rs"
version: "0.1"
Code generated with Faust 2.72.14 (https://faust.grame.fr)
Compilation options: -a /run/user/1001/.tmp8wPesE -lang rust -ct 1 -cn LambRs192k -es 1 -mcd 16 -mdd 1024 -mdy 33 -double -ftz 0
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
			let mut iTemp68: i32 = i32::wrapping_add(self.iRec13[0], -1);
			let mut fTemp69: F64 = (iTemp68 % 3) as F64 as i32 as F64;
			let mut fTemp70: F64 = 0.5 * fTemp69;
			let mut fTemp71: F64 = F64::powf(fTemp70, 0.21 * fTemp69 + 1.0);
			let mut fTemp72: F64 = (0.3333333333333333 * (iTemp68 % 786432) as F64) as i32 as F64;
			table[i1 as usize] = F64::min(1.0, F64::max(0.0, if (fTemp70 == 0.0) as i32 != 0 {0.5 * (F64::sin(1.1984270621720943e-05 * fTemp72 + 4.71238898038469) + 1.0)} else {0.5 * (F64::sin(3.141592653589793 * ((1.0 - F64::exp(-(9.231602598581689e-06 * fTemp71 * fTemp72))) / (1.0 - F64::exp(-(2.42 * fTemp71)))) + 4.71238898038469) + 1.0)}));
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
		m.declare("compile_options", r"-a /run/user/1001/.tmp8wPesE -lang rust -ct 1 -cn LambRs192k -es 1 -mcd 16 -mdd 1024 -mdy 33 -double -ftz 0");
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
		self.fHbargraph0 = if (fSlow15) as i32 != 0 {1.92e+04} else {fSlow17};
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
		let mut iSlow50: i32 = (F64::floor(0.0001220703125 * fSlow25)) as i32 % 2;
		let mut iSlow51: i32 = i32::wrapping_add(iSlow49, i32::wrapping_mul(4096, iSlow48));
		let mut iSlow52: i32 = (F64::floor(0.5 * fSlow3)) as i32 % 2;
		let mut iSlow53: i32 = (F64::floor(0.25 * fSlow3)) as i32 % 2;
		let mut iSlow54: i32 = i32::wrapping_add(iSlow4, i32::wrapping_mul(2, iSlow52));
		let mut iSlow55: i32 = (F64::floor(0.125 * fSlow3)) as i32 % 2;
		let mut iSlow56: i32 = i32::wrapping_add(iSlow54, i32::wrapping_mul(4, iSlow53));
		let mut iSlow57: i32 = (F64::floor(0.0625 * fSlow3)) as i32 % 2;
		let mut iSlow58: i32 = i32::wrapping_add(iSlow56, i32::wrapping_mul(8, iSlow55));
		let mut iSlow59: i32 = (F64::floor(0.03125 * fSlow3)) as i32 % 2;
		let mut iSlow60: i32 = i32::wrapping_add(iSlow58, i32::wrapping_mul(16, iSlow57));
		let mut iSlow61: i32 = (F64::floor(0.015625 * fSlow3)) as i32 % 2;
		let mut iSlow62: i32 = i32::wrapping_add(iSlow60, i32::wrapping_mul(32, iSlow59));
		let mut iSlow63: i32 = (F64::floor(0.0078125 * fSlow3)) as i32 % 2;
		let mut iSlow64: i32 = i32::wrapping_add(iSlow62, i32::wrapping_mul(64, iSlow61));
		let mut iSlow65: i32 = (F64::floor(0.00390625 * fSlow3)) as i32 % 2;
		let mut iSlow66: i32 = i32::wrapping_add(iSlow64, i32::wrapping_mul(128, iSlow63));
		let mut iSlow67: i32 = (F64::floor(0.001953125 * fSlow3)) as i32 % 2;
		let mut iSlow68: i32 = i32::wrapping_add(iSlow66, i32::wrapping_mul(256, iSlow65));
		let mut iSlow69: i32 = (F64::floor(0.0009765625 * fSlow3)) as i32 % 2;
		let mut iSlow70: i32 = i32::wrapping_add(iSlow68, i32::wrapping_mul(512, iSlow67));
		let mut iSlow71: i32 = (F64::floor(0.00048828125 * fSlow3)) as i32 % 2;
		let mut iSlow72: i32 = i32::wrapping_add(iSlow70, i32::wrapping_mul(1024, iSlow69));
		let mut iSlow73: i32 = (F64::floor(0.000244140625 * fSlow3)) as i32 % 2;
		let mut iSlow74: i32 = i32::wrapping_add(iSlow72, i32::wrapping_mul(2048, iSlow71));
		let mut iSlow75: i32 = (F64::floor(0.0001220703125 * fSlow3)) as i32 % 2;
		let mut iSlow76: i32 = i32::wrapping_add(iSlow74, i32::wrapping_mul(4096, iSlow73));
		let mut fSlow77: F64 = self.fHslider10;
		let mut fSlow78: F64 = self.fHslider11;
		let mut iSlow79: i32 = (F64::max(0.0, fSlow15 * (1.92e+04 - fSlow17))) as i32;
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
			let mut fTemp41: F64 = F64::min(fTemp34, F64::powf(1e+01, -(fSlow22 * F64::max(0.0, if (iTemp39 == 0) as i32 != 0 {0.0} else {if (iTemp39 == 1) as i32 != 0 {fSlow12 * LambRs192k_faustpower2_f(fSlow7 + fTemp40)} else {fTemp40}}))));
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
			let mut fTemp53: F64 = F64::min(fTemp52, self.fVec16[((i32::wrapping_sub(self.IOTA0, 2048)) & 4095) as usize]);
			self.fVec17[(self.IOTA0 & 8191) as usize] = fTemp53;
			self.fVec18[(self.IOTA0 & 16383) as usize] = F64::min(fTemp53, self.fVec17[((i32::wrapping_sub(self.IOTA0, 4096)) & 8191) as usize]);
			self.fRec3[0] = F64::max(F64::min(self.fRec3[1], self.fVec5[((i32::wrapping_sub(self.IOTA0, iSlow24)) & 16383) as usize]), F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow26 != 0 {fTemp41} else {1.7976931348623157e+308}, if iSlow27 != 0 {self.fVec6[iSlow26 as usize]} else {1.7976931348623157e+308}), if iSlow28 != 0 {self.fVec7[iSlow29 as usize]} else {1.7976931348623157e+308}), if iSlow30 != 0 {self.fVec8[iSlow31 as usize]} else {1.7976931348623157e+308}), if iSlow32 != 0 {self.fVec9[((i32::wrapping_sub(self.IOTA0, iSlow33)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow34 != 0 {self.fVec10[((i32::wrapping_sub(self.IOTA0, iSlow35)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow36 != 0 {self.fVec11[((i32::wrapping_sub(self.IOTA0, iSlow37)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow38 != 0 {self.fVec12[((i32::wrapping_sub(self.IOTA0, iSlow39)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow40 != 0 {self.fVec13[((i32::wrapping_sub(self.IOTA0, iSlow41)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow42 != 0 {self.fVec14[((i32::wrapping_sub(self.IOTA0, iSlow43)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow44 != 0 {self.fVec15[((i32::wrapping_sub(self.IOTA0, iSlow45)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow46 != 0 {self.fVec16[((i32::wrapping_sub(self.IOTA0, iSlow47)) & 4095) as usize]} else {1.7976931348623157e+308}), if iSlow48 != 0 {self.fVec17[((i32::wrapping_sub(self.IOTA0, iSlow49)) & 8191) as usize]} else {1.7976931348623157e+308}), if iSlow50 != 0 {self.fVec18[((i32::wrapping_sub(self.IOTA0, iSlow51)) & 16383) as usize]} else {1.7976931348623157e+308}));
			let mut fTemp54: F64 = F64::min(self.fRec3[0], self.fRec3[1]);
			self.fVec19[0] = fTemp54;
			let mut fTemp55: F64 = F64::min(fTemp54, self.fVec19[2]);
			self.fVec20[0] = fTemp55;
			let mut fTemp56: F64 = F64::min(fTemp55, self.fVec20[4]);
			self.fVec21[0] = fTemp56;
			let mut fTemp57: F64 = F64::min(fTemp56, self.fVec21[8]);
			self.fVec22[(self.IOTA0 & 31) as usize] = fTemp57;
			let mut fTemp58: F64 = F64::min(fTemp57, self.fVec22[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize]);
			self.fVec23[(self.IOTA0 & 63) as usize] = fTemp58;
			let mut fTemp59: F64 = F64::min(fTemp58, self.fVec23[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize]);
			self.fVec24[(self.IOTA0 & 127) as usize] = fTemp59;
			let mut fTemp60: F64 = F64::min(fTemp59, self.fVec24[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize]);
			self.fVec25[(self.IOTA0 & 255) as usize] = fTemp60;
			let mut fTemp61: F64 = F64::min(fTemp60, self.fVec25[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize]);
			self.fVec26[(self.IOTA0 & 511) as usize] = fTemp61;
			let mut fTemp62: F64 = F64::min(fTemp61, self.fVec26[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize]);
			self.fVec27[(self.IOTA0 & 1023) as usize] = fTemp62;
			let mut fTemp63: F64 = F64::min(fTemp62, self.fVec27[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize]);
			self.fVec28[(self.IOTA0 & 2047) as usize] = fTemp63;
			let mut fTemp64: F64 = F64::min(fTemp63, self.fVec28[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			self.fVec29[(self.IOTA0 & 4095) as usize] = fTemp64;
			let mut fTemp65: F64 = F64::min(fTemp64, self.fVec29[((i32::wrapping_sub(self.IOTA0, 2048)) & 4095) as usize]);
			self.fVec30[(self.IOTA0 & 8191) as usize] = fTemp65;
			self.fVec31[(self.IOTA0 & 16383) as usize] = F64::min(fTemp65, self.fVec30[((i32::wrapping_sub(self.IOTA0, 4096)) & 8191) as usize]);
			let mut fTemp66: F64 = F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow4 != 0 {self.fRec3[0]} else {1.7976931348623157e+308}, if iSlow52 != 0 {self.fVec19[iSlow4 as usize]} else {1.7976931348623157e+308}), if iSlow53 != 0 {self.fVec20[iSlow54 as usize]} else {1.7976931348623157e+308}), if iSlow55 != 0 {self.fVec21[iSlow56 as usize]} else {1.7976931348623157e+308}), if iSlow57 != 0 {self.fVec22[((i32::wrapping_sub(self.IOTA0, iSlow58)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow59 != 0 {self.fVec23[((i32::wrapping_sub(self.IOTA0, iSlow60)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow61 != 0 {self.fVec24[((i32::wrapping_sub(self.IOTA0, iSlow62)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow63 != 0 {self.fVec25[((i32::wrapping_sub(self.IOTA0, iSlow64)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow65 != 0 {self.fVec26[((i32::wrapping_sub(self.IOTA0, iSlow66)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow67 != 0 {self.fVec27[((i32::wrapping_sub(self.IOTA0, iSlow68)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow69 != 0 {self.fVec28[((i32::wrapping_sub(self.IOTA0, iSlow70)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow71 != 0 {self.fVec29[((i32::wrapping_sub(self.IOTA0, iSlow72)) & 4095) as usize]} else {1.7976931348623157e+308}), if iSlow73 != 0 {self.fVec30[((i32::wrapping_sub(self.IOTA0, iSlow74)) & 8191) as usize]} else {1.7976931348623157e+308}), if iSlow75 != 0 {self.fVec31[((i32::wrapping_sub(self.IOTA0, iSlow76)) & 16383) as usize]} else {1.7976931348623157e+308}) - fTemp5;
			self.fVec32[0] = fTemp66;
			let mut iTemp67: i32 = (fTemp66 > 0.0) as i32;
			let mut fTemp73: F64 = if iTemp67 != 0 {fSlow78} else {fSlow77};
			self.fVec33[0] = fTemp73;
			let mut fTemp74: F64 = 2.0 * fTemp73;
			let mut iTemp75: i32 = (fTemp74) as i32;
			let mut iTemp76: i32 = std::cmp::max(0, std::cmp::min(iTemp75, 2));
			let mut iTemp77: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, 393213), 786431));
			let mut fTemp78: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp77, 3)) as usize] };
			let mut fTemp79: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp77 as usize] };
			let mut fTemp80: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp77, 1)) as usize] } - fTemp79;
			let mut fTemp81: F64 = fTemp74 - (iTemp75) as F64;
			let mut fTemp82: F64 = fTemp79 + fTemp81 * fTemp80 + 0.5 * (fTemp78 - (fTemp79 + fTemp81 * (fTemp80 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp77, 4)) as usize] } - fTemp78))));
			let mut fTemp83: F64 = if iTemp67 != 0 {fTemp82} else {1.0 - fTemp82};
			let mut iTemp84: i32 = (fTemp66 < 0.0) as i32;
			let mut fTemp85: F64 = fSlow1 * (iTemp84) as F64 + fSlow13 * (iTemp67) as F64;
			self.fVec34[0] = fTemp85;
			let mut fTemp86: F64 = self.fConst10 / fTemp85;
			let mut fTemp87: F64 = fTemp86 + 0.5;
			let mut fTemp88: F64 = 262143.0 * (1.0 - fTemp87);
			let mut iTemp89: i32 = (fTemp88) as i32;
			let mut iTemp90: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp89, 262143)))), 786431));
			let mut fTemp91: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp90, 3)) as usize] };
			let mut fTemp92: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp90 as usize] };
			let mut fTemp93: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp90, 1)) as usize] } - fTemp92;
			let mut fTemp94: F64 = 262143.0 * fTemp87;
			let mut iTemp95: i32 = (fTemp94) as i32;
			let mut iTemp96: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp95, 262143)))), 786431));
			let mut fTemp97: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp96, 3), 786431))) as usize] };
			let mut fTemp98: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp96 as usize] };
			let mut fTemp99: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp96, 1), 786431))) as usize] } - fTemp98;
			let mut fTemp100: F64 = 2.0 * self.fVec33[1];
			let mut iTemp101: i32 = (fTemp100) as i32;
			let mut iTemp102: i32 = std::cmp::max(0, std::cmp::min(iTemp101, 2));
			let mut fTemp103: F64 = 262143.0 * (1.0 - self.fRec1[1]);
			let mut iTemp104: i32 = (fTemp103) as i32;
			let mut iTemp105: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp104, 262143))), iTemp102), 786431));
			let mut fTemp106: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp105, 3), 786431))) as usize] };
			let mut fTemp107: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp105 as usize] };
			let mut fTemp108: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp105, 1), 786431))) as usize] } - fTemp107;
			let mut fTemp109: F64 = fTemp100 - (iTemp101) as F64;
			let mut fTemp110: F64 = 262143.0 * self.fRec1[1];
			let mut iTemp111: i32 = (fTemp110) as i32;
			let mut iTemp112: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp102, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp111, 262143)))), 786431));
			let mut fTemp113: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp112, 3), 786431))) as usize] };
			let mut fTemp114: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp112 as usize] };
			let mut fTemp115: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp112, 1), 786431))) as usize] } - fTemp114;
			let mut fTemp116: F64 = self.fRec1[1] + fTemp86;
			let mut fTemp117: F64 = 262143.0 * (1.0 - fTemp116);
			let mut iTemp118: i32 = (fTemp117) as i32;
			let mut iTemp119: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp118, 262143)))), 786431));
			let mut fTemp120: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp119, 3)) as usize] };
			let mut fTemp121: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp119 as usize] };
			let mut fTemp122: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp119, 1)) as usize] } - fTemp121;
			let mut fTemp123: F64 = 262143.0 * fTemp116;
			let mut iTemp124: i32 = (fTemp123) as i32;
			let mut iTemp125: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp124, 262143)))), 786431));
			let mut fTemp126: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp125, 3), 786431))) as usize] };
			let mut fTemp127: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp125 as usize] };
			let mut fTemp128: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp125, 1), 786431))) as usize] } - fTemp127;
			let mut fTemp129: F64 = self.fRec1[1] + self.fConst10 * (1.0 / fTemp85 + 1.0 / self.fVec34[1]);
			let mut fTemp130: F64 = 262143.0 * (1.0 - fTemp129);
			let mut iTemp131: i32 = (fTemp130) as i32;
			let mut iTemp132: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp131, 262143))), iTemp76), 786431));
			let mut fTemp133: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp132, 3)) as usize] };
			let mut fTemp134: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp132 as usize] };
			let mut fTemp135: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp132, 1)) as usize] } - fTemp134;
			let mut fTemp136: F64 = 262143.0 * fTemp129;
			let mut iTemp137: i32 = (fTemp136) as i32;
			let mut iTemp138: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp137, 262143)))), 786431));
			let mut fTemp139: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp138, 3), 786431))) as usize] };
			let mut fTemp140: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp138 as usize] };
			let mut fTemp141: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp138, 1), 786431))) as usize] } - fTemp140;
			let mut fTemp142: F64 = (if iTemp67 != 0 {fTemp140 + fTemp81 * fTemp141 + (fTemp136 - (iTemp137) as F64) * (fTemp139 - (fTemp140 + fTemp81 * (fTemp141 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp138, 4), 786431))) as usize] } - fTemp139))))} else {1.0 - (fTemp134 + fTemp81 * fTemp135 + (fTemp130 - (iTemp131) as F64) * (fTemp133 - (fTemp134 + fTemp81 * (fTemp135 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp132, 4)) as usize] } - fTemp133)))))} - if iTemp67 != 0 {fTemp127 + fTemp81 * fTemp128 + (fTemp123 - (iTemp124) as F64) * (fTemp126 - (fTemp127 + fTemp81 * (fTemp128 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp125, 4), 786431))) as usize] } - fTemp126))))} else {1.0 - (fTemp121 + fTemp81 * fTemp122 + (fTemp117 - (iTemp118) as F64) * (fTemp120 - (fTemp121 + fTemp81 * (fTemp122 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp119, 4)) as usize] } - fTemp120)))))}) * self.fVec32[1] / (fTemp66 * (1.0 - if iTemp67 != 0 {fTemp114 + fTemp109 * fTemp115 + (fTemp110 - (iTemp111) as F64) * (fTemp113 - (fTemp114 + fTemp109 * (fTemp115 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp112, 4), 786431))) as usize] } - fTemp113))))} else {1.0 - (fTemp107 + fTemp109 * fTemp108 + (fTemp103 - (iTemp104) as F64) * (fTemp106 - (fTemp107 + fTemp109 * (fTemp108 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp105, 4), 786431))) as usize] } - fTemp106)))))}));
			let mut iTemp143: i32 = (fTemp142 > ((if iTemp67 != 0 {fTemp98 + fTemp81 * fTemp99 + (fTemp94 - (iTemp95) as F64) * (fTemp97 - (fTemp98 + fTemp81 * (fTemp99 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp96, 4), 786431))) as usize] } - fTemp97))))} else {1.0 - (fTemp92 + fTemp81 * fTemp93 + (fTemp88 - (iTemp89) as F64) * (fTemp91 - (fTemp92 + fTemp81 * (fTemp93 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp90, 4)) as usize] } - fTemp91)))))} - fTemp83) / (1.0 - fTemp83))) as i32;
			let mut fTemp144: F64 = if iTemp143 != 0 {1.0} else {0.5};
			let mut fTemp145: F64 = if iTemp143 != 0 {0.5} else {0.0};
			let mut fTemp146: F64 = fTemp145 + fTemp144;
			let mut fTemp147: F64 = 0.5 * fTemp146;
			let mut fTemp148: F64 = 262143.0 * (1.0 - fTemp147);
			let mut iTemp149: i32 = (fTemp148) as i32;
			let mut iTemp150: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp149, 262143)))), 786431));
			let mut fTemp151: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp150, 3)) as usize] };
			let mut fTemp152: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp150 as usize] };
			let mut fTemp153: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp150, 1)) as usize] } - fTemp152;
			let mut fTemp154: F64 = 131071.5 * fTemp146;
			let mut iTemp155: i32 = (fTemp154) as i32;
			let mut iTemp156: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp155, 262143)))), 786431));
			let mut fTemp157: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp156, 3)) as usize] };
			let mut fTemp158: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp156 as usize] };
			let mut fTemp159: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp156, 1)) as usize] } - fTemp158;
			let mut fTemp160: F64 = if iTemp67 != 0 {fTemp158 + fTemp81 * fTemp159 + (fTemp154 - (iTemp155) as F64) * (fTemp157 - (fTemp158 + fTemp81 * (fTemp159 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp156, 4)) as usize] } - fTemp157))))} else {1.0 - (fTemp152 + fTemp81 * fTemp153 + (fTemp148 - (iTemp149) as F64) * (fTemp151 - (fTemp152 + fTemp81 * (fTemp153 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp150, 4)) as usize] } - fTemp151)))))};
			let mut fTemp161: F64 = fTemp86 + fTemp147;
			let mut fTemp162: F64 = 262143.0 * (1.0 - fTemp161);
			let mut iTemp163: i32 = (fTemp162) as i32;
			let mut iTemp164: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp163, 262143)))), 786431));
			let mut fTemp165: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp164, 3)) as usize] };
			let mut fTemp166: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp164 as usize] };
			let mut fTemp167: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp164, 1)) as usize] } - fTemp166;
			let mut fTemp168: F64 = 262143.0 * fTemp161;
			let mut iTemp169: i32 = (fTemp168) as i32;
			let mut iTemp170: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp169, 262143)))), 786431));
			let mut fTemp171: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp170, 3), 786431))) as usize] };
			let mut fTemp172: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp170 as usize] };
			let mut fTemp173: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp170, 1), 786431))) as usize] } - fTemp172;
			let mut iTemp174: i32 = (fTemp142 > ((if iTemp67 != 0 {fTemp172 + fTemp81 * fTemp173 + (fTemp168 - (iTemp169) as F64) * (fTemp171 - (fTemp172 + fTemp81 * (fTemp173 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp170, 4), 786431))) as usize] } - fTemp171))))} else {1.0 - (fTemp166 + fTemp81 * fTemp167 + (fTemp162 - (iTemp163) as F64) * (fTemp165 - (fTemp166 + fTemp81 * (fTemp167 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp164, 4)) as usize] } - fTemp165)))))} - fTemp160) / (1.0 - fTemp160))) as i32;
			let mut fTemp175: F64 = if iTemp174 != 0 {fTemp144} else {fTemp147};
			let mut fTemp176: F64 = if iTemp174 != 0 {fTemp147} else {fTemp145};
			let mut fTemp177: F64 = fTemp176 + fTemp175;
			let mut fTemp178: F64 = 0.5 * fTemp177;
			let mut fTemp179: F64 = 262143.0 * (1.0 - fTemp178);
			let mut iTemp180: i32 = (fTemp179) as i32;
			let mut iTemp181: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp180, 262143)))), 786431));
			let mut fTemp182: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp181, 3)) as usize] };
			let mut fTemp183: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp181 as usize] };
			let mut fTemp184: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp181, 1)) as usize] } - fTemp183;
			let mut fTemp185: F64 = 131071.5 * fTemp177;
			let mut iTemp186: i32 = (fTemp185) as i32;
			let mut iTemp187: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp186, 262143)))), 786431));
			let mut fTemp188: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp187, 3)) as usize] };
			let mut fTemp189: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp187 as usize] };
			let mut fTemp190: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp187, 1)) as usize] } - fTemp189;
			let mut fTemp191: F64 = if iTemp67 != 0 {fTemp189 + fTemp81 * fTemp190 + (fTemp185 - (iTemp186) as F64) * (fTemp188 - (fTemp189 + fTemp81 * (fTemp190 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp187, 4)) as usize] } - fTemp188))))} else {1.0 - (fTemp183 + fTemp81 * fTemp184 + (fTemp179 - (iTemp180) as F64) * (fTemp182 - (fTemp183 + fTemp81 * (fTemp184 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp181, 4)) as usize] } - fTemp182)))))};
			let mut fTemp192: F64 = fTemp86 + fTemp178;
			let mut fTemp193: F64 = 262143.0 * (1.0 - fTemp192);
			let mut iTemp194: i32 = (fTemp193) as i32;
			let mut iTemp195: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp194, 262143)))), 786431));
			let mut fTemp196: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp195, 3)) as usize] };
			let mut fTemp197: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp195 as usize] };
			let mut fTemp198: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp195, 1)) as usize] } - fTemp197;
			let mut fTemp199: F64 = 262143.0 * fTemp192;
			let mut iTemp200: i32 = (fTemp199) as i32;
			let mut iTemp201: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp200, 262143)))), 786431));
			let mut fTemp202: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp201, 3), 786431))) as usize] };
			let mut fTemp203: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp201 as usize] };
			let mut fTemp204: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp201, 1), 786431))) as usize] } - fTemp203;
			let mut iTemp205: i32 = (fTemp142 > ((if iTemp67 != 0 {fTemp203 + fTemp81 * fTemp204 + (fTemp199 - (iTemp200) as F64) * (fTemp202 - (fTemp203 + fTemp81 * (fTemp204 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp201, 4), 786431))) as usize] } - fTemp202))))} else {1.0 - (fTemp197 + fTemp81 * fTemp198 + (fTemp193 - (iTemp194) as F64) * (fTemp196 - (fTemp197 + fTemp81 * (fTemp198 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp195, 4)) as usize] } - fTemp196)))))} - fTemp191) / (1.0 - fTemp191))) as i32;
			let mut fTemp206: F64 = if iTemp205 != 0 {fTemp175} else {fTemp178};
			let mut fTemp207: F64 = if iTemp205 != 0 {fTemp178} else {fTemp176};
			let mut fTemp208: F64 = fTemp207 + fTemp206;
			let mut fTemp209: F64 = 0.5 * fTemp208;
			let mut fTemp210: F64 = 262143.0 * (1.0 - fTemp209);
			let mut iTemp211: i32 = (fTemp210) as i32;
			let mut iTemp212: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp211, 262143)))), 786431));
			let mut fTemp213: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp212, 3)) as usize] };
			let mut fTemp214: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp212 as usize] };
			let mut fTemp215: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp212, 1)) as usize] } - fTemp214;
			let mut fTemp216: F64 = 131071.5 * fTemp208;
			let mut iTemp217: i32 = (fTemp216) as i32;
			let mut iTemp218: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp217, 262143)))), 786431));
			let mut fTemp219: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp218, 3)) as usize] };
			let mut fTemp220: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp218 as usize] };
			let mut fTemp221: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp218, 1)) as usize] } - fTemp220;
			let mut fTemp222: F64 = if iTemp67 != 0 {fTemp220 + fTemp81 * fTemp221 + (fTemp216 - (iTemp217) as F64) * (fTemp219 - (fTemp220 + fTemp81 * (fTemp221 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp218, 4)) as usize] } - fTemp219))))} else {1.0 - (fTemp214 + fTemp81 * fTemp215 + (fTemp210 - (iTemp211) as F64) * (fTemp213 - (fTemp214 + fTemp81 * (fTemp215 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp212, 4)) as usize] } - fTemp213)))))};
			let mut fTemp223: F64 = fTemp86 + fTemp209;
			let mut fTemp224: F64 = 262143.0 * (1.0 - fTemp223);
			let mut iTemp225: i32 = (fTemp224) as i32;
			let mut iTemp226: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp225, 262143)))), 786431));
			let mut fTemp227: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp226, 3)) as usize] };
			let mut fTemp228: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp226 as usize] };
			let mut fTemp229: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp226, 1)) as usize] } - fTemp228;
			let mut fTemp230: F64 = 262143.0 * fTemp223;
			let mut iTemp231: i32 = (fTemp230) as i32;
			let mut iTemp232: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp231, 262143)))), 786431));
			let mut fTemp233: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp232, 3), 786431))) as usize] };
			let mut fTemp234: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp232 as usize] };
			let mut fTemp235: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp232, 1), 786431))) as usize] } - fTemp234;
			let mut iTemp236: i32 = (fTemp142 > ((if iTemp67 != 0 {fTemp234 + fTemp81 * fTemp235 + (fTemp230 - (iTemp231) as F64) * (fTemp233 - (fTemp234 + fTemp81 * (fTemp235 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp232, 4), 786431))) as usize] } - fTemp233))))} else {1.0 - (fTemp228 + fTemp81 * fTemp229 + (fTemp224 - (iTemp225) as F64) * (fTemp227 - (fTemp228 + fTemp81 * (fTemp229 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp226, 4)) as usize] } - fTemp227)))))} - fTemp222) / (1.0 - fTemp222))) as i32;
			let mut fTemp237: F64 = if iTemp236 != 0 {fTemp206} else {fTemp209};
			let mut fTemp238: F64 = if iTemp236 != 0 {fTemp209} else {fTemp207};
			let mut fTemp239: F64 = fTemp238 + fTemp237;
			let mut fTemp240: F64 = 0.5 * fTemp239;
			let mut fTemp241: F64 = 262143.0 * (1.0 - fTemp240);
			let mut iTemp242: i32 = (fTemp241) as i32;
			let mut iTemp243: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp242, 262143)))), 786431));
			let mut fTemp244: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp243, 3)) as usize] };
			let mut fTemp245: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp243 as usize] };
			let mut fTemp246: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp243, 1)) as usize] } - fTemp245;
			let mut fTemp247: F64 = 131071.5 * fTemp239;
			let mut iTemp248: i32 = (fTemp247) as i32;
			let mut iTemp249: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp248, 262143)))), 786431));
			let mut fTemp250: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp249, 3)) as usize] };
			let mut fTemp251: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp249 as usize] };
			let mut fTemp252: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp249, 1)) as usize] } - fTemp251;
			let mut fTemp253: F64 = if iTemp67 != 0 {fTemp251 + fTemp81 * fTemp252 + (fTemp247 - (iTemp248) as F64) * (fTemp250 - (fTemp251 + fTemp81 * (fTemp252 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp249, 4)) as usize] } - fTemp250))))} else {1.0 - (fTemp245 + fTemp81 * fTemp246 + (fTemp241 - (iTemp242) as F64) * (fTemp244 - (fTemp245 + fTemp81 * (fTemp246 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp243, 4)) as usize] } - fTemp244)))))};
			let mut fTemp254: F64 = fTemp86 + fTemp240;
			let mut fTemp255: F64 = 262143.0 * (1.0 - fTemp254);
			let mut iTemp256: i32 = (fTemp255) as i32;
			let mut iTemp257: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp256, 262143)))), 786431));
			let mut fTemp258: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp257, 3)) as usize] };
			let mut fTemp259: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp257 as usize] };
			let mut fTemp260: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp257, 1)) as usize] } - fTemp259;
			let mut fTemp261: F64 = 262143.0 * fTemp254;
			let mut iTemp262: i32 = (fTemp261) as i32;
			let mut iTemp263: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp262, 262143)))), 786431));
			let mut fTemp264: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp263, 3), 786431))) as usize] };
			let mut fTemp265: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp263 as usize] };
			let mut fTemp266: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp263, 1), 786431))) as usize] } - fTemp265;
			let mut iTemp267: i32 = (fTemp142 > ((if iTemp67 != 0 {fTemp265 + fTemp81 * fTemp266 + (fTemp261 - (iTemp262) as F64) * (fTemp264 - (fTemp265 + fTemp81 * (fTemp266 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp263, 4), 786431))) as usize] } - fTemp264))))} else {1.0 - (fTemp259 + fTemp81 * fTemp260 + (fTemp255 - (iTemp256) as F64) * (fTemp258 - (fTemp259 + fTemp81 * (fTemp260 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp257, 4)) as usize] } - fTemp258)))))} - fTemp253) / (1.0 - fTemp253))) as i32;
			let mut fTemp268: F64 = if iTemp267 != 0 {fTemp237} else {fTemp240};
			let mut fTemp269: F64 = if iTemp267 != 0 {fTemp240} else {fTemp238};
			let mut fTemp270: F64 = fTemp269 + fTemp268;
			let mut fTemp271: F64 = 0.5 * fTemp270;
			let mut fTemp272: F64 = 262143.0 * (1.0 - fTemp271);
			let mut iTemp273: i32 = (fTemp272) as i32;
			let mut iTemp274: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp273, 262143)))), 786431));
			let mut fTemp275: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp274, 3)) as usize] };
			let mut fTemp276: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp274 as usize] };
			let mut fTemp277: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp274, 1)) as usize] } - fTemp276;
			let mut fTemp278: F64 = 131071.5 * fTemp270;
			let mut iTemp279: i32 = (fTemp278) as i32;
			let mut iTemp280: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp279, 262143)))), 786431));
			let mut fTemp281: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp280, 3)) as usize] };
			let mut fTemp282: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp280 as usize] };
			let mut fTemp283: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp280, 1)) as usize] } - fTemp282;
			let mut fTemp284: F64 = if iTemp67 != 0 {fTemp282 + fTemp81 * fTemp283 + (fTemp278 - (iTemp279) as F64) * (fTemp281 - (fTemp282 + fTemp81 * (fTemp283 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp280, 4)) as usize] } - fTemp281))))} else {1.0 - (fTemp276 + fTemp81 * fTemp277 + (fTemp272 - (iTemp273) as F64) * (fTemp275 - (fTemp276 + fTemp81 * (fTemp277 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp274, 4)) as usize] } - fTemp275)))))};
			let mut fTemp285: F64 = fTemp86 + fTemp271;
			let mut fTemp286: F64 = 262143.0 * (1.0 - fTemp285);
			let mut iTemp287: i32 = (fTemp286) as i32;
			let mut iTemp288: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp287, 262143)))), 786431));
			let mut fTemp289: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp288, 3)) as usize] };
			let mut fTemp290: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp288 as usize] };
			let mut fTemp291: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp288, 1)) as usize] } - fTemp290;
			let mut fTemp292: F64 = 262143.0 * fTemp285;
			let mut iTemp293: i32 = (fTemp292) as i32;
			let mut iTemp294: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp293, 262143)))), 786431));
			let mut fTemp295: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp294, 3), 786431))) as usize] };
			let mut fTemp296: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp294 as usize] };
			let mut fTemp297: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp294, 1), 786431))) as usize] } - fTemp296;
			let mut iTemp298: i32 = (fTemp142 > ((if iTemp67 != 0 {fTemp296 + fTemp81 * fTemp297 + (fTemp292 - (iTemp293) as F64) * (fTemp295 - (fTemp296 + fTemp81 * (fTemp297 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp294, 4), 786431))) as usize] } - fTemp295))))} else {1.0 - (fTemp290 + fTemp81 * fTemp291 + (fTemp286 - (iTemp287) as F64) * (fTemp289 - (fTemp290 + fTemp81 * (fTemp291 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp288, 4)) as usize] } - fTemp289)))))} - fTemp284) / (1.0 - fTemp284))) as i32;
			let mut fTemp299: F64 = if iTemp298 != 0 {fTemp268} else {fTemp271};
			let mut fTemp300: F64 = if iTemp298 != 0 {fTemp271} else {fTemp269};
			let mut fTemp301: F64 = fTemp300 + fTemp299;
			let mut fTemp302: F64 = 0.5 * fTemp301;
			let mut fTemp303: F64 = 262143.0 * (1.0 - fTemp302);
			let mut iTemp304: i32 = (fTemp303) as i32;
			let mut iTemp305: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp304, 262143)))), 786431));
			let mut fTemp306: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp305, 3)) as usize] };
			let mut fTemp307: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp305 as usize] };
			let mut fTemp308: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp305, 1)) as usize] } - fTemp307;
			let mut fTemp309: F64 = 131071.5 * fTemp301;
			let mut iTemp310: i32 = (fTemp309) as i32;
			let mut iTemp311: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp310, 262143)))), 786431));
			let mut fTemp312: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp311, 3)) as usize] };
			let mut fTemp313: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp311 as usize] };
			let mut fTemp314: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp311, 1)) as usize] } - fTemp313;
			let mut fTemp315: F64 = if iTemp67 != 0 {fTemp313 + fTemp81 * fTemp314 + (fTemp309 - (iTemp310) as F64) * (fTemp312 - (fTemp313 + fTemp81 * (fTemp314 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp311, 4)) as usize] } - fTemp312))))} else {1.0 - (fTemp307 + fTemp81 * fTemp308 + (fTemp303 - (iTemp304) as F64) * (fTemp306 - (fTemp307 + fTemp81 * (fTemp308 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp305, 4)) as usize] } - fTemp306)))))};
			let mut fTemp316: F64 = fTemp86 + fTemp302;
			let mut fTemp317: F64 = 262143.0 * (1.0 - fTemp316);
			let mut iTemp318: i32 = (fTemp317) as i32;
			let mut iTemp319: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp318, 262143)))), 786431));
			let mut fTemp320: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp319, 3)) as usize] };
			let mut fTemp321: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp319 as usize] };
			let mut fTemp322: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp319, 1)) as usize] } - fTemp321;
			let mut fTemp323: F64 = 262143.0 * fTemp316;
			let mut iTemp324: i32 = (fTemp323) as i32;
			let mut iTemp325: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp324, 262143)))), 786431));
			let mut fTemp326: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp325, 3), 786431))) as usize] };
			let mut fTemp327: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp325 as usize] };
			let mut fTemp328: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp325, 1), 786431))) as usize] } - fTemp327;
			let mut iTemp329: i32 = (fTemp142 > ((if iTemp67 != 0 {fTemp327 + fTemp81 * fTemp328 + (fTemp323 - (iTemp324) as F64) * (fTemp326 - (fTemp327 + fTemp81 * (fTemp328 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp325, 4), 786431))) as usize] } - fTemp326))))} else {1.0 - (fTemp321 + fTemp81 * fTemp322 + (fTemp317 - (iTemp318) as F64) * (fTemp320 - (fTemp321 + fTemp81 * (fTemp322 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp319, 4)) as usize] } - fTemp320)))))} - fTemp315) / (1.0 - fTemp315))) as i32;
			let mut fTemp330: F64 = if iTemp329 != 0 {fTemp299} else {fTemp302};
			let mut fTemp331: F64 = if iTemp329 != 0 {fTemp302} else {fTemp300};
			let mut fTemp332: F64 = fTemp331 + fTemp330;
			let mut fTemp333: F64 = 0.5 * fTemp332;
			let mut fTemp334: F64 = 262143.0 * (1.0 - fTemp333);
			let mut iTemp335: i32 = (fTemp334) as i32;
			let mut iTemp336: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp335, 262143)))), 786431));
			let mut fTemp337: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp336, 3)) as usize] };
			let mut fTemp338: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp336 as usize] };
			let mut fTemp339: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp336, 1)) as usize] } - fTemp338;
			let mut fTemp340: F64 = 131071.5 * fTemp332;
			let mut iTemp341: i32 = (fTemp340) as i32;
			let mut iTemp342: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp341, 262143)))), 786431));
			let mut fTemp343: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp342, 3)) as usize] };
			let mut fTemp344: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp342 as usize] };
			let mut fTemp345: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp342, 1)) as usize] } - fTemp344;
			let mut fTemp346: F64 = if iTemp67 != 0 {fTemp344 + fTemp81 * fTemp345 + (fTemp340 - (iTemp341) as F64) * (fTemp343 - (fTemp344 + fTemp81 * (fTemp345 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp342, 4)) as usize] } - fTemp343))))} else {1.0 - (fTemp338 + fTemp81 * fTemp339 + (fTemp334 - (iTemp335) as F64) * (fTemp337 - (fTemp338 + fTemp81 * (fTemp339 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp336, 4)) as usize] } - fTemp337)))))};
			let mut fTemp347: F64 = fTemp86 + fTemp333;
			let mut fTemp348: F64 = 262143.0 * (1.0 - fTemp347);
			let mut iTemp349: i32 = (fTemp348) as i32;
			let mut iTemp350: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp349, 262143)))), 786431));
			let mut fTemp351: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp350, 3)) as usize] };
			let mut fTemp352: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp350 as usize] };
			let mut fTemp353: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp350, 1)) as usize] } - fTemp352;
			let mut fTemp354: F64 = 262143.0 * fTemp347;
			let mut iTemp355: i32 = (fTemp354) as i32;
			let mut iTemp356: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp355, 262143)))), 786431));
			let mut fTemp357: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp356, 3), 786431))) as usize] };
			let mut fTemp358: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp356 as usize] };
			let mut fTemp359: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp356, 1), 786431))) as usize] } - fTemp358;
			let mut iTemp360: i32 = (fTemp142 > ((if iTemp67 != 0 {fTemp358 + fTemp81 * fTemp359 + (fTemp354 - (iTemp355) as F64) * (fTemp357 - (fTemp358 + fTemp81 * (fTemp359 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp356, 4), 786431))) as usize] } - fTemp357))))} else {1.0 - (fTemp352 + fTemp81 * fTemp353 + (fTemp348 - (iTemp349) as F64) * (fTemp351 - (fTemp352 + fTemp81 * (fTemp353 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp350, 4)) as usize] } - fTemp351)))))} - fTemp346) / (1.0 - fTemp346))) as i32;
			let mut fTemp361: F64 = if iTemp360 != 0 {fTemp330} else {fTemp333};
			let mut fTemp362: F64 = if iTemp360 != 0 {fTemp333} else {fTemp331};
			let mut fTemp363: F64 = fTemp362 + fTemp361;
			let mut fTemp364: F64 = 0.5 * fTemp363;
			let mut fTemp365: F64 = 262143.0 * (1.0 - fTemp364);
			let mut iTemp366: i32 = (fTemp365) as i32;
			let mut iTemp367: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp366, 262143)))), 786431));
			let mut fTemp368: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp367, 3)) as usize] };
			let mut fTemp369: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp367 as usize] };
			let mut fTemp370: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp367, 1)) as usize] } - fTemp369;
			let mut fTemp371: F64 = 131071.5 * fTemp363;
			let mut iTemp372: i32 = (fTemp371) as i32;
			let mut iTemp373: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp372, 262143)))), 786431));
			let mut fTemp374: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp373, 3)) as usize] };
			let mut fTemp375: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp373 as usize] };
			let mut fTemp376: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp373, 1)) as usize] } - fTemp375;
			let mut fTemp377: F64 = if iTemp67 != 0 {fTemp375 + fTemp81 * fTemp376 + (fTemp371 - (iTemp372) as F64) * (fTemp374 - (fTemp375 + fTemp81 * (fTemp376 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp373, 4)) as usize] } - fTemp374))))} else {1.0 - (fTemp369 + fTemp81 * fTemp370 + (fTemp365 - (iTemp366) as F64) * (fTemp368 - (fTemp369 + fTemp81 * (fTemp370 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp367, 4)) as usize] } - fTemp368)))))};
			let mut fTemp378: F64 = fTemp86 + fTemp364;
			let mut fTemp379: F64 = 262143.0 * (1.0 - fTemp378);
			let mut iTemp380: i32 = (fTemp379) as i32;
			let mut iTemp381: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp380, 262143)))), 786431));
			let mut fTemp382: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp381, 3)) as usize] };
			let mut fTemp383: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp381 as usize] };
			let mut fTemp384: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp381, 1)) as usize] } - fTemp383;
			let mut fTemp385: F64 = 262143.0 * fTemp378;
			let mut iTemp386: i32 = (fTemp385) as i32;
			let mut iTemp387: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp386, 262143)))), 786431));
			let mut fTemp388: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp387, 3), 786431))) as usize] };
			let mut fTemp389: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp387 as usize] };
			let mut fTemp390: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp387, 1), 786431))) as usize] } - fTemp389;
			let mut iTemp391: i32 = (fTemp142 > ((if iTemp67 != 0 {fTemp389 + fTemp81 * fTemp390 + (fTemp385 - (iTemp386) as F64) * (fTemp388 - (fTemp389 + fTemp81 * (fTemp390 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp387, 4), 786431))) as usize] } - fTemp388))))} else {1.0 - (fTemp383 + fTemp81 * fTemp384 + (fTemp379 - (iTemp380) as F64) * (fTemp382 - (fTemp383 + fTemp81 * (fTemp384 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp381, 4)) as usize] } - fTemp382)))))} - fTemp377) / (1.0 - fTemp377))) as i32;
			let mut fTemp392: F64 = if iTemp391 != 0 {fTemp361} else {fTemp364};
			let mut fTemp393: F64 = if iTemp391 != 0 {fTemp364} else {fTemp362};
			let mut fTemp394: F64 = fTemp393 + fTemp392;
			let mut fTemp395: F64 = 0.5 * fTemp394;
			let mut fTemp396: F64 = 262143.0 * (1.0 - fTemp395);
			let mut iTemp397: i32 = (fTemp396) as i32;
			let mut iTemp398: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp397, 262143)))), 786431));
			let mut fTemp399: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp398, 3)) as usize] };
			let mut fTemp400: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp398 as usize] };
			let mut fTemp401: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp398, 1)) as usize] } - fTemp400;
			let mut fTemp402: F64 = 131071.5 * fTemp394;
			let mut iTemp403: i32 = (fTemp402) as i32;
			let mut iTemp404: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp403, 262143)))), 786431));
			let mut fTemp405: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp404, 3)) as usize] };
			let mut fTemp406: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp404 as usize] };
			let mut fTemp407: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp404, 1)) as usize] } - fTemp406;
			let mut fTemp408: F64 = if iTemp67 != 0 {fTemp406 + fTemp81 * fTemp407 + (fTemp402 - (iTemp403) as F64) * (fTemp405 - (fTemp406 + fTemp81 * (fTemp407 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp404, 4)) as usize] } - fTemp405))))} else {1.0 - (fTemp400 + fTemp81 * fTemp401 + (fTemp396 - (iTemp397) as F64) * (fTemp399 - (fTemp400 + fTemp81 * (fTemp401 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp398, 4)) as usize] } - fTemp399)))))};
			let mut fTemp409: F64 = fTemp86 + fTemp395;
			let mut fTemp410: F64 = 262143.0 * (1.0 - fTemp409);
			let mut iTemp411: i32 = (fTemp410) as i32;
			let mut iTemp412: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp411, 262143)))), 786431));
			let mut fTemp413: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp412, 3)) as usize] };
			let mut fTemp414: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp412 as usize] };
			let mut fTemp415: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp412, 1)) as usize] } - fTemp414;
			let mut fTemp416: F64 = 262143.0 * fTemp409;
			let mut iTemp417: i32 = (fTemp416) as i32;
			let mut iTemp418: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp417, 262143)))), 786431));
			let mut fTemp419: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp418, 3), 786431))) as usize] };
			let mut fTemp420: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp418 as usize] };
			let mut fTemp421: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp418, 1), 786431))) as usize] } - fTemp420;
			let mut iTemp422: i32 = (fTemp142 > ((if iTemp67 != 0 {fTemp420 + fTemp81 * fTemp421 + (fTemp416 - (iTemp417) as F64) * (fTemp419 - (fTemp420 + fTemp81 * (fTemp421 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp418, 4), 786431))) as usize] } - fTemp419))))} else {1.0 - (fTemp414 + fTemp81 * fTemp415 + (fTemp410 - (iTemp411) as F64) * (fTemp413 - (fTemp414 + fTemp81 * (fTemp415 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp412, 4)) as usize] } - fTemp413)))))} - fTemp408) / (1.0 - fTemp408))) as i32;
			let mut fTemp423: F64 = if iTemp422 != 0 {fTemp392} else {fTemp395};
			let mut fTemp424: F64 = if iTemp422 != 0 {fTemp395} else {fTemp393};
			let mut fTemp425: F64 = fTemp424 + fTemp423;
			let mut fTemp426: F64 = 0.5 * fTemp425;
			let mut fTemp427: F64 = 262143.0 * (1.0 - fTemp426);
			let mut iTemp428: i32 = (fTemp427) as i32;
			let mut iTemp429: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp428, 262143)))), 786431));
			let mut fTemp430: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp429, 3)) as usize] };
			let mut fTemp431: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp429 as usize] };
			let mut fTemp432: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp429, 1)) as usize] } - fTemp431;
			let mut fTemp433: F64 = 131071.5 * fTemp425;
			let mut iTemp434: i32 = (fTemp433) as i32;
			let mut iTemp435: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp434, 262143)))), 786431));
			let mut fTemp436: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp435, 3)) as usize] };
			let mut fTemp437: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp435 as usize] };
			let mut fTemp438: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp435, 1)) as usize] } - fTemp437;
			let mut fTemp439: F64 = if iTemp67 != 0 {fTemp437 + fTemp81 * fTemp438 + (fTemp433 - (iTemp434) as F64) * (fTemp436 - (fTemp437 + fTemp81 * (fTemp438 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp435, 4)) as usize] } - fTemp436))))} else {1.0 - (fTemp431 + fTemp81 * fTemp432 + (fTemp427 - (iTemp428) as F64) * (fTemp430 - (fTemp431 + fTemp81 * (fTemp432 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp429, 4)) as usize] } - fTemp430)))))};
			let mut fTemp440: F64 = fTemp86 + fTemp426;
			let mut fTemp441: F64 = 262143.0 * (1.0 - fTemp440);
			let mut iTemp442: i32 = (fTemp441) as i32;
			let mut iTemp443: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp442, 262143)))), 786431));
			let mut fTemp444: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp443, 3)) as usize] };
			let mut fTemp445: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp443 as usize] };
			let mut fTemp446: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp443, 1)) as usize] } - fTemp445;
			let mut fTemp447: F64 = 262143.0 * fTemp440;
			let mut iTemp448: i32 = (fTemp447) as i32;
			let mut iTemp449: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp448, 262143)))), 786431));
			let mut fTemp450: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp449, 3), 786431))) as usize] };
			let mut fTemp451: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp449 as usize] };
			let mut fTemp452: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp449, 1), 786431))) as usize] } - fTemp451;
			let mut iTemp453: i32 = (fTemp142 > ((if iTemp67 != 0 {fTemp451 + fTemp81 * fTemp452 + (fTemp447 - (iTemp448) as F64) * (fTemp450 - (fTemp451 + fTemp81 * (fTemp452 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp449, 4), 786431))) as usize] } - fTemp450))))} else {1.0 - (fTemp445 + fTemp81 * fTemp446 + (fTemp441 - (iTemp442) as F64) * (fTemp444 - (fTemp445 + fTemp81 * (fTemp446 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp443, 4)) as usize] } - fTemp444)))))} - fTemp439) / (1.0 - fTemp439))) as i32;
			let mut fTemp454: F64 = if iTemp453 != 0 {fTemp423} else {fTemp426};
			let mut fTemp455: F64 = if iTemp453 != 0 {fTemp426} else {fTemp424};
			let mut fTemp456: F64 = fTemp455 + fTemp454;
			let mut fTemp457: F64 = 0.5 * fTemp456;
			let mut fTemp458: F64 = 262143.0 * (1.0 - fTemp457);
			let mut iTemp459: i32 = (fTemp458) as i32;
			let mut iTemp460: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp459, 262143)))), 786431));
			let mut fTemp461: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp460, 3)) as usize] };
			let mut fTemp462: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp460 as usize] };
			let mut fTemp463: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp460, 1)) as usize] } - fTemp462;
			let mut fTemp464: F64 = 131071.5 * fTemp456;
			let mut iTemp465: i32 = (fTemp464) as i32;
			let mut iTemp466: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp465, 262143)))), 786431));
			let mut fTemp467: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp466, 3)) as usize] };
			let mut fTemp468: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp466 as usize] };
			let mut fTemp469: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp466, 1)) as usize] } - fTemp468;
			let mut fTemp470: F64 = if iTemp67 != 0 {fTemp468 + fTemp81 * fTemp469 + (fTemp464 - (iTemp465) as F64) * (fTemp467 - (fTemp468 + fTemp81 * (fTemp469 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp466, 4)) as usize] } - fTemp467))))} else {1.0 - (fTemp462 + fTemp81 * fTemp463 + (fTemp458 - (iTemp459) as F64) * (fTemp461 - (fTemp462 + fTemp81 * (fTemp463 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp460, 4)) as usize] } - fTemp461)))))};
			let mut fTemp471: F64 = fTemp86 + fTemp457;
			let mut fTemp472: F64 = 262143.0 * (1.0 - fTemp471);
			let mut iTemp473: i32 = (fTemp472) as i32;
			let mut iTemp474: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp473, 262143)))), 786431));
			let mut fTemp475: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp474, 3)) as usize] };
			let mut fTemp476: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp474 as usize] };
			let mut fTemp477: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp474, 1)) as usize] } - fTemp476;
			let mut fTemp478: F64 = 262143.0 * fTemp471;
			let mut iTemp479: i32 = (fTemp478) as i32;
			let mut iTemp480: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp479, 262143)))), 786431));
			let mut fTemp481: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp480, 3), 786431))) as usize] };
			let mut fTemp482: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp480 as usize] };
			let mut fTemp483: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp480, 1), 786431))) as usize] } - fTemp482;
			let mut iTemp484: i32 = (fTemp142 > ((if iTemp67 != 0 {fTemp482 + fTemp81 * fTemp483 + (fTemp478 - (iTemp479) as F64) * (fTemp481 - (fTemp482 + fTemp81 * (fTemp483 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp480, 4), 786431))) as usize] } - fTemp481))))} else {1.0 - (fTemp476 + fTemp81 * fTemp477 + (fTemp472 - (iTemp473) as F64) * (fTemp475 - (fTemp476 + fTemp81 * (fTemp477 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp474, 4)) as usize] } - fTemp475)))))} - fTemp470) / (1.0 - fTemp470))) as i32;
			let mut fTemp485: F64 = if iTemp484 != 0 {fTemp454} else {fTemp457};
			let mut fTemp486: F64 = if iTemp484 != 0 {fTemp457} else {fTemp455};
			let mut fTemp487: F64 = fTemp486 + fTemp485;
			let mut fTemp488: F64 = 0.5 * fTemp487;
			let mut fTemp489: F64 = 262143.0 * (1.0 - fTemp488);
			let mut iTemp490: i32 = (fTemp489) as i32;
			let mut iTemp491: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp490, 262143)))), 786431));
			let mut fTemp492: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp491, 3)) as usize] };
			let mut fTemp493: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp491 as usize] };
			let mut fTemp494: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp491, 1)) as usize] } - fTemp493;
			let mut fTemp495: F64 = 131071.5 * fTemp487;
			let mut iTemp496: i32 = (fTemp495) as i32;
			let mut iTemp497: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp496, 262143)))), 786431));
			let mut fTemp498: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp497, 3)) as usize] };
			let mut fTemp499: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp497 as usize] };
			let mut fTemp500: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp497, 1)) as usize] } - fTemp499;
			let mut fTemp501: F64 = if iTemp67 != 0 {fTemp499 + fTemp81 * fTemp500 + (fTemp495 - (iTemp496) as F64) * (fTemp498 - (fTemp499 + fTemp81 * (fTemp500 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp497, 4)) as usize] } - fTemp498))))} else {1.0 - (fTemp493 + fTemp81 * fTemp494 + (fTemp489 - (iTemp490) as F64) * (fTemp492 - (fTemp493 + fTemp81 * (fTemp494 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp491, 4)) as usize] } - fTemp492)))))};
			let mut fTemp502: F64 = fTemp86 + fTemp488;
			let mut fTemp503: F64 = 262143.0 * (1.0 - fTemp502);
			let mut iTemp504: i32 = (fTemp503) as i32;
			let mut iTemp505: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp504, 262143)))), 786431));
			let mut fTemp506: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp505, 3)) as usize] };
			let mut fTemp507: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp505 as usize] };
			let mut fTemp508: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp505, 1)) as usize] } - fTemp507;
			let mut fTemp509: F64 = 262143.0 * fTemp502;
			let mut iTemp510: i32 = (fTemp509) as i32;
			let mut iTemp511: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp510, 262143)))), 786431));
			let mut fTemp512: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp511, 3), 786431))) as usize] };
			let mut fTemp513: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp511 as usize] };
			let mut fTemp514: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp511, 1), 786431))) as usize] } - fTemp513;
			let mut iTemp515: i32 = (fTemp142 > ((if iTemp67 != 0 {fTemp513 + fTemp81 * fTemp514 + (fTemp509 - (iTemp510) as F64) * (fTemp512 - (fTemp513 + fTemp81 * (fTemp514 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp511, 4), 786431))) as usize] } - fTemp512))))} else {1.0 - (fTemp507 + fTemp81 * fTemp508 + (fTemp503 - (iTemp504) as F64) * (fTemp506 - (fTemp507 + fTemp81 * (fTemp508 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp505, 4)) as usize] } - fTemp506)))))} - fTemp501) / (1.0 - fTemp501))) as i32;
			let mut fTemp516: F64 = if iTemp515 != 0 {fTemp485} else {fTemp488};
			let mut fTemp517: F64 = if iTemp515 != 0 {fTemp488} else {fTemp486};
			let mut fTemp518: F64 = fTemp517 + fTemp516;
			let mut fTemp519: F64 = 0.5 * fTemp518;
			let mut fTemp520: F64 = 262143.0 * (1.0 - fTemp519);
			let mut iTemp521: i32 = (fTemp520) as i32;
			let mut iTemp522: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp521, 262143)))), 786431));
			let mut fTemp523: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp522, 3)) as usize] };
			let mut fTemp524: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp522 as usize] };
			let mut fTemp525: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp522, 1)) as usize] } - fTemp524;
			let mut fTemp526: F64 = 131071.5 * fTemp518;
			let mut iTemp527: i32 = (fTemp526) as i32;
			let mut iTemp528: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp527, 262143)))), 786431));
			let mut fTemp529: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp528, 3)) as usize] };
			let mut fTemp530: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp528 as usize] };
			let mut fTemp531: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp528, 1)) as usize] } - fTemp530;
			let mut fTemp532: F64 = if iTemp67 != 0 {fTemp530 + fTemp81 * fTemp531 + (fTemp526 - (iTemp527) as F64) * (fTemp529 - (fTemp530 + fTemp81 * (fTemp531 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp528, 4)) as usize] } - fTemp529))))} else {1.0 - (fTemp524 + fTemp81 * fTemp525 + (fTemp520 - (iTemp521) as F64) * (fTemp523 - (fTemp524 + fTemp81 * (fTemp525 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp522, 4)) as usize] } - fTemp523)))))};
			let mut fTemp533: F64 = fTemp86 + fTemp519;
			let mut fTemp534: F64 = 262143.0 * (1.0 - fTemp533);
			let mut iTemp535: i32 = (fTemp534) as i32;
			let mut iTemp536: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp535, 262143)))), 786431));
			let mut fTemp537: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp536, 3)) as usize] };
			let mut fTemp538: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp536 as usize] };
			let mut fTemp539: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp536, 1)) as usize] } - fTemp538;
			let mut fTemp540: F64 = 262143.0 * fTemp533;
			let mut iTemp541: i32 = (fTemp540) as i32;
			let mut iTemp542: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp541, 262143)))), 786431));
			let mut fTemp543: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp542, 3), 786431))) as usize] };
			let mut fTemp544: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp542 as usize] };
			let mut fTemp545: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp542, 1), 786431))) as usize] } - fTemp544;
			let mut iTemp546: i32 = (fTemp142 > ((if iTemp67 != 0 {fTemp544 + fTemp81 * fTemp545 + (fTemp540 - (iTemp541) as F64) * (fTemp543 - (fTemp544 + fTemp81 * (fTemp545 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp542, 4), 786431))) as usize] } - fTemp543))))} else {1.0 - (fTemp538 + fTemp81 * fTemp539 + (fTemp534 - (iTemp535) as F64) * (fTemp537 - (fTemp538 + fTemp81 * (fTemp539 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp536, 4)) as usize] } - fTemp537)))))} - fTemp532) / (1.0 - fTemp532))) as i32;
			let mut fTemp547: F64 = if iTemp546 != 0 {fTemp516} else {fTemp519};
			let mut fTemp548: F64 = if iTemp546 != 0 {fTemp519} else {fTemp517};
			let mut fTemp549: F64 = fTemp548 + fTemp547;
			let mut fTemp550: F64 = 0.5 * fTemp549;
			let mut fTemp551: F64 = 262143.0 * (1.0 - fTemp550);
			let mut iTemp552: i32 = (fTemp551) as i32;
			let mut iTemp553: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp552, 262143)))), 786431));
			let mut fTemp554: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp553, 3)) as usize] };
			let mut fTemp555: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp553 as usize] };
			let mut fTemp556: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp553, 1)) as usize] } - fTemp555;
			let mut fTemp557: F64 = 131071.5 * fTemp549;
			let mut iTemp558: i32 = (fTemp557) as i32;
			let mut iTemp559: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp558, 262143)))), 786431));
			let mut fTemp560: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp559, 3)) as usize] };
			let mut fTemp561: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp559 as usize] };
			let mut fTemp562: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp559, 1)) as usize] } - fTemp561;
			let mut fTemp563: F64 = if iTemp67 != 0 {fTemp561 + fTemp81 * fTemp562 + (fTemp557 - (iTemp558) as F64) * (fTemp560 - (fTemp561 + fTemp81 * (fTemp562 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp559, 4)) as usize] } - fTemp560))))} else {1.0 - (fTemp555 + fTemp81 * fTemp556 + (fTemp551 - (iTemp552) as F64) * (fTemp554 - (fTemp555 + fTemp81 * (fTemp556 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp553, 4)) as usize] } - fTemp554)))))};
			let mut fTemp564: F64 = fTemp86 + fTemp550;
			let mut fTemp565: F64 = 262143.0 * (1.0 - fTemp564);
			let mut iTemp566: i32 = (fTemp565) as i32;
			let mut iTemp567: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp566, 262143)))), 786431));
			let mut fTemp568: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp567, 3)) as usize] };
			let mut fTemp569: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp567 as usize] };
			let mut fTemp570: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp567, 1)) as usize] } - fTemp569;
			let mut fTemp571: F64 = 262143.0 * fTemp564;
			let mut iTemp572: i32 = (fTemp571) as i32;
			let mut iTemp573: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp572, 262143)))), 786431));
			let mut fTemp574: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp573, 3), 786431))) as usize] };
			let mut fTemp575: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp573 as usize] };
			let mut fTemp576: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp573, 1), 786431))) as usize] } - fTemp575;
			let mut iTemp577: i32 = (fTemp142 > ((if iTemp67 != 0 {fTemp575 + fTemp81 * fTemp576 + (fTemp571 - (iTemp572) as F64) * (fTemp574 - (fTemp575 + fTemp81 * (fTemp576 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp573, 4), 786431))) as usize] } - fTemp574))))} else {1.0 - (fTemp569 + fTemp81 * fTemp570 + (fTemp565 - (iTemp566) as F64) * (fTemp568 - (fTemp569 + fTemp81 * (fTemp570 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp567, 4)) as usize] } - fTemp568)))))} - fTemp563) / (1.0 - fTemp563))) as i32;
			let mut fTemp578: F64 = if iTemp577 != 0 {fTemp547} else {fTemp550};
			let mut fTemp579: F64 = if iTemp577 != 0 {fTemp550} else {fTemp548};
			let mut fTemp580: F64 = fTemp579 + fTemp578;
			let mut fTemp581: F64 = 0.5 * fTemp580;
			let mut fTemp582: F64 = 262143.0 * (1.0 - fTemp581);
			let mut iTemp583: i32 = (fTemp582) as i32;
			let mut iTemp584: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp583, 262143)))), 786431));
			let mut fTemp585: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp584, 3)) as usize] };
			let mut fTemp586: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp584 as usize] };
			let mut fTemp587: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp584, 1)) as usize] } - fTemp586;
			let mut fTemp588: F64 = 131071.5 * fTemp580;
			let mut iTemp589: i32 = (fTemp588) as i32;
			let mut iTemp590: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp589, 262143)))), 786431));
			let mut fTemp591: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp590, 3)) as usize] };
			let mut fTemp592: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp590 as usize] };
			let mut fTemp593: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp590, 1)) as usize] } - fTemp592;
			let mut fTemp594: F64 = if iTemp67 != 0 {fTemp592 + fTemp81 * fTemp593 + (fTemp588 - (iTemp589) as F64) * (fTemp591 - (fTemp592 + fTemp81 * (fTemp593 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp590, 4)) as usize] } - fTemp591))))} else {1.0 - (fTemp586 + fTemp81 * fTemp587 + (fTemp582 - (iTemp583) as F64) * (fTemp585 - (fTemp586 + fTemp81 * (fTemp587 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp584, 4)) as usize] } - fTemp585)))))};
			let mut fTemp595: F64 = fTemp86 + fTemp581;
			let mut fTemp596: F64 = 262143.0 * (1.0 - fTemp595);
			let mut iTemp597: i32 = (fTemp596) as i32;
			let mut iTemp598: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp597, 262143)))), 786431));
			let mut fTemp599: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp598, 3)) as usize] };
			let mut fTemp600: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp598 as usize] };
			let mut fTemp601: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp598, 1)) as usize] } - fTemp600;
			let mut fTemp602: F64 = 262143.0 * fTemp595;
			let mut iTemp603: i32 = (fTemp602) as i32;
			let mut iTemp604: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp603, 262143)))), 786431));
			let mut fTemp605: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp604, 3), 786431))) as usize] };
			let mut fTemp606: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp604 as usize] };
			let mut fTemp607: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp604, 1), 786431))) as usize] } - fTemp606;
			let mut iTemp608: i32 = (fTemp142 > ((if iTemp67 != 0 {fTemp606 + fTemp81 * fTemp607 + (fTemp602 - (iTemp603) as F64) * (fTemp605 - (fTemp606 + fTemp81 * (fTemp607 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp604, 4), 786431))) as usize] } - fTemp605))))} else {1.0 - (fTemp600 + fTemp81 * fTemp601 + (fTemp596 - (iTemp597) as F64) * (fTemp599 - (fTemp600 + fTemp81 * (fTemp601 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp598, 4)) as usize] } - fTemp599)))))} - fTemp594) / (1.0 - fTemp594))) as i32;
			let mut fTemp609: F64 = if iTemp608 != 0 {fTemp578} else {fTemp581};
			let mut fTemp610: F64 = if iTemp608 != 0 {fTemp581} else {fTemp579};
			let mut fTemp611: F64 = fTemp610 + fTemp609;
			let mut fTemp612: F64 = 0.5 * fTemp611;
			let mut fTemp613: F64 = 262143.0 * (1.0 - fTemp612);
			let mut iTemp614: i32 = (fTemp613) as i32;
			let mut iTemp615: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp614, 262143)))), 786431));
			let mut fTemp616: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp615, 3)) as usize] };
			let mut fTemp617: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp615 as usize] };
			let mut fTemp618: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp615, 1)) as usize] } - fTemp617;
			let mut fTemp619: F64 = 131071.5 * fTemp611;
			let mut iTemp620: i32 = (fTemp619) as i32;
			let mut iTemp621: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp620, 262143)))), 786431));
			let mut fTemp622: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp621, 3)) as usize] };
			let mut fTemp623: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp621 as usize] };
			let mut fTemp624: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp621, 1)) as usize] } - fTemp623;
			let mut fTemp625: F64 = if iTemp67 != 0 {fTemp623 + fTemp81 * fTemp624 + (fTemp619 - (iTemp620) as F64) * (fTemp622 - (fTemp623 + fTemp81 * (fTemp624 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp621, 4)) as usize] } - fTemp622))))} else {1.0 - (fTemp617 + fTemp81 * fTemp618 + (fTemp613 - (iTemp614) as F64) * (fTemp616 - (fTemp617 + fTemp81 * (fTemp618 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp615, 4)) as usize] } - fTemp616)))))};
			let mut fTemp626: F64 = fTemp86 + fTemp612;
			let mut fTemp627: F64 = 262143.0 * (1.0 - fTemp626);
			let mut iTemp628: i32 = (fTemp627) as i32;
			let mut iTemp629: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp628, 262143)))), 786431));
			let mut fTemp630: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp629, 3)) as usize] };
			let mut fTemp631: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp629 as usize] };
			let mut fTemp632: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp629, 1)) as usize] } - fTemp631;
			let mut fTemp633: F64 = 262143.0 * fTemp626;
			let mut iTemp634: i32 = (fTemp633) as i32;
			let mut iTemp635: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp634, 262143)))), 786431));
			let mut fTemp636: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp635, 3), 786431))) as usize] };
			let mut fTemp637: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp635 as usize] };
			let mut fTemp638: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp635, 1), 786431))) as usize] } - fTemp637;
			let mut iTemp639: i32 = (fTemp142 > ((if iTemp67 != 0 {fTemp637 + fTemp81 * fTemp638 + (fTemp633 - (iTemp634) as F64) * (fTemp636 - (fTemp637 + fTemp81 * (fTemp638 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp635, 4), 786431))) as usize] } - fTemp636))))} else {1.0 - (fTemp631 + fTemp81 * fTemp632 + (fTemp627 - (iTemp628) as F64) * (fTemp630 - (fTemp631 + fTemp81 * (fTemp632 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp629, 4)) as usize] } - fTemp630)))))} - fTemp625) / (1.0 - fTemp625))) as i32;
			let mut fTemp640: F64 = if iTemp639 != 0 {fTemp609} else {fTemp612};
			let mut fTemp641: F64 = if iTemp639 != 0 {fTemp612} else {fTemp610};
			let mut fTemp642: F64 = fTemp641 + fTemp640;
			let mut fTemp643: F64 = 0.5 * fTemp642;
			let mut fTemp644: F64 = 262143.0 * (1.0 - fTemp643);
			let mut iTemp645: i32 = (fTemp644) as i32;
			let mut iTemp646: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp645, 262143)))), 786431));
			let mut fTemp647: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp646, 3)) as usize] };
			let mut fTemp648: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp646 as usize] };
			let mut fTemp649: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp646, 1)) as usize] } - fTemp648;
			let mut fTemp650: F64 = 131071.5 * fTemp642;
			let mut iTemp651: i32 = (fTemp650) as i32;
			let mut iTemp652: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp651, 262143)))), 786431));
			let mut fTemp653: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp652, 3)) as usize] };
			let mut fTemp654: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp652 as usize] };
			let mut fTemp655: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp652, 1)) as usize] } - fTemp654;
			let mut fTemp656: F64 = if iTemp67 != 0 {fTemp654 + fTemp81 * fTemp655 + (fTemp650 - (iTemp651) as F64) * (fTemp653 - (fTemp654 + fTemp81 * (fTemp655 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp652, 4), 786431))) as usize] } - fTemp653))))} else {1.0 - (fTemp648 + fTemp81 * fTemp649 + (fTemp644 - (iTemp645) as F64) * (fTemp647 - (fTemp648 + fTemp81 * (fTemp649 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp646, 4), 786431))) as usize] } - fTemp647)))))};
			let mut fTemp657: F64 = fTemp86 + fTemp643;
			let mut fTemp658: F64 = 262143.0 * (1.0 - fTemp657);
			let mut iTemp659: i32 = (fTemp658) as i32;
			let mut iTemp660: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp659, 262143)))), 786431));
			let mut fTemp661: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp660, 3)) as usize] };
			let mut fTemp662: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp660 as usize] };
			let mut fTemp663: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp660, 1)) as usize] } - fTemp662;
			let mut fTemp664: F64 = 262143.0 * fTemp657;
			let mut iTemp665: i32 = (fTemp664) as i32;
			let mut iTemp666: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp665, 262143)))), 786431));
			let mut fTemp667: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp666, 3), 786431))) as usize] };
			let mut fTemp668: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp666 as usize] };
			let mut fTemp669: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp666, 1), 786431))) as usize] } - fTemp668;
			let mut iTemp670: i32 = (fTemp142 > ((if iTemp67 != 0 {fTemp668 + fTemp81 * fTemp669 + (fTemp664 - (iTemp665) as F64) * (fTemp667 - (fTemp668 + fTemp81 * (fTemp669 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp666, 4), 786431))) as usize] } - fTemp667))))} else {1.0 - (fTemp662 + fTemp81 * fTemp663 + (fTemp658 - (iTemp659) as F64) * (fTemp661 - (fTemp662 + fTemp81 * (fTemp663 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp660, 4)) as usize] } - fTemp661)))))} - fTemp656) / (1.0 - fTemp656))) as i32;
			let mut fTemp671: F64 = F64::min(1.0, F64::max(0.0, 0.5 * (if iTemp670 != 0 {fTemp643} else {fTemp641} + if iTemp670 != 0 {fTemp640} else {fTemp643})));
			self.fRec1[0] = fTemp671;
			let mut fTemp672: F64 = 262143.0 * (1.0 - fTemp671);
			let mut iTemp673: i32 = (fTemp672) as i32;
			let mut iTemp674: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp673, 262143)))), 786431));
			let mut fTemp675: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp674, 3)) as usize] };
			let mut fTemp676: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp674 as usize] };
			let mut fTemp677: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp674, 1)) as usize] } - fTemp676;
			let mut fTemp678: F64 = 262143.0 * fTemp671;
			let mut iTemp679: i32 = (fTemp678) as i32;
			let mut iTemp680: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp679, 262143)))), 786431));
			let mut fTemp681: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp680, 3)) as usize] };
			let mut fTemp682: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp680 as usize] };
			let mut fTemp683: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp680, 1)) as usize] } - fTemp682;
			let mut fTemp684: F64 = if iTemp67 != 0 {fTemp682 + fTemp81 * fTemp683 + (fTemp678 - (iTemp679) as F64) * (fTemp681 - (fTemp682 + fTemp81 * (fTemp683 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp680, 4), 786431))) as usize] } - fTemp681))))} else {1.0 - (fTemp676 + fTemp81 * fTemp677 + (fTemp672 - (iTemp673) as F64) * (fTemp675 - (fTemp676 + fTemp81 * (fTemp677 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp674, 4), 786431))) as usize] } - fTemp675)))))};
			let mut fTemp685: F64 = fTemp86 + fTemp671;
			let mut fTemp686: F64 = 262143.0 * (1.0 - fTemp685);
			let mut iTemp687: i32 = (fTemp686) as i32;
			let mut iTemp688: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp687, 262143)))), 786431));
			let mut fTemp689: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp688, 3)) as usize] };
			let mut fTemp690: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp688 as usize] };
			let mut fTemp691: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp688, 1)) as usize] } - fTemp690;
			let mut fTemp692: F64 = 262143.0 * fTemp685;
			let mut iTemp693: i32 = (fTemp692) as i32;
			let mut iTemp694: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp693, 262143)))), 786431));
			let mut fTemp695: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, 3), 786431))) as usize] };
			let mut fTemp696: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp694 as usize] };
			let mut fTemp697: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, 1), 786431))) as usize] } - fTemp696;
			let mut fTemp698: F64 = fTemp5 + if ((0.001 * fTemp85) == 0.0) as i32 != 0 {fTemp66} else {fTemp66 * (if iTemp67 != 0 {fTemp696 + fTemp81 * fTemp697 + (fTemp692 - (iTemp693) as F64) * (fTemp695 - (fTemp696 + fTemp81 * (fTemp697 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp694, 4), 786431))) as usize] } - fTemp695))))} else {1.0 - (fTemp690 + fTemp81 * fTemp691 + (fTemp686 - (iTemp687) as F64) * (fTemp689 - (fTemp690 + fTemp81 * (fTemp691 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp688, 4)) as usize] } - fTemp689)))))} - fTemp684) / (1.0 - fTemp684)};
			self.fRec2[(self.IOTA0 & 32767) as usize] = if iTemp84 != 0 {F64::min(fTemp698, fTemp5)} else {F64::max(fTemp698, fTemp5)};
			let mut fTemp699: F64 = self.fRec2[((i32::wrapping_sub(self.IOTA0, iSlow79)) & 32767) as usize];
			self.fRec14[0] = fSlow80 + self.fConst4 * self.fRec14[1];
			*output0 = 0.5 * self.fVec1[((i32::wrapping_sub(self.IOTA0, iSlow18)) & 32767) as usize] * fTemp2 + self.fRec14[0] * self.fVec2[((i32::wrapping_sub(self.IOTA0, iSlow18)) & 32767) as usize] * fTemp699 * fTemp4;
			let mut fTemp700: F64 = self.fRec16[((i32::wrapping_sub(self.IOTA0, 1)) & 32767) as usize];
			let mut fTemp701: F64 = fTemp36 + fSlow21 * (fTemp37 - fTemp36);
			let mut iTemp702: i32 = ((fTemp701 > fSlow11) as i32) + ((fTemp701 > fSlow9) as i32);
			let mut fTemp703: F64 = fTemp701 - fSlow8;
			let mut fTemp704: F64 = F64::min(fTemp34, F64::powf(1e+01, -(fSlow22 * F64::max(0.0, if (iTemp702 == 0) as i32 != 0 {0.0} else {if (iTemp702 == 1) as i32 != 0 {fSlow12 * LambRs192k_faustpower2_f(fSlow7 + fTemp703)} else {fTemp703}}))));
			self.fVec35[(self.IOTA0 & 16383) as usize] = fTemp704;
			let mut fTemp705: F64 = F64::min(fTemp704, self.fVec35[((i32::wrapping_sub(self.IOTA0, 1)) & 16383) as usize]);
			self.fVec36[0] = fTemp705;
			let mut fTemp706: F64 = F64::min(fTemp705, self.fVec36[2]);
			self.fVec37[0] = fTemp706;
			let mut fTemp707: F64 = F64::min(fTemp706, self.fVec37[4]);
			self.fVec38[0] = fTemp707;
			let mut fTemp708: F64 = F64::min(fTemp707, self.fVec38[8]);
			self.fVec39[(self.IOTA0 & 31) as usize] = fTemp708;
			let mut fTemp709: F64 = F64::min(fTemp708, self.fVec39[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize]);
			self.fVec40[(self.IOTA0 & 63) as usize] = fTemp709;
			let mut fTemp710: F64 = F64::min(fTemp709, self.fVec40[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize]);
			self.fVec41[(self.IOTA0 & 127) as usize] = fTemp710;
			let mut fTemp711: F64 = F64::min(fTemp710, self.fVec41[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize]);
			self.fVec42[(self.IOTA0 & 255) as usize] = fTemp711;
			let mut fTemp712: F64 = F64::min(fTemp711, self.fVec42[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize]);
			self.fVec43[(self.IOTA0 & 511) as usize] = fTemp712;
			let mut fTemp713: F64 = F64::min(fTemp712, self.fVec43[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize]);
			self.fVec44[(self.IOTA0 & 1023) as usize] = fTemp713;
			let mut fTemp714: F64 = F64::min(fTemp713, self.fVec44[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize]);
			self.fVec45[(self.IOTA0 & 2047) as usize] = fTemp714;
			let mut fTemp715: F64 = F64::min(fTemp714, self.fVec45[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			self.fVec46[(self.IOTA0 & 4095) as usize] = fTemp715;
			let mut fTemp716: F64 = F64::min(fTemp715, self.fVec46[((i32::wrapping_sub(self.IOTA0, 2048)) & 4095) as usize]);
			self.fVec47[(self.IOTA0 & 8191) as usize] = fTemp716;
			self.fVec48[(self.IOTA0 & 16383) as usize] = F64::min(fTemp716, self.fVec47[((i32::wrapping_sub(self.IOTA0, 4096)) & 8191) as usize]);
			self.fRec17[0] = F64::max(F64::min(self.fRec17[1], self.fVec35[((i32::wrapping_sub(self.IOTA0, iSlow24)) & 16383) as usize]), F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow26 != 0 {fTemp704} else {1.7976931348623157e+308}, if iSlow27 != 0 {self.fVec36[iSlow26 as usize]} else {1.7976931348623157e+308}), if iSlow28 != 0 {self.fVec37[iSlow29 as usize]} else {1.7976931348623157e+308}), if iSlow30 != 0 {self.fVec38[iSlow31 as usize]} else {1.7976931348623157e+308}), if iSlow32 != 0 {self.fVec39[((i32::wrapping_sub(self.IOTA0, iSlow33)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow34 != 0 {self.fVec40[((i32::wrapping_sub(self.IOTA0, iSlow35)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow36 != 0 {self.fVec41[((i32::wrapping_sub(self.IOTA0, iSlow37)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow38 != 0 {self.fVec42[((i32::wrapping_sub(self.IOTA0, iSlow39)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow40 != 0 {self.fVec43[((i32::wrapping_sub(self.IOTA0, iSlow41)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow42 != 0 {self.fVec44[((i32::wrapping_sub(self.IOTA0, iSlow43)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow44 != 0 {self.fVec45[((i32::wrapping_sub(self.IOTA0, iSlow45)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow46 != 0 {self.fVec46[((i32::wrapping_sub(self.IOTA0, iSlow47)) & 4095) as usize]} else {1.7976931348623157e+308}), if iSlow48 != 0 {self.fVec47[((i32::wrapping_sub(self.IOTA0, iSlow49)) & 8191) as usize]} else {1.7976931348623157e+308}), if iSlow50 != 0 {self.fVec48[((i32::wrapping_sub(self.IOTA0, iSlow51)) & 16383) as usize]} else {1.7976931348623157e+308}));
			let mut fTemp717: F64 = F64::min(self.fRec17[0], self.fRec17[1]);
			self.fVec49[0] = fTemp717;
			let mut fTemp718: F64 = F64::min(fTemp717, self.fVec49[2]);
			self.fVec50[0] = fTemp718;
			let mut fTemp719: F64 = F64::min(fTemp718, self.fVec50[4]);
			self.fVec51[0] = fTemp719;
			let mut fTemp720: F64 = F64::min(fTemp719, self.fVec51[8]);
			self.fVec52[(self.IOTA0 & 31) as usize] = fTemp720;
			let mut fTemp721: F64 = F64::min(fTemp720, self.fVec52[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize]);
			self.fVec53[(self.IOTA0 & 63) as usize] = fTemp721;
			let mut fTemp722: F64 = F64::min(fTemp721, self.fVec53[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize]);
			self.fVec54[(self.IOTA0 & 127) as usize] = fTemp722;
			let mut fTemp723: F64 = F64::min(fTemp722, self.fVec54[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize]);
			self.fVec55[(self.IOTA0 & 255) as usize] = fTemp723;
			let mut fTemp724: F64 = F64::min(fTemp723, self.fVec55[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize]);
			self.fVec56[(self.IOTA0 & 511) as usize] = fTemp724;
			let mut fTemp725: F64 = F64::min(fTemp724, self.fVec56[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize]);
			self.fVec57[(self.IOTA0 & 1023) as usize] = fTemp725;
			let mut fTemp726: F64 = F64::min(fTemp725, self.fVec57[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize]);
			self.fVec58[(self.IOTA0 & 2047) as usize] = fTemp726;
			let mut fTemp727: F64 = F64::min(fTemp726, self.fVec58[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			self.fVec59[(self.IOTA0 & 4095) as usize] = fTemp727;
			let mut fTemp728: F64 = F64::min(fTemp727, self.fVec59[((i32::wrapping_sub(self.IOTA0, 2048)) & 4095) as usize]);
			self.fVec60[(self.IOTA0 & 8191) as usize] = fTemp728;
			self.fVec61[(self.IOTA0 & 16383) as usize] = F64::min(fTemp728, self.fVec60[((i32::wrapping_sub(self.IOTA0, 4096)) & 8191) as usize]);
			let mut fTemp729: F64 = F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow4 != 0 {self.fRec17[0]} else {1.7976931348623157e+308}, if iSlow52 != 0 {self.fVec49[iSlow4 as usize]} else {1.7976931348623157e+308}), if iSlow53 != 0 {self.fVec50[iSlow54 as usize]} else {1.7976931348623157e+308}), if iSlow55 != 0 {self.fVec51[iSlow56 as usize]} else {1.7976931348623157e+308}), if iSlow57 != 0 {self.fVec52[((i32::wrapping_sub(self.IOTA0, iSlow58)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow59 != 0 {self.fVec53[((i32::wrapping_sub(self.IOTA0, iSlow60)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow61 != 0 {self.fVec54[((i32::wrapping_sub(self.IOTA0, iSlow62)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow63 != 0 {self.fVec55[((i32::wrapping_sub(self.IOTA0, iSlow64)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow65 != 0 {self.fVec56[((i32::wrapping_sub(self.IOTA0, iSlow66)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow67 != 0 {self.fVec57[((i32::wrapping_sub(self.IOTA0, iSlow68)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow69 != 0 {self.fVec58[((i32::wrapping_sub(self.IOTA0, iSlow70)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow71 != 0 {self.fVec59[((i32::wrapping_sub(self.IOTA0, iSlow72)) & 4095) as usize]} else {1.7976931348623157e+308}), if iSlow73 != 0 {self.fVec60[((i32::wrapping_sub(self.IOTA0, iSlow74)) & 8191) as usize]} else {1.7976931348623157e+308}), if iSlow75 != 0 {self.fVec61[((i32::wrapping_sub(self.IOTA0, iSlow76)) & 16383) as usize]} else {1.7976931348623157e+308}) - fTemp700;
			self.fVec62[0] = fTemp729;
			let mut iTemp730: i32 = (fTemp729 > 0.0) as i32;
			let mut fTemp731: F64 = if iTemp730 != 0 {fSlow78} else {fSlow77};
			self.fVec63[0] = fTemp731;
			let mut fTemp732: F64 = 2.0 * fTemp731;
			let mut iTemp733: i32 = (fTemp732) as i32;
			let mut iTemp734: i32 = std::cmp::max(0, std::cmp::min(iTemp733, 2));
			let mut iTemp735: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, 393213), 786431));
			let mut fTemp736: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp735, 3)) as usize] };
			let mut fTemp737: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp735 as usize] };
			let mut fTemp738: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp735, 1)) as usize] } - fTemp737;
			let mut fTemp739: F64 = fTemp732 - (iTemp733) as F64;
			let mut fTemp740: F64 = fTemp737 + fTemp739 * fTemp738 + 0.5 * (fTemp736 - (fTemp737 + fTemp739 * (fTemp738 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp735, 4)) as usize] } - fTemp736))));
			let mut fTemp741: F64 = if iTemp730 != 0 {fTemp740} else {1.0 - fTemp740};
			let mut iTemp742: i32 = (fTemp729 < 0.0) as i32;
			let mut fTemp743: F64 = fSlow1 * (iTemp742) as F64 + fSlow13 * (iTemp730) as F64;
			self.fVec64[0] = fTemp743;
			let mut fTemp744: F64 = self.fConst10 / fTemp743;
			let mut fTemp745: F64 = fTemp744 + 0.5;
			let mut fTemp746: F64 = 262143.0 * (1.0 - fTemp745);
			let mut iTemp747: i32 = (fTemp746) as i32;
			let mut iTemp748: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp747, 262143)))), 786431));
			let mut fTemp749: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp748, 3)) as usize] };
			let mut fTemp750: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp748 as usize] };
			let mut fTemp751: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp748, 1)) as usize] } - fTemp750;
			let mut fTemp752: F64 = 262143.0 * fTemp745;
			let mut iTemp753: i32 = (fTemp752) as i32;
			let mut iTemp754: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp753, 262143)))), 786431));
			let mut fTemp755: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp754, 3), 786431))) as usize] };
			let mut fTemp756: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp754 as usize] };
			let mut fTemp757: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp754, 1), 786431))) as usize] } - fTemp756;
			let mut fTemp758: F64 = 2.0 * self.fVec63[1];
			let mut iTemp759: i32 = (fTemp758) as i32;
			let mut iTemp760: i32 = std::cmp::max(0, std::cmp::min(iTemp759, 2));
			let mut fTemp761: F64 = 262143.0 * (1.0 - self.fRec15[1]);
			let mut iTemp762: i32 = (fTemp761) as i32;
			let mut iTemp763: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp762, 262143))), iTemp760), 786431));
			let mut fTemp764: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp763, 3), 786431))) as usize] };
			let mut fTemp765: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp763 as usize] };
			let mut fTemp766: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp763, 1), 786431))) as usize] } - fTemp765;
			let mut fTemp767: F64 = fTemp758 - (iTemp759) as F64;
			let mut fTemp768: F64 = 262143.0 * self.fRec15[1];
			let mut iTemp769: i32 = (fTemp768) as i32;
			let mut iTemp770: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp760, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp769, 262143)))), 786431));
			let mut fTemp771: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp770, 3), 786431))) as usize] };
			let mut fTemp772: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp770 as usize] };
			let mut fTemp773: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp770, 1), 786431))) as usize] } - fTemp772;
			let mut fTemp774: F64 = self.fRec15[1] + fTemp744;
			let mut fTemp775: F64 = 262143.0 * (1.0 - fTemp774);
			let mut iTemp776: i32 = (fTemp775) as i32;
			let mut iTemp777: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp776, 262143)))), 786431));
			let mut fTemp778: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp777, 3)) as usize] };
			let mut fTemp779: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp777 as usize] };
			let mut fTemp780: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp777, 1)) as usize] } - fTemp779;
			let mut fTemp781: F64 = 262143.0 * fTemp774;
			let mut iTemp782: i32 = (fTemp781) as i32;
			let mut iTemp783: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp782, 262143)))), 786431));
			let mut fTemp784: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp783, 3), 786431))) as usize] };
			let mut fTemp785: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp783 as usize] };
			let mut fTemp786: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp783, 1), 786431))) as usize] } - fTemp785;
			let mut fTemp787: F64 = self.fRec15[1] + self.fConst10 * (1.0 / fTemp743 + 1.0 / self.fVec64[1]);
			let mut fTemp788: F64 = 262143.0 * (1.0 - fTemp787);
			let mut iTemp789: i32 = (fTemp788) as i32;
			let mut iTemp790: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp789, 262143))), iTemp734), 786431));
			let mut fTemp791: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp790, 3)) as usize] };
			let mut fTemp792: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp790 as usize] };
			let mut fTemp793: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp790, 1)) as usize] } - fTemp792;
			let mut fTemp794: F64 = 262143.0 * fTemp787;
			let mut iTemp795: i32 = (fTemp794) as i32;
			let mut iTemp796: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp795, 262143)))), 786431));
			let mut fTemp797: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp796, 3), 786431))) as usize] };
			let mut fTemp798: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp796 as usize] };
			let mut fTemp799: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp796, 1), 786431))) as usize] } - fTemp798;
			let mut fTemp800: F64 = (if iTemp730 != 0 {fTemp798 + fTemp739 * fTemp799 + (fTemp794 - (iTemp795) as F64) * (fTemp797 - (fTemp798 + fTemp739 * (fTemp799 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp796, 4), 786431))) as usize] } - fTemp797))))} else {1.0 - (fTemp792 + fTemp739 * fTemp793 + (fTemp788 - (iTemp789) as F64) * (fTemp791 - (fTemp792 + fTemp739 * (fTemp793 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp790, 4)) as usize] } - fTemp791)))))} - if iTemp730 != 0 {fTemp785 + fTemp739 * fTemp786 + (fTemp781 - (iTemp782) as F64) * (fTemp784 - (fTemp785 + fTemp739 * (fTemp786 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp783, 4), 786431))) as usize] } - fTemp784))))} else {1.0 - (fTemp779 + fTemp739 * fTemp780 + (fTemp775 - (iTemp776) as F64) * (fTemp778 - (fTemp779 + fTemp739 * (fTemp780 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp777, 4)) as usize] } - fTemp778)))))}) * self.fVec62[1] / (fTemp729 * (1.0 - if iTemp730 != 0 {fTemp772 + fTemp767 * fTemp773 + (fTemp768 - (iTemp769) as F64) * (fTemp771 - (fTemp772 + fTemp767 * (fTemp773 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp770, 4), 786431))) as usize] } - fTemp771))))} else {1.0 - (fTemp765 + fTemp767 * fTemp766 + (fTemp761 - (iTemp762) as F64) * (fTemp764 - (fTemp765 + fTemp767 * (fTemp766 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp763, 4), 786431))) as usize] } - fTemp764)))))}));
			let mut iTemp801: i32 = (fTemp800 > ((if iTemp730 != 0 {fTemp756 + fTemp739 * fTemp757 + (fTemp752 - (iTemp753) as F64) * (fTemp755 - (fTemp756 + fTemp739 * (fTemp757 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp754, 4), 786431))) as usize] } - fTemp755))))} else {1.0 - (fTemp750 + fTemp739 * fTemp751 + (fTemp746 - (iTemp747) as F64) * (fTemp749 - (fTemp750 + fTemp739 * (fTemp751 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp748, 4)) as usize] } - fTemp749)))))} - fTemp741) / (1.0 - fTemp741))) as i32;
			let mut fTemp802: F64 = if iTemp801 != 0 {1.0} else {0.5};
			let mut fTemp803: F64 = if iTemp801 != 0 {0.5} else {0.0};
			let mut fTemp804: F64 = fTemp803 + fTemp802;
			let mut fTemp805: F64 = 0.5 * fTemp804;
			let mut fTemp806: F64 = 262143.0 * (1.0 - fTemp805);
			let mut iTemp807: i32 = (fTemp806) as i32;
			let mut iTemp808: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp807, 262143)))), 786431));
			let mut fTemp809: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp808, 3)) as usize] };
			let mut fTemp810: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp808 as usize] };
			let mut fTemp811: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp808, 1)) as usize] } - fTemp810;
			let mut fTemp812: F64 = 131071.5 * fTemp804;
			let mut iTemp813: i32 = (fTemp812) as i32;
			let mut iTemp814: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp813, 262143)))), 786431));
			let mut fTemp815: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp814, 3)) as usize] };
			let mut fTemp816: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp814 as usize] };
			let mut fTemp817: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp814, 1)) as usize] } - fTemp816;
			let mut fTemp818: F64 = if iTemp730 != 0 {fTemp816 + fTemp739 * fTemp817 + (fTemp812 - (iTemp813) as F64) * (fTemp815 - (fTemp816 + fTemp739 * (fTemp817 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp814, 4)) as usize] } - fTemp815))))} else {1.0 - (fTemp810 + fTemp739 * fTemp811 + (fTemp806 - (iTemp807) as F64) * (fTemp809 - (fTemp810 + fTemp739 * (fTemp811 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp808, 4)) as usize] } - fTemp809)))))};
			let mut fTemp819: F64 = fTemp744 + fTemp805;
			let mut fTemp820: F64 = 262143.0 * (1.0 - fTemp819);
			let mut iTemp821: i32 = (fTemp820) as i32;
			let mut iTemp822: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp821, 262143)))), 786431));
			let mut fTemp823: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp822, 3)) as usize] };
			let mut fTemp824: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp822 as usize] };
			let mut fTemp825: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp822, 1)) as usize] } - fTemp824;
			let mut fTemp826: F64 = 262143.0 * fTemp819;
			let mut iTemp827: i32 = (fTemp826) as i32;
			let mut iTemp828: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp827, 262143)))), 786431));
			let mut fTemp829: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp828, 3), 786431))) as usize] };
			let mut fTemp830: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp828 as usize] };
			let mut fTemp831: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp828, 1), 786431))) as usize] } - fTemp830;
			let mut iTemp832: i32 = (fTemp800 > ((if iTemp730 != 0 {fTemp830 + fTemp739 * fTemp831 + (fTemp826 - (iTemp827) as F64) * (fTemp829 - (fTemp830 + fTemp739 * (fTemp831 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp828, 4), 786431))) as usize] } - fTemp829))))} else {1.0 - (fTemp824 + fTemp739 * fTemp825 + (fTemp820 - (iTemp821) as F64) * (fTemp823 - (fTemp824 + fTemp739 * (fTemp825 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp822, 4)) as usize] } - fTemp823)))))} - fTemp818) / (1.0 - fTemp818))) as i32;
			let mut fTemp833: F64 = if iTemp832 != 0 {fTemp802} else {fTemp805};
			let mut fTemp834: F64 = if iTemp832 != 0 {fTemp805} else {fTemp803};
			let mut fTemp835: F64 = fTemp834 + fTemp833;
			let mut fTemp836: F64 = 0.5 * fTemp835;
			let mut fTemp837: F64 = 262143.0 * (1.0 - fTemp836);
			let mut iTemp838: i32 = (fTemp837) as i32;
			let mut iTemp839: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp838, 262143)))), 786431));
			let mut fTemp840: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp839, 3)) as usize] };
			let mut fTemp841: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp839 as usize] };
			let mut fTemp842: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp839, 1)) as usize] } - fTemp841;
			let mut fTemp843: F64 = 131071.5 * fTemp835;
			let mut iTemp844: i32 = (fTemp843) as i32;
			let mut iTemp845: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp844, 262143)))), 786431));
			let mut fTemp846: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp845, 3)) as usize] };
			let mut fTemp847: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp845 as usize] };
			let mut fTemp848: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp845, 1)) as usize] } - fTemp847;
			let mut fTemp849: F64 = if iTemp730 != 0 {fTemp847 + fTemp739 * fTemp848 + (fTemp843 - (iTemp844) as F64) * (fTemp846 - (fTemp847 + fTemp739 * (fTemp848 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp845, 4)) as usize] } - fTemp846))))} else {1.0 - (fTemp841 + fTemp739 * fTemp842 + (fTemp837 - (iTemp838) as F64) * (fTemp840 - (fTemp841 + fTemp739 * (fTemp842 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp839, 4)) as usize] } - fTemp840)))))};
			let mut fTemp850: F64 = fTemp744 + fTemp836;
			let mut fTemp851: F64 = 262143.0 * (1.0 - fTemp850);
			let mut iTemp852: i32 = (fTemp851) as i32;
			let mut iTemp853: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp852, 262143)))), 786431));
			let mut fTemp854: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp853, 3)) as usize] };
			let mut fTemp855: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp853 as usize] };
			let mut fTemp856: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp853, 1)) as usize] } - fTemp855;
			let mut fTemp857: F64 = 262143.0 * fTemp850;
			let mut iTemp858: i32 = (fTemp857) as i32;
			let mut iTemp859: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp858, 262143)))), 786431));
			let mut fTemp860: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp859, 3), 786431))) as usize] };
			let mut fTemp861: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp859 as usize] };
			let mut fTemp862: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp859, 1), 786431))) as usize] } - fTemp861;
			let mut iTemp863: i32 = (fTemp800 > ((if iTemp730 != 0 {fTemp861 + fTemp739 * fTemp862 + (fTemp857 - (iTemp858) as F64) * (fTemp860 - (fTemp861 + fTemp739 * (fTemp862 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp859, 4), 786431))) as usize] } - fTemp860))))} else {1.0 - (fTemp855 + fTemp739 * fTemp856 + (fTemp851 - (iTemp852) as F64) * (fTemp854 - (fTemp855 + fTemp739 * (fTemp856 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp853, 4)) as usize] } - fTemp854)))))} - fTemp849) / (1.0 - fTemp849))) as i32;
			let mut fTemp864: F64 = if iTemp863 != 0 {fTemp833} else {fTemp836};
			let mut fTemp865: F64 = if iTemp863 != 0 {fTemp836} else {fTemp834};
			let mut fTemp866: F64 = fTemp865 + fTemp864;
			let mut fTemp867: F64 = 0.5 * fTemp866;
			let mut fTemp868: F64 = 262143.0 * (1.0 - fTemp867);
			let mut iTemp869: i32 = (fTemp868) as i32;
			let mut iTemp870: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp869, 262143)))), 786431));
			let mut fTemp871: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp870, 3)) as usize] };
			let mut fTemp872: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp870 as usize] };
			let mut fTemp873: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp870, 1)) as usize] } - fTemp872;
			let mut fTemp874: F64 = 131071.5 * fTemp866;
			let mut iTemp875: i32 = (fTemp874) as i32;
			let mut iTemp876: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp875, 262143)))), 786431));
			let mut fTemp877: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp876, 3)) as usize] };
			let mut fTemp878: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp876 as usize] };
			let mut fTemp879: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp876, 1)) as usize] } - fTemp878;
			let mut fTemp880: F64 = if iTemp730 != 0 {fTemp878 + fTemp739 * fTemp879 + (fTemp874 - (iTemp875) as F64) * (fTemp877 - (fTemp878 + fTemp739 * (fTemp879 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp876, 4)) as usize] } - fTemp877))))} else {1.0 - (fTemp872 + fTemp739 * fTemp873 + (fTemp868 - (iTemp869) as F64) * (fTemp871 - (fTemp872 + fTemp739 * (fTemp873 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp870, 4)) as usize] } - fTemp871)))))};
			let mut fTemp881: F64 = fTemp744 + fTemp867;
			let mut fTemp882: F64 = 262143.0 * (1.0 - fTemp881);
			let mut iTemp883: i32 = (fTemp882) as i32;
			let mut iTemp884: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp883, 262143)))), 786431));
			let mut fTemp885: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp884, 3)) as usize] };
			let mut fTemp886: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp884 as usize] };
			let mut fTemp887: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp884, 1)) as usize] } - fTemp886;
			let mut fTemp888: F64 = 262143.0 * fTemp881;
			let mut iTemp889: i32 = (fTemp888) as i32;
			let mut iTemp890: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp889, 262143)))), 786431));
			let mut fTemp891: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp890, 3), 786431))) as usize] };
			let mut fTemp892: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp890 as usize] };
			let mut fTemp893: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp890, 1), 786431))) as usize] } - fTemp892;
			let mut iTemp894: i32 = (fTemp800 > ((if iTemp730 != 0 {fTemp892 + fTemp739 * fTemp893 + (fTemp888 - (iTemp889) as F64) * (fTemp891 - (fTemp892 + fTemp739 * (fTemp893 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp890, 4), 786431))) as usize] } - fTemp891))))} else {1.0 - (fTemp886 + fTemp739 * fTemp887 + (fTemp882 - (iTemp883) as F64) * (fTemp885 - (fTemp886 + fTemp739 * (fTemp887 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp884, 4)) as usize] } - fTemp885)))))} - fTemp880) / (1.0 - fTemp880))) as i32;
			let mut fTemp895: F64 = if iTemp894 != 0 {fTemp864} else {fTemp867};
			let mut fTemp896: F64 = if iTemp894 != 0 {fTemp867} else {fTemp865};
			let mut fTemp897: F64 = fTemp896 + fTemp895;
			let mut fTemp898: F64 = 0.5 * fTemp897;
			let mut fTemp899: F64 = 262143.0 * (1.0 - fTemp898);
			let mut iTemp900: i32 = (fTemp899) as i32;
			let mut iTemp901: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp900, 262143)))), 786431));
			let mut fTemp902: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp901, 3)) as usize] };
			let mut fTemp903: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp901 as usize] };
			let mut fTemp904: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp901, 1)) as usize] } - fTemp903;
			let mut fTemp905: F64 = 131071.5 * fTemp897;
			let mut iTemp906: i32 = (fTemp905) as i32;
			let mut iTemp907: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp906, 262143)))), 786431));
			let mut fTemp908: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp907, 3)) as usize] };
			let mut fTemp909: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp907 as usize] };
			let mut fTemp910: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp907, 1)) as usize] } - fTemp909;
			let mut fTemp911: F64 = if iTemp730 != 0 {fTemp909 + fTemp739 * fTemp910 + (fTemp905 - (iTemp906) as F64) * (fTemp908 - (fTemp909 + fTemp739 * (fTemp910 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp907, 4)) as usize] } - fTemp908))))} else {1.0 - (fTemp903 + fTemp739 * fTemp904 + (fTemp899 - (iTemp900) as F64) * (fTemp902 - (fTemp903 + fTemp739 * (fTemp904 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp901, 4)) as usize] } - fTemp902)))))};
			let mut fTemp912: F64 = fTemp744 + fTemp898;
			let mut fTemp913: F64 = 262143.0 * (1.0 - fTemp912);
			let mut iTemp914: i32 = (fTemp913) as i32;
			let mut iTemp915: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp914, 262143)))), 786431));
			let mut fTemp916: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp915, 3)) as usize] };
			let mut fTemp917: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp915 as usize] };
			let mut fTemp918: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp915, 1)) as usize] } - fTemp917;
			let mut fTemp919: F64 = 262143.0 * fTemp912;
			let mut iTemp920: i32 = (fTemp919) as i32;
			let mut iTemp921: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp920, 262143)))), 786431));
			let mut fTemp922: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp921, 3), 786431))) as usize] };
			let mut fTemp923: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp921 as usize] };
			let mut fTemp924: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp921, 1), 786431))) as usize] } - fTemp923;
			let mut iTemp925: i32 = (fTemp800 > ((if iTemp730 != 0 {fTemp923 + fTemp739 * fTemp924 + (fTemp919 - (iTemp920) as F64) * (fTemp922 - (fTemp923 + fTemp739 * (fTemp924 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp921, 4), 786431))) as usize] } - fTemp922))))} else {1.0 - (fTemp917 + fTemp739 * fTemp918 + (fTemp913 - (iTemp914) as F64) * (fTemp916 - (fTemp917 + fTemp739 * (fTemp918 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp915, 4)) as usize] } - fTemp916)))))} - fTemp911) / (1.0 - fTemp911))) as i32;
			let mut fTemp926: F64 = if iTemp925 != 0 {fTemp895} else {fTemp898};
			let mut fTemp927: F64 = if iTemp925 != 0 {fTemp898} else {fTemp896};
			let mut fTemp928: F64 = fTemp927 + fTemp926;
			let mut fTemp929: F64 = 0.5 * fTemp928;
			let mut fTemp930: F64 = 262143.0 * (1.0 - fTemp929);
			let mut iTemp931: i32 = (fTemp930) as i32;
			let mut iTemp932: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp931, 262143)))), 786431));
			let mut fTemp933: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp932, 3)) as usize] };
			let mut fTemp934: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp932 as usize] };
			let mut fTemp935: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp932, 1)) as usize] } - fTemp934;
			let mut fTemp936: F64 = 131071.5 * fTemp928;
			let mut iTemp937: i32 = (fTemp936) as i32;
			let mut iTemp938: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp937, 262143)))), 786431));
			let mut fTemp939: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp938, 3)) as usize] };
			let mut fTemp940: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp938 as usize] };
			let mut fTemp941: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp938, 1)) as usize] } - fTemp940;
			let mut fTemp942: F64 = if iTemp730 != 0 {fTemp940 + fTemp739 * fTemp941 + (fTemp936 - (iTemp937) as F64) * (fTemp939 - (fTemp940 + fTemp739 * (fTemp941 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp938, 4)) as usize] } - fTemp939))))} else {1.0 - (fTemp934 + fTemp739 * fTemp935 + (fTemp930 - (iTemp931) as F64) * (fTemp933 - (fTemp934 + fTemp739 * (fTemp935 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp932, 4)) as usize] } - fTemp933)))))};
			let mut fTemp943: F64 = fTemp744 + fTemp929;
			let mut fTemp944: F64 = 262143.0 * (1.0 - fTemp943);
			let mut iTemp945: i32 = (fTemp944) as i32;
			let mut iTemp946: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp945, 262143)))), 786431));
			let mut fTemp947: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp946, 3)) as usize] };
			let mut fTemp948: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp946 as usize] };
			let mut fTemp949: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp946, 1)) as usize] } - fTemp948;
			let mut fTemp950: F64 = 262143.0 * fTemp943;
			let mut iTemp951: i32 = (fTemp950) as i32;
			let mut iTemp952: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp951, 262143)))), 786431));
			let mut fTemp953: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp952, 3), 786431))) as usize] };
			let mut fTemp954: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp952 as usize] };
			let mut fTemp955: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp952, 1), 786431))) as usize] } - fTemp954;
			let mut iTemp956: i32 = (fTemp800 > ((if iTemp730 != 0 {fTemp954 + fTemp739 * fTemp955 + (fTemp950 - (iTemp951) as F64) * (fTemp953 - (fTemp954 + fTemp739 * (fTemp955 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp952, 4), 786431))) as usize] } - fTemp953))))} else {1.0 - (fTemp948 + fTemp739 * fTemp949 + (fTemp944 - (iTemp945) as F64) * (fTemp947 - (fTemp948 + fTemp739 * (fTemp949 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp946, 4)) as usize] } - fTemp947)))))} - fTemp942) / (1.0 - fTemp942))) as i32;
			let mut fTemp957: F64 = if iTemp956 != 0 {fTemp926} else {fTemp929};
			let mut fTemp958: F64 = if iTemp956 != 0 {fTemp929} else {fTemp927};
			let mut fTemp959: F64 = fTemp958 + fTemp957;
			let mut fTemp960: F64 = 0.5 * fTemp959;
			let mut fTemp961: F64 = 262143.0 * (1.0 - fTemp960);
			let mut iTemp962: i32 = (fTemp961) as i32;
			let mut iTemp963: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp962, 262143)))), 786431));
			let mut fTemp964: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp963, 3)) as usize] };
			let mut fTemp965: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp963 as usize] };
			let mut fTemp966: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp963, 1)) as usize] } - fTemp965;
			let mut fTemp967: F64 = 131071.5 * fTemp959;
			let mut iTemp968: i32 = (fTemp967) as i32;
			let mut iTemp969: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp968, 262143)))), 786431));
			let mut fTemp970: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp969, 3)) as usize] };
			let mut fTemp971: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp969 as usize] };
			let mut fTemp972: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp969, 1)) as usize] } - fTemp971;
			let mut fTemp973: F64 = if iTemp730 != 0 {fTemp971 + fTemp739 * fTemp972 + (fTemp967 - (iTemp968) as F64) * (fTemp970 - (fTemp971 + fTemp739 * (fTemp972 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp969, 4)) as usize] } - fTemp970))))} else {1.0 - (fTemp965 + fTemp739 * fTemp966 + (fTemp961 - (iTemp962) as F64) * (fTemp964 - (fTemp965 + fTemp739 * (fTemp966 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp963, 4)) as usize] } - fTemp964)))))};
			let mut fTemp974: F64 = fTemp744 + fTemp960;
			let mut fTemp975: F64 = 262143.0 * (1.0 - fTemp974);
			let mut iTemp976: i32 = (fTemp975) as i32;
			let mut iTemp977: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp976, 262143)))), 786431));
			let mut fTemp978: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp977, 3)) as usize] };
			let mut fTemp979: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp977 as usize] };
			let mut fTemp980: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp977, 1)) as usize] } - fTemp979;
			let mut fTemp981: F64 = 262143.0 * fTemp974;
			let mut iTemp982: i32 = (fTemp981) as i32;
			let mut iTemp983: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp982, 262143)))), 786431));
			let mut fTemp984: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp983, 3), 786431))) as usize] };
			let mut fTemp985: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp983 as usize] };
			let mut fTemp986: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp983, 1), 786431))) as usize] } - fTemp985;
			let mut iTemp987: i32 = (fTemp800 > ((if iTemp730 != 0 {fTemp985 + fTemp739 * fTemp986 + (fTemp981 - (iTemp982) as F64) * (fTemp984 - (fTemp985 + fTemp739 * (fTemp986 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp983, 4), 786431))) as usize] } - fTemp984))))} else {1.0 - (fTemp979 + fTemp739 * fTemp980 + (fTemp975 - (iTemp976) as F64) * (fTemp978 - (fTemp979 + fTemp739 * (fTemp980 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp977, 4)) as usize] } - fTemp978)))))} - fTemp973) / (1.0 - fTemp973))) as i32;
			let mut fTemp988: F64 = if iTemp987 != 0 {fTemp957} else {fTemp960};
			let mut fTemp989: F64 = if iTemp987 != 0 {fTemp960} else {fTemp958};
			let mut fTemp990: F64 = fTemp989 + fTemp988;
			let mut fTemp991: F64 = 0.5 * fTemp990;
			let mut fTemp992: F64 = 262143.0 * (1.0 - fTemp991);
			let mut iTemp993: i32 = (fTemp992) as i32;
			let mut iTemp994: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp993, 262143)))), 786431));
			let mut fTemp995: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp994, 3)) as usize] };
			let mut fTemp996: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp994 as usize] };
			let mut fTemp997: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp994, 1)) as usize] } - fTemp996;
			let mut fTemp998: F64 = 131071.5 * fTemp990;
			let mut iTemp999: i32 = (fTemp998) as i32;
			let mut iTemp1000: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp999, 262143)))), 786431));
			let mut fTemp1001: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1000, 3)) as usize] };
			let mut fTemp1002: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1000 as usize] };
			let mut fTemp1003: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1000, 1)) as usize] } - fTemp1002;
			let mut fTemp1004: F64 = if iTemp730 != 0 {fTemp1002 + fTemp739 * fTemp1003 + (fTemp998 - (iTemp999) as F64) * (fTemp1001 - (fTemp1002 + fTemp739 * (fTemp1003 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1000, 4)) as usize] } - fTemp1001))))} else {1.0 - (fTemp996 + fTemp739 * fTemp997 + (fTemp992 - (iTemp993) as F64) * (fTemp995 - (fTemp996 + fTemp739 * (fTemp997 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp994, 4)) as usize] } - fTemp995)))))};
			let mut fTemp1005: F64 = fTemp744 + fTemp991;
			let mut fTemp1006: F64 = 262143.0 * (1.0 - fTemp1005);
			let mut iTemp1007: i32 = (fTemp1006) as i32;
			let mut iTemp1008: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1007, 262143)))), 786431));
			let mut fTemp1009: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1008, 3)) as usize] };
			let mut fTemp1010: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1008 as usize] };
			let mut fTemp1011: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1008, 1)) as usize] } - fTemp1010;
			let mut fTemp1012: F64 = 262143.0 * fTemp1005;
			let mut iTemp1013: i32 = (fTemp1012) as i32;
			let mut iTemp1014: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1013, 262143)))), 786431));
			let mut fTemp1015: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1014, 3), 786431))) as usize] };
			let mut fTemp1016: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1014 as usize] };
			let mut fTemp1017: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1014, 1), 786431))) as usize] } - fTemp1016;
			let mut iTemp1018: i32 = (fTemp800 > ((if iTemp730 != 0 {fTemp1016 + fTemp739 * fTemp1017 + (fTemp1012 - (iTemp1013) as F64) * (fTemp1015 - (fTemp1016 + fTemp739 * (fTemp1017 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1014, 4), 786431))) as usize] } - fTemp1015))))} else {1.0 - (fTemp1010 + fTemp739 * fTemp1011 + (fTemp1006 - (iTemp1007) as F64) * (fTemp1009 - (fTemp1010 + fTemp739 * (fTemp1011 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1008, 4)) as usize] } - fTemp1009)))))} - fTemp1004) / (1.0 - fTemp1004))) as i32;
			let mut fTemp1019: F64 = if iTemp1018 != 0 {fTemp988} else {fTemp991};
			let mut fTemp1020: F64 = if iTemp1018 != 0 {fTemp991} else {fTemp989};
			let mut fTemp1021: F64 = fTemp1020 + fTemp1019;
			let mut fTemp1022: F64 = 0.5 * fTemp1021;
			let mut fTemp1023: F64 = 262143.0 * (1.0 - fTemp1022);
			let mut iTemp1024: i32 = (fTemp1023) as i32;
			let mut iTemp1025: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1024, 262143)))), 786431));
			let mut fTemp1026: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1025, 3)) as usize] };
			let mut fTemp1027: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1025 as usize] };
			let mut fTemp1028: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1025, 1)) as usize] } - fTemp1027;
			let mut fTemp1029: F64 = 131071.5 * fTemp1021;
			let mut iTemp1030: i32 = (fTemp1029) as i32;
			let mut iTemp1031: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1030, 262143)))), 786431));
			let mut fTemp1032: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1031, 3)) as usize] };
			let mut fTemp1033: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1031 as usize] };
			let mut fTemp1034: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1031, 1)) as usize] } - fTemp1033;
			let mut fTemp1035: F64 = if iTemp730 != 0 {fTemp1033 + fTemp739 * fTemp1034 + (fTemp1029 - (iTemp1030) as F64) * (fTemp1032 - (fTemp1033 + fTemp739 * (fTemp1034 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1031, 4)) as usize] } - fTemp1032))))} else {1.0 - (fTemp1027 + fTemp739 * fTemp1028 + (fTemp1023 - (iTemp1024) as F64) * (fTemp1026 - (fTemp1027 + fTemp739 * (fTemp1028 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1025, 4)) as usize] } - fTemp1026)))))};
			let mut fTemp1036: F64 = fTemp744 + fTemp1022;
			let mut fTemp1037: F64 = 262143.0 * (1.0 - fTemp1036);
			let mut iTemp1038: i32 = (fTemp1037) as i32;
			let mut iTemp1039: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1038, 262143)))), 786431));
			let mut fTemp1040: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1039, 3)) as usize] };
			let mut fTemp1041: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1039 as usize] };
			let mut fTemp1042: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1039, 1)) as usize] } - fTemp1041;
			let mut fTemp1043: F64 = 262143.0 * fTemp1036;
			let mut iTemp1044: i32 = (fTemp1043) as i32;
			let mut iTemp1045: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1044, 262143)))), 786431));
			let mut fTemp1046: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1045, 3), 786431))) as usize] };
			let mut fTemp1047: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1045 as usize] };
			let mut fTemp1048: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1045, 1), 786431))) as usize] } - fTemp1047;
			let mut iTemp1049: i32 = (fTemp800 > ((if iTemp730 != 0 {fTemp1047 + fTemp739 * fTemp1048 + (fTemp1043 - (iTemp1044) as F64) * (fTemp1046 - (fTemp1047 + fTemp739 * (fTemp1048 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1045, 4), 786431))) as usize] } - fTemp1046))))} else {1.0 - (fTemp1041 + fTemp739 * fTemp1042 + (fTemp1037 - (iTemp1038) as F64) * (fTemp1040 - (fTemp1041 + fTemp739 * (fTemp1042 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1039, 4)) as usize] } - fTemp1040)))))} - fTemp1035) / (1.0 - fTemp1035))) as i32;
			let mut fTemp1050: F64 = if iTemp1049 != 0 {fTemp1019} else {fTemp1022};
			let mut fTemp1051: F64 = if iTemp1049 != 0 {fTemp1022} else {fTemp1020};
			let mut fTemp1052: F64 = fTemp1051 + fTemp1050;
			let mut fTemp1053: F64 = 0.5 * fTemp1052;
			let mut fTemp1054: F64 = 262143.0 * (1.0 - fTemp1053);
			let mut iTemp1055: i32 = (fTemp1054) as i32;
			let mut iTemp1056: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1055, 262143)))), 786431));
			let mut fTemp1057: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1056, 3)) as usize] };
			let mut fTemp1058: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1056 as usize] };
			let mut fTemp1059: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1056, 1)) as usize] } - fTemp1058;
			let mut fTemp1060: F64 = 131071.5 * fTemp1052;
			let mut iTemp1061: i32 = (fTemp1060) as i32;
			let mut iTemp1062: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1061, 262143)))), 786431));
			let mut fTemp1063: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1062, 3)) as usize] };
			let mut fTemp1064: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1062 as usize] };
			let mut fTemp1065: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1062, 1)) as usize] } - fTemp1064;
			let mut fTemp1066: F64 = if iTemp730 != 0 {fTemp1064 + fTemp739 * fTemp1065 + (fTemp1060 - (iTemp1061) as F64) * (fTemp1063 - (fTemp1064 + fTemp739 * (fTemp1065 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1062, 4)) as usize] } - fTemp1063))))} else {1.0 - (fTemp1058 + fTemp739 * fTemp1059 + (fTemp1054 - (iTemp1055) as F64) * (fTemp1057 - (fTemp1058 + fTemp739 * (fTemp1059 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1056, 4)) as usize] } - fTemp1057)))))};
			let mut fTemp1067: F64 = fTemp744 + fTemp1053;
			let mut fTemp1068: F64 = 262143.0 * (1.0 - fTemp1067);
			let mut iTemp1069: i32 = (fTemp1068) as i32;
			let mut iTemp1070: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1069, 262143)))), 786431));
			let mut fTemp1071: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1070, 3)) as usize] };
			let mut fTemp1072: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1070 as usize] };
			let mut fTemp1073: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1070, 1)) as usize] } - fTemp1072;
			let mut fTemp1074: F64 = 262143.0 * fTemp1067;
			let mut iTemp1075: i32 = (fTemp1074) as i32;
			let mut iTemp1076: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1075, 262143)))), 786431));
			let mut fTemp1077: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1076, 3), 786431))) as usize] };
			let mut fTemp1078: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1076 as usize] };
			let mut fTemp1079: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1076, 1), 786431))) as usize] } - fTemp1078;
			let mut iTemp1080: i32 = (fTemp800 > ((if iTemp730 != 0 {fTemp1078 + fTemp739 * fTemp1079 + (fTemp1074 - (iTemp1075) as F64) * (fTemp1077 - (fTemp1078 + fTemp739 * (fTemp1079 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1076, 4), 786431))) as usize] } - fTemp1077))))} else {1.0 - (fTemp1072 + fTemp739 * fTemp1073 + (fTemp1068 - (iTemp1069) as F64) * (fTemp1071 - (fTemp1072 + fTemp739 * (fTemp1073 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1070, 4)) as usize] } - fTemp1071)))))} - fTemp1066) / (1.0 - fTemp1066))) as i32;
			let mut fTemp1081: F64 = if iTemp1080 != 0 {fTemp1050} else {fTemp1053};
			let mut fTemp1082: F64 = if iTemp1080 != 0 {fTemp1053} else {fTemp1051};
			let mut fTemp1083: F64 = fTemp1082 + fTemp1081;
			let mut fTemp1084: F64 = 0.5 * fTemp1083;
			let mut fTemp1085: F64 = 262143.0 * (1.0 - fTemp1084);
			let mut iTemp1086: i32 = (fTemp1085) as i32;
			let mut iTemp1087: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1086, 262143)))), 786431));
			let mut fTemp1088: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1087, 3)) as usize] };
			let mut fTemp1089: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1087 as usize] };
			let mut fTemp1090: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1087, 1)) as usize] } - fTemp1089;
			let mut fTemp1091: F64 = 131071.5 * fTemp1083;
			let mut iTemp1092: i32 = (fTemp1091) as i32;
			let mut iTemp1093: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1092, 262143)))), 786431));
			let mut fTemp1094: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1093, 3)) as usize] };
			let mut fTemp1095: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1093 as usize] };
			let mut fTemp1096: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1093, 1)) as usize] } - fTemp1095;
			let mut fTemp1097: F64 = if iTemp730 != 0 {fTemp1095 + fTemp739 * fTemp1096 + (fTemp1091 - (iTemp1092) as F64) * (fTemp1094 - (fTemp1095 + fTemp739 * (fTemp1096 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1093, 4)) as usize] } - fTemp1094))))} else {1.0 - (fTemp1089 + fTemp739 * fTemp1090 + (fTemp1085 - (iTemp1086) as F64) * (fTemp1088 - (fTemp1089 + fTemp739 * (fTemp1090 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1087, 4)) as usize] } - fTemp1088)))))};
			let mut fTemp1098: F64 = fTemp744 + fTemp1084;
			let mut fTemp1099: F64 = 262143.0 * (1.0 - fTemp1098);
			let mut iTemp1100: i32 = (fTemp1099) as i32;
			let mut iTemp1101: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1100, 262143)))), 786431));
			let mut fTemp1102: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1101, 3)) as usize] };
			let mut fTemp1103: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1101 as usize] };
			let mut fTemp1104: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1101, 1)) as usize] } - fTemp1103;
			let mut fTemp1105: F64 = 262143.0 * fTemp1098;
			let mut iTemp1106: i32 = (fTemp1105) as i32;
			let mut iTemp1107: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1106, 262143)))), 786431));
			let mut fTemp1108: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1107, 3), 786431))) as usize] };
			let mut fTemp1109: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1107 as usize] };
			let mut fTemp1110: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1107, 1), 786431))) as usize] } - fTemp1109;
			let mut iTemp1111: i32 = (fTemp800 > ((if iTemp730 != 0 {fTemp1109 + fTemp739 * fTemp1110 + (fTemp1105 - (iTemp1106) as F64) * (fTemp1108 - (fTemp1109 + fTemp739 * (fTemp1110 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1107, 4), 786431))) as usize] } - fTemp1108))))} else {1.0 - (fTemp1103 + fTemp739 * fTemp1104 + (fTemp1099 - (iTemp1100) as F64) * (fTemp1102 - (fTemp1103 + fTemp739 * (fTemp1104 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1101, 4)) as usize] } - fTemp1102)))))} - fTemp1097) / (1.0 - fTemp1097))) as i32;
			let mut fTemp1112: F64 = if iTemp1111 != 0 {fTemp1081} else {fTemp1084};
			let mut fTemp1113: F64 = if iTemp1111 != 0 {fTemp1084} else {fTemp1082};
			let mut fTemp1114: F64 = fTemp1113 + fTemp1112;
			let mut fTemp1115: F64 = 0.5 * fTemp1114;
			let mut fTemp1116: F64 = 262143.0 * (1.0 - fTemp1115);
			let mut iTemp1117: i32 = (fTemp1116) as i32;
			let mut iTemp1118: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1117, 262143)))), 786431));
			let mut fTemp1119: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1118, 3)) as usize] };
			let mut fTemp1120: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1118 as usize] };
			let mut fTemp1121: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1118, 1)) as usize] } - fTemp1120;
			let mut fTemp1122: F64 = 131071.5 * fTemp1114;
			let mut iTemp1123: i32 = (fTemp1122) as i32;
			let mut iTemp1124: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1123, 262143)))), 786431));
			let mut fTemp1125: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1124, 3)) as usize] };
			let mut fTemp1126: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1124 as usize] };
			let mut fTemp1127: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1124, 1)) as usize] } - fTemp1126;
			let mut fTemp1128: F64 = if iTemp730 != 0 {fTemp1126 + fTemp739 * fTemp1127 + (fTemp1122 - (iTemp1123) as F64) * (fTemp1125 - (fTemp1126 + fTemp739 * (fTemp1127 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1124, 4)) as usize] } - fTemp1125))))} else {1.0 - (fTemp1120 + fTemp739 * fTemp1121 + (fTemp1116 - (iTemp1117) as F64) * (fTemp1119 - (fTemp1120 + fTemp739 * (fTemp1121 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1118, 4)) as usize] } - fTemp1119)))))};
			let mut fTemp1129: F64 = fTemp744 + fTemp1115;
			let mut fTemp1130: F64 = 262143.0 * (1.0 - fTemp1129);
			let mut iTemp1131: i32 = (fTemp1130) as i32;
			let mut iTemp1132: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1131, 262143)))), 786431));
			let mut fTemp1133: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1132, 3)) as usize] };
			let mut fTemp1134: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1132 as usize] };
			let mut fTemp1135: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1132, 1)) as usize] } - fTemp1134;
			let mut fTemp1136: F64 = 262143.0 * fTemp1129;
			let mut iTemp1137: i32 = (fTemp1136) as i32;
			let mut iTemp1138: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1137, 262143)))), 786431));
			let mut fTemp1139: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1138, 3), 786431))) as usize] };
			let mut fTemp1140: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1138 as usize] };
			let mut fTemp1141: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1138, 1), 786431))) as usize] } - fTemp1140;
			let mut iTemp1142: i32 = (fTemp800 > ((if iTemp730 != 0 {fTemp1140 + fTemp739 * fTemp1141 + (fTemp1136 - (iTemp1137) as F64) * (fTemp1139 - (fTemp1140 + fTemp739 * (fTemp1141 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1138, 4), 786431))) as usize] } - fTemp1139))))} else {1.0 - (fTemp1134 + fTemp739 * fTemp1135 + (fTemp1130 - (iTemp1131) as F64) * (fTemp1133 - (fTemp1134 + fTemp739 * (fTemp1135 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1132, 4)) as usize] } - fTemp1133)))))} - fTemp1128) / (1.0 - fTemp1128))) as i32;
			let mut fTemp1143: F64 = if iTemp1142 != 0 {fTemp1112} else {fTemp1115};
			let mut fTemp1144: F64 = if iTemp1142 != 0 {fTemp1115} else {fTemp1113};
			let mut fTemp1145: F64 = fTemp1144 + fTemp1143;
			let mut fTemp1146: F64 = 0.5 * fTemp1145;
			let mut fTemp1147: F64 = 262143.0 * (1.0 - fTemp1146);
			let mut iTemp1148: i32 = (fTemp1147) as i32;
			let mut iTemp1149: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1148, 262143)))), 786431));
			let mut fTemp1150: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1149, 3)) as usize] };
			let mut fTemp1151: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1149 as usize] };
			let mut fTemp1152: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1149, 1)) as usize] } - fTemp1151;
			let mut fTemp1153: F64 = 131071.5 * fTemp1145;
			let mut iTemp1154: i32 = (fTemp1153) as i32;
			let mut iTemp1155: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1154, 262143)))), 786431));
			let mut fTemp1156: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1155, 3)) as usize] };
			let mut fTemp1157: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1155 as usize] };
			let mut fTemp1158: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1155, 1)) as usize] } - fTemp1157;
			let mut fTemp1159: F64 = if iTemp730 != 0 {fTemp1157 + fTemp739 * fTemp1158 + (fTemp1153 - (iTemp1154) as F64) * (fTemp1156 - (fTemp1157 + fTemp739 * (fTemp1158 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1155, 4)) as usize] } - fTemp1156))))} else {1.0 - (fTemp1151 + fTemp739 * fTemp1152 + (fTemp1147 - (iTemp1148) as F64) * (fTemp1150 - (fTemp1151 + fTemp739 * (fTemp1152 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1149, 4)) as usize] } - fTemp1150)))))};
			let mut fTemp1160: F64 = fTemp744 + fTemp1146;
			let mut fTemp1161: F64 = 262143.0 * (1.0 - fTemp1160);
			let mut iTemp1162: i32 = (fTemp1161) as i32;
			let mut iTemp1163: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1162, 262143)))), 786431));
			let mut fTemp1164: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1163, 3)) as usize] };
			let mut fTemp1165: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1163 as usize] };
			let mut fTemp1166: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1163, 1)) as usize] } - fTemp1165;
			let mut fTemp1167: F64 = 262143.0 * fTemp1160;
			let mut iTemp1168: i32 = (fTemp1167) as i32;
			let mut iTemp1169: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1168, 262143)))), 786431));
			let mut fTemp1170: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1169, 3), 786431))) as usize] };
			let mut fTemp1171: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1169 as usize] };
			let mut fTemp1172: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1169, 1), 786431))) as usize] } - fTemp1171;
			let mut iTemp1173: i32 = (fTemp800 > ((if iTemp730 != 0 {fTemp1171 + fTemp739 * fTemp1172 + (fTemp1167 - (iTemp1168) as F64) * (fTemp1170 - (fTemp1171 + fTemp739 * (fTemp1172 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1169, 4), 786431))) as usize] } - fTemp1170))))} else {1.0 - (fTemp1165 + fTemp739 * fTemp1166 + (fTemp1161 - (iTemp1162) as F64) * (fTemp1164 - (fTemp1165 + fTemp739 * (fTemp1166 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1163, 4)) as usize] } - fTemp1164)))))} - fTemp1159) / (1.0 - fTemp1159))) as i32;
			let mut fTemp1174: F64 = if iTemp1173 != 0 {fTemp1143} else {fTemp1146};
			let mut fTemp1175: F64 = if iTemp1173 != 0 {fTemp1146} else {fTemp1144};
			let mut fTemp1176: F64 = fTemp1175 + fTemp1174;
			let mut fTemp1177: F64 = 0.5 * fTemp1176;
			let mut fTemp1178: F64 = 262143.0 * (1.0 - fTemp1177);
			let mut iTemp1179: i32 = (fTemp1178) as i32;
			let mut iTemp1180: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1179, 262143)))), 786431));
			let mut fTemp1181: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1180, 3)) as usize] };
			let mut fTemp1182: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1180 as usize] };
			let mut fTemp1183: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1180, 1)) as usize] } - fTemp1182;
			let mut fTemp1184: F64 = 131071.5 * fTemp1176;
			let mut iTemp1185: i32 = (fTemp1184) as i32;
			let mut iTemp1186: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1185, 262143)))), 786431));
			let mut fTemp1187: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1186, 3)) as usize] };
			let mut fTemp1188: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1186 as usize] };
			let mut fTemp1189: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1186, 1)) as usize] } - fTemp1188;
			let mut fTemp1190: F64 = if iTemp730 != 0 {fTemp1188 + fTemp739 * fTemp1189 + (fTemp1184 - (iTemp1185) as F64) * (fTemp1187 - (fTemp1188 + fTemp739 * (fTemp1189 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1186, 4)) as usize] } - fTemp1187))))} else {1.0 - (fTemp1182 + fTemp739 * fTemp1183 + (fTemp1178 - (iTemp1179) as F64) * (fTemp1181 - (fTemp1182 + fTemp739 * (fTemp1183 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1180, 4)) as usize] } - fTemp1181)))))};
			let mut fTemp1191: F64 = fTemp744 + fTemp1177;
			let mut fTemp1192: F64 = 262143.0 * (1.0 - fTemp1191);
			let mut iTemp1193: i32 = (fTemp1192) as i32;
			let mut iTemp1194: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1193, 262143)))), 786431));
			let mut fTemp1195: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1194, 3)) as usize] };
			let mut fTemp1196: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1194 as usize] };
			let mut fTemp1197: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1194, 1)) as usize] } - fTemp1196;
			let mut fTemp1198: F64 = 262143.0 * fTemp1191;
			let mut iTemp1199: i32 = (fTemp1198) as i32;
			let mut iTemp1200: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1199, 262143)))), 786431));
			let mut fTemp1201: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1200, 3), 786431))) as usize] };
			let mut fTemp1202: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1200 as usize] };
			let mut fTemp1203: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1200, 1), 786431))) as usize] } - fTemp1202;
			let mut iTemp1204: i32 = (fTemp800 > ((if iTemp730 != 0 {fTemp1202 + fTemp739 * fTemp1203 + (fTemp1198 - (iTemp1199) as F64) * (fTemp1201 - (fTemp1202 + fTemp739 * (fTemp1203 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1200, 4), 786431))) as usize] } - fTemp1201))))} else {1.0 - (fTemp1196 + fTemp739 * fTemp1197 + (fTemp1192 - (iTemp1193) as F64) * (fTemp1195 - (fTemp1196 + fTemp739 * (fTemp1197 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1194, 4)) as usize] } - fTemp1195)))))} - fTemp1190) / (1.0 - fTemp1190))) as i32;
			let mut fTemp1205: F64 = if iTemp1204 != 0 {fTemp1174} else {fTemp1177};
			let mut fTemp1206: F64 = if iTemp1204 != 0 {fTemp1177} else {fTemp1175};
			let mut fTemp1207: F64 = fTemp1206 + fTemp1205;
			let mut fTemp1208: F64 = 0.5 * fTemp1207;
			let mut fTemp1209: F64 = 262143.0 * (1.0 - fTemp1208);
			let mut iTemp1210: i32 = (fTemp1209) as i32;
			let mut iTemp1211: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1210, 262143)))), 786431));
			let mut fTemp1212: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1211, 3)) as usize] };
			let mut fTemp1213: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1211 as usize] };
			let mut fTemp1214: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1211, 1)) as usize] } - fTemp1213;
			let mut fTemp1215: F64 = 131071.5 * fTemp1207;
			let mut iTemp1216: i32 = (fTemp1215) as i32;
			let mut iTemp1217: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1216, 262143)))), 786431));
			let mut fTemp1218: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1217, 3)) as usize] };
			let mut fTemp1219: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1217 as usize] };
			let mut fTemp1220: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1217, 1)) as usize] } - fTemp1219;
			let mut fTemp1221: F64 = if iTemp730 != 0 {fTemp1219 + fTemp739 * fTemp1220 + (fTemp1215 - (iTemp1216) as F64) * (fTemp1218 - (fTemp1219 + fTemp739 * (fTemp1220 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1217, 4)) as usize] } - fTemp1218))))} else {1.0 - (fTemp1213 + fTemp739 * fTemp1214 + (fTemp1209 - (iTemp1210) as F64) * (fTemp1212 - (fTemp1213 + fTemp739 * (fTemp1214 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1211, 4)) as usize] } - fTemp1212)))))};
			let mut fTemp1222: F64 = fTemp744 + fTemp1208;
			let mut fTemp1223: F64 = 262143.0 * (1.0 - fTemp1222);
			let mut iTemp1224: i32 = (fTemp1223) as i32;
			let mut iTemp1225: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1224, 262143)))), 786431));
			let mut fTemp1226: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1225, 3)) as usize] };
			let mut fTemp1227: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1225 as usize] };
			let mut fTemp1228: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1225, 1)) as usize] } - fTemp1227;
			let mut fTemp1229: F64 = 262143.0 * fTemp1222;
			let mut iTemp1230: i32 = (fTemp1229) as i32;
			let mut iTemp1231: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1230, 262143)))), 786431));
			let mut fTemp1232: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1231, 3), 786431))) as usize] };
			let mut fTemp1233: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1231 as usize] };
			let mut fTemp1234: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1231, 1), 786431))) as usize] } - fTemp1233;
			let mut iTemp1235: i32 = (fTemp800 > ((if iTemp730 != 0 {fTemp1233 + fTemp739 * fTemp1234 + (fTemp1229 - (iTemp1230) as F64) * (fTemp1232 - (fTemp1233 + fTemp739 * (fTemp1234 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1231, 4), 786431))) as usize] } - fTemp1232))))} else {1.0 - (fTemp1227 + fTemp739 * fTemp1228 + (fTemp1223 - (iTemp1224) as F64) * (fTemp1226 - (fTemp1227 + fTemp739 * (fTemp1228 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1225, 4)) as usize] } - fTemp1226)))))} - fTemp1221) / (1.0 - fTemp1221))) as i32;
			let mut fTemp1236: F64 = if iTemp1235 != 0 {fTemp1205} else {fTemp1208};
			let mut fTemp1237: F64 = if iTemp1235 != 0 {fTemp1208} else {fTemp1206};
			let mut fTemp1238: F64 = fTemp1237 + fTemp1236;
			let mut fTemp1239: F64 = 0.5 * fTemp1238;
			let mut fTemp1240: F64 = 262143.0 * (1.0 - fTemp1239);
			let mut iTemp1241: i32 = (fTemp1240) as i32;
			let mut iTemp1242: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1241, 262143)))), 786431));
			let mut fTemp1243: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1242, 3)) as usize] };
			let mut fTemp1244: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1242 as usize] };
			let mut fTemp1245: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1242, 1)) as usize] } - fTemp1244;
			let mut fTemp1246: F64 = 131071.5 * fTemp1238;
			let mut iTemp1247: i32 = (fTemp1246) as i32;
			let mut iTemp1248: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1247, 262143)))), 786431));
			let mut fTemp1249: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1248, 3)) as usize] };
			let mut fTemp1250: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1248 as usize] };
			let mut fTemp1251: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1248, 1)) as usize] } - fTemp1250;
			let mut fTemp1252: F64 = if iTemp730 != 0 {fTemp1250 + fTemp739 * fTemp1251 + (fTemp1246 - (iTemp1247) as F64) * (fTemp1249 - (fTemp1250 + fTemp739 * (fTemp1251 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1248, 4)) as usize] } - fTemp1249))))} else {1.0 - (fTemp1244 + fTemp739 * fTemp1245 + (fTemp1240 - (iTemp1241) as F64) * (fTemp1243 - (fTemp1244 + fTemp739 * (fTemp1245 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1242, 4)) as usize] } - fTemp1243)))))};
			let mut fTemp1253: F64 = fTemp744 + fTemp1239;
			let mut fTemp1254: F64 = 262143.0 * (1.0 - fTemp1253);
			let mut iTemp1255: i32 = (fTemp1254) as i32;
			let mut iTemp1256: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1255, 262143)))), 786431));
			let mut fTemp1257: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1256, 3)) as usize] };
			let mut fTemp1258: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1256 as usize] };
			let mut fTemp1259: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1256, 1)) as usize] } - fTemp1258;
			let mut fTemp1260: F64 = 262143.0 * fTemp1253;
			let mut iTemp1261: i32 = (fTemp1260) as i32;
			let mut iTemp1262: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1261, 262143)))), 786431));
			let mut fTemp1263: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1262, 3), 786431))) as usize] };
			let mut fTemp1264: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1262 as usize] };
			let mut fTemp1265: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1262, 1), 786431))) as usize] } - fTemp1264;
			let mut iTemp1266: i32 = (fTemp800 > ((if iTemp730 != 0 {fTemp1264 + fTemp739 * fTemp1265 + (fTemp1260 - (iTemp1261) as F64) * (fTemp1263 - (fTemp1264 + fTemp739 * (fTemp1265 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1262, 4), 786431))) as usize] } - fTemp1263))))} else {1.0 - (fTemp1258 + fTemp739 * fTemp1259 + (fTemp1254 - (iTemp1255) as F64) * (fTemp1257 - (fTemp1258 + fTemp739 * (fTemp1259 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1256, 4)) as usize] } - fTemp1257)))))} - fTemp1252) / (1.0 - fTemp1252))) as i32;
			let mut fTemp1267: F64 = if iTemp1266 != 0 {fTemp1236} else {fTemp1239};
			let mut fTemp1268: F64 = if iTemp1266 != 0 {fTemp1239} else {fTemp1237};
			let mut fTemp1269: F64 = fTemp1268 + fTemp1267;
			let mut fTemp1270: F64 = 0.5 * fTemp1269;
			let mut fTemp1271: F64 = 262143.0 * (1.0 - fTemp1270);
			let mut iTemp1272: i32 = (fTemp1271) as i32;
			let mut iTemp1273: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1272, 262143)))), 786431));
			let mut fTemp1274: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1273, 3)) as usize] };
			let mut fTemp1275: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1273 as usize] };
			let mut fTemp1276: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1273, 1)) as usize] } - fTemp1275;
			let mut fTemp1277: F64 = 131071.5 * fTemp1269;
			let mut iTemp1278: i32 = (fTemp1277) as i32;
			let mut iTemp1279: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1278, 262143)))), 786431));
			let mut fTemp1280: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1279, 3)) as usize] };
			let mut fTemp1281: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1279 as usize] };
			let mut fTemp1282: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1279, 1)) as usize] } - fTemp1281;
			let mut fTemp1283: F64 = if iTemp730 != 0 {fTemp1281 + fTemp739 * fTemp1282 + (fTemp1277 - (iTemp1278) as F64) * (fTemp1280 - (fTemp1281 + fTemp739 * (fTemp1282 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1279, 4)) as usize] } - fTemp1280))))} else {1.0 - (fTemp1275 + fTemp739 * fTemp1276 + (fTemp1271 - (iTemp1272) as F64) * (fTemp1274 - (fTemp1275 + fTemp739 * (fTemp1276 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1273, 4)) as usize] } - fTemp1274)))))};
			let mut fTemp1284: F64 = fTemp744 + fTemp1270;
			let mut fTemp1285: F64 = 262143.0 * (1.0 - fTemp1284);
			let mut iTemp1286: i32 = (fTemp1285) as i32;
			let mut iTemp1287: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1286, 262143)))), 786431));
			let mut fTemp1288: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1287, 3)) as usize] };
			let mut fTemp1289: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1287 as usize] };
			let mut fTemp1290: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1287, 1)) as usize] } - fTemp1289;
			let mut fTemp1291: F64 = 262143.0 * fTemp1284;
			let mut iTemp1292: i32 = (fTemp1291) as i32;
			let mut iTemp1293: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1292, 262143)))), 786431));
			let mut fTemp1294: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1293, 3), 786431))) as usize] };
			let mut fTemp1295: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1293 as usize] };
			let mut fTemp1296: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1293, 1), 786431))) as usize] } - fTemp1295;
			let mut iTemp1297: i32 = (fTemp800 > ((if iTemp730 != 0 {fTemp1295 + fTemp739 * fTemp1296 + (fTemp1291 - (iTemp1292) as F64) * (fTemp1294 - (fTemp1295 + fTemp739 * (fTemp1296 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1293, 4), 786431))) as usize] } - fTemp1294))))} else {1.0 - (fTemp1289 + fTemp739 * fTemp1290 + (fTemp1285 - (iTemp1286) as F64) * (fTemp1288 - (fTemp1289 + fTemp739 * (fTemp1290 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1287, 4)) as usize] } - fTemp1288)))))} - fTemp1283) / (1.0 - fTemp1283))) as i32;
			let mut fTemp1298: F64 = if iTemp1297 != 0 {fTemp1267} else {fTemp1270};
			let mut fTemp1299: F64 = if iTemp1297 != 0 {fTemp1270} else {fTemp1268};
			let mut fTemp1300: F64 = fTemp1299 + fTemp1298;
			let mut fTemp1301: F64 = 0.5 * fTemp1300;
			let mut fTemp1302: F64 = 262143.0 * (1.0 - fTemp1301);
			let mut iTemp1303: i32 = (fTemp1302) as i32;
			let mut iTemp1304: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1303, 262143)))), 786431));
			let mut fTemp1305: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1304, 3)) as usize] };
			let mut fTemp1306: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1304 as usize] };
			let mut fTemp1307: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1304, 1)) as usize] } - fTemp1306;
			let mut fTemp1308: F64 = 131071.5 * fTemp1300;
			let mut iTemp1309: i32 = (fTemp1308) as i32;
			let mut iTemp1310: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1309, 262143)))), 786431));
			let mut fTemp1311: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1310, 3)) as usize] };
			let mut fTemp1312: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1310 as usize] };
			let mut fTemp1313: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1310, 1)) as usize] } - fTemp1312;
			let mut fTemp1314: F64 = if iTemp730 != 0 {fTemp1312 + fTemp739 * fTemp1313 + (fTemp1308 - (iTemp1309) as F64) * (fTemp1311 - (fTemp1312 + fTemp739 * (fTemp1313 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1310, 4), 786431))) as usize] } - fTemp1311))))} else {1.0 - (fTemp1306 + fTemp739 * fTemp1307 + (fTemp1302 - (iTemp1303) as F64) * (fTemp1305 - (fTemp1306 + fTemp739 * (fTemp1307 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1304, 4), 786431))) as usize] } - fTemp1305)))))};
			let mut fTemp1315: F64 = fTemp744 + fTemp1301;
			let mut fTemp1316: F64 = 262143.0 * (1.0 - fTemp1315);
			let mut iTemp1317: i32 = (fTemp1316) as i32;
			let mut iTemp1318: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1317, 262143)))), 786431));
			let mut fTemp1319: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1318, 3)) as usize] };
			let mut fTemp1320: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1318 as usize] };
			let mut fTemp1321: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1318, 1)) as usize] } - fTemp1320;
			let mut fTemp1322: F64 = 262143.0 * fTemp1315;
			let mut iTemp1323: i32 = (fTemp1322) as i32;
			let mut iTemp1324: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1323, 262143)))), 786431));
			let mut fTemp1325: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1324, 3), 786431))) as usize] };
			let mut fTemp1326: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1324 as usize] };
			let mut fTemp1327: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1324, 1), 786431))) as usize] } - fTemp1326;
			let mut iTemp1328: i32 = (fTemp800 > ((if iTemp730 != 0 {fTemp1326 + fTemp739 * fTemp1327 + (fTemp1322 - (iTemp1323) as F64) * (fTemp1325 - (fTemp1326 + fTemp739 * (fTemp1327 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1324, 4), 786431))) as usize] } - fTemp1325))))} else {1.0 - (fTemp1320 + fTemp739 * fTemp1321 + (fTemp1316 - (iTemp1317) as F64) * (fTemp1319 - (fTemp1320 + fTemp739 * (fTemp1321 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1318, 4)) as usize] } - fTemp1319)))))} - fTemp1314) / (1.0 - fTemp1314))) as i32;
			let mut fTemp1329: F64 = F64::min(1.0, F64::max(0.0, 0.5 * (if iTemp1328 != 0 {fTemp1301} else {fTemp1299} + if iTemp1328 != 0 {fTemp1298} else {fTemp1301})));
			self.fRec15[0] = fTemp1329;
			let mut fTemp1330: F64 = 262143.0 * (1.0 - fTemp1329);
			let mut iTemp1331: i32 = (fTemp1330) as i32;
			let mut iTemp1332: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1331, 262143)))), 786431));
			let mut fTemp1333: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1332, 3)) as usize] };
			let mut fTemp1334: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1332 as usize] };
			let mut fTemp1335: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1332, 1)) as usize] } - fTemp1334;
			let mut fTemp1336: F64 = 262143.0 * fTemp1329;
			let mut iTemp1337: i32 = (fTemp1336) as i32;
			let mut iTemp1338: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1337, 262143)))), 786431));
			let mut fTemp1339: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1338, 3)) as usize] };
			let mut fTemp1340: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1338 as usize] };
			let mut fTemp1341: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1338, 1)) as usize] } - fTemp1340;
			let mut fTemp1342: F64 = if iTemp730 != 0 {fTemp1340 + fTemp739 * fTemp1341 + (fTemp1336 - (iTemp1337) as F64) * (fTemp1339 - (fTemp1340 + fTemp739 * (fTemp1341 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1338, 4), 786431))) as usize] } - fTemp1339))))} else {1.0 - (fTemp1334 + fTemp739 * fTemp1335 + (fTemp1330 - (iTemp1331) as F64) * (fTemp1333 - (fTemp1334 + fTemp739 * (fTemp1335 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1332, 4), 786431))) as usize] } - fTemp1333)))))};
			let mut fTemp1343: F64 = fTemp744 + fTemp1329;
			let mut fTemp1344: F64 = 262143.0 * (1.0 - fTemp1343);
			let mut iTemp1345: i32 = (fTemp1344) as i32;
			let mut iTemp1346: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1345, 262143)))), 786431));
			let mut fTemp1347: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1346, 3)) as usize] };
			let mut fTemp1348: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1346 as usize] };
			let mut fTemp1349: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1346, 1)) as usize] } - fTemp1348;
			let mut fTemp1350: F64 = 262143.0 * fTemp1343;
			let mut iTemp1351: i32 = (fTemp1350) as i32;
			let mut iTemp1352: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp734, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1351, 262143)))), 786431));
			let mut fTemp1353: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1352, 3), 786431))) as usize] };
			let mut fTemp1354: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1352 as usize] };
			let mut fTemp1355: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1352, 1), 786431))) as usize] } - fTemp1354;
			let mut fTemp1356: F64 = fTemp700 + if ((0.001 * fTemp743) == 0.0) as i32 != 0 {fTemp729} else {fTemp729 * (if iTemp730 != 0 {fTemp1354 + fTemp739 * fTemp1355 + (fTemp1350 - (iTemp1351) as F64) * (fTemp1353 - (fTemp1354 + fTemp739 * (fTemp1355 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1352, 4), 786431))) as usize] } - fTemp1353))))} else {1.0 - (fTemp1348 + fTemp739 * fTemp1349 + (fTemp1344 - (iTemp1345) as F64) * (fTemp1347 - (fTemp1348 + fTemp739 * (fTemp1349 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1346, 4)) as usize] } - fTemp1347)))))} - fTemp1342) / (1.0 - fTemp1342)};
			self.fRec16[(self.IOTA0 & 32767) as usize] = if iTemp742 != 0 {F64::min(fTemp1356, fTemp700)} else {F64::max(fTemp1356, fTemp700)};
			let mut fTemp1357: F64 = self.fRec16[((i32::wrapping_sub(self.IOTA0, iSlow79)) & 32767) as usize];
			*output1 = 0.5 * fTemp2 * self.fVec3[((i32::wrapping_sub(self.IOTA0, iSlow18)) & 32767) as usize] + self.fRec14[0] * fTemp4 * self.fVec4[((i32::wrapping_sub(self.IOTA0, iSlow18)) & 32767) as usize] * fTemp1357;
			*output2 = fTemp3 + fTemp699 * fTemp4;
			*output3 = fTemp3 + fTemp4 * fTemp1357;
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
