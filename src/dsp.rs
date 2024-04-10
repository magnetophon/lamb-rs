mod dsp {
    /* ------------------------------------------------------------
author: "Bart Brouns"
license: "AGPLv3"
name: "lamb-rs"
version: "0.1"
Code generated with Faust 2.72.14 (https://faust.grame.fr)
Compilation options: -a /run/user/1001/.tmp25Ofh2 -lang rust -ct 1 -es 1 -mcd 16 -mdd 1024 -mdy 33 -double -ftz 0
------------------------------------------------------------ */
#![allow(clippy::all)]
#![allow(unused_parens)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_upper_case_globals)]

use faust_types::*;



#[derive(Debug,Clone)]
pub struct mydspSIG0 {
	iRec13: [i32;2],
}

impl mydspSIG0 {
	
	fn get_num_inputsmydspSIG0(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsmydspSIG0(&self) -> i32 {
		return 1;
	}
	
	fn instance_initmydspSIG0(&mut self, sample_rate: i32) {
		for l39 in 0..2 {
			self.iRec13[l39 as usize] = 0;
		}
	}
	
	fn fillmydspSIG0(&mut self, count: i32, table: &mut[F64]) {
		for i1 in 0..count {
			self.iRec13[0] = i32::wrapping_add(self.iRec13[1], 1);
			let mut iTemp59: i32 = i32::wrapping_add(self.iRec13[0], -1);
			let mut fTemp60: F64 = (iTemp59 % 3) as F64 as i32 as F64;
			let mut fTemp61: F64 = 0.5 * fTemp60;
			let mut fTemp62: F64 = F64::powf(fTemp61, 0.21 * fTemp60 + 1.0);
			let mut fTemp63: F64 = (0.3333333333333333 * (iTemp59 % 196608) as F64) as i32 as F64;
			table[i1 as usize] = F64::min(1.0, F64::max(0.0, if (fTemp61 == 0.0) as i32 != 0 {0.5 * (F64::sin(4.793763109162727e-05 * fTemp63 + 4.71238898038469) + 1.0)} else {0.5 * (F64::sin(3.141592653589793 * ((1.0 - F64::exp(-(3.692683299000534e-05 * fTemp62 * fTemp63))) / (1.0 - F64::exp(-(2.42 * fTemp62)))) + 4.71238898038469) + 1.0)}));
			self.iRec13[1] = self.iRec13[0];
		}
	}

}


pub fn newmydspSIG0() -> mydspSIG0 { 
	mydspSIG0 {
		iRec13: [0;2],
	}
}
fn mydsp_faustpower2_f(value: F64) -> F64 {
	return value * value;
}
static mut ftbl0mydspSIG0: [F64;196608] = [0.0;196608];
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
#[derive(Debug,Clone)]
pub struct mydsp {
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

impl FaustDsp for mydsp {
	type T = F64;
		
	fn new() -> mydsp { 
		mydsp {
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
		m.declare("compile_options", r"-a /run/user/1001/.tmp25Ofh2 -lang rust -ct 1 -es 1 -mcd 16 -mdd 1024 -mdy 33 -double -ftz 0");
		m.declare("filename", r"lamb-rs.dsp");
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
		let mut sig0: mydspSIG0 = newmydspSIG0();
		sig0.instance_initmydspSIG0(sample_rate);
		sig0.fillmydspSIG0(196608, unsafe { &mut ftbl0mydspSIG0 });
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
		mydsp::class_init(sample_rate);
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
			let mut fTemp5: F64 = 10.588235294117647 * (F64::max(0.15, self.fRec4[0]) + -0.15);
			let mut fTemp6: F64 = 15.0 - fTemp5;
			let mut fTemp7: F64 = 12.0 - fTemp5;
			let mut fTemp8: F64 = fTemp5 + -12.0;
			self.fRec11[0] = fSlow10 + self.fConst4 * self.fRec11[1];
			let mut fTemp9: F64 = *input0;
			self.fVec0[(self.IOTA0 & 32767) as usize] = fTemp9;
			let mut fTemp10: F64 = fTemp9 * self.fRec11[0];
			self.fVec1[(self.IOTA0 & 32767) as usize] = fTemp10;
			let mut fTemp11: F64 = F64::abs(fTemp10);
			let mut fTemp12: F64 = *input1;
			self.fVec2[(self.IOTA0 & 32767) as usize] = fTemp12;
			let mut fTemp13: F64 = fTemp12 * self.fRec11[0];
			self.fVec3[(self.IOTA0 & 32767) as usize] = fTemp13;
			let mut fTemp14: F64 = F64::abs(fTemp13);
			let mut fTemp15: F64 = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, F64::max(fTemp11, fTemp14)));
			let mut iTemp16: i32 = ((fTemp15 > fSlow11) as i32) + ((fTemp15 > fSlow9) as i32);
			let mut fTemp17: F64 = fTemp15 - fSlow8;
			let mut fTemp18: F64 = F64::powf(1e+01, -(0.05 * F64::max(0.0, if (iTemp16 == 0) as i32 != 0 {0.0} else {if (iTemp16 == 1) as i32 != 0 {fSlow12 * mydsp_faustpower2_f(fSlow7 + fTemp17)} else {fTemp17}})));
			let mut fTemp19: F64 = 4.0 * (F64::max(0.25, self.fRec4[0]) + -0.25);
			let mut fTemp20: F64 = 9.0 - fTemp19;
			let mut fTemp21: F64 = self.fRec5[1] - self.fRec6[1];
			let mut fTemp22: F64 = if (fTemp18 > self.fRec10[1]) as i32 != 0 {F64::exp(-(self.fConst7 / F64::max(2.220446049250313e-16, fSlow14 / F64::max(1.0 - (F64::max(fTemp19 + -9.0, F64::min(2.0, fTemp21)) + fTemp20) / (11.0 - fTemp19), 2.220446049250313e-16))))} else {self.fConst6};
			self.fRec10[0] = self.fRec10[1] * fTemp22 + fTemp18 * (1.0 - fTemp22);
			let mut fTemp23: F64 = if (self.fRec10[0] > self.fRec9[1]) as i32 != 0 {0.0} else {self.fConst6};
			self.fRec9[0] = self.fRec9[1] * fTemp23 + self.fRec10[0] * (1.0 - fTemp23);
			let mut fTemp24: F64 = if (self.fRec9[0] > self.fRec8[1]) as i32 != 0 {0.0} else {self.fConst6};
			self.fRec8[0] = self.fRec8[1] * fTemp24 + self.fRec9[0] * (1.0 - fTemp24);
			let mut fTemp25: F64 = if (self.fRec8[0] > self.fRec7[1]) as i32 != 0 {0.0} else {self.fConst6};
			self.fRec7[0] = self.fRec7[1] * fTemp25 + self.fRec8[0] * (1.0 - fTemp25);
			self.fRec5[0] = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, self.fRec7[0]));
			let mut fTemp26: F64 = F64::powf(1e+01, 0.05 * (self.fRec5[1] + fTemp20));
			let mut fTemp27: F64 = if (fTemp26 > self.fRec12[1]) as i32 != 0 {F64::exp(-(self.fConst7 / F64::max(2.220446049250313e-16, 0.05 * F64::powf(4503599627370496.0, 1.0 - (F64::max(fTemp8, F64::min(3.0, fTemp21)) + fTemp7) / fTemp6))))} else {self.fConst8};
			self.fRec12[0] = self.fRec12[1] * fTemp27 + fTemp26 * (1.0 - fTemp27);
			self.fRec6[0] = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, self.fRec12[0]));
			let mut fTemp28: F64 = self.fRec5[0] - self.fRec6[0];
			let mut fTemp29: F64 = F64::powf(1e+01, fSlow16 * F64::min(0.25, self.fRec4[0]) * (self.fRec6[0] + fTemp28 * (F64::max(fTemp8, F64::min(3.0, fTemp28)) + fTemp7) / fTemp6));
			let mut fTemp30: F64 = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, fTemp11));
			let mut fTemp31: F64 = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, fTemp14));
			let mut fTemp32: F64 = F64::max(fTemp30, fTemp31);
			let mut fTemp33: F64 = fTemp30 + fSlow17 * (fTemp32 - fTemp30);
			let mut iTemp34: i32 = ((fTemp33 > fSlow11) as i32) + ((fTemp33 > fSlow9) as i32);
			let mut fTemp35: F64 = fTemp33 - fSlow8;
			let mut fTemp36: F64 = F64::min(fTemp29, F64::powf(1e+01, -(fSlow18 * F64::max(0.0, if (iTemp34 == 0) as i32 != 0 {0.0} else {if (iTemp34 == 1) as i32 != 0 {fSlow12 * mydsp_faustpower2_f(fSlow7 + fTemp35)} else {fTemp35}}))));
			self.fVec4[(self.IOTA0 & 16383) as usize] = fTemp36;
			let mut fTemp37: F64 = F64::min(fTemp36, self.fVec4[((i32::wrapping_sub(self.IOTA0, 1)) & 16383) as usize]);
			self.fVec5[0] = fTemp37;
			let mut fTemp38: F64 = F64::min(fTemp37, self.fVec5[2]);
			self.fVec6[0] = fTemp38;
			let mut fTemp39: F64 = F64::min(fTemp38, self.fVec6[4]);
			self.fVec7[0] = fTemp39;
			let mut fTemp40: F64 = F64::min(fTemp39, self.fVec7[8]);
			self.fVec8[(self.IOTA0 & 31) as usize] = fTemp40;
			let mut fTemp41: F64 = F64::min(fTemp40, self.fVec8[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize]);
			self.fVec9[(self.IOTA0 & 63) as usize] = fTemp41;
			let mut fTemp42: F64 = F64::min(fTemp41, self.fVec9[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize]);
			self.fVec10[(self.IOTA0 & 127) as usize] = fTemp42;
			let mut fTemp43: F64 = F64::min(fTemp42, self.fVec10[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize]);
			self.fVec11[(self.IOTA0 & 255) as usize] = fTemp43;
			let mut fTemp44: F64 = F64::min(fTemp43, self.fVec11[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize]);
			self.fVec12[(self.IOTA0 & 511) as usize] = fTemp44;
			let mut fTemp45: F64 = F64::min(fTemp44, self.fVec12[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize]);
			self.fVec13[(self.IOTA0 & 1023) as usize] = fTemp45;
			let mut fTemp46: F64 = F64::min(fTemp45, self.fVec13[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize]);
			self.fVec14[(self.IOTA0 & 2047) as usize] = fTemp46;
			self.fVec15[(self.IOTA0 & 4095) as usize] = F64::min(fTemp46, self.fVec14[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			self.fRec3[0] = F64::max(F64::min(self.fRec3[1], self.fVec4[((i32::wrapping_sub(self.IOTA0, iSlow21)) & 16383) as usize]), F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow23 != 0 {fTemp36} else {1.7976931348623157e+308}, if iSlow24 != 0 {self.fVec5[iSlow23 as usize]} else {1.7976931348623157e+308}), if iSlow25 != 0 {self.fVec6[iSlow26 as usize]} else {1.7976931348623157e+308}), if iSlow27 != 0 {self.fVec7[iSlow28 as usize]} else {1.7976931348623157e+308}), if iSlow29 != 0 {self.fVec8[((i32::wrapping_sub(self.IOTA0, iSlow30)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow31 != 0 {self.fVec9[((i32::wrapping_sub(self.IOTA0, iSlow32)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow33 != 0 {self.fVec10[((i32::wrapping_sub(self.IOTA0, iSlow34)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow35 != 0 {self.fVec11[((i32::wrapping_sub(self.IOTA0, iSlow36)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow37 != 0 {self.fVec12[((i32::wrapping_sub(self.IOTA0, iSlow38)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow39 != 0 {self.fVec13[((i32::wrapping_sub(self.IOTA0, iSlow40)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow41 != 0 {self.fVec14[((i32::wrapping_sub(self.IOTA0, iSlow42)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow43 != 0 {self.fVec15[((i32::wrapping_sub(self.IOTA0, iSlow44)) & 4095) as usize]} else {1.7976931348623157e+308}));
			let mut fTemp47: F64 = F64::min(self.fRec3[0], self.fRec3[1]);
			self.fVec16[0] = fTemp47;
			let mut fTemp48: F64 = F64::min(fTemp47, self.fVec16[2]);
			self.fVec17[0] = fTemp48;
			let mut fTemp49: F64 = F64::min(fTemp48, self.fVec17[4]);
			self.fVec18[0] = fTemp49;
			let mut fTemp50: F64 = F64::min(fTemp49, self.fVec18[8]);
			self.fVec19[(self.IOTA0 & 31) as usize] = fTemp50;
			let mut fTemp51: F64 = F64::min(fTemp50, self.fVec19[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize]);
			self.fVec20[(self.IOTA0 & 63) as usize] = fTemp51;
			let mut fTemp52: F64 = F64::min(fTemp51, self.fVec20[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize]);
			self.fVec21[(self.IOTA0 & 127) as usize] = fTemp52;
			let mut fTemp53: F64 = F64::min(fTemp52, self.fVec21[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize]);
			self.fVec22[(self.IOTA0 & 255) as usize] = fTemp53;
			let mut fTemp54: F64 = F64::min(fTemp53, self.fVec22[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize]);
			self.fVec23[(self.IOTA0 & 511) as usize] = fTemp54;
			let mut fTemp55: F64 = F64::min(fTemp54, self.fVec23[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize]);
			self.fVec24[(self.IOTA0 & 1023) as usize] = fTemp55;
			let mut fTemp56: F64 = F64::min(fTemp55, self.fVec24[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize]);
			self.fVec25[(self.IOTA0 & 2047) as usize] = fTemp56;
			self.fVec26[(self.IOTA0 & 4095) as usize] = F64::min(fTemp56, self.fVec25[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			let mut fTemp57: F64 = F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow4 != 0 {self.fRec3[0]} else {1.7976931348623157e+308}, if iSlow45 != 0 {self.fVec16[iSlow4 as usize]} else {1.7976931348623157e+308}), if iSlow46 != 0 {self.fVec17[iSlow47 as usize]} else {1.7976931348623157e+308}), if iSlow48 != 0 {self.fVec18[iSlow49 as usize]} else {1.7976931348623157e+308}), if iSlow50 != 0 {self.fVec19[((i32::wrapping_sub(self.IOTA0, iSlow51)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow52 != 0 {self.fVec20[((i32::wrapping_sub(self.IOTA0, iSlow53)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow54 != 0 {self.fVec21[((i32::wrapping_sub(self.IOTA0, iSlow55)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow56 != 0 {self.fVec22[((i32::wrapping_sub(self.IOTA0, iSlow57)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow58 != 0 {self.fVec23[((i32::wrapping_sub(self.IOTA0, iSlow59)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow60 != 0 {self.fVec24[((i32::wrapping_sub(self.IOTA0, iSlow61)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow62 != 0 {self.fVec25[((i32::wrapping_sub(self.IOTA0, iSlow63)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow64 != 0 {self.fVec26[((i32::wrapping_sub(self.IOTA0, iSlow65)) & 4095) as usize]} else {1.7976931348623157e+308}) - fTemp4;
			self.fVec27[0] = fTemp57;
			let mut iTemp58: i32 = (fTemp57 > 0.0) as i32;
			let mut fTemp64: F64 = if iTemp58 != 0 {fSlow67} else {fSlow66};
			self.fVec28[0] = fTemp64;
			let mut fTemp65: F64 = 2.0 * fTemp64;
			let mut iTemp66: i32 = (fTemp65) as i32;
			let mut iTemp67: i32 = std::cmp::max(0, std::cmp::min(iTemp66, 2));
			let mut iTemp68: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, 98301), 196607));
			let mut fTemp69: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp68, 3)) as usize] };
			let mut fTemp70: F64 = unsafe { ftbl0mydspSIG0[iTemp68 as usize] };
			let mut fTemp71: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp68, 1)) as usize] } - fTemp70;
			let mut fTemp72: F64 = fTemp65 - (iTemp66) as F64;
			let mut fTemp73: F64 = fTemp70 + fTemp72 * fTemp71 + 0.5 * (fTemp69 - (fTemp70 + fTemp72 * (fTemp71 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp68, 4)) as usize] } - fTemp69))));
			let mut fTemp74: F64 = if iTemp58 != 0 {fTemp73} else {1.0 - fTemp73};
			let mut iTemp75: i32 = (fTemp57 < 0.0) as i32;
			let mut fTemp76: F64 = fSlow1 * (iTemp75) as F64 + fSlow13 * (iTemp58) as F64;
			self.fVec29[0] = fTemp76;
			let mut fTemp77: F64 = self.fConst10 / fTemp76;
			let mut fTemp78: F64 = fTemp77 + 0.5;
			let mut fTemp79: F64 = 65535.0 * (1.0 - fTemp78);
			let mut iTemp80: i32 = (fTemp79) as i32;
			let mut iTemp81: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp80, 65535)))), 196607));
			let mut fTemp82: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp81, 3)) as usize] };
			let mut fTemp83: F64 = unsafe { ftbl0mydspSIG0[iTemp81 as usize] };
			let mut fTemp84: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp81, 1)) as usize] } - fTemp83;
			let mut fTemp85: F64 = 65535.0 * fTemp78;
			let mut iTemp86: i32 = (fTemp85) as i32;
			let mut iTemp87: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp86, 65535)))), 196607));
			let mut fTemp88: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp87, 3), 196607))) as usize] };
			let mut fTemp89: F64 = unsafe { ftbl0mydspSIG0[iTemp87 as usize] };
			let mut fTemp90: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp87, 1), 196607))) as usize] } - fTemp89;
			let mut fTemp91: F64 = 2.0 * self.fVec28[1];
			let mut iTemp92: i32 = (fTemp91) as i32;
			let mut iTemp93: i32 = std::cmp::max(0, std::cmp::min(iTemp92, 2));
			let mut fTemp94: F64 = 65535.0 * (1.0 - self.fRec1[1]);
			let mut iTemp95: i32 = (fTemp94) as i32;
			let mut iTemp96: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp95, 65535))), iTemp93), 196607));
			let mut fTemp97: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp96, 3), 196607))) as usize] };
			let mut fTemp98: F64 = unsafe { ftbl0mydspSIG0[iTemp96 as usize] };
			let mut fTemp99: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp96, 1), 196607))) as usize] } - fTemp98;
			let mut fTemp100: F64 = fTemp91 - (iTemp92) as F64;
			let mut fTemp101: F64 = 65535.0 * self.fRec1[1];
			let mut iTemp102: i32 = (fTemp101) as i32;
			let mut iTemp103: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp93, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp102, 65535)))), 196607));
			let mut fTemp104: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp103, 3), 196607))) as usize] };
			let mut fTemp105: F64 = unsafe { ftbl0mydspSIG0[iTemp103 as usize] };
			let mut fTemp106: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp103, 1), 196607))) as usize] } - fTemp105;
			let mut fTemp107: F64 = self.fRec1[1] + fTemp77;
			let mut fTemp108: F64 = 65535.0 * (1.0 - fTemp107);
			let mut iTemp109: i32 = (fTemp108) as i32;
			let mut iTemp110: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp109, 65535)))), 196607));
			let mut fTemp111: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp110, 3)) as usize] };
			let mut fTemp112: F64 = unsafe { ftbl0mydspSIG0[iTemp110 as usize] };
			let mut fTemp113: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp110, 1)) as usize] } - fTemp112;
			let mut fTemp114: F64 = 65535.0 * fTemp107;
			let mut iTemp115: i32 = (fTemp114) as i32;
			let mut iTemp116: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp115, 65535)))), 196607));
			let mut fTemp117: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp116, 3), 196607))) as usize] };
			let mut fTemp118: F64 = unsafe { ftbl0mydspSIG0[iTemp116 as usize] };
			let mut fTemp119: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp116, 1), 196607))) as usize] } - fTemp118;
			let mut fTemp120: F64 = self.fRec1[1] + self.fConst10 * (1.0 / fTemp76 + 1.0 / self.fVec29[1]);
			let mut fTemp121: F64 = 65535.0 * (1.0 - fTemp120);
			let mut iTemp122: i32 = (fTemp121) as i32;
			let mut iTemp123: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp122, 65535))), iTemp67), 196607));
			let mut fTemp124: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp123, 3)) as usize] };
			let mut fTemp125: F64 = unsafe { ftbl0mydspSIG0[iTemp123 as usize] };
			let mut fTemp126: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp123, 1)) as usize] } - fTemp125;
			let mut fTemp127: F64 = 65535.0 * fTemp120;
			let mut iTemp128: i32 = (fTemp127) as i32;
			let mut iTemp129: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp128, 65535)))), 196607));
			let mut fTemp130: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp129, 3), 196607))) as usize] };
			let mut fTemp131: F64 = unsafe { ftbl0mydspSIG0[iTemp129 as usize] };
			let mut fTemp132: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp129, 1), 196607))) as usize] } - fTemp131;
			let mut fTemp133: F64 = (if iTemp58 != 0 {fTemp131 + fTemp72 * fTemp132 + (fTemp127 - (iTemp128) as F64) * (fTemp130 - (fTemp131 + fTemp72 * (fTemp132 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp129, 4), 196607))) as usize] } - fTemp130))))} else {1.0 - (fTemp125 + fTemp72 * fTemp126 + (fTemp121 - (iTemp122) as F64) * (fTemp124 - (fTemp125 + fTemp72 * (fTemp126 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp123, 4)) as usize] } - fTemp124)))))} - if iTemp58 != 0 {fTemp118 + fTemp72 * fTemp119 + (fTemp114 - (iTemp115) as F64) * (fTemp117 - (fTemp118 + fTemp72 * (fTemp119 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp116, 4), 196607))) as usize] } - fTemp117))))} else {1.0 - (fTemp112 + fTemp72 * fTemp113 + (fTemp108 - (iTemp109) as F64) * (fTemp111 - (fTemp112 + fTemp72 * (fTemp113 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp110, 4), 196607))) as usize] } - fTemp111)))))}) * self.fVec27[1] / (fTemp57 * (1.0 - if iTemp58 != 0 {fTemp105 + fTemp100 * fTemp106 + (fTemp101 - (iTemp102) as F64) * (fTemp104 - (fTemp105 + fTemp100 * (fTemp106 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp103, 4), 196607))) as usize] } - fTemp104))))} else {1.0 - (fTemp98 + fTemp100 * fTemp99 + (fTemp94 - (iTemp95) as F64) * (fTemp97 - (fTemp98 + fTemp100 * (fTemp99 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp96, 4), 196607))) as usize] } - fTemp97)))))}));
			let mut iTemp134: i32 = (fTemp133 > ((if iTemp58 != 0 {fTemp89 + fTemp72 * fTemp90 + (fTemp85 - (iTemp86) as F64) * (fTemp88 - (fTemp89 + fTemp72 * (fTemp90 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp87, 4), 196607))) as usize] } - fTemp88))))} else {1.0 - (fTemp83 + fTemp72 * fTemp84 + (fTemp79 - (iTemp80) as F64) * (fTemp82 - (fTemp83 + fTemp72 * (fTemp84 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp81, 4)) as usize] } - fTemp82)))))} - fTemp74) / (1.0 - fTemp74))) as i32;
			let mut fTemp135: F64 = if iTemp134 != 0 {1.0} else {0.5};
			let mut fTemp136: F64 = if iTemp134 != 0 {0.5} else {0.0};
			let mut fTemp137: F64 = fTemp136 + fTemp135;
			let mut fTemp138: F64 = 0.5 * fTemp137;
			let mut fTemp139: F64 = 65535.0 * (1.0 - fTemp138);
			let mut iTemp140: i32 = (fTemp139) as i32;
			let mut iTemp141: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp140, 65535)))), 196607));
			let mut fTemp142: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp141, 3)) as usize] };
			let mut fTemp143: F64 = unsafe { ftbl0mydspSIG0[iTemp141 as usize] };
			let mut fTemp144: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp141, 1)) as usize] } - fTemp143;
			let mut fTemp145: F64 = 32767.5 * fTemp137;
			let mut iTemp146: i32 = (fTemp145) as i32;
			let mut iTemp147: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp146, 65535)))), 196607));
			let mut fTemp148: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp147, 3)) as usize] };
			let mut fTemp149: F64 = unsafe { ftbl0mydspSIG0[iTemp147 as usize] };
			let mut fTemp150: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp147, 1)) as usize] } - fTemp149;
			let mut fTemp151: F64 = if iTemp58 != 0 {fTemp149 + fTemp72 * fTemp150 + (fTemp145 - (iTemp146) as F64) * (fTemp148 - (fTemp149 + fTemp72 * (fTemp150 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp147, 4)) as usize] } - fTemp148))))} else {1.0 - (fTemp143 + fTemp72 * fTemp144 + (fTemp139 - (iTemp140) as F64) * (fTemp142 - (fTemp143 + fTemp72 * (fTemp144 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp141, 4)) as usize] } - fTemp142)))))};
			let mut fTemp152: F64 = fTemp77 + fTemp138;
			let mut fTemp153: F64 = 65535.0 * (1.0 - fTemp152);
			let mut iTemp154: i32 = (fTemp153) as i32;
			let mut iTemp155: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp154, 65535)))), 196607));
			let mut fTemp156: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp155, 3)) as usize] };
			let mut fTemp157: F64 = unsafe { ftbl0mydspSIG0[iTemp155 as usize] };
			let mut fTemp158: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp155, 1)) as usize] } - fTemp157;
			let mut fTemp159: F64 = 65535.0 * fTemp152;
			let mut iTemp160: i32 = (fTemp159) as i32;
			let mut iTemp161: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp160, 65535)))), 196607));
			let mut fTemp162: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp161, 3), 196607))) as usize] };
			let mut fTemp163: F64 = unsafe { ftbl0mydspSIG0[iTemp161 as usize] };
			let mut fTemp164: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp161, 1), 196607))) as usize] } - fTemp163;
			let mut iTemp165: i32 = (fTemp133 > ((if iTemp58 != 0 {fTemp163 + fTemp72 * fTemp164 + (fTemp159 - (iTemp160) as F64) * (fTemp162 - (fTemp163 + fTemp72 * (fTemp164 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp161, 4), 196607))) as usize] } - fTemp162))))} else {1.0 - (fTemp157 + fTemp72 * fTemp158 + (fTemp153 - (iTemp154) as F64) * (fTemp156 - (fTemp157 + fTemp72 * (fTemp158 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp155, 4)) as usize] } - fTemp156)))))} - fTemp151) / (1.0 - fTemp151))) as i32;
			let mut fTemp166: F64 = if iTemp165 != 0 {fTemp135} else {fTemp138};
			let mut fTemp167: F64 = if iTemp165 != 0 {fTemp138} else {fTemp136};
			let mut fTemp168: F64 = fTemp167 + fTemp166;
			let mut fTemp169: F64 = 0.5 * fTemp168;
			let mut fTemp170: F64 = 65535.0 * (1.0 - fTemp169);
			let mut iTemp171: i32 = (fTemp170) as i32;
			let mut iTemp172: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp171, 65535)))), 196607));
			let mut fTemp173: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp172, 3)) as usize] };
			let mut fTemp174: F64 = unsafe { ftbl0mydspSIG0[iTemp172 as usize] };
			let mut fTemp175: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp172, 1)) as usize] } - fTemp174;
			let mut fTemp176: F64 = 32767.5 * fTemp168;
			let mut iTemp177: i32 = (fTemp176) as i32;
			let mut iTemp178: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp177, 65535)))), 196607));
			let mut fTemp179: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp178, 3)) as usize] };
			let mut fTemp180: F64 = unsafe { ftbl0mydspSIG0[iTemp178 as usize] };
			let mut fTemp181: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp178, 1)) as usize] } - fTemp180;
			let mut fTemp182: F64 = if iTemp58 != 0 {fTemp180 + fTemp72 * fTemp181 + (fTemp176 - (iTemp177) as F64) * (fTemp179 - (fTemp180 + fTemp72 * (fTemp181 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp178, 4)) as usize] } - fTemp179))))} else {1.0 - (fTemp174 + fTemp72 * fTemp175 + (fTemp170 - (iTemp171) as F64) * (fTemp173 - (fTemp174 + fTemp72 * (fTemp175 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp172, 4)) as usize] } - fTemp173)))))};
			let mut fTemp183: F64 = fTemp77 + fTemp169;
			let mut fTemp184: F64 = 65535.0 * (1.0 - fTemp183);
			let mut iTemp185: i32 = (fTemp184) as i32;
			let mut iTemp186: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp185, 65535)))), 196607));
			let mut fTemp187: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp186, 3)) as usize] };
			let mut fTemp188: F64 = unsafe { ftbl0mydspSIG0[iTemp186 as usize] };
			let mut fTemp189: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp186, 1)) as usize] } - fTemp188;
			let mut fTemp190: F64 = 65535.0 * fTemp183;
			let mut iTemp191: i32 = (fTemp190) as i32;
			let mut iTemp192: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp191, 65535)))), 196607));
			let mut fTemp193: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp192, 3), 196607))) as usize] };
			let mut fTemp194: F64 = unsafe { ftbl0mydspSIG0[iTemp192 as usize] };
			let mut fTemp195: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp192, 1), 196607))) as usize] } - fTemp194;
			let mut iTemp196: i32 = (fTemp133 > ((if iTemp58 != 0 {fTemp194 + fTemp72 * fTemp195 + (fTemp190 - (iTemp191) as F64) * (fTemp193 - (fTemp194 + fTemp72 * (fTemp195 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp192, 4), 196607))) as usize] } - fTemp193))))} else {1.0 - (fTemp188 + fTemp72 * fTemp189 + (fTemp184 - (iTemp185) as F64) * (fTemp187 - (fTemp188 + fTemp72 * (fTemp189 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp186, 4)) as usize] } - fTemp187)))))} - fTemp182) / (1.0 - fTemp182))) as i32;
			let mut fTemp197: F64 = if iTemp196 != 0 {fTemp166} else {fTemp169};
			let mut fTemp198: F64 = if iTemp196 != 0 {fTemp169} else {fTemp167};
			let mut fTemp199: F64 = fTemp198 + fTemp197;
			let mut fTemp200: F64 = 0.5 * fTemp199;
			let mut fTemp201: F64 = 65535.0 * (1.0 - fTemp200);
			let mut iTemp202: i32 = (fTemp201) as i32;
			let mut iTemp203: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp202, 65535)))), 196607));
			let mut fTemp204: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp203, 3)) as usize] };
			let mut fTemp205: F64 = unsafe { ftbl0mydspSIG0[iTemp203 as usize] };
			let mut fTemp206: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp203, 1)) as usize] } - fTemp205;
			let mut fTemp207: F64 = 32767.5 * fTemp199;
			let mut iTemp208: i32 = (fTemp207) as i32;
			let mut iTemp209: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp208, 65535)))), 196607));
			let mut fTemp210: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp209, 3)) as usize] };
			let mut fTemp211: F64 = unsafe { ftbl0mydspSIG0[iTemp209 as usize] };
			let mut fTemp212: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp209, 1)) as usize] } - fTemp211;
			let mut fTemp213: F64 = if iTemp58 != 0 {fTemp211 + fTemp72 * fTemp212 + (fTemp207 - (iTemp208) as F64) * (fTemp210 - (fTemp211 + fTemp72 * (fTemp212 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp209, 4)) as usize] } - fTemp210))))} else {1.0 - (fTemp205 + fTemp72 * fTemp206 + (fTemp201 - (iTemp202) as F64) * (fTemp204 - (fTemp205 + fTemp72 * (fTemp206 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp203, 4)) as usize] } - fTemp204)))))};
			let mut fTemp214: F64 = fTemp77 + fTemp200;
			let mut fTemp215: F64 = 65535.0 * (1.0 - fTemp214);
			let mut iTemp216: i32 = (fTemp215) as i32;
			let mut iTemp217: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp216, 65535)))), 196607));
			let mut fTemp218: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp217, 3)) as usize] };
			let mut fTemp219: F64 = unsafe { ftbl0mydspSIG0[iTemp217 as usize] };
			let mut fTemp220: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp217, 1)) as usize] } - fTemp219;
			let mut fTemp221: F64 = 65535.0 * fTemp214;
			let mut iTemp222: i32 = (fTemp221) as i32;
			let mut iTemp223: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp222, 65535)))), 196607));
			let mut fTemp224: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp223, 3), 196607))) as usize] };
			let mut fTemp225: F64 = unsafe { ftbl0mydspSIG0[iTemp223 as usize] };
			let mut fTemp226: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp223, 1), 196607))) as usize] } - fTemp225;
			let mut iTemp227: i32 = (fTemp133 > ((if iTemp58 != 0 {fTemp225 + fTemp72 * fTemp226 + (fTemp221 - (iTemp222) as F64) * (fTemp224 - (fTemp225 + fTemp72 * (fTemp226 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp223, 4), 196607))) as usize] } - fTemp224))))} else {1.0 - (fTemp219 + fTemp72 * fTemp220 + (fTemp215 - (iTemp216) as F64) * (fTemp218 - (fTemp219 + fTemp72 * (fTemp220 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp217, 4)) as usize] } - fTemp218)))))} - fTemp213) / (1.0 - fTemp213))) as i32;
			let mut fTemp228: F64 = if iTemp227 != 0 {fTemp197} else {fTemp200};
			let mut fTemp229: F64 = if iTemp227 != 0 {fTemp200} else {fTemp198};
			let mut fTemp230: F64 = fTemp229 + fTemp228;
			let mut fTemp231: F64 = 0.5 * fTemp230;
			let mut fTemp232: F64 = 65535.0 * (1.0 - fTemp231);
			let mut iTemp233: i32 = (fTemp232) as i32;
			let mut iTemp234: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp233, 65535)))), 196607));
			let mut fTemp235: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp234, 3)) as usize] };
			let mut fTemp236: F64 = unsafe { ftbl0mydspSIG0[iTemp234 as usize] };
			let mut fTemp237: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp234, 1)) as usize] } - fTemp236;
			let mut fTemp238: F64 = 32767.5 * fTemp230;
			let mut iTemp239: i32 = (fTemp238) as i32;
			let mut iTemp240: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp239, 65535)))), 196607));
			let mut fTemp241: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp240, 3)) as usize] };
			let mut fTemp242: F64 = unsafe { ftbl0mydspSIG0[iTemp240 as usize] };
			let mut fTemp243: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp240, 1)) as usize] } - fTemp242;
			let mut fTemp244: F64 = if iTemp58 != 0 {fTemp242 + fTemp72 * fTemp243 + (fTemp238 - (iTemp239) as F64) * (fTemp241 - (fTemp242 + fTemp72 * (fTemp243 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp240, 4)) as usize] } - fTemp241))))} else {1.0 - (fTemp236 + fTemp72 * fTemp237 + (fTemp232 - (iTemp233) as F64) * (fTemp235 - (fTemp236 + fTemp72 * (fTemp237 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp234, 4)) as usize] } - fTemp235)))))};
			let mut fTemp245: F64 = fTemp77 + fTemp231;
			let mut fTemp246: F64 = 65535.0 * (1.0 - fTemp245);
			let mut iTemp247: i32 = (fTemp246) as i32;
			let mut iTemp248: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp247, 65535)))), 196607));
			let mut fTemp249: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp248, 3)) as usize] };
			let mut fTemp250: F64 = unsafe { ftbl0mydspSIG0[iTemp248 as usize] };
			let mut fTemp251: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp248, 1)) as usize] } - fTemp250;
			let mut fTemp252: F64 = 65535.0 * fTemp245;
			let mut iTemp253: i32 = (fTemp252) as i32;
			let mut iTemp254: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp253, 65535)))), 196607));
			let mut fTemp255: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp254, 3), 196607))) as usize] };
			let mut fTemp256: F64 = unsafe { ftbl0mydspSIG0[iTemp254 as usize] };
			let mut fTemp257: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp254, 1), 196607))) as usize] } - fTemp256;
			let mut iTemp258: i32 = (fTemp133 > ((if iTemp58 != 0 {fTemp256 + fTemp72 * fTemp257 + (fTemp252 - (iTemp253) as F64) * (fTemp255 - (fTemp256 + fTemp72 * (fTemp257 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp254, 4), 196607))) as usize] } - fTemp255))))} else {1.0 - (fTemp250 + fTemp72 * fTemp251 + (fTemp246 - (iTemp247) as F64) * (fTemp249 - (fTemp250 + fTemp72 * (fTemp251 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp248, 4)) as usize] } - fTemp249)))))} - fTemp244) / (1.0 - fTemp244))) as i32;
			let mut fTemp259: F64 = if iTemp258 != 0 {fTemp228} else {fTemp231};
			let mut fTemp260: F64 = if iTemp258 != 0 {fTemp231} else {fTemp229};
			let mut fTemp261: F64 = fTemp260 + fTemp259;
			let mut fTemp262: F64 = 0.5 * fTemp261;
			let mut fTemp263: F64 = 65535.0 * (1.0 - fTemp262);
			let mut iTemp264: i32 = (fTemp263) as i32;
			let mut iTemp265: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp264, 65535)))), 196607));
			let mut fTemp266: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp265, 3)) as usize] };
			let mut fTemp267: F64 = unsafe { ftbl0mydspSIG0[iTemp265 as usize] };
			let mut fTemp268: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp265, 1)) as usize] } - fTemp267;
			let mut fTemp269: F64 = 32767.5 * fTemp261;
			let mut iTemp270: i32 = (fTemp269) as i32;
			let mut iTemp271: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp270, 65535)))), 196607));
			let mut fTemp272: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp271, 3)) as usize] };
			let mut fTemp273: F64 = unsafe { ftbl0mydspSIG0[iTemp271 as usize] };
			let mut fTemp274: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp271, 1)) as usize] } - fTemp273;
			let mut fTemp275: F64 = if iTemp58 != 0 {fTemp273 + fTemp72 * fTemp274 + (fTemp269 - (iTemp270) as F64) * (fTemp272 - (fTemp273 + fTemp72 * (fTemp274 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp271, 4)) as usize] } - fTemp272))))} else {1.0 - (fTemp267 + fTemp72 * fTemp268 + (fTemp263 - (iTemp264) as F64) * (fTemp266 - (fTemp267 + fTemp72 * (fTemp268 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp265, 4)) as usize] } - fTemp266)))))};
			let mut fTemp276: F64 = fTemp77 + fTemp262;
			let mut fTemp277: F64 = 65535.0 * (1.0 - fTemp276);
			let mut iTemp278: i32 = (fTemp277) as i32;
			let mut iTemp279: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp278, 65535)))), 196607));
			let mut fTemp280: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp279, 3)) as usize] };
			let mut fTemp281: F64 = unsafe { ftbl0mydspSIG0[iTemp279 as usize] };
			let mut fTemp282: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp279, 1)) as usize] } - fTemp281;
			let mut fTemp283: F64 = 65535.0 * fTemp276;
			let mut iTemp284: i32 = (fTemp283) as i32;
			let mut iTemp285: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp284, 65535)))), 196607));
			let mut fTemp286: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp285, 3), 196607))) as usize] };
			let mut fTemp287: F64 = unsafe { ftbl0mydspSIG0[iTemp285 as usize] };
			let mut fTemp288: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp285, 1), 196607))) as usize] } - fTemp287;
			let mut iTemp289: i32 = (fTemp133 > ((if iTemp58 != 0 {fTemp287 + fTemp72 * fTemp288 + (fTemp283 - (iTemp284) as F64) * (fTemp286 - (fTemp287 + fTemp72 * (fTemp288 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp285, 4), 196607))) as usize] } - fTemp286))))} else {1.0 - (fTemp281 + fTemp72 * fTemp282 + (fTemp277 - (iTemp278) as F64) * (fTemp280 - (fTemp281 + fTemp72 * (fTemp282 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp279, 4)) as usize] } - fTemp280)))))} - fTemp275) / (1.0 - fTemp275))) as i32;
			let mut fTemp290: F64 = if iTemp289 != 0 {fTemp259} else {fTemp262};
			let mut fTemp291: F64 = if iTemp289 != 0 {fTemp262} else {fTemp260};
			let mut fTemp292: F64 = fTemp291 + fTemp290;
			let mut fTemp293: F64 = 0.5 * fTemp292;
			let mut fTemp294: F64 = 65535.0 * (1.0 - fTemp293);
			let mut iTemp295: i32 = (fTemp294) as i32;
			let mut iTemp296: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp295, 65535)))), 196607));
			let mut fTemp297: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp296, 3)) as usize] };
			let mut fTemp298: F64 = unsafe { ftbl0mydspSIG0[iTemp296 as usize] };
			let mut fTemp299: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp296, 1)) as usize] } - fTemp298;
			let mut fTemp300: F64 = 32767.5 * fTemp292;
			let mut iTemp301: i32 = (fTemp300) as i32;
			let mut iTemp302: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp301, 65535)))), 196607));
			let mut fTemp303: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp302, 3)) as usize] };
			let mut fTemp304: F64 = unsafe { ftbl0mydspSIG0[iTemp302 as usize] };
			let mut fTemp305: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp302, 1)) as usize] } - fTemp304;
			let mut fTemp306: F64 = if iTemp58 != 0 {fTemp304 + fTemp72 * fTemp305 + (fTemp300 - (iTemp301) as F64) * (fTemp303 - (fTemp304 + fTemp72 * (fTemp305 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp302, 4)) as usize] } - fTemp303))))} else {1.0 - (fTemp298 + fTemp72 * fTemp299 + (fTemp294 - (iTemp295) as F64) * (fTemp297 - (fTemp298 + fTemp72 * (fTemp299 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp296, 4)) as usize] } - fTemp297)))))};
			let mut fTemp307: F64 = fTemp77 + fTemp293;
			let mut fTemp308: F64 = 65535.0 * (1.0 - fTemp307);
			let mut iTemp309: i32 = (fTemp308) as i32;
			let mut iTemp310: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp309, 65535)))), 196607));
			let mut fTemp311: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp310, 3)) as usize] };
			let mut fTemp312: F64 = unsafe { ftbl0mydspSIG0[iTemp310 as usize] };
			let mut fTemp313: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp310, 1)) as usize] } - fTemp312;
			let mut fTemp314: F64 = 65535.0 * fTemp307;
			let mut iTemp315: i32 = (fTemp314) as i32;
			let mut iTemp316: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp315, 65535)))), 196607));
			let mut fTemp317: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp316, 3), 196607))) as usize] };
			let mut fTemp318: F64 = unsafe { ftbl0mydspSIG0[iTemp316 as usize] };
			let mut fTemp319: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp316, 1), 196607))) as usize] } - fTemp318;
			let mut iTemp320: i32 = (fTemp133 > ((if iTemp58 != 0 {fTemp318 + fTemp72 * fTemp319 + (fTemp314 - (iTemp315) as F64) * (fTemp317 - (fTemp318 + fTemp72 * (fTemp319 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp316, 4), 196607))) as usize] } - fTemp317))))} else {1.0 - (fTemp312 + fTemp72 * fTemp313 + (fTemp308 - (iTemp309) as F64) * (fTemp311 - (fTemp312 + fTemp72 * (fTemp313 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp310, 4)) as usize] } - fTemp311)))))} - fTemp306) / (1.0 - fTemp306))) as i32;
			let mut fTemp321: F64 = if iTemp320 != 0 {fTemp290} else {fTemp293};
			let mut fTemp322: F64 = if iTemp320 != 0 {fTemp293} else {fTemp291};
			let mut fTemp323: F64 = fTemp322 + fTemp321;
			let mut fTemp324: F64 = 0.5 * fTemp323;
			let mut fTemp325: F64 = 65535.0 * (1.0 - fTemp324);
			let mut iTemp326: i32 = (fTemp325) as i32;
			let mut iTemp327: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp326, 65535)))), 196607));
			let mut fTemp328: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp327, 3)) as usize] };
			let mut fTemp329: F64 = unsafe { ftbl0mydspSIG0[iTemp327 as usize] };
			let mut fTemp330: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp327, 1)) as usize] } - fTemp329;
			let mut fTemp331: F64 = 32767.5 * fTemp323;
			let mut iTemp332: i32 = (fTemp331) as i32;
			let mut iTemp333: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp332, 65535)))), 196607));
			let mut fTemp334: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp333, 3)) as usize] };
			let mut fTemp335: F64 = unsafe { ftbl0mydspSIG0[iTemp333 as usize] };
			let mut fTemp336: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp333, 1)) as usize] } - fTemp335;
			let mut fTemp337: F64 = if iTemp58 != 0 {fTemp335 + fTemp72 * fTemp336 + (fTemp331 - (iTemp332) as F64) * (fTemp334 - (fTemp335 + fTemp72 * (fTemp336 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp333, 4)) as usize] } - fTemp334))))} else {1.0 - (fTemp329 + fTemp72 * fTemp330 + (fTemp325 - (iTemp326) as F64) * (fTemp328 - (fTemp329 + fTemp72 * (fTemp330 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp327, 4)) as usize] } - fTemp328)))))};
			let mut fTemp338: F64 = fTemp77 + fTemp324;
			let mut fTemp339: F64 = 65535.0 * (1.0 - fTemp338);
			let mut iTemp340: i32 = (fTemp339) as i32;
			let mut iTemp341: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp340, 65535)))), 196607));
			let mut fTemp342: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp341, 3)) as usize] };
			let mut fTemp343: F64 = unsafe { ftbl0mydspSIG0[iTemp341 as usize] };
			let mut fTemp344: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp341, 1)) as usize] } - fTemp343;
			let mut fTemp345: F64 = 65535.0 * fTemp338;
			let mut iTemp346: i32 = (fTemp345) as i32;
			let mut iTemp347: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp346, 65535)))), 196607));
			let mut fTemp348: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp347, 3), 196607))) as usize] };
			let mut fTemp349: F64 = unsafe { ftbl0mydspSIG0[iTemp347 as usize] };
			let mut fTemp350: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp347, 1), 196607))) as usize] } - fTemp349;
			let mut iTemp351: i32 = (fTemp133 > ((if iTemp58 != 0 {fTemp349 + fTemp72 * fTemp350 + (fTemp345 - (iTemp346) as F64) * (fTemp348 - (fTemp349 + fTemp72 * (fTemp350 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp347, 4), 196607))) as usize] } - fTemp348))))} else {1.0 - (fTemp343 + fTemp72 * fTemp344 + (fTemp339 - (iTemp340) as F64) * (fTemp342 - (fTemp343 + fTemp72 * (fTemp344 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp341, 4)) as usize] } - fTemp342)))))} - fTemp337) / (1.0 - fTemp337))) as i32;
			let mut fTemp352: F64 = if iTemp351 != 0 {fTemp321} else {fTemp324};
			let mut fTemp353: F64 = if iTemp351 != 0 {fTemp324} else {fTemp322};
			let mut fTemp354: F64 = fTemp353 + fTemp352;
			let mut fTemp355: F64 = 0.5 * fTemp354;
			let mut fTemp356: F64 = 65535.0 * (1.0 - fTemp355);
			let mut iTemp357: i32 = (fTemp356) as i32;
			let mut iTemp358: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp357, 65535)))), 196607));
			let mut fTemp359: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp358, 3)) as usize] };
			let mut fTemp360: F64 = unsafe { ftbl0mydspSIG0[iTemp358 as usize] };
			let mut fTemp361: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp358, 1)) as usize] } - fTemp360;
			let mut fTemp362: F64 = 32767.5 * fTemp354;
			let mut iTemp363: i32 = (fTemp362) as i32;
			let mut iTemp364: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp363, 65535)))), 196607));
			let mut fTemp365: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp364, 3)) as usize] };
			let mut fTemp366: F64 = unsafe { ftbl0mydspSIG0[iTemp364 as usize] };
			let mut fTemp367: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp364, 1)) as usize] } - fTemp366;
			let mut fTemp368: F64 = if iTemp58 != 0 {fTemp366 + fTemp72 * fTemp367 + (fTemp362 - (iTemp363) as F64) * (fTemp365 - (fTemp366 + fTemp72 * (fTemp367 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp364, 4)) as usize] } - fTemp365))))} else {1.0 - (fTemp360 + fTemp72 * fTemp361 + (fTemp356 - (iTemp357) as F64) * (fTemp359 - (fTemp360 + fTemp72 * (fTemp361 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp358, 4)) as usize] } - fTemp359)))))};
			let mut fTemp369: F64 = fTemp77 + fTemp355;
			let mut fTemp370: F64 = 65535.0 * (1.0 - fTemp369);
			let mut iTemp371: i32 = (fTemp370) as i32;
			let mut iTemp372: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp371, 65535)))), 196607));
			let mut fTemp373: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp372, 3)) as usize] };
			let mut fTemp374: F64 = unsafe { ftbl0mydspSIG0[iTemp372 as usize] };
			let mut fTemp375: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp372, 1)) as usize] } - fTemp374;
			let mut fTemp376: F64 = 65535.0 * fTemp369;
			let mut iTemp377: i32 = (fTemp376) as i32;
			let mut iTemp378: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp377, 65535)))), 196607));
			let mut fTemp379: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp378, 3), 196607))) as usize] };
			let mut fTemp380: F64 = unsafe { ftbl0mydspSIG0[iTemp378 as usize] };
			let mut fTemp381: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp378, 1), 196607))) as usize] } - fTemp380;
			let mut iTemp382: i32 = (fTemp133 > ((if iTemp58 != 0 {fTemp380 + fTemp72 * fTemp381 + (fTemp376 - (iTemp377) as F64) * (fTemp379 - (fTemp380 + fTemp72 * (fTemp381 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp378, 4), 196607))) as usize] } - fTemp379))))} else {1.0 - (fTemp374 + fTemp72 * fTemp375 + (fTemp370 - (iTemp371) as F64) * (fTemp373 - (fTemp374 + fTemp72 * (fTemp375 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp372, 4)) as usize] } - fTemp373)))))} - fTemp368) / (1.0 - fTemp368))) as i32;
			let mut fTemp383: F64 = if iTemp382 != 0 {fTemp352} else {fTemp355};
			let mut fTemp384: F64 = if iTemp382 != 0 {fTemp355} else {fTemp353};
			let mut fTemp385: F64 = fTemp384 + fTemp383;
			let mut fTemp386: F64 = 0.5 * fTemp385;
			let mut fTemp387: F64 = 65535.0 * (1.0 - fTemp386);
			let mut iTemp388: i32 = (fTemp387) as i32;
			let mut iTemp389: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp388, 65535)))), 196607));
			let mut fTemp390: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp389, 3)) as usize] };
			let mut fTemp391: F64 = unsafe { ftbl0mydspSIG0[iTemp389 as usize] };
			let mut fTemp392: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp389, 1)) as usize] } - fTemp391;
			let mut fTemp393: F64 = 32767.5 * fTemp385;
			let mut iTemp394: i32 = (fTemp393) as i32;
			let mut iTemp395: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp394, 65535)))), 196607));
			let mut fTemp396: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp395, 3)) as usize] };
			let mut fTemp397: F64 = unsafe { ftbl0mydspSIG0[iTemp395 as usize] };
			let mut fTemp398: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp395, 1)) as usize] } - fTemp397;
			let mut fTemp399: F64 = if iTemp58 != 0 {fTemp397 + fTemp72 * fTemp398 + (fTemp393 - (iTemp394) as F64) * (fTemp396 - (fTemp397 + fTemp72 * (fTemp398 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp395, 4)) as usize] } - fTemp396))))} else {1.0 - (fTemp391 + fTemp72 * fTemp392 + (fTemp387 - (iTemp388) as F64) * (fTemp390 - (fTemp391 + fTemp72 * (fTemp392 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp389, 4)) as usize] } - fTemp390)))))};
			let mut fTemp400: F64 = fTemp77 + fTemp386;
			let mut fTemp401: F64 = 65535.0 * (1.0 - fTemp400);
			let mut iTemp402: i32 = (fTemp401) as i32;
			let mut iTemp403: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp402, 65535)))), 196607));
			let mut fTemp404: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp403, 3)) as usize] };
			let mut fTemp405: F64 = unsafe { ftbl0mydspSIG0[iTemp403 as usize] };
			let mut fTemp406: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp403, 1)) as usize] } - fTemp405;
			let mut fTemp407: F64 = 65535.0 * fTemp400;
			let mut iTemp408: i32 = (fTemp407) as i32;
			let mut iTemp409: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp408, 65535)))), 196607));
			let mut fTemp410: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp409, 3), 196607))) as usize] };
			let mut fTemp411: F64 = unsafe { ftbl0mydspSIG0[iTemp409 as usize] };
			let mut fTemp412: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp409, 1), 196607))) as usize] } - fTemp411;
			let mut iTemp413: i32 = (fTemp133 > ((if iTemp58 != 0 {fTemp411 + fTemp72 * fTemp412 + (fTemp407 - (iTemp408) as F64) * (fTemp410 - (fTemp411 + fTemp72 * (fTemp412 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp409, 4), 196607))) as usize] } - fTemp410))))} else {1.0 - (fTemp405 + fTemp72 * fTemp406 + (fTemp401 - (iTemp402) as F64) * (fTemp404 - (fTemp405 + fTemp72 * (fTemp406 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp403, 4)) as usize] } - fTemp404)))))} - fTemp399) / (1.0 - fTemp399))) as i32;
			let mut fTemp414: F64 = if iTemp413 != 0 {fTemp383} else {fTemp386};
			let mut fTemp415: F64 = if iTemp413 != 0 {fTemp386} else {fTemp384};
			let mut fTemp416: F64 = fTemp415 + fTemp414;
			let mut fTemp417: F64 = 0.5 * fTemp416;
			let mut fTemp418: F64 = 65535.0 * (1.0 - fTemp417);
			let mut iTemp419: i32 = (fTemp418) as i32;
			let mut iTemp420: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp419, 65535)))), 196607));
			let mut fTemp421: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp420, 3)) as usize] };
			let mut fTemp422: F64 = unsafe { ftbl0mydspSIG0[iTemp420 as usize] };
			let mut fTemp423: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp420, 1)) as usize] } - fTemp422;
			let mut fTemp424: F64 = 32767.5 * fTemp416;
			let mut iTemp425: i32 = (fTemp424) as i32;
			let mut iTemp426: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp425, 65535)))), 196607));
			let mut fTemp427: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp426, 3)) as usize] };
			let mut fTemp428: F64 = unsafe { ftbl0mydspSIG0[iTemp426 as usize] };
			let mut fTemp429: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp426, 1)) as usize] } - fTemp428;
			let mut fTemp430: F64 = if iTemp58 != 0 {fTemp428 + fTemp72 * fTemp429 + (fTemp424 - (iTemp425) as F64) * (fTemp427 - (fTemp428 + fTemp72 * (fTemp429 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp426, 4)) as usize] } - fTemp427))))} else {1.0 - (fTemp422 + fTemp72 * fTemp423 + (fTemp418 - (iTemp419) as F64) * (fTemp421 - (fTemp422 + fTemp72 * (fTemp423 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp420, 4)) as usize] } - fTemp421)))))};
			let mut fTemp431: F64 = fTemp77 + fTemp417;
			let mut fTemp432: F64 = 65535.0 * (1.0 - fTemp431);
			let mut iTemp433: i32 = (fTemp432) as i32;
			let mut iTemp434: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp433, 65535)))), 196607));
			let mut fTemp435: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp434, 3)) as usize] };
			let mut fTemp436: F64 = unsafe { ftbl0mydspSIG0[iTemp434 as usize] };
			let mut fTemp437: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp434, 1)) as usize] } - fTemp436;
			let mut fTemp438: F64 = 65535.0 * fTemp431;
			let mut iTemp439: i32 = (fTemp438) as i32;
			let mut iTemp440: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp439, 65535)))), 196607));
			let mut fTemp441: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp440, 3), 196607))) as usize] };
			let mut fTemp442: F64 = unsafe { ftbl0mydspSIG0[iTemp440 as usize] };
			let mut fTemp443: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp440, 1), 196607))) as usize] } - fTemp442;
			let mut iTemp444: i32 = (fTemp133 > ((if iTemp58 != 0 {fTemp442 + fTemp72 * fTemp443 + (fTemp438 - (iTemp439) as F64) * (fTemp441 - (fTemp442 + fTemp72 * (fTemp443 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp440, 4), 196607))) as usize] } - fTemp441))))} else {1.0 - (fTemp436 + fTemp72 * fTemp437 + (fTemp432 - (iTemp433) as F64) * (fTemp435 - (fTemp436 + fTemp72 * (fTemp437 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp434, 4)) as usize] } - fTemp435)))))} - fTemp430) / (1.0 - fTemp430))) as i32;
			let mut fTemp445: F64 = if iTemp444 != 0 {fTemp414} else {fTemp417};
			let mut fTemp446: F64 = if iTemp444 != 0 {fTemp417} else {fTemp415};
			let mut fTemp447: F64 = fTemp446 + fTemp445;
			let mut fTemp448: F64 = 0.5 * fTemp447;
			let mut fTemp449: F64 = 65535.0 * (1.0 - fTemp448);
			let mut iTemp450: i32 = (fTemp449) as i32;
			let mut iTemp451: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp450, 65535)))), 196607));
			let mut fTemp452: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp451, 3)) as usize] };
			let mut fTemp453: F64 = unsafe { ftbl0mydspSIG0[iTemp451 as usize] };
			let mut fTemp454: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp451, 1)) as usize] } - fTemp453;
			let mut fTemp455: F64 = 32767.5 * fTemp447;
			let mut iTemp456: i32 = (fTemp455) as i32;
			let mut iTemp457: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp456, 65535)))), 196607));
			let mut fTemp458: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp457, 3)) as usize] };
			let mut fTemp459: F64 = unsafe { ftbl0mydspSIG0[iTemp457 as usize] };
			let mut fTemp460: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp457, 1)) as usize] } - fTemp459;
			let mut fTemp461: F64 = if iTemp58 != 0 {fTemp459 + fTemp72 * fTemp460 + (fTemp455 - (iTemp456) as F64) * (fTemp458 - (fTemp459 + fTemp72 * (fTemp460 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp457, 4)) as usize] } - fTemp458))))} else {1.0 - (fTemp453 + fTemp72 * fTemp454 + (fTemp449 - (iTemp450) as F64) * (fTemp452 - (fTemp453 + fTemp72 * (fTemp454 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp451, 4)) as usize] } - fTemp452)))))};
			let mut fTemp462: F64 = fTemp77 + fTemp448;
			let mut fTemp463: F64 = 65535.0 * (1.0 - fTemp462);
			let mut iTemp464: i32 = (fTemp463) as i32;
			let mut iTemp465: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp464, 65535)))), 196607));
			let mut fTemp466: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp465, 3)) as usize] };
			let mut fTemp467: F64 = unsafe { ftbl0mydspSIG0[iTemp465 as usize] };
			let mut fTemp468: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp465, 1)) as usize] } - fTemp467;
			let mut fTemp469: F64 = 65535.0 * fTemp462;
			let mut iTemp470: i32 = (fTemp469) as i32;
			let mut iTemp471: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp470, 65535)))), 196607));
			let mut fTemp472: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp471, 3), 196607))) as usize] };
			let mut fTemp473: F64 = unsafe { ftbl0mydspSIG0[iTemp471 as usize] };
			let mut fTemp474: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp471, 1), 196607))) as usize] } - fTemp473;
			let mut iTemp475: i32 = (fTemp133 > ((if iTemp58 != 0 {fTemp473 + fTemp72 * fTemp474 + (fTemp469 - (iTemp470) as F64) * (fTemp472 - (fTemp473 + fTemp72 * (fTemp474 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp471, 4), 196607))) as usize] } - fTemp472))))} else {1.0 - (fTemp467 + fTemp72 * fTemp468 + (fTemp463 - (iTemp464) as F64) * (fTemp466 - (fTemp467 + fTemp72 * (fTemp468 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp465, 4)) as usize] } - fTemp466)))))} - fTemp461) / (1.0 - fTemp461))) as i32;
			let mut fTemp476: F64 = if iTemp475 != 0 {fTemp445} else {fTemp448};
			let mut fTemp477: F64 = if iTemp475 != 0 {fTemp448} else {fTemp446};
			let mut fTemp478: F64 = fTemp477 + fTemp476;
			let mut fTemp479: F64 = 0.5 * fTemp478;
			let mut fTemp480: F64 = 65535.0 * (1.0 - fTemp479);
			let mut iTemp481: i32 = (fTemp480) as i32;
			let mut iTemp482: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp481, 65535)))), 196607));
			let mut fTemp483: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp482, 3)) as usize] };
			let mut fTemp484: F64 = unsafe { ftbl0mydspSIG0[iTemp482 as usize] };
			let mut fTemp485: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp482, 1)) as usize] } - fTemp484;
			let mut fTemp486: F64 = 32767.5 * fTemp478;
			let mut iTemp487: i32 = (fTemp486) as i32;
			let mut iTemp488: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp487, 65535)))), 196607));
			let mut fTemp489: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp488, 3)) as usize] };
			let mut fTemp490: F64 = unsafe { ftbl0mydspSIG0[iTemp488 as usize] };
			let mut fTemp491: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp488, 1)) as usize] } - fTemp490;
			let mut fTemp492: F64 = if iTemp58 != 0 {fTemp490 + fTemp72 * fTemp491 + (fTemp486 - (iTemp487) as F64) * (fTemp489 - (fTemp490 + fTemp72 * (fTemp491 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp488, 4)) as usize] } - fTemp489))))} else {1.0 - (fTemp484 + fTemp72 * fTemp485 + (fTemp480 - (iTemp481) as F64) * (fTemp483 - (fTemp484 + fTemp72 * (fTemp485 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp482, 4)) as usize] } - fTemp483)))))};
			let mut fTemp493: F64 = fTemp77 + fTemp479;
			let mut fTemp494: F64 = 65535.0 * (1.0 - fTemp493);
			let mut iTemp495: i32 = (fTemp494) as i32;
			let mut iTemp496: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp495, 65535)))), 196607));
			let mut fTemp497: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp496, 3)) as usize] };
			let mut fTemp498: F64 = unsafe { ftbl0mydspSIG0[iTemp496 as usize] };
			let mut fTemp499: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp496, 1)) as usize] } - fTemp498;
			let mut fTemp500: F64 = 65535.0 * fTemp493;
			let mut iTemp501: i32 = (fTemp500) as i32;
			let mut iTemp502: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp501, 65535)))), 196607));
			let mut fTemp503: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp502, 3), 196607))) as usize] };
			let mut fTemp504: F64 = unsafe { ftbl0mydspSIG0[iTemp502 as usize] };
			let mut fTemp505: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp502, 1), 196607))) as usize] } - fTemp504;
			let mut iTemp506: i32 = (fTemp133 > ((if iTemp58 != 0 {fTemp504 + fTemp72 * fTemp505 + (fTemp500 - (iTemp501) as F64) * (fTemp503 - (fTemp504 + fTemp72 * (fTemp505 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp502, 4), 196607))) as usize] } - fTemp503))))} else {1.0 - (fTemp498 + fTemp72 * fTemp499 + (fTemp494 - (iTemp495) as F64) * (fTemp497 - (fTemp498 + fTemp72 * (fTemp499 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp496, 4)) as usize] } - fTemp497)))))} - fTemp492) / (1.0 - fTemp492))) as i32;
			let mut fTemp507: F64 = if iTemp506 != 0 {fTemp476} else {fTemp479};
			let mut fTemp508: F64 = if iTemp506 != 0 {fTemp479} else {fTemp477};
			let mut fTemp509: F64 = fTemp508 + fTemp507;
			let mut fTemp510: F64 = 0.5 * fTemp509;
			let mut fTemp511: F64 = 65535.0 * (1.0 - fTemp510);
			let mut iTemp512: i32 = (fTemp511) as i32;
			let mut iTemp513: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp512, 65535)))), 196607));
			let mut fTemp514: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp513, 3)) as usize] };
			let mut fTemp515: F64 = unsafe { ftbl0mydspSIG0[iTemp513 as usize] };
			let mut fTemp516: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp513, 1)) as usize] } - fTemp515;
			let mut fTemp517: F64 = 32767.5 * fTemp509;
			let mut iTemp518: i32 = (fTemp517) as i32;
			let mut iTemp519: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp518, 65535)))), 196607));
			let mut fTemp520: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp519, 3)) as usize] };
			let mut fTemp521: F64 = unsafe { ftbl0mydspSIG0[iTemp519 as usize] };
			let mut fTemp522: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp519, 1)) as usize] } - fTemp521;
			let mut fTemp523: F64 = if iTemp58 != 0 {fTemp521 + fTemp72 * fTemp522 + (fTemp517 - (iTemp518) as F64) * (fTemp520 - (fTemp521 + fTemp72 * (fTemp522 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp519, 4)) as usize] } - fTemp520))))} else {1.0 - (fTemp515 + fTemp72 * fTemp516 + (fTemp511 - (iTemp512) as F64) * (fTemp514 - (fTemp515 + fTemp72 * (fTemp516 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp513, 4)) as usize] } - fTemp514)))))};
			let mut fTemp524: F64 = fTemp77 + fTemp510;
			let mut fTemp525: F64 = 65535.0 * (1.0 - fTemp524);
			let mut iTemp526: i32 = (fTemp525) as i32;
			let mut iTemp527: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp526, 65535)))), 196607));
			let mut fTemp528: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp527, 3)) as usize] };
			let mut fTemp529: F64 = unsafe { ftbl0mydspSIG0[iTemp527 as usize] };
			let mut fTemp530: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp527, 1)) as usize] } - fTemp529;
			let mut fTemp531: F64 = 65535.0 * fTemp524;
			let mut iTemp532: i32 = (fTemp531) as i32;
			let mut iTemp533: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp532, 65535)))), 196607));
			let mut fTemp534: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp533, 3), 196607))) as usize] };
			let mut fTemp535: F64 = unsafe { ftbl0mydspSIG0[iTemp533 as usize] };
			let mut fTemp536: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp533, 1), 196607))) as usize] } - fTemp535;
			let mut iTemp537: i32 = (fTemp133 > ((if iTemp58 != 0 {fTemp535 + fTemp72 * fTemp536 + (fTemp531 - (iTemp532) as F64) * (fTemp534 - (fTemp535 + fTemp72 * (fTemp536 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp533, 4), 196607))) as usize] } - fTemp534))))} else {1.0 - (fTemp529 + fTemp72 * fTemp530 + (fTemp525 - (iTemp526) as F64) * (fTemp528 - (fTemp529 + fTemp72 * (fTemp530 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp527, 4)) as usize] } - fTemp528)))))} - fTemp523) / (1.0 - fTemp523))) as i32;
			let mut fTemp538: F64 = if iTemp537 != 0 {fTemp507} else {fTemp510};
			let mut fTemp539: F64 = if iTemp537 != 0 {fTemp510} else {fTemp508};
			let mut fTemp540: F64 = fTemp539 + fTemp538;
			let mut fTemp541: F64 = 0.5 * fTemp540;
			let mut fTemp542: F64 = 65535.0 * (1.0 - fTemp541);
			let mut iTemp543: i32 = (fTemp542) as i32;
			let mut iTemp544: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp543, 65535)))), 196607));
			let mut fTemp545: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp544, 3)) as usize] };
			let mut fTemp546: F64 = unsafe { ftbl0mydspSIG0[iTemp544 as usize] };
			let mut fTemp547: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp544, 1)) as usize] } - fTemp546;
			let mut fTemp548: F64 = 32767.5 * fTemp540;
			let mut iTemp549: i32 = (fTemp548) as i32;
			let mut iTemp550: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp549, 65535)))), 196607));
			let mut fTemp551: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp550, 3)) as usize] };
			let mut fTemp552: F64 = unsafe { ftbl0mydspSIG0[iTemp550 as usize] };
			let mut fTemp553: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp550, 1)) as usize] } - fTemp552;
			let mut fTemp554: F64 = if iTemp58 != 0 {fTemp552 + fTemp72 * fTemp553 + (fTemp548 - (iTemp549) as F64) * (fTemp551 - (fTemp552 + fTemp72 * (fTemp553 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp550, 4)) as usize] } - fTemp551))))} else {1.0 - (fTemp546 + fTemp72 * fTemp547 + (fTemp542 - (iTemp543) as F64) * (fTemp545 - (fTemp546 + fTemp72 * (fTemp547 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp544, 4)) as usize] } - fTemp545)))))};
			let mut fTemp555: F64 = fTemp77 + fTemp541;
			let mut fTemp556: F64 = 65535.0 * (1.0 - fTemp555);
			let mut iTemp557: i32 = (fTemp556) as i32;
			let mut iTemp558: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp557, 65535)))), 196607));
			let mut fTemp559: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp558, 3)) as usize] };
			let mut fTemp560: F64 = unsafe { ftbl0mydspSIG0[iTemp558 as usize] };
			let mut fTemp561: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp558, 1)) as usize] } - fTemp560;
			let mut fTemp562: F64 = 65535.0 * fTemp555;
			let mut iTemp563: i32 = (fTemp562) as i32;
			let mut iTemp564: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp563, 65535)))), 196607));
			let mut fTemp565: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp564, 3), 196607))) as usize] };
			let mut fTemp566: F64 = unsafe { ftbl0mydspSIG0[iTemp564 as usize] };
			let mut fTemp567: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp564, 1), 196607))) as usize] } - fTemp566;
			let mut iTemp568: i32 = (fTemp133 > ((if iTemp58 != 0 {fTemp566 + fTemp72 * fTemp567 + (fTemp562 - (iTemp563) as F64) * (fTemp565 - (fTemp566 + fTemp72 * (fTemp567 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp564, 4), 196607))) as usize] } - fTemp565))))} else {1.0 - (fTemp560 + fTemp72 * fTemp561 + (fTemp556 - (iTemp557) as F64) * (fTemp559 - (fTemp560 + fTemp72 * (fTemp561 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp558, 4)) as usize] } - fTemp559)))))} - fTemp554) / (1.0 - fTemp554))) as i32;
			let mut fTemp569: F64 = if iTemp568 != 0 {fTemp538} else {fTemp541};
			let mut fTemp570: F64 = if iTemp568 != 0 {fTemp541} else {fTemp539};
			let mut fTemp571: F64 = fTemp570 + fTemp569;
			let mut fTemp572: F64 = 0.5 * fTemp571;
			let mut fTemp573: F64 = 65535.0 * (1.0 - fTemp572);
			let mut iTemp574: i32 = (fTemp573) as i32;
			let mut iTemp575: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp574, 65535)))), 196607));
			let mut fTemp576: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp575, 3)) as usize] };
			let mut fTemp577: F64 = unsafe { ftbl0mydspSIG0[iTemp575 as usize] };
			let mut fTemp578: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp575, 1)) as usize] } - fTemp577;
			let mut fTemp579: F64 = 32767.5 * fTemp571;
			let mut iTemp580: i32 = (fTemp579) as i32;
			let mut iTemp581: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp580, 65535)))), 196607));
			let mut fTemp582: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp581, 3)) as usize] };
			let mut fTemp583: F64 = unsafe { ftbl0mydspSIG0[iTemp581 as usize] };
			let mut fTemp584: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp581, 1)) as usize] } - fTemp583;
			let mut fTemp585: F64 = if iTemp58 != 0 {fTemp583 + fTemp72 * fTemp584 + (fTemp579 - (iTemp580) as F64) * (fTemp582 - (fTemp583 + fTemp72 * (fTemp584 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp581, 4), 196607))) as usize] } - fTemp582))))} else {1.0 - (fTemp577 + fTemp72 * fTemp578 + (fTemp573 - (iTemp574) as F64) * (fTemp576 - (fTemp577 + fTemp72 * (fTemp578 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp575, 4), 196607))) as usize] } - fTemp576)))))};
			let mut fTemp586: F64 = fTemp77 + fTemp572;
			let mut fTemp587: F64 = 65535.0 * (1.0 - fTemp586);
			let mut iTemp588: i32 = (fTemp587) as i32;
			let mut iTemp589: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp588, 65535)))), 196607));
			let mut fTemp590: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp589, 3)) as usize] };
			let mut fTemp591: F64 = unsafe { ftbl0mydspSIG0[iTemp589 as usize] };
			let mut fTemp592: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp589, 1)) as usize] } - fTemp591;
			let mut fTemp593: F64 = 65535.0 * fTemp586;
			let mut iTemp594: i32 = (fTemp593) as i32;
			let mut iTemp595: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp594, 65535)))), 196607));
			let mut fTemp596: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp595, 3), 196607))) as usize] };
			let mut fTemp597: F64 = unsafe { ftbl0mydspSIG0[iTemp595 as usize] };
			let mut fTemp598: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp595, 1), 196607))) as usize] } - fTemp597;
			let mut iTemp599: i32 = (fTemp133 > ((if iTemp58 != 0 {fTemp597 + fTemp72 * fTemp598 + (fTemp593 - (iTemp594) as F64) * (fTemp596 - (fTemp597 + fTemp72 * (fTemp598 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp595, 4), 196607))) as usize] } - fTemp596))))} else {1.0 - (fTemp591 + fTemp72 * fTemp592 + (fTemp587 - (iTemp588) as F64) * (fTemp590 - (fTemp591 + fTemp72 * (fTemp592 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp589, 4)) as usize] } - fTemp590)))))} - fTemp585) / (1.0 - fTemp585))) as i32;
			let mut fTemp600: F64 = F64::min(1.0, F64::max(0.0, 0.5 * (if iTemp599 != 0 {fTemp572} else {fTemp570} + if iTemp599 != 0 {fTemp569} else {fTemp572})));
			self.fRec1[0] = fTemp600;
			let mut fTemp601: F64 = 65535.0 * (1.0 - fTemp600);
			let mut iTemp602: i32 = (fTemp601) as i32;
			let mut iTemp603: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp602, 65535)))), 196607));
			let mut fTemp604: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp603, 3)) as usize] };
			let mut fTemp605: F64 = unsafe { ftbl0mydspSIG0[iTemp603 as usize] };
			let mut fTemp606: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp603, 1)) as usize] } - fTemp605;
			let mut fTemp607: F64 = 65535.0 * fTemp600;
			let mut iTemp608: i32 = (fTemp607) as i32;
			let mut iTemp609: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp608, 65535)))), 196607));
			let mut fTemp610: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp609, 3)) as usize] };
			let mut fTemp611: F64 = unsafe { ftbl0mydspSIG0[iTemp609 as usize] };
			let mut fTemp612: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp609, 1)) as usize] } - fTemp611;
			let mut fTemp613: F64 = if iTemp58 != 0 {fTemp611 + fTemp72 * fTemp612 + (fTemp607 - (iTemp608) as F64) * (fTemp610 - (fTemp611 + fTemp72 * (fTemp612 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp609, 4), 196607))) as usize] } - fTemp610))))} else {1.0 - (fTemp605 + fTemp72 * fTemp606 + (fTemp601 - (iTemp602) as F64) * (fTemp604 - (fTemp605 + fTemp72 * (fTemp606 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp603, 4), 196607))) as usize] } - fTemp604)))))};
			let mut fTemp614: F64 = fTemp77 + fTemp600;
			let mut fTemp615: F64 = 65535.0 * (1.0 - fTemp614);
			let mut iTemp616: i32 = (fTemp615) as i32;
			let mut iTemp617: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp616, 65535)))), 196607));
			let mut fTemp618: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp617, 3)) as usize] };
			let mut fTemp619: F64 = unsafe { ftbl0mydspSIG0[iTemp617 as usize] };
			let mut fTemp620: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp617, 1)) as usize] } - fTemp619;
			let mut fTemp621: F64 = 65535.0 * fTemp614;
			let mut iTemp622: i32 = (fTemp621) as i32;
			let mut iTemp623: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp622, 65535)))), 196607));
			let mut fTemp624: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp623, 3), 196607))) as usize] };
			let mut fTemp625: F64 = unsafe { ftbl0mydspSIG0[iTemp623 as usize] };
			let mut fTemp626: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp623, 1), 196607))) as usize] } - fTemp625;
			let mut fTemp627: F64 = fTemp4 + if ((0.001 * fTemp76) == 0.0) as i32 != 0 {fTemp57} else {fTemp57 * (if iTemp58 != 0 {fTemp625 + fTemp72 * fTemp626 + (fTemp621 - (iTemp622) as F64) * (fTemp624 - (fTemp625 + fTemp72 * (fTemp626 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp623, 4), 196607))) as usize] } - fTemp624))))} else {1.0 - (fTemp619 + fTemp72 * fTemp620 + (fTemp615 - (iTemp616) as F64) * (fTemp618 - (fTemp619 + fTemp72 * (fTemp620 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp617, 4)) as usize] } - fTemp618)))))} - fTemp613) / (1.0 - fTemp613)};
			self.fRec2[(self.IOTA0 & 8191) as usize] = if iTemp75 != 0 {F64::min(fTemp627, fTemp4)} else {F64::max(fTemp627, fTemp4)};
			let mut fTemp628: F64 = self.fRec2[((i32::wrapping_sub(self.IOTA0, iSlow70)) & 8191) as usize];
			self.fHbargraph0 = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, fTemp628));
			self.fRec14[0] = fSlow72 + self.fConst4 * self.fRec14[1];
			*output0 = 0.5 * self.fVec0[((i32::wrapping_sub(self.IOTA0, iSlow71)) & 32767) as usize] * fTemp2 + self.fRec14[0] * self.fVec1[((i32::wrapping_sub(self.IOTA0, iSlow71)) & 32767) as usize] * fTemp628 * fTemp3;
			let mut fTemp629: F64 = self.fRec16[((i32::wrapping_sub(self.IOTA0, 1)) & 8191) as usize];
			let mut fTemp630: F64 = fTemp31 + fSlow17 * (fTemp32 - fTemp31);
			let mut iTemp631: i32 = ((fTemp630 > fSlow11) as i32) + ((fTemp630 > fSlow9) as i32);
			let mut fTemp632: F64 = fTemp630 - fSlow8;
			let mut fTemp633: F64 = F64::min(fTemp29, F64::powf(1e+01, -(fSlow18 * F64::max(0.0, if (iTemp631 == 0) as i32 != 0 {0.0} else {if (iTemp631 == 1) as i32 != 0 {fSlow12 * mydsp_faustpower2_f(fSlow7 + fTemp632)} else {fTemp632}}))));
			self.fVec30[(self.IOTA0 & 16383) as usize] = fTemp633;
			let mut fTemp634: F64 = F64::min(fTemp633, self.fVec30[((i32::wrapping_sub(self.IOTA0, 1)) & 16383) as usize]);
			self.fVec31[0] = fTemp634;
			let mut fTemp635: F64 = F64::min(fTemp634, self.fVec31[2]);
			self.fVec32[0] = fTemp635;
			let mut fTemp636: F64 = F64::min(fTemp635, self.fVec32[4]);
			self.fVec33[0] = fTemp636;
			let mut fTemp637: F64 = F64::min(fTemp636, self.fVec33[8]);
			self.fVec34[(self.IOTA0 & 31) as usize] = fTemp637;
			let mut fTemp638: F64 = F64::min(fTemp637, self.fVec34[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize]);
			self.fVec35[(self.IOTA0 & 63) as usize] = fTemp638;
			let mut fTemp639: F64 = F64::min(fTemp638, self.fVec35[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize]);
			self.fVec36[(self.IOTA0 & 127) as usize] = fTemp639;
			let mut fTemp640: F64 = F64::min(fTemp639, self.fVec36[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize]);
			self.fVec37[(self.IOTA0 & 255) as usize] = fTemp640;
			let mut fTemp641: F64 = F64::min(fTemp640, self.fVec37[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize]);
			self.fVec38[(self.IOTA0 & 511) as usize] = fTemp641;
			let mut fTemp642: F64 = F64::min(fTemp641, self.fVec38[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize]);
			self.fVec39[(self.IOTA0 & 1023) as usize] = fTemp642;
			let mut fTemp643: F64 = F64::min(fTemp642, self.fVec39[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize]);
			self.fVec40[(self.IOTA0 & 2047) as usize] = fTemp643;
			self.fVec41[(self.IOTA0 & 4095) as usize] = F64::min(fTemp643, self.fVec40[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			self.fRec17[0] = F64::max(F64::min(self.fRec17[1], self.fVec30[((i32::wrapping_sub(self.IOTA0, iSlow21)) & 16383) as usize]), F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow23 != 0 {fTemp633} else {1.7976931348623157e+308}, if iSlow24 != 0 {self.fVec31[iSlow23 as usize]} else {1.7976931348623157e+308}), if iSlow25 != 0 {self.fVec32[iSlow26 as usize]} else {1.7976931348623157e+308}), if iSlow27 != 0 {self.fVec33[iSlow28 as usize]} else {1.7976931348623157e+308}), if iSlow29 != 0 {self.fVec34[((i32::wrapping_sub(self.IOTA0, iSlow30)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow31 != 0 {self.fVec35[((i32::wrapping_sub(self.IOTA0, iSlow32)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow33 != 0 {self.fVec36[((i32::wrapping_sub(self.IOTA0, iSlow34)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow35 != 0 {self.fVec37[((i32::wrapping_sub(self.IOTA0, iSlow36)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow37 != 0 {self.fVec38[((i32::wrapping_sub(self.IOTA0, iSlow38)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow39 != 0 {self.fVec39[((i32::wrapping_sub(self.IOTA0, iSlow40)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow41 != 0 {self.fVec40[((i32::wrapping_sub(self.IOTA0, iSlow42)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow43 != 0 {self.fVec41[((i32::wrapping_sub(self.IOTA0, iSlow44)) & 4095) as usize]} else {1.7976931348623157e+308}));
			let mut fTemp644: F64 = F64::min(self.fRec17[0], self.fRec17[1]);
			self.fVec42[0] = fTemp644;
			let mut fTemp645: F64 = F64::min(fTemp644, self.fVec42[2]);
			self.fVec43[0] = fTemp645;
			let mut fTemp646: F64 = F64::min(fTemp645, self.fVec43[4]);
			self.fVec44[0] = fTemp646;
			let mut fTemp647: F64 = F64::min(fTemp646, self.fVec44[8]);
			self.fVec45[(self.IOTA0 & 31) as usize] = fTemp647;
			let mut fTemp648: F64 = F64::min(fTemp647, self.fVec45[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize]);
			self.fVec46[(self.IOTA0 & 63) as usize] = fTemp648;
			let mut fTemp649: F64 = F64::min(fTemp648, self.fVec46[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize]);
			self.fVec47[(self.IOTA0 & 127) as usize] = fTemp649;
			let mut fTemp650: F64 = F64::min(fTemp649, self.fVec47[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize]);
			self.fVec48[(self.IOTA0 & 255) as usize] = fTemp650;
			let mut fTemp651: F64 = F64::min(fTemp650, self.fVec48[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize]);
			self.fVec49[(self.IOTA0 & 511) as usize] = fTemp651;
			let mut fTemp652: F64 = F64::min(fTemp651, self.fVec49[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize]);
			self.fVec50[(self.IOTA0 & 1023) as usize] = fTemp652;
			let mut fTemp653: F64 = F64::min(fTemp652, self.fVec50[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize]);
			self.fVec51[(self.IOTA0 & 2047) as usize] = fTemp653;
			self.fVec52[(self.IOTA0 & 4095) as usize] = F64::min(fTemp653, self.fVec51[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			let mut fTemp654: F64 = F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow4 != 0 {self.fRec17[0]} else {1.7976931348623157e+308}, if iSlow45 != 0 {self.fVec42[iSlow4 as usize]} else {1.7976931348623157e+308}), if iSlow46 != 0 {self.fVec43[iSlow47 as usize]} else {1.7976931348623157e+308}), if iSlow48 != 0 {self.fVec44[iSlow49 as usize]} else {1.7976931348623157e+308}), if iSlow50 != 0 {self.fVec45[((i32::wrapping_sub(self.IOTA0, iSlow51)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow52 != 0 {self.fVec46[((i32::wrapping_sub(self.IOTA0, iSlow53)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow54 != 0 {self.fVec47[((i32::wrapping_sub(self.IOTA0, iSlow55)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow56 != 0 {self.fVec48[((i32::wrapping_sub(self.IOTA0, iSlow57)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow58 != 0 {self.fVec49[((i32::wrapping_sub(self.IOTA0, iSlow59)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow60 != 0 {self.fVec50[((i32::wrapping_sub(self.IOTA0, iSlow61)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow62 != 0 {self.fVec51[((i32::wrapping_sub(self.IOTA0, iSlow63)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow64 != 0 {self.fVec52[((i32::wrapping_sub(self.IOTA0, iSlow65)) & 4095) as usize]} else {1.7976931348623157e+308}) - fTemp629;
			self.fVec53[0] = fTemp654;
			let mut iTemp655: i32 = (fTemp654 > 0.0) as i32;
			let mut fTemp656: F64 = if iTemp655 != 0 {fSlow67} else {fSlow66};
			self.fVec54[0] = fTemp656;
			let mut fTemp657: F64 = 2.0 * fTemp656;
			let mut iTemp658: i32 = (fTemp657) as i32;
			let mut iTemp659: i32 = std::cmp::max(0, std::cmp::min(iTemp658, 2));
			let mut iTemp660: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, 98301), 196607));
			let mut fTemp661: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp660, 3)) as usize] };
			let mut fTemp662: F64 = unsafe { ftbl0mydspSIG0[iTemp660 as usize] };
			let mut fTemp663: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp660, 1)) as usize] } - fTemp662;
			let mut fTemp664: F64 = fTemp657 - (iTemp658) as F64;
			let mut fTemp665: F64 = fTemp662 + fTemp664 * fTemp663 + 0.5 * (fTemp661 - (fTemp662 + fTemp664 * (fTemp663 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp660, 4)) as usize] } - fTemp661))));
			let mut fTemp666: F64 = if iTemp655 != 0 {fTemp665} else {1.0 - fTemp665};
			let mut iTemp667: i32 = (fTemp654 < 0.0) as i32;
			let mut fTemp668: F64 = fSlow1 * (iTemp667) as F64 + fSlow13 * (iTemp655) as F64;
			self.fVec55[0] = fTemp668;
			let mut fTemp669: F64 = self.fConst10 / fTemp668;
			let mut fTemp670: F64 = fTemp669 + 0.5;
			let mut fTemp671: F64 = 65535.0 * (1.0 - fTemp670);
			let mut iTemp672: i32 = (fTemp671) as i32;
			let mut iTemp673: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp672, 65535)))), 196607));
			let mut fTemp674: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp673, 3)) as usize] };
			let mut fTemp675: F64 = unsafe { ftbl0mydspSIG0[iTemp673 as usize] };
			let mut fTemp676: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp673, 1)) as usize] } - fTemp675;
			let mut fTemp677: F64 = 65535.0 * fTemp670;
			let mut iTemp678: i32 = (fTemp677) as i32;
			let mut iTemp679: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp678, 65535)))), 196607));
			let mut fTemp680: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp679, 3), 196607))) as usize] };
			let mut fTemp681: F64 = unsafe { ftbl0mydspSIG0[iTemp679 as usize] };
			let mut fTemp682: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp679, 1), 196607))) as usize] } - fTemp681;
			let mut fTemp683: F64 = 2.0 * self.fVec54[1];
			let mut iTemp684: i32 = (fTemp683) as i32;
			let mut iTemp685: i32 = std::cmp::max(0, std::cmp::min(iTemp684, 2));
			let mut fTemp686: F64 = 65535.0 * (1.0 - self.fRec15[1]);
			let mut iTemp687: i32 = (fTemp686) as i32;
			let mut iTemp688: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp687, 65535))), iTemp685), 196607));
			let mut fTemp689: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp688, 3), 196607))) as usize] };
			let mut fTemp690: F64 = unsafe { ftbl0mydspSIG0[iTemp688 as usize] };
			let mut fTemp691: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp688, 1), 196607))) as usize] } - fTemp690;
			let mut fTemp692: F64 = fTemp683 - (iTemp684) as F64;
			let mut fTemp693: F64 = 65535.0 * self.fRec15[1];
			let mut iTemp694: i32 = (fTemp693) as i32;
			let mut iTemp695: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp685, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp694, 65535)))), 196607));
			let mut fTemp696: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp695, 3), 196607))) as usize] };
			let mut fTemp697: F64 = unsafe { ftbl0mydspSIG0[iTemp695 as usize] };
			let mut fTemp698: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp695, 1), 196607))) as usize] } - fTemp697;
			let mut fTemp699: F64 = self.fRec15[1] + fTemp669;
			let mut fTemp700: F64 = 65535.0 * (1.0 - fTemp699);
			let mut iTemp701: i32 = (fTemp700) as i32;
			let mut iTemp702: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp701, 65535)))), 196607));
			let mut fTemp703: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp702, 3)) as usize] };
			let mut fTemp704: F64 = unsafe { ftbl0mydspSIG0[iTemp702 as usize] };
			let mut fTemp705: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp702, 1)) as usize] } - fTemp704;
			let mut fTemp706: F64 = 65535.0 * fTemp699;
			let mut iTemp707: i32 = (fTemp706) as i32;
			let mut iTemp708: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp707, 65535)))), 196607));
			let mut fTemp709: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp708, 3), 196607))) as usize] };
			let mut fTemp710: F64 = unsafe { ftbl0mydspSIG0[iTemp708 as usize] };
			let mut fTemp711: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp708, 1), 196607))) as usize] } - fTemp710;
			let mut fTemp712: F64 = self.fRec15[1] + self.fConst10 * (1.0 / fTemp668 + 1.0 / self.fVec55[1]);
			let mut fTemp713: F64 = 65535.0 * (1.0 - fTemp712);
			let mut iTemp714: i32 = (fTemp713) as i32;
			let mut iTemp715: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp714, 65535))), iTemp659), 196607));
			let mut fTemp716: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp715, 3)) as usize] };
			let mut fTemp717: F64 = unsafe { ftbl0mydspSIG0[iTemp715 as usize] };
			let mut fTemp718: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp715, 1)) as usize] } - fTemp717;
			let mut fTemp719: F64 = 65535.0 * fTemp712;
			let mut iTemp720: i32 = (fTemp719) as i32;
			let mut iTemp721: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp720, 65535)))), 196607));
			let mut fTemp722: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp721, 3), 196607))) as usize] };
			let mut fTemp723: F64 = unsafe { ftbl0mydspSIG0[iTemp721 as usize] };
			let mut fTemp724: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp721, 1), 196607))) as usize] } - fTemp723;
			let mut fTemp725: F64 = (if iTemp655 != 0 {fTemp723 + fTemp664 * fTemp724 + (fTemp719 - (iTemp720) as F64) * (fTemp722 - (fTemp723 + fTemp664 * (fTemp724 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp721, 4), 196607))) as usize] } - fTemp722))))} else {1.0 - (fTemp717 + fTemp664 * fTemp718 + (fTemp713 - (iTemp714) as F64) * (fTemp716 - (fTemp717 + fTemp664 * (fTemp718 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp715, 4)) as usize] } - fTemp716)))))} - if iTemp655 != 0 {fTemp710 + fTemp664 * fTemp711 + (fTemp706 - (iTemp707) as F64) * (fTemp709 - (fTemp710 + fTemp664 * (fTemp711 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp708, 4), 196607))) as usize] } - fTemp709))))} else {1.0 - (fTemp704 + fTemp664 * fTemp705 + (fTemp700 - (iTemp701) as F64) * (fTemp703 - (fTemp704 + fTemp664 * (fTemp705 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp702, 4), 196607))) as usize] } - fTemp703)))))}) * self.fVec53[1] / (fTemp654 * (1.0 - if iTemp655 != 0 {fTemp697 + fTemp692 * fTemp698 + (fTemp693 - (iTemp694) as F64) * (fTemp696 - (fTemp697 + fTemp692 * (fTemp698 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp695, 4), 196607))) as usize] } - fTemp696))))} else {1.0 - (fTemp690 + fTemp692 * fTemp691 + (fTemp686 - (iTemp687) as F64) * (fTemp689 - (fTemp690 + fTemp692 * (fTemp691 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp688, 4), 196607))) as usize] } - fTemp689)))))}));
			let mut iTemp726: i32 = (fTemp725 > ((if iTemp655 != 0 {fTemp681 + fTemp664 * fTemp682 + (fTemp677 - (iTemp678) as F64) * (fTemp680 - (fTemp681 + fTemp664 * (fTemp682 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp679, 4), 196607))) as usize] } - fTemp680))))} else {1.0 - (fTemp675 + fTemp664 * fTemp676 + (fTemp671 - (iTemp672) as F64) * (fTemp674 - (fTemp675 + fTemp664 * (fTemp676 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp673, 4)) as usize] } - fTemp674)))))} - fTemp666) / (1.0 - fTemp666))) as i32;
			let mut fTemp727: F64 = if iTemp726 != 0 {1.0} else {0.5};
			let mut fTemp728: F64 = if iTemp726 != 0 {0.5} else {0.0};
			let mut fTemp729: F64 = fTemp728 + fTemp727;
			let mut fTemp730: F64 = 0.5 * fTemp729;
			let mut fTemp731: F64 = 65535.0 * (1.0 - fTemp730);
			let mut iTemp732: i32 = (fTemp731) as i32;
			let mut iTemp733: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp732, 65535)))), 196607));
			let mut fTemp734: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp733, 3)) as usize] };
			let mut fTemp735: F64 = unsafe { ftbl0mydspSIG0[iTemp733 as usize] };
			let mut fTemp736: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp733, 1)) as usize] } - fTemp735;
			let mut fTemp737: F64 = 32767.5 * fTemp729;
			let mut iTemp738: i32 = (fTemp737) as i32;
			let mut iTemp739: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp738, 65535)))), 196607));
			let mut fTemp740: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp739, 3)) as usize] };
			let mut fTemp741: F64 = unsafe { ftbl0mydspSIG0[iTemp739 as usize] };
			let mut fTemp742: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp739, 1)) as usize] } - fTemp741;
			let mut fTemp743: F64 = if iTemp655 != 0 {fTemp741 + fTemp664 * fTemp742 + (fTemp737 - (iTemp738) as F64) * (fTemp740 - (fTemp741 + fTemp664 * (fTemp742 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp739, 4)) as usize] } - fTemp740))))} else {1.0 - (fTemp735 + fTemp664 * fTemp736 + (fTemp731 - (iTemp732) as F64) * (fTemp734 - (fTemp735 + fTemp664 * (fTemp736 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp733, 4)) as usize] } - fTemp734)))))};
			let mut fTemp744: F64 = fTemp669 + fTemp730;
			let mut fTemp745: F64 = 65535.0 * (1.0 - fTemp744);
			let mut iTemp746: i32 = (fTemp745) as i32;
			let mut iTemp747: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp746, 65535)))), 196607));
			let mut fTemp748: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp747, 3)) as usize] };
			let mut fTemp749: F64 = unsafe { ftbl0mydspSIG0[iTemp747 as usize] };
			let mut fTemp750: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp747, 1)) as usize] } - fTemp749;
			let mut fTemp751: F64 = 65535.0 * fTemp744;
			let mut iTemp752: i32 = (fTemp751) as i32;
			let mut iTemp753: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp752, 65535)))), 196607));
			let mut fTemp754: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp753, 3), 196607))) as usize] };
			let mut fTemp755: F64 = unsafe { ftbl0mydspSIG0[iTemp753 as usize] };
			let mut fTemp756: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp753, 1), 196607))) as usize] } - fTemp755;
			let mut iTemp757: i32 = (fTemp725 > ((if iTemp655 != 0 {fTemp755 + fTemp664 * fTemp756 + (fTemp751 - (iTemp752) as F64) * (fTemp754 - (fTemp755 + fTemp664 * (fTemp756 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp753, 4), 196607))) as usize] } - fTemp754))))} else {1.0 - (fTemp749 + fTemp664 * fTemp750 + (fTemp745 - (iTemp746) as F64) * (fTemp748 - (fTemp749 + fTemp664 * (fTemp750 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp747, 4)) as usize] } - fTemp748)))))} - fTemp743) / (1.0 - fTemp743))) as i32;
			let mut fTemp758: F64 = if iTemp757 != 0 {fTemp727} else {fTemp730};
			let mut fTemp759: F64 = if iTemp757 != 0 {fTemp730} else {fTemp728};
			let mut fTemp760: F64 = fTemp759 + fTemp758;
			let mut fTemp761: F64 = 0.5 * fTemp760;
			let mut fTemp762: F64 = 65535.0 * (1.0 - fTemp761);
			let mut iTemp763: i32 = (fTemp762) as i32;
			let mut iTemp764: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp763, 65535)))), 196607));
			let mut fTemp765: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp764, 3)) as usize] };
			let mut fTemp766: F64 = unsafe { ftbl0mydspSIG0[iTemp764 as usize] };
			let mut fTemp767: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp764, 1)) as usize] } - fTemp766;
			let mut fTemp768: F64 = 32767.5 * fTemp760;
			let mut iTemp769: i32 = (fTemp768) as i32;
			let mut iTemp770: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp769, 65535)))), 196607));
			let mut fTemp771: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp770, 3)) as usize] };
			let mut fTemp772: F64 = unsafe { ftbl0mydspSIG0[iTemp770 as usize] };
			let mut fTemp773: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp770, 1)) as usize] } - fTemp772;
			let mut fTemp774: F64 = if iTemp655 != 0 {fTemp772 + fTemp664 * fTemp773 + (fTemp768 - (iTemp769) as F64) * (fTemp771 - (fTemp772 + fTemp664 * (fTemp773 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp770, 4)) as usize] } - fTemp771))))} else {1.0 - (fTemp766 + fTemp664 * fTemp767 + (fTemp762 - (iTemp763) as F64) * (fTemp765 - (fTemp766 + fTemp664 * (fTemp767 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp764, 4)) as usize] } - fTemp765)))))};
			let mut fTemp775: F64 = fTemp669 + fTemp761;
			let mut fTemp776: F64 = 65535.0 * (1.0 - fTemp775);
			let mut iTemp777: i32 = (fTemp776) as i32;
			let mut iTemp778: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp777, 65535)))), 196607));
			let mut fTemp779: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp778, 3)) as usize] };
			let mut fTemp780: F64 = unsafe { ftbl0mydspSIG0[iTemp778 as usize] };
			let mut fTemp781: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp778, 1)) as usize] } - fTemp780;
			let mut fTemp782: F64 = 65535.0 * fTemp775;
			let mut iTemp783: i32 = (fTemp782) as i32;
			let mut iTemp784: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp783, 65535)))), 196607));
			let mut fTemp785: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp784, 3), 196607))) as usize] };
			let mut fTemp786: F64 = unsafe { ftbl0mydspSIG0[iTemp784 as usize] };
			let mut fTemp787: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp784, 1), 196607))) as usize] } - fTemp786;
			let mut iTemp788: i32 = (fTemp725 > ((if iTemp655 != 0 {fTemp786 + fTemp664 * fTemp787 + (fTemp782 - (iTemp783) as F64) * (fTemp785 - (fTemp786 + fTemp664 * (fTemp787 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp784, 4), 196607))) as usize] } - fTemp785))))} else {1.0 - (fTemp780 + fTemp664 * fTemp781 + (fTemp776 - (iTemp777) as F64) * (fTemp779 - (fTemp780 + fTemp664 * (fTemp781 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp778, 4)) as usize] } - fTemp779)))))} - fTemp774) / (1.0 - fTemp774))) as i32;
			let mut fTemp789: F64 = if iTemp788 != 0 {fTemp758} else {fTemp761};
			let mut fTemp790: F64 = if iTemp788 != 0 {fTemp761} else {fTemp759};
			let mut fTemp791: F64 = fTemp790 + fTemp789;
			let mut fTemp792: F64 = 0.5 * fTemp791;
			let mut fTemp793: F64 = 65535.0 * (1.0 - fTemp792);
			let mut iTemp794: i32 = (fTemp793) as i32;
			let mut iTemp795: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp794, 65535)))), 196607));
			let mut fTemp796: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp795, 3)) as usize] };
			let mut fTemp797: F64 = unsafe { ftbl0mydspSIG0[iTemp795 as usize] };
			let mut fTemp798: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp795, 1)) as usize] } - fTemp797;
			let mut fTemp799: F64 = 32767.5 * fTemp791;
			let mut iTemp800: i32 = (fTemp799) as i32;
			let mut iTemp801: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp800, 65535)))), 196607));
			let mut fTemp802: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp801, 3)) as usize] };
			let mut fTemp803: F64 = unsafe { ftbl0mydspSIG0[iTemp801 as usize] };
			let mut fTemp804: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp801, 1)) as usize] } - fTemp803;
			let mut fTemp805: F64 = if iTemp655 != 0 {fTemp803 + fTemp664 * fTemp804 + (fTemp799 - (iTemp800) as F64) * (fTemp802 - (fTemp803 + fTemp664 * (fTemp804 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp801, 4)) as usize] } - fTemp802))))} else {1.0 - (fTemp797 + fTemp664 * fTemp798 + (fTemp793 - (iTemp794) as F64) * (fTemp796 - (fTemp797 + fTemp664 * (fTemp798 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp795, 4)) as usize] } - fTemp796)))))};
			let mut fTemp806: F64 = fTemp669 + fTemp792;
			let mut fTemp807: F64 = 65535.0 * (1.0 - fTemp806);
			let mut iTemp808: i32 = (fTemp807) as i32;
			let mut iTemp809: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp808, 65535)))), 196607));
			let mut fTemp810: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp809, 3)) as usize] };
			let mut fTemp811: F64 = unsafe { ftbl0mydspSIG0[iTemp809 as usize] };
			let mut fTemp812: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp809, 1)) as usize] } - fTemp811;
			let mut fTemp813: F64 = 65535.0 * fTemp806;
			let mut iTemp814: i32 = (fTemp813) as i32;
			let mut iTemp815: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp814, 65535)))), 196607));
			let mut fTemp816: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp815, 3), 196607))) as usize] };
			let mut fTemp817: F64 = unsafe { ftbl0mydspSIG0[iTemp815 as usize] };
			let mut fTemp818: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp815, 1), 196607))) as usize] } - fTemp817;
			let mut iTemp819: i32 = (fTemp725 > ((if iTemp655 != 0 {fTemp817 + fTemp664 * fTemp818 + (fTemp813 - (iTemp814) as F64) * (fTemp816 - (fTemp817 + fTemp664 * (fTemp818 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp815, 4), 196607))) as usize] } - fTemp816))))} else {1.0 - (fTemp811 + fTemp664 * fTemp812 + (fTemp807 - (iTemp808) as F64) * (fTemp810 - (fTemp811 + fTemp664 * (fTemp812 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp809, 4)) as usize] } - fTemp810)))))} - fTemp805) / (1.0 - fTemp805))) as i32;
			let mut fTemp820: F64 = if iTemp819 != 0 {fTemp789} else {fTemp792};
			let mut fTemp821: F64 = if iTemp819 != 0 {fTemp792} else {fTemp790};
			let mut fTemp822: F64 = fTemp821 + fTemp820;
			let mut fTemp823: F64 = 0.5 * fTemp822;
			let mut fTemp824: F64 = 65535.0 * (1.0 - fTemp823);
			let mut iTemp825: i32 = (fTemp824) as i32;
			let mut iTemp826: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp825, 65535)))), 196607));
			let mut fTemp827: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp826, 3)) as usize] };
			let mut fTemp828: F64 = unsafe { ftbl0mydspSIG0[iTemp826 as usize] };
			let mut fTemp829: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp826, 1)) as usize] } - fTemp828;
			let mut fTemp830: F64 = 32767.5 * fTemp822;
			let mut iTemp831: i32 = (fTemp830) as i32;
			let mut iTemp832: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp831, 65535)))), 196607));
			let mut fTemp833: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp832, 3)) as usize] };
			let mut fTemp834: F64 = unsafe { ftbl0mydspSIG0[iTemp832 as usize] };
			let mut fTemp835: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp832, 1)) as usize] } - fTemp834;
			let mut fTemp836: F64 = if iTemp655 != 0 {fTemp834 + fTemp664 * fTemp835 + (fTemp830 - (iTemp831) as F64) * (fTemp833 - (fTemp834 + fTemp664 * (fTemp835 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp832, 4)) as usize] } - fTemp833))))} else {1.0 - (fTemp828 + fTemp664 * fTemp829 + (fTemp824 - (iTemp825) as F64) * (fTemp827 - (fTemp828 + fTemp664 * (fTemp829 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp826, 4)) as usize] } - fTemp827)))))};
			let mut fTemp837: F64 = fTemp669 + fTemp823;
			let mut fTemp838: F64 = 65535.0 * (1.0 - fTemp837);
			let mut iTemp839: i32 = (fTemp838) as i32;
			let mut iTemp840: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp839, 65535)))), 196607));
			let mut fTemp841: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp840, 3)) as usize] };
			let mut fTemp842: F64 = unsafe { ftbl0mydspSIG0[iTemp840 as usize] };
			let mut fTemp843: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp840, 1)) as usize] } - fTemp842;
			let mut fTemp844: F64 = 65535.0 * fTemp837;
			let mut iTemp845: i32 = (fTemp844) as i32;
			let mut iTemp846: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp845, 65535)))), 196607));
			let mut fTemp847: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp846, 3), 196607))) as usize] };
			let mut fTemp848: F64 = unsafe { ftbl0mydspSIG0[iTemp846 as usize] };
			let mut fTemp849: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp846, 1), 196607))) as usize] } - fTemp848;
			let mut iTemp850: i32 = (fTemp725 > ((if iTemp655 != 0 {fTemp848 + fTemp664 * fTemp849 + (fTemp844 - (iTemp845) as F64) * (fTemp847 - (fTemp848 + fTemp664 * (fTemp849 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp846, 4), 196607))) as usize] } - fTemp847))))} else {1.0 - (fTemp842 + fTemp664 * fTemp843 + (fTemp838 - (iTemp839) as F64) * (fTemp841 - (fTemp842 + fTemp664 * (fTemp843 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp840, 4)) as usize] } - fTemp841)))))} - fTemp836) / (1.0 - fTemp836))) as i32;
			let mut fTemp851: F64 = if iTemp850 != 0 {fTemp820} else {fTemp823};
			let mut fTemp852: F64 = if iTemp850 != 0 {fTemp823} else {fTemp821};
			let mut fTemp853: F64 = fTemp852 + fTemp851;
			let mut fTemp854: F64 = 0.5 * fTemp853;
			let mut fTemp855: F64 = 65535.0 * (1.0 - fTemp854);
			let mut iTemp856: i32 = (fTemp855) as i32;
			let mut iTemp857: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp856, 65535)))), 196607));
			let mut fTemp858: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp857, 3)) as usize] };
			let mut fTemp859: F64 = unsafe { ftbl0mydspSIG0[iTemp857 as usize] };
			let mut fTemp860: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp857, 1)) as usize] } - fTemp859;
			let mut fTemp861: F64 = 32767.5 * fTemp853;
			let mut iTemp862: i32 = (fTemp861) as i32;
			let mut iTemp863: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp862, 65535)))), 196607));
			let mut fTemp864: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp863, 3)) as usize] };
			let mut fTemp865: F64 = unsafe { ftbl0mydspSIG0[iTemp863 as usize] };
			let mut fTemp866: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp863, 1)) as usize] } - fTemp865;
			let mut fTemp867: F64 = if iTemp655 != 0 {fTemp865 + fTemp664 * fTemp866 + (fTemp861 - (iTemp862) as F64) * (fTemp864 - (fTemp865 + fTemp664 * (fTemp866 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp863, 4)) as usize] } - fTemp864))))} else {1.0 - (fTemp859 + fTemp664 * fTemp860 + (fTemp855 - (iTemp856) as F64) * (fTemp858 - (fTemp859 + fTemp664 * (fTemp860 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp857, 4)) as usize] } - fTemp858)))))};
			let mut fTemp868: F64 = fTemp669 + fTemp854;
			let mut fTemp869: F64 = 65535.0 * (1.0 - fTemp868);
			let mut iTemp870: i32 = (fTemp869) as i32;
			let mut iTemp871: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp870, 65535)))), 196607));
			let mut fTemp872: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp871, 3)) as usize] };
			let mut fTemp873: F64 = unsafe { ftbl0mydspSIG0[iTemp871 as usize] };
			let mut fTemp874: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp871, 1)) as usize] } - fTemp873;
			let mut fTemp875: F64 = 65535.0 * fTemp868;
			let mut iTemp876: i32 = (fTemp875) as i32;
			let mut iTemp877: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp876, 65535)))), 196607));
			let mut fTemp878: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp877, 3), 196607))) as usize] };
			let mut fTemp879: F64 = unsafe { ftbl0mydspSIG0[iTemp877 as usize] };
			let mut fTemp880: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp877, 1), 196607))) as usize] } - fTemp879;
			let mut iTemp881: i32 = (fTemp725 > ((if iTemp655 != 0 {fTemp879 + fTemp664 * fTemp880 + (fTemp875 - (iTemp876) as F64) * (fTemp878 - (fTemp879 + fTemp664 * (fTemp880 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp877, 4), 196607))) as usize] } - fTemp878))))} else {1.0 - (fTemp873 + fTemp664 * fTemp874 + (fTemp869 - (iTemp870) as F64) * (fTemp872 - (fTemp873 + fTemp664 * (fTemp874 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp871, 4)) as usize] } - fTemp872)))))} - fTemp867) / (1.0 - fTemp867))) as i32;
			let mut fTemp882: F64 = if iTemp881 != 0 {fTemp851} else {fTemp854};
			let mut fTemp883: F64 = if iTemp881 != 0 {fTemp854} else {fTemp852};
			let mut fTemp884: F64 = fTemp883 + fTemp882;
			let mut fTemp885: F64 = 0.5 * fTemp884;
			let mut fTemp886: F64 = 65535.0 * (1.0 - fTemp885);
			let mut iTemp887: i32 = (fTemp886) as i32;
			let mut iTemp888: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp887, 65535)))), 196607));
			let mut fTemp889: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp888, 3)) as usize] };
			let mut fTemp890: F64 = unsafe { ftbl0mydspSIG0[iTemp888 as usize] };
			let mut fTemp891: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp888, 1)) as usize] } - fTemp890;
			let mut fTemp892: F64 = 32767.5 * fTemp884;
			let mut iTemp893: i32 = (fTemp892) as i32;
			let mut iTemp894: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp893, 65535)))), 196607));
			let mut fTemp895: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp894, 3)) as usize] };
			let mut fTemp896: F64 = unsafe { ftbl0mydspSIG0[iTemp894 as usize] };
			let mut fTemp897: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp894, 1)) as usize] } - fTemp896;
			let mut fTemp898: F64 = if iTemp655 != 0 {fTemp896 + fTemp664 * fTemp897 + (fTemp892 - (iTemp893) as F64) * (fTemp895 - (fTemp896 + fTemp664 * (fTemp897 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp894, 4)) as usize] } - fTemp895))))} else {1.0 - (fTemp890 + fTemp664 * fTemp891 + (fTemp886 - (iTemp887) as F64) * (fTemp889 - (fTemp890 + fTemp664 * (fTemp891 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp888, 4)) as usize] } - fTemp889)))))};
			let mut fTemp899: F64 = fTemp669 + fTemp885;
			let mut fTemp900: F64 = 65535.0 * (1.0 - fTemp899);
			let mut iTemp901: i32 = (fTemp900) as i32;
			let mut iTemp902: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp901, 65535)))), 196607));
			let mut fTemp903: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp902, 3)) as usize] };
			let mut fTemp904: F64 = unsafe { ftbl0mydspSIG0[iTemp902 as usize] };
			let mut fTemp905: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp902, 1)) as usize] } - fTemp904;
			let mut fTemp906: F64 = 65535.0 * fTemp899;
			let mut iTemp907: i32 = (fTemp906) as i32;
			let mut iTemp908: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp907, 65535)))), 196607));
			let mut fTemp909: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp908, 3), 196607))) as usize] };
			let mut fTemp910: F64 = unsafe { ftbl0mydspSIG0[iTemp908 as usize] };
			let mut fTemp911: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp908, 1), 196607))) as usize] } - fTemp910;
			let mut iTemp912: i32 = (fTemp725 > ((if iTemp655 != 0 {fTemp910 + fTemp664 * fTemp911 + (fTemp906 - (iTemp907) as F64) * (fTemp909 - (fTemp910 + fTemp664 * (fTemp911 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp908, 4), 196607))) as usize] } - fTemp909))))} else {1.0 - (fTemp904 + fTemp664 * fTemp905 + (fTemp900 - (iTemp901) as F64) * (fTemp903 - (fTemp904 + fTemp664 * (fTemp905 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp902, 4)) as usize] } - fTemp903)))))} - fTemp898) / (1.0 - fTemp898))) as i32;
			let mut fTemp913: F64 = if iTemp912 != 0 {fTemp882} else {fTemp885};
			let mut fTemp914: F64 = if iTemp912 != 0 {fTemp885} else {fTemp883};
			let mut fTemp915: F64 = fTemp914 + fTemp913;
			let mut fTemp916: F64 = 0.5 * fTemp915;
			let mut fTemp917: F64 = 65535.0 * (1.0 - fTemp916);
			let mut iTemp918: i32 = (fTemp917) as i32;
			let mut iTemp919: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp918, 65535)))), 196607));
			let mut fTemp920: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp919, 3)) as usize] };
			let mut fTemp921: F64 = unsafe { ftbl0mydspSIG0[iTemp919 as usize] };
			let mut fTemp922: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp919, 1)) as usize] } - fTemp921;
			let mut fTemp923: F64 = 32767.5 * fTemp915;
			let mut iTemp924: i32 = (fTemp923) as i32;
			let mut iTemp925: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp924, 65535)))), 196607));
			let mut fTemp926: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp925, 3)) as usize] };
			let mut fTemp927: F64 = unsafe { ftbl0mydspSIG0[iTemp925 as usize] };
			let mut fTemp928: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp925, 1)) as usize] } - fTemp927;
			let mut fTemp929: F64 = if iTemp655 != 0 {fTemp927 + fTemp664 * fTemp928 + (fTemp923 - (iTemp924) as F64) * (fTemp926 - (fTemp927 + fTemp664 * (fTemp928 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp925, 4)) as usize] } - fTemp926))))} else {1.0 - (fTemp921 + fTemp664 * fTemp922 + (fTemp917 - (iTemp918) as F64) * (fTemp920 - (fTemp921 + fTemp664 * (fTemp922 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp919, 4)) as usize] } - fTemp920)))))};
			let mut fTemp930: F64 = fTemp669 + fTemp916;
			let mut fTemp931: F64 = 65535.0 * (1.0 - fTemp930);
			let mut iTemp932: i32 = (fTemp931) as i32;
			let mut iTemp933: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp932, 65535)))), 196607));
			let mut fTemp934: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp933, 3)) as usize] };
			let mut fTemp935: F64 = unsafe { ftbl0mydspSIG0[iTemp933 as usize] };
			let mut fTemp936: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp933, 1)) as usize] } - fTemp935;
			let mut fTemp937: F64 = 65535.0 * fTemp930;
			let mut iTemp938: i32 = (fTemp937) as i32;
			let mut iTemp939: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp938, 65535)))), 196607));
			let mut fTemp940: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp939, 3), 196607))) as usize] };
			let mut fTemp941: F64 = unsafe { ftbl0mydspSIG0[iTemp939 as usize] };
			let mut fTemp942: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp939, 1), 196607))) as usize] } - fTemp941;
			let mut iTemp943: i32 = (fTemp725 > ((if iTemp655 != 0 {fTemp941 + fTemp664 * fTemp942 + (fTemp937 - (iTemp938) as F64) * (fTemp940 - (fTemp941 + fTemp664 * (fTemp942 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp939, 4), 196607))) as usize] } - fTemp940))))} else {1.0 - (fTemp935 + fTemp664 * fTemp936 + (fTemp931 - (iTemp932) as F64) * (fTemp934 - (fTemp935 + fTemp664 * (fTemp936 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp933, 4)) as usize] } - fTemp934)))))} - fTemp929) / (1.0 - fTemp929))) as i32;
			let mut fTemp944: F64 = if iTemp943 != 0 {fTemp913} else {fTemp916};
			let mut fTemp945: F64 = if iTemp943 != 0 {fTemp916} else {fTemp914};
			let mut fTemp946: F64 = fTemp945 + fTemp944;
			let mut fTemp947: F64 = 0.5 * fTemp946;
			let mut fTemp948: F64 = 65535.0 * (1.0 - fTemp947);
			let mut iTemp949: i32 = (fTemp948) as i32;
			let mut iTemp950: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp949, 65535)))), 196607));
			let mut fTemp951: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp950, 3)) as usize] };
			let mut fTemp952: F64 = unsafe { ftbl0mydspSIG0[iTemp950 as usize] };
			let mut fTemp953: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp950, 1)) as usize] } - fTemp952;
			let mut fTemp954: F64 = 32767.5 * fTemp946;
			let mut iTemp955: i32 = (fTemp954) as i32;
			let mut iTemp956: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp955, 65535)))), 196607));
			let mut fTemp957: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp956, 3)) as usize] };
			let mut fTemp958: F64 = unsafe { ftbl0mydspSIG0[iTemp956 as usize] };
			let mut fTemp959: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp956, 1)) as usize] } - fTemp958;
			let mut fTemp960: F64 = if iTemp655 != 0 {fTemp958 + fTemp664 * fTemp959 + (fTemp954 - (iTemp955) as F64) * (fTemp957 - (fTemp958 + fTemp664 * (fTemp959 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp956, 4)) as usize] } - fTemp957))))} else {1.0 - (fTemp952 + fTemp664 * fTemp953 + (fTemp948 - (iTemp949) as F64) * (fTemp951 - (fTemp952 + fTemp664 * (fTemp953 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp950, 4)) as usize] } - fTemp951)))))};
			let mut fTemp961: F64 = fTemp669 + fTemp947;
			let mut fTemp962: F64 = 65535.0 * (1.0 - fTemp961);
			let mut iTemp963: i32 = (fTemp962) as i32;
			let mut iTemp964: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp963, 65535)))), 196607));
			let mut fTemp965: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp964, 3)) as usize] };
			let mut fTemp966: F64 = unsafe { ftbl0mydspSIG0[iTemp964 as usize] };
			let mut fTemp967: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp964, 1)) as usize] } - fTemp966;
			let mut fTemp968: F64 = 65535.0 * fTemp961;
			let mut iTemp969: i32 = (fTemp968) as i32;
			let mut iTemp970: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp969, 65535)))), 196607));
			let mut fTemp971: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp970, 3), 196607))) as usize] };
			let mut fTemp972: F64 = unsafe { ftbl0mydspSIG0[iTemp970 as usize] };
			let mut fTemp973: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp970, 1), 196607))) as usize] } - fTemp972;
			let mut iTemp974: i32 = (fTemp725 > ((if iTemp655 != 0 {fTemp972 + fTemp664 * fTemp973 + (fTemp968 - (iTemp969) as F64) * (fTemp971 - (fTemp972 + fTemp664 * (fTemp973 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp970, 4), 196607))) as usize] } - fTemp971))))} else {1.0 - (fTemp966 + fTemp664 * fTemp967 + (fTemp962 - (iTemp963) as F64) * (fTemp965 - (fTemp966 + fTemp664 * (fTemp967 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp964, 4)) as usize] } - fTemp965)))))} - fTemp960) / (1.0 - fTemp960))) as i32;
			let mut fTemp975: F64 = if iTemp974 != 0 {fTemp944} else {fTemp947};
			let mut fTemp976: F64 = if iTemp974 != 0 {fTemp947} else {fTemp945};
			let mut fTemp977: F64 = fTemp976 + fTemp975;
			let mut fTemp978: F64 = 0.5 * fTemp977;
			let mut fTemp979: F64 = 65535.0 * (1.0 - fTemp978);
			let mut iTemp980: i32 = (fTemp979) as i32;
			let mut iTemp981: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp980, 65535)))), 196607));
			let mut fTemp982: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp981, 3)) as usize] };
			let mut fTemp983: F64 = unsafe { ftbl0mydspSIG0[iTemp981 as usize] };
			let mut fTemp984: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp981, 1)) as usize] } - fTemp983;
			let mut fTemp985: F64 = 32767.5 * fTemp977;
			let mut iTemp986: i32 = (fTemp985) as i32;
			let mut iTemp987: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp986, 65535)))), 196607));
			let mut fTemp988: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp987, 3)) as usize] };
			let mut fTemp989: F64 = unsafe { ftbl0mydspSIG0[iTemp987 as usize] };
			let mut fTemp990: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp987, 1)) as usize] } - fTemp989;
			let mut fTemp991: F64 = if iTemp655 != 0 {fTemp989 + fTemp664 * fTemp990 + (fTemp985 - (iTemp986) as F64) * (fTemp988 - (fTemp989 + fTemp664 * (fTemp990 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp987, 4)) as usize] } - fTemp988))))} else {1.0 - (fTemp983 + fTemp664 * fTemp984 + (fTemp979 - (iTemp980) as F64) * (fTemp982 - (fTemp983 + fTemp664 * (fTemp984 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp981, 4)) as usize] } - fTemp982)))))};
			let mut fTemp992: F64 = fTemp669 + fTemp978;
			let mut fTemp993: F64 = 65535.0 * (1.0 - fTemp992);
			let mut iTemp994: i32 = (fTemp993) as i32;
			let mut iTemp995: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp994, 65535)))), 196607));
			let mut fTemp996: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp995, 3)) as usize] };
			let mut fTemp997: F64 = unsafe { ftbl0mydspSIG0[iTemp995 as usize] };
			let mut fTemp998: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp995, 1)) as usize] } - fTemp997;
			let mut fTemp999: F64 = 65535.0 * fTemp992;
			let mut iTemp1000: i32 = (fTemp999) as i32;
			let mut iTemp1001: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1000, 65535)))), 196607));
			let mut fTemp1002: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1001, 3), 196607))) as usize] };
			let mut fTemp1003: F64 = unsafe { ftbl0mydspSIG0[iTemp1001 as usize] };
			let mut fTemp1004: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1001, 1), 196607))) as usize] } - fTemp1003;
			let mut iTemp1005: i32 = (fTemp725 > ((if iTemp655 != 0 {fTemp1003 + fTemp664 * fTemp1004 + (fTemp999 - (iTemp1000) as F64) * (fTemp1002 - (fTemp1003 + fTemp664 * (fTemp1004 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1001, 4), 196607))) as usize] } - fTemp1002))))} else {1.0 - (fTemp997 + fTemp664 * fTemp998 + (fTemp993 - (iTemp994) as F64) * (fTemp996 - (fTemp997 + fTemp664 * (fTemp998 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp995, 4)) as usize] } - fTemp996)))))} - fTemp991) / (1.0 - fTemp991))) as i32;
			let mut fTemp1006: F64 = if iTemp1005 != 0 {fTemp975} else {fTemp978};
			let mut fTemp1007: F64 = if iTemp1005 != 0 {fTemp978} else {fTemp976};
			let mut fTemp1008: F64 = fTemp1007 + fTemp1006;
			let mut fTemp1009: F64 = 0.5 * fTemp1008;
			let mut fTemp1010: F64 = 65535.0 * (1.0 - fTemp1009);
			let mut iTemp1011: i32 = (fTemp1010) as i32;
			let mut iTemp1012: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1011, 65535)))), 196607));
			let mut fTemp1013: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1012, 3)) as usize] };
			let mut fTemp1014: F64 = unsafe { ftbl0mydspSIG0[iTemp1012 as usize] };
			let mut fTemp1015: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1012, 1)) as usize] } - fTemp1014;
			let mut fTemp1016: F64 = 32767.5 * fTemp1008;
			let mut iTemp1017: i32 = (fTemp1016) as i32;
			let mut iTemp1018: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1017, 65535)))), 196607));
			let mut fTemp1019: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1018, 3)) as usize] };
			let mut fTemp1020: F64 = unsafe { ftbl0mydspSIG0[iTemp1018 as usize] };
			let mut fTemp1021: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1018, 1)) as usize] } - fTemp1020;
			let mut fTemp1022: F64 = if iTemp655 != 0 {fTemp1020 + fTemp664 * fTemp1021 + (fTemp1016 - (iTemp1017) as F64) * (fTemp1019 - (fTemp1020 + fTemp664 * (fTemp1021 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1018, 4)) as usize] } - fTemp1019))))} else {1.0 - (fTemp1014 + fTemp664 * fTemp1015 + (fTemp1010 - (iTemp1011) as F64) * (fTemp1013 - (fTemp1014 + fTemp664 * (fTemp1015 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1012, 4)) as usize] } - fTemp1013)))))};
			let mut fTemp1023: F64 = fTemp669 + fTemp1009;
			let mut fTemp1024: F64 = 65535.0 * (1.0 - fTemp1023);
			let mut iTemp1025: i32 = (fTemp1024) as i32;
			let mut iTemp1026: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1025, 65535)))), 196607));
			let mut fTemp1027: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1026, 3)) as usize] };
			let mut fTemp1028: F64 = unsafe { ftbl0mydspSIG0[iTemp1026 as usize] };
			let mut fTemp1029: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1026, 1)) as usize] } - fTemp1028;
			let mut fTemp1030: F64 = 65535.0 * fTemp1023;
			let mut iTemp1031: i32 = (fTemp1030) as i32;
			let mut iTemp1032: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1031, 65535)))), 196607));
			let mut fTemp1033: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1032, 3), 196607))) as usize] };
			let mut fTemp1034: F64 = unsafe { ftbl0mydspSIG0[iTemp1032 as usize] };
			let mut fTemp1035: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1032, 1), 196607))) as usize] } - fTemp1034;
			let mut iTemp1036: i32 = (fTemp725 > ((if iTemp655 != 0 {fTemp1034 + fTemp664 * fTemp1035 + (fTemp1030 - (iTemp1031) as F64) * (fTemp1033 - (fTemp1034 + fTemp664 * (fTemp1035 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1032, 4), 196607))) as usize] } - fTemp1033))))} else {1.0 - (fTemp1028 + fTemp664 * fTemp1029 + (fTemp1024 - (iTemp1025) as F64) * (fTemp1027 - (fTemp1028 + fTemp664 * (fTemp1029 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1026, 4)) as usize] } - fTemp1027)))))} - fTemp1022) / (1.0 - fTemp1022))) as i32;
			let mut fTemp1037: F64 = if iTemp1036 != 0 {fTemp1006} else {fTemp1009};
			let mut fTemp1038: F64 = if iTemp1036 != 0 {fTemp1009} else {fTemp1007};
			let mut fTemp1039: F64 = fTemp1038 + fTemp1037;
			let mut fTemp1040: F64 = 0.5 * fTemp1039;
			let mut fTemp1041: F64 = 65535.0 * (1.0 - fTemp1040);
			let mut iTemp1042: i32 = (fTemp1041) as i32;
			let mut iTemp1043: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1042, 65535)))), 196607));
			let mut fTemp1044: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1043, 3)) as usize] };
			let mut fTemp1045: F64 = unsafe { ftbl0mydspSIG0[iTemp1043 as usize] };
			let mut fTemp1046: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1043, 1)) as usize] } - fTemp1045;
			let mut fTemp1047: F64 = 32767.5 * fTemp1039;
			let mut iTemp1048: i32 = (fTemp1047) as i32;
			let mut iTemp1049: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1048, 65535)))), 196607));
			let mut fTemp1050: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1049, 3)) as usize] };
			let mut fTemp1051: F64 = unsafe { ftbl0mydspSIG0[iTemp1049 as usize] };
			let mut fTemp1052: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1049, 1)) as usize] } - fTemp1051;
			let mut fTemp1053: F64 = if iTemp655 != 0 {fTemp1051 + fTemp664 * fTemp1052 + (fTemp1047 - (iTemp1048) as F64) * (fTemp1050 - (fTemp1051 + fTemp664 * (fTemp1052 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1049, 4)) as usize] } - fTemp1050))))} else {1.0 - (fTemp1045 + fTemp664 * fTemp1046 + (fTemp1041 - (iTemp1042) as F64) * (fTemp1044 - (fTemp1045 + fTemp664 * (fTemp1046 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1043, 4)) as usize] } - fTemp1044)))))};
			let mut fTemp1054: F64 = fTemp669 + fTemp1040;
			let mut fTemp1055: F64 = 65535.0 * (1.0 - fTemp1054);
			let mut iTemp1056: i32 = (fTemp1055) as i32;
			let mut iTemp1057: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1056, 65535)))), 196607));
			let mut fTemp1058: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1057, 3)) as usize] };
			let mut fTemp1059: F64 = unsafe { ftbl0mydspSIG0[iTemp1057 as usize] };
			let mut fTemp1060: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1057, 1)) as usize] } - fTemp1059;
			let mut fTemp1061: F64 = 65535.0 * fTemp1054;
			let mut iTemp1062: i32 = (fTemp1061) as i32;
			let mut iTemp1063: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1062, 65535)))), 196607));
			let mut fTemp1064: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1063, 3), 196607))) as usize] };
			let mut fTemp1065: F64 = unsafe { ftbl0mydspSIG0[iTemp1063 as usize] };
			let mut fTemp1066: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1063, 1), 196607))) as usize] } - fTemp1065;
			let mut iTemp1067: i32 = (fTemp725 > ((if iTemp655 != 0 {fTemp1065 + fTemp664 * fTemp1066 + (fTemp1061 - (iTemp1062) as F64) * (fTemp1064 - (fTemp1065 + fTemp664 * (fTemp1066 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1063, 4), 196607))) as usize] } - fTemp1064))))} else {1.0 - (fTemp1059 + fTemp664 * fTemp1060 + (fTemp1055 - (iTemp1056) as F64) * (fTemp1058 - (fTemp1059 + fTemp664 * (fTemp1060 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1057, 4)) as usize] } - fTemp1058)))))} - fTemp1053) / (1.0 - fTemp1053))) as i32;
			let mut fTemp1068: F64 = if iTemp1067 != 0 {fTemp1037} else {fTemp1040};
			let mut fTemp1069: F64 = if iTemp1067 != 0 {fTemp1040} else {fTemp1038};
			let mut fTemp1070: F64 = fTemp1069 + fTemp1068;
			let mut fTemp1071: F64 = 0.5 * fTemp1070;
			let mut fTemp1072: F64 = 65535.0 * (1.0 - fTemp1071);
			let mut iTemp1073: i32 = (fTemp1072) as i32;
			let mut iTemp1074: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1073, 65535)))), 196607));
			let mut fTemp1075: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1074, 3)) as usize] };
			let mut fTemp1076: F64 = unsafe { ftbl0mydspSIG0[iTemp1074 as usize] };
			let mut fTemp1077: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1074, 1)) as usize] } - fTemp1076;
			let mut fTemp1078: F64 = 32767.5 * fTemp1070;
			let mut iTemp1079: i32 = (fTemp1078) as i32;
			let mut iTemp1080: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1079, 65535)))), 196607));
			let mut fTemp1081: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1080, 3)) as usize] };
			let mut fTemp1082: F64 = unsafe { ftbl0mydspSIG0[iTemp1080 as usize] };
			let mut fTemp1083: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1080, 1)) as usize] } - fTemp1082;
			let mut fTemp1084: F64 = if iTemp655 != 0 {fTemp1082 + fTemp664 * fTemp1083 + (fTemp1078 - (iTemp1079) as F64) * (fTemp1081 - (fTemp1082 + fTemp664 * (fTemp1083 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1080, 4)) as usize] } - fTemp1081))))} else {1.0 - (fTemp1076 + fTemp664 * fTemp1077 + (fTemp1072 - (iTemp1073) as F64) * (fTemp1075 - (fTemp1076 + fTemp664 * (fTemp1077 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1074, 4)) as usize] } - fTemp1075)))))};
			let mut fTemp1085: F64 = fTemp669 + fTemp1071;
			let mut fTemp1086: F64 = 65535.0 * (1.0 - fTemp1085);
			let mut iTemp1087: i32 = (fTemp1086) as i32;
			let mut iTemp1088: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1087, 65535)))), 196607));
			let mut fTemp1089: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1088, 3)) as usize] };
			let mut fTemp1090: F64 = unsafe { ftbl0mydspSIG0[iTemp1088 as usize] };
			let mut fTemp1091: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1088, 1)) as usize] } - fTemp1090;
			let mut fTemp1092: F64 = 65535.0 * fTemp1085;
			let mut iTemp1093: i32 = (fTemp1092) as i32;
			let mut iTemp1094: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1093, 65535)))), 196607));
			let mut fTemp1095: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1094, 3), 196607))) as usize] };
			let mut fTemp1096: F64 = unsafe { ftbl0mydspSIG0[iTemp1094 as usize] };
			let mut fTemp1097: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1094, 1), 196607))) as usize] } - fTemp1096;
			let mut iTemp1098: i32 = (fTemp725 > ((if iTemp655 != 0 {fTemp1096 + fTemp664 * fTemp1097 + (fTemp1092 - (iTemp1093) as F64) * (fTemp1095 - (fTemp1096 + fTemp664 * (fTemp1097 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1094, 4), 196607))) as usize] } - fTemp1095))))} else {1.0 - (fTemp1090 + fTemp664 * fTemp1091 + (fTemp1086 - (iTemp1087) as F64) * (fTemp1089 - (fTemp1090 + fTemp664 * (fTemp1091 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1088, 4)) as usize] } - fTemp1089)))))} - fTemp1084) / (1.0 - fTemp1084))) as i32;
			let mut fTemp1099: F64 = if iTemp1098 != 0 {fTemp1068} else {fTemp1071};
			let mut fTemp1100: F64 = if iTemp1098 != 0 {fTemp1071} else {fTemp1069};
			let mut fTemp1101: F64 = fTemp1100 + fTemp1099;
			let mut fTemp1102: F64 = 0.5 * fTemp1101;
			let mut fTemp1103: F64 = 65535.0 * (1.0 - fTemp1102);
			let mut iTemp1104: i32 = (fTemp1103) as i32;
			let mut iTemp1105: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1104, 65535)))), 196607));
			let mut fTemp1106: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1105, 3)) as usize] };
			let mut fTemp1107: F64 = unsafe { ftbl0mydspSIG0[iTemp1105 as usize] };
			let mut fTemp1108: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1105, 1)) as usize] } - fTemp1107;
			let mut fTemp1109: F64 = 32767.5 * fTemp1101;
			let mut iTemp1110: i32 = (fTemp1109) as i32;
			let mut iTemp1111: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1110, 65535)))), 196607));
			let mut fTemp1112: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1111, 3)) as usize] };
			let mut fTemp1113: F64 = unsafe { ftbl0mydspSIG0[iTemp1111 as usize] };
			let mut fTemp1114: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1111, 1)) as usize] } - fTemp1113;
			let mut fTemp1115: F64 = if iTemp655 != 0 {fTemp1113 + fTemp664 * fTemp1114 + (fTemp1109 - (iTemp1110) as F64) * (fTemp1112 - (fTemp1113 + fTemp664 * (fTemp1114 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1111, 4)) as usize] } - fTemp1112))))} else {1.0 - (fTemp1107 + fTemp664 * fTemp1108 + (fTemp1103 - (iTemp1104) as F64) * (fTemp1106 - (fTemp1107 + fTemp664 * (fTemp1108 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1105, 4)) as usize] } - fTemp1106)))))};
			let mut fTemp1116: F64 = fTemp669 + fTemp1102;
			let mut fTemp1117: F64 = 65535.0 * (1.0 - fTemp1116);
			let mut iTemp1118: i32 = (fTemp1117) as i32;
			let mut iTemp1119: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1118, 65535)))), 196607));
			let mut fTemp1120: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1119, 3)) as usize] };
			let mut fTemp1121: F64 = unsafe { ftbl0mydspSIG0[iTemp1119 as usize] };
			let mut fTemp1122: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1119, 1)) as usize] } - fTemp1121;
			let mut fTemp1123: F64 = 65535.0 * fTemp1116;
			let mut iTemp1124: i32 = (fTemp1123) as i32;
			let mut iTemp1125: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1124, 65535)))), 196607));
			let mut fTemp1126: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1125, 3), 196607))) as usize] };
			let mut fTemp1127: F64 = unsafe { ftbl0mydspSIG0[iTemp1125 as usize] };
			let mut fTemp1128: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1125, 1), 196607))) as usize] } - fTemp1127;
			let mut iTemp1129: i32 = (fTemp725 > ((if iTemp655 != 0 {fTemp1127 + fTemp664 * fTemp1128 + (fTemp1123 - (iTemp1124) as F64) * (fTemp1126 - (fTemp1127 + fTemp664 * (fTemp1128 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1125, 4), 196607))) as usize] } - fTemp1126))))} else {1.0 - (fTemp1121 + fTemp664 * fTemp1122 + (fTemp1117 - (iTemp1118) as F64) * (fTemp1120 - (fTemp1121 + fTemp664 * (fTemp1122 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1119, 4)) as usize] } - fTemp1120)))))} - fTemp1115) / (1.0 - fTemp1115))) as i32;
			let mut fTemp1130: F64 = if iTemp1129 != 0 {fTemp1099} else {fTemp1102};
			let mut fTemp1131: F64 = if iTemp1129 != 0 {fTemp1102} else {fTemp1100};
			let mut fTemp1132: F64 = fTemp1131 + fTemp1130;
			let mut fTemp1133: F64 = 0.5 * fTemp1132;
			let mut fTemp1134: F64 = 65535.0 * (1.0 - fTemp1133);
			let mut iTemp1135: i32 = (fTemp1134) as i32;
			let mut iTemp1136: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1135, 65535)))), 196607));
			let mut fTemp1137: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1136, 3)) as usize] };
			let mut fTemp1138: F64 = unsafe { ftbl0mydspSIG0[iTemp1136 as usize] };
			let mut fTemp1139: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1136, 1)) as usize] } - fTemp1138;
			let mut fTemp1140: F64 = 32767.5 * fTemp1132;
			let mut iTemp1141: i32 = (fTemp1140) as i32;
			let mut iTemp1142: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1141, 65535)))), 196607));
			let mut fTemp1143: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1142, 3)) as usize] };
			let mut fTemp1144: F64 = unsafe { ftbl0mydspSIG0[iTemp1142 as usize] };
			let mut fTemp1145: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1142, 1)) as usize] } - fTemp1144;
			let mut fTemp1146: F64 = if iTemp655 != 0 {fTemp1144 + fTemp664 * fTemp1145 + (fTemp1140 - (iTemp1141) as F64) * (fTemp1143 - (fTemp1144 + fTemp664 * (fTemp1145 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1142, 4)) as usize] } - fTemp1143))))} else {1.0 - (fTemp1138 + fTemp664 * fTemp1139 + (fTemp1134 - (iTemp1135) as F64) * (fTemp1137 - (fTemp1138 + fTemp664 * (fTemp1139 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1136, 4)) as usize] } - fTemp1137)))))};
			let mut fTemp1147: F64 = fTemp669 + fTemp1133;
			let mut fTemp1148: F64 = 65535.0 * (1.0 - fTemp1147);
			let mut iTemp1149: i32 = (fTemp1148) as i32;
			let mut iTemp1150: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1149, 65535)))), 196607));
			let mut fTemp1151: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1150, 3)) as usize] };
			let mut fTemp1152: F64 = unsafe { ftbl0mydspSIG0[iTemp1150 as usize] };
			let mut fTemp1153: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1150, 1)) as usize] } - fTemp1152;
			let mut fTemp1154: F64 = 65535.0 * fTemp1147;
			let mut iTemp1155: i32 = (fTemp1154) as i32;
			let mut iTemp1156: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1155, 65535)))), 196607));
			let mut fTemp1157: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1156, 3), 196607))) as usize] };
			let mut fTemp1158: F64 = unsafe { ftbl0mydspSIG0[iTemp1156 as usize] };
			let mut fTemp1159: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1156, 1), 196607))) as usize] } - fTemp1158;
			let mut iTemp1160: i32 = (fTemp725 > ((if iTemp655 != 0 {fTemp1158 + fTemp664 * fTemp1159 + (fTemp1154 - (iTemp1155) as F64) * (fTemp1157 - (fTemp1158 + fTemp664 * (fTemp1159 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1156, 4), 196607))) as usize] } - fTemp1157))))} else {1.0 - (fTemp1152 + fTemp664 * fTemp1153 + (fTemp1148 - (iTemp1149) as F64) * (fTemp1151 - (fTemp1152 + fTemp664 * (fTemp1153 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1150, 4)) as usize] } - fTemp1151)))))} - fTemp1146) / (1.0 - fTemp1146))) as i32;
			let mut fTemp1161: F64 = if iTemp1160 != 0 {fTemp1130} else {fTemp1133};
			let mut fTemp1162: F64 = if iTemp1160 != 0 {fTemp1133} else {fTemp1131};
			let mut fTemp1163: F64 = fTemp1162 + fTemp1161;
			let mut fTemp1164: F64 = 0.5 * fTemp1163;
			let mut fTemp1165: F64 = 65535.0 * (1.0 - fTemp1164);
			let mut iTemp1166: i32 = (fTemp1165) as i32;
			let mut iTemp1167: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1166, 65535)))), 196607));
			let mut fTemp1168: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1167, 3)) as usize] };
			let mut fTemp1169: F64 = unsafe { ftbl0mydspSIG0[iTemp1167 as usize] };
			let mut fTemp1170: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1167, 1)) as usize] } - fTemp1169;
			let mut fTemp1171: F64 = 32767.5 * fTemp1163;
			let mut iTemp1172: i32 = (fTemp1171) as i32;
			let mut iTemp1173: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1172, 65535)))), 196607));
			let mut fTemp1174: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1173, 3)) as usize] };
			let mut fTemp1175: F64 = unsafe { ftbl0mydspSIG0[iTemp1173 as usize] };
			let mut fTemp1176: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1173, 1)) as usize] } - fTemp1175;
			let mut fTemp1177: F64 = if iTemp655 != 0 {fTemp1175 + fTemp664 * fTemp1176 + (fTemp1171 - (iTemp1172) as F64) * (fTemp1174 - (fTemp1175 + fTemp664 * (fTemp1176 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1173, 4), 196607))) as usize] } - fTemp1174))))} else {1.0 - (fTemp1169 + fTemp664 * fTemp1170 + (fTemp1165 - (iTemp1166) as F64) * (fTemp1168 - (fTemp1169 + fTemp664 * (fTemp1170 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1167, 4), 196607))) as usize] } - fTemp1168)))))};
			let mut fTemp1178: F64 = fTemp669 + fTemp1164;
			let mut fTemp1179: F64 = 65535.0 * (1.0 - fTemp1178);
			let mut iTemp1180: i32 = (fTemp1179) as i32;
			let mut iTemp1181: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1180, 65535)))), 196607));
			let mut fTemp1182: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1181, 3)) as usize] };
			let mut fTemp1183: F64 = unsafe { ftbl0mydspSIG0[iTemp1181 as usize] };
			let mut fTemp1184: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1181, 1)) as usize] } - fTemp1183;
			let mut fTemp1185: F64 = 65535.0 * fTemp1178;
			let mut iTemp1186: i32 = (fTemp1185) as i32;
			let mut iTemp1187: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1186, 65535)))), 196607));
			let mut fTemp1188: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1187, 3), 196607))) as usize] };
			let mut fTemp1189: F64 = unsafe { ftbl0mydspSIG0[iTemp1187 as usize] };
			let mut fTemp1190: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1187, 1), 196607))) as usize] } - fTemp1189;
			let mut iTemp1191: i32 = (fTemp725 > ((if iTemp655 != 0 {fTemp1189 + fTemp664 * fTemp1190 + (fTemp1185 - (iTemp1186) as F64) * (fTemp1188 - (fTemp1189 + fTemp664 * (fTemp1190 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1187, 4), 196607))) as usize] } - fTemp1188))))} else {1.0 - (fTemp1183 + fTemp664 * fTemp1184 + (fTemp1179 - (iTemp1180) as F64) * (fTemp1182 - (fTemp1183 + fTemp664 * (fTemp1184 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1181, 4)) as usize] } - fTemp1182)))))} - fTemp1177) / (1.0 - fTemp1177))) as i32;
			let mut fTemp1192: F64 = F64::min(1.0, F64::max(0.0, 0.5 * (if iTemp1191 != 0 {fTemp1164} else {fTemp1162} + if iTemp1191 != 0 {fTemp1161} else {fTemp1164})));
			self.fRec15[0] = fTemp1192;
			let mut fTemp1193: F64 = 65535.0 * (1.0 - fTemp1192);
			let mut iTemp1194: i32 = (fTemp1193) as i32;
			let mut iTemp1195: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1194, 65535)))), 196607));
			let mut fTemp1196: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1195, 3)) as usize] };
			let mut fTemp1197: F64 = unsafe { ftbl0mydspSIG0[iTemp1195 as usize] };
			let mut fTemp1198: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1195, 1)) as usize] } - fTemp1197;
			let mut fTemp1199: F64 = 65535.0 * fTemp1192;
			let mut iTemp1200: i32 = (fTemp1199) as i32;
			let mut iTemp1201: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1200, 65535)))), 196607));
			let mut fTemp1202: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1201, 3)) as usize] };
			let mut fTemp1203: F64 = unsafe { ftbl0mydspSIG0[iTemp1201 as usize] };
			let mut fTemp1204: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1201, 1)) as usize] } - fTemp1203;
			let mut fTemp1205: F64 = if iTemp655 != 0 {fTemp1203 + fTemp664 * fTemp1204 + (fTemp1199 - (iTemp1200) as F64) * (fTemp1202 - (fTemp1203 + fTemp664 * (fTemp1204 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1201, 4), 196607))) as usize] } - fTemp1202))))} else {1.0 - (fTemp1197 + fTemp664 * fTemp1198 + (fTemp1193 - (iTemp1194) as F64) * (fTemp1196 - (fTemp1197 + fTemp664 * (fTemp1198 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1195, 4), 196607))) as usize] } - fTemp1196)))))};
			let mut fTemp1206: F64 = fTemp669 + fTemp1192;
			let mut fTemp1207: F64 = 65535.0 * (1.0 - fTemp1206);
			let mut iTemp1208: i32 = (fTemp1207) as i32;
			let mut iTemp1209: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1208, 65535)))), 196607));
			let mut fTemp1210: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1209, 3)) as usize] };
			let mut fTemp1211: F64 = unsafe { ftbl0mydspSIG0[iTemp1209 as usize] };
			let mut fTemp1212: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1209, 1)) as usize] } - fTemp1211;
			let mut fTemp1213: F64 = 65535.0 * fTemp1206;
			let mut iTemp1214: i32 = (fTemp1213) as i32;
			let mut iTemp1215: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1214, 65535)))), 196607));
			let mut fTemp1216: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1215, 3), 196607))) as usize] };
			let mut fTemp1217: F64 = unsafe { ftbl0mydspSIG0[iTemp1215 as usize] };
			let mut fTemp1218: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1215, 1), 196607))) as usize] } - fTemp1217;
			let mut fTemp1219: F64 = fTemp629 + if ((0.001 * fTemp668) == 0.0) as i32 != 0 {fTemp654} else {fTemp654 * (if iTemp655 != 0 {fTemp1217 + fTemp664 * fTemp1218 + (fTemp1213 - (iTemp1214) as F64) * (fTemp1216 - (fTemp1217 + fTemp664 * (fTemp1218 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1215, 4), 196607))) as usize] } - fTemp1216))))} else {1.0 - (fTemp1211 + fTemp664 * fTemp1212 + (fTemp1207 - (iTemp1208) as F64) * (fTemp1210 - (fTemp1211 + fTemp664 * (fTemp1212 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1209, 4)) as usize] } - fTemp1210)))))} - fTemp1205) / (1.0 - fTemp1205)};
			self.fRec16[(self.IOTA0 & 8191) as usize] = if iTemp667 != 0 {F64::min(fTemp1219, fTemp629)} else {F64::max(fTemp1219, fTemp629)};
			let mut fTemp1220: F64 = self.fRec16[((i32::wrapping_sub(self.IOTA0, iSlow70)) & 8191) as usize];
			self.fHbargraph2 = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, fTemp1220));
			*output1 = 0.5 * fTemp2 * self.fVec2[((i32::wrapping_sub(self.IOTA0, iSlow71)) & 32767) as usize] + self.fRec14[0] * fTemp3 * self.fVec3[((i32::wrapping_sub(self.IOTA0, iSlow71)) & 32767) as usize] * fTemp1220;
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
pub use dsp::mydsp as LambRs;
