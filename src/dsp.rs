mod dsp {
    /* ------------------------------------------------------------
author: "Bart Brouns"
license: "AGPLv3"
name: "lamb"
version: "0.1"
Code generated with Faust 2.72.10 (https://faust.grame.fr)
Compilation options: -a /run/user/1001/.tmpsojc2U -lang rust -ct 1 -es 1 -mcd 16 -mdd 1024 -mdy 33 -double -ftz 0
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
	iRec4: [i32;2],
}

impl mydspSIG0 {
	
	fn get_num_inputsmydspSIG0(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsmydspSIG0(&self) -> i32 {
		return 1;
	}
	
	fn instance_initmydspSIG0(&mut self, sample_rate: i32) {
		for l28 in 0..2 {
			self.iRec4[l28 as usize] = 0;
		}
	}
	
	fn fillmydspSIG0(&mut self, count: i32, table: &mut[F64]) {
		for i1 in 0..count {
			self.iRec4[0] = i32::wrapping_add(self.iRec4[1], 1);
			let mut iTemp33: i32 = i32::wrapping_add(self.iRec4[0], -1);
			let mut fTemp34: F64 = (iTemp33 % 3) as F64 as i32 as F64;
			let mut fTemp35: F64 = 0.5 * fTemp34;
			let mut fTemp36: F64 = F64::powf(fTemp35, 0.21 * fTemp34 + 1.0);
			let mut fTemp37: F64 = (0.3333333333333333 * (iTemp33 % 196608) as F64) as i32 as F64;
			table[i1 as usize] = F64::min(1.0, F64::max(0.0, if (fTemp35 == 0.0) as i32 != 0 {0.5 * (F64::sin(4.793763109162727e-05 * fTemp37 + 4.71238898038469) + 1.0)} else {0.5 * (F64::sin(3.141592653589793 * ((1.0 - F64::exp(-(3.692683299000534e-05 * fTemp36 * fTemp37))) / (1.0 - F64::exp(-(2.42 * fTemp36)))) + 4.71238898038469) + 1.0)}));
			self.iRec4[1] = self.iRec4[0];
		}
	}

}


pub fn newmydspSIG0() -> mydspSIG0 { 
	mydspSIG0 {
		iRec4: [0;2],
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
	fSampleRate: i32,
	fConst1: F64,
	fConst2: F64,
	fHslider0: F64,
	fRec0: [F64;2],
	IOTA0: i32,
	fVec0: [F64;32768],
	fHslider1: F64,
	fHslider2: F64,
	fConst3: F64,
	fHslider3: F64,
	fHslider4: F64,
	fVec1: [F64;32768],
	fHslider5: F64,
	fHslider6: F64,
	fVec2: [F64;16384],
	fVec3: [F64;3],
	fVec4: [F64;7],
	fVec5: [F64;15],
	fVec6: [F64;32],
	fVec7: [F64;64],
	fVec8: [F64;128],
	fVec9: [F64;256],
	fVec10: [F64;512],
	fVec11: [F64;1024],
	fVec12: [F64;2048],
	fVec13: [F64;4096],
	fRec3: [F64;16384],
	fVec14: [F64;3],
	fVec15: [F64;7],
	fVec16: [F64;15],
	fVec17: [F64;32],
	fVec18: [F64;64],
	fVec19: [F64;128],
	fVec20: [F64;256],
	fVec21: [F64;512],
	fVec22: [F64;1024],
	fVec23: [F64;2048],
	fVec24: [F64;4096],
	fVec25: [F64;2],
	fHslider7: F64,
	fHslider8: F64,
	fVec26: [F64;2],
	fHslider9: F64,
	fVec27: [F64;2],
	fConst4: F64,
	fRec1: [F64;2],
	fRec2: [F64;2],
	fVbargraph0: F64,
	fVec28: [F64;16384],
	fVec29: [F64;3],
	fVec30: [F64;7],
	fVec31: [F64;15],
	fVec32: [F64;32],
	fVec33: [F64;64],
	fVec34: [F64;128],
	fVec35: [F64;256],
	fVec36: [F64;512],
	fVec37: [F64;1024],
	fVec38: [F64;2048],
	fVec39: [F64;4096],
	fRec7: [F64;16384],
	fVec40: [F64;3],
	fVec41: [F64;7],
	fVec42: [F64;15],
	fVec43: [F64;32],
	fVec44: [F64;64],
	fVec45: [F64;128],
	fVec46: [F64;256],
	fVec47: [F64;512],
	fVec48: [F64;1024],
	fVec49: [F64;2048],
	fVec50: [F64;4096],
	fVec51: [F64;2],
	fVec52: [F64;2],
	fVec53: [F64;2],
	fRec5: [F64;2],
	fRec6: [F64;2],
	fVbargraph1: F64,
}

impl FaustDsp for mydsp {
	type T = F64;
		
	fn new() -> mydsp { 
		mydsp {
			fSampleRate: 0,
			fConst1: 0.0,
			fConst2: 0.0,
			fHslider0: 0.0,
			fRec0: [0.0;2],
			IOTA0: 0,
			fVec0: [0.0;32768],
			fHslider1: 0.0,
			fHslider2: 0.0,
			fConst3: 0.0,
			fHslider3: 0.0,
			fHslider4: 0.0,
			fVec1: [0.0;32768],
			fHslider5: 0.0,
			fHslider6: 0.0,
			fVec2: [0.0;16384],
			fVec3: [0.0;3],
			fVec4: [0.0;7],
			fVec5: [0.0;15],
			fVec6: [0.0;32],
			fVec7: [0.0;64],
			fVec8: [0.0;128],
			fVec9: [0.0;256],
			fVec10: [0.0;512],
			fVec11: [0.0;1024],
			fVec12: [0.0;2048],
			fVec13: [0.0;4096],
			fRec3: [0.0;16384],
			fVec14: [0.0;3],
			fVec15: [0.0;7],
			fVec16: [0.0;15],
			fVec17: [0.0;32],
			fVec18: [0.0;64],
			fVec19: [0.0;128],
			fVec20: [0.0;256],
			fVec21: [0.0;512],
			fVec22: [0.0;1024],
			fVec23: [0.0;2048],
			fVec24: [0.0;4096],
			fVec25: [0.0;2],
			fHslider7: 0.0,
			fHslider8: 0.0,
			fVec26: [0.0;2],
			fHslider9: 0.0,
			fVec27: [0.0;2],
			fConst4: 0.0,
			fRec1: [0.0;2],
			fRec2: [0.0;2],
			fVbargraph0: 0.0,
			fVec28: [0.0;16384],
			fVec29: [0.0;3],
			fVec30: [0.0;7],
			fVec31: [0.0;15],
			fVec32: [0.0;32],
			fVec33: [0.0;64],
			fVec34: [0.0;128],
			fVec35: [0.0;256],
			fVec36: [0.0;512],
			fVec37: [0.0;1024],
			fVec38: [0.0;2048],
			fVec39: [0.0;4096],
			fRec7: [0.0;16384],
			fVec40: [0.0;3],
			fVec41: [0.0;7],
			fVec42: [0.0;15],
			fVec43: [0.0;32],
			fVec44: [0.0;64],
			fVec45: [0.0;128],
			fVec46: [0.0;256],
			fVec47: [0.0;512],
			fVec48: [0.0;1024],
			fVec49: [0.0;2048],
			fVec50: [0.0;4096],
			fVec51: [0.0;2],
			fVec52: [0.0;2],
			fVec53: [0.0;2],
			fRec5: [0.0;2],
			fRec6: [0.0;2],
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
		m.declare("compile_options", r"-a /run/user/1001/.tmpsojc2U -lang rust -ct 1 -es 1 -mcd 16 -mdd 1024 -mdy 33 -double -ftz 0");
		m.declare("filename", r"lamb.dsp");
		m.declare("interpolators.lib/interpolate_linear:author", r"Stéphane Letz");
		m.declare("interpolators.lib/interpolate_linear:licence", r"MIT");
		m.declare("interpolators.lib/name", r"Faust Interpolator Library");
		m.declare("interpolators.lib/version", r"1.3.1");
		m.declare("license", r"AGPLv3");
		m.declare("maths.lib/author", r"GRAME");
		m.declare("maths.lib/copyright", r"GRAME");
		m.declare("maths.lib/license", r"LGPL with exception");
		m.declare("maths.lib/name", r"Faust Math Library");
		m.declare("maths.lib/version", r"2.8.0");
		m.declare("name", r"lamb");
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
		self.fHslider0 = 0.0;
		self.fHslider1 = 9.0;
		self.fHslider2 = 5e+01;
		self.fHslider3 = 1.0;
		self.fHslider4 = -1.0;
		self.fHslider5 = 0.0;
		self.fHslider6 = 1e+02;
		self.fHslider7 = 0.0;
		self.fHslider8 = 0.5;
		self.fHslider9 = 6e+01;
	}
	fn instance_clear(&mut self) {
		for l0 in 0..2 {
			self.fRec0[l0 as usize] = 0.0;
		}
		self.IOTA0 = 0;
		for l1 in 0..32768 {
			self.fVec0[l1 as usize] = 0.0;
		}
		for l2 in 0..32768 {
			self.fVec1[l2 as usize] = 0.0;
		}
		for l3 in 0..16384 {
			self.fVec2[l3 as usize] = 0.0;
		}
		for l4 in 0..3 {
			self.fVec3[l4 as usize] = 0.0;
		}
		for l5 in 0..7 {
			self.fVec4[l5 as usize] = 0.0;
		}
		for l6 in 0..15 {
			self.fVec5[l6 as usize] = 0.0;
		}
		for l7 in 0..32 {
			self.fVec6[l7 as usize] = 0.0;
		}
		for l8 in 0..64 {
			self.fVec7[l8 as usize] = 0.0;
		}
		for l9 in 0..128 {
			self.fVec8[l9 as usize] = 0.0;
		}
		for l10 in 0..256 {
			self.fVec9[l10 as usize] = 0.0;
		}
		for l11 in 0..512 {
			self.fVec10[l11 as usize] = 0.0;
		}
		for l12 in 0..1024 {
			self.fVec11[l12 as usize] = 0.0;
		}
		for l13 in 0..2048 {
			self.fVec12[l13 as usize] = 0.0;
		}
		for l14 in 0..4096 {
			self.fVec13[l14 as usize] = 0.0;
		}
		for l15 in 0..16384 {
			self.fRec3[l15 as usize] = 0.0;
		}
		for l16 in 0..3 {
			self.fVec14[l16 as usize] = 0.0;
		}
		for l17 in 0..7 {
			self.fVec15[l17 as usize] = 0.0;
		}
		for l18 in 0..15 {
			self.fVec16[l18 as usize] = 0.0;
		}
		for l19 in 0..32 {
			self.fVec17[l19 as usize] = 0.0;
		}
		for l20 in 0..64 {
			self.fVec18[l20 as usize] = 0.0;
		}
		for l21 in 0..128 {
			self.fVec19[l21 as usize] = 0.0;
		}
		for l22 in 0..256 {
			self.fVec20[l22 as usize] = 0.0;
		}
		for l23 in 0..512 {
			self.fVec21[l23 as usize] = 0.0;
		}
		for l24 in 0..1024 {
			self.fVec22[l24 as usize] = 0.0;
		}
		for l25 in 0..2048 {
			self.fVec23[l25 as usize] = 0.0;
		}
		for l26 in 0..4096 {
			self.fVec24[l26 as usize] = 0.0;
		}
		for l27 in 0..2 {
			self.fVec25[l27 as usize] = 0.0;
		}
		for l29 in 0..2 {
			self.fVec26[l29 as usize] = 0.0;
		}
		for l30 in 0..2 {
			self.fVec27[l30 as usize] = 0.0;
		}
		for l31 in 0..2 {
			self.fRec1[l31 as usize] = 0.0;
		}
		for l32 in 0..2 {
			self.fRec2[l32 as usize] = 0.0;
		}
		for l33 in 0..16384 {
			self.fVec28[l33 as usize] = 0.0;
		}
		for l34 in 0..3 {
			self.fVec29[l34 as usize] = 0.0;
		}
		for l35 in 0..7 {
			self.fVec30[l35 as usize] = 0.0;
		}
		for l36 in 0..15 {
			self.fVec31[l36 as usize] = 0.0;
		}
		for l37 in 0..32 {
			self.fVec32[l37 as usize] = 0.0;
		}
		for l38 in 0..64 {
			self.fVec33[l38 as usize] = 0.0;
		}
		for l39 in 0..128 {
			self.fVec34[l39 as usize] = 0.0;
		}
		for l40 in 0..256 {
			self.fVec35[l40 as usize] = 0.0;
		}
		for l41 in 0..512 {
			self.fVec36[l41 as usize] = 0.0;
		}
		for l42 in 0..1024 {
			self.fVec37[l42 as usize] = 0.0;
		}
		for l43 in 0..2048 {
			self.fVec38[l43 as usize] = 0.0;
		}
		for l44 in 0..4096 {
			self.fVec39[l44 as usize] = 0.0;
		}
		for l45 in 0..16384 {
			self.fRec7[l45 as usize] = 0.0;
		}
		for l46 in 0..3 {
			self.fVec40[l46 as usize] = 0.0;
		}
		for l47 in 0..7 {
			self.fVec41[l47 as usize] = 0.0;
		}
		for l48 in 0..15 {
			self.fVec42[l48 as usize] = 0.0;
		}
		for l49 in 0..32 {
			self.fVec43[l49 as usize] = 0.0;
		}
		for l50 in 0..64 {
			self.fVec44[l50 as usize] = 0.0;
		}
		for l51 in 0..128 {
			self.fVec45[l51 as usize] = 0.0;
		}
		for l52 in 0..256 {
			self.fVec46[l52 as usize] = 0.0;
		}
		for l53 in 0..512 {
			self.fVec47[l53 as usize] = 0.0;
		}
		for l54 in 0..1024 {
			self.fVec48[l54 as usize] = 0.0;
		}
		for l55 in 0..2048 {
			self.fVec49[l55 as usize] = 0.0;
		}
		for l56 in 0..4096 {
			self.fVec50[l56 as usize] = 0.0;
		}
		for l57 in 0..2 {
			self.fVec51[l57 as usize] = 0.0;
		}
		for l58 in 0..2 {
			self.fVec52[l58 as usize] = 0.0;
		}
		for l59 in 0..2 {
			self.fVec53[l59 as usize] = 0.0;
		}
		for l60 in 0..2 {
			self.fRec5[l60 as usize] = 0.0;
		}
		for l61 in 0..2 {
			self.fRec6[l61 as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		let mut fConst0: F64 = F64::min(1.92e+05, F64::max(1.0, (self.fSampleRate) as F64));
		self.fConst1 = 44.1 / fConst0;
		self.fConst2 = 1.0 - self.fConst1;
		self.fConst3 = 0.001 * fConst0;
		self.fConst4 = 1e+03 / fConst0;
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
		ui_interface.declare(None, "02", "");
		ui_interface.open_horizontal_box("lamb");
		ui_interface.declare(None, "0", "");
		ui_interface.open_vertical_box("0x00");
		ui_interface.declare(Some(ParamIndex(0)), "01", "");
		ui_interface.add_horizontal_slider("input gain", ParamIndex(0), 0.0, -24.0, 24.0, 0.1);
		ui_interface.declare(Some(ParamIndex(1)), "02", "");
		ui_interface.add_horizontal_slider("strength", ParamIndex(1), 1e+02, 0.0, 1e+02, 1.0);
		ui_interface.declare(Some(ParamIndex(2)), "03", "");
		ui_interface.add_horizontal_slider("thresh", ParamIndex(2), -1.0, -3e+01, 0.0, 0.1);
		ui_interface.declare(Some(ParamIndex(3)), "04", "");
		ui_interface.declare(Some(ParamIndex(3)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(3)), "unit", "ms");
		ui_interface.add_horizontal_slider("attack", ParamIndex(3), 9.0, 0.0, 5e+01, 0.1);
		ui_interface.declare(Some(ParamIndex(4)), "05", "");
		ui_interface.add_horizontal_slider("attack shape", ParamIndex(4), 0.0, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(5)), "06", "");
		ui_interface.declare(Some(ParamIndex(5)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(5)), "unit", "ms");
		ui_interface.add_horizontal_slider("release", ParamIndex(5), 6e+01, 1.0, 5e+02, 1.0);
		ui_interface.declare(Some(ParamIndex(6)), "07", "");
		ui_interface.add_horizontal_slider("release shape", ParamIndex(6), 0.5, 0.0, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(7)), "08", "");
		ui_interface.declare(Some(ParamIndex(7)), "unit", "ms");
		ui_interface.add_horizontal_slider("release hold", ParamIndex(7), 5e+01, 0.020833333333333332, 5e+01, 1.0);
		ui_interface.declare(Some(ParamIndex(8)), "09", "");
		ui_interface.add_horizontal_slider("knee", ParamIndex(8), 1.0, 0.0, 3e+01, 0.1);
		ui_interface.declare(Some(ParamIndex(9)), "10", "");
		ui_interface.add_horizontal_slider("link", ParamIndex(9), 0.0, 0.0, 1e+02, 1.0);
		ui_interface.close_box();
		ui_interface.declare(None, "99", "");
		ui_interface.open_horizontal_box("gain reduction");
		ui_interface.declare(Some(ParamIndex(10)), "unit", "dB");
		ui_interface.add_vertical_bargraph("0", ParamIndex(10), -12.0, 0.0);
		ui_interface.declare(Some(ParamIndex(11)), "unit", "dB");
		ui_interface.add_vertical_bargraph("1", ParamIndex(11), -12.0, 0.0);
		ui_interface.close_box();
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			0 => Some(self.fHslider0),
			3 => Some(self.fHslider1),
			7 => Some(self.fHslider2),
			8 => Some(self.fHslider3),
			2 => Some(self.fHslider4),
			9 => Some(self.fHslider5),
			1 => Some(self.fHslider6),
			4 => Some(self.fHslider7),
			6 => Some(self.fHslider8),
			5 => Some(self.fHslider9),
			10 => Some(self.fVbargraph0),
			11 => Some(self.fVbargraph1),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			0 => { self.fHslider0 = value }
			3 => { self.fHslider1 = value }
			7 => { self.fHslider2 = value }
			8 => { self.fHslider3 = value }
			2 => { self.fHslider4 = value }
			9 => { self.fHslider5 = value }
			1 => { self.fHslider6 = value }
			4 => { self.fHslider7 = value }
			6 => { self.fHslider8 = value }
			5 => { self.fHslider9 = value }
			10 => { self.fVbargraph0 = value }
			11 => { self.fVbargraph1 = value }
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
		let mut fSlow0: F64 = self.fConst1 * F64::powf(1e+01, 0.05 * self.fHslider0);
		let mut fSlow1: F64 = self.fHslider1;
		let mut fSlow2: F64 = self.fHslider2;
		let mut iSlow3: i32 = (self.fConst3 * (fSlow2 + fSlow1)) as i32;
		let mut fSlow4: F64 = self.fConst3 * fSlow1;
		let mut fSlow5: F64 = fSlow4 + 1.0;
		let mut iSlow6: i32 = (F64::floor(fSlow5)) as i32 % 2;
		let mut fSlow7: F64 = self.fHslider3;
		let mut fSlow8: F64 = 0.5 * fSlow7;
		let mut fSlow9: F64 = self.fHslider4;
		let mut fSlow10: F64 = fSlow9 + fSlow8;
		let mut fSlow11: F64 = 0.01 * self.fHslider5;
		let mut fSlow12: F64 = fSlow9 - fSlow8;
		let mut fSlow13: F64 = 0.5 / F64::max(2.220446049250313e-16, fSlow7);
		let mut fSlow14: F64 = 0.0005 * self.fHslider6;
		let mut fSlow15: F64 = self.fConst3 * fSlow2;
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
		let mut iSlow40: i32 = (F64::floor(0.5 * fSlow5)) as i32 % 2;
		let mut iSlow41: i32 = (F64::floor(0.25 * fSlow5)) as i32 % 2;
		let mut iSlow42: i32 = i32::wrapping_add(iSlow6, i32::wrapping_mul(2, iSlow40));
		let mut iSlow43: i32 = (F64::floor(0.125 * fSlow5)) as i32 % 2;
		let mut iSlow44: i32 = i32::wrapping_add(iSlow42, i32::wrapping_mul(4, iSlow41));
		let mut iSlow45: i32 = (F64::floor(0.0625 * fSlow5)) as i32 % 2;
		let mut iSlow46: i32 = i32::wrapping_add(iSlow44, i32::wrapping_mul(8, iSlow43));
		let mut iSlow47: i32 = (F64::floor(0.03125 * fSlow5)) as i32 % 2;
		let mut iSlow48: i32 = i32::wrapping_add(iSlow46, i32::wrapping_mul(16, iSlow45));
		let mut iSlow49: i32 = (F64::floor(0.015625 * fSlow5)) as i32 % 2;
		let mut iSlow50: i32 = i32::wrapping_add(iSlow48, i32::wrapping_mul(32, iSlow47));
		let mut iSlow51: i32 = (F64::floor(0.0078125 * fSlow5)) as i32 % 2;
		let mut iSlow52: i32 = i32::wrapping_add(iSlow50, i32::wrapping_mul(64, iSlow49));
		let mut iSlow53: i32 = (F64::floor(0.00390625 * fSlow5)) as i32 % 2;
		let mut iSlow54: i32 = i32::wrapping_add(iSlow52, i32::wrapping_mul(128, iSlow51));
		let mut iSlow55: i32 = (F64::floor(0.001953125 * fSlow5)) as i32 % 2;
		let mut iSlow56: i32 = i32::wrapping_add(iSlow54, i32::wrapping_mul(256, iSlow53));
		let mut iSlow57: i32 = (F64::floor(0.0009765625 * fSlow5)) as i32 % 2;
		let mut iSlow58: i32 = i32::wrapping_add(iSlow56, i32::wrapping_mul(512, iSlow55));
		let mut iSlow59: i32 = (F64::floor(0.00048828125 * fSlow5)) as i32 % 2;
		let mut iSlow60: i32 = i32::wrapping_add(iSlow58, i32::wrapping_mul(1024, iSlow57));
		let mut fSlow61: F64 = self.fHslider7;
		let mut fSlow62: F64 = self.fHslider8;
		let mut fSlow63: F64 = self.fHslider9;
		let mut iSlow64: i32 = (fSlow4) as i32;
		let zipped_iterators = inputs0.zip(inputs1).zip(outputs0).zip(outputs1);
		for (((input0, input1), output0), output1) in zipped_iterators {
			self.fRec0[0] = fSlow0 + self.fConst2 * self.fRec0[1];
			let mut fTemp0: F64 = *input0 * self.fRec0[0];
			self.fVec0[(self.IOTA0 & 32767) as usize] = fTemp0;
			let mut fTemp1: F64 = self.fRec3[((i32::wrapping_sub(self.IOTA0, 1)) & 16383) as usize];
			let mut fTemp2: F64 = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, F64::abs(fTemp0)));
			let mut fTemp3: F64 = *input1 * self.fRec0[0];
			self.fVec1[(self.IOTA0 & 32767) as usize] = fTemp3;
			let mut fTemp4: F64 = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, F64::abs(fTemp3)));
			let mut fTemp5: F64 = F64::max(fTemp2, fTemp4);
			let mut fTemp6: F64 = fTemp2 + fSlow11 * (fTemp5 - fTemp2);
			let mut iTemp7: i32 = ((fTemp6 > fSlow12) as i32) + ((fTemp6 > fSlow10) as i32);
			let mut fTemp8: F64 = fTemp6 - fSlow9;
			let mut fTemp9: F64 = F64::min(1.0, F64::powf(1e+01, -(fSlow14 * F64::max(0.0, if (iTemp7 == 0) as i32 != 0 {0.0} else {if (iTemp7 == 1) as i32 != 0 {fSlow13 * mydsp_faustpower2_f(fSlow8 + fTemp8)} else {fTemp8}}))));
			self.fVec2[(self.IOTA0 & 16383) as usize] = fTemp9;
			let mut fTemp10: F64 = F64::min(fTemp9, self.fVec2[((i32::wrapping_sub(self.IOTA0, 1)) & 16383) as usize]);
			self.fVec3[0] = fTemp10;
			let mut fTemp11: F64 = F64::min(fTemp10, self.fVec3[2]);
			self.fVec4[0] = fTemp11;
			let mut fTemp12: F64 = F64::min(fTemp11, self.fVec4[4]);
			self.fVec5[0] = fTemp12;
			let mut fTemp13: F64 = F64::min(fTemp12, self.fVec5[8]);
			self.fVec6[(self.IOTA0 & 31) as usize] = fTemp13;
			let mut fTemp14: F64 = F64::min(fTemp13, self.fVec6[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize]);
			self.fVec7[(self.IOTA0 & 63) as usize] = fTemp14;
			let mut fTemp15: F64 = F64::min(fTemp14, self.fVec7[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize]);
			self.fVec8[(self.IOTA0 & 127) as usize] = fTemp15;
			let mut fTemp16: F64 = F64::min(fTemp15, self.fVec8[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize]);
			self.fVec9[(self.IOTA0 & 255) as usize] = fTemp16;
			let mut fTemp17: F64 = F64::min(fTemp16, self.fVec9[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize]);
			self.fVec10[(self.IOTA0 & 511) as usize] = fTemp17;
			let mut fTemp18: F64 = F64::min(fTemp17, self.fVec10[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize]);
			self.fVec11[(self.IOTA0 & 1023) as usize] = fTemp18;
			let mut fTemp19: F64 = F64::min(fTemp18, self.fVec11[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize]);
			self.fVec12[(self.IOTA0 & 2047) as usize] = fTemp19;
			self.fVec13[(self.IOTA0 & 4095) as usize] = F64::min(fTemp19, self.fVec12[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			self.fRec3[(self.IOTA0 & 16383) as usize] = F64::max(F64::min(fTemp1, self.fVec2[((i32::wrapping_sub(self.IOTA0, iSlow16)) & 16383) as usize]), F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow18 != 0 {fTemp9} else {1.7976931348623157e+308}, if iSlow19 != 0 {self.fVec3[iSlow18 as usize]} else {1.7976931348623157e+308}), if iSlow20 != 0 {self.fVec4[iSlow21 as usize]} else {1.7976931348623157e+308}), if iSlow22 != 0 {self.fVec5[iSlow23 as usize]} else {1.7976931348623157e+308}), if iSlow24 != 0 {self.fVec6[((i32::wrapping_sub(self.IOTA0, iSlow25)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow26 != 0 {self.fVec7[((i32::wrapping_sub(self.IOTA0, iSlow27)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow28 != 0 {self.fVec8[((i32::wrapping_sub(self.IOTA0, iSlow29)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow30 != 0 {self.fVec9[((i32::wrapping_sub(self.IOTA0, iSlow31)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow32 != 0 {self.fVec10[((i32::wrapping_sub(self.IOTA0, iSlow33)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow34 != 0 {self.fVec11[((i32::wrapping_sub(self.IOTA0, iSlow35)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow36 != 0 {self.fVec12[((i32::wrapping_sub(self.IOTA0, iSlow37)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow38 != 0 {self.fVec13[((i32::wrapping_sub(self.IOTA0, iSlow39)) & 4095) as usize]} else {1.7976931348623157e+308}));
			let mut fTemp20: F64 = self.fRec3[(self.IOTA0 & 16383) as usize];
			let mut fTemp21: F64 = F64::min(fTemp20, fTemp1);
			self.fVec14[0] = fTemp21;
			let mut fTemp22: F64 = F64::min(fTemp21, self.fVec14[2]);
			self.fVec15[0] = fTemp22;
			let mut fTemp23: F64 = F64::min(fTemp22, self.fVec15[4]);
			self.fVec16[0] = fTemp23;
			let mut fTemp24: F64 = F64::min(fTemp23, self.fVec16[8]);
			self.fVec17[(self.IOTA0 & 31) as usize] = fTemp24;
			let mut fTemp25: F64 = F64::min(fTemp24, self.fVec17[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize]);
			self.fVec18[(self.IOTA0 & 63) as usize] = fTemp25;
			let mut fTemp26: F64 = F64::min(fTemp25, self.fVec18[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize]);
			self.fVec19[(self.IOTA0 & 127) as usize] = fTemp26;
			let mut fTemp27: F64 = F64::min(fTemp26, self.fVec19[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize]);
			self.fVec20[(self.IOTA0 & 255) as usize] = fTemp27;
			let mut fTemp28: F64 = F64::min(fTemp27, self.fVec20[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize]);
			self.fVec21[(self.IOTA0 & 511) as usize] = fTemp28;
			let mut fTemp29: F64 = F64::min(fTemp28, self.fVec21[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize]);
			self.fVec22[(self.IOTA0 & 1023) as usize] = fTemp29;
			let mut fTemp30: F64 = F64::min(fTemp29, self.fVec22[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize]);
			self.fVec23[(self.IOTA0 & 2047) as usize] = fTemp30;
			self.fVec24[(self.IOTA0 & 4095) as usize] = F64::min(fTemp30, self.fVec23[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			let mut fTemp31: F64 = F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow6 != 0 {fTemp20} else {1.7976931348623157e+308}, if iSlow40 != 0 {self.fVec14[iSlow6 as usize]} else {1.7976931348623157e+308}), if iSlow41 != 0 {self.fVec15[iSlow42 as usize]} else {1.7976931348623157e+308}), if iSlow43 != 0 {self.fVec16[iSlow44 as usize]} else {1.7976931348623157e+308}), if iSlow45 != 0 {self.fVec17[((i32::wrapping_sub(self.IOTA0, iSlow46)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow47 != 0 {self.fVec18[((i32::wrapping_sub(self.IOTA0, iSlow48)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow49 != 0 {self.fVec19[((i32::wrapping_sub(self.IOTA0, iSlow50)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow51 != 0 {self.fVec20[((i32::wrapping_sub(self.IOTA0, iSlow52)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow53 != 0 {self.fVec21[((i32::wrapping_sub(self.IOTA0, iSlow54)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow55 != 0 {self.fVec22[((i32::wrapping_sub(self.IOTA0, iSlow56)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow57 != 0 {self.fVec23[((i32::wrapping_sub(self.IOTA0, iSlow58)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow59 != 0 {self.fVec24[((i32::wrapping_sub(self.IOTA0, iSlow60)) & 4095) as usize]} else {1.7976931348623157e+308}) - self.fRec2[1];
			self.fVec25[0] = fTemp31;
			let mut iTemp32: i32 = (fTemp31 > 0.0) as i32;
			let mut fTemp38: F64 = if iTemp32 != 0 {fSlow62} else {fSlow61};
			self.fVec26[0] = fTemp38;
			let mut fTemp39: F64 = 2.0 * fTemp38;
			let mut iTemp40: i32 = (fTemp39) as i32;
			let mut fTemp41: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp40, 98304)) as usize] };
			let mut fTemp42: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp40, 98301)) as usize] };
			let mut fTemp43: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp40, 98302)) as usize] } - fTemp42;
			let mut fTemp44: F64 = fTemp39 - (iTemp40) as F64;
			let mut fTemp45: F64 = fTemp42 + fTemp44 * fTemp43 + 0.5 * (fTemp41 - (fTemp42 + fTemp44 * (fTemp43 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp40, 98305)) as usize] } - fTemp41))));
			let mut fTemp46: F64 = if iTemp32 != 0 {fTemp45} else {1.0 - fTemp45};
			let mut iTemp47: i32 = (fTemp31 < 0.0) as i32;
			let mut fTemp48: F64 = fSlow1 * (iTemp47) as F64 + fSlow63 * (iTemp32) as F64;
			self.fVec27[0] = fTemp48;
			let mut fTemp49: F64 = self.fConst4 / fTemp48;
			let mut fTemp50: F64 = fTemp49 + 0.5;
			let mut fTemp51: F64 = 65535.0 * (1.0 - fTemp50);
			let mut iTemp52: i32 = (fTemp51) as i32;
			let mut fTemp53: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp52, 1))), 196607))) as usize] };
			let mut iTemp54: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp52));
			let mut fTemp55: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp54, 196607))) as usize] };
			let mut fTemp56: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp54, 1), 196607))) as usize] } - fTemp55;
			let mut fTemp57: F64 = 65535.0 * fTemp50;
			let mut iTemp58: i32 = (fTemp57) as i32;
			let mut fTemp59: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp58, 1))), 196607))) as usize] };
			let mut iTemp60: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp58));
			let mut fTemp61: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp60, 196607))) as usize] };
			let mut fTemp62: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp60, 1), 196607))) as usize] } - fTemp61;
			let mut fTemp63: F64 = 65535.0 * (1.0 - self.fRec1[1]);
			let mut iTemp64: i32 = (fTemp63) as i32;
			let mut fTemp65: F64 = 2.0 * self.fVec26[1];
			let mut iTemp66: i32 = (fTemp65) as i32;
			let mut fTemp67: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp66, i32::wrapping_mul(3, i32::wrapping_add(iTemp64, 1))), 196607))) as usize] };
			let mut iTemp68: i32 = i32::wrapping_add(i32::wrapping_mul(3, iTemp64), iTemp66);
			let mut fTemp69: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp68, 196607))) as usize] };
			let mut fTemp70: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp68, 1), 196607))) as usize] } - fTemp69;
			let mut fTemp71: F64 = fTemp65 - (iTemp66) as F64;
			let mut fTemp72: F64 = 65535.0 * self.fRec1[1];
			let mut iTemp73: i32 = (fTemp72) as i32;
			let mut fTemp74: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp66, i32::wrapping_mul(3, i32::wrapping_add(iTemp73, 1))), 196607))) as usize] };
			let mut iTemp75: i32 = i32::wrapping_add(iTemp66, i32::wrapping_mul(3, iTemp73));
			let mut fTemp76: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp75, 196607))) as usize] };
			let mut fTemp77: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, 1), 196607))) as usize] } - fTemp76;
			let mut fTemp78: F64 = self.fRec1[1] + fTemp49;
			let mut fTemp79: F64 = 65535.0 * (1.0 - fTemp78);
			let mut iTemp80: i32 = (fTemp79) as i32;
			let mut fTemp81: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp80, 1))), 196607))) as usize] };
			let mut iTemp82: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp80));
			let mut fTemp83: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp82, 196607))) as usize] };
			let mut fTemp84: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp82, 1), 196607))) as usize] } - fTemp83;
			let mut fTemp85: F64 = 65535.0 * fTemp78;
			let mut iTemp86: i32 = (fTemp85) as i32;
			let mut fTemp87: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp86, 1))), 196607))) as usize] };
			let mut iTemp88: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp86));
			let mut fTemp89: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp88, 196607))) as usize] };
			let mut fTemp90: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp88, 1), 196607))) as usize] } - fTemp89;
			let mut fTemp91: F64 = self.fRec1[1] + self.fConst4 * (1.0 / fTemp48 + 1.0 / self.fVec27[1]);
			let mut fTemp92: F64 = 65535.0 * (1.0 - fTemp91);
			let mut iTemp93: i32 = (fTemp92) as i32;
			let mut fTemp94: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp93, 1))), 196607))) as usize] };
			let mut iTemp95: i32 = i32::wrapping_add(i32::wrapping_mul(3, iTemp93), iTemp40);
			let mut fTemp96: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp95, 196607))) as usize] };
			let mut fTemp97: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp95, 1), 196607))) as usize] } - fTemp96;
			let mut fTemp98: F64 = 65535.0 * fTemp91;
			let mut iTemp99: i32 = (fTemp98) as i32;
			let mut fTemp100: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp99, 1))), 196607))) as usize] };
			let mut iTemp101: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp99));
			let mut fTemp102: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp101, 196607))) as usize] };
			let mut fTemp103: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp101, 1), 196607))) as usize] } - fTemp102;
			let mut fTemp104: F64 = (if iTemp32 != 0 {fTemp102 + fTemp44 * fTemp103 + (fTemp98 - (iTemp99) as F64) * (fTemp100 - (fTemp102 + fTemp44 * (fTemp103 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp101, 4), 196607))) as usize] } - fTemp100))))} else {1.0 - (fTemp96 + fTemp44 * fTemp97 + (fTemp92 - (iTemp93) as F64) * (fTemp94 - (fTemp96 + fTemp44 * (fTemp97 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp95, 4), 196607))) as usize] } - fTemp94)))))} - if iTemp32 != 0 {fTemp89 + fTemp44 * fTemp90 + (fTemp85 - (iTemp86) as F64) * (fTemp87 - (fTemp89 + fTemp44 * (fTemp90 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp88, 4), 196607))) as usize] } - fTemp87))))} else {1.0 - (fTemp83 + fTemp44 * fTemp84 + (fTemp79 - (iTemp80) as F64) * (fTemp81 - (fTemp83 + fTemp44 * (fTemp84 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp82, 4), 196607))) as usize] } - fTemp81)))))}) * self.fVec25[1] / (fTemp31 * (1.0 - if iTemp32 != 0 {fTemp76 + fTemp71 * fTemp77 + (fTemp72 - (iTemp73) as F64) * (fTemp74 - (fTemp76 + fTemp71 * (fTemp77 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp75, 4), 196607))) as usize] } - fTemp74))))} else {1.0 - (fTemp69 + fTemp71 * fTemp70 + (fTemp63 - (iTemp64) as F64) * (fTemp67 - (fTemp69 + fTemp71 * (fTemp70 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp68, 4), 196607))) as usize] } - fTemp67)))))}));
			let mut iTemp105: i32 = (fTemp104 > ((if iTemp32 != 0 {fTemp61 + fTemp44 * fTemp62 + (fTemp57 - (iTemp58) as F64) * (fTemp59 - (fTemp61 + fTemp44 * (fTemp62 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp60, 4), 196607))) as usize] } - fTemp59))))} else {1.0 - (fTemp55 + fTemp44 * fTemp56 + (fTemp51 - (iTemp52) as F64) * (fTemp53 - (fTemp55 + fTemp44 * (fTemp56 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp54, 4), 196607))) as usize] } - fTemp53)))))} - fTemp46) / (1.0 - fTemp46))) as i32;
			let mut fTemp106: F64 = if iTemp105 != 0 {1.0} else {0.5};
			let mut fTemp107: F64 = if iTemp105 != 0 {0.5} else {0.0};
			let mut fTemp108: F64 = fTemp107 + fTemp106;
			let mut fTemp109: F64 = 0.5 * fTemp108;
			let mut fTemp110: F64 = 65535.0 * (1.0 - fTemp109);
			let mut iTemp111: i32 = (fTemp110) as i32;
			let mut fTemp112: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp111, 1)))) as usize] };
			let mut iTemp113: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp111));
			let mut fTemp114: F64 = unsafe { ftbl0mydspSIG0[iTemp113 as usize] };
			let mut fTemp115: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp113, 1)) as usize] } - fTemp114;
			let mut fTemp116: F64 = 32767.5 * fTemp108;
			let mut iTemp117: i32 = (fTemp116) as i32;
			let mut fTemp118: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp117, 1)))) as usize] };
			let mut iTemp119: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp117));
			let mut fTemp120: F64 = unsafe { ftbl0mydspSIG0[iTemp119 as usize] };
			let mut fTemp121: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp119, 1)) as usize] } - fTemp120;
			let mut fTemp122: F64 = if iTemp32 != 0 {fTemp120 + fTemp44 * fTemp121 + (fTemp116 - (iTemp117) as F64) * (fTemp118 - (fTemp120 + fTemp44 * (fTemp121 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp119, 4)) as usize] } - fTemp118))))} else {1.0 - (fTemp114 + fTemp44 * fTemp115 + (fTemp110 - (iTemp111) as F64) * (fTemp112 - (fTemp114 + fTemp44 * (fTemp115 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp113, 4)) as usize] } - fTemp112)))))};
			let mut fTemp123: F64 = fTemp49 + fTemp109;
			let mut fTemp124: F64 = 65535.0 * (1.0 - fTemp123);
			let mut iTemp125: i32 = (fTemp124) as i32;
			let mut fTemp126: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp125, 1))), 196607))) as usize] };
			let mut iTemp127: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp125));
			let mut fTemp128: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp127, 196607))) as usize] };
			let mut fTemp129: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp127, 1), 196607))) as usize] } - fTemp128;
			let mut fTemp130: F64 = 65535.0 * fTemp123;
			let mut iTemp131: i32 = (fTemp130) as i32;
			let mut fTemp132: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp131, 1))), 196607))) as usize] };
			let mut iTemp133: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp131));
			let mut fTemp134: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp133, 196607))) as usize] };
			let mut fTemp135: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp133, 1), 196607))) as usize] } - fTemp134;
			let mut iTemp136: i32 = (fTemp104 > ((if iTemp32 != 0 {fTemp134 + fTemp44 * fTemp135 + (fTemp130 - (iTemp131) as F64) * (fTemp132 - (fTemp134 + fTemp44 * (fTemp135 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp133, 4), 196607))) as usize] } - fTemp132))))} else {1.0 - (fTemp128 + fTemp44 * fTemp129 + (fTemp124 - (iTemp125) as F64) * (fTemp126 - (fTemp128 + fTemp44 * (fTemp129 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp127, 4), 196607))) as usize] } - fTemp126)))))} - fTemp122) / (1.0 - fTemp122))) as i32;
			let mut fTemp137: F64 = if iTemp136 != 0 {fTemp106} else {fTemp109};
			let mut fTemp138: F64 = if iTemp136 != 0 {fTemp109} else {fTemp107};
			let mut fTemp139: F64 = fTemp138 + fTemp137;
			let mut fTemp140: F64 = 0.5 * fTemp139;
			let mut fTemp141: F64 = 65535.0 * (1.0 - fTemp140);
			let mut iTemp142: i32 = (fTemp141) as i32;
			let mut fTemp143: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp142, 1)))) as usize] };
			let mut iTemp144: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp142));
			let mut fTemp145: F64 = unsafe { ftbl0mydspSIG0[iTemp144 as usize] };
			let mut fTemp146: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp144, 1)) as usize] } - fTemp145;
			let mut fTemp147: F64 = 32767.5 * fTemp139;
			let mut iTemp148: i32 = (fTemp147) as i32;
			let mut fTemp149: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp148, 1)))) as usize] };
			let mut iTemp150: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp148));
			let mut fTemp151: F64 = unsafe { ftbl0mydspSIG0[iTemp150 as usize] };
			let mut fTemp152: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp150, 1)) as usize] } - fTemp151;
			let mut fTemp153: F64 = if iTemp32 != 0 {fTemp151 + fTemp44 * fTemp152 + (fTemp147 - (iTemp148) as F64) * (fTemp149 - (fTemp151 + fTemp44 * (fTemp152 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp150, 4)) as usize] } - fTemp149))))} else {1.0 - (fTemp145 + fTemp44 * fTemp146 + (fTemp141 - (iTemp142) as F64) * (fTemp143 - (fTemp145 + fTemp44 * (fTemp146 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp144, 4)) as usize] } - fTemp143)))))};
			let mut fTemp154: F64 = fTemp49 + fTemp140;
			let mut fTemp155: F64 = 65535.0 * (1.0 - fTemp154);
			let mut iTemp156: i32 = (fTemp155) as i32;
			let mut fTemp157: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp156, 1))), 196607))) as usize] };
			let mut iTemp158: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp156));
			let mut fTemp159: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp158, 196607))) as usize] };
			let mut fTemp160: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp158, 1), 196607))) as usize] } - fTemp159;
			let mut fTemp161: F64 = 65535.0 * fTemp154;
			let mut iTemp162: i32 = (fTemp161) as i32;
			let mut fTemp163: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp162, 1))), 196607))) as usize] };
			let mut iTemp164: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp162));
			let mut fTemp165: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp164, 196607))) as usize] };
			let mut fTemp166: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp164, 1), 196607))) as usize] } - fTemp165;
			let mut iTemp167: i32 = (fTemp104 > ((if iTemp32 != 0 {fTemp165 + fTemp44 * fTemp166 + (fTemp161 - (iTemp162) as F64) * (fTemp163 - (fTemp165 + fTemp44 * (fTemp166 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp164, 4), 196607))) as usize] } - fTemp163))))} else {1.0 - (fTemp159 + fTemp44 * fTemp160 + (fTemp155 - (iTemp156) as F64) * (fTemp157 - (fTemp159 + fTemp44 * (fTemp160 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp158, 4), 196607))) as usize] } - fTemp157)))))} - fTemp153) / (1.0 - fTemp153))) as i32;
			let mut fTemp168: F64 = if iTemp167 != 0 {fTemp137} else {fTemp140};
			let mut fTemp169: F64 = if iTemp167 != 0 {fTemp140} else {fTemp138};
			let mut fTemp170: F64 = fTemp169 + fTemp168;
			let mut fTemp171: F64 = 0.5 * fTemp170;
			let mut fTemp172: F64 = 65535.0 * (1.0 - fTemp171);
			let mut iTemp173: i32 = (fTemp172) as i32;
			let mut fTemp174: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp173, 1)))) as usize] };
			let mut iTemp175: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp173));
			let mut fTemp176: F64 = unsafe { ftbl0mydspSIG0[iTemp175 as usize] };
			let mut fTemp177: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp175, 1)) as usize] } - fTemp176;
			let mut fTemp178: F64 = 32767.5 * fTemp170;
			let mut iTemp179: i32 = (fTemp178) as i32;
			let mut fTemp180: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp179, 1)))) as usize] };
			let mut iTemp181: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp179));
			let mut fTemp182: F64 = unsafe { ftbl0mydspSIG0[iTemp181 as usize] };
			let mut fTemp183: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp181, 1)) as usize] } - fTemp182;
			let mut fTemp184: F64 = if iTemp32 != 0 {fTemp182 + fTemp44 * fTemp183 + (fTemp178 - (iTemp179) as F64) * (fTemp180 - (fTemp182 + fTemp44 * (fTemp183 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp181, 4)) as usize] } - fTemp180))))} else {1.0 - (fTemp176 + fTemp44 * fTemp177 + (fTemp172 - (iTemp173) as F64) * (fTemp174 - (fTemp176 + fTemp44 * (fTemp177 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp175, 4)) as usize] } - fTemp174)))))};
			let mut fTemp185: F64 = fTemp49 + fTemp171;
			let mut fTemp186: F64 = 65535.0 * (1.0 - fTemp185);
			let mut iTemp187: i32 = (fTemp186) as i32;
			let mut fTemp188: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp187, 1))), 196607))) as usize] };
			let mut iTemp189: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp187));
			let mut fTemp190: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp189, 196607))) as usize] };
			let mut fTemp191: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp189, 1), 196607))) as usize] } - fTemp190;
			let mut fTemp192: F64 = 65535.0 * fTemp185;
			let mut iTemp193: i32 = (fTemp192) as i32;
			let mut fTemp194: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp193, 1))), 196607))) as usize] };
			let mut iTemp195: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp193));
			let mut fTemp196: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp195, 196607))) as usize] };
			let mut fTemp197: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp195, 1), 196607))) as usize] } - fTemp196;
			let mut iTemp198: i32 = (fTemp104 > ((if iTemp32 != 0 {fTemp196 + fTemp44 * fTemp197 + (fTemp192 - (iTemp193) as F64) * (fTemp194 - (fTemp196 + fTemp44 * (fTemp197 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp195, 4), 196607))) as usize] } - fTemp194))))} else {1.0 - (fTemp190 + fTemp44 * fTemp191 + (fTemp186 - (iTemp187) as F64) * (fTemp188 - (fTemp190 + fTemp44 * (fTemp191 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp189, 4), 196607))) as usize] } - fTemp188)))))} - fTemp184) / (1.0 - fTemp184))) as i32;
			let mut fTemp199: F64 = if iTemp198 != 0 {fTemp168} else {fTemp171};
			let mut fTemp200: F64 = if iTemp198 != 0 {fTemp171} else {fTemp169};
			let mut fTemp201: F64 = fTemp200 + fTemp199;
			let mut fTemp202: F64 = 0.5 * fTemp201;
			let mut fTemp203: F64 = 65535.0 * (1.0 - fTemp202);
			let mut iTemp204: i32 = (fTemp203) as i32;
			let mut fTemp205: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp204, 1)))) as usize] };
			let mut iTemp206: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp204));
			let mut fTemp207: F64 = unsafe { ftbl0mydspSIG0[iTemp206 as usize] };
			let mut fTemp208: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp206, 1)) as usize] } - fTemp207;
			let mut fTemp209: F64 = 32767.5 * fTemp201;
			let mut iTemp210: i32 = (fTemp209) as i32;
			let mut fTemp211: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp210, 1)))) as usize] };
			let mut iTemp212: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp210));
			let mut fTemp213: F64 = unsafe { ftbl0mydspSIG0[iTemp212 as usize] };
			let mut fTemp214: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp212, 1)) as usize] } - fTemp213;
			let mut fTemp215: F64 = if iTemp32 != 0 {fTemp213 + fTemp44 * fTemp214 + (fTemp209 - (iTemp210) as F64) * (fTemp211 - (fTemp213 + fTemp44 * (fTemp214 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp212, 4)) as usize] } - fTemp211))))} else {1.0 - (fTemp207 + fTemp44 * fTemp208 + (fTemp203 - (iTemp204) as F64) * (fTemp205 - (fTemp207 + fTemp44 * (fTemp208 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp206, 4)) as usize] } - fTemp205)))))};
			let mut fTemp216: F64 = fTemp49 + fTemp202;
			let mut fTemp217: F64 = 65535.0 * (1.0 - fTemp216);
			let mut iTemp218: i32 = (fTemp217) as i32;
			let mut fTemp219: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp218, 1))), 196607))) as usize] };
			let mut iTemp220: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp218));
			let mut fTemp221: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp220, 196607))) as usize] };
			let mut fTemp222: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp220, 1), 196607))) as usize] } - fTemp221;
			let mut fTemp223: F64 = 65535.0 * fTemp216;
			let mut iTemp224: i32 = (fTemp223) as i32;
			let mut fTemp225: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp224, 1))), 196607))) as usize] };
			let mut iTemp226: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp224));
			let mut fTemp227: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp226, 196607))) as usize] };
			let mut fTemp228: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp226, 1), 196607))) as usize] } - fTemp227;
			let mut iTemp229: i32 = (fTemp104 > ((if iTemp32 != 0 {fTemp227 + fTemp44 * fTemp228 + (fTemp223 - (iTemp224) as F64) * (fTemp225 - (fTemp227 + fTemp44 * (fTemp228 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp226, 4), 196607))) as usize] } - fTemp225))))} else {1.0 - (fTemp221 + fTemp44 * fTemp222 + (fTemp217 - (iTemp218) as F64) * (fTemp219 - (fTemp221 + fTemp44 * (fTemp222 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp220, 4), 196607))) as usize] } - fTemp219)))))} - fTemp215) / (1.0 - fTemp215))) as i32;
			let mut fTemp230: F64 = if iTemp229 != 0 {fTemp199} else {fTemp202};
			let mut fTemp231: F64 = if iTemp229 != 0 {fTemp202} else {fTemp200};
			let mut fTemp232: F64 = fTemp231 + fTemp230;
			let mut fTemp233: F64 = 0.5 * fTemp232;
			let mut fTemp234: F64 = 65535.0 * (1.0 - fTemp233);
			let mut iTemp235: i32 = (fTemp234) as i32;
			let mut fTemp236: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp235, 1)))) as usize] };
			let mut iTemp237: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp235));
			let mut fTemp238: F64 = unsafe { ftbl0mydspSIG0[iTemp237 as usize] };
			let mut fTemp239: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp237, 1)) as usize] } - fTemp238;
			let mut fTemp240: F64 = 32767.5 * fTemp232;
			let mut iTemp241: i32 = (fTemp240) as i32;
			let mut fTemp242: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp241, 1)))) as usize] };
			let mut iTemp243: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp241));
			let mut fTemp244: F64 = unsafe { ftbl0mydspSIG0[iTemp243 as usize] };
			let mut fTemp245: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp243, 1)) as usize] } - fTemp244;
			let mut fTemp246: F64 = if iTemp32 != 0 {fTemp244 + fTemp44 * fTemp245 + (fTemp240 - (iTemp241) as F64) * (fTemp242 - (fTemp244 + fTemp44 * (fTemp245 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp243, 4)) as usize] } - fTemp242))))} else {1.0 - (fTemp238 + fTemp44 * fTemp239 + (fTemp234 - (iTemp235) as F64) * (fTemp236 - (fTemp238 + fTemp44 * (fTemp239 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp237, 4)) as usize] } - fTemp236)))))};
			let mut fTemp247: F64 = fTemp49 + fTemp233;
			let mut fTemp248: F64 = 65535.0 * (1.0 - fTemp247);
			let mut iTemp249: i32 = (fTemp248) as i32;
			let mut fTemp250: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp249, 1))), 196607))) as usize] };
			let mut iTemp251: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp249));
			let mut fTemp252: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp251, 196607))) as usize] };
			let mut fTemp253: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp251, 1), 196607))) as usize] } - fTemp252;
			let mut fTemp254: F64 = 65535.0 * fTemp247;
			let mut iTemp255: i32 = (fTemp254) as i32;
			let mut fTemp256: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp255, 1))), 196607))) as usize] };
			let mut iTemp257: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp255));
			let mut fTemp258: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp257, 196607))) as usize] };
			let mut fTemp259: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp257, 1), 196607))) as usize] } - fTemp258;
			let mut iTemp260: i32 = (fTemp104 > ((if iTemp32 != 0 {fTemp258 + fTemp44 * fTemp259 + (fTemp254 - (iTemp255) as F64) * (fTemp256 - (fTemp258 + fTemp44 * (fTemp259 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp257, 4), 196607))) as usize] } - fTemp256))))} else {1.0 - (fTemp252 + fTemp44 * fTemp253 + (fTemp248 - (iTemp249) as F64) * (fTemp250 - (fTemp252 + fTemp44 * (fTemp253 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp251, 4), 196607))) as usize] } - fTemp250)))))} - fTemp246) / (1.0 - fTemp246))) as i32;
			let mut fTemp261: F64 = if iTemp260 != 0 {fTemp230} else {fTemp233};
			let mut fTemp262: F64 = if iTemp260 != 0 {fTemp233} else {fTemp231};
			let mut fTemp263: F64 = fTemp262 + fTemp261;
			let mut fTemp264: F64 = 0.5 * fTemp263;
			let mut fTemp265: F64 = 65535.0 * (1.0 - fTemp264);
			let mut iTemp266: i32 = (fTemp265) as i32;
			let mut fTemp267: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp266, 1)))) as usize] };
			let mut iTemp268: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp266));
			let mut fTemp269: F64 = unsafe { ftbl0mydspSIG0[iTemp268 as usize] };
			let mut fTemp270: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp268, 1)) as usize] } - fTemp269;
			let mut fTemp271: F64 = 32767.5 * fTemp263;
			let mut iTemp272: i32 = (fTemp271) as i32;
			let mut fTemp273: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp272, 1)))) as usize] };
			let mut iTemp274: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp272));
			let mut fTemp275: F64 = unsafe { ftbl0mydspSIG0[iTemp274 as usize] };
			let mut fTemp276: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp274, 1)) as usize] } - fTemp275;
			let mut fTemp277: F64 = if iTemp32 != 0 {fTemp275 + fTemp44 * fTemp276 + (fTemp271 - (iTemp272) as F64) * (fTemp273 - (fTemp275 + fTemp44 * (fTemp276 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp274, 4)) as usize] } - fTemp273))))} else {1.0 - (fTemp269 + fTemp44 * fTemp270 + (fTemp265 - (iTemp266) as F64) * (fTemp267 - (fTemp269 + fTemp44 * (fTemp270 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp268, 4)) as usize] } - fTemp267)))))};
			let mut fTemp278: F64 = fTemp49 + fTemp264;
			let mut fTemp279: F64 = 65535.0 * (1.0 - fTemp278);
			let mut iTemp280: i32 = (fTemp279) as i32;
			let mut fTemp281: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp280, 1))), 196607))) as usize] };
			let mut iTemp282: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp280));
			let mut fTemp283: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp282, 196607))) as usize] };
			let mut fTemp284: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp282, 1), 196607))) as usize] } - fTemp283;
			let mut fTemp285: F64 = 65535.0 * fTemp278;
			let mut iTemp286: i32 = (fTemp285) as i32;
			let mut fTemp287: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp286, 1))), 196607))) as usize] };
			let mut iTemp288: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp286));
			let mut fTemp289: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp288, 196607))) as usize] };
			let mut fTemp290: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp288, 1), 196607))) as usize] } - fTemp289;
			let mut iTemp291: i32 = (fTemp104 > ((if iTemp32 != 0 {fTemp289 + fTemp44 * fTemp290 + (fTemp285 - (iTemp286) as F64) * (fTemp287 - (fTemp289 + fTemp44 * (fTemp290 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp288, 4), 196607))) as usize] } - fTemp287))))} else {1.0 - (fTemp283 + fTemp44 * fTemp284 + (fTemp279 - (iTemp280) as F64) * (fTemp281 - (fTemp283 + fTemp44 * (fTemp284 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp282, 4), 196607))) as usize] } - fTemp281)))))} - fTemp277) / (1.0 - fTemp277))) as i32;
			let mut fTemp292: F64 = if iTemp291 != 0 {fTemp261} else {fTemp264};
			let mut fTemp293: F64 = if iTemp291 != 0 {fTemp264} else {fTemp262};
			let mut fTemp294: F64 = fTemp293 + fTemp292;
			let mut fTemp295: F64 = 0.5 * fTemp294;
			let mut fTemp296: F64 = 65535.0 * (1.0 - fTemp295);
			let mut iTemp297: i32 = (fTemp296) as i32;
			let mut fTemp298: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp297, 1)))) as usize] };
			let mut iTemp299: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp297));
			let mut fTemp300: F64 = unsafe { ftbl0mydspSIG0[iTemp299 as usize] };
			let mut fTemp301: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp299, 1)) as usize] } - fTemp300;
			let mut fTemp302: F64 = 32767.5 * fTemp294;
			let mut iTemp303: i32 = (fTemp302) as i32;
			let mut fTemp304: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp303, 1)))) as usize] };
			let mut iTemp305: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp303));
			let mut fTemp306: F64 = unsafe { ftbl0mydspSIG0[iTemp305 as usize] };
			let mut fTemp307: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp305, 1)) as usize] } - fTemp306;
			let mut fTemp308: F64 = if iTemp32 != 0 {fTemp306 + fTemp44 * fTemp307 + (fTemp302 - (iTemp303) as F64) * (fTemp304 - (fTemp306 + fTemp44 * (fTemp307 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp305, 4)) as usize] } - fTemp304))))} else {1.0 - (fTemp300 + fTemp44 * fTemp301 + (fTemp296 - (iTemp297) as F64) * (fTemp298 - (fTemp300 + fTemp44 * (fTemp301 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp299, 4)) as usize] } - fTemp298)))))};
			let mut fTemp309: F64 = fTemp49 + fTemp295;
			let mut fTemp310: F64 = 65535.0 * (1.0 - fTemp309);
			let mut iTemp311: i32 = (fTemp310) as i32;
			let mut fTemp312: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp311, 1))), 196607))) as usize] };
			let mut iTemp313: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp311));
			let mut fTemp314: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp313, 196607))) as usize] };
			let mut fTemp315: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp313, 1), 196607))) as usize] } - fTemp314;
			let mut fTemp316: F64 = 65535.0 * fTemp309;
			let mut iTemp317: i32 = (fTemp316) as i32;
			let mut fTemp318: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp317, 1))), 196607))) as usize] };
			let mut iTemp319: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp317));
			let mut fTemp320: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp319, 196607))) as usize] };
			let mut fTemp321: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp319, 1), 196607))) as usize] } - fTemp320;
			let mut iTemp322: i32 = (fTemp104 > ((if iTemp32 != 0 {fTemp320 + fTemp44 * fTemp321 + (fTemp316 - (iTemp317) as F64) * (fTemp318 - (fTemp320 + fTemp44 * (fTemp321 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp319, 4), 196607))) as usize] } - fTemp318))))} else {1.0 - (fTemp314 + fTemp44 * fTemp315 + (fTemp310 - (iTemp311) as F64) * (fTemp312 - (fTemp314 + fTemp44 * (fTemp315 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp313, 4), 196607))) as usize] } - fTemp312)))))} - fTemp308) / (1.0 - fTemp308))) as i32;
			let mut fTemp323: F64 = if iTemp322 != 0 {fTemp292} else {fTemp295};
			let mut fTemp324: F64 = if iTemp322 != 0 {fTemp295} else {fTemp293};
			let mut fTemp325: F64 = fTemp324 + fTemp323;
			let mut fTemp326: F64 = 0.5 * fTemp325;
			let mut fTemp327: F64 = 65535.0 * (1.0 - fTemp326);
			let mut iTemp328: i32 = (fTemp327) as i32;
			let mut fTemp329: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp328, 1)))) as usize] };
			let mut iTemp330: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp328));
			let mut fTemp331: F64 = unsafe { ftbl0mydspSIG0[iTemp330 as usize] };
			let mut fTemp332: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp330, 1)) as usize] } - fTemp331;
			let mut fTemp333: F64 = 32767.5 * fTemp325;
			let mut iTemp334: i32 = (fTemp333) as i32;
			let mut fTemp335: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp334, 1)))) as usize] };
			let mut iTemp336: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp334));
			let mut fTemp337: F64 = unsafe { ftbl0mydspSIG0[iTemp336 as usize] };
			let mut fTemp338: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp336, 1)) as usize] } - fTemp337;
			let mut fTemp339: F64 = if iTemp32 != 0 {fTemp337 + fTemp44 * fTemp338 + (fTemp333 - (iTemp334) as F64) * (fTemp335 - (fTemp337 + fTemp44 * (fTemp338 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp336, 4)) as usize] } - fTemp335))))} else {1.0 - (fTemp331 + fTemp44 * fTemp332 + (fTemp327 - (iTemp328) as F64) * (fTemp329 - (fTemp331 + fTemp44 * (fTemp332 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp330, 4)) as usize] } - fTemp329)))))};
			let mut fTemp340: F64 = fTemp49 + fTemp326;
			let mut fTemp341: F64 = 65535.0 * (1.0 - fTemp340);
			let mut iTemp342: i32 = (fTemp341) as i32;
			let mut fTemp343: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp342, 1))), 196607))) as usize] };
			let mut iTemp344: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp342));
			let mut fTemp345: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp344, 196607))) as usize] };
			let mut fTemp346: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp344, 1), 196607))) as usize] } - fTemp345;
			let mut fTemp347: F64 = 65535.0 * fTemp340;
			let mut iTemp348: i32 = (fTemp347) as i32;
			let mut fTemp349: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp348, 1))), 196607))) as usize] };
			let mut iTemp350: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp348));
			let mut fTemp351: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp350, 196607))) as usize] };
			let mut fTemp352: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp350, 1), 196607))) as usize] } - fTemp351;
			let mut iTemp353: i32 = (fTemp104 > ((if iTemp32 != 0 {fTemp351 + fTemp44 * fTemp352 + (fTemp347 - (iTemp348) as F64) * (fTemp349 - (fTemp351 + fTemp44 * (fTemp352 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp350, 4), 196607))) as usize] } - fTemp349))))} else {1.0 - (fTemp345 + fTemp44 * fTemp346 + (fTemp341 - (iTemp342) as F64) * (fTemp343 - (fTemp345 + fTemp44 * (fTemp346 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp344, 4), 196607))) as usize] } - fTemp343)))))} - fTemp339) / (1.0 - fTemp339))) as i32;
			let mut fTemp354: F64 = if iTemp353 != 0 {fTemp323} else {fTemp326};
			let mut fTemp355: F64 = if iTemp353 != 0 {fTemp326} else {fTemp324};
			let mut fTemp356: F64 = fTemp355 + fTemp354;
			let mut fTemp357: F64 = 0.5 * fTemp356;
			let mut fTemp358: F64 = 65535.0 * (1.0 - fTemp357);
			let mut iTemp359: i32 = (fTemp358) as i32;
			let mut fTemp360: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp359, 1)))) as usize] };
			let mut iTemp361: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp359));
			let mut fTemp362: F64 = unsafe { ftbl0mydspSIG0[iTemp361 as usize] };
			let mut fTemp363: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp361, 1)) as usize] } - fTemp362;
			let mut fTemp364: F64 = 32767.5 * fTemp356;
			let mut iTemp365: i32 = (fTemp364) as i32;
			let mut fTemp366: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp365, 1)))) as usize] };
			let mut iTemp367: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp365));
			let mut fTemp368: F64 = unsafe { ftbl0mydspSIG0[iTemp367 as usize] };
			let mut fTemp369: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp367, 1)) as usize] } - fTemp368;
			let mut fTemp370: F64 = if iTemp32 != 0 {fTemp368 + fTemp44 * fTemp369 + (fTemp364 - (iTemp365) as F64) * (fTemp366 - (fTemp368 + fTemp44 * (fTemp369 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp367, 4)) as usize] } - fTemp366))))} else {1.0 - (fTemp362 + fTemp44 * fTemp363 + (fTemp358 - (iTemp359) as F64) * (fTemp360 - (fTemp362 + fTemp44 * (fTemp363 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp361, 4)) as usize] } - fTemp360)))))};
			let mut fTemp371: F64 = fTemp49 + fTemp357;
			let mut fTemp372: F64 = 65535.0 * (1.0 - fTemp371);
			let mut iTemp373: i32 = (fTemp372) as i32;
			let mut fTemp374: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp373, 1))), 196607))) as usize] };
			let mut iTemp375: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp373));
			let mut fTemp376: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp375, 196607))) as usize] };
			let mut fTemp377: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp375, 1), 196607))) as usize] } - fTemp376;
			let mut fTemp378: F64 = 65535.0 * fTemp371;
			let mut iTemp379: i32 = (fTemp378) as i32;
			let mut fTemp380: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp379, 1))), 196607))) as usize] };
			let mut iTemp381: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp379));
			let mut fTemp382: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp381, 196607))) as usize] };
			let mut fTemp383: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp381, 1), 196607))) as usize] } - fTemp382;
			let mut iTemp384: i32 = (fTemp104 > ((if iTemp32 != 0 {fTemp382 + fTemp44 * fTemp383 + (fTemp378 - (iTemp379) as F64) * (fTemp380 - (fTemp382 + fTemp44 * (fTemp383 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp381, 4), 196607))) as usize] } - fTemp380))))} else {1.0 - (fTemp376 + fTemp44 * fTemp377 + (fTemp372 - (iTemp373) as F64) * (fTemp374 - (fTemp376 + fTemp44 * (fTemp377 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp375, 4), 196607))) as usize] } - fTemp374)))))} - fTemp370) / (1.0 - fTemp370))) as i32;
			let mut fTemp385: F64 = if iTemp384 != 0 {fTemp354} else {fTemp357};
			let mut fTemp386: F64 = if iTemp384 != 0 {fTemp357} else {fTemp355};
			let mut fTemp387: F64 = fTemp386 + fTemp385;
			let mut fTemp388: F64 = 0.5 * fTemp387;
			let mut fTemp389: F64 = 65535.0 * (1.0 - fTemp388);
			let mut iTemp390: i32 = (fTemp389) as i32;
			let mut fTemp391: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp390, 1)))) as usize] };
			let mut iTemp392: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp390));
			let mut fTemp393: F64 = unsafe { ftbl0mydspSIG0[iTemp392 as usize] };
			let mut fTemp394: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp392, 1)) as usize] } - fTemp393;
			let mut fTemp395: F64 = 32767.5 * fTemp387;
			let mut iTemp396: i32 = (fTemp395) as i32;
			let mut fTemp397: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp396, 1)))) as usize] };
			let mut iTemp398: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp396));
			let mut fTemp399: F64 = unsafe { ftbl0mydspSIG0[iTemp398 as usize] };
			let mut fTemp400: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp398, 1)) as usize] } - fTemp399;
			let mut fTemp401: F64 = if iTemp32 != 0 {fTemp399 + fTemp44 * fTemp400 + (fTemp395 - (iTemp396) as F64) * (fTemp397 - (fTemp399 + fTemp44 * (fTemp400 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp398, 4)) as usize] } - fTemp397))))} else {1.0 - (fTemp393 + fTemp44 * fTemp394 + (fTemp389 - (iTemp390) as F64) * (fTemp391 - (fTemp393 + fTemp44 * (fTemp394 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp392, 4)) as usize] } - fTemp391)))))};
			let mut fTemp402: F64 = fTemp49 + fTemp388;
			let mut fTemp403: F64 = 65535.0 * (1.0 - fTemp402);
			let mut iTemp404: i32 = (fTemp403) as i32;
			let mut fTemp405: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp404, 1))), 196607))) as usize] };
			let mut iTemp406: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp404));
			let mut fTemp407: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp406, 196607))) as usize] };
			let mut fTemp408: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp406, 1), 196607))) as usize] } - fTemp407;
			let mut fTemp409: F64 = 65535.0 * fTemp402;
			let mut iTemp410: i32 = (fTemp409) as i32;
			let mut fTemp411: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp410, 1))), 196607))) as usize] };
			let mut iTemp412: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp410));
			let mut fTemp413: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp412, 196607))) as usize] };
			let mut fTemp414: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp412, 1), 196607))) as usize] } - fTemp413;
			let mut iTemp415: i32 = (fTemp104 > ((if iTemp32 != 0 {fTemp413 + fTemp44 * fTemp414 + (fTemp409 - (iTemp410) as F64) * (fTemp411 - (fTemp413 + fTemp44 * (fTemp414 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp412, 4), 196607))) as usize] } - fTemp411))))} else {1.0 - (fTemp407 + fTemp44 * fTemp408 + (fTemp403 - (iTemp404) as F64) * (fTemp405 - (fTemp407 + fTemp44 * (fTemp408 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp406, 4), 196607))) as usize] } - fTemp405)))))} - fTemp401) / (1.0 - fTemp401))) as i32;
			let mut fTemp416: F64 = if iTemp415 != 0 {fTemp385} else {fTemp388};
			let mut fTemp417: F64 = if iTemp415 != 0 {fTemp388} else {fTemp386};
			let mut fTemp418: F64 = fTemp417 + fTemp416;
			let mut fTemp419: F64 = 0.5 * fTemp418;
			let mut fTemp420: F64 = 65535.0 * (1.0 - fTemp419);
			let mut iTemp421: i32 = (fTemp420) as i32;
			let mut fTemp422: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp421, 1)))) as usize] };
			let mut iTemp423: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp421));
			let mut fTemp424: F64 = unsafe { ftbl0mydspSIG0[iTemp423 as usize] };
			let mut fTemp425: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp423, 1)) as usize] } - fTemp424;
			let mut fTemp426: F64 = 32767.5 * fTemp418;
			let mut iTemp427: i32 = (fTemp426) as i32;
			let mut fTemp428: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp427, 1)))) as usize] };
			let mut iTemp429: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp427));
			let mut fTemp430: F64 = unsafe { ftbl0mydspSIG0[iTemp429 as usize] };
			let mut fTemp431: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp429, 1)) as usize] } - fTemp430;
			let mut fTemp432: F64 = if iTemp32 != 0 {fTemp430 + fTemp44 * fTemp431 + (fTemp426 - (iTemp427) as F64) * (fTemp428 - (fTemp430 + fTemp44 * (fTemp431 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp429, 4)) as usize] } - fTemp428))))} else {1.0 - (fTemp424 + fTemp44 * fTemp425 + (fTemp420 - (iTemp421) as F64) * (fTemp422 - (fTemp424 + fTemp44 * (fTemp425 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp423, 4)) as usize] } - fTemp422)))))};
			let mut fTemp433: F64 = fTemp49 + fTemp419;
			let mut fTemp434: F64 = 65535.0 * (1.0 - fTemp433);
			let mut iTemp435: i32 = (fTemp434) as i32;
			let mut fTemp436: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp435, 1))), 196607))) as usize] };
			let mut iTemp437: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp435));
			let mut fTemp438: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp437, 196607))) as usize] };
			let mut fTemp439: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp437, 1), 196607))) as usize] } - fTemp438;
			let mut fTemp440: F64 = 65535.0 * fTemp433;
			let mut iTemp441: i32 = (fTemp440) as i32;
			let mut fTemp442: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp441, 1))), 196607))) as usize] };
			let mut iTemp443: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp441));
			let mut fTemp444: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp443, 196607))) as usize] };
			let mut fTemp445: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp443, 1), 196607))) as usize] } - fTemp444;
			let mut iTemp446: i32 = (fTemp104 > ((if iTemp32 != 0 {fTemp444 + fTemp44 * fTemp445 + (fTemp440 - (iTemp441) as F64) * (fTemp442 - (fTemp444 + fTemp44 * (fTemp445 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp443, 4), 196607))) as usize] } - fTemp442))))} else {1.0 - (fTemp438 + fTemp44 * fTemp439 + (fTemp434 - (iTemp435) as F64) * (fTemp436 - (fTemp438 + fTemp44 * (fTemp439 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp437, 4), 196607))) as usize] } - fTemp436)))))} - fTemp432) / (1.0 - fTemp432))) as i32;
			let mut fTemp447: F64 = if iTemp446 != 0 {fTemp416} else {fTemp419};
			let mut fTemp448: F64 = if iTemp446 != 0 {fTemp419} else {fTemp417};
			let mut fTemp449: F64 = fTemp448 + fTemp447;
			let mut fTemp450: F64 = 0.5 * fTemp449;
			let mut fTemp451: F64 = 65535.0 * (1.0 - fTemp450);
			let mut iTemp452: i32 = (fTemp451) as i32;
			let mut fTemp453: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp452, 1)))) as usize] };
			let mut iTemp454: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp452));
			let mut fTemp455: F64 = unsafe { ftbl0mydspSIG0[iTemp454 as usize] };
			let mut fTemp456: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp454, 1)) as usize] } - fTemp455;
			let mut fTemp457: F64 = 32767.5 * fTemp449;
			let mut iTemp458: i32 = (fTemp457) as i32;
			let mut fTemp459: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp458, 1)))) as usize] };
			let mut iTemp460: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp458));
			let mut fTemp461: F64 = unsafe { ftbl0mydspSIG0[iTemp460 as usize] };
			let mut fTemp462: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp460, 1)) as usize] } - fTemp461;
			let mut fTemp463: F64 = if iTemp32 != 0 {fTemp461 + fTemp44 * fTemp462 + (fTemp457 - (iTemp458) as F64) * (fTemp459 - (fTemp461 + fTemp44 * (fTemp462 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp460, 4)) as usize] } - fTemp459))))} else {1.0 - (fTemp455 + fTemp44 * fTemp456 + (fTemp451 - (iTemp452) as F64) * (fTemp453 - (fTemp455 + fTemp44 * (fTemp456 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp454, 4)) as usize] } - fTemp453)))))};
			let mut fTemp464: F64 = fTemp49 + fTemp450;
			let mut fTemp465: F64 = 65535.0 * (1.0 - fTemp464);
			let mut iTemp466: i32 = (fTemp465) as i32;
			let mut fTemp467: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp466, 1))), 196607))) as usize] };
			let mut iTemp468: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp466));
			let mut fTemp469: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp468, 196607))) as usize] };
			let mut fTemp470: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp468, 1), 196607))) as usize] } - fTemp469;
			let mut fTemp471: F64 = 65535.0 * fTemp464;
			let mut iTemp472: i32 = (fTemp471) as i32;
			let mut fTemp473: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp472, 1))), 196607))) as usize] };
			let mut iTemp474: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp472));
			let mut fTemp475: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp474, 196607))) as usize] };
			let mut fTemp476: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp474, 1), 196607))) as usize] } - fTemp475;
			let mut iTemp477: i32 = (fTemp104 > ((if iTemp32 != 0 {fTemp475 + fTemp44 * fTemp476 + (fTemp471 - (iTemp472) as F64) * (fTemp473 - (fTemp475 + fTemp44 * (fTemp476 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp474, 4), 196607))) as usize] } - fTemp473))))} else {1.0 - (fTemp469 + fTemp44 * fTemp470 + (fTemp465 - (iTemp466) as F64) * (fTemp467 - (fTemp469 + fTemp44 * (fTemp470 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp468, 4), 196607))) as usize] } - fTemp467)))))} - fTemp463) / (1.0 - fTemp463))) as i32;
			let mut fTemp478: F64 = if iTemp477 != 0 {fTemp447} else {fTemp450};
			let mut fTemp479: F64 = if iTemp477 != 0 {fTemp450} else {fTemp448};
			let mut fTemp480: F64 = fTemp479 + fTemp478;
			let mut fTemp481: F64 = 0.5 * fTemp480;
			let mut fTemp482: F64 = 65535.0 * (1.0 - fTemp481);
			let mut iTemp483: i32 = (fTemp482) as i32;
			let mut fTemp484: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp483, 1)))) as usize] };
			let mut iTemp485: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp483));
			let mut fTemp486: F64 = unsafe { ftbl0mydspSIG0[iTemp485 as usize] };
			let mut fTemp487: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp485, 1)) as usize] } - fTemp486;
			let mut fTemp488: F64 = 32767.5 * fTemp480;
			let mut iTemp489: i32 = (fTemp488) as i32;
			let mut fTemp490: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp489, 1)))) as usize] };
			let mut iTemp491: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp489));
			let mut fTemp492: F64 = unsafe { ftbl0mydspSIG0[iTemp491 as usize] };
			let mut fTemp493: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp491, 1)) as usize] } - fTemp492;
			let mut fTemp494: F64 = if iTemp32 != 0 {fTemp492 + fTemp44 * fTemp493 + (fTemp488 - (iTemp489) as F64) * (fTemp490 - (fTemp492 + fTemp44 * (fTemp493 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp491, 4)) as usize] } - fTemp490))))} else {1.0 - (fTemp486 + fTemp44 * fTemp487 + (fTemp482 - (iTemp483) as F64) * (fTemp484 - (fTemp486 + fTemp44 * (fTemp487 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp485, 4)) as usize] } - fTemp484)))))};
			let mut fTemp495: F64 = fTemp49 + fTemp481;
			let mut fTemp496: F64 = 65535.0 * (1.0 - fTemp495);
			let mut iTemp497: i32 = (fTemp496) as i32;
			let mut fTemp498: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp497, 1))), 196607))) as usize] };
			let mut iTemp499: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp497));
			let mut fTemp500: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp499, 196607))) as usize] };
			let mut fTemp501: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp499, 1), 196607))) as usize] } - fTemp500;
			let mut fTemp502: F64 = 65535.0 * fTemp495;
			let mut iTemp503: i32 = (fTemp502) as i32;
			let mut fTemp504: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp503, 1))), 196607))) as usize] };
			let mut iTemp505: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp503));
			let mut fTemp506: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp505, 196607))) as usize] };
			let mut fTemp507: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp505, 1), 196607))) as usize] } - fTemp506;
			let mut iTemp508: i32 = (fTemp104 > ((if iTemp32 != 0 {fTemp506 + fTemp44 * fTemp507 + (fTemp502 - (iTemp503) as F64) * (fTemp504 - (fTemp506 + fTemp44 * (fTemp507 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp505, 4), 196607))) as usize] } - fTemp504))))} else {1.0 - (fTemp500 + fTemp44 * fTemp501 + (fTemp496 - (iTemp497) as F64) * (fTemp498 - (fTemp500 + fTemp44 * (fTemp501 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp499, 4), 196607))) as usize] } - fTemp498)))))} - fTemp494) / (1.0 - fTemp494))) as i32;
			let mut fTemp509: F64 = if iTemp508 != 0 {fTemp478} else {fTemp481};
			let mut fTemp510: F64 = if iTemp508 != 0 {fTemp481} else {fTemp479};
			let mut fTemp511: F64 = fTemp510 + fTemp509;
			let mut fTemp512: F64 = 0.5 * fTemp511;
			let mut fTemp513: F64 = 65535.0 * (1.0 - fTemp512);
			let mut iTemp514: i32 = (fTemp513) as i32;
			let mut fTemp515: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp514, 1)))) as usize] };
			let mut iTemp516: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp514));
			let mut fTemp517: F64 = unsafe { ftbl0mydspSIG0[iTemp516 as usize] };
			let mut fTemp518: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp516, 1)) as usize] } - fTemp517;
			let mut fTemp519: F64 = 32767.5 * fTemp511;
			let mut iTemp520: i32 = (fTemp519) as i32;
			let mut fTemp521: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp520, 1)))) as usize] };
			let mut iTemp522: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp520));
			let mut fTemp523: F64 = unsafe { ftbl0mydspSIG0[iTemp522 as usize] };
			let mut fTemp524: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp522, 1)) as usize] } - fTemp523;
			let mut fTemp525: F64 = if iTemp32 != 0 {fTemp523 + fTemp44 * fTemp524 + (fTemp519 - (iTemp520) as F64) * (fTemp521 - (fTemp523 + fTemp44 * (fTemp524 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp522, 4)) as usize] } - fTemp521))))} else {1.0 - (fTemp517 + fTemp44 * fTemp518 + (fTemp513 - (iTemp514) as F64) * (fTemp515 - (fTemp517 + fTemp44 * (fTemp518 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp516, 4)) as usize] } - fTemp515)))))};
			let mut fTemp526: F64 = fTemp49 + fTemp512;
			let mut fTemp527: F64 = 65535.0 * (1.0 - fTemp526);
			let mut iTemp528: i32 = (fTemp527) as i32;
			let mut fTemp529: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp528, 1))), 196607))) as usize] };
			let mut iTemp530: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp528));
			let mut fTemp531: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp530, 196607))) as usize] };
			let mut fTemp532: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp530, 1), 196607))) as usize] } - fTemp531;
			let mut fTemp533: F64 = 65535.0 * fTemp526;
			let mut iTemp534: i32 = (fTemp533) as i32;
			let mut fTemp535: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp534, 1))), 196607))) as usize] };
			let mut iTemp536: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp534));
			let mut fTemp537: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp536, 196607))) as usize] };
			let mut fTemp538: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp536, 1), 196607))) as usize] } - fTemp537;
			let mut iTemp539: i32 = (fTemp104 > ((if iTemp32 != 0 {fTemp537 + fTemp44 * fTemp538 + (fTemp533 - (iTemp534) as F64) * (fTemp535 - (fTemp537 + fTemp44 * (fTemp538 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp536, 4), 196607))) as usize] } - fTemp535))))} else {1.0 - (fTemp531 + fTemp44 * fTemp532 + (fTemp527 - (iTemp528) as F64) * (fTemp529 - (fTemp531 + fTemp44 * (fTemp532 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp530, 4), 196607))) as usize] } - fTemp529)))))} - fTemp525) / (1.0 - fTemp525))) as i32;
			let mut fTemp540: F64 = if iTemp539 != 0 {fTemp509} else {fTemp512};
			let mut fTemp541: F64 = if iTemp539 != 0 {fTemp512} else {fTemp510};
			let mut fTemp542: F64 = fTemp541 + fTemp540;
			let mut fTemp543: F64 = 0.5 * fTemp542;
			let mut fTemp544: F64 = 65535.0 * (1.0 - fTemp543);
			let mut iTemp545: i32 = (fTemp544) as i32;
			let mut fTemp546: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp545, 1)))) as usize] };
			let mut iTemp547: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp545));
			let mut fTemp548: F64 = unsafe { ftbl0mydspSIG0[iTemp547 as usize] };
			let mut fTemp549: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp547, 1)) as usize] } - fTemp548;
			let mut fTemp550: F64 = 32767.5 * fTemp542;
			let mut iTemp551: i32 = (fTemp550) as i32;
			let mut fTemp552: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp551, 1)))) as usize] };
			let mut iTemp553: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp551));
			let mut fTemp554: F64 = unsafe { ftbl0mydspSIG0[iTemp553 as usize] };
			let mut fTemp555: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp553, 1)) as usize] } - fTemp554;
			let mut fTemp556: F64 = if iTemp32 != 0 {fTemp554 + fTemp44 * fTemp555 + (fTemp550 - (iTemp551) as F64) * (fTemp552 - (fTemp554 + fTemp44 * (fTemp555 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp553, 4), 196607))) as usize] } - fTemp552))))} else {1.0 - (fTemp548 + fTemp44 * fTemp549 + (fTemp544 - (iTemp545) as F64) * (fTemp546 - (fTemp548 + fTemp44 * (fTemp549 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp547, 4), 196607))) as usize] } - fTemp546)))))};
			let mut fTemp557: F64 = fTemp49 + fTemp543;
			let mut fTemp558: F64 = 65535.0 * (1.0 - fTemp557);
			let mut iTemp559: i32 = (fTemp558) as i32;
			let mut fTemp560: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp559, 1))), 196607))) as usize] };
			let mut iTemp561: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp559));
			let mut fTemp562: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp561, 196607))) as usize] };
			let mut fTemp563: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp561, 1), 196607))) as usize] } - fTemp562;
			let mut fTemp564: F64 = 65535.0 * fTemp557;
			let mut iTemp565: i32 = (fTemp564) as i32;
			let mut fTemp566: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp565, 1))), 196607))) as usize] };
			let mut iTemp567: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp565));
			let mut fTemp568: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp567, 196607))) as usize] };
			let mut fTemp569: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp567, 1), 196607))) as usize] } - fTemp568;
			let mut iTemp570: i32 = (fTemp104 > ((if iTemp32 != 0 {fTemp568 + fTemp44 * fTemp569 + (fTemp564 - (iTemp565) as F64) * (fTemp566 - (fTemp568 + fTemp44 * (fTemp569 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp567, 4), 196607))) as usize] } - fTemp566))))} else {1.0 - (fTemp562 + fTemp44 * fTemp563 + (fTemp558 - (iTemp559) as F64) * (fTemp560 - (fTemp562 + fTemp44 * (fTemp563 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp561, 4), 196607))) as usize] } - fTemp560)))))} - fTemp556) / (1.0 - fTemp556))) as i32;
			let mut fTemp571: F64 = F64::min(1.0, F64::max(0.0, 0.5 * (if iTemp570 != 0 {fTemp543} else {fTemp541} + if iTemp570 != 0 {fTemp540} else {fTemp543})));
			self.fRec1[0] = fTemp571;
			let mut fTemp572: F64 = 65535.0 * (1.0 - fTemp571);
			let mut iTemp573: i32 = (fTemp572) as i32;
			let mut fTemp574: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp573, 1)))) as usize] };
			let mut iTemp575: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp573));
			let mut fTemp576: F64 = unsafe { ftbl0mydspSIG0[iTemp575 as usize] };
			let mut fTemp577: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp575, 1)) as usize] } - fTemp576;
			let mut fTemp578: F64 = 65535.0 * fTemp571;
			let mut iTemp579: i32 = (fTemp578) as i32;
			let mut fTemp580: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp579, 1)))) as usize] };
			let mut iTemp581: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp579));
			let mut fTemp582: F64 = unsafe { ftbl0mydspSIG0[iTemp581 as usize] };
			let mut fTemp583: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp581, 1)) as usize] } - fTemp582;
			let mut fTemp584: F64 = if iTemp32 != 0 {fTemp582 + fTemp44 * fTemp583 + (fTemp578 - (iTemp579) as F64) * (fTemp580 - (fTemp582 + fTemp44 * (fTemp583 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp581, 4), 196607))) as usize] } - fTemp580))))} else {1.0 - (fTemp576 + fTemp44 * fTemp577 + (fTemp572 - (iTemp573) as F64) * (fTemp574 - (fTemp576 + fTemp44 * (fTemp577 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp575, 4), 196607))) as usize] } - fTemp574)))))};
			let mut fTemp585: F64 = fTemp49 + fTemp571;
			let mut fTemp586: F64 = 65535.0 * (1.0 - fTemp585);
			let mut iTemp587: i32 = (fTemp586) as i32;
			let mut fTemp588: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp587, 1))), 196607))) as usize] };
			let mut iTemp589: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp587));
			let mut fTemp590: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp589, 196607))) as usize] };
			let mut fTemp591: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp589, 1), 196607))) as usize] } - fTemp590;
			let mut fTemp592: F64 = 65535.0 * fTemp585;
			let mut iTemp593: i32 = (fTemp592) as i32;
			let mut fTemp594: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp40, i32::wrapping_mul(3, i32::wrapping_add(iTemp593, 1))), 196607))) as usize] };
			let mut iTemp595: i32 = i32::wrapping_add(iTemp40, i32::wrapping_mul(3, iTemp593));
			let mut fTemp596: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp595, 196607))) as usize] };
			let mut fTemp597: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp595, 1), 196607))) as usize] } - fTemp596;
			let mut fTemp598: F64 = self.fRec2[1] + fTemp31 * (if iTemp32 != 0 {fTemp596 + fTemp44 * fTemp597 + (fTemp592 - (iTemp593) as F64) * (fTemp594 - (fTemp596 + fTemp44 * (fTemp597 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp595, 4), 196607))) as usize] } - fTemp594))))} else {1.0 - (fTemp590 + fTemp44 * fTemp591 + (fTemp586 - (iTemp587) as F64) * (fTemp588 - (fTemp590 + fTemp44 * (fTemp591 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp589, 4), 196607))) as usize] } - fTemp588)))))} - fTemp584) / (1.0 - fTemp584);
			self.fRec2[0] = F64::min(self.fRec3[((i32::wrapping_sub(self.IOTA0, iSlow64)) & 16383) as usize], if iTemp47 != 0 {F64::min(fTemp598, self.fRec2[1])} else {F64::max(fTemp598, self.fRec2[1])});
			self.fVbargraph0 = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, self.fRec2[0]));
			*output0 = self.fRec2[0] * self.fVec0[((i32::wrapping_sub(self.IOTA0, iSlow3)) & 32767) as usize];
			let mut fTemp599: F64 = self.fRec7[((i32::wrapping_sub(self.IOTA0, 1)) & 16383) as usize];
			let mut fTemp600: F64 = fTemp4 + fSlow11 * (fTemp5 - fTemp4);
			let mut iTemp601: i32 = ((fTemp600 > fSlow12) as i32) + ((fTemp600 > fSlow10) as i32);
			let mut fTemp602: F64 = fTemp600 - fSlow9;
			let mut fTemp603: F64 = F64::min(1.0, F64::powf(1e+01, -(fSlow14 * F64::max(0.0, if (iTemp601 == 0) as i32 != 0 {0.0} else {if (iTemp601 == 1) as i32 != 0 {fSlow13 * mydsp_faustpower2_f(fSlow8 + fTemp602)} else {fTemp602}}))));
			self.fVec28[(self.IOTA0 & 16383) as usize] = fTemp603;
			let mut fTemp604: F64 = F64::min(fTemp603, self.fVec28[((i32::wrapping_sub(self.IOTA0, 1)) & 16383) as usize]);
			self.fVec29[0] = fTemp604;
			let mut fTemp605: F64 = F64::min(fTemp604, self.fVec29[2]);
			self.fVec30[0] = fTemp605;
			let mut fTemp606: F64 = F64::min(fTemp605, self.fVec30[4]);
			self.fVec31[0] = fTemp606;
			let mut fTemp607: F64 = F64::min(fTemp606, self.fVec31[8]);
			self.fVec32[(self.IOTA0 & 31) as usize] = fTemp607;
			let mut fTemp608: F64 = F64::min(fTemp607, self.fVec32[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize]);
			self.fVec33[(self.IOTA0 & 63) as usize] = fTemp608;
			let mut fTemp609: F64 = F64::min(fTemp608, self.fVec33[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize]);
			self.fVec34[(self.IOTA0 & 127) as usize] = fTemp609;
			let mut fTemp610: F64 = F64::min(fTemp609, self.fVec34[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize]);
			self.fVec35[(self.IOTA0 & 255) as usize] = fTemp610;
			let mut fTemp611: F64 = F64::min(fTemp610, self.fVec35[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize]);
			self.fVec36[(self.IOTA0 & 511) as usize] = fTemp611;
			let mut fTemp612: F64 = F64::min(fTemp611, self.fVec36[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize]);
			self.fVec37[(self.IOTA0 & 1023) as usize] = fTemp612;
			let mut fTemp613: F64 = F64::min(fTemp612, self.fVec37[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize]);
			self.fVec38[(self.IOTA0 & 2047) as usize] = fTemp613;
			self.fVec39[(self.IOTA0 & 4095) as usize] = F64::min(fTemp613, self.fVec38[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			self.fRec7[(self.IOTA0 & 16383) as usize] = F64::max(F64::min(fTemp599, self.fVec28[((i32::wrapping_sub(self.IOTA0, iSlow16)) & 16383) as usize]), F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow18 != 0 {fTemp603} else {1.7976931348623157e+308}, if iSlow19 != 0 {self.fVec29[iSlow18 as usize]} else {1.7976931348623157e+308}), if iSlow20 != 0 {self.fVec30[iSlow21 as usize]} else {1.7976931348623157e+308}), if iSlow22 != 0 {self.fVec31[iSlow23 as usize]} else {1.7976931348623157e+308}), if iSlow24 != 0 {self.fVec32[((i32::wrapping_sub(self.IOTA0, iSlow25)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow26 != 0 {self.fVec33[((i32::wrapping_sub(self.IOTA0, iSlow27)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow28 != 0 {self.fVec34[((i32::wrapping_sub(self.IOTA0, iSlow29)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow30 != 0 {self.fVec35[((i32::wrapping_sub(self.IOTA0, iSlow31)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow32 != 0 {self.fVec36[((i32::wrapping_sub(self.IOTA0, iSlow33)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow34 != 0 {self.fVec37[((i32::wrapping_sub(self.IOTA0, iSlow35)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow36 != 0 {self.fVec38[((i32::wrapping_sub(self.IOTA0, iSlow37)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow38 != 0 {self.fVec39[((i32::wrapping_sub(self.IOTA0, iSlow39)) & 4095) as usize]} else {1.7976931348623157e+308}));
			let mut fTemp614: F64 = self.fRec7[(self.IOTA0 & 16383) as usize];
			let mut fTemp615: F64 = F64::min(fTemp614, fTemp599);
			self.fVec40[0] = fTemp615;
			let mut fTemp616: F64 = F64::min(fTemp615, self.fVec40[2]);
			self.fVec41[0] = fTemp616;
			let mut fTemp617: F64 = F64::min(fTemp616, self.fVec41[4]);
			self.fVec42[0] = fTemp617;
			let mut fTemp618: F64 = F64::min(fTemp617, self.fVec42[8]);
			self.fVec43[(self.IOTA0 & 31) as usize] = fTemp618;
			let mut fTemp619: F64 = F64::min(fTemp618, self.fVec43[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize]);
			self.fVec44[(self.IOTA0 & 63) as usize] = fTemp619;
			let mut fTemp620: F64 = F64::min(fTemp619, self.fVec44[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize]);
			self.fVec45[(self.IOTA0 & 127) as usize] = fTemp620;
			let mut fTemp621: F64 = F64::min(fTemp620, self.fVec45[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize]);
			self.fVec46[(self.IOTA0 & 255) as usize] = fTemp621;
			let mut fTemp622: F64 = F64::min(fTemp621, self.fVec46[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize]);
			self.fVec47[(self.IOTA0 & 511) as usize] = fTemp622;
			let mut fTemp623: F64 = F64::min(fTemp622, self.fVec47[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize]);
			self.fVec48[(self.IOTA0 & 1023) as usize] = fTemp623;
			let mut fTemp624: F64 = F64::min(fTemp623, self.fVec48[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize]);
			self.fVec49[(self.IOTA0 & 2047) as usize] = fTemp624;
			self.fVec50[(self.IOTA0 & 4095) as usize] = F64::min(fTemp624, self.fVec49[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			let mut fTemp625: F64 = F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow6 != 0 {fTemp614} else {1.7976931348623157e+308}, if iSlow40 != 0 {self.fVec40[iSlow6 as usize]} else {1.7976931348623157e+308}), if iSlow41 != 0 {self.fVec41[iSlow42 as usize]} else {1.7976931348623157e+308}), if iSlow43 != 0 {self.fVec42[iSlow44 as usize]} else {1.7976931348623157e+308}), if iSlow45 != 0 {self.fVec43[((i32::wrapping_sub(self.IOTA0, iSlow46)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow47 != 0 {self.fVec44[((i32::wrapping_sub(self.IOTA0, iSlow48)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow49 != 0 {self.fVec45[((i32::wrapping_sub(self.IOTA0, iSlow50)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow51 != 0 {self.fVec46[((i32::wrapping_sub(self.IOTA0, iSlow52)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow53 != 0 {self.fVec47[((i32::wrapping_sub(self.IOTA0, iSlow54)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow55 != 0 {self.fVec48[((i32::wrapping_sub(self.IOTA0, iSlow56)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow57 != 0 {self.fVec49[((i32::wrapping_sub(self.IOTA0, iSlow58)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow59 != 0 {self.fVec50[((i32::wrapping_sub(self.IOTA0, iSlow60)) & 4095) as usize]} else {1.7976931348623157e+308}) - self.fRec6[1];
			self.fVec51[0] = fTemp625;
			let mut iTemp626: i32 = (fTemp625 > 0.0) as i32;
			let mut fTemp627: F64 = if iTemp626 != 0 {fSlow62} else {fSlow61};
			self.fVec52[0] = fTemp627;
			let mut fTemp628: F64 = 2.0 * fTemp627;
			let mut iTemp629: i32 = (fTemp628) as i32;
			let mut fTemp630: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp629, 98304)) as usize] };
			let mut fTemp631: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp629, 98301)) as usize] };
			let mut fTemp632: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp629, 98302)) as usize] } - fTemp631;
			let mut fTemp633: F64 = fTemp628 - (iTemp629) as F64;
			let mut fTemp634: F64 = fTemp631 + fTemp633 * fTemp632 + 0.5 * (fTemp630 - (fTemp631 + fTemp633 * (fTemp632 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp629, 98305)) as usize] } - fTemp630))));
			let mut fTemp635: F64 = if iTemp626 != 0 {fTemp634} else {1.0 - fTemp634};
			let mut iTemp636: i32 = (fTemp625 < 0.0) as i32;
			let mut fTemp637: F64 = fSlow1 * (iTemp636) as F64 + fSlow63 * (iTemp626) as F64;
			self.fVec53[0] = fTemp637;
			let mut fTemp638: F64 = self.fConst4 / fTemp637;
			let mut fTemp639: F64 = fTemp638 + 0.5;
			let mut fTemp640: F64 = 65535.0 * (1.0 - fTemp639);
			let mut iTemp641: i32 = (fTemp640) as i32;
			let mut fTemp642: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp641, 1))), 196607))) as usize] };
			let mut iTemp643: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp641));
			let mut fTemp644: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp643, 196607))) as usize] };
			let mut fTemp645: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp643, 1), 196607))) as usize] } - fTemp644;
			let mut fTemp646: F64 = 65535.0 * fTemp639;
			let mut iTemp647: i32 = (fTemp646) as i32;
			let mut fTemp648: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp647, 1))), 196607))) as usize] };
			let mut iTemp649: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp647));
			let mut fTemp650: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp649, 196607))) as usize] };
			let mut fTemp651: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp649, 1), 196607))) as usize] } - fTemp650;
			let mut fTemp652: F64 = 65535.0 * (1.0 - self.fRec5[1]);
			let mut iTemp653: i32 = (fTemp652) as i32;
			let mut fTemp654: F64 = 2.0 * self.fVec52[1];
			let mut iTemp655: i32 = (fTemp654) as i32;
			let mut fTemp656: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, i32::wrapping_add(iTemp653, 1))), 196607))) as usize] };
			let mut iTemp657: i32 = i32::wrapping_add(i32::wrapping_mul(3, iTemp653), iTemp655);
			let mut fTemp658: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp657, 196607))) as usize] };
			let mut fTemp659: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp657, 1), 196607))) as usize] } - fTemp658;
			let mut fTemp660: F64 = fTemp654 - (iTemp655) as F64;
			let mut fTemp661: F64 = 65535.0 * self.fRec5[1];
			let mut iTemp662: i32 = (fTemp661) as i32;
			let mut fTemp663: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp655, i32::wrapping_mul(3, i32::wrapping_add(iTemp662, 1))), 196607))) as usize] };
			let mut iTemp664: i32 = i32::wrapping_add(iTemp655, i32::wrapping_mul(3, iTemp662));
			let mut fTemp665: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp664, 196607))) as usize] };
			let mut fTemp666: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, 1), 196607))) as usize] } - fTemp665;
			let mut fTemp667: F64 = self.fRec5[1] + fTemp638;
			let mut fTemp668: F64 = 65535.0 * (1.0 - fTemp667);
			let mut iTemp669: i32 = (fTemp668) as i32;
			let mut fTemp670: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp669, 1))), 196607))) as usize] };
			let mut iTemp671: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp669));
			let mut fTemp672: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp671, 196607))) as usize] };
			let mut fTemp673: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, 1), 196607))) as usize] } - fTemp672;
			let mut fTemp674: F64 = 65535.0 * fTemp667;
			let mut iTemp675: i32 = (fTemp674) as i32;
			let mut fTemp676: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp675, 1))), 196607))) as usize] };
			let mut iTemp677: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp675));
			let mut fTemp678: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp677, 196607))) as usize] };
			let mut fTemp679: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp677, 1), 196607))) as usize] } - fTemp678;
			let mut fTemp680: F64 = self.fRec5[1] + self.fConst4 * (1.0 / fTemp637 + 1.0 / self.fVec53[1]);
			let mut fTemp681: F64 = 65535.0 * (1.0 - fTemp680);
			let mut iTemp682: i32 = (fTemp681) as i32;
			let mut fTemp683: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp682, 1))), 196607))) as usize] };
			let mut iTemp684: i32 = i32::wrapping_add(i32::wrapping_mul(3, iTemp682), iTemp629);
			let mut fTemp685: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp684, 196607))) as usize] };
			let mut fTemp686: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp684, 1), 196607))) as usize] } - fTemp685;
			let mut fTemp687: F64 = 65535.0 * fTemp680;
			let mut iTemp688: i32 = (fTemp687) as i32;
			let mut fTemp689: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp688, 1))), 196607))) as usize] };
			let mut iTemp690: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp688));
			let mut fTemp691: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp690, 196607))) as usize] };
			let mut fTemp692: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp690, 1), 196607))) as usize] } - fTemp691;
			let mut fTemp693: F64 = (if iTemp626 != 0 {fTemp691 + fTemp633 * fTemp692 + (fTemp687 - (iTemp688) as F64) * (fTemp689 - (fTemp691 + fTemp633 * (fTemp692 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp690, 4), 196607))) as usize] } - fTemp689))))} else {1.0 - (fTemp685 + fTemp633 * fTemp686 + (fTemp681 - (iTemp682) as F64) * (fTemp683 - (fTemp685 + fTemp633 * (fTemp686 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp684, 4), 196607))) as usize] } - fTemp683)))))} - if iTemp626 != 0 {fTemp678 + fTemp633 * fTemp679 + (fTemp674 - (iTemp675) as F64) * (fTemp676 - (fTemp678 + fTemp633 * (fTemp679 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp677, 4), 196607))) as usize] } - fTemp676))))} else {1.0 - (fTemp672 + fTemp633 * fTemp673 + (fTemp668 - (iTemp669) as F64) * (fTemp670 - (fTemp672 + fTemp633 * (fTemp673 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp671, 4), 196607))) as usize] } - fTemp670)))))}) * self.fVec51[1] / (fTemp625 * (1.0 - if iTemp626 != 0 {fTemp665 + fTemp660 * fTemp666 + (fTemp661 - (iTemp662) as F64) * (fTemp663 - (fTemp665 + fTemp660 * (fTemp666 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, 4), 196607))) as usize] } - fTemp663))))} else {1.0 - (fTemp658 + fTemp660 * fTemp659 + (fTemp652 - (iTemp653) as F64) * (fTemp656 - (fTemp658 + fTemp660 * (fTemp659 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp657, 4), 196607))) as usize] } - fTemp656)))))}));
			let mut iTemp694: i32 = (fTemp693 > ((if iTemp626 != 0 {fTemp650 + fTemp633 * fTemp651 + (fTemp646 - (iTemp647) as F64) * (fTemp648 - (fTemp650 + fTemp633 * (fTemp651 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp649, 4), 196607))) as usize] } - fTemp648))))} else {1.0 - (fTemp644 + fTemp633 * fTemp645 + (fTemp640 - (iTemp641) as F64) * (fTemp642 - (fTemp644 + fTemp633 * (fTemp645 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp643, 4), 196607))) as usize] } - fTemp642)))))} - fTemp635) / (1.0 - fTemp635))) as i32;
			let mut fTemp695: F64 = if iTemp694 != 0 {1.0} else {0.5};
			let mut fTemp696: F64 = if iTemp694 != 0 {0.5} else {0.0};
			let mut fTemp697: F64 = fTemp696 + fTemp695;
			let mut fTemp698: F64 = 0.5 * fTemp697;
			let mut fTemp699: F64 = 65535.0 * (1.0 - fTemp698);
			let mut iTemp700: i32 = (fTemp699) as i32;
			let mut fTemp701: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp700, 1)))) as usize] };
			let mut iTemp702: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp700));
			let mut fTemp703: F64 = unsafe { ftbl0mydspSIG0[iTemp702 as usize] };
			let mut fTemp704: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp702, 1)) as usize] } - fTemp703;
			let mut fTemp705: F64 = 32767.5 * fTemp697;
			let mut iTemp706: i32 = (fTemp705) as i32;
			let mut fTemp707: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp706, 1)))) as usize] };
			let mut iTemp708: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp706));
			let mut fTemp709: F64 = unsafe { ftbl0mydspSIG0[iTemp708 as usize] };
			let mut fTemp710: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp708, 1)) as usize] } - fTemp709;
			let mut fTemp711: F64 = if iTemp626 != 0 {fTemp709 + fTemp633 * fTemp710 + (fTemp705 - (iTemp706) as F64) * (fTemp707 - (fTemp709 + fTemp633 * (fTemp710 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp708, 4)) as usize] } - fTemp707))))} else {1.0 - (fTemp703 + fTemp633 * fTemp704 + (fTemp699 - (iTemp700) as F64) * (fTemp701 - (fTemp703 + fTemp633 * (fTemp704 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp702, 4)) as usize] } - fTemp701)))))};
			let mut fTemp712: F64 = fTemp638 + fTemp698;
			let mut fTemp713: F64 = 65535.0 * (1.0 - fTemp712);
			let mut iTemp714: i32 = (fTemp713) as i32;
			let mut fTemp715: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp714, 1))), 196607))) as usize] };
			let mut iTemp716: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp714));
			let mut fTemp717: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp716, 196607))) as usize] };
			let mut fTemp718: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp716, 1), 196607))) as usize] } - fTemp717;
			let mut fTemp719: F64 = 65535.0 * fTemp712;
			let mut iTemp720: i32 = (fTemp719) as i32;
			let mut fTemp721: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp720, 1))), 196607))) as usize] };
			let mut iTemp722: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp720));
			let mut fTemp723: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp722, 196607))) as usize] };
			let mut fTemp724: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp722, 1), 196607))) as usize] } - fTemp723;
			let mut iTemp725: i32 = (fTemp693 > ((if iTemp626 != 0 {fTemp723 + fTemp633 * fTemp724 + (fTemp719 - (iTemp720) as F64) * (fTemp721 - (fTemp723 + fTemp633 * (fTemp724 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp722, 4), 196607))) as usize] } - fTemp721))))} else {1.0 - (fTemp717 + fTemp633 * fTemp718 + (fTemp713 - (iTemp714) as F64) * (fTemp715 - (fTemp717 + fTemp633 * (fTemp718 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp716, 4), 196607))) as usize] } - fTemp715)))))} - fTemp711) / (1.0 - fTemp711))) as i32;
			let mut fTemp726: F64 = if iTemp725 != 0 {fTemp695} else {fTemp698};
			let mut fTemp727: F64 = if iTemp725 != 0 {fTemp698} else {fTemp696};
			let mut fTemp728: F64 = fTemp727 + fTemp726;
			let mut fTemp729: F64 = 0.5 * fTemp728;
			let mut fTemp730: F64 = 65535.0 * (1.0 - fTemp729);
			let mut iTemp731: i32 = (fTemp730) as i32;
			let mut fTemp732: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp731, 1)))) as usize] };
			let mut iTemp733: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp731));
			let mut fTemp734: F64 = unsafe { ftbl0mydspSIG0[iTemp733 as usize] };
			let mut fTemp735: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp733, 1)) as usize] } - fTemp734;
			let mut fTemp736: F64 = 32767.5 * fTemp728;
			let mut iTemp737: i32 = (fTemp736) as i32;
			let mut fTemp738: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp737, 1)))) as usize] };
			let mut iTemp739: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp737));
			let mut fTemp740: F64 = unsafe { ftbl0mydspSIG0[iTemp739 as usize] };
			let mut fTemp741: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp739, 1)) as usize] } - fTemp740;
			let mut fTemp742: F64 = if iTemp626 != 0 {fTemp740 + fTemp633 * fTemp741 + (fTemp736 - (iTemp737) as F64) * (fTemp738 - (fTemp740 + fTemp633 * (fTemp741 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp739, 4)) as usize] } - fTemp738))))} else {1.0 - (fTemp734 + fTemp633 * fTemp735 + (fTemp730 - (iTemp731) as F64) * (fTemp732 - (fTemp734 + fTemp633 * (fTemp735 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp733, 4)) as usize] } - fTemp732)))))};
			let mut fTemp743: F64 = fTemp638 + fTemp729;
			let mut fTemp744: F64 = 65535.0 * (1.0 - fTemp743);
			let mut iTemp745: i32 = (fTemp744) as i32;
			let mut fTemp746: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp745, 1))), 196607))) as usize] };
			let mut iTemp747: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp745));
			let mut fTemp748: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp747, 196607))) as usize] };
			let mut fTemp749: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp747, 1), 196607))) as usize] } - fTemp748;
			let mut fTemp750: F64 = 65535.0 * fTemp743;
			let mut iTemp751: i32 = (fTemp750) as i32;
			let mut fTemp752: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp751, 1))), 196607))) as usize] };
			let mut iTemp753: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp751));
			let mut fTemp754: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp753, 196607))) as usize] };
			let mut fTemp755: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp753, 1), 196607))) as usize] } - fTemp754;
			let mut iTemp756: i32 = (fTemp693 > ((if iTemp626 != 0 {fTemp754 + fTemp633 * fTemp755 + (fTemp750 - (iTemp751) as F64) * (fTemp752 - (fTemp754 + fTemp633 * (fTemp755 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp753, 4), 196607))) as usize] } - fTemp752))))} else {1.0 - (fTemp748 + fTemp633 * fTemp749 + (fTemp744 - (iTemp745) as F64) * (fTemp746 - (fTemp748 + fTemp633 * (fTemp749 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp747, 4), 196607))) as usize] } - fTemp746)))))} - fTemp742) / (1.0 - fTemp742))) as i32;
			let mut fTemp757: F64 = if iTemp756 != 0 {fTemp726} else {fTemp729};
			let mut fTemp758: F64 = if iTemp756 != 0 {fTemp729} else {fTemp727};
			let mut fTemp759: F64 = fTemp758 + fTemp757;
			let mut fTemp760: F64 = 0.5 * fTemp759;
			let mut fTemp761: F64 = 65535.0 * (1.0 - fTemp760);
			let mut iTemp762: i32 = (fTemp761) as i32;
			let mut fTemp763: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp762, 1)))) as usize] };
			let mut iTemp764: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp762));
			let mut fTemp765: F64 = unsafe { ftbl0mydspSIG0[iTemp764 as usize] };
			let mut fTemp766: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp764, 1)) as usize] } - fTemp765;
			let mut fTemp767: F64 = 32767.5 * fTemp759;
			let mut iTemp768: i32 = (fTemp767) as i32;
			let mut fTemp769: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp768, 1)))) as usize] };
			let mut iTemp770: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp768));
			let mut fTemp771: F64 = unsafe { ftbl0mydspSIG0[iTemp770 as usize] };
			let mut fTemp772: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp770, 1)) as usize] } - fTemp771;
			let mut fTemp773: F64 = if iTemp626 != 0 {fTemp771 + fTemp633 * fTemp772 + (fTemp767 - (iTemp768) as F64) * (fTemp769 - (fTemp771 + fTemp633 * (fTemp772 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp770, 4)) as usize] } - fTemp769))))} else {1.0 - (fTemp765 + fTemp633 * fTemp766 + (fTemp761 - (iTemp762) as F64) * (fTemp763 - (fTemp765 + fTemp633 * (fTemp766 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp764, 4)) as usize] } - fTemp763)))))};
			let mut fTemp774: F64 = fTemp638 + fTemp760;
			let mut fTemp775: F64 = 65535.0 * (1.0 - fTemp774);
			let mut iTemp776: i32 = (fTemp775) as i32;
			let mut fTemp777: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp776, 1))), 196607))) as usize] };
			let mut iTemp778: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp776));
			let mut fTemp779: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp778, 196607))) as usize] };
			let mut fTemp780: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp778, 1), 196607))) as usize] } - fTemp779;
			let mut fTemp781: F64 = 65535.0 * fTemp774;
			let mut iTemp782: i32 = (fTemp781) as i32;
			let mut fTemp783: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp782, 1))), 196607))) as usize] };
			let mut iTemp784: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp782));
			let mut fTemp785: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp784, 196607))) as usize] };
			let mut fTemp786: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp784, 1), 196607))) as usize] } - fTemp785;
			let mut iTemp787: i32 = (fTemp693 > ((if iTemp626 != 0 {fTemp785 + fTemp633 * fTemp786 + (fTemp781 - (iTemp782) as F64) * (fTemp783 - (fTemp785 + fTemp633 * (fTemp786 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp784, 4), 196607))) as usize] } - fTemp783))))} else {1.0 - (fTemp779 + fTemp633 * fTemp780 + (fTemp775 - (iTemp776) as F64) * (fTemp777 - (fTemp779 + fTemp633 * (fTemp780 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp778, 4), 196607))) as usize] } - fTemp777)))))} - fTemp773) / (1.0 - fTemp773))) as i32;
			let mut fTemp788: F64 = if iTemp787 != 0 {fTemp757} else {fTemp760};
			let mut fTemp789: F64 = if iTemp787 != 0 {fTemp760} else {fTemp758};
			let mut fTemp790: F64 = fTemp789 + fTemp788;
			let mut fTemp791: F64 = 0.5 * fTemp790;
			let mut fTemp792: F64 = 65535.0 * (1.0 - fTemp791);
			let mut iTemp793: i32 = (fTemp792) as i32;
			let mut fTemp794: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp793, 1)))) as usize] };
			let mut iTemp795: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp793));
			let mut fTemp796: F64 = unsafe { ftbl0mydspSIG0[iTemp795 as usize] };
			let mut fTemp797: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp795, 1)) as usize] } - fTemp796;
			let mut fTemp798: F64 = 32767.5 * fTemp790;
			let mut iTemp799: i32 = (fTemp798) as i32;
			let mut fTemp800: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp799, 1)))) as usize] };
			let mut iTemp801: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp799));
			let mut fTemp802: F64 = unsafe { ftbl0mydspSIG0[iTemp801 as usize] };
			let mut fTemp803: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp801, 1)) as usize] } - fTemp802;
			let mut fTemp804: F64 = if iTemp626 != 0 {fTemp802 + fTemp633 * fTemp803 + (fTemp798 - (iTemp799) as F64) * (fTemp800 - (fTemp802 + fTemp633 * (fTemp803 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp801, 4)) as usize] } - fTemp800))))} else {1.0 - (fTemp796 + fTemp633 * fTemp797 + (fTemp792 - (iTemp793) as F64) * (fTemp794 - (fTemp796 + fTemp633 * (fTemp797 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp795, 4)) as usize] } - fTemp794)))))};
			let mut fTemp805: F64 = fTemp638 + fTemp791;
			let mut fTemp806: F64 = 65535.0 * (1.0 - fTemp805);
			let mut iTemp807: i32 = (fTemp806) as i32;
			let mut fTemp808: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp807, 1))), 196607))) as usize] };
			let mut iTemp809: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp807));
			let mut fTemp810: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp809, 196607))) as usize] };
			let mut fTemp811: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp809, 1), 196607))) as usize] } - fTemp810;
			let mut fTemp812: F64 = 65535.0 * fTemp805;
			let mut iTemp813: i32 = (fTemp812) as i32;
			let mut fTemp814: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp813, 1))), 196607))) as usize] };
			let mut iTemp815: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp813));
			let mut fTemp816: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp815, 196607))) as usize] };
			let mut fTemp817: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp815, 1), 196607))) as usize] } - fTemp816;
			let mut iTemp818: i32 = (fTemp693 > ((if iTemp626 != 0 {fTemp816 + fTemp633 * fTemp817 + (fTemp812 - (iTemp813) as F64) * (fTemp814 - (fTemp816 + fTemp633 * (fTemp817 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp815, 4), 196607))) as usize] } - fTemp814))))} else {1.0 - (fTemp810 + fTemp633 * fTemp811 + (fTemp806 - (iTemp807) as F64) * (fTemp808 - (fTemp810 + fTemp633 * (fTemp811 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp809, 4), 196607))) as usize] } - fTemp808)))))} - fTemp804) / (1.0 - fTemp804))) as i32;
			let mut fTemp819: F64 = if iTemp818 != 0 {fTemp788} else {fTemp791};
			let mut fTemp820: F64 = if iTemp818 != 0 {fTemp791} else {fTemp789};
			let mut fTemp821: F64 = fTemp820 + fTemp819;
			let mut fTemp822: F64 = 0.5 * fTemp821;
			let mut fTemp823: F64 = 65535.0 * (1.0 - fTemp822);
			let mut iTemp824: i32 = (fTemp823) as i32;
			let mut fTemp825: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp824, 1)))) as usize] };
			let mut iTemp826: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp824));
			let mut fTemp827: F64 = unsafe { ftbl0mydspSIG0[iTemp826 as usize] };
			let mut fTemp828: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp826, 1)) as usize] } - fTemp827;
			let mut fTemp829: F64 = 32767.5 * fTemp821;
			let mut iTemp830: i32 = (fTemp829) as i32;
			let mut fTemp831: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp830, 1)))) as usize] };
			let mut iTemp832: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp830));
			let mut fTemp833: F64 = unsafe { ftbl0mydspSIG0[iTemp832 as usize] };
			let mut fTemp834: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp832, 1)) as usize] } - fTemp833;
			let mut fTemp835: F64 = if iTemp626 != 0 {fTemp833 + fTemp633 * fTemp834 + (fTemp829 - (iTemp830) as F64) * (fTemp831 - (fTemp833 + fTemp633 * (fTemp834 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp832, 4)) as usize] } - fTemp831))))} else {1.0 - (fTemp827 + fTemp633 * fTemp828 + (fTemp823 - (iTemp824) as F64) * (fTemp825 - (fTemp827 + fTemp633 * (fTemp828 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp826, 4)) as usize] } - fTemp825)))))};
			let mut fTemp836: F64 = fTemp638 + fTemp822;
			let mut fTemp837: F64 = 65535.0 * (1.0 - fTemp836);
			let mut iTemp838: i32 = (fTemp837) as i32;
			let mut fTemp839: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp838, 1))), 196607))) as usize] };
			let mut iTemp840: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp838));
			let mut fTemp841: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp840, 196607))) as usize] };
			let mut fTemp842: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp840, 1), 196607))) as usize] } - fTemp841;
			let mut fTemp843: F64 = 65535.0 * fTemp836;
			let mut iTemp844: i32 = (fTemp843) as i32;
			let mut fTemp845: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp844, 1))), 196607))) as usize] };
			let mut iTemp846: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp844));
			let mut fTemp847: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp846, 196607))) as usize] };
			let mut fTemp848: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp846, 1), 196607))) as usize] } - fTemp847;
			let mut iTemp849: i32 = (fTemp693 > ((if iTemp626 != 0 {fTemp847 + fTemp633 * fTemp848 + (fTemp843 - (iTemp844) as F64) * (fTemp845 - (fTemp847 + fTemp633 * (fTemp848 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp846, 4), 196607))) as usize] } - fTemp845))))} else {1.0 - (fTemp841 + fTemp633 * fTemp842 + (fTemp837 - (iTemp838) as F64) * (fTemp839 - (fTemp841 + fTemp633 * (fTemp842 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp840, 4), 196607))) as usize] } - fTemp839)))))} - fTemp835) / (1.0 - fTemp835))) as i32;
			let mut fTemp850: F64 = if iTemp849 != 0 {fTemp819} else {fTemp822};
			let mut fTemp851: F64 = if iTemp849 != 0 {fTemp822} else {fTemp820};
			let mut fTemp852: F64 = fTemp851 + fTemp850;
			let mut fTemp853: F64 = 0.5 * fTemp852;
			let mut fTemp854: F64 = 65535.0 * (1.0 - fTemp853);
			let mut iTemp855: i32 = (fTemp854) as i32;
			let mut fTemp856: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp855, 1)))) as usize] };
			let mut iTemp857: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp855));
			let mut fTemp858: F64 = unsafe { ftbl0mydspSIG0[iTemp857 as usize] };
			let mut fTemp859: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp857, 1)) as usize] } - fTemp858;
			let mut fTemp860: F64 = 32767.5 * fTemp852;
			let mut iTemp861: i32 = (fTemp860) as i32;
			let mut fTemp862: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp861, 1)))) as usize] };
			let mut iTemp863: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp861));
			let mut fTemp864: F64 = unsafe { ftbl0mydspSIG0[iTemp863 as usize] };
			let mut fTemp865: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp863, 1)) as usize] } - fTemp864;
			let mut fTemp866: F64 = if iTemp626 != 0 {fTemp864 + fTemp633 * fTemp865 + (fTemp860 - (iTemp861) as F64) * (fTemp862 - (fTemp864 + fTemp633 * (fTemp865 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp863, 4)) as usize] } - fTemp862))))} else {1.0 - (fTemp858 + fTemp633 * fTemp859 + (fTemp854 - (iTemp855) as F64) * (fTemp856 - (fTemp858 + fTemp633 * (fTemp859 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp857, 4)) as usize] } - fTemp856)))))};
			let mut fTemp867: F64 = fTemp638 + fTemp853;
			let mut fTemp868: F64 = 65535.0 * (1.0 - fTemp867);
			let mut iTemp869: i32 = (fTemp868) as i32;
			let mut fTemp870: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp869, 1))), 196607))) as usize] };
			let mut iTemp871: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp869));
			let mut fTemp872: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp871, 196607))) as usize] };
			let mut fTemp873: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp871, 1), 196607))) as usize] } - fTemp872;
			let mut fTemp874: F64 = 65535.0 * fTemp867;
			let mut iTemp875: i32 = (fTemp874) as i32;
			let mut fTemp876: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp875, 1))), 196607))) as usize] };
			let mut iTemp877: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp875));
			let mut fTemp878: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp877, 196607))) as usize] };
			let mut fTemp879: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp877, 1), 196607))) as usize] } - fTemp878;
			let mut iTemp880: i32 = (fTemp693 > ((if iTemp626 != 0 {fTemp878 + fTemp633 * fTemp879 + (fTemp874 - (iTemp875) as F64) * (fTemp876 - (fTemp878 + fTemp633 * (fTemp879 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp877, 4), 196607))) as usize] } - fTemp876))))} else {1.0 - (fTemp872 + fTemp633 * fTemp873 + (fTemp868 - (iTemp869) as F64) * (fTemp870 - (fTemp872 + fTemp633 * (fTemp873 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp871, 4), 196607))) as usize] } - fTemp870)))))} - fTemp866) / (1.0 - fTemp866))) as i32;
			let mut fTemp881: F64 = if iTemp880 != 0 {fTemp850} else {fTemp853};
			let mut fTemp882: F64 = if iTemp880 != 0 {fTemp853} else {fTemp851};
			let mut fTemp883: F64 = fTemp882 + fTemp881;
			let mut fTemp884: F64 = 0.5 * fTemp883;
			let mut fTemp885: F64 = 65535.0 * (1.0 - fTemp884);
			let mut iTemp886: i32 = (fTemp885) as i32;
			let mut fTemp887: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp886, 1)))) as usize] };
			let mut iTemp888: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp886));
			let mut fTemp889: F64 = unsafe { ftbl0mydspSIG0[iTemp888 as usize] };
			let mut fTemp890: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp888, 1)) as usize] } - fTemp889;
			let mut fTemp891: F64 = 32767.5 * fTemp883;
			let mut iTemp892: i32 = (fTemp891) as i32;
			let mut fTemp893: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp892, 1)))) as usize] };
			let mut iTemp894: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp892));
			let mut fTemp895: F64 = unsafe { ftbl0mydspSIG0[iTemp894 as usize] };
			let mut fTemp896: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp894, 1)) as usize] } - fTemp895;
			let mut fTemp897: F64 = if iTemp626 != 0 {fTemp895 + fTemp633 * fTemp896 + (fTemp891 - (iTemp892) as F64) * (fTemp893 - (fTemp895 + fTemp633 * (fTemp896 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp894, 4)) as usize] } - fTemp893))))} else {1.0 - (fTemp889 + fTemp633 * fTemp890 + (fTemp885 - (iTemp886) as F64) * (fTemp887 - (fTemp889 + fTemp633 * (fTemp890 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp888, 4)) as usize] } - fTemp887)))))};
			let mut fTemp898: F64 = fTemp638 + fTemp884;
			let mut fTemp899: F64 = 65535.0 * (1.0 - fTemp898);
			let mut iTemp900: i32 = (fTemp899) as i32;
			let mut fTemp901: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp900, 1))), 196607))) as usize] };
			let mut iTemp902: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp900));
			let mut fTemp903: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp902, 196607))) as usize] };
			let mut fTemp904: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp902, 1), 196607))) as usize] } - fTemp903;
			let mut fTemp905: F64 = 65535.0 * fTemp898;
			let mut iTemp906: i32 = (fTemp905) as i32;
			let mut fTemp907: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp906, 1))), 196607))) as usize] };
			let mut iTemp908: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp906));
			let mut fTemp909: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp908, 196607))) as usize] };
			let mut fTemp910: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp908, 1), 196607))) as usize] } - fTemp909;
			let mut iTemp911: i32 = (fTemp693 > ((if iTemp626 != 0 {fTemp909 + fTemp633 * fTemp910 + (fTemp905 - (iTemp906) as F64) * (fTemp907 - (fTemp909 + fTemp633 * (fTemp910 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp908, 4), 196607))) as usize] } - fTemp907))))} else {1.0 - (fTemp903 + fTemp633 * fTemp904 + (fTemp899 - (iTemp900) as F64) * (fTemp901 - (fTemp903 + fTemp633 * (fTemp904 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp902, 4), 196607))) as usize] } - fTemp901)))))} - fTemp897) / (1.0 - fTemp897))) as i32;
			let mut fTemp912: F64 = if iTemp911 != 0 {fTemp881} else {fTemp884};
			let mut fTemp913: F64 = if iTemp911 != 0 {fTemp884} else {fTemp882};
			let mut fTemp914: F64 = fTemp913 + fTemp912;
			let mut fTemp915: F64 = 0.5 * fTemp914;
			let mut fTemp916: F64 = 65535.0 * (1.0 - fTemp915);
			let mut iTemp917: i32 = (fTemp916) as i32;
			let mut fTemp918: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp917, 1)))) as usize] };
			let mut iTemp919: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp917));
			let mut fTemp920: F64 = unsafe { ftbl0mydspSIG0[iTemp919 as usize] };
			let mut fTemp921: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp919, 1)) as usize] } - fTemp920;
			let mut fTemp922: F64 = 32767.5 * fTemp914;
			let mut iTemp923: i32 = (fTemp922) as i32;
			let mut fTemp924: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp923, 1)))) as usize] };
			let mut iTemp925: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp923));
			let mut fTemp926: F64 = unsafe { ftbl0mydspSIG0[iTemp925 as usize] };
			let mut fTemp927: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp925, 1)) as usize] } - fTemp926;
			let mut fTemp928: F64 = if iTemp626 != 0 {fTemp926 + fTemp633 * fTemp927 + (fTemp922 - (iTemp923) as F64) * (fTemp924 - (fTemp926 + fTemp633 * (fTemp927 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp925, 4)) as usize] } - fTemp924))))} else {1.0 - (fTemp920 + fTemp633 * fTemp921 + (fTemp916 - (iTemp917) as F64) * (fTemp918 - (fTemp920 + fTemp633 * (fTemp921 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp919, 4)) as usize] } - fTemp918)))))};
			let mut fTemp929: F64 = fTemp638 + fTemp915;
			let mut fTemp930: F64 = 65535.0 * (1.0 - fTemp929);
			let mut iTemp931: i32 = (fTemp930) as i32;
			let mut fTemp932: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp931, 1))), 196607))) as usize] };
			let mut iTemp933: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp931));
			let mut fTemp934: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp933, 196607))) as usize] };
			let mut fTemp935: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp933, 1), 196607))) as usize] } - fTemp934;
			let mut fTemp936: F64 = 65535.0 * fTemp929;
			let mut iTemp937: i32 = (fTemp936) as i32;
			let mut fTemp938: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp937, 1))), 196607))) as usize] };
			let mut iTemp939: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp937));
			let mut fTemp940: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp939, 196607))) as usize] };
			let mut fTemp941: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp939, 1), 196607))) as usize] } - fTemp940;
			let mut iTemp942: i32 = (fTemp693 > ((if iTemp626 != 0 {fTemp940 + fTemp633 * fTemp941 + (fTemp936 - (iTemp937) as F64) * (fTemp938 - (fTemp940 + fTemp633 * (fTemp941 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp939, 4), 196607))) as usize] } - fTemp938))))} else {1.0 - (fTemp934 + fTemp633 * fTemp935 + (fTemp930 - (iTemp931) as F64) * (fTemp932 - (fTemp934 + fTemp633 * (fTemp935 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp933, 4), 196607))) as usize] } - fTemp932)))))} - fTemp928) / (1.0 - fTemp928))) as i32;
			let mut fTemp943: F64 = if iTemp942 != 0 {fTemp912} else {fTemp915};
			let mut fTemp944: F64 = if iTemp942 != 0 {fTemp915} else {fTemp913};
			let mut fTemp945: F64 = fTemp944 + fTemp943;
			let mut fTemp946: F64 = 0.5 * fTemp945;
			let mut fTemp947: F64 = 65535.0 * (1.0 - fTemp946);
			let mut iTemp948: i32 = (fTemp947) as i32;
			let mut fTemp949: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp948, 1)))) as usize] };
			let mut iTemp950: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp948));
			let mut fTemp951: F64 = unsafe { ftbl0mydspSIG0[iTemp950 as usize] };
			let mut fTemp952: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp950, 1)) as usize] } - fTemp951;
			let mut fTemp953: F64 = 32767.5 * fTemp945;
			let mut iTemp954: i32 = (fTemp953) as i32;
			let mut fTemp955: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp954, 1)))) as usize] };
			let mut iTemp956: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp954));
			let mut fTemp957: F64 = unsafe { ftbl0mydspSIG0[iTemp956 as usize] };
			let mut fTemp958: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp956, 1)) as usize] } - fTemp957;
			let mut fTemp959: F64 = if iTemp626 != 0 {fTemp957 + fTemp633 * fTemp958 + (fTemp953 - (iTemp954) as F64) * (fTemp955 - (fTemp957 + fTemp633 * (fTemp958 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp956, 4)) as usize] } - fTemp955))))} else {1.0 - (fTemp951 + fTemp633 * fTemp952 + (fTemp947 - (iTemp948) as F64) * (fTemp949 - (fTemp951 + fTemp633 * (fTemp952 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp950, 4)) as usize] } - fTemp949)))))};
			let mut fTemp960: F64 = fTemp638 + fTemp946;
			let mut fTemp961: F64 = 65535.0 * (1.0 - fTemp960);
			let mut iTemp962: i32 = (fTemp961) as i32;
			let mut fTemp963: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp962, 1))), 196607))) as usize] };
			let mut iTemp964: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp962));
			let mut fTemp965: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp964, 196607))) as usize] };
			let mut fTemp966: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp964, 1), 196607))) as usize] } - fTemp965;
			let mut fTemp967: F64 = 65535.0 * fTemp960;
			let mut iTemp968: i32 = (fTemp967) as i32;
			let mut fTemp969: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp968, 1))), 196607))) as usize] };
			let mut iTemp970: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp968));
			let mut fTemp971: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp970, 196607))) as usize] };
			let mut fTemp972: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp970, 1), 196607))) as usize] } - fTemp971;
			let mut iTemp973: i32 = (fTemp693 > ((if iTemp626 != 0 {fTemp971 + fTemp633 * fTemp972 + (fTemp967 - (iTemp968) as F64) * (fTemp969 - (fTemp971 + fTemp633 * (fTemp972 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp970, 4), 196607))) as usize] } - fTemp969))))} else {1.0 - (fTemp965 + fTemp633 * fTemp966 + (fTemp961 - (iTemp962) as F64) * (fTemp963 - (fTemp965 + fTemp633 * (fTemp966 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp964, 4), 196607))) as usize] } - fTemp963)))))} - fTemp959) / (1.0 - fTemp959))) as i32;
			let mut fTemp974: F64 = if iTemp973 != 0 {fTemp943} else {fTemp946};
			let mut fTemp975: F64 = if iTemp973 != 0 {fTemp946} else {fTemp944};
			let mut fTemp976: F64 = fTemp975 + fTemp974;
			let mut fTemp977: F64 = 0.5 * fTemp976;
			let mut fTemp978: F64 = 65535.0 * (1.0 - fTemp977);
			let mut iTemp979: i32 = (fTemp978) as i32;
			let mut fTemp980: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp979, 1)))) as usize] };
			let mut iTemp981: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp979));
			let mut fTemp982: F64 = unsafe { ftbl0mydspSIG0[iTemp981 as usize] };
			let mut fTemp983: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp981, 1)) as usize] } - fTemp982;
			let mut fTemp984: F64 = 32767.5 * fTemp976;
			let mut iTemp985: i32 = (fTemp984) as i32;
			let mut fTemp986: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp985, 1)))) as usize] };
			let mut iTemp987: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp985));
			let mut fTemp988: F64 = unsafe { ftbl0mydspSIG0[iTemp987 as usize] };
			let mut fTemp989: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp987, 1)) as usize] } - fTemp988;
			let mut fTemp990: F64 = if iTemp626 != 0 {fTemp988 + fTemp633 * fTemp989 + (fTemp984 - (iTemp985) as F64) * (fTemp986 - (fTemp988 + fTemp633 * (fTemp989 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp987, 4)) as usize] } - fTemp986))))} else {1.0 - (fTemp982 + fTemp633 * fTemp983 + (fTemp978 - (iTemp979) as F64) * (fTemp980 - (fTemp982 + fTemp633 * (fTemp983 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp981, 4)) as usize] } - fTemp980)))))};
			let mut fTemp991: F64 = fTemp638 + fTemp977;
			let mut fTemp992: F64 = 65535.0 * (1.0 - fTemp991);
			let mut iTemp993: i32 = (fTemp992) as i32;
			let mut fTemp994: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp993, 1))), 196607))) as usize] };
			let mut iTemp995: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp993));
			let mut fTemp996: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp995, 196607))) as usize] };
			let mut fTemp997: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp995, 1), 196607))) as usize] } - fTemp996;
			let mut fTemp998: F64 = 65535.0 * fTemp991;
			let mut iTemp999: i32 = (fTemp998) as i32;
			let mut fTemp1000: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp999, 1))), 196607))) as usize] };
			let mut iTemp1001: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp999));
			let mut fTemp1002: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp1001, 196607))) as usize] };
			let mut fTemp1003: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1001, 1), 196607))) as usize] } - fTemp1002;
			let mut iTemp1004: i32 = (fTemp693 > ((if iTemp626 != 0 {fTemp1002 + fTemp633 * fTemp1003 + (fTemp998 - (iTemp999) as F64) * (fTemp1000 - (fTemp1002 + fTemp633 * (fTemp1003 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1001, 4), 196607))) as usize] } - fTemp1000))))} else {1.0 - (fTemp996 + fTemp633 * fTemp997 + (fTemp992 - (iTemp993) as F64) * (fTemp994 - (fTemp996 + fTemp633 * (fTemp997 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp995, 4), 196607))) as usize] } - fTemp994)))))} - fTemp990) / (1.0 - fTemp990))) as i32;
			let mut fTemp1005: F64 = if iTemp1004 != 0 {fTemp974} else {fTemp977};
			let mut fTemp1006: F64 = if iTemp1004 != 0 {fTemp977} else {fTemp975};
			let mut fTemp1007: F64 = fTemp1006 + fTemp1005;
			let mut fTemp1008: F64 = 0.5 * fTemp1007;
			let mut fTemp1009: F64 = 65535.0 * (1.0 - fTemp1008);
			let mut iTemp1010: i32 = (fTemp1009) as i32;
			let mut fTemp1011: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp1010, 1)))) as usize] };
			let mut iTemp1012: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp1010));
			let mut fTemp1013: F64 = unsafe { ftbl0mydspSIG0[iTemp1012 as usize] };
			let mut fTemp1014: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1012, 1)) as usize] } - fTemp1013;
			let mut fTemp1015: F64 = 32767.5 * fTemp1007;
			let mut iTemp1016: i32 = (fTemp1015) as i32;
			let mut fTemp1017: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp1016, 1)))) as usize] };
			let mut iTemp1018: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp1016));
			let mut fTemp1019: F64 = unsafe { ftbl0mydspSIG0[iTemp1018 as usize] };
			let mut fTemp1020: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1018, 1)) as usize] } - fTemp1019;
			let mut fTemp1021: F64 = if iTemp626 != 0 {fTemp1019 + fTemp633 * fTemp1020 + (fTemp1015 - (iTemp1016) as F64) * (fTemp1017 - (fTemp1019 + fTemp633 * (fTemp1020 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1018, 4)) as usize] } - fTemp1017))))} else {1.0 - (fTemp1013 + fTemp633 * fTemp1014 + (fTemp1009 - (iTemp1010) as F64) * (fTemp1011 - (fTemp1013 + fTemp633 * (fTemp1014 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1012, 4)) as usize] } - fTemp1011)))))};
			let mut fTemp1022: F64 = fTemp638 + fTemp1008;
			let mut fTemp1023: F64 = 65535.0 * (1.0 - fTemp1022);
			let mut iTemp1024: i32 = (fTemp1023) as i32;
			let mut fTemp1025: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp1024, 1))), 196607))) as usize] };
			let mut iTemp1026: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp1024));
			let mut fTemp1027: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp1026, 196607))) as usize] };
			let mut fTemp1028: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1026, 1), 196607))) as usize] } - fTemp1027;
			let mut fTemp1029: F64 = 65535.0 * fTemp1022;
			let mut iTemp1030: i32 = (fTemp1029) as i32;
			let mut fTemp1031: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp1030, 1))), 196607))) as usize] };
			let mut iTemp1032: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp1030));
			let mut fTemp1033: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp1032, 196607))) as usize] };
			let mut fTemp1034: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1032, 1), 196607))) as usize] } - fTemp1033;
			let mut iTemp1035: i32 = (fTemp693 > ((if iTemp626 != 0 {fTemp1033 + fTemp633 * fTemp1034 + (fTemp1029 - (iTemp1030) as F64) * (fTemp1031 - (fTemp1033 + fTemp633 * (fTemp1034 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1032, 4), 196607))) as usize] } - fTemp1031))))} else {1.0 - (fTemp1027 + fTemp633 * fTemp1028 + (fTemp1023 - (iTemp1024) as F64) * (fTemp1025 - (fTemp1027 + fTemp633 * (fTemp1028 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1026, 4), 196607))) as usize] } - fTemp1025)))))} - fTemp1021) / (1.0 - fTemp1021))) as i32;
			let mut fTemp1036: F64 = if iTemp1035 != 0 {fTemp1005} else {fTemp1008};
			let mut fTemp1037: F64 = if iTemp1035 != 0 {fTemp1008} else {fTemp1006};
			let mut fTemp1038: F64 = fTemp1037 + fTemp1036;
			let mut fTemp1039: F64 = 0.5 * fTemp1038;
			let mut fTemp1040: F64 = 65535.0 * (1.0 - fTemp1039);
			let mut iTemp1041: i32 = (fTemp1040) as i32;
			let mut fTemp1042: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp1041, 1)))) as usize] };
			let mut iTemp1043: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp1041));
			let mut fTemp1044: F64 = unsafe { ftbl0mydspSIG0[iTemp1043 as usize] };
			let mut fTemp1045: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1043, 1)) as usize] } - fTemp1044;
			let mut fTemp1046: F64 = 32767.5 * fTemp1038;
			let mut iTemp1047: i32 = (fTemp1046) as i32;
			let mut fTemp1048: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp1047, 1)))) as usize] };
			let mut iTemp1049: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp1047));
			let mut fTemp1050: F64 = unsafe { ftbl0mydspSIG0[iTemp1049 as usize] };
			let mut fTemp1051: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1049, 1)) as usize] } - fTemp1050;
			let mut fTemp1052: F64 = if iTemp626 != 0 {fTemp1050 + fTemp633 * fTemp1051 + (fTemp1046 - (iTemp1047) as F64) * (fTemp1048 - (fTemp1050 + fTemp633 * (fTemp1051 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1049, 4)) as usize] } - fTemp1048))))} else {1.0 - (fTemp1044 + fTemp633 * fTemp1045 + (fTemp1040 - (iTemp1041) as F64) * (fTemp1042 - (fTemp1044 + fTemp633 * (fTemp1045 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1043, 4)) as usize] } - fTemp1042)))))};
			let mut fTemp1053: F64 = fTemp638 + fTemp1039;
			let mut fTemp1054: F64 = 65535.0 * (1.0 - fTemp1053);
			let mut iTemp1055: i32 = (fTemp1054) as i32;
			let mut fTemp1056: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp1055, 1))), 196607))) as usize] };
			let mut iTemp1057: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp1055));
			let mut fTemp1058: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp1057, 196607))) as usize] };
			let mut fTemp1059: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1057, 1), 196607))) as usize] } - fTemp1058;
			let mut fTemp1060: F64 = 65535.0 * fTemp1053;
			let mut iTemp1061: i32 = (fTemp1060) as i32;
			let mut fTemp1062: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp1061, 1))), 196607))) as usize] };
			let mut iTemp1063: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp1061));
			let mut fTemp1064: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp1063, 196607))) as usize] };
			let mut fTemp1065: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1063, 1), 196607))) as usize] } - fTemp1064;
			let mut iTemp1066: i32 = (fTemp693 > ((if iTemp626 != 0 {fTemp1064 + fTemp633 * fTemp1065 + (fTemp1060 - (iTemp1061) as F64) * (fTemp1062 - (fTemp1064 + fTemp633 * (fTemp1065 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1063, 4), 196607))) as usize] } - fTemp1062))))} else {1.0 - (fTemp1058 + fTemp633 * fTemp1059 + (fTemp1054 - (iTemp1055) as F64) * (fTemp1056 - (fTemp1058 + fTemp633 * (fTemp1059 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1057, 4), 196607))) as usize] } - fTemp1056)))))} - fTemp1052) / (1.0 - fTemp1052))) as i32;
			let mut fTemp1067: F64 = if iTemp1066 != 0 {fTemp1036} else {fTemp1039};
			let mut fTemp1068: F64 = if iTemp1066 != 0 {fTemp1039} else {fTemp1037};
			let mut fTemp1069: F64 = fTemp1068 + fTemp1067;
			let mut fTemp1070: F64 = 0.5 * fTemp1069;
			let mut fTemp1071: F64 = 65535.0 * (1.0 - fTemp1070);
			let mut iTemp1072: i32 = (fTemp1071) as i32;
			let mut fTemp1073: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp1072, 1)))) as usize] };
			let mut iTemp1074: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp1072));
			let mut fTemp1075: F64 = unsafe { ftbl0mydspSIG0[iTemp1074 as usize] };
			let mut fTemp1076: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1074, 1)) as usize] } - fTemp1075;
			let mut fTemp1077: F64 = 32767.5 * fTemp1069;
			let mut iTemp1078: i32 = (fTemp1077) as i32;
			let mut fTemp1079: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp1078, 1)))) as usize] };
			let mut iTemp1080: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp1078));
			let mut fTemp1081: F64 = unsafe { ftbl0mydspSIG0[iTemp1080 as usize] };
			let mut fTemp1082: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1080, 1)) as usize] } - fTemp1081;
			let mut fTemp1083: F64 = if iTemp626 != 0 {fTemp1081 + fTemp633 * fTemp1082 + (fTemp1077 - (iTemp1078) as F64) * (fTemp1079 - (fTemp1081 + fTemp633 * (fTemp1082 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1080, 4)) as usize] } - fTemp1079))))} else {1.0 - (fTemp1075 + fTemp633 * fTemp1076 + (fTemp1071 - (iTemp1072) as F64) * (fTemp1073 - (fTemp1075 + fTemp633 * (fTemp1076 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1074, 4)) as usize] } - fTemp1073)))))};
			let mut fTemp1084: F64 = fTemp638 + fTemp1070;
			let mut fTemp1085: F64 = 65535.0 * (1.0 - fTemp1084);
			let mut iTemp1086: i32 = (fTemp1085) as i32;
			let mut fTemp1087: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp1086, 1))), 196607))) as usize] };
			let mut iTemp1088: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp1086));
			let mut fTemp1089: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp1088, 196607))) as usize] };
			let mut fTemp1090: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1088, 1), 196607))) as usize] } - fTemp1089;
			let mut fTemp1091: F64 = 65535.0 * fTemp1084;
			let mut iTemp1092: i32 = (fTemp1091) as i32;
			let mut fTemp1093: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp1092, 1))), 196607))) as usize] };
			let mut iTemp1094: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp1092));
			let mut fTemp1095: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp1094, 196607))) as usize] };
			let mut fTemp1096: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1094, 1), 196607))) as usize] } - fTemp1095;
			let mut iTemp1097: i32 = (fTemp693 > ((if iTemp626 != 0 {fTemp1095 + fTemp633 * fTemp1096 + (fTemp1091 - (iTemp1092) as F64) * (fTemp1093 - (fTemp1095 + fTemp633 * (fTemp1096 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1094, 4), 196607))) as usize] } - fTemp1093))))} else {1.0 - (fTemp1089 + fTemp633 * fTemp1090 + (fTemp1085 - (iTemp1086) as F64) * (fTemp1087 - (fTemp1089 + fTemp633 * (fTemp1090 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1088, 4), 196607))) as usize] } - fTemp1087)))))} - fTemp1083) / (1.0 - fTemp1083))) as i32;
			let mut fTemp1098: F64 = if iTemp1097 != 0 {fTemp1067} else {fTemp1070};
			let mut fTemp1099: F64 = if iTemp1097 != 0 {fTemp1070} else {fTemp1068};
			let mut fTemp1100: F64 = fTemp1099 + fTemp1098;
			let mut fTemp1101: F64 = 0.5 * fTemp1100;
			let mut fTemp1102: F64 = 65535.0 * (1.0 - fTemp1101);
			let mut iTemp1103: i32 = (fTemp1102) as i32;
			let mut fTemp1104: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp1103, 1)))) as usize] };
			let mut iTemp1105: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp1103));
			let mut fTemp1106: F64 = unsafe { ftbl0mydspSIG0[iTemp1105 as usize] };
			let mut fTemp1107: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1105, 1)) as usize] } - fTemp1106;
			let mut fTemp1108: F64 = 32767.5 * fTemp1100;
			let mut iTemp1109: i32 = (fTemp1108) as i32;
			let mut fTemp1110: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp1109, 1)))) as usize] };
			let mut iTemp1111: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp1109));
			let mut fTemp1112: F64 = unsafe { ftbl0mydspSIG0[iTemp1111 as usize] };
			let mut fTemp1113: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1111, 1)) as usize] } - fTemp1112;
			let mut fTemp1114: F64 = if iTemp626 != 0 {fTemp1112 + fTemp633 * fTemp1113 + (fTemp1108 - (iTemp1109) as F64) * (fTemp1110 - (fTemp1112 + fTemp633 * (fTemp1113 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1111, 4)) as usize] } - fTemp1110))))} else {1.0 - (fTemp1106 + fTemp633 * fTemp1107 + (fTemp1102 - (iTemp1103) as F64) * (fTemp1104 - (fTemp1106 + fTemp633 * (fTemp1107 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1105, 4)) as usize] } - fTemp1104)))))};
			let mut fTemp1115: F64 = fTemp638 + fTemp1101;
			let mut fTemp1116: F64 = 65535.0 * (1.0 - fTemp1115);
			let mut iTemp1117: i32 = (fTemp1116) as i32;
			let mut fTemp1118: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp1117, 1))), 196607))) as usize] };
			let mut iTemp1119: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp1117));
			let mut fTemp1120: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp1119, 196607))) as usize] };
			let mut fTemp1121: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1119, 1), 196607))) as usize] } - fTemp1120;
			let mut fTemp1122: F64 = 65535.0 * fTemp1115;
			let mut iTemp1123: i32 = (fTemp1122) as i32;
			let mut fTemp1124: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp1123, 1))), 196607))) as usize] };
			let mut iTemp1125: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp1123));
			let mut fTemp1126: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp1125, 196607))) as usize] };
			let mut fTemp1127: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1125, 1), 196607))) as usize] } - fTemp1126;
			let mut iTemp1128: i32 = (fTemp693 > ((if iTemp626 != 0 {fTemp1126 + fTemp633 * fTemp1127 + (fTemp1122 - (iTemp1123) as F64) * (fTemp1124 - (fTemp1126 + fTemp633 * (fTemp1127 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1125, 4), 196607))) as usize] } - fTemp1124))))} else {1.0 - (fTemp1120 + fTemp633 * fTemp1121 + (fTemp1116 - (iTemp1117) as F64) * (fTemp1118 - (fTemp1120 + fTemp633 * (fTemp1121 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1119, 4), 196607))) as usize] } - fTemp1118)))))} - fTemp1114) / (1.0 - fTemp1114))) as i32;
			let mut fTemp1129: F64 = if iTemp1128 != 0 {fTemp1098} else {fTemp1101};
			let mut fTemp1130: F64 = if iTemp1128 != 0 {fTemp1101} else {fTemp1099};
			let mut fTemp1131: F64 = fTemp1130 + fTemp1129;
			let mut fTemp1132: F64 = 0.5 * fTemp1131;
			let mut fTemp1133: F64 = 65535.0 * (1.0 - fTemp1132);
			let mut iTemp1134: i32 = (fTemp1133) as i32;
			let mut fTemp1135: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp1134, 1)))) as usize] };
			let mut iTemp1136: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp1134));
			let mut fTemp1137: F64 = unsafe { ftbl0mydspSIG0[iTemp1136 as usize] };
			let mut fTemp1138: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1136, 1)) as usize] } - fTemp1137;
			let mut fTemp1139: F64 = 32767.5 * fTemp1131;
			let mut iTemp1140: i32 = (fTemp1139) as i32;
			let mut fTemp1141: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp1140, 1)))) as usize] };
			let mut iTemp1142: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp1140));
			let mut fTemp1143: F64 = unsafe { ftbl0mydspSIG0[iTemp1142 as usize] };
			let mut fTemp1144: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1142, 1)) as usize] } - fTemp1143;
			let mut fTemp1145: F64 = if iTemp626 != 0 {fTemp1143 + fTemp633 * fTemp1144 + (fTemp1139 - (iTemp1140) as F64) * (fTemp1141 - (fTemp1143 + fTemp633 * (fTemp1144 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1142, 4), 196607))) as usize] } - fTemp1141))))} else {1.0 - (fTemp1137 + fTemp633 * fTemp1138 + (fTemp1133 - (iTemp1134) as F64) * (fTemp1135 - (fTemp1137 + fTemp633 * (fTemp1138 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1136, 4), 196607))) as usize] } - fTemp1135)))))};
			let mut fTemp1146: F64 = fTemp638 + fTemp1132;
			let mut fTemp1147: F64 = 65535.0 * (1.0 - fTemp1146);
			let mut iTemp1148: i32 = (fTemp1147) as i32;
			let mut fTemp1149: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp1148, 1))), 196607))) as usize] };
			let mut iTemp1150: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp1148));
			let mut fTemp1151: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp1150, 196607))) as usize] };
			let mut fTemp1152: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1150, 1), 196607))) as usize] } - fTemp1151;
			let mut fTemp1153: F64 = 65535.0 * fTemp1146;
			let mut iTemp1154: i32 = (fTemp1153) as i32;
			let mut fTemp1155: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp1154, 1))), 196607))) as usize] };
			let mut iTemp1156: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp1154));
			let mut fTemp1157: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp1156, 196607))) as usize] };
			let mut fTemp1158: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1156, 1), 196607))) as usize] } - fTemp1157;
			let mut iTemp1159: i32 = (fTemp693 > ((if iTemp626 != 0 {fTemp1157 + fTemp633 * fTemp1158 + (fTemp1153 - (iTemp1154) as F64) * (fTemp1155 - (fTemp1157 + fTemp633 * (fTemp1158 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1156, 4), 196607))) as usize] } - fTemp1155))))} else {1.0 - (fTemp1151 + fTemp633 * fTemp1152 + (fTemp1147 - (iTemp1148) as F64) * (fTemp1149 - (fTemp1151 + fTemp633 * (fTemp1152 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1150, 4), 196607))) as usize] } - fTemp1149)))))} - fTemp1145) / (1.0 - fTemp1145))) as i32;
			let mut fTemp1160: F64 = F64::min(1.0, F64::max(0.0, 0.5 * (if iTemp1159 != 0 {fTemp1132} else {fTemp1130} + if iTemp1159 != 0 {fTemp1129} else {fTemp1132})));
			self.fRec5[0] = fTemp1160;
			let mut fTemp1161: F64 = 65535.0 * (1.0 - fTemp1160);
			let mut iTemp1162: i32 = (fTemp1161) as i32;
			let mut fTemp1163: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp1162, 1)))) as usize] };
			let mut iTemp1164: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp1162));
			let mut fTemp1165: F64 = unsafe { ftbl0mydspSIG0[iTemp1164 as usize] };
			let mut fTemp1166: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1164, 1)) as usize] } - fTemp1165;
			let mut fTemp1167: F64 = 65535.0 * fTemp1160;
			let mut iTemp1168: i32 = (fTemp1167) as i32;
			let mut fTemp1169: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp1168, 1)))) as usize] };
			let mut iTemp1170: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp1168));
			let mut fTemp1171: F64 = unsafe { ftbl0mydspSIG0[iTemp1170 as usize] };
			let mut fTemp1172: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp1170, 1)) as usize] } - fTemp1171;
			let mut fTemp1173: F64 = if iTemp626 != 0 {fTemp1171 + fTemp633 * fTemp1172 + (fTemp1167 - (iTemp1168) as F64) * (fTemp1169 - (fTemp1171 + fTemp633 * (fTemp1172 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1170, 4), 196607))) as usize] } - fTemp1169))))} else {1.0 - (fTemp1165 + fTemp633 * fTemp1166 + (fTemp1161 - (iTemp1162) as F64) * (fTemp1163 - (fTemp1165 + fTemp633 * (fTemp1166 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1164, 4), 196607))) as usize] } - fTemp1163)))))};
			let mut fTemp1174: F64 = fTemp638 + fTemp1160;
			let mut fTemp1175: F64 = 65535.0 * (1.0 - fTemp1174);
			let mut iTemp1176: i32 = (fTemp1175) as i32;
			let mut fTemp1177: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp1176, 1))), 196607))) as usize] };
			let mut iTemp1178: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp1176));
			let mut fTemp1179: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp1178, 196607))) as usize] };
			let mut fTemp1180: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1178, 1), 196607))) as usize] } - fTemp1179;
			let mut fTemp1181: F64 = 65535.0 * fTemp1174;
			let mut iTemp1182: i32 = (fTemp1181) as i32;
			let mut fTemp1183: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp629, i32::wrapping_mul(3, i32::wrapping_add(iTemp1182, 1))), 196607))) as usize] };
			let mut iTemp1184: i32 = i32::wrapping_add(iTemp629, i32::wrapping_mul(3, iTemp1182));
			let mut fTemp1185: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp1184, 196607))) as usize] };
			let mut fTemp1186: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1184, 1), 196607))) as usize] } - fTemp1185;
			let mut fTemp1187: F64 = self.fRec6[1] + fTemp625 * (if iTemp626 != 0 {fTemp1185 + fTemp633 * fTemp1186 + (fTemp1181 - (iTemp1182) as F64) * (fTemp1183 - (fTemp1185 + fTemp633 * (fTemp1186 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1184, 4), 196607))) as usize] } - fTemp1183))))} else {1.0 - (fTemp1179 + fTemp633 * fTemp1180 + (fTemp1175 - (iTemp1176) as F64) * (fTemp1177 - (fTemp1179 + fTemp633 * (fTemp1180 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp1178, 4), 196607))) as usize] } - fTemp1177)))))} - fTemp1173) / (1.0 - fTemp1173);
			self.fRec6[0] = F64::min(self.fRec7[((i32::wrapping_sub(self.IOTA0, iSlow64)) & 16383) as usize], if iTemp636 != 0 {F64::min(fTemp1187, self.fRec6[1])} else {F64::max(fTemp1187, self.fRec6[1])});
			self.fVbargraph1 = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, self.fRec6[0]));
			*output1 = self.fRec6[0] * self.fVec1[((i32::wrapping_sub(self.IOTA0, iSlow3)) & 32767) as usize];
			self.fRec0[1] = self.fRec0[0];
			self.IOTA0 = i32::wrapping_add(self.IOTA0, 1);
			self.fVec3[2] = self.fVec3[1];
			self.fVec3[1] = self.fVec3[0];
			for j0 in (1..=6).rev() {
				self.fVec4[j0 as usize] = self.fVec4[(i32::wrapping_sub(j0, 1)) as usize];
			}
			for j1 in (1..=14).rev() {
				self.fVec5[j1 as usize] = self.fVec5[(i32::wrapping_sub(j1, 1)) as usize];
			}
			self.fVec14[2] = self.fVec14[1];
			self.fVec14[1] = self.fVec14[0];
			for j2 in (1..=6).rev() {
				self.fVec15[j2 as usize] = self.fVec15[(i32::wrapping_sub(j2, 1)) as usize];
			}
			for j3 in (1..=14).rev() {
				self.fVec16[j3 as usize] = self.fVec16[(i32::wrapping_sub(j3, 1)) as usize];
			}
			self.fVec25[1] = self.fVec25[0];
			self.fVec26[1] = self.fVec26[0];
			self.fVec27[1] = self.fVec27[0];
			self.fRec1[1] = self.fRec1[0];
			self.fRec2[1] = self.fRec2[0];
			self.fVec29[2] = self.fVec29[1];
			self.fVec29[1] = self.fVec29[0];
			for j4 in (1..=6).rev() {
				self.fVec30[j4 as usize] = self.fVec30[(i32::wrapping_sub(j4, 1)) as usize];
			}
			for j5 in (1..=14).rev() {
				self.fVec31[j5 as usize] = self.fVec31[(i32::wrapping_sub(j5, 1)) as usize];
			}
			self.fVec40[2] = self.fVec40[1];
			self.fVec40[1] = self.fVec40[0];
			for j6 in (1..=6).rev() {
				self.fVec41[j6 as usize] = self.fVec41[(i32::wrapping_sub(j6, 1)) as usize];
			}
			for j7 in (1..=14).rev() {
				self.fVec42[j7 as usize] = self.fVec42[(i32::wrapping_sub(j7, 1)) as usize];
			}
			self.fVec51[1] = self.fVec51[0];
			self.fVec52[1] = self.fVec52[0];
			self.fVec53[1] = self.fVec53[0];
			self.fRec5[1] = self.fRec5[0];
			self.fRec6[1] = self.fRec6[0];
		}
	}

}


}
pub use dsp::mydsp as Lamb;
