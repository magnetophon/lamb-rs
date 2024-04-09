mod dsp {
    /* ------------------------------------------------------------
author: "Bart Brouns"
license: "AGPLv3"
name: "lamb-rs"
version: "0.1"
Code generated with Faust 2.72.14 (https://faust.grame.fr)
Compilation options: -a /run/user/1001/.tmp6cUSBa -lang rust -ct 1 -es 1 -mcd 16 -mdd 1024 -mdy 33 -double -ftz 0
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
			let mut iTemp55: i32 = i32::wrapping_add(self.iRec13[0], -1);
			let mut fTemp56: F64 = (iTemp55 % 3) as F64 as i32 as F64;
			let mut fTemp57: F64 = 0.5 * fTemp56;
			let mut fTemp58: F64 = F64::powf(fTemp57, 0.21 * fTemp56 + 1.0);
			let mut fTemp59: F64 = (0.3333333333333333 * (iTemp55 % 196608) as F64) as i32 as F64;
			table[i1 as usize] = F64::min(1.0, F64::max(0.0, if (fTemp57 == 0.0) as i32 != 0 {0.5 * (F64::sin(4.793763109162727e-05 * fTemp59 + 4.71238898038469) + 1.0)} else {0.5 * (F64::sin(3.141592653589793 * ((1.0 - F64::exp(-(3.692683299000534e-05 * fTemp58 * fTemp59))) / (1.0 - F64::exp(-(2.42 * fTemp58)))) + 4.71238898038469) + 1.0)}));
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
		m.declare("compile_options", r"-a /run/user/1001/.tmp6cUSBa -lang rust -ct 1 -es 1 -mcd 16 -mdd 1024 -mdy 33 -double -ftz 0");
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
			self.fRec11[0] = fSlow10 + self.fConst4 * self.fRec11[1];
			let mut fTemp5: F64 = *input0;
			self.fVec0[(self.IOTA0 & 32767) as usize] = fTemp5;
			let mut fTemp6: F64 = fTemp5 * self.fRec11[0];
			self.fVec1[(self.IOTA0 & 32767) as usize] = fTemp6;
			let mut fTemp7: F64 = F64::abs(fTemp6);
			let mut fTemp8: F64 = *input1;
			self.fVec2[(self.IOTA0 & 32767) as usize] = fTemp8;
			let mut fTemp9: F64 = fTemp8 * self.fRec11[0];
			self.fVec3[(self.IOTA0 & 32767) as usize] = fTemp9;
			let mut fTemp10: F64 = F64::abs(fTemp9);
			let mut fTemp11: F64 = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, F64::max(fTemp7, fTemp10)));
			let mut iTemp12: i32 = ((fTemp11 > fSlow11) as i32) + ((fTemp11 > fSlow9) as i32);
			let mut fTemp13: F64 = fTemp11 - fSlow8;
			let mut fTemp14: F64 = F64::powf(1e+01, -(0.05 * F64::max(0.0, if (iTemp12 == 0) as i32 != 0 {0.0} else {if (iTemp12 == 1) as i32 != 0 {fSlow12 * mydsp_faustpower2_f(fSlow7 + fTemp13)} else {fTemp13}})));
			let mut fTemp15: F64 = 4.0 * (F64::max(0.25, self.fRec4[0]) + -0.25);
			let mut fTemp16: F64 = 9.0 - fTemp15;
			let mut fTemp17: F64 = self.fRec5[1] - self.fRec6[1];
			let mut fTemp18: F64 = if (fTemp14 > self.fRec10[1]) as i32 != 0 {F64::exp(-(self.fConst7 / F64::max(2.220446049250313e-16, fSlow14 / F64::max(1.0 - (F64::max(fTemp15 + -9.0, F64::min(2.0, fTemp17)) + fTemp16) / (11.0 - fTemp15), 2.220446049250313e-16))))} else {self.fConst6};
			self.fRec10[0] = self.fRec10[1] * fTemp18 + fTemp14 * (1.0 - fTemp18);
			let mut fTemp19: F64 = if (self.fRec10[0] > self.fRec9[1]) as i32 != 0 {0.0} else {self.fConst6};
			self.fRec9[0] = self.fRec9[1] * fTemp19 + self.fRec10[0] * (1.0 - fTemp19);
			let mut fTemp20: F64 = if (self.fRec9[0] > self.fRec8[1]) as i32 != 0 {0.0} else {self.fConst6};
			self.fRec8[0] = self.fRec8[1] * fTemp20 + self.fRec9[0] * (1.0 - fTemp20);
			let mut fTemp21: F64 = if (self.fRec8[0] > self.fRec7[1]) as i32 != 0 {0.0} else {self.fConst6};
			self.fRec7[0] = self.fRec7[1] * fTemp21 + self.fRec8[0] * (1.0 - fTemp21);
			self.fRec5[0] = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, self.fRec7[0]));
			let mut fTemp22: F64 = F64::powf(1e+01, 0.05 * (self.fRec5[1] + fTemp16));
			let mut fTemp23: F64 = 10.588235294117647 * (F64::max(0.15, self.fRec4[0]) + -0.15);
			let mut fTemp24: F64 = if (fTemp22 > self.fRec12[1]) as i32 != 0 {F64::exp(-(self.fConst7 / F64::max(2.220446049250313e-16, 0.05 * F64::powf(4503599627370496.0, 1.0 - (F64::max(fTemp23 + -12.0, F64::min(3.0, fTemp17)) + (12.0 - fTemp23)) / (15.0 - fTemp23)))))} else {self.fConst8};
			self.fRec12[0] = self.fRec12[1] * fTemp24 + fTemp22 * (1.0 - fTemp24);
			self.fRec6[0] = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, self.fRec12[0]));
			let mut fTemp25: F64 = F64::powf(1e+01, fSlow16 * self.fRec5[0] * F64::min(0.25, self.fRec4[0]));
			let mut fTemp26: F64 = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, fTemp7));
			let mut fTemp27: F64 = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, fTemp10));
			let mut fTemp28: F64 = F64::max(fTemp26, fTemp27);
			let mut fTemp29: F64 = fTemp26 + fSlow17 * (fTemp28 - fTemp26);
			let mut iTemp30: i32 = ((fTemp29 > fSlow11) as i32) + ((fTemp29 > fSlow9) as i32);
			let mut fTemp31: F64 = fTemp29 - fSlow8;
			let mut fTemp32: F64 = F64::min(fTemp25, F64::powf(1e+01, -(fSlow18 * F64::max(0.0, if (iTemp30 == 0) as i32 != 0 {0.0} else {if (iTemp30 == 1) as i32 != 0 {fSlow12 * mydsp_faustpower2_f(fSlow7 + fTemp31)} else {fTemp31}}))));
			self.fVec4[(self.IOTA0 & 16383) as usize] = fTemp32;
			let mut fTemp33: F64 = F64::min(fTemp32, self.fVec4[((i32::wrapping_sub(self.IOTA0, 1)) & 16383) as usize]);
			self.fVec5[0] = fTemp33;
			let mut fTemp34: F64 = F64::min(fTemp33, self.fVec5[2]);
			self.fVec6[0] = fTemp34;
			let mut fTemp35: F64 = F64::min(fTemp34, self.fVec6[4]);
			self.fVec7[0] = fTemp35;
			let mut fTemp36: F64 = F64::min(fTemp35, self.fVec7[8]);
			self.fVec8[(self.IOTA0 & 31) as usize] = fTemp36;
			let mut fTemp37: F64 = F64::min(fTemp36, self.fVec8[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize]);
			self.fVec9[(self.IOTA0 & 63) as usize] = fTemp37;
			let mut fTemp38: F64 = F64::min(fTemp37, self.fVec9[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize]);
			self.fVec10[(self.IOTA0 & 127) as usize] = fTemp38;
			let mut fTemp39: F64 = F64::min(fTemp38, self.fVec10[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize]);
			self.fVec11[(self.IOTA0 & 255) as usize] = fTemp39;
			let mut fTemp40: F64 = F64::min(fTemp39, self.fVec11[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize]);
			self.fVec12[(self.IOTA0 & 511) as usize] = fTemp40;
			let mut fTemp41: F64 = F64::min(fTemp40, self.fVec12[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize]);
			self.fVec13[(self.IOTA0 & 1023) as usize] = fTemp41;
			let mut fTemp42: F64 = F64::min(fTemp41, self.fVec13[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize]);
			self.fVec14[(self.IOTA0 & 2047) as usize] = fTemp42;
			self.fVec15[(self.IOTA0 & 4095) as usize] = F64::min(fTemp42, self.fVec14[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			self.fRec3[0] = F64::max(F64::min(self.fRec3[1], self.fVec4[((i32::wrapping_sub(self.IOTA0, iSlow21)) & 16383) as usize]), F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow23 != 0 {fTemp32} else {1.7976931348623157e+308}, if iSlow24 != 0 {self.fVec5[iSlow23 as usize]} else {1.7976931348623157e+308}), if iSlow25 != 0 {self.fVec6[iSlow26 as usize]} else {1.7976931348623157e+308}), if iSlow27 != 0 {self.fVec7[iSlow28 as usize]} else {1.7976931348623157e+308}), if iSlow29 != 0 {self.fVec8[((i32::wrapping_sub(self.IOTA0, iSlow30)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow31 != 0 {self.fVec9[((i32::wrapping_sub(self.IOTA0, iSlow32)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow33 != 0 {self.fVec10[((i32::wrapping_sub(self.IOTA0, iSlow34)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow35 != 0 {self.fVec11[((i32::wrapping_sub(self.IOTA0, iSlow36)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow37 != 0 {self.fVec12[((i32::wrapping_sub(self.IOTA0, iSlow38)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow39 != 0 {self.fVec13[((i32::wrapping_sub(self.IOTA0, iSlow40)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow41 != 0 {self.fVec14[((i32::wrapping_sub(self.IOTA0, iSlow42)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow43 != 0 {self.fVec15[((i32::wrapping_sub(self.IOTA0, iSlow44)) & 4095) as usize]} else {1.7976931348623157e+308}));
			let mut fTemp43: F64 = F64::min(self.fRec3[0], self.fRec3[1]);
			self.fVec16[0] = fTemp43;
			let mut fTemp44: F64 = F64::min(fTemp43, self.fVec16[2]);
			self.fVec17[0] = fTemp44;
			let mut fTemp45: F64 = F64::min(fTemp44, self.fVec17[4]);
			self.fVec18[0] = fTemp45;
			let mut fTemp46: F64 = F64::min(fTemp45, self.fVec18[8]);
			self.fVec19[(self.IOTA0 & 31) as usize] = fTemp46;
			let mut fTemp47: F64 = F64::min(fTemp46, self.fVec19[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize]);
			self.fVec20[(self.IOTA0 & 63) as usize] = fTemp47;
			let mut fTemp48: F64 = F64::min(fTemp47, self.fVec20[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize]);
			self.fVec21[(self.IOTA0 & 127) as usize] = fTemp48;
			let mut fTemp49: F64 = F64::min(fTemp48, self.fVec21[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize]);
			self.fVec22[(self.IOTA0 & 255) as usize] = fTemp49;
			let mut fTemp50: F64 = F64::min(fTemp49, self.fVec22[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize]);
			self.fVec23[(self.IOTA0 & 511) as usize] = fTemp50;
			let mut fTemp51: F64 = F64::min(fTemp50, self.fVec23[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize]);
			self.fVec24[(self.IOTA0 & 1023) as usize] = fTemp51;
			let mut fTemp52: F64 = F64::min(fTemp51, self.fVec24[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize]);
			self.fVec25[(self.IOTA0 & 2047) as usize] = fTemp52;
			self.fVec26[(self.IOTA0 & 4095) as usize] = F64::min(fTemp52, self.fVec25[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			let mut fTemp53: F64 = F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow4 != 0 {self.fRec3[0]} else {1.7976931348623157e+308}, if iSlow45 != 0 {self.fVec16[iSlow4 as usize]} else {1.7976931348623157e+308}), if iSlow46 != 0 {self.fVec17[iSlow47 as usize]} else {1.7976931348623157e+308}), if iSlow48 != 0 {self.fVec18[iSlow49 as usize]} else {1.7976931348623157e+308}), if iSlow50 != 0 {self.fVec19[((i32::wrapping_sub(self.IOTA0, iSlow51)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow52 != 0 {self.fVec20[((i32::wrapping_sub(self.IOTA0, iSlow53)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow54 != 0 {self.fVec21[((i32::wrapping_sub(self.IOTA0, iSlow55)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow56 != 0 {self.fVec22[((i32::wrapping_sub(self.IOTA0, iSlow57)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow58 != 0 {self.fVec23[((i32::wrapping_sub(self.IOTA0, iSlow59)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow60 != 0 {self.fVec24[((i32::wrapping_sub(self.IOTA0, iSlow61)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow62 != 0 {self.fVec25[((i32::wrapping_sub(self.IOTA0, iSlow63)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow64 != 0 {self.fVec26[((i32::wrapping_sub(self.IOTA0, iSlow65)) & 4095) as usize]} else {1.7976931348623157e+308}) - fTemp4;
			self.fVec27[0] = fTemp53;
			let mut iTemp54: i32 = (fTemp53 > 0.0) as i32;
			let mut fTemp60: F64 = if iTemp54 != 0 {fSlow67} else {fSlow66};
			self.fVec28[0] = fTemp60;
			let mut fTemp61: F64 = 2.0 * fTemp60;
			let mut iTemp62: i32 = (fTemp61) as i32;
			let mut iTemp63: i32 = std::cmp::max(0, std::cmp::min(iTemp62, 2));
			let mut iTemp64: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, 98301), 196607));
			let mut fTemp65: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp64, 3)) as usize] };
			let mut fTemp66: F64 = unsafe { ftbl0mydspSIG0[iTemp64 as usize] };
			let mut fTemp67: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp64, 1)) as usize] } - fTemp66;
			let mut fTemp68: F64 = fTemp61 - (iTemp62) as F64;
			let mut fTemp69: F64 = fTemp66 + fTemp68 * fTemp67 + 0.5 * (fTemp65 - (fTemp66 + fTemp68 * (fTemp67 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp64, 4)) as usize] } - fTemp65))));
			let mut fTemp70: F64 = if iTemp54 != 0 {fTemp69} else {1.0 - fTemp69};
			let mut iTemp71: i32 = (fTemp53 < 0.0) as i32;
			let mut fTemp72: F64 = fSlow1 * (iTemp71) as F64 + fSlow13 * (iTemp54) as F64;
			self.fVec29[0] = fTemp72;
			let mut fTemp73: F64 = self.fConst10 / fTemp72;
			let mut fTemp74: F64 = fTemp73 + 0.5;
			let mut fTemp75: F64 = 65535.0 * (1.0 - fTemp74);
			let mut iTemp76: i32 = (fTemp75) as i32;
			let mut iTemp77: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp76, 65535)))), 196607));
			let mut fTemp78: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp77, 3)) as usize] };
			let mut fTemp79: F64 = unsafe { ftbl0mydspSIG0[iTemp77 as usize] };
			let mut fTemp80: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp77, 1)) as usize] } - fTemp79;
			let mut fTemp81: F64 = 65535.0 * fTemp74;
			let mut iTemp82: i32 = (fTemp81) as i32;
			let mut iTemp83: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp82, 65535)))), 196607));
			let mut fTemp84: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp83, 3), 196607))) as usize] };
			let mut fTemp85: F64 = unsafe { ftbl0mydspSIG0[iTemp83 as usize] };
			let mut fTemp86: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp83, 1), 196607))) as usize] } - fTemp85;
			let mut fTemp87: F64 = 2.0 * self.fVec28[1];
			let mut iTemp88: i32 = (fTemp87) as i32;
			let mut iTemp89: i32 = std::cmp::max(0, std::cmp::min(iTemp88, 2));
			let mut fTemp90: F64 = 65535.0 * (1.0 - self.fRec1[1]);
			let mut iTemp91: i32 = (fTemp90) as i32;
			let mut iTemp92: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp91, 65535))), iTemp89), 196607));
			let mut fTemp93: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp92, 3), 196607))) as usize] };
			let mut fTemp94: F64 = unsafe { ftbl0mydspSIG0[iTemp92 as usize] };
			let mut fTemp95: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp92, 1), 196607))) as usize] } - fTemp94;
			let mut fTemp96: F64 = fTemp87 - (iTemp88) as F64;
			let mut fTemp97: F64 = 65535.0 * self.fRec1[1];
			let mut iTemp98: i32 = (fTemp97) as i32;
			let mut iTemp99: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp89, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp98, 65535)))), 196607));
			let mut fTemp100: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp99, 3), 196607))) as usize] };
			let mut fTemp101: F64 = unsafe { ftbl0mydspSIG0[iTemp99 as usize] };
			let mut fTemp102: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp99, 1), 196607))) as usize] } - fTemp101;
			let mut fTemp103: F64 = self.fRec1[1] + fTemp73;
			let mut fTemp104: F64 = 65535.0 * (1.0 - fTemp103);
			let mut iTemp105: i32 = (fTemp104) as i32;
			let mut iTemp106: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp105, 65535)))), 196607));
			let mut fTemp107: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp106, 3)) as usize] };
			let mut fTemp108: F64 = unsafe { ftbl0mydspSIG0[iTemp106 as usize] };
			let mut fTemp109: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp106, 1)) as usize] } - fTemp108;
			let mut fTemp110: F64 = 65535.0 * fTemp103;
			let mut iTemp111: i32 = (fTemp110) as i32;
			let mut iTemp112: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp111, 65535)))), 196607));
			let mut fTemp113: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp112, 3), 196607))) as usize] };
			let mut fTemp114: F64 = unsafe { ftbl0mydspSIG0[iTemp112 as usize] };
			let mut fTemp115: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp112, 1), 196607))) as usize] } - fTemp114;
			let mut fTemp116: F64 = self.fRec1[1] + self.fConst10 * (1.0 / fTemp72 + 1.0 / self.fVec29[1]);
			let mut fTemp117: F64 = 65535.0 * (1.0 - fTemp116);
			let mut iTemp118: i32 = (fTemp117) as i32;
			let mut iTemp119: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp118, 65535))), iTemp63), 196607));
			let mut fTemp120: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp119, 3)) as usize] };
			let mut fTemp121: F64 = unsafe { ftbl0mydspSIG0[iTemp119 as usize] };
			let mut fTemp122: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp119, 1)) as usize] } - fTemp121;
			let mut fTemp123: F64 = 65535.0 * fTemp116;
			let mut iTemp124: i32 = (fTemp123) as i32;
			let mut iTemp125: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp124, 65535)))), 196607));
			let mut fTemp126: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp125, 3), 196607))) as usize] };
			let mut fTemp127: F64 = unsafe { ftbl0mydspSIG0[iTemp125 as usize] };
			let mut fTemp128: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp125, 1), 196607))) as usize] } - fTemp127;
			let mut fTemp129: F64 = (if iTemp54 != 0 {fTemp127 + fTemp68 * fTemp128 + (fTemp123 - (iTemp124) as F64) * (fTemp126 - (fTemp127 + fTemp68 * (fTemp128 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp125, 4), 196607))) as usize] } - fTemp126))))} else {1.0 - (fTemp121 + fTemp68 * fTemp122 + (fTemp117 - (iTemp118) as F64) * (fTemp120 - (fTemp121 + fTemp68 * (fTemp122 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp119, 4)) as usize] } - fTemp120)))))} - if iTemp54 != 0 {fTemp114 + fTemp68 * fTemp115 + (fTemp110 - (iTemp111) as F64) * (fTemp113 - (fTemp114 + fTemp68 * (fTemp115 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp112, 4), 196607))) as usize] } - fTemp113))))} else {1.0 - (fTemp108 + fTemp68 * fTemp109 + (fTemp104 - (iTemp105) as F64) * (fTemp107 - (fTemp108 + fTemp68 * (fTemp109 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp106, 4), 196607))) as usize] } - fTemp107)))))}) * self.fVec27[1] / (fTemp53 * (1.0 - if iTemp54 != 0 {fTemp101 + fTemp96 * fTemp102 + (fTemp97 - (iTemp98) as F64) * (fTemp100 - (fTemp101 + fTemp96 * (fTemp102 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp99, 4), 196607))) as usize] } - fTemp100))))} else {1.0 - (fTemp94 + fTemp96 * fTemp95 + (fTemp90 - (iTemp91) as F64) * (fTemp93 - (fTemp94 + fTemp96 * (fTemp95 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp92, 4), 196607))) as usize] } - fTemp93)))))}));
			let mut iTemp130: i32 = (fTemp129 > ((if iTemp54 != 0 {fTemp85 + fTemp68 * fTemp86 + (fTemp81 - (iTemp82) as F64) * (fTemp84 - (fTemp85 + fTemp68 * (fTemp86 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp83, 4), 196607))) as usize] } - fTemp84))))} else {1.0 - (fTemp79 + fTemp68 * fTemp80 + (fTemp75 - (iTemp76) as F64) * (fTemp78 - (fTemp79 + fTemp68 * (fTemp80 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp77, 4)) as usize] } - fTemp78)))))} - fTemp70) / (1.0 - fTemp70))) as i32;
			let mut fTemp131: F64 = if iTemp130 != 0 {1.0} else {0.5};
			let mut fTemp132: F64 = if iTemp130 != 0 {0.5} else {0.0};
			let mut fTemp133: F64 = fTemp132 + fTemp131;
			let mut fTemp134: F64 = 0.5 * fTemp133;
			let mut fTemp135: F64 = 65535.0 * (1.0 - fTemp134);
			let mut iTemp136: i32 = (fTemp135) as i32;
			let mut iTemp137: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp136, 65535)))), 196607));
			let mut fTemp138: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp137, 3)) as usize] };
			let mut fTemp139: F64 = unsafe { ftbl0mydspSIG0[iTemp137 as usize] };
			let mut fTemp140: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp137, 1)) as usize] } - fTemp139;
			let mut fTemp141: F64 = 32767.5 * fTemp133;
			let mut iTemp142: i32 = (fTemp141) as i32;
			let mut iTemp143: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp142, 65535)))), 196607));
			let mut fTemp144: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp143, 3)) as usize] };
			let mut fTemp145: F64 = unsafe { ftbl0mydspSIG0[iTemp143 as usize] };
			let mut fTemp146: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp143, 1)) as usize] } - fTemp145;
			let mut fTemp147: F64 = if iTemp54 != 0 {fTemp145 + fTemp68 * fTemp146 + (fTemp141 - (iTemp142) as F64) * (fTemp144 - (fTemp145 + fTemp68 * (fTemp146 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp143, 4)) as usize] } - fTemp144))))} else {1.0 - (fTemp139 + fTemp68 * fTemp140 + (fTemp135 - (iTemp136) as F64) * (fTemp138 - (fTemp139 + fTemp68 * (fTemp140 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp137, 4)) as usize] } - fTemp138)))))};
			let mut fTemp148: F64 = fTemp73 + fTemp134;
			let mut fTemp149: F64 = 65535.0 * (1.0 - fTemp148);
			let mut iTemp150: i32 = (fTemp149) as i32;
			let mut iTemp151: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp150, 65535)))), 196607));
			let mut fTemp152: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp151, 3)) as usize] };
			let mut fTemp153: F64 = unsafe { ftbl0mydspSIG0[iTemp151 as usize] };
			let mut fTemp154: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp151, 1)) as usize] } - fTemp153;
			let mut fTemp155: F64 = 65535.0 * fTemp148;
			let mut iTemp156: i32 = (fTemp155) as i32;
			let mut iTemp157: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp156, 65535)))), 196607));
			let mut fTemp158: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp157, 3), 196607))) as usize] };
			let mut fTemp159: F64 = unsafe { ftbl0mydspSIG0[iTemp157 as usize] };
			let mut fTemp160: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp157, 1), 196607))) as usize] } - fTemp159;
			let mut iTemp161: i32 = (fTemp129 > ((if iTemp54 != 0 {fTemp159 + fTemp68 * fTemp160 + (fTemp155 - (iTemp156) as F64) * (fTemp158 - (fTemp159 + fTemp68 * (fTemp160 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp157, 4), 196607))) as usize] } - fTemp158))))} else {1.0 - (fTemp153 + fTemp68 * fTemp154 + (fTemp149 - (iTemp150) as F64) * (fTemp152 - (fTemp153 + fTemp68 * (fTemp154 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp151, 4)) as usize] } - fTemp152)))))} - fTemp147) / (1.0 - fTemp147))) as i32;
			let mut fTemp162: F64 = if iTemp161 != 0 {fTemp131} else {fTemp134};
			let mut fTemp163: F64 = if iTemp161 != 0 {fTemp134} else {fTemp132};
			let mut fTemp164: F64 = fTemp163 + fTemp162;
			let mut fTemp165: F64 = 0.5 * fTemp164;
			let mut fTemp166: F64 = 65535.0 * (1.0 - fTemp165);
			let mut iTemp167: i32 = (fTemp166) as i32;
			let mut iTemp168: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp167, 65535)))), 196607));
			let mut fTemp169: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp168, 3)) as usize] };
			let mut fTemp170: F64 = unsafe { ftbl0mydspSIG0[iTemp168 as usize] };
			let mut fTemp171: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp168, 1)) as usize] } - fTemp170;
			let mut fTemp172: F64 = 32767.5 * fTemp164;
			let mut iTemp173: i32 = (fTemp172) as i32;
			let mut iTemp174: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp173, 65535)))), 196607));
			let mut fTemp175: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp174, 3)) as usize] };
			let mut fTemp176: F64 = unsafe { ftbl0mydspSIG0[iTemp174 as usize] };
			let mut fTemp177: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp174, 1)) as usize] } - fTemp176;
			let mut fTemp178: F64 = if iTemp54 != 0 {fTemp176 + fTemp68 * fTemp177 + (fTemp172 - (iTemp173) as F64) * (fTemp175 - (fTemp176 + fTemp68 * (fTemp177 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp174, 4)) as usize] } - fTemp175))))} else {1.0 - (fTemp170 + fTemp68 * fTemp171 + (fTemp166 - (iTemp167) as F64) * (fTemp169 - (fTemp170 + fTemp68 * (fTemp171 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp168, 4)) as usize] } - fTemp169)))))};
			let mut fTemp179: F64 = fTemp73 + fTemp165;
			let mut fTemp180: F64 = 65535.0 * (1.0 - fTemp179);
			let mut iTemp181: i32 = (fTemp180) as i32;
			let mut iTemp182: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp181, 65535)))), 196607));
			let mut fTemp183: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp182, 3)) as usize] };
			let mut fTemp184: F64 = unsafe { ftbl0mydspSIG0[iTemp182 as usize] };
			let mut fTemp185: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp182, 1)) as usize] } - fTemp184;
			let mut fTemp186: F64 = 65535.0 * fTemp179;
			let mut iTemp187: i32 = (fTemp186) as i32;
			let mut iTemp188: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp187, 65535)))), 196607));
			let mut fTemp189: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp188, 3), 196607))) as usize] };
			let mut fTemp190: F64 = unsafe { ftbl0mydspSIG0[iTemp188 as usize] };
			let mut fTemp191: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp188, 1), 196607))) as usize] } - fTemp190;
			let mut iTemp192: i32 = (fTemp129 > ((if iTemp54 != 0 {fTemp190 + fTemp68 * fTemp191 + (fTemp186 - (iTemp187) as F64) * (fTemp189 - (fTemp190 + fTemp68 * (fTemp191 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp188, 4), 196607))) as usize] } - fTemp189))))} else {1.0 - (fTemp184 + fTemp68 * fTemp185 + (fTemp180 - (iTemp181) as F64) * (fTemp183 - (fTemp184 + fTemp68 * (fTemp185 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp182, 4)) as usize] } - fTemp183)))))} - fTemp178) / (1.0 - fTemp178))) as i32;
			let mut fTemp193: F64 = if iTemp192 != 0 {fTemp162} else {fTemp165};
			let mut fTemp194: F64 = if iTemp192 != 0 {fTemp165} else {fTemp163};
			let mut fTemp195: F64 = fTemp194 + fTemp193;
			let mut fTemp196: F64 = 0.5 * fTemp195;
			let mut fTemp197: F64 = 65535.0 * (1.0 - fTemp196);
			let mut iTemp198: i32 = (fTemp197) as i32;
			let mut iTemp199: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp198, 65535)))), 196607));
			let mut fTemp200: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp199, 3)) as usize] };
			let mut fTemp201: F64 = unsafe { ftbl0mydspSIG0[iTemp199 as usize] };
			let mut fTemp202: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp199, 1)) as usize] } - fTemp201;
			let mut fTemp203: F64 = 32767.5 * fTemp195;
			let mut iTemp204: i32 = (fTemp203) as i32;
			let mut iTemp205: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp204, 65535)))), 196607));
			let mut fTemp206: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp205, 3)) as usize] };
			let mut fTemp207: F64 = unsafe { ftbl0mydspSIG0[iTemp205 as usize] };
			let mut fTemp208: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp205, 1)) as usize] } - fTemp207;
			let mut fTemp209: F64 = if iTemp54 != 0 {fTemp207 + fTemp68 * fTemp208 + (fTemp203 - (iTemp204) as F64) * (fTemp206 - (fTemp207 + fTemp68 * (fTemp208 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp205, 4)) as usize] } - fTemp206))))} else {1.0 - (fTemp201 + fTemp68 * fTemp202 + (fTemp197 - (iTemp198) as F64) * (fTemp200 - (fTemp201 + fTemp68 * (fTemp202 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp199, 4)) as usize] } - fTemp200)))))};
			let mut fTemp210: F64 = fTemp73 + fTemp196;
			let mut fTemp211: F64 = 65535.0 * (1.0 - fTemp210);
			let mut iTemp212: i32 = (fTemp211) as i32;
			let mut iTemp213: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp212, 65535)))), 196607));
			let mut fTemp214: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp213, 3)) as usize] };
			let mut fTemp215: F64 = unsafe { ftbl0mydspSIG0[iTemp213 as usize] };
			let mut fTemp216: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp213, 1)) as usize] } - fTemp215;
			let mut fTemp217: F64 = 65535.0 * fTemp210;
			let mut iTemp218: i32 = (fTemp217) as i32;
			let mut iTemp219: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp218, 65535)))), 196607));
			let mut fTemp220: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp219, 3), 196607))) as usize] };
			let mut fTemp221: F64 = unsafe { ftbl0mydspSIG0[iTemp219 as usize] };
			let mut fTemp222: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp219, 1), 196607))) as usize] } - fTemp221;
			let mut iTemp223: i32 = (fTemp129 > ((if iTemp54 != 0 {fTemp221 + fTemp68 * fTemp222 + (fTemp217 - (iTemp218) as F64) * (fTemp220 - (fTemp221 + fTemp68 * (fTemp222 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp219, 4), 196607))) as usize] } - fTemp220))))} else {1.0 - (fTemp215 + fTemp68 * fTemp216 + (fTemp211 - (iTemp212) as F64) * (fTemp214 - (fTemp215 + fTemp68 * (fTemp216 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp213, 4)) as usize] } - fTemp214)))))} - fTemp209) / (1.0 - fTemp209))) as i32;
			let mut fTemp224: F64 = if iTemp223 != 0 {fTemp193} else {fTemp196};
			let mut fTemp225: F64 = if iTemp223 != 0 {fTemp196} else {fTemp194};
			let mut fTemp226: F64 = fTemp225 + fTemp224;
			let mut fTemp227: F64 = 0.5 * fTemp226;
			let mut fTemp228: F64 = 65535.0 * (1.0 - fTemp227);
			let mut iTemp229: i32 = (fTemp228) as i32;
			let mut iTemp230: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp229, 65535)))), 196607));
			let mut fTemp231: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp230, 3)) as usize] };
			let mut fTemp232: F64 = unsafe { ftbl0mydspSIG0[iTemp230 as usize] };
			let mut fTemp233: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp230, 1)) as usize] } - fTemp232;
			let mut fTemp234: F64 = 32767.5 * fTemp226;
			let mut iTemp235: i32 = (fTemp234) as i32;
			let mut iTemp236: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp235, 65535)))), 196607));
			let mut fTemp237: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp236, 3)) as usize] };
			let mut fTemp238: F64 = unsafe { ftbl0mydspSIG0[iTemp236 as usize] };
			let mut fTemp239: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp236, 1)) as usize] } - fTemp238;
			let mut fTemp240: F64 = if iTemp54 != 0 {fTemp238 + fTemp68 * fTemp239 + (fTemp234 - (iTemp235) as F64) * (fTemp237 - (fTemp238 + fTemp68 * (fTemp239 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp236, 4)) as usize] } - fTemp237))))} else {1.0 - (fTemp232 + fTemp68 * fTemp233 + (fTemp228 - (iTemp229) as F64) * (fTemp231 - (fTemp232 + fTemp68 * (fTemp233 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp230, 4)) as usize] } - fTemp231)))))};
			let mut fTemp241: F64 = fTemp73 + fTemp227;
			let mut fTemp242: F64 = 65535.0 * (1.0 - fTemp241);
			let mut iTemp243: i32 = (fTemp242) as i32;
			let mut iTemp244: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp243, 65535)))), 196607));
			let mut fTemp245: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp244, 3)) as usize] };
			let mut fTemp246: F64 = unsafe { ftbl0mydspSIG0[iTemp244 as usize] };
			let mut fTemp247: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp244, 1)) as usize] } - fTemp246;
			let mut fTemp248: F64 = 65535.0 * fTemp241;
			let mut iTemp249: i32 = (fTemp248) as i32;
			let mut iTemp250: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp249, 65535)))), 196607));
			let mut fTemp251: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp250, 3), 196607))) as usize] };
			let mut fTemp252: F64 = unsafe { ftbl0mydspSIG0[iTemp250 as usize] };
			let mut fTemp253: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp250, 1), 196607))) as usize] } - fTemp252;
			let mut iTemp254: i32 = (fTemp129 > ((if iTemp54 != 0 {fTemp252 + fTemp68 * fTemp253 + (fTemp248 - (iTemp249) as F64) * (fTemp251 - (fTemp252 + fTemp68 * (fTemp253 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp250, 4), 196607))) as usize] } - fTemp251))))} else {1.0 - (fTemp246 + fTemp68 * fTemp247 + (fTemp242 - (iTemp243) as F64) * (fTemp245 - (fTemp246 + fTemp68 * (fTemp247 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp244, 4)) as usize] } - fTemp245)))))} - fTemp240) / (1.0 - fTemp240))) as i32;
			let mut fTemp255: F64 = if iTemp254 != 0 {fTemp224} else {fTemp227};
			let mut fTemp256: F64 = if iTemp254 != 0 {fTemp227} else {fTemp225};
			let mut fTemp257: F64 = fTemp256 + fTemp255;
			let mut fTemp258: F64 = 0.5 * fTemp257;
			let mut fTemp259: F64 = 65535.0 * (1.0 - fTemp258);
			let mut iTemp260: i32 = (fTemp259) as i32;
			let mut iTemp261: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp260, 65535)))), 196607));
			let mut fTemp262: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp261, 3)) as usize] };
			let mut fTemp263: F64 = unsafe { ftbl0mydspSIG0[iTemp261 as usize] };
			let mut fTemp264: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp261, 1)) as usize] } - fTemp263;
			let mut fTemp265: F64 = 32767.5 * fTemp257;
			let mut iTemp266: i32 = (fTemp265) as i32;
			let mut iTemp267: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp266, 65535)))), 196607));
			let mut fTemp268: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp267, 3)) as usize] };
			let mut fTemp269: F64 = unsafe { ftbl0mydspSIG0[iTemp267 as usize] };
			let mut fTemp270: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp267, 1)) as usize] } - fTemp269;
			let mut fTemp271: F64 = if iTemp54 != 0 {fTemp269 + fTemp68 * fTemp270 + (fTemp265 - (iTemp266) as F64) * (fTemp268 - (fTemp269 + fTemp68 * (fTemp270 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp267, 4)) as usize] } - fTemp268))))} else {1.0 - (fTemp263 + fTemp68 * fTemp264 + (fTemp259 - (iTemp260) as F64) * (fTemp262 - (fTemp263 + fTemp68 * (fTemp264 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp261, 4)) as usize] } - fTemp262)))))};
			let mut fTemp272: F64 = fTemp73 + fTemp258;
			let mut fTemp273: F64 = 65535.0 * (1.0 - fTemp272);
			let mut iTemp274: i32 = (fTemp273) as i32;
			let mut iTemp275: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp274, 65535)))), 196607));
			let mut fTemp276: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp275, 3)) as usize] };
			let mut fTemp277: F64 = unsafe { ftbl0mydspSIG0[iTemp275 as usize] };
			let mut fTemp278: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp275, 1)) as usize] } - fTemp277;
			let mut fTemp279: F64 = 65535.0 * fTemp272;
			let mut iTemp280: i32 = (fTemp279) as i32;
			let mut iTemp281: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp280, 65535)))), 196607));
			let mut fTemp282: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp281, 3), 196607))) as usize] };
			let mut fTemp283: F64 = unsafe { ftbl0mydspSIG0[iTemp281 as usize] };
			let mut fTemp284: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp281, 1), 196607))) as usize] } - fTemp283;
			let mut iTemp285: i32 = (fTemp129 > ((if iTemp54 != 0 {fTemp283 + fTemp68 * fTemp284 + (fTemp279 - (iTemp280) as F64) * (fTemp282 - (fTemp283 + fTemp68 * (fTemp284 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp281, 4), 196607))) as usize] } - fTemp282))))} else {1.0 - (fTemp277 + fTemp68 * fTemp278 + (fTemp273 - (iTemp274) as F64) * (fTemp276 - (fTemp277 + fTemp68 * (fTemp278 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp275, 4)) as usize] } - fTemp276)))))} - fTemp271) / (1.0 - fTemp271))) as i32;
			let mut fTemp286: F64 = if iTemp285 != 0 {fTemp255} else {fTemp258};
			let mut fTemp287: F64 = if iTemp285 != 0 {fTemp258} else {fTemp256};
			let mut fTemp288: F64 = fTemp287 + fTemp286;
			let mut fTemp289: F64 = 0.5 * fTemp288;
			let mut fTemp290: F64 = 65535.0 * (1.0 - fTemp289);
			let mut iTemp291: i32 = (fTemp290) as i32;
			let mut iTemp292: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp291, 65535)))), 196607));
			let mut fTemp293: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp292, 3)) as usize] };
			let mut fTemp294: F64 = unsafe { ftbl0mydspSIG0[iTemp292 as usize] };
			let mut fTemp295: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp292, 1)) as usize] } - fTemp294;
			let mut fTemp296: F64 = 32767.5 * fTemp288;
			let mut iTemp297: i32 = (fTemp296) as i32;
			let mut iTemp298: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp297, 65535)))), 196607));
			let mut fTemp299: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp298, 3)) as usize] };
			let mut fTemp300: F64 = unsafe { ftbl0mydspSIG0[iTemp298 as usize] };
			let mut fTemp301: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp298, 1)) as usize] } - fTemp300;
			let mut fTemp302: F64 = if iTemp54 != 0 {fTemp300 + fTemp68 * fTemp301 + (fTemp296 - (iTemp297) as F64) * (fTemp299 - (fTemp300 + fTemp68 * (fTemp301 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp298, 4)) as usize] } - fTemp299))))} else {1.0 - (fTemp294 + fTemp68 * fTemp295 + (fTemp290 - (iTemp291) as F64) * (fTemp293 - (fTemp294 + fTemp68 * (fTemp295 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp292, 4)) as usize] } - fTemp293)))))};
			let mut fTemp303: F64 = fTemp73 + fTemp289;
			let mut fTemp304: F64 = 65535.0 * (1.0 - fTemp303);
			let mut iTemp305: i32 = (fTemp304) as i32;
			let mut iTemp306: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp305, 65535)))), 196607));
			let mut fTemp307: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp306, 3)) as usize] };
			let mut fTemp308: F64 = unsafe { ftbl0mydspSIG0[iTemp306 as usize] };
			let mut fTemp309: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp306, 1)) as usize] } - fTemp308;
			let mut fTemp310: F64 = 65535.0 * fTemp303;
			let mut iTemp311: i32 = (fTemp310) as i32;
			let mut iTemp312: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp311, 65535)))), 196607));
			let mut fTemp313: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp312, 3), 196607))) as usize] };
			let mut fTemp314: F64 = unsafe { ftbl0mydspSIG0[iTemp312 as usize] };
			let mut fTemp315: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp312, 1), 196607))) as usize] } - fTemp314;
			let mut iTemp316: i32 = (fTemp129 > ((if iTemp54 != 0 {fTemp314 + fTemp68 * fTemp315 + (fTemp310 - (iTemp311) as F64) * (fTemp313 - (fTemp314 + fTemp68 * (fTemp315 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp312, 4), 196607))) as usize] } - fTemp313))))} else {1.0 - (fTemp308 + fTemp68 * fTemp309 + (fTemp304 - (iTemp305) as F64) * (fTemp307 - (fTemp308 + fTemp68 * (fTemp309 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp306, 4)) as usize] } - fTemp307)))))} - fTemp302) / (1.0 - fTemp302))) as i32;
			let mut fTemp317: F64 = if iTemp316 != 0 {fTemp286} else {fTemp289};
			let mut fTemp318: F64 = if iTemp316 != 0 {fTemp289} else {fTemp287};
			let mut fTemp319: F64 = fTemp318 + fTemp317;
			let mut fTemp320: F64 = 0.5 * fTemp319;
			let mut fTemp321: F64 = 65535.0 * (1.0 - fTemp320);
			let mut iTemp322: i32 = (fTemp321) as i32;
			let mut iTemp323: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp322, 65535)))), 196607));
			let mut fTemp324: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp323, 3)) as usize] };
			let mut fTemp325: F64 = unsafe { ftbl0mydspSIG0[iTemp323 as usize] };
			let mut fTemp326: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp323, 1)) as usize] } - fTemp325;
			let mut fTemp327: F64 = 32767.5 * fTemp319;
			let mut iTemp328: i32 = (fTemp327) as i32;
			let mut iTemp329: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp328, 65535)))), 196607));
			let mut fTemp330: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp329, 3)) as usize] };
			let mut fTemp331: F64 = unsafe { ftbl0mydspSIG0[iTemp329 as usize] };
			let mut fTemp332: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp329, 1)) as usize] } - fTemp331;
			let mut fTemp333: F64 = if iTemp54 != 0 {fTemp331 + fTemp68 * fTemp332 + (fTemp327 - (iTemp328) as F64) * (fTemp330 - (fTemp331 + fTemp68 * (fTemp332 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp329, 4)) as usize] } - fTemp330))))} else {1.0 - (fTemp325 + fTemp68 * fTemp326 + (fTemp321 - (iTemp322) as F64) * (fTemp324 - (fTemp325 + fTemp68 * (fTemp326 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp323, 4)) as usize] } - fTemp324)))))};
			let mut fTemp334: F64 = fTemp73 + fTemp320;
			let mut fTemp335: F64 = 65535.0 * (1.0 - fTemp334);
			let mut iTemp336: i32 = (fTemp335) as i32;
			let mut iTemp337: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp336, 65535)))), 196607));
			let mut fTemp338: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp337, 3)) as usize] };
			let mut fTemp339: F64 = unsafe { ftbl0mydspSIG0[iTemp337 as usize] };
			let mut fTemp340: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp337, 1)) as usize] } - fTemp339;
			let mut fTemp341: F64 = 65535.0 * fTemp334;
			let mut iTemp342: i32 = (fTemp341) as i32;
			let mut iTemp343: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp342, 65535)))), 196607));
			let mut fTemp344: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp343, 3), 196607))) as usize] };
			let mut fTemp345: F64 = unsafe { ftbl0mydspSIG0[iTemp343 as usize] };
			let mut fTemp346: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp343, 1), 196607))) as usize] } - fTemp345;
			let mut iTemp347: i32 = (fTemp129 > ((if iTemp54 != 0 {fTemp345 + fTemp68 * fTemp346 + (fTemp341 - (iTemp342) as F64) * (fTemp344 - (fTemp345 + fTemp68 * (fTemp346 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp343, 4), 196607))) as usize] } - fTemp344))))} else {1.0 - (fTemp339 + fTemp68 * fTemp340 + (fTemp335 - (iTemp336) as F64) * (fTemp338 - (fTemp339 + fTemp68 * (fTemp340 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp337, 4)) as usize] } - fTemp338)))))} - fTemp333) / (1.0 - fTemp333))) as i32;
			let mut fTemp348: F64 = if iTemp347 != 0 {fTemp317} else {fTemp320};
			let mut fTemp349: F64 = if iTemp347 != 0 {fTemp320} else {fTemp318};
			let mut fTemp350: F64 = fTemp349 + fTemp348;
			let mut fTemp351: F64 = 0.5 * fTemp350;
			let mut fTemp352: F64 = 65535.0 * (1.0 - fTemp351);
			let mut iTemp353: i32 = (fTemp352) as i32;
			let mut iTemp354: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp353, 65535)))), 196607));
			let mut fTemp355: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp354, 3)) as usize] };
			let mut fTemp356: F64 = unsafe { ftbl0mydspSIG0[iTemp354 as usize] };
			let mut fTemp357: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp354, 1)) as usize] } - fTemp356;
			let mut fTemp358: F64 = 32767.5 * fTemp350;
			let mut iTemp359: i32 = (fTemp358) as i32;
			let mut iTemp360: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp359, 65535)))), 196607));
			let mut fTemp361: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp360, 3)) as usize] };
			let mut fTemp362: F64 = unsafe { ftbl0mydspSIG0[iTemp360 as usize] };
			let mut fTemp363: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp360, 1)) as usize] } - fTemp362;
			let mut fTemp364: F64 = if iTemp54 != 0 {fTemp362 + fTemp68 * fTemp363 + (fTemp358 - (iTemp359) as F64) * (fTemp361 - (fTemp362 + fTemp68 * (fTemp363 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp360, 4)) as usize] } - fTemp361))))} else {1.0 - (fTemp356 + fTemp68 * fTemp357 + (fTemp352 - (iTemp353) as F64) * (fTemp355 - (fTemp356 + fTemp68 * (fTemp357 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp354, 4)) as usize] } - fTemp355)))))};
			let mut fTemp365: F64 = fTemp73 + fTemp351;
			let mut fTemp366: F64 = 65535.0 * (1.0 - fTemp365);
			let mut iTemp367: i32 = (fTemp366) as i32;
			let mut iTemp368: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp367, 65535)))), 196607));
			let mut fTemp369: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp368, 3)) as usize] };
			let mut fTemp370: F64 = unsafe { ftbl0mydspSIG0[iTemp368 as usize] };
			let mut fTemp371: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp368, 1)) as usize] } - fTemp370;
			let mut fTemp372: F64 = 65535.0 * fTemp365;
			let mut iTemp373: i32 = (fTemp372) as i32;
			let mut iTemp374: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp373, 65535)))), 196607));
			let mut fTemp375: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp374, 3), 196607))) as usize] };
			let mut fTemp376: F64 = unsafe { ftbl0mydspSIG0[iTemp374 as usize] };
			let mut fTemp377: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp374, 1), 196607))) as usize] } - fTemp376;
			let mut iTemp378: i32 = (fTemp129 > ((if iTemp54 != 0 {fTemp376 + fTemp68 * fTemp377 + (fTemp372 - (iTemp373) as F64) * (fTemp375 - (fTemp376 + fTemp68 * (fTemp377 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp374, 4), 196607))) as usize] } - fTemp375))))} else {1.0 - (fTemp370 + fTemp68 * fTemp371 + (fTemp366 - (iTemp367) as F64) * (fTemp369 - (fTemp370 + fTemp68 * (fTemp371 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp368, 4)) as usize] } - fTemp369)))))} - fTemp364) / (1.0 - fTemp364))) as i32;
			let mut fTemp379: F64 = if iTemp378 != 0 {fTemp348} else {fTemp351};
			let mut fTemp380: F64 = if iTemp378 != 0 {fTemp351} else {fTemp349};
			let mut fTemp381: F64 = fTemp380 + fTemp379;
			let mut fTemp382: F64 = 0.5 * fTemp381;
			let mut fTemp383: F64 = 65535.0 * (1.0 - fTemp382);
			let mut iTemp384: i32 = (fTemp383) as i32;
			let mut iTemp385: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp384, 65535)))), 196607));
			let mut fTemp386: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp385, 3)) as usize] };
			let mut fTemp387: F64 = unsafe { ftbl0mydspSIG0[iTemp385 as usize] };
			let mut fTemp388: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp385, 1)) as usize] } - fTemp387;
			let mut fTemp389: F64 = 32767.5 * fTemp381;
			let mut iTemp390: i32 = (fTemp389) as i32;
			let mut iTemp391: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp390, 65535)))), 196607));
			let mut fTemp392: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp391, 3)) as usize] };
			let mut fTemp393: F64 = unsafe { ftbl0mydspSIG0[iTemp391 as usize] };
			let mut fTemp394: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp391, 1)) as usize] } - fTemp393;
			let mut fTemp395: F64 = if iTemp54 != 0 {fTemp393 + fTemp68 * fTemp394 + (fTemp389 - (iTemp390) as F64) * (fTemp392 - (fTemp393 + fTemp68 * (fTemp394 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp391, 4)) as usize] } - fTemp392))))} else {1.0 - (fTemp387 + fTemp68 * fTemp388 + (fTemp383 - (iTemp384) as F64) * (fTemp386 - (fTemp387 + fTemp68 * (fTemp388 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp385, 4)) as usize] } - fTemp386)))))};
			let mut fTemp396: F64 = fTemp73 + fTemp382;
			let mut fTemp397: F64 = 65535.0 * (1.0 - fTemp396);
			let mut iTemp398: i32 = (fTemp397) as i32;
			let mut iTemp399: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp398, 65535)))), 196607));
			let mut fTemp400: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp399, 3)) as usize] };
			let mut fTemp401: F64 = unsafe { ftbl0mydspSIG0[iTemp399 as usize] };
			let mut fTemp402: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp399, 1)) as usize] } - fTemp401;
			let mut fTemp403: F64 = 65535.0 * fTemp396;
			let mut iTemp404: i32 = (fTemp403) as i32;
			let mut iTemp405: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp404, 65535)))), 196607));
			let mut fTemp406: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp405, 3), 196607))) as usize] };
			let mut fTemp407: F64 = unsafe { ftbl0mydspSIG0[iTemp405 as usize] };
			let mut fTemp408: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp405, 1), 196607))) as usize] } - fTemp407;
			let mut iTemp409: i32 = (fTemp129 > ((if iTemp54 != 0 {fTemp407 + fTemp68 * fTemp408 + (fTemp403 - (iTemp404) as F64) * (fTemp406 - (fTemp407 + fTemp68 * (fTemp408 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp405, 4), 196607))) as usize] } - fTemp406))))} else {1.0 - (fTemp401 + fTemp68 * fTemp402 + (fTemp397 - (iTemp398) as F64) * (fTemp400 - (fTemp401 + fTemp68 * (fTemp402 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp399, 4)) as usize] } - fTemp400)))))} - fTemp395) / (1.0 - fTemp395))) as i32;
			let mut fTemp410: F64 = if iTemp409 != 0 {fTemp379} else {fTemp382};
			let mut fTemp411: F64 = if iTemp409 != 0 {fTemp382} else {fTemp380};
			let mut fTemp412: F64 = fTemp411 + fTemp410;
			let mut fTemp413: F64 = 0.5 * fTemp412;
			let mut fTemp414: F64 = 65535.0 * (1.0 - fTemp413);
			let mut iTemp415: i32 = (fTemp414) as i32;
			let mut iTemp416: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp415, 65535)))), 196607));
			let mut fTemp417: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp416, 3)) as usize] };
			let mut fTemp418: F64 = unsafe { ftbl0mydspSIG0[iTemp416 as usize] };
			let mut fTemp419: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp416, 1)) as usize] } - fTemp418;
			let mut fTemp420: F64 = 32767.5 * fTemp412;
			let mut iTemp421: i32 = (fTemp420) as i32;
			let mut iTemp422: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp421, 65535)))), 196607));
			let mut fTemp423: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp422, 3)) as usize] };
			let mut fTemp424: F64 = unsafe { ftbl0mydspSIG0[iTemp422 as usize] };
			let mut fTemp425: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp422, 1)) as usize] } - fTemp424;
			let mut fTemp426: F64 = if iTemp54 != 0 {fTemp424 + fTemp68 * fTemp425 + (fTemp420 - (iTemp421) as F64) * (fTemp423 - (fTemp424 + fTemp68 * (fTemp425 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp422, 4)) as usize] } - fTemp423))))} else {1.0 - (fTemp418 + fTemp68 * fTemp419 + (fTemp414 - (iTemp415) as F64) * (fTemp417 - (fTemp418 + fTemp68 * (fTemp419 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp416, 4)) as usize] } - fTemp417)))))};
			let mut fTemp427: F64 = fTemp73 + fTemp413;
			let mut fTemp428: F64 = 65535.0 * (1.0 - fTemp427);
			let mut iTemp429: i32 = (fTemp428) as i32;
			let mut iTemp430: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp429, 65535)))), 196607));
			let mut fTemp431: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp430, 3)) as usize] };
			let mut fTemp432: F64 = unsafe { ftbl0mydspSIG0[iTemp430 as usize] };
			let mut fTemp433: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp430, 1)) as usize] } - fTemp432;
			let mut fTemp434: F64 = 65535.0 * fTemp427;
			let mut iTemp435: i32 = (fTemp434) as i32;
			let mut iTemp436: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp435, 65535)))), 196607));
			let mut fTemp437: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp436, 3), 196607))) as usize] };
			let mut fTemp438: F64 = unsafe { ftbl0mydspSIG0[iTemp436 as usize] };
			let mut fTemp439: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp436, 1), 196607))) as usize] } - fTemp438;
			let mut iTemp440: i32 = (fTemp129 > ((if iTemp54 != 0 {fTemp438 + fTemp68 * fTemp439 + (fTemp434 - (iTemp435) as F64) * (fTemp437 - (fTemp438 + fTemp68 * (fTemp439 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp436, 4), 196607))) as usize] } - fTemp437))))} else {1.0 - (fTemp432 + fTemp68 * fTemp433 + (fTemp428 - (iTemp429) as F64) * (fTemp431 - (fTemp432 + fTemp68 * (fTemp433 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp430, 4)) as usize] } - fTemp431)))))} - fTemp426) / (1.0 - fTemp426))) as i32;
			let mut fTemp441: F64 = if iTemp440 != 0 {fTemp410} else {fTemp413};
			let mut fTemp442: F64 = if iTemp440 != 0 {fTemp413} else {fTemp411};
			let mut fTemp443: F64 = fTemp442 + fTemp441;
			let mut fTemp444: F64 = 0.5 * fTemp443;
			let mut fTemp445: F64 = 65535.0 * (1.0 - fTemp444);
			let mut iTemp446: i32 = (fTemp445) as i32;
			let mut iTemp447: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp446, 65535)))), 196607));
			let mut fTemp448: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp447, 3)) as usize] };
			let mut fTemp449: F64 = unsafe { ftbl0mydspSIG0[iTemp447 as usize] };
			let mut fTemp450: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp447, 1)) as usize] } - fTemp449;
			let mut fTemp451: F64 = 32767.5 * fTemp443;
			let mut iTemp452: i32 = (fTemp451) as i32;
			let mut iTemp453: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp452, 65535)))), 196607));
			let mut fTemp454: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp453, 3)) as usize] };
			let mut fTemp455: F64 = unsafe { ftbl0mydspSIG0[iTemp453 as usize] };
			let mut fTemp456: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp453, 1)) as usize] } - fTemp455;
			let mut fTemp457: F64 = if iTemp54 != 0 {fTemp455 + fTemp68 * fTemp456 + (fTemp451 - (iTemp452) as F64) * (fTemp454 - (fTemp455 + fTemp68 * (fTemp456 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp453, 4)) as usize] } - fTemp454))))} else {1.0 - (fTemp449 + fTemp68 * fTemp450 + (fTemp445 - (iTemp446) as F64) * (fTemp448 - (fTemp449 + fTemp68 * (fTemp450 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp447, 4)) as usize] } - fTemp448)))))};
			let mut fTemp458: F64 = fTemp73 + fTemp444;
			let mut fTemp459: F64 = 65535.0 * (1.0 - fTemp458);
			let mut iTemp460: i32 = (fTemp459) as i32;
			let mut iTemp461: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp460, 65535)))), 196607));
			let mut fTemp462: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp461, 3)) as usize] };
			let mut fTemp463: F64 = unsafe { ftbl0mydspSIG0[iTemp461 as usize] };
			let mut fTemp464: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp461, 1)) as usize] } - fTemp463;
			let mut fTemp465: F64 = 65535.0 * fTemp458;
			let mut iTemp466: i32 = (fTemp465) as i32;
			let mut iTemp467: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp466, 65535)))), 196607));
			let mut fTemp468: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp467, 3), 196607))) as usize] };
			let mut fTemp469: F64 = unsafe { ftbl0mydspSIG0[iTemp467 as usize] };
			let mut fTemp470: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp467, 1), 196607))) as usize] } - fTemp469;
			let mut iTemp471: i32 = (fTemp129 > ((if iTemp54 != 0 {fTemp469 + fTemp68 * fTemp470 + (fTemp465 - (iTemp466) as F64) * (fTemp468 - (fTemp469 + fTemp68 * (fTemp470 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp467, 4), 196607))) as usize] } - fTemp468))))} else {1.0 - (fTemp463 + fTemp68 * fTemp464 + (fTemp459 - (iTemp460) as F64) * (fTemp462 - (fTemp463 + fTemp68 * (fTemp464 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp461, 4)) as usize] } - fTemp462)))))} - fTemp457) / (1.0 - fTemp457))) as i32;
			let mut fTemp472: F64 = if iTemp471 != 0 {fTemp441} else {fTemp444};
			let mut fTemp473: F64 = if iTemp471 != 0 {fTemp444} else {fTemp442};
			let mut fTemp474: F64 = fTemp473 + fTemp472;
			let mut fTemp475: F64 = 0.5 * fTemp474;
			let mut fTemp476: F64 = 65535.0 * (1.0 - fTemp475);
			let mut iTemp477: i32 = (fTemp476) as i32;
			let mut iTemp478: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp477, 65535)))), 196607));
			let mut fTemp479: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp478, 3)) as usize] };
			let mut fTemp480: F64 = unsafe { ftbl0mydspSIG0[iTemp478 as usize] };
			let mut fTemp481: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp478, 1)) as usize] } - fTemp480;
			let mut fTemp482: F64 = 32767.5 * fTemp474;
			let mut iTemp483: i32 = (fTemp482) as i32;
			let mut iTemp484: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp483, 65535)))), 196607));
			let mut fTemp485: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp484, 3)) as usize] };
			let mut fTemp486: F64 = unsafe { ftbl0mydspSIG0[iTemp484 as usize] };
			let mut fTemp487: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp484, 1)) as usize] } - fTemp486;
			let mut fTemp488: F64 = if iTemp54 != 0 {fTemp486 + fTemp68 * fTemp487 + (fTemp482 - (iTemp483) as F64) * (fTemp485 - (fTemp486 + fTemp68 * (fTemp487 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp484, 4)) as usize] } - fTemp485))))} else {1.0 - (fTemp480 + fTemp68 * fTemp481 + (fTemp476 - (iTemp477) as F64) * (fTemp479 - (fTemp480 + fTemp68 * (fTemp481 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp478, 4)) as usize] } - fTemp479)))))};
			let mut fTemp489: F64 = fTemp73 + fTemp475;
			let mut fTemp490: F64 = 65535.0 * (1.0 - fTemp489);
			let mut iTemp491: i32 = (fTemp490) as i32;
			let mut iTemp492: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp491, 65535)))), 196607));
			let mut fTemp493: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp492, 3)) as usize] };
			let mut fTemp494: F64 = unsafe { ftbl0mydspSIG0[iTemp492 as usize] };
			let mut fTemp495: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp492, 1)) as usize] } - fTemp494;
			let mut fTemp496: F64 = 65535.0 * fTemp489;
			let mut iTemp497: i32 = (fTemp496) as i32;
			let mut iTemp498: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp497, 65535)))), 196607));
			let mut fTemp499: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp498, 3), 196607))) as usize] };
			let mut fTemp500: F64 = unsafe { ftbl0mydspSIG0[iTemp498 as usize] };
			let mut fTemp501: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp498, 1), 196607))) as usize] } - fTemp500;
			let mut iTemp502: i32 = (fTemp129 > ((if iTemp54 != 0 {fTemp500 + fTemp68 * fTemp501 + (fTemp496 - (iTemp497) as F64) * (fTemp499 - (fTemp500 + fTemp68 * (fTemp501 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp498, 4), 196607))) as usize] } - fTemp499))))} else {1.0 - (fTemp494 + fTemp68 * fTemp495 + (fTemp490 - (iTemp491) as F64) * (fTemp493 - (fTemp494 + fTemp68 * (fTemp495 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp492, 4)) as usize] } - fTemp493)))))} - fTemp488) / (1.0 - fTemp488))) as i32;
			let mut fTemp503: F64 = if iTemp502 != 0 {fTemp472} else {fTemp475};
			let mut fTemp504: F64 = if iTemp502 != 0 {fTemp475} else {fTemp473};
			let mut fTemp505: F64 = fTemp504 + fTemp503;
			let mut fTemp506: F64 = 0.5 * fTemp505;
			let mut fTemp507: F64 = 65535.0 * (1.0 - fTemp506);
			let mut iTemp508: i32 = (fTemp507) as i32;
			let mut iTemp509: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp508, 65535)))), 196607));
			let mut fTemp510: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp509, 3)) as usize] };
			let mut fTemp511: F64 = unsafe { ftbl0mydspSIG0[iTemp509 as usize] };
			let mut fTemp512: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp509, 1)) as usize] } - fTemp511;
			let mut fTemp513: F64 = 32767.5 * fTemp505;
			let mut iTemp514: i32 = (fTemp513) as i32;
			let mut iTemp515: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp514, 65535)))), 196607));
			let mut fTemp516: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp515, 3)) as usize] };
			let mut fTemp517: F64 = unsafe { ftbl0mydspSIG0[iTemp515 as usize] };
			let mut fTemp518: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp515, 1)) as usize] } - fTemp517;
			let mut fTemp519: F64 = if iTemp54 != 0 {fTemp517 + fTemp68 * fTemp518 + (fTemp513 - (iTemp514) as F64) * (fTemp516 - (fTemp517 + fTemp68 * (fTemp518 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp515, 4)) as usize] } - fTemp516))))} else {1.0 - (fTemp511 + fTemp68 * fTemp512 + (fTemp507 - (iTemp508) as F64) * (fTemp510 - (fTemp511 + fTemp68 * (fTemp512 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp509, 4)) as usize] } - fTemp510)))))};
			let mut fTemp520: F64 = fTemp73 + fTemp506;
			let mut fTemp521: F64 = 65535.0 * (1.0 - fTemp520);
			let mut iTemp522: i32 = (fTemp521) as i32;
			let mut iTemp523: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp522, 65535)))), 196607));
			let mut fTemp524: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp523, 3)) as usize] };
			let mut fTemp525: F64 = unsafe { ftbl0mydspSIG0[iTemp523 as usize] };
			let mut fTemp526: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp523, 1)) as usize] } - fTemp525;
			let mut fTemp527: F64 = 65535.0 * fTemp520;
			let mut iTemp528: i32 = (fTemp527) as i32;
			let mut iTemp529: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp528, 65535)))), 196607));
			let mut fTemp530: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp529, 3), 196607))) as usize] };
			let mut fTemp531: F64 = unsafe { ftbl0mydspSIG0[iTemp529 as usize] };
			let mut fTemp532: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp529, 1), 196607))) as usize] } - fTemp531;
			let mut iTemp533: i32 = (fTemp129 > ((if iTemp54 != 0 {fTemp531 + fTemp68 * fTemp532 + (fTemp527 - (iTemp528) as F64) * (fTemp530 - (fTemp531 + fTemp68 * (fTemp532 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp529, 4), 196607))) as usize] } - fTemp530))))} else {1.0 - (fTemp525 + fTemp68 * fTemp526 + (fTemp521 - (iTemp522) as F64) * (fTemp524 - (fTemp525 + fTemp68 * (fTemp526 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp523, 4)) as usize] } - fTemp524)))))} - fTemp519) / (1.0 - fTemp519))) as i32;
			let mut fTemp534: F64 = if iTemp533 != 0 {fTemp503} else {fTemp506};
			let mut fTemp535: F64 = if iTemp533 != 0 {fTemp506} else {fTemp504};
			let mut fTemp536: F64 = fTemp535 + fTemp534;
			let mut fTemp537: F64 = 0.5 * fTemp536;
			let mut fTemp538: F64 = 65535.0 * (1.0 - fTemp537);
			let mut iTemp539: i32 = (fTemp538) as i32;
			let mut iTemp540: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp539, 65535)))), 196607));
			let mut fTemp541: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp540, 3)) as usize] };
			let mut fTemp542: F64 = unsafe { ftbl0mydspSIG0[iTemp540 as usize] };
			let mut fTemp543: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp540, 1)) as usize] } - fTemp542;
			let mut fTemp544: F64 = 32767.5 * fTemp536;
			let mut iTemp545: i32 = (fTemp544) as i32;
			let mut iTemp546: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp545, 65535)))), 196607));
			let mut fTemp547: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp546, 3)) as usize] };
			let mut fTemp548: F64 = unsafe { ftbl0mydspSIG0[iTemp546 as usize] };
			let mut fTemp549: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp546, 1)) as usize] } - fTemp548;
			let mut fTemp550: F64 = if iTemp54 != 0 {fTemp548 + fTemp68 * fTemp549 + (fTemp544 - (iTemp545) as F64) * (fTemp547 - (fTemp548 + fTemp68 * (fTemp549 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp546, 4)) as usize] } - fTemp547))))} else {1.0 - (fTemp542 + fTemp68 * fTemp543 + (fTemp538 - (iTemp539) as F64) * (fTemp541 - (fTemp542 + fTemp68 * (fTemp543 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp540, 4)) as usize] } - fTemp541)))))};
			let mut fTemp551: F64 = fTemp73 + fTemp537;
			let mut fTemp552: F64 = 65535.0 * (1.0 - fTemp551);
			let mut iTemp553: i32 = (fTemp552) as i32;
			let mut iTemp554: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp553, 65535)))), 196607));
			let mut fTemp555: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp554, 3)) as usize] };
			let mut fTemp556: F64 = unsafe { ftbl0mydspSIG0[iTemp554 as usize] };
			let mut fTemp557: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp554, 1)) as usize] } - fTemp556;
			let mut fTemp558: F64 = 65535.0 * fTemp551;
			let mut iTemp559: i32 = (fTemp558) as i32;
			let mut iTemp560: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp559, 65535)))), 196607));
			let mut fTemp561: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp560, 3), 196607))) as usize] };
			let mut fTemp562: F64 = unsafe { ftbl0mydspSIG0[iTemp560 as usize] };
			let mut fTemp563: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp560, 1), 196607))) as usize] } - fTemp562;
			let mut iTemp564: i32 = (fTemp129 > ((if iTemp54 != 0 {fTemp562 + fTemp68 * fTemp563 + (fTemp558 - (iTemp559) as F64) * (fTemp561 - (fTemp562 + fTemp68 * (fTemp563 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp560, 4), 196607))) as usize] } - fTemp561))))} else {1.0 - (fTemp556 + fTemp68 * fTemp557 + (fTemp552 - (iTemp553) as F64) * (fTemp555 - (fTemp556 + fTemp68 * (fTemp557 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp554, 4)) as usize] } - fTemp555)))))} - fTemp550) / (1.0 - fTemp550))) as i32;
			let mut fTemp565: F64 = if iTemp564 != 0 {fTemp534} else {fTemp537};
			let mut fTemp566: F64 = if iTemp564 != 0 {fTemp537} else {fTemp535};
			let mut fTemp567: F64 = fTemp566 + fTemp565;
			let mut fTemp568: F64 = 0.5 * fTemp567;
			let mut fTemp569: F64 = 65535.0 * (1.0 - fTemp568);
			let mut iTemp570: i32 = (fTemp569) as i32;
			let mut iTemp571: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp570, 65535)))), 196607));
			let mut fTemp572: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp571, 3)) as usize] };
			let mut fTemp573: F64 = unsafe { ftbl0mydspSIG0[iTemp571 as usize] };
			let mut fTemp574: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp571, 1)) as usize] } - fTemp573;
			let mut fTemp575: F64 = 32767.5 * fTemp567;
			let mut iTemp576: i32 = (fTemp575) as i32;
			let mut iTemp577: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp576, 65535)))), 196607));
			let mut fTemp578: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp577, 3)) as usize] };
			let mut fTemp579: F64 = unsafe { ftbl0mydspSIG0[iTemp577 as usize] };
			let mut fTemp580: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp577, 1)) as usize] } - fTemp579;
			let mut fTemp581: F64 = if iTemp54 != 0 {fTemp579 + fTemp68 * fTemp580 + (fTemp575 - (iTemp576) as F64) * (fTemp578 - (fTemp579 + fTemp68 * (fTemp580 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp577, 4), 196607))) as usize] } - fTemp578))))} else {1.0 - (fTemp573 + fTemp68 * fTemp574 + (fTemp569 - (iTemp570) as F64) * (fTemp572 - (fTemp573 + fTemp68 * (fTemp574 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp571, 4), 196607))) as usize] } - fTemp572)))))};
			let mut fTemp582: F64 = fTemp73 + fTemp568;
			let mut fTemp583: F64 = 65535.0 * (1.0 - fTemp582);
			let mut iTemp584: i32 = (fTemp583) as i32;
			let mut iTemp585: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp584, 65535)))), 196607));
			let mut fTemp586: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp585, 3)) as usize] };
			let mut fTemp587: F64 = unsafe { ftbl0mydspSIG0[iTemp585 as usize] };
			let mut fTemp588: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp585, 1)) as usize] } - fTemp587;
			let mut fTemp589: F64 = 65535.0 * fTemp582;
			let mut iTemp590: i32 = (fTemp589) as i32;
			let mut iTemp591: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp590, 65535)))), 196607));
			let mut fTemp592: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp591, 3), 196607))) as usize] };
			let mut fTemp593: F64 = unsafe { ftbl0mydspSIG0[iTemp591 as usize] };
			let mut fTemp594: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp591, 1), 196607))) as usize] } - fTemp593;
			let mut iTemp595: i32 = (fTemp129 > ((if iTemp54 != 0 {fTemp593 + fTemp68 * fTemp594 + (fTemp589 - (iTemp590) as F64) * (fTemp592 - (fTemp593 + fTemp68 * (fTemp594 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp591, 4), 196607))) as usize] } - fTemp592))))} else {1.0 - (fTemp587 + fTemp68 * fTemp588 + (fTemp583 - (iTemp584) as F64) * (fTemp586 - (fTemp587 + fTemp68 * (fTemp588 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp585, 4)) as usize] } - fTemp586)))))} - fTemp581) / (1.0 - fTemp581))) as i32;
			let mut fTemp596: F64 = F64::min(1.0, F64::max(0.0, 0.5 * (if iTemp595 != 0 {fTemp568} else {fTemp566} + if iTemp595 != 0 {fTemp565} else {fTemp568})));
			self.fRec1[0] = fTemp596;
			let mut fTemp597: F64 = 65535.0 * (1.0 - fTemp596);
			let mut iTemp598: i32 = (fTemp597) as i32;
			let mut iTemp599: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp598, 65535)))), 196607));
			let mut fTemp600: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp599, 3)) as usize] };
			let mut fTemp601: F64 = unsafe { ftbl0mydspSIG0[iTemp599 as usize] };
			let mut fTemp602: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp599, 1)) as usize] } - fTemp601;
			let mut fTemp603: F64 = 65535.0 * fTemp596;
			let mut iTemp604: i32 = (fTemp603) as i32;
			let mut iTemp605: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp604, 65535)))), 196607));
			let mut fTemp606: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp605, 3)) as usize] };
			let mut fTemp607: F64 = unsafe { ftbl0mydspSIG0[iTemp605 as usize] };
			let mut fTemp608: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp605, 1)) as usize] } - fTemp607;
			let mut fTemp609: F64 = if iTemp54 != 0 {fTemp607 + fTemp68 * fTemp608 + (fTemp603 - (iTemp604) as F64) * (fTemp606 - (fTemp607 + fTemp68 * (fTemp608 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp605, 4), 196607))) as usize] } - fTemp606))))} else {1.0 - (fTemp601 + fTemp68 * fTemp602 + (fTemp597 - (iTemp598) as F64) * (fTemp600 - (fTemp601 + fTemp68 * (fTemp602 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp599, 4), 196607))) as usize] } - fTemp600)))))};
			let mut fTemp610: F64 = fTemp73 + fTemp596;
			let mut fTemp611: F64 = 65535.0 * (1.0 - fTemp610);
			let mut iTemp612: i32 = (fTemp611) as i32;
			let mut iTemp613: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp612, 65535)))), 196607));
			let mut fTemp614: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp613, 3)) as usize] };
			let mut fTemp615: F64 = unsafe { ftbl0mydspSIG0[iTemp613 as usize] };
			let mut fTemp616: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp613, 1)) as usize] } - fTemp615;
			let mut fTemp617: F64 = 65535.0 * fTemp610;
			let mut iTemp618: i32 = (fTemp617) as i32;
			let mut iTemp619: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp618, 65535)))), 196607));
			let mut fTemp620: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp619, 3), 196607))) as usize] };
			let mut fTemp621: F64 = unsafe { ftbl0mydspSIG0[iTemp619 as usize] };
			let mut fTemp622: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp619, 1), 196607))) as usize] } - fTemp621;
			let mut fTemp623: F64 = fTemp4 + if ((0.001 * fTemp72) == 0.0) as i32 != 0 {fTemp53} else {fTemp53 * (if iTemp54 != 0 {fTemp621 + fTemp68 * fTemp622 + (fTemp617 - (iTemp618) as F64) * (fTemp620 - (fTemp621 + fTemp68 * (fTemp622 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp619, 4), 196607))) as usize] } - fTemp620))))} else {1.0 - (fTemp615 + fTemp68 * fTemp616 + (fTemp611 - (iTemp612) as F64) * (fTemp614 - (fTemp615 + fTemp68 * (fTemp616 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp613, 4)) as usize] } - fTemp614)))))} - fTemp609) / (1.0 - fTemp609)};
			self.fRec2[(self.IOTA0 & 8191) as usize] = if iTemp71 != 0 {F64::min(fTemp623, fTemp4)} else {F64::max(fTemp623, fTemp4)};
			let mut fTemp624: F64 = self.fRec2[((i32::wrapping_sub(self.IOTA0, iSlow70)) & 8191) as usize];
			self.fHbargraph0 = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, fTemp624));
			self.fRec14[0] = fSlow72 + self.fConst4 * self.fRec14[1];
			*output0 = 0.5 * self.fVec0[((i32::wrapping_sub(self.IOTA0, iSlow71)) & 32767) as usize] * fTemp2 + self.fRec14[0] * self.fVec1[((i32::wrapping_sub(self.IOTA0, iSlow71)) & 32767) as usize] * fTemp624 * fTemp3;
			let mut fTemp625: F64 = self.fRec16[((i32::wrapping_sub(self.IOTA0, 1)) & 8191) as usize];
			let mut fTemp626: F64 = fTemp27 + fSlow17 * (fTemp28 - fTemp27);
			let mut iTemp627: i32 = ((fTemp626 > fSlow11) as i32) + ((fTemp626 > fSlow9) as i32);
			let mut fTemp628: F64 = fTemp626 - fSlow8;
			let mut fTemp629: F64 = F64::min(fTemp25, F64::powf(1e+01, -(fSlow18 * F64::max(0.0, if (iTemp627 == 0) as i32 != 0 {0.0} else {if (iTemp627 == 1) as i32 != 0 {fSlow12 * mydsp_faustpower2_f(fSlow7 + fTemp628)} else {fTemp628}}))));
			self.fVec30[(self.IOTA0 & 16383) as usize] = fTemp629;
			let mut fTemp630: F64 = F64::min(fTemp629, self.fVec30[((i32::wrapping_sub(self.IOTA0, 1)) & 16383) as usize]);
			self.fVec31[0] = fTemp630;
			let mut fTemp631: F64 = F64::min(fTemp630, self.fVec31[2]);
			self.fVec32[0] = fTemp631;
			let mut fTemp632: F64 = F64::min(fTemp631, self.fVec32[4]);
			self.fVec33[0] = fTemp632;
			let mut fTemp633: F64 = F64::min(fTemp632, self.fVec33[8]);
			self.fVec34[(self.IOTA0 & 31) as usize] = fTemp633;
			let mut fTemp634: F64 = F64::min(fTemp633, self.fVec34[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize]);
			self.fVec35[(self.IOTA0 & 63) as usize] = fTemp634;
			let mut fTemp635: F64 = F64::min(fTemp634, self.fVec35[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize]);
			self.fVec36[(self.IOTA0 & 127) as usize] = fTemp635;
			let mut fTemp636: F64 = F64::min(fTemp635, self.fVec36[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize]);
			self.fVec37[(self.IOTA0 & 255) as usize] = fTemp636;
			let mut fTemp637: F64 = F64::min(fTemp636, self.fVec37[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize]);
			self.fVec38[(self.IOTA0 & 511) as usize] = fTemp637;
			let mut fTemp638: F64 = F64::min(fTemp637, self.fVec38[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize]);
			self.fVec39[(self.IOTA0 & 1023) as usize] = fTemp638;
			let mut fTemp639: F64 = F64::min(fTemp638, self.fVec39[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize]);
			self.fVec40[(self.IOTA0 & 2047) as usize] = fTemp639;
			self.fVec41[(self.IOTA0 & 4095) as usize] = F64::min(fTemp639, self.fVec40[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			self.fRec17[0] = F64::max(F64::min(self.fRec17[1], self.fVec30[((i32::wrapping_sub(self.IOTA0, iSlow21)) & 16383) as usize]), F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow23 != 0 {fTemp629} else {1.7976931348623157e+308}, if iSlow24 != 0 {self.fVec31[iSlow23 as usize]} else {1.7976931348623157e+308}), if iSlow25 != 0 {self.fVec32[iSlow26 as usize]} else {1.7976931348623157e+308}), if iSlow27 != 0 {self.fVec33[iSlow28 as usize]} else {1.7976931348623157e+308}), if iSlow29 != 0 {self.fVec34[((i32::wrapping_sub(self.IOTA0, iSlow30)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow31 != 0 {self.fVec35[((i32::wrapping_sub(self.IOTA0, iSlow32)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow33 != 0 {self.fVec36[((i32::wrapping_sub(self.IOTA0, iSlow34)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow35 != 0 {self.fVec37[((i32::wrapping_sub(self.IOTA0, iSlow36)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow37 != 0 {self.fVec38[((i32::wrapping_sub(self.IOTA0, iSlow38)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow39 != 0 {self.fVec39[((i32::wrapping_sub(self.IOTA0, iSlow40)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow41 != 0 {self.fVec40[((i32::wrapping_sub(self.IOTA0, iSlow42)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow43 != 0 {self.fVec41[((i32::wrapping_sub(self.IOTA0, iSlow44)) & 4095) as usize]} else {1.7976931348623157e+308}));
			let mut fTemp640: F64 = F64::min(self.fRec17[0], self.fRec17[1]);
			self.fVec42[0] = fTemp640;
			let mut fTemp641: F64 = F64::min(fTemp640, self.fVec42[2]);
			self.fVec43[0] = fTemp641;
			let mut fTemp642: F64 = F64::min(fTemp641, self.fVec43[4]);
			self.fVec44[0] = fTemp642;
			let mut fTemp643: F64 = F64::min(fTemp642, self.fVec44[8]);
			self.fVec45[(self.IOTA0 & 31) as usize] = fTemp643;
			let mut fTemp644: F64 = F64::min(fTemp643, self.fVec45[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize]);
			self.fVec46[(self.IOTA0 & 63) as usize] = fTemp644;
			let mut fTemp645: F64 = F64::min(fTemp644, self.fVec46[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize]);
			self.fVec47[(self.IOTA0 & 127) as usize] = fTemp645;
			let mut fTemp646: F64 = F64::min(fTemp645, self.fVec47[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize]);
			self.fVec48[(self.IOTA0 & 255) as usize] = fTemp646;
			let mut fTemp647: F64 = F64::min(fTemp646, self.fVec48[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize]);
			self.fVec49[(self.IOTA0 & 511) as usize] = fTemp647;
			let mut fTemp648: F64 = F64::min(fTemp647, self.fVec49[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize]);
			self.fVec50[(self.IOTA0 & 1023) as usize] = fTemp648;
			let mut fTemp649: F64 = F64::min(fTemp648, self.fVec50[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize]);
			self.fVec51[(self.IOTA0 & 2047) as usize] = fTemp649;
			self.fVec52[(self.IOTA0 & 4095) as usize] = F64::min(fTemp649, self.fVec51[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			let mut fTemp650: F64 = F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow4 != 0 {self.fRec17[0]} else {1.7976931348623157e+308}, if iSlow45 != 0 {self.fVec42[iSlow4 as usize]} else {1.7976931348623157e+308}), if iSlow46 != 0 {self.fVec43[iSlow47 as usize]} else {1.7976931348623157e+308}), if iSlow48 != 0 {self.fVec44[iSlow49 as usize]} else {1.7976931348623157e+308}), if iSlow50 != 0 {self.fVec45[((i32::wrapping_sub(self.IOTA0, iSlow51)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow52 != 0 {self.fVec46[((i32::wrapping_sub(self.IOTA0, iSlow53)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow54 != 0 {self.fVec47[((i32::wrapping_sub(self.IOTA0, iSlow55)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow56 != 0 {self.fVec48[((i32::wrapping_sub(self.IOTA0, iSlow57)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow58 != 0 {self.fVec49[((i32::wrapping_sub(self.IOTA0, iSlow59)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow60 != 0 {self.fVec50[((i32::wrapping_sub(self.IOTA0, iSlow61)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow62 != 0 {self.fVec51[((i32::wrapping_sub(self.IOTA0, iSlow63)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow64 != 0 {self.fVec52[((i32::wrapping_sub(self.IOTA0, iSlow65)) & 4095) as usize]} else {1.7976931348623157e+308}) - fTemp625;
			self.fVec53[0] = fTemp650;
			let mut iTemp651: i32 = (fTemp650 > 0.0) as i32;
			let mut fTemp652: F64 = if iTemp651 != 0 {fSlow67} else {fSlow66};
			self.fVec54[0] = fTemp652;
			let mut fTemp653: F64 = 2.0 * fTemp652;
			let mut iTemp654: i32 = (fTemp653) as i32;
			let mut iTemp655: i32 = std::cmp::max(0, std::cmp::min(iTemp654, 2));
			let mut iTemp656: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, 98301), 196607));
			let mut fTemp657: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp656, 3)) as usize] };
			let mut fTemp658: F64 = unsafe { ftbl0mydspSIG0[iTemp656 as usize] };
			let mut fTemp659: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp656, 1)) as usize] } - fTemp658;
			let mut fTemp660: F64 = fTemp653 - (iTemp654) as F64;
			let mut fTemp661: F64 = fTemp658 + fTemp660 * fTemp659 + 0.5 * (fTemp657 - (fTemp658 + fTemp660 * (fTemp659 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp656, 4)) as usize] } - fTemp657))));
			let mut fTemp662: F64 = if iTemp651 != 0 {fTemp661} else {1.0 - fTemp661};
			let mut iTemp663: i32 = (fTemp650 < 0.0) as i32;
			let mut fTemp664: F64 = fSlow1 * (iTemp663) as F64 + fSlow13 * (iTemp651) as F64;
			self.fVec55[0] = fTemp664;
			let mut fTemp665: F64 = self.fConst10 / fTemp664;
			let mut fTemp666: F64 = fTemp665 + 0.5;
			let mut fTemp667: F64 = 65535.0 * (1.0 - fTemp666);
			let mut iTemp668: i32 = (fTemp667) as i32;
			let mut iTemp669: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp668, 65535)))), 196607));
			let mut fTemp670: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp669, 3)) as usize] };
			let mut fTemp671: F64 = unsafe { ftbl0mydspSIG0[iTemp669 as usize] };
			let mut fTemp672: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp669, 1)) as usize] } - fTemp671;
			let mut fTemp673: F64 = 65535.0 * fTemp666;
			let mut iTemp674: i32 = (fTemp673) as i32;
			let mut iTemp675: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp674, 65535)))), 196607));
			let mut fTemp676: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp675, 3), 196607))) as usize] };
			let mut fTemp677: F64 = unsafe { ftbl0mydspSIG0[iTemp675 as usize] };
			let mut fTemp678: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp675, 1), 196607))) as usize] } - fTemp677;
			let mut fTemp679: F64 = 2.0 * self.fVec54[1];
			let mut iTemp680: i32 = (fTemp679) as i32;
			let mut iTemp681: i32 = std::cmp::max(0, std::cmp::min(iTemp680, 2));
			let mut fTemp682: F64 = 65535.0 * (1.0 - self.fRec15[1]);
			let mut iTemp683: i32 = (fTemp682) as i32;
			let mut iTemp684: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp683, 65535))), iTemp681), 196607));
			let mut fTemp685: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp684, 3), 196607))) as usize] };
			let mut fTemp686: F64 = unsafe { ftbl0mydspSIG0[iTemp684 as usize] };
			let mut fTemp687: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp684, 1), 196607))) as usize] } - fTemp686;
			let mut fTemp688: F64 = fTemp679 - (iTemp680) as F64;
			let mut fTemp689: F64 = 65535.0 * self.fRec15[1];
			let mut iTemp690: i32 = (fTemp689) as i32;
			let mut iTemp691: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp681, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp690, 65535)))), 196607));
			let mut fTemp692: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp691, 3), 196607))) as usize] };
			let mut fTemp693: F64 = unsafe { ftbl0mydspSIG0[iTemp691 as usize] };
			let mut fTemp694: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp691, 1), 196607))) as usize] } - fTemp693;
			let mut fTemp695: F64 = self.fRec15[1] + fTemp665;
			let mut fTemp696: F64 = 65535.0 * (1.0 - fTemp695);
			let mut iTemp697: i32 = (fTemp696) as i32;
			let mut iTemp698: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp697, 65535)))), 196607));
			let mut fTemp699: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp698, 3)) as usize] };
			let mut fTemp700: F64 = unsafe { ftbl0mydspSIG0[iTemp698 as usize] };
			let mut fTemp701: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp698, 1)) as usize] } - fTemp700;
			let mut fTemp702: F64 = 65535.0 * fTemp695;
			let mut iTemp703: i32 = (fTemp702) as i32;
			let mut iTemp704: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp703, 65535)))), 196607));
			let mut fTemp705: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp704, 3), 196607))) as usize] };
			let mut fTemp706: F64 = unsafe { ftbl0mydspSIG0[iTemp704 as usize] };
			let mut fTemp707: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp704, 1), 196607))) as usize] } - fTemp706;
			let mut fTemp708: F64 = self.fRec15[1] + self.fConst10 * (1.0 / fTemp664 + 1.0 / self.fVec55[1]);
			let mut fTemp709: F64 = 65535.0 * (1.0 - fTemp708);
			let mut iTemp710: i32 = (fTemp709) as i32;
			let mut iTemp711: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp710, 65535))), iTemp655), 196607));
			let mut fTemp712: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp711, 3)) as usize] };
			let mut fTemp713: F64 = unsafe { ftbl0mydspSIG0[iTemp711 as usize] };
			let mut fTemp714: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp711, 1)) as usize] } - fTemp713;
			let mut fTemp715: F64 = 65535.0 * fTemp708;
			let mut iTemp716: i32 = (fTemp715) as i32;
			let mut iTemp717: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp716, 65535)))), 196607));
			let mut fTemp718: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp717, 3), 196607))) as usize] };
			let mut fTemp719: F64 = unsafe { ftbl0mydspSIG0[iTemp717 as usize] };
			let mut fTemp720: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp717, 1), 196607))) as usize] } - fTemp719;
			let mut fTemp721: F64 = (if iTemp651 != 0 {fTemp719 + fTemp660 * fTemp720 + (fTemp715 - (iTemp716) as F64) * (fTemp718 - (fTemp719 + fTemp660 * (fTemp720 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp717, 4), 196607))) as usize] } - fTemp718))))} else {1.0 - (fTemp713 + fTemp660 * fTemp714 + (fTemp709 - (iTemp710) as F64) * (fTemp712 - (fTemp713 + fTemp660 * (fTemp714 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp711, 4)) as usize] } - fTemp712)))))} - if iTemp651 != 0 {fTemp706 + fTemp660 * fTemp707 + (fTemp702 - (iTemp703) as F64) * (fTemp705 - (fTemp706 + fTemp660 * (fTemp707 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp704, 4), 196607))) as usize] } - fTemp705))))} else {1.0 - (fTemp700 + fTemp660 * fTemp701 + (fTemp696 - (iTemp697) as F64) * (fTemp699 - (fTemp700 + fTemp660 * (fTemp701 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp698, 4), 196607))) as usize] } - fTemp699)))))}) * self.fVec53[1] / (fTemp650 * (1.0 - if iTemp651 != 0 {fTemp693 + fTemp688 * fTemp694 + (fTemp689 - (iTemp690) as F64) * (fTemp692 - (fTemp693 + fTemp688 * (fTemp694 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp691, 4), 196607))) as usize] } - fTemp692))))} else {1.0 - (fTemp686 + fTemp688 * fTemp687 + (fTemp682 - (iTemp683) as F64) * (fTemp685 - (fTemp686 + fTemp688 * (fTemp687 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp684, 4), 196607))) as usize] } - fTemp685)))))}));
			let mut iTemp722: i32 = (fTemp721 > ((if iTemp651 != 0 {fTemp677 + fTemp660 * fTemp678 + (fTemp673 - (iTemp674) as F64) * (fTemp676 - (fTemp677 + fTemp660 * (fTemp678 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp675, 4), 196607))) as usize] } - fTemp676))))} else {1.0 - (fTemp671 + fTemp660 * fTemp672 + (fTemp667 - (iTemp668) as F64) * (fTemp670 - (fTemp671 + fTemp660 * (fTemp672 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp669, 4)) as usize] } - fTemp670)))))} - fTemp662) / (1.0 - fTemp662))) as i32;
			let mut fTemp723: F64 = if iTemp722 != 0 {1.0} else {0.5};
			let mut fTemp724: F64 = if iTemp722 != 0 {0.5} else {0.0};
			let mut fTemp725: F64 = fTemp724 + fTemp723;
			let mut fTemp726: F64 = 0.5 * fTemp725;
			let mut fTemp727: F64 = 65535.0 * (1.0 - fTemp726);
			let mut iTemp728: i32 = (fTemp727) as i32;
			let mut iTemp729: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp728, 65535)))), 196607));
			let mut fTemp730: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp729, 3)) as usize] };
			let mut fTemp731: F64 = unsafe { ftbl0mydspSIG0[iTemp729 as usize] };
			let mut fTemp732: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp729, 1)) as usize] } - fTemp731;
			let mut fTemp733: F64 = 32767.5 * fTemp725;
			let mut iTemp734: i32 = (fTemp733) as i32;
			let mut iTemp735: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp734, 65535)))), 196607));
			let mut fTemp736: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp735, 3)) as usize] };
			let mut fTemp737: F64 = unsafe { ftbl0mydspSIG0[iTemp735 as usize] };
			let mut fTemp738: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp735, 1)) as usize] } - fTemp737;
			let mut fTemp739: F64 = if iTemp651 != 0 {fTemp737 + fTemp660 * fTemp738 + (fTemp733 - (iTemp734) as F64) * (fTemp736 - (fTemp737 + fTemp660 * (fTemp738 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp735, 4)) as usize] } - fTemp736))))} else {1.0 - (fTemp731 + fTemp660 * fTemp732 + (fTemp727 - (iTemp728) as F64) * (fTemp730 - (fTemp731 + fTemp660 * (fTemp732 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp729, 4)) as usize] } - fTemp730)))))};
			let mut fTemp740: F64 = fTemp665 + fTemp726;
			let mut fTemp741: F64 = 65535.0 * (1.0 - fTemp740);
			let mut iTemp742: i32 = (fTemp741) as i32;
			let mut iTemp743: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp742, 65535)))), 196607));
			let mut fTemp744: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp743, 3)) as usize] };
			let mut fTemp745: F64 = unsafe { ftbl0mydspSIG0[iTemp743 as usize] };
			let mut fTemp746: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp743, 1)) as usize] } - fTemp745;
			let mut fTemp747: F64 = 65535.0 * fTemp740;
			let mut iTemp748: i32 = (fTemp747) as i32;
			let mut iTemp749: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp748, 65535)))), 196607));
			let mut fTemp750: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp749, 3), 196607))) as usize] };
			let mut fTemp751: F64 = unsafe { ftbl0mydspSIG0[iTemp749 as usize] };
			let mut fTemp752: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp749, 1), 196607))) as usize] } - fTemp751;
			let mut iTemp753: i32 = (fTemp721 > ((if iTemp651 != 0 {fTemp751 + fTemp660 * fTemp752 + (fTemp747 - (iTemp748) as F64) * (fTemp750 - (fTemp751 + fTemp660 * (fTemp752 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp749, 4), 196607))) as usize] } - fTemp750))))} else {1.0 - (fTemp745 + fTemp660 * fTemp746 + (fTemp741 - (iTemp742) as F64) * (fTemp744 - (fTemp745 + fTemp660 * (fTemp746 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp743, 4)) as usize] } - fTemp744)))))} - fTemp739) / (1.0 - fTemp739))) as i32;
			let mut fTemp754: F64 = if iTemp753 != 0 {fTemp723} else {fTemp726};
			let mut fTemp755: F64 = if iTemp753 != 0 {fTemp726} else {fTemp724};
			let mut fTemp756: F64 = fTemp755 + fTemp754;
			let mut fTemp757: F64 = 0.5 * fTemp756;
			let mut fTemp758: F64 = 65535.0 * (1.0 - fTemp757);
			let mut iTemp759: i32 = (fTemp758) as i32;
			let mut iTemp760: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp759, 65535)))), 196607));
			let mut fTemp761: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp760, 3)) as usize] };
			let mut fTemp762: F64 = unsafe { ftbl0mydspSIG0[iTemp760 as usize] };
			let mut fTemp763: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp760, 1)) as usize] } - fTemp762;
			let mut fTemp764: F64 = 32767.5 * fTemp756;
			let mut iTemp765: i32 = (fTemp764) as i32;
			let mut iTemp766: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp765, 65535)))), 196607));
			let mut fTemp767: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp766, 3)) as usize] };
			let mut fTemp768: F64 = unsafe { ftbl0mydspSIG0[iTemp766 as usize] };
			let mut fTemp769: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp766, 1)) as usize] } - fTemp768;
			let mut fTemp770: F64 = if iTemp651 != 0 {fTemp768 + fTemp660 * fTemp769 + (fTemp764 - (iTemp765) as F64) * (fTemp767 - (fTemp768 + fTemp660 * (fTemp769 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp766, 4)) as usize] } - fTemp767))))} else {1.0 - (fTemp762 + fTemp660 * fTemp763 + (fTemp758 - (iTemp759) as F64) * (fTemp761 - (fTemp762 + fTemp660 * (fTemp763 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp760, 4)) as usize] } - fTemp761)))))};
			let mut fTemp771: F64 = fTemp665 + fTemp757;
			let mut fTemp772: F64 = 65535.0 * (1.0 - fTemp771);
			let mut iTemp773: i32 = (fTemp772) as i32;
			let mut iTemp774: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp773, 65535)))), 196607));
			let mut fTemp775: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp774, 3)) as usize] };
			let mut fTemp776: F64 = unsafe { ftbl0mydspSIG0[iTemp774 as usize] };
			let mut fTemp777: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp774, 1)) as usize] } - fTemp776;
			let mut fTemp778: F64 = 65535.0 * fTemp771;
			let mut iTemp779: i32 = (fTemp778) as i32;
			let mut iTemp780: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp779, 65535)))), 196607));
			let mut fTemp781: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp780, 3), 196607))) as usize] };
			let mut fTemp782: F64 = unsafe { ftbl0mydspSIG0[iTemp780 as usize] };
			let mut fTemp783: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp780, 1), 196607))) as usize] } - fTemp782;
			let mut iTemp784: i32 = (fTemp721 > ((if iTemp651 != 0 {fTemp782 + fTemp660 * fTemp783 + (fTemp778 - (iTemp779) as F64) * (fTemp781 - (fTemp782 + fTemp660 * (fTemp783 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp780, 4), 196607))) as usize] } - fTemp781))))} else {1.0 - (fTemp776 + fTemp660 * fTemp777 + (fTemp772 - (iTemp773) as F64) * (fTemp775 - (fTemp776 + fTemp660 * (fTemp777 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp774, 4)) as usize] } - fTemp775)))))} - fTemp770) / (1.0 - fTemp770))) as i32;
			let mut fTemp785: F64 = if iTemp784 != 0 {fTemp754} else {fTemp757};
			let mut fTemp786: F64 = if iTemp784 != 0 {fTemp757} else {fTemp755};
			let mut fTemp787: F64 = fTemp786 + fTemp785;
			let mut fTemp788: F64 = 0.5 * fTemp787;
			let mut fTemp789: F64 = 65535.0 * (1.0 - fTemp788);
			let mut iTemp790: i32 = (fTemp789) as i32;
			let mut iTemp791: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp790, 65535)))), 196607));
			let mut fTemp792: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp791, 3)) as usize] };
			let mut fTemp793: F64 = unsafe { ftbl0mydspSIG0[iTemp791 as usize] };
			let mut fTemp794: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp791, 1)) as usize] } - fTemp793;
			let mut fTemp795: F64 = 32767.5 * fTemp787;
			let mut iTemp796: i32 = (fTemp795) as i32;
			let mut iTemp797: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp796, 65535)))), 196607));
			let mut fTemp798: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp797, 3)) as usize] };
			let mut fTemp799: F64 = unsafe { ftbl0mydspSIG0[iTemp797 as usize] };
			let mut fTemp800: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp797, 1)) as usize] } - fTemp799;
			let mut fTemp801: F64 = if iTemp651 != 0 {fTemp799 + fTemp660 * fTemp800 + (fTemp795 - (iTemp796) as F64) * (fTemp798 - (fTemp799 + fTemp660 * (fTemp800 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp797, 4)) as usize] } - fTemp798))))} else {1.0 - (fTemp793 + fTemp660 * fTemp794 + (fTemp789 - (iTemp790) as F64) * (fTemp792 - (fTemp793 + fTemp660 * (fTemp794 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp791, 4)) as usize] } - fTemp792)))))};
			let mut fTemp802: F64 = fTemp665 + fTemp788;
			let mut fTemp803: F64 = 65535.0 * (1.0 - fTemp802);
			let mut iTemp804: i32 = (fTemp803) as i32;
			let mut iTemp805: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp804, 65535)))), 196607));
			let mut fTemp806: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp805, 3)) as usize] };
			let mut fTemp807: F64 = unsafe { ftbl0mydspSIG0[iTemp805 as usize] };
			let mut fTemp808: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp805, 1)) as usize] } - fTemp807;
			let mut fTemp809: F64 = 65535.0 * fTemp802;
			let mut iTemp810: i32 = (fTemp809) as i32;
			let mut iTemp811: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp810, 65535)))), 196607));
			let mut fTemp812: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp811, 3), 196607))) as usize] };
			let mut fTemp813: F64 = unsafe { ftbl0mydspSIG0[iTemp811 as usize] };
			let mut fTemp814: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp811, 1), 196607))) as usize] } - fTemp813;
			let mut iTemp815: i32 = (fTemp721 > ((if iTemp651 != 0 {fTemp813 + fTemp660 * fTemp814 + (fTemp809 - (iTemp810) as F64) * (fTemp812 - (fTemp813 + fTemp660 * (fTemp814 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp811, 4), 196607))) as usize] } - fTemp812))))} else {1.0 - (fTemp807 + fTemp660 * fTemp808 + (fTemp803 - (iTemp804) as F64) * (fTemp806 - (fTemp807 + fTemp660 * (fTemp808 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp805, 4)) as usize] } - fTemp806)))))} - fTemp801) / (1.0 - fTemp801))) as i32;
			let mut fTemp816: F64 = if iTemp815 != 0 {fTemp785} else {fTemp788};
			let mut fTemp817: F64 = if iTemp815 != 0 {fTemp788} else {fTemp786};
			let mut fTemp818: F64 = fTemp817 + fTemp816;
			let mut fTemp819: F64 = 0.5 * fTemp818;
			let mut fTemp820: F64 = 65535.0 * (1.0 - fTemp819);
			let mut iTemp821: i32 = (fTemp820) as i32;
			let mut iTemp822: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp821, 65535)))), 196607));
			let mut fTemp823: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp822, 3)) as usize] };
			let mut fTemp824: F64 = unsafe { ftbl0mydspSIG0[iTemp822 as usize] };
			let mut fTemp825: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp822, 1)) as usize] } - fTemp824;
			let mut fTemp826: F64 = 32767.5 * fTemp818;
			let mut iTemp827: i32 = (fTemp826) as i32;
			let mut iTemp828: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp827, 65535)))), 196607));
			let mut fTemp829: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp828, 3)) as usize] };
			let mut fTemp830: F64 = unsafe { ftbl0mydspSIG0[iTemp828 as usize] };
			let mut fTemp831: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp828, 1)) as usize] } - fTemp830;
			let mut fTemp832: F64 = if iTemp651 != 0 {fTemp830 + fTemp660 * fTemp831 + (fTemp826 - (iTemp827) as F64) * (fTemp829 - (fTemp830 + fTemp660 * (fTemp831 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp828, 4)) as usize] } - fTemp829))))} else {1.0 - (fTemp824 + fTemp660 * fTemp825 + (fTemp820 - (iTemp821) as F64) * (fTemp823 - (fTemp824 + fTemp660 * (fTemp825 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp822, 4)) as usize] } - fTemp823)))))};
			let mut fTemp833: F64 = fTemp665 + fTemp819;
			let mut fTemp834: F64 = 65535.0 * (1.0 - fTemp833);
			let mut iTemp835: i32 = (fTemp834) as i32;
			let mut iTemp836: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp835, 65535)))), 196607));
			let mut fTemp837: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp836, 3)) as usize] };
			let mut fTemp838: F64 = unsafe { ftbl0mydspSIG0[iTemp836 as usize] };
			let mut fTemp839: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp836, 1)) as usize] } - fTemp838;
			let mut fTemp840: F64 = 65535.0 * fTemp833;
			let mut iTemp841: i32 = (fTemp840) as i32;
			let mut iTemp842: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp841, 65535)))), 196607));
			let mut fTemp843: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp842, 3), 196607))) as usize] };
			let mut fTemp844: F64 = unsafe { ftbl0mydspSIG0[iTemp842 as usize] };
			let mut fTemp845: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp842, 1), 196607))) as usize] } - fTemp844;
			let mut iTemp846: i32 = (fTemp721 > ((if iTemp651 != 0 {fTemp844 + fTemp660 * fTemp845 + (fTemp840 - (iTemp841) as F64) * (fTemp843 - (fTemp844 + fTemp660 * (fTemp845 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp842, 4), 196607))) as usize] } - fTemp843))))} else {1.0 - (fTemp838 + fTemp660 * fTemp839 + (fTemp834 - (iTemp835) as F64) * (fTemp837 - (fTemp838 + fTemp660 * (fTemp839 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp836, 4)) as usize] } - fTemp837)))))} - fTemp832) / (1.0 - fTemp832))) as i32;
			let mut fTemp847: F64 = if iTemp846 != 0 {fTemp816} else {fTemp819};
			let mut fTemp848: F64 = if iTemp846 != 0 {fTemp819} else {fTemp817};
			let mut fTemp849: F64 = fTemp848 + fTemp847;
			let mut fTemp850: F64 = 0.5 * fTemp849;
			let mut fTemp851: F64 = 65535.0 * (1.0 - fTemp850);
			let mut iTemp852: i32 = (fTemp851) as i32;
			let mut iTemp853: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp852, 65535)))), 196607));
			let mut fTemp854: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp853, 3)) as usize] };
			let mut fTemp855: F64 = unsafe { ftbl0mydspSIG0[iTemp853 as usize] };
			let mut fTemp856: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp853, 1)) as usize] } - fTemp855;
			let mut fTemp857: F64 = 32767.5 * fTemp849;
			let mut iTemp858: i32 = (fTemp857) as i32;
			let mut iTemp859: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp858, 65535)))), 196607));
			let mut fTemp860: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp859, 3)) as usize] };
			let mut fTemp861: F64 = unsafe { ftbl0mydspSIG0[iTemp859 as usize] };
			let mut fTemp862: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp859, 1)) as usize] } - fTemp861;
			let mut fTemp863: F64 = if iTemp651 != 0 {fTemp861 + fTemp660 * fTemp862 + (fTemp857 - (iTemp858) as F64) * (fTemp860 - (fTemp861 + fTemp660 * (fTemp862 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp859, 4)) as usize] } - fTemp860))))} else {1.0 - (fTemp855 + fTemp660 * fTemp856 + (fTemp851 - (iTemp852) as F64) * (fTemp854 - (fTemp855 + fTemp660 * (fTemp856 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp853, 4)) as usize] } - fTemp854)))))};
			let mut fTemp864: F64 = fTemp665 + fTemp850;
			let mut fTemp865: F64 = 65535.0 * (1.0 - fTemp864);
			let mut iTemp866: i32 = (fTemp865) as i32;
			let mut iTemp867: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp866, 65535)))), 196607));
			let mut fTemp868: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp867, 3)) as usize] };
			let mut fTemp869: F64 = unsafe { ftbl0mydspSIG0[iTemp867 as usize] };
			let mut fTemp870: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp867, 1)) as usize] } - fTemp869;
			let mut fTemp871: F64 = 65535.0 * fTemp864;
			let mut iTemp872: i32 = (fTemp871) as i32;
			let mut iTemp873: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp872, 65535)))), 196607));
			let mut fTemp874: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp873, 3), 196607))) as usize] };
			let mut fTemp875: F64 = unsafe { ftbl0mydspSIG0[iTemp873 as usize] };
			let mut fTemp876: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp873, 1), 196607))) as usize] } - fTemp875;
			let mut iTemp877: i32 = (fTemp721 > ((if iTemp651 != 0 {fTemp875 + fTemp660 * fTemp876 + (fTemp871 - (iTemp872) as F64) * (fTemp874 - (fTemp875 + fTemp660 * (fTemp876 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp873, 4), 196607))) as usize] } - fTemp874))))} else {1.0 - (fTemp869 + fTemp660 * fTemp870 + (fTemp865 - (iTemp866) as F64) * (fTemp868 - (fTemp869 + fTemp660 * (fTemp870 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp867, 4)) as usize] } - fTemp868)))))} - fTemp863) / (1.0 - fTemp863))) as i32;
			let mut fTemp878: F64 = if iTemp877 != 0 {fTemp847} else {fTemp850};
			let mut fTemp879: F64 = if iTemp877 != 0 {fTemp850} else {fTemp848};
			let mut fTemp880: F64 = fTemp879 + fTemp878;
			let mut fTemp881: F64 = 0.5 * fTemp880;
			let mut fTemp882: F64 = 65535.0 * (1.0 - fTemp881);
			let mut iTemp883: i32 = (fTemp882) as i32;
			let mut iTemp884: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp883, 65535)))), 196607));
			let mut fTemp885: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp884, 3)) as usize] };
			let mut fTemp886: F64 = unsafe { ftbl0mydspSIG0[iTemp884 as usize] };
			let mut fTemp887: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp884, 1)) as usize] } - fTemp886;
			let mut fTemp888: F64 = 32767.5 * fTemp880;
			let mut iTemp889: i32 = (fTemp888) as i32;
			let mut iTemp890: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp889, 65535)))), 196607));
			let mut fTemp891: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp890, 3)) as usize] };
			let mut fTemp892: F64 = unsafe { ftbl0mydspSIG0[iTemp890 as usize] };
			let mut fTemp893: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp890, 1)) as usize] } - fTemp892;
			let mut fTemp894: F64 = if iTemp651 != 0 {fTemp892 + fTemp660 * fTemp893 + (fTemp888 - (iTemp889) as F64) * (fTemp891 - (fTemp892 + fTemp660 * (fTemp893 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp890, 4)) as usize] } - fTemp891))))} else {1.0 - (fTemp886 + fTemp660 * fTemp887 + (fTemp882 - (iTemp883) as F64) * (fTemp885 - (fTemp886 + fTemp660 * (fTemp887 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp884, 4)) as usize] } - fTemp885)))))};
			let mut fTemp895: F64 = fTemp665 + fTemp881;
			let mut fTemp896: F64 = 65535.0 * (1.0 - fTemp895);
			let mut iTemp897: i32 = (fTemp896) as i32;
			let mut iTemp898: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp897, 65535)))), 196607));
			let mut fTemp899: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp898, 3)) as usize] };
			let mut fTemp900: F64 = unsafe { ftbl0mydspSIG0[iTemp898 as usize] };
			let mut fTemp901: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp898, 1)) as usize] } - fTemp900;
			let mut fTemp902: F64 = 65535.0 * fTemp895;
			let mut iTemp903: i32 = (fTemp902) as i32;
			let mut iTemp904: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp903, 65535)))), 196607));
			let mut fTemp905: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp904, 3), 196607))) as usize] };
			let mut fTemp906: F64 = unsafe { ftbl0mydspSIG0[iTemp904 as usize] };
			let mut fTemp907: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp904, 1), 196607))) as usize] } - fTemp906;
			let mut iTemp908: i32 = (fTemp721 > ((if iTemp651 != 0 {fTemp906 + fTemp660 * fTemp907 + (fTemp902 - (iTemp903) as F64) * (fTemp905 - (fTemp906 + fTemp660 * (fTemp907 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp904, 4), 196607))) as usize] } - fTemp905))))} else {1.0 - (fTemp900 + fTemp660 * fTemp901 + (fTemp896 - (iTemp897) as F64) * (fTemp899 - (fTemp900 + fTemp660 * (fTemp901 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp898, 4)) as usize] } - fTemp899)))))} - fTemp894) / (1.0 - fTemp894))) as i32;
			let mut fTemp909: F64 = if iTemp908 != 0 {fTemp878} else {fTemp881};
			let mut fTemp910: F64 = if iTemp908 != 0 {fTemp881} else {fTemp879};
			let mut fTemp911: F64 = fTemp910 + fTemp909;
			let mut fTemp912: F64 = 0.5 * fTemp911;
			let mut fTemp913: F64 = 65535.0 * (1.0 - fTemp912);
			let mut iTemp914: i32 = (fTemp913) as i32;
			let mut iTemp915: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp914, 65535)))), 196607));
			let mut fTemp916: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp915, 3)) as usize] };
			let mut fTemp917: F64 = unsafe { ftbl0mydspSIG0[iTemp915 as usize] };
			let mut fTemp918: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp915, 1)) as usize] } - fTemp917;
			let mut fTemp919: F64 = 32767.5 * fTemp911;
			let mut iTemp920: i32 = (fTemp919) as i32;
			let mut iTemp921: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp920, 65535)))), 196607));
			let mut fTemp922: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp921, 3)) as usize] };
			let mut fTemp923: F64 = unsafe { ftbl0mydspSIG0[iTemp921 as usize] };
			let mut fTemp924: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp921, 1)) as usize] } - fTemp923;
			let mut fTemp925: F64 = if iTemp651 != 0 {fTemp923 + fTemp660 * fTemp924 + (fTemp919 - (iTemp920) as F64) * (fTemp922 - (fTemp923 + fTemp660 * (fTemp924 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp921, 4)) as usize] } - fTemp922))))} else {1.0 - (fTemp917 + fTemp660 * fTemp918 + (fTemp913 - (iTemp914) as F64) * (fTemp916 - (fTemp917 + fTemp660 * (fTemp918 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp915, 4)) as usize] } - fTemp916)))))};
			let mut fTemp926: F64 = fTemp665 + fTemp912;
			let mut fTemp927: F64 = 65535.0 * (1.0 - fTemp926);
			let mut iTemp928: i32 = (fTemp927) as i32;
			let mut iTemp929: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp928, 65535)))), 196607));
			let mut fTemp930: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp929, 3)) as usize] };
			let mut fTemp931: F64 = unsafe { ftbl0mydspSIG0[iTemp929 as usize] };
			let mut fTemp932: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp929, 1)) as usize] } - fTemp931;
			let mut fTemp933: F64 = 65535.0 * fTemp926;
			let mut iTemp934: i32 = (fTemp933) as i32;
			let mut iTemp935: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp934, 65535)))), 196607));
			let mut fTemp936: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp935, 3), 196607))) as usize] };
			let mut fTemp937: F64 = unsafe { ftbl0mydspSIG0[iTemp935 as usize] };
			let mut fTemp938: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp935, 1), 196607))) as usize] } - fTemp937;
			let mut iTemp939: i32 = (fTemp721 > ((if iTemp651 != 0 {fTemp937 + fTemp660 * fTemp938 + (fTemp933 - (iTemp934) as F64) * (fTemp936 - (fTemp937 + fTemp660 * (fTemp938 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp935, 4), 196607))) as usize] } - fTemp936))))} else {1.0 - (fTemp931 + fTemp660 * fTemp932 + (fTemp927 - (iTemp928) as F64) * (fTemp930 - (fTemp931 + fTemp660 * (fTemp932 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp929, 4)) as usize] } - fTemp930)))))} - fTemp925) / (1.0 - fTemp925))) as i32;
			let mut fTemp940: F64 = if iTemp939 != 0 {fTemp909} else {fTemp912};
			let mut fTemp941: F64 = if iTemp939 != 0 {fTemp912} else {fTemp910};
			let mut fTemp942: F64 = fTemp941 + fTemp940;
			let mut fTemp943: F64 = 0.5 * fTemp942;
			let mut fTemp944: F64 = 65535.0 * (1.0 - fTemp943);
			let mut iTemp945: i32 = (fTemp944) as i32;
			let mut iTemp946: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp945, 65535)))), 196607));
			let mut fTemp947: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp946, 3)) as usize] };
			let mut fTemp948: F64 = unsafe { ftbl0mydspSIG0[iTemp946 as usize] };
			let mut fTemp949: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp946, 1)) as usize] } - fTemp948;
			let mut fTemp950: F64 = 32767.5 * fTemp942;
			let mut iTemp951: i32 = (fTemp950) as i32;
			let mut iTemp952: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp951, 65535)))), 196607));
			let mut fTemp953: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp952, 3)) as usize] };
			let mut fTemp954: F64 = unsafe { ftbl0mydspSIG0[iTemp952 as usize] };
			let mut fTemp955: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp952, 1)) as usize] } - fTemp954;
			let mut fTemp956: F64 = if iTemp651 != 0 {fTemp954 + fTemp660 * fTemp955 + (fTemp950 - (iTemp951) as F64) * (fTemp953 - (fTemp954 + fTemp660 * (fTemp955 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp952, 4)) as usize] } - fTemp953))))} else {1.0 - (fTemp948 + fTemp660 * fTemp949 + (fTemp944 - (iTemp945) as F64) * (fTemp947 - (fTemp948 + fTemp660 * (fTemp949 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp946, 4)) as usize] } - fTemp947)))))};
			let mut fTemp957: F64 = fTemp665 + fTemp943;
			let mut fTemp958: F64 = 65535.0 * (1.0 - fTemp957);
			let mut iTemp959: i32 = (fTemp958) as i32;
			let mut iTemp960: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp959, 65535)))), 196607));
			let mut fTemp961: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp960, 3)) as usize] };
			let mut fTemp962: F64 = unsafe { ftbl0mydspSIG0[iTemp960 as usize] };
			let mut fTemp963: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp960, 1)) as usize] } - fTemp962;
			let mut fTemp964: F64 = 65535.0 * fTemp957;
			let mut iTemp965: i32 = (fTemp964) as i32;
			let mut iTemp966: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp965, 65535)))), 196607));
			let mut fTemp967: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp966, 3), 196607))) as usize] };
			let mut fTemp968: F64 = unsafe { ftbl0mydspSIG0[iTemp966 as usize] };
			let mut fTemp969: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp966, 1), 196607))) as usize] } - fTemp968;
			let mut iTemp970: i32 = (fTemp721 > ((if iTemp651 != 0 {fTemp968 + fTemp660 * fTemp969 + (fTemp964 - (iTemp965) as F64) * (fTemp967 - (fTemp968 + fTemp660 * (fTemp969 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp966, 4), 196607))) as usize] } - fTemp967))))} else {1.0 - (fTemp962 + fTemp660 * fTemp963 + (fTemp958 - (iTemp959) as F64) * (fTemp961 - (fTemp962 + fTemp660 * (fTemp963 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp960, 4)) as usize] } - fTemp961)))))} - fTemp956) / (1.0 - fTemp956))) as i32;
			let mut fTemp971: F64 = if iTemp970 != 0 {fTemp940} else {fTemp943};
			let mut fTemp972: F64 = if iTemp970 != 0 {fTemp943} else {fTemp941};
			let mut fTemp973: F64 = fTemp972 + fTemp971;
			let mut fTemp974: F64 = 0.5 * fTemp973;
			let mut fTemp975: F64 = 65535.0 * (1.0 - fTemp974);
			let mut iTemp976: i32 = (fTemp975) as i32;
			let mut iTemp977: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp976, 65535)))), 196607));
			let mut fTemp978: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp977, 3)) as usize] };
			let mut fTemp979: F64 = unsafe { ftbl0mydspSIG0[iTemp977 as usize] };
			let mut fTemp980: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp977, 1)) as usize] } - fTemp979;
			let mut fTemp981: F64 = 32767.5 * fTemp973;
			let mut iTemp982: i32 = (fTemp981) as i32;
			let mut iTemp983: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp982, 65535)))), 196607));
			let mut fTemp984: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp983, 3)) as usize] };
			let mut fTemp985: F64 = unsafe { ftbl0mydspSIG0[iTemp983 as usize] };
			let mut fTemp986: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp983, 1)) as usize] } - fTemp985;
			let mut fTemp987: F64 = if iTemp651 != 0 {fTemp985 + fTemp660 * fTemp986 + (fTemp981 - (iTemp982) as F64) * (fTemp984 - (fTemp985 + fTemp660 * (fTemp986 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp983, 4)) as usize] } - fTemp984))))} else {1.0 - (fTemp979 + fTemp660 * fTemp980 + (fTemp975 - (iTemp976) as F64) * (fTemp978 - (fTemp979 + fTemp660 * (fTemp980 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp977, 4)) as usize] } - fTemp978)))))};
			let mut fTemp988: F64 = fTemp665 + fTemp974;
			let mut fTemp989: F64 = 65535.0 * (1.0 - fTemp988);
			let mut iTemp990: i32 = (fTemp989) as i32;
			let mut iTemp991: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp990, 65535)))), 196607));
			let mut fTemp992: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp991, 3)) as usize] };
			let mut fTemp993: F64 = unsafe { ftbl0mydspSIG0[iTemp991 as usize] };
			let mut fTemp994: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp991, 1)) as usize] } - fTemp993;
			let mut fTemp995: F64 = 65535.0 * fTemp988;
			let mut iTemp996: i32 = (fTemp995) as i32;
			let mut iTemp997: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp996, 65535)))), 196607));
			let mut fTemp998: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp997, 3), 196607))) as usize] };
			let mut fTemp999: F64 = unsafe { ftbl0mydspSIG0[iTemp997 as usize] };
			let mut fTemp1000: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp997, 1), 196607))) as usize] } - fTemp999;
			let mut iTemp1001: i32 = (fTemp721 > ((if iTemp651 != 0 {fTemp999 + fTemp660 * fTemp1000 + (fTemp995 - (iTemp996) as F64) * (fTemp998 - (fTemp999 + fTemp660 * (fTemp1000 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp997, 4), 196607))) as usize] } - fTemp998))))} else {1.0 - (fTemp993 + fTemp660 * fTemp994 + (fTemp989 - (iTemp990) as F64) * (fTemp992 - (fTemp993 + fTemp660 * (fTemp994 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp991, 4)) as usize] } - fTemp992)))))} - fTemp987) / (1.0 - fTemp987))) as i32;
			let mut fTemp1002: F64 = if iTemp1001 != 0 {fTemp971} else {fTemp974};
			let mut fTemp1003: F64 = if iTemp1001 != 0 {fTemp974} else {fTemp972};
			let mut fTemp1004: F64 = fTemp1003 + fTemp1002;
			let mut fTemp1005: F64 = 0.5 * fTemp1004;
			let mut fTemp1006: F64 = 65535.0 * (1.0 - fTemp1005);
			let mut iTemp1007: i32 = (fTemp1006) as i32;
			let mut iTemp1008: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1007, 65535)))), 196607));
			let mut fTemp1009: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1008, 3)) as usize] };
			let mut fTemp1010: F64 = unsafe { ftbl0mydspSIG0[iTemp1008 as usize] };
			let mut fTemp1011: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1008, 1)) as usize] } - fTemp1010;
			let mut fTemp1012: F64 = 32767.5 * fTemp1004;
			let mut iTemp1013: i32 = (fTemp1012) as i32;
			let mut iTemp1014: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1013, 65535)))), 196607));
			let mut fTemp1015: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1014, 3)) as usize] };
			let mut fTemp1016: F64 = unsafe { ftbl0mydspSIG0[iTemp1014 as usize] };
			let mut fTemp1017: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1014, 1)) as usize] } - fTemp1016;
			let mut fTemp1018: F64 = if iTemp651 != 0 {fTemp1016 + fTemp660 * fTemp1017 + (fTemp1012 - (iTemp1013) as F64) * (fTemp1015 - (fTemp1016 + fTemp660 * (fTemp1017 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1014, 4)) as usize] } - fTemp1015))))} else {1.0 - (fTemp1010 + fTemp660 * fTemp1011 + (fTemp1006 - (iTemp1007) as F64) * (fTemp1009 - (fTemp1010 + fTemp660 * (fTemp1011 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1008, 4)) as usize] } - fTemp1009)))))};
			let mut fTemp1019: F64 = fTemp665 + fTemp1005;
			let mut fTemp1020: F64 = 65535.0 * (1.0 - fTemp1019);
			let mut iTemp1021: i32 = (fTemp1020) as i32;
			let mut iTemp1022: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1021, 65535)))), 196607));
			let mut fTemp1023: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1022, 3)) as usize] };
			let mut fTemp1024: F64 = unsafe { ftbl0mydspSIG0[iTemp1022 as usize] };
			let mut fTemp1025: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1022, 1)) as usize] } - fTemp1024;
			let mut fTemp1026: F64 = 65535.0 * fTemp1019;
			let mut iTemp1027: i32 = (fTemp1026) as i32;
			let mut iTemp1028: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1027, 65535)))), 196607));
			let mut fTemp1029: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1028, 3), 196607))) as usize] };
			let mut fTemp1030: F64 = unsafe { ftbl0mydspSIG0[iTemp1028 as usize] };
			let mut fTemp1031: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1028, 1), 196607))) as usize] } - fTemp1030;
			let mut iTemp1032: i32 = (fTemp721 > ((if iTemp651 != 0 {fTemp1030 + fTemp660 * fTemp1031 + (fTemp1026 - (iTemp1027) as F64) * (fTemp1029 - (fTemp1030 + fTemp660 * (fTemp1031 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1028, 4), 196607))) as usize] } - fTemp1029))))} else {1.0 - (fTemp1024 + fTemp660 * fTemp1025 + (fTemp1020 - (iTemp1021) as F64) * (fTemp1023 - (fTemp1024 + fTemp660 * (fTemp1025 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1022, 4)) as usize] } - fTemp1023)))))} - fTemp1018) / (1.0 - fTemp1018))) as i32;
			let mut fTemp1033: F64 = if iTemp1032 != 0 {fTemp1002} else {fTemp1005};
			let mut fTemp1034: F64 = if iTemp1032 != 0 {fTemp1005} else {fTemp1003};
			let mut fTemp1035: F64 = fTemp1034 + fTemp1033;
			let mut fTemp1036: F64 = 0.5 * fTemp1035;
			let mut fTemp1037: F64 = 65535.0 * (1.0 - fTemp1036);
			let mut iTemp1038: i32 = (fTemp1037) as i32;
			let mut iTemp1039: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1038, 65535)))), 196607));
			let mut fTemp1040: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1039, 3)) as usize] };
			let mut fTemp1041: F64 = unsafe { ftbl0mydspSIG0[iTemp1039 as usize] };
			let mut fTemp1042: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1039, 1)) as usize] } - fTemp1041;
			let mut fTemp1043: F64 = 32767.5 * fTemp1035;
			let mut iTemp1044: i32 = (fTemp1043) as i32;
			let mut iTemp1045: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1044, 65535)))), 196607));
			let mut fTemp1046: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1045, 3)) as usize] };
			let mut fTemp1047: F64 = unsafe { ftbl0mydspSIG0[iTemp1045 as usize] };
			let mut fTemp1048: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1045, 1)) as usize] } - fTemp1047;
			let mut fTemp1049: F64 = if iTemp651 != 0 {fTemp1047 + fTemp660 * fTemp1048 + (fTemp1043 - (iTemp1044) as F64) * (fTemp1046 - (fTemp1047 + fTemp660 * (fTemp1048 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1045, 4)) as usize] } - fTemp1046))))} else {1.0 - (fTemp1041 + fTemp660 * fTemp1042 + (fTemp1037 - (iTemp1038) as F64) * (fTemp1040 - (fTemp1041 + fTemp660 * (fTemp1042 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1039, 4)) as usize] } - fTemp1040)))))};
			let mut fTemp1050: F64 = fTemp665 + fTemp1036;
			let mut fTemp1051: F64 = 65535.0 * (1.0 - fTemp1050);
			let mut iTemp1052: i32 = (fTemp1051) as i32;
			let mut iTemp1053: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1052, 65535)))), 196607));
			let mut fTemp1054: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1053, 3)) as usize] };
			let mut fTemp1055: F64 = unsafe { ftbl0mydspSIG0[iTemp1053 as usize] };
			let mut fTemp1056: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1053, 1)) as usize] } - fTemp1055;
			let mut fTemp1057: F64 = 65535.0 * fTemp1050;
			let mut iTemp1058: i32 = (fTemp1057) as i32;
			let mut iTemp1059: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1058, 65535)))), 196607));
			let mut fTemp1060: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1059, 3), 196607))) as usize] };
			let mut fTemp1061: F64 = unsafe { ftbl0mydspSIG0[iTemp1059 as usize] };
			let mut fTemp1062: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1059, 1), 196607))) as usize] } - fTemp1061;
			let mut iTemp1063: i32 = (fTemp721 > ((if iTemp651 != 0 {fTemp1061 + fTemp660 * fTemp1062 + (fTemp1057 - (iTemp1058) as F64) * (fTemp1060 - (fTemp1061 + fTemp660 * (fTemp1062 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1059, 4), 196607))) as usize] } - fTemp1060))))} else {1.0 - (fTemp1055 + fTemp660 * fTemp1056 + (fTemp1051 - (iTemp1052) as F64) * (fTemp1054 - (fTemp1055 + fTemp660 * (fTemp1056 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1053, 4)) as usize] } - fTemp1054)))))} - fTemp1049) / (1.0 - fTemp1049))) as i32;
			let mut fTemp1064: F64 = if iTemp1063 != 0 {fTemp1033} else {fTemp1036};
			let mut fTemp1065: F64 = if iTemp1063 != 0 {fTemp1036} else {fTemp1034};
			let mut fTemp1066: F64 = fTemp1065 + fTemp1064;
			let mut fTemp1067: F64 = 0.5 * fTemp1066;
			let mut fTemp1068: F64 = 65535.0 * (1.0 - fTemp1067);
			let mut iTemp1069: i32 = (fTemp1068) as i32;
			let mut iTemp1070: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1069, 65535)))), 196607));
			let mut fTemp1071: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1070, 3)) as usize] };
			let mut fTemp1072: F64 = unsafe { ftbl0mydspSIG0[iTemp1070 as usize] };
			let mut fTemp1073: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1070, 1)) as usize] } - fTemp1072;
			let mut fTemp1074: F64 = 32767.5 * fTemp1066;
			let mut iTemp1075: i32 = (fTemp1074) as i32;
			let mut iTemp1076: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1075, 65535)))), 196607));
			let mut fTemp1077: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1076, 3)) as usize] };
			let mut fTemp1078: F64 = unsafe { ftbl0mydspSIG0[iTemp1076 as usize] };
			let mut fTemp1079: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1076, 1)) as usize] } - fTemp1078;
			let mut fTemp1080: F64 = if iTemp651 != 0 {fTemp1078 + fTemp660 * fTemp1079 + (fTemp1074 - (iTemp1075) as F64) * (fTemp1077 - (fTemp1078 + fTemp660 * (fTemp1079 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1076, 4)) as usize] } - fTemp1077))))} else {1.0 - (fTemp1072 + fTemp660 * fTemp1073 + (fTemp1068 - (iTemp1069) as F64) * (fTemp1071 - (fTemp1072 + fTemp660 * (fTemp1073 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1070, 4)) as usize] } - fTemp1071)))))};
			let mut fTemp1081: F64 = fTemp665 + fTemp1067;
			let mut fTemp1082: F64 = 65535.0 * (1.0 - fTemp1081);
			let mut iTemp1083: i32 = (fTemp1082) as i32;
			let mut iTemp1084: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1083, 65535)))), 196607));
			let mut fTemp1085: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1084, 3)) as usize] };
			let mut fTemp1086: F64 = unsafe { ftbl0mydspSIG0[iTemp1084 as usize] };
			let mut fTemp1087: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1084, 1)) as usize] } - fTemp1086;
			let mut fTemp1088: F64 = 65535.0 * fTemp1081;
			let mut iTemp1089: i32 = (fTemp1088) as i32;
			let mut iTemp1090: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1089, 65535)))), 196607));
			let mut fTemp1091: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1090, 3), 196607))) as usize] };
			let mut fTemp1092: F64 = unsafe { ftbl0mydspSIG0[iTemp1090 as usize] };
			let mut fTemp1093: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1090, 1), 196607))) as usize] } - fTemp1092;
			let mut iTemp1094: i32 = (fTemp721 > ((if iTemp651 != 0 {fTemp1092 + fTemp660 * fTemp1093 + (fTemp1088 - (iTemp1089) as F64) * (fTemp1091 - (fTemp1092 + fTemp660 * (fTemp1093 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1090, 4), 196607))) as usize] } - fTemp1091))))} else {1.0 - (fTemp1086 + fTemp660 * fTemp1087 + (fTemp1082 - (iTemp1083) as F64) * (fTemp1085 - (fTemp1086 + fTemp660 * (fTemp1087 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1084, 4)) as usize] } - fTemp1085)))))} - fTemp1080) / (1.0 - fTemp1080))) as i32;
			let mut fTemp1095: F64 = if iTemp1094 != 0 {fTemp1064} else {fTemp1067};
			let mut fTemp1096: F64 = if iTemp1094 != 0 {fTemp1067} else {fTemp1065};
			let mut fTemp1097: F64 = fTemp1096 + fTemp1095;
			let mut fTemp1098: F64 = 0.5 * fTemp1097;
			let mut fTemp1099: F64 = 65535.0 * (1.0 - fTemp1098);
			let mut iTemp1100: i32 = (fTemp1099) as i32;
			let mut iTemp1101: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1100, 65535)))), 196607));
			let mut fTemp1102: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1101, 3)) as usize] };
			let mut fTemp1103: F64 = unsafe { ftbl0mydspSIG0[iTemp1101 as usize] };
			let mut fTemp1104: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1101, 1)) as usize] } - fTemp1103;
			let mut fTemp1105: F64 = 32767.5 * fTemp1097;
			let mut iTemp1106: i32 = (fTemp1105) as i32;
			let mut iTemp1107: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1106, 65535)))), 196607));
			let mut fTemp1108: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1107, 3)) as usize] };
			let mut fTemp1109: F64 = unsafe { ftbl0mydspSIG0[iTemp1107 as usize] };
			let mut fTemp1110: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1107, 1)) as usize] } - fTemp1109;
			let mut fTemp1111: F64 = if iTemp651 != 0 {fTemp1109 + fTemp660 * fTemp1110 + (fTemp1105 - (iTemp1106) as F64) * (fTemp1108 - (fTemp1109 + fTemp660 * (fTemp1110 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1107, 4)) as usize] } - fTemp1108))))} else {1.0 - (fTemp1103 + fTemp660 * fTemp1104 + (fTemp1099 - (iTemp1100) as F64) * (fTemp1102 - (fTemp1103 + fTemp660 * (fTemp1104 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1101, 4)) as usize] } - fTemp1102)))))};
			let mut fTemp1112: F64 = fTemp665 + fTemp1098;
			let mut fTemp1113: F64 = 65535.0 * (1.0 - fTemp1112);
			let mut iTemp1114: i32 = (fTemp1113) as i32;
			let mut iTemp1115: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1114, 65535)))), 196607));
			let mut fTemp1116: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1115, 3)) as usize] };
			let mut fTemp1117: F64 = unsafe { ftbl0mydspSIG0[iTemp1115 as usize] };
			let mut fTemp1118: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1115, 1)) as usize] } - fTemp1117;
			let mut fTemp1119: F64 = 65535.0 * fTemp1112;
			let mut iTemp1120: i32 = (fTemp1119) as i32;
			let mut iTemp1121: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1120, 65535)))), 196607));
			let mut fTemp1122: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1121, 3), 196607))) as usize] };
			let mut fTemp1123: F64 = unsafe { ftbl0mydspSIG0[iTemp1121 as usize] };
			let mut fTemp1124: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1121, 1), 196607))) as usize] } - fTemp1123;
			let mut iTemp1125: i32 = (fTemp721 > ((if iTemp651 != 0 {fTemp1123 + fTemp660 * fTemp1124 + (fTemp1119 - (iTemp1120) as F64) * (fTemp1122 - (fTemp1123 + fTemp660 * (fTemp1124 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1121, 4), 196607))) as usize] } - fTemp1122))))} else {1.0 - (fTemp1117 + fTemp660 * fTemp1118 + (fTemp1113 - (iTemp1114) as F64) * (fTemp1116 - (fTemp1117 + fTemp660 * (fTemp1118 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1115, 4)) as usize] } - fTemp1116)))))} - fTemp1111) / (1.0 - fTemp1111))) as i32;
			let mut fTemp1126: F64 = if iTemp1125 != 0 {fTemp1095} else {fTemp1098};
			let mut fTemp1127: F64 = if iTemp1125 != 0 {fTemp1098} else {fTemp1096};
			let mut fTemp1128: F64 = fTemp1127 + fTemp1126;
			let mut fTemp1129: F64 = 0.5 * fTemp1128;
			let mut fTemp1130: F64 = 65535.0 * (1.0 - fTemp1129);
			let mut iTemp1131: i32 = (fTemp1130) as i32;
			let mut iTemp1132: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1131, 65535)))), 196607));
			let mut fTemp1133: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1132, 3)) as usize] };
			let mut fTemp1134: F64 = unsafe { ftbl0mydspSIG0[iTemp1132 as usize] };
			let mut fTemp1135: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1132, 1)) as usize] } - fTemp1134;
			let mut fTemp1136: F64 = 32767.5 * fTemp1128;
			let mut iTemp1137: i32 = (fTemp1136) as i32;
			let mut iTemp1138: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1137, 65535)))), 196607));
			let mut fTemp1139: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1138, 3)) as usize] };
			let mut fTemp1140: F64 = unsafe { ftbl0mydspSIG0[iTemp1138 as usize] };
			let mut fTemp1141: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1138, 1)) as usize] } - fTemp1140;
			let mut fTemp1142: F64 = if iTemp651 != 0 {fTemp1140 + fTemp660 * fTemp1141 + (fTemp1136 - (iTemp1137) as F64) * (fTemp1139 - (fTemp1140 + fTemp660 * (fTemp1141 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1138, 4)) as usize] } - fTemp1139))))} else {1.0 - (fTemp1134 + fTemp660 * fTemp1135 + (fTemp1130 - (iTemp1131) as F64) * (fTemp1133 - (fTemp1134 + fTemp660 * (fTemp1135 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1132, 4)) as usize] } - fTemp1133)))))};
			let mut fTemp1143: F64 = fTemp665 + fTemp1129;
			let mut fTemp1144: F64 = 65535.0 * (1.0 - fTemp1143);
			let mut iTemp1145: i32 = (fTemp1144) as i32;
			let mut iTemp1146: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1145, 65535)))), 196607));
			let mut fTemp1147: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1146, 3)) as usize] };
			let mut fTemp1148: F64 = unsafe { ftbl0mydspSIG0[iTemp1146 as usize] };
			let mut fTemp1149: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1146, 1)) as usize] } - fTemp1148;
			let mut fTemp1150: F64 = 65535.0 * fTemp1143;
			let mut iTemp1151: i32 = (fTemp1150) as i32;
			let mut iTemp1152: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1151, 65535)))), 196607));
			let mut fTemp1153: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1152, 3), 196607))) as usize] };
			let mut fTemp1154: F64 = unsafe { ftbl0mydspSIG0[iTemp1152 as usize] };
			let mut fTemp1155: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1152, 1), 196607))) as usize] } - fTemp1154;
			let mut iTemp1156: i32 = (fTemp721 > ((if iTemp651 != 0 {fTemp1154 + fTemp660 * fTemp1155 + (fTemp1150 - (iTemp1151) as F64) * (fTemp1153 - (fTemp1154 + fTemp660 * (fTemp1155 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1152, 4), 196607))) as usize] } - fTemp1153))))} else {1.0 - (fTemp1148 + fTemp660 * fTemp1149 + (fTemp1144 - (iTemp1145) as F64) * (fTemp1147 - (fTemp1148 + fTemp660 * (fTemp1149 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1146, 4)) as usize] } - fTemp1147)))))} - fTemp1142) / (1.0 - fTemp1142))) as i32;
			let mut fTemp1157: F64 = if iTemp1156 != 0 {fTemp1126} else {fTemp1129};
			let mut fTemp1158: F64 = if iTemp1156 != 0 {fTemp1129} else {fTemp1127};
			let mut fTemp1159: F64 = fTemp1158 + fTemp1157;
			let mut fTemp1160: F64 = 0.5 * fTemp1159;
			let mut fTemp1161: F64 = 65535.0 * (1.0 - fTemp1160);
			let mut iTemp1162: i32 = (fTemp1161) as i32;
			let mut iTemp1163: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1162, 65535)))), 196607));
			let mut fTemp1164: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1163, 3)) as usize] };
			let mut fTemp1165: F64 = unsafe { ftbl0mydspSIG0[iTemp1163 as usize] };
			let mut fTemp1166: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1163, 1)) as usize] } - fTemp1165;
			let mut fTemp1167: F64 = 32767.5 * fTemp1159;
			let mut iTemp1168: i32 = (fTemp1167) as i32;
			let mut iTemp1169: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1168, 65535)))), 196607));
			let mut fTemp1170: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1169, 3)) as usize] };
			let mut fTemp1171: F64 = unsafe { ftbl0mydspSIG0[iTemp1169 as usize] };
			let mut fTemp1172: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1169, 1)) as usize] } - fTemp1171;
			let mut fTemp1173: F64 = if iTemp651 != 0 {fTemp1171 + fTemp660 * fTemp1172 + (fTemp1167 - (iTemp1168) as F64) * (fTemp1170 - (fTemp1171 + fTemp660 * (fTemp1172 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1169, 4), 196607))) as usize] } - fTemp1170))))} else {1.0 - (fTemp1165 + fTemp660 * fTemp1166 + (fTemp1161 - (iTemp1162) as F64) * (fTemp1164 - (fTemp1165 + fTemp660 * (fTemp1166 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1163, 4), 196607))) as usize] } - fTemp1164)))))};
			let mut fTemp1174: F64 = fTemp665 + fTemp1160;
			let mut fTemp1175: F64 = 65535.0 * (1.0 - fTemp1174);
			let mut iTemp1176: i32 = (fTemp1175) as i32;
			let mut iTemp1177: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1176, 65535)))), 196607));
			let mut fTemp1178: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1177, 3)) as usize] };
			let mut fTemp1179: F64 = unsafe { ftbl0mydspSIG0[iTemp1177 as usize] };
			let mut fTemp1180: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1177, 1)) as usize] } - fTemp1179;
			let mut fTemp1181: F64 = 65535.0 * fTemp1174;
			let mut iTemp1182: i32 = (fTemp1181) as i32;
			let mut iTemp1183: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1182, 65535)))), 196607));
			let mut fTemp1184: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1183, 3), 196607))) as usize] };
			let mut fTemp1185: F64 = unsafe { ftbl0mydspSIG0[iTemp1183 as usize] };
			let mut fTemp1186: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1183, 1), 196607))) as usize] } - fTemp1185;
			let mut iTemp1187: i32 = (fTemp721 > ((if iTemp651 != 0 {fTemp1185 + fTemp660 * fTemp1186 + (fTemp1181 - (iTemp1182) as F64) * (fTemp1184 - (fTemp1185 + fTemp660 * (fTemp1186 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1183, 4), 196607))) as usize] } - fTemp1184))))} else {1.0 - (fTemp1179 + fTemp660 * fTemp1180 + (fTemp1175 - (iTemp1176) as F64) * (fTemp1178 - (fTemp1179 + fTemp660 * (fTemp1180 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1177, 4)) as usize] } - fTemp1178)))))} - fTemp1173) / (1.0 - fTemp1173))) as i32;
			let mut fTemp1188: F64 = F64::min(1.0, F64::max(0.0, 0.5 * (if iTemp1187 != 0 {fTemp1160} else {fTemp1158} + if iTemp1187 != 0 {fTemp1157} else {fTemp1160})));
			self.fRec15[0] = fTemp1188;
			let mut fTemp1189: F64 = 65535.0 * (1.0 - fTemp1188);
			let mut iTemp1190: i32 = (fTemp1189) as i32;
			let mut iTemp1191: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1190, 65535)))), 196607));
			let mut fTemp1192: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1191, 3)) as usize] };
			let mut fTemp1193: F64 = unsafe { ftbl0mydspSIG0[iTemp1191 as usize] };
			let mut fTemp1194: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1191, 1)) as usize] } - fTemp1193;
			let mut fTemp1195: F64 = 65535.0 * fTemp1188;
			let mut iTemp1196: i32 = (fTemp1195) as i32;
			let mut iTemp1197: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1196, 65535)))), 196607));
			let mut fTemp1198: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1197, 3)) as usize] };
			let mut fTemp1199: F64 = unsafe { ftbl0mydspSIG0[iTemp1197 as usize] };
			let mut fTemp1200: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1197, 1)) as usize] } - fTemp1199;
			let mut fTemp1201: F64 = if iTemp651 != 0 {fTemp1199 + fTemp660 * fTemp1200 + (fTemp1195 - (iTemp1196) as F64) * (fTemp1198 - (fTemp1199 + fTemp660 * (fTemp1200 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1197, 4), 196607))) as usize] } - fTemp1198))))} else {1.0 - (fTemp1193 + fTemp660 * fTemp1194 + (fTemp1189 - (iTemp1190) as F64) * (fTemp1192 - (fTemp1193 + fTemp660 * (fTemp1194 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1191, 4), 196607))) as usize] } - fTemp1192)))))};
			let mut fTemp1202: F64 = fTemp665 + fTemp1188;
			let mut fTemp1203: F64 = 65535.0 * (1.0 - fTemp1202);
			let mut iTemp1204: i32 = (fTemp1203) as i32;
			let mut iTemp1205: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1204, 65535)))), 196607));
			let mut fTemp1206: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1205, 3)) as usize] };
			let mut fTemp1207: F64 = unsafe { ftbl0mydspSIG0[iTemp1205 as usize] };
			let mut fTemp1208: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1205, 1)) as usize] } - fTemp1207;
			let mut fTemp1209: F64 = 65535.0 * fTemp1202;
			let mut iTemp1210: i32 = (fTemp1209) as i32;
			let mut iTemp1211: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1210, 65535)))), 196607));
			let mut fTemp1212: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1211, 3), 196607))) as usize] };
			let mut fTemp1213: F64 = unsafe { ftbl0mydspSIG0[iTemp1211 as usize] };
			let mut fTemp1214: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1211, 1), 196607))) as usize] } - fTemp1213;
			let mut fTemp1215: F64 = fTemp625 + if ((0.001 * fTemp664) == 0.0) as i32 != 0 {fTemp650} else {fTemp650 * (if iTemp651 != 0 {fTemp1213 + fTemp660 * fTemp1214 + (fTemp1209 - (iTemp1210) as F64) * (fTemp1212 - (fTemp1213 + fTemp660 * (fTemp1214 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1211, 4), 196607))) as usize] } - fTemp1212))))} else {1.0 - (fTemp1207 + fTemp660 * fTemp1208 + (fTemp1203 - (iTemp1204) as F64) * (fTemp1206 - (fTemp1207 + fTemp660 * (fTemp1208 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1205, 4)) as usize] } - fTemp1206)))))} - fTemp1201) / (1.0 - fTemp1201)};
			self.fRec16[(self.IOTA0 & 8191) as usize] = if iTemp663 != 0 {F64::min(fTemp1215, fTemp625)} else {F64::max(fTemp1215, fTemp625)};
			let mut fTemp1216: F64 = self.fRec16[((i32::wrapping_sub(self.IOTA0, iSlow70)) & 8191) as usize];
			self.fHbargraph2 = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, fTemp1216));
			*output1 = 0.5 * fTemp2 * self.fVec2[((i32::wrapping_sub(self.IOTA0, iSlow71)) & 32767) as usize] + self.fRec14[0] * fTemp3 * self.fVec3[((i32::wrapping_sub(self.IOTA0, iSlow71)) & 32767) as usize] * fTemp1216;
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
