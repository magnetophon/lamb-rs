/* ------------------------------------------------------------
author: "Bart Brouns"
license: "AGPLv3"
name: "lamb-rs"
version: "0.1"
Code generated with Faust 2.72.14 (https://faust.grame.fr)
Compilation options: -a /run/user/1001/.tmpls0jT0 -lang rust -ct 1 -cn LambRs48k -es 1 -mcd 16 -mdd 1024 -mdy 33 -double -ftz 0
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
		for l39 in 0..2 {
			self.iRec13[l39 as usize] = 0;
		}
	}
	
	fn fillLambRs48kSIG0(&mut self, count: i32, table: &mut[F64]) {
		for i1 in 0..count {
			self.iRec13[0] = i32::wrapping_add(self.iRec13[1], 1);
			let mut iTemp63: i32 = i32::wrapping_add(self.iRec13[0], -1);
			let mut fTemp64: F64 = (iTemp63 % 3) as F64 as i32 as F64;
			let mut fTemp65: F64 = 0.5 * fTemp64;
			let mut fTemp66: F64 = F64::powf(fTemp65, 0.21 * fTemp64 + 1.0);
			let mut fTemp67: F64 = (0.3333333333333333 * (iTemp63 % 196608) as F64) as i32 as F64;
			table[i1 as usize] = F64::min(1.0, F64::max(0.0, if (fTemp65 == 0.0) as i32 != 0 {0.5 * (F64::sin(4.793763109162727e-05 * fTemp67 + 4.71238898038469) + 1.0)} else {0.5 * (F64::sin(3.141592653589793 * ((1.0 - F64::exp(-(3.692683299000534e-05 * fTemp66 * fTemp67))) / (1.0 - F64::exp(-(2.42 * fTemp66)))) + 4.71238898038469) + 1.0)}));
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
static mut ftbl0LambRs48kSIG0: [F64;196608] = [0.0;196608];
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
	fRec3: [F64;2],
	fVec16: [F64;3],
	fVec17: [F64;7],
	fVec18: [F64;15],
	fVec19: [F64;32],
	fVec20: [F64;64],
	fVec21: [F64;128],
	fVec22: [F64;256],
	fVec23: [F64;512],
	fVec24: [F64;1024],
	fVec25: [F64;2048],
	fVec26: [F64;4096],
	fVec27: [F64;2],
	fHslider10: F64,
	fHslider11: F64,
	fVec28: [F64;2],
	fVec29: [F64;2],
	fConst10: F64,
	fRec1: [F64;2],
	fRec2: [F64;8192],
	fCheckbox1: F64,
	fHbargraph0: F64,
	fHbargraph1: F64,
	fHslider12: F64,
	fRec14: [F64;2],
	fVec30: [F64;16384],
	fVec31: [F64;3],
	fVec32: [F64;7],
	fVec33: [F64;15],
	fVec34: [F64;32],
	fVec35: [F64;64],
	fVec36: [F64;128],
	fVec37: [F64;256],
	fVec38: [F64;512],
	fVec39: [F64;1024],
	fVec40: [F64;2048],
	fVec41: [F64;4096],
	fRec17: [F64;2],
	fVec42: [F64;3],
	fVec43: [F64;7],
	fVec44: [F64;15],
	fVec45: [F64;32],
	fVec46: [F64;64],
	fVec47: [F64;128],
	fVec48: [F64;256],
	fVec49: [F64;512],
	fVec50: [F64;1024],
	fVec51: [F64;2048],
	fVec52: [F64;4096],
	fVec53: [F64;2],
	fVec54: [F64;2],
	fVec55: [F64;2],
	fRec15: [F64;2],
	fRec16: [F64;8192],
	fHbargraph2: F64,
}

impl FaustDsp for LambRs48k {
	type T = F64;
		
	fn new() -> LambRs48k { 
		LambRs48k {
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
			fRec3: [0.0;2],
			fVec16: [0.0;3],
			fVec17: [0.0;7],
			fVec18: [0.0;15],
			fVec19: [0.0;32],
			fVec20: [0.0;64],
			fVec21: [0.0;128],
			fVec22: [0.0;256],
			fVec23: [0.0;512],
			fVec24: [0.0;1024],
			fVec25: [0.0;2048],
			fVec26: [0.0;4096],
			fVec27: [0.0;2],
			fHslider10: 0.0,
			fHslider11: 0.0,
			fVec28: [0.0;2],
			fVec29: [0.0;2],
			fConst10: 0.0,
			fRec1: [0.0;2],
			fRec2: [0.0;8192],
			fCheckbox1: 0.0,
			fHbargraph0: 0.0,
			fHbargraph1: 0.0,
			fHslider12: 0.0,
			fRec14: [0.0;2],
			fVec30: [0.0;16384],
			fVec31: [0.0;3],
			fVec32: [0.0;7],
			fVec33: [0.0;15],
			fVec34: [0.0;32],
			fVec35: [0.0;64],
			fVec36: [0.0;128],
			fVec37: [0.0;256],
			fVec38: [0.0;512],
			fVec39: [0.0;1024],
			fVec40: [0.0;2048],
			fVec41: [0.0;4096],
			fRec17: [0.0;2],
			fVec42: [0.0;3],
			fVec43: [0.0;7],
			fVec44: [0.0;15],
			fVec45: [0.0;32],
			fVec46: [0.0;64],
			fVec47: [0.0;128],
			fVec48: [0.0;256],
			fVec49: [0.0;512],
			fVec50: [0.0;1024],
			fVec51: [0.0;2048],
			fVec52: [0.0;4096],
			fVec53: [0.0;2],
			fVec54: [0.0;2],
			fVec55: [0.0;2],
			fRec15: [0.0;2],
			fRec16: [0.0;8192],
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
		m.declare("compile_options", r"-a /run/user/1001/.tmpls0jT0 -lang rust -ct 1 -cn LambRs48k -es 1 -mcd 16 -mdd 1024 -mdy 33 -double -ftz 0");
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
		return 2;
	}
	
	fn class_init(sample_rate: i32) {
		let mut sig0: LambRs48kSIG0 = newLambRs48kSIG0();
		sig0.instance_initLambRs48kSIG0(sample_rate);
		sig0.fillLambRs48kSIG0(196608, unsafe { &mut ftbl0LambRs48kSIG0 });
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
		for l26 in 0..2 {
			self.fRec3[l26 as usize] = 0.0;
		}
		for l27 in 0..3 {
			self.fVec16[l27 as usize] = 0.0;
		}
		for l28 in 0..7 {
			self.fVec17[l28 as usize] = 0.0;
		}
		for l29 in 0..15 {
			self.fVec18[l29 as usize] = 0.0;
		}
		for l30 in 0..32 {
			self.fVec19[l30 as usize] = 0.0;
		}
		for l31 in 0..64 {
			self.fVec20[l31 as usize] = 0.0;
		}
		for l32 in 0..128 {
			self.fVec21[l32 as usize] = 0.0;
		}
		for l33 in 0..256 {
			self.fVec22[l33 as usize] = 0.0;
		}
		for l34 in 0..512 {
			self.fVec23[l34 as usize] = 0.0;
		}
		for l35 in 0..1024 {
			self.fVec24[l35 as usize] = 0.0;
		}
		for l36 in 0..2048 {
			self.fVec25[l36 as usize] = 0.0;
		}
		for l37 in 0..4096 {
			self.fVec26[l37 as usize] = 0.0;
		}
		for l38 in 0..2 {
			self.fVec27[l38 as usize] = 0.0;
		}
		for l40 in 0..2 {
			self.fVec28[l40 as usize] = 0.0;
		}
		for l41 in 0..2 {
			self.fVec29[l41 as usize] = 0.0;
		}
		for l42 in 0..2 {
			self.fRec1[l42 as usize] = 0.0;
		}
		for l43 in 0..8192 {
			self.fRec2[l43 as usize] = 0.0;
		}
		for l44 in 0..2 {
			self.fRec14[l44 as usize] = 0.0;
		}
		for l45 in 0..16384 {
			self.fVec30[l45 as usize] = 0.0;
		}
		for l46 in 0..3 {
			self.fVec31[l46 as usize] = 0.0;
		}
		for l47 in 0..7 {
			self.fVec32[l47 as usize] = 0.0;
		}
		for l48 in 0..15 {
			self.fVec33[l48 as usize] = 0.0;
		}
		for l49 in 0..32 {
			self.fVec34[l49 as usize] = 0.0;
		}
		for l50 in 0..64 {
			self.fVec35[l50 as usize] = 0.0;
		}
		for l51 in 0..128 {
			self.fVec36[l51 as usize] = 0.0;
		}
		for l52 in 0..256 {
			self.fVec37[l52 as usize] = 0.0;
		}
		for l53 in 0..512 {
			self.fVec38[l53 as usize] = 0.0;
		}
		for l54 in 0..1024 {
			self.fVec39[l54 as usize] = 0.0;
		}
		for l55 in 0..2048 {
			self.fVec40[l55 as usize] = 0.0;
		}
		for l56 in 0..4096 {
			self.fVec41[l56 as usize] = 0.0;
		}
		for l57 in 0..2 {
			self.fRec17[l57 as usize] = 0.0;
		}
		for l58 in 0..3 {
			self.fVec42[l58 as usize] = 0.0;
		}
		for l59 in 0..7 {
			self.fVec43[l59 as usize] = 0.0;
		}
		for l60 in 0..15 {
			self.fVec44[l60 as usize] = 0.0;
		}
		for l61 in 0..32 {
			self.fVec45[l61 as usize] = 0.0;
		}
		for l62 in 0..64 {
			self.fVec46[l62 as usize] = 0.0;
		}
		for l63 in 0..128 {
			self.fVec47[l63 as usize] = 0.0;
		}
		for l64 in 0..256 {
			self.fVec48[l64 as usize] = 0.0;
		}
		for l65 in 0..512 {
			self.fVec49[l65 as usize] = 0.0;
		}
		for l66 in 0..1024 {
			self.fVec50[l66 as usize] = 0.0;
		}
		for l67 in 0..2048 {
			self.fVec51[l67 as usize] = 0.0;
		}
		for l68 in 0..4096 {
			self.fVec52[l68 as usize] = 0.0;
		}
		for l69 in 0..2 {
			self.fVec53[l69 as usize] = 0.0;
		}
		for l70 in 0..2 {
			self.fVec54[l70 as usize] = 0.0;
		}
		for l71 in 0..2 {
			self.fVec55[l71 as usize] = 0.0;
		}
		for l72 in 0..2 {
			self.fRec15[l72 as usize] = 0.0;
		}
		for l73 in 0..8192 {
			self.fRec16[l73 as usize] = 0.0;
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
		ui_interface.declare(None, "99", "");
		ui_interface.open_vertical_box("gain reduction");
		ui_interface.declare(Some(ParamIndex(15)), "unit", "dB");
		ui_interface.add_horizontal_bargraph("0", ParamIndex(15), -24.0, 0.0);
		ui_interface.declare(Some(ParamIndex(16)), "unit", "dB");
		ui_interface.add_horizontal_bargraph("1", ParamIndex(16), -24.0, 0.0);
		ui_interface.close_box();
		ui_interface.declare(Some(ParamIndex(17)), "99", "");
		ui_interface.declare(Some(ParamIndex(17)), "unit", "samples");
		ui_interface.add_horizontal_bargraph("latency", ParamIndex(17), 0.0, 4.8e+03);
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
		self.fHbargraph1 = if (fSlow69) as i32 != 0 {4.8e+03} else {fSlow68};
		let mut iSlow71: i32 = (self.fHbargraph1) as i32;
		let mut fSlow72: F64 = self.fConst3 * F64::powf(1e+01, 0.05 * self.fHslider12);
		let zipped_iterators = inputs0.zip(inputs1).zip(outputs0).zip(outputs1);
		for (((input0, input1), output0), output1) in zipped_iterators {
			let mut fTemp0: F64 = self.fConst1 + self.fRec0[1];
			let mut fTemp1: F64 = self.fRec0[1] - self.fConst1;
			self.fRec0[0] = if (fTemp0 < fSlow0) as i32 != 0 {fTemp0} else {if (fTemp1 > fSlow0) as i32 != 0 {fTemp1} else {fSlow0}};
			let mut fTemp2: F64 = F64::sin(6.283185307179586 * (0.5 * self.fRec0[0] + 0.75)) + 1.0;
			let mut fTemp3: F64 = 1.0 - 0.5 * fTemp2;
			let mut fTemp4: F64 = self.fRec2[((i32::wrapping_sub(self.IOTA0, 1)) & 8191) as usize];
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
			let mut fTemp21: F64 = F64::powf(1e+01, -(0.05 * F64::max(0.0, if (iTemp19 == 0) as i32 != 0 {0.0} else {if (iTemp19 == 1) as i32 != 0 {fSlow12 * LambRs48k_faustpower2_f(fSlow7 + fTemp20)} else {fTemp20}})));
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
			let mut fTemp40: F64 = F64::min(fTemp33, F64::powf(1e+01, -(fSlow18 * F64::max(0.0, if (iTemp38 == 0) as i32 != 0 {0.0} else {if (iTemp38 == 1) as i32 != 0 {fSlow12 * LambRs48k_faustpower2_f(fSlow7 + fTemp39)} else {fTemp39}}))));
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
			self.fVec15[(self.IOTA0 & 4095) as usize] = F64::min(fTemp50, self.fVec14[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			self.fRec3[0] = F64::max(F64::min(self.fRec3[1], self.fVec4[((i32::wrapping_sub(self.IOTA0, iSlow21)) & 16383) as usize]), F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow23 != 0 {fTemp40} else {1.7976931348623157e+308}, if iSlow24 != 0 {self.fVec5[iSlow23 as usize]} else {1.7976931348623157e+308}), if iSlow25 != 0 {self.fVec6[iSlow26 as usize]} else {1.7976931348623157e+308}), if iSlow27 != 0 {self.fVec7[iSlow28 as usize]} else {1.7976931348623157e+308}), if iSlow29 != 0 {self.fVec8[((i32::wrapping_sub(self.IOTA0, iSlow30)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow31 != 0 {self.fVec9[((i32::wrapping_sub(self.IOTA0, iSlow32)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow33 != 0 {self.fVec10[((i32::wrapping_sub(self.IOTA0, iSlow34)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow35 != 0 {self.fVec11[((i32::wrapping_sub(self.IOTA0, iSlow36)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow37 != 0 {self.fVec12[((i32::wrapping_sub(self.IOTA0, iSlow38)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow39 != 0 {self.fVec13[((i32::wrapping_sub(self.IOTA0, iSlow40)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow41 != 0 {self.fVec14[((i32::wrapping_sub(self.IOTA0, iSlow42)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow43 != 0 {self.fVec15[((i32::wrapping_sub(self.IOTA0, iSlow44)) & 4095) as usize]} else {1.7976931348623157e+308}));
			let mut fTemp51: F64 = F64::min(self.fRec3[0], self.fRec3[1]);
			self.fVec16[0] = fTemp51;
			let mut fTemp52: F64 = F64::min(fTemp51, self.fVec16[2]);
			self.fVec17[0] = fTemp52;
			let mut fTemp53: F64 = F64::min(fTemp52, self.fVec17[4]);
			self.fVec18[0] = fTemp53;
			let mut fTemp54: F64 = F64::min(fTemp53, self.fVec18[8]);
			self.fVec19[(self.IOTA0 & 31) as usize] = fTemp54;
			let mut fTemp55: F64 = F64::min(fTemp54, self.fVec19[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize]);
			self.fVec20[(self.IOTA0 & 63) as usize] = fTemp55;
			let mut fTemp56: F64 = F64::min(fTemp55, self.fVec20[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize]);
			self.fVec21[(self.IOTA0 & 127) as usize] = fTemp56;
			let mut fTemp57: F64 = F64::min(fTemp56, self.fVec21[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize]);
			self.fVec22[(self.IOTA0 & 255) as usize] = fTemp57;
			let mut fTemp58: F64 = F64::min(fTemp57, self.fVec22[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize]);
			self.fVec23[(self.IOTA0 & 511) as usize] = fTemp58;
			let mut fTemp59: F64 = F64::min(fTemp58, self.fVec23[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize]);
			self.fVec24[(self.IOTA0 & 1023) as usize] = fTemp59;
			let mut fTemp60: F64 = F64::min(fTemp59, self.fVec24[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize]);
			self.fVec25[(self.IOTA0 & 2047) as usize] = fTemp60;
			self.fVec26[(self.IOTA0 & 4095) as usize] = F64::min(fTemp60, self.fVec25[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			let mut fTemp61: F64 = F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow4 != 0 {self.fRec3[0]} else {1.7976931348623157e+308}, if iSlow45 != 0 {self.fVec16[iSlow4 as usize]} else {1.7976931348623157e+308}), if iSlow46 != 0 {self.fVec17[iSlow47 as usize]} else {1.7976931348623157e+308}), if iSlow48 != 0 {self.fVec18[iSlow49 as usize]} else {1.7976931348623157e+308}), if iSlow50 != 0 {self.fVec19[((i32::wrapping_sub(self.IOTA0, iSlow51)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow52 != 0 {self.fVec20[((i32::wrapping_sub(self.IOTA0, iSlow53)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow54 != 0 {self.fVec21[((i32::wrapping_sub(self.IOTA0, iSlow55)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow56 != 0 {self.fVec22[((i32::wrapping_sub(self.IOTA0, iSlow57)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow58 != 0 {self.fVec23[((i32::wrapping_sub(self.IOTA0, iSlow59)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow60 != 0 {self.fVec24[((i32::wrapping_sub(self.IOTA0, iSlow61)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow62 != 0 {self.fVec25[((i32::wrapping_sub(self.IOTA0, iSlow63)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow64 != 0 {self.fVec26[((i32::wrapping_sub(self.IOTA0, iSlow65)) & 4095) as usize]} else {1.7976931348623157e+308}) - fTemp4;
			self.fVec27[0] = fTemp61;
			let mut iTemp62: i32 = (fTemp61 > 0.0) as i32;
			let mut fTemp68: F64 = if iTemp62 != 0 {fSlow67} else {fSlow66};
			self.fVec28[0] = fTemp68;
			let mut fTemp69: F64 = 2.0 * fTemp68;
			let mut iTemp70: i32 = (fTemp69) as i32;
			let mut iTemp71: i32 = std::cmp::max(0, std::cmp::min(iTemp70, 2));
			let mut iTemp72: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, 98301), 196607));
			let mut fTemp73: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp72, 3)) as usize] };
			let mut fTemp74: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp72 as usize] };
			let mut fTemp75: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp72, 1)) as usize] } - fTemp74;
			let mut fTemp76: F64 = fTemp69 - (iTemp70) as F64;
			let mut fTemp77: F64 = fTemp74 + fTemp76 * fTemp75 + 0.5 * (fTemp73 - (fTemp74 + fTemp76 * (fTemp75 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp72, 4)) as usize] } - fTemp73))));
			let mut fTemp78: F64 = if iTemp62 != 0 {fTemp77} else {1.0 - fTemp77};
			let mut iTemp79: i32 = (fTemp61 < 0.0) as i32;
			let mut fTemp80: F64 = fSlow1 * (iTemp79) as F64 + fSlow13 * (iTemp62) as F64;
			self.fVec29[0] = fTemp80;
			let mut fTemp81: F64 = self.fConst10 / fTemp80;
			let mut fTemp82: F64 = fTemp81 + 0.5;
			let mut fTemp83: F64 = 65535.0 * (1.0 - fTemp82);
			let mut iTemp84: i32 = (fTemp83) as i32;
			let mut iTemp85: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp84, 65535)))), 196607));
			let mut fTemp86: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp85, 3)) as usize] };
			let mut fTemp87: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp85 as usize] };
			let mut fTemp88: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp85, 1)) as usize] } - fTemp87;
			let mut fTemp89: F64 = 65535.0 * fTemp82;
			let mut iTemp90: i32 = (fTemp89) as i32;
			let mut iTemp91: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp90, 65535)))), 196607));
			let mut fTemp92: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp91, 3), 196607))) as usize] };
			let mut fTemp93: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp91 as usize] };
			let mut fTemp94: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp91, 1), 196607))) as usize] } - fTemp93;
			let mut fTemp95: F64 = 2.0 * self.fVec28[1];
			let mut iTemp96: i32 = (fTemp95) as i32;
			let mut iTemp97: i32 = std::cmp::max(0, std::cmp::min(iTemp96, 2));
			let mut fTemp98: F64 = 65535.0 * (1.0 - self.fRec1[1]);
			let mut iTemp99: i32 = (fTemp98) as i32;
			let mut iTemp100: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp99, 65535))), iTemp97), 196607));
			let mut fTemp101: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp100, 3), 196607))) as usize] };
			let mut fTemp102: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp100 as usize] };
			let mut fTemp103: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp100, 1), 196607))) as usize] } - fTemp102;
			let mut fTemp104: F64 = fTemp95 - (iTemp96) as F64;
			let mut fTemp105: F64 = 65535.0 * self.fRec1[1];
			let mut iTemp106: i32 = (fTemp105) as i32;
			let mut iTemp107: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp97, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp106, 65535)))), 196607));
			let mut fTemp108: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp107, 3), 196607))) as usize] };
			let mut fTemp109: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp107 as usize] };
			let mut fTemp110: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp107, 1), 196607))) as usize] } - fTemp109;
			let mut fTemp111: F64 = self.fRec1[1] + fTemp81;
			let mut fTemp112: F64 = 65535.0 * (1.0 - fTemp111);
			let mut iTemp113: i32 = (fTemp112) as i32;
			let mut iTemp114: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp113, 65535)))), 196607));
			let mut fTemp115: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp114, 3)) as usize] };
			let mut fTemp116: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp114 as usize] };
			let mut fTemp117: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp114, 1)) as usize] } - fTemp116;
			let mut fTemp118: F64 = 65535.0 * fTemp111;
			let mut iTemp119: i32 = (fTemp118) as i32;
			let mut iTemp120: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp119, 65535)))), 196607));
			let mut fTemp121: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp120, 3), 196607))) as usize] };
			let mut fTemp122: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp120 as usize] };
			let mut fTemp123: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp120, 1), 196607))) as usize] } - fTemp122;
			let mut fTemp124: F64 = self.fRec1[1] + self.fConst10 * (1.0 / fTemp80 + 1.0 / self.fVec29[1]);
			let mut fTemp125: F64 = 65535.0 * (1.0 - fTemp124);
			let mut iTemp126: i32 = (fTemp125) as i32;
			let mut iTemp127: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp126, 65535))), iTemp71), 196607));
			let mut fTemp128: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp127, 3)) as usize] };
			let mut fTemp129: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp127 as usize] };
			let mut fTemp130: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp127, 1)) as usize] } - fTemp129;
			let mut fTemp131: F64 = 65535.0 * fTemp124;
			let mut iTemp132: i32 = (fTemp131) as i32;
			let mut iTemp133: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp132, 65535)))), 196607));
			let mut fTemp134: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp133, 3), 196607))) as usize] };
			let mut fTemp135: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp133 as usize] };
			let mut fTemp136: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp133, 1), 196607))) as usize] } - fTemp135;
			let mut fTemp137: F64 = (if iTemp62 != 0 {fTemp135 + fTemp76 * fTemp136 + (fTemp131 - (iTemp132) as F64) * (fTemp134 - (fTemp135 + fTemp76 * (fTemp136 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp133, 4), 196607))) as usize] } - fTemp134))))} else {1.0 - (fTemp129 + fTemp76 * fTemp130 + (fTemp125 - (iTemp126) as F64) * (fTemp128 - (fTemp129 + fTemp76 * (fTemp130 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp127, 4)) as usize] } - fTemp128)))))} - if iTemp62 != 0 {fTemp122 + fTemp76 * fTemp123 + (fTemp118 - (iTemp119) as F64) * (fTemp121 - (fTemp122 + fTemp76 * (fTemp123 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp120, 4), 196607))) as usize] } - fTemp121))))} else {1.0 - (fTemp116 + fTemp76 * fTemp117 + (fTemp112 - (iTemp113) as F64) * (fTemp115 - (fTemp116 + fTemp76 * (fTemp117 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp114, 4), 196607))) as usize] } - fTemp115)))))}) * self.fVec27[1] / (fTemp61 * (1.0 - if iTemp62 != 0 {fTemp109 + fTemp104 * fTemp110 + (fTemp105 - (iTemp106) as F64) * (fTemp108 - (fTemp109 + fTemp104 * (fTemp110 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp107, 4), 196607))) as usize] } - fTemp108))))} else {1.0 - (fTemp102 + fTemp104 * fTemp103 + (fTemp98 - (iTemp99) as F64) * (fTemp101 - (fTemp102 + fTemp104 * (fTemp103 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp100, 4), 196607))) as usize] } - fTemp101)))))}));
			let mut iTemp138: i32 = (fTemp137 > ((if iTemp62 != 0 {fTemp93 + fTemp76 * fTemp94 + (fTemp89 - (iTemp90) as F64) * (fTemp92 - (fTemp93 + fTemp76 * (fTemp94 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp91, 4), 196607))) as usize] } - fTemp92))))} else {1.0 - (fTemp87 + fTemp76 * fTemp88 + (fTemp83 - (iTemp84) as F64) * (fTemp86 - (fTemp87 + fTemp76 * (fTemp88 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp85, 4)) as usize] } - fTemp86)))))} - fTemp78) / (1.0 - fTemp78))) as i32;
			let mut fTemp139: F64 = if iTemp138 != 0 {1.0} else {0.5};
			let mut fTemp140: F64 = if iTemp138 != 0 {0.5} else {0.0};
			let mut fTemp141: F64 = fTemp140 + fTemp139;
			let mut fTemp142: F64 = 0.5 * fTemp141;
			let mut fTemp143: F64 = 65535.0 * (1.0 - fTemp142);
			let mut iTemp144: i32 = (fTemp143) as i32;
			let mut iTemp145: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp144, 65535)))), 196607));
			let mut fTemp146: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp145, 3)) as usize] };
			let mut fTemp147: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp145 as usize] };
			let mut fTemp148: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp145, 1)) as usize] } - fTemp147;
			let mut fTemp149: F64 = 32767.5 * fTemp141;
			let mut iTemp150: i32 = (fTemp149) as i32;
			let mut iTemp151: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp150, 65535)))), 196607));
			let mut fTemp152: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp151, 3)) as usize] };
			let mut fTemp153: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp151 as usize] };
			let mut fTemp154: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp151, 1)) as usize] } - fTemp153;
			let mut fTemp155: F64 = if iTemp62 != 0 {fTemp153 + fTemp76 * fTemp154 + (fTemp149 - (iTemp150) as F64) * (fTemp152 - (fTemp153 + fTemp76 * (fTemp154 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp151, 4)) as usize] } - fTemp152))))} else {1.0 - (fTemp147 + fTemp76 * fTemp148 + (fTemp143 - (iTemp144) as F64) * (fTemp146 - (fTemp147 + fTemp76 * (fTemp148 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp145, 4)) as usize] } - fTemp146)))))};
			let mut fTemp156: F64 = fTemp81 + fTemp142;
			let mut fTemp157: F64 = 65535.0 * (1.0 - fTemp156);
			let mut iTemp158: i32 = (fTemp157) as i32;
			let mut iTemp159: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp158, 65535)))), 196607));
			let mut fTemp160: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp159, 3)) as usize] };
			let mut fTemp161: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp159 as usize] };
			let mut fTemp162: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp159, 1)) as usize] } - fTemp161;
			let mut fTemp163: F64 = 65535.0 * fTemp156;
			let mut iTemp164: i32 = (fTemp163) as i32;
			let mut iTemp165: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp164, 65535)))), 196607));
			let mut fTemp166: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp165, 3), 196607))) as usize] };
			let mut fTemp167: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp165 as usize] };
			let mut fTemp168: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp165, 1), 196607))) as usize] } - fTemp167;
			let mut iTemp169: i32 = (fTemp137 > ((if iTemp62 != 0 {fTemp167 + fTemp76 * fTemp168 + (fTemp163 - (iTemp164) as F64) * (fTemp166 - (fTemp167 + fTemp76 * (fTemp168 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp165, 4), 196607))) as usize] } - fTemp166))))} else {1.0 - (fTemp161 + fTemp76 * fTemp162 + (fTemp157 - (iTemp158) as F64) * (fTemp160 - (fTemp161 + fTemp76 * (fTemp162 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp159, 4)) as usize] } - fTemp160)))))} - fTemp155) / (1.0 - fTemp155))) as i32;
			let mut fTemp170: F64 = if iTemp169 != 0 {fTemp139} else {fTemp142};
			let mut fTemp171: F64 = if iTemp169 != 0 {fTemp142} else {fTemp140};
			let mut fTemp172: F64 = fTemp171 + fTemp170;
			let mut fTemp173: F64 = 0.5 * fTemp172;
			let mut fTemp174: F64 = 65535.0 * (1.0 - fTemp173);
			let mut iTemp175: i32 = (fTemp174) as i32;
			let mut iTemp176: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp175, 65535)))), 196607));
			let mut fTemp177: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp176, 3)) as usize] };
			let mut fTemp178: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp176 as usize] };
			let mut fTemp179: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp176, 1)) as usize] } - fTemp178;
			let mut fTemp180: F64 = 32767.5 * fTemp172;
			let mut iTemp181: i32 = (fTemp180) as i32;
			let mut iTemp182: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp181, 65535)))), 196607));
			let mut fTemp183: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp182, 3)) as usize] };
			let mut fTemp184: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp182 as usize] };
			let mut fTemp185: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp182, 1)) as usize] } - fTemp184;
			let mut fTemp186: F64 = if iTemp62 != 0 {fTemp184 + fTemp76 * fTemp185 + (fTemp180 - (iTemp181) as F64) * (fTemp183 - (fTemp184 + fTemp76 * (fTemp185 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp182, 4)) as usize] } - fTemp183))))} else {1.0 - (fTemp178 + fTemp76 * fTemp179 + (fTemp174 - (iTemp175) as F64) * (fTemp177 - (fTemp178 + fTemp76 * (fTemp179 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp176, 4)) as usize] } - fTemp177)))))};
			let mut fTemp187: F64 = fTemp81 + fTemp173;
			let mut fTemp188: F64 = 65535.0 * (1.0 - fTemp187);
			let mut iTemp189: i32 = (fTemp188) as i32;
			let mut iTemp190: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp189, 65535)))), 196607));
			let mut fTemp191: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp190, 3)) as usize] };
			let mut fTemp192: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp190 as usize] };
			let mut fTemp193: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp190, 1)) as usize] } - fTemp192;
			let mut fTemp194: F64 = 65535.0 * fTemp187;
			let mut iTemp195: i32 = (fTemp194) as i32;
			let mut iTemp196: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp195, 65535)))), 196607));
			let mut fTemp197: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp196, 3), 196607))) as usize] };
			let mut fTemp198: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp196 as usize] };
			let mut fTemp199: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp196, 1), 196607))) as usize] } - fTemp198;
			let mut iTemp200: i32 = (fTemp137 > ((if iTemp62 != 0 {fTemp198 + fTemp76 * fTemp199 + (fTemp194 - (iTemp195) as F64) * (fTemp197 - (fTemp198 + fTemp76 * (fTemp199 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp196, 4), 196607))) as usize] } - fTemp197))))} else {1.0 - (fTemp192 + fTemp76 * fTemp193 + (fTemp188 - (iTemp189) as F64) * (fTemp191 - (fTemp192 + fTemp76 * (fTemp193 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp190, 4)) as usize] } - fTemp191)))))} - fTemp186) / (1.0 - fTemp186))) as i32;
			let mut fTemp201: F64 = if iTemp200 != 0 {fTemp170} else {fTemp173};
			let mut fTemp202: F64 = if iTemp200 != 0 {fTemp173} else {fTemp171};
			let mut fTemp203: F64 = fTemp202 + fTemp201;
			let mut fTemp204: F64 = 0.5 * fTemp203;
			let mut fTemp205: F64 = 65535.0 * (1.0 - fTemp204);
			let mut iTemp206: i32 = (fTemp205) as i32;
			let mut iTemp207: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp206, 65535)))), 196607));
			let mut fTemp208: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp207, 3)) as usize] };
			let mut fTemp209: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp207 as usize] };
			let mut fTemp210: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp207, 1)) as usize] } - fTemp209;
			let mut fTemp211: F64 = 32767.5 * fTemp203;
			let mut iTemp212: i32 = (fTemp211) as i32;
			let mut iTemp213: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp212, 65535)))), 196607));
			let mut fTemp214: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp213, 3)) as usize] };
			let mut fTemp215: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp213 as usize] };
			let mut fTemp216: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp213, 1)) as usize] } - fTemp215;
			let mut fTemp217: F64 = if iTemp62 != 0 {fTemp215 + fTemp76 * fTemp216 + (fTemp211 - (iTemp212) as F64) * (fTemp214 - (fTemp215 + fTemp76 * (fTemp216 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp213, 4)) as usize] } - fTemp214))))} else {1.0 - (fTemp209 + fTemp76 * fTemp210 + (fTemp205 - (iTemp206) as F64) * (fTemp208 - (fTemp209 + fTemp76 * (fTemp210 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp207, 4)) as usize] } - fTemp208)))))};
			let mut fTemp218: F64 = fTemp81 + fTemp204;
			let mut fTemp219: F64 = 65535.0 * (1.0 - fTemp218);
			let mut iTemp220: i32 = (fTemp219) as i32;
			let mut iTemp221: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp220, 65535)))), 196607));
			let mut fTemp222: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp221, 3)) as usize] };
			let mut fTemp223: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp221 as usize] };
			let mut fTemp224: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp221, 1)) as usize] } - fTemp223;
			let mut fTemp225: F64 = 65535.0 * fTemp218;
			let mut iTemp226: i32 = (fTemp225) as i32;
			let mut iTemp227: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp226, 65535)))), 196607));
			let mut fTemp228: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp227, 3), 196607))) as usize] };
			let mut fTemp229: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp227 as usize] };
			let mut fTemp230: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp227, 1), 196607))) as usize] } - fTemp229;
			let mut iTemp231: i32 = (fTemp137 > ((if iTemp62 != 0 {fTemp229 + fTemp76 * fTemp230 + (fTemp225 - (iTemp226) as F64) * (fTemp228 - (fTemp229 + fTemp76 * (fTemp230 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp227, 4), 196607))) as usize] } - fTemp228))))} else {1.0 - (fTemp223 + fTemp76 * fTemp224 + (fTemp219 - (iTemp220) as F64) * (fTemp222 - (fTemp223 + fTemp76 * (fTemp224 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp221, 4)) as usize] } - fTemp222)))))} - fTemp217) / (1.0 - fTemp217))) as i32;
			let mut fTemp232: F64 = if iTemp231 != 0 {fTemp201} else {fTemp204};
			let mut fTemp233: F64 = if iTemp231 != 0 {fTemp204} else {fTemp202};
			let mut fTemp234: F64 = fTemp233 + fTemp232;
			let mut fTemp235: F64 = 0.5 * fTemp234;
			let mut fTemp236: F64 = 65535.0 * (1.0 - fTemp235);
			let mut iTemp237: i32 = (fTemp236) as i32;
			let mut iTemp238: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp237, 65535)))), 196607));
			let mut fTemp239: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp238, 3)) as usize] };
			let mut fTemp240: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp238 as usize] };
			let mut fTemp241: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp238, 1)) as usize] } - fTemp240;
			let mut fTemp242: F64 = 32767.5 * fTemp234;
			let mut iTemp243: i32 = (fTemp242) as i32;
			let mut iTemp244: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp243, 65535)))), 196607));
			let mut fTemp245: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp244, 3)) as usize] };
			let mut fTemp246: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp244 as usize] };
			let mut fTemp247: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp244, 1)) as usize] } - fTemp246;
			let mut fTemp248: F64 = if iTemp62 != 0 {fTemp246 + fTemp76 * fTemp247 + (fTemp242 - (iTemp243) as F64) * (fTemp245 - (fTemp246 + fTemp76 * (fTemp247 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp244, 4)) as usize] } - fTemp245))))} else {1.0 - (fTemp240 + fTemp76 * fTemp241 + (fTemp236 - (iTemp237) as F64) * (fTemp239 - (fTemp240 + fTemp76 * (fTemp241 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp238, 4)) as usize] } - fTemp239)))))};
			let mut fTemp249: F64 = fTemp81 + fTemp235;
			let mut fTemp250: F64 = 65535.0 * (1.0 - fTemp249);
			let mut iTemp251: i32 = (fTemp250) as i32;
			let mut iTemp252: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp251, 65535)))), 196607));
			let mut fTemp253: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp252, 3)) as usize] };
			let mut fTemp254: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp252 as usize] };
			let mut fTemp255: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp252, 1)) as usize] } - fTemp254;
			let mut fTemp256: F64 = 65535.0 * fTemp249;
			let mut iTemp257: i32 = (fTemp256) as i32;
			let mut iTemp258: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp257, 65535)))), 196607));
			let mut fTemp259: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp258, 3), 196607))) as usize] };
			let mut fTemp260: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp258 as usize] };
			let mut fTemp261: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp258, 1), 196607))) as usize] } - fTemp260;
			let mut iTemp262: i32 = (fTemp137 > ((if iTemp62 != 0 {fTemp260 + fTemp76 * fTemp261 + (fTemp256 - (iTemp257) as F64) * (fTemp259 - (fTemp260 + fTemp76 * (fTemp261 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp258, 4), 196607))) as usize] } - fTemp259))))} else {1.0 - (fTemp254 + fTemp76 * fTemp255 + (fTemp250 - (iTemp251) as F64) * (fTemp253 - (fTemp254 + fTemp76 * (fTemp255 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp252, 4)) as usize] } - fTemp253)))))} - fTemp248) / (1.0 - fTemp248))) as i32;
			let mut fTemp263: F64 = if iTemp262 != 0 {fTemp232} else {fTemp235};
			let mut fTemp264: F64 = if iTemp262 != 0 {fTemp235} else {fTemp233};
			let mut fTemp265: F64 = fTemp264 + fTemp263;
			let mut fTemp266: F64 = 0.5 * fTemp265;
			let mut fTemp267: F64 = 65535.0 * (1.0 - fTemp266);
			let mut iTemp268: i32 = (fTemp267) as i32;
			let mut iTemp269: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp268, 65535)))), 196607));
			let mut fTemp270: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp269, 3)) as usize] };
			let mut fTemp271: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp269 as usize] };
			let mut fTemp272: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp269, 1)) as usize] } - fTemp271;
			let mut fTemp273: F64 = 32767.5 * fTemp265;
			let mut iTemp274: i32 = (fTemp273) as i32;
			let mut iTemp275: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp274, 65535)))), 196607));
			let mut fTemp276: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp275, 3)) as usize] };
			let mut fTemp277: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp275 as usize] };
			let mut fTemp278: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp275, 1)) as usize] } - fTemp277;
			let mut fTemp279: F64 = if iTemp62 != 0 {fTemp277 + fTemp76 * fTemp278 + (fTemp273 - (iTemp274) as F64) * (fTemp276 - (fTemp277 + fTemp76 * (fTemp278 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp275, 4)) as usize] } - fTemp276))))} else {1.0 - (fTemp271 + fTemp76 * fTemp272 + (fTemp267 - (iTemp268) as F64) * (fTemp270 - (fTemp271 + fTemp76 * (fTemp272 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp269, 4)) as usize] } - fTemp270)))))};
			let mut fTemp280: F64 = fTemp81 + fTemp266;
			let mut fTemp281: F64 = 65535.0 * (1.0 - fTemp280);
			let mut iTemp282: i32 = (fTemp281) as i32;
			let mut iTemp283: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp282, 65535)))), 196607));
			let mut fTemp284: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp283, 3)) as usize] };
			let mut fTemp285: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp283 as usize] };
			let mut fTemp286: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp283, 1)) as usize] } - fTemp285;
			let mut fTemp287: F64 = 65535.0 * fTemp280;
			let mut iTemp288: i32 = (fTemp287) as i32;
			let mut iTemp289: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp288, 65535)))), 196607));
			let mut fTemp290: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp289, 3), 196607))) as usize] };
			let mut fTemp291: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp289 as usize] };
			let mut fTemp292: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp289, 1), 196607))) as usize] } - fTemp291;
			let mut iTemp293: i32 = (fTemp137 > ((if iTemp62 != 0 {fTemp291 + fTemp76 * fTemp292 + (fTemp287 - (iTemp288) as F64) * (fTemp290 - (fTemp291 + fTemp76 * (fTemp292 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp289, 4), 196607))) as usize] } - fTemp290))))} else {1.0 - (fTemp285 + fTemp76 * fTemp286 + (fTemp281 - (iTemp282) as F64) * (fTemp284 - (fTemp285 + fTemp76 * (fTemp286 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp283, 4)) as usize] } - fTemp284)))))} - fTemp279) / (1.0 - fTemp279))) as i32;
			let mut fTemp294: F64 = if iTemp293 != 0 {fTemp263} else {fTemp266};
			let mut fTemp295: F64 = if iTemp293 != 0 {fTemp266} else {fTemp264};
			let mut fTemp296: F64 = fTemp295 + fTemp294;
			let mut fTemp297: F64 = 0.5 * fTemp296;
			let mut fTemp298: F64 = 65535.0 * (1.0 - fTemp297);
			let mut iTemp299: i32 = (fTemp298) as i32;
			let mut iTemp300: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp299, 65535)))), 196607));
			let mut fTemp301: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp300, 3)) as usize] };
			let mut fTemp302: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp300 as usize] };
			let mut fTemp303: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp300, 1)) as usize] } - fTemp302;
			let mut fTemp304: F64 = 32767.5 * fTemp296;
			let mut iTemp305: i32 = (fTemp304) as i32;
			let mut iTemp306: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp305, 65535)))), 196607));
			let mut fTemp307: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp306, 3)) as usize] };
			let mut fTemp308: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp306 as usize] };
			let mut fTemp309: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp306, 1)) as usize] } - fTemp308;
			let mut fTemp310: F64 = if iTemp62 != 0 {fTemp308 + fTemp76 * fTemp309 + (fTemp304 - (iTemp305) as F64) * (fTemp307 - (fTemp308 + fTemp76 * (fTemp309 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp306, 4)) as usize] } - fTemp307))))} else {1.0 - (fTemp302 + fTemp76 * fTemp303 + (fTemp298 - (iTemp299) as F64) * (fTemp301 - (fTemp302 + fTemp76 * (fTemp303 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp300, 4)) as usize] } - fTemp301)))))};
			let mut fTemp311: F64 = fTemp81 + fTemp297;
			let mut fTemp312: F64 = 65535.0 * (1.0 - fTemp311);
			let mut iTemp313: i32 = (fTemp312) as i32;
			let mut iTemp314: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp313, 65535)))), 196607));
			let mut fTemp315: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp314, 3)) as usize] };
			let mut fTemp316: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp314 as usize] };
			let mut fTemp317: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp314, 1)) as usize] } - fTemp316;
			let mut fTemp318: F64 = 65535.0 * fTemp311;
			let mut iTemp319: i32 = (fTemp318) as i32;
			let mut iTemp320: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp319, 65535)))), 196607));
			let mut fTemp321: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp320, 3), 196607))) as usize] };
			let mut fTemp322: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp320 as usize] };
			let mut fTemp323: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp320, 1), 196607))) as usize] } - fTemp322;
			let mut iTemp324: i32 = (fTemp137 > ((if iTemp62 != 0 {fTemp322 + fTemp76 * fTemp323 + (fTemp318 - (iTemp319) as F64) * (fTemp321 - (fTemp322 + fTemp76 * (fTemp323 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp320, 4), 196607))) as usize] } - fTemp321))))} else {1.0 - (fTemp316 + fTemp76 * fTemp317 + (fTemp312 - (iTemp313) as F64) * (fTemp315 - (fTemp316 + fTemp76 * (fTemp317 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp314, 4)) as usize] } - fTemp315)))))} - fTemp310) / (1.0 - fTemp310))) as i32;
			let mut fTemp325: F64 = if iTemp324 != 0 {fTemp294} else {fTemp297};
			let mut fTemp326: F64 = if iTemp324 != 0 {fTemp297} else {fTemp295};
			let mut fTemp327: F64 = fTemp326 + fTemp325;
			let mut fTemp328: F64 = 0.5 * fTemp327;
			let mut fTemp329: F64 = 65535.0 * (1.0 - fTemp328);
			let mut iTemp330: i32 = (fTemp329) as i32;
			let mut iTemp331: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp330, 65535)))), 196607));
			let mut fTemp332: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp331, 3)) as usize] };
			let mut fTemp333: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp331 as usize] };
			let mut fTemp334: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp331, 1)) as usize] } - fTemp333;
			let mut fTemp335: F64 = 32767.5 * fTemp327;
			let mut iTemp336: i32 = (fTemp335) as i32;
			let mut iTemp337: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp336, 65535)))), 196607));
			let mut fTemp338: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp337, 3)) as usize] };
			let mut fTemp339: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp337 as usize] };
			let mut fTemp340: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp337, 1)) as usize] } - fTemp339;
			let mut fTemp341: F64 = if iTemp62 != 0 {fTemp339 + fTemp76 * fTemp340 + (fTemp335 - (iTemp336) as F64) * (fTemp338 - (fTemp339 + fTemp76 * (fTemp340 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp337, 4)) as usize] } - fTemp338))))} else {1.0 - (fTemp333 + fTemp76 * fTemp334 + (fTemp329 - (iTemp330) as F64) * (fTemp332 - (fTemp333 + fTemp76 * (fTemp334 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp331, 4)) as usize] } - fTemp332)))))};
			let mut fTemp342: F64 = fTemp81 + fTemp328;
			let mut fTemp343: F64 = 65535.0 * (1.0 - fTemp342);
			let mut iTemp344: i32 = (fTemp343) as i32;
			let mut iTemp345: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp344, 65535)))), 196607));
			let mut fTemp346: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp345, 3)) as usize] };
			let mut fTemp347: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp345 as usize] };
			let mut fTemp348: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp345, 1)) as usize] } - fTemp347;
			let mut fTemp349: F64 = 65535.0 * fTemp342;
			let mut iTemp350: i32 = (fTemp349) as i32;
			let mut iTemp351: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp350, 65535)))), 196607));
			let mut fTemp352: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp351, 3), 196607))) as usize] };
			let mut fTemp353: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp351 as usize] };
			let mut fTemp354: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp351, 1), 196607))) as usize] } - fTemp353;
			let mut iTemp355: i32 = (fTemp137 > ((if iTemp62 != 0 {fTemp353 + fTemp76 * fTemp354 + (fTemp349 - (iTemp350) as F64) * (fTemp352 - (fTemp353 + fTemp76 * (fTemp354 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp351, 4), 196607))) as usize] } - fTemp352))))} else {1.0 - (fTemp347 + fTemp76 * fTemp348 + (fTemp343 - (iTemp344) as F64) * (fTemp346 - (fTemp347 + fTemp76 * (fTemp348 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp345, 4)) as usize] } - fTemp346)))))} - fTemp341) / (1.0 - fTemp341))) as i32;
			let mut fTemp356: F64 = if iTemp355 != 0 {fTemp325} else {fTemp328};
			let mut fTemp357: F64 = if iTemp355 != 0 {fTemp328} else {fTemp326};
			let mut fTemp358: F64 = fTemp357 + fTemp356;
			let mut fTemp359: F64 = 0.5 * fTemp358;
			let mut fTemp360: F64 = 65535.0 * (1.0 - fTemp359);
			let mut iTemp361: i32 = (fTemp360) as i32;
			let mut iTemp362: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp361, 65535)))), 196607));
			let mut fTemp363: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp362, 3)) as usize] };
			let mut fTemp364: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp362 as usize] };
			let mut fTemp365: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp362, 1)) as usize] } - fTemp364;
			let mut fTemp366: F64 = 32767.5 * fTemp358;
			let mut iTemp367: i32 = (fTemp366) as i32;
			let mut iTemp368: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp367, 65535)))), 196607));
			let mut fTemp369: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp368, 3)) as usize] };
			let mut fTemp370: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp368 as usize] };
			let mut fTemp371: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp368, 1)) as usize] } - fTemp370;
			let mut fTemp372: F64 = if iTemp62 != 0 {fTemp370 + fTemp76 * fTemp371 + (fTemp366 - (iTemp367) as F64) * (fTemp369 - (fTemp370 + fTemp76 * (fTemp371 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp368, 4)) as usize] } - fTemp369))))} else {1.0 - (fTemp364 + fTemp76 * fTemp365 + (fTemp360 - (iTemp361) as F64) * (fTemp363 - (fTemp364 + fTemp76 * (fTemp365 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp362, 4)) as usize] } - fTemp363)))))};
			let mut fTemp373: F64 = fTemp81 + fTemp359;
			let mut fTemp374: F64 = 65535.0 * (1.0 - fTemp373);
			let mut iTemp375: i32 = (fTemp374) as i32;
			let mut iTemp376: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp375, 65535)))), 196607));
			let mut fTemp377: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp376, 3)) as usize] };
			let mut fTemp378: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp376 as usize] };
			let mut fTemp379: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp376, 1)) as usize] } - fTemp378;
			let mut fTemp380: F64 = 65535.0 * fTemp373;
			let mut iTemp381: i32 = (fTemp380) as i32;
			let mut iTemp382: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp381, 65535)))), 196607));
			let mut fTemp383: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp382, 3), 196607))) as usize] };
			let mut fTemp384: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp382 as usize] };
			let mut fTemp385: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp382, 1), 196607))) as usize] } - fTemp384;
			let mut iTemp386: i32 = (fTemp137 > ((if iTemp62 != 0 {fTemp384 + fTemp76 * fTemp385 + (fTemp380 - (iTemp381) as F64) * (fTemp383 - (fTemp384 + fTemp76 * (fTemp385 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp382, 4), 196607))) as usize] } - fTemp383))))} else {1.0 - (fTemp378 + fTemp76 * fTemp379 + (fTemp374 - (iTemp375) as F64) * (fTemp377 - (fTemp378 + fTemp76 * (fTemp379 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp376, 4)) as usize] } - fTemp377)))))} - fTemp372) / (1.0 - fTemp372))) as i32;
			let mut fTemp387: F64 = if iTemp386 != 0 {fTemp356} else {fTemp359};
			let mut fTemp388: F64 = if iTemp386 != 0 {fTemp359} else {fTemp357};
			let mut fTemp389: F64 = fTemp388 + fTemp387;
			let mut fTemp390: F64 = 0.5 * fTemp389;
			let mut fTemp391: F64 = 65535.0 * (1.0 - fTemp390);
			let mut iTemp392: i32 = (fTemp391) as i32;
			let mut iTemp393: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp392, 65535)))), 196607));
			let mut fTemp394: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp393, 3)) as usize] };
			let mut fTemp395: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp393 as usize] };
			let mut fTemp396: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp393, 1)) as usize] } - fTemp395;
			let mut fTemp397: F64 = 32767.5 * fTemp389;
			let mut iTemp398: i32 = (fTemp397) as i32;
			let mut iTemp399: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp398, 65535)))), 196607));
			let mut fTemp400: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp399, 3)) as usize] };
			let mut fTemp401: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp399 as usize] };
			let mut fTemp402: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp399, 1)) as usize] } - fTemp401;
			let mut fTemp403: F64 = if iTemp62 != 0 {fTemp401 + fTemp76 * fTemp402 + (fTemp397 - (iTemp398) as F64) * (fTemp400 - (fTemp401 + fTemp76 * (fTemp402 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp399, 4)) as usize] } - fTemp400))))} else {1.0 - (fTemp395 + fTemp76 * fTemp396 + (fTemp391 - (iTemp392) as F64) * (fTemp394 - (fTemp395 + fTemp76 * (fTemp396 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp393, 4)) as usize] } - fTemp394)))))};
			let mut fTemp404: F64 = fTemp81 + fTemp390;
			let mut fTemp405: F64 = 65535.0 * (1.0 - fTemp404);
			let mut iTemp406: i32 = (fTemp405) as i32;
			let mut iTemp407: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp406, 65535)))), 196607));
			let mut fTemp408: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp407, 3)) as usize] };
			let mut fTemp409: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp407 as usize] };
			let mut fTemp410: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp407, 1)) as usize] } - fTemp409;
			let mut fTemp411: F64 = 65535.0 * fTemp404;
			let mut iTemp412: i32 = (fTemp411) as i32;
			let mut iTemp413: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp412, 65535)))), 196607));
			let mut fTemp414: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp413, 3), 196607))) as usize] };
			let mut fTemp415: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp413 as usize] };
			let mut fTemp416: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp413, 1), 196607))) as usize] } - fTemp415;
			let mut iTemp417: i32 = (fTemp137 > ((if iTemp62 != 0 {fTemp415 + fTemp76 * fTemp416 + (fTemp411 - (iTemp412) as F64) * (fTemp414 - (fTemp415 + fTemp76 * (fTemp416 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp413, 4), 196607))) as usize] } - fTemp414))))} else {1.0 - (fTemp409 + fTemp76 * fTemp410 + (fTemp405 - (iTemp406) as F64) * (fTemp408 - (fTemp409 + fTemp76 * (fTemp410 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp407, 4)) as usize] } - fTemp408)))))} - fTemp403) / (1.0 - fTemp403))) as i32;
			let mut fTemp418: F64 = if iTemp417 != 0 {fTemp387} else {fTemp390};
			let mut fTemp419: F64 = if iTemp417 != 0 {fTemp390} else {fTemp388};
			let mut fTemp420: F64 = fTemp419 + fTemp418;
			let mut fTemp421: F64 = 0.5 * fTemp420;
			let mut fTemp422: F64 = 65535.0 * (1.0 - fTemp421);
			let mut iTemp423: i32 = (fTemp422) as i32;
			let mut iTemp424: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp423, 65535)))), 196607));
			let mut fTemp425: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp424, 3)) as usize] };
			let mut fTemp426: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp424 as usize] };
			let mut fTemp427: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp424, 1)) as usize] } - fTemp426;
			let mut fTemp428: F64 = 32767.5 * fTemp420;
			let mut iTemp429: i32 = (fTemp428) as i32;
			let mut iTemp430: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp429, 65535)))), 196607));
			let mut fTemp431: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp430, 3)) as usize] };
			let mut fTemp432: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp430 as usize] };
			let mut fTemp433: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp430, 1)) as usize] } - fTemp432;
			let mut fTemp434: F64 = if iTemp62 != 0 {fTemp432 + fTemp76 * fTemp433 + (fTemp428 - (iTemp429) as F64) * (fTemp431 - (fTemp432 + fTemp76 * (fTemp433 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp430, 4)) as usize] } - fTemp431))))} else {1.0 - (fTemp426 + fTemp76 * fTemp427 + (fTemp422 - (iTemp423) as F64) * (fTemp425 - (fTemp426 + fTemp76 * (fTemp427 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp424, 4)) as usize] } - fTemp425)))))};
			let mut fTemp435: F64 = fTemp81 + fTemp421;
			let mut fTemp436: F64 = 65535.0 * (1.0 - fTemp435);
			let mut iTemp437: i32 = (fTemp436) as i32;
			let mut iTemp438: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp437, 65535)))), 196607));
			let mut fTemp439: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp438, 3)) as usize] };
			let mut fTemp440: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp438 as usize] };
			let mut fTemp441: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp438, 1)) as usize] } - fTemp440;
			let mut fTemp442: F64 = 65535.0 * fTemp435;
			let mut iTemp443: i32 = (fTemp442) as i32;
			let mut iTemp444: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp443, 65535)))), 196607));
			let mut fTemp445: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp444, 3), 196607))) as usize] };
			let mut fTemp446: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp444 as usize] };
			let mut fTemp447: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp444, 1), 196607))) as usize] } - fTemp446;
			let mut iTemp448: i32 = (fTemp137 > ((if iTemp62 != 0 {fTemp446 + fTemp76 * fTemp447 + (fTemp442 - (iTemp443) as F64) * (fTemp445 - (fTemp446 + fTemp76 * (fTemp447 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp444, 4), 196607))) as usize] } - fTemp445))))} else {1.0 - (fTemp440 + fTemp76 * fTemp441 + (fTemp436 - (iTemp437) as F64) * (fTemp439 - (fTemp440 + fTemp76 * (fTemp441 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp438, 4)) as usize] } - fTemp439)))))} - fTemp434) / (1.0 - fTemp434))) as i32;
			let mut fTemp449: F64 = if iTemp448 != 0 {fTemp418} else {fTemp421};
			let mut fTemp450: F64 = if iTemp448 != 0 {fTemp421} else {fTemp419};
			let mut fTemp451: F64 = fTemp450 + fTemp449;
			let mut fTemp452: F64 = 0.5 * fTemp451;
			let mut fTemp453: F64 = 65535.0 * (1.0 - fTemp452);
			let mut iTemp454: i32 = (fTemp453) as i32;
			let mut iTemp455: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp454, 65535)))), 196607));
			let mut fTemp456: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp455, 3)) as usize] };
			let mut fTemp457: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp455 as usize] };
			let mut fTemp458: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp455, 1)) as usize] } - fTemp457;
			let mut fTemp459: F64 = 32767.5 * fTemp451;
			let mut iTemp460: i32 = (fTemp459) as i32;
			let mut iTemp461: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp460, 65535)))), 196607));
			let mut fTemp462: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp461, 3)) as usize] };
			let mut fTemp463: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp461 as usize] };
			let mut fTemp464: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp461, 1)) as usize] } - fTemp463;
			let mut fTemp465: F64 = if iTemp62 != 0 {fTemp463 + fTemp76 * fTemp464 + (fTemp459 - (iTemp460) as F64) * (fTemp462 - (fTemp463 + fTemp76 * (fTemp464 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp461, 4)) as usize] } - fTemp462))))} else {1.0 - (fTemp457 + fTemp76 * fTemp458 + (fTemp453 - (iTemp454) as F64) * (fTemp456 - (fTemp457 + fTemp76 * (fTemp458 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp455, 4)) as usize] } - fTemp456)))))};
			let mut fTemp466: F64 = fTemp81 + fTemp452;
			let mut fTemp467: F64 = 65535.0 * (1.0 - fTemp466);
			let mut iTemp468: i32 = (fTemp467) as i32;
			let mut iTemp469: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp468, 65535)))), 196607));
			let mut fTemp470: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp469, 3)) as usize] };
			let mut fTemp471: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp469 as usize] };
			let mut fTemp472: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp469, 1)) as usize] } - fTemp471;
			let mut fTemp473: F64 = 65535.0 * fTemp466;
			let mut iTemp474: i32 = (fTemp473) as i32;
			let mut iTemp475: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp474, 65535)))), 196607));
			let mut fTemp476: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp475, 3), 196607))) as usize] };
			let mut fTemp477: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp475 as usize] };
			let mut fTemp478: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp475, 1), 196607))) as usize] } - fTemp477;
			let mut iTemp479: i32 = (fTemp137 > ((if iTemp62 != 0 {fTemp477 + fTemp76 * fTemp478 + (fTemp473 - (iTemp474) as F64) * (fTemp476 - (fTemp477 + fTemp76 * (fTemp478 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp475, 4), 196607))) as usize] } - fTemp476))))} else {1.0 - (fTemp471 + fTemp76 * fTemp472 + (fTemp467 - (iTemp468) as F64) * (fTemp470 - (fTemp471 + fTemp76 * (fTemp472 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp469, 4)) as usize] } - fTemp470)))))} - fTemp465) / (1.0 - fTemp465))) as i32;
			let mut fTemp480: F64 = if iTemp479 != 0 {fTemp449} else {fTemp452};
			let mut fTemp481: F64 = if iTemp479 != 0 {fTemp452} else {fTemp450};
			let mut fTemp482: F64 = fTemp481 + fTemp480;
			let mut fTemp483: F64 = 0.5 * fTemp482;
			let mut fTemp484: F64 = 65535.0 * (1.0 - fTemp483);
			let mut iTemp485: i32 = (fTemp484) as i32;
			let mut iTemp486: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp485, 65535)))), 196607));
			let mut fTemp487: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp486, 3)) as usize] };
			let mut fTemp488: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp486 as usize] };
			let mut fTemp489: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp486, 1)) as usize] } - fTemp488;
			let mut fTemp490: F64 = 32767.5 * fTemp482;
			let mut iTemp491: i32 = (fTemp490) as i32;
			let mut iTemp492: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp491, 65535)))), 196607));
			let mut fTemp493: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp492, 3)) as usize] };
			let mut fTemp494: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp492 as usize] };
			let mut fTemp495: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp492, 1)) as usize] } - fTemp494;
			let mut fTemp496: F64 = if iTemp62 != 0 {fTemp494 + fTemp76 * fTemp495 + (fTemp490 - (iTemp491) as F64) * (fTemp493 - (fTemp494 + fTemp76 * (fTemp495 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp492, 4)) as usize] } - fTemp493))))} else {1.0 - (fTemp488 + fTemp76 * fTemp489 + (fTemp484 - (iTemp485) as F64) * (fTemp487 - (fTemp488 + fTemp76 * (fTemp489 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp486, 4)) as usize] } - fTemp487)))))};
			let mut fTemp497: F64 = fTemp81 + fTemp483;
			let mut fTemp498: F64 = 65535.0 * (1.0 - fTemp497);
			let mut iTemp499: i32 = (fTemp498) as i32;
			let mut iTemp500: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp499, 65535)))), 196607));
			let mut fTemp501: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp500, 3)) as usize] };
			let mut fTemp502: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp500 as usize] };
			let mut fTemp503: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp500, 1)) as usize] } - fTemp502;
			let mut fTemp504: F64 = 65535.0 * fTemp497;
			let mut iTemp505: i32 = (fTemp504) as i32;
			let mut iTemp506: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp505, 65535)))), 196607));
			let mut fTemp507: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp506, 3), 196607))) as usize] };
			let mut fTemp508: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp506 as usize] };
			let mut fTemp509: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp506, 1), 196607))) as usize] } - fTemp508;
			let mut iTemp510: i32 = (fTemp137 > ((if iTemp62 != 0 {fTemp508 + fTemp76 * fTemp509 + (fTemp504 - (iTemp505) as F64) * (fTemp507 - (fTemp508 + fTemp76 * (fTemp509 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp506, 4), 196607))) as usize] } - fTemp507))))} else {1.0 - (fTemp502 + fTemp76 * fTemp503 + (fTemp498 - (iTemp499) as F64) * (fTemp501 - (fTemp502 + fTemp76 * (fTemp503 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp500, 4)) as usize] } - fTemp501)))))} - fTemp496) / (1.0 - fTemp496))) as i32;
			let mut fTemp511: F64 = if iTemp510 != 0 {fTemp480} else {fTemp483};
			let mut fTemp512: F64 = if iTemp510 != 0 {fTemp483} else {fTemp481};
			let mut fTemp513: F64 = fTemp512 + fTemp511;
			let mut fTemp514: F64 = 0.5 * fTemp513;
			let mut fTemp515: F64 = 65535.0 * (1.0 - fTemp514);
			let mut iTemp516: i32 = (fTemp515) as i32;
			let mut iTemp517: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp516, 65535)))), 196607));
			let mut fTemp518: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp517, 3)) as usize] };
			let mut fTemp519: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp517 as usize] };
			let mut fTemp520: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp517, 1)) as usize] } - fTemp519;
			let mut fTemp521: F64 = 32767.5 * fTemp513;
			let mut iTemp522: i32 = (fTemp521) as i32;
			let mut iTemp523: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp522, 65535)))), 196607));
			let mut fTemp524: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp523, 3)) as usize] };
			let mut fTemp525: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp523 as usize] };
			let mut fTemp526: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp523, 1)) as usize] } - fTemp525;
			let mut fTemp527: F64 = if iTemp62 != 0 {fTemp525 + fTemp76 * fTemp526 + (fTemp521 - (iTemp522) as F64) * (fTemp524 - (fTemp525 + fTemp76 * (fTemp526 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp523, 4)) as usize] } - fTemp524))))} else {1.0 - (fTemp519 + fTemp76 * fTemp520 + (fTemp515 - (iTemp516) as F64) * (fTemp518 - (fTemp519 + fTemp76 * (fTemp520 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp517, 4)) as usize] } - fTemp518)))))};
			let mut fTemp528: F64 = fTemp81 + fTemp514;
			let mut fTemp529: F64 = 65535.0 * (1.0 - fTemp528);
			let mut iTemp530: i32 = (fTemp529) as i32;
			let mut iTemp531: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp530, 65535)))), 196607));
			let mut fTemp532: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp531, 3)) as usize] };
			let mut fTemp533: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp531 as usize] };
			let mut fTemp534: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp531, 1)) as usize] } - fTemp533;
			let mut fTemp535: F64 = 65535.0 * fTemp528;
			let mut iTemp536: i32 = (fTemp535) as i32;
			let mut iTemp537: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp536, 65535)))), 196607));
			let mut fTemp538: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp537, 3), 196607))) as usize] };
			let mut fTemp539: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp537 as usize] };
			let mut fTemp540: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp537, 1), 196607))) as usize] } - fTemp539;
			let mut iTemp541: i32 = (fTemp137 > ((if iTemp62 != 0 {fTemp539 + fTemp76 * fTemp540 + (fTemp535 - (iTemp536) as F64) * (fTemp538 - (fTemp539 + fTemp76 * (fTemp540 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp537, 4), 196607))) as usize] } - fTemp538))))} else {1.0 - (fTemp533 + fTemp76 * fTemp534 + (fTemp529 - (iTemp530) as F64) * (fTemp532 - (fTemp533 + fTemp76 * (fTemp534 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp531, 4)) as usize] } - fTemp532)))))} - fTemp527) / (1.0 - fTemp527))) as i32;
			let mut fTemp542: F64 = if iTemp541 != 0 {fTemp511} else {fTemp514};
			let mut fTemp543: F64 = if iTemp541 != 0 {fTemp514} else {fTemp512};
			let mut fTemp544: F64 = fTemp543 + fTemp542;
			let mut fTemp545: F64 = 0.5 * fTemp544;
			let mut fTemp546: F64 = 65535.0 * (1.0 - fTemp545);
			let mut iTemp547: i32 = (fTemp546) as i32;
			let mut iTemp548: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp547, 65535)))), 196607));
			let mut fTemp549: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp548, 3)) as usize] };
			let mut fTemp550: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp548 as usize] };
			let mut fTemp551: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp548, 1)) as usize] } - fTemp550;
			let mut fTemp552: F64 = 32767.5 * fTemp544;
			let mut iTemp553: i32 = (fTemp552) as i32;
			let mut iTemp554: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp553, 65535)))), 196607));
			let mut fTemp555: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp554, 3)) as usize] };
			let mut fTemp556: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp554 as usize] };
			let mut fTemp557: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp554, 1)) as usize] } - fTemp556;
			let mut fTemp558: F64 = if iTemp62 != 0 {fTemp556 + fTemp76 * fTemp557 + (fTemp552 - (iTemp553) as F64) * (fTemp555 - (fTemp556 + fTemp76 * (fTemp557 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp554, 4)) as usize] } - fTemp555))))} else {1.0 - (fTemp550 + fTemp76 * fTemp551 + (fTemp546 - (iTemp547) as F64) * (fTemp549 - (fTemp550 + fTemp76 * (fTemp551 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp548, 4)) as usize] } - fTemp549)))))};
			let mut fTemp559: F64 = fTemp81 + fTemp545;
			let mut fTemp560: F64 = 65535.0 * (1.0 - fTemp559);
			let mut iTemp561: i32 = (fTemp560) as i32;
			let mut iTemp562: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp561, 65535)))), 196607));
			let mut fTemp563: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp562, 3)) as usize] };
			let mut fTemp564: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp562 as usize] };
			let mut fTemp565: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp562, 1)) as usize] } - fTemp564;
			let mut fTemp566: F64 = 65535.0 * fTemp559;
			let mut iTemp567: i32 = (fTemp566) as i32;
			let mut iTemp568: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp567, 65535)))), 196607));
			let mut fTemp569: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp568, 3), 196607))) as usize] };
			let mut fTemp570: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp568 as usize] };
			let mut fTemp571: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp568, 1), 196607))) as usize] } - fTemp570;
			let mut iTemp572: i32 = (fTemp137 > ((if iTemp62 != 0 {fTemp570 + fTemp76 * fTemp571 + (fTemp566 - (iTemp567) as F64) * (fTemp569 - (fTemp570 + fTemp76 * (fTemp571 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp568, 4), 196607))) as usize] } - fTemp569))))} else {1.0 - (fTemp564 + fTemp76 * fTemp565 + (fTemp560 - (iTemp561) as F64) * (fTemp563 - (fTemp564 + fTemp76 * (fTemp565 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp562, 4)) as usize] } - fTemp563)))))} - fTemp558) / (1.0 - fTemp558))) as i32;
			let mut fTemp573: F64 = if iTemp572 != 0 {fTemp542} else {fTemp545};
			let mut fTemp574: F64 = if iTemp572 != 0 {fTemp545} else {fTemp543};
			let mut fTemp575: F64 = fTemp574 + fTemp573;
			let mut fTemp576: F64 = 0.5 * fTemp575;
			let mut fTemp577: F64 = 65535.0 * (1.0 - fTemp576);
			let mut iTemp578: i32 = (fTemp577) as i32;
			let mut iTemp579: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp578, 65535)))), 196607));
			let mut fTemp580: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp579, 3)) as usize] };
			let mut fTemp581: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp579 as usize] };
			let mut fTemp582: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp579, 1)) as usize] } - fTemp581;
			let mut fTemp583: F64 = 32767.5 * fTemp575;
			let mut iTemp584: i32 = (fTemp583) as i32;
			let mut iTemp585: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp584, 65535)))), 196607));
			let mut fTemp586: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp585, 3)) as usize] };
			let mut fTemp587: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp585 as usize] };
			let mut fTemp588: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp585, 1)) as usize] } - fTemp587;
			let mut fTemp589: F64 = if iTemp62 != 0 {fTemp587 + fTemp76 * fTemp588 + (fTemp583 - (iTemp584) as F64) * (fTemp586 - (fTemp587 + fTemp76 * (fTemp588 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp585, 4), 196607))) as usize] } - fTemp586))))} else {1.0 - (fTemp581 + fTemp76 * fTemp582 + (fTemp577 - (iTemp578) as F64) * (fTemp580 - (fTemp581 + fTemp76 * (fTemp582 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp579, 4), 196607))) as usize] } - fTemp580)))))};
			let mut fTemp590: F64 = fTemp81 + fTemp576;
			let mut fTemp591: F64 = 65535.0 * (1.0 - fTemp590);
			let mut iTemp592: i32 = (fTemp591) as i32;
			let mut iTemp593: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp592, 65535)))), 196607));
			let mut fTemp594: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp593, 3)) as usize] };
			let mut fTemp595: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp593 as usize] };
			let mut fTemp596: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp593, 1)) as usize] } - fTemp595;
			let mut fTemp597: F64 = 65535.0 * fTemp590;
			let mut iTemp598: i32 = (fTemp597) as i32;
			let mut iTemp599: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp598, 65535)))), 196607));
			let mut fTemp600: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp599, 3), 196607))) as usize] };
			let mut fTemp601: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp599 as usize] };
			let mut fTemp602: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp599, 1), 196607))) as usize] } - fTemp601;
			let mut iTemp603: i32 = (fTemp137 > ((if iTemp62 != 0 {fTemp601 + fTemp76 * fTemp602 + (fTemp597 - (iTemp598) as F64) * (fTemp600 - (fTemp601 + fTemp76 * (fTemp602 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp599, 4), 196607))) as usize] } - fTemp600))))} else {1.0 - (fTemp595 + fTemp76 * fTemp596 + (fTemp591 - (iTemp592) as F64) * (fTemp594 - (fTemp595 + fTemp76 * (fTemp596 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp593, 4)) as usize] } - fTemp594)))))} - fTemp589) / (1.0 - fTemp589))) as i32;
			let mut fTemp604: F64 = F64::min(1.0, F64::max(0.0, 0.5 * (if iTemp603 != 0 {fTemp576} else {fTemp574} + if iTemp603 != 0 {fTemp573} else {fTemp576})));
			self.fRec1[0] = fTemp604;
			let mut fTemp605: F64 = 65535.0 * (1.0 - fTemp604);
			let mut iTemp606: i32 = (fTemp605) as i32;
			let mut iTemp607: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp606, 65535)))), 196607));
			let mut fTemp608: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp607, 3)) as usize] };
			let mut fTemp609: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp607 as usize] };
			let mut fTemp610: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp607, 1)) as usize] } - fTemp609;
			let mut fTemp611: F64 = 65535.0 * fTemp604;
			let mut iTemp612: i32 = (fTemp611) as i32;
			let mut iTemp613: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp612, 65535)))), 196607));
			let mut fTemp614: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp613, 3)) as usize] };
			let mut fTemp615: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp613 as usize] };
			let mut fTemp616: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp613, 1)) as usize] } - fTemp615;
			let mut fTemp617: F64 = if iTemp62 != 0 {fTemp615 + fTemp76 * fTemp616 + (fTemp611 - (iTemp612) as F64) * (fTemp614 - (fTemp615 + fTemp76 * (fTemp616 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp613, 4), 196607))) as usize] } - fTemp614))))} else {1.0 - (fTemp609 + fTemp76 * fTemp610 + (fTemp605 - (iTemp606) as F64) * (fTemp608 - (fTemp609 + fTemp76 * (fTemp610 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp607, 4), 196607))) as usize] } - fTemp608)))))};
			let mut fTemp618: F64 = fTemp81 + fTemp604;
			let mut fTemp619: F64 = 65535.0 * (1.0 - fTemp618);
			let mut iTemp620: i32 = (fTemp619) as i32;
			let mut iTemp621: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp620, 65535)))), 196607));
			let mut fTemp622: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp621, 3)) as usize] };
			let mut fTemp623: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp621 as usize] };
			let mut fTemp624: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp621, 1)) as usize] } - fTemp623;
			let mut fTemp625: F64 = 65535.0 * fTemp618;
			let mut iTemp626: i32 = (fTemp625) as i32;
			let mut iTemp627: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp71, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp626, 65535)))), 196607));
			let mut fTemp628: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp627, 3), 196607))) as usize] };
			let mut fTemp629: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp627 as usize] };
			let mut fTemp630: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp627, 1), 196607))) as usize] } - fTemp629;
			let mut fTemp631: F64 = fTemp4 + if ((0.001 * fTemp80) == 0.0) as i32 != 0 {fTemp61} else {fTemp61 * (if iTemp62 != 0 {fTemp629 + fTemp76 * fTemp630 + (fTemp625 - (iTemp626) as F64) * (fTemp628 - (fTemp629 + fTemp76 * (fTemp630 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp627, 4), 196607))) as usize] } - fTemp628))))} else {1.0 - (fTemp623 + fTemp76 * fTemp624 + (fTemp619 - (iTemp620) as F64) * (fTemp622 - (fTemp623 + fTemp76 * (fTemp624 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp621, 4)) as usize] } - fTemp622)))))} - fTemp617) / (1.0 - fTemp617)};
			self.fRec2[(self.IOTA0 & 8191) as usize] = if iTemp79 != 0 {F64::min(fTemp631, fTemp4)} else {F64::max(fTemp631, fTemp4)};
			let mut fTemp632: F64 = self.fRec2[((i32::wrapping_sub(self.IOTA0, iSlow70)) & 8191) as usize];
			self.fHbargraph0 = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, fTemp632));
			self.fRec14[0] = fSlow72 + self.fConst4 * self.fRec14[1];
			*output0 = 0.5 * self.fVec0[((i32::wrapping_sub(self.IOTA0, iSlow71)) & 32767) as usize] * fTemp2 + self.fRec14[0] * self.fVec1[((i32::wrapping_sub(self.IOTA0, iSlow71)) & 32767) as usize] * fTemp632 * fTemp3;
			let mut fTemp633: F64 = self.fRec16[((i32::wrapping_sub(self.IOTA0, 1)) & 8191) as usize];
			let mut fTemp634: F64 = fTemp35 + fSlow17 * (fTemp36 - fTemp35);
			let mut iTemp635: i32 = ((fTemp634 > fSlow11) as i32) + ((fTemp634 > fSlow9) as i32);
			let mut fTemp636: F64 = fTemp634 - fSlow8;
			let mut fTemp637: F64 = F64::min(fTemp33, F64::powf(1e+01, -(fSlow18 * F64::max(0.0, if (iTemp635 == 0) as i32 != 0 {0.0} else {if (iTemp635 == 1) as i32 != 0 {fSlow12 * LambRs48k_faustpower2_f(fSlow7 + fTemp636)} else {fTemp636}}))));
			self.fVec30[(self.IOTA0 & 16383) as usize] = fTemp637;
			let mut fTemp638: F64 = F64::min(fTemp637, self.fVec30[((i32::wrapping_sub(self.IOTA0, 1)) & 16383) as usize]);
			self.fVec31[0] = fTemp638;
			let mut fTemp639: F64 = F64::min(fTemp638, self.fVec31[2]);
			self.fVec32[0] = fTemp639;
			let mut fTemp640: F64 = F64::min(fTemp639, self.fVec32[4]);
			self.fVec33[0] = fTemp640;
			let mut fTemp641: F64 = F64::min(fTemp640, self.fVec33[8]);
			self.fVec34[(self.IOTA0 & 31) as usize] = fTemp641;
			let mut fTemp642: F64 = F64::min(fTemp641, self.fVec34[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize]);
			self.fVec35[(self.IOTA0 & 63) as usize] = fTemp642;
			let mut fTemp643: F64 = F64::min(fTemp642, self.fVec35[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize]);
			self.fVec36[(self.IOTA0 & 127) as usize] = fTemp643;
			let mut fTemp644: F64 = F64::min(fTemp643, self.fVec36[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize]);
			self.fVec37[(self.IOTA0 & 255) as usize] = fTemp644;
			let mut fTemp645: F64 = F64::min(fTemp644, self.fVec37[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize]);
			self.fVec38[(self.IOTA0 & 511) as usize] = fTemp645;
			let mut fTemp646: F64 = F64::min(fTemp645, self.fVec38[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize]);
			self.fVec39[(self.IOTA0 & 1023) as usize] = fTemp646;
			let mut fTemp647: F64 = F64::min(fTemp646, self.fVec39[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize]);
			self.fVec40[(self.IOTA0 & 2047) as usize] = fTemp647;
			self.fVec41[(self.IOTA0 & 4095) as usize] = F64::min(fTemp647, self.fVec40[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			self.fRec17[0] = F64::max(F64::min(self.fRec17[1], self.fVec30[((i32::wrapping_sub(self.IOTA0, iSlow21)) & 16383) as usize]), F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow23 != 0 {fTemp637} else {1.7976931348623157e+308}, if iSlow24 != 0 {self.fVec31[iSlow23 as usize]} else {1.7976931348623157e+308}), if iSlow25 != 0 {self.fVec32[iSlow26 as usize]} else {1.7976931348623157e+308}), if iSlow27 != 0 {self.fVec33[iSlow28 as usize]} else {1.7976931348623157e+308}), if iSlow29 != 0 {self.fVec34[((i32::wrapping_sub(self.IOTA0, iSlow30)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow31 != 0 {self.fVec35[((i32::wrapping_sub(self.IOTA0, iSlow32)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow33 != 0 {self.fVec36[((i32::wrapping_sub(self.IOTA0, iSlow34)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow35 != 0 {self.fVec37[((i32::wrapping_sub(self.IOTA0, iSlow36)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow37 != 0 {self.fVec38[((i32::wrapping_sub(self.IOTA0, iSlow38)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow39 != 0 {self.fVec39[((i32::wrapping_sub(self.IOTA0, iSlow40)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow41 != 0 {self.fVec40[((i32::wrapping_sub(self.IOTA0, iSlow42)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow43 != 0 {self.fVec41[((i32::wrapping_sub(self.IOTA0, iSlow44)) & 4095) as usize]} else {1.7976931348623157e+308}));
			let mut fTemp648: F64 = F64::min(self.fRec17[0], self.fRec17[1]);
			self.fVec42[0] = fTemp648;
			let mut fTemp649: F64 = F64::min(fTemp648, self.fVec42[2]);
			self.fVec43[0] = fTemp649;
			let mut fTemp650: F64 = F64::min(fTemp649, self.fVec43[4]);
			self.fVec44[0] = fTemp650;
			let mut fTemp651: F64 = F64::min(fTemp650, self.fVec44[8]);
			self.fVec45[(self.IOTA0 & 31) as usize] = fTemp651;
			let mut fTemp652: F64 = F64::min(fTemp651, self.fVec45[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize]);
			self.fVec46[(self.IOTA0 & 63) as usize] = fTemp652;
			let mut fTemp653: F64 = F64::min(fTemp652, self.fVec46[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize]);
			self.fVec47[(self.IOTA0 & 127) as usize] = fTemp653;
			let mut fTemp654: F64 = F64::min(fTemp653, self.fVec47[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize]);
			self.fVec48[(self.IOTA0 & 255) as usize] = fTemp654;
			let mut fTemp655: F64 = F64::min(fTemp654, self.fVec48[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize]);
			self.fVec49[(self.IOTA0 & 511) as usize] = fTemp655;
			let mut fTemp656: F64 = F64::min(fTemp655, self.fVec49[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize]);
			self.fVec50[(self.IOTA0 & 1023) as usize] = fTemp656;
			let mut fTemp657: F64 = F64::min(fTemp656, self.fVec50[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize]);
			self.fVec51[(self.IOTA0 & 2047) as usize] = fTemp657;
			self.fVec52[(self.IOTA0 & 4095) as usize] = F64::min(fTemp657, self.fVec51[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			let mut fTemp658: F64 = F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow4 != 0 {self.fRec17[0]} else {1.7976931348623157e+308}, if iSlow45 != 0 {self.fVec42[iSlow4 as usize]} else {1.7976931348623157e+308}), if iSlow46 != 0 {self.fVec43[iSlow47 as usize]} else {1.7976931348623157e+308}), if iSlow48 != 0 {self.fVec44[iSlow49 as usize]} else {1.7976931348623157e+308}), if iSlow50 != 0 {self.fVec45[((i32::wrapping_sub(self.IOTA0, iSlow51)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow52 != 0 {self.fVec46[((i32::wrapping_sub(self.IOTA0, iSlow53)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow54 != 0 {self.fVec47[((i32::wrapping_sub(self.IOTA0, iSlow55)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow56 != 0 {self.fVec48[((i32::wrapping_sub(self.IOTA0, iSlow57)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow58 != 0 {self.fVec49[((i32::wrapping_sub(self.IOTA0, iSlow59)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow60 != 0 {self.fVec50[((i32::wrapping_sub(self.IOTA0, iSlow61)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow62 != 0 {self.fVec51[((i32::wrapping_sub(self.IOTA0, iSlow63)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow64 != 0 {self.fVec52[((i32::wrapping_sub(self.IOTA0, iSlow65)) & 4095) as usize]} else {1.7976931348623157e+308}) - fTemp633;
			self.fVec53[0] = fTemp658;
			let mut iTemp659: i32 = (fTemp658 > 0.0) as i32;
			let mut fTemp660: F64 = if iTemp659 != 0 {fSlow67} else {fSlow66};
			self.fVec54[0] = fTemp660;
			let mut fTemp661: F64 = 2.0 * fTemp660;
			let mut iTemp662: i32 = (fTemp661) as i32;
			let mut iTemp663: i32 = std::cmp::max(0, std::cmp::min(iTemp662, 2));
			let mut iTemp664: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, 98301), 196607));
			let mut fTemp665: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp664, 3)) as usize] };
			let mut fTemp666: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp664 as usize] };
			let mut fTemp667: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp664, 1)) as usize] } - fTemp666;
			let mut fTemp668: F64 = fTemp661 - (iTemp662) as F64;
			let mut fTemp669: F64 = fTemp666 + fTemp668 * fTemp667 + 0.5 * (fTemp665 - (fTemp666 + fTemp668 * (fTemp667 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp664, 4)) as usize] } - fTemp665))));
			let mut fTemp670: F64 = if iTemp659 != 0 {fTemp669} else {1.0 - fTemp669};
			let mut iTemp671: i32 = (fTemp658 < 0.0) as i32;
			let mut fTemp672: F64 = fSlow1 * (iTemp671) as F64 + fSlow13 * (iTemp659) as F64;
			self.fVec55[0] = fTemp672;
			let mut fTemp673: F64 = self.fConst10 / fTemp672;
			let mut fTemp674: F64 = fTemp673 + 0.5;
			let mut fTemp675: F64 = 65535.0 * (1.0 - fTemp674);
			let mut iTemp676: i32 = (fTemp675) as i32;
			let mut iTemp677: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp676, 65535)))), 196607));
			let mut fTemp678: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp677, 3)) as usize] };
			let mut fTemp679: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp677 as usize] };
			let mut fTemp680: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp677, 1)) as usize] } - fTemp679;
			let mut fTemp681: F64 = 65535.0 * fTemp674;
			let mut iTemp682: i32 = (fTemp681) as i32;
			let mut iTemp683: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp682, 65535)))), 196607));
			let mut fTemp684: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp683, 3), 196607))) as usize] };
			let mut fTemp685: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp683 as usize] };
			let mut fTemp686: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp683, 1), 196607))) as usize] } - fTemp685;
			let mut fTemp687: F64 = 2.0 * self.fVec54[1];
			let mut iTemp688: i32 = (fTemp687) as i32;
			let mut iTemp689: i32 = std::cmp::max(0, std::cmp::min(iTemp688, 2));
			let mut fTemp690: F64 = 65535.0 * (1.0 - self.fRec15[1]);
			let mut iTemp691: i32 = (fTemp690) as i32;
			let mut iTemp692: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp691, 65535))), iTemp689), 196607));
			let mut fTemp693: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp692, 3), 196607))) as usize] };
			let mut fTemp694: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp692 as usize] };
			let mut fTemp695: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp692, 1), 196607))) as usize] } - fTemp694;
			let mut fTemp696: F64 = fTemp687 - (iTemp688) as F64;
			let mut fTemp697: F64 = 65535.0 * self.fRec15[1];
			let mut iTemp698: i32 = (fTemp697) as i32;
			let mut iTemp699: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp689, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp698, 65535)))), 196607));
			let mut fTemp700: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, 3), 196607))) as usize] };
			let mut fTemp701: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp699 as usize] };
			let mut fTemp702: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, 1), 196607))) as usize] } - fTemp701;
			let mut fTemp703: F64 = self.fRec15[1] + fTemp673;
			let mut fTemp704: F64 = 65535.0 * (1.0 - fTemp703);
			let mut iTemp705: i32 = (fTemp704) as i32;
			let mut iTemp706: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp705, 65535)))), 196607));
			let mut fTemp707: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp706, 3)) as usize] };
			let mut fTemp708: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp706 as usize] };
			let mut fTemp709: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp706, 1)) as usize] } - fTemp708;
			let mut fTemp710: F64 = 65535.0 * fTemp703;
			let mut iTemp711: i32 = (fTemp710) as i32;
			let mut iTemp712: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp711, 65535)))), 196607));
			let mut fTemp713: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp712, 3), 196607))) as usize] };
			let mut fTemp714: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp712 as usize] };
			let mut fTemp715: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp712, 1), 196607))) as usize] } - fTemp714;
			let mut fTemp716: F64 = self.fRec15[1] + self.fConst10 * (1.0 / fTemp672 + 1.0 / self.fVec55[1]);
			let mut fTemp717: F64 = 65535.0 * (1.0 - fTemp716);
			let mut iTemp718: i32 = (fTemp717) as i32;
			let mut iTemp719: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp718, 65535))), iTemp663), 196607));
			let mut fTemp720: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp719, 3)) as usize] };
			let mut fTemp721: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp719 as usize] };
			let mut fTemp722: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp719, 1)) as usize] } - fTemp721;
			let mut fTemp723: F64 = 65535.0 * fTemp716;
			let mut iTemp724: i32 = (fTemp723) as i32;
			let mut iTemp725: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp724, 65535)))), 196607));
			let mut fTemp726: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp725, 3), 196607))) as usize] };
			let mut fTemp727: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp725 as usize] };
			let mut fTemp728: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp725, 1), 196607))) as usize] } - fTemp727;
			let mut fTemp729: F64 = (if iTemp659 != 0 {fTemp727 + fTemp668 * fTemp728 + (fTemp723 - (iTemp724) as F64) * (fTemp726 - (fTemp727 + fTemp668 * (fTemp728 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp725, 4), 196607))) as usize] } - fTemp726))))} else {1.0 - (fTemp721 + fTemp668 * fTemp722 + (fTemp717 - (iTemp718) as F64) * (fTemp720 - (fTemp721 + fTemp668 * (fTemp722 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp719, 4)) as usize] } - fTemp720)))))} - if iTemp659 != 0 {fTemp714 + fTemp668 * fTemp715 + (fTemp710 - (iTemp711) as F64) * (fTemp713 - (fTemp714 + fTemp668 * (fTemp715 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp712, 4), 196607))) as usize] } - fTemp713))))} else {1.0 - (fTemp708 + fTemp668 * fTemp709 + (fTemp704 - (iTemp705) as F64) * (fTemp707 - (fTemp708 + fTemp668 * (fTemp709 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp706, 4), 196607))) as usize] } - fTemp707)))))}) * self.fVec53[1] / (fTemp658 * (1.0 - if iTemp659 != 0 {fTemp701 + fTemp696 * fTemp702 + (fTemp697 - (iTemp698) as F64) * (fTemp700 - (fTemp701 + fTemp696 * (fTemp702 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, 4), 196607))) as usize] } - fTemp700))))} else {1.0 - (fTemp694 + fTemp696 * fTemp695 + (fTemp690 - (iTemp691) as F64) * (fTemp693 - (fTemp694 + fTemp696 * (fTemp695 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp692, 4), 196607))) as usize] } - fTemp693)))))}));
			let mut iTemp730: i32 = (fTemp729 > ((if iTemp659 != 0 {fTemp685 + fTemp668 * fTemp686 + (fTemp681 - (iTemp682) as F64) * (fTemp684 - (fTemp685 + fTemp668 * (fTemp686 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp683, 4), 196607))) as usize] } - fTemp684))))} else {1.0 - (fTemp679 + fTemp668 * fTemp680 + (fTemp675 - (iTemp676) as F64) * (fTemp678 - (fTemp679 + fTemp668 * (fTemp680 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp677, 4)) as usize] } - fTemp678)))))} - fTemp670) / (1.0 - fTemp670))) as i32;
			let mut fTemp731: F64 = if iTemp730 != 0 {1.0} else {0.5};
			let mut fTemp732: F64 = if iTemp730 != 0 {0.5} else {0.0};
			let mut fTemp733: F64 = fTemp732 + fTemp731;
			let mut fTemp734: F64 = 0.5 * fTemp733;
			let mut fTemp735: F64 = 65535.0 * (1.0 - fTemp734);
			let mut iTemp736: i32 = (fTemp735) as i32;
			let mut iTemp737: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp736, 65535)))), 196607));
			let mut fTemp738: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp737, 3)) as usize] };
			let mut fTemp739: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp737 as usize] };
			let mut fTemp740: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp737, 1)) as usize] } - fTemp739;
			let mut fTemp741: F64 = 32767.5 * fTemp733;
			let mut iTemp742: i32 = (fTemp741) as i32;
			let mut iTemp743: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp742, 65535)))), 196607));
			let mut fTemp744: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp743, 3)) as usize] };
			let mut fTemp745: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp743 as usize] };
			let mut fTemp746: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp743, 1)) as usize] } - fTemp745;
			let mut fTemp747: F64 = if iTemp659 != 0 {fTemp745 + fTemp668 * fTemp746 + (fTemp741 - (iTemp742) as F64) * (fTemp744 - (fTemp745 + fTemp668 * (fTemp746 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp743, 4)) as usize] } - fTemp744))))} else {1.0 - (fTemp739 + fTemp668 * fTemp740 + (fTemp735 - (iTemp736) as F64) * (fTemp738 - (fTemp739 + fTemp668 * (fTemp740 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp737, 4)) as usize] } - fTemp738)))))};
			let mut fTemp748: F64 = fTemp673 + fTemp734;
			let mut fTemp749: F64 = 65535.0 * (1.0 - fTemp748);
			let mut iTemp750: i32 = (fTemp749) as i32;
			let mut iTemp751: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp750, 65535)))), 196607));
			let mut fTemp752: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp751, 3)) as usize] };
			let mut fTemp753: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp751 as usize] };
			let mut fTemp754: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp751, 1)) as usize] } - fTemp753;
			let mut fTemp755: F64 = 65535.0 * fTemp748;
			let mut iTemp756: i32 = (fTemp755) as i32;
			let mut iTemp757: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp756, 65535)))), 196607));
			let mut fTemp758: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp757, 3), 196607))) as usize] };
			let mut fTemp759: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp757 as usize] };
			let mut fTemp760: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp757, 1), 196607))) as usize] } - fTemp759;
			let mut iTemp761: i32 = (fTemp729 > ((if iTemp659 != 0 {fTemp759 + fTemp668 * fTemp760 + (fTemp755 - (iTemp756) as F64) * (fTemp758 - (fTemp759 + fTemp668 * (fTemp760 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp757, 4), 196607))) as usize] } - fTemp758))))} else {1.0 - (fTemp753 + fTemp668 * fTemp754 + (fTemp749 - (iTemp750) as F64) * (fTemp752 - (fTemp753 + fTemp668 * (fTemp754 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp751, 4)) as usize] } - fTemp752)))))} - fTemp747) / (1.0 - fTemp747))) as i32;
			let mut fTemp762: F64 = if iTemp761 != 0 {fTemp731} else {fTemp734};
			let mut fTemp763: F64 = if iTemp761 != 0 {fTemp734} else {fTemp732};
			let mut fTemp764: F64 = fTemp763 + fTemp762;
			let mut fTemp765: F64 = 0.5 * fTemp764;
			let mut fTemp766: F64 = 65535.0 * (1.0 - fTemp765);
			let mut iTemp767: i32 = (fTemp766) as i32;
			let mut iTemp768: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp767, 65535)))), 196607));
			let mut fTemp769: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp768, 3)) as usize] };
			let mut fTemp770: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp768 as usize] };
			let mut fTemp771: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp768, 1)) as usize] } - fTemp770;
			let mut fTemp772: F64 = 32767.5 * fTemp764;
			let mut iTemp773: i32 = (fTemp772) as i32;
			let mut iTemp774: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp773, 65535)))), 196607));
			let mut fTemp775: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp774, 3)) as usize] };
			let mut fTemp776: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp774 as usize] };
			let mut fTemp777: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp774, 1)) as usize] } - fTemp776;
			let mut fTemp778: F64 = if iTemp659 != 0 {fTemp776 + fTemp668 * fTemp777 + (fTemp772 - (iTemp773) as F64) * (fTemp775 - (fTemp776 + fTemp668 * (fTemp777 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp774, 4)) as usize] } - fTemp775))))} else {1.0 - (fTemp770 + fTemp668 * fTemp771 + (fTemp766 - (iTemp767) as F64) * (fTemp769 - (fTemp770 + fTemp668 * (fTemp771 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp768, 4)) as usize] } - fTemp769)))))};
			let mut fTemp779: F64 = fTemp673 + fTemp765;
			let mut fTemp780: F64 = 65535.0 * (1.0 - fTemp779);
			let mut iTemp781: i32 = (fTemp780) as i32;
			let mut iTemp782: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp781, 65535)))), 196607));
			let mut fTemp783: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp782, 3)) as usize] };
			let mut fTemp784: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp782 as usize] };
			let mut fTemp785: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp782, 1)) as usize] } - fTemp784;
			let mut fTemp786: F64 = 65535.0 * fTemp779;
			let mut iTemp787: i32 = (fTemp786) as i32;
			let mut iTemp788: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp787, 65535)))), 196607));
			let mut fTemp789: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp788, 3), 196607))) as usize] };
			let mut fTemp790: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp788 as usize] };
			let mut fTemp791: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp788, 1), 196607))) as usize] } - fTemp790;
			let mut iTemp792: i32 = (fTemp729 > ((if iTemp659 != 0 {fTemp790 + fTemp668 * fTemp791 + (fTemp786 - (iTemp787) as F64) * (fTemp789 - (fTemp790 + fTemp668 * (fTemp791 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp788, 4), 196607))) as usize] } - fTemp789))))} else {1.0 - (fTemp784 + fTemp668 * fTemp785 + (fTemp780 - (iTemp781) as F64) * (fTemp783 - (fTemp784 + fTemp668 * (fTemp785 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp782, 4)) as usize] } - fTemp783)))))} - fTemp778) / (1.0 - fTemp778))) as i32;
			let mut fTemp793: F64 = if iTemp792 != 0 {fTemp762} else {fTemp765};
			let mut fTemp794: F64 = if iTemp792 != 0 {fTemp765} else {fTemp763};
			let mut fTemp795: F64 = fTemp794 + fTemp793;
			let mut fTemp796: F64 = 0.5 * fTemp795;
			let mut fTemp797: F64 = 65535.0 * (1.0 - fTemp796);
			let mut iTemp798: i32 = (fTemp797) as i32;
			let mut iTemp799: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp798, 65535)))), 196607));
			let mut fTemp800: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp799, 3)) as usize] };
			let mut fTemp801: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp799 as usize] };
			let mut fTemp802: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp799, 1)) as usize] } - fTemp801;
			let mut fTemp803: F64 = 32767.5 * fTemp795;
			let mut iTemp804: i32 = (fTemp803) as i32;
			let mut iTemp805: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp804, 65535)))), 196607));
			let mut fTemp806: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp805, 3)) as usize] };
			let mut fTemp807: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp805 as usize] };
			let mut fTemp808: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp805, 1)) as usize] } - fTemp807;
			let mut fTemp809: F64 = if iTemp659 != 0 {fTemp807 + fTemp668 * fTemp808 + (fTemp803 - (iTemp804) as F64) * (fTemp806 - (fTemp807 + fTemp668 * (fTemp808 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp805, 4)) as usize] } - fTemp806))))} else {1.0 - (fTemp801 + fTemp668 * fTemp802 + (fTemp797 - (iTemp798) as F64) * (fTemp800 - (fTemp801 + fTemp668 * (fTemp802 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp799, 4)) as usize] } - fTemp800)))))};
			let mut fTemp810: F64 = fTemp673 + fTemp796;
			let mut fTemp811: F64 = 65535.0 * (1.0 - fTemp810);
			let mut iTemp812: i32 = (fTemp811) as i32;
			let mut iTemp813: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp812, 65535)))), 196607));
			let mut fTemp814: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp813, 3)) as usize] };
			let mut fTemp815: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp813 as usize] };
			let mut fTemp816: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp813, 1)) as usize] } - fTemp815;
			let mut fTemp817: F64 = 65535.0 * fTemp810;
			let mut iTemp818: i32 = (fTemp817) as i32;
			let mut iTemp819: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp818, 65535)))), 196607));
			let mut fTemp820: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp819, 3), 196607))) as usize] };
			let mut fTemp821: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp819 as usize] };
			let mut fTemp822: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp819, 1), 196607))) as usize] } - fTemp821;
			let mut iTemp823: i32 = (fTemp729 > ((if iTemp659 != 0 {fTemp821 + fTemp668 * fTemp822 + (fTemp817 - (iTemp818) as F64) * (fTemp820 - (fTemp821 + fTemp668 * (fTemp822 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp819, 4), 196607))) as usize] } - fTemp820))))} else {1.0 - (fTemp815 + fTemp668 * fTemp816 + (fTemp811 - (iTemp812) as F64) * (fTemp814 - (fTemp815 + fTemp668 * (fTemp816 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp813, 4)) as usize] } - fTemp814)))))} - fTemp809) / (1.0 - fTemp809))) as i32;
			let mut fTemp824: F64 = if iTemp823 != 0 {fTemp793} else {fTemp796};
			let mut fTemp825: F64 = if iTemp823 != 0 {fTemp796} else {fTemp794};
			let mut fTemp826: F64 = fTemp825 + fTemp824;
			let mut fTemp827: F64 = 0.5 * fTemp826;
			let mut fTemp828: F64 = 65535.0 * (1.0 - fTemp827);
			let mut iTemp829: i32 = (fTemp828) as i32;
			let mut iTemp830: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp829, 65535)))), 196607));
			let mut fTemp831: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp830, 3)) as usize] };
			let mut fTemp832: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp830 as usize] };
			let mut fTemp833: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp830, 1)) as usize] } - fTemp832;
			let mut fTemp834: F64 = 32767.5 * fTemp826;
			let mut iTemp835: i32 = (fTemp834) as i32;
			let mut iTemp836: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp835, 65535)))), 196607));
			let mut fTemp837: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp836, 3)) as usize] };
			let mut fTemp838: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp836 as usize] };
			let mut fTemp839: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp836, 1)) as usize] } - fTemp838;
			let mut fTemp840: F64 = if iTemp659 != 0 {fTemp838 + fTemp668 * fTemp839 + (fTemp834 - (iTemp835) as F64) * (fTemp837 - (fTemp838 + fTemp668 * (fTemp839 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp836, 4)) as usize] } - fTemp837))))} else {1.0 - (fTemp832 + fTemp668 * fTemp833 + (fTemp828 - (iTemp829) as F64) * (fTemp831 - (fTemp832 + fTemp668 * (fTemp833 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp830, 4)) as usize] } - fTemp831)))))};
			let mut fTemp841: F64 = fTemp673 + fTemp827;
			let mut fTemp842: F64 = 65535.0 * (1.0 - fTemp841);
			let mut iTemp843: i32 = (fTemp842) as i32;
			let mut iTemp844: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp843, 65535)))), 196607));
			let mut fTemp845: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp844, 3)) as usize] };
			let mut fTemp846: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp844 as usize] };
			let mut fTemp847: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp844, 1)) as usize] } - fTemp846;
			let mut fTemp848: F64 = 65535.0 * fTemp841;
			let mut iTemp849: i32 = (fTemp848) as i32;
			let mut iTemp850: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp849, 65535)))), 196607));
			let mut fTemp851: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp850, 3), 196607))) as usize] };
			let mut fTemp852: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp850 as usize] };
			let mut fTemp853: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp850, 1), 196607))) as usize] } - fTemp852;
			let mut iTemp854: i32 = (fTemp729 > ((if iTemp659 != 0 {fTemp852 + fTemp668 * fTemp853 + (fTemp848 - (iTemp849) as F64) * (fTemp851 - (fTemp852 + fTemp668 * (fTemp853 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp850, 4), 196607))) as usize] } - fTemp851))))} else {1.0 - (fTemp846 + fTemp668 * fTemp847 + (fTemp842 - (iTemp843) as F64) * (fTemp845 - (fTemp846 + fTemp668 * (fTemp847 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp844, 4)) as usize] } - fTemp845)))))} - fTemp840) / (1.0 - fTemp840))) as i32;
			let mut fTemp855: F64 = if iTemp854 != 0 {fTemp824} else {fTemp827};
			let mut fTemp856: F64 = if iTemp854 != 0 {fTemp827} else {fTemp825};
			let mut fTemp857: F64 = fTemp856 + fTemp855;
			let mut fTemp858: F64 = 0.5 * fTemp857;
			let mut fTemp859: F64 = 65535.0 * (1.0 - fTemp858);
			let mut iTemp860: i32 = (fTemp859) as i32;
			let mut iTemp861: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp860, 65535)))), 196607));
			let mut fTemp862: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp861, 3)) as usize] };
			let mut fTemp863: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp861 as usize] };
			let mut fTemp864: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp861, 1)) as usize] } - fTemp863;
			let mut fTemp865: F64 = 32767.5 * fTemp857;
			let mut iTemp866: i32 = (fTemp865) as i32;
			let mut iTemp867: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp866, 65535)))), 196607));
			let mut fTemp868: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp867, 3)) as usize] };
			let mut fTemp869: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp867 as usize] };
			let mut fTemp870: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp867, 1)) as usize] } - fTemp869;
			let mut fTemp871: F64 = if iTemp659 != 0 {fTemp869 + fTemp668 * fTemp870 + (fTemp865 - (iTemp866) as F64) * (fTemp868 - (fTemp869 + fTemp668 * (fTemp870 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp867, 4)) as usize] } - fTemp868))))} else {1.0 - (fTemp863 + fTemp668 * fTemp864 + (fTemp859 - (iTemp860) as F64) * (fTemp862 - (fTemp863 + fTemp668 * (fTemp864 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp861, 4)) as usize] } - fTemp862)))))};
			let mut fTemp872: F64 = fTemp673 + fTemp858;
			let mut fTemp873: F64 = 65535.0 * (1.0 - fTemp872);
			let mut iTemp874: i32 = (fTemp873) as i32;
			let mut iTemp875: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp874, 65535)))), 196607));
			let mut fTemp876: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp875, 3)) as usize] };
			let mut fTemp877: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp875 as usize] };
			let mut fTemp878: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp875, 1)) as usize] } - fTemp877;
			let mut fTemp879: F64 = 65535.0 * fTemp872;
			let mut iTemp880: i32 = (fTemp879) as i32;
			let mut iTemp881: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp880, 65535)))), 196607));
			let mut fTemp882: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp881, 3), 196607))) as usize] };
			let mut fTemp883: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp881 as usize] };
			let mut fTemp884: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp881, 1), 196607))) as usize] } - fTemp883;
			let mut iTemp885: i32 = (fTemp729 > ((if iTemp659 != 0 {fTemp883 + fTemp668 * fTemp884 + (fTemp879 - (iTemp880) as F64) * (fTemp882 - (fTemp883 + fTemp668 * (fTemp884 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp881, 4), 196607))) as usize] } - fTemp882))))} else {1.0 - (fTemp877 + fTemp668 * fTemp878 + (fTemp873 - (iTemp874) as F64) * (fTemp876 - (fTemp877 + fTemp668 * (fTemp878 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp875, 4)) as usize] } - fTemp876)))))} - fTemp871) / (1.0 - fTemp871))) as i32;
			let mut fTemp886: F64 = if iTemp885 != 0 {fTemp855} else {fTemp858};
			let mut fTemp887: F64 = if iTemp885 != 0 {fTemp858} else {fTemp856};
			let mut fTemp888: F64 = fTemp887 + fTemp886;
			let mut fTemp889: F64 = 0.5 * fTemp888;
			let mut fTemp890: F64 = 65535.0 * (1.0 - fTemp889);
			let mut iTemp891: i32 = (fTemp890) as i32;
			let mut iTemp892: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp891, 65535)))), 196607));
			let mut fTemp893: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp892, 3)) as usize] };
			let mut fTemp894: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp892 as usize] };
			let mut fTemp895: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp892, 1)) as usize] } - fTemp894;
			let mut fTemp896: F64 = 32767.5 * fTemp888;
			let mut iTemp897: i32 = (fTemp896) as i32;
			let mut iTemp898: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp897, 65535)))), 196607));
			let mut fTemp899: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp898, 3)) as usize] };
			let mut fTemp900: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp898 as usize] };
			let mut fTemp901: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp898, 1)) as usize] } - fTemp900;
			let mut fTemp902: F64 = if iTemp659 != 0 {fTemp900 + fTemp668 * fTemp901 + (fTemp896 - (iTemp897) as F64) * (fTemp899 - (fTemp900 + fTemp668 * (fTemp901 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp898, 4)) as usize] } - fTemp899))))} else {1.0 - (fTemp894 + fTemp668 * fTemp895 + (fTemp890 - (iTemp891) as F64) * (fTemp893 - (fTemp894 + fTemp668 * (fTemp895 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp892, 4)) as usize] } - fTemp893)))))};
			let mut fTemp903: F64 = fTemp673 + fTemp889;
			let mut fTemp904: F64 = 65535.0 * (1.0 - fTemp903);
			let mut iTemp905: i32 = (fTemp904) as i32;
			let mut iTemp906: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp905, 65535)))), 196607));
			let mut fTemp907: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp906, 3)) as usize] };
			let mut fTemp908: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp906 as usize] };
			let mut fTemp909: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp906, 1)) as usize] } - fTemp908;
			let mut fTemp910: F64 = 65535.0 * fTemp903;
			let mut iTemp911: i32 = (fTemp910) as i32;
			let mut iTemp912: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp911, 65535)))), 196607));
			let mut fTemp913: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp912, 3), 196607))) as usize] };
			let mut fTemp914: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp912 as usize] };
			let mut fTemp915: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp912, 1), 196607))) as usize] } - fTemp914;
			let mut iTemp916: i32 = (fTemp729 > ((if iTemp659 != 0 {fTemp914 + fTemp668 * fTemp915 + (fTemp910 - (iTemp911) as F64) * (fTemp913 - (fTemp914 + fTemp668 * (fTemp915 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp912, 4), 196607))) as usize] } - fTemp913))))} else {1.0 - (fTemp908 + fTemp668 * fTemp909 + (fTemp904 - (iTemp905) as F64) * (fTemp907 - (fTemp908 + fTemp668 * (fTemp909 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp906, 4)) as usize] } - fTemp907)))))} - fTemp902) / (1.0 - fTemp902))) as i32;
			let mut fTemp917: F64 = if iTemp916 != 0 {fTemp886} else {fTemp889};
			let mut fTemp918: F64 = if iTemp916 != 0 {fTemp889} else {fTemp887};
			let mut fTemp919: F64 = fTemp918 + fTemp917;
			let mut fTemp920: F64 = 0.5 * fTemp919;
			let mut fTemp921: F64 = 65535.0 * (1.0 - fTemp920);
			let mut iTemp922: i32 = (fTemp921) as i32;
			let mut iTemp923: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp922, 65535)))), 196607));
			let mut fTemp924: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp923, 3)) as usize] };
			let mut fTemp925: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp923 as usize] };
			let mut fTemp926: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp923, 1)) as usize] } - fTemp925;
			let mut fTemp927: F64 = 32767.5 * fTemp919;
			let mut iTemp928: i32 = (fTemp927) as i32;
			let mut iTemp929: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp928, 65535)))), 196607));
			let mut fTemp930: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp929, 3)) as usize] };
			let mut fTemp931: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp929 as usize] };
			let mut fTemp932: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp929, 1)) as usize] } - fTemp931;
			let mut fTemp933: F64 = if iTemp659 != 0 {fTemp931 + fTemp668 * fTemp932 + (fTemp927 - (iTemp928) as F64) * (fTemp930 - (fTemp931 + fTemp668 * (fTemp932 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp929, 4)) as usize] } - fTemp930))))} else {1.0 - (fTemp925 + fTemp668 * fTemp926 + (fTemp921 - (iTemp922) as F64) * (fTemp924 - (fTemp925 + fTemp668 * (fTemp926 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp923, 4)) as usize] } - fTemp924)))))};
			let mut fTemp934: F64 = fTemp673 + fTemp920;
			let mut fTemp935: F64 = 65535.0 * (1.0 - fTemp934);
			let mut iTemp936: i32 = (fTemp935) as i32;
			let mut iTemp937: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp936, 65535)))), 196607));
			let mut fTemp938: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp937, 3)) as usize] };
			let mut fTemp939: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp937 as usize] };
			let mut fTemp940: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp937, 1)) as usize] } - fTemp939;
			let mut fTemp941: F64 = 65535.0 * fTemp934;
			let mut iTemp942: i32 = (fTemp941) as i32;
			let mut iTemp943: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp942, 65535)))), 196607));
			let mut fTemp944: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp943, 3), 196607))) as usize] };
			let mut fTemp945: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp943 as usize] };
			let mut fTemp946: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp943, 1), 196607))) as usize] } - fTemp945;
			let mut iTemp947: i32 = (fTemp729 > ((if iTemp659 != 0 {fTemp945 + fTemp668 * fTemp946 + (fTemp941 - (iTemp942) as F64) * (fTemp944 - (fTemp945 + fTemp668 * (fTemp946 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp943, 4), 196607))) as usize] } - fTemp944))))} else {1.0 - (fTemp939 + fTemp668 * fTemp940 + (fTemp935 - (iTemp936) as F64) * (fTemp938 - (fTemp939 + fTemp668 * (fTemp940 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp937, 4)) as usize] } - fTemp938)))))} - fTemp933) / (1.0 - fTemp933))) as i32;
			let mut fTemp948: F64 = if iTemp947 != 0 {fTemp917} else {fTemp920};
			let mut fTemp949: F64 = if iTemp947 != 0 {fTemp920} else {fTemp918};
			let mut fTemp950: F64 = fTemp949 + fTemp948;
			let mut fTemp951: F64 = 0.5 * fTemp950;
			let mut fTemp952: F64 = 65535.0 * (1.0 - fTemp951);
			let mut iTemp953: i32 = (fTemp952) as i32;
			let mut iTemp954: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp953, 65535)))), 196607));
			let mut fTemp955: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp954, 3)) as usize] };
			let mut fTemp956: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp954 as usize] };
			let mut fTemp957: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp954, 1)) as usize] } - fTemp956;
			let mut fTemp958: F64 = 32767.5 * fTemp950;
			let mut iTemp959: i32 = (fTemp958) as i32;
			let mut iTemp960: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp959, 65535)))), 196607));
			let mut fTemp961: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp960, 3)) as usize] };
			let mut fTemp962: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp960 as usize] };
			let mut fTemp963: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp960, 1)) as usize] } - fTemp962;
			let mut fTemp964: F64 = if iTemp659 != 0 {fTemp962 + fTemp668 * fTemp963 + (fTemp958 - (iTemp959) as F64) * (fTemp961 - (fTemp962 + fTemp668 * (fTemp963 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp960, 4)) as usize] } - fTemp961))))} else {1.0 - (fTemp956 + fTemp668 * fTemp957 + (fTemp952 - (iTemp953) as F64) * (fTemp955 - (fTemp956 + fTemp668 * (fTemp957 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp954, 4)) as usize] } - fTemp955)))))};
			let mut fTemp965: F64 = fTemp673 + fTemp951;
			let mut fTemp966: F64 = 65535.0 * (1.0 - fTemp965);
			let mut iTemp967: i32 = (fTemp966) as i32;
			let mut iTemp968: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp967, 65535)))), 196607));
			let mut fTemp969: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp968, 3)) as usize] };
			let mut fTemp970: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp968 as usize] };
			let mut fTemp971: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp968, 1)) as usize] } - fTemp970;
			let mut fTemp972: F64 = 65535.0 * fTemp965;
			let mut iTemp973: i32 = (fTemp972) as i32;
			let mut iTemp974: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp973, 65535)))), 196607));
			let mut fTemp975: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp974, 3), 196607))) as usize] };
			let mut fTemp976: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp974 as usize] };
			let mut fTemp977: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp974, 1), 196607))) as usize] } - fTemp976;
			let mut iTemp978: i32 = (fTemp729 > ((if iTemp659 != 0 {fTemp976 + fTemp668 * fTemp977 + (fTemp972 - (iTemp973) as F64) * (fTemp975 - (fTemp976 + fTemp668 * (fTemp977 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp974, 4), 196607))) as usize] } - fTemp975))))} else {1.0 - (fTemp970 + fTemp668 * fTemp971 + (fTemp966 - (iTemp967) as F64) * (fTemp969 - (fTemp970 + fTemp668 * (fTemp971 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp968, 4)) as usize] } - fTemp969)))))} - fTemp964) / (1.0 - fTemp964))) as i32;
			let mut fTemp979: F64 = if iTemp978 != 0 {fTemp948} else {fTemp951};
			let mut fTemp980: F64 = if iTemp978 != 0 {fTemp951} else {fTemp949};
			let mut fTemp981: F64 = fTemp980 + fTemp979;
			let mut fTemp982: F64 = 0.5 * fTemp981;
			let mut fTemp983: F64 = 65535.0 * (1.0 - fTemp982);
			let mut iTemp984: i32 = (fTemp983) as i32;
			let mut iTemp985: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp984, 65535)))), 196607));
			let mut fTemp986: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp985, 3)) as usize] };
			let mut fTemp987: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp985 as usize] };
			let mut fTemp988: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp985, 1)) as usize] } - fTemp987;
			let mut fTemp989: F64 = 32767.5 * fTemp981;
			let mut iTemp990: i32 = (fTemp989) as i32;
			let mut iTemp991: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp990, 65535)))), 196607));
			let mut fTemp992: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp991, 3)) as usize] };
			let mut fTemp993: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp991 as usize] };
			let mut fTemp994: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp991, 1)) as usize] } - fTemp993;
			let mut fTemp995: F64 = if iTemp659 != 0 {fTemp993 + fTemp668 * fTemp994 + (fTemp989 - (iTemp990) as F64) * (fTemp992 - (fTemp993 + fTemp668 * (fTemp994 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp991, 4)) as usize] } - fTemp992))))} else {1.0 - (fTemp987 + fTemp668 * fTemp988 + (fTemp983 - (iTemp984) as F64) * (fTemp986 - (fTemp987 + fTemp668 * (fTemp988 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp985, 4)) as usize] } - fTemp986)))))};
			let mut fTemp996: F64 = fTemp673 + fTemp982;
			let mut fTemp997: F64 = 65535.0 * (1.0 - fTemp996);
			let mut iTemp998: i32 = (fTemp997) as i32;
			let mut iTemp999: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp998, 65535)))), 196607));
			let mut fTemp1000: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp999, 3)) as usize] };
			let mut fTemp1001: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp999 as usize] };
			let mut fTemp1002: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp999, 1)) as usize] } - fTemp1001;
			let mut fTemp1003: F64 = 65535.0 * fTemp996;
			let mut iTemp1004: i32 = (fTemp1003) as i32;
			let mut iTemp1005: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1004, 65535)))), 196607));
			let mut fTemp1006: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1005, 3), 196607))) as usize] };
			let mut fTemp1007: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1005 as usize] };
			let mut fTemp1008: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1005, 1), 196607))) as usize] } - fTemp1007;
			let mut iTemp1009: i32 = (fTemp729 > ((if iTemp659 != 0 {fTemp1007 + fTemp668 * fTemp1008 + (fTemp1003 - (iTemp1004) as F64) * (fTemp1006 - (fTemp1007 + fTemp668 * (fTemp1008 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1005, 4), 196607))) as usize] } - fTemp1006))))} else {1.0 - (fTemp1001 + fTemp668 * fTemp1002 + (fTemp997 - (iTemp998) as F64) * (fTemp1000 - (fTemp1001 + fTemp668 * (fTemp1002 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp999, 4)) as usize] } - fTemp1000)))))} - fTemp995) / (1.0 - fTemp995))) as i32;
			let mut fTemp1010: F64 = if iTemp1009 != 0 {fTemp979} else {fTemp982};
			let mut fTemp1011: F64 = if iTemp1009 != 0 {fTemp982} else {fTemp980};
			let mut fTemp1012: F64 = fTemp1011 + fTemp1010;
			let mut fTemp1013: F64 = 0.5 * fTemp1012;
			let mut fTemp1014: F64 = 65535.0 * (1.0 - fTemp1013);
			let mut iTemp1015: i32 = (fTemp1014) as i32;
			let mut iTemp1016: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1015, 65535)))), 196607));
			let mut fTemp1017: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1016, 3)) as usize] };
			let mut fTemp1018: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1016 as usize] };
			let mut fTemp1019: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1016, 1)) as usize] } - fTemp1018;
			let mut fTemp1020: F64 = 32767.5 * fTemp1012;
			let mut iTemp1021: i32 = (fTemp1020) as i32;
			let mut iTemp1022: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1021, 65535)))), 196607));
			let mut fTemp1023: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1022, 3)) as usize] };
			let mut fTemp1024: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1022 as usize] };
			let mut fTemp1025: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1022, 1)) as usize] } - fTemp1024;
			let mut fTemp1026: F64 = if iTemp659 != 0 {fTemp1024 + fTemp668 * fTemp1025 + (fTemp1020 - (iTemp1021) as F64) * (fTemp1023 - (fTemp1024 + fTemp668 * (fTemp1025 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1022, 4)) as usize] } - fTemp1023))))} else {1.0 - (fTemp1018 + fTemp668 * fTemp1019 + (fTemp1014 - (iTemp1015) as F64) * (fTemp1017 - (fTemp1018 + fTemp668 * (fTemp1019 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1016, 4)) as usize] } - fTemp1017)))))};
			let mut fTemp1027: F64 = fTemp673 + fTemp1013;
			let mut fTemp1028: F64 = 65535.0 * (1.0 - fTemp1027);
			let mut iTemp1029: i32 = (fTemp1028) as i32;
			let mut iTemp1030: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1029, 65535)))), 196607));
			let mut fTemp1031: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1030, 3)) as usize] };
			let mut fTemp1032: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1030 as usize] };
			let mut fTemp1033: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1030, 1)) as usize] } - fTemp1032;
			let mut fTemp1034: F64 = 65535.0 * fTemp1027;
			let mut iTemp1035: i32 = (fTemp1034) as i32;
			let mut iTemp1036: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1035, 65535)))), 196607));
			let mut fTemp1037: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1036, 3), 196607))) as usize] };
			let mut fTemp1038: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1036 as usize] };
			let mut fTemp1039: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1036, 1), 196607))) as usize] } - fTemp1038;
			let mut iTemp1040: i32 = (fTemp729 > ((if iTemp659 != 0 {fTemp1038 + fTemp668 * fTemp1039 + (fTemp1034 - (iTemp1035) as F64) * (fTemp1037 - (fTemp1038 + fTemp668 * (fTemp1039 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1036, 4), 196607))) as usize] } - fTemp1037))))} else {1.0 - (fTemp1032 + fTemp668 * fTemp1033 + (fTemp1028 - (iTemp1029) as F64) * (fTemp1031 - (fTemp1032 + fTemp668 * (fTemp1033 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1030, 4)) as usize] } - fTemp1031)))))} - fTemp1026) / (1.0 - fTemp1026))) as i32;
			let mut fTemp1041: F64 = if iTemp1040 != 0 {fTemp1010} else {fTemp1013};
			let mut fTemp1042: F64 = if iTemp1040 != 0 {fTemp1013} else {fTemp1011};
			let mut fTemp1043: F64 = fTemp1042 + fTemp1041;
			let mut fTemp1044: F64 = 0.5 * fTemp1043;
			let mut fTemp1045: F64 = 65535.0 * (1.0 - fTemp1044);
			let mut iTemp1046: i32 = (fTemp1045) as i32;
			let mut iTemp1047: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1046, 65535)))), 196607));
			let mut fTemp1048: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1047, 3)) as usize] };
			let mut fTemp1049: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1047 as usize] };
			let mut fTemp1050: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1047, 1)) as usize] } - fTemp1049;
			let mut fTemp1051: F64 = 32767.5 * fTemp1043;
			let mut iTemp1052: i32 = (fTemp1051) as i32;
			let mut iTemp1053: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1052, 65535)))), 196607));
			let mut fTemp1054: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1053, 3)) as usize] };
			let mut fTemp1055: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1053 as usize] };
			let mut fTemp1056: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1053, 1)) as usize] } - fTemp1055;
			let mut fTemp1057: F64 = if iTemp659 != 0 {fTemp1055 + fTemp668 * fTemp1056 + (fTemp1051 - (iTemp1052) as F64) * (fTemp1054 - (fTemp1055 + fTemp668 * (fTemp1056 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1053, 4)) as usize] } - fTemp1054))))} else {1.0 - (fTemp1049 + fTemp668 * fTemp1050 + (fTemp1045 - (iTemp1046) as F64) * (fTemp1048 - (fTemp1049 + fTemp668 * (fTemp1050 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1047, 4)) as usize] } - fTemp1048)))))};
			let mut fTemp1058: F64 = fTemp673 + fTemp1044;
			let mut fTemp1059: F64 = 65535.0 * (1.0 - fTemp1058);
			let mut iTemp1060: i32 = (fTemp1059) as i32;
			let mut iTemp1061: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1060, 65535)))), 196607));
			let mut fTemp1062: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1061, 3)) as usize] };
			let mut fTemp1063: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1061 as usize] };
			let mut fTemp1064: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1061, 1)) as usize] } - fTemp1063;
			let mut fTemp1065: F64 = 65535.0 * fTemp1058;
			let mut iTemp1066: i32 = (fTemp1065) as i32;
			let mut iTemp1067: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1066, 65535)))), 196607));
			let mut fTemp1068: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1067, 3), 196607))) as usize] };
			let mut fTemp1069: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1067 as usize] };
			let mut fTemp1070: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1067, 1), 196607))) as usize] } - fTemp1069;
			let mut iTemp1071: i32 = (fTemp729 > ((if iTemp659 != 0 {fTemp1069 + fTemp668 * fTemp1070 + (fTemp1065 - (iTemp1066) as F64) * (fTemp1068 - (fTemp1069 + fTemp668 * (fTemp1070 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1067, 4), 196607))) as usize] } - fTemp1068))))} else {1.0 - (fTemp1063 + fTemp668 * fTemp1064 + (fTemp1059 - (iTemp1060) as F64) * (fTemp1062 - (fTemp1063 + fTemp668 * (fTemp1064 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1061, 4)) as usize] } - fTemp1062)))))} - fTemp1057) / (1.0 - fTemp1057))) as i32;
			let mut fTemp1072: F64 = if iTemp1071 != 0 {fTemp1041} else {fTemp1044};
			let mut fTemp1073: F64 = if iTemp1071 != 0 {fTemp1044} else {fTemp1042};
			let mut fTemp1074: F64 = fTemp1073 + fTemp1072;
			let mut fTemp1075: F64 = 0.5 * fTemp1074;
			let mut fTemp1076: F64 = 65535.0 * (1.0 - fTemp1075);
			let mut iTemp1077: i32 = (fTemp1076) as i32;
			let mut iTemp1078: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1077, 65535)))), 196607));
			let mut fTemp1079: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1078, 3)) as usize] };
			let mut fTemp1080: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1078 as usize] };
			let mut fTemp1081: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1078, 1)) as usize] } - fTemp1080;
			let mut fTemp1082: F64 = 32767.5 * fTemp1074;
			let mut iTemp1083: i32 = (fTemp1082) as i32;
			let mut iTemp1084: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1083, 65535)))), 196607));
			let mut fTemp1085: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1084, 3)) as usize] };
			let mut fTemp1086: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1084 as usize] };
			let mut fTemp1087: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1084, 1)) as usize] } - fTemp1086;
			let mut fTemp1088: F64 = if iTemp659 != 0 {fTemp1086 + fTemp668 * fTemp1087 + (fTemp1082 - (iTemp1083) as F64) * (fTemp1085 - (fTemp1086 + fTemp668 * (fTemp1087 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1084, 4)) as usize] } - fTemp1085))))} else {1.0 - (fTemp1080 + fTemp668 * fTemp1081 + (fTemp1076 - (iTemp1077) as F64) * (fTemp1079 - (fTemp1080 + fTemp668 * (fTemp1081 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1078, 4)) as usize] } - fTemp1079)))))};
			let mut fTemp1089: F64 = fTemp673 + fTemp1075;
			let mut fTemp1090: F64 = 65535.0 * (1.0 - fTemp1089);
			let mut iTemp1091: i32 = (fTemp1090) as i32;
			let mut iTemp1092: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1091, 65535)))), 196607));
			let mut fTemp1093: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1092, 3)) as usize] };
			let mut fTemp1094: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1092 as usize] };
			let mut fTemp1095: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1092, 1)) as usize] } - fTemp1094;
			let mut fTemp1096: F64 = 65535.0 * fTemp1089;
			let mut iTemp1097: i32 = (fTemp1096) as i32;
			let mut iTemp1098: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1097, 65535)))), 196607));
			let mut fTemp1099: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1098, 3), 196607))) as usize] };
			let mut fTemp1100: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1098 as usize] };
			let mut fTemp1101: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1098, 1), 196607))) as usize] } - fTemp1100;
			let mut iTemp1102: i32 = (fTemp729 > ((if iTemp659 != 0 {fTemp1100 + fTemp668 * fTemp1101 + (fTemp1096 - (iTemp1097) as F64) * (fTemp1099 - (fTemp1100 + fTemp668 * (fTemp1101 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1098, 4), 196607))) as usize] } - fTemp1099))))} else {1.0 - (fTemp1094 + fTemp668 * fTemp1095 + (fTemp1090 - (iTemp1091) as F64) * (fTemp1093 - (fTemp1094 + fTemp668 * (fTemp1095 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1092, 4)) as usize] } - fTemp1093)))))} - fTemp1088) / (1.0 - fTemp1088))) as i32;
			let mut fTemp1103: F64 = if iTemp1102 != 0 {fTemp1072} else {fTemp1075};
			let mut fTemp1104: F64 = if iTemp1102 != 0 {fTemp1075} else {fTemp1073};
			let mut fTemp1105: F64 = fTemp1104 + fTemp1103;
			let mut fTemp1106: F64 = 0.5 * fTemp1105;
			let mut fTemp1107: F64 = 65535.0 * (1.0 - fTemp1106);
			let mut iTemp1108: i32 = (fTemp1107) as i32;
			let mut iTemp1109: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1108, 65535)))), 196607));
			let mut fTemp1110: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1109, 3)) as usize] };
			let mut fTemp1111: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1109 as usize] };
			let mut fTemp1112: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1109, 1)) as usize] } - fTemp1111;
			let mut fTemp1113: F64 = 32767.5 * fTemp1105;
			let mut iTemp1114: i32 = (fTemp1113) as i32;
			let mut iTemp1115: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1114, 65535)))), 196607));
			let mut fTemp1116: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1115, 3)) as usize] };
			let mut fTemp1117: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1115 as usize] };
			let mut fTemp1118: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1115, 1)) as usize] } - fTemp1117;
			let mut fTemp1119: F64 = if iTemp659 != 0 {fTemp1117 + fTemp668 * fTemp1118 + (fTemp1113 - (iTemp1114) as F64) * (fTemp1116 - (fTemp1117 + fTemp668 * (fTemp1118 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1115, 4)) as usize] } - fTemp1116))))} else {1.0 - (fTemp1111 + fTemp668 * fTemp1112 + (fTemp1107 - (iTemp1108) as F64) * (fTemp1110 - (fTemp1111 + fTemp668 * (fTemp1112 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1109, 4)) as usize] } - fTemp1110)))))};
			let mut fTemp1120: F64 = fTemp673 + fTemp1106;
			let mut fTemp1121: F64 = 65535.0 * (1.0 - fTemp1120);
			let mut iTemp1122: i32 = (fTemp1121) as i32;
			let mut iTemp1123: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1122, 65535)))), 196607));
			let mut fTemp1124: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1123, 3)) as usize] };
			let mut fTemp1125: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1123 as usize] };
			let mut fTemp1126: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1123, 1)) as usize] } - fTemp1125;
			let mut fTemp1127: F64 = 65535.0 * fTemp1120;
			let mut iTemp1128: i32 = (fTemp1127) as i32;
			let mut iTemp1129: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1128, 65535)))), 196607));
			let mut fTemp1130: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1129, 3), 196607))) as usize] };
			let mut fTemp1131: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1129 as usize] };
			let mut fTemp1132: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1129, 1), 196607))) as usize] } - fTemp1131;
			let mut iTemp1133: i32 = (fTemp729 > ((if iTemp659 != 0 {fTemp1131 + fTemp668 * fTemp1132 + (fTemp1127 - (iTemp1128) as F64) * (fTemp1130 - (fTemp1131 + fTemp668 * (fTemp1132 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1129, 4), 196607))) as usize] } - fTemp1130))))} else {1.0 - (fTemp1125 + fTemp668 * fTemp1126 + (fTemp1121 - (iTemp1122) as F64) * (fTemp1124 - (fTemp1125 + fTemp668 * (fTemp1126 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1123, 4)) as usize] } - fTemp1124)))))} - fTemp1119) / (1.0 - fTemp1119))) as i32;
			let mut fTemp1134: F64 = if iTemp1133 != 0 {fTemp1103} else {fTemp1106};
			let mut fTemp1135: F64 = if iTemp1133 != 0 {fTemp1106} else {fTemp1104};
			let mut fTemp1136: F64 = fTemp1135 + fTemp1134;
			let mut fTemp1137: F64 = 0.5 * fTemp1136;
			let mut fTemp1138: F64 = 65535.0 * (1.0 - fTemp1137);
			let mut iTemp1139: i32 = (fTemp1138) as i32;
			let mut iTemp1140: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1139, 65535)))), 196607));
			let mut fTemp1141: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1140, 3)) as usize] };
			let mut fTemp1142: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1140 as usize] };
			let mut fTemp1143: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1140, 1)) as usize] } - fTemp1142;
			let mut fTemp1144: F64 = 32767.5 * fTemp1136;
			let mut iTemp1145: i32 = (fTemp1144) as i32;
			let mut iTemp1146: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1145, 65535)))), 196607));
			let mut fTemp1147: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1146, 3)) as usize] };
			let mut fTemp1148: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1146 as usize] };
			let mut fTemp1149: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1146, 1)) as usize] } - fTemp1148;
			let mut fTemp1150: F64 = if iTemp659 != 0 {fTemp1148 + fTemp668 * fTemp1149 + (fTemp1144 - (iTemp1145) as F64) * (fTemp1147 - (fTemp1148 + fTemp668 * (fTemp1149 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1146, 4)) as usize] } - fTemp1147))))} else {1.0 - (fTemp1142 + fTemp668 * fTemp1143 + (fTemp1138 - (iTemp1139) as F64) * (fTemp1141 - (fTemp1142 + fTemp668 * (fTemp1143 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1140, 4)) as usize] } - fTemp1141)))))};
			let mut fTemp1151: F64 = fTemp673 + fTemp1137;
			let mut fTemp1152: F64 = 65535.0 * (1.0 - fTemp1151);
			let mut iTemp1153: i32 = (fTemp1152) as i32;
			let mut iTemp1154: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1153, 65535)))), 196607));
			let mut fTemp1155: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1154, 3)) as usize] };
			let mut fTemp1156: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1154 as usize] };
			let mut fTemp1157: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1154, 1)) as usize] } - fTemp1156;
			let mut fTemp1158: F64 = 65535.0 * fTemp1151;
			let mut iTemp1159: i32 = (fTemp1158) as i32;
			let mut iTemp1160: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1159, 65535)))), 196607));
			let mut fTemp1161: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1160, 3), 196607))) as usize] };
			let mut fTemp1162: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1160 as usize] };
			let mut fTemp1163: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1160, 1), 196607))) as usize] } - fTemp1162;
			let mut iTemp1164: i32 = (fTemp729 > ((if iTemp659 != 0 {fTemp1162 + fTemp668 * fTemp1163 + (fTemp1158 - (iTemp1159) as F64) * (fTemp1161 - (fTemp1162 + fTemp668 * (fTemp1163 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1160, 4), 196607))) as usize] } - fTemp1161))))} else {1.0 - (fTemp1156 + fTemp668 * fTemp1157 + (fTemp1152 - (iTemp1153) as F64) * (fTemp1155 - (fTemp1156 + fTemp668 * (fTemp1157 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1154, 4)) as usize] } - fTemp1155)))))} - fTemp1150) / (1.0 - fTemp1150))) as i32;
			let mut fTemp1165: F64 = if iTemp1164 != 0 {fTemp1134} else {fTemp1137};
			let mut fTemp1166: F64 = if iTemp1164 != 0 {fTemp1137} else {fTemp1135};
			let mut fTemp1167: F64 = fTemp1166 + fTemp1165;
			let mut fTemp1168: F64 = 0.5 * fTemp1167;
			let mut fTemp1169: F64 = 65535.0 * (1.0 - fTemp1168);
			let mut iTemp1170: i32 = (fTemp1169) as i32;
			let mut iTemp1171: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1170, 65535)))), 196607));
			let mut fTemp1172: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1171, 3)) as usize] };
			let mut fTemp1173: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1171 as usize] };
			let mut fTemp1174: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1171, 1)) as usize] } - fTemp1173;
			let mut fTemp1175: F64 = 32767.5 * fTemp1167;
			let mut iTemp1176: i32 = (fTemp1175) as i32;
			let mut iTemp1177: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1176, 65535)))), 196607));
			let mut fTemp1178: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1177, 3)) as usize] };
			let mut fTemp1179: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1177 as usize] };
			let mut fTemp1180: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1177, 1)) as usize] } - fTemp1179;
			let mut fTemp1181: F64 = if iTemp659 != 0 {fTemp1179 + fTemp668 * fTemp1180 + (fTemp1175 - (iTemp1176) as F64) * (fTemp1178 - (fTemp1179 + fTemp668 * (fTemp1180 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1177, 4), 196607))) as usize] } - fTemp1178))))} else {1.0 - (fTemp1173 + fTemp668 * fTemp1174 + (fTemp1169 - (iTemp1170) as F64) * (fTemp1172 - (fTemp1173 + fTemp668 * (fTemp1174 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1171, 4), 196607))) as usize] } - fTemp1172)))))};
			let mut fTemp1182: F64 = fTemp673 + fTemp1168;
			let mut fTemp1183: F64 = 65535.0 * (1.0 - fTemp1182);
			let mut iTemp1184: i32 = (fTemp1183) as i32;
			let mut iTemp1185: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1184, 65535)))), 196607));
			let mut fTemp1186: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1185, 3)) as usize] };
			let mut fTemp1187: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1185 as usize] };
			let mut fTemp1188: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1185, 1)) as usize] } - fTemp1187;
			let mut fTemp1189: F64 = 65535.0 * fTemp1182;
			let mut iTemp1190: i32 = (fTemp1189) as i32;
			let mut iTemp1191: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1190, 65535)))), 196607));
			let mut fTemp1192: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1191, 3), 196607))) as usize] };
			let mut fTemp1193: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1191 as usize] };
			let mut fTemp1194: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1191, 1), 196607))) as usize] } - fTemp1193;
			let mut iTemp1195: i32 = (fTemp729 > ((if iTemp659 != 0 {fTemp1193 + fTemp668 * fTemp1194 + (fTemp1189 - (iTemp1190) as F64) * (fTemp1192 - (fTemp1193 + fTemp668 * (fTemp1194 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1191, 4), 196607))) as usize] } - fTemp1192))))} else {1.0 - (fTemp1187 + fTemp668 * fTemp1188 + (fTemp1183 - (iTemp1184) as F64) * (fTemp1186 - (fTemp1187 + fTemp668 * (fTemp1188 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1185, 4)) as usize] } - fTemp1186)))))} - fTemp1181) / (1.0 - fTemp1181))) as i32;
			let mut fTemp1196: F64 = F64::min(1.0, F64::max(0.0, 0.5 * (if iTemp1195 != 0 {fTemp1168} else {fTemp1166} + if iTemp1195 != 0 {fTemp1165} else {fTemp1168})));
			self.fRec15[0] = fTemp1196;
			let mut fTemp1197: F64 = 65535.0 * (1.0 - fTemp1196);
			let mut iTemp1198: i32 = (fTemp1197) as i32;
			let mut iTemp1199: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1198, 65535)))), 196607));
			let mut fTemp1200: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1199, 3)) as usize] };
			let mut fTemp1201: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1199 as usize] };
			let mut fTemp1202: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1199, 1)) as usize] } - fTemp1201;
			let mut fTemp1203: F64 = 65535.0 * fTemp1196;
			let mut iTemp1204: i32 = (fTemp1203) as i32;
			let mut iTemp1205: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1204, 65535)))), 196607));
			let mut fTemp1206: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1205, 3)) as usize] };
			let mut fTemp1207: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1205 as usize] };
			let mut fTemp1208: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1205, 1)) as usize] } - fTemp1207;
			let mut fTemp1209: F64 = if iTemp659 != 0 {fTemp1207 + fTemp668 * fTemp1208 + (fTemp1203 - (iTemp1204) as F64) * (fTemp1206 - (fTemp1207 + fTemp668 * (fTemp1208 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1205, 4), 196607))) as usize] } - fTemp1206))))} else {1.0 - (fTemp1201 + fTemp668 * fTemp1202 + (fTemp1197 - (iTemp1198) as F64) * (fTemp1200 - (fTemp1201 + fTemp668 * (fTemp1202 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1199, 4), 196607))) as usize] } - fTemp1200)))))};
			let mut fTemp1210: F64 = fTemp673 + fTemp1196;
			let mut fTemp1211: F64 = 65535.0 * (1.0 - fTemp1210);
			let mut iTemp1212: i32 = (fTemp1211) as i32;
			let mut iTemp1213: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1212, 65535)))), 196607));
			let mut fTemp1214: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1213, 3)) as usize] };
			let mut fTemp1215: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1213 as usize] };
			let mut fTemp1216: F64 = unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1213, 1)) as usize] } - fTemp1215;
			let mut fTemp1217: F64 = 65535.0 * fTemp1210;
			let mut iTemp1218: i32 = (fTemp1217) as i32;
			let mut iTemp1219: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp663, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1218, 65535)))), 196607));
			let mut fTemp1220: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1219, 3), 196607))) as usize] };
			let mut fTemp1221: F64 = unsafe { ftbl0LambRs48kSIG0[iTemp1219 as usize] };
			let mut fTemp1222: F64 = unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1219, 1), 196607))) as usize] } - fTemp1221;
			let mut fTemp1223: F64 = fTemp633 + if ((0.001 * fTemp672) == 0.0) as i32 != 0 {fTemp658} else {fTemp658 * (if iTemp659 != 0 {fTemp1221 + fTemp668 * fTemp1222 + (fTemp1217 - (iTemp1218) as F64) * (fTemp1220 - (fTemp1221 + fTemp668 * (fTemp1222 - (unsafe { ftbl0LambRs48kSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1219, 4), 196607))) as usize] } - fTemp1220))))} else {1.0 - (fTemp1215 + fTemp668 * fTemp1216 + (fTemp1211 - (iTemp1212) as F64) * (fTemp1214 - (fTemp1215 + fTemp668 * (fTemp1216 - (unsafe { ftbl0LambRs48kSIG0[(i32::wrapping_add(iTemp1213, 4)) as usize] } - fTemp1214)))))} - fTemp1209) / (1.0 - fTemp1209)};
			self.fRec16[(self.IOTA0 & 8191) as usize] = if iTemp671 != 0 {F64::min(fTemp1223, fTemp633)} else {F64::max(fTemp1223, fTemp633)};
			let mut fTemp1224: F64 = self.fRec16[((i32::wrapping_sub(self.IOTA0, iSlow70)) & 8191) as usize];
			self.fHbargraph2 = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, fTemp1224));
			*output1 = 0.5 * fTemp2 * self.fVec2[((i32::wrapping_sub(self.IOTA0, iSlow71)) & 32767) as usize] + self.fRec14[0] * fTemp3 * self.fVec3[((i32::wrapping_sub(self.IOTA0, iSlow71)) & 32767) as usize] * fTemp1224;
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
			self.fVec16[2] = self.fVec16[1];
			self.fVec16[1] = self.fVec16[0];
			for j2 in (1..=6).rev() {
				self.fVec17[j2 as usize] = self.fVec17[(i32::wrapping_sub(j2, 1)) as usize];
			}
			for j3 in (1..=14).rev() {
				self.fVec18[j3 as usize] = self.fVec18[(i32::wrapping_sub(j3, 1)) as usize];
			}
			self.fVec27[1] = self.fVec27[0];
			self.fVec28[1] = self.fVec28[0];
			self.fVec29[1] = self.fVec29[0];
			self.fRec1[1] = self.fRec1[0];
			self.fRec14[1] = self.fRec14[0];
			self.fVec31[2] = self.fVec31[1];
			self.fVec31[1] = self.fVec31[0];
			for j4 in (1..=6).rev() {
				self.fVec32[j4 as usize] = self.fVec32[(i32::wrapping_sub(j4, 1)) as usize];
			}
			for j5 in (1..=14).rev() {
				self.fVec33[j5 as usize] = self.fVec33[(i32::wrapping_sub(j5, 1)) as usize];
			}
			self.fRec17[1] = self.fRec17[0];
			self.fVec42[2] = self.fVec42[1];
			self.fVec42[1] = self.fVec42[0];
			for j6 in (1..=6).rev() {
				self.fVec43[j6 as usize] = self.fVec43[(i32::wrapping_sub(j6, 1)) as usize];
			}
			for j7 in (1..=14).rev() {
				self.fVec44[j7 as usize] = self.fVec44[(i32::wrapping_sub(j7, 1)) as usize];
			}
			self.fVec53[1] = self.fVec53[0];
			self.fVec54[1] = self.fVec54[0];
			self.fVec55[1] = self.fVec55[0];
			self.fRec15[1] = self.fRec15[0];
		}
	}

}

}

pub use dsp_48k::LambRs48k;
