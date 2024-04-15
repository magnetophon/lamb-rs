mod dsp {
    /* ------------------------------------------------------------
author: "Bart Brouns"
license: "AGPLv3"
name: "lamb-rs"
version: "0.1"
Code generated with Faust 2.72.14 (https://faust.grame.fr)
Compilation options: -a /run/user/1001/.tmpGFKYe5 -lang rust -ct 1 -es 1 -mcd 16 -mdd 1024 -mdy 33 -double -ftz 0
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
			let mut iTemp64: i32 = i32::wrapping_add(self.iRec13[0], -1);
			let mut fTemp65: F64 = (iTemp64 % 3) as F64 as i32 as F64;
			let mut fTemp66: F64 = 0.5 * fTemp65;
			let mut fTemp67: F64 = F64::powf(fTemp66, 0.21 * fTemp65 + 1.0);
			let mut fTemp68: F64 = (0.3333333333333333 * (iTemp64 % 196608) as F64) as i32 as F64;
			table[i1 as usize] = F64::min(1.0, F64::max(0.0, if (fTemp66 == 0.0) as i32 != 0 {0.5 * (F64::sin(4.793763109162727e-05 * fTemp68 + 4.71238898038469) + 1.0)} else {0.5 * (F64::sin(3.141592653589793 * ((1.0 - F64::exp(-(3.692683299000534e-05 * fTemp67 * fTemp68))) / (1.0 - F64::exp(-(2.42 * fTemp67)))) + 4.71238898038469) + 1.0)}));
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
		m.declare("compile_options", r"-a /run/user/1001/.tmpGFKYe5 -lang rust -ct 1 -es 1 -mcd 16 -mdd 1024 -mdy 33 -double -ftz 0");
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
		return 4;
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
		let zipped_iterators = inputs0.zip(inputs1).zip(outputs0).zip(outputs1).zip(outputs2).zip(outputs3);
		for (((((input0, input1), output0), output1), output2), output3) in zipped_iterators {
			let mut fTemp0: F64 = self.fConst1 + self.fRec0[1];
			let mut fTemp1: F64 = self.fRec0[1] - self.fConst1;
			self.fRec0[0] = if (fTemp0 < fSlow0) as i32 != 0 {fTemp0} else {if (fTemp1 > fSlow0) as i32 != 0 {fTemp1} else {fSlow0}};
			let mut fTemp2: F64 = F64::sin(6.283185307179586 * (0.5 * self.fRec0[0] + 0.75)) + 1.0;
			let mut fTemp3: F64 = 0.5 * fTemp2;
			let mut fTemp4: F64 = 1.0 - fTemp3;
			let mut fTemp5: F64 = self.fRec2[((i32::wrapping_sub(self.IOTA0, 1)) & 8191) as usize];
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
			self.fVec0[(self.IOTA0 & 32767) as usize] = fTemp13;
			let mut fTemp14: F64 = fTemp13 * self.fRec11[0];
			self.fVec1[(self.IOTA0 & 32767) as usize] = fTemp14;
			let mut fTemp15: F64 = F64::abs(fTemp14);
			let mut fTemp16: F64 = *input1;
			self.fVec2[(self.IOTA0 & 32767) as usize] = fTemp16;
			let mut fTemp17: F64 = fTemp16 * self.fRec11[0];
			self.fVec3[(self.IOTA0 & 32767) as usize] = fTemp17;
			let mut fTemp18: F64 = F64::abs(fTemp17);
			let mut fTemp19: F64 = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, F64::max(fTemp15, fTemp18)));
			let mut iTemp20: i32 = ((fTemp19 > fSlow11) as i32) + ((fTemp19 > fSlow9) as i32);
			let mut fTemp21: F64 = fTemp19 - fSlow8;
			let mut fTemp22: F64 = F64::powf(1e+01, -(0.05 * F64::max(0.0, if (iTemp20 == 0) as i32 != 0 {0.0} else {if (iTemp20 == 1) as i32 != 0 {fSlow12 * mydsp_faustpower2_f(fSlow7 + fTemp21)} else {fTemp21}})));
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
			self.fRec6[0] = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, self.fRec12[0]));
			let mut fTemp33: F64 = self.fRec5[0] - self.fRec6[0];
			let mut fTemp34: F64 = F64::powf(1e+01, fSlow16 * F64::min(0.25, self.fRec4[0]) * (self.fRec6[0] + fTemp33 * (F64::max(fTemp11, F64::min(fTemp12, fTemp33)) + fTemp10) / fTemp9));
			let mut fTemp35: F64 = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, fTemp15));
			let mut fTemp36: F64 = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, fTemp18));
			let mut fTemp37: F64 = F64::max(fTemp35, fTemp36);
			let mut fTemp38: F64 = fTemp35 + fSlow17 * (fTemp37 - fTemp35);
			let mut iTemp39: i32 = ((fTemp38 > fSlow11) as i32) + ((fTemp38 > fSlow9) as i32);
			let mut fTemp40: F64 = fTemp38 - fSlow8;
			let mut fTemp41: F64 = F64::min(fTemp34, F64::powf(1e+01, -(fSlow18 * F64::max(0.0, if (iTemp39 == 0) as i32 != 0 {0.0} else {if (iTemp39 == 1) as i32 != 0 {fSlow12 * mydsp_faustpower2_f(fSlow7 + fTemp40)} else {fTemp40}}))));
			self.fVec4[(self.IOTA0 & 16383) as usize] = fTemp41;
			let mut fTemp42: F64 = F64::min(fTemp41, self.fVec4[((i32::wrapping_sub(self.IOTA0, 1)) & 16383) as usize]);
			self.fVec5[0] = fTemp42;
			let mut fTemp43: F64 = F64::min(fTemp42, self.fVec5[2]);
			self.fVec6[0] = fTemp43;
			let mut fTemp44: F64 = F64::min(fTemp43, self.fVec6[4]);
			self.fVec7[0] = fTemp44;
			let mut fTemp45: F64 = F64::min(fTemp44, self.fVec7[8]);
			self.fVec8[(self.IOTA0 & 31) as usize] = fTemp45;
			let mut fTemp46: F64 = F64::min(fTemp45, self.fVec8[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize]);
			self.fVec9[(self.IOTA0 & 63) as usize] = fTemp46;
			let mut fTemp47: F64 = F64::min(fTemp46, self.fVec9[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize]);
			self.fVec10[(self.IOTA0 & 127) as usize] = fTemp47;
			let mut fTemp48: F64 = F64::min(fTemp47, self.fVec10[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize]);
			self.fVec11[(self.IOTA0 & 255) as usize] = fTemp48;
			let mut fTemp49: F64 = F64::min(fTemp48, self.fVec11[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize]);
			self.fVec12[(self.IOTA0 & 511) as usize] = fTemp49;
			let mut fTemp50: F64 = F64::min(fTemp49, self.fVec12[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize]);
			self.fVec13[(self.IOTA0 & 1023) as usize] = fTemp50;
			let mut fTemp51: F64 = F64::min(fTemp50, self.fVec13[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize]);
			self.fVec14[(self.IOTA0 & 2047) as usize] = fTemp51;
			self.fVec15[(self.IOTA0 & 4095) as usize] = F64::min(fTemp51, self.fVec14[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			self.fRec3[0] = F64::max(F64::min(self.fRec3[1], self.fVec4[((i32::wrapping_sub(self.IOTA0, iSlow21)) & 16383) as usize]), F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow23 != 0 {fTemp41} else {1.7976931348623157e+308}, if iSlow24 != 0 {self.fVec5[iSlow23 as usize]} else {1.7976931348623157e+308}), if iSlow25 != 0 {self.fVec6[iSlow26 as usize]} else {1.7976931348623157e+308}), if iSlow27 != 0 {self.fVec7[iSlow28 as usize]} else {1.7976931348623157e+308}), if iSlow29 != 0 {self.fVec8[((i32::wrapping_sub(self.IOTA0, iSlow30)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow31 != 0 {self.fVec9[((i32::wrapping_sub(self.IOTA0, iSlow32)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow33 != 0 {self.fVec10[((i32::wrapping_sub(self.IOTA0, iSlow34)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow35 != 0 {self.fVec11[((i32::wrapping_sub(self.IOTA0, iSlow36)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow37 != 0 {self.fVec12[((i32::wrapping_sub(self.IOTA0, iSlow38)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow39 != 0 {self.fVec13[((i32::wrapping_sub(self.IOTA0, iSlow40)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow41 != 0 {self.fVec14[((i32::wrapping_sub(self.IOTA0, iSlow42)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow43 != 0 {self.fVec15[((i32::wrapping_sub(self.IOTA0, iSlow44)) & 4095) as usize]} else {1.7976931348623157e+308}));
			let mut fTemp52: F64 = F64::min(self.fRec3[0], self.fRec3[1]);
			self.fVec16[0] = fTemp52;
			let mut fTemp53: F64 = F64::min(fTemp52, self.fVec16[2]);
			self.fVec17[0] = fTemp53;
			let mut fTemp54: F64 = F64::min(fTemp53, self.fVec17[4]);
			self.fVec18[0] = fTemp54;
			let mut fTemp55: F64 = F64::min(fTemp54, self.fVec18[8]);
			self.fVec19[(self.IOTA0 & 31) as usize] = fTemp55;
			let mut fTemp56: F64 = F64::min(fTemp55, self.fVec19[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize]);
			self.fVec20[(self.IOTA0 & 63) as usize] = fTemp56;
			let mut fTemp57: F64 = F64::min(fTemp56, self.fVec20[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize]);
			self.fVec21[(self.IOTA0 & 127) as usize] = fTemp57;
			let mut fTemp58: F64 = F64::min(fTemp57, self.fVec21[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize]);
			self.fVec22[(self.IOTA0 & 255) as usize] = fTemp58;
			let mut fTemp59: F64 = F64::min(fTemp58, self.fVec22[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize]);
			self.fVec23[(self.IOTA0 & 511) as usize] = fTemp59;
			let mut fTemp60: F64 = F64::min(fTemp59, self.fVec23[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize]);
			self.fVec24[(self.IOTA0 & 1023) as usize] = fTemp60;
			let mut fTemp61: F64 = F64::min(fTemp60, self.fVec24[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize]);
			self.fVec25[(self.IOTA0 & 2047) as usize] = fTemp61;
			self.fVec26[(self.IOTA0 & 4095) as usize] = F64::min(fTemp61, self.fVec25[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			let mut fTemp62: F64 = F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow4 != 0 {self.fRec3[0]} else {1.7976931348623157e+308}, if iSlow45 != 0 {self.fVec16[iSlow4 as usize]} else {1.7976931348623157e+308}), if iSlow46 != 0 {self.fVec17[iSlow47 as usize]} else {1.7976931348623157e+308}), if iSlow48 != 0 {self.fVec18[iSlow49 as usize]} else {1.7976931348623157e+308}), if iSlow50 != 0 {self.fVec19[((i32::wrapping_sub(self.IOTA0, iSlow51)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow52 != 0 {self.fVec20[((i32::wrapping_sub(self.IOTA0, iSlow53)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow54 != 0 {self.fVec21[((i32::wrapping_sub(self.IOTA0, iSlow55)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow56 != 0 {self.fVec22[((i32::wrapping_sub(self.IOTA0, iSlow57)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow58 != 0 {self.fVec23[((i32::wrapping_sub(self.IOTA0, iSlow59)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow60 != 0 {self.fVec24[((i32::wrapping_sub(self.IOTA0, iSlow61)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow62 != 0 {self.fVec25[((i32::wrapping_sub(self.IOTA0, iSlow63)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow64 != 0 {self.fVec26[((i32::wrapping_sub(self.IOTA0, iSlow65)) & 4095) as usize]} else {1.7976931348623157e+308}) - fTemp5;
			self.fVec27[0] = fTemp62;
			let mut iTemp63: i32 = (fTemp62 > 0.0) as i32;
			let mut fTemp69: F64 = if iTemp63 != 0 {fSlow67} else {fSlow66};
			self.fVec28[0] = fTemp69;
			let mut fTemp70: F64 = 2.0 * fTemp69;
			let mut iTemp71: i32 = (fTemp70) as i32;
			let mut iTemp72: i32 = std::cmp::max(0, std::cmp::min(iTemp71, 2));
			let mut iTemp73: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, 98301), 196607));
			let mut fTemp74: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp73, 3)) as usize] };
			let mut fTemp75: F64 = unsafe { ftbl0mydspSIG0[iTemp73 as usize] };
			let mut fTemp76: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp73, 1)) as usize] } - fTemp75;
			let mut fTemp77: F64 = fTemp70 - (iTemp71) as F64;
			let mut fTemp78: F64 = fTemp75 + fTemp77 * fTemp76 + 0.5 * (fTemp74 - (fTemp75 + fTemp77 * (fTemp76 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp73, 4)) as usize] } - fTemp74))));
			let mut fTemp79: F64 = if iTemp63 != 0 {fTemp78} else {1.0 - fTemp78};
			let mut iTemp80: i32 = (fTemp62 < 0.0) as i32;
			let mut fTemp81: F64 = fSlow1 * (iTemp80) as F64 + fSlow13 * (iTemp63) as F64;
			self.fVec29[0] = fTemp81;
			let mut fTemp82: F64 = self.fConst10 / fTemp81;
			let mut fTemp83: F64 = fTemp82 + 0.5;
			let mut fTemp84: F64 = 65535.0 * (1.0 - fTemp83);
			let mut iTemp85: i32 = (fTemp84) as i32;
			let mut iTemp86: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp85, 65535)))), 196607));
			let mut fTemp87: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp86, 3)) as usize] };
			let mut fTemp88: F64 = unsafe { ftbl0mydspSIG0[iTemp86 as usize] };
			let mut fTemp89: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp86, 1)) as usize] } - fTemp88;
			let mut fTemp90: F64 = 65535.0 * fTemp83;
			let mut iTemp91: i32 = (fTemp90) as i32;
			let mut iTemp92: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp91, 65535)))), 196607));
			let mut fTemp93: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp92, 3), 196607))) as usize] };
			let mut fTemp94: F64 = unsafe { ftbl0mydspSIG0[iTemp92 as usize] };
			let mut fTemp95: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp92, 1), 196607))) as usize] } - fTemp94;
			let mut fTemp96: F64 = 2.0 * self.fVec28[1];
			let mut iTemp97: i32 = (fTemp96) as i32;
			let mut iTemp98: i32 = std::cmp::max(0, std::cmp::min(iTemp97, 2));
			let mut fTemp99: F64 = 65535.0 * (1.0 - self.fRec1[1]);
			let mut iTemp100: i32 = (fTemp99) as i32;
			let mut iTemp101: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp100, 65535))), iTemp98), 196607));
			let mut fTemp102: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp101, 3), 196607))) as usize] };
			let mut fTemp103: F64 = unsafe { ftbl0mydspSIG0[iTemp101 as usize] };
			let mut fTemp104: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp101, 1), 196607))) as usize] } - fTemp103;
			let mut fTemp105: F64 = fTemp96 - (iTemp97) as F64;
			let mut fTemp106: F64 = 65535.0 * self.fRec1[1];
			let mut iTemp107: i32 = (fTemp106) as i32;
			let mut iTemp108: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp98, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp107, 65535)))), 196607));
			let mut fTemp109: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp108, 3), 196607))) as usize] };
			let mut fTemp110: F64 = unsafe { ftbl0mydspSIG0[iTemp108 as usize] };
			let mut fTemp111: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp108, 1), 196607))) as usize] } - fTemp110;
			let mut fTemp112: F64 = self.fRec1[1] + fTemp82;
			let mut fTemp113: F64 = 65535.0 * (1.0 - fTemp112);
			let mut iTemp114: i32 = (fTemp113) as i32;
			let mut iTemp115: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp114, 65535)))), 196607));
			let mut fTemp116: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp115, 3)) as usize] };
			let mut fTemp117: F64 = unsafe { ftbl0mydspSIG0[iTemp115 as usize] };
			let mut fTemp118: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp115, 1)) as usize] } - fTemp117;
			let mut fTemp119: F64 = 65535.0 * fTemp112;
			let mut iTemp120: i32 = (fTemp119) as i32;
			let mut iTemp121: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp120, 65535)))), 196607));
			let mut fTemp122: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp121, 3), 196607))) as usize] };
			let mut fTemp123: F64 = unsafe { ftbl0mydspSIG0[iTemp121 as usize] };
			let mut fTemp124: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp121, 1), 196607))) as usize] } - fTemp123;
			let mut fTemp125: F64 = self.fRec1[1] + self.fConst10 * (1.0 / fTemp81 + 1.0 / self.fVec29[1]);
			let mut fTemp126: F64 = 65535.0 * (1.0 - fTemp125);
			let mut iTemp127: i32 = (fTemp126) as i32;
			let mut iTemp128: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp127, 65535))), iTemp72), 196607));
			let mut fTemp129: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp128, 3)) as usize] };
			let mut fTemp130: F64 = unsafe { ftbl0mydspSIG0[iTemp128 as usize] };
			let mut fTemp131: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp128, 1)) as usize] } - fTemp130;
			let mut fTemp132: F64 = 65535.0 * fTemp125;
			let mut iTemp133: i32 = (fTemp132) as i32;
			let mut iTemp134: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp133, 65535)))), 196607));
			let mut fTemp135: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp134, 3), 196607))) as usize] };
			let mut fTemp136: F64 = unsafe { ftbl0mydspSIG0[iTemp134 as usize] };
			let mut fTemp137: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp134, 1), 196607))) as usize] } - fTemp136;
			let mut fTemp138: F64 = (if iTemp63 != 0 {fTemp136 + fTemp77 * fTemp137 + (fTemp132 - (iTemp133) as F64) * (fTemp135 - (fTemp136 + fTemp77 * (fTemp137 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp134, 4), 196607))) as usize] } - fTemp135))))} else {1.0 - (fTemp130 + fTemp77 * fTemp131 + (fTemp126 - (iTemp127) as F64) * (fTemp129 - (fTemp130 + fTemp77 * (fTemp131 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp128, 4)) as usize] } - fTemp129)))))} - if iTemp63 != 0 {fTemp123 + fTemp77 * fTemp124 + (fTemp119 - (iTemp120) as F64) * (fTemp122 - (fTemp123 + fTemp77 * (fTemp124 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp121, 4), 196607))) as usize] } - fTemp122))))} else {1.0 - (fTemp117 + fTemp77 * fTemp118 + (fTemp113 - (iTemp114) as F64) * (fTemp116 - (fTemp117 + fTemp77 * (fTemp118 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp115, 4), 196607))) as usize] } - fTemp116)))))}) * self.fVec27[1] / (fTemp62 * (1.0 - if iTemp63 != 0 {fTemp110 + fTemp105 * fTemp111 + (fTemp106 - (iTemp107) as F64) * (fTemp109 - (fTemp110 + fTemp105 * (fTemp111 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp108, 4), 196607))) as usize] } - fTemp109))))} else {1.0 - (fTemp103 + fTemp105 * fTemp104 + (fTemp99 - (iTemp100) as F64) * (fTemp102 - (fTemp103 + fTemp105 * (fTemp104 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp101, 4), 196607))) as usize] } - fTemp102)))))}));
			let mut iTemp139: i32 = (fTemp138 > ((if iTemp63 != 0 {fTemp94 + fTemp77 * fTemp95 + (fTemp90 - (iTemp91) as F64) * (fTemp93 - (fTemp94 + fTemp77 * (fTemp95 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp92, 4), 196607))) as usize] } - fTemp93))))} else {1.0 - (fTemp88 + fTemp77 * fTemp89 + (fTemp84 - (iTemp85) as F64) * (fTemp87 - (fTemp88 + fTemp77 * (fTemp89 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp86, 4)) as usize] } - fTemp87)))))} - fTemp79) / (1.0 - fTemp79))) as i32;
			let mut fTemp140: F64 = if iTemp139 != 0 {1.0} else {0.5};
			let mut fTemp141: F64 = if iTemp139 != 0 {0.5} else {0.0};
			let mut fTemp142: F64 = fTemp141 + fTemp140;
			let mut fTemp143: F64 = 0.5 * fTemp142;
			let mut fTemp144: F64 = 65535.0 * (1.0 - fTemp143);
			let mut iTemp145: i32 = (fTemp144) as i32;
			let mut iTemp146: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp145, 65535)))), 196607));
			let mut fTemp147: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp146, 3)) as usize] };
			let mut fTemp148: F64 = unsafe { ftbl0mydspSIG0[iTemp146 as usize] };
			let mut fTemp149: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp146, 1)) as usize] } - fTemp148;
			let mut fTemp150: F64 = 32767.5 * fTemp142;
			let mut iTemp151: i32 = (fTemp150) as i32;
			let mut iTemp152: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp151, 65535)))), 196607));
			let mut fTemp153: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp152, 3)) as usize] };
			let mut fTemp154: F64 = unsafe { ftbl0mydspSIG0[iTemp152 as usize] };
			let mut fTemp155: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp152, 1)) as usize] } - fTemp154;
			let mut fTemp156: F64 = if iTemp63 != 0 {fTemp154 + fTemp77 * fTemp155 + (fTemp150 - (iTemp151) as F64) * (fTemp153 - (fTemp154 + fTemp77 * (fTemp155 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp152, 4)) as usize] } - fTemp153))))} else {1.0 - (fTemp148 + fTemp77 * fTemp149 + (fTemp144 - (iTemp145) as F64) * (fTemp147 - (fTemp148 + fTemp77 * (fTemp149 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp146, 4)) as usize] } - fTemp147)))))};
			let mut fTemp157: F64 = fTemp82 + fTemp143;
			let mut fTemp158: F64 = 65535.0 * (1.0 - fTemp157);
			let mut iTemp159: i32 = (fTemp158) as i32;
			let mut iTemp160: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp159, 65535)))), 196607));
			let mut fTemp161: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp160, 3)) as usize] };
			let mut fTemp162: F64 = unsafe { ftbl0mydspSIG0[iTemp160 as usize] };
			let mut fTemp163: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp160, 1)) as usize] } - fTemp162;
			let mut fTemp164: F64 = 65535.0 * fTemp157;
			let mut iTemp165: i32 = (fTemp164) as i32;
			let mut iTemp166: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp165, 65535)))), 196607));
			let mut fTemp167: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp166, 3), 196607))) as usize] };
			let mut fTemp168: F64 = unsafe { ftbl0mydspSIG0[iTemp166 as usize] };
			let mut fTemp169: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp166, 1), 196607))) as usize] } - fTemp168;
			let mut iTemp170: i32 = (fTemp138 > ((if iTemp63 != 0 {fTemp168 + fTemp77 * fTemp169 + (fTemp164 - (iTemp165) as F64) * (fTemp167 - (fTemp168 + fTemp77 * (fTemp169 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp166, 4), 196607))) as usize] } - fTemp167))))} else {1.0 - (fTemp162 + fTemp77 * fTemp163 + (fTemp158 - (iTemp159) as F64) * (fTemp161 - (fTemp162 + fTemp77 * (fTemp163 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp160, 4)) as usize] } - fTemp161)))))} - fTemp156) / (1.0 - fTemp156))) as i32;
			let mut fTemp171: F64 = if iTemp170 != 0 {fTemp140} else {fTemp143};
			let mut fTemp172: F64 = if iTemp170 != 0 {fTemp143} else {fTemp141};
			let mut fTemp173: F64 = fTemp172 + fTemp171;
			let mut fTemp174: F64 = 0.5 * fTemp173;
			let mut fTemp175: F64 = 65535.0 * (1.0 - fTemp174);
			let mut iTemp176: i32 = (fTemp175) as i32;
			let mut iTemp177: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp176, 65535)))), 196607));
			let mut fTemp178: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp177, 3)) as usize] };
			let mut fTemp179: F64 = unsafe { ftbl0mydspSIG0[iTemp177 as usize] };
			let mut fTemp180: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp177, 1)) as usize] } - fTemp179;
			let mut fTemp181: F64 = 32767.5 * fTemp173;
			let mut iTemp182: i32 = (fTemp181) as i32;
			let mut iTemp183: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp182, 65535)))), 196607));
			let mut fTemp184: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp183, 3)) as usize] };
			let mut fTemp185: F64 = unsafe { ftbl0mydspSIG0[iTemp183 as usize] };
			let mut fTemp186: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp183, 1)) as usize] } - fTemp185;
			let mut fTemp187: F64 = if iTemp63 != 0 {fTemp185 + fTemp77 * fTemp186 + (fTemp181 - (iTemp182) as F64) * (fTemp184 - (fTemp185 + fTemp77 * (fTemp186 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp183, 4)) as usize] } - fTemp184))))} else {1.0 - (fTemp179 + fTemp77 * fTemp180 + (fTemp175 - (iTemp176) as F64) * (fTemp178 - (fTemp179 + fTemp77 * (fTemp180 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp177, 4)) as usize] } - fTemp178)))))};
			let mut fTemp188: F64 = fTemp82 + fTemp174;
			let mut fTemp189: F64 = 65535.0 * (1.0 - fTemp188);
			let mut iTemp190: i32 = (fTemp189) as i32;
			let mut iTemp191: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp190, 65535)))), 196607));
			let mut fTemp192: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp191, 3)) as usize] };
			let mut fTemp193: F64 = unsafe { ftbl0mydspSIG0[iTemp191 as usize] };
			let mut fTemp194: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp191, 1)) as usize] } - fTemp193;
			let mut fTemp195: F64 = 65535.0 * fTemp188;
			let mut iTemp196: i32 = (fTemp195) as i32;
			let mut iTemp197: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp196, 65535)))), 196607));
			let mut fTemp198: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp197, 3), 196607))) as usize] };
			let mut fTemp199: F64 = unsafe { ftbl0mydspSIG0[iTemp197 as usize] };
			let mut fTemp200: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp197, 1), 196607))) as usize] } - fTemp199;
			let mut iTemp201: i32 = (fTemp138 > ((if iTemp63 != 0 {fTemp199 + fTemp77 * fTemp200 + (fTemp195 - (iTemp196) as F64) * (fTemp198 - (fTemp199 + fTemp77 * (fTemp200 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp197, 4), 196607))) as usize] } - fTemp198))))} else {1.0 - (fTemp193 + fTemp77 * fTemp194 + (fTemp189 - (iTemp190) as F64) * (fTemp192 - (fTemp193 + fTemp77 * (fTemp194 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp191, 4)) as usize] } - fTemp192)))))} - fTemp187) / (1.0 - fTemp187))) as i32;
			let mut fTemp202: F64 = if iTemp201 != 0 {fTemp171} else {fTemp174};
			let mut fTemp203: F64 = if iTemp201 != 0 {fTemp174} else {fTemp172};
			let mut fTemp204: F64 = fTemp203 + fTemp202;
			let mut fTemp205: F64 = 0.5 * fTemp204;
			let mut fTemp206: F64 = 65535.0 * (1.0 - fTemp205);
			let mut iTemp207: i32 = (fTemp206) as i32;
			let mut iTemp208: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp207, 65535)))), 196607));
			let mut fTemp209: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp208, 3)) as usize] };
			let mut fTemp210: F64 = unsafe { ftbl0mydspSIG0[iTemp208 as usize] };
			let mut fTemp211: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp208, 1)) as usize] } - fTemp210;
			let mut fTemp212: F64 = 32767.5 * fTemp204;
			let mut iTemp213: i32 = (fTemp212) as i32;
			let mut iTemp214: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp213, 65535)))), 196607));
			let mut fTemp215: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp214, 3)) as usize] };
			let mut fTemp216: F64 = unsafe { ftbl0mydspSIG0[iTemp214 as usize] };
			let mut fTemp217: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp214, 1)) as usize] } - fTemp216;
			let mut fTemp218: F64 = if iTemp63 != 0 {fTemp216 + fTemp77 * fTemp217 + (fTemp212 - (iTemp213) as F64) * (fTemp215 - (fTemp216 + fTemp77 * (fTemp217 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp214, 4)) as usize] } - fTemp215))))} else {1.0 - (fTemp210 + fTemp77 * fTemp211 + (fTemp206 - (iTemp207) as F64) * (fTemp209 - (fTemp210 + fTemp77 * (fTemp211 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp208, 4)) as usize] } - fTemp209)))))};
			let mut fTemp219: F64 = fTemp82 + fTemp205;
			let mut fTemp220: F64 = 65535.0 * (1.0 - fTemp219);
			let mut iTemp221: i32 = (fTemp220) as i32;
			let mut iTemp222: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp221, 65535)))), 196607));
			let mut fTemp223: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp222, 3)) as usize] };
			let mut fTemp224: F64 = unsafe { ftbl0mydspSIG0[iTemp222 as usize] };
			let mut fTemp225: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp222, 1)) as usize] } - fTemp224;
			let mut fTemp226: F64 = 65535.0 * fTemp219;
			let mut iTemp227: i32 = (fTemp226) as i32;
			let mut iTemp228: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp227, 65535)))), 196607));
			let mut fTemp229: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp228, 3), 196607))) as usize] };
			let mut fTemp230: F64 = unsafe { ftbl0mydspSIG0[iTemp228 as usize] };
			let mut fTemp231: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp228, 1), 196607))) as usize] } - fTemp230;
			let mut iTemp232: i32 = (fTemp138 > ((if iTemp63 != 0 {fTemp230 + fTemp77 * fTemp231 + (fTemp226 - (iTemp227) as F64) * (fTemp229 - (fTemp230 + fTemp77 * (fTemp231 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp228, 4), 196607))) as usize] } - fTemp229))))} else {1.0 - (fTemp224 + fTemp77 * fTemp225 + (fTemp220 - (iTemp221) as F64) * (fTemp223 - (fTemp224 + fTemp77 * (fTemp225 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp222, 4)) as usize] } - fTemp223)))))} - fTemp218) / (1.0 - fTemp218))) as i32;
			let mut fTemp233: F64 = if iTemp232 != 0 {fTemp202} else {fTemp205};
			let mut fTemp234: F64 = if iTemp232 != 0 {fTemp205} else {fTemp203};
			let mut fTemp235: F64 = fTemp234 + fTemp233;
			let mut fTemp236: F64 = 0.5 * fTemp235;
			let mut fTemp237: F64 = 65535.0 * (1.0 - fTemp236);
			let mut iTemp238: i32 = (fTemp237) as i32;
			let mut iTemp239: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp238, 65535)))), 196607));
			let mut fTemp240: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp239, 3)) as usize] };
			let mut fTemp241: F64 = unsafe { ftbl0mydspSIG0[iTemp239 as usize] };
			let mut fTemp242: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp239, 1)) as usize] } - fTemp241;
			let mut fTemp243: F64 = 32767.5 * fTemp235;
			let mut iTemp244: i32 = (fTemp243) as i32;
			let mut iTemp245: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp244, 65535)))), 196607));
			let mut fTemp246: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp245, 3)) as usize] };
			let mut fTemp247: F64 = unsafe { ftbl0mydspSIG0[iTemp245 as usize] };
			let mut fTemp248: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp245, 1)) as usize] } - fTemp247;
			let mut fTemp249: F64 = if iTemp63 != 0 {fTemp247 + fTemp77 * fTemp248 + (fTemp243 - (iTemp244) as F64) * (fTemp246 - (fTemp247 + fTemp77 * (fTemp248 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp245, 4)) as usize] } - fTemp246))))} else {1.0 - (fTemp241 + fTemp77 * fTemp242 + (fTemp237 - (iTemp238) as F64) * (fTemp240 - (fTemp241 + fTemp77 * (fTemp242 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp239, 4)) as usize] } - fTemp240)))))};
			let mut fTemp250: F64 = fTemp82 + fTemp236;
			let mut fTemp251: F64 = 65535.0 * (1.0 - fTemp250);
			let mut iTemp252: i32 = (fTemp251) as i32;
			let mut iTemp253: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp252, 65535)))), 196607));
			let mut fTemp254: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp253, 3)) as usize] };
			let mut fTemp255: F64 = unsafe { ftbl0mydspSIG0[iTemp253 as usize] };
			let mut fTemp256: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp253, 1)) as usize] } - fTemp255;
			let mut fTemp257: F64 = 65535.0 * fTemp250;
			let mut iTemp258: i32 = (fTemp257) as i32;
			let mut iTemp259: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp258, 65535)))), 196607));
			let mut fTemp260: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp259, 3), 196607))) as usize] };
			let mut fTemp261: F64 = unsafe { ftbl0mydspSIG0[iTemp259 as usize] };
			let mut fTemp262: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp259, 1), 196607))) as usize] } - fTemp261;
			let mut iTemp263: i32 = (fTemp138 > ((if iTemp63 != 0 {fTemp261 + fTemp77 * fTemp262 + (fTemp257 - (iTemp258) as F64) * (fTemp260 - (fTemp261 + fTemp77 * (fTemp262 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp259, 4), 196607))) as usize] } - fTemp260))))} else {1.0 - (fTemp255 + fTemp77 * fTemp256 + (fTemp251 - (iTemp252) as F64) * (fTemp254 - (fTemp255 + fTemp77 * (fTemp256 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp253, 4)) as usize] } - fTemp254)))))} - fTemp249) / (1.0 - fTemp249))) as i32;
			let mut fTemp264: F64 = if iTemp263 != 0 {fTemp233} else {fTemp236};
			let mut fTemp265: F64 = if iTemp263 != 0 {fTemp236} else {fTemp234};
			let mut fTemp266: F64 = fTemp265 + fTemp264;
			let mut fTemp267: F64 = 0.5 * fTemp266;
			let mut fTemp268: F64 = 65535.0 * (1.0 - fTemp267);
			let mut iTemp269: i32 = (fTemp268) as i32;
			let mut iTemp270: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp269, 65535)))), 196607));
			let mut fTemp271: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp270, 3)) as usize] };
			let mut fTemp272: F64 = unsafe { ftbl0mydspSIG0[iTemp270 as usize] };
			let mut fTemp273: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp270, 1)) as usize] } - fTemp272;
			let mut fTemp274: F64 = 32767.5 * fTemp266;
			let mut iTemp275: i32 = (fTemp274) as i32;
			let mut iTemp276: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp275, 65535)))), 196607));
			let mut fTemp277: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp276, 3)) as usize] };
			let mut fTemp278: F64 = unsafe { ftbl0mydspSIG0[iTemp276 as usize] };
			let mut fTemp279: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp276, 1)) as usize] } - fTemp278;
			let mut fTemp280: F64 = if iTemp63 != 0 {fTemp278 + fTemp77 * fTemp279 + (fTemp274 - (iTemp275) as F64) * (fTemp277 - (fTemp278 + fTemp77 * (fTemp279 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp276, 4)) as usize] } - fTemp277))))} else {1.0 - (fTemp272 + fTemp77 * fTemp273 + (fTemp268 - (iTemp269) as F64) * (fTemp271 - (fTemp272 + fTemp77 * (fTemp273 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp270, 4)) as usize] } - fTemp271)))))};
			let mut fTemp281: F64 = fTemp82 + fTemp267;
			let mut fTemp282: F64 = 65535.0 * (1.0 - fTemp281);
			let mut iTemp283: i32 = (fTemp282) as i32;
			let mut iTemp284: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp283, 65535)))), 196607));
			let mut fTemp285: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp284, 3)) as usize] };
			let mut fTemp286: F64 = unsafe { ftbl0mydspSIG0[iTemp284 as usize] };
			let mut fTemp287: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp284, 1)) as usize] } - fTemp286;
			let mut fTemp288: F64 = 65535.0 * fTemp281;
			let mut iTemp289: i32 = (fTemp288) as i32;
			let mut iTemp290: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp289, 65535)))), 196607));
			let mut fTemp291: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp290, 3), 196607))) as usize] };
			let mut fTemp292: F64 = unsafe { ftbl0mydspSIG0[iTemp290 as usize] };
			let mut fTemp293: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp290, 1), 196607))) as usize] } - fTemp292;
			let mut iTemp294: i32 = (fTemp138 > ((if iTemp63 != 0 {fTemp292 + fTemp77 * fTemp293 + (fTemp288 - (iTemp289) as F64) * (fTemp291 - (fTemp292 + fTemp77 * (fTemp293 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp290, 4), 196607))) as usize] } - fTemp291))))} else {1.0 - (fTemp286 + fTemp77 * fTemp287 + (fTemp282 - (iTemp283) as F64) * (fTemp285 - (fTemp286 + fTemp77 * (fTemp287 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp284, 4)) as usize] } - fTemp285)))))} - fTemp280) / (1.0 - fTemp280))) as i32;
			let mut fTemp295: F64 = if iTemp294 != 0 {fTemp264} else {fTemp267};
			let mut fTemp296: F64 = if iTemp294 != 0 {fTemp267} else {fTemp265};
			let mut fTemp297: F64 = fTemp296 + fTemp295;
			let mut fTemp298: F64 = 0.5 * fTemp297;
			let mut fTemp299: F64 = 65535.0 * (1.0 - fTemp298);
			let mut iTemp300: i32 = (fTemp299) as i32;
			let mut iTemp301: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp300, 65535)))), 196607));
			let mut fTemp302: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp301, 3)) as usize] };
			let mut fTemp303: F64 = unsafe { ftbl0mydspSIG0[iTemp301 as usize] };
			let mut fTemp304: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp301, 1)) as usize] } - fTemp303;
			let mut fTemp305: F64 = 32767.5 * fTemp297;
			let mut iTemp306: i32 = (fTemp305) as i32;
			let mut iTemp307: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp306, 65535)))), 196607));
			let mut fTemp308: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp307, 3)) as usize] };
			let mut fTemp309: F64 = unsafe { ftbl0mydspSIG0[iTemp307 as usize] };
			let mut fTemp310: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp307, 1)) as usize] } - fTemp309;
			let mut fTemp311: F64 = if iTemp63 != 0 {fTemp309 + fTemp77 * fTemp310 + (fTemp305 - (iTemp306) as F64) * (fTemp308 - (fTemp309 + fTemp77 * (fTemp310 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp307, 4)) as usize] } - fTemp308))))} else {1.0 - (fTemp303 + fTemp77 * fTemp304 + (fTemp299 - (iTemp300) as F64) * (fTemp302 - (fTemp303 + fTemp77 * (fTemp304 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp301, 4)) as usize] } - fTemp302)))))};
			let mut fTemp312: F64 = fTemp82 + fTemp298;
			let mut fTemp313: F64 = 65535.0 * (1.0 - fTemp312);
			let mut iTemp314: i32 = (fTemp313) as i32;
			let mut iTemp315: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp314, 65535)))), 196607));
			let mut fTemp316: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp315, 3)) as usize] };
			let mut fTemp317: F64 = unsafe { ftbl0mydspSIG0[iTemp315 as usize] };
			let mut fTemp318: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp315, 1)) as usize] } - fTemp317;
			let mut fTemp319: F64 = 65535.0 * fTemp312;
			let mut iTemp320: i32 = (fTemp319) as i32;
			let mut iTemp321: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp320, 65535)))), 196607));
			let mut fTemp322: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp321, 3), 196607))) as usize] };
			let mut fTemp323: F64 = unsafe { ftbl0mydspSIG0[iTemp321 as usize] };
			let mut fTemp324: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp321, 1), 196607))) as usize] } - fTemp323;
			let mut iTemp325: i32 = (fTemp138 > ((if iTemp63 != 0 {fTemp323 + fTemp77 * fTemp324 + (fTemp319 - (iTemp320) as F64) * (fTemp322 - (fTemp323 + fTemp77 * (fTemp324 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp321, 4), 196607))) as usize] } - fTemp322))))} else {1.0 - (fTemp317 + fTemp77 * fTemp318 + (fTemp313 - (iTemp314) as F64) * (fTemp316 - (fTemp317 + fTemp77 * (fTemp318 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp315, 4)) as usize] } - fTemp316)))))} - fTemp311) / (1.0 - fTemp311))) as i32;
			let mut fTemp326: F64 = if iTemp325 != 0 {fTemp295} else {fTemp298};
			let mut fTemp327: F64 = if iTemp325 != 0 {fTemp298} else {fTemp296};
			let mut fTemp328: F64 = fTemp327 + fTemp326;
			let mut fTemp329: F64 = 0.5 * fTemp328;
			let mut fTemp330: F64 = 65535.0 * (1.0 - fTemp329);
			let mut iTemp331: i32 = (fTemp330) as i32;
			let mut iTemp332: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp331, 65535)))), 196607));
			let mut fTemp333: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp332, 3)) as usize] };
			let mut fTemp334: F64 = unsafe { ftbl0mydspSIG0[iTemp332 as usize] };
			let mut fTemp335: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp332, 1)) as usize] } - fTemp334;
			let mut fTemp336: F64 = 32767.5 * fTemp328;
			let mut iTemp337: i32 = (fTemp336) as i32;
			let mut iTemp338: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp337, 65535)))), 196607));
			let mut fTemp339: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp338, 3)) as usize] };
			let mut fTemp340: F64 = unsafe { ftbl0mydspSIG0[iTemp338 as usize] };
			let mut fTemp341: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp338, 1)) as usize] } - fTemp340;
			let mut fTemp342: F64 = if iTemp63 != 0 {fTemp340 + fTemp77 * fTemp341 + (fTemp336 - (iTemp337) as F64) * (fTemp339 - (fTemp340 + fTemp77 * (fTemp341 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp338, 4)) as usize] } - fTemp339))))} else {1.0 - (fTemp334 + fTemp77 * fTemp335 + (fTemp330 - (iTemp331) as F64) * (fTemp333 - (fTemp334 + fTemp77 * (fTemp335 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp332, 4)) as usize] } - fTemp333)))))};
			let mut fTemp343: F64 = fTemp82 + fTemp329;
			let mut fTemp344: F64 = 65535.0 * (1.0 - fTemp343);
			let mut iTemp345: i32 = (fTemp344) as i32;
			let mut iTemp346: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp345, 65535)))), 196607));
			let mut fTemp347: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp346, 3)) as usize] };
			let mut fTemp348: F64 = unsafe { ftbl0mydspSIG0[iTemp346 as usize] };
			let mut fTemp349: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp346, 1)) as usize] } - fTemp348;
			let mut fTemp350: F64 = 65535.0 * fTemp343;
			let mut iTemp351: i32 = (fTemp350) as i32;
			let mut iTemp352: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp351, 65535)))), 196607));
			let mut fTemp353: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp352, 3), 196607))) as usize] };
			let mut fTemp354: F64 = unsafe { ftbl0mydspSIG0[iTemp352 as usize] };
			let mut fTemp355: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp352, 1), 196607))) as usize] } - fTemp354;
			let mut iTemp356: i32 = (fTemp138 > ((if iTemp63 != 0 {fTemp354 + fTemp77 * fTemp355 + (fTemp350 - (iTemp351) as F64) * (fTemp353 - (fTemp354 + fTemp77 * (fTemp355 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp352, 4), 196607))) as usize] } - fTemp353))))} else {1.0 - (fTemp348 + fTemp77 * fTemp349 + (fTemp344 - (iTemp345) as F64) * (fTemp347 - (fTemp348 + fTemp77 * (fTemp349 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp346, 4)) as usize] } - fTemp347)))))} - fTemp342) / (1.0 - fTemp342))) as i32;
			let mut fTemp357: F64 = if iTemp356 != 0 {fTemp326} else {fTemp329};
			let mut fTemp358: F64 = if iTemp356 != 0 {fTemp329} else {fTemp327};
			let mut fTemp359: F64 = fTemp358 + fTemp357;
			let mut fTemp360: F64 = 0.5 * fTemp359;
			let mut fTemp361: F64 = 65535.0 * (1.0 - fTemp360);
			let mut iTemp362: i32 = (fTemp361) as i32;
			let mut iTemp363: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp362, 65535)))), 196607));
			let mut fTemp364: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp363, 3)) as usize] };
			let mut fTemp365: F64 = unsafe { ftbl0mydspSIG0[iTemp363 as usize] };
			let mut fTemp366: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp363, 1)) as usize] } - fTemp365;
			let mut fTemp367: F64 = 32767.5 * fTemp359;
			let mut iTemp368: i32 = (fTemp367) as i32;
			let mut iTemp369: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp368, 65535)))), 196607));
			let mut fTemp370: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp369, 3)) as usize] };
			let mut fTemp371: F64 = unsafe { ftbl0mydspSIG0[iTemp369 as usize] };
			let mut fTemp372: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp369, 1)) as usize] } - fTemp371;
			let mut fTemp373: F64 = if iTemp63 != 0 {fTemp371 + fTemp77 * fTemp372 + (fTemp367 - (iTemp368) as F64) * (fTemp370 - (fTemp371 + fTemp77 * (fTemp372 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp369, 4)) as usize] } - fTemp370))))} else {1.0 - (fTemp365 + fTemp77 * fTemp366 + (fTemp361 - (iTemp362) as F64) * (fTemp364 - (fTemp365 + fTemp77 * (fTemp366 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp363, 4)) as usize] } - fTemp364)))))};
			let mut fTemp374: F64 = fTemp82 + fTemp360;
			let mut fTemp375: F64 = 65535.0 * (1.0 - fTemp374);
			let mut iTemp376: i32 = (fTemp375) as i32;
			let mut iTemp377: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp376, 65535)))), 196607));
			let mut fTemp378: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp377, 3)) as usize] };
			let mut fTemp379: F64 = unsafe { ftbl0mydspSIG0[iTemp377 as usize] };
			let mut fTemp380: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp377, 1)) as usize] } - fTemp379;
			let mut fTemp381: F64 = 65535.0 * fTemp374;
			let mut iTemp382: i32 = (fTemp381) as i32;
			let mut iTemp383: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp382, 65535)))), 196607));
			let mut fTemp384: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp383, 3), 196607))) as usize] };
			let mut fTemp385: F64 = unsafe { ftbl0mydspSIG0[iTemp383 as usize] };
			let mut fTemp386: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp383, 1), 196607))) as usize] } - fTemp385;
			let mut iTemp387: i32 = (fTemp138 > ((if iTemp63 != 0 {fTemp385 + fTemp77 * fTemp386 + (fTemp381 - (iTemp382) as F64) * (fTemp384 - (fTemp385 + fTemp77 * (fTemp386 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp383, 4), 196607))) as usize] } - fTemp384))))} else {1.0 - (fTemp379 + fTemp77 * fTemp380 + (fTemp375 - (iTemp376) as F64) * (fTemp378 - (fTemp379 + fTemp77 * (fTemp380 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp377, 4)) as usize] } - fTemp378)))))} - fTemp373) / (1.0 - fTemp373))) as i32;
			let mut fTemp388: F64 = if iTemp387 != 0 {fTemp357} else {fTemp360};
			let mut fTemp389: F64 = if iTemp387 != 0 {fTemp360} else {fTemp358};
			let mut fTemp390: F64 = fTemp389 + fTemp388;
			let mut fTemp391: F64 = 0.5 * fTemp390;
			let mut fTemp392: F64 = 65535.0 * (1.0 - fTemp391);
			let mut iTemp393: i32 = (fTemp392) as i32;
			let mut iTemp394: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp393, 65535)))), 196607));
			let mut fTemp395: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp394, 3)) as usize] };
			let mut fTemp396: F64 = unsafe { ftbl0mydspSIG0[iTemp394 as usize] };
			let mut fTemp397: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp394, 1)) as usize] } - fTemp396;
			let mut fTemp398: F64 = 32767.5 * fTemp390;
			let mut iTemp399: i32 = (fTemp398) as i32;
			let mut iTemp400: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp399, 65535)))), 196607));
			let mut fTemp401: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp400, 3)) as usize] };
			let mut fTemp402: F64 = unsafe { ftbl0mydspSIG0[iTemp400 as usize] };
			let mut fTemp403: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp400, 1)) as usize] } - fTemp402;
			let mut fTemp404: F64 = if iTemp63 != 0 {fTemp402 + fTemp77 * fTemp403 + (fTemp398 - (iTemp399) as F64) * (fTemp401 - (fTemp402 + fTemp77 * (fTemp403 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp400, 4)) as usize] } - fTemp401))))} else {1.0 - (fTemp396 + fTemp77 * fTemp397 + (fTemp392 - (iTemp393) as F64) * (fTemp395 - (fTemp396 + fTemp77 * (fTemp397 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp394, 4)) as usize] } - fTemp395)))))};
			let mut fTemp405: F64 = fTemp82 + fTemp391;
			let mut fTemp406: F64 = 65535.0 * (1.0 - fTemp405);
			let mut iTemp407: i32 = (fTemp406) as i32;
			let mut iTemp408: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp407, 65535)))), 196607));
			let mut fTemp409: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp408, 3)) as usize] };
			let mut fTemp410: F64 = unsafe { ftbl0mydspSIG0[iTemp408 as usize] };
			let mut fTemp411: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp408, 1)) as usize] } - fTemp410;
			let mut fTemp412: F64 = 65535.0 * fTemp405;
			let mut iTemp413: i32 = (fTemp412) as i32;
			let mut iTemp414: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp413, 65535)))), 196607));
			let mut fTemp415: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp414, 3), 196607))) as usize] };
			let mut fTemp416: F64 = unsafe { ftbl0mydspSIG0[iTemp414 as usize] };
			let mut fTemp417: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp414, 1), 196607))) as usize] } - fTemp416;
			let mut iTemp418: i32 = (fTemp138 > ((if iTemp63 != 0 {fTemp416 + fTemp77 * fTemp417 + (fTemp412 - (iTemp413) as F64) * (fTemp415 - (fTemp416 + fTemp77 * (fTemp417 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp414, 4), 196607))) as usize] } - fTemp415))))} else {1.0 - (fTemp410 + fTemp77 * fTemp411 + (fTemp406 - (iTemp407) as F64) * (fTemp409 - (fTemp410 + fTemp77 * (fTemp411 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp408, 4)) as usize] } - fTemp409)))))} - fTemp404) / (1.0 - fTemp404))) as i32;
			let mut fTemp419: F64 = if iTemp418 != 0 {fTemp388} else {fTemp391};
			let mut fTemp420: F64 = if iTemp418 != 0 {fTemp391} else {fTemp389};
			let mut fTemp421: F64 = fTemp420 + fTemp419;
			let mut fTemp422: F64 = 0.5 * fTemp421;
			let mut fTemp423: F64 = 65535.0 * (1.0 - fTemp422);
			let mut iTemp424: i32 = (fTemp423) as i32;
			let mut iTemp425: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp424, 65535)))), 196607));
			let mut fTemp426: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp425, 3)) as usize] };
			let mut fTemp427: F64 = unsafe { ftbl0mydspSIG0[iTemp425 as usize] };
			let mut fTemp428: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp425, 1)) as usize] } - fTemp427;
			let mut fTemp429: F64 = 32767.5 * fTemp421;
			let mut iTemp430: i32 = (fTemp429) as i32;
			let mut iTemp431: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp430, 65535)))), 196607));
			let mut fTemp432: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp431, 3)) as usize] };
			let mut fTemp433: F64 = unsafe { ftbl0mydspSIG0[iTemp431 as usize] };
			let mut fTemp434: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp431, 1)) as usize] } - fTemp433;
			let mut fTemp435: F64 = if iTemp63 != 0 {fTemp433 + fTemp77 * fTemp434 + (fTemp429 - (iTemp430) as F64) * (fTemp432 - (fTemp433 + fTemp77 * (fTemp434 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp431, 4)) as usize] } - fTemp432))))} else {1.0 - (fTemp427 + fTemp77 * fTemp428 + (fTemp423 - (iTemp424) as F64) * (fTemp426 - (fTemp427 + fTemp77 * (fTemp428 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp425, 4)) as usize] } - fTemp426)))))};
			let mut fTemp436: F64 = fTemp82 + fTemp422;
			let mut fTemp437: F64 = 65535.0 * (1.0 - fTemp436);
			let mut iTemp438: i32 = (fTemp437) as i32;
			let mut iTemp439: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp438, 65535)))), 196607));
			let mut fTemp440: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp439, 3)) as usize] };
			let mut fTemp441: F64 = unsafe { ftbl0mydspSIG0[iTemp439 as usize] };
			let mut fTemp442: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp439, 1)) as usize] } - fTemp441;
			let mut fTemp443: F64 = 65535.0 * fTemp436;
			let mut iTemp444: i32 = (fTemp443) as i32;
			let mut iTemp445: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp444, 65535)))), 196607));
			let mut fTemp446: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp445, 3), 196607))) as usize] };
			let mut fTemp447: F64 = unsafe { ftbl0mydspSIG0[iTemp445 as usize] };
			let mut fTemp448: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp445, 1), 196607))) as usize] } - fTemp447;
			let mut iTemp449: i32 = (fTemp138 > ((if iTemp63 != 0 {fTemp447 + fTemp77 * fTemp448 + (fTemp443 - (iTemp444) as F64) * (fTemp446 - (fTemp447 + fTemp77 * (fTemp448 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp445, 4), 196607))) as usize] } - fTemp446))))} else {1.0 - (fTemp441 + fTemp77 * fTemp442 + (fTemp437 - (iTemp438) as F64) * (fTemp440 - (fTemp441 + fTemp77 * (fTemp442 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp439, 4)) as usize] } - fTemp440)))))} - fTemp435) / (1.0 - fTemp435))) as i32;
			let mut fTemp450: F64 = if iTemp449 != 0 {fTemp419} else {fTemp422};
			let mut fTemp451: F64 = if iTemp449 != 0 {fTemp422} else {fTemp420};
			let mut fTemp452: F64 = fTemp451 + fTemp450;
			let mut fTemp453: F64 = 0.5 * fTemp452;
			let mut fTemp454: F64 = 65535.0 * (1.0 - fTemp453);
			let mut iTemp455: i32 = (fTemp454) as i32;
			let mut iTemp456: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp455, 65535)))), 196607));
			let mut fTemp457: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp456, 3)) as usize] };
			let mut fTemp458: F64 = unsafe { ftbl0mydspSIG0[iTemp456 as usize] };
			let mut fTemp459: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp456, 1)) as usize] } - fTemp458;
			let mut fTemp460: F64 = 32767.5 * fTemp452;
			let mut iTemp461: i32 = (fTemp460) as i32;
			let mut iTemp462: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp461, 65535)))), 196607));
			let mut fTemp463: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp462, 3)) as usize] };
			let mut fTemp464: F64 = unsafe { ftbl0mydspSIG0[iTemp462 as usize] };
			let mut fTemp465: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp462, 1)) as usize] } - fTemp464;
			let mut fTemp466: F64 = if iTemp63 != 0 {fTemp464 + fTemp77 * fTemp465 + (fTemp460 - (iTemp461) as F64) * (fTemp463 - (fTemp464 + fTemp77 * (fTemp465 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp462, 4)) as usize] } - fTemp463))))} else {1.0 - (fTemp458 + fTemp77 * fTemp459 + (fTemp454 - (iTemp455) as F64) * (fTemp457 - (fTemp458 + fTemp77 * (fTemp459 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp456, 4)) as usize] } - fTemp457)))))};
			let mut fTemp467: F64 = fTemp82 + fTemp453;
			let mut fTemp468: F64 = 65535.0 * (1.0 - fTemp467);
			let mut iTemp469: i32 = (fTemp468) as i32;
			let mut iTemp470: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp469, 65535)))), 196607));
			let mut fTemp471: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp470, 3)) as usize] };
			let mut fTemp472: F64 = unsafe { ftbl0mydspSIG0[iTemp470 as usize] };
			let mut fTemp473: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp470, 1)) as usize] } - fTemp472;
			let mut fTemp474: F64 = 65535.0 * fTemp467;
			let mut iTemp475: i32 = (fTemp474) as i32;
			let mut iTemp476: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp475, 65535)))), 196607));
			let mut fTemp477: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp476, 3), 196607))) as usize] };
			let mut fTemp478: F64 = unsafe { ftbl0mydspSIG0[iTemp476 as usize] };
			let mut fTemp479: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp476, 1), 196607))) as usize] } - fTemp478;
			let mut iTemp480: i32 = (fTemp138 > ((if iTemp63 != 0 {fTemp478 + fTemp77 * fTemp479 + (fTemp474 - (iTemp475) as F64) * (fTemp477 - (fTemp478 + fTemp77 * (fTemp479 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp476, 4), 196607))) as usize] } - fTemp477))))} else {1.0 - (fTemp472 + fTemp77 * fTemp473 + (fTemp468 - (iTemp469) as F64) * (fTemp471 - (fTemp472 + fTemp77 * (fTemp473 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp470, 4)) as usize] } - fTemp471)))))} - fTemp466) / (1.0 - fTemp466))) as i32;
			let mut fTemp481: F64 = if iTemp480 != 0 {fTemp450} else {fTemp453};
			let mut fTemp482: F64 = if iTemp480 != 0 {fTemp453} else {fTemp451};
			let mut fTemp483: F64 = fTemp482 + fTemp481;
			let mut fTemp484: F64 = 0.5 * fTemp483;
			let mut fTemp485: F64 = 65535.0 * (1.0 - fTemp484);
			let mut iTemp486: i32 = (fTemp485) as i32;
			let mut iTemp487: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp486, 65535)))), 196607));
			let mut fTemp488: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp487, 3)) as usize] };
			let mut fTemp489: F64 = unsafe { ftbl0mydspSIG0[iTemp487 as usize] };
			let mut fTemp490: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp487, 1)) as usize] } - fTemp489;
			let mut fTemp491: F64 = 32767.5 * fTemp483;
			let mut iTemp492: i32 = (fTemp491) as i32;
			let mut iTemp493: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp492, 65535)))), 196607));
			let mut fTemp494: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp493, 3)) as usize] };
			let mut fTemp495: F64 = unsafe { ftbl0mydspSIG0[iTemp493 as usize] };
			let mut fTemp496: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp493, 1)) as usize] } - fTemp495;
			let mut fTemp497: F64 = if iTemp63 != 0 {fTemp495 + fTemp77 * fTemp496 + (fTemp491 - (iTemp492) as F64) * (fTemp494 - (fTemp495 + fTemp77 * (fTemp496 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp493, 4)) as usize] } - fTemp494))))} else {1.0 - (fTemp489 + fTemp77 * fTemp490 + (fTemp485 - (iTemp486) as F64) * (fTemp488 - (fTemp489 + fTemp77 * (fTemp490 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp487, 4)) as usize] } - fTemp488)))))};
			let mut fTemp498: F64 = fTemp82 + fTemp484;
			let mut fTemp499: F64 = 65535.0 * (1.0 - fTemp498);
			let mut iTemp500: i32 = (fTemp499) as i32;
			let mut iTemp501: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp500, 65535)))), 196607));
			let mut fTemp502: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp501, 3)) as usize] };
			let mut fTemp503: F64 = unsafe { ftbl0mydspSIG0[iTemp501 as usize] };
			let mut fTemp504: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp501, 1)) as usize] } - fTemp503;
			let mut fTemp505: F64 = 65535.0 * fTemp498;
			let mut iTemp506: i32 = (fTemp505) as i32;
			let mut iTemp507: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp506, 65535)))), 196607));
			let mut fTemp508: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp507, 3), 196607))) as usize] };
			let mut fTemp509: F64 = unsafe { ftbl0mydspSIG0[iTemp507 as usize] };
			let mut fTemp510: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp507, 1), 196607))) as usize] } - fTemp509;
			let mut iTemp511: i32 = (fTemp138 > ((if iTemp63 != 0 {fTemp509 + fTemp77 * fTemp510 + (fTemp505 - (iTemp506) as F64) * (fTemp508 - (fTemp509 + fTemp77 * (fTemp510 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp507, 4), 196607))) as usize] } - fTemp508))))} else {1.0 - (fTemp503 + fTemp77 * fTemp504 + (fTemp499 - (iTemp500) as F64) * (fTemp502 - (fTemp503 + fTemp77 * (fTemp504 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp501, 4)) as usize] } - fTemp502)))))} - fTemp497) / (1.0 - fTemp497))) as i32;
			let mut fTemp512: F64 = if iTemp511 != 0 {fTemp481} else {fTemp484};
			let mut fTemp513: F64 = if iTemp511 != 0 {fTemp484} else {fTemp482};
			let mut fTemp514: F64 = fTemp513 + fTemp512;
			let mut fTemp515: F64 = 0.5 * fTemp514;
			let mut fTemp516: F64 = 65535.0 * (1.0 - fTemp515);
			let mut iTemp517: i32 = (fTemp516) as i32;
			let mut iTemp518: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp517, 65535)))), 196607));
			let mut fTemp519: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp518, 3)) as usize] };
			let mut fTemp520: F64 = unsafe { ftbl0mydspSIG0[iTemp518 as usize] };
			let mut fTemp521: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp518, 1)) as usize] } - fTemp520;
			let mut fTemp522: F64 = 32767.5 * fTemp514;
			let mut iTemp523: i32 = (fTemp522) as i32;
			let mut iTemp524: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp523, 65535)))), 196607));
			let mut fTemp525: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp524, 3)) as usize] };
			let mut fTemp526: F64 = unsafe { ftbl0mydspSIG0[iTemp524 as usize] };
			let mut fTemp527: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp524, 1)) as usize] } - fTemp526;
			let mut fTemp528: F64 = if iTemp63 != 0 {fTemp526 + fTemp77 * fTemp527 + (fTemp522 - (iTemp523) as F64) * (fTemp525 - (fTemp526 + fTemp77 * (fTemp527 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp524, 4)) as usize] } - fTemp525))))} else {1.0 - (fTemp520 + fTemp77 * fTemp521 + (fTemp516 - (iTemp517) as F64) * (fTemp519 - (fTemp520 + fTemp77 * (fTemp521 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp518, 4)) as usize] } - fTemp519)))))};
			let mut fTemp529: F64 = fTemp82 + fTemp515;
			let mut fTemp530: F64 = 65535.0 * (1.0 - fTemp529);
			let mut iTemp531: i32 = (fTemp530) as i32;
			let mut iTemp532: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp531, 65535)))), 196607));
			let mut fTemp533: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp532, 3)) as usize] };
			let mut fTemp534: F64 = unsafe { ftbl0mydspSIG0[iTemp532 as usize] };
			let mut fTemp535: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp532, 1)) as usize] } - fTemp534;
			let mut fTemp536: F64 = 65535.0 * fTemp529;
			let mut iTemp537: i32 = (fTemp536) as i32;
			let mut iTemp538: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp537, 65535)))), 196607));
			let mut fTemp539: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp538, 3), 196607))) as usize] };
			let mut fTemp540: F64 = unsafe { ftbl0mydspSIG0[iTemp538 as usize] };
			let mut fTemp541: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp538, 1), 196607))) as usize] } - fTemp540;
			let mut iTemp542: i32 = (fTemp138 > ((if iTemp63 != 0 {fTemp540 + fTemp77 * fTemp541 + (fTemp536 - (iTemp537) as F64) * (fTemp539 - (fTemp540 + fTemp77 * (fTemp541 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp538, 4), 196607))) as usize] } - fTemp539))))} else {1.0 - (fTemp534 + fTemp77 * fTemp535 + (fTemp530 - (iTemp531) as F64) * (fTemp533 - (fTemp534 + fTemp77 * (fTemp535 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp532, 4)) as usize] } - fTemp533)))))} - fTemp528) / (1.0 - fTemp528))) as i32;
			let mut fTemp543: F64 = if iTemp542 != 0 {fTemp512} else {fTemp515};
			let mut fTemp544: F64 = if iTemp542 != 0 {fTemp515} else {fTemp513};
			let mut fTemp545: F64 = fTemp544 + fTemp543;
			let mut fTemp546: F64 = 0.5 * fTemp545;
			let mut fTemp547: F64 = 65535.0 * (1.0 - fTemp546);
			let mut iTemp548: i32 = (fTemp547) as i32;
			let mut iTemp549: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp548, 65535)))), 196607));
			let mut fTemp550: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp549, 3)) as usize] };
			let mut fTemp551: F64 = unsafe { ftbl0mydspSIG0[iTemp549 as usize] };
			let mut fTemp552: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp549, 1)) as usize] } - fTemp551;
			let mut fTemp553: F64 = 32767.5 * fTemp545;
			let mut iTemp554: i32 = (fTemp553) as i32;
			let mut iTemp555: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp554, 65535)))), 196607));
			let mut fTemp556: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp555, 3)) as usize] };
			let mut fTemp557: F64 = unsafe { ftbl0mydspSIG0[iTemp555 as usize] };
			let mut fTemp558: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp555, 1)) as usize] } - fTemp557;
			let mut fTemp559: F64 = if iTemp63 != 0 {fTemp557 + fTemp77 * fTemp558 + (fTemp553 - (iTemp554) as F64) * (fTemp556 - (fTemp557 + fTemp77 * (fTemp558 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp555, 4)) as usize] } - fTemp556))))} else {1.0 - (fTemp551 + fTemp77 * fTemp552 + (fTemp547 - (iTemp548) as F64) * (fTemp550 - (fTemp551 + fTemp77 * (fTemp552 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp549, 4)) as usize] } - fTemp550)))))};
			let mut fTemp560: F64 = fTemp82 + fTemp546;
			let mut fTemp561: F64 = 65535.0 * (1.0 - fTemp560);
			let mut iTemp562: i32 = (fTemp561) as i32;
			let mut iTemp563: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp562, 65535)))), 196607));
			let mut fTemp564: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp563, 3)) as usize] };
			let mut fTemp565: F64 = unsafe { ftbl0mydspSIG0[iTemp563 as usize] };
			let mut fTemp566: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp563, 1)) as usize] } - fTemp565;
			let mut fTemp567: F64 = 65535.0 * fTemp560;
			let mut iTemp568: i32 = (fTemp567) as i32;
			let mut iTemp569: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp568, 65535)))), 196607));
			let mut fTemp570: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp569, 3), 196607))) as usize] };
			let mut fTemp571: F64 = unsafe { ftbl0mydspSIG0[iTemp569 as usize] };
			let mut fTemp572: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp569, 1), 196607))) as usize] } - fTemp571;
			let mut iTemp573: i32 = (fTemp138 > ((if iTemp63 != 0 {fTemp571 + fTemp77 * fTemp572 + (fTemp567 - (iTemp568) as F64) * (fTemp570 - (fTemp571 + fTemp77 * (fTemp572 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp569, 4), 196607))) as usize] } - fTemp570))))} else {1.0 - (fTemp565 + fTemp77 * fTemp566 + (fTemp561 - (iTemp562) as F64) * (fTemp564 - (fTemp565 + fTemp77 * (fTemp566 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp563, 4)) as usize] } - fTemp564)))))} - fTemp559) / (1.0 - fTemp559))) as i32;
			let mut fTemp574: F64 = if iTemp573 != 0 {fTemp543} else {fTemp546};
			let mut fTemp575: F64 = if iTemp573 != 0 {fTemp546} else {fTemp544};
			let mut fTemp576: F64 = fTemp575 + fTemp574;
			let mut fTemp577: F64 = 0.5 * fTemp576;
			let mut fTemp578: F64 = 65535.0 * (1.0 - fTemp577);
			let mut iTemp579: i32 = (fTemp578) as i32;
			let mut iTemp580: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp579, 65535)))), 196607));
			let mut fTemp581: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp580, 3)) as usize] };
			let mut fTemp582: F64 = unsafe { ftbl0mydspSIG0[iTemp580 as usize] };
			let mut fTemp583: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp580, 1)) as usize] } - fTemp582;
			let mut fTemp584: F64 = 32767.5 * fTemp576;
			let mut iTemp585: i32 = (fTemp584) as i32;
			let mut iTemp586: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp585, 65535)))), 196607));
			let mut fTemp587: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp586, 3)) as usize] };
			let mut fTemp588: F64 = unsafe { ftbl0mydspSIG0[iTemp586 as usize] };
			let mut fTemp589: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp586, 1)) as usize] } - fTemp588;
			let mut fTemp590: F64 = if iTemp63 != 0 {fTemp588 + fTemp77 * fTemp589 + (fTemp584 - (iTemp585) as F64) * (fTemp587 - (fTemp588 + fTemp77 * (fTemp589 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp586, 4), 196607))) as usize] } - fTemp587))))} else {1.0 - (fTemp582 + fTemp77 * fTemp583 + (fTemp578 - (iTemp579) as F64) * (fTemp581 - (fTemp582 + fTemp77 * (fTemp583 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp580, 4), 196607))) as usize] } - fTemp581)))))};
			let mut fTemp591: F64 = fTemp82 + fTemp577;
			let mut fTemp592: F64 = 65535.0 * (1.0 - fTemp591);
			let mut iTemp593: i32 = (fTemp592) as i32;
			let mut iTemp594: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp593, 65535)))), 196607));
			let mut fTemp595: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp594, 3)) as usize] };
			let mut fTemp596: F64 = unsafe { ftbl0mydspSIG0[iTemp594 as usize] };
			let mut fTemp597: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp594, 1)) as usize] } - fTemp596;
			let mut fTemp598: F64 = 65535.0 * fTemp591;
			let mut iTemp599: i32 = (fTemp598) as i32;
			let mut iTemp600: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp599, 65535)))), 196607));
			let mut fTemp601: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp600, 3), 196607))) as usize] };
			let mut fTemp602: F64 = unsafe { ftbl0mydspSIG0[iTemp600 as usize] };
			let mut fTemp603: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp600, 1), 196607))) as usize] } - fTemp602;
			let mut iTemp604: i32 = (fTemp138 > ((if iTemp63 != 0 {fTemp602 + fTemp77 * fTemp603 + (fTemp598 - (iTemp599) as F64) * (fTemp601 - (fTemp602 + fTemp77 * (fTemp603 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp600, 4), 196607))) as usize] } - fTemp601))))} else {1.0 - (fTemp596 + fTemp77 * fTemp597 + (fTemp592 - (iTemp593) as F64) * (fTemp595 - (fTemp596 + fTemp77 * (fTemp597 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp594, 4)) as usize] } - fTemp595)))))} - fTemp590) / (1.0 - fTemp590))) as i32;
			let mut fTemp605: F64 = F64::min(1.0, F64::max(0.0, 0.5 * (if iTemp604 != 0 {fTemp577} else {fTemp575} + if iTemp604 != 0 {fTemp574} else {fTemp577})));
			self.fRec1[0] = fTemp605;
			let mut fTemp606: F64 = 65535.0 * (1.0 - fTemp605);
			let mut iTemp607: i32 = (fTemp606) as i32;
			let mut iTemp608: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp607, 65535)))), 196607));
			let mut fTemp609: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp608, 3)) as usize] };
			let mut fTemp610: F64 = unsafe { ftbl0mydspSIG0[iTemp608 as usize] };
			let mut fTemp611: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp608, 1)) as usize] } - fTemp610;
			let mut fTemp612: F64 = 65535.0 * fTemp605;
			let mut iTemp613: i32 = (fTemp612) as i32;
			let mut iTemp614: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp613, 65535)))), 196607));
			let mut fTemp615: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp614, 3)) as usize] };
			let mut fTemp616: F64 = unsafe { ftbl0mydspSIG0[iTemp614 as usize] };
			let mut fTemp617: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp614, 1)) as usize] } - fTemp616;
			let mut fTemp618: F64 = if iTemp63 != 0 {fTemp616 + fTemp77 * fTemp617 + (fTemp612 - (iTemp613) as F64) * (fTemp615 - (fTemp616 + fTemp77 * (fTemp617 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp614, 4), 196607))) as usize] } - fTemp615))))} else {1.0 - (fTemp610 + fTemp77 * fTemp611 + (fTemp606 - (iTemp607) as F64) * (fTemp609 - (fTemp610 + fTemp77 * (fTemp611 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp608, 4), 196607))) as usize] } - fTemp609)))))};
			let mut fTemp619: F64 = fTemp82 + fTemp605;
			let mut fTemp620: F64 = 65535.0 * (1.0 - fTemp619);
			let mut iTemp621: i32 = (fTemp620) as i32;
			let mut iTemp622: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp621, 65535)))), 196607));
			let mut fTemp623: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp622, 3)) as usize] };
			let mut fTemp624: F64 = unsafe { ftbl0mydspSIG0[iTemp622 as usize] };
			let mut fTemp625: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp622, 1)) as usize] } - fTemp624;
			let mut fTemp626: F64 = 65535.0 * fTemp619;
			let mut iTemp627: i32 = (fTemp626) as i32;
			let mut iTemp628: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp72, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp627, 65535)))), 196607));
			let mut fTemp629: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp628, 3), 196607))) as usize] };
			let mut fTemp630: F64 = unsafe { ftbl0mydspSIG0[iTemp628 as usize] };
			let mut fTemp631: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp628, 1), 196607))) as usize] } - fTemp630;
			let mut fTemp632: F64 = fTemp5 + if ((0.001 * fTemp81) == 0.0) as i32 != 0 {fTemp62} else {fTemp62 * (if iTemp63 != 0 {fTemp630 + fTemp77 * fTemp631 + (fTemp626 - (iTemp627) as F64) * (fTemp629 - (fTemp630 + fTemp77 * (fTemp631 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp628, 4), 196607))) as usize] } - fTemp629))))} else {1.0 - (fTemp624 + fTemp77 * fTemp625 + (fTemp620 - (iTemp621) as F64) * (fTemp623 - (fTemp624 + fTemp77 * (fTemp625 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp622, 4)) as usize] } - fTemp623)))))} - fTemp618) / (1.0 - fTemp618)};
			self.fRec2[(self.IOTA0 & 8191) as usize] = if iTemp80 != 0 {F64::min(fTemp632, fTemp5)} else {F64::max(fTemp632, fTemp5)};
			let mut fTemp633: F64 = self.fRec2[((i32::wrapping_sub(self.IOTA0, iSlow70)) & 8191) as usize];
			self.fHbargraph0 = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, fTemp633));
			self.fRec14[0] = fSlow72 + self.fConst4 * self.fRec14[1];
			*output0 = 0.5 * self.fVec0[((i32::wrapping_sub(self.IOTA0, iSlow71)) & 32767) as usize] * fTemp2 + self.fRec14[0] * self.fVec1[((i32::wrapping_sub(self.IOTA0, iSlow71)) & 32767) as usize] * fTemp633 * fTemp4;
			let mut fTemp634: F64 = self.fRec16[((i32::wrapping_sub(self.IOTA0, 1)) & 8191) as usize];
			let mut fTemp635: F64 = fTemp36 + fSlow17 * (fTemp37 - fTemp36);
			let mut iTemp636: i32 = ((fTemp635 > fSlow11) as i32) + ((fTemp635 > fSlow9) as i32);
			let mut fTemp637: F64 = fTemp635 - fSlow8;
			let mut fTemp638: F64 = F64::min(fTemp34, F64::powf(1e+01, -(fSlow18 * F64::max(0.0, if (iTemp636 == 0) as i32 != 0 {0.0} else {if (iTemp636 == 1) as i32 != 0 {fSlow12 * mydsp_faustpower2_f(fSlow7 + fTemp637)} else {fTemp637}}))));
			self.fVec30[(self.IOTA0 & 16383) as usize] = fTemp638;
			let mut fTemp639: F64 = F64::min(fTemp638, self.fVec30[((i32::wrapping_sub(self.IOTA0, 1)) & 16383) as usize]);
			self.fVec31[0] = fTemp639;
			let mut fTemp640: F64 = F64::min(fTemp639, self.fVec31[2]);
			self.fVec32[0] = fTemp640;
			let mut fTemp641: F64 = F64::min(fTemp640, self.fVec32[4]);
			self.fVec33[0] = fTemp641;
			let mut fTemp642: F64 = F64::min(fTemp641, self.fVec33[8]);
			self.fVec34[(self.IOTA0 & 31) as usize] = fTemp642;
			let mut fTemp643: F64 = F64::min(fTemp642, self.fVec34[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize]);
			self.fVec35[(self.IOTA0 & 63) as usize] = fTemp643;
			let mut fTemp644: F64 = F64::min(fTemp643, self.fVec35[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize]);
			self.fVec36[(self.IOTA0 & 127) as usize] = fTemp644;
			let mut fTemp645: F64 = F64::min(fTemp644, self.fVec36[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize]);
			self.fVec37[(self.IOTA0 & 255) as usize] = fTemp645;
			let mut fTemp646: F64 = F64::min(fTemp645, self.fVec37[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize]);
			self.fVec38[(self.IOTA0 & 511) as usize] = fTemp646;
			let mut fTemp647: F64 = F64::min(fTemp646, self.fVec38[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize]);
			self.fVec39[(self.IOTA0 & 1023) as usize] = fTemp647;
			let mut fTemp648: F64 = F64::min(fTemp647, self.fVec39[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize]);
			self.fVec40[(self.IOTA0 & 2047) as usize] = fTemp648;
			self.fVec41[(self.IOTA0 & 4095) as usize] = F64::min(fTemp648, self.fVec40[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			self.fRec17[0] = F64::max(F64::min(self.fRec17[1], self.fVec30[((i32::wrapping_sub(self.IOTA0, iSlow21)) & 16383) as usize]), F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow23 != 0 {fTemp638} else {1.7976931348623157e+308}, if iSlow24 != 0 {self.fVec31[iSlow23 as usize]} else {1.7976931348623157e+308}), if iSlow25 != 0 {self.fVec32[iSlow26 as usize]} else {1.7976931348623157e+308}), if iSlow27 != 0 {self.fVec33[iSlow28 as usize]} else {1.7976931348623157e+308}), if iSlow29 != 0 {self.fVec34[((i32::wrapping_sub(self.IOTA0, iSlow30)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow31 != 0 {self.fVec35[((i32::wrapping_sub(self.IOTA0, iSlow32)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow33 != 0 {self.fVec36[((i32::wrapping_sub(self.IOTA0, iSlow34)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow35 != 0 {self.fVec37[((i32::wrapping_sub(self.IOTA0, iSlow36)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow37 != 0 {self.fVec38[((i32::wrapping_sub(self.IOTA0, iSlow38)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow39 != 0 {self.fVec39[((i32::wrapping_sub(self.IOTA0, iSlow40)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow41 != 0 {self.fVec40[((i32::wrapping_sub(self.IOTA0, iSlow42)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow43 != 0 {self.fVec41[((i32::wrapping_sub(self.IOTA0, iSlow44)) & 4095) as usize]} else {1.7976931348623157e+308}));
			let mut fTemp649: F64 = F64::min(self.fRec17[0], self.fRec17[1]);
			self.fVec42[0] = fTemp649;
			let mut fTemp650: F64 = F64::min(fTemp649, self.fVec42[2]);
			self.fVec43[0] = fTemp650;
			let mut fTemp651: F64 = F64::min(fTemp650, self.fVec43[4]);
			self.fVec44[0] = fTemp651;
			let mut fTemp652: F64 = F64::min(fTemp651, self.fVec44[8]);
			self.fVec45[(self.IOTA0 & 31) as usize] = fTemp652;
			let mut fTemp653: F64 = F64::min(fTemp652, self.fVec45[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize]);
			self.fVec46[(self.IOTA0 & 63) as usize] = fTemp653;
			let mut fTemp654: F64 = F64::min(fTemp653, self.fVec46[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize]);
			self.fVec47[(self.IOTA0 & 127) as usize] = fTemp654;
			let mut fTemp655: F64 = F64::min(fTemp654, self.fVec47[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize]);
			self.fVec48[(self.IOTA0 & 255) as usize] = fTemp655;
			let mut fTemp656: F64 = F64::min(fTemp655, self.fVec48[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize]);
			self.fVec49[(self.IOTA0 & 511) as usize] = fTemp656;
			let mut fTemp657: F64 = F64::min(fTemp656, self.fVec49[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize]);
			self.fVec50[(self.IOTA0 & 1023) as usize] = fTemp657;
			let mut fTemp658: F64 = F64::min(fTemp657, self.fVec50[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize]);
			self.fVec51[(self.IOTA0 & 2047) as usize] = fTemp658;
			self.fVec52[(self.IOTA0 & 4095) as usize] = F64::min(fTemp658, self.fVec51[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			let mut fTemp659: F64 = F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow4 != 0 {self.fRec17[0]} else {1.7976931348623157e+308}, if iSlow45 != 0 {self.fVec42[iSlow4 as usize]} else {1.7976931348623157e+308}), if iSlow46 != 0 {self.fVec43[iSlow47 as usize]} else {1.7976931348623157e+308}), if iSlow48 != 0 {self.fVec44[iSlow49 as usize]} else {1.7976931348623157e+308}), if iSlow50 != 0 {self.fVec45[((i32::wrapping_sub(self.IOTA0, iSlow51)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow52 != 0 {self.fVec46[((i32::wrapping_sub(self.IOTA0, iSlow53)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow54 != 0 {self.fVec47[((i32::wrapping_sub(self.IOTA0, iSlow55)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow56 != 0 {self.fVec48[((i32::wrapping_sub(self.IOTA0, iSlow57)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow58 != 0 {self.fVec49[((i32::wrapping_sub(self.IOTA0, iSlow59)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow60 != 0 {self.fVec50[((i32::wrapping_sub(self.IOTA0, iSlow61)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow62 != 0 {self.fVec51[((i32::wrapping_sub(self.IOTA0, iSlow63)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow64 != 0 {self.fVec52[((i32::wrapping_sub(self.IOTA0, iSlow65)) & 4095) as usize]} else {1.7976931348623157e+308}) - fTemp634;
			self.fVec53[0] = fTemp659;
			let mut iTemp660: i32 = (fTemp659 > 0.0) as i32;
			let mut fTemp661: F64 = if iTemp660 != 0 {fSlow67} else {fSlow66};
			self.fVec54[0] = fTemp661;
			let mut fTemp662: F64 = 2.0 * fTemp661;
			let mut iTemp663: i32 = (fTemp662) as i32;
			let mut iTemp664: i32 = std::cmp::max(0, std::cmp::min(iTemp663, 2));
			let mut iTemp665: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, 98301), 196607));
			let mut fTemp666: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp665, 3)) as usize] };
			let mut fTemp667: F64 = unsafe { ftbl0mydspSIG0[iTemp665 as usize] };
			let mut fTemp668: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp665, 1)) as usize] } - fTemp667;
			let mut fTemp669: F64 = fTemp662 - (iTemp663) as F64;
			let mut fTemp670: F64 = fTemp667 + fTemp669 * fTemp668 + 0.5 * (fTemp666 - (fTemp667 + fTemp669 * (fTemp668 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp665, 4)) as usize] } - fTemp666))));
			let mut fTemp671: F64 = if iTemp660 != 0 {fTemp670} else {1.0 - fTemp670};
			let mut iTemp672: i32 = (fTemp659 < 0.0) as i32;
			let mut fTemp673: F64 = fSlow1 * (iTemp672) as F64 + fSlow13 * (iTemp660) as F64;
			self.fVec55[0] = fTemp673;
			let mut fTemp674: F64 = self.fConst10 / fTemp673;
			let mut fTemp675: F64 = fTemp674 + 0.5;
			let mut fTemp676: F64 = 65535.0 * (1.0 - fTemp675);
			let mut iTemp677: i32 = (fTemp676) as i32;
			let mut iTemp678: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp677, 65535)))), 196607));
			let mut fTemp679: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp678, 3)) as usize] };
			let mut fTemp680: F64 = unsafe { ftbl0mydspSIG0[iTemp678 as usize] };
			let mut fTemp681: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp678, 1)) as usize] } - fTemp680;
			let mut fTemp682: F64 = 65535.0 * fTemp675;
			let mut iTemp683: i32 = (fTemp682) as i32;
			let mut iTemp684: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp683, 65535)))), 196607));
			let mut fTemp685: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp684, 3), 196607))) as usize] };
			let mut fTemp686: F64 = unsafe { ftbl0mydspSIG0[iTemp684 as usize] };
			let mut fTemp687: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp684, 1), 196607))) as usize] } - fTemp686;
			let mut fTemp688: F64 = 2.0 * self.fVec54[1];
			let mut iTemp689: i32 = (fTemp688) as i32;
			let mut iTemp690: i32 = std::cmp::max(0, std::cmp::min(iTemp689, 2));
			let mut fTemp691: F64 = 65535.0 * (1.0 - self.fRec15[1]);
			let mut iTemp692: i32 = (fTemp691) as i32;
			let mut iTemp693: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp692, 65535))), iTemp690), 196607));
			let mut fTemp694: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp693, 3), 196607))) as usize] };
			let mut fTemp695: F64 = unsafe { ftbl0mydspSIG0[iTemp693 as usize] };
			let mut fTemp696: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp693, 1), 196607))) as usize] } - fTemp695;
			let mut fTemp697: F64 = fTemp688 - (iTemp689) as F64;
			let mut fTemp698: F64 = 65535.0 * self.fRec15[1];
			let mut iTemp699: i32 = (fTemp698) as i32;
			let mut iTemp700: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp690, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp699, 65535)))), 196607));
			let mut fTemp701: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp700, 3), 196607))) as usize] };
			let mut fTemp702: F64 = unsafe { ftbl0mydspSIG0[iTemp700 as usize] };
			let mut fTemp703: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp700, 1), 196607))) as usize] } - fTemp702;
			let mut fTemp704: F64 = self.fRec15[1] + fTemp674;
			let mut fTemp705: F64 = 65535.0 * (1.0 - fTemp704);
			let mut iTemp706: i32 = (fTemp705) as i32;
			let mut iTemp707: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp706, 65535)))), 196607));
			let mut fTemp708: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp707, 3)) as usize] };
			let mut fTemp709: F64 = unsafe { ftbl0mydspSIG0[iTemp707 as usize] };
			let mut fTemp710: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp707, 1)) as usize] } - fTemp709;
			let mut fTemp711: F64 = 65535.0 * fTemp704;
			let mut iTemp712: i32 = (fTemp711) as i32;
			let mut iTemp713: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp712, 65535)))), 196607));
			let mut fTemp714: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp713, 3), 196607))) as usize] };
			let mut fTemp715: F64 = unsafe { ftbl0mydspSIG0[iTemp713 as usize] };
			let mut fTemp716: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp713, 1), 196607))) as usize] } - fTemp715;
			let mut fTemp717: F64 = self.fRec15[1] + self.fConst10 * (1.0 / fTemp673 + 1.0 / self.fVec55[1]);
			let mut fTemp718: F64 = 65535.0 * (1.0 - fTemp717);
			let mut iTemp719: i32 = (fTemp718) as i32;
			let mut iTemp720: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp719, 65535))), iTemp664), 196607));
			let mut fTemp721: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp720, 3)) as usize] };
			let mut fTemp722: F64 = unsafe { ftbl0mydspSIG0[iTemp720 as usize] };
			let mut fTemp723: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp720, 1)) as usize] } - fTemp722;
			let mut fTemp724: F64 = 65535.0 * fTemp717;
			let mut iTemp725: i32 = (fTemp724) as i32;
			let mut iTemp726: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp725, 65535)))), 196607));
			let mut fTemp727: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp726, 3), 196607))) as usize] };
			let mut fTemp728: F64 = unsafe { ftbl0mydspSIG0[iTemp726 as usize] };
			let mut fTemp729: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp726, 1), 196607))) as usize] } - fTemp728;
			let mut fTemp730: F64 = (if iTemp660 != 0 {fTemp728 + fTemp669 * fTemp729 + (fTemp724 - (iTemp725) as F64) * (fTemp727 - (fTemp728 + fTemp669 * (fTemp729 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp726, 4), 196607))) as usize] } - fTemp727))))} else {1.0 - (fTemp722 + fTemp669 * fTemp723 + (fTemp718 - (iTemp719) as F64) * (fTemp721 - (fTemp722 + fTemp669 * (fTemp723 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp720, 4)) as usize] } - fTemp721)))))} - if iTemp660 != 0 {fTemp715 + fTemp669 * fTemp716 + (fTemp711 - (iTemp712) as F64) * (fTemp714 - (fTemp715 + fTemp669 * (fTemp716 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp713, 4), 196607))) as usize] } - fTemp714))))} else {1.0 - (fTemp709 + fTemp669 * fTemp710 + (fTemp705 - (iTemp706) as F64) * (fTemp708 - (fTemp709 + fTemp669 * (fTemp710 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp707, 4), 196607))) as usize] } - fTemp708)))))}) * self.fVec53[1] / (fTemp659 * (1.0 - if iTemp660 != 0 {fTemp702 + fTemp697 * fTemp703 + (fTemp698 - (iTemp699) as F64) * (fTemp701 - (fTemp702 + fTemp697 * (fTemp703 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp700, 4), 196607))) as usize] } - fTemp701))))} else {1.0 - (fTemp695 + fTemp697 * fTemp696 + (fTemp691 - (iTemp692) as F64) * (fTemp694 - (fTemp695 + fTemp697 * (fTemp696 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp693, 4), 196607))) as usize] } - fTemp694)))))}));
			let mut iTemp731: i32 = (fTemp730 > ((if iTemp660 != 0 {fTemp686 + fTemp669 * fTemp687 + (fTemp682 - (iTemp683) as F64) * (fTemp685 - (fTemp686 + fTemp669 * (fTemp687 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp684, 4), 196607))) as usize] } - fTemp685))))} else {1.0 - (fTemp680 + fTemp669 * fTemp681 + (fTemp676 - (iTemp677) as F64) * (fTemp679 - (fTemp680 + fTemp669 * (fTemp681 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp678, 4)) as usize] } - fTemp679)))))} - fTemp671) / (1.0 - fTemp671))) as i32;
			let mut fTemp732: F64 = if iTemp731 != 0 {1.0} else {0.5};
			let mut fTemp733: F64 = if iTemp731 != 0 {0.5} else {0.0};
			let mut fTemp734: F64 = fTemp733 + fTemp732;
			let mut fTemp735: F64 = 0.5 * fTemp734;
			let mut fTemp736: F64 = 65535.0 * (1.0 - fTemp735);
			let mut iTemp737: i32 = (fTemp736) as i32;
			let mut iTemp738: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp737, 65535)))), 196607));
			let mut fTemp739: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp738, 3)) as usize] };
			let mut fTemp740: F64 = unsafe { ftbl0mydspSIG0[iTemp738 as usize] };
			let mut fTemp741: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp738, 1)) as usize] } - fTemp740;
			let mut fTemp742: F64 = 32767.5 * fTemp734;
			let mut iTemp743: i32 = (fTemp742) as i32;
			let mut iTemp744: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp743, 65535)))), 196607));
			let mut fTemp745: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp744, 3)) as usize] };
			let mut fTemp746: F64 = unsafe { ftbl0mydspSIG0[iTemp744 as usize] };
			let mut fTemp747: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp744, 1)) as usize] } - fTemp746;
			let mut fTemp748: F64 = if iTemp660 != 0 {fTemp746 + fTemp669 * fTemp747 + (fTemp742 - (iTemp743) as F64) * (fTemp745 - (fTemp746 + fTemp669 * (fTemp747 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp744, 4)) as usize] } - fTemp745))))} else {1.0 - (fTemp740 + fTemp669 * fTemp741 + (fTemp736 - (iTemp737) as F64) * (fTemp739 - (fTemp740 + fTemp669 * (fTemp741 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp738, 4)) as usize] } - fTemp739)))))};
			let mut fTemp749: F64 = fTemp674 + fTemp735;
			let mut fTemp750: F64 = 65535.0 * (1.0 - fTemp749);
			let mut iTemp751: i32 = (fTemp750) as i32;
			let mut iTemp752: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp751, 65535)))), 196607));
			let mut fTemp753: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp752, 3)) as usize] };
			let mut fTemp754: F64 = unsafe { ftbl0mydspSIG0[iTemp752 as usize] };
			let mut fTemp755: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp752, 1)) as usize] } - fTemp754;
			let mut fTemp756: F64 = 65535.0 * fTemp749;
			let mut iTemp757: i32 = (fTemp756) as i32;
			let mut iTemp758: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp757, 65535)))), 196607));
			let mut fTemp759: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp758, 3), 196607))) as usize] };
			let mut fTemp760: F64 = unsafe { ftbl0mydspSIG0[iTemp758 as usize] };
			let mut fTemp761: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp758, 1), 196607))) as usize] } - fTemp760;
			let mut iTemp762: i32 = (fTemp730 > ((if iTemp660 != 0 {fTemp760 + fTemp669 * fTemp761 + (fTemp756 - (iTemp757) as F64) * (fTemp759 - (fTemp760 + fTemp669 * (fTemp761 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp758, 4), 196607))) as usize] } - fTemp759))))} else {1.0 - (fTemp754 + fTemp669 * fTemp755 + (fTemp750 - (iTemp751) as F64) * (fTemp753 - (fTemp754 + fTemp669 * (fTemp755 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp752, 4)) as usize] } - fTemp753)))))} - fTemp748) / (1.0 - fTemp748))) as i32;
			let mut fTemp763: F64 = if iTemp762 != 0 {fTemp732} else {fTemp735};
			let mut fTemp764: F64 = if iTemp762 != 0 {fTemp735} else {fTemp733};
			let mut fTemp765: F64 = fTemp764 + fTemp763;
			let mut fTemp766: F64 = 0.5 * fTemp765;
			let mut fTemp767: F64 = 65535.0 * (1.0 - fTemp766);
			let mut iTemp768: i32 = (fTemp767) as i32;
			let mut iTemp769: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp768, 65535)))), 196607));
			let mut fTemp770: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp769, 3)) as usize] };
			let mut fTemp771: F64 = unsafe { ftbl0mydspSIG0[iTemp769 as usize] };
			let mut fTemp772: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp769, 1)) as usize] } - fTemp771;
			let mut fTemp773: F64 = 32767.5 * fTemp765;
			let mut iTemp774: i32 = (fTemp773) as i32;
			let mut iTemp775: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp774, 65535)))), 196607));
			let mut fTemp776: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp775, 3)) as usize] };
			let mut fTemp777: F64 = unsafe { ftbl0mydspSIG0[iTemp775 as usize] };
			let mut fTemp778: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp775, 1)) as usize] } - fTemp777;
			let mut fTemp779: F64 = if iTemp660 != 0 {fTemp777 + fTemp669 * fTemp778 + (fTemp773 - (iTemp774) as F64) * (fTemp776 - (fTemp777 + fTemp669 * (fTemp778 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp775, 4)) as usize] } - fTemp776))))} else {1.0 - (fTemp771 + fTemp669 * fTemp772 + (fTemp767 - (iTemp768) as F64) * (fTemp770 - (fTemp771 + fTemp669 * (fTemp772 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp769, 4)) as usize] } - fTemp770)))))};
			let mut fTemp780: F64 = fTemp674 + fTemp766;
			let mut fTemp781: F64 = 65535.0 * (1.0 - fTemp780);
			let mut iTemp782: i32 = (fTemp781) as i32;
			let mut iTemp783: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp782, 65535)))), 196607));
			let mut fTemp784: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp783, 3)) as usize] };
			let mut fTemp785: F64 = unsafe { ftbl0mydspSIG0[iTemp783 as usize] };
			let mut fTemp786: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp783, 1)) as usize] } - fTemp785;
			let mut fTemp787: F64 = 65535.0 * fTemp780;
			let mut iTemp788: i32 = (fTemp787) as i32;
			let mut iTemp789: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp788, 65535)))), 196607));
			let mut fTemp790: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp789, 3), 196607))) as usize] };
			let mut fTemp791: F64 = unsafe { ftbl0mydspSIG0[iTemp789 as usize] };
			let mut fTemp792: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp789, 1), 196607))) as usize] } - fTemp791;
			let mut iTemp793: i32 = (fTemp730 > ((if iTemp660 != 0 {fTemp791 + fTemp669 * fTemp792 + (fTemp787 - (iTemp788) as F64) * (fTemp790 - (fTemp791 + fTemp669 * (fTemp792 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp789, 4), 196607))) as usize] } - fTemp790))))} else {1.0 - (fTemp785 + fTemp669 * fTemp786 + (fTemp781 - (iTemp782) as F64) * (fTemp784 - (fTemp785 + fTemp669 * (fTemp786 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp783, 4)) as usize] } - fTemp784)))))} - fTemp779) / (1.0 - fTemp779))) as i32;
			let mut fTemp794: F64 = if iTemp793 != 0 {fTemp763} else {fTemp766};
			let mut fTemp795: F64 = if iTemp793 != 0 {fTemp766} else {fTemp764};
			let mut fTemp796: F64 = fTemp795 + fTemp794;
			let mut fTemp797: F64 = 0.5 * fTemp796;
			let mut fTemp798: F64 = 65535.0 * (1.0 - fTemp797);
			let mut iTemp799: i32 = (fTemp798) as i32;
			let mut iTemp800: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp799, 65535)))), 196607));
			let mut fTemp801: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp800, 3)) as usize] };
			let mut fTemp802: F64 = unsafe { ftbl0mydspSIG0[iTemp800 as usize] };
			let mut fTemp803: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp800, 1)) as usize] } - fTemp802;
			let mut fTemp804: F64 = 32767.5 * fTemp796;
			let mut iTemp805: i32 = (fTemp804) as i32;
			let mut iTemp806: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp805, 65535)))), 196607));
			let mut fTemp807: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp806, 3)) as usize] };
			let mut fTemp808: F64 = unsafe { ftbl0mydspSIG0[iTemp806 as usize] };
			let mut fTemp809: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp806, 1)) as usize] } - fTemp808;
			let mut fTemp810: F64 = if iTemp660 != 0 {fTemp808 + fTemp669 * fTemp809 + (fTemp804 - (iTemp805) as F64) * (fTemp807 - (fTemp808 + fTemp669 * (fTemp809 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp806, 4)) as usize] } - fTemp807))))} else {1.0 - (fTemp802 + fTemp669 * fTemp803 + (fTemp798 - (iTemp799) as F64) * (fTemp801 - (fTemp802 + fTemp669 * (fTemp803 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp800, 4)) as usize] } - fTemp801)))))};
			let mut fTemp811: F64 = fTemp674 + fTemp797;
			let mut fTemp812: F64 = 65535.0 * (1.0 - fTemp811);
			let mut iTemp813: i32 = (fTemp812) as i32;
			let mut iTemp814: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp813, 65535)))), 196607));
			let mut fTemp815: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp814, 3)) as usize] };
			let mut fTemp816: F64 = unsafe { ftbl0mydspSIG0[iTemp814 as usize] };
			let mut fTemp817: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp814, 1)) as usize] } - fTemp816;
			let mut fTemp818: F64 = 65535.0 * fTemp811;
			let mut iTemp819: i32 = (fTemp818) as i32;
			let mut iTemp820: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp819, 65535)))), 196607));
			let mut fTemp821: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp820, 3), 196607))) as usize] };
			let mut fTemp822: F64 = unsafe { ftbl0mydspSIG0[iTemp820 as usize] };
			let mut fTemp823: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp820, 1), 196607))) as usize] } - fTemp822;
			let mut iTemp824: i32 = (fTemp730 > ((if iTemp660 != 0 {fTemp822 + fTemp669 * fTemp823 + (fTemp818 - (iTemp819) as F64) * (fTemp821 - (fTemp822 + fTemp669 * (fTemp823 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp820, 4), 196607))) as usize] } - fTemp821))))} else {1.0 - (fTemp816 + fTemp669 * fTemp817 + (fTemp812 - (iTemp813) as F64) * (fTemp815 - (fTemp816 + fTemp669 * (fTemp817 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp814, 4)) as usize] } - fTemp815)))))} - fTemp810) / (1.0 - fTemp810))) as i32;
			let mut fTemp825: F64 = if iTemp824 != 0 {fTemp794} else {fTemp797};
			let mut fTemp826: F64 = if iTemp824 != 0 {fTemp797} else {fTemp795};
			let mut fTemp827: F64 = fTemp826 + fTemp825;
			let mut fTemp828: F64 = 0.5 * fTemp827;
			let mut fTemp829: F64 = 65535.0 * (1.0 - fTemp828);
			let mut iTemp830: i32 = (fTemp829) as i32;
			let mut iTemp831: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp830, 65535)))), 196607));
			let mut fTemp832: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp831, 3)) as usize] };
			let mut fTemp833: F64 = unsafe { ftbl0mydspSIG0[iTemp831 as usize] };
			let mut fTemp834: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp831, 1)) as usize] } - fTemp833;
			let mut fTemp835: F64 = 32767.5 * fTemp827;
			let mut iTemp836: i32 = (fTemp835) as i32;
			let mut iTemp837: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp836, 65535)))), 196607));
			let mut fTemp838: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp837, 3)) as usize] };
			let mut fTemp839: F64 = unsafe { ftbl0mydspSIG0[iTemp837 as usize] };
			let mut fTemp840: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp837, 1)) as usize] } - fTemp839;
			let mut fTemp841: F64 = if iTemp660 != 0 {fTemp839 + fTemp669 * fTemp840 + (fTemp835 - (iTemp836) as F64) * (fTemp838 - (fTemp839 + fTemp669 * (fTemp840 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp837, 4)) as usize] } - fTemp838))))} else {1.0 - (fTemp833 + fTemp669 * fTemp834 + (fTemp829 - (iTemp830) as F64) * (fTemp832 - (fTemp833 + fTemp669 * (fTemp834 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp831, 4)) as usize] } - fTemp832)))))};
			let mut fTemp842: F64 = fTemp674 + fTemp828;
			let mut fTemp843: F64 = 65535.0 * (1.0 - fTemp842);
			let mut iTemp844: i32 = (fTemp843) as i32;
			let mut iTemp845: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp844, 65535)))), 196607));
			let mut fTemp846: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp845, 3)) as usize] };
			let mut fTemp847: F64 = unsafe { ftbl0mydspSIG0[iTemp845 as usize] };
			let mut fTemp848: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp845, 1)) as usize] } - fTemp847;
			let mut fTemp849: F64 = 65535.0 * fTemp842;
			let mut iTemp850: i32 = (fTemp849) as i32;
			let mut iTemp851: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp850, 65535)))), 196607));
			let mut fTemp852: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp851, 3), 196607))) as usize] };
			let mut fTemp853: F64 = unsafe { ftbl0mydspSIG0[iTemp851 as usize] };
			let mut fTemp854: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp851, 1), 196607))) as usize] } - fTemp853;
			let mut iTemp855: i32 = (fTemp730 > ((if iTemp660 != 0 {fTemp853 + fTemp669 * fTemp854 + (fTemp849 - (iTemp850) as F64) * (fTemp852 - (fTemp853 + fTemp669 * (fTemp854 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp851, 4), 196607))) as usize] } - fTemp852))))} else {1.0 - (fTemp847 + fTemp669 * fTemp848 + (fTemp843 - (iTemp844) as F64) * (fTemp846 - (fTemp847 + fTemp669 * (fTemp848 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp845, 4)) as usize] } - fTemp846)))))} - fTemp841) / (1.0 - fTemp841))) as i32;
			let mut fTemp856: F64 = if iTemp855 != 0 {fTemp825} else {fTemp828};
			let mut fTemp857: F64 = if iTemp855 != 0 {fTemp828} else {fTemp826};
			let mut fTemp858: F64 = fTemp857 + fTemp856;
			let mut fTemp859: F64 = 0.5 * fTemp858;
			let mut fTemp860: F64 = 65535.0 * (1.0 - fTemp859);
			let mut iTemp861: i32 = (fTemp860) as i32;
			let mut iTemp862: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp861, 65535)))), 196607));
			let mut fTemp863: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp862, 3)) as usize] };
			let mut fTemp864: F64 = unsafe { ftbl0mydspSIG0[iTemp862 as usize] };
			let mut fTemp865: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp862, 1)) as usize] } - fTemp864;
			let mut fTemp866: F64 = 32767.5 * fTemp858;
			let mut iTemp867: i32 = (fTemp866) as i32;
			let mut iTemp868: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp867, 65535)))), 196607));
			let mut fTemp869: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp868, 3)) as usize] };
			let mut fTemp870: F64 = unsafe { ftbl0mydspSIG0[iTemp868 as usize] };
			let mut fTemp871: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp868, 1)) as usize] } - fTemp870;
			let mut fTemp872: F64 = if iTemp660 != 0 {fTemp870 + fTemp669 * fTemp871 + (fTemp866 - (iTemp867) as F64) * (fTemp869 - (fTemp870 + fTemp669 * (fTemp871 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp868, 4)) as usize] } - fTemp869))))} else {1.0 - (fTemp864 + fTemp669 * fTemp865 + (fTemp860 - (iTemp861) as F64) * (fTemp863 - (fTemp864 + fTemp669 * (fTemp865 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp862, 4)) as usize] } - fTemp863)))))};
			let mut fTemp873: F64 = fTemp674 + fTemp859;
			let mut fTemp874: F64 = 65535.0 * (1.0 - fTemp873);
			let mut iTemp875: i32 = (fTemp874) as i32;
			let mut iTemp876: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp875, 65535)))), 196607));
			let mut fTemp877: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp876, 3)) as usize] };
			let mut fTemp878: F64 = unsafe { ftbl0mydspSIG0[iTemp876 as usize] };
			let mut fTemp879: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp876, 1)) as usize] } - fTemp878;
			let mut fTemp880: F64 = 65535.0 * fTemp873;
			let mut iTemp881: i32 = (fTemp880) as i32;
			let mut iTemp882: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp881, 65535)))), 196607));
			let mut fTemp883: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp882, 3), 196607))) as usize] };
			let mut fTemp884: F64 = unsafe { ftbl0mydspSIG0[iTemp882 as usize] };
			let mut fTemp885: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp882, 1), 196607))) as usize] } - fTemp884;
			let mut iTemp886: i32 = (fTemp730 > ((if iTemp660 != 0 {fTemp884 + fTemp669 * fTemp885 + (fTemp880 - (iTemp881) as F64) * (fTemp883 - (fTemp884 + fTemp669 * (fTemp885 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp882, 4), 196607))) as usize] } - fTemp883))))} else {1.0 - (fTemp878 + fTemp669 * fTemp879 + (fTemp874 - (iTemp875) as F64) * (fTemp877 - (fTemp878 + fTemp669 * (fTemp879 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp876, 4)) as usize] } - fTemp877)))))} - fTemp872) / (1.0 - fTemp872))) as i32;
			let mut fTemp887: F64 = if iTemp886 != 0 {fTemp856} else {fTemp859};
			let mut fTemp888: F64 = if iTemp886 != 0 {fTemp859} else {fTemp857};
			let mut fTemp889: F64 = fTemp888 + fTemp887;
			let mut fTemp890: F64 = 0.5 * fTemp889;
			let mut fTemp891: F64 = 65535.0 * (1.0 - fTemp890);
			let mut iTemp892: i32 = (fTemp891) as i32;
			let mut iTemp893: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp892, 65535)))), 196607));
			let mut fTemp894: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp893, 3)) as usize] };
			let mut fTemp895: F64 = unsafe { ftbl0mydspSIG0[iTemp893 as usize] };
			let mut fTemp896: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp893, 1)) as usize] } - fTemp895;
			let mut fTemp897: F64 = 32767.5 * fTemp889;
			let mut iTemp898: i32 = (fTemp897) as i32;
			let mut iTemp899: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp898, 65535)))), 196607));
			let mut fTemp900: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp899, 3)) as usize] };
			let mut fTemp901: F64 = unsafe { ftbl0mydspSIG0[iTemp899 as usize] };
			let mut fTemp902: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp899, 1)) as usize] } - fTemp901;
			let mut fTemp903: F64 = if iTemp660 != 0 {fTemp901 + fTemp669 * fTemp902 + (fTemp897 - (iTemp898) as F64) * (fTemp900 - (fTemp901 + fTemp669 * (fTemp902 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp899, 4)) as usize] } - fTemp900))))} else {1.0 - (fTemp895 + fTemp669 * fTemp896 + (fTemp891 - (iTemp892) as F64) * (fTemp894 - (fTemp895 + fTemp669 * (fTemp896 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp893, 4)) as usize] } - fTemp894)))))};
			let mut fTemp904: F64 = fTemp674 + fTemp890;
			let mut fTemp905: F64 = 65535.0 * (1.0 - fTemp904);
			let mut iTemp906: i32 = (fTemp905) as i32;
			let mut iTemp907: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp906, 65535)))), 196607));
			let mut fTemp908: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp907, 3)) as usize] };
			let mut fTemp909: F64 = unsafe { ftbl0mydspSIG0[iTemp907 as usize] };
			let mut fTemp910: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp907, 1)) as usize] } - fTemp909;
			let mut fTemp911: F64 = 65535.0 * fTemp904;
			let mut iTemp912: i32 = (fTemp911) as i32;
			let mut iTemp913: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp912, 65535)))), 196607));
			let mut fTemp914: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp913, 3), 196607))) as usize] };
			let mut fTemp915: F64 = unsafe { ftbl0mydspSIG0[iTemp913 as usize] };
			let mut fTemp916: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp913, 1), 196607))) as usize] } - fTemp915;
			let mut iTemp917: i32 = (fTemp730 > ((if iTemp660 != 0 {fTemp915 + fTemp669 * fTemp916 + (fTemp911 - (iTemp912) as F64) * (fTemp914 - (fTemp915 + fTemp669 * (fTemp916 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp913, 4), 196607))) as usize] } - fTemp914))))} else {1.0 - (fTemp909 + fTemp669 * fTemp910 + (fTemp905 - (iTemp906) as F64) * (fTemp908 - (fTemp909 + fTemp669 * (fTemp910 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp907, 4)) as usize] } - fTemp908)))))} - fTemp903) / (1.0 - fTemp903))) as i32;
			let mut fTemp918: F64 = if iTemp917 != 0 {fTemp887} else {fTemp890};
			let mut fTemp919: F64 = if iTemp917 != 0 {fTemp890} else {fTemp888};
			let mut fTemp920: F64 = fTemp919 + fTemp918;
			let mut fTemp921: F64 = 0.5 * fTemp920;
			let mut fTemp922: F64 = 65535.0 * (1.0 - fTemp921);
			let mut iTemp923: i32 = (fTemp922) as i32;
			let mut iTemp924: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp923, 65535)))), 196607));
			let mut fTemp925: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp924, 3)) as usize] };
			let mut fTemp926: F64 = unsafe { ftbl0mydspSIG0[iTemp924 as usize] };
			let mut fTemp927: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp924, 1)) as usize] } - fTemp926;
			let mut fTemp928: F64 = 32767.5 * fTemp920;
			let mut iTemp929: i32 = (fTemp928) as i32;
			let mut iTemp930: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp929, 65535)))), 196607));
			let mut fTemp931: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp930, 3)) as usize] };
			let mut fTemp932: F64 = unsafe { ftbl0mydspSIG0[iTemp930 as usize] };
			let mut fTemp933: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp930, 1)) as usize] } - fTemp932;
			let mut fTemp934: F64 = if iTemp660 != 0 {fTemp932 + fTemp669 * fTemp933 + (fTemp928 - (iTemp929) as F64) * (fTemp931 - (fTemp932 + fTemp669 * (fTemp933 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp930, 4)) as usize] } - fTemp931))))} else {1.0 - (fTemp926 + fTemp669 * fTemp927 + (fTemp922 - (iTemp923) as F64) * (fTemp925 - (fTemp926 + fTemp669 * (fTemp927 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp924, 4)) as usize] } - fTemp925)))))};
			let mut fTemp935: F64 = fTemp674 + fTemp921;
			let mut fTemp936: F64 = 65535.0 * (1.0 - fTemp935);
			let mut iTemp937: i32 = (fTemp936) as i32;
			let mut iTemp938: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp937, 65535)))), 196607));
			let mut fTemp939: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp938, 3)) as usize] };
			let mut fTemp940: F64 = unsafe { ftbl0mydspSIG0[iTemp938 as usize] };
			let mut fTemp941: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp938, 1)) as usize] } - fTemp940;
			let mut fTemp942: F64 = 65535.0 * fTemp935;
			let mut iTemp943: i32 = (fTemp942) as i32;
			let mut iTemp944: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp943, 65535)))), 196607));
			let mut fTemp945: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp944, 3), 196607))) as usize] };
			let mut fTemp946: F64 = unsafe { ftbl0mydspSIG0[iTemp944 as usize] };
			let mut fTemp947: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp944, 1), 196607))) as usize] } - fTemp946;
			let mut iTemp948: i32 = (fTemp730 > ((if iTemp660 != 0 {fTemp946 + fTemp669 * fTemp947 + (fTemp942 - (iTemp943) as F64) * (fTemp945 - (fTemp946 + fTemp669 * (fTemp947 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp944, 4), 196607))) as usize] } - fTemp945))))} else {1.0 - (fTemp940 + fTemp669 * fTemp941 + (fTemp936 - (iTemp937) as F64) * (fTemp939 - (fTemp940 + fTemp669 * (fTemp941 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp938, 4)) as usize] } - fTemp939)))))} - fTemp934) / (1.0 - fTemp934))) as i32;
			let mut fTemp949: F64 = if iTemp948 != 0 {fTemp918} else {fTemp921};
			let mut fTemp950: F64 = if iTemp948 != 0 {fTemp921} else {fTemp919};
			let mut fTemp951: F64 = fTemp950 + fTemp949;
			let mut fTemp952: F64 = 0.5 * fTemp951;
			let mut fTemp953: F64 = 65535.0 * (1.0 - fTemp952);
			let mut iTemp954: i32 = (fTemp953) as i32;
			let mut iTemp955: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp954, 65535)))), 196607));
			let mut fTemp956: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp955, 3)) as usize] };
			let mut fTemp957: F64 = unsafe { ftbl0mydspSIG0[iTemp955 as usize] };
			let mut fTemp958: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp955, 1)) as usize] } - fTemp957;
			let mut fTemp959: F64 = 32767.5 * fTemp951;
			let mut iTemp960: i32 = (fTemp959) as i32;
			let mut iTemp961: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp960, 65535)))), 196607));
			let mut fTemp962: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp961, 3)) as usize] };
			let mut fTemp963: F64 = unsafe { ftbl0mydspSIG0[iTemp961 as usize] };
			let mut fTemp964: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp961, 1)) as usize] } - fTemp963;
			let mut fTemp965: F64 = if iTemp660 != 0 {fTemp963 + fTemp669 * fTemp964 + (fTemp959 - (iTemp960) as F64) * (fTemp962 - (fTemp963 + fTemp669 * (fTemp964 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp961, 4)) as usize] } - fTemp962))))} else {1.0 - (fTemp957 + fTemp669 * fTemp958 + (fTemp953 - (iTemp954) as F64) * (fTemp956 - (fTemp957 + fTemp669 * (fTemp958 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp955, 4)) as usize] } - fTemp956)))))};
			let mut fTemp966: F64 = fTemp674 + fTemp952;
			let mut fTemp967: F64 = 65535.0 * (1.0 - fTemp966);
			let mut iTemp968: i32 = (fTemp967) as i32;
			let mut iTemp969: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp968, 65535)))), 196607));
			let mut fTemp970: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp969, 3)) as usize] };
			let mut fTemp971: F64 = unsafe { ftbl0mydspSIG0[iTemp969 as usize] };
			let mut fTemp972: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp969, 1)) as usize] } - fTemp971;
			let mut fTemp973: F64 = 65535.0 * fTemp966;
			let mut iTemp974: i32 = (fTemp973) as i32;
			let mut iTemp975: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp974, 65535)))), 196607));
			let mut fTemp976: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp975, 3), 196607))) as usize] };
			let mut fTemp977: F64 = unsafe { ftbl0mydspSIG0[iTemp975 as usize] };
			let mut fTemp978: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp975, 1), 196607))) as usize] } - fTemp977;
			let mut iTemp979: i32 = (fTemp730 > ((if iTemp660 != 0 {fTemp977 + fTemp669 * fTemp978 + (fTemp973 - (iTemp974) as F64) * (fTemp976 - (fTemp977 + fTemp669 * (fTemp978 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp975, 4), 196607))) as usize] } - fTemp976))))} else {1.0 - (fTemp971 + fTemp669 * fTemp972 + (fTemp967 - (iTemp968) as F64) * (fTemp970 - (fTemp971 + fTemp669 * (fTemp972 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp969, 4)) as usize] } - fTemp970)))))} - fTemp965) / (1.0 - fTemp965))) as i32;
			let mut fTemp980: F64 = if iTemp979 != 0 {fTemp949} else {fTemp952};
			let mut fTemp981: F64 = if iTemp979 != 0 {fTemp952} else {fTemp950};
			let mut fTemp982: F64 = fTemp981 + fTemp980;
			let mut fTemp983: F64 = 0.5 * fTemp982;
			let mut fTemp984: F64 = 65535.0 * (1.0 - fTemp983);
			let mut iTemp985: i32 = (fTemp984) as i32;
			let mut iTemp986: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp985, 65535)))), 196607));
			let mut fTemp987: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp986, 3)) as usize] };
			let mut fTemp988: F64 = unsafe { ftbl0mydspSIG0[iTemp986 as usize] };
			let mut fTemp989: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp986, 1)) as usize] } - fTemp988;
			let mut fTemp990: F64 = 32767.5 * fTemp982;
			let mut iTemp991: i32 = (fTemp990) as i32;
			let mut iTemp992: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp991, 65535)))), 196607));
			let mut fTemp993: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp992, 3)) as usize] };
			let mut fTemp994: F64 = unsafe { ftbl0mydspSIG0[iTemp992 as usize] };
			let mut fTemp995: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp992, 1)) as usize] } - fTemp994;
			let mut fTemp996: F64 = if iTemp660 != 0 {fTemp994 + fTemp669 * fTemp995 + (fTemp990 - (iTemp991) as F64) * (fTemp993 - (fTemp994 + fTemp669 * (fTemp995 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp992, 4)) as usize] } - fTemp993))))} else {1.0 - (fTemp988 + fTemp669 * fTemp989 + (fTemp984 - (iTemp985) as F64) * (fTemp987 - (fTemp988 + fTemp669 * (fTemp989 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp986, 4)) as usize] } - fTemp987)))))};
			let mut fTemp997: F64 = fTemp674 + fTemp983;
			let mut fTemp998: F64 = 65535.0 * (1.0 - fTemp997);
			let mut iTemp999: i32 = (fTemp998) as i32;
			let mut iTemp1000: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp999, 65535)))), 196607));
			let mut fTemp1001: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1000, 3)) as usize] };
			let mut fTemp1002: F64 = unsafe { ftbl0mydspSIG0[iTemp1000 as usize] };
			let mut fTemp1003: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1000, 1)) as usize] } - fTemp1002;
			let mut fTemp1004: F64 = 65535.0 * fTemp997;
			let mut iTemp1005: i32 = (fTemp1004) as i32;
			let mut iTemp1006: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1005, 65535)))), 196607));
			let mut fTemp1007: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1006, 3), 196607))) as usize] };
			let mut fTemp1008: F64 = unsafe { ftbl0mydspSIG0[iTemp1006 as usize] };
			let mut fTemp1009: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1006, 1), 196607))) as usize] } - fTemp1008;
			let mut iTemp1010: i32 = (fTemp730 > ((if iTemp660 != 0 {fTemp1008 + fTemp669 * fTemp1009 + (fTemp1004 - (iTemp1005) as F64) * (fTemp1007 - (fTemp1008 + fTemp669 * (fTemp1009 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1006, 4), 196607))) as usize] } - fTemp1007))))} else {1.0 - (fTemp1002 + fTemp669 * fTemp1003 + (fTemp998 - (iTemp999) as F64) * (fTemp1001 - (fTemp1002 + fTemp669 * (fTemp1003 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1000, 4)) as usize] } - fTemp1001)))))} - fTemp996) / (1.0 - fTemp996))) as i32;
			let mut fTemp1011: F64 = if iTemp1010 != 0 {fTemp980} else {fTemp983};
			let mut fTemp1012: F64 = if iTemp1010 != 0 {fTemp983} else {fTemp981};
			let mut fTemp1013: F64 = fTemp1012 + fTemp1011;
			let mut fTemp1014: F64 = 0.5 * fTemp1013;
			let mut fTemp1015: F64 = 65535.0 * (1.0 - fTemp1014);
			let mut iTemp1016: i32 = (fTemp1015) as i32;
			let mut iTemp1017: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1016, 65535)))), 196607));
			let mut fTemp1018: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1017, 3)) as usize] };
			let mut fTemp1019: F64 = unsafe { ftbl0mydspSIG0[iTemp1017 as usize] };
			let mut fTemp1020: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1017, 1)) as usize] } - fTemp1019;
			let mut fTemp1021: F64 = 32767.5 * fTemp1013;
			let mut iTemp1022: i32 = (fTemp1021) as i32;
			let mut iTemp1023: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1022, 65535)))), 196607));
			let mut fTemp1024: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1023, 3)) as usize] };
			let mut fTemp1025: F64 = unsafe { ftbl0mydspSIG0[iTemp1023 as usize] };
			let mut fTemp1026: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1023, 1)) as usize] } - fTemp1025;
			let mut fTemp1027: F64 = if iTemp660 != 0 {fTemp1025 + fTemp669 * fTemp1026 + (fTemp1021 - (iTemp1022) as F64) * (fTemp1024 - (fTemp1025 + fTemp669 * (fTemp1026 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1023, 4)) as usize] } - fTemp1024))))} else {1.0 - (fTemp1019 + fTemp669 * fTemp1020 + (fTemp1015 - (iTemp1016) as F64) * (fTemp1018 - (fTemp1019 + fTemp669 * (fTemp1020 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1017, 4)) as usize] } - fTemp1018)))))};
			let mut fTemp1028: F64 = fTemp674 + fTemp1014;
			let mut fTemp1029: F64 = 65535.0 * (1.0 - fTemp1028);
			let mut iTemp1030: i32 = (fTemp1029) as i32;
			let mut iTemp1031: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1030, 65535)))), 196607));
			let mut fTemp1032: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1031, 3)) as usize] };
			let mut fTemp1033: F64 = unsafe { ftbl0mydspSIG0[iTemp1031 as usize] };
			let mut fTemp1034: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1031, 1)) as usize] } - fTemp1033;
			let mut fTemp1035: F64 = 65535.0 * fTemp1028;
			let mut iTemp1036: i32 = (fTemp1035) as i32;
			let mut iTemp1037: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1036, 65535)))), 196607));
			let mut fTemp1038: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1037, 3), 196607))) as usize] };
			let mut fTemp1039: F64 = unsafe { ftbl0mydspSIG0[iTemp1037 as usize] };
			let mut fTemp1040: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1037, 1), 196607))) as usize] } - fTemp1039;
			let mut iTemp1041: i32 = (fTemp730 > ((if iTemp660 != 0 {fTemp1039 + fTemp669 * fTemp1040 + (fTemp1035 - (iTemp1036) as F64) * (fTemp1038 - (fTemp1039 + fTemp669 * (fTemp1040 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1037, 4), 196607))) as usize] } - fTemp1038))))} else {1.0 - (fTemp1033 + fTemp669 * fTemp1034 + (fTemp1029 - (iTemp1030) as F64) * (fTemp1032 - (fTemp1033 + fTemp669 * (fTemp1034 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1031, 4)) as usize] } - fTemp1032)))))} - fTemp1027) / (1.0 - fTemp1027))) as i32;
			let mut fTemp1042: F64 = if iTemp1041 != 0 {fTemp1011} else {fTemp1014};
			let mut fTemp1043: F64 = if iTemp1041 != 0 {fTemp1014} else {fTemp1012};
			let mut fTemp1044: F64 = fTemp1043 + fTemp1042;
			let mut fTemp1045: F64 = 0.5 * fTemp1044;
			let mut fTemp1046: F64 = 65535.0 * (1.0 - fTemp1045);
			let mut iTemp1047: i32 = (fTemp1046) as i32;
			let mut iTemp1048: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1047, 65535)))), 196607));
			let mut fTemp1049: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1048, 3)) as usize] };
			let mut fTemp1050: F64 = unsafe { ftbl0mydspSIG0[iTemp1048 as usize] };
			let mut fTemp1051: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1048, 1)) as usize] } - fTemp1050;
			let mut fTemp1052: F64 = 32767.5 * fTemp1044;
			let mut iTemp1053: i32 = (fTemp1052) as i32;
			let mut iTemp1054: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1053, 65535)))), 196607));
			let mut fTemp1055: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1054, 3)) as usize] };
			let mut fTemp1056: F64 = unsafe { ftbl0mydspSIG0[iTemp1054 as usize] };
			let mut fTemp1057: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1054, 1)) as usize] } - fTemp1056;
			let mut fTemp1058: F64 = if iTemp660 != 0 {fTemp1056 + fTemp669 * fTemp1057 + (fTemp1052 - (iTemp1053) as F64) * (fTemp1055 - (fTemp1056 + fTemp669 * (fTemp1057 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1054, 4)) as usize] } - fTemp1055))))} else {1.0 - (fTemp1050 + fTemp669 * fTemp1051 + (fTemp1046 - (iTemp1047) as F64) * (fTemp1049 - (fTemp1050 + fTemp669 * (fTemp1051 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1048, 4)) as usize] } - fTemp1049)))))};
			let mut fTemp1059: F64 = fTemp674 + fTemp1045;
			let mut fTemp1060: F64 = 65535.0 * (1.0 - fTemp1059);
			let mut iTemp1061: i32 = (fTemp1060) as i32;
			let mut iTemp1062: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1061, 65535)))), 196607));
			let mut fTemp1063: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1062, 3)) as usize] };
			let mut fTemp1064: F64 = unsafe { ftbl0mydspSIG0[iTemp1062 as usize] };
			let mut fTemp1065: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1062, 1)) as usize] } - fTemp1064;
			let mut fTemp1066: F64 = 65535.0 * fTemp1059;
			let mut iTemp1067: i32 = (fTemp1066) as i32;
			let mut iTemp1068: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1067, 65535)))), 196607));
			let mut fTemp1069: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1068, 3), 196607))) as usize] };
			let mut fTemp1070: F64 = unsafe { ftbl0mydspSIG0[iTemp1068 as usize] };
			let mut fTemp1071: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1068, 1), 196607))) as usize] } - fTemp1070;
			let mut iTemp1072: i32 = (fTemp730 > ((if iTemp660 != 0 {fTemp1070 + fTemp669 * fTemp1071 + (fTemp1066 - (iTemp1067) as F64) * (fTemp1069 - (fTemp1070 + fTemp669 * (fTemp1071 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1068, 4), 196607))) as usize] } - fTemp1069))))} else {1.0 - (fTemp1064 + fTemp669 * fTemp1065 + (fTemp1060 - (iTemp1061) as F64) * (fTemp1063 - (fTemp1064 + fTemp669 * (fTemp1065 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1062, 4)) as usize] } - fTemp1063)))))} - fTemp1058) / (1.0 - fTemp1058))) as i32;
			let mut fTemp1073: F64 = if iTemp1072 != 0 {fTemp1042} else {fTemp1045};
			let mut fTemp1074: F64 = if iTemp1072 != 0 {fTemp1045} else {fTemp1043};
			let mut fTemp1075: F64 = fTemp1074 + fTemp1073;
			let mut fTemp1076: F64 = 0.5 * fTemp1075;
			let mut fTemp1077: F64 = 65535.0 * (1.0 - fTemp1076);
			let mut iTemp1078: i32 = (fTemp1077) as i32;
			let mut iTemp1079: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1078, 65535)))), 196607));
			let mut fTemp1080: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1079, 3)) as usize] };
			let mut fTemp1081: F64 = unsafe { ftbl0mydspSIG0[iTemp1079 as usize] };
			let mut fTemp1082: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1079, 1)) as usize] } - fTemp1081;
			let mut fTemp1083: F64 = 32767.5 * fTemp1075;
			let mut iTemp1084: i32 = (fTemp1083) as i32;
			let mut iTemp1085: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1084, 65535)))), 196607));
			let mut fTemp1086: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1085, 3)) as usize] };
			let mut fTemp1087: F64 = unsafe { ftbl0mydspSIG0[iTemp1085 as usize] };
			let mut fTemp1088: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1085, 1)) as usize] } - fTemp1087;
			let mut fTemp1089: F64 = if iTemp660 != 0 {fTemp1087 + fTemp669 * fTemp1088 + (fTemp1083 - (iTemp1084) as F64) * (fTemp1086 - (fTemp1087 + fTemp669 * (fTemp1088 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1085, 4)) as usize] } - fTemp1086))))} else {1.0 - (fTemp1081 + fTemp669 * fTemp1082 + (fTemp1077 - (iTemp1078) as F64) * (fTemp1080 - (fTemp1081 + fTemp669 * (fTemp1082 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1079, 4)) as usize] } - fTemp1080)))))};
			let mut fTemp1090: F64 = fTemp674 + fTemp1076;
			let mut fTemp1091: F64 = 65535.0 * (1.0 - fTemp1090);
			let mut iTemp1092: i32 = (fTemp1091) as i32;
			let mut iTemp1093: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1092, 65535)))), 196607));
			let mut fTemp1094: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1093, 3)) as usize] };
			let mut fTemp1095: F64 = unsafe { ftbl0mydspSIG0[iTemp1093 as usize] };
			let mut fTemp1096: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1093, 1)) as usize] } - fTemp1095;
			let mut fTemp1097: F64 = 65535.0 * fTemp1090;
			let mut iTemp1098: i32 = (fTemp1097) as i32;
			let mut iTemp1099: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1098, 65535)))), 196607));
			let mut fTemp1100: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1099, 3), 196607))) as usize] };
			let mut fTemp1101: F64 = unsafe { ftbl0mydspSIG0[iTemp1099 as usize] };
			let mut fTemp1102: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1099, 1), 196607))) as usize] } - fTemp1101;
			let mut iTemp1103: i32 = (fTemp730 > ((if iTemp660 != 0 {fTemp1101 + fTemp669 * fTemp1102 + (fTemp1097 - (iTemp1098) as F64) * (fTemp1100 - (fTemp1101 + fTemp669 * (fTemp1102 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1099, 4), 196607))) as usize] } - fTemp1100))))} else {1.0 - (fTemp1095 + fTemp669 * fTemp1096 + (fTemp1091 - (iTemp1092) as F64) * (fTemp1094 - (fTemp1095 + fTemp669 * (fTemp1096 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1093, 4)) as usize] } - fTemp1094)))))} - fTemp1089) / (1.0 - fTemp1089))) as i32;
			let mut fTemp1104: F64 = if iTemp1103 != 0 {fTemp1073} else {fTemp1076};
			let mut fTemp1105: F64 = if iTemp1103 != 0 {fTemp1076} else {fTemp1074};
			let mut fTemp1106: F64 = fTemp1105 + fTemp1104;
			let mut fTemp1107: F64 = 0.5 * fTemp1106;
			let mut fTemp1108: F64 = 65535.0 * (1.0 - fTemp1107);
			let mut iTemp1109: i32 = (fTemp1108) as i32;
			let mut iTemp1110: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1109, 65535)))), 196607));
			let mut fTemp1111: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1110, 3)) as usize] };
			let mut fTemp1112: F64 = unsafe { ftbl0mydspSIG0[iTemp1110 as usize] };
			let mut fTemp1113: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1110, 1)) as usize] } - fTemp1112;
			let mut fTemp1114: F64 = 32767.5 * fTemp1106;
			let mut iTemp1115: i32 = (fTemp1114) as i32;
			let mut iTemp1116: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1115, 65535)))), 196607));
			let mut fTemp1117: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1116, 3)) as usize] };
			let mut fTemp1118: F64 = unsafe { ftbl0mydspSIG0[iTemp1116 as usize] };
			let mut fTemp1119: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1116, 1)) as usize] } - fTemp1118;
			let mut fTemp1120: F64 = if iTemp660 != 0 {fTemp1118 + fTemp669 * fTemp1119 + (fTemp1114 - (iTemp1115) as F64) * (fTemp1117 - (fTemp1118 + fTemp669 * (fTemp1119 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1116, 4)) as usize] } - fTemp1117))))} else {1.0 - (fTemp1112 + fTemp669 * fTemp1113 + (fTemp1108 - (iTemp1109) as F64) * (fTemp1111 - (fTemp1112 + fTemp669 * (fTemp1113 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1110, 4)) as usize] } - fTemp1111)))))};
			let mut fTemp1121: F64 = fTemp674 + fTemp1107;
			let mut fTemp1122: F64 = 65535.0 * (1.0 - fTemp1121);
			let mut iTemp1123: i32 = (fTemp1122) as i32;
			let mut iTemp1124: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1123, 65535)))), 196607));
			let mut fTemp1125: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1124, 3)) as usize] };
			let mut fTemp1126: F64 = unsafe { ftbl0mydspSIG0[iTemp1124 as usize] };
			let mut fTemp1127: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1124, 1)) as usize] } - fTemp1126;
			let mut fTemp1128: F64 = 65535.0 * fTemp1121;
			let mut iTemp1129: i32 = (fTemp1128) as i32;
			let mut iTemp1130: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1129, 65535)))), 196607));
			let mut fTemp1131: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1130, 3), 196607))) as usize] };
			let mut fTemp1132: F64 = unsafe { ftbl0mydspSIG0[iTemp1130 as usize] };
			let mut fTemp1133: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1130, 1), 196607))) as usize] } - fTemp1132;
			let mut iTemp1134: i32 = (fTemp730 > ((if iTemp660 != 0 {fTemp1132 + fTemp669 * fTemp1133 + (fTemp1128 - (iTemp1129) as F64) * (fTemp1131 - (fTemp1132 + fTemp669 * (fTemp1133 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1130, 4), 196607))) as usize] } - fTemp1131))))} else {1.0 - (fTemp1126 + fTemp669 * fTemp1127 + (fTemp1122 - (iTemp1123) as F64) * (fTemp1125 - (fTemp1126 + fTemp669 * (fTemp1127 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1124, 4)) as usize] } - fTemp1125)))))} - fTemp1120) / (1.0 - fTemp1120))) as i32;
			let mut fTemp1135: F64 = if iTemp1134 != 0 {fTemp1104} else {fTemp1107};
			let mut fTemp1136: F64 = if iTemp1134 != 0 {fTemp1107} else {fTemp1105};
			let mut fTemp1137: F64 = fTemp1136 + fTemp1135;
			let mut fTemp1138: F64 = 0.5 * fTemp1137;
			let mut fTemp1139: F64 = 65535.0 * (1.0 - fTemp1138);
			let mut iTemp1140: i32 = (fTemp1139) as i32;
			let mut iTemp1141: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1140, 65535)))), 196607));
			let mut fTemp1142: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1141, 3)) as usize] };
			let mut fTemp1143: F64 = unsafe { ftbl0mydspSIG0[iTemp1141 as usize] };
			let mut fTemp1144: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1141, 1)) as usize] } - fTemp1143;
			let mut fTemp1145: F64 = 32767.5 * fTemp1137;
			let mut iTemp1146: i32 = (fTemp1145) as i32;
			let mut iTemp1147: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1146, 65535)))), 196607));
			let mut fTemp1148: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1147, 3)) as usize] };
			let mut fTemp1149: F64 = unsafe { ftbl0mydspSIG0[iTemp1147 as usize] };
			let mut fTemp1150: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1147, 1)) as usize] } - fTemp1149;
			let mut fTemp1151: F64 = if iTemp660 != 0 {fTemp1149 + fTemp669 * fTemp1150 + (fTemp1145 - (iTemp1146) as F64) * (fTemp1148 - (fTemp1149 + fTemp669 * (fTemp1150 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1147, 4)) as usize] } - fTemp1148))))} else {1.0 - (fTemp1143 + fTemp669 * fTemp1144 + (fTemp1139 - (iTemp1140) as F64) * (fTemp1142 - (fTemp1143 + fTemp669 * (fTemp1144 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1141, 4)) as usize] } - fTemp1142)))))};
			let mut fTemp1152: F64 = fTemp674 + fTemp1138;
			let mut fTemp1153: F64 = 65535.0 * (1.0 - fTemp1152);
			let mut iTemp1154: i32 = (fTemp1153) as i32;
			let mut iTemp1155: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1154, 65535)))), 196607));
			let mut fTemp1156: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1155, 3)) as usize] };
			let mut fTemp1157: F64 = unsafe { ftbl0mydspSIG0[iTemp1155 as usize] };
			let mut fTemp1158: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1155, 1)) as usize] } - fTemp1157;
			let mut fTemp1159: F64 = 65535.0 * fTemp1152;
			let mut iTemp1160: i32 = (fTemp1159) as i32;
			let mut iTemp1161: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1160, 65535)))), 196607));
			let mut fTemp1162: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1161, 3), 196607))) as usize] };
			let mut fTemp1163: F64 = unsafe { ftbl0mydspSIG0[iTemp1161 as usize] };
			let mut fTemp1164: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1161, 1), 196607))) as usize] } - fTemp1163;
			let mut iTemp1165: i32 = (fTemp730 > ((if iTemp660 != 0 {fTemp1163 + fTemp669 * fTemp1164 + (fTemp1159 - (iTemp1160) as F64) * (fTemp1162 - (fTemp1163 + fTemp669 * (fTemp1164 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1161, 4), 196607))) as usize] } - fTemp1162))))} else {1.0 - (fTemp1157 + fTemp669 * fTemp1158 + (fTemp1153 - (iTemp1154) as F64) * (fTemp1156 - (fTemp1157 + fTemp669 * (fTemp1158 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1155, 4)) as usize] } - fTemp1156)))))} - fTemp1151) / (1.0 - fTemp1151))) as i32;
			let mut fTemp1166: F64 = if iTemp1165 != 0 {fTemp1135} else {fTemp1138};
			let mut fTemp1167: F64 = if iTemp1165 != 0 {fTemp1138} else {fTemp1136};
			let mut fTemp1168: F64 = fTemp1167 + fTemp1166;
			let mut fTemp1169: F64 = 0.5 * fTemp1168;
			let mut fTemp1170: F64 = 65535.0 * (1.0 - fTemp1169);
			let mut iTemp1171: i32 = (fTemp1170) as i32;
			let mut iTemp1172: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1171, 65535)))), 196607));
			let mut fTemp1173: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1172, 3)) as usize] };
			let mut fTemp1174: F64 = unsafe { ftbl0mydspSIG0[iTemp1172 as usize] };
			let mut fTemp1175: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1172, 1)) as usize] } - fTemp1174;
			let mut fTemp1176: F64 = 32767.5 * fTemp1168;
			let mut iTemp1177: i32 = (fTemp1176) as i32;
			let mut iTemp1178: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1177, 65535)))), 196607));
			let mut fTemp1179: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1178, 3)) as usize] };
			let mut fTemp1180: F64 = unsafe { ftbl0mydspSIG0[iTemp1178 as usize] };
			let mut fTemp1181: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1178, 1)) as usize] } - fTemp1180;
			let mut fTemp1182: F64 = if iTemp660 != 0 {fTemp1180 + fTemp669 * fTemp1181 + (fTemp1176 - (iTemp1177) as F64) * (fTemp1179 - (fTemp1180 + fTemp669 * (fTemp1181 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1178, 4), 196607))) as usize] } - fTemp1179))))} else {1.0 - (fTemp1174 + fTemp669 * fTemp1175 + (fTemp1170 - (iTemp1171) as F64) * (fTemp1173 - (fTemp1174 + fTemp669 * (fTemp1175 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1172, 4), 196607))) as usize] } - fTemp1173)))))};
			let mut fTemp1183: F64 = fTemp674 + fTemp1169;
			let mut fTemp1184: F64 = 65535.0 * (1.0 - fTemp1183);
			let mut iTemp1185: i32 = (fTemp1184) as i32;
			let mut iTemp1186: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1185, 65535)))), 196607));
			let mut fTemp1187: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1186, 3)) as usize] };
			let mut fTemp1188: F64 = unsafe { ftbl0mydspSIG0[iTemp1186 as usize] };
			let mut fTemp1189: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1186, 1)) as usize] } - fTemp1188;
			let mut fTemp1190: F64 = 65535.0 * fTemp1183;
			let mut iTemp1191: i32 = (fTemp1190) as i32;
			let mut iTemp1192: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1191, 65535)))), 196607));
			let mut fTemp1193: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1192, 3), 196607))) as usize] };
			let mut fTemp1194: F64 = unsafe { ftbl0mydspSIG0[iTemp1192 as usize] };
			let mut fTemp1195: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1192, 1), 196607))) as usize] } - fTemp1194;
			let mut iTemp1196: i32 = (fTemp730 > ((if iTemp660 != 0 {fTemp1194 + fTemp669 * fTemp1195 + (fTemp1190 - (iTemp1191) as F64) * (fTemp1193 - (fTemp1194 + fTemp669 * (fTemp1195 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1192, 4), 196607))) as usize] } - fTemp1193))))} else {1.0 - (fTemp1188 + fTemp669 * fTemp1189 + (fTemp1184 - (iTemp1185) as F64) * (fTemp1187 - (fTemp1188 + fTemp669 * (fTemp1189 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1186, 4)) as usize] } - fTemp1187)))))} - fTemp1182) / (1.0 - fTemp1182))) as i32;
			let mut fTemp1197: F64 = F64::min(1.0, F64::max(0.0, 0.5 * (if iTemp1196 != 0 {fTemp1169} else {fTemp1167} + if iTemp1196 != 0 {fTemp1166} else {fTemp1169})));
			self.fRec15[0] = fTemp1197;
			let mut fTemp1198: F64 = 65535.0 * (1.0 - fTemp1197);
			let mut iTemp1199: i32 = (fTemp1198) as i32;
			let mut iTemp1200: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1199, 65535)))), 196607));
			let mut fTemp1201: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1200, 3)) as usize] };
			let mut fTemp1202: F64 = unsafe { ftbl0mydspSIG0[iTemp1200 as usize] };
			let mut fTemp1203: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1200, 1)) as usize] } - fTemp1202;
			let mut fTemp1204: F64 = 65535.0 * fTemp1197;
			let mut iTemp1205: i32 = (fTemp1204) as i32;
			let mut iTemp1206: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1205, 65535)))), 196607));
			let mut fTemp1207: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1206, 3)) as usize] };
			let mut fTemp1208: F64 = unsafe { ftbl0mydspSIG0[iTemp1206 as usize] };
			let mut fTemp1209: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1206, 1)) as usize] } - fTemp1208;
			let mut fTemp1210: F64 = if iTemp660 != 0 {fTemp1208 + fTemp669 * fTemp1209 + (fTemp1204 - (iTemp1205) as F64) * (fTemp1207 - (fTemp1208 + fTemp669 * (fTemp1209 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1206, 4), 196607))) as usize] } - fTemp1207))))} else {1.0 - (fTemp1202 + fTemp669 * fTemp1203 + (fTemp1198 - (iTemp1199) as F64) * (fTemp1201 - (fTemp1202 + fTemp669 * (fTemp1203 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1200, 4), 196607))) as usize] } - fTemp1201)))))};
			let mut fTemp1211: F64 = fTemp674 + fTemp1197;
			let mut fTemp1212: F64 = 65535.0 * (1.0 - fTemp1211);
			let mut iTemp1213: i32 = (fTemp1212) as i32;
			let mut iTemp1214: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1213, 65535)))), 196607));
			let mut fTemp1215: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1214, 3)) as usize] };
			let mut fTemp1216: F64 = unsafe { ftbl0mydspSIG0[iTemp1214 as usize] };
			let mut fTemp1217: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1214, 1)) as usize] } - fTemp1216;
			let mut fTemp1218: F64 = 65535.0 * fTemp1211;
			let mut iTemp1219: i32 = (fTemp1218) as i32;
			let mut iTemp1220: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1219, 65535)))), 196607));
			let mut fTemp1221: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1220, 3), 196607))) as usize] };
			let mut fTemp1222: F64 = unsafe { ftbl0mydspSIG0[iTemp1220 as usize] };
			let mut fTemp1223: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1220, 1), 196607))) as usize] } - fTemp1222;
			let mut fTemp1224: F64 = fTemp634 + if ((0.001 * fTemp673) == 0.0) as i32 != 0 {fTemp659} else {fTemp659 * (if iTemp660 != 0 {fTemp1222 + fTemp669 * fTemp1223 + (fTemp1218 - (iTemp1219) as F64) * (fTemp1221 - (fTemp1222 + fTemp669 * (fTemp1223 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1220, 4), 196607))) as usize] } - fTemp1221))))} else {1.0 - (fTemp1216 + fTemp669 * fTemp1217 + (fTemp1212 - (iTemp1213) as F64) * (fTemp1215 - (fTemp1216 + fTemp669 * (fTemp1217 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1214, 4)) as usize] } - fTemp1215)))))} - fTemp1210) / (1.0 - fTemp1210)};
			self.fRec16[(self.IOTA0 & 8191) as usize] = if iTemp672 != 0 {F64::min(fTemp1224, fTemp634)} else {F64::max(fTemp1224, fTemp634)};
			let mut fTemp1225: F64 = self.fRec16[((i32::wrapping_sub(self.IOTA0, iSlow70)) & 8191) as usize];
			self.fHbargraph2 = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, fTemp1225));
			*output1 = 0.5 * fTemp2 * self.fVec2[((i32::wrapping_sub(self.IOTA0, iSlow71)) & 32767) as usize] + self.fRec14[0] * fTemp4 * self.fVec3[((i32::wrapping_sub(self.IOTA0, iSlow71)) & 32767) as usize] * fTemp1225;
			*output2 = fTemp3 + fTemp633 * fTemp4;
			*output3 = fTemp3 + fTemp4 * fTemp1225;
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
