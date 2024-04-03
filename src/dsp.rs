mod dsp {
    /* ------------------------------------------------------------
author: "Bart Brouns"
license: "AGPLv3"
name: "lamb-rs"
version: "0.1"
Code generated with Faust 2.72.14 (https://faust.grame.fr)
Compilation options: -a /run/user/1001/.tmpMNww7d -lang rust -ct 1 -es 1 -mcd 16 -mdd 1024 -mdy 33 -double -ftz 0
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
	iRec5: [i32;2],
}

impl mydspSIG0 {
	
	fn get_num_inputsmydspSIG0(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsmydspSIG0(&self) -> i32 {
		return 1;
	}
	
	fn instance_initmydspSIG0(&mut self, sample_rate: i32) {
		for l31 in 0..2 {
			self.iRec5[l31 as usize] = 0;
		}
	}
	
	fn fillmydspSIG0(&mut self, count: i32, table: &mut[F64]) {
		for i1 in 0..count {
			self.iRec5[0] = i32::wrapping_add(self.iRec5[1], 1);
			let mut iTemp40: i32 = i32::wrapping_add(self.iRec5[0], -1);
			let mut fTemp41: F64 = (iTemp40 % 3) as F64 as i32 as F64;
			let mut fTemp42: F64 = 0.5 * fTemp41;
			let mut fTemp43: F64 = F64::powf(fTemp42, 0.21 * fTemp41 + 1.0);
			let mut fTemp44: F64 = (0.3333333333333333 * (iTemp40 % 196608) as F64) as i32 as F64;
			table[i1 as usize] = F64::min(1.0, F64::max(0.0, if (fTemp42 == 0.0) as i32 != 0 {0.5 * (F64::sin(4.793763109162727e-05 * fTemp44 + 4.71238898038469) + 1.0)} else {0.5 * (F64::sin(3.141592653589793 * ((1.0 - F64::exp(-(3.692683299000534e-05 * fTemp43 * fTemp44))) / (1.0 - F64::exp(-(2.42 * fTemp43)))) + 4.71238898038469) + 1.0)}));
			self.iRec5[1] = self.iRec5[0];
		}
	}

}


pub fn newmydspSIG0() -> mydspSIG0 { 
	mydspSIG0 {
		iRec5: [0;2],
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
	fConst2: F64,
	fHslider1: F64,
	fHslider2: F64,
	fConst3: F64,
	fConst4: F64,
	fHslider3: F64,
	fRec4: [F64;2],
	fVec0: [F64;32768],
	fVec1: [F64;32768],
	fVec2: [F64;32768],
	fVec3: [F64;32768],
	fHslider4: F64,
	fHslider5: F64,
	fVec4: [F64;16384],
	fHslider6: F64,
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
	fRec3: [F64;16384],
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
	fHslider7: F64,
	fHslider8: F64,
	fVec28: [F64;2],
	fHslider9: F64,
	fVec29: [F64;2],
	fConst5: F64,
	fRec1: [F64;2],
	fRec2: [F64;8192],
	fCheckbox1: F64,
	fVbargraph0: F64,
	fHbargraph0: F64,
	fHslider10: F64,
	fRec6: [F64;2],
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
	fRec8: [F64;8192],
	fVbargraph1: F64,
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
			fConst2: 0.0,
			fHslider1: 0.0,
			fHslider2: 0.0,
			fConst3: 0.0,
			fConst4: 0.0,
			fHslider3: 0.0,
			fRec4: [0.0;2],
			fVec0: [0.0;32768],
			fVec1: [0.0;32768],
			fVec2: [0.0;32768],
			fVec3: [0.0;32768],
			fHslider4: 0.0,
			fHslider5: 0.0,
			fVec4: [0.0;16384],
			fHslider6: 0.0,
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
			fRec3: [0.0;16384],
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
			fHslider7: 0.0,
			fHslider8: 0.0,
			fVec28: [0.0;2],
			fHslider9: 0.0,
			fVec29: [0.0;2],
			fConst5: 0.0,
			fRec1: [0.0;2],
			fRec2: [0.0;8192],
			fCheckbox1: 0.0,
			fVbargraph0: 0.0,
			fHbargraph0: 0.0,
			fHslider10: 0.0,
			fRec6: [0.0;2],
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
			fRec8: [0.0;8192],
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
		m.declare("compile_options", r"-a /run/user/1001/.tmpMNww7d -lang rust -ct 1 -es 1 -mcd 16 -mdd 1024 -mdy 33 -double -ftz 0");
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
		self.fHslider0 = 9.0;
		self.fHslider1 = 1.0;
		self.fHslider2 = -1.0;
		self.fHslider3 = 0.0;
		self.fHslider4 = 0.0;
		self.fHslider5 = 1e+02;
		self.fHslider6 = 5e+01;
		self.fHslider7 = 0.0;
		self.fHslider8 = 0.5;
		self.fHslider9 = 6e+01;
		self.fCheckbox1 = 0.0;
		self.fHslider10 = 0.0;
	}
	fn instance_clear(&mut self) {
		for l0 in 0..2 {
			self.fRec0[l0 as usize] = 0.0;
		}
		self.IOTA0 = 0;
		for l1 in 0..2 {
			self.fRec4[l1 as usize] = 0.0;
		}
		for l2 in 0..32768 {
			self.fVec0[l2 as usize] = 0.0;
		}
		for l3 in 0..32768 {
			self.fVec1[l3 as usize] = 0.0;
		}
		for l4 in 0..32768 {
			self.fVec2[l4 as usize] = 0.0;
		}
		for l5 in 0..32768 {
			self.fVec3[l5 as usize] = 0.0;
		}
		for l6 in 0..16384 {
			self.fVec4[l6 as usize] = 0.0;
		}
		for l7 in 0..3 {
			self.fVec5[l7 as usize] = 0.0;
		}
		for l8 in 0..7 {
			self.fVec6[l8 as usize] = 0.0;
		}
		for l9 in 0..15 {
			self.fVec7[l9 as usize] = 0.0;
		}
		for l10 in 0..32 {
			self.fVec8[l10 as usize] = 0.0;
		}
		for l11 in 0..64 {
			self.fVec9[l11 as usize] = 0.0;
		}
		for l12 in 0..128 {
			self.fVec10[l12 as usize] = 0.0;
		}
		for l13 in 0..256 {
			self.fVec11[l13 as usize] = 0.0;
		}
		for l14 in 0..512 {
			self.fVec12[l14 as usize] = 0.0;
		}
		for l15 in 0..1024 {
			self.fVec13[l15 as usize] = 0.0;
		}
		for l16 in 0..2048 {
			self.fVec14[l16 as usize] = 0.0;
		}
		for l17 in 0..4096 {
			self.fVec15[l17 as usize] = 0.0;
		}
		for l18 in 0..16384 {
			self.fRec3[l18 as usize] = 0.0;
		}
		for l19 in 0..3 {
			self.fVec16[l19 as usize] = 0.0;
		}
		for l20 in 0..7 {
			self.fVec17[l20 as usize] = 0.0;
		}
		for l21 in 0..15 {
			self.fVec18[l21 as usize] = 0.0;
		}
		for l22 in 0..32 {
			self.fVec19[l22 as usize] = 0.0;
		}
		for l23 in 0..64 {
			self.fVec20[l23 as usize] = 0.0;
		}
		for l24 in 0..128 {
			self.fVec21[l24 as usize] = 0.0;
		}
		for l25 in 0..256 {
			self.fVec22[l25 as usize] = 0.0;
		}
		for l26 in 0..512 {
			self.fVec23[l26 as usize] = 0.0;
		}
		for l27 in 0..1024 {
			self.fVec24[l27 as usize] = 0.0;
		}
		for l28 in 0..2048 {
			self.fVec25[l28 as usize] = 0.0;
		}
		for l29 in 0..4096 {
			self.fVec26[l29 as usize] = 0.0;
		}
		for l30 in 0..2 {
			self.fVec27[l30 as usize] = 0.0;
		}
		for l32 in 0..2 {
			self.fVec28[l32 as usize] = 0.0;
		}
		for l33 in 0..2 {
			self.fVec29[l33 as usize] = 0.0;
		}
		for l34 in 0..2 {
			self.fRec1[l34 as usize] = 0.0;
		}
		for l35 in 0..8192 {
			self.fRec2[l35 as usize] = 0.0;
		}
		for l36 in 0..2 {
			self.fRec6[l36 as usize] = 0.0;
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
		for l65 in 0..8192 {
			self.fRec8[l65 as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		self.fConst0 = F64::min(1.92e+05, F64::max(1.0, (self.fSampleRate) as F64));
		self.fConst1 = 1e+02 / self.fConst0;
		self.fConst2 = 0.001 * self.fConst0;
		self.fConst3 = 44.1 / self.fConst0;
		self.fConst4 = 1.0 - self.fConst3;
		self.fConst5 = 1e+03 / self.fConst0;
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
		ui_interface.open_horizontal_box("0x00");
		ui_interface.declare(Some(ParamIndex(0)), "00", "");
		ui_interface.add_check_button("bypass", ParamIndex(0));
		ui_interface.declare(Some(ParamIndex(1)), "01", "");
		ui_interface.add_check_button("fixed latency", ParamIndex(1));
		ui_interface.close_box();
		ui_interface.declare(Some(ParamIndex(2)), "01", "");
		ui_interface.add_horizontal_slider("input gain", ParamIndex(2), 0.0, -24.0, 24.0, 0.1);
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
		ui_interface.add_horizontal_slider("output gain", ParamIndex(12), 0.0, -24.0, 24.0, 0.1);
		ui_interface.close_box();
		ui_interface.declare(None, "99", "");
		ui_interface.open_horizontal_box("gain reduction");
		ui_interface.declare(Some(ParamIndex(13)), "unit", "dB");
		ui_interface.add_vertical_bargraph("0", ParamIndex(13), -24.0, 0.0);
		ui_interface.declare(Some(ParamIndex(14)), "unit", "dB");
		ui_interface.add_vertical_bargraph("1", ParamIndex(14), -24.0, 0.0);
		ui_interface.close_box();
		ui_interface.close_box();
		ui_interface.declare(Some(ParamIndex(15)), "99", "");
		ui_interface.declare(Some(ParamIndex(15)), "unit", "samples");
		ui_interface.add_horizontal_bargraph("latency", ParamIndex(15), 0.0, 4.8e+03);
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			0 => Some(self.fCheckbox0),
			1 => Some(self.fCheckbox1),
			15 => Some(self.fHbargraph0),
			5 => Some(self.fHslider0),
			10 => Some(self.fHslider1),
			12 => Some(self.fHslider10),
			4 => Some(self.fHslider2),
			2 => Some(self.fHslider3),
			11 => Some(self.fHslider4),
			3 => Some(self.fHslider5),
			9 => Some(self.fHslider6),
			6 => Some(self.fHslider7),
			8 => Some(self.fHslider8),
			7 => Some(self.fHslider9),
			13 => Some(self.fVbargraph0),
			14 => Some(self.fVbargraph1),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			0 => { self.fCheckbox0 = value }
			1 => { self.fCheckbox1 = value }
			15 => { self.fHbargraph0 = value }
			5 => { self.fHslider0 = value }
			10 => { self.fHslider1 = value }
			12 => { self.fHslider10 = value }
			4 => { self.fHslider2 = value }
			2 => { self.fHslider3 = value }
			11 => { self.fHslider4 = value }
			3 => { self.fHslider5 = value }
			9 => { self.fHslider6 = value }
			6 => { self.fHslider7 = value }
			8 => { self.fHslider8 = value }
			7 => { self.fHslider9 = value }
			13 => { self.fVbargraph0 = value }
			14 => { self.fVbargraph1 = value }
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
		let mut fSlow1: F64 = self.fHslider0;
		let mut fSlow2: F64 = self.fConst2 * fSlow1;
		let mut fSlow3: F64 = fSlow2 + 1.0;
		let mut iSlow4: i32 = (F64::floor(fSlow3)) as i32 % 2;
		let mut fSlow5: F64 = self.fHslider1;
		let mut fSlow6: F64 = 0.5 * fSlow5;
		let mut fSlow7: F64 = self.fHslider2;
		let mut fSlow8: F64 = fSlow7 + fSlow6;
		let mut fSlow9: F64 = self.fConst3 * F64::powf(1e+01, 0.05 * self.fHslider3);
		let mut fSlow10: F64 = 0.01 * self.fHslider4;
		let mut fSlow11: F64 = fSlow7 - fSlow6;
		let mut fSlow12: F64 = 0.5 / F64::max(2.220446049250313e-16, fSlow5);
		let mut fSlow13: F64 = 0.0005 * self.fHslider5;
		let mut fSlow14: F64 = self.fHslider6;
		let mut fSlow15: F64 = self.fConst2 * fSlow14;
		let mut iSlow16: i32 = (fSlow15) as i32;
		let mut fSlow17: F64 = fSlow15 + 1.0;
		let mut iSlow18: i32 = (F64::floor(fSlow17)) as i32 % 2;
		let mut iSlow19: i32 = (F64::floor(0.5 * fSlow17)) as i32 % 2;
		let mut iSlow20: i32 = (F64::floor(0.25 * fSlow17)) as i32 % 2;
		let mut iSlow21: i32 = i32::wrapping_add(iSlow18, i32::wrapping_mul(2, iSlow19));
		let mut iSlow22: i32 = (F64::floor(0.125 * fSlow17)) as i32 % 2;
		let mut iSlow23: i32 = i32::wrapping_add(iSlow21, i32::wrapping_mul(4, iSlow20));
		let mut iSlow24: i32 = (F64::floor(0.0625 * fSlow17)) as i32 % 2;
		let mut iSlow25: i32 = i32::wrapping_add(iSlow23, i32::wrapping_mul(8, iSlow22));
		let mut iSlow26: i32 = (F64::floor(0.03125 * fSlow17)) as i32 % 2;
		let mut iSlow27: i32 = i32::wrapping_add(iSlow25, i32::wrapping_mul(16, iSlow24));
		let mut iSlow28: i32 = (F64::floor(0.015625 * fSlow17)) as i32 % 2;
		let mut iSlow29: i32 = i32::wrapping_add(iSlow27, i32::wrapping_mul(32, iSlow26));
		let mut iSlow30: i32 = (F64::floor(0.0078125 * fSlow17)) as i32 % 2;
		let mut iSlow31: i32 = i32::wrapping_add(iSlow29, i32::wrapping_mul(64, iSlow28));
		let mut iSlow32: i32 = (F64::floor(0.00390625 * fSlow17)) as i32 % 2;
		let mut iSlow33: i32 = i32::wrapping_add(iSlow31, i32::wrapping_mul(128, iSlow30));
		let mut iSlow34: i32 = (F64::floor(0.001953125 * fSlow17)) as i32 % 2;
		let mut iSlow35: i32 = i32::wrapping_add(iSlow33, i32::wrapping_mul(256, iSlow32));
		let mut iSlow36: i32 = (F64::floor(0.0009765625 * fSlow17)) as i32 % 2;
		let mut iSlow37: i32 = i32::wrapping_add(iSlow35, i32::wrapping_mul(512, iSlow34));
		let mut iSlow38: i32 = (F64::floor(0.00048828125 * fSlow17)) as i32 % 2;
		let mut iSlow39: i32 = i32::wrapping_add(iSlow37, i32::wrapping_mul(1024, iSlow36));
		let mut iSlow40: i32 = (F64::floor(0.5 * fSlow3)) as i32 % 2;
		let mut iSlow41: i32 = (F64::floor(0.25 * fSlow3)) as i32 % 2;
		let mut iSlow42: i32 = i32::wrapping_add(iSlow4, i32::wrapping_mul(2, iSlow40));
		let mut iSlow43: i32 = (F64::floor(0.125 * fSlow3)) as i32 % 2;
		let mut iSlow44: i32 = i32::wrapping_add(iSlow42, i32::wrapping_mul(4, iSlow41));
		let mut iSlow45: i32 = (F64::floor(0.0625 * fSlow3)) as i32 % 2;
		let mut iSlow46: i32 = i32::wrapping_add(iSlow44, i32::wrapping_mul(8, iSlow43));
		let mut iSlow47: i32 = (F64::floor(0.03125 * fSlow3)) as i32 % 2;
		let mut iSlow48: i32 = i32::wrapping_add(iSlow46, i32::wrapping_mul(16, iSlow45));
		let mut iSlow49: i32 = (F64::floor(0.015625 * fSlow3)) as i32 % 2;
		let mut iSlow50: i32 = i32::wrapping_add(iSlow48, i32::wrapping_mul(32, iSlow47));
		let mut iSlow51: i32 = (F64::floor(0.0078125 * fSlow3)) as i32 % 2;
		let mut iSlow52: i32 = i32::wrapping_add(iSlow50, i32::wrapping_mul(64, iSlow49));
		let mut iSlow53: i32 = (F64::floor(0.00390625 * fSlow3)) as i32 % 2;
		let mut iSlow54: i32 = i32::wrapping_add(iSlow52, i32::wrapping_mul(128, iSlow51));
		let mut iSlow55: i32 = (F64::floor(0.001953125 * fSlow3)) as i32 % 2;
		let mut iSlow56: i32 = i32::wrapping_add(iSlow54, i32::wrapping_mul(256, iSlow53));
		let mut iSlow57: i32 = (F64::floor(0.0009765625 * fSlow3)) as i32 % 2;
		let mut iSlow58: i32 = i32::wrapping_add(iSlow56, i32::wrapping_mul(512, iSlow55));
		let mut iSlow59: i32 = (F64::floor(0.00048828125 * fSlow3)) as i32 % 2;
		let mut iSlow60: i32 = i32::wrapping_add(iSlow58, i32::wrapping_mul(1024, iSlow57));
		let mut fSlow61: F64 = self.fHslider7;
		let mut fSlow62: F64 = self.fHslider8;
		let mut fSlow63: F64 = self.fHslider9;
		let mut iSlow64: i32 = (fSlow2) as i32;
		let mut fSlow65: F64 = self.fConst2 * (fSlow1 + fSlow14);
		let mut fSlow66: F64 = self.fCheckbox1;
		let mut iSlow67: i32 = (F64::max(0.0, fSlow66 * (4.8e+03 - fSlow65))) as i32;
		self.fHbargraph0 = if (fSlow66) as i32 != 0 {4.8e+03} else {fSlow65};
		let mut iSlow68: i32 = (self.fHbargraph0) as i32;
		let mut fSlow69: F64 = self.fConst3 * F64::powf(1e+01, 0.05 * self.fHslider10);
		let zipped_iterators = inputs0.zip(inputs1).zip(outputs0).zip(outputs1);
		for (((input0, input1), output0), output1) in zipped_iterators {
			let mut fTemp0: F64 = self.fConst1 + self.fRec0[1];
			let mut fTemp1: F64 = self.fRec0[1] - self.fConst1;
			self.fRec0[0] = if (fTemp0 < fSlow0) as i32 != 0 {fTemp0} else {if (fTemp1 > fSlow0) as i32 != 0 {fTemp1} else {fSlow0}};
			let mut fTemp2: F64 = F64::sin(6.283185307179586 * (0.5 * self.fRec0[0] + 0.75)) + 1.0;
			let mut fTemp3: F64 = 1.0 - 0.5 * fTemp2;
			let mut fTemp4: F64 = self.fRec2[((i32::wrapping_sub(self.IOTA0, 1)) & 8191) as usize];
			let mut fTemp5: F64 = self.fRec3[((i32::wrapping_sub(self.IOTA0, 1)) & 16383) as usize];
			self.fRec4[0] = fSlow9 + self.fConst4 * self.fRec4[1];
			let mut fTemp6: F64 = *input0;
			self.fVec0[(self.IOTA0 & 32767) as usize] = fTemp6;
			let mut fTemp7: F64 = fTemp6 * self.fRec4[0];
			self.fVec1[(self.IOTA0 & 32767) as usize] = fTemp7;
			let mut fTemp8: F64 = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, F64::abs(fTemp7)));
			let mut fTemp9: F64 = *input1;
			self.fVec2[(self.IOTA0 & 32767) as usize] = fTemp9;
			let mut fTemp10: F64 = fTemp9 * self.fRec4[0];
			self.fVec3[(self.IOTA0 & 32767) as usize] = fTemp10;
			let mut fTemp11: F64 = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, F64::abs(fTemp10)));
			let mut fTemp12: F64 = F64::max(fTemp8, fTemp11);
			let mut fTemp13: F64 = fTemp8 + fSlow10 * (fTemp12 - fTemp8);
			let mut iTemp14: i32 = ((fTemp13 > fSlow11) as i32) + ((fTemp13 > fSlow8) as i32);
			let mut fTemp15: F64 = fTemp13 - fSlow7;
			let mut fTemp16: F64 = F64::min(1.0, F64::powf(1e+01, -(fSlow13 * F64::max(0.0, if (iTemp14 == 0) as i32 != 0 {0.0} else {if (iTemp14 == 1) as i32 != 0 {fSlow12 * mydsp_faustpower2_f(fSlow6 + fTemp15)} else {fTemp15}}))));
			self.fVec4[(self.IOTA0 & 16383) as usize] = fTemp16;
			let mut fTemp17: F64 = F64::min(fTemp16, self.fVec4[((i32::wrapping_sub(self.IOTA0, 1)) & 16383) as usize]);
			self.fVec5[0] = fTemp17;
			let mut fTemp18: F64 = F64::min(fTemp17, self.fVec5[2]);
			self.fVec6[0] = fTemp18;
			let mut fTemp19: F64 = F64::min(fTemp18, self.fVec6[4]);
			self.fVec7[0] = fTemp19;
			let mut fTemp20: F64 = F64::min(fTemp19, self.fVec7[8]);
			self.fVec8[(self.IOTA0 & 31) as usize] = fTemp20;
			let mut fTemp21: F64 = F64::min(fTemp20, self.fVec8[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize]);
			self.fVec9[(self.IOTA0 & 63) as usize] = fTemp21;
			let mut fTemp22: F64 = F64::min(fTemp21, self.fVec9[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize]);
			self.fVec10[(self.IOTA0 & 127) as usize] = fTemp22;
			let mut fTemp23: F64 = F64::min(fTemp22, self.fVec10[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize]);
			self.fVec11[(self.IOTA0 & 255) as usize] = fTemp23;
			let mut fTemp24: F64 = F64::min(fTemp23, self.fVec11[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize]);
			self.fVec12[(self.IOTA0 & 511) as usize] = fTemp24;
			let mut fTemp25: F64 = F64::min(fTemp24, self.fVec12[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize]);
			self.fVec13[(self.IOTA0 & 1023) as usize] = fTemp25;
			let mut fTemp26: F64 = F64::min(fTemp25, self.fVec13[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize]);
			self.fVec14[(self.IOTA0 & 2047) as usize] = fTemp26;
			self.fVec15[(self.IOTA0 & 4095) as usize] = F64::min(fTemp26, self.fVec14[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			self.fRec3[(self.IOTA0 & 16383) as usize] = F64::max(F64::min(fTemp5, self.fVec4[((i32::wrapping_sub(self.IOTA0, iSlow16)) & 16383) as usize]), F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow18 != 0 {fTemp16} else {1.7976931348623157e+308}, if iSlow19 != 0 {self.fVec5[iSlow18 as usize]} else {1.7976931348623157e+308}), if iSlow20 != 0 {self.fVec6[iSlow21 as usize]} else {1.7976931348623157e+308}), if iSlow22 != 0 {self.fVec7[iSlow23 as usize]} else {1.7976931348623157e+308}), if iSlow24 != 0 {self.fVec8[((i32::wrapping_sub(self.IOTA0, iSlow25)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow26 != 0 {self.fVec9[((i32::wrapping_sub(self.IOTA0, iSlow27)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow28 != 0 {self.fVec10[((i32::wrapping_sub(self.IOTA0, iSlow29)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow30 != 0 {self.fVec11[((i32::wrapping_sub(self.IOTA0, iSlow31)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow32 != 0 {self.fVec12[((i32::wrapping_sub(self.IOTA0, iSlow33)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow34 != 0 {self.fVec13[((i32::wrapping_sub(self.IOTA0, iSlow35)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow36 != 0 {self.fVec14[((i32::wrapping_sub(self.IOTA0, iSlow37)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow38 != 0 {self.fVec15[((i32::wrapping_sub(self.IOTA0, iSlow39)) & 4095) as usize]} else {1.7976931348623157e+308}));
			let mut fTemp27: F64 = self.fRec3[(self.IOTA0 & 16383) as usize];
			let mut fTemp28: F64 = F64::min(fTemp27, fTemp5);
			self.fVec16[0] = fTemp28;
			let mut fTemp29: F64 = F64::min(fTemp28, self.fVec16[2]);
			self.fVec17[0] = fTemp29;
			let mut fTemp30: F64 = F64::min(fTemp29, self.fVec17[4]);
			self.fVec18[0] = fTemp30;
			let mut fTemp31: F64 = F64::min(fTemp30, self.fVec18[8]);
			self.fVec19[(self.IOTA0 & 31) as usize] = fTemp31;
			let mut fTemp32: F64 = F64::min(fTemp31, self.fVec19[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize]);
			self.fVec20[(self.IOTA0 & 63) as usize] = fTemp32;
			let mut fTemp33: F64 = F64::min(fTemp32, self.fVec20[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize]);
			self.fVec21[(self.IOTA0 & 127) as usize] = fTemp33;
			let mut fTemp34: F64 = F64::min(fTemp33, self.fVec21[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize]);
			self.fVec22[(self.IOTA0 & 255) as usize] = fTemp34;
			let mut fTemp35: F64 = F64::min(fTemp34, self.fVec22[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize]);
			self.fVec23[(self.IOTA0 & 511) as usize] = fTemp35;
			let mut fTemp36: F64 = F64::min(fTemp35, self.fVec23[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize]);
			self.fVec24[(self.IOTA0 & 1023) as usize] = fTemp36;
			let mut fTemp37: F64 = F64::min(fTemp36, self.fVec24[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize]);
			self.fVec25[(self.IOTA0 & 2047) as usize] = fTemp37;
			self.fVec26[(self.IOTA0 & 4095) as usize] = F64::min(fTemp37, self.fVec25[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			let mut fTemp38: F64 = F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow4 != 0 {fTemp27} else {1.7976931348623157e+308}, if iSlow40 != 0 {self.fVec16[iSlow4 as usize]} else {1.7976931348623157e+308}), if iSlow41 != 0 {self.fVec17[iSlow42 as usize]} else {1.7976931348623157e+308}), if iSlow43 != 0 {self.fVec18[iSlow44 as usize]} else {1.7976931348623157e+308}), if iSlow45 != 0 {self.fVec19[((i32::wrapping_sub(self.IOTA0, iSlow46)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow47 != 0 {self.fVec20[((i32::wrapping_sub(self.IOTA0, iSlow48)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow49 != 0 {self.fVec21[((i32::wrapping_sub(self.IOTA0, iSlow50)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow51 != 0 {self.fVec22[((i32::wrapping_sub(self.IOTA0, iSlow52)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow53 != 0 {self.fVec23[((i32::wrapping_sub(self.IOTA0, iSlow54)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow55 != 0 {self.fVec24[((i32::wrapping_sub(self.IOTA0, iSlow56)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow57 != 0 {self.fVec25[((i32::wrapping_sub(self.IOTA0, iSlow58)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow59 != 0 {self.fVec26[((i32::wrapping_sub(self.IOTA0, iSlow60)) & 4095) as usize]} else {1.7976931348623157e+308}) - fTemp4;
			self.fVec27[0] = fTemp38;
			let mut iTemp39: i32 = (fTemp38 > 0.0) as i32;
			let mut fTemp45: F64 = if iTemp39 != 0 {fSlow62} else {fSlow61};
			self.fVec28[0] = fTemp45;
			let mut fTemp46: F64 = 2.0 * fTemp45;
			let mut iTemp47: i32 = (fTemp46) as i32;
			let mut iTemp48: i32 = std::cmp::max(0, std::cmp::min(iTemp47, 2));
			let mut iTemp49: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, 98301), 196607));
			let mut fTemp50: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp49, 3)) as usize] };
			let mut fTemp51: F64 = unsafe { ftbl0mydspSIG0[iTemp49 as usize] };
			let mut fTemp52: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp49, 1)) as usize] } - fTemp51;
			let mut fTemp53: F64 = fTemp46 - (iTemp47) as F64;
			let mut fTemp54: F64 = fTemp51 + fTemp53 * fTemp52 + 0.5 * (fTemp50 - (fTemp51 + fTemp53 * (fTemp52 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp49, 4)) as usize] } - fTemp50))));
			let mut fTemp55: F64 = if iTemp39 != 0 {fTemp54} else {1.0 - fTemp54};
			let mut iTemp56: i32 = (fTemp38 < 0.0) as i32;
			let mut fTemp57: F64 = fSlow1 * (iTemp56) as F64 + fSlow63 * (iTemp39) as F64;
			self.fVec29[0] = fTemp57;
			let mut fTemp58: F64 = self.fConst5 / fTemp57;
			let mut fTemp59: F64 = fTemp58 + 0.5;
			let mut fTemp60: F64 = 65535.0 * (1.0 - fTemp59);
			let mut iTemp61: i32 = (fTemp60) as i32;
			let mut iTemp62: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp61, 65535)))), 196607));
			let mut fTemp63: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp62, 3)) as usize] };
			let mut fTemp64: F64 = unsafe { ftbl0mydspSIG0[iTemp62 as usize] };
			let mut fTemp65: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp62, 1)) as usize] } - fTemp64;
			let mut fTemp66: F64 = 65535.0 * fTemp59;
			let mut iTemp67: i32 = (fTemp66) as i32;
			let mut iTemp68: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp67, 65535)))), 196607));
			let mut fTemp69: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp68, 3), 196607))) as usize] };
			let mut fTemp70: F64 = unsafe { ftbl0mydspSIG0[iTemp68 as usize] };
			let mut fTemp71: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp68, 1), 196607))) as usize] } - fTemp70;
			let mut fTemp72: F64 = 2.0 * self.fVec28[1];
			let mut iTemp73: i32 = (fTemp72) as i32;
			let mut iTemp74: i32 = std::cmp::max(0, std::cmp::min(iTemp73, 2));
			let mut fTemp75: F64 = 65535.0 * (1.0 - self.fRec1[1]);
			let mut iTemp76: i32 = (fTemp75) as i32;
			let mut iTemp77: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp76, 65535))), iTemp74), 196607));
			let mut fTemp78: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp77, 3), 196607))) as usize] };
			let mut fTemp79: F64 = unsafe { ftbl0mydspSIG0[iTemp77 as usize] };
			let mut fTemp80: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp77, 1), 196607))) as usize] } - fTemp79;
			let mut fTemp81: F64 = fTemp72 - (iTemp73) as F64;
			let mut fTemp82: F64 = 65535.0 * self.fRec1[1];
			let mut iTemp83: i32 = (fTemp82) as i32;
			let mut iTemp84: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp74, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp83, 65535)))), 196607));
			let mut fTemp85: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp84, 3), 196607))) as usize] };
			let mut fTemp86: F64 = unsafe { ftbl0mydspSIG0[iTemp84 as usize] };
			let mut fTemp87: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp84, 1), 196607))) as usize] } - fTemp86;
			let mut fTemp88: F64 = self.fRec1[1] + fTemp58;
			let mut fTemp89: F64 = 65535.0 * (1.0 - fTemp88);
			let mut iTemp90: i32 = (fTemp89) as i32;
			let mut iTemp91: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp90, 65535)))), 196607));
			let mut fTemp92: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp91, 3)) as usize] };
			let mut fTemp93: F64 = unsafe { ftbl0mydspSIG0[iTemp91 as usize] };
			let mut fTemp94: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp91, 1)) as usize] } - fTemp93;
			let mut fTemp95: F64 = 65535.0 * fTemp88;
			let mut iTemp96: i32 = (fTemp95) as i32;
			let mut iTemp97: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp96, 65535)))), 196607));
			let mut fTemp98: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp97, 3), 196607))) as usize] };
			let mut fTemp99: F64 = unsafe { ftbl0mydspSIG0[iTemp97 as usize] };
			let mut fTemp100: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp97, 1), 196607))) as usize] } - fTemp99;
			let mut fTemp101: F64 = self.fRec1[1] + self.fConst5 * (1.0 / fTemp57 + 1.0 / self.fVec29[1]);
			let mut fTemp102: F64 = 65535.0 * (1.0 - fTemp101);
			let mut iTemp103: i32 = (fTemp102) as i32;
			let mut iTemp104: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp103, 65535))), iTemp48), 196607));
			let mut fTemp105: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp104, 3)) as usize] };
			let mut fTemp106: F64 = unsafe { ftbl0mydspSIG0[iTemp104 as usize] };
			let mut fTemp107: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp104, 1)) as usize] } - fTemp106;
			let mut fTemp108: F64 = 65535.0 * fTemp101;
			let mut iTemp109: i32 = (fTemp108) as i32;
			let mut iTemp110: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp109, 65535)))), 196607));
			let mut fTemp111: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp110, 3), 196607))) as usize] };
			let mut fTemp112: F64 = unsafe { ftbl0mydspSIG0[iTemp110 as usize] };
			let mut fTemp113: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp110, 1), 196607))) as usize] } - fTemp112;
			let mut fTemp114: F64 = (if iTemp39 != 0 {fTemp112 + fTemp53 * fTemp113 + (fTemp108 - (iTemp109) as F64) * (fTemp111 - (fTemp112 + fTemp53 * (fTemp113 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp110, 4), 196607))) as usize] } - fTemp111))))} else {1.0 - (fTemp106 + fTemp53 * fTemp107 + (fTemp102 - (iTemp103) as F64) * (fTemp105 - (fTemp106 + fTemp53 * (fTemp107 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp104, 4)) as usize] } - fTemp105)))))} - if iTemp39 != 0 {fTemp99 + fTemp53 * fTemp100 + (fTemp95 - (iTemp96) as F64) * (fTemp98 - (fTemp99 + fTemp53 * (fTemp100 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp97, 4), 196607))) as usize] } - fTemp98))))} else {1.0 - (fTemp93 + fTemp53 * fTemp94 + (fTemp89 - (iTemp90) as F64) * (fTemp92 - (fTemp93 + fTemp53 * (fTemp94 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp91, 4), 196607))) as usize] } - fTemp92)))))}) * self.fVec27[1] / (fTemp38 * (1.0 - if iTemp39 != 0 {fTemp86 + fTemp81 * fTemp87 + (fTemp82 - (iTemp83) as F64) * (fTemp85 - (fTemp86 + fTemp81 * (fTemp87 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp84, 4), 196607))) as usize] } - fTemp85))))} else {1.0 - (fTemp79 + fTemp81 * fTemp80 + (fTemp75 - (iTemp76) as F64) * (fTemp78 - (fTemp79 + fTemp81 * (fTemp80 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp77, 4), 196607))) as usize] } - fTemp78)))))}));
			let mut iTemp115: i32 = (fTemp114 > ((if iTemp39 != 0 {fTemp70 + fTemp53 * fTemp71 + (fTemp66 - (iTemp67) as F64) * (fTemp69 - (fTemp70 + fTemp53 * (fTemp71 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp68, 4), 196607))) as usize] } - fTemp69))))} else {1.0 - (fTemp64 + fTemp53 * fTemp65 + (fTemp60 - (iTemp61) as F64) * (fTemp63 - (fTemp64 + fTemp53 * (fTemp65 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp62, 4)) as usize] } - fTemp63)))))} - fTemp55) / (1.0 - fTemp55))) as i32;
			let mut fTemp116: F64 = if iTemp115 != 0 {1.0} else {0.5};
			let mut fTemp117: F64 = if iTemp115 != 0 {0.5} else {0.0};
			let mut fTemp118: F64 = fTemp117 + fTemp116;
			let mut fTemp119: F64 = 0.5 * fTemp118;
			let mut fTemp120: F64 = 65535.0 * (1.0 - fTemp119);
			let mut iTemp121: i32 = (fTemp120) as i32;
			let mut iTemp122: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp121, 65535)))), 196607));
			let mut fTemp123: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp122, 3)) as usize] };
			let mut fTemp124: F64 = unsafe { ftbl0mydspSIG0[iTemp122 as usize] };
			let mut fTemp125: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp122, 1)) as usize] } - fTemp124;
			let mut fTemp126: F64 = 32767.5 * fTemp118;
			let mut iTemp127: i32 = (fTemp126) as i32;
			let mut iTemp128: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp127, 65535)))), 196607));
			let mut fTemp129: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp128, 3)) as usize] };
			let mut fTemp130: F64 = unsafe { ftbl0mydspSIG0[iTemp128 as usize] };
			let mut fTemp131: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp128, 1)) as usize] } - fTemp130;
			let mut fTemp132: F64 = if iTemp39 != 0 {fTemp130 + fTemp53 * fTemp131 + (fTemp126 - (iTemp127) as F64) * (fTemp129 - (fTemp130 + fTemp53 * (fTemp131 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp128, 4)) as usize] } - fTemp129))))} else {1.0 - (fTemp124 + fTemp53 * fTemp125 + (fTemp120 - (iTemp121) as F64) * (fTemp123 - (fTemp124 + fTemp53 * (fTemp125 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp122, 4)) as usize] } - fTemp123)))))};
			let mut fTemp133: F64 = fTemp58 + fTemp119;
			let mut fTemp134: F64 = 65535.0 * (1.0 - fTemp133);
			let mut iTemp135: i32 = (fTemp134) as i32;
			let mut iTemp136: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp135, 65535)))), 196607));
			let mut fTemp137: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp136, 3)) as usize] };
			let mut fTemp138: F64 = unsafe { ftbl0mydspSIG0[iTemp136 as usize] };
			let mut fTemp139: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp136, 1)) as usize] } - fTemp138;
			let mut fTemp140: F64 = 65535.0 * fTemp133;
			let mut iTemp141: i32 = (fTemp140) as i32;
			let mut iTemp142: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp141, 65535)))), 196607));
			let mut fTemp143: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp142, 3), 196607))) as usize] };
			let mut fTemp144: F64 = unsafe { ftbl0mydspSIG0[iTemp142 as usize] };
			let mut fTemp145: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp142, 1), 196607))) as usize] } - fTemp144;
			let mut iTemp146: i32 = (fTemp114 > ((if iTemp39 != 0 {fTemp144 + fTemp53 * fTemp145 + (fTemp140 - (iTemp141) as F64) * (fTemp143 - (fTemp144 + fTemp53 * (fTemp145 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp142, 4), 196607))) as usize] } - fTemp143))))} else {1.0 - (fTemp138 + fTemp53 * fTemp139 + (fTemp134 - (iTemp135) as F64) * (fTemp137 - (fTemp138 + fTemp53 * (fTemp139 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp136, 4)) as usize] } - fTemp137)))))} - fTemp132) / (1.0 - fTemp132))) as i32;
			let mut fTemp147: F64 = if iTemp146 != 0 {fTemp116} else {fTemp119};
			let mut fTemp148: F64 = if iTemp146 != 0 {fTemp119} else {fTemp117};
			let mut fTemp149: F64 = fTemp148 + fTemp147;
			let mut fTemp150: F64 = 0.5 * fTemp149;
			let mut fTemp151: F64 = 65535.0 * (1.0 - fTemp150);
			let mut iTemp152: i32 = (fTemp151) as i32;
			let mut iTemp153: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp152, 65535)))), 196607));
			let mut fTemp154: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp153, 3)) as usize] };
			let mut fTemp155: F64 = unsafe { ftbl0mydspSIG0[iTemp153 as usize] };
			let mut fTemp156: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp153, 1)) as usize] } - fTemp155;
			let mut fTemp157: F64 = 32767.5 * fTemp149;
			let mut iTemp158: i32 = (fTemp157) as i32;
			let mut iTemp159: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp158, 65535)))), 196607));
			let mut fTemp160: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp159, 3)) as usize] };
			let mut fTemp161: F64 = unsafe { ftbl0mydspSIG0[iTemp159 as usize] };
			let mut fTemp162: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp159, 1)) as usize] } - fTemp161;
			let mut fTemp163: F64 = if iTemp39 != 0 {fTemp161 + fTemp53 * fTemp162 + (fTemp157 - (iTemp158) as F64) * (fTemp160 - (fTemp161 + fTemp53 * (fTemp162 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp159, 4)) as usize] } - fTemp160))))} else {1.0 - (fTemp155 + fTemp53 * fTemp156 + (fTemp151 - (iTemp152) as F64) * (fTemp154 - (fTemp155 + fTemp53 * (fTemp156 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp153, 4)) as usize] } - fTemp154)))))};
			let mut fTemp164: F64 = fTemp58 + fTemp150;
			let mut fTemp165: F64 = 65535.0 * (1.0 - fTemp164);
			let mut iTemp166: i32 = (fTemp165) as i32;
			let mut iTemp167: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp166, 65535)))), 196607));
			let mut fTemp168: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp167, 3)) as usize] };
			let mut fTemp169: F64 = unsafe { ftbl0mydspSIG0[iTemp167 as usize] };
			let mut fTemp170: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp167, 1)) as usize] } - fTemp169;
			let mut fTemp171: F64 = 65535.0 * fTemp164;
			let mut iTemp172: i32 = (fTemp171) as i32;
			let mut iTemp173: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp172, 65535)))), 196607));
			let mut fTemp174: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp173, 3), 196607))) as usize] };
			let mut fTemp175: F64 = unsafe { ftbl0mydspSIG0[iTemp173 as usize] };
			let mut fTemp176: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp173, 1), 196607))) as usize] } - fTemp175;
			let mut iTemp177: i32 = (fTemp114 > ((if iTemp39 != 0 {fTemp175 + fTemp53 * fTemp176 + (fTemp171 - (iTemp172) as F64) * (fTemp174 - (fTemp175 + fTemp53 * (fTemp176 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp173, 4), 196607))) as usize] } - fTemp174))))} else {1.0 - (fTemp169 + fTemp53 * fTemp170 + (fTemp165 - (iTemp166) as F64) * (fTemp168 - (fTemp169 + fTemp53 * (fTemp170 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp167, 4)) as usize] } - fTemp168)))))} - fTemp163) / (1.0 - fTemp163))) as i32;
			let mut fTemp178: F64 = if iTemp177 != 0 {fTemp147} else {fTemp150};
			let mut fTemp179: F64 = if iTemp177 != 0 {fTemp150} else {fTemp148};
			let mut fTemp180: F64 = fTemp179 + fTemp178;
			let mut fTemp181: F64 = 0.5 * fTemp180;
			let mut fTemp182: F64 = 65535.0 * (1.0 - fTemp181);
			let mut iTemp183: i32 = (fTemp182) as i32;
			let mut iTemp184: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp183, 65535)))), 196607));
			let mut fTemp185: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp184, 3)) as usize] };
			let mut fTemp186: F64 = unsafe { ftbl0mydspSIG0[iTemp184 as usize] };
			let mut fTemp187: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp184, 1)) as usize] } - fTemp186;
			let mut fTemp188: F64 = 32767.5 * fTemp180;
			let mut iTemp189: i32 = (fTemp188) as i32;
			let mut iTemp190: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp189, 65535)))), 196607));
			let mut fTemp191: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp190, 3)) as usize] };
			let mut fTemp192: F64 = unsafe { ftbl0mydspSIG0[iTemp190 as usize] };
			let mut fTemp193: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp190, 1)) as usize] } - fTemp192;
			let mut fTemp194: F64 = if iTemp39 != 0 {fTemp192 + fTemp53 * fTemp193 + (fTemp188 - (iTemp189) as F64) * (fTemp191 - (fTemp192 + fTemp53 * (fTemp193 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp190, 4)) as usize] } - fTemp191))))} else {1.0 - (fTemp186 + fTemp53 * fTemp187 + (fTemp182 - (iTemp183) as F64) * (fTemp185 - (fTemp186 + fTemp53 * (fTemp187 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp184, 4)) as usize] } - fTemp185)))))};
			let mut fTemp195: F64 = fTemp58 + fTemp181;
			let mut fTemp196: F64 = 65535.0 * (1.0 - fTemp195);
			let mut iTemp197: i32 = (fTemp196) as i32;
			let mut iTemp198: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp197, 65535)))), 196607));
			let mut fTemp199: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp198, 3)) as usize] };
			let mut fTemp200: F64 = unsafe { ftbl0mydspSIG0[iTemp198 as usize] };
			let mut fTemp201: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp198, 1)) as usize] } - fTemp200;
			let mut fTemp202: F64 = 65535.0 * fTemp195;
			let mut iTemp203: i32 = (fTemp202) as i32;
			let mut iTemp204: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp203, 65535)))), 196607));
			let mut fTemp205: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp204, 3), 196607))) as usize] };
			let mut fTemp206: F64 = unsafe { ftbl0mydspSIG0[iTemp204 as usize] };
			let mut fTemp207: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp204, 1), 196607))) as usize] } - fTemp206;
			let mut iTemp208: i32 = (fTemp114 > ((if iTemp39 != 0 {fTemp206 + fTemp53 * fTemp207 + (fTemp202 - (iTemp203) as F64) * (fTemp205 - (fTemp206 + fTemp53 * (fTemp207 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp204, 4), 196607))) as usize] } - fTemp205))))} else {1.0 - (fTemp200 + fTemp53 * fTemp201 + (fTemp196 - (iTemp197) as F64) * (fTemp199 - (fTemp200 + fTemp53 * (fTemp201 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp198, 4)) as usize] } - fTemp199)))))} - fTemp194) / (1.0 - fTemp194))) as i32;
			let mut fTemp209: F64 = if iTemp208 != 0 {fTemp178} else {fTemp181};
			let mut fTemp210: F64 = if iTemp208 != 0 {fTemp181} else {fTemp179};
			let mut fTemp211: F64 = fTemp210 + fTemp209;
			let mut fTemp212: F64 = 0.5 * fTemp211;
			let mut fTemp213: F64 = 65535.0 * (1.0 - fTemp212);
			let mut iTemp214: i32 = (fTemp213) as i32;
			let mut iTemp215: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp214, 65535)))), 196607));
			let mut fTemp216: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp215, 3)) as usize] };
			let mut fTemp217: F64 = unsafe { ftbl0mydspSIG0[iTemp215 as usize] };
			let mut fTemp218: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp215, 1)) as usize] } - fTemp217;
			let mut fTemp219: F64 = 32767.5 * fTemp211;
			let mut iTemp220: i32 = (fTemp219) as i32;
			let mut iTemp221: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp220, 65535)))), 196607));
			let mut fTemp222: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp221, 3)) as usize] };
			let mut fTemp223: F64 = unsafe { ftbl0mydspSIG0[iTemp221 as usize] };
			let mut fTemp224: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp221, 1)) as usize] } - fTemp223;
			let mut fTemp225: F64 = if iTemp39 != 0 {fTemp223 + fTemp53 * fTemp224 + (fTemp219 - (iTemp220) as F64) * (fTemp222 - (fTemp223 + fTemp53 * (fTemp224 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp221, 4)) as usize] } - fTemp222))))} else {1.0 - (fTemp217 + fTemp53 * fTemp218 + (fTemp213 - (iTemp214) as F64) * (fTemp216 - (fTemp217 + fTemp53 * (fTemp218 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp215, 4)) as usize] } - fTemp216)))))};
			let mut fTemp226: F64 = fTemp58 + fTemp212;
			let mut fTemp227: F64 = 65535.0 * (1.0 - fTemp226);
			let mut iTemp228: i32 = (fTemp227) as i32;
			let mut iTemp229: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp228, 65535)))), 196607));
			let mut fTemp230: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp229, 3)) as usize] };
			let mut fTemp231: F64 = unsafe { ftbl0mydspSIG0[iTemp229 as usize] };
			let mut fTemp232: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp229, 1)) as usize] } - fTemp231;
			let mut fTemp233: F64 = 65535.0 * fTemp226;
			let mut iTemp234: i32 = (fTemp233) as i32;
			let mut iTemp235: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp234, 65535)))), 196607));
			let mut fTemp236: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp235, 3), 196607))) as usize] };
			let mut fTemp237: F64 = unsafe { ftbl0mydspSIG0[iTemp235 as usize] };
			let mut fTemp238: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp235, 1), 196607))) as usize] } - fTemp237;
			let mut iTemp239: i32 = (fTemp114 > ((if iTemp39 != 0 {fTemp237 + fTemp53 * fTemp238 + (fTemp233 - (iTemp234) as F64) * (fTemp236 - (fTemp237 + fTemp53 * (fTemp238 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp235, 4), 196607))) as usize] } - fTemp236))))} else {1.0 - (fTemp231 + fTemp53 * fTemp232 + (fTemp227 - (iTemp228) as F64) * (fTemp230 - (fTemp231 + fTemp53 * (fTemp232 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp229, 4)) as usize] } - fTemp230)))))} - fTemp225) / (1.0 - fTemp225))) as i32;
			let mut fTemp240: F64 = if iTemp239 != 0 {fTemp209} else {fTemp212};
			let mut fTemp241: F64 = if iTemp239 != 0 {fTemp212} else {fTemp210};
			let mut fTemp242: F64 = fTemp241 + fTemp240;
			let mut fTemp243: F64 = 0.5 * fTemp242;
			let mut fTemp244: F64 = 65535.0 * (1.0 - fTemp243);
			let mut iTemp245: i32 = (fTemp244) as i32;
			let mut iTemp246: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp245, 65535)))), 196607));
			let mut fTemp247: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp246, 3)) as usize] };
			let mut fTemp248: F64 = unsafe { ftbl0mydspSIG0[iTemp246 as usize] };
			let mut fTemp249: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp246, 1)) as usize] } - fTemp248;
			let mut fTemp250: F64 = 32767.5 * fTemp242;
			let mut iTemp251: i32 = (fTemp250) as i32;
			let mut iTemp252: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp251, 65535)))), 196607));
			let mut fTemp253: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp252, 3)) as usize] };
			let mut fTemp254: F64 = unsafe { ftbl0mydspSIG0[iTemp252 as usize] };
			let mut fTemp255: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp252, 1)) as usize] } - fTemp254;
			let mut fTemp256: F64 = if iTemp39 != 0 {fTemp254 + fTemp53 * fTemp255 + (fTemp250 - (iTemp251) as F64) * (fTemp253 - (fTemp254 + fTemp53 * (fTemp255 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp252, 4)) as usize] } - fTemp253))))} else {1.0 - (fTemp248 + fTemp53 * fTemp249 + (fTemp244 - (iTemp245) as F64) * (fTemp247 - (fTemp248 + fTemp53 * (fTemp249 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp246, 4)) as usize] } - fTemp247)))))};
			let mut fTemp257: F64 = fTemp58 + fTemp243;
			let mut fTemp258: F64 = 65535.0 * (1.0 - fTemp257);
			let mut iTemp259: i32 = (fTemp258) as i32;
			let mut iTemp260: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp259, 65535)))), 196607));
			let mut fTemp261: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp260, 3)) as usize] };
			let mut fTemp262: F64 = unsafe { ftbl0mydspSIG0[iTemp260 as usize] };
			let mut fTemp263: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp260, 1)) as usize] } - fTemp262;
			let mut fTemp264: F64 = 65535.0 * fTemp257;
			let mut iTemp265: i32 = (fTemp264) as i32;
			let mut iTemp266: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp265, 65535)))), 196607));
			let mut fTemp267: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp266, 3), 196607))) as usize] };
			let mut fTemp268: F64 = unsafe { ftbl0mydspSIG0[iTemp266 as usize] };
			let mut fTemp269: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp266, 1), 196607))) as usize] } - fTemp268;
			let mut iTemp270: i32 = (fTemp114 > ((if iTemp39 != 0 {fTemp268 + fTemp53 * fTemp269 + (fTemp264 - (iTemp265) as F64) * (fTemp267 - (fTemp268 + fTemp53 * (fTemp269 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp266, 4), 196607))) as usize] } - fTemp267))))} else {1.0 - (fTemp262 + fTemp53 * fTemp263 + (fTemp258 - (iTemp259) as F64) * (fTemp261 - (fTemp262 + fTemp53 * (fTemp263 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp260, 4)) as usize] } - fTemp261)))))} - fTemp256) / (1.0 - fTemp256))) as i32;
			let mut fTemp271: F64 = if iTemp270 != 0 {fTemp240} else {fTemp243};
			let mut fTemp272: F64 = if iTemp270 != 0 {fTemp243} else {fTemp241};
			let mut fTemp273: F64 = fTemp272 + fTemp271;
			let mut fTemp274: F64 = 0.5 * fTemp273;
			let mut fTemp275: F64 = 65535.0 * (1.0 - fTemp274);
			let mut iTemp276: i32 = (fTemp275) as i32;
			let mut iTemp277: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp276, 65535)))), 196607));
			let mut fTemp278: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp277, 3)) as usize] };
			let mut fTemp279: F64 = unsafe { ftbl0mydspSIG0[iTemp277 as usize] };
			let mut fTemp280: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp277, 1)) as usize] } - fTemp279;
			let mut fTemp281: F64 = 32767.5 * fTemp273;
			let mut iTemp282: i32 = (fTemp281) as i32;
			let mut iTemp283: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp282, 65535)))), 196607));
			let mut fTemp284: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp283, 3)) as usize] };
			let mut fTemp285: F64 = unsafe { ftbl0mydspSIG0[iTemp283 as usize] };
			let mut fTemp286: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp283, 1)) as usize] } - fTemp285;
			let mut fTemp287: F64 = if iTemp39 != 0 {fTemp285 + fTemp53 * fTemp286 + (fTemp281 - (iTemp282) as F64) * (fTemp284 - (fTemp285 + fTemp53 * (fTemp286 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp283, 4)) as usize] } - fTemp284))))} else {1.0 - (fTemp279 + fTemp53 * fTemp280 + (fTemp275 - (iTemp276) as F64) * (fTemp278 - (fTemp279 + fTemp53 * (fTemp280 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp277, 4)) as usize] } - fTemp278)))))};
			let mut fTemp288: F64 = fTemp58 + fTemp274;
			let mut fTemp289: F64 = 65535.0 * (1.0 - fTemp288);
			let mut iTemp290: i32 = (fTemp289) as i32;
			let mut iTemp291: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp290, 65535)))), 196607));
			let mut fTemp292: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp291, 3)) as usize] };
			let mut fTemp293: F64 = unsafe { ftbl0mydspSIG0[iTemp291 as usize] };
			let mut fTemp294: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp291, 1)) as usize] } - fTemp293;
			let mut fTemp295: F64 = 65535.0 * fTemp288;
			let mut iTemp296: i32 = (fTemp295) as i32;
			let mut iTemp297: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp296, 65535)))), 196607));
			let mut fTemp298: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp297, 3), 196607))) as usize] };
			let mut fTemp299: F64 = unsafe { ftbl0mydspSIG0[iTemp297 as usize] };
			let mut fTemp300: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp297, 1), 196607))) as usize] } - fTemp299;
			let mut iTemp301: i32 = (fTemp114 > ((if iTemp39 != 0 {fTemp299 + fTemp53 * fTemp300 + (fTemp295 - (iTemp296) as F64) * (fTemp298 - (fTemp299 + fTemp53 * (fTemp300 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp297, 4), 196607))) as usize] } - fTemp298))))} else {1.0 - (fTemp293 + fTemp53 * fTemp294 + (fTemp289 - (iTemp290) as F64) * (fTemp292 - (fTemp293 + fTemp53 * (fTemp294 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp291, 4)) as usize] } - fTemp292)))))} - fTemp287) / (1.0 - fTemp287))) as i32;
			let mut fTemp302: F64 = if iTemp301 != 0 {fTemp271} else {fTemp274};
			let mut fTemp303: F64 = if iTemp301 != 0 {fTemp274} else {fTemp272};
			let mut fTemp304: F64 = fTemp303 + fTemp302;
			let mut fTemp305: F64 = 0.5 * fTemp304;
			let mut fTemp306: F64 = 65535.0 * (1.0 - fTemp305);
			let mut iTemp307: i32 = (fTemp306) as i32;
			let mut iTemp308: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp307, 65535)))), 196607));
			let mut fTemp309: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp308, 3)) as usize] };
			let mut fTemp310: F64 = unsafe { ftbl0mydspSIG0[iTemp308 as usize] };
			let mut fTemp311: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp308, 1)) as usize] } - fTemp310;
			let mut fTemp312: F64 = 32767.5 * fTemp304;
			let mut iTemp313: i32 = (fTemp312) as i32;
			let mut iTemp314: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp313, 65535)))), 196607));
			let mut fTemp315: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp314, 3)) as usize] };
			let mut fTemp316: F64 = unsafe { ftbl0mydspSIG0[iTemp314 as usize] };
			let mut fTemp317: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp314, 1)) as usize] } - fTemp316;
			let mut fTemp318: F64 = if iTemp39 != 0 {fTemp316 + fTemp53 * fTemp317 + (fTemp312 - (iTemp313) as F64) * (fTemp315 - (fTemp316 + fTemp53 * (fTemp317 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp314, 4)) as usize] } - fTemp315))))} else {1.0 - (fTemp310 + fTemp53 * fTemp311 + (fTemp306 - (iTemp307) as F64) * (fTemp309 - (fTemp310 + fTemp53 * (fTemp311 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp308, 4)) as usize] } - fTemp309)))))};
			let mut fTemp319: F64 = fTemp58 + fTemp305;
			let mut fTemp320: F64 = 65535.0 * (1.0 - fTemp319);
			let mut iTemp321: i32 = (fTemp320) as i32;
			let mut iTemp322: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp321, 65535)))), 196607));
			let mut fTemp323: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp322, 3)) as usize] };
			let mut fTemp324: F64 = unsafe { ftbl0mydspSIG0[iTemp322 as usize] };
			let mut fTemp325: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp322, 1)) as usize] } - fTemp324;
			let mut fTemp326: F64 = 65535.0 * fTemp319;
			let mut iTemp327: i32 = (fTemp326) as i32;
			let mut iTemp328: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp327, 65535)))), 196607));
			let mut fTemp329: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp328, 3), 196607))) as usize] };
			let mut fTemp330: F64 = unsafe { ftbl0mydspSIG0[iTemp328 as usize] };
			let mut fTemp331: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp328, 1), 196607))) as usize] } - fTemp330;
			let mut iTemp332: i32 = (fTemp114 > ((if iTemp39 != 0 {fTemp330 + fTemp53 * fTemp331 + (fTemp326 - (iTemp327) as F64) * (fTemp329 - (fTemp330 + fTemp53 * (fTemp331 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp328, 4), 196607))) as usize] } - fTemp329))))} else {1.0 - (fTemp324 + fTemp53 * fTemp325 + (fTemp320 - (iTemp321) as F64) * (fTemp323 - (fTemp324 + fTemp53 * (fTemp325 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp322, 4)) as usize] } - fTemp323)))))} - fTemp318) / (1.0 - fTemp318))) as i32;
			let mut fTemp333: F64 = if iTemp332 != 0 {fTemp302} else {fTemp305};
			let mut fTemp334: F64 = if iTemp332 != 0 {fTemp305} else {fTemp303};
			let mut fTemp335: F64 = fTemp334 + fTemp333;
			let mut fTemp336: F64 = 0.5 * fTemp335;
			let mut fTemp337: F64 = 65535.0 * (1.0 - fTemp336);
			let mut iTemp338: i32 = (fTemp337) as i32;
			let mut iTemp339: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp338, 65535)))), 196607));
			let mut fTemp340: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp339, 3)) as usize] };
			let mut fTemp341: F64 = unsafe { ftbl0mydspSIG0[iTemp339 as usize] };
			let mut fTemp342: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp339, 1)) as usize] } - fTemp341;
			let mut fTemp343: F64 = 32767.5 * fTemp335;
			let mut iTemp344: i32 = (fTemp343) as i32;
			let mut iTemp345: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp344, 65535)))), 196607));
			let mut fTemp346: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp345, 3)) as usize] };
			let mut fTemp347: F64 = unsafe { ftbl0mydspSIG0[iTemp345 as usize] };
			let mut fTemp348: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp345, 1)) as usize] } - fTemp347;
			let mut fTemp349: F64 = if iTemp39 != 0 {fTemp347 + fTemp53 * fTemp348 + (fTemp343 - (iTemp344) as F64) * (fTemp346 - (fTemp347 + fTemp53 * (fTemp348 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp345, 4)) as usize] } - fTemp346))))} else {1.0 - (fTemp341 + fTemp53 * fTemp342 + (fTemp337 - (iTemp338) as F64) * (fTemp340 - (fTemp341 + fTemp53 * (fTemp342 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp339, 4)) as usize] } - fTemp340)))))};
			let mut fTemp350: F64 = fTemp58 + fTemp336;
			let mut fTemp351: F64 = 65535.0 * (1.0 - fTemp350);
			let mut iTemp352: i32 = (fTemp351) as i32;
			let mut iTemp353: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp352, 65535)))), 196607));
			let mut fTemp354: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp353, 3)) as usize] };
			let mut fTemp355: F64 = unsafe { ftbl0mydspSIG0[iTemp353 as usize] };
			let mut fTemp356: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp353, 1)) as usize] } - fTemp355;
			let mut fTemp357: F64 = 65535.0 * fTemp350;
			let mut iTemp358: i32 = (fTemp357) as i32;
			let mut iTemp359: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp358, 65535)))), 196607));
			let mut fTemp360: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp359, 3), 196607))) as usize] };
			let mut fTemp361: F64 = unsafe { ftbl0mydspSIG0[iTemp359 as usize] };
			let mut fTemp362: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp359, 1), 196607))) as usize] } - fTemp361;
			let mut iTemp363: i32 = (fTemp114 > ((if iTemp39 != 0 {fTemp361 + fTemp53 * fTemp362 + (fTemp357 - (iTemp358) as F64) * (fTemp360 - (fTemp361 + fTemp53 * (fTemp362 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp359, 4), 196607))) as usize] } - fTemp360))))} else {1.0 - (fTemp355 + fTemp53 * fTemp356 + (fTemp351 - (iTemp352) as F64) * (fTemp354 - (fTemp355 + fTemp53 * (fTemp356 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp353, 4)) as usize] } - fTemp354)))))} - fTemp349) / (1.0 - fTemp349))) as i32;
			let mut fTemp364: F64 = if iTemp363 != 0 {fTemp333} else {fTemp336};
			let mut fTemp365: F64 = if iTemp363 != 0 {fTemp336} else {fTemp334};
			let mut fTemp366: F64 = fTemp365 + fTemp364;
			let mut fTemp367: F64 = 0.5 * fTemp366;
			let mut fTemp368: F64 = 65535.0 * (1.0 - fTemp367);
			let mut iTemp369: i32 = (fTemp368) as i32;
			let mut iTemp370: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp369, 65535)))), 196607));
			let mut fTemp371: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp370, 3)) as usize] };
			let mut fTemp372: F64 = unsafe { ftbl0mydspSIG0[iTemp370 as usize] };
			let mut fTemp373: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp370, 1)) as usize] } - fTemp372;
			let mut fTemp374: F64 = 32767.5 * fTemp366;
			let mut iTemp375: i32 = (fTemp374) as i32;
			let mut iTemp376: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp375, 65535)))), 196607));
			let mut fTemp377: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp376, 3)) as usize] };
			let mut fTemp378: F64 = unsafe { ftbl0mydspSIG0[iTemp376 as usize] };
			let mut fTemp379: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp376, 1)) as usize] } - fTemp378;
			let mut fTemp380: F64 = if iTemp39 != 0 {fTemp378 + fTemp53 * fTemp379 + (fTemp374 - (iTemp375) as F64) * (fTemp377 - (fTemp378 + fTemp53 * (fTemp379 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp376, 4)) as usize] } - fTemp377))))} else {1.0 - (fTemp372 + fTemp53 * fTemp373 + (fTemp368 - (iTemp369) as F64) * (fTemp371 - (fTemp372 + fTemp53 * (fTemp373 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp370, 4)) as usize] } - fTemp371)))))};
			let mut fTemp381: F64 = fTemp58 + fTemp367;
			let mut fTemp382: F64 = 65535.0 * (1.0 - fTemp381);
			let mut iTemp383: i32 = (fTemp382) as i32;
			let mut iTemp384: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp383, 65535)))), 196607));
			let mut fTemp385: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp384, 3)) as usize] };
			let mut fTemp386: F64 = unsafe { ftbl0mydspSIG0[iTemp384 as usize] };
			let mut fTemp387: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp384, 1)) as usize] } - fTemp386;
			let mut fTemp388: F64 = 65535.0 * fTemp381;
			let mut iTemp389: i32 = (fTemp388) as i32;
			let mut iTemp390: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp389, 65535)))), 196607));
			let mut fTemp391: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp390, 3), 196607))) as usize] };
			let mut fTemp392: F64 = unsafe { ftbl0mydspSIG0[iTemp390 as usize] };
			let mut fTemp393: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp390, 1), 196607))) as usize] } - fTemp392;
			let mut iTemp394: i32 = (fTemp114 > ((if iTemp39 != 0 {fTemp392 + fTemp53 * fTemp393 + (fTemp388 - (iTemp389) as F64) * (fTemp391 - (fTemp392 + fTemp53 * (fTemp393 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp390, 4), 196607))) as usize] } - fTemp391))))} else {1.0 - (fTemp386 + fTemp53 * fTemp387 + (fTemp382 - (iTemp383) as F64) * (fTemp385 - (fTemp386 + fTemp53 * (fTemp387 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp384, 4)) as usize] } - fTemp385)))))} - fTemp380) / (1.0 - fTemp380))) as i32;
			let mut fTemp395: F64 = if iTemp394 != 0 {fTemp364} else {fTemp367};
			let mut fTemp396: F64 = if iTemp394 != 0 {fTemp367} else {fTemp365};
			let mut fTemp397: F64 = fTemp396 + fTemp395;
			let mut fTemp398: F64 = 0.5 * fTemp397;
			let mut fTemp399: F64 = 65535.0 * (1.0 - fTemp398);
			let mut iTemp400: i32 = (fTemp399) as i32;
			let mut iTemp401: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp400, 65535)))), 196607));
			let mut fTemp402: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp401, 3)) as usize] };
			let mut fTemp403: F64 = unsafe { ftbl0mydspSIG0[iTemp401 as usize] };
			let mut fTemp404: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp401, 1)) as usize] } - fTemp403;
			let mut fTemp405: F64 = 32767.5 * fTemp397;
			let mut iTemp406: i32 = (fTemp405) as i32;
			let mut iTemp407: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp406, 65535)))), 196607));
			let mut fTemp408: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp407, 3)) as usize] };
			let mut fTemp409: F64 = unsafe { ftbl0mydspSIG0[iTemp407 as usize] };
			let mut fTemp410: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp407, 1)) as usize] } - fTemp409;
			let mut fTemp411: F64 = if iTemp39 != 0 {fTemp409 + fTemp53 * fTemp410 + (fTemp405 - (iTemp406) as F64) * (fTemp408 - (fTemp409 + fTemp53 * (fTemp410 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp407, 4)) as usize] } - fTemp408))))} else {1.0 - (fTemp403 + fTemp53 * fTemp404 + (fTemp399 - (iTemp400) as F64) * (fTemp402 - (fTemp403 + fTemp53 * (fTemp404 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp401, 4)) as usize] } - fTemp402)))))};
			let mut fTemp412: F64 = fTemp58 + fTemp398;
			let mut fTemp413: F64 = 65535.0 * (1.0 - fTemp412);
			let mut iTemp414: i32 = (fTemp413) as i32;
			let mut iTemp415: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp414, 65535)))), 196607));
			let mut fTemp416: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp415, 3)) as usize] };
			let mut fTemp417: F64 = unsafe { ftbl0mydspSIG0[iTemp415 as usize] };
			let mut fTemp418: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp415, 1)) as usize] } - fTemp417;
			let mut fTemp419: F64 = 65535.0 * fTemp412;
			let mut iTemp420: i32 = (fTemp419) as i32;
			let mut iTemp421: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp420, 65535)))), 196607));
			let mut fTemp422: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp421, 3), 196607))) as usize] };
			let mut fTemp423: F64 = unsafe { ftbl0mydspSIG0[iTemp421 as usize] };
			let mut fTemp424: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp421, 1), 196607))) as usize] } - fTemp423;
			let mut iTemp425: i32 = (fTemp114 > ((if iTemp39 != 0 {fTemp423 + fTemp53 * fTemp424 + (fTemp419 - (iTemp420) as F64) * (fTemp422 - (fTemp423 + fTemp53 * (fTemp424 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp421, 4), 196607))) as usize] } - fTemp422))))} else {1.0 - (fTemp417 + fTemp53 * fTemp418 + (fTemp413 - (iTemp414) as F64) * (fTemp416 - (fTemp417 + fTemp53 * (fTemp418 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp415, 4)) as usize] } - fTemp416)))))} - fTemp411) / (1.0 - fTemp411))) as i32;
			let mut fTemp426: F64 = if iTemp425 != 0 {fTemp395} else {fTemp398};
			let mut fTemp427: F64 = if iTemp425 != 0 {fTemp398} else {fTemp396};
			let mut fTemp428: F64 = fTemp427 + fTemp426;
			let mut fTemp429: F64 = 0.5 * fTemp428;
			let mut fTemp430: F64 = 65535.0 * (1.0 - fTemp429);
			let mut iTemp431: i32 = (fTemp430) as i32;
			let mut iTemp432: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp431, 65535)))), 196607));
			let mut fTemp433: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp432, 3)) as usize] };
			let mut fTemp434: F64 = unsafe { ftbl0mydspSIG0[iTemp432 as usize] };
			let mut fTemp435: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp432, 1)) as usize] } - fTemp434;
			let mut fTemp436: F64 = 32767.5 * fTemp428;
			let mut iTemp437: i32 = (fTemp436) as i32;
			let mut iTemp438: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp437, 65535)))), 196607));
			let mut fTemp439: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp438, 3)) as usize] };
			let mut fTemp440: F64 = unsafe { ftbl0mydspSIG0[iTemp438 as usize] };
			let mut fTemp441: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp438, 1)) as usize] } - fTemp440;
			let mut fTemp442: F64 = if iTemp39 != 0 {fTemp440 + fTemp53 * fTemp441 + (fTemp436 - (iTemp437) as F64) * (fTemp439 - (fTemp440 + fTemp53 * (fTemp441 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp438, 4)) as usize] } - fTemp439))))} else {1.0 - (fTemp434 + fTemp53 * fTemp435 + (fTemp430 - (iTemp431) as F64) * (fTemp433 - (fTemp434 + fTemp53 * (fTemp435 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp432, 4)) as usize] } - fTemp433)))))};
			let mut fTemp443: F64 = fTemp58 + fTemp429;
			let mut fTemp444: F64 = 65535.0 * (1.0 - fTemp443);
			let mut iTemp445: i32 = (fTemp444) as i32;
			let mut iTemp446: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp445, 65535)))), 196607));
			let mut fTemp447: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp446, 3)) as usize] };
			let mut fTemp448: F64 = unsafe { ftbl0mydspSIG0[iTemp446 as usize] };
			let mut fTemp449: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp446, 1)) as usize] } - fTemp448;
			let mut fTemp450: F64 = 65535.0 * fTemp443;
			let mut iTemp451: i32 = (fTemp450) as i32;
			let mut iTemp452: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp451, 65535)))), 196607));
			let mut fTemp453: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp452, 3), 196607))) as usize] };
			let mut fTemp454: F64 = unsafe { ftbl0mydspSIG0[iTemp452 as usize] };
			let mut fTemp455: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp452, 1), 196607))) as usize] } - fTemp454;
			let mut iTemp456: i32 = (fTemp114 > ((if iTemp39 != 0 {fTemp454 + fTemp53 * fTemp455 + (fTemp450 - (iTemp451) as F64) * (fTemp453 - (fTemp454 + fTemp53 * (fTemp455 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp452, 4), 196607))) as usize] } - fTemp453))))} else {1.0 - (fTemp448 + fTemp53 * fTemp449 + (fTemp444 - (iTemp445) as F64) * (fTemp447 - (fTemp448 + fTemp53 * (fTemp449 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp446, 4)) as usize] } - fTemp447)))))} - fTemp442) / (1.0 - fTemp442))) as i32;
			let mut fTemp457: F64 = if iTemp456 != 0 {fTemp426} else {fTemp429};
			let mut fTemp458: F64 = if iTemp456 != 0 {fTemp429} else {fTemp427};
			let mut fTemp459: F64 = fTemp458 + fTemp457;
			let mut fTemp460: F64 = 0.5 * fTemp459;
			let mut fTemp461: F64 = 65535.0 * (1.0 - fTemp460);
			let mut iTemp462: i32 = (fTemp461) as i32;
			let mut iTemp463: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp462, 65535)))), 196607));
			let mut fTemp464: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp463, 3)) as usize] };
			let mut fTemp465: F64 = unsafe { ftbl0mydspSIG0[iTemp463 as usize] };
			let mut fTemp466: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp463, 1)) as usize] } - fTemp465;
			let mut fTemp467: F64 = 32767.5 * fTemp459;
			let mut iTemp468: i32 = (fTemp467) as i32;
			let mut iTemp469: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp468, 65535)))), 196607));
			let mut fTemp470: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp469, 3)) as usize] };
			let mut fTemp471: F64 = unsafe { ftbl0mydspSIG0[iTemp469 as usize] };
			let mut fTemp472: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp469, 1)) as usize] } - fTemp471;
			let mut fTemp473: F64 = if iTemp39 != 0 {fTemp471 + fTemp53 * fTemp472 + (fTemp467 - (iTemp468) as F64) * (fTemp470 - (fTemp471 + fTemp53 * (fTemp472 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp469, 4)) as usize] } - fTemp470))))} else {1.0 - (fTemp465 + fTemp53 * fTemp466 + (fTemp461 - (iTemp462) as F64) * (fTemp464 - (fTemp465 + fTemp53 * (fTemp466 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp463, 4)) as usize] } - fTemp464)))))};
			let mut fTemp474: F64 = fTemp58 + fTemp460;
			let mut fTemp475: F64 = 65535.0 * (1.0 - fTemp474);
			let mut iTemp476: i32 = (fTemp475) as i32;
			let mut iTemp477: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp476, 65535)))), 196607));
			let mut fTemp478: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp477, 3)) as usize] };
			let mut fTemp479: F64 = unsafe { ftbl0mydspSIG0[iTemp477 as usize] };
			let mut fTemp480: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp477, 1)) as usize] } - fTemp479;
			let mut fTemp481: F64 = 65535.0 * fTemp474;
			let mut iTemp482: i32 = (fTemp481) as i32;
			let mut iTemp483: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp482, 65535)))), 196607));
			let mut fTemp484: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp483, 3), 196607))) as usize] };
			let mut fTemp485: F64 = unsafe { ftbl0mydspSIG0[iTemp483 as usize] };
			let mut fTemp486: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp483, 1), 196607))) as usize] } - fTemp485;
			let mut iTemp487: i32 = (fTemp114 > ((if iTemp39 != 0 {fTemp485 + fTemp53 * fTemp486 + (fTemp481 - (iTemp482) as F64) * (fTemp484 - (fTemp485 + fTemp53 * (fTemp486 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp483, 4), 196607))) as usize] } - fTemp484))))} else {1.0 - (fTemp479 + fTemp53 * fTemp480 + (fTemp475 - (iTemp476) as F64) * (fTemp478 - (fTemp479 + fTemp53 * (fTemp480 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp477, 4)) as usize] } - fTemp478)))))} - fTemp473) / (1.0 - fTemp473))) as i32;
			let mut fTemp488: F64 = if iTemp487 != 0 {fTemp457} else {fTemp460};
			let mut fTemp489: F64 = if iTemp487 != 0 {fTemp460} else {fTemp458};
			let mut fTemp490: F64 = fTemp489 + fTemp488;
			let mut fTemp491: F64 = 0.5 * fTemp490;
			let mut fTemp492: F64 = 65535.0 * (1.0 - fTemp491);
			let mut iTemp493: i32 = (fTemp492) as i32;
			let mut iTemp494: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp493, 65535)))), 196607));
			let mut fTemp495: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp494, 3)) as usize] };
			let mut fTemp496: F64 = unsafe { ftbl0mydspSIG0[iTemp494 as usize] };
			let mut fTemp497: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp494, 1)) as usize] } - fTemp496;
			let mut fTemp498: F64 = 32767.5 * fTemp490;
			let mut iTemp499: i32 = (fTemp498) as i32;
			let mut iTemp500: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp499, 65535)))), 196607));
			let mut fTemp501: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp500, 3)) as usize] };
			let mut fTemp502: F64 = unsafe { ftbl0mydspSIG0[iTemp500 as usize] };
			let mut fTemp503: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp500, 1)) as usize] } - fTemp502;
			let mut fTemp504: F64 = if iTemp39 != 0 {fTemp502 + fTemp53 * fTemp503 + (fTemp498 - (iTemp499) as F64) * (fTemp501 - (fTemp502 + fTemp53 * (fTemp503 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp500, 4)) as usize] } - fTemp501))))} else {1.0 - (fTemp496 + fTemp53 * fTemp497 + (fTemp492 - (iTemp493) as F64) * (fTemp495 - (fTemp496 + fTemp53 * (fTemp497 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp494, 4)) as usize] } - fTemp495)))))};
			let mut fTemp505: F64 = fTemp58 + fTemp491;
			let mut fTemp506: F64 = 65535.0 * (1.0 - fTemp505);
			let mut iTemp507: i32 = (fTemp506) as i32;
			let mut iTemp508: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp507, 65535)))), 196607));
			let mut fTemp509: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp508, 3)) as usize] };
			let mut fTemp510: F64 = unsafe { ftbl0mydspSIG0[iTemp508 as usize] };
			let mut fTemp511: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp508, 1)) as usize] } - fTemp510;
			let mut fTemp512: F64 = 65535.0 * fTemp505;
			let mut iTemp513: i32 = (fTemp512) as i32;
			let mut iTemp514: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp513, 65535)))), 196607));
			let mut fTemp515: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp514, 3), 196607))) as usize] };
			let mut fTemp516: F64 = unsafe { ftbl0mydspSIG0[iTemp514 as usize] };
			let mut fTemp517: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp514, 1), 196607))) as usize] } - fTemp516;
			let mut iTemp518: i32 = (fTemp114 > ((if iTemp39 != 0 {fTemp516 + fTemp53 * fTemp517 + (fTemp512 - (iTemp513) as F64) * (fTemp515 - (fTemp516 + fTemp53 * (fTemp517 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp514, 4), 196607))) as usize] } - fTemp515))))} else {1.0 - (fTemp510 + fTemp53 * fTemp511 + (fTemp506 - (iTemp507) as F64) * (fTemp509 - (fTemp510 + fTemp53 * (fTemp511 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp508, 4)) as usize] } - fTemp509)))))} - fTemp504) / (1.0 - fTemp504))) as i32;
			let mut fTemp519: F64 = if iTemp518 != 0 {fTemp488} else {fTemp491};
			let mut fTemp520: F64 = if iTemp518 != 0 {fTemp491} else {fTemp489};
			let mut fTemp521: F64 = fTemp520 + fTemp519;
			let mut fTemp522: F64 = 0.5 * fTemp521;
			let mut fTemp523: F64 = 65535.0 * (1.0 - fTemp522);
			let mut iTemp524: i32 = (fTemp523) as i32;
			let mut iTemp525: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp524, 65535)))), 196607));
			let mut fTemp526: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp525, 3)) as usize] };
			let mut fTemp527: F64 = unsafe { ftbl0mydspSIG0[iTemp525 as usize] };
			let mut fTemp528: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp525, 1)) as usize] } - fTemp527;
			let mut fTemp529: F64 = 32767.5 * fTemp521;
			let mut iTemp530: i32 = (fTemp529) as i32;
			let mut iTemp531: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp530, 65535)))), 196607));
			let mut fTemp532: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp531, 3)) as usize] };
			let mut fTemp533: F64 = unsafe { ftbl0mydspSIG0[iTemp531 as usize] };
			let mut fTemp534: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp531, 1)) as usize] } - fTemp533;
			let mut fTemp535: F64 = if iTemp39 != 0 {fTemp533 + fTemp53 * fTemp534 + (fTemp529 - (iTemp530) as F64) * (fTemp532 - (fTemp533 + fTemp53 * (fTemp534 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp531, 4)) as usize] } - fTemp532))))} else {1.0 - (fTemp527 + fTemp53 * fTemp528 + (fTemp523 - (iTemp524) as F64) * (fTemp526 - (fTemp527 + fTemp53 * (fTemp528 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp525, 4)) as usize] } - fTemp526)))))};
			let mut fTemp536: F64 = fTemp58 + fTemp522;
			let mut fTemp537: F64 = 65535.0 * (1.0 - fTemp536);
			let mut iTemp538: i32 = (fTemp537) as i32;
			let mut iTemp539: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp538, 65535)))), 196607));
			let mut fTemp540: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp539, 3)) as usize] };
			let mut fTemp541: F64 = unsafe { ftbl0mydspSIG0[iTemp539 as usize] };
			let mut fTemp542: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp539, 1)) as usize] } - fTemp541;
			let mut fTemp543: F64 = 65535.0 * fTemp536;
			let mut iTemp544: i32 = (fTemp543) as i32;
			let mut iTemp545: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp544, 65535)))), 196607));
			let mut fTemp546: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp545, 3), 196607))) as usize] };
			let mut fTemp547: F64 = unsafe { ftbl0mydspSIG0[iTemp545 as usize] };
			let mut fTemp548: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp545, 1), 196607))) as usize] } - fTemp547;
			let mut iTemp549: i32 = (fTemp114 > ((if iTemp39 != 0 {fTemp547 + fTemp53 * fTemp548 + (fTemp543 - (iTemp544) as F64) * (fTemp546 - (fTemp547 + fTemp53 * (fTemp548 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp545, 4), 196607))) as usize] } - fTemp546))))} else {1.0 - (fTemp541 + fTemp53 * fTemp542 + (fTemp537 - (iTemp538) as F64) * (fTemp540 - (fTemp541 + fTemp53 * (fTemp542 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp539, 4)) as usize] } - fTemp540)))))} - fTemp535) / (1.0 - fTemp535))) as i32;
			let mut fTemp550: F64 = if iTemp549 != 0 {fTemp519} else {fTemp522};
			let mut fTemp551: F64 = if iTemp549 != 0 {fTemp522} else {fTemp520};
			let mut fTemp552: F64 = fTemp551 + fTemp550;
			let mut fTemp553: F64 = 0.5 * fTemp552;
			let mut fTemp554: F64 = 65535.0 * (1.0 - fTemp553);
			let mut iTemp555: i32 = (fTemp554) as i32;
			let mut iTemp556: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp555, 65535)))), 196607));
			let mut fTemp557: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp556, 3)) as usize] };
			let mut fTemp558: F64 = unsafe { ftbl0mydspSIG0[iTemp556 as usize] };
			let mut fTemp559: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp556, 1)) as usize] } - fTemp558;
			let mut fTemp560: F64 = 32767.5 * fTemp552;
			let mut iTemp561: i32 = (fTemp560) as i32;
			let mut iTemp562: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp561, 65535)))), 196607));
			let mut fTemp563: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp562, 3)) as usize] };
			let mut fTemp564: F64 = unsafe { ftbl0mydspSIG0[iTemp562 as usize] };
			let mut fTemp565: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp562, 1)) as usize] } - fTemp564;
			let mut fTemp566: F64 = if iTemp39 != 0 {fTemp564 + fTemp53 * fTemp565 + (fTemp560 - (iTemp561) as F64) * (fTemp563 - (fTemp564 + fTemp53 * (fTemp565 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp562, 4), 196607))) as usize] } - fTemp563))))} else {1.0 - (fTemp558 + fTemp53 * fTemp559 + (fTemp554 - (iTemp555) as F64) * (fTemp557 - (fTemp558 + fTemp53 * (fTemp559 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp556, 4), 196607))) as usize] } - fTemp557)))))};
			let mut fTemp567: F64 = fTemp58 + fTemp553;
			let mut fTemp568: F64 = 65535.0 * (1.0 - fTemp567);
			let mut iTemp569: i32 = (fTemp568) as i32;
			let mut iTemp570: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp569, 65535)))), 196607));
			let mut fTemp571: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp570, 3)) as usize] };
			let mut fTemp572: F64 = unsafe { ftbl0mydspSIG0[iTemp570 as usize] };
			let mut fTemp573: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp570, 1)) as usize] } - fTemp572;
			let mut fTemp574: F64 = 65535.0 * fTemp567;
			let mut iTemp575: i32 = (fTemp574) as i32;
			let mut iTemp576: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp575, 65535)))), 196607));
			let mut fTemp577: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp576, 3), 196607))) as usize] };
			let mut fTemp578: F64 = unsafe { ftbl0mydspSIG0[iTemp576 as usize] };
			let mut fTemp579: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp576, 1), 196607))) as usize] } - fTemp578;
			let mut iTemp580: i32 = (fTemp114 > ((if iTemp39 != 0 {fTemp578 + fTemp53 * fTemp579 + (fTemp574 - (iTemp575) as F64) * (fTemp577 - (fTemp578 + fTemp53 * (fTemp579 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp576, 4), 196607))) as usize] } - fTemp577))))} else {1.0 - (fTemp572 + fTemp53 * fTemp573 + (fTemp568 - (iTemp569) as F64) * (fTemp571 - (fTemp572 + fTemp53 * (fTemp573 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp570, 4)) as usize] } - fTemp571)))))} - fTemp566) / (1.0 - fTemp566))) as i32;
			let mut fTemp581: F64 = F64::min(1.0, F64::max(0.0, 0.5 * (if iTemp580 != 0 {fTemp553} else {fTemp551} + if iTemp580 != 0 {fTemp550} else {fTemp553})));
			self.fRec1[0] = fTemp581;
			let mut fTemp582: F64 = 65535.0 * (1.0 - fTemp581);
			let mut iTemp583: i32 = (fTemp582) as i32;
			let mut iTemp584: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp583, 65535)))), 196607));
			let mut fTemp585: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp584, 3)) as usize] };
			let mut fTemp586: F64 = unsafe { ftbl0mydspSIG0[iTemp584 as usize] };
			let mut fTemp587: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp584, 1)) as usize] } - fTemp586;
			let mut fTemp588: F64 = 65535.0 * fTemp581;
			let mut iTemp589: i32 = (fTemp588) as i32;
			let mut iTemp590: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp589, 65535)))), 196607));
			let mut fTemp591: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp590, 3)) as usize] };
			let mut fTemp592: F64 = unsafe { ftbl0mydspSIG0[iTemp590 as usize] };
			let mut fTemp593: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp590, 1)) as usize] } - fTemp592;
			let mut fTemp594: F64 = if iTemp39 != 0 {fTemp592 + fTemp53 * fTemp593 + (fTemp588 - (iTemp589) as F64) * (fTemp591 - (fTemp592 + fTemp53 * (fTemp593 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp590, 4), 196607))) as usize] } - fTemp591))))} else {1.0 - (fTemp586 + fTemp53 * fTemp587 + (fTemp582 - (iTemp583) as F64) * (fTemp585 - (fTemp586 + fTemp53 * (fTemp587 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp584, 4), 196607))) as usize] } - fTemp585)))))};
			let mut fTemp595: F64 = fTemp58 + fTemp581;
			let mut fTemp596: F64 = 65535.0 * (1.0 - fTemp595);
			let mut iTemp597: i32 = (fTemp596) as i32;
			let mut iTemp598: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp597, 65535)))), 196607));
			let mut fTemp599: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp598, 3)) as usize] };
			let mut fTemp600: F64 = unsafe { ftbl0mydspSIG0[iTemp598 as usize] };
			let mut fTemp601: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp598, 1)) as usize] } - fTemp600;
			let mut fTemp602: F64 = 65535.0 * fTemp595;
			let mut iTemp603: i32 = (fTemp602) as i32;
			let mut iTemp604: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp603, 65535)))), 196607));
			let mut fTemp605: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp604, 3), 196607))) as usize] };
			let mut fTemp606: F64 = unsafe { ftbl0mydspSIG0[iTemp604 as usize] };
			let mut fTemp607: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp604, 1), 196607))) as usize] } - fTemp606;
			let mut fTemp608: F64 = fTemp4 + if ((0.001 * fTemp57) == 0.0) as i32 != 0 {fTemp38} else {fTemp38 * (if iTemp39 != 0 {fTemp606 + fTemp53 * fTemp607 + (fTemp602 - (iTemp603) as F64) * (fTemp605 - (fTemp606 + fTemp53 * (fTemp607 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp604, 4), 196607))) as usize] } - fTemp605))))} else {1.0 - (fTemp600 + fTemp53 * fTemp601 + (fTemp596 - (iTemp597) as F64) * (fTemp599 - (fTemp600 + fTemp53 * (fTemp601 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp598, 4)) as usize] } - fTemp599)))))} - fTemp594) / (1.0 - fTemp594)};
			self.fRec2[(self.IOTA0 & 8191) as usize] = F64::min(self.fRec3[((i32::wrapping_sub(self.IOTA0, iSlow64)) & 16383) as usize], if iTemp56 != 0 {F64::min(fTemp608, fTemp4)} else {F64::max(fTemp608, fTemp4)});
			let mut fTemp609: F64 = self.fRec2[((i32::wrapping_sub(self.IOTA0, iSlow67)) & 8191) as usize];
			self.fVbargraph0 = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, fTemp609));
			self.fRec6[0] = fSlow69 + self.fConst4 * self.fRec6[1];
			*output0 = 0.5 * self.fVec0[((i32::wrapping_sub(self.IOTA0, iSlow68)) & 32767) as usize] * fTemp2 + self.fRec6[0] * self.fVec1[((i32::wrapping_sub(self.IOTA0, iSlow68)) & 32767) as usize] * fTemp609 * fTemp3;
			let mut fTemp610: F64 = self.fRec8[((i32::wrapping_sub(self.IOTA0, 1)) & 8191) as usize];
			let mut fTemp611: F64 = self.fRec9[((i32::wrapping_sub(self.IOTA0, 1)) & 16383) as usize];
			let mut fTemp612: F64 = fTemp11 + fSlow10 * (fTemp12 - fTemp11);
			let mut iTemp613: i32 = ((fTemp612 > fSlow11) as i32) + ((fTemp612 > fSlow8) as i32);
			let mut fTemp614: F64 = fTemp612 - fSlow7;
			let mut fTemp615: F64 = F64::min(1.0, F64::powf(1e+01, -(fSlow13 * F64::max(0.0, if (iTemp613 == 0) as i32 != 0 {0.0} else {if (iTemp613 == 1) as i32 != 0 {fSlow12 * mydsp_faustpower2_f(fSlow6 + fTemp614)} else {fTemp614}}))));
			self.fVec30[(self.IOTA0 & 16383) as usize] = fTemp615;
			let mut fTemp616: F64 = F64::min(fTemp615, self.fVec30[((i32::wrapping_sub(self.IOTA0, 1)) & 16383) as usize]);
			self.fVec31[0] = fTemp616;
			let mut fTemp617: F64 = F64::min(fTemp616, self.fVec31[2]);
			self.fVec32[0] = fTemp617;
			let mut fTemp618: F64 = F64::min(fTemp617, self.fVec32[4]);
			self.fVec33[0] = fTemp618;
			let mut fTemp619: F64 = F64::min(fTemp618, self.fVec33[8]);
			self.fVec34[(self.IOTA0 & 31) as usize] = fTemp619;
			let mut fTemp620: F64 = F64::min(fTemp619, self.fVec34[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize]);
			self.fVec35[(self.IOTA0 & 63) as usize] = fTemp620;
			let mut fTemp621: F64 = F64::min(fTemp620, self.fVec35[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize]);
			self.fVec36[(self.IOTA0 & 127) as usize] = fTemp621;
			let mut fTemp622: F64 = F64::min(fTemp621, self.fVec36[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize]);
			self.fVec37[(self.IOTA0 & 255) as usize] = fTemp622;
			let mut fTemp623: F64 = F64::min(fTemp622, self.fVec37[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize]);
			self.fVec38[(self.IOTA0 & 511) as usize] = fTemp623;
			let mut fTemp624: F64 = F64::min(fTemp623, self.fVec38[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize]);
			self.fVec39[(self.IOTA0 & 1023) as usize] = fTemp624;
			let mut fTemp625: F64 = F64::min(fTemp624, self.fVec39[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize]);
			self.fVec40[(self.IOTA0 & 2047) as usize] = fTemp625;
			self.fVec41[(self.IOTA0 & 4095) as usize] = F64::min(fTemp625, self.fVec40[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			self.fRec9[(self.IOTA0 & 16383) as usize] = F64::max(F64::min(fTemp611, self.fVec30[((i32::wrapping_sub(self.IOTA0, iSlow16)) & 16383) as usize]), F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow18 != 0 {fTemp615} else {1.7976931348623157e+308}, if iSlow19 != 0 {self.fVec31[iSlow18 as usize]} else {1.7976931348623157e+308}), if iSlow20 != 0 {self.fVec32[iSlow21 as usize]} else {1.7976931348623157e+308}), if iSlow22 != 0 {self.fVec33[iSlow23 as usize]} else {1.7976931348623157e+308}), if iSlow24 != 0 {self.fVec34[((i32::wrapping_sub(self.IOTA0, iSlow25)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow26 != 0 {self.fVec35[((i32::wrapping_sub(self.IOTA0, iSlow27)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow28 != 0 {self.fVec36[((i32::wrapping_sub(self.IOTA0, iSlow29)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow30 != 0 {self.fVec37[((i32::wrapping_sub(self.IOTA0, iSlow31)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow32 != 0 {self.fVec38[((i32::wrapping_sub(self.IOTA0, iSlow33)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow34 != 0 {self.fVec39[((i32::wrapping_sub(self.IOTA0, iSlow35)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow36 != 0 {self.fVec40[((i32::wrapping_sub(self.IOTA0, iSlow37)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow38 != 0 {self.fVec41[((i32::wrapping_sub(self.IOTA0, iSlow39)) & 4095) as usize]} else {1.7976931348623157e+308}));
			let mut fTemp626: F64 = self.fRec9[(self.IOTA0 & 16383) as usize];
			let mut fTemp627: F64 = F64::min(fTemp626, fTemp611);
			self.fVec42[0] = fTemp627;
			let mut fTemp628: F64 = F64::min(fTemp627, self.fVec42[2]);
			self.fVec43[0] = fTemp628;
			let mut fTemp629: F64 = F64::min(fTemp628, self.fVec43[4]);
			self.fVec44[0] = fTemp629;
			let mut fTemp630: F64 = F64::min(fTemp629, self.fVec44[8]);
			self.fVec45[(self.IOTA0 & 31) as usize] = fTemp630;
			let mut fTemp631: F64 = F64::min(fTemp630, self.fVec45[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize]);
			self.fVec46[(self.IOTA0 & 63) as usize] = fTemp631;
			let mut fTemp632: F64 = F64::min(fTemp631, self.fVec46[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize]);
			self.fVec47[(self.IOTA0 & 127) as usize] = fTemp632;
			let mut fTemp633: F64 = F64::min(fTemp632, self.fVec47[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize]);
			self.fVec48[(self.IOTA0 & 255) as usize] = fTemp633;
			let mut fTemp634: F64 = F64::min(fTemp633, self.fVec48[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize]);
			self.fVec49[(self.IOTA0 & 511) as usize] = fTemp634;
			let mut fTemp635: F64 = F64::min(fTemp634, self.fVec49[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize]);
			self.fVec50[(self.IOTA0 & 1023) as usize] = fTemp635;
			let mut fTemp636: F64 = F64::min(fTemp635, self.fVec50[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize]);
			self.fVec51[(self.IOTA0 & 2047) as usize] = fTemp636;
			self.fVec52[(self.IOTA0 & 4095) as usize] = F64::min(fTemp636, self.fVec51[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			let mut fTemp637: F64 = F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow4 != 0 {fTemp626} else {1.7976931348623157e+308}, if iSlow40 != 0 {self.fVec42[iSlow4 as usize]} else {1.7976931348623157e+308}), if iSlow41 != 0 {self.fVec43[iSlow42 as usize]} else {1.7976931348623157e+308}), if iSlow43 != 0 {self.fVec44[iSlow44 as usize]} else {1.7976931348623157e+308}), if iSlow45 != 0 {self.fVec45[((i32::wrapping_sub(self.IOTA0, iSlow46)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow47 != 0 {self.fVec46[((i32::wrapping_sub(self.IOTA0, iSlow48)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow49 != 0 {self.fVec47[((i32::wrapping_sub(self.IOTA0, iSlow50)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow51 != 0 {self.fVec48[((i32::wrapping_sub(self.IOTA0, iSlow52)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow53 != 0 {self.fVec49[((i32::wrapping_sub(self.IOTA0, iSlow54)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow55 != 0 {self.fVec50[((i32::wrapping_sub(self.IOTA0, iSlow56)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow57 != 0 {self.fVec51[((i32::wrapping_sub(self.IOTA0, iSlow58)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow59 != 0 {self.fVec52[((i32::wrapping_sub(self.IOTA0, iSlow60)) & 4095) as usize]} else {1.7976931348623157e+308}) - fTemp610;
			self.fVec53[0] = fTemp637;
			let mut iTemp638: i32 = (fTemp637 > 0.0) as i32;
			let mut fTemp639: F64 = if iTemp638 != 0 {fSlow62} else {fSlow61};
			self.fVec54[0] = fTemp639;
			let mut fTemp640: F64 = 2.0 * fTemp639;
			let mut iTemp641: i32 = (fTemp640) as i32;
			let mut iTemp642: i32 = std::cmp::max(0, std::cmp::min(iTemp641, 2));
			let mut iTemp643: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, 98301), 196607));
			let mut fTemp644: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp643, 3)) as usize] };
			let mut fTemp645: F64 = unsafe { ftbl0mydspSIG0[iTemp643 as usize] };
			let mut fTemp646: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp643, 1)) as usize] } - fTemp645;
			let mut fTemp647: F64 = fTemp640 - (iTemp641) as F64;
			let mut fTemp648: F64 = fTemp645 + fTemp647 * fTemp646 + 0.5 * (fTemp644 - (fTemp645 + fTemp647 * (fTemp646 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp643, 4)) as usize] } - fTemp644))));
			let mut fTemp649: F64 = if iTemp638 != 0 {fTemp648} else {1.0 - fTemp648};
			let mut iTemp650: i32 = (fTemp637 < 0.0) as i32;
			let mut fTemp651: F64 = fSlow1 * (iTemp650) as F64 + fSlow63 * (iTemp638) as F64;
			self.fVec55[0] = fTemp651;
			let mut fTemp652: F64 = self.fConst5 / fTemp651;
			let mut fTemp653: F64 = fTemp652 + 0.5;
			let mut fTemp654: F64 = 65535.0 * (1.0 - fTemp653);
			let mut iTemp655: i32 = (fTemp654) as i32;
			let mut iTemp656: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp655, 65535)))), 196607));
			let mut fTemp657: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp656, 3)) as usize] };
			let mut fTemp658: F64 = unsafe { ftbl0mydspSIG0[iTemp656 as usize] };
			let mut fTemp659: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp656, 1)) as usize] } - fTemp658;
			let mut fTemp660: F64 = 65535.0 * fTemp653;
			let mut iTemp661: i32 = (fTemp660) as i32;
			let mut iTemp662: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp661, 65535)))), 196607));
			let mut fTemp663: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp662, 3), 196607))) as usize] };
			let mut fTemp664: F64 = unsafe { ftbl0mydspSIG0[iTemp662 as usize] };
			let mut fTemp665: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp662, 1), 196607))) as usize] } - fTemp664;
			let mut fTemp666: F64 = 2.0 * self.fVec54[1];
			let mut iTemp667: i32 = (fTemp666) as i32;
			let mut iTemp668: i32 = std::cmp::max(0, std::cmp::min(iTemp667, 2));
			let mut fTemp669: F64 = 65535.0 * (1.0 - self.fRec7[1]);
			let mut iTemp670: i32 = (fTemp669) as i32;
			let mut iTemp671: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp670, 65535))), iTemp668), 196607));
			let mut fTemp672: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, 3), 196607))) as usize] };
			let mut fTemp673: F64 = unsafe { ftbl0mydspSIG0[iTemp671 as usize] };
			let mut fTemp674: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, 1), 196607))) as usize] } - fTemp673;
			let mut fTemp675: F64 = fTemp666 - (iTemp667) as F64;
			let mut fTemp676: F64 = 65535.0 * self.fRec7[1];
			let mut iTemp677: i32 = (fTemp676) as i32;
			let mut iTemp678: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp668, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp677, 65535)))), 196607));
			let mut fTemp679: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp678, 3), 196607))) as usize] };
			let mut fTemp680: F64 = unsafe { ftbl0mydspSIG0[iTemp678 as usize] };
			let mut fTemp681: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp678, 1), 196607))) as usize] } - fTemp680;
			let mut fTemp682: F64 = self.fRec7[1] + fTemp652;
			let mut fTemp683: F64 = 65535.0 * (1.0 - fTemp682);
			let mut iTemp684: i32 = (fTemp683) as i32;
			let mut iTemp685: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp684, 65535)))), 196607));
			let mut fTemp686: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp685, 3)) as usize] };
			let mut fTemp687: F64 = unsafe { ftbl0mydspSIG0[iTemp685 as usize] };
			let mut fTemp688: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp685, 1)) as usize] } - fTemp687;
			let mut fTemp689: F64 = 65535.0 * fTemp682;
			let mut iTemp690: i32 = (fTemp689) as i32;
			let mut iTemp691: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp690, 65535)))), 196607));
			let mut fTemp692: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp691, 3), 196607))) as usize] };
			let mut fTemp693: F64 = unsafe { ftbl0mydspSIG0[iTemp691 as usize] };
			let mut fTemp694: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp691, 1), 196607))) as usize] } - fTemp693;
			let mut fTemp695: F64 = self.fRec7[1] + self.fConst5 * (1.0 / fTemp651 + 1.0 / self.fVec55[1]);
			let mut fTemp696: F64 = 65535.0 * (1.0 - fTemp695);
			let mut iTemp697: i32 = (fTemp696) as i32;
			let mut iTemp698: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp697, 65535))), iTemp642), 196607));
			let mut fTemp699: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp698, 3)) as usize] };
			let mut fTemp700: F64 = unsafe { ftbl0mydspSIG0[iTemp698 as usize] };
			let mut fTemp701: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp698, 1)) as usize] } - fTemp700;
			let mut fTemp702: F64 = 65535.0 * fTemp695;
			let mut iTemp703: i32 = (fTemp702) as i32;
			let mut iTemp704: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp703, 65535)))), 196607));
			let mut fTemp705: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp704, 3), 196607))) as usize] };
			let mut fTemp706: F64 = unsafe { ftbl0mydspSIG0[iTemp704 as usize] };
			let mut fTemp707: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp704, 1), 196607))) as usize] } - fTemp706;
			let mut fTemp708: F64 = (if iTemp638 != 0 {fTemp706 + fTemp647 * fTemp707 + (fTemp702 - (iTemp703) as F64) * (fTemp705 - (fTemp706 + fTemp647 * (fTemp707 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp704, 4), 196607))) as usize] } - fTemp705))))} else {1.0 - (fTemp700 + fTemp647 * fTemp701 + (fTemp696 - (iTemp697) as F64) * (fTemp699 - (fTemp700 + fTemp647 * (fTemp701 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp698, 4)) as usize] } - fTemp699)))))} - if iTemp638 != 0 {fTemp693 + fTemp647 * fTemp694 + (fTemp689 - (iTemp690) as F64) * (fTemp692 - (fTemp693 + fTemp647 * (fTemp694 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp691, 4), 196607))) as usize] } - fTemp692))))} else {1.0 - (fTemp687 + fTemp647 * fTemp688 + (fTemp683 - (iTemp684) as F64) * (fTemp686 - (fTemp687 + fTemp647 * (fTemp688 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp685, 4), 196607))) as usize] } - fTemp686)))))}) * self.fVec53[1] / (fTemp637 * (1.0 - if iTemp638 != 0 {fTemp680 + fTemp675 * fTemp681 + (fTemp676 - (iTemp677) as F64) * (fTemp679 - (fTemp680 + fTemp675 * (fTemp681 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp678, 4), 196607))) as usize] } - fTemp679))))} else {1.0 - (fTemp673 + fTemp675 * fTemp674 + (fTemp669 - (iTemp670) as F64) * (fTemp672 - (fTemp673 + fTemp675 * (fTemp674 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, 4), 196607))) as usize] } - fTemp672)))))}));
			let mut iTemp709: i32 = (fTemp708 > ((if iTemp638 != 0 {fTemp664 + fTemp647 * fTemp665 + (fTemp660 - (iTemp661) as F64) * (fTemp663 - (fTemp664 + fTemp647 * (fTemp665 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp662, 4), 196607))) as usize] } - fTemp663))))} else {1.0 - (fTemp658 + fTemp647 * fTemp659 + (fTemp654 - (iTemp655) as F64) * (fTemp657 - (fTemp658 + fTemp647 * (fTemp659 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp656, 4)) as usize] } - fTemp657)))))} - fTemp649) / (1.0 - fTemp649))) as i32;
			let mut fTemp710: F64 = if iTemp709 != 0 {1.0} else {0.5};
			let mut fTemp711: F64 = if iTemp709 != 0 {0.5} else {0.0};
			let mut fTemp712: F64 = fTemp711 + fTemp710;
			let mut fTemp713: F64 = 0.5 * fTemp712;
			let mut fTemp714: F64 = 65535.0 * (1.0 - fTemp713);
			let mut iTemp715: i32 = (fTemp714) as i32;
			let mut iTemp716: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp715, 65535)))), 196607));
			let mut fTemp717: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp716, 3)) as usize] };
			let mut fTemp718: F64 = unsafe { ftbl0mydspSIG0[iTemp716 as usize] };
			let mut fTemp719: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp716, 1)) as usize] } - fTemp718;
			let mut fTemp720: F64 = 32767.5 * fTemp712;
			let mut iTemp721: i32 = (fTemp720) as i32;
			let mut iTemp722: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp721, 65535)))), 196607));
			let mut fTemp723: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp722, 3)) as usize] };
			let mut fTemp724: F64 = unsafe { ftbl0mydspSIG0[iTemp722 as usize] };
			let mut fTemp725: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp722, 1)) as usize] } - fTemp724;
			let mut fTemp726: F64 = if iTemp638 != 0 {fTemp724 + fTemp647 * fTemp725 + (fTemp720 - (iTemp721) as F64) * (fTemp723 - (fTemp724 + fTemp647 * (fTemp725 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp722, 4)) as usize] } - fTemp723))))} else {1.0 - (fTemp718 + fTemp647 * fTemp719 + (fTemp714 - (iTemp715) as F64) * (fTemp717 - (fTemp718 + fTemp647 * (fTemp719 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp716, 4)) as usize] } - fTemp717)))))};
			let mut fTemp727: F64 = fTemp652 + fTemp713;
			let mut fTemp728: F64 = 65535.0 * (1.0 - fTemp727);
			let mut iTemp729: i32 = (fTemp728) as i32;
			let mut iTemp730: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp729, 65535)))), 196607));
			let mut fTemp731: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp730, 3)) as usize] };
			let mut fTemp732: F64 = unsafe { ftbl0mydspSIG0[iTemp730 as usize] };
			let mut fTemp733: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp730, 1)) as usize] } - fTemp732;
			let mut fTemp734: F64 = 65535.0 * fTemp727;
			let mut iTemp735: i32 = (fTemp734) as i32;
			let mut iTemp736: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp735, 65535)))), 196607));
			let mut fTemp737: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp736, 3), 196607))) as usize] };
			let mut fTemp738: F64 = unsafe { ftbl0mydspSIG0[iTemp736 as usize] };
			let mut fTemp739: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp736, 1), 196607))) as usize] } - fTemp738;
			let mut iTemp740: i32 = (fTemp708 > ((if iTemp638 != 0 {fTemp738 + fTemp647 * fTemp739 + (fTemp734 - (iTemp735) as F64) * (fTemp737 - (fTemp738 + fTemp647 * (fTemp739 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp736, 4), 196607))) as usize] } - fTemp737))))} else {1.0 - (fTemp732 + fTemp647 * fTemp733 + (fTemp728 - (iTemp729) as F64) * (fTemp731 - (fTemp732 + fTemp647 * (fTemp733 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp730, 4)) as usize] } - fTemp731)))))} - fTemp726) / (1.0 - fTemp726))) as i32;
			let mut fTemp741: F64 = if iTemp740 != 0 {fTemp710} else {fTemp713};
			let mut fTemp742: F64 = if iTemp740 != 0 {fTemp713} else {fTemp711};
			let mut fTemp743: F64 = fTemp742 + fTemp741;
			let mut fTemp744: F64 = 0.5 * fTemp743;
			let mut fTemp745: F64 = 65535.0 * (1.0 - fTemp744);
			let mut iTemp746: i32 = (fTemp745) as i32;
			let mut iTemp747: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp746, 65535)))), 196607));
			let mut fTemp748: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp747, 3)) as usize] };
			let mut fTemp749: F64 = unsafe { ftbl0mydspSIG0[iTemp747 as usize] };
			let mut fTemp750: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp747, 1)) as usize] } - fTemp749;
			let mut fTemp751: F64 = 32767.5 * fTemp743;
			let mut iTemp752: i32 = (fTemp751) as i32;
			let mut iTemp753: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp752, 65535)))), 196607));
			let mut fTemp754: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp753, 3)) as usize] };
			let mut fTemp755: F64 = unsafe { ftbl0mydspSIG0[iTemp753 as usize] };
			let mut fTemp756: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp753, 1)) as usize] } - fTemp755;
			let mut fTemp757: F64 = if iTemp638 != 0 {fTemp755 + fTemp647 * fTemp756 + (fTemp751 - (iTemp752) as F64) * (fTemp754 - (fTemp755 + fTemp647 * (fTemp756 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp753, 4)) as usize] } - fTemp754))))} else {1.0 - (fTemp749 + fTemp647 * fTemp750 + (fTemp745 - (iTemp746) as F64) * (fTemp748 - (fTemp749 + fTemp647 * (fTemp750 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp747, 4)) as usize] } - fTemp748)))))};
			let mut fTemp758: F64 = fTemp652 + fTemp744;
			let mut fTemp759: F64 = 65535.0 * (1.0 - fTemp758);
			let mut iTemp760: i32 = (fTemp759) as i32;
			let mut iTemp761: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp760, 65535)))), 196607));
			let mut fTemp762: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp761, 3)) as usize] };
			let mut fTemp763: F64 = unsafe { ftbl0mydspSIG0[iTemp761 as usize] };
			let mut fTemp764: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp761, 1)) as usize] } - fTemp763;
			let mut fTemp765: F64 = 65535.0 * fTemp758;
			let mut iTemp766: i32 = (fTemp765) as i32;
			let mut iTemp767: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp766, 65535)))), 196607));
			let mut fTemp768: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp767, 3), 196607))) as usize] };
			let mut fTemp769: F64 = unsafe { ftbl0mydspSIG0[iTemp767 as usize] };
			let mut fTemp770: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp767, 1), 196607))) as usize] } - fTemp769;
			let mut iTemp771: i32 = (fTemp708 > ((if iTemp638 != 0 {fTemp769 + fTemp647 * fTemp770 + (fTemp765 - (iTemp766) as F64) * (fTemp768 - (fTemp769 + fTemp647 * (fTemp770 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp767, 4), 196607))) as usize] } - fTemp768))))} else {1.0 - (fTemp763 + fTemp647 * fTemp764 + (fTemp759 - (iTemp760) as F64) * (fTemp762 - (fTemp763 + fTemp647 * (fTemp764 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp761, 4)) as usize] } - fTemp762)))))} - fTemp757) / (1.0 - fTemp757))) as i32;
			let mut fTemp772: F64 = if iTemp771 != 0 {fTemp741} else {fTemp744};
			let mut fTemp773: F64 = if iTemp771 != 0 {fTemp744} else {fTemp742};
			let mut fTemp774: F64 = fTemp773 + fTemp772;
			let mut fTemp775: F64 = 0.5 * fTemp774;
			let mut fTemp776: F64 = 65535.0 * (1.0 - fTemp775);
			let mut iTemp777: i32 = (fTemp776) as i32;
			let mut iTemp778: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp777, 65535)))), 196607));
			let mut fTemp779: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp778, 3)) as usize] };
			let mut fTemp780: F64 = unsafe { ftbl0mydspSIG0[iTemp778 as usize] };
			let mut fTemp781: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp778, 1)) as usize] } - fTemp780;
			let mut fTemp782: F64 = 32767.5 * fTemp774;
			let mut iTemp783: i32 = (fTemp782) as i32;
			let mut iTemp784: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp783, 65535)))), 196607));
			let mut fTemp785: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp784, 3)) as usize] };
			let mut fTemp786: F64 = unsafe { ftbl0mydspSIG0[iTemp784 as usize] };
			let mut fTemp787: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp784, 1)) as usize] } - fTemp786;
			let mut fTemp788: F64 = if iTemp638 != 0 {fTemp786 + fTemp647 * fTemp787 + (fTemp782 - (iTemp783) as F64) * (fTemp785 - (fTemp786 + fTemp647 * (fTemp787 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp784, 4)) as usize] } - fTemp785))))} else {1.0 - (fTemp780 + fTemp647 * fTemp781 + (fTemp776 - (iTemp777) as F64) * (fTemp779 - (fTemp780 + fTemp647 * (fTemp781 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp778, 4)) as usize] } - fTemp779)))))};
			let mut fTemp789: F64 = fTemp652 + fTemp775;
			let mut fTemp790: F64 = 65535.0 * (1.0 - fTemp789);
			let mut iTemp791: i32 = (fTemp790) as i32;
			let mut iTemp792: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp791, 65535)))), 196607));
			let mut fTemp793: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp792, 3)) as usize] };
			let mut fTemp794: F64 = unsafe { ftbl0mydspSIG0[iTemp792 as usize] };
			let mut fTemp795: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp792, 1)) as usize] } - fTemp794;
			let mut fTemp796: F64 = 65535.0 * fTemp789;
			let mut iTemp797: i32 = (fTemp796) as i32;
			let mut iTemp798: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp797, 65535)))), 196607));
			let mut fTemp799: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp798, 3), 196607))) as usize] };
			let mut fTemp800: F64 = unsafe { ftbl0mydspSIG0[iTemp798 as usize] };
			let mut fTemp801: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp798, 1), 196607))) as usize] } - fTemp800;
			let mut iTemp802: i32 = (fTemp708 > ((if iTemp638 != 0 {fTemp800 + fTemp647 * fTemp801 + (fTemp796 - (iTemp797) as F64) * (fTemp799 - (fTemp800 + fTemp647 * (fTemp801 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp798, 4), 196607))) as usize] } - fTemp799))))} else {1.0 - (fTemp794 + fTemp647 * fTemp795 + (fTemp790 - (iTemp791) as F64) * (fTemp793 - (fTemp794 + fTemp647 * (fTemp795 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp792, 4)) as usize] } - fTemp793)))))} - fTemp788) / (1.0 - fTemp788))) as i32;
			let mut fTemp803: F64 = if iTemp802 != 0 {fTemp772} else {fTemp775};
			let mut fTemp804: F64 = if iTemp802 != 0 {fTemp775} else {fTemp773};
			let mut fTemp805: F64 = fTemp804 + fTemp803;
			let mut fTemp806: F64 = 0.5 * fTemp805;
			let mut fTemp807: F64 = 65535.0 * (1.0 - fTemp806);
			let mut iTemp808: i32 = (fTemp807) as i32;
			let mut iTemp809: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp808, 65535)))), 196607));
			let mut fTemp810: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp809, 3)) as usize] };
			let mut fTemp811: F64 = unsafe { ftbl0mydspSIG0[iTemp809 as usize] };
			let mut fTemp812: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp809, 1)) as usize] } - fTemp811;
			let mut fTemp813: F64 = 32767.5 * fTemp805;
			let mut iTemp814: i32 = (fTemp813) as i32;
			let mut iTemp815: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp814, 65535)))), 196607));
			let mut fTemp816: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp815, 3)) as usize] };
			let mut fTemp817: F64 = unsafe { ftbl0mydspSIG0[iTemp815 as usize] };
			let mut fTemp818: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp815, 1)) as usize] } - fTemp817;
			let mut fTemp819: F64 = if iTemp638 != 0 {fTemp817 + fTemp647 * fTemp818 + (fTemp813 - (iTemp814) as F64) * (fTemp816 - (fTemp817 + fTemp647 * (fTemp818 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp815, 4)) as usize] } - fTemp816))))} else {1.0 - (fTemp811 + fTemp647 * fTemp812 + (fTemp807 - (iTemp808) as F64) * (fTemp810 - (fTemp811 + fTemp647 * (fTemp812 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp809, 4)) as usize] } - fTemp810)))))};
			let mut fTemp820: F64 = fTemp652 + fTemp806;
			let mut fTemp821: F64 = 65535.0 * (1.0 - fTemp820);
			let mut iTemp822: i32 = (fTemp821) as i32;
			let mut iTemp823: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp822, 65535)))), 196607));
			let mut fTemp824: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp823, 3)) as usize] };
			let mut fTemp825: F64 = unsafe { ftbl0mydspSIG0[iTemp823 as usize] };
			let mut fTemp826: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp823, 1)) as usize] } - fTemp825;
			let mut fTemp827: F64 = 65535.0 * fTemp820;
			let mut iTemp828: i32 = (fTemp827) as i32;
			let mut iTemp829: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp828, 65535)))), 196607));
			let mut fTemp830: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp829, 3), 196607))) as usize] };
			let mut fTemp831: F64 = unsafe { ftbl0mydspSIG0[iTemp829 as usize] };
			let mut fTemp832: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp829, 1), 196607))) as usize] } - fTemp831;
			let mut iTemp833: i32 = (fTemp708 > ((if iTemp638 != 0 {fTemp831 + fTemp647 * fTemp832 + (fTemp827 - (iTemp828) as F64) * (fTemp830 - (fTemp831 + fTemp647 * (fTemp832 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp829, 4), 196607))) as usize] } - fTemp830))))} else {1.0 - (fTemp825 + fTemp647 * fTemp826 + (fTemp821 - (iTemp822) as F64) * (fTemp824 - (fTemp825 + fTemp647 * (fTemp826 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp823, 4)) as usize] } - fTemp824)))))} - fTemp819) / (1.0 - fTemp819))) as i32;
			let mut fTemp834: F64 = if iTemp833 != 0 {fTemp803} else {fTemp806};
			let mut fTemp835: F64 = if iTemp833 != 0 {fTemp806} else {fTemp804};
			let mut fTemp836: F64 = fTemp835 + fTemp834;
			let mut fTemp837: F64 = 0.5 * fTemp836;
			let mut fTemp838: F64 = 65535.0 * (1.0 - fTemp837);
			let mut iTemp839: i32 = (fTemp838) as i32;
			let mut iTemp840: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp839, 65535)))), 196607));
			let mut fTemp841: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp840, 3)) as usize] };
			let mut fTemp842: F64 = unsafe { ftbl0mydspSIG0[iTemp840 as usize] };
			let mut fTemp843: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp840, 1)) as usize] } - fTemp842;
			let mut fTemp844: F64 = 32767.5 * fTemp836;
			let mut iTemp845: i32 = (fTemp844) as i32;
			let mut iTemp846: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp845, 65535)))), 196607));
			let mut fTemp847: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp846, 3)) as usize] };
			let mut fTemp848: F64 = unsafe { ftbl0mydspSIG0[iTemp846 as usize] };
			let mut fTemp849: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp846, 1)) as usize] } - fTemp848;
			let mut fTemp850: F64 = if iTemp638 != 0 {fTemp848 + fTemp647 * fTemp849 + (fTemp844 - (iTemp845) as F64) * (fTemp847 - (fTemp848 + fTemp647 * (fTemp849 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp846, 4)) as usize] } - fTemp847))))} else {1.0 - (fTemp842 + fTemp647 * fTemp843 + (fTemp838 - (iTemp839) as F64) * (fTemp841 - (fTemp842 + fTemp647 * (fTemp843 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp840, 4)) as usize] } - fTemp841)))))};
			let mut fTemp851: F64 = fTemp652 + fTemp837;
			let mut fTemp852: F64 = 65535.0 * (1.0 - fTemp851);
			let mut iTemp853: i32 = (fTemp852) as i32;
			let mut iTemp854: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp853, 65535)))), 196607));
			let mut fTemp855: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp854, 3)) as usize] };
			let mut fTemp856: F64 = unsafe { ftbl0mydspSIG0[iTemp854 as usize] };
			let mut fTemp857: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp854, 1)) as usize] } - fTemp856;
			let mut fTemp858: F64 = 65535.0 * fTemp851;
			let mut iTemp859: i32 = (fTemp858) as i32;
			let mut iTemp860: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp859, 65535)))), 196607));
			let mut fTemp861: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp860, 3), 196607))) as usize] };
			let mut fTemp862: F64 = unsafe { ftbl0mydspSIG0[iTemp860 as usize] };
			let mut fTemp863: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp860, 1), 196607))) as usize] } - fTemp862;
			let mut iTemp864: i32 = (fTemp708 > ((if iTemp638 != 0 {fTemp862 + fTemp647 * fTemp863 + (fTemp858 - (iTemp859) as F64) * (fTemp861 - (fTemp862 + fTemp647 * (fTemp863 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp860, 4), 196607))) as usize] } - fTemp861))))} else {1.0 - (fTemp856 + fTemp647 * fTemp857 + (fTemp852 - (iTemp853) as F64) * (fTemp855 - (fTemp856 + fTemp647 * (fTemp857 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp854, 4)) as usize] } - fTemp855)))))} - fTemp850) / (1.0 - fTemp850))) as i32;
			let mut fTemp865: F64 = if iTemp864 != 0 {fTemp834} else {fTemp837};
			let mut fTemp866: F64 = if iTemp864 != 0 {fTemp837} else {fTemp835};
			let mut fTemp867: F64 = fTemp866 + fTemp865;
			let mut fTemp868: F64 = 0.5 * fTemp867;
			let mut fTemp869: F64 = 65535.0 * (1.0 - fTemp868);
			let mut iTemp870: i32 = (fTemp869) as i32;
			let mut iTemp871: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp870, 65535)))), 196607));
			let mut fTemp872: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp871, 3)) as usize] };
			let mut fTemp873: F64 = unsafe { ftbl0mydspSIG0[iTemp871 as usize] };
			let mut fTemp874: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp871, 1)) as usize] } - fTemp873;
			let mut fTemp875: F64 = 32767.5 * fTemp867;
			let mut iTemp876: i32 = (fTemp875) as i32;
			let mut iTemp877: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp876, 65535)))), 196607));
			let mut fTemp878: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp877, 3)) as usize] };
			let mut fTemp879: F64 = unsafe { ftbl0mydspSIG0[iTemp877 as usize] };
			let mut fTemp880: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp877, 1)) as usize] } - fTemp879;
			let mut fTemp881: F64 = if iTemp638 != 0 {fTemp879 + fTemp647 * fTemp880 + (fTemp875 - (iTemp876) as F64) * (fTemp878 - (fTemp879 + fTemp647 * (fTemp880 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp877, 4)) as usize] } - fTemp878))))} else {1.0 - (fTemp873 + fTemp647 * fTemp874 + (fTemp869 - (iTemp870) as F64) * (fTemp872 - (fTemp873 + fTemp647 * (fTemp874 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp871, 4)) as usize] } - fTemp872)))))};
			let mut fTemp882: F64 = fTemp652 + fTemp868;
			let mut fTemp883: F64 = 65535.0 * (1.0 - fTemp882);
			let mut iTemp884: i32 = (fTemp883) as i32;
			let mut iTemp885: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp884, 65535)))), 196607));
			let mut fTemp886: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp885, 3)) as usize] };
			let mut fTemp887: F64 = unsafe { ftbl0mydspSIG0[iTemp885 as usize] };
			let mut fTemp888: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp885, 1)) as usize] } - fTemp887;
			let mut fTemp889: F64 = 65535.0 * fTemp882;
			let mut iTemp890: i32 = (fTemp889) as i32;
			let mut iTemp891: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp890, 65535)))), 196607));
			let mut fTemp892: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp891, 3), 196607))) as usize] };
			let mut fTemp893: F64 = unsafe { ftbl0mydspSIG0[iTemp891 as usize] };
			let mut fTemp894: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp891, 1), 196607))) as usize] } - fTemp893;
			let mut iTemp895: i32 = (fTemp708 > ((if iTemp638 != 0 {fTemp893 + fTemp647 * fTemp894 + (fTemp889 - (iTemp890) as F64) * (fTemp892 - (fTemp893 + fTemp647 * (fTemp894 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp891, 4), 196607))) as usize] } - fTemp892))))} else {1.0 - (fTemp887 + fTemp647 * fTemp888 + (fTemp883 - (iTemp884) as F64) * (fTemp886 - (fTemp887 + fTemp647 * (fTemp888 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp885, 4)) as usize] } - fTemp886)))))} - fTemp881) / (1.0 - fTemp881))) as i32;
			let mut fTemp896: F64 = if iTemp895 != 0 {fTemp865} else {fTemp868};
			let mut fTemp897: F64 = if iTemp895 != 0 {fTemp868} else {fTemp866};
			let mut fTemp898: F64 = fTemp897 + fTemp896;
			let mut fTemp899: F64 = 0.5 * fTemp898;
			let mut fTemp900: F64 = 65535.0 * (1.0 - fTemp899);
			let mut iTemp901: i32 = (fTemp900) as i32;
			let mut iTemp902: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp901, 65535)))), 196607));
			let mut fTemp903: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp902, 3)) as usize] };
			let mut fTemp904: F64 = unsafe { ftbl0mydspSIG0[iTemp902 as usize] };
			let mut fTemp905: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp902, 1)) as usize] } - fTemp904;
			let mut fTemp906: F64 = 32767.5 * fTemp898;
			let mut iTemp907: i32 = (fTemp906) as i32;
			let mut iTemp908: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp907, 65535)))), 196607));
			let mut fTemp909: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp908, 3)) as usize] };
			let mut fTemp910: F64 = unsafe { ftbl0mydspSIG0[iTemp908 as usize] };
			let mut fTemp911: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp908, 1)) as usize] } - fTemp910;
			let mut fTemp912: F64 = if iTemp638 != 0 {fTemp910 + fTemp647 * fTemp911 + (fTemp906 - (iTemp907) as F64) * (fTemp909 - (fTemp910 + fTemp647 * (fTemp911 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp908, 4)) as usize] } - fTemp909))))} else {1.0 - (fTemp904 + fTemp647 * fTemp905 + (fTemp900 - (iTemp901) as F64) * (fTemp903 - (fTemp904 + fTemp647 * (fTemp905 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp902, 4)) as usize] } - fTemp903)))))};
			let mut fTemp913: F64 = fTemp652 + fTemp899;
			let mut fTemp914: F64 = 65535.0 * (1.0 - fTemp913);
			let mut iTemp915: i32 = (fTemp914) as i32;
			let mut iTemp916: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp915, 65535)))), 196607));
			let mut fTemp917: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp916, 3)) as usize] };
			let mut fTemp918: F64 = unsafe { ftbl0mydspSIG0[iTemp916 as usize] };
			let mut fTemp919: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp916, 1)) as usize] } - fTemp918;
			let mut fTemp920: F64 = 65535.0 * fTemp913;
			let mut iTemp921: i32 = (fTemp920) as i32;
			let mut iTemp922: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp921, 65535)))), 196607));
			let mut fTemp923: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp922, 3), 196607))) as usize] };
			let mut fTemp924: F64 = unsafe { ftbl0mydspSIG0[iTemp922 as usize] };
			let mut fTemp925: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp922, 1), 196607))) as usize] } - fTemp924;
			let mut iTemp926: i32 = (fTemp708 > ((if iTemp638 != 0 {fTemp924 + fTemp647 * fTemp925 + (fTemp920 - (iTemp921) as F64) * (fTemp923 - (fTemp924 + fTemp647 * (fTemp925 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp922, 4), 196607))) as usize] } - fTemp923))))} else {1.0 - (fTemp918 + fTemp647 * fTemp919 + (fTemp914 - (iTemp915) as F64) * (fTemp917 - (fTemp918 + fTemp647 * (fTemp919 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp916, 4)) as usize] } - fTemp917)))))} - fTemp912) / (1.0 - fTemp912))) as i32;
			let mut fTemp927: F64 = if iTemp926 != 0 {fTemp896} else {fTemp899};
			let mut fTemp928: F64 = if iTemp926 != 0 {fTemp899} else {fTemp897};
			let mut fTemp929: F64 = fTemp928 + fTemp927;
			let mut fTemp930: F64 = 0.5 * fTemp929;
			let mut fTemp931: F64 = 65535.0 * (1.0 - fTemp930);
			let mut iTemp932: i32 = (fTemp931) as i32;
			let mut iTemp933: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp932, 65535)))), 196607));
			let mut fTemp934: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp933, 3)) as usize] };
			let mut fTemp935: F64 = unsafe { ftbl0mydspSIG0[iTemp933 as usize] };
			let mut fTemp936: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp933, 1)) as usize] } - fTemp935;
			let mut fTemp937: F64 = 32767.5 * fTemp929;
			let mut iTemp938: i32 = (fTemp937) as i32;
			let mut iTemp939: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp938, 65535)))), 196607));
			let mut fTemp940: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp939, 3)) as usize] };
			let mut fTemp941: F64 = unsafe { ftbl0mydspSIG0[iTemp939 as usize] };
			let mut fTemp942: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp939, 1)) as usize] } - fTemp941;
			let mut fTemp943: F64 = if iTemp638 != 0 {fTemp941 + fTemp647 * fTemp942 + (fTemp937 - (iTemp938) as F64) * (fTemp940 - (fTemp941 + fTemp647 * (fTemp942 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp939, 4)) as usize] } - fTemp940))))} else {1.0 - (fTemp935 + fTemp647 * fTemp936 + (fTemp931 - (iTemp932) as F64) * (fTemp934 - (fTemp935 + fTemp647 * (fTemp936 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp933, 4)) as usize] } - fTemp934)))))};
			let mut fTemp944: F64 = fTemp652 + fTemp930;
			let mut fTemp945: F64 = 65535.0 * (1.0 - fTemp944);
			let mut iTemp946: i32 = (fTemp945) as i32;
			let mut iTemp947: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp946, 65535)))), 196607));
			let mut fTemp948: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp947, 3)) as usize] };
			let mut fTemp949: F64 = unsafe { ftbl0mydspSIG0[iTemp947 as usize] };
			let mut fTemp950: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp947, 1)) as usize] } - fTemp949;
			let mut fTemp951: F64 = 65535.0 * fTemp944;
			let mut iTemp952: i32 = (fTemp951) as i32;
			let mut iTemp953: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp952, 65535)))), 196607));
			let mut fTemp954: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp953, 3), 196607))) as usize] };
			let mut fTemp955: F64 = unsafe { ftbl0mydspSIG0[iTemp953 as usize] };
			let mut fTemp956: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp953, 1), 196607))) as usize] } - fTemp955;
			let mut iTemp957: i32 = (fTemp708 > ((if iTemp638 != 0 {fTemp955 + fTemp647 * fTemp956 + (fTemp951 - (iTemp952) as F64) * (fTemp954 - (fTemp955 + fTemp647 * (fTemp956 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp953, 4), 196607))) as usize] } - fTemp954))))} else {1.0 - (fTemp949 + fTemp647 * fTemp950 + (fTemp945 - (iTemp946) as F64) * (fTemp948 - (fTemp949 + fTemp647 * (fTemp950 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp947, 4)) as usize] } - fTemp948)))))} - fTemp943) / (1.0 - fTemp943))) as i32;
			let mut fTemp958: F64 = if iTemp957 != 0 {fTemp927} else {fTemp930};
			let mut fTemp959: F64 = if iTemp957 != 0 {fTemp930} else {fTemp928};
			let mut fTemp960: F64 = fTemp959 + fTemp958;
			let mut fTemp961: F64 = 0.5 * fTemp960;
			let mut fTemp962: F64 = 65535.0 * (1.0 - fTemp961);
			let mut iTemp963: i32 = (fTemp962) as i32;
			let mut iTemp964: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp963, 65535)))), 196607));
			let mut fTemp965: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp964, 3)) as usize] };
			let mut fTemp966: F64 = unsafe { ftbl0mydspSIG0[iTemp964 as usize] };
			let mut fTemp967: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp964, 1)) as usize] } - fTemp966;
			let mut fTemp968: F64 = 32767.5 * fTemp960;
			let mut iTemp969: i32 = (fTemp968) as i32;
			let mut iTemp970: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp969, 65535)))), 196607));
			let mut fTemp971: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp970, 3)) as usize] };
			let mut fTemp972: F64 = unsafe { ftbl0mydspSIG0[iTemp970 as usize] };
			let mut fTemp973: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp970, 1)) as usize] } - fTemp972;
			let mut fTemp974: F64 = if iTemp638 != 0 {fTemp972 + fTemp647 * fTemp973 + (fTemp968 - (iTemp969) as F64) * (fTemp971 - (fTemp972 + fTemp647 * (fTemp973 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp970, 4)) as usize] } - fTemp971))))} else {1.0 - (fTemp966 + fTemp647 * fTemp967 + (fTemp962 - (iTemp963) as F64) * (fTemp965 - (fTemp966 + fTemp647 * (fTemp967 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp964, 4)) as usize] } - fTemp965)))))};
			let mut fTemp975: F64 = fTemp652 + fTemp961;
			let mut fTemp976: F64 = 65535.0 * (1.0 - fTemp975);
			let mut iTemp977: i32 = (fTemp976) as i32;
			let mut iTemp978: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp977, 65535)))), 196607));
			let mut fTemp979: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp978, 3)) as usize] };
			let mut fTemp980: F64 = unsafe { ftbl0mydspSIG0[iTemp978 as usize] };
			let mut fTemp981: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp978, 1)) as usize] } - fTemp980;
			let mut fTemp982: F64 = 65535.0 * fTemp975;
			let mut iTemp983: i32 = (fTemp982) as i32;
			let mut iTemp984: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp983, 65535)))), 196607));
			let mut fTemp985: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp984, 3), 196607))) as usize] };
			let mut fTemp986: F64 = unsafe { ftbl0mydspSIG0[iTemp984 as usize] };
			let mut fTemp987: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp984, 1), 196607))) as usize] } - fTemp986;
			let mut iTemp988: i32 = (fTemp708 > ((if iTemp638 != 0 {fTemp986 + fTemp647 * fTemp987 + (fTemp982 - (iTemp983) as F64) * (fTemp985 - (fTemp986 + fTemp647 * (fTemp987 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp984, 4), 196607))) as usize] } - fTemp985))))} else {1.0 - (fTemp980 + fTemp647 * fTemp981 + (fTemp976 - (iTemp977) as F64) * (fTemp979 - (fTemp980 + fTemp647 * (fTemp981 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp978, 4)) as usize] } - fTemp979)))))} - fTemp974) / (1.0 - fTemp974))) as i32;
			let mut fTemp989: F64 = if iTemp988 != 0 {fTemp958} else {fTemp961};
			let mut fTemp990: F64 = if iTemp988 != 0 {fTemp961} else {fTemp959};
			let mut fTemp991: F64 = fTemp990 + fTemp989;
			let mut fTemp992: F64 = 0.5 * fTemp991;
			let mut fTemp993: F64 = 65535.0 * (1.0 - fTemp992);
			let mut iTemp994: i32 = (fTemp993) as i32;
			let mut iTemp995: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp994, 65535)))), 196607));
			let mut fTemp996: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp995, 3)) as usize] };
			let mut fTemp997: F64 = unsafe { ftbl0mydspSIG0[iTemp995 as usize] };
			let mut fTemp998: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp995, 1)) as usize] } - fTemp997;
			let mut fTemp999: F64 = 32767.5 * fTemp991;
			let mut iTemp1000: i32 = (fTemp999) as i32;
			let mut iTemp1001: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1000, 65535)))), 196607));
			let mut fTemp1002: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1001, 3)) as usize] };
			let mut fTemp1003: F64 = unsafe { ftbl0mydspSIG0[iTemp1001 as usize] };
			let mut fTemp1004: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1001, 1)) as usize] } - fTemp1003;
			let mut fTemp1005: F64 = if iTemp638 != 0 {fTemp1003 + fTemp647 * fTemp1004 + (fTemp999 - (iTemp1000) as F64) * (fTemp1002 - (fTemp1003 + fTemp647 * (fTemp1004 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1001, 4)) as usize] } - fTemp1002))))} else {1.0 - (fTemp997 + fTemp647 * fTemp998 + (fTemp993 - (iTemp994) as F64) * (fTemp996 - (fTemp997 + fTemp647 * (fTemp998 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp995, 4)) as usize] } - fTemp996)))))};
			let mut fTemp1006: F64 = fTemp652 + fTemp992;
			let mut fTemp1007: F64 = 65535.0 * (1.0 - fTemp1006);
			let mut iTemp1008: i32 = (fTemp1007) as i32;
			let mut iTemp1009: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1008, 65535)))), 196607));
			let mut fTemp1010: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1009, 3)) as usize] };
			let mut fTemp1011: F64 = unsafe { ftbl0mydspSIG0[iTemp1009 as usize] };
			let mut fTemp1012: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1009, 1)) as usize] } - fTemp1011;
			let mut fTemp1013: F64 = 65535.0 * fTemp1006;
			let mut iTemp1014: i32 = (fTemp1013) as i32;
			let mut iTemp1015: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1014, 65535)))), 196607));
			let mut fTemp1016: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1015, 3), 196607))) as usize] };
			let mut fTemp1017: F64 = unsafe { ftbl0mydspSIG0[iTemp1015 as usize] };
			let mut fTemp1018: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1015, 1), 196607))) as usize] } - fTemp1017;
			let mut iTemp1019: i32 = (fTemp708 > ((if iTemp638 != 0 {fTemp1017 + fTemp647 * fTemp1018 + (fTemp1013 - (iTemp1014) as F64) * (fTemp1016 - (fTemp1017 + fTemp647 * (fTemp1018 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1015, 4), 196607))) as usize] } - fTemp1016))))} else {1.0 - (fTemp1011 + fTemp647 * fTemp1012 + (fTemp1007 - (iTemp1008) as F64) * (fTemp1010 - (fTemp1011 + fTemp647 * (fTemp1012 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1009, 4)) as usize] } - fTemp1010)))))} - fTemp1005) / (1.0 - fTemp1005))) as i32;
			let mut fTemp1020: F64 = if iTemp1019 != 0 {fTemp989} else {fTemp992};
			let mut fTemp1021: F64 = if iTemp1019 != 0 {fTemp992} else {fTemp990};
			let mut fTemp1022: F64 = fTemp1021 + fTemp1020;
			let mut fTemp1023: F64 = 0.5 * fTemp1022;
			let mut fTemp1024: F64 = 65535.0 * (1.0 - fTemp1023);
			let mut iTemp1025: i32 = (fTemp1024) as i32;
			let mut iTemp1026: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1025, 65535)))), 196607));
			let mut fTemp1027: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1026, 3)) as usize] };
			let mut fTemp1028: F64 = unsafe { ftbl0mydspSIG0[iTemp1026 as usize] };
			let mut fTemp1029: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1026, 1)) as usize] } - fTemp1028;
			let mut fTemp1030: F64 = 32767.5 * fTemp1022;
			let mut iTemp1031: i32 = (fTemp1030) as i32;
			let mut iTemp1032: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1031, 65535)))), 196607));
			let mut fTemp1033: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1032, 3)) as usize] };
			let mut fTemp1034: F64 = unsafe { ftbl0mydspSIG0[iTemp1032 as usize] };
			let mut fTemp1035: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1032, 1)) as usize] } - fTemp1034;
			let mut fTemp1036: F64 = if iTemp638 != 0 {fTemp1034 + fTemp647 * fTemp1035 + (fTemp1030 - (iTemp1031) as F64) * (fTemp1033 - (fTemp1034 + fTemp647 * (fTemp1035 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1032, 4)) as usize] } - fTemp1033))))} else {1.0 - (fTemp1028 + fTemp647 * fTemp1029 + (fTemp1024 - (iTemp1025) as F64) * (fTemp1027 - (fTemp1028 + fTemp647 * (fTemp1029 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1026, 4)) as usize] } - fTemp1027)))))};
			let mut fTemp1037: F64 = fTemp652 + fTemp1023;
			let mut fTemp1038: F64 = 65535.0 * (1.0 - fTemp1037);
			let mut iTemp1039: i32 = (fTemp1038) as i32;
			let mut iTemp1040: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1039, 65535)))), 196607));
			let mut fTemp1041: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1040, 3)) as usize] };
			let mut fTemp1042: F64 = unsafe { ftbl0mydspSIG0[iTemp1040 as usize] };
			let mut fTemp1043: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1040, 1)) as usize] } - fTemp1042;
			let mut fTemp1044: F64 = 65535.0 * fTemp1037;
			let mut iTemp1045: i32 = (fTemp1044) as i32;
			let mut iTemp1046: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1045, 65535)))), 196607));
			let mut fTemp1047: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1046, 3), 196607))) as usize] };
			let mut fTemp1048: F64 = unsafe { ftbl0mydspSIG0[iTemp1046 as usize] };
			let mut fTemp1049: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1046, 1), 196607))) as usize] } - fTemp1048;
			let mut iTemp1050: i32 = (fTemp708 > ((if iTemp638 != 0 {fTemp1048 + fTemp647 * fTemp1049 + (fTemp1044 - (iTemp1045) as F64) * (fTemp1047 - (fTemp1048 + fTemp647 * (fTemp1049 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1046, 4), 196607))) as usize] } - fTemp1047))))} else {1.0 - (fTemp1042 + fTemp647 * fTemp1043 + (fTemp1038 - (iTemp1039) as F64) * (fTemp1041 - (fTemp1042 + fTemp647 * (fTemp1043 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1040, 4)) as usize] } - fTemp1041)))))} - fTemp1036) / (1.0 - fTemp1036))) as i32;
			let mut fTemp1051: F64 = if iTemp1050 != 0 {fTemp1020} else {fTemp1023};
			let mut fTemp1052: F64 = if iTemp1050 != 0 {fTemp1023} else {fTemp1021};
			let mut fTemp1053: F64 = fTemp1052 + fTemp1051;
			let mut fTemp1054: F64 = 0.5 * fTemp1053;
			let mut fTemp1055: F64 = 65535.0 * (1.0 - fTemp1054);
			let mut iTemp1056: i32 = (fTemp1055) as i32;
			let mut iTemp1057: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1056, 65535)))), 196607));
			let mut fTemp1058: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1057, 3)) as usize] };
			let mut fTemp1059: F64 = unsafe { ftbl0mydspSIG0[iTemp1057 as usize] };
			let mut fTemp1060: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1057, 1)) as usize] } - fTemp1059;
			let mut fTemp1061: F64 = 32767.5 * fTemp1053;
			let mut iTemp1062: i32 = (fTemp1061) as i32;
			let mut iTemp1063: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1062, 65535)))), 196607));
			let mut fTemp1064: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1063, 3)) as usize] };
			let mut fTemp1065: F64 = unsafe { ftbl0mydspSIG0[iTemp1063 as usize] };
			let mut fTemp1066: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1063, 1)) as usize] } - fTemp1065;
			let mut fTemp1067: F64 = if iTemp638 != 0 {fTemp1065 + fTemp647 * fTemp1066 + (fTemp1061 - (iTemp1062) as F64) * (fTemp1064 - (fTemp1065 + fTemp647 * (fTemp1066 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1063, 4)) as usize] } - fTemp1064))))} else {1.0 - (fTemp1059 + fTemp647 * fTemp1060 + (fTemp1055 - (iTemp1056) as F64) * (fTemp1058 - (fTemp1059 + fTemp647 * (fTemp1060 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1057, 4)) as usize] } - fTemp1058)))))};
			let mut fTemp1068: F64 = fTemp652 + fTemp1054;
			let mut fTemp1069: F64 = 65535.0 * (1.0 - fTemp1068);
			let mut iTemp1070: i32 = (fTemp1069) as i32;
			let mut iTemp1071: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1070, 65535)))), 196607));
			let mut fTemp1072: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1071, 3)) as usize] };
			let mut fTemp1073: F64 = unsafe { ftbl0mydspSIG0[iTemp1071 as usize] };
			let mut fTemp1074: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1071, 1)) as usize] } - fTemp1073;
			let mut fTemp1075: F64 = 65535.0 * fTemp1068;
			let mut iTemp1076: i32 = (fTemp1075) as i32;
			let mut iTemp1077: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1076, 65535)))), 196607));
			let mut fTemp1078: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1077, 3), 196607))) as usize] };
			let mut fTemp1079: F64 = unsafe { ftbl0mydspSIG0[iTemp1077 as usize] };
			let mut fTemp1080: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1077, 1), 196607))) as usize] } - fTemp1079;
			let mut iTemp1081: i32 = (fTemp708 > ((if iTemp638 != 0 {fTemp1079 + fTemp647 * fTemp1080 + (fTemp1075 - (iTemp1076) as F64) * (fTemp1078 - (fTemp1079 + fTemp647 * (fTemp1080 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1077, 4), 196607))) as usize] } - fTemp1078))))} else {1.0 - (fTemp1073 + fTemp647 * fTemp1074 + (fTemp1069 - (iTemp1070) as F64) * (fTemp1072 - (fTemp1073 + fTemp647 * (fTemp1074 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1071, 4)) as usize] } - fTemp1072)))))} - fTemp1067) / (1.0 - fTemp1067))) as i32;
			let mut fTemp1082: F64 = if iTemp1081 != 0 {fTemp1051} else {fTemp1054};
			let mut fTemp1083: F64 = if iTemp1081 != 0 {fTemp1054} else {fTemp1052};
			let mut fTemp1084: F64 = fTemp1083 + fTemp1082;
			let mut fTemp1085: F64 = 0.5 * fTemp1084;
			let mut fTemp1086: F64 = 65535.0 * (1.0 - fTemp1085);
			let mut iTemp1087: i32 = (fTemp1086) as i32;
			let mut iTemp1088: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1087, 65535)))), 196607));
			let mut fTemp1089: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1088, 3)) as usize] };
			let mut fTemp1090: F64 = unsafe { ftbl0mydspSIG0[iTemp1088 as usize] };
			let mut fTemp1091: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1088, 1)) as usize] } - fTemp1090;
			let mut fTemp1092: F64 = 32767.5 * fTemp1084;
			let mut iTemp1093: i32 = (fTemp1092) as i32;
			let mut iTemp1094: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1093, 65535)))), 196607));
			let mut fTemp1095: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1094, 3)) as usize] };
			let mut fTemp1096: F64 = unsafe { ftbl0mydspSIG0[iTemp1094 as usize] };
			let mut fTemp1097: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1094, 1)) as usize] } - fTemp1096;
			let mut fTemp1098: F64 = if iTemp638 != 0 {fTemp1096 + fTemp647 * fTemp1097 + (fTemp1092 - (iTemp1093) as F64) * (fTemp1095 - (fTemp1096 + fTemp647 * (fTemp1097 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1094, 4)) as usize] } - fTemp1095))))} else {1.0 - (fTemp1090 + fTemp647 * fTemp1091 + (fTemp1086 - (iTemp1087) as F64) * (fTemp1089 - (fTemp1090 + fTemp647 * (fTemp1091 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1088, 4)) as usize] } - fTemp1089)))))};
			let mut fTemp1099: F64 = fTemp652 + fTemp1085;
			let mut fTemp1100: F64 = 65535.0 * (1.0 - fTemp1099);
			let mut iTemp1101: i32 = (fTemp1100) as i32;
			let mut iTemp1102: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1101, 65535)))), 196607));
			let mut fTemp1103: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1102, 3)) as usize] };
			let mut fTemp1104: F64 = unsafe { ftbl0mydspSIG0[iTemp1102 as usize] };
			let mut fTemp1105: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1102, 1)) as usize] } - fTemp1104;
			let mut fTemp1106: F64 = 65535.0 * fTemp1099;
			let mut iTemp1107: i32 = (fTemp1106) as i32;
			let mut iTemp1108: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1107, 65535)))), 196607));
			let mut fTemp1109: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1108, 3), 196607))) as usize] };
			let mut fTemp1110: F64 = unsafe { ftbl0mydspSIG0[iTemp1108 as usize] };
			let mut fTemp1111: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1108, 1), 196607))) as usize] } - fTemp1110;
			let mut iTemp1112: i32 = (fTemp708 > ((if iTemp638 != 0 {fTemp1110 + fTemp647 * fTemp1111 + (fTemp1106 - (iTemp1107) as F64) * (fTemp1109 - (fTemp1110 + fTemp647 * (fTemp1111 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1108, 4), 196607))) as usize] } - fTemp1109))))} else {1.0 - (fTemp1104 + fTemp647 * fTemp1105 + (fTemp1100 - (iTemp1101) as F64) * (fTemp1103 - (fTemp1104 + fTemp647 * (fTemp1105 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1102, 4)) as usize] } - fTemp1103)))))} - fTemp1098) / (1.0 - fTemp1098))) as i32;
			let mut fTemp1113: F64 = if iTemp1112 != 0 {fTemp1082} else {fTemp1085};
			let mut fTemp1114: F64 = if iTemp1112 != 0 {fTemp1085} else {fTemp1083};
			let mut fTemp1115: F64 = fTemp1114 + fTemp1113;
			let mut fTemp1116: F64 = 0.5 * fTemp1115;
			let mut fTemp1117: F64 = 65535.0 * (1.0 - fTemp1116);
			let mut iTemp1118: i32 = (fTemp1117) as i32;
			let mut iTemp1119: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1118, 65535)))), 196607));
			let mut fTemp1120: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1119, 3)) as usize] };
			let mut fTemp1121: F64 = unsafe { ftbl0mydspSIG0[iTemp1119 as usize] };
			let mut fTemp1122: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1119, 1)) as usize] } - fTemp1121;
			let mut fTemp1123: F64 = 32767.5 * fTemp1115;
			let mut iTemp1124: i32 = (fTemp1123) as i32;
			let mut iTemp1125: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1124, 65535)))), 196607));
			let mut fTemp1126: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1125, 3)) as usize] };
			let mut fTemp1127: F64 = unsafe { ftbl0mydspSIG0[iTemp1125 as usize] };
			let mut fTemp1128: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1125, 1)) as usize] } - fTemp1127;
			let mut fTemp1129: F64 = if iTemp638 != 0 {fTemp1127 + fTemp647 * fTemp1128 + (fTemp1123 - (iTemp1124) as F64) * (fTemp1126 - (fTemp1127 + fTemp647 * (fTemp1128 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1125, 4)) as usize] } - fTemp1126))))} else {1.0 - (fTemp1121 + fTemp647 * fTemp1122 + (fTemp1117 - (iTemp1118) as F64) * (fTemp1120 - (fTemp1121 + fTemp647 * (fTemp1122 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1119, 4)) as usize] } - fTemp1120)))))};
			let mut fTemp1130: F64 = fTemp652 + fTemp1116;
			let mut fTemp1131: F64 = 65535.0 * (1.0 - fTemp1130);
			let mut iTemp1132: i32 = (fTemp1131) as i32;
			let mut iTemp1133: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1132, 65535)))), 196607));
			let mut fTemp1134: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1133, 3)) as usize] };
			let mut fTemp1135: F64 = unsafe { ftbl0mydspSIG0[iTemp1133 as usize] };
			let mut fTemp1136: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1133, 1)) as usize] } - fTemp1135;
			let mut fTemp1137: F64 = 65535.0 * fTemp1130;
			let mut iTemp1138: i32 = (fTemp1137) as i32;
			let mut iTemp1139: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1138, 65535)))), 196607));
			let mut fTemp1140: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1139, 3), 196607))) as usize] };
			let mut fTemp1141: F64 = unsafe { ftbl0mydspSIG0[iTemp1139 as usize] };
			let mut fTemp1142: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1139, 1), 196607))) as usize] } - fTemp1141;
			let mut iTemp1143: i32 = (fTemp708 > ((if iTemp638 != 0 {fTemp1141 + fTemp647 * fTemp1142 + (fTemp1137 - (iTemp1138) as F64) * (fTemp1140 - (fTemp1141 + fTemp647 * (fTemp1142 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1139, 4), 196607))) as usize] } - fTemp1140))))} else {1.0 - (fTemp1135 + fTemp647 * fTemp1136 + (fTemp1131 - (iTemp1132) as F64) * (fTemp1134 - (fTemp1135 + fTemp647 * (fTemp1136 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1133, 4)) as usize] } - fTemp1134)))))} - fTemp1129) / (1.0 - fTemp1129))) as i32;
			let mut fTemp1144: F64 = if iTemp1143 != 0 {fTemp1113} else {fTemp1116};
			let mut fTemp1145: F64 = if iTemp1143 != 0 {fTemp1116} else {fTemp1114};
			let mut fTemp1146: F64 = fTemp1145 + fTemp1144;
			let mut fTemp1147: F64 = 0.5 * fTemp1146;
			let mut fTemp1148: F64 = 65535.0 * (1.0 - fTemp1147);
			let mut iTemp1149: i32 = (fTemp1148) as i32;
			let mut iTemp1150: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1149, 65535)))), 196607));
			let mut fTemp1151: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1150, 3)) as usize] };
			let mut fTemp1152: F64 = unsafe { ftbl0mydspSIG0[iTemp1150 as usize] };
			let mut fTemp1153: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1150, 1)) as usize] } - fTemp1152;
			let mut fTemp1154: F64 = 32767.5 * fTemp1146;
			let mut iTemp1155: i32 = (fTemp1154) as i32;
			let mut iTemp1156: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1155, 65535)))), 196607));
			let mut fTemp1157: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1156, 3)) as usize] };
			let mut fTemp1158: F64 = unsafe { ftbl0mydspSIG0[iTemp1156 as usize] };
			let mut fTemp1159: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1156, 1)) as usize] } - fTemp1158;
			let mut fTemp1160: F64 = if iTemp638 != 0 {fTemp1158 + fTemp647 * fTemp1159 + (fTemp1154 - (iTemp1155) as F64) * (fTemp1157 - (fTemp1158 + fTemp647 * (fTemp1159 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1156, 4), 196607))) as usize] } - fTemp1157))))} else {1.0 - (fTemp1152 + fTemp647 * fTemp1153 + (fTemp1148 - (iTemp1149) as F64) * (fTemp1151 - (fTemp1152 + fTemp647 * (fTemp1153 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1150, 4), 196607))) as usize] } - fTemp1151)))))};
			let mut fTemp1161: F64 = fTemp652 + fTemp1147;
			let mut fTemp1162: F64 = 65535.0 * (1.0 - fTemp1161);
			let mut iTemp1163: i32 = (fTemp1162) as i32;
			let mut iTemp1164: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1163, 65535)))), 196607));
			let mut fTemp1165: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1164, 3)) as usize] };
			let mut fTemp1166: F64 = unsafe { ftbl0mydspSIG0[iTemp1164 as usize] };
			let mut fTemp1167: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1164, 1)) as usize] } - fTemp1166;
			let mut fTemp1168: F64 = 65535.0 * fTemp1161;
			let mut iTemp1169: i32 = (fTemp1168) as i32;
			let mut iTemp1170: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1169, 65535)))), 196607));
			let mut fTemp1171: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1170, 3), 196607))) as usize] };
			let mut fTemp1172: F64 = unsafe { ftbl0mydspSIG0[iTemp1170 as usize] };
			let mut fTemp1173: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1170, 1), 196607))) as usize] } - fTemp1172;
			let mut iTemp1174: i32 = (fTemp708 > ((if iTemp638 != 0 {fTemp1172 + fTemp647 * fTemp1173 + (fTemp1168 - (iTemp1169) as F64) * (fTemp1171 - (fTemp1172 + fTemp647 * (fTemp1173 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1170, 4), 196607))) as usize] } - fTemp1171))))} else {1.0 - (fTemp1166 + fTemp647 * fTemp1167 + (fTemp1162 - (iTemp1163) as F64) * (fTemp1165 - (fTemp1166 + fTemp647 * (fTemp1167 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1164, 4)) as usize] } - fTemp1165)))))} - fTemp1160) / (1.0 - fTemp1160))) as i32;
			let mut fTemp1175: F64 = F64::min(1.0, F64::max(0.0, 0.5 * (if iTemp1174 != 0 {fTemp1147} else {fTemp1145} + if iTemp1174 != 0 {fTemp1144} else {fTemp1147})));
			self.fRec7[0] = fTemp1175;
			let mut fTemp1176: F64 = 65535.0 * (1.0 - fTemp1175);
			let mut iTemp1177: i32 = (fTemp1176) as i32;
			let mut iTemp1178: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1177, 65535)))), 196607));
			let mut fTemp1179: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1178, 3)) as usize] };
			let mut fTemp1180: F64 = unsafe { ftbl0mydspSIG0[iTemp1178 as usize] };
			let mut fTemp1181: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1178, 1)) as usize] } - fTemp1180;
			let mut fTemp1182: F64 = 65535.0 * fTemp1175;
			let mut iTemp1183: i32 = (fTemp1182) as i32;
			let mut iTemp1184: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1183, 65535)))), 196607));
			let mut fTemp1185: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1184, 3)) as usize] };
			let mut fTemp1186: F64 = unsafe { ftbl0mydspSIG0[iTemp1184 as usize] };
			let mut fTemp1187: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1184, 1)) as usize] } - fTemp1186;
			let mut fTemp1188: F64 = if iTemp638 != 0 {fTemp1186 + fTemp647 * fTemp1187 + (fTemp1182 - (iTemp1183) as F64) * (fTemp1185 - (fTemp1186 + fTemp647 * (fTemp1187 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1184, 4), 196607))) as usize] } - fTemp1185))))} else {1.0 - (fTemp1180 + fTemp647 * fTemp1181 + (fTemp1176 - (iTemp1177) as F64) * (fTemp1179 - (fTemp1180 + fTemp647 * (fTemp1181 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1178, 4), 196607))) as usize] } - fTemp1179)))))};
			let mut fTemp1189: F64 = fTemp652 + fTemp1175;
			let mut fTemp1190: F64 = 65535.0 * (1.0 - fTemp1189);
			let mut iTemp1191: i32 = (fTemp1190) as i32;
			let mut iTemp1192: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1191, 65535)))), 196607));
			let mut fTemp1193: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1192, 3)) as usize] };
			let mut fTemp1194: F64 = unsafe { ftbl0mydspSIG0[iTemp1192 as usize] };
			let mut fTemp1195: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1192, 1)) as usize] } - fTemp1194;
			let mut fTemp1196: F64 = 65535.0 * fTemp1189;
			let mut iTemp1197: i32 = (fTemp1196) as i32;
			let mut iTemp1198: i32 = std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, i32::wrapping_mul(3, std::cmp::max(0, std::cmp::min(iTemp1197, 65535)))), 196607));
			let mut fTemp1199: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1198, 3), 196607))) as usize] };
			let mut fTemp1200: F64 = unsafe { ftbl0mydspSIG0[iTemp1198 as usize] };
			let mut fTemp1201: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1198, 1), 196607))) as usize] } - fTemp1200;
			let mut fTemp1202: F64 = fTemp610 + if ((0.001 * fTemp651) == 0.0) as i32 != 0 {fTemp637} else {fTemp637 * (if iTemp638 != 0 {fTemp1200 + fTemp647 * fTemp1201 + (fTemp1196 - (iTemp1197) as F64) * (fTemp1199 - (fTemp1200 + fTemp647 * (fTemp1201 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1198, 4), 196607))) as usize] } - fTemp1199))))} else {1.0 - (fTemp1194 + fTemp647 * fTemp1195 + (fTemp1190 - (iTemp1191) as F64) * (fTemp1193 - (fTemp1194 + fTemp647 * (fTemp1195 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1192, 4)) as usize] } - fTemp1193)))))} - fTemp1188) / (1.0 - fTemp1188)};
			self.fRec8[(self.IOTA0 & 8191) as usize] = F64::min(self.fRec9[((i32::wrapping_sub(self.IOTA0, iSlow64)) & 16383) as usize], if iTemp650 != 0 {F64::min(fTemp1202, fTemp610)} else {F64::max(fTemp1202, fTemp610)});
			let mut fTemp1203: F64 = self.fRec8[((i32::wrapping_sub(self.IOTA0, iSlow67)) & 8191) as usize];
			self.fVbargraph1 = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, fTemp1203));
			*output1 = 0.5 * fTemp2 * self.fVec2[((i32::wrapping_sub(self.IOTA0, iSlow68)) & 32767) as usize] + self.fRec6[0] * fTemp3 * self.fVec3[((i32::wrapping_sub(self.IOTA0, iSlow68)) & 32767) as usize] * fTemp1203;
			self.fRec0[1] = self.fRec0[0];
			self.IOTA0 = i32::wrapping_add(self.IOTA0, 1);
			self.fRec4[1] = self.fRec4[0];
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
			self.fRec1[1] = self.fRec1[0];
			self.fRec6[1] = self.fRec6[0];
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
		}
	}

}


}
pub use dsp::mydsp as LambRs;
