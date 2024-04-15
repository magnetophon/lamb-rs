/* ------------------------------------------------------------
author: "Bart Brouns"
license: "AGPLv3"
name: "lamb-rs"
version: "0.1"
Code generated with Faust 2.72.14 (https://faust.grame.fr)
Compilation options: -a /run/user/1001/.tmpSToded -lang rust -ct 1 -cn LambRs96k -es 1 -mcd 16 -mdd 1024 -mdy 33 -double -ftz 0
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
		for l41 in 0..2 {
			self.iRec13[l41 as usize] = 0;
		}
	}
	
	fn fillLambRs96kSIG0(&mut self, count: i32, table: &mut[F64]) {
		for i1 in 0..count {
			self.iRec13[0] = i32::wrapping_add(self.iRec13[1], 1);
			let mut iTemp65: i32 = i32::wrapping_add(self.iRec13[0], -1);
			let mut fTemp66: F64 = (iTemp65 % 3) as F64 as i32 as F64;
			let mut fTemp67: F64 = 0.5 * fTemp66;
			let mut fTemp68: F64 = F64::powf(fTemp67, 0.21 * fTemp66 + 1.0);
			let mut fTemp69: F64 = (0.3333333333333333 * (iTemp65 % 196608) as F64) as i32 as F64;
			table[i1 as usize] = F64::min(1.0, F64::max(0.0, if (fTemp67 == 0.0) as i32 != 0 {0.5 * (F64::sin(4.793763109162727e-05 * fTemp69 + 4.71238898038469) + 1.0)} else {0.5 * (F64::sin(3.141592653589793 * ((1.0 - F64::exp(-(3.692683299000534e-05 * fTemp68 * fTemp69))) / (1.0 - F64::exp(-(2.42 * fTemp68)))) + 4.71238898038469) + 1.0)}));
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
static mut ftbl0LambRs96kSIG0: [F64;196608] = [0.0;196608];
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
	fVec28: [F64;8192],
	fVec29: [F64;2],
	fHslider10: F64,
	fHslider11: F64,
	fVec30: [F64;2],
	fVec31: [F64;2],
	fConst10: F64,
	fRec1: [F64;2],
	fRec2: [F64;16384],
	fCheckbox1: F64,
	fHbargraph0: F64,
	fHbargraph1: F64,
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
	fVec44: [F64;8192],
	fRec17: [F64;2],
	fVec45: [F64;3],
	fVec46: [F64;7],
	fVec47: [F64;15],
	fVec48: [F64;32],
	fVec49: [F64;64],
	fVec50: [F64;128],
	fVec51: [F64;256],
	fVec52: [F64;512],
	fVec53: [F64;1024],
	fVec54: [F64;2048],
	fVec55: [F64;4096],
	fVec56: [F64;8192],
	fVec57: [F64;2],
	fVec58: [F64;2],
	fVec59: [F64;2],
	fRec15: [F64;2],
	fRec16: [F64;16384],
	fHbargraph2: F64,
}

impl FaustDsp for LambRs96k {
	type T = F64;
		
	fn new() -> LambRs96k { 
		LambRs96k {
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
			fVec28: [0.0;8192],
			fVec29: [0.0;2],
			fHslider10: 0.0,
			fHslider11: 0.0,
			fVec30: [0.0;2],
			fVec31: [0.0;2],
			fConst10: 0.0,
			fRec1: [0.0;2],
			fRec2: [0.0;16384],
			fCheckbox1: 0.0,
			fHbargraph0: 0.0,
			fHbargraph1: 0.0,
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
			fVec44: [0.0;8192],
			fRec17: [0.0;2],
			fVec45: [0.0;3],
			fVec46: [0.0;7],
			fVec47: [0.0;15],
			fVec48: [0.0;32],
			fVec49: [0.0;64],
			fVec50: [0.0;128],
			fVec51: [0.0;256],
			fVec52: [0.0;512],
			fVec53: [0.0;1024],
			fVec54: [0.0;2048],
			fVec55: [0.0;4096],
			fVec56: [0.0;8192],
			fVec57: [0.0;2],
			fVec58: [0.0;2],
			fVec59: [0.0;2],
			fRec15: [0.0;2],
			fRec16: [0.0;16384],
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
		m.declare("compile_options", r"-a /run/user/1001/.tmpSToded -lang rust -ct 1 -cn LambRs96k -es 1 -mcd 16 -mdd 1024 -mdy 33 -double -ftz 0");
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
		return 2;
	}
	
	fn class_init(sample_rate: i32) {
		let mut sig0: LambRs96kSIG0 = newLambRs96kSIG0();
		sig0.instance_initLambRs96kSIG0(sample_rate);
		sig0.fillLambRs96kSIG0(196608, unsafe { &mut ftbl0LambRs96kSIG0 });
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
		for l39 in 0..8192 {
			self.fVec28[l39 as usize] = 0.0;
		}
		for l40 in 0..2 {
			self.fVec29[l40 as usize] = 0.0;
		}
		for l42 in 0..2 {
			self.fVec30[l42 as usize] = 0.0;
		}
		for l43 in 0..2 {
			self.fVec31[l43 as usize] = 0.0;
		}
		for l44 in 0..2 {
			self.fRec1[l44 as usize] = 0.0;
		}
		for l45 in 0..16384 {
			self.fRec2[l45 as usize] = 0.0;
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
		for l59 in 0..8192 {
			self.fVec44[l59 as usize] = 0.0;
		}
		for l60 in 0..2 {
			self.fRec17[l60 as usize] = 0.0;
		}
		for l61 in 0..3 {
			self.fVec45[l61 as usize] = 0.0;
		}
		for l62 in 0..7 {
			self.fVec46[l62 as usize] = 0.0;
		}
		for l63 in 0..15 {
			self.fVec47[l63 as usize] = 0.0;
		}
		for l64 in 0..32 {
			self.fVec48[l64 as usize] = 0.0;
		}
		for l65 in 0..64 {
			self.fVec49[l65 as usize] = 0.0;
		}
		for l66 in 0..128 {
			self.fVec50[l66 as usize] = 0.0;
		}
		for l67 in 0..256 {
			self.fVec51[l67 as usize] = 0.0;
		}
		for l68 in 0..512 {
			self.fVec52[l68 as usize] = 0.0;
		}
		for l69 in 0..1024 {
			self.fVec53[l69 as usize] = 0.0;
		}
		for l70 in 0..2048 {
			self.fVec54[l70 as usize] = 0.0;
		}
		for l71 in 0..4096 {
			self.fVec55[l71 as usize] = 0.0;
		}
		for l72 in 0..8192 {
			self.fVec56[l72 as usize] = 0.0;
		}
		for l73 in 0..2 {
			self.fVec57[l73 as usize] = 0.0;
		}
		for l74 in 0..2 {
			self.fVec58[l74 as usize] = 0.0;
		}
		for l75 in 0..2 {
			self.fVec59[l75 as usize] = 0.0;
		}
		for l76 in 0..2 {
			self.fRec15[l76 as usize] = 0.0;
		}
		for l77 in 0..16384 {
			self.fRec16[l77 as usize] = 0.0;
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
		ui_interface.declare(None, "99", "");
		ui_interface.open_vertical_box("gain reduction");
		ui_interface.declare(Some(ParamIndex(15)), "unit", "dB");
		ui_interface.add_horizontal_bargraph("0", ParamIndex(15), -24.0, 0.0);
		ui_interface.declare(Some(ParamIndex(16)), "unit", "dB");
		ui_interface.add_horizontal_bargraph("1", ParamIndex(16), -24.0, 0.0);
		ui_interface.close_box();
		ui_interface.declare(Some(ParamIndex(17)), "99", "");
		ui_interface.declare(Some(ParamIndex(17)), "unit", "samples");
		ui_interface.add_horizontal_bargraph("latency", ParamIndex(17), 0.0, 9.6e+03);
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
		self.fHbargraph1 = if (fSlow73) as i32 != 0 {9.6e+03} else {fSlow72};
		let mut iSlow75: i32 = (self.fHbargraph1) as i32;
		let mut fSlow76: F64 = self.fConst3 * F64::powf(1e+01, 0.05 * self.fHslider12);
		let zipped_iterators = inputs0.zip(inputs1).zip(outputs0).zip(outputs1);
		for (((input0, input1), output0), output1) in zipped_iterators {
			let mut fTemp0: F64 = self.fConst1 + self.fRec0[1];
			let mut fTemp1: F64 = self.fRec0[1] - self.fConst1;
			self.fRec0[0] = if (fTemp0 < fSlow0) as i32 != 0 {fTemp0} else {if (fTemp1 > fSlow0) as i32 != 0 {fTemp1} else {fSlow0}};
			let mut fTemp2: F64 = F64::sin(6.283185307179586 * (0.5 * self.fRec0[0] + 0.75)) + 1.0;
			let mut fTemp3: F64 = 1.0 - 0.5 * fTemp2;
			let mut fTemp4: F64 = self.fRec2[((i32::wrapping_sub(self.IOTA0, 1)) & 16383) as usize];
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
			let mut fTemp21: F64 = F64::powf(1e+01, -(0.05 * F64::max(0.0, if (iTemp19 == 0) as i32 != 0 {0.0} else {if (iTemp19 == 1) as i32 != 0 {fSlow12 * LambRs96k_faustpower2_f(fSlow7 + fTemp20)} else {fTemp20}})));
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
			let mut fTemp40: F64 = F64::min(fTemp33, F64::powf(1e+01, -(fSlow18 * F64::max(0.0, if (iTemp38 == 0) as i32 != 0 {0.0} else {if (iTemp38 == 1) as i32 != 0 {fSlow12 * LambRs96k_faustpower2_f(fSlow7 + fTemp39)} else {fTemp39}}))));
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
			self.fVec16[(self.IOTA0 & 8191) as usize] = F64::min(fTemp51, self.fVec15[((i32::wrapping_sub(self.IOTA0, 2048)) & 4095) as usize]);
			self.fRec3[0] = F64::max(F64::min(self.fRec3[1], self.fVec4[((i32::wrapping_sub(self.IOTA0, iSlow21)) & 16383) as usize]), F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow23 != 0 {fTemp40} else {1.7976931348623157e+308}, if iSlow24 != 0 {self.fVec5[iSlow23 as usize]} else {1.7976931348623157e+308}), if iSlow25 != 0 {self.fVec6[iSlow26 as usize]} else {1.7976931348623157e+308}), if iSlow27 != 0 {self.fVec7[iSlow28 as usize]} else {1.7976931348623157e+308}), if iSlow29 != 0 {self.fVec8[((i32::wrapping_sub(self.IOTA0, iSlow30)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow31 != 0 {self.fVec9[((i32::wrapping_sub(self.IOTA0, iSlow32)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow33 != 0 {self.fVec10[((i32::wrapping_sub(self.IOTA0, iSlow34)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow35 != 0 {self.fVec11[((i32::wrapping_sub(self.IOTA0, iSlow36)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow37 != 0 {self.fVec12[((i32::wrapping_sub(self.IOTA0, iSlow38)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow39 != 0 {self.fVec13[((i32::wrapping_sub(self.IOTA0, iSlow40)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow41 != 0 {self.fVec14[((i32::wrapping_sub(self.IOTA0, iSlow42)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow43 != 0 {self.fVec15[((i32::wrapping_sub(self.IOTA0, iSlow44)) & 4095) as usize]} else {1.7976931348623157e+308}), if iSlow45 != 0 {self.fVec16[((i32::wrapping_sub(self.IOTA0, iSlow46)) & 8191) as usize]} else {1.7976931348623157e+308}));
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
			let mut fTemp62: F64 = F64::min(fTemp61, self.fVec26[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			self.fVec27[(self.IOTA0 & 4095) as usize] = fTemp62;
			self.fVec28[(self.IOTA0 & 8191) as usize] = F64::min(fTemp62, self.fVec27[((i32::wrapping_sub(self.IOTA0, 2048)) & 4095) as usize]);
			let mut fTemp63: F64 = F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow4 != 0 {self.fRec3[0]} else {1.7976931348623157e+308}, if iSlow47 != 0 {self.fVec17[iSlow4 as usize]} else {1.7976931348623157e+308}), if iSlow48 != 0 {self.fVec18[iSlow49 as usize]} else {1.7976931348623157e+308}), if iSlow50 != 0 {self.fVec19[iSlow51 as usize]} else {1.7976931348623157e+308}), if iSlow52 != 0 {self.fVec20[((i32::wrapping_sub(self.IOTA0, iSlow53)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow54 != 0 {self.fVec21[((i32::wrapping_sub(self.IOTA0, iSlow55)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow56 != 0 {self.fVec22[((i32::wrapping_sub(self.IOTA0, iSlow57)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow58 != 0 {self.fVec23[((i32::wrapping_sub(self.IOTA0, iSlow59)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow60 != 0 {self.fVec24[((i32::wrapping_sub(self.IOTA0, iSlow61)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow62 != 0 {self.fVec25[((i32::wrapping_sub(self.IOTA0, iSlow63)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow64 != 0 {self.fVec26[((i32::wrapping_sub(self.IOTA0, iSlow65)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow66 != 0 {self.fVec27[((i32::wrapping_sub(self.IOTA0, iSlow67)) & 4095) as usize]} else {1.7976931348623157e+308}), if iSlow68 != 0 {self.fVec28[((i32::wrapping_sub(self.IOTA0, iSlow69)) & 8191) as usize]} else {1.7976931348623157e+308}) - fTemp4;
			self.fVec29[0] = fTemp63;
			let mut iTemp64: i32 = (fTemp63 > 0.0) as i32;
			let mut fTemp70: F64 = if iTemp64 != 0 {fSlow71} else {fSlow70};
			self.fVec30[0] = fTemp70;
			let mut fTemp71: F64 = 2.0 * fTemp70;
			let mut iTemp72: i32 = (fTemp71) as i32;
			let mut iTemp73: i32 = std::cmp::max(0, std::cmp::min(iTemp72, 2));
			let mut iTemp74: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, 98301), 196607));
			let mut fTemp75: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp74, 3)) as usize] };
			let mut fTemp76: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp74 as usize] };
			let mut fTemp77: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp74, 1)) as usize] } - fTemp76;
			let mut fTemp78: F64 = fTemp71 - (iTemp72) as F64;
			let mut fTemp79: F64 = fTemp76 + fTemp78 * fTemp77 + 0.5 * (fTemp75 - (fTemp76 + fTemp78 * (fTemp77 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp74, 4)) as usize] } - fTemp75))));
			let mut fTemp80: F64 = if iTemp64 != 0 {fTemp79} else {1.0 - fTemp79};
			let mut iTemp81: i32 = (fTemp63 < 0.0) as i32;
			let mut fTemp82: F64 = fSlow1 * (iTemp81) as F64 + fSlow13 * (iTemp64) as F64;
			self.fVec31[0] = fTemp82;
			let mut fTemp83: F64 = self.fConst10 / fTemp82;
			let mut fTemp84: F64 = fTemp83 + 0.5;
			let mut fTemp85: F64 = 65535.0 * (1.0 - fTemp84);
			let mut iTemp86: i32 = (fTemp85) as i32;
			let mut iTemp87: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp86, 65535)))), 196607));
			let mut fTemp88: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp87, 3)) as usize] };
			let mut fTemp89: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp87 as usize] };
			let mut fTemp90: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp87, 1)) as usize] } - fTemp89;
			let mut fTemp91: F64 = 65535.0 * fTemp84;
			let mut iTemp92: i32 = (fTemp91) as i32;
			let mut iTemp93: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp92, 65535)))), 196607));
			let mut fTemp94: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp93, 3), 196607))) as usize] };
			let mut fTemp95: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp93 as usize] };
			let mut fTemp96: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp93, 1), 196607))) as usize] } - fTemp95;
			let mut fTemp97: F64 = 2.0 * self.fVec30[1];
			let mut iTemp98: i32 = (fTemp97) as i32;
			let mut iTemp99: i32 = std::cmp::max(0, std::cmp::min(iTemp98, 2));
			let mut fTemp100: F64 = 65535.0 * (1.0 - self.fRec1[1]);
			let mut iTemp101: i32 = (fTemp100) as i32;
			let mut iTemp102: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp101, 65535))), iTemp99), 196607));
			let mut fTemp103: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp102, 3), 196607))) as usize] };
			let mut fTemp104: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp102 as usize] };
			let mut fTemp105: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp102, 1), 196607))) as usize] } - fTemp104;
			let mut fTemp106: F64 = fTemp97 - (iTemp98) as F64;
			let mut fTemp107: F64 = 65535.0 * self.fRec1[1];
			let mut iTemp108: i32 = (fTemp107) as i32;
			let mut iTemp109: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp99, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp108, 65535)))), 196607));
			let mut fTemp110: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp109, 3), 196607))) as usize] };
			let mut fTemp111: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp109 as usize] };
			let mut fTemp112: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp109, 1), 196607))) as usize] } - fTemp111;
			let mut fTemp113: F64 = self.fRec1[1] + fTemp83;
			let mut fTemp114: F64 = 65535.0 * (1.0 - fTemp113);
			let mut iTemp115: i32 = (fTemp114) as i32;
			let mut iTemp116: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp115, 65535)))), 196607));
			let mut fTemp117: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp116, 3)) as usize] };
			let mut fTemp118: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp116 as usize] };
			let mut fTemp119: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp116, 1)) as usize] } - fTemp118;
			let mut fTemp120: F64 = 65535.0 * fTemp113;
			let mut iTemp121: i32 = (fTemp120) as i32;
			let mut iTemp122: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp121, 65535)))), 196607));
			let mut fTemp123: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp122, 3), 196607))) as usize] };
			let mut fTemp124: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp122 as usize] };
			let mut fTemp125: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp122, 1), 196607))) as usize] } - fTemp124;
			let mut fTemp126: F64 = self.fRec1[1] + self.fConst10 * (1.0 / fTemp82 + 1.0 / self.fVec31[1]);
			let mut fTemp127: F64 = 65535.0 * (1.0 - fTemp126);
			let mut iTemp128: i32 = (fTemp127) as i32;
			let mut iTemp129: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp128, 65535))), iTemp73), 196607));
			let mut fTemp130: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp129, 3)) as usize] };
			let mut fTemp131: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp129 as usize] };
			let mut fTemp132: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp129, 1)) as usize] } - fTemp131;
			let mut fTemp133: F64 = 65535.0 * fTemp126;
			let mut iTemp134: i32 = (fTemp133) as i32;
			let mut iTemp135: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp134, 65535)))), 196607));
			let mut fTemp136: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp135, 3), 196607))) as usize] };
			let mut fTemp137: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp135 as usize] };
			let mut fTemp138: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp135, 1), 196607))) as usize] } - fTemp137;
			let mut fTemp139: F64 = (if iTemp64 != 0 {fTemp137 + fTemp78 * fTemp138 + (fTemp133 - (iTemp134) as F64) * (fTemp136 - (fTemp137 + fTemp78 * (fTemp138 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp135, 4), 196607))) as usize] } - fTemp136))))} else {1.0 - (fTemp131 + fTemp78 * fTemp132 + (fTemp127 - (iTemp128) as F64) * (fTemp130 - (fTemp131 + fTemp78 * (fTemp132 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp129, 4)) as usize] } - fTemp130)))))} - if iTemp64 != 0 {fTemp124 + fTemp78 * fTemp125 + (fTemp120 - (iTemp121) as F64) * (fTemp123 - (fTemp124 + fTemp78 * (fTemp125 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp122, 4), 196607))) as usize] } - fTemp123))))} else {1.0 - (fTemp118 + fTemp78 * fTemp119 + (fTemp114 - (iTemp115) as F64) * (fTemp117 - (fTemp118 + fTemp78 * (fTemp119 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp116, 4), 196607))) as usize] } - fTemp117)))))}) * self.fVec29[1] / (fTemp63 * (1.0 - if iTemp64 != 0 {fTemp111 + fTemp106 * fTemp112 + (fTemp107 - (iTemp108) as F64) * (fTemp110 - (fTemp111 + fTemp106 * (fTemp112 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp109, 4), 196607))) as usize] } - fTemp110))))} else {1.0 - (fTemp104 + fTemp106 * fTemp105 + (fTemp100 - (iTemp101) as F64) * (fTemp103 - (fTemp104 + fTemp106 * (fTemp105 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp102, 4), 196607))) as usize] } - fTemp103)))))}));
			let mut iTemp140: i32 = (fTemp139 > ((if iTemp64 != 0 {fTemp95 + fTemp78 * fTemp96 + (fTemp91 - (iTemp92) as F64) * (fTemp94 - (fTemp95 + fTemp78 * (fTemp96 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp93, 4), 196607))) as usize] } - fTemp94))))} else {1.0 - (fTemp89 + fTemp78 * fTemp90 + (fTemp85 - (iTemp86) as F64) * (fTemp88 - (fTemp89 + fTemp78 * (fTemp90 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp87, 4)) as usize] } - fTemp88)))))} - fTemp80) / (1.0 - fTemp80))) as i32;
			let mut fTemp141: F64 = if iTemp140 != 0 {1.0} else {0.5};
			let mut fTemp142: F64 = if iTemp140 != 0 {0.5} else {0.0};
			let mut fTemp143: F64 = fTemp142 + fTemp141;
			let mut fTemp144: F64 = 0.5 * fTemp143;
			let mut fTemp145: F64 = 65535.0 * (1.0 - fTemp144);
			let mut iTemp146: i32 = (fTemp145) as i32;
			let mut iTemp147: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp146, 65535)))), 196607));
			let mut fTemp148: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp147, 3)) as usize] };
			let mut fTemp149: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp147 as usize] };
			let mut fTemp150: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp147, 1)) as usize] } - fTemp149;
			let mut fTemp151: F64 = 32767.5 * fTemp143;
			let mut iTemp152: i32 = (fTemp151) as i32;
			let mut iTemp153: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp152, 65535)))), 196607));
			let mut fTemp154: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp153, 3)) as usize] };
			let mut fTemp155: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp153 as usize] };
			let mut fTemp156: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp153, 1)) as usize] } - fTemp155;
			let mut fTemp157: F64 = if iTemp64 != 0 {fTemp155 + fTemp78 * fTemp156 + (fTemp151 - (iTemp152) as F64) * (fTemp154 - (fTemp155 + fTemp78 * (fTemp156 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp153, 4)) as usize] } - fTemp154))))} else {1.0 - (fTemp149 + fTemp78 * fTemp150 + (fTemp145 - (iTemp146) as F64) * (fTemp148 - (fTemp149 + fTemp78 * (fTemp150 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp147, 4)) as usize] } - fTemp148)))))};
			let mut fTemp158: F64 = fTemp83 + fTemp144;
			let mut fTemp159: F64 = 65535.0 * (1.0 - fTemp158);
			let mut iTemp160: i32 = (fTemp159) as i32;
			let mut iTemp161: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp160, 65535)))), 196607));
			let mut fTemp162: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp161, 3)) as usize] };
			let mut fTemp163: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp161 as usize] };
			let mut fTemp164: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp161, 1)) as usize] } - fTemp163;
			let mut fTemp165: F64 = 65535.0 * fTemp158;
			let mut iTemp166: i32 = (fTemp165) as i32;
			let mut iTemp167: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp166, 65535)))), 196607));
			let mut fTemp168: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp167, 3), 196607))) as usize] };
			let mut fTemp169: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp167 as usize] };
			let mut fTemp170: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp167, 1), 196607))) as usize] } - fTemp169;
			let mut iTemp171: i32 = (fTemp139 > ((if iTemp64 != 0 {fTemp169 + fTemp78 * fTemp170 + (fTemp165 - (iTemp166) as F64) * (fTemp168 - (fTemp169 + fTemp78 * (fTemp170 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp167, 4), 196607))) as usize] } - fTemp168))))} else {1.0 - (fTemp163 + fTemp78 * fTemp164 + (fTemp159 - (iTemp160) as F64) * (fTemp162 - (fTemp163 + fTemp78 * (fTemp164 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp161, 4)) as usize] } - fTemp162)))))} - fTemp157) / (1.0 - fTemp157))) as i32;
			let mut fTemp172: F64 = if iTemp171 != 0 {fTemp141} else {fTemp144};
			let mut fTemp173: F64 = if iTemp171 != 0 {fTemp144} else {fTemp142};
			let mut fTemp174: F64 = fTemp173 + fTemp172;
			let mut fTemp175: F64 = 0.5 * fTemp174;
			let mut fTemp176: F64 = 65535.0 * (1.0 - fTemp175);
			let mut iTemp177: i32 = (fTemp176) as i32;
			let mut iTemp178: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp177, 65535)))), 196607));
			let mut fTemp179: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp178, 3)) as usize] };
			let mut fTemp180: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp178 as usize] };
			let mut fTemp181: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp178, 1)) as usize] } - fTemp180;
			let mut fTemp182: F64 = 32767.5 * fTemp174;
			let mut iTemp183: i32 = (fTemp182) as i32;
			let mut iTemp184: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp183, 65535)))), 196607));
			let mut fTemp185: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp184, 3)) as usize] };
			let mut fTemp186: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp184 as usize] };
			let mut fTemp187: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp184, 1)) as usize] } - fTemp186;
			let mut fTemp188: F64 = if iTemp64 != 0 {fTemp186 + fTemp78 * fTemp187 + (fTemp182 - (iTemp183) as F64) * (fTemp185 - (fTemp186 + fTemp78 * (fTemp187 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp184, 4)) as usize] } - fTemp185))))} else {1.0 - (fTemp180 + fTemp78 * fTemp181 + (fTemp176 - (iTemp177) as F64) * (fTemp179 - (fTemp180 + fTemp78 * (fTemp181 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp178, 4)) as usize] } - fTemp179)))))};
			let mut fTemp189: F64 = fTemp83 + fTemp175;
			let mut fTemp190: F64 = 65535.0 * (1.0 - fTemp189);
			let mut iTemp191: i32 = (fTemp190) as i32;
			let mut iTemp192: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp191, 65535)))), 196607));
			let mut fTemp193: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp192, 3)) as usize] };
			let mut fTemp194: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp192 as usize] };
			let mut fTemp195: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp192, 1)) as usize] } - fTemp194;
			let mut fTemp196: F64 = 65535.0 * fTemp189;
			let mut iTemp197: i32 = (fTemp196) as i32;
			let mut iTemp198: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp197, 65535)))), 196607));
			let mut fTemp199: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp198, 3), 196607))) as usize] };
			let mut fTemp200: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp198 as usize] };
			let mut fTemp201: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp198, 1), 196607))) as usize] } - fTemp200;
			let mut iTemp202: i32 = (fTemp139 > ((if iTemp64 != 0 {fTemp200 + fTemp78 * fTemp201 + (fTemp196 - (iTemp197) as F64) * (fTemp199 - (fTemp200 + fTemp78 * (fTemp201 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp198, 4), 196607))) as usize] } - fTemp199))))} else {1.0 - (fTemp194 + fTemp78 * fTemp195 + (fTemp190 - (iTemp191) as F64) * (fTemp193 - (fTemp194 + fTemp78 * (fTemp195 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp192, 4)) as usize] } - fTemp193)))))} - fTemp188) / (1.0 - fTemp188))) as i32;
			let mut fTemp203: F64 = if iTemp202 != 0 {fTemp172} else {fTemp175};
			let mut fTemp204: F64 = if iTemp202 != 0 {fTemp175} else {fTemp173};
			let mut fTemp205: F64 = fTemp204 + fTemp203;
			let mut fTemp206: F64 = 0.5 * fTemp205;
			let mut fTemp207: F64 = 65535.0 * (1.0 - fTemp206);
			let mut iTemp208: i32 = (fTemp207) as i32;
			let mut iTemp209: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp208, 65535)))), 196607));
			let mut fTemp210: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp209, 3)) as usize] };
			let mut fTemp211: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp209 as usize] };
			let mut fTemp212: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp209, 1)) as usize] } - fTemp211;
			let mut fTemp213: F64 = 32767.5 * fTemp205;
			let mut iTemp214: i32 = (fTemp213) as i32;
			let mut iTemp215: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp214, 65535)))), 196607));
			let mut fTemp216: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp215, 3)) as usize] };
			let mut fTemp217: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp215 as usize] };
			let mut fTemp218: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp215, 1)) as usize] } - fTemp217;
			let mut fTemp219: F64 = if iTemp64 != 0 {fTemp217 + fTemp78 * fTemp218 + (fTemp213 - (iTemp214) as F64) * (fTemp216 - (fTemp217 + fTemp78 * (fTemp218 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp215, 4)) as usize] } - fTemp216))))} else {1.0 - (fTemp211 + fTemp78 * fTemp212 + (fTemp207 - (iTemp208) as F64) * (fTemp210 - (fTemp211 + fTemp78 * (fTemp212 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp209, 4)) as usize] } - fTemp210)))))};
			let mut fTemp220: F64 = fTemp83 + fTemp206;
			let mut fTemp221: F64 = 65535.0 * (1.0 - fTemp220);
			let mut iTemp222: i32 = (fTemp221) as i32;
			let mut iTemp223: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp222, 65535)))), 196607));
			let mut fTemp224: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp223, 3)) as usize] };
			let mut fTemp225: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp223 as usize] };
			let mut fTemp226: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp223, 1)) as usize] } - fTemp225;
			let mut fTemp227: F64 = 65535.0 * fTemp220;
			let mut iTemp228: i32 = (fTemp227) as i32;
			let mut iTemp229: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp228, 65535)))), 196607));
			let mut fTemp230: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp229, 3), 196607))) as usize] };
			let mut fTemp231: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp229 as usize] };
			let mut fTemp232: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp229, 1), 196607))) as usize] } - fTemp231;
			let mut iTemp233: i32 = (fTemp139 > ((if iTemp64 != 0 {fTemp231 + fTemp78 * fTemp232 + (fTemp227 - (iTemp228) as F64) * (fTemp230 - (fTemp231 + fTemp78 * (fTemp232 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp229, 4), 196607))) as usize] } - fTemp230))))} else {1.0 - (fTemp225 + fTemp78 * fTemp226 + (fTemp221 - (iTemp222) as F64) * (fTemp224 - (fTemp225 + fTemp78 * (fTemp226 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp223, 4)) as usize] } - fTemp224)))))} - fTemp219) / (1.0 - fTemp219))) as i32;
			let mut fTemp234: F64 = if iTemp233 != 0 {fTemp203} else {fTemp206};
			let mut fTemp235: F64 = if iTemp233 != 0 {fTemp206} else {fTemp204};
			let mut fTemp236: F64 = fTemp235 + fTemp234;
			let mut fTemp237: F64 = 0.5 * fTemp236;
			let mut fTemp238: F64 = 65535.0 * (1.0 - fTemp237);
			let mut iTemp239: i32 = (fTemp238) as i32;
			let mut iTemp240: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp239, 65535)))), 196607));
			let mut fTemp241: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp240, 3)) as usize] };
			let mut fTemp242: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp240 as usize] };
			let mut fTemp243: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp240, 1)) as usize] } - fTemp242;
			let mut fTemp244: F64 = 32767.5 * fTemp236;
			let mut iTemp245: i32 = (fTemp244) as i32;
			let mut iTemp246: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp245, 65535)))), 196607));
			let mut fTemp247: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp246, 3)) as usize] };
			let mut fTemp248: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp246 as usize] };
			let mut fTemp249: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp246, 1)) as usize] } - fTemp248;
			let mut fTemp250: F64 = if iTemp64 != 0 {fTemp248 + fTemp78 * fTemp249 + (fTemp244 - (iTemp245) as F64) * (fTemp247 - (fTemp248 + fTemp78 * (fTemp249 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp246, 4)) as usize] } - fTemp247))))} else {1.0 - (fTemp242 + fTemp78 * fTemp243 + (fTemp238 - (iTemp239) as F64) * (fTemp241 - (fTemp242 + fTemp78 * (fTemp243 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp240, 4)) as usize] } - fTemp241)))))};
			let mut fTemp251: F64 = fTemp83 + fTemp237;
			let mut fTemp252: F64 = 65535.0 * (1.0 - fTemp251);
			let mut iTemp253: i32 = (fTemp252) as i32;
			let mut iTemp254: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp253, 65535)))), 196607));
			let mut fTemp255: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp254, 3)) as usize] };
			let mut fTemp256: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp254 as usize] };
			let mut fTemp257: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp254, 1)) as usize] } - fTemp256;
			let mut fTemp258: F64 = 65535.0 * fTemp251;
			let mut iTemp259: i32 = (fTemp258) as i32;
			let mut iTemp260: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp259, 65535)))), 196607));
			let mut fTemp261: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp260, 3), 196607))) as usize] };
			let mut fTemp262: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp260 as usize] };
			let mut fTemp263: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp260, 1), 196607))) as usize] } - fTemp262;
			let mut iTemp264: i32 = (fTemp139 > ((if iTemp64 != 0 {fTemp262 + fTemp78 * fTemp263 + (fTemp258 - (iTemp259) as F64) * (fTemp261 - (fTemp262 + fTemp78 * (fTemp263 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp260, 4), 196607))) as usize] } - fTemp261))))} else {1.0 - (fTemp256 + fTemp78 * fTemp257 + (fTemp252 - (iTemp253) as F64) * (fTemp255 - (fTemp256 + fTemp78 * (fTemp257 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp254, 4)) as usize] } - fTemp255)))))} - fTemp250) / (1.0 - fTemp250))) as i32;
			let mut fTemp265: F64 = if iTemp264 != 0 {fTemp234} else {fTemp237};
			let mut fTemp266: F64 = if iTemp264 != 0 {fTemp237} else {fTemp235};
			let mut fTemp267: F64 = fTemp266 + fTemp265;
			let mut fTemp268: F64 = 0.5 * fTemp267;
			let mut fTemp269: F64 = 65535.0 * (1.0 - fTemp268);
			let mut iTemp270: i32 = (fTemp269) as i32;
			let mut iTemp271: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp270, 65535)))), 196607));
			let mut fTemp272: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp271, 3)) as usize] };
			let mut fTemp273: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp271 as usize] };
			let mut fTemp274: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp271, 1)) as usize] } - fTemp273;
			let mut fTemp275: F64 = 32767.5 * fTemp267;
			let mut iTemp276: i32 = (fTemp275) as i32;
			let mut iTemp277: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp276, 65535)))), 196607));
			let mut fTemp278: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp277, 3)) as usize] };
			let mut fTemp279: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp277 as usize] };
			let mut fTemp280: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp277, 1)) as usize] } - fTemp279;
			let mut fTemp281: F64 = if iTemp64 != 0 {fTemp279 + fTemp78 * fTemp280 + (fTemp275 - (iTemp276) as F64) * (fTemp278 - (fTemp279 + fTemp78 * (fTemp280 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp277, 4)) as usize] } - fTemp278))))} else {1.0 - (fTemp273 + fTemp78 * fTemp274 + (fTemp269 - (iTemp270) as F64) * (fTemp272 - (fTemp273 + fTemp78 * (fTemp274 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp271, 4)) as usize] } - fTemp272)))))};
			let mut fTemp282: F64 = fTemp83 + fTemp268;
			let mut fTemp283: F64 = 65535.0 * (1.0 - fTemp282);
			let mut iTemp284: i32 = (fTemp283) as i32;
			let mut iTemp285: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp284, 65535)))), 196607));
			let mut fTemp286: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp285, 3)) as usize] };
			let mut fTemp287: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp285 as usize] };
			let mut fTemp288: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp285, 1)) as usize] } - fTemp287;
			let mut fTemp289: F64 = 65535.0 * fTemp282;
			let mut iTemp290: i32 = (fTemp289) as i32;
			let mut iTemp291: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp290, 65535)))), 196607));
			let mut fTemp292: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp291, 3), 196607))) as usize] };
			let mut fTemp293: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp291 as usize] };
			let mut fTemp294: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp291, 1), 196607))) as usize] } - fTemp293;
			let mut iTemp295: i32 = (fTemp139 > ((if iTemp64 != 0 {fTemp293 + fTemp78 * fTemp294 + (fTemp289 - (iTemp290) as F64) * (fTemp292 - (fTemp293 + fTemp78 * (fTemp294 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp291, 4), 196607))) as usize] } - fTemp292))))} else {1.0 - (fTemp287 + fTemp78 * fTemp288 + (fTemp283 - (iTemp284) as F64) * (fTemp286 - (fTemp287 + fTemp78 * (fTemp288 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp285, 4)) as usize] } - fTemp286)))))} - fTemp281) / (1.0 - fTemp281))) as i32;
			let mut fTemp296: F64 = if iTemp295 != 0 {fTemp265} else {fTemp268};
			let mut fTemp297: F64 = if iTemp295 != 0 {fTemp268} else {fTemp266};
			let mut fTemp298: F64 = fTemp297 + fTemp296;
			let mut fTemp299: F64 = 0.5 * fTemp298;
			let mut fTemp300: F64 = 65535.0 * (1.0 - fTemp299);
			let mut iTemp301: i32 = (fTemp300) as i32;
			let mut iTemp302: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp301, 65535)))), 196607));
			let mut fTemp303: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp302, 3)) as usize] };
			let mut fTemp304: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp302 as usize] };
			let mut fTemp305: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp302, 1)) as usize] } - fTemp304;
			let mut fTemp306: F64 = 32767.5 * fTemp298;
			let mut iTemp307: i32 = (fTemp306) as i32;
			let mut iTemp308: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp307, 65535)))), 196607));
			let mut fTemp309: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp308, 3)) as usize] };
			let mut fTemp310: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp308 as usize] };
			let mut fTemp311: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp308, 1)) as usize] } - fTemp310;
			let mut fTemp312: F64 = if iTemp64 != 0 {fTemp310 + fTemp78 * fTemp311 + (fTemp306 - (iTemp307) as F64) * (fTemp309 - (fTemp310 + fTemp78 * (fTemp311 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp308, 4)) as usize] } - fTemp309))))} else {1.0 - (fTemp304 + fTemp78 * fTemp305 + (fTemp300 - (iTemp301) as F64) * (fTemp303 - (fTemp304 + fTemp78 * (fTemp305 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp302, 4)) as usize] } - fTemp303)))))};
			let mut fTemp313: F64 = fTemp83 + fTemp299;
			let mut fTemp314: F64 = 65535.0 * (1.0 - fTemp313);
			let mut iTemp315: i32 = (fTemp314) as i32;
			let mut iTemp316: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp315, 65535)))), 196607));
			let mut fTemp317: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp316, 3)) as usize] };
			let mut fTemp318: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp316 as usize] };
			let mut fTemp319: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp316, 1)) as usize] } - fTemp318;
			let mut fTemp320: F64 = 65535.0 * fTemp313;
			let mut iTemp321: i32 = (fTemp320) as i32;
			let mut iTemp322: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp321, 65535)))), 196607));
			let mut fTemp323: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp322, 3), 196607))) as usize] };
			let mut fTemp324: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp322 as usize] };
			let mut fTemp325: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp322, 1), 196607))) as usize] } - fTemp324;
			let mut iTemp326: i32 = (fTemp139 > ((if iTemp64 != 0 {fTemp324 + fTemp78 * fTemp325 + (fTemp320 - (iTemp321) as F64) * (fTemp323 - (fTemp324 + fTemp78 * (fTemp325 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp322, 4), 196607))) as usize] } - fTemp323))))} else {1.0 - (fTemp318 + fTemp78 * fTemp319 + (fTemp314 - (iTemp315) as F64) * (fTemp317 - (fTemp318 + fTemp78 * (fTemp319 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp316, 4)) as usize] } - fTemp317)))))} - fTemp312) / (1.0 - fTemp312))) as i32;
			let mut fTemp327: F64 = if iTemp326 != 0 {fTemp296} else {fTemp299};
			let mut fTemp328: F64 = if iTemp326 != 0 {fTemp299} else {fTemp297};
			let mut fTemp329: F64 = fTemp328 + fTemp327;
			let mut fTemp330: F64 = 0.5 * fTemp329;
			let mut fTemp331: F64 = 65535.0 * (1.0 - fTemp330);
			let mut iTemp332: i32 = (fTemp331) as i32;
			let mut iTemp333: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp332, 65535)))), 196607));
			let mut fTemp334: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp333, 3)) as usize] };
			let mut fTemp335: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp333 as usize] };
			let mut fTemp336: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp333, 1)) as usize] } - fTemp335;
			let mut fTemp337: F64 = 32767.5 * fTemp329;
			let mut iTemp338: i32 = (fTemp337) as i32;
			let mut iTemp339: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp338, 65535)))), 196607));
			let mut fTemp340: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp339, 3)) as usize] };
			let mut fTemp341: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp339 as usize] };
			let mut fTemp342: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp339, 1)) as usize] } - fTemp341;
			let mut fTemp343: F64 = if iTemp64 != 0 {fTemp341 + fTemp78 * fTemp342 + (fTemp337 - (iTemp338) as F64) * (fTemp340 - (fTemp341 + fTemp78 * (fTemp342 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp339, 4)) as usize] } - fTemp340))))} else {1.0 - (fTemp335 + fTemp78 * fTemp336 + (fTemp331 - (iTemp332) as F64) * (fTemp334 - (fTemp335 + fTemp78 * (fTemp336 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp333, 4)) as usize] } - fTemp334)))))};
			let mut fTemp344: F64 = fTemp83 + fTemp330;
			let mut fTemp345: F64 = 65535.0 * (1.0 - fTemp344);
			let mut iTemp346: i32 = (fTemp345) as i32;
			let mut iTemp347: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp346, 65535)))), 196607));
			let mut fTemp348: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp347, 3)) as usize] };
			let mut fTemp349: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp347 as usize] };
			let mut fTemp350: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp347, 1)) as usize] } - fTemp349;
			let mut fTemp351: F64 = 65535.0 * fTemp344;
			let mut iTemp352: i32 = (fTemp351) as i32;
			let mut iTemp353: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp352, 65535)))), 196607));
			let mut fTemp354: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp353, 3), 196607))) as usize] };
			let mut fTemp355: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp353 as usize] };
			let mut fTemp356: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp353, 1), 196607))) as usize] } - fTemp355;
			let mut iTemp357: i32 = (fTemp139 > ((if iTemp64 != 0 {fTemp355 + fTemp78 * fTemp356 + (fTemp351 - (iTemp352) as F64) * (fTemp354 - (fTemp355 + fTemp78 * (fTemp356 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp353, 4), 196607))) as usize] } - fTemp354))))} else {1.0 - (fTemp349 + fTemp78 * fTemp350 + (fTemp345 - (iTemp346) as F64) * (fTemp348 - (fTemp349 + fTemp78 * (fTemp350 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp347, 4)) as usize] } - fTemp348)))))} - fTemp343) / (1.0 - fTemp343))) as i32;
			let mut fTemp358: F64 = if iTemp357 != 0 {fTemp327} else {fTemp330};
			let mut fTemp359: F64 = if iTemp357 != 0 {fTemp330} else {fTemp328};
			let mut fTemp360: F64 = fTemp359 + fTemp358;
			let mut fTemp361: F64 = 0.5 * fTemp360;
			let mut fTemp362: F64 = 65535.0 * (1.0 - fTemp361);
			let mut iTemp363: i32 = (fTemp362) as i32;
			let mut iTemp364: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp363, 65535)))), 196607));
			let mut fTemp365: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp364, 3)) as usize] };
			let mut fTemp366: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp364 as usize] };
			let mut fTemp367: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp364, 1)) as usize] } - fTemp366;
			let mut fTemp368: F64 = 32767.5 * fTemp360;
			let mut iTemp369: i32 = (fTemp368) as i32;
			let mut iTemp370: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp369, 65535)))), 196607));
			let mut fTemp371: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp370, 3)) as usize] };
			let mut fTemp372: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp370 as usize] };
			let mut fTemp373: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp370, 1)) as usize] } - fTemp372;
			let mut fTemp374: F64 = if iTemp64 != 0 {fTemp372 + fTemp78 * fTemp373 + (fTemp368 - (iTemp369) as F64) * (fTemp371 - (fTemp372 + fTemp78 * (fTemp373 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp370, 4)) as usize] } - fTemp371))))} else {1.0 - (fTemp366 + fTemp78 * fTemp367 + (fTemp362 - (iTemp363) as F64) * (fTemp365 - (fTemp366 + fTemp78 * (fTemp367 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp364, 4)) as usize] } - fTemp365)))))};
			let mut fTemp375: F64 = fTemp83 + fTemp361;
			let mut fTemp376: F64 = 65535.0 * (1.0 - fTemp375);
			let mut iTemp377: i32 = (fTemp376) as i32;
			let mut iTemp378: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp377, 65535)))), 196607));
			let mut fTemp379: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp378, 3)) as usize] };
			let mut fTemp380: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp378 as usize] };
			let mut fTemp381: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp378, 1)) as usize] } - fTemp380;
			let mut fTemp382: F64 = 65535.0 * fTemp375;
			let mut iTemp383: i32 = (fTemp382) as i32;
			let mut iTemp384: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp383, 65535)))), 196607));
			let mut fTemp385: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp384, 3), 196607))) as usize] };
			let mut fTemp386: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp384 as usize] };
			let mut fTemp387: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp384, 1), 196607))) as usize] } - fTemp386;
			let mut iTemp388: i32 = (fTemp139 > ((if iTemp64 != 0 {fTemp386 + fTemp78 * fTemp387 + (fTemp382 - (iTemp383) as F64) * (fTemp385 - (fTemp386 + fTemp78 * (fTemp387 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp384, 4), 196607))) as usize] } - fTemp385))))} else {1.0 - (fTemp380 + fTemp78 * fTemp381 + (fTemp376 - (iTemp377) as F64) * (fTemp379 - (fTemp380 + fTemp78 * (fTemp381 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp378, 4)) as usize] } - fTemp379)))))} - fTemp374) / (1.0 - fTemp374))) as i32;
			let mut fTemp389: F64 = if iTemp388 != 0 {fTemp358} else {fTemp361};
			let mut fTemp390: F64 = if iTemp388 != 0 {fTemp361} else {fTemp359};
			let mut fTemp391: F64 = fTemp390 + fTemp389;
			let mut fTemp392: F64 = 0.5 * fTemp391;
			let mut fTemp393: F64 = 65535.0 * (1.0 - fTemp392);
			let mut iTemp394: i32 = (fTemp393) as i32;
			let mut iTemp395: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp394, 65535)))), 196607));
			let mut fTemp396: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp395, 3)) as usize] };
			let mut fTemp397: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp395 as usize] };
			let mut fTemp398: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp395, 1)) as usize] } - fTemp397;
			let mut fTemp399: F64 = 32767.5 * fTemp391;
			let mut iTemp400: i32 = (fTemp399) as i32;
			let mut iTemp401: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp400, 65535)))), 196607));
			let mut fTemp402: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp401, 3)) as usize] };
			let mut fTemp403: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp401 as usize] };
			let mut fTemp404: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp401, 1)) as usize] } - fTemp403;
			let mut fTemp405: F64 = if iTemp64 != 0 {fTemp403 + fTemp78 * fTemp404 + (fTemp399 - (iTemp400) as F64) * (fTemp402 - (fTemp403 + fTemp78 * (fTemp404 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp401, 4)) as usize] } - fTemp402))))} else {1.0 - (fTemp397 + fTemp78 * fTemp398 + (fTemp393 - (iTemp394) as F64) * (fTemp396 - (fTemp397 + fTemp78 * (fTemp398 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp395, 4)) as usize] } - fTemp396)))))};
			let mut fTemp406: F64 = fTemp83 + fTemp392;
			let mut fTemp407: F64 = 65535.0 * (1.0 - fTemp406);
			let mut iTemp408: i32 = (fTemp407) as i32;
			let mut iTemp409: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp408, 65535)))), 196607));
			let mut fTemp410: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp409, 3)) as usize] };
			let mut fTemp411: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp409 as usize] };
			let mut fTemp412: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp409, 1)) as usize] } - fTemp411;
			let mut fTemp413: F64 = 65535.0 * fTemp406;
			let mut iTemp414: i32 = (fTemp413) as i32;
			let mut iTemp415: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp414, 65535)))), 196607));
			let mut fTemp416: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp415, 3), 196607))) as usize] };
			let mut fTemp417: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp415 as usize] };
			let mut fTemp418: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp415, 1), 196607))) as usize] } - fTemp417;
			let mut iTemp419: i32 = (fTemp139 > ((if iTemp64 != 0 {fTemp417 + fTemp78 * fTemp418 + (fTemp413 - (iTemp414) as F64) * (fTemp416 - (fTemp417 + fTemp78 * (fTemp418 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp415, 4), 196607))) as usize] } - fTemp416))))} else {1.0 - (fTemp411 + fTemp78 * fTemp412 + (fTemp407 - (iTemp408) as F64) * (fTemp410 - (fTemp411 + fTemp78 * (fTemp412 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp409, 4)) as usize] } - fTemp410)))))} - fTemp405) / (1.0 - fTemp405))) as i32;
			let mut fTemp420: F64 = if iTemp419 != 0 {fTemp389} else {fTemp392};
			let mut fTemp421: F64 = if iTemp419 != 0 {fTemp392} else {fTemp390};
			let mut fTemp422: F64 = fTemp421 + fTemp420;
			let mut fTemp423: F64 = 0.5 * fTemp422;
			let mut fTemp424: F64 = 65535.0 * (1.0 - fTemp423);
			let mut iTemp425: i32 = (fTemp424) as i32;
			let mut iTemp426: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp425, 65535)))), 196607));
			let mut fTemp427: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp426, 3)) as usize] };
			let mut fTemp428: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp426 as usize] };
			let mut fTemp429: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp426, 1)) as usize] } - fTemp428;
			let mut fTemp430: F64 = 32767.5 * fTemp422;
			let mut iTemp431: i32 = (fTemp430) as i32;
			let mut iTemp432: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp431, 65535)))), 196607));
			let mut fTemp433: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp432, 3)) as usize] };
			let mut fTemp434: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp432 as usize] };
			let mut fTemp435: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp432, 1)) as usize] } - fTemp434;
			let mut fTemp436: F64 = if iTemp64 != 0 {fTemp434 + fTemp78 * fTemp435 + (fTemp430 - (iTemp431) as F64) * (fTemp433 - (fTemp434 + fTemp78 * (fTemp435 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp432, 4)) as usize] } - fTemp433))))} else {1.0 - (fTemp428 + fTemp78 * fTemp429 + (fTemp424 - (iTemp425) as F64) * (fTemp427 - (fTemp428 + fTemp78 * (fTemp429 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp426, 4)) as usize] } - fTemp427)))))};
			let mut fTemp437: F64 = fTemp83 + fTemp423;
			let mut fTemp438: F64 = 65535.0 * (1.0 - fTemp437);
			let mut iTemp439: i32 = (fTemp438) as i32;
			let mut iTemp440: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp439, 65535)))), 196607));
			let mut fTemp441: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp440, 3)) as usize] };
			let mut fTemp442: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp440 as usize] };
			let mut fTemp443: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp440, 1)) as usize] } - fTemp442;
			let mut fTemp444: F64 = 65535.0 * fTemp437;
			let mut iTemp445: i32 = (fTemp444) as i32;
			let mut iTemp446: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp445, 65535)))), 196607));
			let mut fTemp447: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp446, 3), 196607))) as usize] };
			let mut fTemp448: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp446 as usize] };
			let mut fTemp449: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp446, 1), 196607))) as usize] } - fTemp448;
			let mut iTemp450: i32 = (fTemp139 > ((if iTemp64 != 0 {fTemp448 + fTemp78 * fTemp449 + (fTemp444 - (iTemp445) as F64) * (fTemp447 - (fTemp448 + fTemp78 * (fTemp449 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp446, 4), 196607))) as usize] } - fTemp447))))} else {1.0 - (fTemp442 + fTemp78 * fTemp443 + (fTemp438 - (iTemp439) as F64) * (fTemp441 - (fTemp442 + fTemp78 * (fTemp443 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp440, 4)) as usize] } - fTemp441)))))} - fTemp436) / (1.0 - fTemp436))) as i32;
			let mut fTemp451: F64 = if iTemp450 != 0 {fTemp420} else {fTemp423};
			let mut fTemp452: F64 = if iTemp450 != 0 {fTemp423} else {fTemp421};
			let mut fTemp453: F64 = fTemp452 + fTemp451;
			let mut fTemp454: F64 = 0.5 * fTemp453;
			let mut fTemp455: F64 = 65535.0 * (1.0 - fTemp454);
			let mut iTemp456: i32 = (fTemp455) as i32;
			let mut iTemp457: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp456, 65535)))), 196607));
			let mut fTemp458: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp457, 3)) as usize] };
			let mut fTemp459: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp457 as usize] };
			let mut fTemp460: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp457, 1)) as usize] } - fTemp459;
			let mut fTemp461: F64 = 32767.5 * fTemp453;
			let mut iTemp462: i32 = (fTemp461) as i32;
			let mut iTemp463: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp462, 65535)))), 196607));
			let mut fTemp464: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp463, 3)) as usize] };
			let mut fTemp465: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp463 as usize] };
			let mut fTemp466: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp463, 1)) as usize] } - fTemp465;
			let mut fTemp467: F64 = if iTemp64 != 0 {fTemp465 + fTemp78 * fTemp466 + (fTemp461 - (iTemp462) as F64) * (fTemp464 - (fTemp465 + fTemp78 * (fTemp466 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp463, 4)) as usize] } - fTemp464))))} else {1.0 - (fTemp459 + fTemp78 * fTemp460 + (fTemp455 - (iTemp456) as F64) * (fTemp458 - (fTemp459 + fTemp78 * (fTemp460 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp457, 4)) as usize] } - fTemp458)))))};
			let mut fTemp468: F64 = fTemp83 + fTemp454;
			let mut fTemp469: F64 = 65535.0 * (1.0 - fTemp468);
			let mut iTemp470: i32 = (fTemp469) as i32;
			let mut iTemp471: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp470, 65535)))), 196607));
			let mut fTemp472: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp471, 3)) as usize] };
			let mut fTemp473: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp471 as usize] };
			let mut fTemp474: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp471, 1)) as usize] } - fTemp473;
			let mut fTemp475: F64 = 65535.0 * fTemp468;
			let mut iTemp476: i32 = (fTemp475) as i32;
			let mut iTemp477: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp476, 65535)))), 196607));
			let mut fTemp478: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp477, 3), 196607))) as usize] };
			let mut fTemp479: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp477 as usize] };
			let mut fTemp480: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp477, 1), 196607))) as usize] } - fTemp479;
			let mut iTemp481: i32 = (fTemp139 > ((if iTemp64 != 0 {fTemp479 + fTemp78 * fTemp480 + (fTemp475 - (iTemp476) as F64) * (fTemp478 - (fTemp479 + fTemp78 * (fTemp480 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp477, 4), 196607))) as usize] } - fTemp478))))} else {1.0 - (fTemp473 + fTemp78 * fTemp474 + (fTemp469 - (iTemp470) as F64) * (fTemp472 - (fTemp473 + fTemp78 * (fTemp474 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp471, 4)) as usize] } - fTemp472)))))} - fTemp467) / (1.0 - fTemp467))) as i32;
			let mut fTemp482: F64 = if iTemp481 != 0 {fTemp451} else {fTemp454};
			let mut fTemp483: F64 = if iTemp481 != 0 {fTemp454} else {fTemp452};
			let mut fTemp484: F64 = fTemp483 + fTemp482;
			let mut fTemp485: F64 = 0.5 * fTemp484;
			let mut fTemp486: F64 = 65535.0 * (1.0 - fTemp485);
			let mut iTemp487: i32 = (fTemp486) as i32;
			let mut iTemp488: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp487, 65535)))), 196607));
			let mut fTemp489: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp488, 3)) as usize] };
			let mut fTemp490: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp488 as usize] };
			let mut fTemp491: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp488, 1)) as usize] } - fTemp490;
			let mut fTemp492: F64 = 32767.5 * fTemp484;
			let mut iTemp493: i32 = (fTemp492) as i32;
			let mut iTemp494: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp493, 65535)))), 196607));
			let mut fTemp495: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp494, 3)) as usize] };
			let mut fTemp496: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp494 as usize] };
			let mut fTemp497: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp494, 1)) as usize] } - fTemp496;
			let mut fTemp498: F64 = if iTemp64 != 0 {fTemp496 + fTemp78 * fTemp497 + (fTemp492 - (iTemp493) as F64) * (fTemp495 - (fTemp496 + fTemp78 * (fTemp497 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp494, 4)) as usize] } - fTemp495))))} else {1.0 - (fTemp490 + fTemp78 * fTemp491 + (fTemp486 - (iTemp487) as F64) * (fTemp489 - (fTemp490 + fTemp78 * (fTemp491 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp488, 4)) as usize] } - fTemp489)))))};
			let mut fTemp499: F64 = fTemp83 + fTemp485;
			let mut fTemp500: F64 = 65535.0 * (1.0 - fTemp499);
			let mut iTemp501: i32 = (fTemp500) as i32;
			let mut iTemp502: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp501, 65535)))), 196607));
			let mut fTemp503: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp502, 3)) as usize] };
			let mut fTemp504: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp502 as usize] };
			let mut fTemp505: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp502, 1)) as usize] } - fTemp504;
			let mut fTemp506: F64 = 65535.0 * fTemp499;
			let mut iTemp507: i32 = (fTemp506) as i32;
			let mut iTemp508: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp507, 65535)))), 196607));
			let mut fTemp509: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp508, 3), 196607))) as usize] };
			let mut fTemp510: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp508 as usize] };
			let mut fTemp511: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp508, 1), 196607))) as usize] } - fTemp510;
			let mut iTemp512: i32 = (fTemp139 > ((if iTemp64 != 0 {fTemp510 + fTemp78 * fTemp511 + (fTemp506 - (iTemp507) as F64) * (fTemp509 - (fTemp510 + fTemp78 * (fTemp511 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp508, 4), 196607))) as usize] } - fTemp509))))} else {1.0 - (fTemp504 + fTemp78 * fTemp505 + (fTemp500 - (iTemp501) as F64) * (fTemp503 - (fTemp504 + fTemp78 * (fTemp505 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp502, 4)) as usize] } - fTemp503)))))} - fTemp498) / (1.0 - fTemp498))) as i32;
			let mut fTemp513: F64 = if iTemp512 != 0 {fTemp482} else {fTemp485};
			let mut fTemp514: F64 = if iTemp512 != 0 {fTemp485} else {fTemp483};
			let mut fTemp515: F64 = fTemp514 + fTemp513;
			let mut fTemp516: F64 = 0.5 * fTemp515;
			let mut fTemp517: F64 = 65535.0 * (1.0 - fTemp516);
			let mut iTemp518: i32 = (fTemp517) as i32;
			let mut iTemp519: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp518, 65535)))), 196607));
			let mut fTemp520: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp519, 3)) as usize] };
			let mut fTemp521: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp519 as usize] };
			let mut fTemp522: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp519, 1)) as usize] } - fTemp521;
			let mut fTemp523: F64 = 32767.5 * fTemp515;
			let mut iTemp524: i32 = (fTemp523) as i32;
			let mut iTemp525: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp524, 65535)))), 196607));
			let mut fTemp526: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp525, 3)) as usize] };
			let mut fTemp527: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp525 as usize] };
			let mut fTemp528: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp525, 1)) as usize] } - fTemp527;
			let mut fTemp529: F64 = if iTemp64 != 0 {fTemp527 + fTemp78 * fTemp528 + (fTemp523 - (iTemp524) as F64) * (fTemp526 - (fTemp527 + fTemp78 * (fTemp528 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp525, 4)) as usize] } - fTemp526))))} else {1.0 - (fTemp521 + fTemp78 * fTemp522 + (fTemp517 - (iTemp518) as F64) * (fTemp520 - (fTemp521 + fTemp78 * (fTemp522 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp519, 4)) as usize] } - fTemp520)))))};
			let mut fTemp530: F64 = fTemp83 + fTemp516;
			let mut fTemp531: F64 = 65535.0 * (1.0 - fTemp530);
			let mut iTemp532: i32 = (fTemp531) as i32;
			let mut iTemp533: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp532, 65535)))), 196607));
			let mut fTemp534: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp533, 3)) as usize] };
			let mut fTemp535: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp533 as usize] };
			let mut fTemp536: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp533, 1)) as usize] } - fTemp535;
			let mut fTemp537: F64 = 65535.0 * fTemp530;
			let mut iTemp538: i32 = (fTemp537) as i32;
			let mut iTemp539: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp538, 65535)))), 196607));
			let mut fTemp540: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp539, 3), 196607))) as usize] };
			let mut fTemp541: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp539 as usize] };
			let mut fTemp542: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp539, 1), 196607))) as usize] } - fTemp541;
			let mut iTemp543: i32 = (fTemp139 > ((if iTemp64 != 0 {fTemp541 + fTemp78 * fTemp542 + (fTemp537 - (iTemp538) as F64) * (fTemp540 - (fTemp541 + fTemp78 * (fTemp542 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp539, 4), 196607))) as usize] } - fTemp540))))} else {1.0 - (fTemp535 + fTemp78 * fTemp536 + (fTemp531 - (iTemp532) as F64) * (fTemp534 - (fTemp535 + fTemp78 * (fTemp536 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp533, 4)) as usize] } - fTemp534)))))} - fTemp529) / (1.0 - fTemp529))) as i32;
			let mut fTemp544: F64 = if iTemp543 != 0 {fTemp513} else {fTemp516};
			let mut fTemp545: F64 = if iTemp543 != 0 {fTemp516} else {fTemp514};
			let mut fTemp546: F64 = fTemp545 + fTemp544;
			let mut fTemp547: F64 = 0.5 * fTemp546;
			let mut fTemp548: F64 = 65535.0 * (1.0 - fTemp547);
			let mut iTemp549: i32 = (fTemp548) as i32;
			let mut iTemp550: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp549, 65535)))), 196607));
			let mut fTemp551: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp550, 3)) as usize] };
			let mut fTemp552: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp550 as usize] };
			let mut fTemp553: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp550, 1)) as usize] } - fTemp552;
			let mut fTemp554: F64 = 32767.5 * fTemp546;
			let mut iTemp555: i32 = (fTemp554) as i32;
			let mut iTemp556: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp555, 65535)))), 196607));
			let mut fTemp557: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp556, 3)) as usize] };
			let mut fTemp558: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp556 as usize] };
			let mut fTemp559: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp556, 1)) as usize] } - fTemp558;
			let mut fTemp560: F64 = if iTemp64 != 0 {fTemp558 + fTemp78 * fTemp559 + (fTemp554 - (iTemp555) as F64) * (fTemp557 - (fTemp558 + fTemp78 * (fTemp559 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp556, 4)) as usize] } - fTemp557))))} else {1.0 - (fTemp552 + fTemp78 * fTemp553 + (fTemp548 - (iTemp549) as F64) * (fTemp551 - (fTemp552 + fTemp78 * (fTemp553 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp550, 4)) as usize] } - fTemp551)))))};
			let mut fTemp561: F64 = fTemp83 + fTemp547;
			let mut fTemp562: F64 = 65535.0 * (1.0 - fTemp561);
			let mut iTemp563: i32 = (fTemp562) as i32;
			let mut iTemp564: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp563, 65535)))), 196607));
			let mut fTemp565: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp564, 3)) as usize] };
			let mut fTemp566: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp564 as usize] };
			let mut fTemp567: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp564, 1)) as usize] } - fTemp566;
			let mut fTemp568: F64 = 65535.0 * fTemp561;
			let mut iTemp569: i32 = (fTemp568) as i32;
			let mut iTemp570: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp569, 65535)))), 196607));
			let mut fTemp571: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp570, 3), 196607))) as usize] };
			let mut fTemp572: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp570 as usize] };
			let mut fTemp573: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp570, 1), 196607))) as usize] } - fTemp572;
			let mut iTemp574: i32 = (fTemp139 > ((if iTemp64 != 0 {fTemp572 + fTemp78 * fTemp573 + (fTemp568 - (iTemp569) as F64) * (fTemp571 - (fTemp572 + fTemp78 * (fTemp573 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp570, 4), 196607))) as usize] } - fTemp571))))} else {1.0 - (fTemp566 + fTemp78 * fTemp567 + (fTemp562 - (iTemp563) as F64) * (fTemp565 - (fTemp566 + fTemp78 * (fTemp567 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp564, 4)) as usize] } - fTemp565)))))} - fTemp560) / (1.0 - fTemp560))) as i32;
			let mut fTemp575: F64 = if iTemp574 != 0 {fTemp544} else {fTemp547};
			let mut fTemp576: F64 = if iTemp574 != 0 {fTemp547} else {fTemp545};
			let mut fTemp577: F64 = fTemp576 + fTemp575;
			let mut fTemp578: F64 = 0.5 * fTemp577;
			let mut fTemp579: F64 = 65535.0 * (1.0 - fTemp578);
			let mut iTemp580: i32 = (fTemp579) as i32;
			let mut iTemp581: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp580, 65535)))), 196607));
			let mut fTemp582: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp581, 3)) as usize] };
			let mut fTemp583: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp581 as usize] };
			let mut fTemp584: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp581, 1)) as usize] } - fTemp583;
			let mut fTemp585: F64 = 32767.5 * fTemp577;
			let mut iTemp586: i32 = (fTemp585) as i32;
			let mut iTemp587: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp586, 65535)))), 196607));
			let mut fTemp588: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp587, 3)) as usize] };
			let mut fTemp589: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp587 as usize] };
			let mut fTemp590: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp587, 1)) as usize] } - fTemp589;
			let mut fTemp591: F64 = if iTemp64 != 0 {fTemp589 + fTemp78 * fTemp590 + (fTemp585 - (iTemp586) as F64) * (fTemp588 - (fTemp589 + fTemp78 * (fTemp590 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp587, 4), 196607))) as usize] } - fTemp588))))} else {1.0 - (fTemp583 + fTemp78 * fTemp584 + (fTemp579 - (iTemp580) as F64) * (fTemp582 - (fTemp583 + fTemp78 * (fTemp584 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp581, 4), 196607))) as usize] } - fTemp582)))))};
			let mut fTemp592: F64 = fTemp83 + fTemp578;
			let mut fTemp593: F64 = 65535.0 * (1.0 - fTemp592);
			let mut iTemp594: i32 = (fTemp593) as i32;
			let mut iTemp595: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp594, 65535)))), 196607));
			let mut fTemp596: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp595, 3)) as usize] };
			let mut fTemp597: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp595 as usize] };
			let mut fTemp598: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp595, 1)) as usize] } - fTemp597;
			let mut fTemp599: F64 = 65535.0 * fTemp592;
			let mut iTemp600: i32 = (fTemp599) as i32;
			let mut iTemp601: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp600, 65535)))), 196607));
			let mut fTemp602: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp601, 3), 196607))) as usize] };
			let mut fTemp603: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp601 as usize] };
			let mut fTemp604: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp601, 1), 196607))) as usize] } - fTemp603;
			let mut iTemp605: i32 = (fTemp139 > ((if iTemp64 != 0 {fTemp603 + fTemp78 * fTemp604 + (fTemp599 - (iTemp600) as F64) * (fTemp602 - (fTemp603 + fTemp78 * (fTemp604 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp601, 4), 196607))) as usize] } - fTemp602))))} else {1.0 - (fTemp597 + fTemp78 * fTemp598 + (fTemp593 - (iTemp594) as F64) * (fTemp596 - (fTemp597 + fTemp78 * (fTemp598 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp595, 4)) as usize] } - fTemp596)))))} - fTemp591) / (1.0 - fTemp591))) as i32;
			let mut fTemp606: F64 = F64::min(1.0, F64::max(0.0, 0.5 * (if iTemp605 != 0 {fTemp578} else {fTemp576} + if iTemp605 != 0 {fTemp575} else {fTemp578})));
			self.fRec1[0] = fTemp606;
			let mut fTemp607: F64 = 65535.0 * (1.0 - fTemp606);
			let mut iTemp608: i32 = (fTemp607) as i32;
			let mut iTemp609: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp608, 65535)))), 196607));
			let mut fTemp610: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp609, 3)) as usize] };
			let mut fTemp611: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp609 as usize] };
			let mut fTemp612: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp609, 1)) as usize] } - fTemp611;
			let mut fTemp613: F64 = 65535.0 * fTemp606;
			let mut iTemp614: i32 = (fTemp613) as i32;
			let mut iTemp615: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp614, 65535)))), 196607));
			let mut fTemp616: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp615, 3)) as usize] };
			let mut fTemp617: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp615 as usize] };
			let mut fTemp618: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp615, 1)) as usize] } - fTemp617;
			let mut fTemp619: F64 = if iTemp64 != 0 {fTemp617 + fTemp78 * fTemp618 + (fTemp613 - (iTemp614) as F64) * (fTemp616 - (fTemp617 + fTemp78 * (fTemp618 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp615, 4), 196607))) as usize] } - fTemp616))))} else {1.0 - (fTemp611 + fTemp78 * fTemp612 + (fTemp607 - (iTemp608) as F64) * (fTemp610 - (fTemp611 + fTemp78 * (fTemp612 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp609, 4), 196607))) as usize] } - fTemp610)))))};
			let mut fTemp620: F64 = fTemp83 + fTemp606;
			let mut fTemp621: F64 = 65535.0 * (1.0 - fTemp620);
			let mut iTemp622: i32 = (fTemp621) as i32;
			let mut iTemp623: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp622, 65535)))), 196607));
			let mut fTemp624: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp623, 3)) as usize] };
			let mut fTemp625: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp623 as usize] };
			let mut fTemp626: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp623, 1)) as usize] } - fTemp625;
			let mut fTemp627: F64 = 65535.0 * fTemp620;
			let mut iTemp628: i32 = (fTemp627) as i32;
			let mut iTemp629: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp628, 65535)))), 196607));
			let mut fTemp630: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp629, 3), 196607))) as usize] };
			let mut fTemp631: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp629 as usize] };
			let mut fTemp632: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp629, 1), 196607))) as usize] } - fTemp631;
			let mut fTemp633: F64 = fTemp4 + if ((0.001 * fTemp82) == 0.0) as i32 != 0 {fTemp63} else {fTemp63 * (if iTemp64 != 0 {fTemp631 + fTemp78 * fTemp632 + (fTemp627 - (iTemp628) as F64) * (fTemp630 - (fTemp631 + fTemp78 * (fTemp632 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp629, 4), 196607))) as usize] } - fTemp630))))} else {1.0 - (fTemp625 + fTemp78 * fTemp626 + (fTemp621 - (iTemp622) as F64) * (fTemp624 - (fTemp625 + fTemp78 * (fTemp626 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp623, 4)) as usize] } - fTemp624)))))} - fTemp619) / (1.0 - fTemp619)};
			self.fRec2[(self.IOTA0 & 16383) as usize] = if iTemp81 != 0 {F64::min(fTemp633, fTemp4)} else {F64::max(fTemp633, fTemp4)};
			let mut fTemp634: F64 = self.fRec2[((i32::wrapping_sub(self.IOTA0, iSlow74)) & 16383) as usize];
			self.fHbargraph0 = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, fTemp634));
			self.fRec14[0] = fSlow76 + self.fConst4 * self.fRec14[1];
			*output0 = 0.5 * self.fVec0[((i32::wrapping_sub(self.IOTA0, iSlow75)) & 32767) as usize] * fTemp2 + self.fRec14[0] * self.fVec1[((i32::wrapping_sub(self.IOTA0, iSlow75)) & 32767) as usize] * fTemp634 * fTemp3;
			let mut fTemp635: F64 = self.fRec16[((i32::wrapping_sub(self.IOTA0, 1)) & 16383) as usize];
			let mut fTemp636: F64 = fTemp35 + fSlow17 * (fTemp36 - fTemp35);
			let mut iTemp637: i32 = ((fTemp636 > fSlow11) as i32) + ((fTemp636 > fSlow9) as i32);
			let mut fTemp638: F64 = fTemp636 - fSlow8;
			let mut fTemp639: F64 = F64::min(fTemp33, F64::powf(1e+01, -(fSlow18 * F64::max(0.0, if (iTemp637 == 0) as i32 != 0 {0.0} else {if (iTemp637 == 1) as i32 != 0 {fSlow12 * LambRs96k_faustpower2_f(fSlow7 + fTemp638)} else {fTemp638}}))));
			self.fVec32[(self.IOTA0 & 16383) as usize] = fTemp639;
			let mut fTemp640: F64 = F64::min(fTemp639, self.fVec32[((i32::wrapping_sub(self.IOTA0, 1)) & 16383) as usize]);
			self.fVec33[0] = fTemp640;
			let mut fTemp641: F64 = F64::min(fTemp640, self.fVec33[2]);
			self.fVec34[0] = fTemp641;
			let mut fTemp642: F64 = F64::min(fTemp641, self.fVec34[4]);
			self.fVec35[0] = fTemp642;
			let mut fTemp643: F64 = F64::min(fTemp642, self.fVec35[8]);
			self.fVec36[(self.IOTA0 & 31) as usize] = fTemp643;
			let mut fTemp644: F64 = F64::min(fTemp643, self.fVec36[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize]);
			self.fVec37[(self.IOTA0 & 63) as usize] = fTemp644;
			let mut fTemp645: F64 = F64::min(fTemp644, self.fVec37[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize]);
			self.fVec38[(self.IOTA0 & 127) as usize] = fTemp645;
			let mut fTemp646: F64 = F64::min(fTemp645, self.fVec38[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize]);
			self.fVec39[(self.IOTA0 & 255) as usize] = fTemp646;
			let mut fTemp647: F64 = F64::min(fTemp646, self.fVec39[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize]);
			self.fVec40[(self.IOTA0 & 511) as usize] = fTemp647;
			let mut fTemp648: F64 = F64::min(fTemp647, self.fVec40[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize]);
			self.fVec41[(self.IOTA0 & 1023) as usize] = fTemp648;
			let mut fTemp649: F64 = F64::min(fTemp648, self.fVec41[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize]);
			self.fVec42[(self.IOTA0 & 2047) as usize] = fTemp649;
			let mut fTemp650: F64 = F64::min(fTemp649, self.fVec42[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			self.fVec43[(self.IOTA0 & 4095) as usize] = fTemp650;
			self.fVec44[(self.IOTA0 & 8191) as usize] = F64::min(fTemp650, self.fVec43[((i32::wrapping_sub(self.IOTA0, 2048)) & 4095) as usize]);
			self.fRec17[0] = F64::max(F64::min(self.fRec17[1], self.fVec32[((i32::wrapping_sub(self.IOTA0, iSlow21)) & 16383) as usize]), F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow23 != 0 {fTemp639} else {1.7976931348623157e+308}, if iSlow24 != 0 {self.fVec33[iSlow23 as usize]} else {1.7976931348623157e+308}), if iSlow25 != 0 {self.fVec34[iSlow26 as usize]} else {1.7976931348623157e+308}), if iSlow27 != 0 {self.fVec35[iSlow28 as usize]} else {1.7976931348623157e+308}), if iSlow29 != 0 {self.fVec36[((i32::wrapping_sub(self.IOTA0, iSlow30)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow31 != 0 {self.fVec37[((i32::wrapping_sub(self.IOTA0, iSlow32)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow33 != 0 {self.fVec38[((i32::wrapping_sub(self.IOTA0, iSlow34)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow35 != 0 {self.fVec39[((i32::wrapping_sub(self.IOTA0, iSlow36)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow37 != 0 {self.fVec40[((i32::wrapping_sub(self.IOTA0, iSlow38)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow39 != 0 {self.fVec41[((i32::wrapping_sub(self.IOTA0, iSlow40)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow41 != 0 {self.fVec42[((i32::wrapping_sub(self.IOTA0, iSlow42)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow43 != 0 {self.fVec43[((i32::wrapping_sub(self.IOTA0, iSlow44)) & 4095) as usize]} else {1.7976931348623157e+308}), if iSlow45 != 0 {self.fVec44[((i32::wrapping_sub(self.IOTA0, iSlow46)) & 8191) as usize]} else {1.7976931348623157e+308}));
			let mut fTemp651: F64 = F64::min(self.fRec17[0], self.fRec17[1]);
			self.fVec45[0] = fTemp651;
			let mut fTemp652: F64 = F64::min(fTemp651, self.fVec45[2]);
			self.fVec46[0] = fTemp652;
			let mut fTemp653: F64 = F64::min(fTemp652, self.fVec46[4]);
			self.fVec47[0] = fTemp653;
			let mut fTemp654: F64 = F64::min(fTemp653, self.fVec47[8]);
			self.fVec48[(self.IOTA0 & 31) as usize] = fTemp654;
			let mut fTemp655: F64 = F64::min(fTemp654, self.fVec48[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize]);
			self.fVec49[(self.IOTA0 & 63) as usize] = fTemp655;
			let mut fTemp656: F64 = F64::min(fTemp655, self.fVec49[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize]);
			self.fVec50[(self.IOTA0 & 127) as usize] = fTemp656;
			let mut fTemp657: F64 = F64::min(fTemp656, self.fVec50[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize]);
			self.fVec51[(self.IOTA0 & 255) as usize] = fTemp657;
			let mut fTemp658: F64 = F64::min(fTemp657, self.fVec51[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize]);
			self.fVec52[(self.IOTA0 & 511) as usize] = fTemp658;
			let mut fTemp659: F64 = F64::min(fTemp658, self.fVec52[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize]);
			self.fVec53[(self.IOTA0 & 1023) as usize] = fTemp659;
			let mut fTemp660: F64 = F64::min(fTemp659, self.fVec53[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize]);
			self.fVec54[(self.IOTA0 & 2047) as usize] = fTemp660;
			let mut fTemp661: F64 = F64::min(fTemp660, self.fVec54[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			self.fVec55[(self.IOTA0 & 4095) as usize] = fTemp661;
			self.fVec56[(self.IOTA0 & 8191) as usize] = F64::min(fTemp661, self.fVec55[((i32::wrapping_sub(self.IOTA0, 2048)) & 4095) as usize]);
			let mut fTemp662: F64 = F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow4 != 0 {self.fRec17[0]} else {1.7976931348623157e+308}, if iSlow47 != 0 {self.fVec45[iSlow4 as usize]} else {1.7976931348623157e+308}), if iSlow48 != 0 {self.fVec46[iSlow49 as usize]} else {1.7976931348623157e+308}), if iSlow50 != 0 {self.fVec47[iSlow51 as usize]} else {1.7976931348623157e+308}), if iSlow52 != 0 {self.fVec48[((i32::wrapping_sub(self.IOTA0, iSlow53)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow54 != 0 {self.fVec49[((i32::wrapping_sub(self.IOTA0, iSlow55)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow56 != 0 {self.fVec50[((i32::wrapping_sub(self.IOTA0, iSlow57)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow58 != 0 {self.fVec51[((i32::wrapping_sub(self.IOTA0, iSlow59)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow60 != 0 {self.fVec52[((i32::wrapping_sub(self.IOTA0, iSlow61)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow62 != 0 {self.fVec53[((i32::wrapping_sub(self.IOTA0, iSlow63)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow64 != 0 {self.fVec54[((i32::wrapping_sub(self.IOTA0, iSlow65)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow66 != 0 {self.fVec55[((i32::wrapping_sub(self.IOTA0, iSlow67)) & 4095) as usize]} else {1.7976931348623157e+308}), if iSlow68 != 0 {self.fVec56[((i32::wrapping_sub(self.IOTA0, iSlow69)) & 8191) as usize]} else {1.7976931348623157e+308}) - fTemp635;
			self.fVec57[0] = fTemp662;
			let mut iTemp663: i32 = (fTemp662 > 0.0) as i32;
			let mut fTemp664: F64 = if iTemp663 != 0 {fSlow71} else {fSlow70};
			self.fVec58[0] = fTemp664;
			let mut fTemp665: F64 = 2.0 * fTemp664;
			let mut iTemp666: i32 = (fTemp665) as i32;
			let mut iTemp667: i32 = std::cmp::max(0, std::cmp::min(iTemp666, 2));
			let mut iTemp668: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, 98301), 196607));
			let mut fTemp669: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp668, 3)) as usize] };
			let mut fTemp670: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp668 as usize] };
			let mut fTemp671: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp668, 1)) as usize] } - fTemp670;
			let mut fTemp672: F64 = fTemp665 - (iTemp666) as F64;
			let mut fTemp673: F64 = fTemp670 + fTemp672 * fTemp671 + 0.5 * (fTemp669 - (fTemp670 + fTemp672 * (fTemp671 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp668, 4)) as usize] } - fTemp669))));
			let mut fTemp674: F64 = if iTemp663 != 0 {fTemp673} else {1.0 - fTemp673};
			let mut iTemp675: i32 = (fTemp662 < 0.0) as i32;
			let mut fTemp676: F64 = fSlow1 * (iTemp675) as F64 + fSlow13 * (iTemp663) as F64;
			self.fVec59[0] = fTemp676;
			let mut fTemp677: F64 = self.fConst10 / fTemp676;
			let mut fTemp678: F64 = fTemp677 + 0.5;
			let mut fTemp679: F64 = 65535.0 * (1.0 - fTemp678);
			let mut iTemp680: i32 = (fTemp679) as i32;
			let mut iTemp681: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp680, 65535)))), 196607));
			let mut fTemp682: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp681, 3)) as usize] };
			let mut fTemp683: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp681 as usize] };
			let mut fTemp684: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp681, 1)) as usize] } - fTemp683;
			let mut fTemp685: F64 = 65535.0 * fTemp678;
			let mut iTemp686: i32 = (fTemp685) as i32;
			let mut iTemp687: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp686, 65535)))), 196607));
			let mut fTemp688: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp687, 3), 196607))) as usize] };
			let mut fTemp689: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp687 as usize] };
			let mut fTemp690: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp687, 1), 196607))) as usize] } - fTemp689;
			let mut fTemp691: F64 = 2.0 * self.fVec58[1];
			let mut iTemp692: i32 = (fTemp691) as i32;
			let mut iTemp693: i32 = std::cmp::max(0, std::cmp::min(iTemp692, 2));
			let mut fTemp694: F64 = 65535.0 * (1.0 - self.fRec15[1]);
			let mut iTemp695: i32 = (fTemp694) as i32;
			let mut iTemp696: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp695, 65535))), iTemp693), 196607));
			let mut fTemp697: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp696, 3), 196607))) as usize] };
			let mut fTemp698: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp696 as usize] };
			let mut fTemp699: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp696, 1), 196607))) as usize] } - fTemp698;
			let mut fTemp700: F64 = fTemp691 - (iTemp692) as F64;
			let mut fTemp701: F64 = 65535.0 * self.fRec15[1];
			let mut iTemp702: i32 = (fTemp701) as i32;
			let mut iTemp703: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp693, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp702, 65535)))), 196607));
			let mut fTemp704: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp703, 3), 196607))) as usize] };
			let mut fTemp705: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp703 as usize] };
			let mut fTemp706: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp703, 1), 196607))) as usize] } - fTemp705;
			let mut fTemp707: F64 = self.fRec15[1] + fTemp677;
			let mut fTemp708: F64 = 65535.0 * (1.0 - fTemp707);
			let mut iTemp709: i32 = (fTemp708) as i32;
			let mut iTemp710: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp709, 65535)))), 196607));
			let mut fTemp711: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp710, 3)) as usize] };
			let mut fTemp712: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp710 as usize] };
			let mut fTemp713: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp710, 1)) as usize] } - fTemp712;
			let mut fTemp714: F64 = 65535.0 * fTemp707;
			let mut iTemp715: i32 = (fTemp714) as i32;
			let mut iTemp716: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp715, 65535)))), 196607));
			let mut fTemp717: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp716, 3), 196607))) as usize] };
			let mut fTemp718: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp716 as usize] };
			let mut fTemp719: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp716, 1), 196607))) as usize] } - fTemp718;
			let mut fTemp720: F64 = self.fRec15[1] + self.fConst10 * (1.0 / fTemp676 + 1.0 / self.fVec59[1]);
			let mut fTemp721: F64 = 65535.0 * (1.0 - fTemp720);
			let mut iTemp722: i32 = (fTemp721) as i32;
			let mut iTemp723: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp722, 65535))), iTemp667), 196607));
			let mut fTemp724: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp723, 3)) as usize] };
			let mut fTemp725: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp723 as usize] };
			let mut fTemp726: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp723, 1)) as usize] } - fTemp725;
			let mut fTemp727: F64 = 65535.0 * fTemp720;
			let mut iTemp728: i32 = (fTemp727) as i32;
			let mut iTemp729: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp728, 65535)))), 196607));
			let mut fTemp730: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp729, 3), 196607))) as usize] };
			let mut fTemp731: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp729 as usize] };
			let mut fTemp732: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp729, 1), 196607))) as usize] } - fTemp731;
			let mut fTemp733: F64 = (if iTemp663 != 0 {fTemp731 + fTemp672 * fTemp732 + (fTemp727 - (iTemp728) as F64) * (fTemp730 - (fTemp731 + fTemp672 * (fTemp732 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp729, 4), 196607))) as usize] } - fTemp730))))} else {1.0 - (fTemp725 + fTemp672 * fTemp726 + (fTemp721 - (iTemp722) as F64) * (fTemp724 - (fTemp725 + fTemp672 * (fTemp726 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp723, 4)) as usize] } - fTemp724)))))} - if iTemp663 != 0 {fTemp718 + fTemp672 * fTemp719 + (fTemp714 - (iTemp715) as F64) * (fTemp717 - (fTemp718 + fTemp672 * (fTemp719 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp716, 4), 196607))) as usize] } - fTemp717))))} else {1.0 - (fTemp712 + fTemp672 * fTemp713 + (fTemp708 - (iTemp709) as F64) * (fTemp711 - (fTemp712 + fTemp672 * (fTemp713 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp710, 4), 196607))) as usize] } - fTemp711)))))}) * self.fVec57[1] / (fTemp662 * (1.0 - if iTemp663 != 0 {fTemp705 + fTemp700 * fTemp706 + (fTemp701 - (iTemp702) as F64) * (fTemp704 - (fTemp705 + fTemp700 * (fTemp706 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp703, 4), 196607))) as usize] } - fTemp704))))} else {1.0 - (fTemp698 + fTemp700 * fTemp699 + (fTemp694 - (iTemp695) as F64) * (fTemp697 - (fTemp698 + fTemp700 * (fTemp699 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp696, 4), 196607))) as usize] } - fTemp697)))))}));
			let mut iTemp734: i32 = (fTemp733 > ((if iTemp663 != 0 {fTemp689 + fTemp672 * fTemp690 + (fTemp685 - (iTemp686) as F64) * (fTemp688 - (fTemp689 + fTemp672 * (fTemp690 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp687, 4), 196607))) as usize] } - fTemp688))))} else {1.0 - (fTemp683 + fTemp672 * fTemp684 + (fTemp679 - (iTemp680) as F64) * (fTemp682 - (fTemp683 + fTemp672 * (fTemp684 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp681, 4)) as usize] } - fTemp682)))))} - fTemp674) / (1.0 - fTemp674))) as i32;
			let mut fTemp735: F64 = if iTemp734 != 0 {1.0} else {0.5};
			let mut fTemp736: F64 = if iTemp734 != 0 {0.5} else {0.0};
			let mut fTemp737: F64 = fTemp736 + fTemp735;
			let mut fTemp738: F64 = 0.5 * fTemp737;
			let mut fTemp739: F64 = 65535.0 * (1.0 - fTemp738);
			let mut iTemp740: i32 = (fTemp739) as i32;
			let mut iTemp741: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp740, 65535)))), 196607));
			let mut fTemp742: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp741, 3)) as usize] };
			let mut fTemp743: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp741 as usize] };
			let mut fTemp744: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp741, 1)) as usize] } - fTemp743;
			let mut fTemp745: F64 = 32767.5 * fTemp737;
			let mut iTemp746: i32 = (fTemp745) as i32;
			let mut iTemp747: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp746, 65535)))), 196607));
			let mut fTemp748: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp747, 3)) as usize] };
			let mut fTemp749: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp747 as usize] };
			let mut fTemp750: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp747, 1)) as usize] } - fTemp749;
			let mut fTemp751: F64 = if iTemp663 != 0 {fTemp749 + fTemp672 * fTemp750 + (fTemp745 - (iTemp746) as F64) * (fTemp748 - (fTemp749 + fTemp672 * (fTemp750 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp747, 4)) as usize] } - fTemp748))))} else {1.0 - (fTemp743 + fTemp672 * fTemp744 + (fTemp739 - (iTemp740) as F64) * (fTemp742 - (fTemp743 + fTemp672 * (fTemp744 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp741, 4)) as usize] } - fTemp742)))))};
			let mut fTemp752: F64 = fTemp677 + fTemp738;
			let mut fTemp753: F64 = 65535.0 * (1.0 - fTemp752);
			let mut iTemp754: i32 = (fTemp753) as i32;
			let mut iTemp755: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp754, 65535)))), 196607));
			let mut fTemp756: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp755, 3)) as usize] };
			let mut fTemp757: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp755 as usize] };
			let mut fTemp758: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp755, 1)) as usize] } - fTemp757;
			let mut fTemp759: F64 = 65535.0 * fTemp752;
			let mut iTemp760: i32 = (fTemp759) as i32;
			let mut iTemp761: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp760, 65535)))), 196607));
			let mut fTemp762: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp761, 3), 196607))) as usize] };
			let mut fTemp763: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp761 as usize] };
			let mut fTemp764: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp761, 1), 196607))) as usize] } - fTemp763;
			let mut iTemp765: i32 = (fTemp733 > ((if iTemp663 != 0 {fTemp763 + fTemp672 * fTemp764 + (fTemp759 - (iTemp760) as F64) * (fTemp762 - (fTemp763 + fTemp672 * (fTemp764 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp761, 4), 196607))) as usize] } - fTemp762))))} else {1.0 - (fTemp757 + fTemp672 * fTemp758 + (fTemp753 - (iTemp754) as F64) * (fTemp756 - (fTemp757 + fTemp672 * (fTemp758 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp755, 4)) as usize] } - fTemp756)))))} - fTemp751) / (1.0 - fTemp751))) as i32;
			let mut fTemp766: F64 = if iTemp765 != 0 {fTemp735} else {fTemp738};
			let mut fTemp767: F64 = if iTemp765 != 0 {fTemp738} else {fTemp736};
			let mut fTemp768: F64 = fTemp767 + fTemp766;
			let mut fTemp769: F64 = 0.5 * fTemp768;
			let mut fTemp770: F64 = 65535.0 * (1.0 - fTemp769);
			let mut iTemp771: i32 = (fTemp770) as i32;
			let mut iTemp772: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp771, 65535)))), 196607));
			let mut fTemp773: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp772, 3)) as usize] };
			let mut fTemp774: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp772 as usize] };
			let mut fTemp775: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp772, 1)) as usize] } - fTemp774;
			let mut fTemp776: F64 = 32767.5 * fTemp768;
			let mut iTemp777: i32 = (fTemp776) as i32;
			let mut iTemp778: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp777, 65535)))), 196607));
			let mut fTemp779: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp778, 3)) as usize] };
			let mut fTemp780: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp778 as usize] };
			let mut fTemp781: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp778, 1)) as usize] } - fTemp780;
			let mut fTemp782: F64 = if iTemp663 != 0 {fTemp780 + fTemp672 * fTemp781 + (fTemp776 - (iTemp777) as F64) * (fTemp779 - (fTemp780 + fTemp672 * (fTemp781 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp778, 4)) as usize] } - fTemp779))))} else {1.0 - (fTemp774 + fTemp672 * fTemp775 + (fTemp770 - (iTemp771) as F64) * (fTemp773 - (fTemp774 + fTemp672 * (fTemp775 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp772, 4)) as usize] } - fTemp773)))))};
			let mut fTemp783: F64 = fTemp677 + fTemp769;
			let mut fTemp784: F64 = 65535.0 * (1.0 - fTemp783);
			let mut iTemp785: i32 = (fTemp784) as i32;
			let mut iTemp786: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp785, 65535)))), 196607));
			let mut fTemp787: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp786, 3)) as usize] };
			let mut fTemp788: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp786 as usize] };
			let mut fTemp789: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp786, 1)) as usize] } - fTemp788;
			let mut fTemp790: F64 = 65535.0 * fTemp783;
			let mut iTemp791: i32 = (fTemp790) as i32;
			let mut iTemp792: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp791, 65535)))), 196607));
			let mut fTemp793: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp792, 3), 196607))) as usize] };
			let mut fTemp794: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp792 as usize] };
			let mut fTemp795: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp792, 1), 196607))) as usize] } - fTemp794;
			let mut iTemp796: i32 = (fTemp733 > ((if iTemp663 != 0 {fTemp794 + fTemp672 * fTemp795 + (fTemp790 - (iTemp791) as F64) * (fTemp793 - (fTemp794 + fTemp672 * (fTemp795 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp792, 4), 196607))) as usize] } - fTemp793))))} else {1.0 - (fTemp788 + fTemp672 * fTemp789 + (fTemp784 - (iTemp785) as F64) * (fTemp787 - (fTemp788 + fTemp672 * (fTemp789 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp786, 4)) as usize] } - fTemp787)))))} - fTemp782) / (1.0 - fTemp782))) as i32;
			let mut fTemp797: F64 = if iTemp796 != 0 {fTemp766} else {fTemp769};
			let mut fTemp798: F64 = if iTemp796 != 0 {fTemp769} else {fTemp767};
			let mut fTemp799: F64 = fTemp798 + fTemp797;
			let mut fTemp800: F64 = 0.5 * fTemp799;
			let mut fTemp801: F64 = 65535.0 * (1.0 - fTemp800);
			let mut iTemp802: i32 = (fTemp801) as i32;
			let mut iTemp803: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp802, 65535)))), 196607));
			let mut fTemp804: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp803, 3)) as usize] };
			let mut fTemp805: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp803 as usize] };
			let mut fTemp806: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp803, 1)) as usize] } - fTemp805;
			let mut fTemp807: F64 = 32767.5 * fTemp799;
			let mut iTemp808: i32 = (fTemp807) as i32;
			let mut iTemp809: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp808, 65535)))), 196607));
			let mut fTemp810: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp809, 3)) as usize] };
			let mut fTemp811: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp809 as usize] };
			let mut fTemp812: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp809, 1)) as usize] } - fTemp811;
			let mut fTemp813: F64 = if iTemp663 != 0 {fTemp811 + fTemp672 * fTemp812 + (fTemp807 - (iTemp808) as F64) * (fTemp810 - (fTemp811 + fTemp672 * (fTemp812 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp809, 4)) as usize] } - fTemp810))))} else {1.0 - (fTemp805 + fTemp672 * fTemp806 + (fTemp801 - (iTemp802) as F64) * (fTemp804 - (fTemp805 + fTemp672 * (fTemp806 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp803, 4)) as usize] } - fTemp804)))))};
			let mut fTemp814: F64 = fTemp677 + fTemp800;
			let mut fTemp815: F64 = 65535.0 * (1.0 - fTemp814);
			let mut iTemp816: i32 = (fTemp815) as i32;
			let mut iTemp817: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp816, 65535)))), 196607));
			let mut fTemp818: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp817, 3)) as usize] };
			let mut fTemp819: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp817 as usize] };
			let mut fTemp820: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp817, 1)) as usize] } - fTemp819;
			let mut fTemp821: F64 = 65535.0 * fTemp814;
			let mut iTemp822: i32 = (fTemp821) as i32;
			let mut iTemp823: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp822, 65535)))), 196607));
			let mut fTemp824: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp823, 3), 196607))) as usize] };
			let mut fTemp825: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp823 as usize] };
			let mut fTemp826: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp823, 1), 196607))) as usize] } - fTemp825;
			let mut iTemp827: i32 = (fTemp733 > ((if iTemp663 != 0 {fTemp825 + fTemp672 * fTemp826 + (fTemp821 - (iTemp822) as F64) * (fTemp824 - (fTemp825 + fTemp672 * (fTemp826 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp823, 4), 196607))) as usize] } - fTemp824))))} else {1.0 - (fTemp819 + fTemp672 * fTemp820 + (fTemp815 - (iTemp816) as F64) * (fTemp818 - (fTemp819 + fTemp672 * (fTemp820 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp817, 4)) as usize] } - fTemp818)))))} - fTemp813) / (1.0 - fTemp813))) as i32;
			let mut fTemp828: F64 = if iTemp827 != 0 {fTemp797} else {fTemp800};
			let mut fTemp829: F64 = if iTemp827 != 0 {fTemp800} else {fTemp798};
			let mut fTemp830: F64 = fTemp829 + fTemp828;
			let mut fTemp831: F64 = 0.5 * fTemp830;
			let mut fTemp832: F64 = 65535.0 * (1.0 - fTemp831);
			let mut iTemp833: i32 = (fTemp832) as i32;
			let mut iTemp834: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp833, 65535)))), 196607));
			let mut fTemp835: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp834, 3)) as usize] };
			let mut fTemp836: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp834 as usize] };
			let mut fTemp837: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp834, 1)) as usize] } - fTemp836;
			let mut fTemp838: F64 = 32767.5 * fTemp830;
			let mut iTemp839: i32 = (fTemp838) as i32;
			let mut iTemp840: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp839, 65535)))), 196607));
			let mut fTemp841: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp840, 3)) as usize] };
			let mut fTemp842: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp840 as usize] };
			let mut fTemp843: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp840, 1)) as usize] } - fTemp842;
			let mut fTemp844: F64 = if iTemp663 != 0 {fTemp842 + fTemp672 * fTemp843 + (fTemp838 - (iTemp839) as F64) * (fTemp841 - (fTemp842 + fTemp672 * (fTemp843 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp840, 4)) as usize] } - fTemp841))))} else {1.0 - (fTemp836 + fTemp672 * fTemp837 + (fTemp832 - (iTemp833) as F64) * (fTemp835 - (fTemp836 + fTemp672 * (fTemp837 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp834, 4)) as usize] } - fTemp835)))))};
			let mut fTemp845: F64 = fTemp677 + fTemp831;
			let mut fTemp846: F64 = 65535.0 * (1.0 - fTemp845);
			let mut iTemp847: i32 = (fTemp846) as i32;
			let mut iTemp848: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp847, 65535)))), 196607));
			let mut fTemp849: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp848, 3)) as usize] };
			let mut fTemp850: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp848 as usize] };
			let mut fTemp851: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp848, 1)) as usize] } - fTemp850;
			let mut fTemp852: F64 = 65535.0 * fTemp845;
			let mut iTemp853: i32 = (fTemp852) as i32;
			let mut iTemp854: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp853, 65535)))), 196607));
			let mut fTemp855: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp854, 3), 196607))) as usize] };
			let mut fTemp856: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp854 as usize] };
			let mut fTemp857: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp854, 1), 196607))) as usize] } - fTemp856;
			let mut iTemp858: i32 = (fTemp733 > ((if iTemp663 != 0 {fTemp856 + fTemp672 * fTemp857 + (fTemp852 - (iTemp853) as F64) * (fTemp855 - (fTemp856 + fTemp672 * (fTemp857 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp854, 4), 196607))) as usize] } - fTemp855))))} else {1.0 - (fTemp850 + fTemp672 * fTemp851 + (fTemp846 - (iTemp847) as F64) * (fTemp849 - (fTemp850 + fTemp672 * (fTemp851 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp848, 4)) as usize] } - fTemp849)))))} - fTemp844) / (1.0 - fTemp844))) as i32;
			let mut fTemp859: F64 = if iTemp858 != 0 {fTemp828} else {fTemp831};
			let mut fTemp860: F64 = if iTemp858 != 0 {fTemp831} else {fTemp829};
			let mut fTemp861: F64 = fTemp860 + fTemp859;
			let mut fTemp862: F64 = 0.5 * fTemp861;
			let mut fTemp863: F64 = 65535.0 * (1.0 - fTemp862);
			let mut iTemp864: i32 = (fTemp863) as i32;
			let mut iTemp865: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp864, 65535)))), 196607));
			let mut fTemp866: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp865, 3)) as usize] };
			let mut fTemp867: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp865 as usize] };
			let mut fTemp868: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp865, 1)) as usize] } - fTemp867;
			let mut fTemp869: F64 = 32767.5 * fTemp861;
			let mut iTemp870: i32 = (fTemp869) as i32;
			let mut iTemp871: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp870, 65535)))), 196607));
			let mut fTemp872: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp871, 3)) as usize] };
			let mut fTemp873: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp871 as usize] };
			let mut fTemp874: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp871, 1)) as usize] } - fTemp873;
			let mut fTemp875: F64 = if iTemp663 != 0 {fTemp873 + fTemp672 * fTemp874 + (fTemp869 - (iTemp870) as F64) * (fTemp872 - (fTemp873 + fTemp672 * (fTemp874 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp871, 4)) as usize] } - fTemp872))))} else {1.0 - (fTemp867 + fTemp672 * fTemp868 + (fTemp863 - (iTemp864) as F64) * (fTemp866 - (fTemp867 + fTemp672 * (fTemp868 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp865, 4)) as usize] } - fTemp866)))))};
			let mut fTemp876: F64 = fTemp677 + fTemp862;
			let mut fTemp877: F64 = 65535.0 * (1.0 - fTemp876);
			let mut iTemp878: i32 = (fTemp877) as i32;
			let mut iTemp879: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp878, 65535)))), 196607));
			let mut fTemp880: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp879, 3)) as usize] };
			let mut fTemp881: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp879 as usize] };
			let mut fTemp882: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp879, 1)) as usize] } - fTemp881;
			let mut fTemp883: F64 = 65535.0 * fTemp876;
			let mut iTemp884: i32 = (fTemp883) as i32;
			let mut iTemp885: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp884, 65535)))), 196607));
			let mut fTemp886: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp885, 3), 196607))) as usize] };
			let mut fTemp887: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp885 as usize] };
			let mut fTemp888: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp885, 1), 196607))) as usize] } - fTemp887;
			let mut iTemp889: i32 = (fTemp733 > ((if iTemp663 != 0 {fTemp887 + fTemp672 * fTemp888 + (fTemp883 - (iTemp884) as F64) * (fTemp886 - (fTemp887 + fTemp672 * (fTemp888 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp885, 4), 196607))) as usize] } - fTemp886))))} else {1.0 - (fTemp881 + fTemp672 * fTemp882 + (fTemp877 - (iTemp878) as F64) * (fTemp880 - (fTemp881 + fTemp672 * (fTemp882 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp879, 4)) as usize] } - fTemp880)))))} - fTemp875) / (1.0 - fTemp875))) as i32;
			let mut fTemp890: F64 = if iTemp889 != 0 {fTemp859} else {fTemp862};
			let mut fTemp891: F64 = if iTemp889 != 0 {fTemp862} else {fTemp860};
			let mut fTemp892: F64 = fTemp891 + fTemp890;
			let mut fTemp893: F64 = 0.5 * fTemp892;
			let mut fTemp894: F64 = 65535.0 * (1.0 - fTemp893);
			let mut iTemp895: i32 = (fTemp894) as i32;
			let mut iTemp896: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp895, 65535)))), 196607));
			let mut fTemp897: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp896, 3)) as usize] };
			let mut fTemp898: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp896 as usize] };
			let mut fTemp899: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp896, 1)) as usize] } - fTemp898;
			let mut fTemp900: F64 = 32767.5 * fTemp892;
			let mut iTemp901: i32 = (fTemp900) as i32;
			let mut iTemp902: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp901, 65535)))), 196607));
			let mut fTemp903: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp902, 3)) as usize] };
			let mut fTemp904: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp902 as usize] };
			let mut fTemp905: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp902, 1)) as usize] } - fTemp904;
			let mut fTemp906: F64 = if iTemp663 != 0 {fTemp904 + fTemp672 * fTemp905 + (fTemp900 - (iTemp901) as F64) * (fTemp903 - (fTemp904 + fTemp672 * (fTemp905 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp902, 4)) as usize] } - fTemp903))))} else {1.0 - (fTemp898 + fTemp672 * fTemp899 + (fTemp894 - (iTemp895) as F64) * (fTemp897 - (fTemp898 + fTemp672 * (fTemp899 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp896, 4)) as usize] } - fTemp897)))))};
			let mut fTemp907: F64 = fTemp677 + fTemp893;
			let mut fTemp908: F64 = 65535.0 * (1.0 - fTemp907);
			let mut iTemp909: i32 = (fTemp908) as i32;
			let mut iTemp910: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp909, 65535)))), 196607));
			let mut fTemp911: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp910, 3)) as usize] };
			let mut fTemp912: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp910 as usize] };
			let mut fTemp913: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp910, 1)) as usize] } - fTemp912;
			let mut fTemp914: F64 = 65535.0 * fTemp907;
			let mut iTemp915: i32 = (fTemp914) as i32;
			let mut iTemp916: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp915, 65535)))), 196607));
			let mut fTemp917: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp916, 3), 196607))) as usize] };
			let mut fTemp918: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp916 as usize] };
			let mut fTemp919: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp916, 1), 196607))) as usize] } - fTemp918;
			let mut iTemp920: i32 = (fTemp733 > ((if iTemp663 != 0 {fTemp918 + fTemp672 * fTemp919 + (fTemp914 - (iTemp915) as F64) * (fTemp917 - (fTemp918 + fTemp672 * (fTemp919 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp916, 4), 196607))) as usize] } - fTemp917))))} else {1.0 - (fTemp912 + fTemp672 * fTemp913 + (fTemp908 - (iTemp909) as F64) * (fTemp911 - (fTemp912 + fTemp672 * (fTemp913 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp910, 4)) as usize] } - fTemp911)))))} - fTemp906) / (1.0 - fTemp906))) as i32;
			let mut fTemp921: F64 = if iTemp920 != 0 {fTemp890} else {fTemp893};
			let mut fTemp922: F64 = if iTemp920 != 0 {fTemp893} else {fTemp891};
			let mut fTemp923: F64 = fTemp922 + fTemp921;
			let mut fTemp924: F64 = 0.5 * fTemp923;
			let mut fTemp925: F64 = 65535.0 * (1.0 - fTemp924);
			let mut iTemp926: i32 = (fTemp925) as i32;
			let mut iTemp927: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp926, 65535)))), 196607));
			let mut fTemp928: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp927, 3)) as usize] };
			let mut fTemp929: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp927 as usize] };
			let mut fTemp930: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp927, 1)) as usize] } - fTemp929;
			let mut fTemp931: F64 = 32767.5 * fTemp923;
			let mut iTemp932: i32 = (fTemp931) as i32;
			let mut iTemp933: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp932, 65535)))), 196607));
			let mut fTemp934: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp933, 3)) as usize] };
			let mut fTemp935: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp933 as usize] };
			let mut fTemp936: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp933, 1)) as usize] } - fTemp935;
			let mut fTemp937: F64 = if iTemp663 != 0 {fTemp935 + fTemp672 * fTemp936 + (fTemp931 - (iTemp932) as F64) * (fTemp934 - (fTemp935 + fTemp672 * (fTemp936 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp933, 4)) as usize] } - fTemp934))))} else {1.0 - (fTemp929 + fTemp672 * fTemp930 + (fTemp925 - (iTemp926) as F64) * (fTemp928 - (fTemp929 + fTemp672 * (fTemp930 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp927, 4)) as usize] } - fTemp928)))))};
			let mut fTemp938: F64 = fTemp677 + fTemp924;
			let mut fTemp939: F64 = 65535.0 * (1.0 - fTemp938);
			let mut iTemp940: i32 = (fTemp939) as i32;
			let mut iTemp941: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp940, 65535)))), 196607));
			let mut fTemp942: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp941, 3)) as usize] };
			let mut fTemp943: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp941 as usize] };
			let mut fTemp944: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp941, 1)) as usize] } - fTemp943;
			let mut fTemp945: F64 = 65535.0 * fTemp938;
			let mut iTemp946: i32 = (fTemp945) as i32;
			let mut iTemp947: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp946, 65535)))), 196607));
			let mut fTemp948: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp947, 3), 196607))) as usize] };
			let mut fTemp949: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp947 as usize] };
			let mut fTemp950: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp947, 1), 196607))) as usize] } - fTemp949;
			let mut iTemp951: i32 = (fTemp733 > ((if iTemp663 != 0 {fTemp949 + fTemp672 * fTemp950 + (fTemp945 - (iTemp946) as F64) * (fTemp948 - (fTemp949 + fTemp672 * (fTemp950 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp947, 4), 196607))) as usize] } - fTemp948))))} else {1.0 - (fTemp943 + fTemp672 * fTemp944 + (fTemp939 - (iTemp940) as F64) * (fTemp942 - (fTemp943 + fTemp672 * (fTemp944 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp941, 4)) as usize] } - fTemp942)))))} - fTemp937) / (1.0 - fTemp937))) as i32;
			let mut fTemp952: F64 = if iTemp951 != 0 {fTemp921} else {fTemp924};
			let mut fTemp953: F64 = if iTemp951 != 0 {fTemp924} else {fTemp922};
			let mut fTemp954: F64 = fTemp953 + fTemp952;
			let mut fTemp955: F64 = 0.5 * fTemp954;
			let mut fTemp956: F64 = 65535.0 * (1.0 - fTemp955);
			let mut iTemp957: i32 = (fTemp956) as i32;
			let mut iTemp958: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp957, 65535)))), 196607));
			let mut fTemp959: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp958, 3)) as usize] };
			let mut fTemp960: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp958 as usize] };
			let mut fTemp961: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp958, 1)) as usize] } - fTemp960;
			let mut fTemp962: F64 = 32767.5 * fTemp954;
			let mut iTemp963: i32 = (fTemp962) as i32;
			let mut iTemp964: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp963, 65535)))), 196607));
			let mut fTemp965: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp964, 3)) as usize] };
			let mut fTemp966: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp964 as usize] };
			let mut fTemp967: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp964, 1)) as usize] } - fTemp966;
			let mut fTemp968: F64 = if iTemp663 != 0 {fTemp966 + fTemp672 * fTemp967 + (fTemp962 - (iTemp963) as F64) * (fTemp965 - (fTemp966 + fTemp672 * (fTemp967 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp964, 4)) as usize] } - fTemp965))))} else {1.0 - (fTemp960 + fTemp672 * fTemp961 + (fTemp956 - (iTemp957) as F64) * (fTemp959 - (fTemp960 + fTemp672 * (fTemp961 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp958, 4)) as usize] } - fTemp959)))))};
			let mut fTemp969: F64 = fTemp677 + fTemp955;
			let mut fTemp970: F64 = 65535.0 * (1.0 - fTemp969);
			let mut iTemp971: i32 = (fTemp970) as i32;
			let mut iTemp972: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp971, 65535)))), 196607));
			let mut fTemp973: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp972, 3)) as usize] };
			let mut fTemp974: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp972 as usize] };
			let mut fTemp975: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp972, 1)) as usize] } - fTemp974;
			let mut fTemp976: F64 = 65535.0 * fTemp969;
			let mut iTemp977: i32 = (fTemp976) as i32;
			let mut iTemp978: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp977, 65535)))), 196607));
			let mut fTemp979: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp978, 3), 196607))) as usize] };
			let mut fTemp980: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp978 as usize] };
			let mut fTemp981: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp978, 1), 196607))) as usize] } - fTemp980;
			let mut iTemp982: i32 = (fTemp733 > ((if iTemp663 != 0 {fTemp980 + fTemp672 * fTemp981 + (fTemp976 - (iTemp977) as F64) * (fTemp979 - (fTemp980 + fTemp672 * (fTemp981 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp978, 4), 196607))) as usize] } - fTemp979))))} else {1.0 - (fTemp974 + fTemp672 * fTemp975 + (fTemp970 - (iTemp971) as F64) * (fTemp973 - (fTemp974 + fTemp672 * (fTemp975 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp972, 4)) as usize] } - fTemp973)))))} - fTemp968) / (1.0 - fTemp968))) as i32;
			let mut fTemp983: F64 = if iTemp982 != 0 {fTemp952} else {fTemp955};
			let mut fTemp984: F64 = if iTemp982 != 0 {fTemp955} else {fTemp953};
			let mut fTemp985: F64 = fTemp984 + fTemp983;
			let mut fTemp986: F64 = 0.5 * fTemp985;
			let mut fTemp987: F64 = 65535.0 * (1.0 - fTemp986);
			let mut iTemp988: i32 = (fTemp987) as i32;
			let mut iTemp989: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp988, 65535)))), 196607));
			let mut fTemp990: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp989, 3)) as usize] };
			let mut fTemp991: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp989 as usize] };
			let mut fTemp992: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp989, 1)) as usize] } - fTemp991;
			let mut fTemp993: F64 = 32767.5 * fTemp985;
			let mut iTemp994: i32 = (fTemp993) as i32;
			let mut iTemp995: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp994, 65535)))), 196607));
			let mut fTemp996: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp995, 3)) as usize] };
			let mut fTemp997: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp995 as usize] };
			let mut fTemp998: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp995, 1)) as usize] } - fTemp997;
			let mut fTemp999: F64 = if iTemp663 != 0 {fTemp997 + fTemp672 * fTemp998 + (fTemp993 - (iTemp994) as F64) * (fTemp996 - (fTemp997 + fTemp672 * (fTemp998 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp995, 4)) as usize] } - fTemp996))))} else {1.0 - (fTemp991 + fTemp672 * fTemp992 + (fTemp987 - (iTemp988) as F64) * (fTemp990 - (fTemp991 + fTemp672 * (fTemp992 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp989, 4)) as usize] } - fTemp990)))))};
			let mut fTemp1000: F64 = fTemp677 + fTemp986;
			let mut fTemp1001: F64 = 65535.0 * (1.0 - fTemp1000);
			let mut iTemp1002: i32 = (fTemp1001) as i32;
			let mut iTemp1003: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1002, 65535)))), 196607));
			let mut fTemp1004: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1003, 3)) as usize] };
			let mut fTemp1005: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1003 as usize] };
			let mut fTemp1006: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1003, 1)) as usize] } - fTemp1005;
			let mut fTemp1007: F64 = 65535.0 * fTemp1000;
			let mut iTemp1008: i32 = (fTemp1007) as i32;
			let mut iTemp1009: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1008, 65535)))), 196607));
			let mut fTemp1010: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1009, 3), 196607))) as usize] };
			let mut fTemp1011: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1009 as usize] };
			let mut fTemp1012: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1009, 1), 196607))) as usize] } - fTemp1011;
			let mut iTemp1013: i32 = (fTemp733 > ((if iTemp663 != 0 {fTemp1011 + fTemp672 * fTemp1012 + (fTemp1007 - (iTemp1008) as F64) * (fTemp1010 - (fTemp1011 + fTemp672 * (fTemp1012 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1009, 4), 196607))) as usize] } - fTemp1010))))} else {1.0 - (fTemp1005 + fTemp672 * fTemp1006 + (fTemp1001 - (iTemp1002) as F64) * (fTemp1004 - (fTemp1005 + fTemp672 * (fTemp1006 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1003, 4)) as usize] } - fTemp1004)))))} - fTemp999) / (1.0 - fTemp999))) as i32;
			let mut fTemp1014: F64 = if iTemp1013 != 0 {fTemp983} else {fTemp986};
			let mut fTemp1015: F64 = if iTemp1013 != 0 {fTemp986} else {fTemp984};
			let mut fTemp1016: F64 = fTemp1015 + fTemp1014;
			let mut fTemp1017: F64 = 0.5 * fTemp1016;
			let mut fTemp1018: F64 = 65535.0 * (1.0 - fTemp1017);
			let mut iTemp1019: i32 = (fTemp1018) as i32;
			let mut iTemp1020: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1019, 65535)))), 196607));
			let mut fTemp1021: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1020, 3)) as usize] };
			let mut fTemp1022: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1020 as usize] };
			let mut fTemp1023: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1020, 1)) as usize] } - fTemp1022;
			let mut fTemp1024: F64 = 32767.5 * fTemp1016;
			let mut iTemp1025: i32 = (fTemp1024) as i32;
			let mut iTemp1026: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1025, 65535)))), 196607));
			let mut fTemp1027: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1026, 3)) as usize] };
			let mut fTemp1028: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1026 as usize] };
			let mut fTemp1029: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1026, 1)) as usize] } - fTemp1028;
			let mut fTemp1030: F64 = if iTemp663 != 0 {fTemp1028 + fTemp672 * fTemp1029 + (fTemp1024 - (iTemp1025) as F64) * (fTemp1027 - (fTemp1028 + fTemp672 * (fTemp1029 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1026, 4)) as usize] } - fTemp1027))))} else {1.0 - (fTemp1022 + fTemp672 * fTemp1023 + (fTemp1018 - (iTemp1019) as F64) * (fTemp1021 - (fTemp1022 + fTemp672 * (fTemp1023 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1020, 4)) as usize] } - fTemp1021)))))};
			let mut fTemp1031: F64 = fTemp677 + fTemp1017;
			let mut fTemp1032: F64 = 65535.0 * (1.0 - fTemp1031);
			let mut iTemp1033: i32 = (fTemp1032) as i32;
			let mut iTemp1034: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1033, 65535)))), 196607));
			let mut fTemp1035: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1034, 3)) as usize] };
			let mut fTemp1036: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1034 as usize] };
			let mut fTemp1037: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1034, 1)) as usize] } - fTemp1036;
			let mut fTemp1038: F64 = 65535.0 * fTemp1031;
			let mut iTemp1039: i32 = (fTemp1038) as i32;
			let mut iTemp1040: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1039, 65535)))), 196607));
			let mut fTemp1041: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1040, 3), 196607))) as usize] };
			let mut fTemp1042: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1040 as usize] };
			let mut fTemp1043: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1040, 1), 196607))) as usize] } - fTemp1042;
			let mut iTemp1044: i32 = (fTemp733 > ((if iTemp663 != 0 {fTemp1042 + fTemp672 * fTemp1043 + (fTemp1038 - (iTemp1039) as F64) * (fTemp1041 - (fTemp1042 + fTemp672 * (fTemp1043 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1040, 4), 196607))) as usize] } - fTemp1041))))} else {1.0 - (fTemp1036 + fTemp672 * fTemp1037 + (fTemp1032 - (iTemp1033) as F64) * (fTemp1035 - (fTemp1036 + fTemp672 * (fTemp1037 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1034, 4)) as usize] } - fTemp1035)))))} - fTemp1030) / (1.0 - fTemp1030))) as i32;
			let mut fTemp1045: F64 = if iTemp1044 != 0 {fTemp1014} else {fTemp1017};
			let mut fTemp1046: F64 = if iTemp1044 != 0 {fTemp1017} else {fTemp1015};
			let mut fTemp1047: F64 = fTemp1046 + fTemp1045;
			let mut fTemp1048: F64 = 0.5 * fTemp1047;
			let mut fTemp1049: F64 = 65535.0 * (1.0 - fTemp1048);
			let mut iTemp1050: i32 = (fTemp1049) as i32;
			let mut iTemp1051: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1050, 65535)))), 196607));
			let mut fTemp1052: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1051, 3)) as usize] };
			let mut fTemp1053: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1051 as usize] };
			let mut fTemp1054: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1051, 1)) as usize] } - fTemp1053;
			let mut fTemp1055: F64 = 32767.5 * fTemp1047;
			let mut iTemp1056: i32 = (fTemp1055) as i32;
			let mut iTemp1057: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1056, 65535)))), 196607));
			let mut fTemp1058: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1057, 3)) as usize] };
			let mut fTemp1059: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1057 as usize] };
			let mut fTemp1060: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1057, 1)) as usize] } - fTemp1059;
			let mut fTemp1061: F64 = if iTemp663 != 0 {fTemp1059 + fTemp672 * fTemp1060 + (fTemp1055 - (iTemp1056) as F64) * (fTemp1058 - (fTemp1059 + fTemp672 * (fTemp1060 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1057, 4)) as usize] } - fTemp1058))))} else {1.0 - (fTemp1053 + fTemp672 * fTemp1054 + (fTemp1049 - (iTemp1050) as F64) * (fTemp1052 - (fTemp1053 + fTemp672 * (fTemp1054 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1051, 4)) as usize] } - fTemp1052)))))};
			let mut fTemp1062: F64 = fTemp677 + fTemp1048;
			let mut fTemp1063: F64 = 65535.0 * (1.0 - fTemp1062);
			let mut iTemp1064: i32 = (fTemp1063) as i32;
			let mut iTemp1065: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1064, 65535)))), 196607));
			let mut fTemp1066: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1065, 3)) as usize] };
			let mut fTemp1067: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1065 as usize] };
			let mut fTemp1068: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1065, 1)) as usize] } - fTemp1067;
			let mut fTemp1069: F64 = 65535.0 * fTemp1062;
			let mut iTemp1070: i32 = (fTemp1069) as i32;
			let mut iTemp1071: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1070, 65535)))), 196607));
			let mut fTemp1072: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1071, 3), 196607))) as usize] };
			let mut fTemp1073: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1071 as usize] };
			let mut fTemp1074: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1071, 1), 196607))) as usize] } - fTemp1073;
			let mut iTemp1075: i32 = (fTemp733 > ((if iTemp663 != 0 {fTemp1073 + fTemp672 * fTemp1074 + (fTemp1069 - (iTemp1070) as F64) * (fTemp1072 - (fTemp1073 + fTemp672 * (fTemp1074 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1071, 4), 196607))) as usize] } - fTemp1072))))} else {1.0 - (fTemp1067 + fTemp672 * fTemp1068 + (fTemp1063 - (iTemp1064) as F64) * (fTemp1066 - (fTemp1067 + fTemp672 * (fTemp1068 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1065, 4)) as usize] } - fTemp1066)))))} - fTemp1061) / (1.0 - fTemp1061))) as i32;
			let mut fTemp1076: F64 = if iTemp1075 != 0 {fTemp1045} else {fTemp1048};
			let mut fTemp1077: F64 = if iTemp1075 != 0 {fTemp1048} else {fTemp1046};
			let mut fTemp1078: F64 = fTemp1077 + fTemp1076;
			let mut fTemp1079: F64 = 0.5 * fTemp1078;
			let mut fTemp1080: F64 = 65535.0 * (1.0 - fTemp1079);
			let mut iTemp1081: i32 = (fTemp1080) as i32;
			let mut iTemp1082: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1081, 65535)))), 196607));
			let mut fTemp1083: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1082, 3)) as usize] };
			let mut fTemp1084: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1082 as usize] };
			let mut fTemp1085: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1082, 1)) as usize] } - fTemp1084;
			let mut fTemp1086: F64 = 32767.5 * fTemp1078;
			let mut iTemp1087: i32 = (fTemp1086) as i32;
			let mut iTemp1088: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1087, 65535)))), 196607));
			let mut fTemp1089: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1088, 3)) as usize] };
			let mut fTemp1090: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1088 as usize] };
			let mut fTemp1091: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1088, 1)) as usize] } - fTemp1090;
			let mut fTemp1092: F64 = if iTemp663 != 0 {fTemp1090 + fTemp672 * fTemp1091 + (fTemp1086 - (iTemp1087) as F64) * (fTemp1089 - (fTemp1090 + fTemp672 * (fTemp1091 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1088, 4)) as usize] } - fTemp1089))))} else {1.0 - (fTemp1084 + fTemp672 * fTemp1085 + (fTemp1080 - (iTemp1081) as F64) * (fTemp1083 - (fTemp1084 + fTemp672 * (fTemp1085 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1082, 4)) as usize] } - fTemp1083)))))};
			let mut fTemp1093: F64 = fTemp677 + fTemp1079;
			let mut fTemp1094: F64 = 65535.0 * (1.0 - fTemp1093);
			let mut iTemp1095: i32 = (fTemp1094) as i32;
			let mut iTemp1096: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1095, 65535)))), 196607));
			let mut fTemp1097: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1096, 3)) as usize] };
			let mut fTemp1098: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1096 as usize] };
			let mut fTemp1099: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1096, 1)) as usize] } - fTemp1098;
			let mut fTemp1100: F64 = 65535.0 * fTemp1093;
			let mut iTemp1101: i32 = (fTemp1100) as i32;
			let mut iTemp1102: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1101, 65535)))), 196607));
			let mut fTemp1103: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1102, 3), 196607))) as usize] };
			let mut fTemp1104: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1102 as usize] };
			let mut fTemp1105: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1102, 1), 196607))) as usize] } - fTemp1104;
			let mut iTemp1106: i32 = (fTemp733 > ((if iTemp663 != 0 {fTemp1104 + fTemp672 * fTemp1105 + (fTemp1100 - (iTemp1101) as F64) * (fTemp1103 - (fTemp1104 + fTemp672 * (fTemp1105 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1102, 4), 196607))) as usize] } - fTemp1103))))} else {1.0 - (fTemp1098 + fTemp672 * fTemp1099 + (fTemp1094 - (iTemp1095) as F64) * (fTemp1097 - (fTemp1098 + fTemp672 * (fTemp1099 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1096, 4)) as usize] } - fTemp1097)))))} - fTemp1092) / (1.0 - fTemp1092))) as i32;
			let mut fTemp1107: F64 = if iTemp1106 != 0 {fTemp1076} else {fTemp1079};
			let mut fTemp1108: F64 = if iTemp1106 != 0 {fTemp1079} else {fTemp1077};
			let mut fTemp1109: F64 = fTemp1108 + fTemp1107;
			let mut fTemp1110: F64 = 0.5 * fTemp1109;
			let mut fTemp1111: F64 = 65535.0 * (1.0 - fTemp1110);
			let mut iTemp1112: i32 = (fTemp1111) as i32;
			let mut iTemp1113: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1112, 65535)))), 196607));
			let mut fTemp1114: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1113, 3)) as usize] };
			let mut fTemp1115: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1113 as usize] };
			let mut fTemp1116: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1113, 1)) as usize] } - fTemp1115;
			let mut fTemp1117: F64 = 32767.5 * fTemp1109;
			let mut iTemp1118: i32 = (fTemp1117) as i32;
			let mut iTemp1119: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1118, 65535)))), 196607));
			let mut fTemp1120: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1119, 3)) as usize] };
			let mut fTemp1121: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1119 as usize] };
			let mut fTemp1122: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1119, 1)) as usize] } - fTemp1121;
			let mut fTemp1123: F64 = if iTemp663 != 0 {fTemp1121 + fTemp672 * fTemp1122 + (fTemp1117 - (iTemp1118) as F64) * (fTemp1120 - (fTemp1121 + fTemp672 * (fTemp1122 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1119, 4)) as usize] } - fTemp1120))))} else {1.0 - (fTemp1115 + fTemp672 * fTemp1116 + (fTemp1111 - (iTemp1112) as F64) * (fTemp1114 - (fTemp1115 + fTemp672 * (fTemp1116 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1113, 4)) as usize] } - fTemp1114)))))};
			let mut fTemp1124: F64 = fTemp677 + fTemp1110;
			let mut fTemp1125: F64 = 65535.0 * (1.0 - fTemp1124);
			let mut iTemp1126: i32 = (fTemp1125) as i32;
			let mut iTemp1127: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1126, 65535)))), 196607));
			let mut fTemp1128: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1127, 3)) as usize] };
			let mut fTemp1129: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1127 as usize] };
			let mut fTemp1130: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1127, 1)) as usize] } - fTemp1129;
			let mut fTemp1131: F64 = 65535.0 * fTemp1124;
			let mut iTemp1132: i32 = (fTemp1131) as i32;
			let mut iTemp1133: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1132, 65535)))), 196607));
			let mut fTemp1134: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1133, 3), 196607))) as usize] };
			let mut fTemp1135: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1133 as usize] };
			let mut fTemp1136: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1133, 1), 196607))) as usize] } - fTemp1135;
			let mut iTemp1137: i32 = (fTemp733 > ((if iTemp663 != 0 {fTemp1135 + fTemp672 * fTemp1136 + (fTemp1131 - (iTemp1132) as F64) * (fTemp1134 - (fTemp1135 + fTemp672 * (fTemp1136 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1133, 4), 196607))) as usize] } - fTemp1134))))} else {1.0 - (fTemp1129 + fTemp672 * fTemp1130 + (fTemp1125 - (iTemp1126) as F64) * (fTemp1128 - (fTemp1129 + fTemp672 * (fTemp1130 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1127, 4)) as usize] } - fTemp1128)))))} - fTemp1123) / (1.0 - fTemp1123))) as i32;
			let mut fTemp1138: F64 = if iTemp1137 != 0 {fTemp1107} else {fTemp1110};
			let mut fTemp1139: F64 = if iTemp1137 != 0 {fTemp1110} else {fTemp1108};
			let mut fTemp1140: F64 = fTemp1139 + fTemp1138;
			let mut fTemp1141: F64 = 0.5 * fTemp1140;
			let mut fTemp1142: F64 = 65535.0 * (1.0 - fTemp1141);
			let mut iTemp1143: i32 = (fTemp1142) as i32;
			let mut iTemp1144: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1143, 65535)))), 196607));
			let mut fTemp1145: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1144, 3)) as usize] };
			let mut fTemp1146: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1144 as usize] };
			let mut fTemp1147: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1144, 1)) as usize] } - fTemp1146;
			let mut fTemp1148: F64 = 32767.5 * fTemp1140;
			let mut iTemp1149: i32 = (fTemp1148) as i32;
			let mut iTemp1150: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1149, 65535)))), 196607));
			let mut fTemp1151: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1150, 3)) as usize] };
			let mut fTemp1152: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1150 as usize] };
			let mut fTemp1153: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1150, 1)) as usize] } - fTemp1152;
			let mut fTemp1154: F64 = if iTemp663 != 0 {fTemp1152 + fTemp672 * fTemp1153 + (fTemp1148 - (iTemp1149) as F64) * (fTemp1151 - (fTemp1152 + fTemp672 * (fTemp1153 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1150, 4)) as usize] } - fTemp1151))))} else {1.0 - (fTemp1146 + fTemp672 * fTemp1147 + (fTemp1142 - (iTemp1143) as F64) * (fTemp1145 - (fTemp1146 + fTemp672 * (fTemp1147 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1144, 4)) as usize] } - fTemp1145)))))};
			let mut fTemp1155: F64 = fTemp677 + fTemp1141;
			let mut fTemp1156: F64 = 65535.0 * (1.0 - fTemp1155);
			let mut iTemp1157: i32 = (fTemp1156) as i32;
			let mut iTemp1158: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1157, 65535)))), 196607));
			let mut fTemp1159: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1158, 3)) as usize] };
			let mut fTemp1160: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1158 as usize] };
			let mut fTemp1161: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1158, 1)) as usize] } - fTemp1160;
			let mut fTemp1162: F64 = 65535.0 * fTemp1155;
			let mut iTemp1163: i32 = (fTemp1162) as i32;
			let mut iTemp1164: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1163, 65535)))), 196607));
			let mut fTemp1165: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1164, 3), 196607))) as usize] };
			let mut fTemp1166: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1164 as usize] };
			let mut fTemp1167: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1164, 1), 196607))) as usize] } - fTemp1166;
			let mut iTemp1168: i32 = (fTemp733 > ((if iTemp663 != 0 {fTemp1166 + fTemp672 * fTemp1167 + (fTemp1162 - (iTemp1163) as F64) * (fTemp1165 - (fTemp1166 + fTemp672 * (fTemp1167 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1164, 4), 196607))) as usize] } - fTemp1165))))} else {1.0 - (fTemp1160 + fTemp672 * fTemp1161 + (fTemp1156 - (iTemp1157) as F64) * (fTemp1159 - (fTemp1160 + fTemp672 * (fTemp1161 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1158, 4)) as usize] } - fTemp1159)))))} - fTemp1154) / (1.0 - fTemp1154))) as i32;
			let mut fTemp1169: F64 = if iTemp1168 != 0 {fTemp1138} else {fTemp1141};
			let mut fTemp1170: F64 = if iTemp1168 != 0 {fTemp1141} else {fTemp1139};
			let mut fTemp1171: F64 = fTemp1170 + fTemp1169;
			let mut fTemp1172: F64 = 0.5 * fTemp1171;
			let mut fTemp1173: F64 = 65535.0 * (1.0 - fTemp1172);
			let mut iTemp1174: i32 = (fTemp1173) as i32;
			let mut iTemp1175: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1174, 65535)))), 196607));
			let mut fTemp1176: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1175, 3)) as usize] };
			let mut fTemp1177: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1175 as usize] };
			let mut fTemp1178: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1175, 1)) as usize] } - fTemp1177;
			let mut fTemp1179: F64 = 32767.5 * fTemp1171;
			let mut iTemp1180: i32 = (fTemp1179) as i32;
			let mut iTemp1181: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1180, 65535)))), 196607));
			let mut fTemp1182: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1181, 3)) as usize] };
			let mut fTemp1183: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1181 as usize] };
			let mut fTemp1184: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1181, 1)) as usize] } - fTemp1183;
			let mut fTemp1185: F64 = if iTemp663 != 0 {fTemp1183 + fTemp672 * fTemp1184 + (fTemp1179 - (iTemp1180) as F64) * (fTemp1182 - (fTemp1183 + fTemp672 * (fTemp1184 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1181, 4), 196607))) as usize] } - fTemp1182))))} else {1.0 - (fTemp1177 + fTemp672 * fTemp1178 + (fTemp1173 - (iTemp1174) as F64) * (fTemp1176 - (fTemp1177 + fTemp672 * (fTemp1178 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1175, 4), 196607))) as usize] } - fTemp1176)))))};
			let mut fTemp1186: F64 = fTemp677 + fTemp1172;
			let mut fTemp1187: F64 = 65535.0 * (1.0 - fTemp1186);
			let mut iTemp1188: i32 = (fTemp1187) as i32;
			let mut iTemp1189: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1188, 65535)))), 196607));
			let mut fTemp1190: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1189, 3)) as usize] };
			let mut fTemp1191: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1189 as usize] };
			let mut fTemp1192: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1189, 1)) as usize] } - fTemp1191;
			let mut fTemp1193: F64 = 65535.0 * fTemp1186;
			let mut iTemp1194: i32 = (fTemp1193) as i32;
			let mut iTemp1195: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1194, 65535)))), 196607));
			let mut fTemp1196: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1195, 3), 196607))) as usize] };
			let mut fTemp1197: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1195 as usize] };
			let mut fTemp1198: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1195, 1), 196607))) as usize] } - fTemp1197;
			let mut iTemp1199: i32 = (fTemp733 > ((if iTemp663 != 0 {fTemp1197 + fTemp672 * fTemp1198 + (fTemp1193 - (iTemp1194) as F64) * (fTemp1196 - (fTemp1197 + fTemp672 * (fTemp1198 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1195, 4), 196607))) as usize] } - fTemp1196))))} else {1.0 - (fTemp1191 + fTemp672 * fTemp1192 + (fTemp1187 - (iTemp1188) as F64) * (fTemp1190 - (fTemp1191 + fTemp672 * (fTemp1192 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1189, 4)) as usize] } - fTemp1190)))))} - fTemp1185) / (1.0 - fTemp1185))) as i32;
			let mut fTemp1200: F64 = F64::min(1.0, F64::max(0.0, 0.5 * (if iTemp1199 != 0 {fTemp1172} else {fTemp1170} + if iTemp1199 != 0 {fTemp1169} else {fTemp1172})));
			self.fRec15[0] = fTemp1200;
			let mut fTemp1201: F64 = 65535.0 * (1.0 - fTemp1200);
			let mut iTemp1202: i32 = (fTemp1201) as i32;
			let mut iTemp1203: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1202, 65535)))), 196607));
			let mut fTemp1204: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1203, 3)) as usize] };
			let mut fTemp1205: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1203 as usize] };
			let mut fTemp1206: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1203, 1)) as usize] } - fTemp1205;
			let mut fTemp1207: F64 = 65535.0 * fTemp1200;
			let mut iTemp1208: i32 = (fTemp1207) as i32;
			let mut iTemp1209: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1208, 65535)))), 196607));
			let mut fTemp1210: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1209, 3)) as usize] };
			let mut fTemp1211: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1209 as usize] };
			let mut fTemp1212: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1209, 1)) as usize] } - fTemp1211;
			let mut fTemp1213: F64 = if iTemp663 != 0 {fTemp1211 + fTemp672 * fTemp1212 + (fTemp1207 - (iTemp1208) as F64) * (fTemp1210 - (fTemp1211 + fTemp672 * (fTemp1212 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1209, 4), 196607))) as usize] } - fTemp1210))))} else {1.0 - (fTemp1205 + fTemp672 * fTemp1206 + (fTemp1201 - (iTemp1202) as F64) * (fTemp1204 - (fTemp1205 + fTemp672 * (fTemp1206 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1203, 4), 196607))) as usize] } - fTemp1204)))))};
			let mut fTemp1214: F64 = fTemp677 + fTemp1200;
			let mut fTemp1215: F64 = 65535.0 * (1.0 - fTemp1214);
			let mut iTemp1216: i32 = (fTemp1215) as i32;
			let mut iTemp1217: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1216, 65535)))), 196607));
			let mut fTemp1218: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1217, 3)) as usize] };
			let mut fTemp1219: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1217 as usize] };
			let mut fTemp1220: F64 = unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1217, 1)) as usize] } - fTemp1219;
			let mut fTemp1221: F64 = 65535.0 * fTemp1214;
			let mut iTemp1222: i32 = (fTemp1221) as i32;
			let mut iTemp1223: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp667, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1222, 65535)))), 196607));
			let mut fTemp1224: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1223, 3), 196607))) as usize] };
			let mut fTemp1225: F64 = unsafe { ftbl0LambRs96kSIG0[iTemp1223 as usize] };
			let mut fTemp1226: F64 = unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1223, 1), 196607))) as usize] } - fTemp1225;
			let mut fTemp1227: F64 = fTemp635 + if ((0.001 * fTemp676) == 0.0) as i32 != 0 {fTemp662} else {fTemp662 * (if iTemp663 != 0 {fTemp1225 + fTemp672 * fTemp1226 + (fTemp1221 - (iTemp1222) as F64) * (fTemp1224 - (fTemp1225 + fTemp672 * (fTemp1226 - (unsafe { ftbl0LambRs96kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1223, 4), 196607))) as usize] } - fTemp1224))))} else {1.0 - (fTemp1219 + fTemp672 * fTemp1220 + (fTemp1215 - (iTemp1216) as F64) * (fTemp1218 - (fTemp1219 + fTemp672 * (fTemp1220 - (unsafe { ftbl0LambRs96kSIG0[(i32::wrapping_add(iTemp1217, 4)) as usize] } - fTemp1218)))))} - fTemp1213) / (1.0 - fTemp1213)};
			self.fRec16[(self.IOTA0 & 16383) as usize] = if iTemp675 != 0 {F64::min(fTemp1227, fTemp635)} else {F64::max(fTemp1227, fTemp635)};
			let mut fTemp1228: F64 = self.fRec16[((i32::wrapping_sub(self.IOTA0, iSlow74)) & 16383) as usize];
			self.fHbargraph2 = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, fTemp1228));
			*output1 = 0.5 * fTemp2 * self.fVec2[((i32::wrapping_sub(self.IOTA0, iSlow75)) & 32767) as usize] + self.fRec14[0] * fTemp3 * self.fVec3[((i32::wrapping_sub(self.IOTA0, iSlow75)) & 32767) as usize] * fTemp1228;
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
			self.fVec17[2] = self.fVec17[1];
			self.fVec17[1] = self.fVec17[0];
			for j2 in (1..=6).rev() {
				self.fVec18[j2 as usize] = self.fVec18[(i32::wrapping_sub(j2, 1)) as usize];
			}
			for j3 in (1..=14).rev() {
				self.fVec19[j3 as usize] = self.fVec19[(i32::wrapping_sub(j3, 1)) as usize];
			}
			self.fVec29[1] = self.fVec29[0];
			self.fVec30[1] = self.fVec30[0];
			self.fVec31[1] = self.fVec31[0];
			self.fRec1[1] = self.fRec1[0];
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
			self.fVec45[2] = self.fVec45[1];
			self.fVec45[1] = self.fVec45[0];
			for j6 in (1..=6).rev() {
				self.fVec46[j6 as usize] = self.fVec46[(i32::wrapping_sub(j6, 1)) as usize];
			}
			for j7 in (1..=14).rev() {
				self.fVec47[j7 as usize] = self.fVec47[(i32::wrapping_sub(j7, 1)) as usize];
			}
			self.fVec57[1] = self.fVec57[0];
			self.fVec58[1] = self.fVec58[0];
			self.fVec59[1] = self.fVec59[0];
			self.fRec15[1] = self.fRec15[0];
		}
	}

}

}

pub use dsp_96k::LambRs96k;
