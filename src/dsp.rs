mod dsp {
    /* ------------------------------------------------------------
author: "Bart Brouns"
license: "AGPLv3"
name: "lamb-rs"
version: "0.1"
Code generated with Faust 2.72.10 (https://faust.grame.fr)
Compilation options: -a /run/user/1001/.tmpCOlaXJ -lang rust -ct 1 -es 1 -mcd 16 -mdd 1024 -mdy 33 -double -ftz 0
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
	iRec6: [i32;2],
}

impl mydspSIG0 {
	
	fn get_num_inputsmydspSIG0(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsmydspSIG0(&self) -> i32 {
		return 1;
	}
	
	fn instance_initmydspSIG0(&mut self, sample_rate: i32) {
		for l32 in 0..2 {
			self.iRec6[l32 as usize] = 0;
		}
	}
	
	fn fillmydspSIG0(&mut self, count: i32, table: &mut[F64]) {
		for i1 in 0..count {
			self.iRec6[0] = i32::wrapping_add(self.iRec6[1], 1);
			let mut iTemp39: i32 = i32::wrapping_add(self.iRec6[0], -1);
			let mut fTemp40: F64 = (iTemp39 % 3) as F64 as i32 as F64;
			let mut fTemp41: F64 = 0.5 * fTemp40;
			let mut fTemp42: F64 = F64::powf(fTemp41, 0.21 * fTemp40 + 1.0);
			let mut fTemp43: F64 = (0.3333333333333333 * (iTemp39 % 196608) as F64) as i32 as F64;
			table[i1 as usize] = F64::min(1.0, F64::max(0.0, if (fTemp41 == 0.0) as i32 != 0 {0.5 * (F64::sin(4.793763109162727e-05 * fTemp43 + 4.71238898038469) + 1.0)} else {0.5 * (F64::sin(3.141592653589793 * ((1.0 - F64::exp(-(3.692683299000534e-05 * fTemp42 * fTemp43))) / (1.0 - F64::exp(-(2.42 * fTemp42)))) + 4.71238898038469) + 1.0)}));
			self.iRec6[1] = self.iRec6[0];
		}
	}

}


pub fn newmydspSIG0() -> mydspSIG0 { 
	mydspSIG0 {
		iRec6: [0;2],
	}
}
fn mydsp_faustpower2_f(value: F64) -> F64 {
	return value * value;
}
static mut ftbl0mydspSIG0: [F64;196608] = [0.0;196608];
mod ffi {
	use std::os::raw::{c_double};
	#[link(name = "m")]
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
	fConst1: F64,
	fRec0: [F64;2],
	fConst2: F64,
	fConst3: F64,
	fHslider0: F64,
	fRec1: [F64;2],
	IOTA0: i32,
	fVec0: [F64;32768],
	fVec1: [F64;32768],
	fHslider1: F64,
	fHslider2: F64,
	fConst4: F64,
	fHbargraph0: F64,
	fHslider3: F64,
	fRec2: [F64;2],
	fHslider4: F64,
	fHslider5: F64,
	fVec2: [F64;32768],
	fVec3: [F64;32768],
	fHslider6: F64,
	fHslider7: F64,
	fVec4: [F64;16384],
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
	fRec5: [F64;16384],
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
	fHslider8: F64,
	fHslider9: F64,
	fVec28: [F64;2],
	fHslider10: F64,
	fVec29: [F64;2],
	fConst5: F64,
	fRec3: [F64;2],
	fRec4: [F64;2],
	fVbargraph0: F64,
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
	fRec9: [F64;16384],
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
	fRec7: [F64;2],
	fRec8: [F64;2],
	fVbargraph1: F64,
}

impl FaustDsp for mydsp {
	type T = F64;
		
	fn new() -> mydsp { 
		mydsp {
			fCheckbox0: 0.0,
			fSampleRate: 0,
			fConst1: 0.0,
			fRec0: [0.0;2],
			fConst2: 0.0,
			fConst3: 0.0,
			fHslider0: 0.0,
			fRec1: [0.0;2],
			IOTA0: 0,
			fVec0: [0.0;32768],
			fVec1: [0.0;32768],
			fHslider1: 0.0,
			fHslider2: 0.0,
			fConst4: 0.0,
			fHbargraph0: 0.0,
			fHslider3: 0.0,
			fRec2: [0.0;2],
			fHslider4: 0.0,
			fHslider5: 0.0,
			fVec2: [0.0;32768],
			fVec3: [0.0;32768],
			fHslider6: 0.0,
			fHslider7: 0.0,
			fVec4: [0.0;16384],
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
			fRec5: [0.0;16384],
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
			fHslider8: 0.0,
			fHslider9: 0.0,
			fVec28: [0.0;2],
			fHslider10: 0.0,
			fVec29: [0.0;2],
			fConst5: 0.0,
			fRec3: [0.0;2],
			fRec4: [0.0;2],
			fVbargraph0: 0.0,
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
			fRec9: [0.0;16384],
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
			fRec7: [0.0;2],
			fRec8: [0.0;2],
			fVbargraph1: 0.0,
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
		m.declare("compile_options", r"-a /run/user/1001/.tmpCOlaXJ -lang rust -ct 1 -es 1 -mcd 16 -mdd 1024 -mdy 33 -double -ftz 0");
		m.declare("filename", r"lamb-rs.dsp");
		m.declare("interpolators.lib/interpolate_linear:author", r"Stéphane Letz");
		m.declare("interpolators.lib/interpolate_linear:licence", r"MIT");
		m.declare("interpolators.lib/name", r"Faust Interpolator Library");
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
		self.fHslider0 = 0.0;
		self.fHslider1 = 5e+01;
		self.fHslider2 = 9.0;
		self.fHslider3 = 0.0;
		self.fHslider4 = 1.0;
		self.fHslider5 = -1.0;
		self.fHslider6 = 0.0;
		self.fHslider7 = 1e+02;
		self.fHslider8 = 0.0;
		self.fHslider9 = 0.5;
		self.fHslider10 = 6e+01;
	}
	fn instance_clear(&mut self) {
		for l0 in 0..2 {
			self.fRec0[l0 as usize] = 0.0;
		}
		for l1 in 0..2 {
			self.fRec1[l1 as usize] = 0.0;
		}
		self.IOTA0 = 0;
		for l2 in 0..32768 {
			self.fVec0[l2 as usize] = 0.0;
		}
		for l3 in 0..32768 {
			self.fVec1[l3 as usize] = 0.0;
		}
		for l4 in 0..2 {
			self.fRec2[l4 as usize] = 0.0;
		}
		for l5 in 0..32768 {
			self.fVec2[l5 as usize] = 0.0;
		}
		for l6 in 0..32768 {
			self.fVec3[l6 as usize] = 0.0;
		}
		for l7 in 0..16384 {
			self.fVec4[l7 as usize] = 0.0;
		}
		for l8 in 0..3 {
			self.fVec5[l8 as usize] = 0.0;
		}
		for l9 in 0..7 {
			self.fVec6[l9 as usize] = 0.0;
		}
		for l10 in 0..15 {
			self.fVec7[l10 as usize] = 0.0;
		}
		for l11 in 0..32 {
			self.fVec8[l11 as usize] = 0.0;
		}
		for l12 in 0..64 {
			self.fVec9[l12 as usize] = 0.0;
		}
		for l13 in 0..128 {
			self.fVec10[l13 as usize] = 0.0;
		}
		for l14 in 0..256 {
			self.fVec11[l14 as usize] = 0.0;
		}
		for l15 in 0..512 {
			self.fVec12[l15 as usize] = 0.0;
		}
		for l16 in 0..1024 {
			self.fVec13[l16 as usize] = 0.0;
		}
		for l17 in 0..2048 {
			self.fVec14[l17 as usize] = 0.0;
		}
		for l18 in 0..4096 {
			self.fVec15[l18 as usize] = 0.0;
		}
		for l19 in 0..16384 {
			self.fRec5[l19 as usize] = 0.0;
		}
		for l20 in 0..3 {
			self.fVec16[l20 as usize] = 0.0;
		}
		for l21 in 0..7 {
			self.fVec17[l21 as usize] = 0.0;
		}
		for l22 in 0..15 {
			self.fVec18[l22 as usize] = 0.0;
		}
		for l23 in 0..32 {
			self.fVec19[l23 as usize] = 0.0;
		}
		for l24 in 0..64 {
			self.fVec20[l24 as usize] = 0.0;
		}
		for l25 in 0..128 {
			self.fVec21[l25 as usize] = 0.0;
		}
		for l26 in 0..256 {
			self.fVec22[l26 as usize] = 0.0;
		}
		for l27 in 0..512 {
			self.fVec23[l27 as usize] = 0.0;
		}
		for l28 in 0..1024 {
			self.fVec24[l28 as usize] = 0.0;
		}
		for l29 in 0..2048 {
			self.fVec25[l29 as usize] = 0.0;
		}
		for l30 in 0..4096 {
			self.fVec26[l30 as usize] = 0.0;
		}
		for l31 in 0..2 {
			self.fVec27[l31 as usize] = 0.0;
		}
		for l33 in 0..2 {
			self.fVec28[l33 as usize] = 0.0;
		}
		for l34 in 0..2 {
			self.fVec29[l34 as usize] = 0.0;
		}
		for l35 in 0..2 {
			self.fRec3[l35 as usize] = 0.0;
		}
		for l36 in 0..2 {
			self.fRec4[l36 as usize] = 0.0;
		}
		for l37 in 0..16384 {
			self.fVec30[l37 as usize] = 0.0;
		}
		for l38 in 0..3 {
			self.fVec31[l38 as usize] = 0.0;
		}
		for l39 in 0..7 {
			self.fVec32[l39 as usize] = 0.0;
		}
		for l40 in 0..15 {
			self.fVec33[l40 as usize] = 0.0;
		}
		for l41 in 0..32 {
			self.fVec34[l41 as usize] = 0.0;
		}
		for l42 in 0..64 {
			self.fVec35[l42 as usize] = 0.0;
		}
		for l43 in 0..128 {
			self.fVec36[l43 as usize] = 0.0;
		}
		for l44 in 0..256 {
			self.fVec37[l44 as usize] = 0.0;
		}
		for l45 in 0..512 {
			self.fVec38[l45 as usize] = 0.0;
		}
		for l46 in 0..1024 {
			self.fVec39[l46 as usize] = 0.0;
		}
		for l47 in 0..2048 {
			self.fVec40[l47 as usize] = 0.0;
		}
		for l48 in 0..4096 {
			self.fVec41[l48 as usize] = 0.0;
		}
		for l49 in 0..16384 {
			self.fRec9[l49 as usize] = 0.0;
		}
		for l50 in 0..3 {
			self.fVec42[l50 as usize] = 0.0;
		}
		for l51 in 0..7 {
			self.fVec43[l51 as usize] = 0.0;
		}
		for l52 in 0..15 {
			self.fVec44[l52 as usize] = 0.0;
		}
		for l53 in 0..32 {
			self.fVec45[l53 as usize] = 0.0;
		}
		for l54 in 0..64 {
			self.fVec46[l54 as usize] = 0.0;
		}
		for l55 in 0..128 {
			self.fVec47[l55 as usize] = 0.0;
		}
		for l56 in 0..256 {
			self.fVec48[l56 as usize] = 0.0;
		}
		for l57 in 0..512 {
			self.fVec49[l57 as usize] = 0.0;
		}
		for l58 in 0..1024 {
			self.fVec50[l58 as usize] = 0.0;
		}
		for l59 in 0..2048 {
			self.fVec51[l59 as usize] = 0.0;
		}
		for l60 in 0..4096 {
			self.fVec52[l60 as usize] = 0.0;
		}
		for l61 in 0..2 {
			self.fVec53[l61 as usize] = 0.0;
		}
		for l62 in 0..2 {
			self.fVec54[l62 as usize] = 0.0;
		}
		for l63 in 0..2 {
			self.fVec55[l63 as usize] = 0.0;
		}
		for l64 in 0..2 {
			self.fRec7[l64 as usize] = 0.0;
		}
		for l65 in 0..2 {
			self.fRec8[l65 as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		let mut fConst0: F64 = F64::min(1.92e+05, F64::max(1.0, (self.fSampleRate) as F64));
		self.fConst1 = 1e+02 / fConst0;
		self.fConst2 = 44.1 / fConst0;
		self.fConst3 = 1.0 - self.fConst2;
		self.fConst4 = 0.001 * fConst0;
		self.fConst5 = 1e+03 / fConst0;
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
		ui_interface.declare(None, "02", "");
		ui_interface.open_horizontal_box("0x00");
		ui_interface.declare(None, "0", "");
		ui_interface.open_vertical_box("0x00");
		ui_interface.declare(Some(ParamIndex(0)), "00", "");
		ui_interface.add_check_button("bypass", ParamIndex(0));
		ui_interface.declare(Some(ParamIndex(1)), "01", "");
		ui_interface.add_horizontal_slider("input gain", ParamIndex(1), 0.0, -24.0, 24.0, 0.1);
		ui_interface.declare(Some(ParamIndex(2)), "02", "");
		ui_interface.add_horizontal_slider("strength", ParamIndex(2), 1e+02, 0.0, 1e+02, 1.0);
		ui_interface.declare(Some(ParamIndex(3)), "03", "");
		ui_interface.add_horizontal_slider("thresh", ParamIndex(3), -1.0, -3e+01, 0.0, 0.1);
		ui_interface.declare(Some(ParamIndex(4)), "04", "");
		ui_interface.declare(Some(ParamIndex(4)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(4)), "unit", "ms");
		ui_interface.add_horizontal_slider("attack", ParamIndex(4), 9.0, 0.0, 5e+01, 0.1);
		ui_interface.declare(Some(ParamIndex(5)), "05", "");
		ui_interface.add_horizontal_slider("attack shape", ParamIndex(5), 0.0, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(6)), "06", "");
		ui_interface.declare(Some(ParamIndex(6)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(6)), "unit", "ms");
		ui_interface.add_horizontal_slider("release", ParamIndex(6), 6e+01, 1.0, 5e+02, 1.0);
		ui_interface.declare(Some(ParamIndex(7)), "07", "");
		ui_interface.add_horizontal_slider("release shape", ParamIndex(7), 0.5, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(8)), "08", "");
		ui_interface.declare(Some(ParamIndex(8)), "unit", "ms");
		ui_interface.add_horizontal_slider("release hold", ParamIndex(8), 5e+01, 0.020833333333333332, 5e+01, 1.0);
		ui_interface.declare(Some(ParamIndex(9)), "09", "");
		ui_interface.add_horizontal_slider("knee", ParamIndex(9), 1.0, 0.0, 3e+01, 0.1);
		ui_interface.declare(Some(ParamIndex(10)), "10", "");
		ui_interface.add_horizontal_slider("link", ParamIndex(10), 0.0, 0.0, 1e+02, 1.0);
		ui_interface.declare(Some(ParamIndex(11)), "11", "");
		ui_interface.add_horizontal_slider("output gain", ParamIndex(11), 0.0, -24.0, 24.0, 0.1);
		ui_interface.close_box();
		ui_interface.declare(None, "99", "");
		ui_interface.open_horizontal_box("gain reduction");
		ui_interface.declare(Some(ParamIndex(12)), "unit", "dB");
		ui_interface.add_vertical_bargraph("0", ParamIndex(12), -24.0, 0.0);
		ui_interface.declare(Some(ParamIndex(13)), "unit", "dB");
		ui_interface.add_vertical_bargraph("1", ParamIndex(13), -24.0, 0.0);
		ui_interface.close_box();
		ui_interface.close_box();
		ui_interface.declare(Some(ParamIndex(14)), "99", "");
		ui_interface.declare(Some(ParamIndex(14)), "unit", "samples");
		ui_interface.add_horizontal_bargraph("latency", ParamIndex(14), 0.0, 4.8e+03);
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			0 => Some(self.fCheckbox0),
			14 => Some(self.fHbargraph0),
			1 => Some(self.fHslider0),
			8 => Some(self.fHslider1),
			6 => Some(self.fHslider10),
			4 => Some(self.fHslider2),
			11 => Some(self.fHslider3),
			9 => Some(self.fHslider4),
			3 => Some(self.fHslider5),
			10 => Some(self.fHslider6),
			2 => Some(self.fHslider7),
			5 => Some(self.fHslider8),
			7 => Some(self.fHslider9),
			12 => Some(self.fVbargraph0),
			13 => Some(self.fVbargraph1),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			0 => { self.fCheckbox0 = value }
			14 => { self.fHbargraph0 = value }
			1 => { self.fHslider0 = value }
			8 => { self.fHslider1 = value }
			6 => { self.fHslider10 = value }
			4 => { self.fHslider2 = value }
			11 => { self.fHslider3 = value }
			9 => { self.fHslider4 = value }
			3 => { self.fHslider5 = value }
			10 => { self.fHslider6 = value }
			2 => { self.fHslider7 = value }
			5 => { self.fHslider8 = value }
			7 => { self.fHslider9 = value }
			12 => { self.fVbargraph0 = value }
			13 => { self.fVbargraph1 = value }
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
		let mut fSlow1: F64 = self.fConst2 * F64::powf(1e+01, 0.05 * self.fHslider0);
		let mut fSlow2: F64 = self.fHslider1;
		let mut fSlow3: F64 = self.fHslider2;
		self.fHbargraph0 = self.fConst4 * (fSlow3 + fSlow2);
		let mut iSlow4: i32 = (self.fHbargraph0) as i32;
		let mut fSlow5: F64 = self.fConst2 * F64::powf(1e+01, 0.05 * self.fHslider3);
		let mut fSlow6: F64 = self.fConst4 * fSlow3;
		let mut fSlow7: F64 = fSlow6 + 1.0;
		let mut iSlow8: i32 = (F64::floor(fSlow7)) as i32 % 2;
		let mut fSlow9: F64 = self.fHslider4;
		let mut fSlow10: F64 = 0.5 * fSlow9;
		let mut fSlow11: F64 = self.fHslider5;
		let mut fSlow12: F64 = fSlow11 + fSlow10;
		let mut fSlow13: F64 = 0.01 * self.fHslider6;
		let mut fSlow14: F64 = fSlow11 - fSlow10;
		let mut fSlow15: F64 = 0.5 / F64::max(2.220446049250313e-16, fSlow9);
		let mut fSlow16: F64 = 0.0005 * self.fHslider7;
		let mut fSlow17: F64 = self.fConst4 * fSlow2;
		let mut iSlow18: i32 = (fSlow17) as i32;
		let mut fSlow19: F64 = fSlow17 + 1.0;
		let mut iSlow20: i32 = (F64::floor(fSlow19)) as i32 % 2;
		let mut iSlow21: i32 = (F64::floor(0.5 * fSlow19)) as i32 % 2;
		let mut iSlow22: i32 = (F64::floor(0.25 * fSlow19)) as i32 % 2;
		let mut iSlow23: i32 = i32::wrapping_add(iSlow20, i32::wrapping_mul(2, iSlow21));
		let mut iSlow24: i32 = (F64::floor(0.125 * fSlow19)) as i32 % 2;
		let mut iSlow25: i32 = i32::wrapping_add(iSlow23, i32::wrapping_mul(4, iSlow22));
		let mut iSlow26: i32 = (F64::floor(0.0625 * fSlow19)) as i32 % 2;
		let mut iSlow27: i32 = i32::wrapping_add(iSlow25, i32::wrapping_mul(8, iSlow24));
		let mut iSlow28: i32 = (F64::floor(0.03125 * fSlow19)) as i32 % 2;
		let mut iSlow29: i32 = i32::wrapping_add(iSlow27, i32::wrapping_mul(16, iSlow26));
		let mut iSlow30: i32 = (F64::floor(0.015625 * fSlow19)) as i32 % 2;
		let mut iSlow31: i32 = i32::wrapping_add(iSlow29, i32::wrapping_mul(32, iSlow28));
		let mut iSlow32: i32 = (F64::floor(0.0078125 * fSlow19)) as i32 % 2;
		let mut iSlow33: i32 = i32::wrapping_add(iSlow31, i32::wrapping_mul(64, iSlow30));
		let mut iSlow34: i32 = (F64::floor(0.00390625 * fSlow19)) as i32 % 2;
		let mut iSlow35: i32 = i32::wrapping_add(iSlow33, i32::wrapping_mul(128, iSlow32));
		let mut iSlow36: i32 = (F64::floor(0.001953125 * fSlow19)) as i32 % 2;
		let mut iSlow37: i32 = i32::wrapping_add(iSlow35, i32::wrapping_mul(256, iSlow34));
		let mut iSlow38: i32 = (F64::floor(0.0009765625 * fSlow19)) as i32 % 2;
		let mut iSlow39: i32 = i32::wrapping_add(iSlow37, i32::wrapping_mul(512, iSlow36));
		let mut iSlow40: i32 = (F64::floor(0.00048828125 * fSlow19)) as i32 % 2;
		let mut iSlow41: i32 = i32::wrapping_add(iSlow39, i32::wrapping_mul(1024, iSlow38));
		let mut iSlow42: i32 = (F64::floor(0.5 * fSlow7)) as i32 % 2;
		let mut iSlow43: i32 = (F64::floor(0.25 * fSlow7)) as i32 % 2;
		let mut iSlow44: i32 = i32::wrapping_add(iSlow8, i32::wrapping_mul(2, iSlow42));
		let mut iSlow45: i32 = (F64::floor(0.125 * fSlow7)) as i32 % 2;
		let mut iSlow46: i32 = i32::wrapping_add(iSlow44, i32::wrapping_mul(4, iSlow43));
		let mut iSlow47: i32 = (F64::floor(0.0625 * fSlow7)) as i32 % 2;
		let mut iSlow48: i32 = i32::wrapping_add(iSlow46, i32::wrapping_mul(8, iSlow45));
		let mut iSlow49: i32 = (F64::floor(0.03125 * fSlow7)) as i32 % 2;
		let mut iSlow50: i32 = i32::wrapping_add(iSlow48, i32::wrapping_mul(16, iSlow47));
		let mut iSlow51: i32 = (F64::floor(0.015625 * fSlow7)) as i32 % 2;
		let mut iSlow52: i32 = i32::wrapping_add(iSlow50, i32::wrapping_mul(32, iSlow49));
		let mut iSlow53: i32 = (F64::floor(0.0078125 * fSlow7)) as i32 % 2;
		let mut iSlow54: i32 = i32::wrapping_add(iSlow52, i32::wrapping_mul(64, iSlow51));
		let mut iSlow55: i32 = (F64::floor(0.00390625 * fSlow7)) as i32 % 2;
		let mut iSlow56: i32 = i32::wrapping_add(iSlow54, i32::wrapping_mul(128, iSlow53));
		let mut iSlow57: i32 = (F64::floor(0.001953125 * fSlow7)) as i32 % 2;
		let mut iSlow58: i32 = i32::wrapping_add(iSlow56, i32::wrapping_mul(256, iSlow55));
		let mut iSlow59: i32 = (F64::floor(0.0009765625 * fSlow7)) as i32 % 2;
		let mut iSlow60: i32 = i32::wrapping_add(iSlow58, i32::wrapping_mul(512, iSlow57));
		let mut iSlow61: i32 = (F64::floor(0.00048828125 * fSlow7)) as i32 % 2;
		let mut iSlow62: i32 = i32::wrapping_add(iSlow60, i32::wrapping_mul(1024, iSlow59));
		let mut fSlow63: F64 = self.fHslider8;
		let mut fSlow64: F64 = self.fHslider9;
		let mut fSlow65: F64 = self.fHslider10;
		let mut iSlow66: i32 = (fSlow6) as i32;
		let zipped_iterators = inputs0.zip(inputs1).zip(outputs0).zip(outputs1);
		for (((input0, input1), output0), output1) in zipped_iterators {
			let mut fTemp0: F64 = self.fConst1 + self.fRec0[1];
			let mut fTemp1: F64 = self.fRec0[1] - self.fConst1;
			self.fRec0[0] = if (fTemp0 < fSlow0) as i32 != 0 {fTemp0} else {if (fTemp1 > fSlow0) as i32 != 0 {fTemp1} else {fSlow0}};
			let mut fTemp2: F64 = F64::sin(6.283185307179586 * (0.5 * self.fRec0[0] + 0.75)) + 1.0;
			let mut fTemp3: F64 = 1.0 - 0.5 * fTemp2;
			self.fRec1[0] = fSlow1 + self.fConst3 * self.fRec1[1];
			let mut fTemp4: F64 = *input0;
			self.fVec0[(self.IOTA0 & 32767) as usize] = fTemp4;
			let mut fTemp5: F64 = fTemp4 * self.fRec1[0];
			self.fVec1[(self.IOTA0 & 32767) as usize] = fTemp5;
			self.fRec2[0] = fSlow5 + self.fConst3 * self.fRec2[1];
			let mut fTemp6: F64 = self.fRec5[((i32::wrapping_sub(self.IOTA0, 1)) & 16383) as usize];
			let mut fTemp7: F64 = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, F64::abs(fTemp5)));
			let mut fTemp8: F64 = *input1;
			self.fVec2[(self.IOTA0 & 32767) as usize] = fTemp8;
			let mut fTemp9: F64 = fTemp8 * self.fRec1[0];
			self.fVec3[(self.IOTA0 & 32767) as usize] = fTemp9;
			let mut fTemp10: F64 = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, F64::abs(fTemp9)));
			let mut fTemp11: F64 = F64::max(fTemp7, fTemp10);
			let mut fTemp12: F64 = fTemp7 + fSlow13 * (fTemp11 - fTemp7);
			let mut iTemp13: i32 = ((fTemp12 > fSlow14) as i32) + ((fTemp12 > fSlow12) as i32);
			let mut fTemp14: F64 = fTemp12 - fSlow11;
			let mut fTemp15: F64 = F64::min(1.0, F64::powf(1e+01, -(fSlow16 * F64::max(0.0, if (iTemp13 == 0) as i32 != 0 {0.0} else {if (iTemp13 == 1) as i32 != 0 {fSlow15 * mydsp_faustpower2_f(fSlow10 + fTemp14)} else {fTemp14}}))));
			self.fVec4[(self.IOTA0 & 16383) as usize] = fTemp15;
			let mut fTemp16: F64 = F64::min(fTemp15, self.fVec4[((i32::wrapping_sub(self.IOTA0, 1)) & 16383) as usize]);
			self.fVec5[0] = fTemp16;
			let mut fTemp17: F64 = F64::min(fTemp16, self.fVec5[2]);
			self.fVec6[0] = fTemp17;
			let mut fTemp18: F64 = F64::min(fTemp17, self.fVec6[4]);
			self.fVec7[0] = fTemp18;
			let mut fTemp19: F64 = F64::min(fTemp18, self.fVec7[8]);
			self.fVec8[(self.IOTA0 & 31) as usize] = fTemp19;
			let mut fTemp20: F64 = F64::min(fTemp19, self.fVec8[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize]);
			self.fVec9[(self.IOTA0 & 63) as usize] = fTemp20;
			let mut fTemp21: F64 = F64::min(fTemp20, self.fVec9[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize]);
			self.fVec10[(self.IOTA0 & 127) as usize] = fTemp21;
			let mut fTemp22: F64 = F64::min(fTemp21, self.fVec10[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize]);
			self.fVec11[(self.IOTA0 & 255) as usize] = fTemp22;
			let mut fTemp23: F64 = F64::min(fTemp22, self.fVec11[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize]);
			self.fVec12[(self.IOTA0 & 511) as usize] = fTemp23;
			let mut fTemp24: F64 = F64::min(fTemp23, self.fVec12[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize]);
			self.fVec13[(self.IOTA0 & 1023) as usize] = fTemp24;
			let mut fTemp25: F64 = F64::min(fTemp24, self.fVec13[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize]);
			self.fVec14[(self.IOTA0 & 2047) as usize] = fTemp25;
			self.fVec15[(self.IOTA0 & 4095) as usize] = F64::min(fTemp25, self.fVec14[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			self.fRec5[(self.IOTA0 & 16383) as usize] = F64::max(F64::min(fTemp6, self.fVec4[((i32::wrapping_sub(self.IOTA0, iSlow18)) & 16383) as usize]), F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow20 != 0 {fTemp15} else {1.7976931348623157e+308}, if iSlow21 != 0 {self.fVec5[iSlow20 as usize]} else {1.7976931348623157e+308}), if iSlow22 != 0 {self.fVec6[iSlow23 as usize]} else {1.7976931348623157e+308}), if iSlow24 != 0 {self.fVec7[iSlow25 as usize]} else {1.7976931348623157e+308}), if iSlow26 != 0 {self.fVec8[((i32::wrapping_sub(self.IOTA0, iSlow27)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow28 != 0 {self.fVec9[((i32::wrapping_sub(self.IOTA0, iSlow29)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow30 != 0 {self.fVec10[((i32::wrapping_sub(self.IOTA0, iSlow31)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow32 != 0 {self.fVec11[((i32::wrapping_sub(self.IOTA0, iSlow33)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow34 != 0 {self.fVec12[((i32::wrapping_sub(self.IOTA0, iSlow35)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow36 != 0 {self.fVec13[((i32::wrapping_sub(self.IOTA0, iSlow37)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow38 != 0 {self.fVec14[((i32::wrapping_sub(self.IOTA0, iSlow39)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow40 != 0 {self.fVec15[((i32::wrapping_sub(self.IOTA0, iSlow41)) & 4095) as usize]} else {1.7976931348623157e+308}));
			let mut fTemp26: F64 = self.fRec5[(self.IOTA0 & 16383) as usize];
			let mut fTemp27: F64 = F64::min(fTemp26, fTemp6);
			self.fVec16[0] = fTemp27;
			let mut fTemp28: F64 = F64::min(fTemp27, self.fVec16[2]);
			self.fVec17[0] = fTemp28;
			let mut fTemp29: F64 = F64::min(fTemp28, self.fVec17[4]);
			self.fVec18[0] = fTemp29;
			let mut fTemp30: F64 = F64::min(fTemp29, self.fVec18[8]);
			self.fVec19[(self.IOTA0 & 31) as usize] = fTemp30;
			let mut fTemp31: F64 = F64::min(fTemp30, self.fVec19[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize]);
			self.fVec20[(self.IOTA0 & 63) as usize] = fTemp31;
			let mut fTemp32: F64 = F64::min(fTemp31, self.fVec20[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize]);
			self.fVec21[(self.IOTA0 & 127) as usize] = fTemp32;
			let mut fTemp33: F64 = F64::min(fTemp32, self.fVec21[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize]);
			self.fVec22[(self.IOTA0 & 255) as usize] = fTemp33;
			let mut fTemp34: F64 = F64::min(fTemp33, self.fVec22[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize]);
			self.fVec23[(self.IOTA0 & 511) as usize] = fTemp34;
			let mut fTemp35: F64 = F64::min(fTemp34, self.fVec23[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize]);
			self.fVec24[(self.IOTA0 & 1023) as usize] = fTemp35;
			let mut fTemp36: F64 = F64::min(fTemp35, self.fVec24[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize]);
			self.fVec25[(self.IOTA0 & 2047) as usize] = fTemp36;
			self.fVec26[(self.IOTA0 & 4095) as usize] = F64::min(fTemp36, self.fVec25[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			let mut fTemp37: F64 = F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow8 != 0 {fTemp26} else {1.7976931348623157e+308}, if iSlow42 != 0 {self.fVec16[iSlow8 as usize]} else {1.7976931348623157e+308}), if iSlow43 != 0 {self.fVec17[iSlow44 as usize]} else {1.7976931348623157e+308}), if iSlow45 != 0 {self.fVec18[iSlow46 as usize]} else {1.7976931348623157e+308}), if iSlow47 != 0 {self.fVec19[((i32::wrapping_sub(self.IOTA0, iSlow48)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow49 != 0 {self.fVec20[((i32::wrapping_sub(self.IOTA0, iSlow50)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow51 != 0 {self.fVec21[((i32::wrapping_sub(self.IOTA0, iSlow52)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow53 != 0 {self.fVec22[((i32::wrapping_sub(self.IOTA0, iSlow54)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow55 != 0 {self.fVec23[((i32::wrapping_sub(self.IOTA0, iSlow56)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow57 != 0 {self.fVec24[((i32::wrapping_sub(self.IOTA0, iSlow58)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow59 != 0 {self.fVec25[((i32::wrapping_sub(self.IOTA0, iSlow60)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow61 != 0 {self.fVec26[((i32::wrapping_sub(self.IOTA0, iSlow62)) & 4095) as usize]} else {1.7976931348623157e+308}) - self.fRec4[1];
			self.fVec27[0] = fTemp37;
			let mut iTemp38: i32 = (fTemp37 > 0.0) as i32;
			let mut fTemp44: F64 = if iTemp38 != 0 {fSlow64} else {fSlow63};
			self.fVec28[0] = fTemp44;
			let mut fTemp45: F64 = 2.0 * fTemp44;
			let mut iTemp46: i32 = (fTemp45) as i32;
			let mut iTemp47: i32 = std::cmp::max(0, std::cmp::min(iTemp46, 2));
			let mut iTemp48: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, 98301), 196607));
			let mut fTemp49: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp48, 3)) as usize] };
			let mut fTemp50: F64 = unsafe { ftbl0mydspSIG0[iTemp48 as usize] };
			let mut fTemp51: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp48, 1)) as usize] } - fTemp50;
			let mut fTemp52: F64 = fTemp45 - (iTemp46) as F64;
			let mut fTemp53: F64 = fTemp50 + fTemp52 * fTemp51 + 0.5 * (fTemp49 - (fTemp50 + fTemp52 * (fTemp51 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp48, 4)) as usize] } - fTemp49))));
			let mut fTemp54: F64 = if iTemp38 != 0 {fTemp53} else {1.0 - fTemp53};
			let mut iTemp55: i32 = (fTemp37 < 0.0) as i32;
			let mut fTemp56: F64 = fSlow3 * (iTemp55) as F64 + fSlow65 * (iTemp38) as F64;
			self.fVec29[0] = fTemp56;
			let mut fTemp57: F64 = self.fConst5 / fTemp56;
			let mut fTemp58: F64 = fTemp57 + 0.5;
			let mut fTemp59: F64 = 65535.0 * (1.0 - fTemp58);
			let mut iTemp60: i32 = (fTemp59) as i32;
			let mut iTemp61: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp60, 65535)))), 196607));
			let mut fTemp62: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp61, 3)) as usize] };
			let mut fTemp63: F64 = unsafe { ftbl0mydspSIG0[iTemp61 as usize] };
			let mut fTemp64: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp61, 1)) as usize] } - fTemp63;
			let mut fTemp65: F64 = 65535.0 * fTemp58;
			let mut iTemp66: i32 = (fTemp65) as i32;
			let mut iTemp67: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp66, 65535)))), 196607));
			let mut fTemp68: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, 3), 196607))) as usize] };
			let mut fTemp69: F64 = unsafe { ftbl0mydspSIG0[iTemp67 as usize] };
			let mut fTemp70: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, 1), 196607))) as usize] } - fTemp69;
			let mut fTemp71: F64 = 2.0 * self.fVec28[1];
			let mut iTemp72: i32 = (fTemp71) as i32;
			let mut iTemp73: i32 = std::cmp::max(0, std::cmp::min(iTemp72, 2));
			let mut fTemp74: F64 = 65535.0 * (1.0 - self.fRec3[1]);
			let mut iTemp75: i32 = (fTemp74) as i32;
			let mut iTemp76: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp75, 65535))), iTemp73), 196607));
			let mut fTemp77: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, 3), 196607))) as usize] };
			let mut fTemp78: F64 = unsafe { ftbl0mydspSIG0[iTemp76 as usize] };
			let mut fTemp79: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, 1), 196607))) as usize] } - fTemp78;
			let mut fTemp80: F64 = fTemp71 - (iTemp72) as F64;
			let mut fTemp81: F64 = 65535.0 * self.fRec3[1];
			let mut iTemp82: i32 = (fTemp81) as i32;
			let mut iTemp83: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp73, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp82, 65535)))), 196607));
			let mut fTemp84: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp83, 3), 196607))) as usize] };
			let mut fTemp85: F64 = unsafe { ftbl0mydspSIG0[iTemp83 as usize] };
			let mut fTemp86: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp83, 1), 196607))) as usize] } - fTemp85;
			let mut fTemp87: F64 = self.fRec3[1] + fTemp57;
			let mut fTemp88: F64 = 65535.0 * (1.0 - fTemp87);
			let mut iTemp89: i32 = (fTemp88) as i32;
			let mut iTemp90: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp89, 65535)))), 196607));
			let mut fTemp91: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp90, 3)) as usize] };
			let mut fTemp92: F64 = unsafe { ftbl0mydspSIG0[iTemp90 as usize] };
			let mut fTemp93: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp90, 1)) as usize] } - fTemp92;
			let mut fTemp94: F64 = 65535.0 * fTemp87;
			let mut iTemp95: i32 = (fTemp94) as i32;
			let mut iTemp96: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp95, 65535)))), 196607));
			let mut fTemp97: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp96, 3), 196607))) as usize] };
			let mut fTemp98: F64 = unsafe { ftbl0mydspSIG0[iTemp96 as usize] };
			let mut fTemp99: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp96, 1), 196607))) as usize] } - fTemp98;
			let mut fTemp100: F64 = self.fRec3[1] + self.fConst5 * (1.0 / fTemp56 + 1.0 / self.fVec29[1]);
			let mut fTemp101: F64 = 65535.0 * (1.0 - fTemp100);
			let mut iTemp102: i32 = (fTemp101) as i32;
			let mut iTemp103: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp102, 65535))), iTemp47), 196607));
			let mut fTemp104: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp103, 3)) as usize] };
			let mut fTemp105: F64 = unsafe { ftbl0mydspSIG0[iTemp103 as usize] };
			let mut fTemp106: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp103, 1)) as usize] } - fTemp105;
			let mut fTemp107: F64 = 65535.0 * fTemp100;
			let mut iTemp108: i32 = (fTemp107) as i32;
			let mut iTemp109: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp108, 65535)))), 196607));
			let mut fTemp110: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp109, 3), 196607))) as usize] };
			let mut fTemp111: F64 = unsafe { ftbl0mydspSIG0[iTemp109 as usize] };
			let mut fTemp112: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp109, 1), 196607))) as usize] } - fTemp111;
			let mut fTemp113: F64 = (if iTemp38 != 0 {fTemp111 + fTemp52 * fTemp112 + (fTemp107 - (iTemp108) as F64) * (fTemp110 - (fTemp111 + fTemp52 * (fTemp112 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp109, 4), 196607))) as usize] } - fTemp110))))} else {1.0 - (fTemp105 + fTemp52 * fTemp106 + (fTemp101 - (iTemp102) as F64) * (fTemp104 - (fTemp105 + fTemp52 * (fTemp106 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp103, 4)) as usize] } - fTemp104)))))} - if iTemp38 != 0 {fTemp98 + fTemp52 * fTemp99 + (fTemp94 - (iTemp95) as F64) * (fTemp97 - (fTemp98 + fTemp52 * (fTemp99 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp96, 4), 196607))) as usize] } - fTemp97))))} else {1.0 - (fTemp92 + fTemp52 * fTemp93 + (fTemp88 - (iTemp89) as F64) * (fTemp91 - (fTemp92 + fTemp52 * (fTemp93 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp90, 4), 196607))) as usize] } - fTemp91)))))}) * self.fVec27[1] / (fTemp37 * (1.0 - if iTemp38 != 0 {fTemp85 + fTemp80 * fTemp86 + (fTemp81 - (iTemp82) as F64) * (fTemp84 - (fTemp85 + fTemp80 * (fTemp86 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp83, 4), 196607))) as usize] } - fTemp84))))} else {1.0 - (fTemp78 + fTemp80 * fTemp79 + (fTemp74 - (iTemp75) as F64) * (fTemp77 - (fTemp78 + fTemp80 * (fTemp79 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp76, 4), 196607))) as usize] } - fTemp77)))))}));
			let mut iTemp114: i32 = (fTemp113 > ((if iTemp38 != 0 {fTemp69 + fTemp52 * fTemp70 + (fTemp65 - (iTemp66) as F64) * (fTemp68 - (fTemp69 + fTemp52 * (fTemp70 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp67, 4), 196607))) as usize] } - fTemp68))))} else {1.0 - (fTemp63 + fTemp52 * fTemp64 + (fTemp59 - (iTemp60) as F64) * (fTemp62 - (fTemp63 + fTemp52 * (fTemp64 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp61, 4)) as usize] } - fTemp62)))))} - fTemp54) / (1.0 - fTemp54))) as i32;
			let mut fTemp115: F64 = if iTemp114 != 0 {1.0} else {0.5};
			let mut fTemp116: F64 = if iTemp114 != 0 {0.5} else {0.0};
			let mut fTemp117: F64 = fTemp116 + fTemp115;
			let mut fTemp118: F64 = 0.5 * fTemp117;
			let mut fTemp119: F64 = 65535.0 * (1.0 - fTemp118);
			let mut iTemp120: i32 = (fTemp119) as i32;
			let mut iTemp121: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp120, 65535)))), 196607));
			let mut fTemp122: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp121, 3)) as usize] };
			let mut fTemp123: F64 = unsafe { ftbl0mydspSIG0[iTemp121 as usize] };
			let mut fTemp124: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp121, 1)) as usize] } - fTemp123;
			let mut fTemp125: F64 = 32767.5 * fTemp117;
			let mut iTemp126: i32 = (fTemp125) as i32;
			let mut iTemp127: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp126, 65535)))), 196607));
			let mut fTemp128: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp127, 3)) as usize] };
			let mut fTemp129: F64 = unsafe { ftbl0mydspSIG0[iTemp127 as usize] };
			let mut fTemp130: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp127, 1)) as usize] } - fTemp129;
			let mut fTemp131: F64 = if iTemp38 != 0 {fTemp129 + fTemp52 * fTemp130 + (fTemp125 - (iTemp126) as F64) * (fTemp128 - (fTemp129 + fTemp52 * (fTemp130 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp127, 4)) as usize] } - fTemp128))))} else {1.0 - (fTemp123 + fTemp52 * fTemp124 + (fTemp119 - (iTemp120) as F64) * (fTemp122 - (fTemp123 + fTemp52 * (fTemp124 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp121, 4)) as usize] } - fTemp122)))))};
			let mut fTemp132: F64 = fTemp57 + fTemp118;
			let mut fTemp133: F64 = 65535.0 * (1.0 - fTemp132);
			let mut iTemp134: i32 = (fTemp133) as i32;
			let mut iTemp135: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp134, 65535)))), 196607));
			let mut fTemp136: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp135, 3)) as usize] };
			let mut fTemp137: F64 = unsafe { ftbl0mydspSIG0[iTemp135 as usize] };
			let mut fTemp138: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp135, 1)) as usize] } - fTemp137;
			let mut fTemp139: F64 = 65535.0 * fTemp132;
			let mut iTemp140: i32 = (fTemp139) as i32;
			let mut iTemp141: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp140, 65535)))), 196607));
			let mut fTemp142: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp141, 3), 196607))) as usize] };
			let mut fTemp143: F64 = unsafe { ftbl0mydspSIG0[iTemp141 as usize] };
			let mut fTemp144: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp141, 1), 196607))) as usize] } - fTemp143;
			let mut iTemp145: i32 = (fTemp113 > ((if iTemp38 != 0 {fTemp143 + fTemp52 * fTemp144 + (fTemp139 - (iTemp140) as F64) * (fTemp142 - (fTemp143 + fTemp52 * (fTemp144 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp141, 4), 196607))) as usize] } - fTemp142))))} else {1.0 - (fTemp137 + fTemp52 * fTemp138 + (fTemp133 - (iTemp134) as F64) * (fTemp136 - (fTemp137 + fTemp52 * (fTemp138 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp135, 4)) as usize] } - fTemp136)))))} - fTemp131) / (1.0 - fTemp131))) as i32;
			let mut fTemp146: F64 = if iTemp145 != 0 {fTemp115} else {fTemp118};
			let mut fTemp147: F64 = if iTemp145 != 0 {fTemp118} else {fTemp116};
			let mut fTemp148: F64 = fTemp147 + fTemp146;
			let mut fTemp149: F64 = 0.5 * fTemp148;
			let mut fTemp150: F64 = 65535.0 * (1.0 - fTemp149);
			let mut iTemp151: i32 = (fTemp150) as i32;
			let mut iTemp152: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp151, 65535)))), 196607));
			let mut fTemp153: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp152, 3)) as usize] };
			let mut fTemp154: F64 = unsafe { ftbl0mydspSIG0[iTemp152 as usize] };
			let mut fTemp155: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp152, 1)) as usize] } - fTemp154;
			let mut fTemp156: F64 = 32767.5 * fTemp148;
			let mut iTemp157: i32 = (fTemp156) as i32;
			let mut iTemp158: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp157, 65535)))), 196607));
			let mut fTemp159: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp158, 3)) as usize] };
			let mut fTemp160: F64 = unsafe { ftbl0mydspSIG0[iTemp158 as usize] };
			let mut fTemp161: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp158, 1)) as usize] } - fTemp160;
			let mut fTemp162: F64 = if iTemp38 != 0 {fTemp160 + fTemp52 * fTemp161 + (fTemp156 - (iTemp157) as F64) * (fTemp159 - (fTemp160 + fTemp52 * (fTemp161 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp158, 4)) as usize] } - fTemp159))))} else {1.0 - (fTemp154 + fTemp52 * fTemp155 + (fTemp150 - (iTemp151) as F64) * (fTemp153 - (fTemp154 + fTemp52 * (fTemp155 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp152, 4)) as usize] } - fTemp153)))))};
			let mut fTemp163: F64 = fTemp57 + fTemp149;
			let mut fTemp164: F64 = 65535.0 * (1.0 - fTemp163);
			let mut iTemp165: i32 = (fTemp164) as i32;
			let mut iTemp166: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp165, 65535)))), 196607));
			let mut fTemp167: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp166, 3)) as usize] };
			let mut fTemp168: F64 = unsafe { ftbl0mydspSIG0[iTemp166 as usize] };
			let mut fTemp169: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp166, 1)) as usize] } - fTemp168;
			let mut fTemp170: F64 = 65535.0 * fTemp163;
			let mut iTemp171: i32 = (fTemp170) as i32;
			let mut iTemp172: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp171, 65535)))), 196607));
			let mut fTemp173: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp172, 3), 196607))) as usize] };
			let mut fTemp174: F64 = unsafe { ftbl0mydspSIG0[iTemp172 as usize] };
			let mut fTemp175: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp172, 1), 196607))) as usize] } - fTemp174;
			let mut iTemp176: i32 = (fTemp113 > ((if iTemp38 != 0 {fTemp174 + fTemp52 * fTemp175 + (fTemp170 - (iTemp171) as F64) * (fTemp173 - (fTemp174 + fTemp52 * (fTemp175 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp172, 4), 196607))) as usize] } - fTemp173))))} else {1.0 - (fTemp168 + fTemp52 * fTemp169 + (fTemp164 - (iTemp165) as F64) * (fTemp167 - (fTemp168 + fTemp52 * (fTemp169 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp166, 4)) as usize] } - fTemp167)))))} - fTemp162) / (1.0 - fTemp162))) as i32;
			let mut fTemp177: F64 = if iTemp176 != 0 {fTemp146} else {fTemp149};
			let mut fTemp178: F64 = if iTemp176 != 0 {fTemp149} else {fTemp147};
			let mut fTemp179: F64 = fTemp178 + fTemp177;
			let mut fTemp180: F64 = 0.5 * fTemp179;
			let mut fTemp181: F64 = 65535.0 * (1.0 - fTemp180);
			let mut iTemp182: i32 = (fTemp181) as i32;
			let mut iTemp183: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp182, 65535)))), 196607));
			let mut fTemp184: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp183, 3)) as usize] };
			let mut fTemp185: F64 = unsafe { ftbl0mydspSIG0[iTemp183 as usize] };
			let mut fTemp186: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp183, 1)) as usize] } - fTemp185;
			let mut fTemp187: F64 = 32767.5 * fTemp179;
			let mut iTemp188: i32 = (fTemp187) as i32;
			let mut iTemp189: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp188, 65535)))), 196607));
			let mut fTemp190: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp189, 3)) as usize] };
			let mut fTemp191: F64 = unsafe { ftbl0mydspSIG0[iTemp189 as usize] };
			let mut fTemp192: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp189, 1)) as usize] } - fTemp191;
			let mut fTemp193: F64 = if iTemp38 != 0 {fTemp191 + fTemp52 * fTemp192 + (fTemp187 - (iTemp188) as F64) * (fTemp190 - (fTemp191 + fTemp52 * (fTemp192 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp189, 4)) as usize] } - fTemp190))))} else {1.0 - (fTemp185 + fTemp52 * fTemp186 + (fTemp181 - (iTemp182) as F64) * (fTemp184 - (fTemp185 + fTemp52 * (fTemp186 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp183, 4)) as usize] } - fTemp184)))))};
			let mut fTemp194: F64 = fTemp57 + fTemp180;
			let mut fTemp195: F64 = 65535.0 * (1.0 - fTemp194);
			let mut iTemp196: i32 = (fTemp195) as i32;
			let mut iTemp197: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp196, 65535)))), 196607));
			let mut fTemp198: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp197, 3)) as usize] };
			let mut fTemp199: F64 = unsafe { ftbl0mydspSIG0[iTemp197 as usize] };
			let mut fTemp200: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp197, 1)) as usize] } - fTemp199;
			let mut fTemp201: F64 = 65535.0 * fTemp194;
			let mut iTemp202: i32 = (fTemp201) as i32;
			let mut iTemp203: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp202, 65535)))), 196607));
			let mut fTemp204: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp203, 3), 196607))) as usize] };
			let mut fTemp205: F64 = unsafe { ftbl0mydspSIG0[iTemp203 as usize] };
			let mut fTemp206: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp203, 1), 196607))) as usize] } - fTemp205;
			let mut iTemp207: i32 = (fTemp113 > ((if iTemp38 != 0 {fTemp205 + fTemp52 * fTemp206 + (fTemp201 - (iTemp202) as F64) * (fTemp204 - (fTemp205 + fTemp52 * (fTemp206 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp203, 4), 196607))) as usize] } - fTemp204))))} else {1.0 - (fTemp199 + fTemp52 * fTemp200 + (fTemp195 - (iTemp196) as F64) * (fTemp198 - (fTemp199 + fTemp52 * (fTemp200 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp197, 4)) as usize] } - fTemp198)))))} - fTemp193) / (1.0 - fTemp193))) as i32;
			let mut fTemp208: F64 = if iTemp207 != 0 {fTemp177} else {fTemp180};
			let mut fTemp209: F64 = if iTemp207 != 0 {fTemp180} else {fTemp178};
			let mut fTemp210: F64 = fTemp209 + fTemp208;
			let mut fTemp211: F64 = 0.5 * fTemp210;
			let mut fTemp212: F64 = 65535.0 * (1.0 - fTemp211);
			let mut iTemp213: i32 = (fTemp212) as i32;
			let mut iTemp214: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp213, 65535)))), 196607));
			let mut fTemp215: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp214, 3)) as usize] };
			let mut fTemp216: F64 = unsafe { ftbl0mydspSIG0[iTemp214 as usize] };
			let mut fTemp217: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp214, 1)) as usize] } - fTemp216;
			let mut fTemp218: F64 = 32767.5 * fTemp210;
			let mut iTemp219: i32 = (fTemp218) as i32;
			let mut iTemp220: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp219, 65535)))), 196607));
			let mut fTemp221: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp220, 3)) as usize] };
			let mut fTemp222: F64 = unsafe { ftbl0mydspSIG0[iTemp220 as usize] };
			let mut fTemp223: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp220, 1)) as usize] } - fTemp222;
			let mut fTemp224: F64 = if iTemp38 != 0 {fTemp222 + fTemp52 * fTemp223 + (fTemp218 - (iTemp219) as F64) * (fTemp221 - (fTemp222 + fTemp52 * (fTemp223 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp220, 4)) as usize] } - fTemp221))))} else {1.0 - (fTemp216 + fTemp52 * fTemp217 + (fTemp212 - (iTemp213) as F64) * (fTemp215 - (fTemp216 + fTemp52 * (fTemp217 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp214, 4)) as usize] } - fTemp215)))))};
			let mut fTemp225: F64 = fTemp57 + fTemp211;
			let mut fTemp226: F64 = 65535.0 * (1.0 - fTemp225);
			let mut iTemp227: i32 = (fTemp226) as i32;
			let mut iTemp228: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp227, 65535)))), 196607));
			let mut fTemp229: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp228, 3)) as usize] };
			let mut fTemp230: F64 = unsafe { ftbl0mydspSIG0[iTemp228 as usize] };
			let mut fTemp231: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp228, 1)) as usize] } - fTemp230;
			let mut fTemp232: F64 = 65535.0 * fTemp225;
			let mut iTemp233: i32 = (fTemp232) as i32;
			let mut iTemp234: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp233, 65535)))), 196607));
			let mut fTemp235: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp234, 3), 196607))) as usize] };
			let mut fTemp236: F64 = unsafe { ftbl0mydspSIG0[iTemp234 as usize] };
			let mut fTemp237: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp234, 1), 196607))) as usize] } - fTemp236;
			let mut iTemp238: i32 = (fTemp113 > ((if iTemp38 != 0 {fTemp236 + fTemp52 * fTemp237 + (fTemp232 - (iTemp233) as F64) * (fTemp235 - (fTemp236 + fTemp52 * (fTemp237 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp234, 4), 196607))) as usize] } - fTemp235))))} else {1.0 - (fTemp230 + fTemp52 * fTemp231 + (fTemp226 - (iTemp227) as F64) * (fTemp229 - (fTemp230 + fTemp52 * (fTemp231 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp228, 4)) as usize] } - fTemp229)))))} - fTemp224) / (1.0 - fTemp224))) as i32;
			let mut fTemp239: F64 = if iTemp238 != 0 {fTemp208} else {fTemp211};
			let mut fTemp240: F64 = if iTemp238 != 0 {fTemp211} else {fTemp209};
			let mut fTemp241: F64 = fTemp240 + fTemp239;
			let mut fTemp242: F64 = 0.5 * fTemp241;
			let mut fTemp243: F64 = 65535.0 * (1.0 - fTemp242);
			let mut iTemp244: i32 = (fTemp243) as i32;
			let mut iTemp245: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp244, 65535)))), 196607));
			let mut fTemp246: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp245, 3)) as usize] };
			let mut fTemp247: F64 = unsafe { ftbl0mydspSIG0[iTemp245 as usize] };
			let mut fTemp248: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp245, 1)) as usize] } - fTemp247;
			let mut fTemp249: F64 = 32767.5 * fTemp241;
			let mut iTemp250: i32 = (fTemp249) as i32;
			let mut iTemp251: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp250, 65535)))), 196607));
			let mut fTemp252: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp251, 3)) as usize] };
			let mut fTemp253: F64 = unsafe { ftbl0mydspSIG0[iTemp251 as usize] };
			let mut fTemp254: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp251, 1)) as usize] } - fTemp253;
			let mut fTemp255: F64 = if iTemp38 != 0 {fTemp253 + fTemp52 * fTemp254 + (fTemp249 - (iTemp250) as F64) * (fTemp252 - (fTemp253 + fTemp52 * (fTemp254 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp251, 4)) as usize] } - fTemp252))))} else {1.0 - (fTemp247 + fTemp52 * fTemp248 + (fTemp243 - (iTemp244) as F64) * (fTemp246 - (fTemp247 + fTemp52 * (fTemp248 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp245, 4)) as usize] } - fTemp246)))))};
			let mut fTemp256: F64 = fTemp57 + fTemp242;
			let mut fTemp257: F64 = 65535.0 * (1.0 - fTemp256);
			let mut iTemp258: i32 = (fTemp257) as i32;
			let mut iTemp259: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp258, 65535)))), 196607));
			let mut fTemp260: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp259, 3)) as usize] };
			let mut fTemp261: F64 = unsafe { ftbl0mydspSIG0[iTemp259 as usize] };
			let mut fTemp262: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp259, 1)) as usize] } - fTemp261;
			let mut fTemp263: F64 = 65535.0 * fTemp256;
			let mut iTemp264: i32 = (fTemp263) as i32;
			let mut iTemp265: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp264, 65535)))), 196607));
			let mut fTemp266: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp265, 3), 196607))) as usize] };
			let mut fTemp267: F64 = unsafe { ftbl0mydspSIG0[iTemp265 as usize] };
			let mut fTemp268: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp265, 1), 196607))) as usize] } - fTemp267;
			let mut iTemp269: i32 = (fTemp113 > ((if iTemp38 != 0 {fTemp267 + fTemp52 * fTemp268 + (fTemp263 - (iTemp264) as F64) * (fTemp266 - (fTemp267 + fTemp52 * (fTemp268 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp265, 4), 196607))) as usize] } - fTemp266))))} else {1.0 - (fTemp261 + fTemp52 * fTemp262 + (fTemp257 - (iTemp258) as F64) * (fTemp260 - (fTemp261 + fTemp52 * (fTemp262 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp259, 4)) as usize] } - fTemp260)))))} - fTemp255) / (1.0 - fTemp255))) as i32;
			let mut fTemp270: F64 = if iTemp269 != 0 {fTemp239} else {fTemp242};
			let mut fTemp271: F64 = if iTemp269 != 0 {fTemp242} else {fTemp240};
			let mut fTemp272: F64 = fTemp271 + fTemp270;
			let mut fTemp273: F64 = 0.5 * fTemp272;
			let mut fTemp274: F64 = 65535.0 * (1.0 - fTemp273);
			let mut iTemp275: i32 = (fTemp274) as i32;
			let mut iTemp276: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp275, 65535)))), 196607));
			let mut fTemp277: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp276, 3)) as usize] };
			let mut fTemp278: F64 = unsafe { ftbl0mydspSIG0[iTemp276 as usize] };
			let mut fTemp279: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp276, 1)) as usize] } - fTemp278;
			let mut fTemp280: F64 = 32767.5 * fTemp272;
			let mut iTemp281: i32 = (fTemp280) as i32;
			let mut iTemp282: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp281, 65535)))), 196607));
			let mut fTemp283: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp282, 3)) as usize] };
			let mut fTemp284: F64 = unsafe { ftbl0mydspSIG0[iTemp282 as usize] };
			let mut fTemp285: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp282, 1)) as usize] } - fTemp284;
			let mut fTemp286: F64 = if iTemp38 != 0 {fTemp284 + fTemp52 * fTemp285 + (fTemp280 - (iTemp281) as F64) * (fTemp283 - (fTemp284 + fTemp52 * (fTemp285 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp282, 4)) as usize] } - fTemp283))))} else {1.0 - (fTemp278 + fTemp52 * fTemp279 + (fTemp274 - (iTemp275) as F64) * (fTemp277 - (fTemp278 + fTemp52 * (fTemp279 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp276, 4)) as usize] } - fTemp277)))))};
			let mut fTemp287: F64 = fTemp57 + fTemp273;
			let mut fTemp288: F64 = 65535.0 * (1.0 - fTemp287);
			let mut iTemp289: i32 = (fTemp288) as i32;
			let mut iTemp290: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp289, 65535)))), 196607));
			let mut fTemp291: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp290, 3)) as usize] };
			let mut fTemp292: F64 = unsafe { ftbl0mydspSIG0[iTemp290 as usize] };
			let mut fTemp293: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp290, 1)) as usize] } - fTemp292;
			let mut fTemp294: F64 = 65535.0 * fTemp287;
			let mut iTemp295: i32 = (fTemp294) as i32;
			let mut iTemp296: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp295, 65535)))), 196607));
			let mut fTemp297: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp296, 3), 196607))) as usize] };
			let mut fTemp298: F64 = unsafe { ftbl0mydspSIG0[iTemp296 as usize] };
			let mut fTemp299: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp296, 1), 196607))) as usize] } - fTemp298;
			let mut iTemp300: i32 = (fTemp113 > ((if iTemp38 != 0 {fTemp298 + fTemp52 * fTemp299 + (fTemp294 - (iTemp295) as F64) * (fTemp297 - (fTemp298 + fTemp52 * (fTemp299 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp296, 4), 196607))) as usize] } - fTemp297))))} else {1.0 - (fTemp292 + fTemp52 * fTemp293 + (fTemp288 - (iTemp289) as F64) * (fTemp291 - (fTemp292 + fTemp52 * (fTemp293 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp290, 4)) as usize] } - fTemp291)))))} - fTemp286) / (1.0 - fTemp286))) as i32;
			let mut fTemp301: F64 = if iTemp300 != 0 {fTemp270} else {fTemp273};
			let mut fTemp302: F64 = if iTemp300 != 0 {fTemp273} else {fTemp271};
			let mut fTemp303: F64 = fTemp302 + fTemp301;
			let mut fTemp304: F64 = 0.5 * fTemp303;
			let mut fTemp305: F64 = 65535.0 * (1.0 - fTemp304);
			let mut iTemp306: i32 = (fTemp305) as i32;
			let mut iTemp307: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp306, 65535)))), 196607));
			let mut fTemp308: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp307, 3)) as usize] };
			let mut fTemp309: F64 = unsafe { ftbl0mydspSIG0[iTemp307 as usize] };
			let mut fTemp310: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp307, 1)) as usize] } - fTemp309;
			let mut fTemp311: F64 = 32767.5 * fTemp303;
			let mut iTemp312: i32 = (fTemp311) as i32;
			let mut iTemp313: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp312, 65535)))), 196607));
			let mut fTemp314: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp313, 3)) as usize] };
			let mut fTemp315: F64 = unsafe { ftbl0mydspSIG0[iTemp313 as usize] };
			let mut fTemp316: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp313, 1)) as usize] } - fTemp315;
			let mut fTemp317: F64 = if iTemp38 != 0 {fTemp315 + fTemp52 * fTemp316 + (fTemp311 - (iTemp312) as F64) * (fTemp314 - (fTemp315 + fTemp52 * (fTemp316 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp313, 4)) as usize] } - fTemp314))))} else {1.0 - (fTemp309 + fTemp52 * fTemp310 + (fTemp305 - (iTemp306) as F64) * (fTemp308 - (fTemp309 + fTemp52 * (fTemp310 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp307, 4)) as usize] } - fTemp308)))))};
			let mut fTemp318: F64 = fTemp57 + fTemp304;
			let mut fTemp319: F64 = 65535.0 * (1.0 - fTemp318);
			let mut iTemp320: i32 = (fTemp319) as i32;
			let mut iTemp321: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp320, 65535)))), 196607));
			let mut fTemp322: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp321, 3)) as usize] };
			let mut fTemp323: F64 = unsafe { ftbl0mydspSIG0[iTemp321 as usize] };
			let mut fTemp324: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp321, 1)) as usize] } - fTemp323;
			let mut fTemp325: F64 = 65535.0 * fTemp318;
			let mut iTemp326: i32 = (fTemp325) as i32;
			let mut iTemp327: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp326, 65535)))), 196607));
			let mut fTemp328: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp327, 3), 196607))) as usize] };
			let mut fTemp329: F64 = unsafe { ftbl0mydspSIG0[iTemp327 as usize] };
			let mut fTemp330: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp327, 1), 196607))) as usize] } - fTemp329;
			let mut iTemp331: i32 = (fTemp113 > ((if iTemp38 != 0 {fTemp329 + fTemp52 * fTemp330 + (fTemp325 - (iTemp326) as F64) * (fTemp328 - (fTemp329 + fTemp52 * (fTemp330 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp327, 4), 196607))) as usize] } - fTemp328))))} else {1.0 - (fTemp323 + fTemp52 * fTemp324 + (fTemp319 - (iTemp320) as F64) * (fTemp322 - (fTemp323 + fTemp52 * (fTemp324 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp321, 4)) as usize] } - fTemp322)))))} - fTemp317) / (1.0 - fTemp317))) as i32;
			let mut fTemp332: F64 = if iTemp331 != 0 {fTemp301} else {fTemp304};
			let mut fTemp333: F64 = if iTemp331 != 0 {fTemp304} else {fTemp302};
			let mut fTemp334: F64 = fTemp333 + fTemp332;
			let mut fTemp335: F64 = 0.5 * fTemp334;
			let mut fTemp336: F64 = 65535.0 * (1.0 - fTemp335);
			let mut iTemp337: i32 = (fTemp336) as i32;
			let mut iTemp338: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp337, 65535)))), 196607));
			let mut fTemp339: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp338, 3)) as usize] };
			let mut fTemp340: F64 = unsafe { ftbl0mydspSIG0[iTemp338 as usize] };
			let mut fTemp341: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp338, 1)) as usize] } - fTemp340;
			let mut fTemp342: F64 = 32767.5 * fTemp334;
			let mut iTemp343: i32 = (fTemp342) as i32;
			let mut iTemp344: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp343, 65535)))), 196607));
			let mut fTemp345: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp344, 3)) as usize] };
			let mut fTemp346: F64 = unsafe { ftbl0mydspSIG0[iTemp344 as usize] };
			let mut fTemp347: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp344, 1)) as usize] } - fTemp346;
			let mut fTemp348: F64 = if iTemp38 != 0 {fTemp346 + fTemp52 * fTemp347 + (fTemp342 - (iTemp343) as F64) * (fTemp345 - (fTemp346 + fTemp52 * (fTemp347 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp344, 4)) as usize] } - fTemp345))))} else {1.0 - (fTemp340 + fTemp52 * fTemp341 + (fTemp336 - (iTemp337) as F64) * (fTemp339 - (fTemp340 + fTemp52 * (fTemp341 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp338, 4)) as usize] } - fTemp339)))))};
			let mut fTemp349: F64 = fTemp57 + fTemp335;
			let mut fTemp350: F64 = 65535.0 * (1.0 - fTemp349);
			let mut iTemp351: i32 = (fTemp350) as i32;
			let mut iTemp352: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp351, 65535)))), 196607));
			let mut fTemp353: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp352, 3)) as usize] };
			let mut fTemp354: F64 = unsafe { ftbl0mydspSIG0[iTemp352 as usize] };
			let mut fTemp355: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp352, 1)) as usize] } - fTemp354;
			let mut fTemp356: F64 = 65535.0 * fTemp349;
			let mut iTemp357: i32 = (fTemp356) as i32;
			let mut iTemp358: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp357, 65535)))), 196607));
			let mut fTemp359: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp358, 3), 196607))) as usize] };
			let mut fTemp360: F64 = unsafe { ftbl0mydspSIG0[iTemp358 as usize] };
			let mut fTemp361: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp358, 1), 196607))) as usize] } - fTemp360;
			let mut iTemp362: i32 = (fTemp113 > ((if iTemp38 != 0 {fTemp360 + fTemp52 * fTemp361 + (fTemp356 - (iTemp357) as F64) * (fTemp359 - (fTemp360 + fTemp52 * (fTemp361 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp358, 4), 196607))) as usize] } - fTemp359))))} else {1.0 - (fTemp354 + fTemp52 * fTemp355 + (fTemp350 - (iTemp351) as F64) * (fTemp353 - (fTemp354 + fTemp52 * (fTemp355 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp352, 4)) as usize] } - fTemp353)))))} - fTemp348) / (1.0 - fTemp348))) as i32;
			let mut fTemp363: F64 = if iTemp362 != 0 {fTemp332} else {fTemp335};
			let mut fTemp364: F64 = if iTemp362 != 0 {fTemp335} else {fTemp333};
			let mut fTemp365: F64 = fTemp364 + fTemp363;
			let mut fTemp366: F64 = 0.5 * fTemp365;
			let mut fTemp367: F64 = 65535.0 * (1.0 - fTemp366);
			let mut iTemp368: i32 = (fTemp367) as i32;
			let mut iTemp369: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp368, 65535)))), 196607));
			let mut fTemp370: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp369, 3)) as usize] };
			let mut fTemp371: F64 = unsafe { ftbl0mydspSIG0[iTemp369 as usize] };
			let mut fTemp372: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp369, 1)) as usize] } - fTemp371;
			let mut fTemp373: F64 = 32767.5 * fTemp365;
			let mut iTemp374: i32 = (fTemp373) as i32;
			let mut iTemp375: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp374, 65535)))), 196607));
			let mut fTemp376: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp375, 3)) as usize] };
			let mut fTemp377: F64 = unsafe { ftbl0mydspSIG0[iTemp375 as usize] };
			let mut fTemp378: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp375, 1)) as usize] } - fTemp377;
			let mut fTemp379: F64 = if iTemp38 != 0 {fTemp377 + fTemp52 * fTemp378 + (fTemp373 - (iTemp374) as F64) * (fTemp376 - (fTemp377 + fTemp52 * (fTemp378 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp375, 4)) as usize] } - fTemp376))))} else {1.0 - (fTemp371 + fTemp52 * fTemp372 + (fTemp367 - (iTemp368) as F64) * (fTemp370 - (fTemp371 + fTemp52 * (fTemp372 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp369, 4)) as usize] } - fTemp370)))))};
			let mut fTemp380: F64 = fTemp57 + fTemp366;
			let mut fTemp381: F64 = 65535.0 * (1.0 - fTemp380);
			let mut iTemp382: i32 = (fTemp381) as i32;
			let mut iTemp383: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp382, 65535)))), 196607));
			let mut fTemp384: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp383, 3)) as usize] };
			let mut fTemp385: F64 = unsafe { ftbl0mydspSIG0[iTemp383 as usize] };
			let mut fTemp386: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp383, 1)) as usize] } - fTemp385;
			let mut fTemp387: F64 = 65535.0 * fTemp380;
			let mut iTemp388: i32 = (fTemp387) as i32;
			let mut iTemp389: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp388, 65535)))), 196607));
			let mut fTemp390: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp389, 3), 196607))) as usize] };
			let mut fTemp391: F64 = unsafe { ftbl0mydspSIG0[iTemp389 as usize] };
			let mut fTemp392: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp389, 1), 196607))) as usize] } - fTemp391;
			let mut iTemp393: i32 = (fTemp113 > ((if iTemp38 != 0 {fTemp391 + fTemp52 * fTemp392 + (fTemp387 - (iTemp388) as F64) * (fTemp390 - (fTemp391 + fTemp52 * (fTemp392 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp389, 4), 196607))) as usize] } - fTemp390))))} else {1.0 - (fTemp385 + fTemp52 * fTemp386 + (fTemp381 - (iTemp382) as F64) * (fTemp384 - (fTemp385 + fTemp52 * (fTemp386 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp383, 4)) as usize] } - fTemp384)))))} - fTemp379) / (1.0 - fTemp379))) as i32;
			let mut fTemp394: F64 = if iTemp393 != 0 {fTemp363} else {fTemp366};
			let mut fTemp395: F64 = if iTemp393 != 0 {fTemp366} else {fTemp364};
			let mut fTemp396: F64 = fTemp395 + fTemp394;
			let mut fTemp397: F64 = 0.5 * fTemp396;
			let mut fTemp398: F64 = 65535.0 * (1.0 - fTemp397);
			let mut iTemp399: i32 = (fTemp398) as i32;
			let mut iTemp400: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp399, 65535)))), 196607));
			let mut fTemp401: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp400, 3)) as usize] };
			let mut fTemp402: F64 = unsafe { ftbl0mydspSIG0[iTemp400 as usize] };
			let mut fTemp403: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp400, 1)) as usize] } - fTemp402;
			let mut fTemp404: F64 = 32767.5 * fTemp396;
			let mut iTemp405: i32 = (fTemp404) as i32;
			let mut iTemp406: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp405, 65535)))), 196607));
			let mut fTemp407: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp406, 3)) as usize] };
			let mut fTemp408: F64 = unsafe { ftbl0mydspSIG0[iTemp406 as usize] };
			let mut fTemp409: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp406, 1)) as usize] } - fTemp408;
			let mut fTemp410: F64 = if iTemp38 != 0 {fTemp408 + fTemp52 * fTemp409 + (fTemp404 - (iTemp405) as F64) * (fTemp407 - (fTemp408 + fTemp52 * (fTemp409 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp406, 4)) as usize] } - fTemp407))))} else {1.0 - (fTemp402 + fTemp52 * fTemp403 + (fTemp398 - (iTemp399) as F64) * (fTemp401 - (fTemp402 + fTemp52 * (fTemp403 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp400, 4)) as usize] } - fTemp401)))))};
			let mut fTemp411: F64 = fTemp57 + fTemp397;
			let mut fTemp412: F64 = 65535.0 * (1.0 - fTemp411);
			let mut iTemp413: i32 = (fTemp412) as i32;
			let mut iTemp414: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp413, 65535)))), 196607));
			let mut fTemp415: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp414, 3)) as usize] };
			let mut fTemp416: F64 = unsafe { ftbl0mydspSIG0[iTemp414 as usize] };
			let mut fTemp417: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp414, 1)) as usize] } - fTemp416;
			let mut fTemp418: F64 = 65535.0 * fTemp411;
			let mut iTemp419: i32 = (fTemp418) as i32;
			let mut iTemp420: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp419, 65535)))), 196607));
			let mut fTemp421: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp420, 3), 196607))) as usize] };
			let mut fTemp422: F64 = unsafe { ftbl0mydspSIG0[iTemp420 as usize] };
			let mut fTemp423: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp420, 1), 196607))) as usize] } - fTemp422;
			let mut iTemp424: i32 = (fTemp113 > ((if iTemp38 != 0 {fTemp422 + fTemp52 * fTemp423 + (fTemp418 - (iTemp419) as F64) * (fTemp421 - (fTemp422 + fTemp52 * (fTemp423 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp420, 4), 196607))) as usize] } - fTemp421))))} else {1.0 - (fTemp416 + fTemp52 * fTemp417 + (fTemp412 - (iTemp413) as F64) * (fTemp415 - (fTemp416 + fTemp52 * (fTemp417 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp414, 4)) as usize] } - fTemp415)))))} - fTemp410) / (1.0 - fTemp410))) as i32;
			let mut fTemp425: F64 = if iTemp424 != 0 {fTemp394} else {fTemp397};
			let mut fTemp426: F64 = if iTemp424 != 0 {fTemp397} else {fTemp395};
			let mut fTemp427: F64 = fTemp426 + fTemp425;
			let mut fTemp428: F64 = 0.5 * fTemp427;
			let mut fTemp429: F64 = 65535.0 * (1.0 - fTemp428);
			let mut iTemp430: i32 = (fTemp429) as i32;
			let mut iTemp431: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp430, 65535)))), 196607));
			let mut fTemp432: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp431, 3)) as usize] };
			let mut fTemp433: F64 = unsafe { ftbl0mydspSIG0[iTemp431 as usize] };
			let mut fTemp434: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp431, 1)) as usize] } - fTemp433;
			let mut fTemp435: F64 = 32767.5 * fTemp427;
			let mut iTemp436: i32 = (fTemp435) as i32;
			let mut iTemp437: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp436, 65535)))), 196607));
			let mut fTemp438: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp437, 3)) as usize] };
			let mut fTemp439: F64 = unsafe { ftbl0mydspSIG0[iTemp437 as usize] };
			let mut fTemp440: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp437, 1)) as usize] } - fTemp439;
			let mut fTemp441: F64 = if iTemp38 != 0 {fTemp439 + fTemp52 * fTemp440 + (fTemp435 - (iTemp436) as F64) * (fTemp438 - (fTemp439 + fTemp52 * (fTemp440 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp437, 4)) as usize] } - fTemp438))))} else {1.0 - (fTemp433 + fTemp52 * fTemp434 + (fTemp429 - (iTemp430) as F64) * (fTemp432 - (fTemp433 + fTemp52 * (fTemp434 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp431, 4)) as usize] } - fTemp432)))))};
			let mut fTemp442: F64 = fTemp57 + fTemp428;
			let mut fTemp443: F64 = 65535.0 * (1.0 - fTemp442);
			let mut iTemp444: i32 = (fTemp443) as i32;
			let mut iTemp445: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp444, 65535)))), 196607));
			let mut fTemp446: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp445, 3)) as usize] };
			let mut fTemp447: F64 = unsafe { ftbl0mydspSIG0[iTemp445 as usize] };
			let mut fTemp448: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp445, 1)) as usize] } - fTemp447;
			let mut fTemp449: F64 = 65535.0 * fTemp442;
			let mut iTemp450: i32 = (fTemp449) as i32;
			let mut iTemp451: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp450, 65535)))), 196607));
			let mut fTemp452: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp451, 3), 196607))) as usize] };
			let mut fTemp453: F64 = unsafe { ftbl0mydspSIG0[iTemp451 as usize] };
			let mut fTemp454: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp451, 1), 196607))) as usize] } - fTemp453;
			let mut iTemp455: i32 = (fTemp113 > ((if iTemp38 != 0 {fTemp453 + fTemp52 * fTemp454 + (fTemp449 - (iTemp450) as F64) * (fTemp452 - (fTemp453 + fTemp52 * (fTemp454 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp451, 4), 196607))) as usize] } - fTemp452))))} else {1.0 - (fTemp447 + fTemp52 * fTemp448 + (fTemp443 - (iTemp444) as F64) * (fTemp446 - (fTemp447 + fTemp52 * (fTemp448 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp445, 4)) as usize] } - fTemp446)))))} - fTemp441) / (1.0 - fTemp441))) as i32;
			let mut fTemp456: F64 = if iTemp455 != 0 {fTemp425} else {fTemp428};
			let mut fTemp457: F64 = if iTemp455 != 0 {fTemp428} else {fTemp426};
			let mut fTemp458: F64 = fTemp457 + fTemp456;
			let mut fTemp459: F64 = 0.5 * fTemp458;
			let mut fTemp460: F64 = 65535.0 * (1.0 - fTemp459);
			let mut iTemp461: i32 = (fTemp460) as i32;
			let mut iTemp462: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp461, 65535)))), 196607));
			let mut fTemp463: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp462, 3)) as usize] };
			let mut fTemp464: F64 = unsafe { ftbl0mydspSIG0[iTemp462 as usize] };
			let mut fTemp465: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp462, 1)) as usize] } - fTemp464;
			let mut fTemp466: F64 = 32767.5 * fTemp458;
			let mut iTemp467: i32 = (fTemp466) as i32;
			let mut iTemp468: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp467, 65535)))), 196607));
			let mut fTemp469: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp468, 3)) as usize] };
			let mut fTemp470: F64 = unsafe { ftbl0mydspSIG0[iTemp468 as usize] };
			let mut fTemp471: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp468, 1)) as usize] } - fTemp470;
			let mut fTemp472: F64 = if iTemp38 != 0 {fTemp470 + fTemp52 * fTemp471 + (fTemp466 - (iTemp467) as F64) * (fTemp469 - (fTemp470 + fTemp52 * (fTemp471 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp468, 4)) as usize] } - fTemp469))))} else {1.0 - (fTemp464 + fTemp52 * fTemp465 + (fTemp460 - (iTemp461) as F64) * (fTemp463 - (fTemp464 + fTemp52 * (fTemp465 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp462, 4)) as usize] } - fTemp463)))))};
			let mut fTemp473: F64 = fTemp57 + fTemp459;
			let mut fTemp474: F64 = 65535.0 * (1.0 - fTemp473);
			let mut iTemp475: i32 = (fTemp474) as i32;
			let mut iTemp476: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp475, 65535)))), 196607));
			let mut fTemp477: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp476, 3)) as usize] };
			let mut fTemp478: F64 = unsafe { ftbl0mydspSIG0[iTemp476 as usize] };
			let mut fTemp479: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp476, 1)) as usize] } - fTemp478;
			let mut fTemp480: F64 = 65535.0 * fTemp473;
			let mut iTemp481: i32 = (fTemp480) as i32;
			let mut iTemp482: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp481, 65535)))), 196607));
			let mut fTemp483: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp482, 3), 196607))) as usize] };
			let mut fTemp484: F64 = unsafe { ftbl0mydspSIG0[iTemp482 as usize] };
			let mut fTemp485: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp482, 1), 196607))) as usize] } - fTemp484;
			let mut iTemp486: i32 = (fTemp113 > ((if iTemp38 != 0 {fTemp484 + fTemp52 * fTemp485 + (fTemp480 - (iTemp481) as F64) * (fTemp483 - (fTemp484 + fTemp52 * (fTemp485 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp482, 4), 196607))) as usize] } - fTemp483))))} else {1.0 - (fTemp478 + fTemp52 * fTemp479 + (fTemp474 - (iTemp475) as F64) * (fTemp477 - (fTemp478 + fTemp52 * (fTemp479 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp476, 4)) as usize] } - fTemp477)))))} - fTemp472) / (1.0 - fTemp472))) as i32;
			let mut fTemp487: F64 = if iTemp486 != 0 {fTemp456} else {fTemp459};
			let mut fTemp488: F64 = if iTemp486 != 0 {fTemp459} else {fTemp457};
			let mut fTemp489: F64 = fTemp488 + fTemp487;
			let mut fTemp490: F64 = 0.5 * fTemp489;
			let mut fTemp491: F64 = 65535.0 * (1.0 - fTemp490);
			let mut iTemp492: i32 = (fTemp491) as i32;
			let mut iTemp493: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp492, 65535)))), 196607));
			let mut fTemp494: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp493, 3)) as usize] };
			let mut fTemp495: F64 = unsafe { ftbl0mydspSIG0[iTemp493 as usize] };
			let mut fTemp496: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp493, 1)) as usize] } - fTemp495;
			let mut fTemp497: F64 = 32767.5 * fTemp489;
			let mut iTemp498: i32 = (fTemp497) as i32;
			let mut iTemp499: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp498, 65535)))), 196607));
			let mut fTemp500: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp499, 3)) as usize] };
			let mut fTemp501: F64 = unsafe { ftbl0mydspSIG0[iTemp499 as usize] };
			let mut fTemp502: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp499, 1)) as usize] } - fTemp501;
			let mut fTemp503: F64 = if iTemp38 != 0 {fTemp501 + fTemp52 * fTemp502 + (fTemp497 - (iTemp498) as F64) * (fTemp500 - (fTemp501 + fTemp52 * (fTemp502 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp499, 4)) as usize] } - fTemp500))))} else {1.0 - (fTemp495 + fTemp52 * fTemp496 + (fTemp491 - (iTemp492) as F64) * (fTemp494 - (fTemp495 + fTemp52 * (fTemp496 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp493, 4)) as usize] } - fTemp494)))))};
			let mut fTemp504: F64 = fTemp57 + fTemp490;
			let mut fTemp505: F64 = 65535.0 * (1.0 - fTemp504);
			let mut iTemp506: i32 = (fTemp505) as i32;
			let mut iTemp507: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp506, 65535)))), 196607));
			let mut fTemp508: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp507, 3)) as usize] };
			let mut fTemp509: F64 = unsafe { ftbl0mydspSIG0[iTemp507 as usize] };
			let mut fTemp510: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp507, 1)) as usize] } - fTemp509;
			let mut fTemp511: F64 = 65535.0 * fTemp504;
			let mut iTemp512: i32 = (fTemp511) as i32;
			let mut iTemp513: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp512, 65535)))), 196607));
			let mut fTemp514: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp513, 3), 196607))) as usize] };
			let mut fTemp515: F64 = unsafe { ftbl0mydspSIG0[iTemp513 as usize] };
			let mut fTemp516: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp513, 1), 196607))) as usize] } - fTemp515;
			let mut iTemp517: i32 = (fTemp113 > ((if iTemp38 != 0 {fTemp515 + fTemp52 * fTemp516 + (fTemp511 - (iTemp512) as F64) * (fTemp514 - (fTemp515 + fTemp52 * (fTemp516 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp513, 4), 196607))) as usize] } - fTemp514))))} else {1.0 - (fTemp509 + fTemp52 * fTemp510 + (fTemp505 - (iTemp506) as F64) * (fTemp508 - (fTemp509 + fTemp52 * (fTemp510 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp507, 4)) as usize] } - fTemp508)))))} - fTemp503) / (1.0 - fTemp503))) as i32;
			let mut fTemp518: F64 = if iTemp517 != 0 {fTemp487} else {fTemp490};
			let mut fTemp519: F64 = if iTemp517 != 0 {fTemp490} else {fTemp488};
			let mut fTemp520: F64 = fTemp519 + fTemp518;
			let mut fTemp521: F64 = 0.5 * fTemp520;
			let mut fTemp522: F64 = 65535.0 * (1.0 - fTemp521);
			let mut iTemp523: i32 = (fTemp522) as i32;
			let mut iTemp524: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp523, 65535)))), 196607));
			let mut fTemp525: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp524, 3)) as usize] };
			let mut fTemp526: F64 = unsafe { ftbl0mydspSIG0[iTemp524 as usize] };
			let mut fTemp527: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp524, 1)) as usize] } - fTemp526;
			let mut fTemp528: F64 = 32767.5 * fTemp520;
			let mut iTemp529: i32 = (fTemp528) as i32;
			let mut iTemp530: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp529, 65535)))), 196607));
			let mut fTemp531: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp530, 3)) as usize] };
			let mut fTemp532: F64 = unsafe { ftbl0mydspSIG0[iTemp530 as usize] };
			let mut fTemp533: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp530, 1)) as usize] } - fTemp532;
			let mut fTemp534: F64 = if iTemp38 != 0 {fTemp532 + fTemp52 * fTemp533 + (fTemp528 - (iTemp529) as F64) * (fTemp531 - (fTemp532 + fTemp52 * (fTemp533 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp530, 4)) as usize] } - fTemp531))))} else {1.0 - (fTemp526 + fTemp52 * fTemp527 + (fTemp522 - (iTemp523) as F64) * (fTemp525 - (fTemp526 + fTemp52 * (fTemp527 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp524, 4)) as usize] } - fTemp525)))))};
			let mut fTemp535: F64 = fTemp57 + fTemp521;
			let mut fTemp536: F64 = 65535.0 * (1.0 - fTemp535);
			let mut iTemp537: i32 = (fTemp536) as i32;
			let mut iTemp538: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp537, 65535)))), 196607));
			let mut fTemp539: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp538, 3)) as usize] };
			let mut fTemp540: F64 = unsafe { ftbl0mydspSIG0[iTemp538 as usize] };
			let mut fTemp541: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp538, 1)) as usize] } - fTemp540;
			let mut fTemp542: F64 = 65535.0 * fTemp535;
			let mut iTemp543: i32 = (fTemp542) as i32;
			let mut iTemp544: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp543, 65535)))), 196607));
			let mut fTemp545: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp544, 3), 196607))) as usize] };
			let mut fTemp546: F64 = unsafe { ftbl0mydspSIG0[iTemp544 as usize] };
			let mut fTemp547: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp544, 1), 196607))) as usize] } - fTemp546;
			let mut iTemp548: i32 = (fTemp113 > ((if iTemp38 != 0 {fTemp546 + fTemp52 * fTemp547 + (fTemp542 - (iTemp543) as F64) * (fTemp545 - (fTemp546 + fTemp52 * (fTemp547 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp544, 4), 196607))) as usize] } - fTemp545))))} else {1.0 - (fTemp540 + fTemp52 * fTemp541 + (fTemp536 - (iTemp537) as F64) * (fTemp539 - (fTemp540 + fTemp52 * (fTemp541 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp538, 4)) as usize] } - fTemp539)))))} - fTemp534) / (1.0 - fTemp534))) as i32;
			let mut fTemp549: F64 = if iTemp548 != 0 {fTemp518} else {fTemp521};
			let mut fTemp550: F64 = if iTemp548 != 0 {fTemp521} else {fTemp519};
			let mut fTemp551: F64 = fTemp550 + fTemp549;
			let mut fTemp552: F64 = 0.5 * fTemp551;
			let mut fTemp553: F64 = 65535.0 * (1.0 - fTemp552);
			let mut iTemp554: i32 = (fTemp553) as i32;
			let mut iTemp555: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp554, 65535)))), 196607));
			let mut fTemp556: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp555, 3)) as usize] };
			let mut fTemp557: F64 = unsafe { ftbl0mydspSIG0[iTemp555 as usize] };
			let mut fTemp558: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp555, 1)) as usize] } - fTemp557;
			let mut fTemp559: F64 = 32767.5 * fTemp551;
			let mut iTemp560: i32 = (fTemp559) as i32;
			let mut iTemp561: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp560, 65535)))), 196607));
			let mut fTemp562: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp561, 3)) as usize] };
			let mut fTemp563: F64 = unsafe { ftbl0mydspSIG0[iTemp561 as usize] };
			let mut fTemp564: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp561, 1)) as usize] } - fTemp563;
			let mut fTemp565: F64 = if iTemp38 != 0 {fTemp563 + fTemp52 * fTemp564 + (fTemp559 - (iTemp560) as F64) * (fTemp562 - (fTemp563 + fTemp52 * (fTemp564 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp561, 4), 196607))) as usize] } - fTemp562))))} else {1.0 - (fTemp557 + fTemp52 * fTemp558 + (fTemp553 - (iTemp554) as F64) * (fTemp556 - (fTemp557 + fTemp52 * (fTemp558 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp555, 4), 196607))) as usize] } - fTemp556)))))};
			let mut fTemp566: F64 = fTemp57 + fTemp552;
			let mut fTemp567: F64 = 65535.0 * (1.0 - fTemp566);
			let mut iTemp568: i32 = (fTemp567) as i32;
			let mut iTemp569: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp568, 65535)))), 196607));
			let mut fTemp570: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp569, 3)) as usize] };
			let mut fTemp571: F64 = unsafe { ftbl0mydspSIG0[iTemp569 as usize] };
			let mut fTemp572: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp569, 1)) as usize] } - fTemp571;
			let mut fTemp573: F64 = 65535.0 * fTemp566;
			let mut iTemp574: i32 = (fTemp573) as i32;
			let mut iTemp575: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp574, 65535)))), 196607));
			let mut fTemp576: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp575, 3), 196607))) as usize] };
			let mut fTemp577: F64 = unsafe { ftbl0mydspSIG0[iTemp575 as usize] };
			let mut fTemp578: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp575, 1), 196607))) as usize] } - fTemp577;
			let mut iTemp579: i32 = (fTemp113 > ((if iTemp38 != 0 {fTemp577 + fTemp52 * fTemp578 + (fTemp573 - (iTemp574) as F64) * (fTemp576 - (fTemp577 + fTemp52 * (fTemp578 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp575, 4), 196607))) as usize] } - fTemp576))))} else {1.0 - (fTemp571 + fTemp52 * fTemp572 + (fTemp567 - (iTemp568) as F64) * (fTemp570 - (fTemp571 + fTemp52 * (fTemp572 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp569, 4)) as usize] } - fTemp570)))))} - fTemp565) / (1.0 - fTemp565))) as i32;
			let mut fTemp580: F64 = F64::min(1.0, F64::max(0.0, 0.5 * (if iTemp579 != 0 {fTemp552} else {fTemp550} + if iTemp579 != 0 {fTemp549} else {fTemp552})));
			self.fRec3[0] = fTemp580;
			let mut fTemp581: F64 = 65535.0 * (1.0 - fTemp580);
			let mut iTemp582: i32 = (fTemp581) as i32;
			let mut iTemp583: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp582, 65535)))), 196607));
			let mut fTemp584: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp583, 3)) as usize] };
			let mut fTemp585: F64 = unsafe { ftbl0mydspSIG0[iTemp583 as usize] };
			let mut fTemp586: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp583, 1)) as usize] } - fTemp585;
			let mut fTemp587: F64 = 65535.0 * fTemp580;
			let mut iTemp588: i32 = (fTemp587) as i32;
			let mut iTemp589: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp588, 65535)))), 196607));
			let mut fTemp590: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp589, 3)) as usize] };
			let mut fTemp591: F64 = unsafe { ftbl0mydspSIG0[iTemp589 as usize] };
			let mut fTemp592: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp589, 1)) as usize] } - fTemp591;
			let mut fTemp593: F64 = if iTemp38 != 0 {fTemp591 + fTemp52 * fTemp592 + (fTemp587 - (iTemp588) as F64) * (fTemp590 - (fTemp591 + fTemp52 * (fTemp592 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp589, 4), 196607))) as usize] } - fTemp590))))} else {1.0 - (fTemp585 + fTemp52 * fTemp586 + (fTemp581 - (iTemp582) as F64) * (fTemp584 - (fTemp585 + fTemp52 * (fTemp586 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp583, 4), 196607))) as usize] } - fTemp584)))))};
			let mut fTemp594: F64 = fTemp57 + fTemp580;
			let mut fTemp595: F64 = 65535.0 * (1.0 - fTemp594);
			let mut iTemp596: i32 = (fTemp595) as i32;
			let mut iTemp597: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp596, 65535)))), 196607));
			let mut fTemp598: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp597, 3)) as usize] };
			let mut fTemp599: F64 = unsafe { ftbl0mydspSIG0[iTemp597 as usize] };
			let mut fTemp600: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp597, 1)) as usize] } - fTemp599;
			let mut fTemp601: F64 = 65535.0 * fTemp594;
			let mut iTemp602: i32 = (fTemp601) as i32;
			let mut iTemp603: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp602, 65535)))), 196607));
			let mut fTemp604: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp603, 3), 196607))) as usize] };
			let mut fTemp605: F64 = unsafe { ftbl0mydspSIG0[iTemp603 as usize] };
			let mut fTemp606: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp603, 1), 196607))) as usize] } - fTemp605;
			let mut fTemp607: F64 = self.fRec4[1] + if ((0.001 * fTemp56) == 0.0) as i32 != 0 {fTemp37} else {fTemp37 * (if iTemp38 != 0 {fTemp605 + fTemp52 * fTemp606 + (fTemp601 - (iTemp602) as F64) * (fTemp604 - (fTemp605 + fTemp52 * (fTemp606 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp603, 4), 196607))) as usize] } - fTemp604))))} else {1.0 - (fTemp599 + fTemp52 * fTemp600 + (fTemp595 - (iTemp596) as F64) * (fTemp598 - (fTemp599 + fTemp52 * (fTemp600 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp597, 4)) as usize] } - fTemp598)))))} - fTemp593) / (1.0 - fTemp593)};
			self.fRec4[0] = F64::min(self.fRec5[((i32::wrapping_sub(self.IOTA0, iSlow66)) & 16383) as usize], if iTemp55 != 0 {F64::min(fTemp607, self.fRec4[1])} else {F64::max(fTemp607, self.fRec4[1])});
			self.fVbargraph0 = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, self.fRec4[0]));
			*output0 = 0.5 * self.fVec0[((i32::wrapping_sub(self.IOTA0, iSlow4)) & 32767) as usize] * fTemp2 + self.fRec4[0] * self.fRec2[0] * self.fVec1[((i32::wrapping_sub(self.IOTA0, iSlow4)) & 32767) as usize] * fTemp3;
			let mut fTemp608: F64 = self.fRec9[((i32::wrapping_sub(self.IOTA0, 1)) & 16383) as usize];
			let mut fTemp609: F64 = fTemp10 + fSlow13 * (fTemp11 - fTemp10);
			let mut iTemp610: i32 = ((fTemp609 > fSlow14) as i32) + ((fTemp609 > fSlow12) as i32);
			let mut fTemp611: F64 = fTemp609 - fSlow11;
			let mut fTemp612: F64 = F64::min(1.0, F64::powf(1e+01, -(fSlow16 * F64::max(0.0, if (iTemp610 == 0) as i32 != 0 {0.0} else {if (iTemp610 == 1) as i32 != 0 {fSlow15 * mydsp_faustpower2_f(fSlow10 + fTemp611)} else {fTemp611}}))));
			self.fVec30[(self.IOTA0 & 16383) as usize] = fTemp612;
			let mut fTemp613: F64 = F64::min(fTemp612, self.fVec30[((i32::wrapping_sub(self.IOTA0, 1)) & 16383) as usize]);
			self.fVec31[0] = fTemp613;
			let mut fTemp614: F64 = F64::min(fTemp613, self.fVec31[2]);
			self.fVec32[0] = fTemp614;
			let mut fTemp615: F64 = F64::min(fTemp614, self.fVec32[4]);
			self.fVec33[0] = fTemp615;
			let mut fTemp616: F64 = F64::min(fTemp615, self.fVec33[8]);
			self.fVec34[(self.IOTA0 & 31) as usize] = fTemp616;
			let mut fTemp617: F64 = F64::min(fTemp616, self.fVec34[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize]);
			self.fVec35[(self.IOTA0 & 63) as usize] = fTemp617;
			let mut fTemp618: F64 = F64::min(fTemp617, self.fVec35[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize]);
			self.fVec36[(self.IOTA0 & 127) as usize] = fTemp618;
			let mut fTemp619: F64 = F64::min(fTemp618, self.fVec36[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize]);
			self.fVec37[(self.IOTA0 & 255) as usize] = fTemp619;
			let mut fTemp620: F64 = F64::min(fTemp619, self.fVec37[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize]);
			self.fVec38[(self.IOTA0 & 511) as usize] = fTemp620;
			let mut fTemp621: F64 = F64::min(fTemp620, self.fVec38[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize]);
			self.fVec39[(self.IOTA0 & 1023) as usize] = fTemp621;
			let mut fTemp622: F64 = F64::min(fTemp621, self.fVec39[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize]);
			self.fVec40[(self.IOTA0 & 2047) as usize] = fTemp622;
			self.fVec41[(self.IOTA0 & 4095) as usize] = F64::min(fTemp622, self.fVec40[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			self.fRec9[(self.IOTA0 & 16383) as usize] = F64::max(F64::min(fTemp608, self.fVec30[((i32::wrapping_sub(self.IOTA0, iSlow18)) & 16383) as usize]), F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow20 != 0 {fTemp612} else {1.7976931348623157e+308}, if iSlow21 != 0 {self.fVec31[iSlow20 as usize]} else {1.7976931348623157e+308}), if iSlow22 != 0 {self.fVec32[iSlow23 as usize]} else {1.7976931348623157e+308}), if iSlow24 != 0 {self.fVec33[iSlow25 as usize]} else {1.7976931348623157e+308}), if iSlow26 != 0 {self.fVec34[((i32::wrapping_sub(self.IOTA0, iSlow27)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow28 != 0 {self.fVec35[((i32::wrapping_sub(self.IOTA0, iSlow29)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow30 != 0 {self.fVec36[((i32::wrapping_sub(self.IOTA0, iSlow31)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow32 != 0 {self.fVec37[((i32::wrapping_sub(self.IOTA0, iSlow33)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow34 != 0 {self.fVec38[((i32::wrapping_sub(self.IOTA0, iSlow35)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow36 != 0 {self.fVec39[((i32::wrapping_sub(self.IOTA0, iSlow37)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow38 != 0 {self.fVec40[((i32::wrapping_sub(self.IOTA0, iSlow39)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow40 != 0 {self.fVec41[((i32::wrapping_sub(self.IOTA0, iSlow41)) & 4095) as usize]} else {1.7976931348623157e+308}));
			let mut fTemp623: F64 = self.fRec9[(self.IOTA0 & 16383) as usize];
			let mut fTemp624: F64 = F64::min(fTemp623, fTemp608);
			self.fVec42[0] = fTemp624;
			let mut fTemp625: F64 = F64::min(fTemp624, self.fVec42[2]);
			self.fVec43[0] = fTemp625;
			let mut fTemp626: F64 = F64::min(fTemp625, self.fVec43[4]);
			self.fVec44[0] = fTemp626;
			let mut fTemp627: F64 = F64::min(fTemp626, self.fVec44[8]);
			self.fVec45[(self.IOTA0 & 31) as usize] = fTemp627;
			let mut fTemp628: F64 = F64::min(fTemp627, self.fVec45[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize]);
			self.fVec46[(self.IOTA0 & 63) as usize] = fTemp628;
			let mut fTemp629: F64 = F64::min(fTemp628, self.fVec46[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize]);
			self.fVec47[(self.IOTA0 & 127) as usize] = fTemp629;
			let mut fTemp630: F64 = F64::min(fTemp629, self.fVec47[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize]);
			self.fVec48[(self.IOTA0 & 255) as usize] = fTemp630;
			let mut fTemp631: F64 = F64::min(fTemp630, self.fVec48[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize]);
			self.fVec49[(self.IOTA0 & 511) as usize] = fTemp631;
			let mut fTemp632: F64 = F64::min(fTemp631, self.fVec49[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize]);
			self.fVec50[(self.IOTA0 & 1023) as usize] = fTemp632;
			let mut fTemp633: F64 = F64::min(fTemp632, self.fVec50[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize]);
			self.fVec51[(self.IOTA0 & 2047) as usize] = fTemp633;
			self.fVec52[(self.IOTA0 & 4095) as usize] = F64::min(fTemp633, self.fVec51[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			let mut fTemp634: F64 = F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow8 != 0 {fTemp623} else {1.7976931348623157e+308}, if iSlow42 != 0 {self.fVec42[iSlow8 as usize]} else {1.7976931348623157e+308}), if iSlow43 != 0 {self.fVec43[iSlow44 as usize]} else {1.7976931348623157e+308}), if iSlow45 != 0 {self.fVec44[iSlow46 as usize]} else {1.7976931348623157e+308}), if iSlow47 != 0 {self.fVec45[((i32::wrapping_sub(self.IOTA0, iSlow48)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow49 != 0 {self.fVec46[((i32::wrapping_sub(self.IOTA0, iSlow50)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow51 != 0 {self.fVec47[((i32::wrapping_sub(self.IOTA0, iSlow52)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow53 != 0 {self.fVec48[((i32::wrapping_sub(self.IOTA0, iSlow54)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow55 != 0 {self.fVec49[((i32::wrapping_sub(self.IOTA0, iSlow56)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow57 != 0 {self.fVec50[((i32::wrapping_sub(self.IOTA0, iSlow58)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow59 != 0 {self.fVec51[((i32::wrapping_sub(self.IOTA0, iSlow60)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow61 != 0 {self.fVec52[((i32::wrapping_sub(self.IOTA0, iSlow62)) & 4095) as usize]} else {1.7976931348623157e+308}) - self.fRec8[1];
			self.fVec53[0] = fTemp634;
			let mut iTemp635: i32 = (fTemp634 > 0.0) as i32;
			let mut fTemp636: F64 = if iTemp635 != 0 {fSlow64} else {fSlow63};
			self.fVec54[0] = fTemp636;
			let mut fTemp637: F64 = 2.0 * fTemp636;
			let mut iTemp638: i32 = (fTemp637) as i32;
			let mut iTemp639: i32 = std::cmp::max(0, std::cmp::min(iTemp638, 2));
			let mut iTemp640: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, 98301), 196607));
			let mut fTemp641: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp640, 3)) as usize] };
			let mut fTemp642: F64 = unsafe { ftbl0mydspSIG0[iTemp640 as usize] };
			let mut fTemp643: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp640, 1)) as usize] } - fTemp642;
			let mut fTemp644: F64 = fTemp637 - (iTemp638) as F64;
			let mut fTemp645: F64 = fTemp642 + fTemp644 * fTemp643 + 0.5 * (fTemp641 - (fTemp642 + fTemp644 * (fTemp643 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp640, 4)) as usize] } - fTemp641))));
			let mut fTemp646: F64 = if iTemp635 != 0 {fTemp645} else {1.0 - fTemp645};
			let mut iTemp647: i32 = (fTemp634 < 0.0) as i32;
			let mut fTemp648: F64 = fSlow3 * (iTemp647) as F64 + fSlow65 * (iTemp635) as F64;
			self.fVec55[0] = fTemp648;
			let mut fTemp649: F64 = self.fConst5 / fTemp648;
			let mut fTemp650: F64 = fTemp649 + 0.5;
			let mut fTemp651: F64 = 65535.0 * (1.0 - fTemp650);
			let mut iTemp652: i32 = (fTemp651) as i32;
			let mut iTemp653: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp652, 65535)))), 196607));
			let mut fTemp654: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp653, 3)) as usize] };
			let mut fTemp655: F64 = unsafe { ftbl0mydspSIG0[iTemp653 as usize] };
			let mut fTemp656: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp653, 1)) as usize] } - fTemp655;
			let mut fTemp657: F64 = 65535.0 * fTemp650;
			let mut iTemp658: i32 = (fTemp657) as i32;
			let mut iTemp659: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp658, 65535)))), 196607));
			let mut fTemp660: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, 3), 196607))) as usize] };
			let mut fTemp661: F64 = unsafe { ftbl0mydspSIG0[iTemp659 as usize] };
			let mut fTemp662: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, 1), 196607))) as usize] } - fTemp661;
			let mut fTemp663: F64 = 2.0 * self.fVec54[1];
			let mut iTemp664: i32 = (fTemp663) as i32;
			let mut iTemp665: i32 = std::cmp::max(0, std::cmp::min(iTemp664, 2));
			let mut fTemp666: F64 = 65535.0 * (1.0 - self.fRec7[1]);
			let mut iTemp667: i32 = (fTemp666) as i32;
			let mut iTemp668: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp667, 65535))), iTemp665), 196607));
			let mut fTemp669: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp668, 3), 196607))) as usize] };
			let mut fTemp670: F64 = unsafe { ftbl0mydspSIG0[iTemp668 as usize] };
			let mut fTemp671: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp668, 1), 196607))) as usize] } - fTemp670;
			let mut fTemp672: F64 = fTemp663 - (iTemp664) as F64;
			let mut fTemp673: F64 = 65535.0 * self.fRec7[1];
			let mut iTemp674: i32 = (fTemp673) as i32;
			let mut iTemp675: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp665, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp674, 65535)))), 196607));
			let mut fTemp676: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp675, 3), 196607))) as usize] };
			let mut fTemp677: F64 = unsafe { ftbl0mydspSIG0[iTemp675 as usize] };
			let mut fTemp678: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp675, 1), 196607))) as usize] } - fTemp677;
			let mut fTemp679: F64 = self.fRec7[1] + fTemp649;
			let mut fTemp680: F64 = 65535.0 * (1.0 - fTemp679);
			let mut iTemp681: i32 = (fTemp680) as i32;
			let mut iTemp682: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp681, 65535)))), 196607));
			let mut fTemp683: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp682, 3)) as usize] };
			let mut fTemp684: F64 = unsafe { ftbl0mydspSIG0[iTemp682 as usize] };
			let mut fTemp685: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp682, 1)) as usize] } - fTemp684;
			let mut fTemp686: F64 = 65535.0 * fTemp679;
			let mut iTemp687: i32 = (fTemp686) as i32;
			let mut iTemp688: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp687, 65535)))), 196607));
			let mut fTemp689: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp688, 3), 196607))) as usize] };
			let mut fTemp690: F64 = unsafe { ftbl0mydspSIG0[iTemp688 as usize] };
			let mut fTemp691: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp688, 1), 196607))) as usize] } - fTemp690;
			let mut fTemp692: F64 = self.fRec7[1] + self.fConst5 * (1.0 / fTemp648 + 1.0 / self.fVec55[1]);
			let mut fTemp693: F64 = 65535.0 * (1.0 - fTemp692);
			let mut iTemp694: i32 = (fTemp693) as i32;
			let mut iTemp695: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp694, 65535))), iTemp639), 196607));
			let mut fTemp696: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp695, 3)) as usize] };
			let mut fTemp697: F64 = unsafe { ftbl0mydspSIG0[iTemp695 as usize] };
			let mut fTemp698: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp695, 1)) as usize] } - fTemp697;
			let mut fTemp699: F64 = 65535.0 * fTemp692;
			let mut iTemp700: i32 = (fTemp699) as i32;
			let mut iTemp701: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp700, 65535)))), 196607));
			let mut fTemp702: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp701, 3), 196607))) as usize] };
			let mut fTemp703: F64 = unsafe { ftbl0mydspSIG0[iTemp701 as usize] };
			let mut fTemp704: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp701, 1), 196607))) as usize] } - fTemp703;
			let mut fTemp705: F64 = (if iTemp635 != 0 {fTemp703 + fTemp644 * fTemp704 + (fTemp699 - (iTemp700) as F64) * (fTemp702 - (fTemp703 + fTemp644 * (fTemp704 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp701, 4), 196607))) as usize] } - fTemp702))))} else {1.0 - (fTemp697 + fTemp644 * fTemp698 + (fTemp693 - (iTemp694) as F64) * (fTemp696 - (fTemp697 + fTemp644 * (fTemp698 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp695, 4)) as usize] } - fTemp696)))))} - if iTemp635 != 0 {fTemp690 + fTemp644 * fTemp691 + (fTemp686 - (iTemp687) as F64) * (fTemp689 - (fTemp690 + fTemp644 * (fTemp691 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp688, 4), 196607))) as usize] } - fTemp689))))} else {1.0 - (fTemp684 + fTemp644 * fTemp685 + (fTemp680 - (iTemp681) as F64) * (fTemp683 - (fTemp684 + fTemp644 * (fTemp685 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp682, 4), 196607))) as usize] } - fTemp683)))))}) * self.fVec53[1] / (fTemp634 * (1.0 - if iTemp635 != 0 {fTemp677 + fTemp672 * fTemp678 + (fTemp673 - (iTemp674) as F64) * (fTemp676 - (fTemp677 + fTemp672 * (fTemp678 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp675, 4), 196607))) as usize] } - fTemp676))))} else {1.0 - (fTemp670 + fTemp672 * fTemp671 + (fTemp666 - (iTemp667) as F64) * (fTemp669 - (fTemp670 + fTemp672 * (fTemp671 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp668, 4), 196607))) as usize] } - fTemp669)))))}));
			let mut iTemp706: i32 = (fTemp705 > ((if iTemp635 != 0 {fTemp661 + fTemp644 * fTemp662 + (fTemp657 - (iTemp658) as F64) * (fTemp660 - (fTemp661 + fTemp644 * (fTemp662 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp659, 4), 196607))) as usize] } - fTemp660))))} else {1.0 - (fTemp655 + fTemp644 * fTemp656 + (fTemp651 - (iTemp652) as F64) * (fTemp654 - (fTemp655 + fTemp644 * (fTemp656 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp653, 4)) as usize] } - fTemp654)))))} - fTemp646) / (1.0 - fTemp646))) as i32;
			let mut fTemp707: F64 = if iTemp706 != 0 {1.0} else {0.5};
			let mut fTemp708: F64 = if iTemp706 != 0 {0.5} else {0.0};
			let mut fTemp709: F64 = fTemp708 + fTemp707;
			let mut fTemp710: F64 = 0.5 * fTemp709;
			let mut fTemp711: F64 = 65535.0 * (1.0 - fTemp710);
			let mut iTemp712: i32 = (fTemp711) as i32;
			let mut iTemp713: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp712, 65535)))), 196607));
			let mut fTemp714: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp713, 3)) as usize] };
			let mut fTemp715: F64 = unsafe { ftbl0mydspSIG0[iTemp713 as usize] };
			let mut fTemp716: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp713, 1)) as usize] } - fTemp715;
			let mut fTemp717: F64 = 32767.5 * fTemp709;
			let mut iTemp718: i32 = (fTemp717) as i32;
			let mut iTemp719: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp718, 65535)))), 196607));
			let mut fTemp720: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp719, 3)) as usize] };
			let mut fTemp721: F64 = unsafe { ftbl0mydspSIG0[iTemp719 as usize] };
			let mut fTemp722: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp719, 1)) as usize] } - fTemp721;
			let mut fTemp723: F64 = if iTemp635 != 0 {fTemp721 + fTemp644 * fTemp722 + (fTemp717 - (iTemp718) as F64) * (fTemp720 - (fTemp721 + fTemp644 * (fTemp722 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp719, 4)) as usize] } - fTemp720))))} else {1.0 - (fTemp715 + fTemp644 * fTemp716 + (fTemp711 - (iTemp712) as F64) * (fTemp714 - (fTemp715 + fTemp644 * (fTemp716 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp713, 4)) as usize] } - fTemp714)))))};
			let mut fTemp724: F64 = fTemp649 + fTemp710;
			let mut fTemp725: F64 = 65535.0 * (1.0 - fTemp724);
			let mut iTemp726: i32 = (fTemp725) as i32;
			let mut iTemp727: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp726, 65535)))), 196607));
			let mut fTemp728: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp727, 3)) as usize] };
			let mut fTemp729: F64 = unsafe { ftbl0mydspSIG0[iTemp727 as usize] };
			let mut fTemp730: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp727, 1)) as usize] } - fTemp729;
			let mut fTemp731: F64 = 65535.0 * fTemp724;
			let mut iTemp732: i32 = (fTemp731) as i32;
			let mut iTemp733: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp732, 65535)))), 196607));
			let mut fTemp734: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp733, 3), 196607))) as usize] };
			let mut fTemp735: F64 = unsafe { ftbl0mydspSIG0[iTemp733 as usize] };
			let mut fTemp736: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp733, 1), 196607))) as usize] } - fTemp735;
			let mut iTemp737: i32 = (fTemp705 > ((if iTemp635 != 0 {fTemp735 + fTemp644 * fTemp736 + (fTemp731 - (iTemp732) as F64) * (fTemp734 - (fTemp735 + fTemp644 * (fTemp736 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp733, 4), 196607))) as usize] } - fTemp734))))} else {1.0 - (fTemp729 + fTemp644 * fTemp730 + (fTemp725 - (iTemp726) as F64) * (fTemp728 - (fTemp729 + fTemp644 * (fTemp730 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp727, 4)) as usize] } - fTemp728)))))} - fTemp723) / (1.0 - fTemp723))) as i32;
			let mut fTemp738: F64 = if iTemp737 != 0 {fTemp707} else {fTemp710};
			let mut fTemp739: F64 = if iTemp737 != 0 {fTemp710} else {fTemp708};
			let mut fTemp740: F64 = fTemp739 + fTemp738;
			let mut fTemp741: F64 = 0.5 * fTemp740;
			let mut fTemp742: F64 = 65535.0 * (1.0 - fTemp741);
			let mut iTemp743: i32 = (fTemp742) as i32;
			let mut iTemp744: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp743, 65535)))), 196607));
			let mut fTemp745: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp744, 3)) as usize] };
			let mut fTemp746: F64 = unsafe { ftbl0mydspSIG0[iTemp744 as usize] };
			let mut fTemp747: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp744, 1)) as usize] } - fTemp746;
			let mut fTemp748: F64 = 32767.5 * fTemp740;
			let mut iTemp749: i32 = (fTemp748) as i32;
			let mut iTemp750: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp749, 65535)))), 196607));
			let mut fTemp751: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp750, 3)) as usize] };
			let mut fTemp752: F64 = unsafe { ftbl0mydspSIG0[iTemp750 as usize] };
			let mut fTemp753: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp750, 1)) as usize] } - fTemp752;
			let mut fTemp754: F64 = if iTemp635 != 0 {fTemp752 + fTemp644 * fTemp753 + (fTemp748 - (iTemp749) as F64) * (fTemp751 - (fTemp752 + fTemp644 * (fTemp753 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp750, 4)) as usize] } - fTemp751))))} else {1.0 - (fTemp746 + fTemp644 * fTemp747 + (fTemp742 - (iTemp743) as F64) * (fTemp745 - (fTemp746 + fTemp644 * (fTemp747 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp744, 4)) as usize] } - fTemp745)))))};
			let mut fTemp755: F64 = fTemp649 + fTemp741;
			let mut fTemp756: F64 = 65535.0 * (1.0 - fTemp755);
			let mut iTemp757: i32 = (fTemp756) as i32;
			let mut iTemp758: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp757, 65535)))), 196607));
			let mut fTemp759: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp758, 3)) as usize] };
			let mut fTemp760: F64 = unsafe { ftbl0mydspSIG0[iTemp758 as usize] };
			let mut fTemp761: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp758, 1)) as usize] } - fTemp760;
			let mut fTemp762: F64 = 65535.0 * fTemp755;
			let mut iTemp763: i32 = (fTemp762) as i32;
			let mut iTemp764: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp763, 65535)))), 196607));
			let mut fTemp765: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp764, 3), 196607))) as usize] };
			let mut fTemp766: F64 = unsafe { ftbl0mydspSIG0[iTemp764 as usize] };
			let mut fTemp767: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp764, 1), 196607))) as usize] } - fTemp766;
			let mut iTemp768: i32 = (fTemp705 > ((if iTemp635 != 0 {fTemp766 + fTemp644 * fTemp767 + (fTemp762 - (iTemp763) as F64) * (fTemp765 - (fTemp766 + fTemp644 * (fTemp767 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp764, 4), 196607))) as usize] } - fTemp765))))} else {1.0 - (fTemp760 + fTemp644 * fTemp761 + (fTemp756 - (iTemp757) as F64) * (fTemp759 - (fTemp760 + fTemp644 * (fTemp761 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp758, 4)) as usize] } - fTemp759)))))} - fTemp754) / (1.0 - fTemp754))) as i32;
			let mut fTemp769: F64 = if iTemp768 != 0 {fTemp738} else {fTemp741};
			let mut fTemp770: F64 = if iTemp768 != 0 {fTemp741} else {fTemp739};
			let mut fTemp771: F64 = fTemp770 + fTemp769;
			let mut fTemp772: F64 = 0.5 * fTemp771;
			let mut fTemp773: F64 = 65535.0 * (1.0 - fTemp772);
			let mut iTemp774: i32 = (fTemp773) as i32;
			let mut iTemp775: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp774, 65535)))), 196607));
			let mut fTemp776: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp775, 3)) as usize] };
			let mut fTemp777: F64 = unsafe { ftbl0mydspSIG0[iTemp775 as usize] };
			let mut fTemp778: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp775, 1)) as usize] } - fTemp777;
			let mut fTemp779: F64 = 32767.5 * fTemp771;
			let mut iTemp780: i32 = (fTemp779) as i32;
			let mut iTemp781: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp780, 65535)))), 196607));
			let mut fTemp782: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp781, 3)) as usize] };
			let mut fTemp783: F64 = unsafe { ftbl0mydspSIG0[iTemp781 as usize] };
			let mut fTemp784: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp781, 1)) as usize] } - fTemp783;
			let mut fTemp785: F64 = if iTemp635 != 0 {fTemp783 + fTemp644 * fTemp784 + (fTemp779 - (iTemp780) as F64) * (fTemp782 - (fTemp783 + fTemp644 * (fTemp784 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp781, 4)) as usize] } - fTemp782))))} else {1.0 - (fTemp777 + fTemp644 * fTemp778 + (fTemp773 - (iTemp774) as F64) * (fTemp776 - (fTemp777 + fTemp644 * (fTemp778 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp775, 4)) as usize] } - fTemp776)))))};
			let mut fTemp786: F64 = fTemp649 + fTemp772;
			let mut fTemp787: F64 = 65535.0 * (1.0 - fTemp786);
			let mut iTemp788: i32 = (fTemp787) as i32;
			let mut iTemp789: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp788, 65535)))), 196607));
			let mut fTemp790: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp789, 3)) as usize] };
			let mut fTemp791: F64 = unsafe { ftbl0mydspSIG0[iTemp789 as usize] };
			let mut fTemp792: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp789, 1)) as usize] } - fTemp791;
			let mut fTemp793: F64 = 65535.0 * fTemp786;
			let mut iTemp794: i32 = (fTemp793) as i32;
			let mut iTemp795: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp794, 65535)))), 196607));
			let mut fTemp796: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp795, 3), 196607))) as usize] };
			let mut fTemp797: F64 = unsafe { ftbl0mydspSIG0[iTemp795 as usize] };
			let mut fTemp798: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp795, 1), 196607))) as usize] } - fTemp797;
			let mut iTemp799: i32 = (fTemp705 > ((if iTemp635 != 0 {fTemp797 + fTemp644 * fTemp798 + (fTemp793 - (iTemp794) as F64) * (fTemp796 - (fTemp797 + fTemp644 * (fTemp798 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp795, 4), 196607))) as usize] } - fTemp796))))} else {1.0 - (fTemp791 + fTemp644 * fTemp792 + (fTemp787 - (iTemp788) as F64) * (fTemp790 - (fTemp791 + fTemp644 * (fTemp792 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp789, 4)) as usize] } - fTemp790)))))} - fTemp785) / (1.0 - fTemp785))) as i32;
			let mut fTemp800: F64 = if iTemp799 != 0 {fTemp769} else {fTemp772};
			let mut fTemp801: F64 = if iTemp799 != 0 {fTemp772} else {fTemp770};
			let mut fTemp802: F64 = fTemp801 + fTemp800;
			let mut fTemp803: F64 = 0.5 * fTemp802;
			let mut fTemp804: F64 = 65535.0 * (1.0 - fTemp803);
			let mut iTemp805: i32 = (fTemp804) as i32;
			let mut iTemp806: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp805, 65535)))), 196607));
			let mut fTemp807: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp806, 3)) as usize] };
			let mut fTemp808: F64 = unsafe { ftbl0mydspSIG0[iTemp806 as usize] };
			let mut fTemp809: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp806, 1)) as usize] } - fTemp808;
			let mut fTemp810: F64 = 32767.5 * fTemp802;
			let mut iTemp811: i32 = (fTemp810) as i32;
			let mut iTemp812: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp811, 65535)))), 196607));
			let mut fTemp813: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp812, 3)) as usize] };
			let mut fTemp814: F64 = unsafe { ftbl0mydspSIG0[iTemp812 as usize] };
			let mut fTemp815: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp812, 1)) as usize] } - fTemp814;
			let mut fTemp816: F64 = if iTemp635 != 0 {fTemp814 + fTemp644 * fTemp815 + (fTemp810 - (iTemp811) as F64) * (fTemp813 - (fTemp814 + fTemp644 * (fTemp815 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp812, 4)) as usize] } - fTemp813))))} else {1.0 - (fTemp808 + fTemp644 * fTemp809 + (fTemp804 - (iTemp805) as F64) * (fTemp807 - (fTemp808 + fTemp644 * (fTemp809 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp806, 4)) as usize] } - fTemp807)))))};
			let mut fTemp817: F64 = fTemp649 + fTemp803;
			let mut fTemp818: F64 = 65535.0 * (1.0 - fTemp817);
			let mut iTemp819: i32 = (fTemp818) as i32;
			let mut iTemp820: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp819, 65535)))), 196607));
			let mut fTemp821: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp820, 3)) as usize] };
			let mut fTemp822: F64 = unsafe { ftbl0mydspSIG0[iTemp820 as usize] };
			let mut fTemp823: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp820, 1)) as usize] } - fTemp822;
			let mut fTemp824: F64 = 65535.0 * fTemp817;
			let mut iTemp825: i32 = (fTemp824) as i32;
			let mut iTemp826: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp825, 65535)))), 196607));
			let mut fTemp827: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp826, 3), 196607))) as usize] };
			let mut fTemp828: F64 = unsafe { ftbl0mydspSIG0[iTemp826 as usize] };
			let mut fTemp829: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp826, 1), 196607))) as usize] } - fTemp828;
			let mut iTemp830: i32 = (fTemp705 > ((if iTemp635 != 0 {fTemp828 + fTemp644 * fTemp829 + (fTemp824 - (iTemp825) as F64) * (fTemp827 - (fTemp828 + fTemp644 * (fTemp829 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp826, 4), 196607))) as usize] } - fTemp827))))} else {1.0 - (fTemp822 + fTemp644 * fTemp823 + (fTemp818 - (iTemp819) as F64) * (fTemp821 - (fTemp822 + fTemp644 * (fTemp823 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp820, 4)) as usize] } - fTemp821)))))} - fTemp816) / (1.0 - fTemp816))) as i32;
			let mut fTemp831: F64 = if iTemp830 != 0 {fTemp800} else {fTemp803};
			let mut fTemp832: F64 = if iTemp830 != 0 {fTemp803} else {fTemp801};
			let mut fTemp833: F64 = fTemp832 + fTemp831;
			let mut fTemp834: F64 = 0.5 * fTemp833;
			let mut fTemp835: F64 = 65535.0 * (1.0 - fTemp834);
			let mut iTemp836: i32 = (fTemp835) as i32;
			let mut iTemp837: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp836, 65535)))), 196607));
			let mut fTemp838: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp837, 3)) as usize] };
			let mut fTemp839: F64 = unsafe { ftbl0mydspSIG0[iTemp837 as usize] };
			let mut fTemp840: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp837, 1)) as usize] } - fTemp839;
			let mut fTemp841: F64 = 32767.5 * fTemp833;
			let mut iTemp842: i32 = (fTemp841) as i32;
			let mut iTemp843: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp842, 65535)))), 196607));
			let mut fTemp844: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp843, 3)) as usize] };
			let mut fTemp845: F64 = unsafe { ftbl0mydspSIG0[iTemp843 as usize] };
			let mut fTemp846: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp843, 1)) as usize] } - fTemp845;
			let mut fTemp847: F64 = if iTemp635 != 0 {fTemp845 + fTemp644 * fTemp846 + (fTemp841 - (iTemp842) as F64) * (fTemp844 - (fTemp845 + fTemp644 * (fTemp846 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp843, 4)) as usize] } - fTemp844))))} else {1.0 - (fTemp839 + fTemp644 * fTemp840 + (fTemp835 - (iTemp836) as F64) * (fTemp838 - (fTemp839 + fTemp644 * (fTemp840 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp837, 4)) as usize] } - fTemp838)))))};
			let mut fTemp848: F64 = fTemp649 + fTemp834;
			let mut fTemp849: F64 = 65535.0 * (1.0 - fTemp848);
			let mut iTemp850: i32 = (fTemp849) as i32;
			let mut iTemp851: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp850, 65535)))), 196607));
			let mut fTemp852: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp851, 3)) as usize] };
			let mut fTemp853: F64 = unsafe { ftbl0mydspSIG0[iTemp851 as usize] };
			let mut fTemp854: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp851, 1)) as usize] } - fTemp853;
			let mut fTemp855: F64 = 65535.0 * fTemp848;
			let mut iTemp856: i32 = (fTemp855) as i32;
			let mut iTemp857: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp856, 65535)))), 196607));
			let mut fTemp858: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp857, 3), 196607))) as usize] };
			let mut fTemp859: F64 = unsafe { ftbl0mydspSIG0[iTemp857 as usize] };
			let mut fTemp860: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp857, 1), 196607))) as usize] } - fTemp859;
			let mut iTemp861: i32 = (fTemp705 > ((if iTemp635 != 0 {fTemp859 + fTemp644 * fTemp860 + (fTemp855 - (iTemp856) as F64) * (fTemp858 - (fTemp859 + fTemp644 * (fTemp860 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp857, 4), 196607))) as usize] } - fTemp858))))} else {1.0 - (fTemp853 + fTemp644 * fTemp854 + (fTemp849 - (iTemp850) as F64) * (fTemp852 - (fTemp853 + fTemp644 * (fTemp854 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp851, 4)) as usize] } - fTemp852)))))} - fTemp847) / (1.0 - fTemp847))) as i32;
			let mut fTemp862: F64 = if iTemp861 != 0 {fTemp831} else {fTemp834};
			let mut fTemp863: F64 = if iTemp861 != 0 {fTemp834} else {fTemp832};
			let mut fTemp864: F64 = fTemp863 + fTemp862;
			let mut fTemp865: F64 = 0.5 * fTemp864;
			let mut fTemp866: F64 = 65535.0 * (1.0 - fTemp865);
			let mut iTemp867: i32 = (fTemp866) as i32;
			let mut iTemp868: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp867, 65535)))), 196607));
			let mut fTemp869: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp868, 3)) as usize] };
			let mut fTemp870: F64 = unsafe { ftbl0mydspSIG0[iTemp868 as usize] };
			let mut fTemp871: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp868, 1)) as usize] } - fTemp870;
			let mut fTemp872: F64 = 32767.5 * fTemp864;
			let mut iTemp873: i32 = (fTemp872) as i32;
			let mut iTemp874: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp873, 65535)))), 196607));
			let mut fTemp875: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp874, 3)) as usize] };
			let mut fTemp876: F64 = unsafe { ftbl0mydspSIG0[iTemp874 as usize] };
			let mut fTemp877: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp874, 1)) as usize] } - fTemp876;
			let mut fTemp878: F64 = if iTemp635 != 0 {fTemp876 + fTemp644 * fTemp877 + (fTemp872 - (iTemp873) as F64) * (fTemp875 - (fTemp876 + fTemp644 * (fTemp877 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp874, 4)) as usize] } - fTemp875))))} else {1.0 - (fTemp870 + fTemp644 * fTemp871 + (fTemp866 - (iTemp867) as F64) * (fTemp869 - (fTemp870 + fTemp644 * (fTemp871 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp868, 4)) as usize] } - fTemp869)))))};
			let mut fTemp879: F64 = fTemp649 + fTemp865;
			let mut fTemp880: F64 = 65535.0 * (1.0 - fTemp879);
			let mut iTemp881: i32 = (fTemp880) as i32;
			let mut iTemp882: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp881, 65535)))), 196607));
			let mut fTemp883: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp882, 3)) as usize] };
			let mut fTemp884: F64 = unsafe { ftbl0mydspSIG0[iTemp882 as usize] };
			let mut fTemp885: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp882, 1)) as usize] } - fTemp884;
			let mut fTemp886: F64 = 65535.0 * fTemp879;
			let mut iTemp887: i32 = (fTemp886) as i32;
			let mut iTemp888: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp887, 65535)))), 196607));
			let mut fTemp889: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp888, 3), 196607))) as usize] };
			let mut fTemp890: F64 = unsafe { ftbl0mydspSIG0[iTemp888 as usize] };
			let mut fTemp891: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp888, 1), 196607))) as usize] } - fTemp890;
			let mut iTemp892: i32 = (fTemp705 > ((if iTemp635 != 0 {fTemp890 + fTemp644 * fTemp891 + (fTemp886 - (iTemp887) as F64) * (fTemp889 - (fTemp890 + fTemp644 * (fTemp891 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp888, 4), 196607))) as usize] } - fTemp889))))} else {1.0 - (fTemp884 + fTemp644 * fTemp885 + (fTemp880 - (iTemp881) as F64) * (fTemp883 - (fTemp884 + fTemp644 * (fTemp885 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp882, 4)) as usize] } - fTemp883)))))} - fTemp878) / (1.0 - fTemp878))) as i32;
			let mut fTemp893: F64 = if iTemp892 != 0 {fTemp862} else {fTemp865};
			let mut fTemp894: F64 = if iTemp892 != 0 {fTemp865} else {fTemp863};
			let mut fTemp895: F64 = fTemp894 + fTemp893;
			let mut fTemp896: F64 = 0.5 * fTemp895;
			let mut fTemp897: F64 = 65535.0 * (1.0 - fTemp896);
			let mut iTemp898: i32 = (fTemp897) as i32;
			let mut iTemp899: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp898, 65535)))), 196607));
			let mut fTemp900: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp899, 3)) as usize] };
			let mut fTemp901: F64 = unsafe { ftbl0mydspSIG0[iTemp899 as usize] };
			let mut fTemp902: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp899, 1)) as usize] } - fTemp901;
			let mut fTemp903: F64 = 32767.5 * fTemp895;
			let mut iTemp904: i32 = (fTemp903) as i32;
			let mut iTemp905: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp904, 65535)))), 196607));
			let mut fTemp906: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp905, 3)) as usize] };
			let mut fTemp907: F64 = unsafe { ftbl0mydspSIG0[iTemp905 as usize] };
			let mut fTemp908: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp905, 1)) as usize] } - fTemp907;
			let mut fTemp909: F64 = if iTemp635 != 0 {fTemp907 + fTemp644 * fTemp908 + (fTemp903 - (iTemp904) as F64) * (fTemp906 - (fTemp907 + fTemp644 * (fTemp908 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp905, 4)) as usize] } - fTemp906))))} else {1.0 - (fTemp901 + fTemp644 * fTemp902 + (fTemp897 - (iTemp898) as F64) * (fTemp900 - (fTemp901 + fTemp644 * (fTemp902 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp899, 4)) as usize] } - fTemp900)))))};
			let mut fTemp910: F64 = fTemp649 + fTemp896;
			let mut fTemp911: F64 = 65535.0 * (1.0 - fTemp910);
			let mut iTemp912: i32 = (fTemp911) as i32;
			let mut iTemp913: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp912, 65535)))), 196607));
			let mut fTemp914: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp913, 3)) as usize] };
			let mut fTemp915: F64 = unsafe { ftbl0mydspSIG0[iTemp913 as usize] };
			let mut fTemp916: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp913, 1)) as usize] } - fTemp915;
			let mut fTemp917: F64 = 65535.0 * fTemp910;
			let mut iTemp918: i32 = (fTemp917) as i32;
			let mut iTemp919: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp918, 65535)))), 196607));
			let mut fTemp920: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp919, 3), 196607))) as usize] };
			let mut fTemp921: F64 = unsafe { ftbl0mydspSIG0[iTemp919 as usize] };
			let mut fTemp922: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp919, 1), 196607))) as usize] } - fTemp921;
			let mut iTemp923: i32 = (fTemp705 > ((if iTemp635 != 0 {fTemp921 + fTemp644 * fTemp922 + (fTemp917 - (iTemp918) as F64) * (fTemp920 - (fTemp921 + fTemp644 * (fTemp922 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp919, 4), 196607))) as usize] } - fTemp920))))} else {1.0 - (fTemp915 + fTemp644 * fTemp916 + (fTemp911 - (iTemp912) as F64) * (fTemp914 - (fTemp915 + fTemp644 * (fTemp916 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp913, 4)) as usize] } - fTemp914)))))} - fTemp909) / (1.0 - fTemp909))) as i32;
			let mut fTemp924: F64 = if iTemp923 != 0 {fTemp893} else {fTemp896};
			let mut fTemp925: F64 = if iTemp923 != 0 {fTemp896} else {fTemp894};
			let mut fTemp926: F64 = fTemp925 + fTemp924;
			let mut fTemp927: F64 = 0.5 * fTemp926;
			let mut fTemp928: F64 = 65535.0 * (1.0 - fTemp927);
			let mut iTemp929: i32 = (fTemp928) as i32;
			let mut iTemp930: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp929, 65535)))), 196607));
			let mut fTemp931: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp930, 3)) as usize] };
			let mut fTemp932: F64 = unsafe { ftbl0mydspSIG0[iTemp930 as usize] };
			let mut fTemp933: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp930, 1)) as usize] } - fTemp932;
			let mut fTemp934: F64 = 32767.5 * fTemp926;
			let mut iTemp935: i32 = (fTemp934) as i32;
			let mut iTemp936: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp935, 65535)))), 196607));
			let mut fTemp937: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp936, 3)) as usize] };
			let mut fTemp938: F64 = unsafe { ftbl0mydspSIG0[iTemp936 as usize] };
			let mut fTemp939: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp936, 1)) as usize] } - fTemp938;
			let mut fTemp940: F64 = if iTemp635 != 0 {fTemp938 + fTemp644 * fTemp939 + (fTemp934 - (iTemp935) as F64) * (fTemp937 - (fTemp938 + fTemp644 * (fTemp939 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp936, 4)) as usize] } - fTemp937))))} else {1.0 - (fTemp932 + fTemp644 * fTemp933 + (fTemp928 - (iTemp929) as F64) * (fTemp931 - (fTemp932 + fTemp644 * (fTemp933 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp930, 4)) as usize] } - fTemp931)))))};
			let mut fTemp941: F64 = fTemp649 + fTemp927;
			let mut fTemp942: F64 = 65535.0 * (1.0 - fTemp941);
			let mut iTemp943: i32 = (fTemp942) as i32;
			let mut iTemp944: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp943, 65535)))), 196607));
			let mut fTemp945: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp944, 3)) as usize] };
			let mut fTemp946: F64 = unsafe { ftbl0mydspSIG0[iTemp944 as usize] };
			let mut fTemp947: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp944, 1)) as usize] } - fTemp946;
			let mut fTemp948: F64 = 65535.0 * fTemp941;
			let mut iTemp949: i32 = (fTemp948) as i32;
			let mut iTemp950: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp949, 65535)))), 196607));
			let mut fTemp951: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp950, 3), 196607))) as usize] };
			let mut fTemp952: F64 = unsafe { ftbl0mydspSIG0[iTemp950 as usize] };
			let mut fTemp953: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp950, 1), 196607))) as usize] } - fTemp952;
			let mut iTemp954: i32 = (fTemp705 > ((if iTemp635 != 0 {fTemp952 + fTemp644 * fTemp953 + (fTemp948 - (iTemp949) as F64) * (fTemp951 - (fTemp952 + fTemp644 * (fTemp953 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp950, 4), 196607))) as usize] } - fTemp951))))} else {1.0 - (fTemp946 + fTemp644 * fTemp947 + (fTemp942 - (iTemp943) as F64) * (fTemp945 - (fTemp946 + fTemp644 * (fTemp947 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp944, 4)) as usize] } - fTemp945)))))} - fTemp940) / (1.0 - fTemp940))) as i32;
			let mut fTemp955: F64 = if iTemp954 != 0 {fTemp924} else {fTemp927};
			let mut fTemp956: F64 = if iTemp954 != 0 {fTemp927} else {fTemp925};
			let mut fTemp957: F64 = fTemp956 + fTemp955;
			let mut fTemp958: F64 = 0.5 * fTemp957;
			let mut fTemp959: F64 = 65535.0 * (1.0 - fTemp958);
			let mut iTemp960: i32 = (fTemp959) as i32;
			let mut iTemp961: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp960, 65535)))), 196607));
			let mut fTemp962: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp961, 3)) as usize] };
			let mut fTemp963: F64 = unsafe { ftbl0mydspSIG0[iTemp961 as usize] };
			let mut fTemp964: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp961, 1)) as usize] } - fTemp963;
			let mut fTemp965: F64 = 32767.5 * fTemp957;
			let mut iTemp966: i32 = (fTemp965) as i32;
			let mut iTemp967: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp966, 65535)))), 196607));
			let mut fTemp968: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp967, 3)) as usize] };
			let mut fTemp969: F64 = unsafe { ftbl0mydspSIG0[iTemp967 as usize] };
			let mut fTemp970: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp967, 1)) as usize] } - fTemp969;
			let mut fTemp971: F64 = if iTemp635 != 0 {fTemp969 + fTemp644 * fTemp970 + (fTemp965 - (iTemp966) as F64) * (fTemp968 - (fTemp969 + fTemp644 * (fTemp970 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp967, 4)) as usize] } - fTemp968))))} else {1.0 - (fTemp963 + fTemp644 * fTemp964 + (fTemp959 - (iTemp960) as F64) * (fTemp962 - (fTemp963 + fTemp644 * (fTemp964 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp961, 4)) as usize] } - fTemp962)))))};
			let mut fTemp972: F64 = fTemp649 + fTemp958;
			let mut fTemp973: F64 = 65535.0 * (1.0 - fTemp972);
			let mut iTemp974: i32 = (fTemp973) as i32;
			let mut iTemp975: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp974, 65535)))), 196607));
			let mut fTemp976: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp975, 3)) as usize] };
			let mut fTemp977: F64 = unsafe { ftbl0mydspSIG0[iTemp975 as usize] };
			let mut fTemp978: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp975, 1)) as usize] } - fTemp977;
			let mut fTemp979: F64 = 65535.0 * fTemp972;
			let mut iTemp980: i32 = (fTemp979) as i32;
			let mut iTemp981: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp980, 65535)))), 196607));
			let mut fTemp982: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp981, 3), 196607))) as usize] };
			let mut fTemp983: F64 = unsafe { ftbl0mydspSIG0[iTemp981 as usize] };
			let mut fTemp984: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp981, 1), 196607))) as usize] } - fTemp983;
			let mut iTemp985: i32 = (fTemp705 > ((if iTemp635 != 0 {fTemp983 + fTemp644 * fTemp984 + (fTemp979 - (iTemp980) as F64) * (fTemp982 - (fTemp983 + fTemp644 * (fTemp984 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp981, 4), 196607))) as usize] } - fTemp982))))} else {1.0 - (fTemp977 + fTemp644 * fTemp978 + (fTemp973 - (iTemp974) as F64) * (fTemp976 - (fTemp977 + fTemp644 * (fTemp978 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp975, 4)) as usize] } - fTemp976)))))} - fTemp971) / (1.0 - fTemp971))) as i32;
			let mut fTemp986: F64 = if iTemp985 != 0 {fTemp955} else {fTemp958};
			let mut fTemp987: F64 = if iTemp985 != 0 {fTemp958} else {fTemp956};
			let mut fTemp988: F64 = fTemp987 + fTemp986;
			let mut fTemp989: F64 = 0.5 * fTemp988;
			let mut fTemp990: F64 = 65535.0 * (1.0 - fTemp989);
			let mut iTemp991: i32 = (fTemp990) as i32;
			let mut iTemp992: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp991, 65535)))), 196607));
			let mut fTemp993: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp992, 3)) as usize] };
			let mut fTemp994: F64 = unsafe { ftbl0mydspSIG0[iTemp992 as usize] };
			let mut fTemp995: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp992, 1)) as usize] } - fTemp994;
			let mut fTemp996: F64 = 32767.5 * fTemp988;
			let mut iTemp997: i32 = (fTemp996) as i32;
			let mut iTemp998: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp997, 65535)))), 196607));
			let mut fTemp999: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp998, 3)) as usize] };
			let mut fTemp1000: F64 = unsafe { ftbl0mydspSIG0[iTemp998 as usize] };
			let mut fTemp1001: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp998, 1)) as usize] } - fTemp1000;
			let mut fTemp1002: F64 = if iTemp635 != 0 {fTemp1000 + fTemp644 * fTemp1001 + (fTemp996 - (iTemp997) as F64) * (fTemp999 - (fTemp1000 + fTemp644 * (fTemp1001 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp998, 4)) as usize] } - fTemp999))))} else {1.0 - (fTemp994 + fTemp644 * fTemp995 + (fTemp990 - (iTemp991) as F64) * (fTemp993 - (fTemp994 + fTemp644 * (fTemp995 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp992, 4)) as usize] } - fTemp993)))))};
			let mut fTemp1003: F64 = fTemp649 + fTemp989;
			let mut fTemp1004: F64 = 65535.0 * (1.0 - fTemp1003);
			let mut iTemp1005: i32 = (fTemp1004) as i32;
			let mut iTemp1006: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1005, 65535)))), 196607));
			let mut fTemp1007: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1006, 3)) as usize] };
			let mut fTemp1008: F64 = unsafe { ftbl0mydspSIG0[iTemp1006 as usize] };
			let mut fTemp1009: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1006, 1)) as usize] } - fTemp1008;
			let mut fTemp1010: F64 = 65535.0 * fTemp1003;
			let mut iTemp1011: i32 = (fTemp1010) as i32;
			let mut iTemp1012: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1011, 65535)))), 196607));
			let mut fTemp1013: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1012, 3), 196607))) as usize] };
			let mut fTemp1014: F64 = unsafe { ftbl0mydspSIG0[iTemp1012 as usize] };
			let mut fTemp1015: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1012, 1), 196607))) as usize] } - fTemp1014;
			let mut iTemp1016: i32 = (fTemp705 > ((if iTemp635 != 0 {fTemp1014 + fTemp644 * fTemp1015 + (fTemp1010 - (iTemp1011) as F64) * (fTemp1013 - (fTemp1014 + fTemp644 * (fTemp1015 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1012, 4), 196607))) as usize] } - fTemp1013))))} else {1.0 - (fTemp1008 + fTemp644 * fTemp1009 + (fTemp1004 - (iTemp1005) as F64) * (fTemp1007 - (fTemp1008 + fTemp644 * (fTemp1009 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1006, 4)) as usize] } - fTemp1007)))))} - fTemp1002) / (1.0 - fTemp1002))) as i32;
			let mut fTemp1017: F64 = if iTemp1016 != 0 {fTemp986} else {fTemp989};
			let mut fTemp1018: F64 = if iTemp1016 != 0 {fTemp989} else {fTemp987};
			let mut fTemp1019: F64 = fTemp1018 + fTemp1017;
			let mut fTemp1020: F64 = 0.5 * fTemp1019;
			let mut fTemp1021: F64 = 65535.0 * (1.0 - fTemp1020);
			let mut iTemp1022: i32 = (fTemp1021) as i32;
			let mut iTemp1023: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1022, 65535)))), 196607));
			let mut fTemp1024: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1023, 3)) as usize] };
			let mut fTemp1025: F64 = unsafe { ftbl0mydspSIG0[iTemp1023 as usize] };
			let mut fTemp1026: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1023, 1)) as usize] } - fTemp1025;
			let mut fTemp1027: F64 = 32767.5 * fTemp1019;
			let mut iTemp1028: i32 = (fTemp1027) as i32;
			let mut iTemp1029: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1028, 65535)))), 196607));
			let mut fTemp1030: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1029, 3)) as usize] };
			let mut fTemp1031: F64 = unsafe { ftbl0mydspSIG0[iTemp1029 as usize] };
			let mut fTemp1032: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1029, 1)) as usize] } - fTemp1031;
			let mut fTemp1033: F64 = if iTemp635 != 0 {fTemp1031 + fTemp644 * fTemp1032 + (fTemp1027 - (iTemp1028) as F64) * (fTemp1030 - (fTemp1031 + fTemp644 * (fTemp1032 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1029, 4)) as usize] } - fTemp1030))))} else {1.0 - (fTemp1025 + fTemp644 * fTemp1026 + (fTemp1021 - (iTemp1022) as F64) * (fTemp1024 - (fTemp1025 + fTemp644 * (fTemp1026 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1023, 4)) as usize] } - fTemp1024)))))};
			let mut fTemp1034: F64 = fTemp649 + fTemp1020;
			let mut fTemp1035: F64 = 65535.0 * (1.0 - fTemp1034);
			let mut iTemp1036: i32 = (fTemp1035) as i32;
			let mut iTemp1037: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1036, 65535)))), 196607));
			let mut fTemp1038: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1037, 3)) as usize] };
			let mut fTemp1039: F64 = unsafe { ftbl0mydspSIG0[iTemp1037 as usize] };
			let mut fTemp1040: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1037, 1)) as usize] } - fTemp1039;
			let mut fTemp1041: F64 = 65535.0 * fTemp1034;
			let mut iTemp1042: i32 = (fTemp1041) as i32;
			let mut iTemp1043: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1042, 65535)))), 196607));
			let mut fTemp1044: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1043, 3), 196607))) as usize] };
			let mut fTemp1045: F64 = unsafe { ftbl0mydspSIG0[iTemp1043 as usize] };
			let mut fTemp1046: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1043, 1), 196607))) as usize] } - fTemp1045;
			let mut iTemp1047: i32 = (fTemp705 > ((if iTemp635 != 0 {fTemp1045 + fTemp644 * fTemp1046 + (fTemp1041 - (iTemp1042) as F64) * (fTemp1044 - (fTemp1045 + fTemp644 * (fTemp1046 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1043, 4), 196607))) as usize] } - fTemp1044))))} else {1.0 - (fTemp1039 + fTemp644 * fTemp1040 + (fTemp1035 - (iTemp1036) as F64) * (fTemp1038 - (fTemp1039 + fTemp644 * (fTemp1040 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1037, 4)) as usize] } - fTemp1038)))))} - fTemp1033) / (1.0 - fTemp1033))) as i32;
			let mut fTemp1048: F64 = if iTemp1047 != 0 {fTemp1017} else {fTemp1020};
			let mut fTemp1049: F64 = if iTemp1047 != 0 {fTemp1020} else {fTemp1018};
			let mut fTemp1050: F64 = fTemp1049 + fTemp1048;
			let mut fTemp1051: F64 = 0.5 * fTemp1050;
			let mut fTemp1052: F64 = 65535.0 * (1.0 - fTemp1051);
			let mut iTemp1053: i32 = (fTemp1052) as i32;
			let mut iTemp1054: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1053, 65535)))), 196607));
			let mut fTemp1055: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1054, 3)) as usize] };
			let mut fTemp1056: F64 = unsafe { ftbl0mydspSIG0[iTemp1054 as usize] };
			let mut fTemp1057: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1054, 1)) as usize] } - fTemp1056;
			let mut fTemp1058: F64 = 32767.5 * fTemp1050;
			let mut iTemp1059: i32 = (fTemp1058) as i32;
			let mut iTemp1060: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1059, 65535)))), 196607));
			let mut fTemp1061: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1060, 3)) as usize] };
			let mut fTemp1062: F64 = unsafe { ftbl0mydspSIG0[iTemp1060 as usize] };
			let mut fTemp1063: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1060, 1)) as usize] } - fTemp1062;
			let mut fTemp1064: F64 = if iTemp635 != 0 {fTemp1062 + fTemp644 * fTemp1063 + (fTemp1058 - (iTemp1059) as F64) * (fTemp1061 - (fTemp1062 + fTemp644 * (fTemp1063 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1060, 4)) as usize] } - fTemp1061))))} else {1.0 - (fTemp1056 + fTemp644 * fTemp1057 + (fTemp1052 - (iTemp1053) as F64) * (fTemp1055 - (fTemp1056 + fTemp644 * (fTemp1057 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1054, 4)) as usize] } - fTemp1055)))))};
			let mut fTemp1065: F64 = fTemp649 + fTemp1051;
			let mut fTemp1066: F64 = 65535.0 * (1.0 - fTemp1065);
			let mut iTemp1067: i32 = (fTemp1066) as i32;
			let mut iTemp1068: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1067, 65535)))), 196607));
			let mut fTemp1069: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1068, 3)) as usize] };
			let mut fTemp1070: F64 = unsafe { ftbl0mydspSIG0[iTemp1068 as usize] };
			let mut fTemp1071: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1068, 1)) as usize] } - fTemp1070;
			let mut fTemp1072: F64 = 65535.0 * fTemp1065;
			let mut iTemp1073: i32 = (fTemp1072) as i32;
			let mut iTemp1074: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1073, 65535)))), 196607));
			let mut fTemp1075: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1074, 3), 196607))) as usize] };
			let mut fTemp1076: F64 = unsafe { ftbl0mydspSIG0[iTemp1074 as usize] };
			let mut fTemp1077: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1074, 1), 196607))) as usize] } - fTemp1076;
			let mut iTemp1078: i32 = (fTemp705 > ((if iTemp635 != 0 {fTemp1076 + fTemp644 * fTemp1077 + (fTemp1072 - (iTemp1073) as F64) * (fTemp1075 - (fTemp1076 + fTemp644 * (fTemp1077 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1074, 4), 196607))) as usize] } - fTemp1075))))} else {1.0 - (fTemp1070 + fTemp644 * fTemp1071 + (fTemp1066 - (iTemp1067) as F64) * (fTemp1069 - (fTemp1070 + fTemp644 * (fTemp1071 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1068, 4)) as usize] } - fTemp1069)))))} - fTemp1064) / (1.0 - fTemp1064))) as i32;
			let mut fTemp1079: F64 = if iTemp1078 != 0 {fTemp1048} else {fTemp1051};
			let mut fTemp1080: F64 = if iTemp1078 != 0 {fTemp1051} else {fTemp1049};
			let mut fTemp1081: F64 = fTemp1080 + fTemp1079;
			let mut fTemp1082: F64 = 0.5 * fTemp1081;
			let mut fTemp1083: F64 = 65535.0 * (1.0 - fTemp1082);
			let mut iTemp1084: i32 = (fTemp1083) as i32;
			let mut iTemp1085: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1084, 65535)))), 196607));
			let mut fTemp1086: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1085, 3)) as usize] };
			let mut fTemp1087: F64 = unsafe { ftbl0mydspSIG0[iTemp1085 as usize] };
			let mut fTemp1088: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1085, 1)) as usize] } - fTemp1087;
			let mut fTemp1089: F64 = 32767.5 * fTemp1081;
			let mut iTemp1090: i32 = (fTemp1089) as i32;
			let mut iTemp1091: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1090, 65535)))), 196607));
			let mut fTemp1092: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1091, 3)) as usize] };
			let mut fTemp1093: F64 = unsafe { ftbl0mydspSIG0[iTemp1091 as usize] };
			let mut fTemp1094: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1091, 1)) as usize] } - fTemp1093;
			let mut fTemp1095: F64 = if iTemp635 != 0 {fTemp1093 + fTemp644 * fTemp1094 + (fTemp1089 - (iTemp1090) as F64) * (fTemp1092 - (fTemp1093 + fTemp644 * (fTemp1094 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1091, 4)) as usize] } - fTemp1092))))} else {1.0 - (fTemp1087 + fTemp644 * fTemp1088 + (fTemp1083 - (iTemp1084) as F64) * (fTemp1086 - (fTemp1087 + fTemp644 * (fTemp1088 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1085, 4)) as usize] } - fTemp1086)))))};
			let mut fTemp1096: F64 = fTemp649 + fTemp1082;
			let mut fTemp1097: F64 = 65535.0 * (1.0 - fTemp1096);
			let mut iTemp1098: i32 = (fTemp1097) as i32;
			let mut iTemp1099: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1098, 65535)))), 196607));
			let mut fTemp1100: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1099, 3)) as usize] };
			let mut fTemp1101: F64 = unsafe { ftbl0mydspSIG0[iTemp1099 as usize] };
			let mut fTemp1102: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1099, 1)) as usize] } - fTemp1101;
			let mut fTemp1103: F64 = 65535.0 * fTemp1096;
			let mut iTemp1104: i32 = (fTemp1103) as i32;
			let mut iTemp1105: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1104, 65535)))), 196607));
			let mut fTemp1106: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1105, 3), 196607))) as usize] };
			let mut fTemp1107: F64 = unsafe { ftbl0mydspSIG0[iTemp1105 as usize] };
			let mut fTemp1108: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1105, 1), 196607))) as usize] } - fTemp1107;
			let mut iTemp1109: i32 = (fTemp705 > ((if iTemp635 != 0 {fTemp1107 + fTemp644 * fTemp1108 + (fTemp1103 - (iTemp1104) as F64) * (fTemp1106 - (fTemp1107 + fTemp644 * (fTemp1108 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1105, 4), 196607))) as usize] } - fTemp1106))))} else {1.0 - (fTemp1101 + fTemp644 * fTemp1102 + (fTemp1097 - (iTemp1098) as F64) * (fTemp1100 - (fTemp1101 + fTemp644 * (fTemp1102 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1099, 4)) as usize] } - fTemp1100)))))} - fTemp1095) / (1.0 - fTemp1095))) as i32;
			let mut fTemp1110: F64 = if iTemp1109 != 0 {fTemp1079} else {fTemp1082};
			let mut fTemp1111: F64 = if iTemp1109 != 0 {fTemp1082} else {fTemp1080};
			let mut fTemp1112: F64 = fTemp1111 + fTemp1110;
			let mut fTemp1113: F64 = 0.5 * fTemp1112;
			let mut fTemp1114: F64 = 65535.0 * (1.0 - fTemp1113);
			let mut iTemp1115: i32 = (fTemp1114) as i32;
			let mut iTemp1116: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1115, 65535)))), 196607));
			let mut fTemp1117: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1116, 3)) as usize] };
			let mut fTemp1118: F64 = unsafe { ftbl0mydspSIG0[iTemp1116 as usize] };
			let mut fTemp1119: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1116, 1)) as usize] } - fTemp1118;
			let mut fTemp1120: F64 = 32767.5 * fTemp1112;
			let mut iTemp1121: i32 = (fTemp1120) as i32;
			let mut iTemp1122: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1121, 65535)))), 196607));
			let mut fTemp1123: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1122, 3)) as usize] };
			let mut fTemp1124: F64 = unsafe { ftbl0mydspSIG0[iTemp1122 as usize] };
			let mut fTemp1125: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1122, 1)) as usize] } - fTemp1124;
			let mut fTemp1126: F64 = if iTemp635 != 0 {fTemp1124 + fTemp644 * fTemp1125 + (fTemp1120 - (iTemp1121) as F64) * (fTemp1123 - (fTemp1124 + fTemp644 * (fTemp1125 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1122, 4)) as usize] } - fTemp1123))))} else {1.0 - (fTemp1118 + fTemp644 * fTemp1119 + (fTemp1114 - (iTemp1115) as F64) * (fTemp1117 - (fTemp1118 + fTemp644 * (fTemp1119 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1116, 4)) as usize] } - fTemp1117)))))};
			let mut fTemp1127: F64 = fTemp649 + fTemp1113;
			let mut fTemp1128: F64 = 65535.0 * (1.0 - fTemp1127);
			let mut iTemp1129: i32 = (fTemp1128) as i32;
			let mut iTemp1130: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1129, 65535)))), 196607));
			let mut fTemp1131: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1130, 3)) as usize] };
			let mut fTemp1132: F64 = unsafe { ftbl0mydspSIG0[iTemp1130 as usize] };
			let mut fTemp1133: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1130, 1)) as usize] } - fTemp1132;
			let mut fTemp1134: F64 = 65535.0 * fTemp1127;
			let mut iTemp1135: i32 = (fTemp1134) as i32;
			let mut iTemp1136: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1135, 65535)))), 196607));
			let mut fTemp1137: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1136, 3), 196607))) as usize] };
			let mut fTemp1138: F64 = unsafe { ftbl0mydspSIG0[iTemp1136 as usize] };
			let mut fTemp1139: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1136, 1), 196607))) as usize] } - fTemp1138;
			let mut iTemp1140: i32 = (fTemp705 > ((if iTemp635 != 0 {fTemp1138 + fTemp644 * fTemp1139 + (fTemp1134 - (iTemp1135) as F64) * (fTemp1137 - (fTemp1138 + fTemp644 * (fTemp1139 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1136, 4), 196607))) as usize] } - fTemp1137))))} else {1.0 - (fTemp1132 + fTemp644 * fTemp1133 + (fTemp1128 - (iTemp1129) as F64) * (fTemp1131 - (fTemp1132 + fTemp644 * (fTemp1133 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1130, 4)) as usize] } - fTemp1131)))))} - fTemp1126) / (1.0 - fTemp1126))) as i32;
			let mut fTemp1141: F64 = if iTemp1140 != 0 {fTemp1110} else {fTemp1113};
			let mut fTemp1142: F64 = if iTemp1140 != 0 {fTemp1113} else {fTemp1111};
			let mut fTemp1143: F64 = fTemp1142 + fTemp1141;
			let mut fTemp1144: F64 = 0.5 * fTemp1143;
			let mut fTemp1145: F64 = 65535.0 * (1.0 - fTemp1144);
			let mut iTemp1146: i32 = (fTemp1145) as i32;
			let mut iTemp1147: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1146, 65535)))), 196607));
			let mut fTemp1148: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1147, 3)) as usize] };
			let mut fTemp1149: F64 = unsafe { ftbl0mydspSIG0[iTemp1147 as usize] };
			let mut fTemp1150: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1147, 1)) as usize] } - fTemp1149;
			let mut fTemp1151: F64 = 32767.5 * fTemp1143;
			let mut iTemp1152: i32 = (fTemp1151) as i32;
			let mut iTemp1153: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1152, 65535)))), 196607));
			let mut fTemp1154: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1153, 3)) as usize] };
			let mut fTemp1155: F64 = unsafe { ftbl0mydspSIG0[iTemp1153 as usize] };
			let mut fTemp1156: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1153, 1)) as usize] } - fTemp1155;
			let mut fTemp1157: F64 = if iTemp635 != 0 {fTemp1155 + fTemp644 * fTemp1156 + (fTemp1151 - (iTemp1152) as F64) * (fTemp1154 - (fTemp1155 + fTemp644 * (fTemp1156 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1153, 4), 196607))) as usize] } - fTemp1154))))} else {1.0 - (fTemp1149 + fTemp644 * fTemp1150 + (fTemp1145 - (iTemp1146) as F64) * (fTemp1148 - (fTemp1149 + fTemp644 * (fTemp1150 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1147, 4), 196607))) as usize] } - fTemp1148)))))};
			let mut fTemp1158: F64 = fTemp649 + fTemp1144;
			let mut fTemp1159: F64 = 65535.0 * (1.0 - fTemp1158);
			let mut iTemp1160: i32 = (fTemp1159) as i32;
			let mut iTemp1161: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1160, 65535)))), 196607));
			let mut fTemp1162: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1161, 3)) as usize] };
			let mut fTemp1163: F64 = unsafe { ftbl0mydspSIG0[iTemp1161 as usize] };
			let mut fTemp1164: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1161, 1)) as usize] } - fTemp1163;
			let mut fTemp1165: F64 = 65535.0 * fTemp1158;
			let mut iTemp1166: i32 = (fTemp1165) as i32;
			let mut iTemp1167: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1166, 65535)))), 196607));
			let mut fTemp1168: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1167, 3), 196607))) as usize] };
			let mut fTemp1169: F64 = unsafe { ftbl0mydspSIG0[iTemp1167 as usize] };
			let mut fTemp1170: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1167, 1), 196607))) as usize] } - fTemp1169;
			let mut iTemp1171: i32 = (fTemp705 > ((if iTemp635 != 0 {fTemp1169 + fTemp644 * fTemp1170 + (fTemp1165 - (iTemp1166) as F64) * (fTemp1168 - (fTemp1169 + fTemp644 * (fTemp1170 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1167, 4), 196607))) as usize] } - fTemp1168))))} else {1.0 - (fTemp1163 + fTemp644 * fTemp1164 + (fTemp1159 - (iTemp1160) as F64) * (fTemp1162 - (fTemp1163 + fTemp644 * (fTemp1164 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1161, 4)) as usize] } - fTemp1162)))))} - fTemp1157) / (1.0 - fTemp1157))) as i32;
			let mut fTemp1172: F64 = F64::min(1.0, F64::max(0.0, 0.5 * (if iTemp1171 != 0 {fTemp1144} else {fTemp1142} + if iTemp1171 != 0 {fTemp1141} else {fTemp1144})));
			self.fRec7[0] = fTemp1172;
			let mut fTemp1173: F64 = 65535.0 * (1.0 - fTemp1172);
			let mut iTemp1174: i32 = (fTemp1173) as i32;
			let mut iTemp1175: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1174, 65535)))), 196607));
			let mut fTemp1176: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1175, 3)) as usize] };
			let mut fTemp1177: F64 = unsafe { ftbl0mydspSIG0[iTemp1175 as usize] };
			let mut fTemp1178: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1175, 1)) as usize] } - fTemp1177;
			let mut fTemp1179: F64 = 65535.0 * fTemp1172;
			let mut iTemp1180: i32 = (fTemp1179) as i32;
			let mut iTemp1181: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1180, 65535)))), 196607));
			let mut fTemp1182: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1181, 3)) as usize] };
			let mut fTemp1183: F64 = unsafe { ftbl0mydspSIG0[iTemp1181 as usize] };
			let mut fTemp1184: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1181, 1)) as usize] } - fTemp1183;
			let mut fTemp1185: F64 = if iTemp635 != 0 {fTemp1183 + fTemp644 * fTemp1184 + (fTemp1179 - (iTemp1180) as F64) * (fTemp1182 - (fTemp1183 + fTemp644 * (fTemp1184 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1181, 4), 196607))) as usize] } - fTemp1182))))} else {1.0 - (fTemp1177 + fTemp644 * fTemp1178 + (fTemp1173 - (iTemp1174) as F64) * (fTemp1176 - (fTemp1177 + fTemp644 * (fTemp1178 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1175, 4), 196607))) as usize] } - fTemp1176)))))};
			let mut fTemp1186: F64 = fTemp649 + fTemp1172;
			let mut fTemp1187: F64 = 65535.0 * (1.0 - fTemp1186);
			let mut iTemp1188: i32 = (fTemp1187) as i32;
			let mut iTemp1189: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1188, 65535)))), 196607));
			let mut fTemp1190: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1189, 3)) as usize] };
			let mut fTemp1191: F64 = unsafe { ftbl0mydspSIG0[iTemp1189 as usize] };
			let mut fTemp1192: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1189, 1)) as usize] } - fTemp1191;
			let mut fTemp1193: F64 = 65535.0 * fTemp1186;
			let mut iTemp1194: i32 = (fTemp1193) as i32;
			let mut iTemp1195: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp639, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1194, 65535)))), 196607));
			let mut fTemp1196: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1195, 3), 196607))) as usize] };
			let mut fTemp1197: F64 = unsafe { ftbl0mydspSIG0[iTemp1195 as usize] };
			let mut fTemp1198: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1195, 1), 196607))) as usize] } - fTemp1197;
			let mut fTemp1199: F64 = self.fRec8[1] + if ((0.001 * fTemp648) == 0.0) as i32 != 0 {fTemp634} else {fTemp634 * (if iTemp635 != 0 {fTemp1197 + fTemp644 * fTemp1198 + (fTemp1193 - (iTemp1194) as F64) * (fTemp1196 - (fTemp1197 + fTemp644 * (fTemp1198 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1195, 4), 196607))) as usize] } - fTemp1196))))} else {1.0 - (fTemp1191 + fTemp644 * fTemp1192 + (fTemp1187 - (iTemp1188) as F64) * (fTemp1190 - (fTemp1191 + fTemp644 * (fTemp1192 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1189, 4)) as usize] } - fTemp1190)))))} - fTemp1185) / (1.0 - fTemp1185)};
			self.fRec8[0] = F64::min(self.fRec9[((i32::wrapping_sub(self.IOTA0, iSlow66)) & 16383) as usize], if iTemp647 != 0 {F64::min(fTemp1199, self.fRec8[1])} else {F64::max(fTemp1199, self.fRec8[1])});
			self.fVbargraph1 = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, self.fRec8[0]));
			*output1 = 0.5 * fTemp2 * self.fVec2[((i32::wrapping_sub(self.IOTA0, iSlow4)) & 32767) as usize] + self.fRec2[0] * self.fRec8[0] * fTemp3 * self.fVec3[((i32::wrapping_sub(self.IOTA0, iSlow4)) & 32767) as usize];
			self.fRec0[1] = self.fRec0[0];
			self.fRec1[1] = self.fRec1[0];
			self.IOTA0 = i32::wrapping_add(self.IOTA0, 1);
			self.fRec2[1] = self.fRec2[0];
			self.fVec5[2] = self.fVec5[1];
			self.fVec5[1] = self.fVec5[0];
			for j0 in (1..=6).rev() {
				self.fVec6[j0 as usize] = self.fVec6[(i32::wrapping_sub(j0, 1)) as usize];
			}
			for j1 in (1..=14).rev() {
				self.fVec7[j1 as usize] = self.fVec7[(i32::wrapping_sub(j1, 1)) as usize];
			}
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
			self.fRec3[1] = self.fRec3[0];
			self.fRec4[1] = self.fRec4[0];
			self.fVec31[2] = self.fVec31[1];
			self.fVec31[1] = self.fVec31[0];
			for j4 in (1..=6).rev() {
				self.fVec32[j4 as usize] = self.fVec32[(i32::wrapping_sub(j4, 1)) as usize];
			}
			for j5 in (1..=14).rev() {
				self.fVec33[j5 as usize] = self.fVec33[(i32::wrapping_sub(j5, 1)) as usize];
			}
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
			self.fRec7[1] = self.fRec7[0];
			self.fRec8[1] = self.fRec8[0];
		}
	}

}


}
pub use dsp::mydsp as LambRs;
