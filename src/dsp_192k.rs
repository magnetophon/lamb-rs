/* ------------------------------------------------------------
author: "Bart Brouns"
license: "AGPLv3"
name: "lamb-rs"
version: "0.1"
Code generated with Faust 2.72.14 (https://faust.grame.fr)
Compilation options: -a /run/user/1001/.tmp0VZWVy -lang rust -ct 1 -cn LambRs192k -es 1 -mcd 16 -mdd 1024 -mdy 33 -double -ftz 0
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
		for l43 in 0..2 {
			self.iRec13[l43 as usize] = 0;
		}
	}
	
	fn fillLambRs192kSIG0(&mut self, count: i32, table: &mut[F64]) {
		for i1 in 0..count {
			self.iRec13[0] = i32::wrapping_add(self.iRec13[1], 1);
			let mut iTemp67: i32 = i32::wrapping_add(self.iRec13[0], -1);
			let mut fTemp68: F64 = (iTemp67 % 3) as F64 as i32 as F64;
			let mut fTemp69: F64 = 0.5 * fTemp68;
			let mut fTemp70: F64 = F64::powf(fTemp69, 0.21 * fTemp68 + 1.0);
			let mut fTemp71: F64 = (0.3333333333333333 * (iTemp67 % 196608) as F64) as i32 as F64;
			table[i1 as usize] = F64::min(1.0, F64::max(0.0, if (fTemp69 == 0.0) as i32 != 0 {0.5 * (F64::sin(4.793763109162727e-05 * fTemp71 + 4.71238898038469) + 1.0)} else {0.5 * (F64::sin(3.141592653589793 * ((1.0 - F64::exp(-(3.692683299000534e-05 * fTemp70 * fTemp71))) / (1.0 - F64::exp(-(2.42 * fTemp70)))) + 4.71238898038469) + 1.0)}));
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
static mut ftbl0LambRs192kSIG0: [F64;196608] = [0.0;196608];
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
	fSampleRate: i32,
	fConst0: F64,
	fConst1: F64,
	fRec0: [F64;2],
	IOTA0: i32,
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
	fVec0: [F64;32768],
	fVec1: [F64;32768],
	fVec2: [F64;32768],
	fVec3: [F64;32768],
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
	fVec4: [F64;16384],
	fHslider9: F64,
	fConst9: F64,
	fVec5: [F64;3],
	fVec6: [F64;7],
	fVec7: [F64;15],
	fVec8: [F64;32],
	fVec9: [F64;64],
	fVec10: [F64;128],
	fVec11: [F64;256],
	fVec12: [F64;512],
	fVec13: [F64;1024],
	fVec14: [F64;2048],
	fVec15: [F64;4096],
	fVec16: [F64;8192],
	fVec17: [F64;16384],
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
	fVec30: [F64;16384],
	fVec31: [F64;2],
	fHslider10: F64,
	fHslider11: F64,
	fVec32: [F64;2],
	fVec33: [F64;2],
	fConst10: F64,
	fRec1: [F64;2],
	fRec2: [F64;32768],
	fCheckbox1: F64,
	fHbargraph0: F64,
	fHbargraph1: F64,
	fHslider12: F64,
	fRec14: [F64;2],
	fVec34: [F64;16384],
	fVec35: [F64;3],
	fVec36: [F64;7],
	fVec37: [F64;15],
	fVec38: [F64;32],
	fVec39: [F64;64],
	fVec40: [F64;128],
	fVec41: [F64;256],
	fVec42: [F64;512],
	fVec43: [F64;1024],
	fVec44: [F64;2048],
	fVec45: [F64;4096],
	fVec46: [F64;8192],
	fVec47: [F64;16384],
	fRec17: [F64;2],
	fVec48: [F64;3],
	fVec49: [F64;7],
	fVec50: [F64;15],
	fVec51: [F64;32],
	fVec52: [F64;64],
	fVec53: [F64;128],
	fVec54: [F64;256],
	fVec55: [F64;512],
	fVec56: [F64;1024],
	fVec57: [F64;2048],
	fVec58: [F64;4096],
	fVec59: [F64;8192],
	fVec60: [F64;16384],
	fVec61: [F64;2],
	fVec62: [F64;2],
	fVec63: [F64;2],
	fRec15: [F64;2],
	fRec16: [F64;32768],
	fHbargraph2: F64,
}

impl FaustDsp for LambRs192k {
	type T = F64;
		
	fn new() -> LambRs192k { 
		LambRs192k {
			fCheckbox0: 0.0,
			fSampleRate: 0,
			fConst0: 0.0,
			fConst1: 0.0,
			fRec0: [0.0;2],
			IOTA0: 0,
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
			fVec0: [0.0;32768],
			fVec1: [0.0;32768],
			fVec2: [0.0;32768],
			fVec3: [0.0;32768],
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
			fVec4: [0.0;16384],
			fHslider9: 0.0,
			fConst9: 0.0,
			fVec5: [0.0;3],
			fVec6: [0.0;7],
			fVec7: [0.0;15],
			fVec8: [0.0;32],
			fVec9: [0.0;64],
			fVec10: [0.0;128],
			fVec11: [0.0;256],
			fVec12: [0.0;512],
			fVec13: [0.0;1024],
			fVec14: [0.0;2048],
			fVec15: [0.0;4096],
			fVec16: [0.0;8192],
			fVec17: [0.0;16384],
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
			fVec30: [0.0;16384],
			fVec31: [0.0;2],
			fHslider10: 0.0,
			fHslider11: 0.0,
			fVec32: [0.0;2],
			fVec33: [0.0;2],
			fConst10: 0.0,
			fRec1: [0.0;2],
			fRec2: [0.0;32768],
			fCheckbox1: 0.0,
			fHbargraph0: 0.0,
			fHbargraph1: 0.0,
			fHslider12: 0.0,
			fRec14: [0.0;2],
			fVec34: [0.0;16384],
			fVec35: [0.0;3],
			fVec36: [0.0;7],
			fVec37: [0.0;15],
			fVec38: [0.0;32],
			fVec39: [0.0;64],
			fVec40: [0.0;128],
			fVec41: [0.0;256],
			fVec42: [0.0;512],
			fVec43: [0.0;1024],
			fVec44: [0.0;2048],
			fVec45: [0.0;4096],
			fVec46: [0.0;8192],
			fVec47: [0.0;16384],
			fRec17: [0.0;2],
			fVec48: [0.0;3],
			fVec49: [0.0;7],
			fVec50: [0.0;15],
			fVec51: [0.0;32],
			fVec52: [0.0;64],
			fVec53: [0.0;128],
			fVec54: [0.0;256],
			fVec55: [0.0;512],
			fVec56: [0.0;1024],
			fVec57: [0.0;2048],
			fVec58: [0.0;4096],
			fVec59: [0.0;8192],
			fVec60: [0.0;16384],
			fVec61: [0.0;2],
			fVec62: [0.0;2],
			fVec63: [0.0;2],
			fRec15: [0.0;2],
			fRec16: [0.0;32768],
			fHbargraph2: 0.0,
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
		m.declare("compile_options", r"-a /run/user/1001/.tmp0VZWVy -lang rust -ct 1 -cn LambRs192k -es 1 -mcd 16 -mdd 1024 -mdy 33 -double -ftz 0");
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
		return 2;
	}
	
	fn class_init(sample_rate: i32) {
		let mut sig0: LambRs192kSIG0 = newLambRs192kSIG0();
		sig0.instance_initLambRs192kSIG0(sample_rate);
		sig0.fillLambRs192kSIG0(196608, unsafe { &mut ftbl0LambRs192kSIG0 });
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
		for l0 in 0..2 {
			self.fRec0[l0 as usize] = 0.0;
		}
		self.IOTA0 = 0;
		for l1 in 0..2 {
			self.fRec4[l1 as usize] = 0.0;
		}
		for l2 in 0..2 {
			self.fRec11[l2 as usize] = 0.0;
		}
		for l3 in 0..32768 {
			self.fVec0[l3 as usize] = 0.0;
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
		for l7 in 0..2 {
			self.fRec10[l7 as usize] = 0.0;
		}
		for l8 in 0..2 {
			self.fRec9[l8 as usize] = 0.0;
		}
		for l9 in 0..2 {
			self.fRec8[l9 as usize] = 0.0;
		}
		for l10 in 0..2 {
			self.fRec7[l10 as usize] = 0.0;
		}
		for l11 in 0..2 {
			self.fRec5[l11 as usize] = 0.0;
		}
		for l12 in 0..2 {
			self.fRec12[l12 as usize] = 0.0;
		}
		for l13 in 0..2 {
			self.fRec6[l13 as usize] = 0.0;
		}
		for l14 in 0..16384 {
			self.fVec4[l14 as usize] = 0.0;
		}
		for l15 in 0..3 {
			self.fVec5[l15 as usize] = 0.0;
		}
		for l16 in 0..7 {
			self.fVec6[l16 as usize] = 0.0;
		}
		for l17 in 0..15 {
			self.fVec7[l17 as usize] = 0.0;
		}
		for l18 in 0..32 {
			self.fVec8[l18 as usize] = 0.0;
		}
		for l19 in 0..64 {
			self.fVec9[l19 as usize] = 0.0;
		}
		for l20 in 0..128 {
			self.fVec10[l20 as usize] = 0.0;
		}
		for l21 in 0..256 {
			self.fVec11[l21 as usize] = 0.0;
		}
		for l22 in 0..512 {
			self.fVec12[l22 as usize] = 0.0;
		}
		for l23 in 0..1024 {
			self.fVec13[l23 as usize] = 0.0;
		}
		for l24 in 0..2048 {
			self.fVec14[l24 as usize] = 0.0;
		}
		for l25 in 0..4096 {
			self.fVec15[l25 as usize] = 0.0;
		}
		for l26 in 0..8192 {
			self.fVec16[l26 as usize] = 0.0;
		}
		for l27 in 0..16384 {
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
		for l41 in 0..16384 {
			self.fVec30[l41 as usize] = 0.0;
		}
		for l42 in 0..2 {
			self.fVec31[l42 as usize] = 0.0;
		}
		for l44 in 0..2 {
			self.fVec32[l44 as usize] = 0.0;
		}
		for l45 in 0..2 {
			self.fVec33[l45 as usize] = 0.0;
		}
		for l46 in 0..2 {
			self.fRec1[l46 as usize] = 0.0;
		}
		for l47 in 0..32768 {
			self.fRec2[l47 as usize] = 0.0;
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
		for l62 in 0..16384 {
			self.fVec47[l62 as usize] = 0.0;
		}
		for l63 in 0..2 {
			self.fRec17[l63 as usize] = 0.0;
		}
		for l64 in 0..3 {
			self.fVec48[l64 as usize] = 0.0;
		}
		for l65 in 0..7 {
			self.fVec49[l65 as usize] = 0.0;
		}
		for l66 in 0..15 {
			self.fVec50[l66 as usize] = 0.0;
		}
		for l67 in 0..32 {
			self.fVec51[l67 as usize] = 0.0;
		}
		for l68 in 0..64 {
			self.fVec52[l68 as usize] = 0.0;
		}
		for l69 in 0..128 {
			self.fVec53[l69 as usize] = 0.0;
		}
		for l70 in 0..256 {
			self.fVec54[l70 as usize] = 0.0;
		}
		for l71 in 0..512 {
			self.fVec55[l71 as usize] = 0.0;
		}
		for l72 in 0..1024 {
			self.fVec56[l72 as usize] = 0.0;
		}
		for l73 in 0..2048 {
			self.fVec57[l73 as usize] = 0.0;
		}
		for l74 in 0..4096 {
			self.fVec58[l74 as usize] = 0.0;
		}
		for l75 in 0..8192 {
			self.fVec59[l75 as usize] = 0.0;
		}
		for l76 in 0..16384 {
			self.fVec60[l76 as usize] = 0.0;
		}
		for l77 in 0..2 {
			self.fVec61[l77 as usize] = 0.0;
		}
		for l78 in 0..2 {
			self.fVec62[l78 as usize] = 0.0;
		}
		for l79 in 0..2 {
			self.fVec63[l79 as usize] = 0.0;
		}
		for l80 in 0..2 {
			self.fRec15[l80 as usize] = 0.0;
		}
		for l81 in 0..32768 {
			self.fRec16[l81 as usize] = 0.0;
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
		ui_interface.declare(None, "99", "");
		ui_interface.open_vertical_box("gain reduction");
		ui_interface.declare(Some(ParamIndex(15)), "unit", "dB");
		ui_interface.add_horizontal_bargraph("0", ParamIndex(15), -24.0, 0.0);
		ui_interface.declare(Some(ParamIndex(16)), "unit", "dB");
		ui_interface.add_horizontal_bargraph("1", ParamIndex(16), -24.0, 0.0);
		ui_interface.close_box();
		ui_interface.declare(Some(ParamIndex(17)), "99", "");
		ui_interface.declare(Some(ParamIndex(17)), "unit", "samples");
		ui_interface.add_horizontal_bargraph("latency", ParamIndex(17), 0.0, 1.92e+04);
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			0 => Some(self.fCheckbox0),
			1 => Some(self.fCheckbox1),
			15 => Some(self.fHbargraph0),
			17 => Some(self.fHbargraph1),
			16 => Some(self.fHbargraph2),
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
			17 => { self.fHbargraph1 = value }
			16 => { self.fHbargraph2 = value }
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
		let (outputs0, outputs1) = if let [outputs0, outputs1, ..] = outputs {
			let outputs0 = outputs0[..count as usize].iter_mut();
			let outputs1 = outputs1[..count as usize].iter_mut();
			(outputs0, outputs1)
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
		self.fHbargraph1 = if (fSlow77) as i32 != 0 {1.92e+04} else {fSlow76};
		let mut iSlow79: i32 = (self.fHbargraph1) as i32;
		let mut fSlow80: F64 = self.fConst3 * F64::powf(1e+01, 0.05 * self.fHslider12);
		let zipped_iterators = inputs0.zip(inputs1).zip(outputs0).zip(outputs1);
		for (((input0, input1), output0), output1) in zipped_iterators {
			let mut fTemp0: F64 = self.fConst1 + self.fRec0[1];
			let mut fTemp1: F64 = self.fRec0[1] - self.fConst1;
			self.fRec0[0] = if (fTemp0 < fSlow0) as i32 != 0 {fTemp0} else {if (fTemp1 > fSlow0) as i32 != 0 {fTemp1} else {fSlow0}};
			let mut fTemp2: F64 = F64::sin(6.283185307179586 * (0.5 * self.fRec0[0] + 0.75)) + 1.0;
			let mut fTemp3: F64 = 1.0 - 0.5 * fTemp2;
			let mut fTemp4: F64 = self.fRec2[((i32::wrapping_sub(self.IOTA0, 1)) & 32767) as usize];
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
			self.fVec0[(self.IOTA0 & 32767) as usize] = fTemp12;
			let mut fTemp13: F64 = fTemp12 * self.fRec11[0];
			self.fVec1[(self.IOTA0 & 32767) as usize] = fTemp13;
			let mut fTemp14: F64 = F64::abs(fTemp13);
			let mut fTemp15: F64 = *input1;
			self.fVec2[(self.IOTA0 & 32767) as usize] = fTemp15;
			let mut fTemp16: F64 = fTemp15 * self.fRec11[0];
			self.fVec3[(self.IOTA0 & 32767) as usize] = fTemp16;
			let mut fTemp17: F64 = F64::abs(fTemp16);
			let mut fTemp18: F64 = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, F64::max(fTemp14, fTemp17)));
			let mut iTemp19: i32 = ((fTemp18 > fSlow11) as i32) + ((fTemp18 > fSlow9) as i32);
			let mut fTemp20: F64 = fTemp18 - fSlow8;
			let mut fTemp21: F64 = F64::powf(1e+01, -(0.05 * F64::max(0.0, if (iTemp19 == 0) as i32 != 0 {0.0} else {if (iTemp19 == 1) as i32 != 0 {fSlow12 * LambRs192k_faustpower2_f(fSlow7 + fTemp20)} else {fTemp20}})));
			let mut fTemp22: F64 = 3.0 * fTemp5;
			let mut fTemp23: F64 = 4.0 * (F64::max(0.25, self.fRec4[0]) + -0.25);
			let mut fTemp24: F64 = 9.0 - fTemp23;
			let mut fTemp25: F64 = self.fRec5[1] - self.fRec6[1];
			let mut fTemp26: F64 = if (fTemp21 > self.fRec10[1]) as i32 != 0 {F64::exp(-(self.fConst7 / F64::max(2.220446049250313e-16, fSlow14 / F64::max(1.0 - (F64::max(fTemp23 + -9.0, F64::min(2.0 - fTemp22, fTemp25)) + fTemp24) / (11.0 - (fTemp23 + fTemp22)), 2.220446049250313e-16))))} else {self.fConst6};
			self.fRec10[0] = self.fRec10[1] * fTemp26 + fTemp21 * (1.0 - fTemp26);
			let mut fTemp27: F64 = if (self.fRec10[0] > self.fRec9[1]) as i32 != 0 {0.0} else {self.fConst6};
			self.fRec9[0] = self.fRec9[1] * fTemp27 + self.fRec10[0] * (1.0 - fTemp27);
			let mut fTemp28: F64 = if (self.fRec9[0] > self.fRec8[1]) as i32 != 0 {0.0} else {self.fConst6};
			self.fRec8[0] = self.fRec8[1] * fTemp28 + self.fRec9[0] * (1.0 - fTemp28);
			let mut fTemp29: F64 = if (self.fRec8[0] > self.fRec7[1]) as i32 != 0 {0.0} else {self.fConst6};
			self.fRec7[0] = self.fRec7[1] * fTemp29 + self.fRec8[0] * (1.0 - fTemp29);
			self.fRec5[0] = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, self.fRec7[0]));
			let mut fTemp30: F64 = F64::powf(1e+01, 0.05 * (self.fRec5[1] + fTemp24));
			let mut fTemp31: F64 = if (fTemp30 > self.fRec12[1]) as i32 != 0 {F64::exp(-(self.fConst7 / F64::max(2.220446049250313e-16, (0.8161290322580644 * (F64::max(0.69, self.fRec4[0]) + -0.69) + 0.05) * F64::powf(4503599627370496.0, 1.0 - (F64::max(fTemp10, F64::min(fTemp11, fTemp25)) + fTemp9) / fTemp8))))} else {self.fConst8};
			self.fRec12[0] = self.fRec12[1] * fTemp31 + fTemp30 * (1.0 - fTemp31);
			self.fRec6[0] = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, self.fRec12[0]));
			let mut fTemp32: F64 = self.fRec5[0] - self.fRec6[0];
			let mut fTemp33: F64 = F64::powf(1e+01, fSlow16 * F64::min(0.25, self.fRec4[0]) * (self.fRec6[0] + fTemp32 * (F64::max(fTemp10, F64::min(fTemp11, fTemp32)) + fTemp9) / fTemp8));
			let mut fTemp34: F64 = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, fTemp14));
			let mut fTemp35: F64 = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, fTemp17));
			let mut fTemp36: F64 = F64::max(fTemp34, fTemp35);
			let mut fTemp37: F64 = fTemp34 + fSlow17 * (fTemp36 - fTemp34);
			let mut iTemp38: i32 = ((fTemp37 > fSlow11) as i32) + ((fTemp37 > fSlow9) as i32);
			let mut fTemp39: F64 = fTemp37 - fSlow8;
			let mut fTemp40: F64 = F64::min(fTemp33, F64::powf(1e+01, -(fSlow18 * F64::max(0.0, if (iTemp38 == 0) as i32 != 0 {0.0} else {if (iTemp38 == 1) as i32 != 0 {fSlow12 * LambRs192k_faustpower2_f(fSlow7 + fTemp39)} else {fTemp39}}))));
			self.fVec4[(self.IOTA0 & 16383) as usize] = fTemp40;
			let mut fTemp41: F64 = F64::min(fTemp40, self.fVec4[((i32::wrapping_sub(self.IOTA0, 1)) & 16383) as usize]);
			self.fVec5[0] = fTemp41;
			let mut fTemp42: F64 = F64::min(fTemp41, self.fVec5[2]);
			self.fVec6[0] = fTemp42;
			let mut fTemp43: F64 = F64::min(fTemp42, self.fVec6[4]);
			self.fVec7[0] = fTemp43;
			let mut fTemp44: F64 = F64::min(fTemp43, self.fVec7[8]);
			self.fVec8[(self.IOTA0 & 31) as usize] = fTemp44;
			let mut fTemp45: F64 = F64::min(fTemp44, self.fVec8[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize]);
			self.fVec9[(self.IOTA0 & 63) as usize] = fTemp45;
			let mut fTemp46: F64 = F64::min(fTemp45, self.fVec9[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize]);
			self.fVec10[(self.IOTA0 & 127) as usize] = fTemp46;
			let mut fTemp47: F64 = F64::min(fTemp46, self.fVec10[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize]);
			self.fVec11[(self.IOTA0 & 255) as usize] = fTemp47;
			let mut fTemp48: F64 = F64::min(fTemp47, self.fVec11[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize]);
			self.fVec12[(self.IOTA0 & 511) as usize] = fTemp48;
			let mut fTemp49: F64 = F64::min(fTemp48, self.fVec12[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize]);
			self.fVec13[(self.IOTA0 & 1023) as usize] = fTemp49;
			let mut fTemp50: F64 = F64::min(fTemp49, self.fVec13[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize]);
			self.fVec14[(self.IOTA0 & 2047) as usize] = fTemp50;
			let mut fTemp51: F64 = F64::min(fTemp50, self.fVec14[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			self.fVec15[(self.IOTA0 & 4095) as usize] = fTemp51;
			let mut fTemp52: F64 = F64::min(fTemp51, self.fVec15[((i32::wrapping_sub(self.IOTA0, 2048)) & 4095) as usize]);
			self.fVec16[(self.IOTA0 & 8191) as usize] = fTemp52;
			self.fVec17[(self.IOTA0 & 16383) as usize] = F64::min(fTemp52, self.fVec16[((i32::wrapping_sub(self.IOTA0, 4096)) & 8191) as usize]);
			self.fRec3[0] = F64::max(F64::min(self.fRec3[1], self.fVec4[((i32::wrapping_sub(self.IOTA0, iSlow21)) & 16383) as usize]), F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow23 != 0 {fTemp40} else {1.7976931348623157e+308}, if iSlow24 != 0 {self.fVec5[iSlow23 as usize]} else {1.7976931348623157e+308}), if iSlow25 != 0 {self.fVec6[iSlow26 as usize]} else {1.7976931348623157e+308}), if iSlow27 != 0 {self.fVec7[iSlow28 as usize]} else {1.7976931348623157e+308}), if iSlow29 != 0 {self.fVec8[((i32::wrapping_sub(self.IOTA0, iSlow30)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow31 != 0 {self.fVec9[((i32::wrapping_sub(self.IOTA0, iSlow32)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow33 != 0 {self.fVec10[((i32::wrapping_sub(self.IOTA0, iSlow34)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow35 != 0 {self.fVec11[((i32::wrapping_sub(self.IOTA0, iSlow36)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow37 != 0 {self.fVec12[((i32::wrapping_sub(self.IOTA0, iSlow38)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow39 != 0 {self.fVec13[((i32::wrapping_sub(self.IOTA0, iSlow40)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow41 != 0 {self.fVec14[((i32::wrapping_sub(self.IOTA0, iSlow42)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow43 != 0 {self.fVec15[((i32::wrapping_sub(self.IOTA0, iSlow44)) & 4095) as usize]} else {1.7976931348623157e+308}), if iSlow45 != 0 {self.fVec16[((i32::wrapping_sub(self.IOTA0, iSlow46)) & 8191) as usize]} else {1.7976931348623157e+308}), if iSlow47 != 0 {self.fVec17[((i32::wrapping_sub(self.IOTA0, iSlow48)) & 16383) as usize]} else {1.7976931348623157e+308}));
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
			let mut fTemp64: F64 = F64::min(fTemp63, self.fVec28[((i32::wrapping_sub(self.IOTA0, 2048)) & 4095) as usize]);
			self.fVec29[(self.IOTA0 & 8191) as usize] = fTemp64;
			self.fVec30[(self.IOTA0 & 16383) as usize] = F64::min(fTemp64, self.fVec29[((i32::wrapping_sub(self.IOTA0, 4096)) & 8191) as usize]);
			let mut fTemp65: F64 = F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow4 != 0 {self.fRec3[0]} else {1.7976931348623157e+308}, if iSlow49 != 0 {self.fVec18[iSlow4 as usize]} else {1.7976931348623157e+308}), if iSlow50 != 0 {self.fVec19[iSlow51 as usize]} else {1.7976931348623157e+308}), if iSlow52 != 0 {self.fVec20[iSlow53 as usize]} else {1.7976931348623157e+308}), if iSlow54 != 0 {self.fVec21[((i32::wrapping_sub(self.IOTA0, iSlow55)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow56 != 0 {self.fVec22[((i32::wrapping_sub(self.IOTA0, iSlow57)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow58 != 0 {self.fVec23[((i32::wrapping_sub(self.IOTA0, iSlow59)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow60 != 0 {self.fVec24[((i32::wrapping_sub(self.IOTA0, iSlow61)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow62 != 0 {self.fVec25[((i32::wrapping_sub(self.IOTA0, iSlow63)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow64 != 0 {self.fVec26[((i32::wrapping_sub(self.IOTA0, iSlow65)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow66 != 0 {self.fVec27[((i32::wrapping_sub(self.IOTA0, iSlow67)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow68 != 0 {self.fVec28[((i32::wrapping_sub(self.IOTA0, iSlow69)) & 4095) as usize]} else {1.7976931348623157e+308}), if iSlow70 != 0 {self.fVec29[((i32::wrapping_sub(self.IOTA0, iSlow71)) & 8191) as usize]} else {1.7976931348623157e+308}), if iSlow72 != 0 {self.fVec30[((i32::wrapping_sub(self.IOTA0, iSlow73)) & 16383) as usize]} else {1.7976931348623157e+308}) - fTemp4;
			self.fVec31[0] = fTemp65;
			let mut iTemp66: i32 = (fTemp65 > 0.0) as i32;
			let mut fTemp72: F64 = if iTemp66 != 0 {fSlow75} else {fSlow74};
			self.fVec32[0] = fTemp72;
			let mut fTemp73: F64 = 2.0 * fTemp72;
			let mut iTemp74: i32 = (fTemp73) as i32;
			let mut iTemp75: i32 = std::cmp::max(0, std::cmp::min(iTemp74, 2));
			let mut iTemp76: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, 98301), 196607));
			let mut fTemp77: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp76, 3)) as usize] };
			let mut fTemp78: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp76 as usize] };
			let mut fTemp79: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp76, 1)) as usize] } - fTemp78;
			let mut fTemp80: F64 = fTemp73 - (iTemp74) as F64;
			let mut fTemp81: F64 = fTemp78 + fTemp80 * fTemp79 + 0.5 * (fTemp77 - (fTemp78 + fTemp80 * (fTemp79 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp76, 4)) as usize] } - fTemp77))));
			let mut fTemp82: F64 = if iTemp66 != 0 {fTemp81} else {1.0 - fTemp81};
			let mut iTemp83: i32 = (fTemp65 < 0.0) as i32;
			let mut fTemp84: F64 = fSlow1 * (iTemp83) as F64 + fSlow13 * (iTemp66) as F64;
			self.fVec33[0] = fTemp84;
			let mut fTemp85: F64 = self.fConst10 / fTemp84;
			let mut fTemp86: F64 = fTemp85 + 0.5;
			let mut fTemp87: F64 = 65535.0 * (1.0 - fTemp86);
			let mut iTemp88: i32 = (fTemp87) as i32;
			let mut iTemp89: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp88, 65535)))), 196607));
			let mut fTemp90: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp89, 3)) as usize] };
			let mut fTemp91: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp89 as usize] };
			let mut fTemp92: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp89, 1)) as usize] } - fTemp91;
			let mut fTemp93: F64 = 65535.0 * fTemp86;
			let mut iTemp94: i32 = (fTemp93) as i32;
			let mut iTemp95: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp94, 65535)))), 196607));
			let mut fTemp96: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp95, 3), 196607))) as usize] };
			let mut fTemp97: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp95 as usize] };
			let mut fTemp98: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp95, 1), 196607))) as usize] } - fTemp97;
			let mut fTemp99: F64 = 2.0 * self.fVec32[1];
			let mut iTemp100: i32 = (fTemp99) as i32;
			let mut iTemp101: i32 = std::cmp::max(0, std::cmp::min(iTemp100, 2));
			let mut fTemp102: F64 = 65535.0 * (1.0 - self.fRec1[1]);
			let mut iTemp103: i32 = (fTemp102) as i32;
			let mut iTemp104: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp103, 65535))), iTemp101), 196607));
			let mut fTemp105: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp104, 3), 196607))) as usize] };
			let mut fTemp106: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp104 as usize] };
			let mut fTemp107: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp104, 1), 196607))) as usize] } - fTemp106;
			let mut fTemp108: F64 = fTemp99 - (iTemp100) as F64;
			let mut fTemp109: F64 = 65535.0 * self.fRec1[1];
			let mut iTemp110: i32 = (fTemp109) as i32;
			let mut iTemp111: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp101, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp110, 65535)))), 196607));
			let mut fTemp112: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp111, 3), 196607))) as usize] };
			let mut fTemp113: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp111 as usize] };
			let mut fTemp114: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp111, 1), 196607))) as usize] } - fTemp113;
			let mut fTemp115: F64 = self.fRec1[1] + fTemp85;
			let mut fTemp116: F64 = 65535.0 * (1.0 - fTemp115);
			let mut iTemp117: i32 = (fTemp116) as i32;
			let mut iTemp118: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp117, 65535)))), 196607));
			let mut fTemp119: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp118, 3)) as usize] };
			let mut fTemp120: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp118 as usize] };
			let mut fTemp121: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp118, 1)) as usize] } - fTemp120;
			let mut fTemp122: F64 = 65535.0 * fTemp115;
			let mut iTemp123: i32 = (fTemp122) as i32;
			let mut iTemp124: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp123, 65535)))), 196607));
			let mut fTemp125: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp124, 3), 196607))) as usize] };
			let mut fTemp126: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp124 as usize] };
			let mut fTemp127: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp124, 1), 196607))) as usize] } - fTemp126;
			let mut fTemp128: F64 = self.fRec1[1] + self.fConst10 * (1.0 / fTemp84 + 1.0 / self.fVec33[1]);
			let mut fTemp129: F64 = 65535.0 * (1.0 - fTemp128);
			let mut iTemp130: i32 = (fTemp129) as i32;
			let mut iTemp131: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp130, 65535))), iTemp75), 196607));
			let mut fTemp132: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp131, 3)) as usize] };
			let mut fTemp133: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp131 as usize] };
			let mut fTemp134: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp131, 1)) as usize] } - fTemp133;
			let mut fTemp135: F64 = 65535.0 * fTemp128;
			let mut iTemp136: i32 = (fTemp135) as i32;
			let mut iTemp137: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp136, 65535)))), 196607));
			let mut fTemp138: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp137, 3), 196607))) as usize] };
			let mut fTemp139: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp137 as usize] };
			let mut fTemp140: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp137, 1), 196607))) as usize] } - fTemp139;
			let mut fTemp141: F64 = (if iTemp66 != 0 {fTemp139 + fTemp80 * fTemp140 + (fTemp135 - (iTemp136) as F64) * (fTemp138 - (fTemp139 + fTemp80 * (fTemp140 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp137, 4), 196607))) as usize] } - fTemp138))))} else {1.0 - (fTemp133 + fTemp80 * fTemp134 + (fTemp129 - (iTemp130) as F64) * (fTemp132 - (fTemp133 + fTemp80 * (fTemp134 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp131, 4)) as usize] } - fTemp132)))))} - if iTemp66 != 0 {fTemp126 + fTemp80 * fTemp127 + (fTemp122 - (iTemp123) as F64) * (fTemp125 - (fTemp126 + fTemp80 * (fTemp127 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp124, 4), 196607))) as usize] } - fTemp125))))} else {1.0 - (fTemp120 + fTemp80 * fTemp121 + (fTemp116 - (iTemp117) as F64) * (fTemp119 - (fTemp120 + fTemp80 * (fTemp121 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp118, 4), 196607))) as usize] } - fTemp119)))))}) * self.fVec31[1] / (fTemp65 * (1.0 - if iTemp66 != 0 {fTemp113 + fTemp108 * fTemp114 + (fTemp109 - (iTemp110) as F64) * (fTemp112 - (fTemp113 + fTemp108 * (fTemp114 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp111, 4), 196607))) as usize] } - fTemp112))))} else {1.0 - (fTemp106 + fTemp108 * fTemp107 + (fTemp102 - (iTemp103) as F64) * (fTemp105 - (fTemp106 + fTemp108 * (fTemp107 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp104, 4), 196607))) as usize] } - fTemp105)))))}));
			let mut iTemp142: i32 = (fTemp141 > ((if iTemp66 != 0 {fTemp97 + fTemp80 * fTemp98 + (fTemp93 - (iTemp94) as F64) * (fTemp96 - (fTemp97 + fTemp80 * (fTemp98 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp95, 4), 196607))) as usize] } - fTemp96))))} else {1.0 - (fTemp91 + fTemp80 * fTemp92 + (fTemp87 - (iTemp88) as F64) * (fTemp90 - (fTemp91 + fTemp80 * (fTemp92 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp89, 4)) as usize] } - fTemp90)))))} - fTemp82) / (1.0 - fTemp82))) as i32;
			let mut fTemp143: F64 = if iTemp142 != 0 {1.0} else {0.5};
			let mut fTemp144: F64 = if iTemp142 != 0 {0.5} else {0.0};
			let mut fTemp145: F64 = fTemp144 + fTemp143;
			let mut fTemp146: F64 = 0.5 * fTemp145;
			let mut fTemp147: F64 = 65535.0 * (1.0 - fTemp146);
			let mut iTemp148: i32 = (fTemp147) as i32;
			let mut iTemp149: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp148, 65535)))), 196607));
			let mut fTemp150: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp149, 3)) as usize] };
			let mut fTemp151: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp149 as usize] };
			let mut fTemp152: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp149, 1)) as usize] } - fTemp151;
			let mut fTemp153: F64 = 32767.5 * fTemp145;
			let mut iTemp154: i32 = (fTemp153) as i32;
			let mut iTemp155: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp154, 65535)))), 196607));
			let mut fTemp156: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp155, 3)) as usize] };
			let mut fTemp157: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp155 as usize] };
			let mut fTemp158: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp155, 1)) as usize] } - fTemp157;
			let mut fTemp159: F64 = if iTemp66 != 0 {fTemp157 + fTemp80 * fTemp158 + (fTemp153 - (iTemp154) as F64) * (fTemp156 - (fTemp157 + fTemp80 * (fTemp158 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp155, 4)) as usize] } - fTemp156))))} else {1.0 - (fTemp151 + fTemp80 * fTemp152 + (fTemp147 - (iTemp148) as F64) * (fTemp150 - (fTemp151 + fTemp80 * (fTemp152 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp149, 4)) as usize] } - fTemp150)))))};
			let mut fTemp160: F64 = fTemp85 + fTemp146;
			let mut fTemp161: F64 = 65535.0 * (1.0 - fTemp160);
			let mut iTemp162: i32 = (fTemp161) as i32;
			let mut iTemp163: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp162, 65535)))), 196607));
			let mut fTemp164: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp163, 3)) as usize] };
			let mut fTemp165: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp163 as usize] };
			let mut fTemp166: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp163, 1)) as usize] } - fTemp165;
			let mut fTemp167: F64 = 65535.0 * fTemp160;
			let mut iTemp168: i32 = (fTemp167) as i32;
			let mut iTemp169: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp168, 65535)))), 196607));
			let mut fTemp170: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp169, 3), 196607))) as usize] };
			let mut fTemp171: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp169 as usize] };
			let mut fTemp172: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp169, 1), 196607))) as usize] } - fTemp171;
			let mut iTemp173: i32 = (fTemp141 > ((if iTemp66 != 0 {fTemp171 + fTemp80 * fTemp172 + (fTemp167 - (iTemp168) as F64) * (fTemp170 - (fTemp171 + fTemp80 * (fTemp172 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp169, 4), 196607))) as usize] } - fTemp170))))} else {1.0 - (fTemp165 + fTemp80 * fTemp166 + (fTemp161 - (iTemp162) as F64) * (fTemp164 - (fTemp165 + fTemp80 * (fTemp166 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp163, 4)) as usize] } - fTemp164)))))} - fTemp159) / (1.0 - fTemp159))) as i32;
			let mut fTemp174: F64 = if iTemp173 != 0 {fTemp143} else {fTemp146};
			let mut fTemp175: F64 = if iTemp173 != 0 {fTemp146} else {fTemp144};
			let mut fTemp176: F64 = fTemp175 + fTemp174;
			let mut fTemp177: F64 = 0.5 * fTemp176;
			let mut fTemp178: F64 = 65535.0 * (1.0 - fTemp177);
			let mut iTemp179: i32 = (fTemp178) as i32;
			let mut iTemp180: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp179, 65535)))), 196607));
			let mut fTemp181: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp180, 3)) as usize] };
			let mut fTemp182: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp180 as usize] };
			let mut fTemp183: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp180, 1)) as usize] } - fTemp182;
			let mut fTemp184: F64 = 32767.5 * fTemp176;
			let mut iTemp185: i32 = (fTemp184) as i32;
			let mut iTemp186: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp185, 65535)))), 196607));
			let mut fTemp187: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp186, 3)) as usize] };
			let mut fTemp188: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp186 as usize] };
			let mut fTemp189: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp186, 1)) as usize] } - fTemp188;
			let mut fTemp190: F64 = if iTemp66 != 0 {fTemp188 + fTemp80 * fTemp189 + (fTemp184 - (iTemp185) as F64) * (fTemp187 - (fTemp188 + fTemp80 * (fTemp189 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp186, 4)) as usize] } - fTemp187))))} else {1.0 - (fTemp182 + fTemp80 * fTemp183 + (fTemp178 - (iTemp179) as F64) * (fTemp181 - (fTemp182 + fTemp80 * (fTemp183 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp180, 4)) as usize] } - fTemp181)))))};
			let mut fTemp191: F64 = fTemp85 + fTemp177;
			let mut fTemp192: F64 = 65535.0 * (1.0 - fTemp191);
			let mut iTemp193: i32 = (fTemp192) as i32;
			let mut iTemp194: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp193, 65535)))), 196607));
			let mut fTemp195: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp194, 3)) as usize] };
			let mut fTemp196: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp194 as usize] };
			let mut fTemp197: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp194, 1)) as usize] } - fTemp196;
			let mut fTemp198: F64 = 65535.0 * fTemp191;
			let mut iTemp199: i32 = (fTemp198) as i32;
			let mut iTemp200: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp199, 65535)))), 196607));
			let mut fTemp201: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp200, 3), 196607))) as usize] };
			let mut fTemp202: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp200 as usize] };
			let mut fTemp203: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp200, 1), 196607))) as usize] } - fTemp202;
			let mut iTemp204: i32 = (fTemp141 > ((if iTemp66 != 0 {fTemp202 + fTemp80 * fTemp203 + (fTemp198 - (iTemp199) as F64) * (fTemp201 - (fTemp202 + fTemp80 * (fTemp203 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp200, 4), 196607))) as usize] } - fTemp201))))} else {1.0 - (fTemp196 + fTemp80 * fTemp197 + (fTemp192 - (iTemp193) as F64) * (fTemp195 - (fTemp196 + fTemp80 * (fTemp197 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp194, 4)) as usize] } - fTemp195)))))} - fTemp190) / (1.0 - fTemp190))) as i32;
			let mut fTemp205: F64 = if iTemp204 != 0 {fTemp174} else {fTemp177};
			let mut fTemp206: F64 = if iTemp204 != 0 {fTemp177} else {fTemp175};
			let mut fTemp207: F64 = fTemp206 + fTemp205;
			let mut fTemp208: F64 = 0.5 * fTemp207;
			let mut fTemp209: F64 = 65535.0 * (1.0 - fTemp208);
			let mut iTemp210: i32 = (fTemp209) as i32;
			let mut iTemp211: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp210, 65535)))), 196607));
			let mut fTemp212: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp211, 3)) as usize] };
			let mut fTemp213: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp211 as usize] };
			let mut fTemp214: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp211, 1)) as usize] } - fTemp213;
			let mut fTemp215: F64 = 32767.5 * fTemp207;
			let mut iTemp216: i32 = (fTemp215) as i32;
			let mut iTemp217: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp216, 65535)))), 196607));
			let mut fTemp218: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp217, 3)) as usize] };
			let mut fTemp219: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp217 as usize] };
			let mut fTemp220: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp217, 1)) as usize] } - fTemp219;
			let mut fTemp221: F64 = if iTemp66 != 0 {fTemp219 + fTemp80 * fTemp220 + (fTemp215 - (iTemp216) as F64) * (fTemp218 - (fTemp219 + fTemp80 * (fTemp220 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp217, 4)) as usize] } - fTemp218))))} else {1.0 - (fTemp213 + fTemp80 * fTemp214 + (fTemp209 - (iTemp210) as F64) * (fTemp212 - (fTemp213 + fTemp80 * (fTemp214 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp211, 4)) as usize] } - fTemp212)))))};
			let mut fTemp222: F64 = fTemp85 + fTemp208;
			let mut fTemp223: F64 = 65535.0 * (1.0 - fTemp222);
			let mut iTemp224: i32 = (fTemp223) as i32;
			let mut iTemp225: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp224, 65535)))), 196607));
			let mut fTemp226: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp225, 3)) as usize] };
			let mut fTemp227: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp225 as usize] };
			let mut fTemp228: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp225, 1)) as usize] } - fTemp227;
			let mut fTemp229: F64 = 65535.0 * fTemp222;
			let mut iTemp230: i32 = (fTemp229) as i32;
			let mut iTemp231: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp230, 65535)))), 196607));
			let mut fTemp232: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp231, 3), 196607))) as usize] };
			let mut fTemp233: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp231 as usize] };
			let mut fTemp234: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp231, 1), 196607))) as usize] } - fTemp233;
			let mut iTemp235: i32 = (fTemp141 > ((if iTemp66 != 0 {fTemp233 + fTemp80 * fTemp234 + (fTemp229 - (iTemp230) as F64) * (fTemp232 - (fTemp233 + fTemp80 * (fTemp234 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp231, 4), 196607))) as usize] } - fTemp232))))} else {1.0 - (fTemp227 + fTemp80 * fTemp228 + (fTemp223 - (iTemp224) as F64) * (fTemp226 - (fTemp227 + fTemp80 * (fTemp228 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp225, 4)) as usize] } - fTemp226)))))} - fTemp221) / (1.0 - fTemp221))) as i32;
			let mut fTemp236: F64 = if iTemp235 != 0 {fTemp205} else {fTemp208};
			let mut fTemp237: F64 = if iTemp235 != 0 {fTemp208} else {fTemp206};
			let mut fTemp238: F64 = fTemp237 + fTemp236;
			let mut fTemp239: F64 = 0.5 * fTemp238;
			let mut fTemp240: F64 = 65535.0 * (1.0 - fTemp239);
			let mut iTemp241: i32 = (fTemp240) as i32;
			let mut iTemp242: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp241, 65535)))), 196607));
			let mut fTemp243: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp242, 3)) as usize] };
			let mut fTemp244: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp242 as usize] };
			let mut fTemp245: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp242, 1)) as usize] } - fTemp244;
			let mut fTemp246: F64 = 32767.5 * fTemp238;
			let mut iTemp247: i32 = (fTemp246) as i32;
			let mut iTemp248: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp247, 65535)))), 196607));
			let mut fTemp249: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp248, 3)) as usize] };
			let mut fTemp250: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp248 as usize] };
			let mut fTemp251: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp248, 1)) as usize] } - fTemp250;
			let mut fTemp252: F64 = if iTemp66 != 0 {fTemp250 + fTemp80 * fTemp251 + (fTemp246 - (iTemp247) as F64) * (fTemp249 - (fTemp250 + fTemp80 * (fTemp251 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp248, 4)) as usize] } - fTemp249))))} else {1.0 - (fTemp244 + fTemp80 * fTemp245 + (fTemp240 - (iTemp241) as F64) * (fTemp243 - (fTemp244 + fTemp80 * (fTemp245 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp242, 4)) as usize] } - fTemp243)))))};
			let mut fTemp253: F64 = fTemp85 + fTemp239;
			let mut fTemp254: F64 = 65535.0 * (1.0 - fTemp253);
			let mut iTemp255: i32 = (fTemp254) as i32;
			let mut iTemp256: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp255, 65535)))), 196607));
			let mut fTemp257: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp256, 3)) as usize] };
			let mut fTemp258: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp256 as usize] };
			let mut fTemp259: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp256, 1)) as usize] } - fTemp258;
			let mut fTemp260: F64 = 65535.0 * fTemp253;
			let mut iTemp261: i32 = (fTemp260) as i32;
			let mut iTemp262: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp261, 65535)))), 196607));
			let mut fTemp263: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp262, 3), 196607))) as usize] };
			let mut fTemp264: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp262 as usize] };
			let mut fTemp265: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp262, 1), 196607))) as usize] } - fTemp264;
			let mut iTemp266: i32 = (fTemp141 > ((if iTemp66 != 0 {fTemp264 + fTemp80 * fTemp265 + (fTemp260 - (iTemp261) as F64) * (fTemp263 - (fTemp264 + fTemp80 * (fTemp265 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp262, 4), 196607))) as usize] } - fTemp263))))} else {1.0 - (fTemp258 + fTemp80 * fTemp259 + (fTemp254 - (iTemp255) as F64) * (fTemp257 - (fTemp258 + fTemp80 * (fTemp259 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp256, 4)) as usize] } - fTemp257)))))} - fTemp252) / (1.0 - fTemp252))) as i32;
			let mut fTemp267: F64 = if iTemp266 != 0 {fTemp236} else {fTemp239};
			let mut fTemp268: F64 = if iTemp266 != 0 {fTemp239} else {fTemp237};
			let mut fTemp269: F64 = fTemp268 + fTemp267;
			let mut fTemp270: F64 = 0.5 * fTemp269;
			let mut fTemp271: F64 = 65535.0 * (1.0 - fTemp270);
			let mut iTemp272: i32 = (fTemp271) as i32;
			let mut iTemp273: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp272, 65535)))), 196607));
			let mut fTemp274: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp273, 3)) as usize] };
			let mut fTemp275: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp273 as usize] };
			let mut fTemp276: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp273, 1)) as usize] } - fTemp275;
			let mut fTemp277: F64 = 32767.5 * fTemp269;
			let mut iTemp278: i32 = (fTemp277) as i32;
			let mut iTemp279: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp278, 65535)))), 196607));
			let mut fTemp280: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp279, 3)) as usize] };
			let mut fTemp281: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp279 as usize] };
			let mut fTemp282: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp279, 1)) as usize] } - fTemp281;
			let mut fTemp283: F64 = if iTemp66 != 0 {fTemp281 + fTemp80 * fTemp282 + (fTemp277 - (iTemp278) as F64) * (fTemp280 - (fTemp281 + fTemp80 * (fTemp282 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp279, 4)) as usize] } - fTemp280))))} else {1.0 - (fTemp275 + fTemp80 * fTemp276 + (fTemp271 - (iTemp272) as F64) * (fTemp274 - (fTemp275 + fTemp80 * (fTemp276 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp273, 4)) as usize] } - fTemp274)))))};
			let mut fTemp284: F64 = fTemp85 + fTemp270;
			let mut fTemp285: F64 = 65535.0 * (1.0 - fTemp284);
			let mut iTemp286: i32 = (fTemp285) as i32;
			let mut iTemp287: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp286, 65535)))), 196607));
			let mut fTemp288: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp287, 3)) as usize] };
			let mut fTemp289: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp287 as usize] };
			let mut fTemp290: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp287, 1)) as usize] } - fTemp289;
			let mut fTemp291: F64 = 65535.0 * fTemp284;
			let mut iTemp292: i32 = (fTemp291) as i32;
			let mut iTemp293: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp292, 65535)))), 196607));
			let mut fTemp294: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp293, 3), 196607))) as usize] };
			let mut fTemp295: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp293 as usize] };
			let mut fTemp296: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp293, 1), 196607))) as usize] } - fTemp295;
			let mut iTemp297: i32 = (fTemp141 > ((if iTemp66 != 0 {fTemp295 + fTemp80 * fTemp296 + (fTemp291 - (iTemp292) as F64) * (fTemp294 - (fTemp295 + fTemp80 * (fTemp296 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp293, 4), 196607))) as usize] } - fTemp294))))} else {1.0 - (fTemp289 + fTemp80 * fTemp290 + (fTemp285 - (iTemp286) as F64) * (fTemp288 - (fTemp289 + fTemp80 * (fTemp290 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp287, 4)) as usize] } - fTemp288)))))} - fTemp283) / (1.0 - fTemp283))) as i32;
			let mut fTemp298: F64 = if iTemp297 != 0 {fTemp267} else {fTemp270};
			let mut fTemp299: F64 = if iTemp297 != 0 {fTemp270} else {fTemp268};
			let mut fTemp300: F64 = fTemp299 + fTemp298;
			let mut fTemp301: F64 = 0.5 * fTemp300;
			let mut fTemp302: F64 = 65535.0 * (1.0 - fTemp301);
			let mut iTemp303: i32 = (fTemp302) as i32;
			let mut iTemp304: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp303, 65535)))), 196607));
			let mut fTemp305: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp304, 3)) as usize] };
			let mut fTemp306: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp304 as usize] };
			let mut fTemp307: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp304, 1)) as usize] } - fTemp306;
			let mut fTemp308: F64 = 32767.5 * fTemp300;
			let mut iTemp309: i32 = (fTemp308) as i32;
			let mut iTemp310: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp309, 65535)))), 196607));
			let mut fTemp311: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp310, 3)) as usize] };
			let mut fTemp312: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp310 as usize] };
			let mut fTemp313: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp310, 1)) as usize] } - fTemp312;
			let mut fTemp314: F64 = if iTemp66 != 0 {fTemp312 + fTemp80 * fTemp313 + (fTemp308 - (iTemp309) as F64) * (fTemp311 - (fTemp312 + fTemp80 * (fTemp313 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp310, 4)) as usize] } - fTemp311))))} else {1.0 - (fTemp306 + fTemp80 * fTemp307 + (fTemp302 - (iTemp303) as F64) * (fTemp305 - (fTemp306 + fTemp80 * (fTemp307 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp304, 4)) as usize] } - fTemp305)))))};
			let mut fTemp315: F64 = fTemp85 + fTemp301;
			let mut fTemp316: F64 = 65535.0 * (1.0 - fTemp315);
			let mut iTemp317: i32 = (fTemp316) as i32;
			let mut iTemp318: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp317, 65535)))), 196607));
			let mut fTemp319: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp318, 3)) as usize] };
			let mut fTemp320: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp318 as usize] };
			let mut fTemp321: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp318, 1)) as usize] } - fTemp320;
			let mut fTemp322: F64 = 65535.0 * fTemp315;
			let mut iTemp323: i32 = (fTemp322) as i32;
			let mut iTemp324: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp323, 65535)))), 196607));
			let mut fTemp325: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp324, 3), 196607))) as usize] };
			let mut fTemp326: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp324 as usize] };
			let mut fTemp327: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp324, 1), 196607))) as usize] } - fTemp326;
			let mut iTemp328: i32 = (fTemp141 > ((if iTemp66 != 0 {fTemp326 + fTemp80 * fTemp327 + (fTemp322 - (iTemp323) as F64) * (fTemp325 - (fTemp326 + fTemp80 * (fTemp327 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp324, 4), 196607))) as usize] } - fTemp325))))} else {1.0 - (fTemp320 + fTemp80 * fTemp321 + (fTemp316 - (iTemp317) as F64) * (fTemp319 - (fTemp320 + fTemp80 * (fTemp321 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp318, 4)) as usize] } - fTemp319)))))} - fTemp314) / (1.0 - fTemp314))) as i32;
			let mut fTemp329: F64 = if iTemp328 != 0 {fTemp298} else {fTemp301};
			let mut fTemp330: F64 = if iTemp328 != 0 {fTemp301} else {fTemp299};
			let mut fTemp331: F64 = fTemp330 + fTemp329;
			let mut fTemp332: F64 = 0.5 * fTemp331;
			let mut fTemp333: F64 = 65535.0 * (1.0 - fTemp332);
			let mut iTemp334: i32 = (fTemp333) as i32;
			let mut iTemp335: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp334, 65535)))), 196607));
			let mut fTemp336: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp335, 3)) as usize] };
			let mut fTemp337: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp335 as usize] };
			let mut fTemp338: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp335, 1)) as usize] } - fTemp337;
			let mut fTemp339: F64 = 32767.5 * fTemp331;
			let mut iTemp340: i32 = (fTemp339) as i32;
			let mut iTemp341: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp340, 65535)))), 196607));
			let mut fTemp342: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp341, 3)) as usize] };
			let mut fTemp343: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp341 as usize] };
			let mut fTemp344: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp341, 1)) as usize] } - fTemp343;
			let mut fTemp345: F64 = if iTemp66 != 0 {fTemp343 + fTemp80 * fTemp344 + (fTemp339 - (iTemp340) as F64) * (fTemp342 - (fTemp343 + fTemp80 * (fTemp344 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp341, 4)) as usize] } - fTemp342))))} else {1.0 - (fTemp337 + fTemp80 * fTemp338 + (fTemp333 - (iTemp334) as F64) * (fTemp336 - (fTemp337 + fTemp80 * (fTemp338 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp335, 4)) as usize] } - fTemp336)))))};
			let mut fTemp346: F64 = fTemp85 + fTemp332;
			let mut fTemp347: F64 = 65535.0 * (1.0 - fTemp346);
			let mut iTemp348: i32 = (fTemp347) as i32;
			let mut iTemp349: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp348, 65535)))), 196607));
			let mut fTemp350: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp349, 3)) as usize] };
			let mut fTemp351: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp349 as usize] };
			let mut fTemp352: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp349, 1)) as usize] } - fTemp351;
			let mut fTemp353: F64 = 65535.0 * fTemp346;
			let mut iTemp354: i32 = (fTemp353) as i32;
			let mut iTemp355: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp354, 65535)))), 196607));
			let mut fTemp356: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp355, 3), 196607))) as usize] };
			let mut fTemp357: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp355 as usize] };
			let mut fTemp358: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp355, 1), 196607))) as usize] } - fTemp357;
			let mut iTemp359: i32 = (fTemp141 > ((if iTemp66 != 0 {fTemp357 + fTemp80 * fTemp358 + (fTemp353 - (iTemp354) as F64) * (fTemp356 - (fTemp357 + fTemp80 * (fTemp358 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp355, 4), 196607))) as usize] } - fTemp356))))} else {1.0 - (fTemp351 + fTemp80 * fTemp352 + (fTemp347 - (iTemp348) as F64) * (fTemp350 - (fTemp351 + fTemp80 * (fTemp352 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp349, 4)) as usize] } - fTemp350)))))} - fTemp345) / (1.0 - fTemp345))) as i32;
			let mut fTemp360: F64 = if iTemp359 != 0 {fTemp329} else {fTemp332};
			let mut fTemp361: F64 = if iTemp359 != 0 {fTemp332} else {fTemp330};
			let mut fTemp362: F64 = fTemp361 + fTemp360;
			let mut fTemp363: F64 = 0.5 * fTemp362;
			let mut fTemp364: F64 = 65535.0 * (1.0 - fTemp363);
			let mut iTemp365: i32 = (fTemp364) as i32;
			let mut iTemp366: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp365, 65535)))), 196607));
			let mut fTemp367: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp366, 3)) as usize] };
			let mut fTemp368: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp366 as usize] };
			let mut fTemp369: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp366, 1)) as usize] } - fTemp368;
			let mut fTemp370: F64 = 32767.5 * fTemp362;
			let mut iTemp371: i32 = (fTemp370) as i32;
			let mut iTemp372: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp371, 65535)))), 196607));
			let mut fTemp373: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp372, 3)) as usize] };
			let mut fTemp374: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp372 as usize] };
			let mut fTemp375: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp372, 1)) as usize] } - fTemp374;
			let mut fTemp376: F64 = if iTemp66 != 0 {fTemp374 + fTemp80 * fTemp375 + (fTemp370 - (iTemp371) as F64) * (fTemp373 - (fTemp374 + fTemp80 * (fTemp375 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp372, 4)) as usize] } - fTemp373))))} else {1.0 - (fTemp368 + fTemp80 * fTemp369 + (fTemp364 - (iTemp365) as F64) * (fTemp367 - (fTemp368 + fTemp80 * (fTemp369 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp366, 4)) as usize] } - fTemp367)))))};
			let mut fTemp377: F64 = fTemp85 + fTemp363;
			let mut fTemp378: F64 = 65535.0 * (1.0 - fTemp377);
			let mut iTemp379: i32 = (fTemp378) as i32;
			let mut iTemp380: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp379, 65535)))), 196607));
			let mut fTemp381: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp380, 3)) as usize] };
			let mut fTemp382: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp380 as usize] };
			let mut fTemp383: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp380, 1)) as usize] } - fTemp382;
			let mut fTemp384: F64 = 65535.0 * fTemp377;
			let mut iTemp385: i32 = (fTemp384) as i32;
			let mut iTemp386: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp385, 65535)))), 196607));
			let mut fTemp387: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp386, 3), 196607))) as usize] };
			let mut fTemp388: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp386 as usize] };
			let mut fTemp389: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp386, 1), 196607))) as usize] } - fTemp388;
			let mut iTemp390: i32 = (fTemp141 > ((if iTemp66 != 0 {fTemp388 + fTemp80 * fTemp389 + (fTemp384 - (iTemp385) as F64) * (fTemp387 - (fTemp388 + fTemp80 * (fTemp389 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp386, 4), 196607))) as usize] } - fTemp387))))} else {1.0 - (fTemp382 + fTemp80 * fTemp383 + (fTemp378 - (iTemp379) as F64) * (fTemp381 - (fTemp382 + fTemp80 * (fTemp383 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp380, 4)) as usize] } - fTemp381)))))} - fTemp376) / (1.0 - fTemp376))) as i32;
			let mut fTemp391: F64 = if iTemp390 != 0 {fTemp360} else {fTemp363};
			let mut fTemp392: F64 = if iTemp390 != 0 {fTemp363} else {fTemp361};
			let mut fTemp393: F64 = fTemp392 + fTemp391;
			let mut fTemp394: F64 = 0.5 * fTemp393;
			let mut fTemp395: F64 = 65535.0 * (1.0 - fTemp394);
			let mut iTemp396: i32 = (fTemp395) as i32;
			let mut iTemp397: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp396, 65535)))), 196607));
			let mut fTemp398: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp397, 3)) as usize] };
			let mut fTemp399: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp397 as usize] };
			let mut fTemp400: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp397, 1)) as usize] } - fTemp399;
			let mut fTemp401: F64 = 32767.5 * fTemp393;
			let mut iTemp402: i32 = (fTemp401) as i32;
			let mut iTemp403: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp402, 65535)))), 196607));
			let mut fTemp404: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp403, 3)) as usize] };
			let mut fTemp405: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp403 as usize] };
			let mut fTemp406: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp403, 1)) as usize] } - fTemp405;
			let mut fTemp407: F64 = if iTemp66 != 0 {fTemp405 + fTemp80 * fTemp406 + (fTemp401 - (iTemp402) as F64) * (fTemp404 - (fTemp405 + fTemp80 * (fTemp406 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp403, 4)) as usize] } - fTemp404))))} else {1.0 - (fTemp399 + fTemp80 * fTemp400 + (fTemp395 - (iTemp396) as F64) * (fTemp398 - (fTemp399 + fTemp80 * (fTemp400 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp397, 4)) as usize] } - fTemp398)))))};
			let mut fTemp408: F64 = fTemp85 + fTemp394;
			let mut fTemp409: F64 = 65535.0 * (1.0 - fTemp408);
			let mut iTemp410: i32 = (fTemp409) as i32;
			let mut iTemp411: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp410, 65535)))), 196607));
			let mut fTemp412: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp411, 3)) as usize] };
			let mut fTemp413: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp411 as usize] };
			let mut fTemp414: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp411, 1)) as usize] } - fTemp413;
			let mut fTemp415: F64 = 65535.0 * fTemp408;
			let mut iTemp416: i32 = (fTemp415) as i32;
			let mut iTemp417: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp416, 65535)))), 196607));
			let mut fTemp418: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp417, 3), 196607))) as usize] };
			let mut fTemp419: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp417 as usize] };
			let mut fTemp420: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp417, 1), 196607))) as usize] } - fTemp419;
			let mut iTemp421: i32 = (fTemp141 > ((if iTemp66 != 0 {fTemp419 + fTemp80 * fTemp420 + (fTemp415 - (iTemp416) as F64) * (fTemp418 - (fTemp419 + fTemp80 * (fTemp420 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp417, 4), 196607))) as usize] } - fTemp418))))} else {1.0 - (fTemp413 + fTemp80 * fTemp414 + (fTemp409 - (iTemp410) as F64) * (fTemp412 - (fTemp413 + fTemp80 * (fTemp414 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp411, 4)) as usize] } - fTemp412)))))} - fTemp407) / (1.0 - fTemp407))) as i32;
			let mut fTemp422: F64 = if iTemp421 != 0 {fTemp391} else {fTemp394};
			let mut fTemp423: F64 = if iTemp421 != 0 {fTemp394} else {fTemp392};
			let mut fTemp424: F64 = fTemp423 + fTemp422;
			let mut fTemp425: F64 = 0.5 * fTemp424;
			let mut fTemp426: F64 = 65535.0 * (1.0 - fTemp425);
			let mut iTemp427: i32 = (fTemp426) as i32;
			let mut iTemp428: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp427, 65535)))), 196607));
			let mut fTemp429: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp428, 3)) as usize] };
			let mut fTemp430: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp428 as usize] };
			let mut fTemp431: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp428, 1)) as usize] } - fTemp430;
			let mut fTemp432: F64 = 32767.5 * fTemp424;
			let mut iTemp433: i32 = (fTemp432) as i32;
			let mut iTemp434: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp433, 65535)))), 196607));
			let mut fTemp435: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp434, 3)) as usize] };
			let mut fTemp436: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp434 as usize] };
			let mut fTemp437: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp434, 1)) as usize] } - fTemp436;
			let mut fTemp438: F64 = if iTemp66 != 0 {fTemp436 + fTemp80 * fTemp437 + (fTemp432 - (iTemp433) as F64) * (fTemp435 - (fTemp436 + fTemp80 * (fTemp437 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp434, 4)) as usize] } - fTemp435))))} else {1.0 - (fTemp430 + fTemp80 * fTemp431 + (fTemp426 - (iTemp427) as F64) * (fTemp429 - (fTemp430 + fTemp80 * (fTemp431 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp428, 4)) as usize] } - fTemp429)))))};
			let mut fTemp439: F64 = fTemp85 + fTemp425;
			let mut fTemp440: F64 = 65535.0 * (1.0 - fTemp439);
			let mut iTemp441: i32 = (fTemp440) as i32;
			let mut iTemp442: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp441, 65535)))), 196607));
			let mut fTemp443: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp442, 3)) as usize] };
			let mut fTemp444: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp442 as usize] };
			let mut fTemp445: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp442, 1)) as usize] } - fTemp444;
			let mut fTemp446: F64 = 65535.0 * fTemp439;
			let mut iTemp447: i32 = (fTemp446) as i32;
			let mut iTemp448: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp447, 65535)))), 196607));
			let mut fTemp449: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp448, 3), 196607))) as usize] };
			let mut fTemp450: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp448 as usize] };
			let mut fTemp451: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp448, 1), 196607))) as usize] } - fTemp450;
			let mut iTemp452: i32 = (fTemp141 > ((if iTemp66 != 0 {fTemp450 + fTemp80 * fTemp451 + (fTemp446 - (iTemp447) as F64) * (fTemp449 - (fTemp450 + fTemp80 * (fTemp451 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp448, 4), 196607))) as usize] } - fTemp449))))} else {1.0 - (fTemp444 + fTemp80 * fTemp445 + (fTemp440 - (iTemp441) as F64) * (fTemp443 - (fTemp444 + fTemp80 * (fTemp445 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp442, 4)) as usize] } - fTemp443)))))} - fTemp438) / (1.0 - fTemp438))) as i32;
			let mut fTemp453: F64 = if iTemp452 != 0 {fTemp422} else {fTemp425};
			let mut fTemp454: F64 = if iTemp452 != 0 {fTemp425} else {fTemp423};
			let mut fTemp455: F64 = fTemp454 + fTemp453;
			let mut fTemp456: F64 = 0.5 * fTemp455;
			let mut fTemp457: F64 = 65535.0 * (1.0 - fTemp456);
			let mut iTemp458: i32 = (fTemp457) as i32;
			let mut iTemp459: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp458, 65535)))), 196607));
			let mut fTemp460: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp459, 3)) as usize] };
			let mut fTemp461: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp459 as usize] };
			let mut fTemp462: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp459, 1)) as usize] } - fTemp461;
			let mut fTemp463: F64 = 32767.5 * fTemp455;
			let mut iTemp464: i32 = (fTemp463) as i32;
			let mut iTemp465: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp464, 65535)))), 196607));
			let mut fTemp466: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp465, 3)) as usize] };
			let mut fTemp467: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp465 as usize] };
			let mut fTemp468: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp465, 1)) as usize] } - fTemp467;
			let mut fTemp469: F64 = if iTemp66 != 0 {fTemp467 + fTemp80 * fTemp468 + (fTemp463 - (iTemp464) as F64) * (fTemp466 - (fTemp467 + fTemp80 * (fTemp468 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp465, 4)) as usize] } - fTemp466))))} else {1.0 - (fTemp461 + fTemp80 * fTemp462 + (fTemp457 - (iTemp458) as F64) * (fTemp460 - (fTemp461 + fTemp80 * (fTemp462 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp459, 4)) as usize] } - fTemp460)))))};
			let mut fTemp470: F64 = fTemp85 + fTemp456;
			let mut fTemp471: F64 = 65535.0 * (1.0 - fTemp470);
			let mut iTemp472: i32 = (fTemp471) as i32;
			let mut iTemp473: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp472, 65535)))), 196607));
			let mut fTemp474: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp473, 3)) as usize] };
			let mut fTemp475: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp473 as usize] };
			let mut fTemp476: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp473, 1)) as usize] } - fTemp475;
			let mut fTemp477: F64 = 65535.0 * fTemp470;
			let mut iTemp478: i32 = (fTemp477) as i32;
			let mut iTemp479: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp478, 65535)))), 196607));
			let mut fTemp480: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp479, 3), 196607))) as usize] };
			let mut fTemp481: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp479 as usize] };
			let mut fTemp482: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp479, 1), 196607))) as usize] } - fTemp481;
			let mut iTemp483: i32 = (fTemp141 > ((if iTemp66 != 0 {fTemp481 + fTemp80 * fTemp482 + (fTemp477 - (iTemp478) as F64) * (fTemp480 - (fTemp481 + fTemp80 * (fTemp482 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp479, 4), 196607))) as usize] } - fTemp480))))} else {1.0 - (fTemp475 + fTemp80 * fTemp476 + (fTemp471 - (iTemp472) as F64) * (fTemp474 - (fTemp475 + fTemp80 * (fTemp476 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp473, 4)) as usize] } - fTemp474)))))} - fTemp469) / (1.0 - fTemp469))) as i32;
			let mut fTemp484: F64 = if iTemp483 != 0 {fTemp453} else {fTemp456};
			let mut fTemp485: F64 = if iTemp483 != 0 {fTemp456} else {fTemp454};
			let mut fTemp486: F64 = fTemp485 + fTemp484;
			let mut fTemp487: F64 = 0.5 * fTemp486;
			let mut fTemp488: F64 = 65535.0 * (1.0 - fTemp487);
			let mut iTemp489: i32 = (fTemp488) as i32;
			let mut iTemp490: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp489, 65535)))), 196607));
			let mut fTemp491: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp490, 3)) as usize] };
			let mut fTemp492: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp490 as usize] };
			let mut fTemp493: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp490, 1)) as usize] } - fTemp492;
			let mut fTemp494: F64 = 32767.5 * fTemp486;
			let mut iTemp495: i32 = (fTemp494) as i32;
			let mut iTemp496: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp495, 65535)))), 196607));
			let mut fTemp497: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp496, 3)) as usize] };
			let mut fTemp498: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp496 as usize] };
			let mut fTemp499: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp496, 1)) as usize] } - fTemp498;
			let mut fTemp500: F64 = if iTemp66 != 0 {fTemp498 + fTemp80 * fTemp499 + (fTemp494 - (iTemp495) as F64) * (fTemp497 - (fTemp498 + fTemp80 * (fTemp499 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp496, 4)) as usize] } - fTemp497))))} else {1.0 - (fTemp492 + fTemp80 * fTemp493 + (fTemp488 - (iTemp489) as F64) * (fTemp491 - (fTemp492 + fTemp80 * (fTemp493 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp490, 4)) as usize] } - fTemp491)))))};
			let mut fTemp501: F64 = fTemp85 + fTemp487;
			let mut fTemp502: F64 = 65535.0 * (1.0 - fTemp501);
			let mut iTemp503: i32 = (fTemp502) as i32;
			let mut iTemp504: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp503, 65535)))), 196607));
			let mut fTemp505: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp504, 3)) as usize] };
			let mut fTemp506: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp504 as usize] };
			let mut fTemp507: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp504, 1)) as usize] } - fTemp506;
			let mut fTemp508: F64 = 65535.0 * fTemp501;
			let mut iTemp509: i32 = (fTemp508) as i32;
			let mut iTemp510: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp509, 65535)))), 196607));
			let mut fTemp511: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp510, 3), 196607))) as usize] };
			let mut fTemp512: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp510 as usize] };
			let mut fTemp513: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp510, 1), 196607))) as usize] } - fTemp512;
			let mut iTemp514: i32 = (fTemp141 > ((if iTemp66 != 0 {fTemp512 + fTemp80 * fTemp513 + (fTemp508 - (iTemp509) as F64) * (fTemp511 - (fTemp512 + fTemp80 * (fTemp513 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp510, 4), 196607))) as usize] } - fTemp511))))} else {1.0 - (fTemp506 + fTemp80 * fTemp507 + (fTemp502 - (iTemp503) as F64) * (fTemp505 - (fTemp506 + fTemp80 * (fTemp507 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp504, 4)) as usize] } - fTemp505)))))} - fTemp500) / (1.0 - fTemp500))) as i32;
			let mut fTemp515: F64 = if iTemp514 != 0 {fTemp484} else {fTemp487};
			let mut fTemp516: F64 = if iTemp514 != 0 {fTemp487} else {fTemp485};
			let mut fTemp517: F64 = fTemp516 + fTemp515;
			let mut fTemp518: F64 = 0.5 * fTemp517;
			let mut fTemp519: F64 = 65535.0 * (1.0 - fTemp518);
			let mut iTemp520: i32 = (fTemp519) as i32;
			let mut iTemp521: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp520, 65535)))), 196607));
			let mut fTemp522: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp521, 3)) as usize] };
			let mut fTemp523: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp521 as usize] };
			let mut fTemp524: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp521, 1)) as usize] } - fTemp523;
			let mut fTemp525: F64 = 32767.5 * fTemp517;
			let mut iTemp526: i32 = (fTemp525) as i32;
			let mut iTemp527: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp526, 65535)))), 196607));
			let mut fTemp528: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp527, 3)) as usize] };
			let mut fTemp529: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp527 as usize] };
			let mut fTemp530: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp527, 1)) as usize] } - fTemp529;
			let mut fTemp531: F64 = if iTemp66 != 0 {fTemp529 + fTemp80 * fTemp530 + (fTemp525 - (iTemp526) as F64) * (fTemp528 - (fTemp529 + fTemp80 * (fTemp530 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp527, 4)) as usize] } - fTemp528))))} else {1.0 - (fTemp523 + fTemp80 * fTemp524 + (fTemp519 - (iTemp520) as F64) * (fTemp522 - (fTemp523 + fTemp80 * (fTemp524 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp521, 4)) as usize] } - fTemp522)))))};
			let mut fTemp532: F64 = fTemp85 + fTemp518;
			let mut fTemp533: F64 = 65535.0 * (1.0 - fTemp532);
			let mut iTemp534: i32 = (fTemp533) as i32;
			let mut iTemp535: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp534, 65535)))), 196607));
			let mut fTemp536: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp535, 3)) as usize] };
			let mut fTemp537: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp535 as usize] };
			let mut fTemp538: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp535, 1)) as usize] } - fTemp537;
			let mut fTemp539: F64 = 65535.0 * fTemp532;
			let mut iTemp540: i32 = (fTemp539) as i32;
			let mut iTemp541: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp540, 65535)))), 196607));
			let mut fTemp542: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp541, 3), 196607))) as usize] };
			let mut fTemp543: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp541 as usize] };
			let mut fTemp544: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp541, 1), 196607))) as usize] } - fTemp543;
			let mut iTemp545: i32 = (fTemp141 > ((if iTemp66 != 0 {fTemp543 + fTemp80 * fTemp544 + (fTemp539 - (iTemp540) as F64) * (fTemp542 - (fTemp543 + fTemp80 * (fTemp544 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp541, 4), 196607))) as usize] } - fTemp542))))} else {1.0 - (fTemp537 + fTemp80 * fTemp538 + (fTemp533 - (iTemp534) as F64) * (fTemp536 - (fTemp537 + fTemp80 * (fTemp538 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp535, 4)) as usize] } - fTemp536)))))} - fTemp531) / (1.0 - fTemp531))) as i32;
			let mut fTemp546: F64 = if iTemp545 != 0 {fTemp515} else {fTemp518};
			let mut fTemp547: F64 = if iTemp545 != 0 {fTemp518} else {fTemp516};
			let mut fTemp548: F64 = fTemp547 + fTemp546;
			let mut fTemp549: F64 = 0.5 * fTemp548;
			let mut fTemp550: F64 = 65535.0 * (1.0 - fTemp549);
			let mut iTemp551: i32 = (fTemp550) as i32;
			let mut iTemp552: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp551, 65535)))), 196607));
			let mut fTemp553: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp552, 3)) as usize] };
			let mut fTemp554: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp552 as usize] };
			let mut fTemp555: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp552, 1)) as usize] } - fTemp554;
			let mut fTemp556: F64 = 32767.5 * fTemp548;
			let mut iTemp557: i32 = (fTemp556) as i32;
			let mut iTemp558: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp557, 65535)))), 196607));
			let mut fTemp559: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp558, 3)) as usize] };
			let mut fTemp560: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp558 as usize] };
			let mut fTemp561: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp558, 1)) as usize] } - fTemp560;
			let mut fTemp562: F64 = if iTemp66 != 0 {fTemp560 + fTemp80 * fTemp561 + (fTemp556 - (iTemp557) as F64) * (fTemp559 - (fTemp560 + fTemp80 * (fTemp561 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp558, 4)) as usize] } - fTemp559))))} else {1.0 - (fTemp554 + fTemp80 * fTemp555 + (fTemp550 - (iTemp551) as F64) * (fTemp553 - (fTemp554 + fTemp80 * (fTemp555 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp552, 4)) as usize] } - fTemp553)))))};
			let mut fTemp563: F64 = fTemp85 + fTemp549;
			let mut fTemp564: F64 = 65535.0 * (1.0 - fTemp563);
			let mut iTemp565: i32 = (fTemp564) as i32;
			let mut iTemp566: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp565, 65535)))), 196607));
			let mut fTemp567: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp566, 3)) as usize] };
			let mut fTemp568: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp566 as usize] };
			let mut fTemp569: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp566, 1)) as usize] } - fTemp568;
			let mut fTemp570: F64 = 65535.0 * fTemp563;
			let mut iTemp571: i32 = (fTemp570) as i32;
			let mut iTemp572: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp571, 65535)))), 196607));
			let mut fTemp573: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp572, 3), 196607))) as usize] };
			let mut fTemp574: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp572 as usize] };
			let mut fTemp575: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp572, 1), 196607))) as usize] } - fTemp574;
			let mut iTemp576: i32 = (fTemp141 > ((if iTemp66 != 0 {fTemp574 + fTemp80 * fTemp575 + (fTemp570 - (iTemp571) as F64) * (fTemp573 - (fTemp574 + fTemp80 * (fTemp575 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp572, 4), 196607))) as usize] } - fTemp573))))} else {1.0 - (fTemp568 + fTemp80 * fTemp569 + (fTemp564 - (iTemp565) as F64) * (fTemp567 - (fTemp568 + fTemp80 * (fTemp569 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp566, 4)) as usize] } - fTemp567)))))} - fTemp562) / (1.0 - fTemp562))) as i32;
			let mut fTemp577: F64 = if iTemp576 != 0 {fTemp546} else {fTemp549};
			let mut fTemp578: F64 = if iTemp576 != 0 {fTemp549} else {fTemp547};
			let mut fTemp579: F64 = fTemp578 + fTemp577;
			let mut fTemp580: F64 = 0.5 * fTemp579;
			let mut fTemp581: F64 = 65535.0 * (1.0 - fTemp580);
			let mut iTemp582: i32 = (fTemp581) as i32;
			let mut iTemp583: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp582, 65535)))), 196607));
			let mut fTemp584: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp583, 3)) as usize] };
			let mut fTemp585: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp583 as usize] };
			let mut fTemp586: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp583, 1)) as usize] } - fTemp585;
			let mut fTemp587: F64 = 32767.5 * fTemp579;
			let mut iTemp588: i32 = (fTemp587) as i32;
			let mut iTemp589: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp588, 65535)))), 196607));
			let mut fTemp590: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp589, 3)) as usize] };
			let mut fTemp591: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp589 as usize] };
			let mut fTemp592: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp589, 1)) as usize] } - fTemp591;
			let mut fTemp593: F64 = if iTemp66 != 0 {fTemp591 + fTemp80 * fTemp592 + (fTemp587 - (iTemp588) as F64) * (fTemp590 - (fTemp591 + fTemp80 * (fTemp592 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp589, 4), 196607))) as usize] } - fTemp590))))} else {1.0 - (fTemp585 + fTemp80 * fTemp586 + (fTemp581 - (iTemp582) as F64) * (fTemp584 - (fTemp585 + fTemp80 * (fTemp586 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp583, 4), 196607))) as usize] } - fTemp584)))))};
			let mut fTemp594: F64 = fTemp85 + fTemp580;
			let mut fTemp595: F64 = 65535.0 * (1.0 - fTemp594);
			let mut iTemp596: i32 = (fTemp595) as i32;
			let mut iTemp597: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp596, 65535)))), 196607));
			let mut fTemp598: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp597, 3)) as usize] };
			let mut fTemp599: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp597 as usize] };
			let mut fTemp600: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp597, 1)) as usize] } - fTemp599;
			let mut fTemp601: F64 = 65535.0 * fTemp594;
			let mut iTemp602: i32 = (fTemp601) as i32;
			let mut iTemp603: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp602, 65535)))), 196607));
			let mut fTemp604: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp603, 3), 196607))) as usize] };
			let mut fTemp605: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp603 as usize] };
			let mut fTemp606: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp603, 1), 196607))) as usize] } - fTemp605;
			let mut iTemp607: i32 = (fTemp141 > ((if iTemp66 != 0 {fTemp605 + fTemp80 * fTemp606 + (fTemp601 - (iTemp602) as F64) * (fTemp604 - (fTemp605 + fTemp80 * (fTemp606 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp603, 4), 196607))) as usize] } - fTemp604))))} else {1.0 - (fTemp599 + fTemp80 * fTemp600 + (fTemp595 - (iTemp596) as F64) * (fTemp598 - (fTemp599 + fTemp80 * (fTemp600 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp597, 4)) as usize] } - fTemp598)))))} - fTemp593) / (1.0 - fTemp593))) as i32;
			let mut fTemp608: F64 = F64::min(1.0, F64::max(0.0, 0.5 * (if iTemp607 != 0 {fTemp580} else {fTemp578} + if iTemp607 != 0 {fTemp577} else {fTemp580})));
			self.fRec1[0] = fTemp608;
			let mut fTemp609: F64 = 65535.0 * (1.0 - fTemp608);
			let mut iTemp610: i32 = (fTemp609) as i32;
			let mut iTemp611: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp610, 65535)))), 196607));
			let mut fTemp612: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp611, 3)) as usize] };
			let mut fTemp613: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp611 as usize] };
			let mut fTemp614: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp611, 1)) as usize] } - fTemp613;
			let mut fTemp615: F64 = 65535.0 * fTemp608;
			let mut iTemp616: i32 = (fTemp615) as i32;
			let mut iTemp617: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp616, 65535)))), 196607));
			let mut fTemp618: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp617, 3)) as usize] };
			let mut fTemp619: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp617 as usize] };
			let mut fTemp620: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp617, 1)) as usize] } - fTemp619;
			let mut fTemp621: F64 = if iTemp66 != 0 {fTemp619 + fTemp80 * fTemp620 + (fTemp615 - (iTemp616) as F64) * (fTemp618 - (fTemp619 + fTemp80 * (fTemp620 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp617, 4), 196607))) as usize] } - fTemp618))))} else {1.0 - (fTemp613 + fTemp80 * fTemp614 + (fTemp609 - (iTemp610) as F64) * (fTemp612 - (fTemp613 + fTemp80 * (fTemp614 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp611, 4), 196607))) as usize] } - fTemp612)))))};
			let mut fTemp622: F64 = fTemp85 + fTemp608;
			let mut fTemp623: F64 = 65535.0 * (1.0 - fTemp622);
			let mut iTemp624: i32 = (fTemp623) as i32;
			let mut iTemp625: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp624, 65535)))), 196607));
			let mut fTemp626: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp625, 3)) as usize] };
			let mut fTemp627: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp625 as usize] };
			let mut fTemp628: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp625, 1)) as usize] } - fTemp627;
			let mut fTemp629: F64 = 65535.0 * fTemp622;
			let mut iTemp630: i32 = (fTemp629) as i32;
			let mut iTemp631: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp630, 65535)))), 196607));
			let mut fTemp632: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp631, 3), 196607))) as usize] };
			let mut fTemp633: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp631 as usize] };
			let mut fTemp634: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp631, 1), 196607))) as usize] } - fTemp633;
			let mut fTemp635: F64 = fTemp4 + if ((0.001 * fTemp84) == 0.0) as i32 != 0 {fTemp65} else {fTemp65 * (if iTemp66 != 0 {fTemp633 + fTemp80 * fTemp634 + (fTemp629 - (iTemp630) as F64) * (fTemp632 - (fTemp633 + fTemp80 * (fTemp634 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp631, 4), 196607))) as usize] } - fTemp632))))} else {1.0 - (fTemp627 + fTemp80 * fTemp628 + (fTemp623 - (iTemp624) as F64) * (fTemp626 - (fTemp627 + fTemp80 * (fTemp628 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp625, 4)) as usize] } - fTemp626)))))} - fTemp621) / (1.0 - fTemp621)};
			self.fRec2[(self.IOTA0 & 32767) as usize] = if iTemp83 != 0 {F64::min(fTemp635, fTemp4)} else {F64::max(fTemp635, fTemp4)};
			let mut fTemp636: F64 = self.fRec2[((i32::wrapping_sub(self.IOTA0, iSlow78)) & 32767) as usize];
			self.fHbargraph0 = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, fTemp636));
			self.fRec14[0] = fSlow80 + self.fConst4 * self.fRec14[1];
			*output0 = 0.5 * self.fVec0[((i32::wrapping_sub(self.IOTA0, iSlow79)) & 32767) as usize] * fTemp2 + self.fRec14[0] * self.fVec1[((i32::wrapping_sub(self.IOTA0, iSlow79)) & 32767) as usize] * fTemp636 * fTemp3;
			let mut fTemp637: F64 = self.fRec16[((i32::wrapping_sub(self.IOTA0, 1)) & 32767) as usize];
			let mut fTemp638: F64 = fTemp35 + fSlow17 * (fTemp36 - fTemp35);
			let mut iTemp639: i32 = ((fTemp638 > fSlow11) as i32) + ((fTemp638 > fSlow9) as i32);
			let mut fTemp640: F64 = fTemp638 - fSlow8;
			let mut fTemp641: F64 = F64::min(fTemp33, F64::powf(1e+01, -(fSlow18 * F64::max(0.0, if (iTemp639 == 0) as i32 != 0 {0.0} else {if (iTemp639 == 1) as i32 != 0 {fSlow12 * LambRs192k_faustpower2_f(fSlow7 + fTemp640)} else {fTemp640}}))));
			self.fVec34[(self.IOTA0 & 16383) as usize] = fTemp641;
			let mut fTemp642: F64 = F64::min(fTemp641, self.fVec34[((i32::wrapping_sub(self.IOTA0, 1)) & 16383) as usize]);
			self.fVec35[0] = fTemp642;
			let mut fTemp643: F64 = F64::min(fTemp642, self.fVec35[2]);
			self.fVec36[0] = fTemp643;
			let mut fTemp644: F64 = F64::min(fTemp643, self.fVec36[4]);
			self.fVec37[0] = fTemp644;
			let mut fTemp645: F64 = F64::min(fTemp644, self.fVec37[8]);
			self.fVec38[(self.IOTA0 & 31) as usize] = fTemp645;
			let mut fTemp646: F64 = F64::min(fTemp645, self.fVec38[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize]);
			self.fVec39[(self.IOTA0 & 63) as usize] = fTemp646;
			let mut fTemp647: F64 = F64::min(fTemp646, self.fVec39[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize]);
			self.fVec40[(self.IOTA0 & 127) as usize] = fTemp647;
			let mut fTemp648: F64 = F64::min(fTemp647, self.fVec40[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize]);
			self.fVec41[(self.IOTA0 & 255) as usize] = fTemp648;
			let mut fTemp649: F64 = F64::min(fTemp648, self.fVec41[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize]);
			self.fVec42[(self.IOTA0 & 511) as usize] = fTemp649;
			let mut fTemp650: F64 = F64::min(fTemp649, self.fVec42[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize]);
			self.fVec43[(self.IOTA0 & 1023) as usize] = fTemp650;
			let mut fTemp651: F64 = F64::min(fTemp650, self.fVec43[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize]);
			self.fVec44[(self.IOTA0 & 2047) as usize] = fTemp651;
			let mut fTemp652: F64 = F64::min(fTemp651, self.fVec44[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			self.fVec45[(self.IOTA0 & 4095) as usize] = fTemp652;
			let mut fTemp653: F64 = F64::min(fTemp652, self.fVec45[((i32::wrapping_sub(self.IOTA0, 2048)) & 4095) as usize]);
			self.fVec46[(self.IOTA0 & 8191) as usize] = fTemp653;
			self.fVec47[(self.IOTA0 & 16383) as usize] = F64::min(fTemp653, self.fVec46[((i32::wrapping_sub(self.IOTA0, 4096)) & 8191) as usize]);
			self.fRec17[0] = F64::max(F64::min(self.fRec17[1], self.fVec34[((i32::wrapping_sub(self.IOTA0, iSlow21)) & 16383) as usize]), F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow23 != 0 {fTemp641} else {1.7976931348623157e+308}, if iSlow24 != 0 {self.fVec35[iSlow23 as usize]} else {1.7976931348623157e+308}), if iSlow25 != 0 {self.fVec36[iSlow26 as usize]} else {1.7976931348623157e+308}), if iSlow27 != 0 {self.fVec37[iSlow28 as usize]} else {1.7976931348623157e+308}), if iSlow29 != 0 {self.fVec38[((i32::wrapping_sub(self.IOTA0, iSlow30)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow31 != 0 {self.fVec39[((i32::wrapping_sub(self.IOTA0, iSlow32)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow33 != 0 {self.fVec40[((i32::wrapping_sub(self.IOTA0, iSlow34)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow35 != 0 {self.fVec41[((i32::wrapping_sub(self.IOTA0, iSlow36)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow37 != 0 {self.fVec42[((i32::wrapping_sub(self.IOTA0, iSlow38)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow39 != 0 {self.fVec43[((i32::wrapping_sub(self.IOTA0, iSlow40)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow41 != 0 {self.fVec44[((i32::wrapping_sub(self.IOTA0, iSlow42)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow43 != 0 {self.fVec45[((i32::wrapping_sub(self.IOTA0, iSlow44)) & 4095) as usize]} else {1.7976931348623157e+308}), if iSlow45 != 0 {self.fVec46[((i32::wrapping_sub(self.IOTA0, iSlow46)) & 8191) as usize]} else {1.7976931348623157e+308}), if iSlow47 != 0 {self.fVec47[((i32::wrapping_sub(self.IOTA0, iSlow48)) & 16383) as usize]} else {1.7976931348623157e+308}));
			let mut fTemp654: F64 = F64::min(self.fRec17[0], self.fRec17[1]);
			self.fVec48[0] = fTemp654;
			let mut fTemp655: F64 = F64::min(fTemp654, self.fVec48[2]);
			self.fVec49[0] = fTemp655;
			let mut fTemp656: F64 = F64::min(fTemp655, self.fVec49[4]);
			self.fVec50[0] = fTemp656;
			let mut fTemp657: F64 = F64::min(fTemp656, self.fVec50[8]);
			self.fVec51[(self.IOTA0 & 31) as usize] = fTemp657;
			let mut fTemp658: F64 = F64::min(fTemp657, self.fVec51[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize]);
			self.fVec52[(self.IOTA0 & 63) as usize] = fTemp658;
			let mut fTemp659: F64 = F64::min(fTemp658, self.fVec52[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize]);
			self.fVec53[(self.IOTA0 & 127) as usize] = fTemp659;
			let mut fTemp660: F64 = F64::min(fTemp659, self.fVec53[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize]);
			self.fVec54[(self.IOTA0 & 255) as usize] = fTemp660;
			let mut fTemp661: F64 = F64::min(fTemp660, self.fVec54[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize]);
			self.fVec55[(self.IOTA0 & 511) as usize] = fTemp661;
			let mut fTemp662: F64 = F64::min(fTemp661, self.fVec55[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize]);
			self.fVec56[(self.IOTA0 & 1023) as usize] = fTemp662;
			let mut fTemp663: F64 = F64::min(fTemp662, self.fVec56[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize]);
			self.fVec57[(self.IOTA0 & 2047) as usize] = fTemp663;
			let mut fTemp664: F64 = F64::min(fTemp663, self.fVec57[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			self.fVec58[(self.IOTA0 & 4095) as usize] = fTemp664;
			let mut fTemp665: F64 = F64::min(fTemp664, self.fVec58[((i32::wrapping_sub(self.IOTA0, 2048)) & 4095) as usize]);
			self.fVec59[(self.IOTA0 & 8191) as usize] = fTemp665;
			self.fVec60[(self.IOTA0 & 16383) as usize] = F64::min(fTemp665, self.fVec59[((i32::wrapping_sub(self.IOTA0, 4096)) & 8191) as usize]);
			let mut fTemp666: F64 = F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow4 != 0 {self.fRec17[0]} else {1.7976931348623157e+308}, if iSlow49 != 0 {self.fVec48[iSlow4 as usize]} else {1.7976931348623157e+308}), if iSlow50 != 0 {self.fVec49[iSlow51 as usize]} else {1.7976931348623157e+308}), if iSlow52 != 0 {self.fVec50[iSlow53 as usize]} else {1.7976931348623157e+308}), if iSlow54 != 0 {self.fVec51[((i32::wrapping_sub(self.IOTA0, iSlow55)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow56 != 0 {self.fVec52[((i32::wrapping_sub(self.IOTA0, iSlow57)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow58 != 0 {self.fVec53[((i32::wrapping_sub(self.IOTA0, iSlow59)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow60 != 0 {self.fVec54[((i32::wrapping_sub(self.IOTA0, iSlow61)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow62 != 0 {self.fVec55[((i32::wrapping_sub(self.IOTA0, iSlow63)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow64 != 0 {self.fVec56[((i32::wrapping_sub(self.IOTA0, iSlow65)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow66 != 0 {self.fVec57[((i32::wrapping_sub(self.IOTA0, iSlow67)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow68 != 0 {self.fVec58[((i32::wrapping_sub(self.IOTA0, iSlow69)) & 4095) as usize]} else {1.7976931348623157e+308}), if iSlow70 != 0 {self.fVec59[((i32::wrapping_sub(self.IOTA0, iSlow71)) & 8191) as usize]} else {1.7976931348623157e+308}), if iSlow72 != 0 {self.fVec60[((i32::wrapping_sub(self.IOTA0, iSlow73)) & 16383) as usize]} else {1.7976931348623157e+308}) - fTemp637;
			self.fVec61[0] = fTemp666;
			let mut iTemp667: i32 = (fTemp666 > 0.0) as i32;
			let mut fTemp668: F64 = if iTemp667 != 0 {fSlow75} else {fSlow74};
			self.fVec62[0] = fTemp668;
			let mut fTemp669: F64 = 2.0 * fTemp668;
			let mut iTemp670: i32 = (fTemp669) as i32;
			let mut iTemp671: i32 = std::cmp::max(0, std::cmp::min(iTemp670, 2));
			let mut iTemp672: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, 98301), 196607));
			let mut fTemp673: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp672, 3)) as usize] };
			let mut fTemp674: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp672 as usize] };
			let mut fTemp675: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp672, 1)) as usize] } - fTemp674;
			let mut fTemp676: F64 = fTemp669 - (iTemp670) as F64;
			let mut fTemp677: F64 = fTemp674 + fTemp676 * fTemp675 + 0.5 * (fTemp673 - (fTemp674 + fTemp676 * (fTemp675 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp672, 4)) as usize] } - fTemp673))));
			let mut fTemp678: F64 = if iTemp667 != 0 {fTemp677} else {1.0 - fTemp677};
			let mut iTemp679: i32 = (fTemp666 < 0.0) as i32;
			let mut fTemp680: F64 = fSlow1 * (iTemp679) as F64 + fSlow13 * (iTemp667) as F64;
			self.fVec63[0] = fTemp680;
			let mut fTemp681: F64 = self.fConst10 / fTemp680;
			let mut fTemp682: F64 = fTemp681 + 0.5;
			let mut fTemp683: F64 = 65535.0 * (1.0 - fTemp682);
			let mut iTemp684: i32 = (fTemp683) as i32;
			let mut iTemp685: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp684, 65535)))), 196607));
			let mut fTemp686: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp685, 3)) as usize] };
			let mut fTemp687: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp685 as usize] };
			let mut fTemp688: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp685, 1)) as usize] } - fTemp687;
			let mut fTemp689: F64 = 65535.0 * fTemp682;
			let mut iTemp690: i32 = (fTemp689) as i32;
			let mut iTemp691: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp690, 65535)))), 196607));
			let mut fTemp692: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp691, 3), 196607))) as usize] };
			let mut fTemp693: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp691 as usize] };
			let mut fTemp694: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp691, 1), 196607))) as usize] } - fTemp693;
			let mut fTemp695: F64 = 2.0 * self.fVec62[1];
			let mut iTemp696: i32 = (fTemp695) as i32;
			let mut iTemp697: i32 = std::cmp::max(0, std::cmp::min(iTemp696, 2));
			let mut fTemp698: F64 = 65535.0 * (1.0 - self.fRec15[1]);
			let mut iTemp699: i32 = (fTemp698) as i32;
			let mut iTemp700: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp699, 65535))), iTemp697), 196607));
			let mut fTemp701: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp700, 3), 196607))) as usize] };
			let mut fTemp702: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp700 as usize] };
			let mut fTemp703: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp700, 1), 196607))) as usize] } - fTemp702;
			let mut fTemp704: F64 = fTemp695 - (iTemp696) as F64;
			let mut fTemp705: F64 = 65535.0 * self.fRec15[1];
			let mut iTemp706: i32 = (fTemp705) as i32;
			let mut iTemp707: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp697, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp706, 65535)))), 196607));
			let mut fTemp708: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp707, 3), 196607))) as usize] };
			let mut fTemp709: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp707 as usize] };
			let mut fTemp710: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp707, 1), 196607))) as usize] } - fTemp709;
			let mut fTemp711: F64 = self.fRec15[1] + fTemp681;
			let mut fTemp712: F64 = 65535.0 * (1.0 - fTemp711);
			let mut iTemp713: i32 = (fTemp712) as i32;
			let mut iTemp714: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp713, 65535)))), 196607));
			let mut fTemp715: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp714, 3)) as usize] };
			let mut fTemp716: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp714 as usize] };
			let mut fTemp717: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp714, 1)) as usize] } - fTemp716;
			let mut fTemp718: F64 = 65535.0 * fTemp711;
			let mut iTemp719: i32 = (fTemp718) as i32;
			let mut iTemp720: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp719, 65535)))), 196607));
			let mut fTemp721: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp720, 3), 196607))) as usize] };
			let mut fTemp722: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp720 as usize] };
			let mut fTemp723: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp720, 1), 196607))) as usize] } - fTemp722;
			let mut fTemp724: F64 = self.fRec15[1] + self.fConst10 * (1.0 / fTemp680 + 1.0 / self.fVec63[1]);
			let mut fTemp725: F64 = 65535.0 * (1.0 - fTemp724);
			let mut iTemp726: i32 = (fTemp725) as i32;
			let mut iTemp727: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp726, 65535))), iTemp671), 196607));
			let mut fTemp728: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp727, 3)) as usize] };
			let mut fTemp729: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp727 as usize] };
			let mut fTemp730: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp727, 1)) as usize] } - fTemp729;
			let mut fTemp731: F64 = 65535.0 * fTemp724;
			let mut iTemp732: i32 = (fTemp731) as i32;
			let mut iTemp733: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp732, 65535)))), 196607));
			let mut fTemp734: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp733, 3), 196607))) as usize] };
			let mut fTemp735: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp733 as usize] };
			let mut fTemp736: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp733, 1), 196607))) as usize] } - fTemp735;
			let mut fTemp737: F64 = (if iTemp667 != 0 {fTemp735 + fTemp676 * fTemp736 + (fTemp731 - (iTemp732) as F64) * (fTemp734 - (fTemp735 + fTemp676 * (fTemp736 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp733, 4), 196607))) as usize] } - fTemp734))))} else {1.0 - (fTemp729 + fTemp676 * fTemp730 + (fTemp725 - (iTemp726) as F64) * (fTemp728 - (fTemp729 + fTemp676 * (fTemp730 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp727, 4)) as usize] } - fTemp728)))))} - if iTemp667 != 0 {fTemp722 + fTemp676 * fTemp723 + (fTemp718 - (iTemp719) as F64) * (fTemp721 - (fTemp722 + fTemp676 * (fTemp723 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp720, 4), 196607))) as usize] } - fTemp721))))} else {1.0 - (fTemp716 + fTemp676 * fTemp717 + (fTemp712 - (iTemp713) as F64) * (fTemp715 - (fTemp716 + fTemp676 * (fTemp717 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp714, 4), 196607))) as usize] } - fTemp715)))))}) * self.fVec61[1] / (fTemp666 * (1.0 - if iTemp667 != 0 {fTemp709 + fTemp704 * fTemp710 + (fTemp705 - (iTemp706) as F64) * (fTemp708 - (fTemp709 + fTemp704 * (fTemp710 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp707, 4), 196607))) as usize] } - fTemp708))))} else {1.0 - (fTemp702 + fTemp704 * fTemp703 + (fTemp698 - (iTemp699) as F64) * (fTemp701 - (fTemp702 + fTemp704 * (fTemp703 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp700, 4), 196607))) as usize] } - fTemp701)))))}));
			let mut iTemp738: i32 = (fTemp737 > ((if iTemp667 != 0 {fTemp693 + fTemp676 * fTemp694 + (fTemp689 - (iTemp690) as F64) * (fTemp692 - (fTemp693 + fTemp676 * (fTemp694 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp691, 4), 196607))) as usize] } - fTemp692))))} else {1.0 - (fTemp687 + fTemp676 * fTemp688 + (fTemp683 - (iTemp684) as F64) * (fTemp686 - (fTemp687 + fTemp676 * (fTemp688 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp685, 4)) as usize] } - fTemp686)))))} - fTemp678) / (1.0 - fTemp678))) as i32;
			let mut fTemp739: F64 = if iTemp738 != 0 {1.0} else {0.5};
			let mut fTemp740: F64 = if iTemp738 != 0 {0.5} else {0.0};
			let mut fTemp741: F64 = fTemp740 + fTemp739;
			let mut fTemp742: F64 = 0.5 * fTemp741;
			let mut fTemp743: F64 = 65535.0 * (1.0 - fTemp742);
			let mut iTemp744: i32 = (fTemp743) as i32;
			let mut iTemp745: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp744, 65535)))), 196607));
			let mut fTemp746: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp745, 3)) as usize] };
			let mut fTemp747: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp745 as usize] };
			let mut fTemp748: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp745, 1)) as usize] } - fTemp747;
			let mut fTemp749: F64 = 32767.5 * fTemp741;
			let mut iTemp750: i32 = (fTemp749) as i32;
			let mut iTemp751: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp750, 65535)))), 196607));
			let mut fTemp752: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp751, 3)) as usize] };
			let mut fTemp753: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp751 as usize] };
			let mut fTemp754: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp751, 1)) as usize] } - fTemp753;
			let mut fTemp755: F64 = if iTemp667 != 0 {fTemp753 + fTemp676 * fTemp754 + (fTemp749 - (iTemp750) as F64) * (fTemp752 - (fTemp753 + fTemp676 * (fTemp754 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp751, 4)) as usize] } - fTemp752))))} else {1.0 - (fTemp747 + fTemp676 * fTemp748 + (fTemp743 - (iTemp744) as F64) * (fTemp746 - (fTemp747 + fTemp676 * (fTemp748 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp745, 4)) as usize] } - fTemp746)))))};
			let mut fTemp756: F64 = fTemp681 + fTemp742;
			let mut fTemp757: F64 = 65535.0 * (1.0 - fTemp756);
			let mut iTemp758: i32 = (fTemp757) as i32;
			let mut iTemp759: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp758, 65535)))), 196607));
			let mut fTemp760: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp759, 3)) as usize] };
			let mut fTemp761: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp759 as usize] };
			let mut fTemp762: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp759, 1)) as usize] } - fTemp761;
			let mut fTemp763: F64 = 65535.0 * fTemp756;
			let mut iTemp764: i32 = (fTemp763) as i32;
			let mut iTemp765: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp764, 65535)))), 196607));
			let mut fTemp766: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp765, 3), 196607))) as usize] };
			let mut fTemp767: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp765 as usize] };
			let mut fTemp768: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp765, 1), 196607))) as usize] } - fTemp767;
			let mut iTemp769: i32 = (fTemp737 > ((if iTemp667 != 0 {fTemp767 + fTemp676 * fTemp768 + (fTemp763 - (iTemp764) as F64) * (fTemp766 - (fTemp767 + fTemp676 * (fTemp768 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp765, 4), 196607))) as usize] } - fTemp766))))} else {1.0 - (fTemp761 + fTemp676 * fTemp762 + (fTemp757 - (iTemp758) as F64) * (fTemp760 - (fTemp761 + fTemp676 * (fTemp762 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp759, 4)) as usize] } - fTemp760)))))} - fTemp755) / (1.0 - fTemp755))) as i32;
			let mut fTemp770: F64 = if iTemp769 != 0 {fTemp739} else {fTemp742};
			let mut fTemp771: F64 = if iTemp769 != 0 {fTemp742} else {fTemp740};
			let mut fTemp772: F64 = fTemp771 + fTemp770;
			let mut fTemp773: F64 = 0.5 * fTemp772;
			let mut fTemp774: F64 = 65535.0 * (1.0 - fTemp773);
			let mut iTemp775: i32 = (fTemp774) as i32;
			let mut iTemp776: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp775, 65535)))), 196607));
			let mut fTemp777: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp776, 3)) as usize] };
			let mut fTemp778: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp776 as usize] };
			let mut fTemp779: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp776, 1)) as usize] } - fTemp778;
			let mut fTemp780: F64 = 32767.5 * fTemp772;
			let mut iTemp781: i32 = (fTemp780) as i32;
			let mut iTemp782: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp781, 65535)))), 196607));
			let mut fTemp783: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp782, 3)) as usize] };
			let mut fTemp784: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp782 as usize] };
			let mut fTemp785: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp782, 1)) as usize] } - fTemp784;
			let mut fTemp786: F64 = if iTemp667 != 0 {fTemp784 + fTemp676 * fTemp785 + (fTemp780 - (iTemp781) as F64) * (fTemp783 - (fTemp784 + fTemp676 * (fTemp785 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp782, 4)) as usize] } - fTemp783))))} else {1.0 - (fTemp778 + fTemp676 * fTemp779 + (fTemp774 - (iTemp775) as F64) * (fTemp777 - (fTemp778 + fTemp676 * (fTemp779 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp776, 4)) as usize] } - fTemp777)))))};
			let mut fTemp787: F64 = fTemp681 + fTemp773;
			let mut fTemp788: F64 = 65535.0 * (1.0 - fTemp787);
			let mut iTemp789: i32 = (fTemp788) as i32;
			let mut iTemp790: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp789, 65535)))), 196607));
			let mut fTemp791: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp790, 3)) as usize] };
			let mut fTemp792: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp790 as usize] };
			let mut fTemp793: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp790, 1)) as usize] } - fTemp792;
			let mut fTemp794: F64 = 65535.0 * fTemp787;
			let mut iTemp795: i32 = (fTemp794) as i32;
			let mut iTemp796: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp795, 65535)))), 196607));
			let mut fTemp797: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp796, 3), 196607))) as usize] };
			let mut fTemp798: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp796 as usize] };
			let mut fTemp799: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp796, 1), 196607))) as usize] } - fTemp798;
			let mut iTemp800: i32 = (fTemp737 > ((if iTemp667 != 0 {fTemp798 + fTemp676 * fTemp799 + (fTemp794 - (iTemp795) as F64) * (fTemp797 - (fTemp798 + fTemp676 * (fTemp799 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp796, 4), 196607))) as usize] } - fTemp797))))} else {1.0 - (fTemp792 + fTemp676 * fTemp793 + (fTemp788 - (iTemp789) as F64) * (fTemp791 - (fTemp792 + fTemp676 * (fTemp793 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp790, 4)) as usize] } - fTemp791)))))} - fTemp786) / (1.0 - fTemp786))) as i32;
			let mut fTemp801: F64 = if iTemp800 != 0 {fTemp770} else {fTemp773};
			let mut fTemp802: F64 = if iTemp800 != 0 {fTemp773} else {fTemp771};
			let mut fTemp803: F64 = fTemp802 + fTemp801;
			let mut fTemp804: F64 = 0.5 * fTemp803;
			let mut fTemp805: F64 = 65535.0 * (1.0 - fTemp804);
			let mut iTemp806: i32 = (fTemp805) as i32;
			let mut iTemp807: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp806, 65535)))), 196607));
			let mut fTemp808: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp807, 3)) as usize] };
			let mut fTemp809: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp807 as usize] };
			let mut fTemp810: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp807, 1)) as usize] } - fTemp809;
			let mut fTemp811: F64 = 32767.5 * fTemp803;
			let mut iTemp812: i32 = (fTemp811) as i32;
			let mut iTemp813: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp812, 65535)))), 196607));
			let mut fTemp814: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp813, 3)) as usize] };
			let mut fTemp815: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp813 as usize] };
			let mut fTemp816: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp813, 1)) as usize] } - fTemp815;
			let mut fTemp817: F64 = if iTemp667 != 0 {fTemp815 + fTemp676 * fTemp816 + (fTemp811 - (iTemp812) as F64) * (fTemp814 - (fTemp815 + fTemp676 * (fTemp816 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp813, 4)) as usize] } - fTemp814))))} else {1.0 - (fTemp809 + fTemp676 * fTemp810 + (fTemp805 - (iTemp806) as F64) * (fTemp808 - (fTemp809 + fTemp676 * (fTemp810 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp807, 4)) as usize] } - fTemp808)))))};
			let mut fTemp818: F64 = fTemp681 + fTemp804;
			let mut fTemp819: F64 = 65535.0 * (1.0 - fTemp818);
			let mut iTemp820: i32 = (fTemp819) as i32;
			let mut iTemp821: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp820, 65535)))), 196607));
			let mut fTemp822: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp821, 3)) as usize] };
			let mut fTemp823: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp821 as usize] };
			let mut fTemp824: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp821, 1)) as usize] } - fTemp823;
			let mut fTemp825: F64 = 65535.0 * fTemp818;
			let mut iTemp826: i32 = (fTemp825) as i32;
			let mut iTemp827: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp826, 65535)))), 196607));
			let mut fTemp828: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp827, 3), 196607))) as usize] };
			let mut fTemp829: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp827 as usize] };
			let mut fTemp830: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp827, 1), 196607))) as usize] } - fTemp829;
			let mut iTemp831: i32 = (fTemp737 > ((if iTemp667 != 0 {fTemp829 + fTemp676 * fTemp830 + (fTemp825 - (iTemp826) as F64) * (fTemp828 - (fTemp829 + fTemp676 * (fTemp830 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp827, 4), 196607))) as usize] } - fTemp828))))} else {1.0 - (fTemp823 + fTemp676 * fTemp824 + (fTemp819 - (iTemp820) as F64) * (fTemp822 - (fTemp823 + fTemp676 * (fTemp824 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp821, 4)) as usize] } - fTemp822)))))} - fTemp817) / (1.0 - fTemp817))) as i32;
			let mut fTemp832: F64 = if iTemp831 != 0 {fTemp801} else {fTemp804};
			let mut fTemp833: F64 = if iTemp831 != 0 {fTemp804} else {fTemp802};
			let mut fTemp834: F64 = fTemp833 + fTemp832;
			let mut fTemp835: F64 = 0.5 * fTemp834;
			let mut fTemp836: F64 = 65535.0 * (1.0 - fTemp835);
			let mut iTemp837: i32 = (fTemp836) as i32;
			let mut iTemp838: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp837, 65535)))), 196607));
			let mut fTemp839: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp838, 3)) as usize] };
			let mut fTemp840: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp838 as usize] };
			let mut fTemp841: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp838, 1)) as usize] } - fTemp840;
			let mut fTemp842: F64 = 32767.5 * fTemp834;
			let mut iTemp843: i32 = (fTemp842) as i32;
			let mut iTemp844: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp843, 65535)))), 196607));
			let mut fTemp845: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp844, 3)) as usize] };
			let mut fTemp846: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp844 as usize] };
			let mut fTemp847: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp844, 1)) as usize] } - fTemp846;
			let mut fTemp848: F64 = if iTemp667 != 0 {fTemp846 + fTemp676 * fTemp847 + (fTemp842 - (iTemp843) as F64) * (fTemp845 - (fTemp846 + fTemp676 * (fTemp847 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp844, 4)) as usize] } - fTemp845))))} else {1.0 - (fTemp840 + fTemp676 * fTemp841 + (fTemp836 - (iTemp837) as F64) * (fTemp839 - (fTemp840 + fTemp676 * (fTemp841 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp838, 4)) as usize] } - fTemp839)))))};
			let mut fTemp849: F64 = fTemp681 + fTemp835;
			let mut fTemp850: F64 = 65535.0 * (1.0 - fTemp849);
			let mut iTemp851: i32 = (fTemp850) as i32;
			let mut iTemp852: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp851, 65535)))), 196607));
			let mut fTemp853: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp852, 3)) as usize] };
			let mut fTemp854: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp852 as usize] };
			let mut fTemp855: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp852, 1)) as usize] } - fTemp854;
			let mut fTemp856: F64 = 65535.0 * fTemp849;
			let mut iTemp857: i32 = (fTemp856) as i32;
			let mut iTemp858: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp857, 65535)))), 196607));
			let mut fTemp859: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp858, 3), 196607))) as usize] };
			let mut fTemp860: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp858 as usize] };
			let mut fTemp861: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp858, 1), 196607))) as usize] } - fTemp860;
			let mut iTemp862: i32 = (fTemp737 > ((if iTemp667 != 0 {fTemp860 + fTemp676 * fTemp861 + (fTemp856 - (iTemp857) as F64) * (fTemp859 - (fTemp860 + fTemp676 * (fTemp861 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp858, 4), 196607))) as usize] } - fTemp859))))} else {1.0 - (fTemp854 + fTemp676 * fTemp855 + (fTemp850 - (iTemp851) as F64) * (fTemp853 - (fTemp854 + fTemp676 * (fTemp855 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp852, 4)) as usize] } - fTemp853)))))} - fTemp848) / (1.0 - fTemp848))) as i32;
			let mut fTemp863: F64 = if iTemp862 != 0 {fTemp832} else {fTemp835};
			let mut fTemp864: F64 = if iTemp862 != 0 {fTemp835} else {fTemp833};
			let mut fTemp865: F64 = fTemp864 + fTemp863;
			let mut fTemp866: F64 = 0.5 * fTemp865;
			let mut fTemp867: F64 = 65535.0 * (1.0 - fTemp866);
			let mut iTemp868: i32 = (fTemp867) as i32;
			let mut iTemp869: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp868, 65535)))), 196607));
			let mut fTemp870: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp869, 3)) as usize] };
			let mut fTemp871: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp869 as usize] };
			let mut fTemp872: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp869, 1)) as usize] } - fTemp871;
			let mut fTemp873: F64 = 32767.5 * fTemp865;
			let mut iTemp874: i32 = (fTemp873) as i32;
			let mut iTemp875: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp874, 65535)))), 196607));
			let mut fTemp876: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp875, 3)) as usize] };
			let mut fTemp877: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp875 as usize] };
			let mut fTemp878: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp875, 1)) as usize] } - fTemp877;
			let mut fTemp879: F64 = if iTemp667 != 0 {fTemp877 + fTemp676 * fTemp878 + (fTemp873 - (iTemp874) as F64) * (fTemp876 - (fTemp877 + fTemp676 * (fTemp878 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp875, 4)) as usize] } - fTemp876))))} else {1.0 - (fTemp871 + fTemp676 * fTemp872 + (fTemp867 - (iTemp868) as F64) * (fTemp870 - (fTemp871 + fTemp676 * (fTemp872 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp869, 4)) as usize] } - fTemp870)))))};
			let mut fTemp880: F64 = fTemp681 + fTemp866;
			let mut fTemp881: F64 = 65535.0 * (1.0 - fTemp880);
			let mut iTemp882: i32 = (fTemp881) as i32;
			let mut iTemp883: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp882, 65535)))), 196607));
			let mut fTemp884: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp883, 3)) as usize] };
			let mut fTemp885: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp883 as usize] };
			let mut fTemp886: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp883, 1)) as usize] } - fTemp885;
			let mut fTemp887: F64 = 65535.0 * fTemp880;
			let mut iTemp888: i32 = (fTemp887) as i32;
			let mut iTemp889: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp888, 65535)))), 196607));
			let mut fTemp890: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp889, 3), 196607))) as usize] };
			let mut fTemp891: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp889 as usize] };
			let mut fTemp892: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp889, 1), 196607))) as usize] } - fTemp891;
			let mut iTemp893: i32 = (fTemp737 > ((if iTemp667 != 0 {fTemp891 + fTemp676 * fTemp892 + (fTemp887 - (iTemp888) as F64) * (fTemp890 - (fTemp891 + fTemp676 * (fTemp892 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp889, 4), 196607))) as usize] } - fTemp890))))} else {1.0 - (fTemp885 + fTemp676 * fTemp886 + (fTemp881 - (iTemp882) as F64) * (fTemp884 - (fTemp885 + fTemp676 * (fTemp886 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp883, 4)) as usize] } - fTemp884)))))} - fTemp879) / (1.0 - fTemp879))) as i32;
			let mut fTemp894: F64 = if iTemp893 != 0 {fTemp863} else {fTemp866};
			let mut fTemp895: F64 = if iTemp893 != 0 {fTemp866} else {fTemp864};
			let mut fTemp896: F64 = fTemp895 + fTemp894;
			let mut fTemp897: F64 = 0.5 * fTemp896;
			let mut fTemp898: F64 = 65535.0 * (1.0 - fTemp897);
			let mut iTemp899: i32 = (fTemp898) as i32;
			let mut iTemp900: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp899, 65535)))), 196607));
			let mut fTemp901: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp900, 3)) as usize] };
			let mut fTemp902: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp900 as usize] };
			let mut fTemp903: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp900, 1)) as usize] } - fTemp902;
			let mut fTemp904: F64 = 32767.5 * fTemp896;
			let mut iTemp905: i32 = (fTemp904) as i32;
			let mut iTemp906: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp905, 65535)))), 196607));
			let mut fTemp907: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp906, 3)) as usize] };
			let mut fTemp908: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp906 as usize] };
			let mut fTemp909: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp906, 1)) as usize] } - fTemp908;
			let mut fTemp910: F64 = if iTemp667 != 0 {fTemp908 + fTemp676 * fTemp909 + (fTemp904 - (iTemp905) as F64) * (fTemp907 - (fTemp908 + fTemp676 * (fTemp909 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp906, 4)) as usize] } - fTemp907))))} else {1.0 - (fTemp902 + fTemp676 * fTemp903 + (fTemp898 - (iTemp899) as F64) * (fTemp901 - (fTemp902 + fTemp676 * (fTemp903 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp900, 4)) as usize] } - fTemp901)))))};
			let mut fTemp911: F64 = fTemp681 + fTemp897;
			let mut fTemp912: F64 = 65535.0 * (1.0 - fTemp911);
			let mut iTemp913: i32 = (fTemp912) as i32;
			let mut iTemp914: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp913, 65535)))), 196607));
			let mut fTemp915: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp914, 3)) as usize] };
			let mut fTemp916: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp914 as usize] };
			let mut fTemp917: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp914, 1)) as usize] } - fTemp916;
			let mut fTemp918: F64 = 65535.0 * fTemp911;
			let mut iTemp919: i32 = (fTemp918) as i32;
			let mut iTemp920: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp919, 65535)))), 196607));
			let mut fTemp921: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp920, 3), 196607))) as usize] };
			let mut fTemp922: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp920 as usize] };
			let mut fTemp923: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp920, 1), 196607))) as usize] } - fTemp922;
			let mut iTemp924: i32 = (fTemp737 > ((if iTemp667 != 0 {fTemp922 + fTemp676 * fTemp923 + (fTemp918 - (iTemp919) as F64) * (fTemp921 - (fTemp922 + fTemp676 * (fTemp923 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp920, 4), 196607))) as usize] } - fTemp921))))} else {1.0 - (fTemp916 + fTemp676 * fTemp917 + (fTemp912 - (iTemp913) as F64) * (fTemp915 - (fTemp916 + fTemp676 * (fTemp917 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp914, 4)) as usize] } - fTemp915)))))} - fTemp910) / (1.0 - fTemp910))) as i32;
			let mut fTemp925: F64 = if iTemp924 != 0 {fTemp894} else {fTemp897};
			let mut fTemp926: F64 = if iTemp924 != 0 {fTemp897} else {fTemp895};
			let mut fTemp927: F64 = fTemp926 + fTemp925;
			let mut fTemp928: F64 = 0.5 * fTemp927;
			let mut fTemp929: F64 = 65535.0 * (1.0 - fTemp928);
			let mut iTemp930: i32 = (fTemp929) as i32;
			let mut iTemp931: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp930, 65535)))), 196607));
			let mut fTemp932: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp931, 3)) as usize] };
			let mut fTemp933: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp931 as usize] };
			let mut fTemp934: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp931, 1)) as usize] } - fTemp933;
			let mut fTemp935: F64 = 32767.5 * fTemp927;
			let mut iTemp936: i32 = (fTemp935) as i32;
			let mut iTemp937: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp936, 65535)))), 196607));
			let mut fTemp938: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp937, 3)) as usize] };
			let mut fTemp939: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp937 as usize] };
			let mut fTemp940: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp937, 1)) as usize] } - fTemp939;
			let mut fTemp941: F64 = if iTemp667 != 0 {fTemp939 + fTemp676 * fTemp940 + (fTemp935 - (iTemp936) as F64) * (fTemp938 - (fTemp939 + fTemp676 * (fTemp940 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp937, 4)) as usize] } - fTemp938))))} else {1.0 - (fTemp933 + fTemp676 * fTemp934 + (fTemp929 - (iTemp930) as F64) * (fTemp932 - (fTemp933 + fTemp676 * (fTemp934 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp931, 4)) as usize] } - fTemp932)))))};
			let mut fTemp942: F64 = fTemp681 + fTemp928;
			let mut fTemp943: F64 = 65535.0 * (1.0 - fTemp942);
			let mut iTemp944: i32 = (fTemp943) as i32;
			let mut iTemp945: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp944, 65535)))), 196607));
			let mut fTemp946: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp945, 3)) as usize] };
			let mut fTemp947: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp945 as usize] };
			let mut fTemp948: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp945, 1)) as usize] } - fTemp947;
			let mut fTemp949: F64 = 65535.0 * fTemp942;
			let mut iTemp950: i32 = (fTemp949) as i32;
			let mut iTemp951: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp950, 65535)))), 196607));
			let mut fTemp952: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp951, 3), 196607))) as usize] };
			let mut fTemp953: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp951 as usize] };
			let mut fTemp954: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp951, 1), 196607))) as usize] } - fTemp953;
			let mut iTemp955: i32 = (fTemp737 > ((if iTemp667 != 0 {fTemp953 + fTemp676 * fTemp954 + (fTemp949 - (iTemp950) as F64) * (fTemp952 - (fTemp953 + fTemp676 * (fTemp954 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp951, 4), 196607))) as usize] } - fTemp952))))} else {1.0 - (fTemp947 + fTemp676 * fTemp948 + (fTemp943 - (iTemp944) as F64) * (fTemp946 - (fTemp947 + fTemp676 * (fTemp948 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp945, 4)) as usize] } - fTemp946)))))} - fTemp941) / (1.0 - fTemp941))) as i32;
			let mut fTemp956: F64 = if iTemp955 != 0 {fTemp925} else {fTemp928};
			let mut fTemp957: F64 = if iTemp955 != 0 {fTemp928} else {fTemp926};
			let mut fTemp958: F64 = fTemp957 + fTemp956;
			let mut fTemp959: F64 = 0.5 * fTemp958;
			let mut fTemp960: F64 = 65535.0 * (1.0 - fTemp959);
			let mut iTemp961: i32 = (fTemp960) as i32;
			let mut iTemp962: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp961, 65535)))), 196607));
			let mut fTemp963: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp962, 3)) as usize] };
			let mut fTemp964: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp962 as usize] };
			let mut fTemp965: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp962, 1)) as usize] } - fTemp964;
			let mut fTemp966: F64 = 32767.5 * fTemp958;
			let mut iTemp967: i32 = (fTemp966) as i32;
			let mut iTemp968: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp967, 65535)))), 196607));
			let mut fTemp969: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp968, 3)) as usize] };
			let mut fTemp970: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp968 as usize] };
			let mut fTemp971: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp968, 1)) as usize] } - fTemp970;
			let mut fTemp972: F64 = if iTemp667 != 0 {fTemp970 + fTemp676 * fTemp971 + (fTemp966 - (iTemp967) as F64) * (fTemp969 - (fTemp970 + fTemp676 * (fTemp971 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp968, 4)) as usize] } - fTemp969))))} else {1.0 - (fTemp964 + fTemp676 * fTemp965 + (fTemp960 - (iTemp961) as F64) * (fTemp963 - (fTemp964 + fTemp676 * (fTemp965 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp962, 4)) as usize] } - fTemp963)))))};
			let mut fTemp973: F64 = fTemp681 + fTemp959;
			let mut fTemp974: F64 = 65535.0 * (1.0 - fTemp973);
			let mut iTemp975: i32 = (fTemp974) as i32;
			let mut iTemp976: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp975, 65535)))), 196607));
			let mut fTemp977: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp976, 3)) as usize] };
			let mut fTemp978: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp976 as usize] };
			let mut fTemp979: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp976, 1)) as usize] } - fTemp978;
			let mut fTemp980: F64 = 65535.0 * fTemp973;
			let mut iTemp981: i32 = (fTemp980) as i32;
			let mut iTemp982: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp981, 65535)))), 196607));
			let mut fTemp983: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp982, 3), 196607))) as usize] };
			let mut fTemp984: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp982 as usize] };
			let mut fTemp985: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp982, 1), 196607))) as usize] } - fTemp984;
			let mut iTemp986: i32 = (fTemp737 > ((if iTemp667 != 0 {fTemp984 + fTemp676 * fTemp985 + (fTemp980 - (iTemp981) as F64) * (fTemp983 - (fTemp984 + fTemp676 * (fTemp985 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp982, 4), 196607))) as usize] } - fTemp983))))} else {1.0 - (fTemp978 + fTemp676 * fTemp979 + (fTemp974 - (iTemp975) as F64) * (fTemp977 - (fTemp978 + fTemp676 * (fTemp979 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp976, 4)) as usize] } - fTemp977)))))} - fTemp972) / (1.0 - fTemp972))) as i32;
			let mut fTemp987: F64 = if iTemp986 != 0 {fTemp956} else {fTemp959};
			let mut fTemp988: F64 = if iTemp986 != 0 {fTemp959} else {fTemp957};
			let mut fTemp989: F64 = fTemp988 + fTemp987;
			let mut fTemp990: F64 = 0.5 * fTemp989;
			let mut fTemp991: F64 = 65535.0 * (1.0 - fTemp990);
			let mut iTemp992: i32 = (fTemp991) as i32;
			let mut iTemp993: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp992, 65535)))), 196607));
			let mut fTemp994: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp993, 3)) as usize] };
			let mut fTemp995: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp993 as usize] };
			let mut fTemp996: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp993, 1)) as usize] } - fTemp995;
			let mut fTemp997: F64 = 32767.5 * fTemp989;
			let mut iTemp998: i32 = (fTemp997) as i32;
			let mut iTemp999: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp998, 65535)))), 196607));
			let mut fTemp1000: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp999, 3)) as usize] };
			let mut fTemp1001: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp999 as usize] };
			let mut fTemp1002: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp999, 1)) as usize] } - fTemp1001;
			let mut fTemp1003: F64 = if iTemp667 != 0 {fTemp1001 + fTemp676 * fTemp1002 + (fTemp997 - (iTemp998) as F64) * (fTemp1000 - (fTemp1001 + fTemp676 * (fTemp1002 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp999, 4)) as usize] } - fTemp1000))))} else {1.0 - (fTemp995 + fTemp676 * fTemp996 + (fTemp991 - (iTemp992) as F64) * (fTemp994 - (fTemp995 + fTemp676 * (fTemp996 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp993, 4)) as usize] } - fTemp994)))))};
			let mut fTemp1004: F64 = fTemp681 + fTemp990;
			let mut fTemp1005: F64 = 65535.0 * (1.0 - fTemp1004);
			let mut iTemp1006: i32 = (fTemp1005) as i32;
			let mut iTemp1007: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1006, 65535)))), 196607));
			let mut fTemp1008: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1007, 3)) as usize] };
			let mut fTemp1009: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1007 as usize] };
			let mut fTemp1010: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1007, 1)) as usize] } - fTemp1009;
			let mut fTemp1011: F64 = 65535.0 * fTemp1004;
			let mut iTemp1012: i32 = (fTemp1011) as i32;
			let mut iTemp1013: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1012, 65535)))), 196607));
			let mut fTemp1014: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1013, 3), 196607))) as usize] };
			let mut fTemp1015: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1013 as usize] };
			let mut fTemp1016: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1013, 1), 196607))) as usize] } - fTemp1015;
			let mut iTemp1017: i32 = (fTemp737 > ((if iTemp667 != 0 {fTemp1015 + fTemp676 * fTemp1016 + (fTemp1011 - (iTemp1012) as F64) * (fTemp1014 - (fTemp1015 + fTemp676 * (fTemp1016 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1013, 4), 196607))) as usize] } - fTemp1014))))} else {1.0 - (fTemp1009 + fTemp676 * fTemp1010 + (fTemp1005 - (iTemp1006) as F64) * (fTemp1008 - (fTemp1009 + fTemp676 * (fTemp1010 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1007, 4)) as usize] } - fTemp1008)))))} - fTemp1003) / (1.0 - fTemp1003))) as i32;
			let mut fTemp1018: F64 = if iTemp1017 != 0 {fTemp987} else {fTemp990};
			let mut fTemp1019: F64 = if iTemp1017 != 0 {fTemp990} else {fTemp988};
			let mut fTemp1020: F64 = fTemp1019 + fTemp1018;
			let mut fTemp1021: F64 = 0.5 * fTemp1020;
			let mut fTemp1022: F64 = 65535.0 * (1.0 - fTemp1021);
			let mut iTemp1023: i32 = (fTemp1022) as i32;
			let mut iTemp1024: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1023, 65535)))), 196607));
			let mut fTemp1025: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1024, 3)) as usize] };
			let mut fTemp1026: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1024 as usize] };
			let mut fTemp1027: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1024, 1)) as usize] } - fTemp1026;
			let mut fTemp1028: F64 = 32767.5 * fTemp1020;
			let mut iTemp1029: i32 = (fTemp1028) as i32;
			let mut iTemp1030: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1029, 65535)))), 196607));
			let mut fTemp1031: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1030, 3)) as usize] };
			let mut fTemp1032: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1030 as usize] };
			let mut fTemp1033: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1030, 1)) as usize] } - fTemp1032;
			let mut fTemp1034: F64 = if iTemp667 != 0 {fTemp1032 + fTemp676 * fTemp1033 + (fTemp1028 - (iTemp1029) as F64) * (fTemp1031 - (fTemp1032 + fTemp676 * (fTemp1033 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1030, 4)) as usize] } - fTemp1031))))} else {1.0 - (fTemp1026 + fTemp676 * fTemp1027 + (fTemp1022 - (iTemp1023) as F64) * (fTemp1025 - (fTemp1026 + fTemp676 * (fTemp1027 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1024, 4)) as usize] } - fTemp1025)))))};
			let mut fTemp1035: F64 = fTemp681 + fTemp1021;
			let mut fTemp1036: F64 = 65535.0 * (1.0 - fTemp1035);
			let mut iTemp1037: i32 = (fTemp1036) as i32;
			let mut iTemp1038: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1037, 65535)))), 196607));
			let mut fTemp1039: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1038, 3)) as usize] };
			let mut fTemp1040: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1038 as usize] };
			let mut fTemp1041: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1038, 1)) as usize] } - fTemp1040;
			let mut fTemp1042: F64 = 65535.0 * fTemp1035;
			let mut iTemp1043: i32 = (fTemp1042) as i32;
			let mut iTemp1044: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1043, 65535)))), 196607));
			let mut fTemp1045: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1044, 3), 196607))) as usize] };
			let mut fTemp1046: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1044 as usize] };
			let mut fTemp1047: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1044, 1), 196607))) as usize] } - fTemp1046;
			let mut iTemp1048: i32 = (fTemp737 > ((if iTemp667 != 0 {fTemp1046 + fTemp676 * fTemp1047 + (fTemp1042 - (iTemp1043) as F64) * (fTemp1045 - (fTemp1046 + fTemp676 * (fTemp1047 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1044, 4), 196607))) as usize] } - fTemp1045))))} else {1.0 - (fTemp1040 + fTemp676 * fTemp1041 + (fTemp1036 - (iTemp1037) as F64) * (fTemp1039 - (fTemp1040 + fTemp676 * (fTemp1041 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1038, 4)) as usize] } - fTemp1039)))))} - fTemp1034) / (1.0 - fTemp1034))) as i32;
			let mut fTemp1049: F64 = if iTemp1048 != 0 {fTemp1018} else {fTemp1021};
			let mut fTemp1050: F64 = if iTemp1048 != 0 {fTemp1021} else {fTemp1019};
			let mut fTemp1051: F64 = fTemp1050 + fTemp1049;
			let mut fTemp1052: F64 = 0.5 * fTemp1051;
			let mut fTemp1053: F64 = 65535.0 * (1.0 - fTemp1052);
			let mut iTemp1054: i32 = (fTemp1053) as i32;
			let mut iTemp1055: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1054, 65535)))), 196607));
			let mut fTemp1056: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1055, 3)) as usize] };
			let mut fTemp1057: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1055 as usize] };
			let mut fTemp1058: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1055, 1)) as usize] } - fTemp1057;
			let mut fTemp1059: F64 = 32767.5 * fTemp1051;
			let mut iTemp1060: i32 = (fTemp1059) as i32;
			let mut iTemp1061: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1060, 65535)))), 196607));
			let mut fTemp1062: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1061, 3)) as usize] };
			let mut fTemp1063: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1061 as usize] };
			let mut fTemp1064: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1061, 1)) as usize] } - fTemp1063;
			let mut fTemp1065: F64 = if iTemp667 != 0 {fTemp1063 + fTemp676 * fTemp1064 + (fTemp1059 - (iTemp1060) as F64) * (fTemp1062 - (fTemp1063 + fTemp676 * (fTemp1064 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1061, 4)) as usize] } - fTemp1062))))} else {1.0 - (fTemp1057 + fTemp676 * fTemp1058 + (fTemp1053 - (iTemp1054) as F64) * (fTemp1056 - (fTemp1057 + fTemp676 * (fTemp1058 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1055, 4)) as usize] } - fTemp1056)))))};
			let mut fTemp1066: F64 = fTemp681 + fTemp1052;
			let mut fTemp1067: F64 = 65535.0 * (1.0 - fTemp1066);
			let mut iTemp1068: i32 = (fTemp1067) as i32;
			let mut iTemp1069: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1068, 65535)))), 196607));
			let mut fTemp1070: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1069, 3)) as usize] };
			let mut fTemp1071: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1069 as usize] };
			let mut fTemp1072: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1069, 1)) as usize] } - fTemp1071;
			let mut fTemp1073: F64 = 65535.0 * fTemp1066;
			let mut iTemp1074: i32 = (fTemp1073) as i32;
			let mut iTemp1075: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1074, 65535)))), 196607));
			let mut fTemp1076: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1075, 3), 196607))) as usize] };
			let mut fTemp1077: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1075 as usize] };
			let mut fTemp1078: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1075, 1), 196607))) as usize] } - fTemp1077;
			let mut iTemp1079: i32 = (fTemp737 > ((if iTemp667 != 0 {fTemp1077 + fTemp676 * fTemp1078 + (fTemp1073 - (iTemp1074) as F64) * (fTemp1076 - (fTemp1077 + fTemp676 * (fTemp1078 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1075, 4), 196607))) as usize] } - fTemp1076))))} else {1.0 - (fTemp1071 + fTemp676 * fTemp1072 + (fTemp1067 - (iTemp1068) as F64) * (fTemp1070 - (fTemp1071 + fTemp676 * (fTemp1072 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1069, 4)) as usize] } - fTemp1070)))))} - fTemp1065) / (1.0 - fTemp1065))) as i32;
			let mut fTemp1080: F64 = if iTemp1079 != 0 {fTemp1049} else {fTemp1052};
			let mut fTemp1081: F64 = if iTemp1079 != 0 {fTemp1052} else {fTemp1050};
			let mut fTemp1082: F64 = fTemp1081 + fTemp1080;
			let mut fTemp1083: F64 = 0.5 * fTemp1082;
			let mut fTemp1084: F64 = 65535.0 * (1.0 - fTemp1083);
			let mut iTemp1085: i32 = (fTemp1084) as i32;
			let mut iTemp1086: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1085, 65535)))), 196607));
			let mut fTemp1087: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1086, 3)) as usize] };
			let mut fTemp1088: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1086 as usize] };
			let mut fTemp1089: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1086, 1)) as usize] } - fTemp1088;
			let mut fTemp1090: F64 = 32767.5 * fTemp1082;
			let mut iTemp1091: i32 = (fTemp1090) as i32;
			let mut iTemp1092: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1091, 65535)))), 196607));
			let mut fTemp1093: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1092, 3)) as usize] };
			let mut fTemp1094: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1092 as usize] };
			let mut fTemp1095: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1092, 1)) as usize] } - fTemp1094;
			let mut fTemp1096: F64 = if iTemp667 != 0 {fTemp1094 + fTemp676 * fTemp1095 + (fTemp1090 - (iTemp1091) as F64) * (fTemp1093 - (fTemp1094 + fTemp676 * (fTemp1095 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1092, 4)) as usize] } - fTemp1093))))} else {1.0 - (fTemp1088 + fTemp676 * fTemp1089 + (fTemp1084 - (iTemp1085) as F64) * (fTemp1087 - (fTemp1088 + fTemp676 * (fTemp1089 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1086, 4)) as usize] } - fTemp1087)))))};
			let mut fTemp1097: F64 = fTemp681 + fTemp1083;
			let mut fTemp1098: F64 = 65535.0 * (1.0 - fTemp1097);
			let mut iTemp1099: i32 = (fTemp1098) as i32;
			let mut iTemp1100: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1099, 65535)))), 196607));
			let mut fTemp1101: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1100, 3)) as usize] };
			let mut fTemp1102: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1100 as usize] };
			let mut fTemp1103: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1100, 1)) as usize] } - fTemp1102;
			let mut fTemp1104: F64 = 65535.0 * fTemp1097;
			let mut iTemp1105: i32 = (fTemp1104) as i32;
			let mut iTemp1106: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1105, 65535)))), 196607));
			let mut fTemp1107: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1106, 3), 196607))) as usize] };
			let mut fTemp1108: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1106 as usize] };
			let mut fTemp1109: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1106, 1), 196607))) as usize] } - fTemp1108;
			let mut iTemp1110: i32 = (fTemp737 > ((if iTemp667 != 0 {fTemp1108 + fTemp676 * fTemp1109 + (fTemp1104 - (iTemp1105) as F64) * (fTemp1107 - (fTemp1108 + fTemp676 * (fTemp1109 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1106, 4), 196607))) as usize] } - fTemp1107))))} else {1.0 - (fTemp1102 + fTemp676 * fTemp1103 + (fTemp1098 - (iTemp1099) as F64) * (fTemp1101 - (fTemp1102 + fTemp676 * (fTemp1103 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1100, 4)) as usize] } - fTemp1101)))))} - fTemp1096) / (1.0 - fTemp1096))) as i32;
			let mut fTemp1111: F64 = if iTemp1110 != 0 {fTemp1080} else {fTemp1083};
			let mut fTemp1112: F64 = if iTemp1110 != 0 {fTemp1083} else {fTemp1081};
			let mut fTemp1113: F64 = fTemp1112 + fTemp1111;
			let mut fTemp1114: F64 = 0.5 * fTemp1113;
			let mut fTemp1115: F64 = 65535.0 * (1.0 - fTemp1114);
			let mut iTemp1116: i32 = (fTemp1115) as i32;
			let mut iTemp1117: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1116, 65535)))), 196607));
			let mut fTemp1118: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1117, 3)) as usize] };
			let mut fTemp1119: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1117 as usize] };
			let mut fTemp1120: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1117, 1)) as usize] } - fTemp1119;
			let mut fTemp1121: F64 = 32767.5 * fTemp1113;
			let mut iTemp1122: i32 = (fTemp1121) as i32;
			let mut iTemp1123: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1122, 65535)))), 196607));
			let mut fTemp1124: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1123, 3)) as usize] };
			let mut fTemp1125: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1123 as usize] };
			let mut fTemp1126: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1123, 1)) as usize] } - fTemp1125;
			let mut fTemp1127: F64 = if iTemp667 != 0 {fTemp1125 + fTemp676 * fTemp1126 + (fTemp1121 - (iTemp1122) as F64) * (fTemp1124 - (fTemp1125 + fTemp676 * (fTemp1126 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1123, 4)) as usize] } - fTemp1124))))} else {1.0 - (fTemp1119 + fTemp676 * fTemp1120 + (fTemp1115 - (iTemp1116) as F64) * (fTemp1118 - (fTemp1119 + fTemp676 * (fTemp1120 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1117, 4)) as usize] } - fTemp1118)))))};
			let mut fTemp1128: F64 = fTemp681 + fTemp1114;
			let mut fTemp1129: F64 = 65535.0 * (1.0 - fTemp1128);
			let mut iTemp1130: i32 = (fTemp1129) as i32;
			let mut iTemp1131: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1130, 65535)))), 196607));
			let mut fTemp1132: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1131, 3)) as usize] };
			let mut fTemp1133: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1131 as usize] };
			let mut fTemp1134: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1131, 1)) as usize] } - fTemp1133;
			let mut fTemp1135: F64 = 65535.0 * fTemp1128;
			let mut iTemp1136: i32 = (fTemp1135) as i32;
			let mut iTemp1137: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1136, 65535)))), 196607));
			let mut fTemp1138: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1137, 3), 196607))) as usize] };
			let mut fTemp1139: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1137 as usize] };
			let mut fTemp1140: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1137, 1), 196607))) as usize] } - fTemp1139;
			let mut iTemp1141: i32 = (fTemp737 > ((if iTemp667 != 0 {fTemp1139 + fTemp676 * fTemp1140 + (fTemp1135 - (iTemp1136) as F64) * (fTemp1138 - (fTemp1139 + fTemp676 * (fTemp1140 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1137, 4), 196607))) as usize] } - fTemp1138))))} else {1.0 - (fTemp1133 + fTemp676 * fTemp1134 + (fTemp1129 - (iTemp1130) as F64) * (fTemp1132 - (fTemp1133 + fTemp676 * (fTemp1134 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1131, 4)) as usize] } - fTemp1132)))))} - fTemp1127) / (1.0 - fTemp1127))) as i32;
			let mut fTemp1142: F64 = if iTemp1141 != 0 {fTemp1111} else {fTemp1114};
			let mut fTemp1143: F64 = if iTemp1141 != 0 {fTemp1114} else {fTemp1112};
			let mut fTemp1144: F64 = fTemp1143 + fTemp1142;
			let mut fTemp1145: F64 = 0.5 * fTemp1144;
			let mut fTemp1146: F64 = 65535.0 * (1.0 - fTemp1145);
			let mut iTemp1147: i32 = (fTemp1146) as i32;
			let mut iTemp1148: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1147, 65535)))), 196607));
			let mut fTemp1149: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1148, 3)) as usize] };
			let mut fTemp1150: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1148 as usize] };
			let mut fTemp1151: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1148, 1)) as usize] } - fTemp1150;
			let mut fTemp1152: F64 = 32767.5 * fTemp1144;
			let mut iTemp1153: i32 = (fTemp1152) as i32;
			let mut iTemp1154: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1153, 65535)))), 196607));
			let mut fTemp1155: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1154, 3)) as usize] };
			let mut fTemp1156: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1154 as usize] };
			let mut fTemp1157: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1154, 1)) as usize] } - fTemp1156;
			let mut fTemp1158: F64 = if iTemp667 != 0 {fTemp1156 + fTemp676 * fTemp1157 + (fTemp1152 - (iTemp1153) as F64) * (fTemp1155 - (fTemp1156 + fTemp676 * (fTemp1157 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1154, 4)) as usize] } - fTemp1155))))} else {1.0 - (fTemp1150 + fTemp676 * fTemp1151 + (fTemp1146 - (iTemp1147) as F64) * (fTemp1149 - (fTemp1150 + fTemp676 * (fTemp1151 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1148, 4)) as usize] } - fTemp1149)))))};
			let mut fTemp1159: F64 = fTemp681 + fTemp1145;
			let mut fTemp1160: F64 = 65535.0 * (1.0 - fTemp1159);
			let mut iTemp1161: i32 = (fTemp1160) as i32;
			let mut iTemp1162: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1161, 65535)))), 196607));
			let mut fTemp1163: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1162, 3)) as usize] };
			let mut fTemp1164: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1162 as usize] };
			let mut fTemp1165: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1162, 1)) as usize] } - fTemp1164;
			let mut fTemp1166: F64 = 65535.0 * fTemp1159;
			let mut iTemp1167: i32 = (fTemp1166) as i32;
			let mut iTemp1168: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1167, 65535)))), 196607));
			let mut fTemp1169: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1168, 3), 196607))) as usize] };
			let mut fTemp1170: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1168 as usize] };
			let mut fTemp1171: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1168, 1), 196607))) as usize] } - fTemp1170;
			let mut iTemp1172: i32 = (fTemp737 > ((if iTemp667 != 0 {fTemp1170 + fTemp676 * fTemp1171 + (fTemp1166 - (iTemp1167) as F64) * (fTemp1169 - (fTemp1170 + fTemp676 * (fTemp1171 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1168, 4), 196607))) as usize] } - fTemp1169))))} else {1.0 - (fTemp1164 + fTemp676 * fTemp1165 + (fTemp1160 - (iTemp1161) as F64) * (fTemp1163 - (fTemp1164 + fTemp676 * (fTemp1165 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1162, 4)) as usize] } - fTemp1163)))))} - fTemp1158) / (1.0 - fTemp1158))) as i32;
			let mut fTemp1173: F64 = if iTemp1172 != 0 {fTemp1142} else {fTemp1145};
			let mut fTemp1174: F64 = if iTemp1172 != 0 {fTemp1145} else {fTemp1143};
			let mut fTemp1175: F64 = fTemp1174 + fTemp1173;
			let mut fTemp1176: F64 = 0.5 * fTemp1175;
			let mut fTemp1177: F64 = 65535.0 * (1.0 - fTemp1176);
			let mut iTemp1178: i32 = (fTemp1177) as i32;
			let mut iTemp1179: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1178, 65535)))), 196607));
			let mut fTemp1180: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1179, 3)) as usize] };
			let mut fTemp1181: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1179 as usize] };
			let mut fTemp1182: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1179, 1)) as usize] } - fTemp1181;
			let mut fTemp1183: F64 = 32767.5 * fTemp1175;
			let mut iTemp1184: i32 = (fTemp1183) as i32;
			let mut iTemp1185: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1184, 65535)))), 196607));
			let mut fTemp1186: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1185, 3)) as usize] };
			let mut fTemp1187: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1185 as usize] };
			let mut fTemp1188: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1185, 1)) as usize] } - fTemp1187;
			let mut fTemp1189: F64 = if iTemp667 != 0 {fTemp1187 + fTemp676 * fTemp1188 + (fTemp1183 - (iTemp1184) as F64) * (fTemp1186 - (fTemp1187 + fTemp676 * (fTemp1188 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1185, 4), 196607))) as usize] } - fTemp1186))))} else {1.0 - (fTemp1181 + fTemp676 * fTemp1182 + (fTemp1177 - (iTemp1178) as F64) * (fTemp1180 - (fTemp1181 + fTemp676 * (fTemp1182 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1179, 4), 196607))) as usize] } - fTemp1180)))))};
			let mut fTemp1190: F64 = fTemp681 + fTemp1176;
			let mut fTemp1191: F64 = 65535.0 * (1.0 - fTemp1190);
			let mut iTemp1192: i32 = (fTemp1191) as i32;
			let mut iTemp1193: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1192, 65535)))), 196607));
			let mut fTemp1194: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1193, 3)) as usize] };
			let mut fTemp1195: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1193 as usize] };
			let mut fTemp1196: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1193, 1)) as usize] } - fTemp1195;
			let mut fTemp1197: F64 = 65535.0 * fTemp1190;
			let mut iTemp1198: i32 = (fTemp1197) as i32;
			let mut iTemp1199: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1198, 65535)))), 196607));
			let mut fTemp1200: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1199, 3), 196607))) as usize] };
			let mut fTemp1201: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1199 as usize] };
			let mut fTemp1202: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1199, 1), 196607))) as usize] } - fTemp1201;
			let mut iTemp1203: i32 = (fTemp737 > ((if iTemp667 != 0 {fTemp1201 + fTemp676 * fTemp1202 + (fTemp1197 - (iTemp1198) as F64) * (fTemp1200 - (fTemp1201 + fTemp676 * (fTemp1202 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1199, 4), 196607))) as usize] } - fTemp1200))))} else {1.0 - (fTemp1195 + fTemp676 * fTemp1196 + (fTemp1191 - (iTemp1192) as F64) * (fTemp1194 - (fTemp1195 + fTemp676 * (fTemp1196 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1193, 4)) as usize] } - fTemp1194)))))} - fTemp1189) / (1.0 - fTemp1189))) as i32;
			let mut fTemp1204: F64 = F64::min(1.0, F64::max(0.0, 0.5 * (if iTemp1203 != 0 {fTemp1176} else {fTemp1174} + if iTemp1203 != 0 {fTemp1173} else {fTemp1176})));
			self.fRec15[0] = fTemp1204;
			let mut fTemp1205: F64 = 65535.0 * (1.0 - fTemp1204);
			let mut iTemp1206: i32 = (fTemp1205) as i32;
			let mut iTemp1207: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1206, 65535)))), 196607));
			let mut fTemp1208: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1207, 3)) as usize] };
			let mut fTemp1209: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1207 as usize] };
			let mut fTemp1210: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1207, 1)) as usize] } - fTemp1209;
			let mut fTemp1211: F64 = 65535.0 * fTemp1204;
			let mut iTemp1212: i32 = (fTemp1211) as i32;
			let mut iTemp1213: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1212, 65535)))), 196607));
			let mut fTemp1214: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1213, 3)) as usize] };
			let mut fTemp1215: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1213 as usize] };
			let mut fTemp1216: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1213, 1)) as usize] } - fTemp1215;
			let mut fTemp1217: F64 = if iTemp667 != 0 {fTemp1215 + fTemp676 * fTemp1216 + (fTemp1211 - (iTemp1212) as F64) * (fTemp1214 - (fTemp1215 + fTemp676 * (fTemp1216 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1213, 4), 196607))) as usize] } - fTemp1214))))} else {1.0 - (fTemp1209 + fTemp676 * fTemp1210 + (fTemp1205 - (iTemp1206) as F64) * (fTemp1208 - (fTemp1209 + fTemp676 * (fTemp1210 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1207, 4), 196607))) as usize] } - fTemp1208)))))};
			let mut fTemp1218: F64 = fTemp681 + fTemp1204;
			let mut fTemp1219: F64 = 65535.0 * (1.0 - fTemp1218);
			let mut iTemp1220: i32 = (fTemp1219) as i32;
			let mut iTemp1221: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1220, 65535)))), 196607));
			let mut fTemp1222: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1221, 3)) as usize] };
			let mut fTemp1223: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1221 as usize] };
			let mut fTemp1224: F64 = unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1221, 1)) as usize] } - fTemp1223;
			let mut fTemp1225: F64 = 65535.0 * fTemp1218;
			let mut iTemp1226: i32 = (fTemp1225) as i32;
			let mut iTemp1227: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1226, 65535)))), 196607));
			let mut fTemp1228: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1227, 3), 196607))) as usize] };
			let mut fTemp1229: F64 = unsafe { ftbl0LambRs192kSIG0[iTemp1227 as usize] };
			let mut fTemp1230: F64 = unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1227, 1), 196607))) as usize] } - fTemp1229;
			let mut fTemp1231: F64 = fTemp637 + if ((0.001 * fTemp680) == 0.0) as i32 != 0 {fTemp666} else {fTemp666 * (if iTemp667 != 0 {fTemp1229 + fTemp676 * fTemp1230 + (fTemp1225 - (iTemp1226) as F64) * (fTemp1228 - (fTemp1229 + fTemp676 * (fTemp1230 - (unsafe { ftbl0LambRs192kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1227, 4), 196607))) as usize] } - fTemp1228))))} else {1.0 - (fTemp1223 + fTemp676 * fTemp1224 + (fTemp1219 - (iTemp1220) as F64) * (fTemp1222 - (fTemp1223 + fTemp676 * (fTemp1224 - (unsafe { ftbl0LambRs192kSIG0[(i32::wrapping_add(iTemp1221, 4)) as usize] } - fTemp1222)))))} - fTemp1217) / (1.0 - fTemp1217)};
			self.fRec16[(self.IOTA0 & 32767) as usize] = if iTemp679 != 0 {F64::min(fTemp1231, fTemp637)} else {F64::max(fTemp1231, fTemp637)};
			let mut fTemp1232: F64 = self.fRec16[((i32::wrapping_sub(self.IOTA0, iSlow78)) & 32767) as usize];
			self.fHbargraph2 = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, fTemp1232));
			*output1 = 0.5 * fTemp2 * self.fVec2[((i32::wrapping_sub(self.IOTA0, iSlow79)) & 32767) as usize] + self.fRec14[0] * fTemp3 * self.fVec3[((i32::wrapping_sub(self.IOTA0, iSlow79)) & 32767) as usize] * fTemp1232;
			self.fRec0[1] = self.fRec0[0];
			self.IOTA0 = i32::wrapping_add(self.IOTA0, 1);
			self.fRec4[1] = self.fRec4[0];
			self.fRec11[1] = self.fRec11[0];
			self.fRec10[1] = self.fRec10[0];
			self.fRec9[1] = self.fRec9[0];
			self.fRec8[1] = self.fRec8[0];
			self.fRec7[1] = self.fRec7[0];
			self.fRec5[1] = self.fRec5[0];
			self.fRec12[1] = self.fRec12[0];
			self.fRec6[1] = self.fRec6[0];
			self.fVec5[2] = self.fVec5[1];
			self.fVec5[1] = self.fVec5[0];
			for j0 in (1..=6).rev() {
				self.fVec6[j0 as usize] = self.fVec6[(i32::wrapping_sub(j0, 1)) as usize];
			}
			for j1 in (1..=14).rev() {
				self.fVec7[j1 as usize] = self.fVec7[(i32::wrapping_sub(j1, 1)) as usize];
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
			self.fVec31[1] = self.fVec31[0];
			self.fVec32[1] = self.fVec32[0];
			self.fVec33[1] = self.fVec33[0];
			self.fRec1[1] = self.fRec1[0];
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
			self.fVec48[2] = self.fVec48[1];
			self.fVec48[1] = self.fVec48[0];
			for j6 in (1..=6).rev() {
				self.fVec49[j6 as usize] = self.fVec49[(i32::wrapping_sub(j6, 1)) as usize];
			}
			for j7 in (1..=14).rev() {
				self.fVec50[j7 as usize] = self.fVec50[(i32::wrapping_sub(j7, 1)) as usize];
			}
			self.fVec61[1] = self.fVec61[0];
			self.fVec62[1] = self.fVec62[0];
			self.fVec63[1] = self.fVec63[0];
			self.fRec15[1] = self.fRec15[0];
		}
	}

}

}

pub use dsp_192k::LambRs192k;
