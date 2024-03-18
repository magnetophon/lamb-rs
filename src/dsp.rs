mod dsp {
    /* ------------------------------------------------------------
author: "Bart Brouns"
license: "AGPLv3"
name: "lamb"
version: "0.1"
Code generated with Faust 2.72.10 (https://faust.grame.fr)
Compilation options: -a /run/user/1001/.tmpzmRGCu -lang rust -ct 1 -es 1 -mcd 16 -mdd 1024 -mdy 33 -double -ftz 0
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


fn mydspSIG0_faustpower2_f(value: F64) -> F64 {
	return value * value;
}

#[derive(Debug,Clone)]
pub struct mydspSIG0 {
	iRec2: [i32;2],
}

impl mydspSIG0 {
	
	fn get_num_inputsmydspSIG0(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsmydspSIG0(&self) -> i32 {
		return 1;
	}
	
	fn instance_initmydspSIG0(&mut self, sample_rate: i32) {
		for l0 in 0..2 {
			self.iRec2[l0 as usize] = 0;
		}
	}
	
	fn fillmydspSIG0(&mut self, count: i32, table: &mut[F64]) {
		for i1 in 0..count {
			self.iRec2[0] = i32::wrapping_add(self.iRec2[1], 1);
			let mut iTemp0: i32 = i32::wrapping_add(self.iRec2[0], -1);
			let mut fTemp1: F64 = 0.05625 * (iTemp0 % 9) as F64 as i32 as F64;
			let mut fTemp2: F64 = fTemp1 + 0.3;
			let mut fTemp3: F64 = 1.0 / fTemp2;
			let mut fTemp4: F64 = F64::min(2.0 * fTemp2, 2.0 * (1.0 - fTemp2));
			let mut fTemp5: F64 = 0.5 * fTemp4;
			let mut fTemp6: F64 = F64::min(1.0, F64::max(0.0, 1.5259021896696422e-05 * (0.1111111111111111 * (iTemp0 % 589824) as F64) as i32 as F64));
			let mut iTemp7: i32 = ((fTemp6 > (fTemp1 + (0.3 - fTemp5))) as i32) + ((fTemp6 > (fTemp1 + fTemp5 + 0.3)) as i32);
			let mut fTemp8: F64 = -0.3 - fTemp1;
			table[i1 as usize] = F64::powf(0.5 * (F64::sin(6.283185307179586 * (0.25 * ((fTemp6 - (fTemp3 + -2.0) * if (iTemp7 == 0) as i32 != 0 {0.0} else {if (iTemp7 == 1) as i32 != 0 {0.5 * (mydspSIG0_faustpower2_f(fTemp6 + fTemp5 + fTemp8) / fTemp4)} else {fTemp6 + fTemp8}} / (fTemp3 + -1.0)) / fTemp2) + 0.75)) + 1.0), 1.3333333333333333 * fTemp2 + 0.3333333333333333);
			self.iRec2[1] = self.iRec2[0];
		}
	}

}


pub fn newmydspSIG0() -> mydspSIG0 { 
	mydspSIG0 {
		iRec2: [0;2],
	}
}
static mut ftbl0mydspSIG0: [F64;589824] = [0.0;589824];
fn mydsp_faustpower2_f(value: F64) -> F64 {
	return value * value;
}
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
	fHslider0: F64,
	fSampleRate: i32,
	fConst1: F64,
	fHslider1: F64,
	fHslider2: F64,
	fConst2: F64,
	fConst3: F64,
	fHslider3: F64,
	fRec3: [F64;2],
	IOTA0: i32,
	fVec0: [F64;32768],
	fVec1: [F64;32768],
	fHslider4: F64,
	fHslider5: F64,
	fVec2: [F64;2],
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
	fVec14: [F64;8192],
	fVec15: [F64;16384],
	fVec16: [F64;32768],
	fVec17: [F64;32768],
	fVec18: [F64;2],
	fHslider6: F64,
	fHslider7: F64,
	fVec19: [F64;2],
	fHslider8: F64,
	fVec20: [F64;2],
	fConst4: F64,
	fRec0: [F64;2],
	fRec1: [F64;2],
	fHbargraph0: F64,
	fVec21: [F64;2],
	fVec22: [F64;3],
	fVec23: [F64;7],
	fVec24: [F64;15],
	fVec25: [F64;32],
	fVec26: [F64;64],
	fVec27: [F64;128],
	fVec28: [F64;256],
	fVec29: [F64;512],
	fVec30: [F64;1024],
	fVec31: [F64;2048],
	fVec32: [F64;4096],
	fVec33: [F64;8192],
	fVec34: [F64;16384],
	fVec35: [F64;32768],
	fVec36: [F64;32768],
	fVec37: [F64;2],
	fVec38: [F64;2],
	fVec39: [F64;2],
	fRec4: [F64;2],
	fRec5: [F64;2],
	fHbargraph1: F64,
}

impl FaustDsp for mydsp {
	type T = F64;
		
	fn new() -> mydsp { 
		mydsp {
			fHslider0: 0.0,
			fSampleRate: 0,
			fConst1: 0.0,
			fHslider1: 0.0,
			fHslider2: 0.0,
			fConst2: 0.0,
			fConst3: 0.0,
			fHslider3: 0.0,
			fRec3: [0.0;2],
			IOTA0: 0,
			fVec0: [0.0;32768],
			fVec1: [0.0;32768],
			fHslider4: 0.0,
			fHslider5: 0.0,
			fVec2: [0.0;2],
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
			fVec14: [0.0;8192],
			fVec15: [0.0;16384],
			fVec16: [0.0;32768],
			fVec17: [0.0;32768],
			fVec18: [0.0;2],
			fHslider6: 0.0,
			fHslider7: 0.0,
			fVec19: [0.0;2],
			fHslider8: 0.0,
			fVec20: [0.0;2],
			fConst4: 0.0,
			fRec0: [0.0;2],
			fRec1: [0.0;2],
			fHbargraph0: 0.0,
			fVec21: [0.0;2],
			fVec22: [0.0;3],
			fVec23: [0.0;7],
			fVec24: [0.0;15],
			fVec25: [0.0;32],
			fVec26: [0.0;64],
			fVec27: [0.0;128],
			fVec28: [0.0;256],
			fVec29: [0.0;512],
			fVec30: [0.0;1024],
			fVec31: [0.0;2048],
			fVec32: [0.0;4096],
			fVec33: [0.0;8192],
			fVec34: [0.0;16384],
			fVec35: [0.0;32768],
			fVec36: [0.0;32768],
			fVec37: [0.0;2],
			fVec38: [0.0;2],
			fVec39: [0.0;2],
			fRec4: [0.0;2],
			fRec5: [0.0;2],
			fHbargraph1: 0.0,
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
		m.declare("compile_options", r"-a /run/user/1001/.tmpzmRGCu -lang rust -ct 1 -es 1 -mcd 16 -mdd 1024 -mdy 33 -double -ftz 0");
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
		sig0.fillmydspSIG0(589824, unsafe { &mut ftbl0mydspSIG0 });
	}
	fn instance_reset_params(&mut self) {
		self.fHslider0 = 3e+01;
		self.fHslider1 = 2.0;
		self.fHslider2 = -1.0;
		self.fHslider3 = 0.0;
		self.fHslider4 = 1e+02;
		self.fHslider5 = 1e+02;
		self.fHslider6 = 2.0;
		self.fHslider7 = -3.0;
		self.fHslider8 = 42.0;
	}
	fn instance_clear(&mut self) {
		for l1 in 0..2 {
			self.fRec3[l1 as usize] = 0.0;
		}
		self.IOTA0 = 0;
		for l2 in 0..32768 {
			self.fVec0[l2 as usize] = 0.0;
		}
		for l3 in 0..32768 {
			self.fVec1[l3 as usize] = 0.0;
		}
		for l4 in 0..2 {
			self.fVec2[l4 as usize] = 0.0;
		}
		for l5 in 0..3 {
			self.fVec3[l5 as usize] = 0.0;
		}
		for l6 in 0..7 {
			self.fVec4[l6 as usize] = 0.0;
		}
		for l7 in 0..15 {
			self.fVec5[l7 as usize] = 0.0;
		}
		for l8 in 0..32 {
			self.fVec6[l8 as usize] = 0.0;
		}
		for l9 in 0..64 {
			self.fVec7[l9 as usize] = 0.0;
		}
		for l10 in 0..128 {
			self.fVec8[l10 as usize] = 0.0;
		}
		for l11 in 0..256 {
			self.fVec9[l11 as usize] = 0.0;
		}
		for l12 in 0..512 {
			self.fVec10[l12 as usize] = 0.0;
		}
		for l13 in 0..1024 {
			self.fVec11[l13 as usize] = 0.0;
		}
		for l14 in 0..2048 {
			self.fVec12[l14 as usize] = 0.0;
		}
		for l15 in 0..4096 {
			self.fVec13[l15 as usize] = 0.0;
		}
		for l16 in 0..8192 {
			self.fVec14[l16 as usize] = 0.0;
		}
		for l17 in 0..16384 {
			self.fVec15[l17 as usize] = 0.0;
		}
		for l18 in 0..32768 {
			self.fVec16[l18 as usize] = 0.0;
		}
		for l19 in 0..32768 {
			self.fVec17[l19 as usize] = 0.0;
		}
		for l20 in 0..2 {
			self.fVec18[l20 as usize] = 0.0;
		}
		for l21 in 0..2 {
			self.fVec19[l21 as usize] = 0.0;
		}
		for l22 in 0..2 {
			self.fVec20[l22 as usize] = 0.0;
		}
		for l23 in 0..2 {
			self.fRec0[l23 as usize] = 0.0;
		}
		for l24 in 0..2 {
			self.fRec1[l24 as usize] = 0.0;
		}
		for l25 in 0..2 {
			self.fVec21[l25 as usize] = 0.0;
		}
		for l26 in 0..3 {
			self.fVec22[l26 as usize] = 0.0;
		}
		for l27 in 0..7 {
			self.fVec23[l27 as usize] = 0.0;
		}
		for l28 in 0..15 {
			self.fVec24[l28 as usize] = 0.0;
		}
		for l29 in 0..32 {
			self.fVec25[l29 as usize] = 0.0;
		}
		for l30 in 0..64 {
			self.fVec26[l30 as usize] = 0.0;
		}
		for l31 in 0..128 {
			self.fVec27[l31 as usize] = 0.0;
		}
		for l32 in 0..256 {
			self.fVec28[l32 as usize] = 0.0;
		}
		for l33 in 0..512 {
			self.fVec29[l33 as usize] = 0.0;
		}
		for l34 in 0..1024 {
			self.fVec30[l34 as usize] = 0.0;
		}
		for l35 in 0..2048 {
			self.fVec31[l35 as usize] = 0.0;
		}
		for l36 in 0..4096 {
			self.fVec32[l36 as usize] = 0.0;
		}
		for l37 in 0..8192 {
			self.fVec33[l37 as usize] = 0.0;
		}
		for l38 in 0..16384 {
			self.fVec34[l38 as usize] = 0.0;
		}
		for l39 in 0..32768 {
			self.fVec35[l39 as usize] = 0.0;
		}
		for l40 in 0..32768 {
			self.fVec36[l40 as usize] = 0.0;
		}
		for l41 in 0..2 {
			self.fVec37[l41 as usize] = 0.0;
		}
		for l42 in 0..2 {
			self.fVec38[l42 as usize] = 0.0;
		}
		for l43 in 0..2 {
			self.fVec39[l43 as usize] = 0.0;
		}
		for l44 in 0..2 {
			self.fRec4[l44 as usize] = 0.0;
		}
		for l45 in 0..2 {
			self.fRec5[l45 as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		let mut fConst0: F64 = F64::min(1.92e+05, F64::max(1.0, (self.fSampleRate) as F64));
		self.fConst1 = 0.001 * fConst0;
		self.fConst2 = 44.1 / fConst0;
		self.fConst3 = 1.0 - self.fConst2;
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
		ui_interface.open_vertical_box("lamb");
		ui_interface.declare(Some(ParamIndex(0)), "00", "");
		ui_interface.add_horizontal_slider("input gain", ParamIndex(0), 0.0, -24.0, 24.0, 1.0);
		ui_interface.declare(Some(ParamIndex(1)), "02", "");
		ui_interface.add_horizontal_slider("strength", ParamIndex(1), 1e+02, 0.0, 1e+02, 1.0);
		ui_interface.declare(Some(ParamIndex(2)), "03", "");
		ui_interface.add_horizontal_slider("thresh", ParamIndex(2), -1.0, -3e+01, 0.0, 1.0);
		ui_interface.declare(Some(ParamIndex(3)), "04", "");
		ui_interface.declare(Some(ParamIndex(3)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(3)), "unit", "ms");
		ui_interface.add_horizontal_slider("attack", ParamIndex(3), 3e+01, 0.0, 1e+02, 1.0);
		ui_interface.declare(Some(ParamIndex(4)), "05", "");
		ui_interface.add_horizontal_slider("attack shape", ParamIndex(4), 2.0, -4.0, 4.0, 0.1);
		ui_interface.declare(Some(ParamIndex(5)), "06", "");
		ui_interface.declare(Some(ParamIndex(5)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(5)), "unit", "ms");
		ui_interface.add_horizontal_slider("release", ParamIndex(5), 42.0, 1.0, 1e+03, 1.0);
		ui_interface.declare(Some(ParamIndex(6)), "07", "");
		ui_interface.add_horizontal_slider("release shape", ParamIndex(6), -3.0, -4.0, 4.0, 0.1);
		ui_interface.declare(Some(ParamIndex(7)), "08", "");
		ui_interface.add_horizontal_slider("knee", ParamIndex(7), 2.0, 0.0, 3e+01, 1.0);
		ui_interface.declare(Some(ParamIndex(8)), "09", "");
		ui_interface.add_horizontal_slider("link", ParamIndex(8), 1e+02, 0.0, 1e+02, 1.0);
		ui_interface.declare(None, "10", "");
		ui_interface.open_vertical_box("meters");
		ui_interface.declare(Some(ParamIndex(9)), "unit", "dB");
		ui_interface.add_horizontal_bargraph("0", ParamIndex(9), -24.0, 0.0);
		ui_interface.declare(Some(ParamIndex(10)), "unit", "dB");
		ui_interface.add_horizontal_bargraph("1", ParamIndex(10), -24.0, 0.0);
		ui_interface.close_box();
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			9 => Some(self.fHbargraph0),
			10 => Some(self.fHbargraph1),
			3 => Some(self.fHslider0),
			7 => Some(self.fHslider1),
			2 => Some(self.fHslider2),
			0 => Some(self.fHslider3),
			8 => Some(self.fHslider4),
			1 => Some(self.fHslider5),
			4 => Some(self.fHslider6),
			6 => Some(self.fHslider7),
			5 => Some(self.fHslider8),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			9 => { self.fHbargraph0 = value }
			10 => { self.fHbargraph1 = value }
			3 => { self.fHslider0 = value }
			7 => { self.fHslider1 = value }
			2 => { self.fHslider2 = value }
			0 => { self.fHslider3 = value }
			8 => { self.fHslider4 = value }
			1 => { self.fHslider5 = value }
			4 => { self.fHslider6 = value }
			6 => { self.fHslider7 = value }
			5 => { self.fHslider8 = value }
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
		let mut fSlow0: F64 = self.fHslider0;
		let mut fSlow1: F64 = self.fConst1 * fSlow0;
		let mut fSlow2: F64 = fSlow1 + 1.0;
		let mut iSlow3: i32 = (F64::floor(fSlow2)) as i32 % 2;
		let mut fSlow4: F64 = self.fHslider1;
		let mut fSlow5: F64 = 0.5 * fSlow4;
		let mut fSlow6: F64 = self.fHslider2;
		let mut fSlow7: F64 = fSlow6 + fSlow5;
		let mut fSlow8: F64 = self.fConst2 * self.fHslider3;
		let mut fSlow9: F64 = 0.01 * self.fHslider4;
		let mut fSlow10: F64 = fSlow6 - fSlow5;
		let mut fSlow11: F64 = 0.5 / F64::max(2.220446049250313e-16, fSlow4);
		let mut fSlow12: F64 = 0.01 * self.fHslider5;
		let mut iSlow13: i32 = (F64::floor(0.5 * fSlow2)) as i32 % 2;
		let mut iSlow14: i32 = (F64::floor(0.25 * fSlow2)) as i32 % 2;
		let mut iSlow15: i32 = i32::wrapping_add(iSlow3, i32::wrapping_mul(2, iSlow13));
		let mut iSlow16: i32 = (F64::floor(0.125 * fSlow2)) as i32 % 2;
		let mut iSlow17: i32 = i32::wrapping_add(iSlow15, i32::wrapping_mul(4, iSlow14));
		let mut iSlow18: i32 = (F64::floor(0.0625 * fSlow2)) as i32 % 2;
		let mut iSlow19: i32 = i32::wrapping_add(iSlow17, i32::wrapping_mul(8, iSlow16));
		let mut iSlow20: i32 = (F64::floor(0.03125 * fSlow2)) as i32 % 2;
		let mut iSlow21: i32 = i32::wrapping_add(iSlow19, i32::wrapping_mul(16, iSlow18));
		let mut iSlow22: i32 = (F64::floor(0.015625 * fSlow2)) as i32 % 2;
		let mut iSlow23: i32 = i32::wrapping_add(iSlow21, i32::wrapping_mul(32, iSlow20));
		let mut iSlow24: i32 = (F64::floor(0.0078125 * fSlow2)) as i32 % 2;
		let mut iSlow25: i32 = i32::wrapping_add(iSlow23, i32::wrapping_mul(64, iSlow22));
		let mut iSlow26: i32 = (F64::floor(0.00390625 * fSlow2)) as i32 % 2;
		let mut iSlow27: i32 = i32::wrapping_add(iSlow25, i32::wrapping_mul(128, iSlow24));
		let mut iSlow28: i32 = (F64::floor(0.001953125 * fSlow2)) as i32 % 2;
		let mut iSlow29: i32 = i32::wrapping_add(iSlow27, i32::wrapping_mul(256, iSlow26));
		let mut iSlow30: i32 = (F64::floor(0.0009765625 * fSlow2)) as i32 % 2;
		let mut iSlow31: i32 = i32::wrapping_add(iSlow29, i32::wrapping_mul(512, iSlow28));
		let mut iSlow32: i32 = (F64::floor(0.00048828125 * fSlow2)) as i32 % 2;
		let mut iSlow33: i32 = i32::wrapping_add(iSlow31, i32::wrapping_mul(1024, iSlow30));
		let mut iSlow34: i32 = (F64::floor(0.000244140625 * fSlow2)) as i32 % 2;
		let mut iSlow35: i32 = i32::wrapping_add(iSlow33, i32::wrapping_mul(2048, iSlow32));
		let mut iSlow36: i32 = (F64::floor(0.0001220703125 * fSlow2)) as i32 % 2;
		let mut iSlow37: i32 = i32::wrapping_add(iSlow35, i32::wrapping_mul(4096, iSlow34));
		let mut iSlow38: i32 = (F64::floor(6.103515625e-05 * fSlow2)) as i32 % 2;
		let mut iSlow39: i32 = i32::wrapping_add(iSlow37, i32::wrapping_mul(8192, iSlow36));
		let mut fSlow40: F64 = self.fHslider6 + 4.0;
		let mut fSlow41: F64 = self.fHslider7 + 4.0;
		let mut fSlow42: F64 = self.fHslider8;
		let mut iSlow43: i32 = (fSlow1) as i32;
		let zipped_iterators = inputs0.zip(inputs1).zip(outputs0).zip(outputs1);
		for (((input0, input1), output0), output1) in zipped_iterators {
			self.fRec3[0] = fSlow8 + self.fConst3 * self.fRec3[1];
			let mut fTemp9: F64 = F64::powf(1e+01, 0.05 * self.fRec3[0]);
			let mut fTemp10: F64 = *input0 * fTemp9;
			self.fVec0[(self.IOTA0 & 32767) as usize] = fTemp10;
			let mut fTemp11: F64 = F64::abs(fTemp10);
			let mut fTemp12: F64 = *input1 * fTemp9;
			self.fVec1[(self.IOTA0 & 32767) as usize] = fTemp12;
			let mut fTemp13: F64 = F64::abs(fTemp12);
			let mut fTemp14: F64 = F64::max(fTemp11, fTemp13);
			let mut fTemp15: F64 = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, fTemp11 + fSlow9 * (fTemp14 - fTemp11)));
			let mut iTemp16: i32 = ((fTemp15 > fSlow10) as i32) + ((fTemp15 > fSlow7) as i32);
			let mut fTemp17: F64 = fTemp15 - fSlow6;
			let mut fTemp18: F64 = fSlow12 * F64::max(0.0, if (iTemp16 == 0) as i32 != 0 {0.0} else {if (iTemp16 == 1) as i32 != 0 {fSlow11 * mydsp_faustpower2_f(fSlow5 + fTemp17)} else {fTemp17}});
			self.fVec2[0] = fTemp18;
			let mut fTemp19: F64 = F64::min(-fTemp18, -self.fVec2[1]);
			self.fVec3[0] = fTemp19;
			let mut fTemp20: F64 = F64::min(fTemp19, self.fVec3[2]);
			self.fVec4[0] = fTemp20;
			let mut fTemp21: F64 = F64::min(fTemp20, self.fVec4[4]);
			self.fVec5[0] = fTemp21;
			let mut fTemp22: F64 = F64::min(fTemp21, self.fVec5[8]);
			self.fVec6[(self.IOTA0 & 31) as usize] = fTemp22;
			let mut fTemp23: F64 = F64::min(fTemp22, self.fVec6[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize]);
			self.fVec7[(self.IOTA0 & 63) as usize] = fTemp23;
			let mut fTemp24: F64 = F64::min(fTemp23, self.fVec7[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize]);
			self.fVec8[(self.IOTA0 & 127) as usize] = fTemp24;
			let mut fTemp25: F64 = F64::min(fTemp24, self.fVec8[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize]);
			self.fVec9[(self.IOTA0 & 255) as usize] = fTemp25;
			let mut fTemp26: F64 = F64::min(fTemp25, self.fVec9[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize]);
			self.fVec10[(self.IOTA0 & 511) as usize] = fTemp26;
			let mut fTemp27: F64 = F64::min(fTemp26, self.fVec10[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize]);
			self.fVec11[(self.IOTA0 & 1023) as usize] = fTemp27;
			let mut fTemp28: F64 = F64::min(fTemp27, self.fVec11[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize]);
			self.fVec12[(self.IOTA0 & 2047) as usize] = fTemp28;
			let mut fTemp29: F64 = F64::min(fTemp28, self.fVec12[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			self.fVec13[(self.IOTA0 & 4095) as usize] = fTemp29;
			let mut fTemp30: F64 = F64::min(fTemp29, self.fVec13[((i32::wrapping_sub(self.IOTA0, 2048)) & 4095) as usize]);
			self.fVec14[(self.IOTA0 & 8191) as usize] = fTemp30;
			let mut fTemp31: F64 = F64::min(fTemp30, self.fVec14[((i32::wrapping_sub(self.IOTA0, 4096)) & 8191) as usize]);
			self.fVec15[(self.IOTA0 & 16383) as usize] = fTemp31;
			self.fVec16[(self.IOTA0 & 32767) as usize] = F64::min(fTemp31, self.fVec15[((i32::wrapping_sub(self.IOTA0, 8192)) & 16383) as usize]);
			let mut fTemp32: F64 = F64::powf(1e+01, 0.05 * F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow3 != 0 {-fTemp18} else {1.7976931348623157e+308}, if iSlow13 != 0 {self.fVec3[iSlow3 as usize]} else {1.7976931348623157e+308}), if iSlow14 != 0 {self.fVec4[iSlow15 as usize]} else {1.7976931348623157e+308}), if iSlow16 != 0 {self.fVec5[iSlow17 as usize]} else {1.7976931348623157e+308}), if iSlow18 != 0 {self.fVec6[((i32::wrapping_sub(self.IOTA0, iSlow19)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow20 != 0 {self.fVec7[((i32::wrapping_sub(self.IOTA0, iSlow21)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow22 != 0 {self.fVec8[((i32::wrapping_sub(self.IOTA0, iSlow23)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow24 != 0 {self.fVec9[((i32::wrapping_sub(self.IOTA0, iSlow25)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow26 != 0 {self.fVec10[((i32::wrapping_sub(self.IOTA0, iSlow27)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow28 != 0 {self.fVec11[((i32::wrapping_sub(self.IOTA0, iSlow29)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow30 != 0 {self.fVec12[((i32::wrapping_sub(self.IOTA0, iSlow31)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow32 != 0 {self.fVec13[((i32::wrapping_sub(self.IOTA0, iSlow33)) & 4095) as usize]} else {1.7976931348623157e+308}), if iSlow34 != 0 {self.fVec14[((i32::wrapping_sub(self.IOTA0, iSlow35)) & 8191) as usize]} else {1.7976931348623157e+308}), if iSlow36 != 0 {self.fVec15[((i32::wrapping_sub(self.IOTA0, iSlow37)) & 16383) as usize]} else {1.7976931348623157e+308}), if iSlow38 != 0 {self.fVec16[((i32::wrapping_sub(self.IOTA0, iSlow39)) & 32767) as usize]} else {1.7976931348623157e+308}));
			self.fVec17[(self.IOTA0 & 32767) as usize] = fTemp32;
			let mut fTemp33: F64 = fTemp32 - self.fRec1[1];
			self.fVec18[0] = fTemp33;
			let mut iTemp34: i32 = (fTemp33 > 0.0) as i32;
			let mut fTemp35: F64 = if iTemp34 != 0 {fSlow41} else {fSlow40};
			self.fVec19[0] = fTemp35;
			let mut fTemp36: F64 = 0.8888888888888888 * fTemp35;
			let mut iTemp37: i32 = (fTemp36) as i32;
			let mut fTemp38: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp37, 294912)) as usize] };
			let mut fTemp39: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp37, 294903)) as usize] };
			let mut fTemp40: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp37, 294904)) as usize] } - fTemp39;
			let mut fTemp41: F64 = fTemp36 - (iTemp37) as F64;
			let mut fTemp42: F64 = 0.5 * (fTemp38 - (fTemp39 + fTemp41 * (fTemp40 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp37, 294913)) as usize] } - fTemp38))));
			let mut fTemp43: F64 = fSlow0 * ((fTemp33 < 0.0) as i32) as u64 as F64 + fSlow42 * (iTemp34) as F64;
			self.fVec20[0] = fTemp43;
			let mut fTemp44: F64 = self.fConst4 / fTemp43;
			let mut fTemp45: F64 = 65535.0 * (fTemp44 + 0.5);
			let mut iTemp46: i32 = (fTemp45) as i32;
			let mut fTemp47: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp37, i32::wrapping_mul(9, i32::wrapping_add(iTemp46, 1))), 589823))) as usize] };
			let mut iTemp48: i32 = i32::wrapping_add(iTemp37, i32::wrapping_mul(9, iTemp46));
			let mut fTemp49: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp48, 589823))) as usize] };
			let mut fTemp50: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, 1), 589823))) as usize] } - fTemp49;
			let mut fTemp51: F64 = 65535.0 * self.fRec0[1];
			let mut iTemp52: i32 = (fTemp51) as i32;
			let mut fTemp53: F64 = 0.8888888888888888 * self.fVec19[1];
			let mut iTemp54: i32 = (fTemp53) as i32;
			let mut fTemp55: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp54, i32::wrapping_mul(9, i32::wrapping_add(iTemp52, 1))), 589823))) as usize] };
			let mut iTemp56: i32 = i32::wrapping_add(i32::wrapping_mul(9, iTemp52), iTemp54);
			let mut fTemp57: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp56, 589823))) as usize] };
			let mut fTemp58: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp56, 1), 589823))) as usize] } - fTemp57;
			let mut fTemp59: F64 = fTemp53 - (iTemp54) as F64;
			let mut fTemp60: F64 = 65535.0 * (self.fRec0[1] + fTemp44);
			let mut iTemp61: i32 = (fTemp60) as i32;
			let mut fTemp62: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp37, i32::wrapping_mul(9, i32::wrapping_add(iTemp61, 1))), 589823))) as usize] };
			let mut iTemp63: i32 = i32::wrapping_add(iTemp37, i32::wrapping_mul(9, iTemp61));
			let mut fTemp64: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp63, 589823))) as usize] };
			let mut fTemp65: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, 1), 589823))) as usize] } - fTemp64;
			let mut fTemp66: F64 = 65535.0 * (self.fRec0[1] + self.fConst4 * (1.0 / fTemp43 + 1.0 / self.fVec20[1]));
			let mut iTemp67: i32 = (fTemp66) as i32;
			let mut fTemp68: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp37, i32::wrapping_mul(9, i32::wrapping_add(iTemp67, 1))), 589823))) as usize] };
			let mut iTemp69: i32 = i32::wrapping_add(i32::wrapping_mul(9, iTemp67), iTemp37);
			let mut fTemp70: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp69, 589823))) as usize] };
			let mut fTemp71: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp69, 1), 589823))) as usize] } - fTemp70;
			let mut fTemp72: F64 = (fTemp41 * (fTemp71 - fTemp65) + fTemp70 + (fTemp66 - (iTemp67) as F64) * (fTemp68 - (fTemp70 + fTemp41 * (fTemp71 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp69, 10), 589823))) as usize] } - fTemp68)))) - (fTemp64 + (fTemp60 - (iTemp61) as F64) * (fTemp62 - (fTemp64 + fTemp41 * (fTemp65 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp63, 10), 589823))) as usize] } - fTemp62)))))) * self.fVec18[1] / (fTemp33 * (1.0 - (fTemp57 + fTemp59 * fTemp58 + (fTemp51 - (iTemp52) as F64) * (fTemp55 - (fTemp57 + fTemp59 * (fTemp58 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp56, 10), 589823))) as usize] } - fTemp55)))))));
			let mut iTemp73: i32 = (fTemp72 > ((fTemp41 * (fTemp50 - fTemp40) + fTemp49 + (fTemp45 - (iTemp46) as F64) * (fTemp47 - (fTemp49 + fTemp41 * (fTemp50 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp48, 10), 589823))) as usize] } - fTemp47)))) - (fTemp39 + fTemp42)) / (1.0 - (fTemp39 + fTemp41 * fTemp40 + fTemp42)))) as i32;
			let mut fTemp74: F64 = if iTemp73 != 0 {1.0} else {0.5};
			let mut fTemp75: F64 = if iTemp73 != 0 {0.5} else {0.0};
			let mut fTemp76: F64 = fTemp75 + fTemp74;
			let mut fTemp77: F64 = 32767.5 * fTemp76;
			let mut iTemp78: i32 = (fTemp77) as i32;
			let mut fTemp79: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp37, i32::wrapping_mul(9, i32::wrapping_add(iTemp78, 1)))) as usize] };
			let mut iTemp80: i32 = i32::wrapping_add(iTemp37, i32::wrapping_mul(9, iTemp78));
			let mut fTemp81: F64 = unsafe { ftbl0mydspSIG0[iTemp80 as usize] };
			let mut fTemp82: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp80, 1)) as usize] } - fTemp81;
			let mut fTemp83: F64 = (fTemp77 - (iTemp78) as F64) * (fTemp79 - (fTemp81 + fTemp41 * (fTemp82 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp80, 10)) as usize] } - fTemp79))));
			let mut fTemp84: F64 = 0.5 * fTemp76;
			let mut fTemp85: F64 = 65535.0 * (fTemp44 + fTemp84);
			let mut iTemp86: i32 = (fTemp85) as i32;
			let mut fTemp87: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp37, i32::wrapping_mul(9, i32::wrapping_add(iTemp86, 1))), 589823))) as usize] };
			let mut iTemp88: i32 = i32::wrapping_add(iTemp37, i32::wrapping_mul(9, iTemp86));
			let mut fTemp89: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp88, 589823))) as usize] };
			let mut fTemp90: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp88, 1), 589823))) as usize] } - fTemp89;
			let mut iTemp91: i32 = (fTemp72 > ((fTemp41 * (fTemp90 - fTemp82) + fTemp89 + (fTemp85 - (iTemp86) as F64) * (fTemp87 - (fTemp89 + fTemp41 * (fTemp90 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp88, 10), 589823))) as usize] } - fTemp87)))) - (fTemp81 + fTemp83)) / (1.0 - (fTemp81 + fTemp41 * fTemp82 + fTemp83)))) as i32;
			let mut fTemp92: F64 = if iTemp91 != 0 {fTemp74} else {fTemp84};
			let mut fTemp93: F64 = if iTemp91 != 0 {fTemp84} else {fTemp75};
			let mut fTemp94: F64 = fTemp93 + fTemp92;
			let mut fTemp95: F64 = 32767.5 * fTemp94;
			let mut iTemp96: i32 = (fTemp95) as i32;
			let mut fTemp97: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp37, i32::wrapping_mul(9, i32::wrapping_add(iTemp96, 1)))) as usize] };
			let mut iTemp98: i32 = i32::wrapping_add(iTemp37, i32::wrapping_mul(9, iTemp96));
			let mut fTemp99: F64 = unsafe { ftbl0mydspSIG0[iTemp98 as usize] };
			let mut fTemp100: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp98, 1)) as usize] } - fTemp99;
			let mut fTemp101: F64 = (fTemp95 - (iTemp96) as F64) * (fTemp97 - (fTemp99 + fTemp41 * (fTemp100 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp98, 10)) as usize] } - fTemp97))));
			let mut fTemp102: F64 = 0.5 * fTemp94;
			let mut fTemp103: F64 = 65535.0 * (fTemp44 + fTemp102);
			let mut iTemp104: i32 = (fTemp103) as i32;
			let mut fTemp105: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp37, i32::wrapping_mul(9, i32::wrapping_add(iTemp104, 1))), 589823))) as usize] };
			let mut iTemp106: i32 = i32::wrapping_add(iTemp37, i32::wrapping_mul(9, iTemp104));
			let mut fTemp107: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp106, 589823))) as usize] };
			let mut fTemp108: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp106, 1), 589823))) as usize] } - fTemp107;
			let mut iTemp109: i32 = (fTemp72 > ((fTemp41 * (fTemp108 - fTemp100) + fTemp107 + (fTemp103 - (iTemp104) as F64) * (fTemp105 - (fTemp107 + fTemp41 * (fTemp108 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp106, 10), 589823))) as usize] } - fTemp105)))) - (fTemp99 + fTemp101)) / (1.0 - (fTemp99 + fTemp41 * fTemp100 + fTemp101)))) as i32;
			let mut fTemp110: F64 = if iTemp109 != 0 {fTemp92} else {fTemp102};
			let mut fTemp111: F64 = if iTemp109 != 0 {fTemp102} else {fTemp93};
			let mut fTemp112: F64 = fTemp111 + fTemp110;
			let mut fTemp113: F64 = 32767.5 * fTemp112;
			let mut iTemp114: i32 = (fTemp113) as i32;
			let mut fTemp115: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp37, i32::wrapping_mul(9, i32::wrapping_add(iTemp114, 1)))) as usize] };
			let mut iTemp116: i32 = i32::wrapping_add(iTemp37, i32::wrapping_mul(9, iTemp114));
			let mut fTemp117: F64 = unsafe { ftbl0mydspSIG0[iTemp116 as usize] };
			let mut fTemp118: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp116, 1)) as usize] } - fTemp117;
			let mut fTemp119: F64 = (fTemp113 - (iTemp114) as F64) * (fTemp115 - (fTemp117 + fTemp41 * (fTemp118 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp116, 10)) as usize] } - fTemp115))));
			let mut fTemp120: F64 = 0.5 * fTemp112;
			let mut fTemp121: F64 = 65535.0 * (fTemp44 + fTemp120);
			let mut iTemp122: i32 = (fTemp121) as i32;
			let mut fTemp123: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp37, i32::wrapping_mul(9, i32::wrapping_add(iTemp122, 1))), 589823))) as usize] };
			let mut iTemp124: i32 = i32::wrapping_add(iTemp37, i32::wrapping_mul(9, iTemp122));
			let mut fTemp125: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp124, 589823))) as usize] };
			let mut fTemp126: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp124, 1), 589823))) as usize] } - fTemp125;
			let mut iTemp127: i32 = (fTemp72 > ((fTemp41 * (fTemp126 - fTemp118) + fTemp125 + (fTemp121 - (iTemp122) as F64) * (fTemp123 - (fTemp125 + fTemp41 * (fTemp126 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp124, 10), 589823))) as usize] } - fTemp123)))) - (fTemp117 + fTemp119)) / (1.0 - (fTemp117 + fTemp41 * fTemp118 + fTemp119)))) as i32;
			let mut fTemp128: F64 = if iTemp127 != 0 {fTemp110} else {fTemp120};
			let mut fTemp129: F64 = if iTemp127 != 0 {fTemp120} else {fTemp111};
			let mut fTemp130: F64 = fTemp129 + fTemp128;
			let mut fTemp131: F64 = 32767.5 * fTemp130;
			let mut iTemp132: i32 = (fTemp131) as i32;
			let mut fTemp133: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp37, i32::wrapping_mul(9, i32::wrapping_add(iTemp132, 1)))) as usize] };
			let mut iTemp134: i32 = i32::wrapping_add(iTemp37, i32::wrapping_mul(9, iTemp132));
			let mut fTemp135: F64 = unsafe { ftbl0mydspSIG0[iTemp134 as usize] };
			let mut fTemp136: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp134, 1)) as usize] } - fTemp135;
			let mut fTemp137: F64 = (fTemp131 - (iTemp132) as F64) * (fTemp133 - (fTemp135 + fTemp41 * (fTemp136 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp134, 10)) as usize] } - fTemp133))));
			let mut fTemp138: F64 = 0.5 * fTemp130;
			let mut fTemp139: F64 = 65535.0 * (fTemp44 + fTemp138);
			let mut iTemp140: i32 = (fTemp139) as i32;
			let mut fTemp141: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp37, i32::wrapping_mul(9, i32::wrapping_add(iTemp140, 1))), 589823))) as usize] };
			let mut iTemp142: i32 = i32::wrapping_add(iTemp37, i32::wrapping_mul(9, iTemp140));
			let mut fTemp143: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp142, 589823))) as usize] };
			let mut fTemp144: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp142, 1), 589823))) as usize] } - fTemp143;
			let mut iTemp145: i32 = (fTemp72 > ((fTemp41 * (fTemp144 - fTemp136) + fTemp143 + (fTemp139 - (iTemp140) as F64) * (fTemp141 - (fTemp143 + fTemp41 * (fTemp144 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp142, 10), 589823))) as usize] } - fTemp141)))) - (fTemp135 + fTemp137)) / (1.0 - (fTemp135 + fTemp41 * fTemp136 + fTemp137)))) as i32;
			let mut fTemp146: F64 = if iTemp145 != 0 {fTemp128} else {fTemp138};
			let mut fTemp147: F64 = if iTemp145 != 0 {fTemp138} else {fTemp129};
			let mut fTemp148: F64 = fTemp147 + fTemp146;
			let mut fTemp149: F64 = 32767.5 * fTemp148;
			let mut iTemp150: i32 = (fTemp149) as i32;
			let mut fTemp151: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp37, i32::wrapping_mul(9, i32::wrapping_add(iTemp150, 1)))) as usize] };
			let mut iTemp152: i32 = i32::wrapping_add(iTemp37, i32::wrapping_mul(9, iTemp150));
			let mut fTemp153: F64 = unsafe { ftbl0mydspSIG0[iTemp152 as usize] };
			let mut fTemp154: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp152, 1)) as usize] } - fTemp153;
			let mut fTemp155: F64 = (fTemp149 - (iTemp150) as F64) * (fTemp151 - (fTemp153 + fTemp41 * (fTemp154 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp152, 10)) as usize] } - fTemp151))));
			let mut fTemp156: F64 = 0.5 * fTemp148;
			let mut fTemp157: F64 = 65535.0 * (fTemp44 + fTemp156);
			let mut iTemp158: i32 = (fTemp157) as i32;
			let mut fTemp159: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp37, i32::wrapping_mul(9, i32::wrapping_add(iTemp158, 1))), 589823))) as usize] };
			let mut iTemp160: i32 = i32::wrapping_add(iTemp37, i32::wrapping_mul(9, iTemp158));
			let mut fTemp161: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp160, 589823))) as usize] };
			let mut fTemp162: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp160, 1), 589823))) as usize] } - fTemp161;
			let mut iTemp163: i32 = (fTemp72 > ((fTemp41 * (fTemp162 - fTemp154) + fTemp161 + (fTemp157 - (iTemp158) as F64) * (fTemp159 - (fTemp161 + fTemp41 * (fTemp162 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp160, 10), 589823))) as usize] } - fTemp159)))) - (fTemp153 + fTemp155)) / (1.0 - (fTemp153 + fTemp41 * fTemp154 + fTemp155)))) as i32;
			let mut fTemp164: F64 = if iTemp163 != 0 {fTemp146} else {fTemp156};
			let mut fTemp165: F64 = if iTemp163 != 0 {fTemp156} else {fTemp147};
			let mut fTemp166: F64 = fTemp165 + fTemp164;
			let mut fTemp167: F64 = 32767.5 * fTemp166;
			let mut iTemp168: i32 = (fTemp167) as i32;
			let mut fTemp169: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp37, i32::wrapping_mul(9, i32::wrapping_add(iTemp168, 1)))) as usize] };
			let mut iTemp170: i32 = i32::wrapping_add(iTemp37, i32::wrapping_mul(9, iTemp168));
			let mut fTemp171: F64 = unsafe { ftbl0mydspSIG0[iTemp170 as usize] };
			let mut fTemp172: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp170, 1)) as usize] } - fTemp171;
			let mut fTemp173: F64 = (fTemp167 - (iTemp168) as F64) * (fTemp169 - (fTemp171 + fTemp41 * (fTemp172 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp170, 10)) as usize] } - fTemp169))));
			let mut fTemp174: F64 = 0.5 * fTemp166;
			let mut fTemp175: F64 = 65535.0 * (fTemp44 + fTemp174);
			let mut iTemp176: i32 = (fTemp175) as i32;
			let mut fTemp177: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp37, i32::wrapping_mul(9, i32::wrapping_add(iTemp176, 1))), 589823))) as usize] };
			let mut iTemp178: i32 = i32::wrapping_add(iTemp37, i32::wrapping_mul(9, iTemp176));
			let mut fTemp179: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp178, 589823))) as usize] };
			let mut fTemp180: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp178, 1), 589823))) as usize] } - fTemp179;
			let mut iTemp181: i32 = (fTemp72 > ((fTemp41 * (fTemp180 - fTemp172) + fTemp179 + (fTemp175 - (iTemp176) as F64) * (fTemp177 - (fTemp179 + fTemp41 * (fTemp180 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp178, 10), 589823))) as usize] } - fTemp177)))) - (fTemp171 + fTemp173)) / (1.0 - (fTemp171 + fTemp41 * fTemp172 + fTemp173)))) as i32;
			let mut fTemp182: F64 = if iTemp181 != 0 {fTemp164} else {fTemp174};
			let mut fTemp183: F64 = if iTemp181 != 0 {fTemp174} else {fTemp165};
			let mut fTemp184: F64 = fTemp183 + fTemp182;
			let mut fTemp185: F64 = 32767.5 * fTemp184;
			let mut iTemp186: i32 = (fTemp185) as i32;
			let mut fTemp187: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp37, i32::wrapping_mul(9, i32::wrapping_add(iTemp186, 1)))) as usize] };
			let mut iTemp188: i32 = i32::wrapping_add(iTemp37, i32::wrapping_mul(9, iTemp186));
			let mut fTemp189: F64 = unsafe { ftbl0mydspSIG0[iTemp188 as usize] };
			let mut fTemp190: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp188, 1)) as usize] } - fTemp189;
			let mut fTemp191: F64 = (fTemp185 - (iTemp186) as F64) * (fTemp187 - (fTemp189 + fTemp41 * (fTemp190 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp188, 10)) as usize] } - fTemp187))));
			let mut fTemp192: F64 = 0.5 * fTemp184;
			let mut fTemp193: F64 = 65535.0 * (fTemp44 + fTemp192);
			let mut iTemp194: i32 = (fTemp193) as i32;
			let mut fTemp195: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp37, i32::wrapping_mul(9, i32::wrapping_add(iTemp194, 1))), 589823))) as usize] };
			let mut iTemp196: i32 = i32::wrapping_add(iTemp37, i32::wrapping_mul(9, iTemp194));
			let mut fTemp197: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp196, 589823))) as usize] };
			let mut fTemp198: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp196, 1), 589823))) as usize] } - fTemp197;
			let mut iTemp199: i32 = (fTemp72 > ((fTemp41 * (fTemp198 - fTemp190) + fTemp197 + (fTemp193 - (iTemp194) as F64) * (fTemp195 - (fTemp197 + fTemp41 * (fTemp198 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp196, 10), 589823))) as usize] } - fTemp195)))) - (fTemp189 + fTemp191)) / (1.0 - (fTemp189 + fTemp41 * fTemp190 + fTemp191)))) as i32;
			let mut fTemp200: F64 = if iTemp199 != 0 {fTemp182} else {fTemp192};
			let mut fTemp201: F64 = if iTemp199 != 0 {fTemp192} else {fTemp183};
			let mut fTemp202: F64 = fTemp201 + fTemp200;
			let mut fTemp203: F64 = 32767.5 * fTemp202;
			let mut iTemp204: i32 = (fTemp203) as i32;
			let mut fTemp205: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp37, i32::wrapping_mul(9, i32::wrapping_add(iTemp204, 1)))) as usize] };
			let mut iTemp206: i32 = i32::wrapping_add(iTemp37, i32::wrapping_mul(9, iTemp204));
			let mut fTemp207: F64 = unsafe { ftbl0mydspSIG0[iTemp206 as usize] };
			let mut fTemp208: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp206, 1)) as usize] } - fTemp207;
			let mut fTemp209: F64 = (fTemp203 - (iTemp204) as F64) * (fTemp205 - (fTemp207 + fTemp41 * (fTemp208 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp206, 10)) as usize] } - fTemp205))));
			let mut fTemp210: F64 = 0.5 * fTemp202;
			let mut fTemp211: F64 = 65535.0 * (fTemp44 + fTemp210);
			let mut iTemp212: i32 = (fTemp211) as i32;
			let mut fTemp213: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp37, i32::wrapping_mul(9, i32::wrapping_add(iTemp212, 1))), 589823))) as usize] };
			let mut iTemp214: i32 = i32::wrapping_add(iTemp37, i32::wrapping_mul(9, iTemp212));
			let mut fTemp215: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp214, 589823))) as usize] };
			let mut fTemp216: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp214, 1), 589823))) as usize] } - fTemp215;
			let mut iTemp217: i32 = (fTemp72 > ((fTemp41 * (fTemp216 - fTemp208) + fTemp215 + (fTemp211 - (iTemp212) as F64) * (fTemp213 - (fTemp215 + fTemp41 * (fTemp216 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp214, 10), 589823))) as usize] } - fTemp213)))) - (fTemp207 + fTemp209)) / (1.0 - (fTemp207 + fTemp41 * fTemp208 + fTemp209)))) as i32;
			let mut fTemp218: F64 = if iTemp217 != 0 {fTemp200} else {fTemp210};
			let mut fTemp219: F64 = if iTemp217 != 0 {fTemp210} else {fTemp201};
			let mut fTemp220: F64 = fTemp219 + fTemp218;
			let mut fTemp221: F64 = 32767.5 * fTemp220;
			let mut iTemp222: i32 = (fTemp221) as i32;
			let mut fTemp223: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp37, i32::wrapping_mul(9, i32::wrapping_add(iTemp222, 1)))) as usize] };
			let mut iTemp224: i32 = i32::wrapping_add(iTemp37, i32::wrapping_mul(9, iTemp222));
			let mut fTemp225: F64 = unsafe { ftbl0mydspSIG0[iTemp224 as usize] };
			let mut fTemp226: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp224, 1)) as usize] } - fTemp225;
			let mut fTemp227: F64 = (fTemp221 - (iTemp222) as F64) * (fTemp223 - (fTemp225 + fTemp41 * (fTemp226 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp224, 10)) as usize] } - fTemp223))));
			let mut fTemp228: F64 = 0.5 * fTemp220;
			let mut fTemp229: F64 = 65535.0 * (fTemp44 + fTemp228);
			let mut iTemp230: i32 = (fTemp229) as i32;
			let mut fTemp231: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp37, i32::wrapping_mul(9, i32::wrapping_add(iTemp230, 1))), 589823))) as usize] };
			let mut iTemp232: i32 = i32::wrapping_add(iTemp37, i32::wrapping_mul(9, iTemp230));
			let mut fTemp233: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp232, 589823))) as usize] };
			let mut fTemp234: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp232, 1), 589823))) as usize] } - fTemp233;
			let mut iTemp235: i32 = (fTemp72 > ((fTemp41 * (fTemp234 - fTemp226) + fTemp233 + (fTemp229 - (iTemp230) as F64) * (fTemp231 - (fTemp233 + fTemp41 * (fTemp234 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp232, 10), 589823))) as usize] } - fTemp231)))) - (fTemp225 + fTemp227)) / (1.0 - (fTemp225 + fTemp41 * fTemp226 + fTemp227)))) as i32;
			let mut fTemp236: F64 = if iTemp235 != 0 {fTemp218} else {fTemp228};
			let mut fTemp237: F64 = if iTemp235 != 0 {fTemp228} else {fTemp219};
			let mut fTemp238: F64 = fTemp237 + fTemp236;
			let mut fTemp239: F64 = 32767.5 * fTemp238;
			let mut iTemp240: i32 = (fTemp239) as i32;
			let mut fTemp241: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp37, i32::wrapping_mul(9, i32::wrapping_add(iTemp240, 1)))) as usize] };
			let mut iTemp242: i32 = i32::wrapping_add(iTemp37, i32::wrapping_mul(9, iTemp240));
			let mut fTemp243: F64 = unsafe { ftbl0mydspSIG0[iTemp242 as usize] };
			let mut fTemp244: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp242, 1)) as usize] } - fTemp243;
			let mut fTemp245: F64 = (fTemp239 - (iTemp240) as F64) * (fTemp241 - (fTemp243 + fTemp41 * (fTemp244 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp242, 10)) as usize] } - fTemp241))));
			let mut fTemp246: F64 = 0.5 * fTemp238;
			let mut fTemp247: F64 = 65535.0 * (fTemp44 + fTemp246);
			let mut iTemp248: i32 = (fTemp247) as i32;
			let mut fTemp249: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp37, i32::wrapping_mul(9, i32::wrapping_add(iTemp248, 1))), 589823))) as usize] };
			let mut iTemp250: i32 = i32::wrapping_add(iTemp37, i32::wrapping_mul(9, iTemp248));
			let mut fTemp251: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp250, 589823))) as usize] };
			let mut fTemp252: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp250, 1), 589823))) as usize] } - fTemp251;
			let mut iTemp253: i32 = (fTemp72 > ((fTemp41 * (fTemp252 - fTemp244) + fTemp251 + (fTemp247 - (iTemp248) as F64) * (fTemp249 - (fTemp251 + fTemp41 * (fTemp252 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp250, 10), 589823))) as usize] } - fTemp249)))) - (fTemp243 + fTemp245)) / (1.0 - (fTemp243 + fTemp41 * fTemp244 + fTemp245)))) as i32;
			let mut fTemp254: F64 = if iTemp253 != 0 {fTemp236} else {fTemp246};
			let mut fTemp255: F64 = if iTemp253 != 0 {fTemp246} else {fTemp237};
			let mut fTemp256: F64 = fTemp255 + fTemp254;
			let mut fTemp257: F64 = 32767.5 * fTemp256;
			let mut iTemp258: i32 = (fTemp257) as i32;
			let mut fTemp259: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp37, i32::wrapping_mul(9, i32::wrapping_add(iTemp258, 1)))) as usize] };
			let mut iTemp260: i32 = i32::wrapping_add(iTemp37, i32::wrapping_mul(9, iTemp258));
			let mut fTemp261: F64 = unsafe { ftbl0mydspSIG0[iTemp260 as usize] };
			let mut fTemp262: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp260, 1)) as usize] } - fTemp261;
			let mut fTemp263: F64 = (fTemp257 - (iTemp258) as F64) * (fTemp259 - (fTemp261 + fTemp41 * (fTemp262 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp260, 10)) as usize] } - fTemp259))));
			let mut fTemp264: F64 = 0.5 * fTemp256;
			let mut fTemp265: F64 = 65535.0 * (fTemp44 + fTemp264);
			let mut iTemp266: i32 = (fTemp265) as i32;
			let mut fTemp267: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp37, i32::wrapping_mul(9, i32::wrapping_add(iTemp266, 1))), 589823))) as usize] };
			let mut iTemp268: i32 = i32::wrapping_add(iTemp37, i32::wrapping_mul(9, iTemp266));
			let mut fTemp269: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp268, 589823))) as usize] };
			let mut fTemp270: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp268, 1), 589823))) as usize] } - fTemp269;
			let mut iTemp271: i32 = (fTemp72 > ((fTemp41 * (fTemp270 - fTemp262) + fTemp269 + (fTemp265 - (iTemp266) as F64) * (fTemp267 - (fTemp269 + fTemp41 * (fTemp270 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp268, 10), 589823))) as usize] } - fTemp267)))) - (fTemp261 + fTemp263)) / (1.0 - (fTemp261 + fTemp41 * fTemp262 + fTemp263)))) as i32;
			let mut fTemp272: F64 = if iTemp271 != 0 {fTemp254} else {fTemp264};
			let mut fTemp273: F64 = if iTemp271 != 0 {fTemp264} else {fTemp255};
			let mut fTemp274: F64 = fTemp273 + fTemp272;
			let mut fTemp275: F64 = 32767.5 * fTemp274;
			let mut iTemp276: i32 = (fTemp275) as i32;
			let mut fTemp277: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp37, i32::wrapping_mul(9, i32::wrapping_add(iTemp276, 1)))) as usize] };
			let mut iTemp278: i32 = i32::wrapping_add(iTemp37, i32::wrapping_mul(9, iTemp276));
			let mut fTemp279: F64 = unsafe { ftbl0mydspSIG0[iTemp278 as usize] };
			let mut fTemp280: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp278, 1)) as usize] } - fTemp279;
			let mut fTemp281: F64 = (fTemp275 - (iTemp276) as F64) * (fTemp277 - (fTemp279 + fTemp41 * (fTemp280 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp278, 10)) as usize] } - fTemp277))));
			let mut fTemp282: F64 = 0.5 * fTemp274;
			let mut fTemp283: F64 = 65535.0 * (fTemp44 + fTemp282);
			let mut iTemp284: i32 = (fTemp283) as i32;
			let mut fTemp285: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp37, i32::wrapping_mul(9, i32::wrapping_add(iTemp284, 1))), 589823))) as usize] };
			let mut iTemp286: i32 = i32::wrapping_add(iTemp37, i32::wrapping_mul(9, iTemp284));
			let mut fTemp287: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp286, 589823))) as usize] };
			let mut fTemp288: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp286, 1), 589823))) as usize] } - fTemp287;
			let mut iTemp289: i32 = (fTemp72 > ((fTemp41 * (fTemp288 - fTemp280) + fTemp287 + (fTemp283 - (iTemp284) as F64) * (fTemp285 - (fTemp287 + fTemp41 * (fTemp288 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp286, 10), 589823))) as usize] } - fTemp285)))) - (fTemp279 + fTemp281)) / (1.0 - (fTemp279 + fTemp41 * fTemp280 + fTemp281)))) as i32;
			let mut fTemp290: F64 = if iTemp289 != 0 {fTemp272} else {fTemp282};
			let mut fTemp291: F64 = if iTemp289 != 0 {fTemp282} else {fTemp273};
			let mut fTemp292: F64 = fTemp291 + fTemp290;
			let mut fTemp293: F64 = 32767.5 * fTemp292;
			let mut iTemp294: i32 = (fTemp293) as i32;
			let mut fTemp295: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp37, i32::wrapping_mul(9, i32::wrapping_add(iTemp294, 1)))) as usize] };
			let mut iTemp296: i32 = i32::wrapping_add(iTemp37, i32::wrapping_mul(9, iTemp294));
			let mut fTemp297: F64 = unsafe { ftbl0mydspSIG0[iTemp296 as usize] };
			let mut fTemp298: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp296, 1)) as usize] } - fTemp297;
			let mut fTemp299: F64 = (fTemp293 - (iTemp294) as F64) * (fTemp295 - (fTemp297 + fTemp41 * (fTemp298 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp296, 10)) as usize] } - fTemp295))));
			let mut fTemp300: F64 = 0.5 * fTemp292;
			let mut fTemp301: F64 = 65535.0 * (fTemp44 + fTemp300);
			let mut iTemp302: i32 = (fTemp301) as i32;
			let mut fTemp303: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp37, i32::wrapping_mul(9, i32::wrapping_add(iTemp302, 1))), 589823))) as usize] };
			let mut iTemp304: i32 = i32::wrapping_add(iTemp37, i32::wrapping_mul(9, iTemp302));
			let mut fTemp305: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp304, 589823))) as usize] };
			let mut fTemp306: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp304, 1), 589823))) as usize] } - fTemp305;
			let mut iTemp307: i32 = (fTemp72 > ((fTemp41 * (fTemp306 - fTemp298) + fTemp305 + (fTemp301 - (iTemp302) as F64) * (fTemp303 - (fTemp305 + fTemp41 * (fTemp306 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp304, 10), 589823))) as usize] } - fTemp303)))) - (fTemp297 + fTemp299)) / (1.0 - (fTemp297 + fTemp41 * fTemp298 + fTemp299)))) as i32;
			let mut fTemp308: F64 = if iTemp307 != 0 {fTemp290} else {fTemp300};
			let mut fTemp309: F64 = if iTemp307 != 0 {fTemp300} else {fTemp291};
			let mut fTemp310: F64 = fTemp309 + fTemp308;
			let mut fTemp311: F64 = 32767.5 * fTemp310;
			let mut iTemp312: i32 = (fTemp311) as i32;
			let mut fTemp313: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp37, i32::wrapping_mul(9, i32::wrapping_add(iTemp312, 1)))) as usize] };
			let mut iTemp314: i32 = i32::wrapping_add(iTemp37, i32::wrapping_mul(9, iTemp312));
			let mut fTemp315: F64 = unsafe { ftbl0mydspSIG0[iTemp314 as usize] };
			let mut fTemp316: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp314, 1)) as usize] } - fTemp315;
			let mut fTemp317: F64 = (fTemp311 - (iTemp312) as F64) * (fTemp313 - (fTemp315 + fTemp41 * (fTemp316 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp314, 10)) as usize] } - fTemp313))));
			let mut fTemp318: F64 = 0.5 * fTemp310;
			let mut fTemp319: F64 = 65535.0 * (fTemp44 + fTemp318);
			let mut iTemp320: i32 = (fTemp319) as i32;
			let mut fTemp321: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp37, i32::wrapping_mul(9, i32::wrapping_add(iTemp320, 1))), 589823))) as usize] };
			let mut iTemp322: i32 = i32::wrapping_add(iTemp37, i32::wrapping_mul(9, iTemp320));
			let mut fTemp323: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp322, 589823))) as usize] };
			let mut fTemp324: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp322, 1), 589823))) as usize] } - fTemp323;
			let mut iTemp325: i32 = (fTemp72 > ((fTemp41 * (fTemp324 - fTemp316) + fTemp323 + (fTemp319 - (iTemp320) as F64) * (fTemp321 - (fTemp323 + fTemp41 * (fTemp324 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp322, 10), 589823))) as usize] } - fTemp321)))) - (fTemp315 + fTemp317)) / (1.0 - (fTemp315 + fTemp41 * fTemp316 + fTemp317)))) as i32;
			let mut fTemp326: F64 = if iTemp325 != 0 {fTemp308} else {fTemp318};
			let mut fTemp327: F64 = if iTemp325 != 0 {fTemp318} else {fTemp309};
			let mut fTemp328: F64 = fTemp327 + fTemp326;
			let mut fTemp329: F64 = 32767.5 * fTemp328;
			let mut iTemp330: i32 = (fTemp329) as i32;
			let mut fTemp331: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp37, i32::wrapping_mul(9, i32::wrapping_add(iTemp330, 1)))) as usize] };
			let mut iTemp332: i32 = i32::wrapping_add(iTemp37, i32::wrapping_mul(9, iTemp330));
			let mut fTemp333: F64 = unsafe { ftbl0mydspSIG0[iTemp332 as usize] };
			let mut fTemp334: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp332, 1)) as usize] } - fTemp333;
			let mut fTemp335: F64 = (fTemp329 - (iTemp330) as F64) * (fTemp331 - (fTemp333 + fTemp41 * (fTemp334 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp332, 10)) as usize] } - fTemp331))));
			let mut fTemp336: F64 = 0.5 * fTemp328;
			let mut fTemp337: F64 = 65535.0 * (fTemp44 + fTemp336);
			let mut iTemp338: i32 = (fTemp337) as i32;
			let mut fTemp339: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp37, i32::wrapping_mul(9, i32::wrapping_add(iTemp338, 1))), 589823))) as usize] };
			let mut iTemp340: i32 = i32::wrapping_add(iTemp37, i32::wrapping_mul(9, iTemp338));
			let mut fTemp341: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp340, 589823))) as usize] };
			let mut fTemp342: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp340, 1), 589823))) as usize] } - fTemp341;
			let mut iTemp343: i32 = (fTemp72 > ((fTemp41 * (fTemp342 - fTemp334) + fTemp341 + (fTemp337 - (iTemp338) as F64) * (fTemp339 - (fTemp341 + fTemp41 * (fTemp342 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp340, 10), 589823))) as usize] } - fTemp339)))) - (fTemp333 + fTemp335)) / (1.0 - (fTemp333 + fTemp41 * fTemp334 + fTemp335)))) as i32;
			let mut fTemp344: F64 = F64::min(1.0, F64::max(0.0, 0.5 * (if iTemp343 != 0 {fTemp336} else {fTemp327} + if iTemp343 != 0 {fTemp326} else {fTemp336})));
			self.fRec0[0] = fTemp344;
			let mut fTemp345: F64 = 65535.0 * fTemp344;
			let mut iTemp346: i32 = (fTemp345) as i32;
			let mut fTemp347: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp37, i32::wrapping_mul(9, i32::wrapping_add(iTemp346, 1)))) as usize] };
			let mut iTemp348: i32 = i32::wrapping_add(iTemp37, i32::wrapping_mul(9, iTemp346));
			let mut fTemp349: F64 = unsafe { ftbl0mydspSIG0[iTemp348 as usize] };
			let mut fTemp350: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp348, 1)) as usize] } - fTemp349;
			let mut fTemp351: F64 = (fTemp345 - (iTemp346) as F64) * (fTemp347 - (fTemp349 + fTemp41 * (fTemp350 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp348, 10)) as usize] } - fTemp347))));
			let mut fTemp352: F64 = 65535.0 * (fTemp44 + fTemp344);
			let mut iTemp353: i32 = (fTemp352) as i32;
			let mut fTemp354: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp37, i32::wrapping_mul(9, i32::wrapping_add(iTemp353, 1))), 589823))) as usize] };
			let mut iTemp355: i32 = i32::wrapping_add(iTemp37, i32::wrapping_mul(9, iTemp353));
			let mut fTemp356: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp355, 589823))) as usize] };
			let mut fTemp357: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp355, 1), 589823))) as usize] } - fTemp356;
			let mut fTemp358: F64 = fTemp33 * (fTemp41 * (fTemp357 - fTemp350) + fTemp356 + (fTemp352 - (iTemp353) as F64) * (fTemp354 - (fTemp356 + fTemp41 * (fTemp357 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp355, 10), 589823))) as usize] } - fTemp354)))) - (fTemp349 + fTemp351)) / (1.0 - (fTemp349 + fTemp41 * fTemp350 + fTemp351));
			self.fRec1[0] = F64::min(self.fVec17[((i32::wrapping_sub(self.IOTA0, iSlow43)) & 32767) as usize], self.fRec1[1] + if iTemp34 != 0 {F64::min(fTemp33, fTemp358)} else {F64::max(fTemp33, fTemp358)});
			self.fHbargraph0 = F64::min(0.0, F64::max(-24.0, 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, self.fRec1[0]))));
			*output0 = self.fVec0[((i32::wrapping_sub(self.IOTA0, iSlow43)) & 32767) as usize] * self.fRec1[0];
			let mut fTemp359: F64 = 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, fTemp13 + fSlow9 * (fTemp14 - fTemp13)));
			let mut iTemp360: i32 = ((fTemp359 > fSlow10) as i32) + ((fTemp359 > fSlow7) as i32);
			let mut fTemp361: F64 = fTemp359 - fSlow6;
			let mut fTemp362: F64 = fSlow12 * F64::max(0.0, if (iTemp360 == 0) as i32 != 0 {0.0} else {if (iTemp360 == 1) as i32 != 0 {fSlow11 * mydsp_faustpower2_f(fSlow5 + fTemp361)} else {fTemp361}});
			self.fVec21[0] = fTemp362;
			let mut fTemp363: F64 = F64::min(-fTemp362, -self.fVec21[1]);
			self.fVec22[0] = fTemp363;
			let mut fTemp364: F64 = F64::min(fTemp363, self.fVec22[2]);
			self.fVec23[0] = fTemp364;
			let mut fTemp365: F64 = F64::min(fTemp364, self.fVec23[4]);
			self.fVec24[0] = fTemp365;
			let mut fTemp366: F64 = F64::min(fTemp365, self.fVec24[8]);
			self.fVec25[(self.IOTA0 & 31) as usize] = fTemp366;
			let mut fTemp367: F64 = F64::min(fTemp366, self.fVec25[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize]);
			self.fVec26[(self.IOTA0 & 63) as usize] = fTemp367;
			let mut fTemp368: F64 = F64::min(fTemp367, self.fVec26[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize]);
			self.fVec27[(self.IOTA0 & 127) as usize] = fTemp368;
			let mut fTemp369: F64 = F64::min(fTemp368, self.fVec27[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize]);
			self.fVec28[(self.IOTA0 & 255) as usize] = fTemp369;
			let mut fTemp370: F64 = F64::min(fTemp369, self.fVec28[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize]);
			self.fVec29[(self.IOTA0 & 511) as usize] = fTemp370;
			let mut fTemp371: F64 = F64::min(fTemp370, self.fVec29[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize]);
			self.fVec30[(self.IOTA0 & 1023) as usize] = fTemp371;
			let mut fTemp372: F64 = F64::min(fTemp371, self.fVec30[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize]);
			self.fVec31[(self.IOTA0 & 2047) as usize] = fTemp372;
			let mut fTemp373: F64 = F64::min(fTemp372, self.fVec31[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			self.fVec32[(self.IOTA0 & 4095) as usize] = fTemp373;
			let mut fTemp374: F64 = F64::min(fTemp373, self.fVec32[((i32::wrapping_sub(self.IOTA0, 2048)) & 4095) as usize]);
			self.fVec33[(self.IOTA0 & 8191) as usize] = fTemp374;
			let mut fTemp375: F64 = F64::min(fTemp374, self.fVec33[((i32::wrapping_sub(self.IOTA0, 4096)) & 8191) as usize]);
			self.fVec34[(self.IOTA0 & 16383) as usize] = fTemp375;
			self.fVec35[(self.IOTA0 & 32767) as usize] = F64::min(fTemp375, self.fVec34[((i32::wrapping_sub(self.IOTA0, 8192)) & 16383) as usize]);
			let mut fTemp376: F64 = F64::powf(1e+01, 0.05 * F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(F64::min(if iSlow3 != 0 {-fTemp362} else {1.7976931348623157e+308}, if iSlow13 != 0 {self.fVec22[iSlow3 as usize]} else {1.7976931348623157e+308}), if iSlow14 != 0 {self.fVec23[iSlow15 as usize]} else {1.7976931348623157e+308}), if iSlow16 != 0 {self.fVec24[iSlow17 as usize]} else {1.7976931348623157e+308}), if iSlow18 != 0 {self.fVec25[((i32::wrapping_sub(self.IOTA0, iSlow19)) & 31) as usize]} else {1.7976931348623157e+308}), if iSlow20 != 0 {self.fVec26[((i32::wrapping_sub(self.IOTA0, iSlow21)) & 63) as usize]} else {1.7976931348623157e+308}), if iSlow22 != 0 {self.fVec27[((i32::wrapping_sub(self.IOTA0, iSlow23)) & 127) as usize]} else {1.7976931348623157e+308}), if iSlow24 != 0 {self.fVec28[((i32::wrapping_sub(self.IOTA0, iSlow25)) & 255) as usize]} else {1.7976931348623157e+308}), if iSlow26 != 0 {self.fVec29[((i32::wrapping_sub(self.IOTA0, iSlow27)) & 511) as usize]} else {1.7976931348623157e+308}), if iSlow28 != 0 {self.fVec30[((i32::wrapping_sub(self.IOTA0, iSlow29)) & 1023) as usize]} else {1.7976931348623157e+308}), if iSlow30 != 0 {self.fVec31[((i32::wrapping_sub(self.IOTA0, iSlow31)) & 2047) as usize]} else {1.7976931348623157e+308}), if iSlow32 != 0 {self.fVec32[((i32::wrapping_sub(self.IOTA0, iSlow33)) & 4095) as usize]} else {1.7976931348623157e+308}), if iSlow34 != 0 {self.fVec33[((i32::wrapping_sub(self.IOTA0, iSlow35)) & 8191) as usize]} else {1.7976931348623157e+308}), if iSlow36 != 0 {self.fVec34[((i32::wrapping_sub(self.IOTA0, iSlow37)) & 16383) as usize]} else {1.7976931348623157e+308}), if iSlow38 != 0 {self.fVec35[((i32::wrapping_sub(self.IOTA0, iSlow39)) & 32767) as usize]} else {1.7976931348623157e+308}));
			self.fVec36[(self.IOTA0 & 32767) as usize] = fTemp376;
			let mut fTemp377: F64 = fTemp376 - self.fRec5[1];
			self.fVec37[0] = fTemp377;
			let mut iTemp378: i32 = (fTemp377 > 0.0) as i32;
			let mut fTemp379: F64 = if iTemp378 != 0 {fSlow41} else {fSlow40};
			self.fVec38[0] = fTemp379;
			let mut fTemp380: F64 = 0.8888888888888888 * fTemp379;
			let mut iTemp381: i32 = (fTemp380) as i32;
			let mut fTemp382: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp381, 294912)) as usize] };
			let mut fTemp383: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp381, 294903)) as usize] };
			let mut fTemp384: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp381, 294904)) as usize] } - fTemp383;
			let mut fTemp385: F64 = fTemp380 - (iTemp381) as F64;
			let mut fTemp386: F64 = 0.5 * (fTemp382 - (fTemp383 + fTemp385 * (fTemp384 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp381, 294913)) as usize] } - fTemp382))));
			let mut fTemp387: F64 = fSlow0 * ((fTemp377 < 0.0) as i32) as u64 as F64 + fSlow42 * (iTemp378) as F64;
			self.fVec39[0] = fTemp387;
			let mut fTemp388: F64 = self.fConst4 / fTemp387;
			let mut fTemp389: F64 = 65535.0 * (fTemp388 + 0.5);
			let mut iTemp390: i32 = (fTemp389) as i32;
			let mut fTemp391: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp381, i32::wrapping_mul(9, i32::wrapping_add(iTemp390, 1))), 589823))) as usize] };
			let mut iTemp392: i32 = i32::wrapping_add(iTemp381, i32::wrapping_mul(9, iTemp390));
			let mut fTemp393: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp392, 589823))) as usize] };
			let mut fTemp394: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp392, 1), 589823))) as usize] } - fTemp393;
			let mut fTemp395: F64 = 65535.0 * self.fRec4[1];
			let mut iTemp396: i32 = (fTemp395) as i32;
			let mut fTemp397: F64 = 0.8888888888888888 * self.fVec38[1];
			let mut iTemp398: i32 = (fTemp397) as i32;
			let mut fTemp399: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp398, i32::wrapping_mul(9, i32::wrapping_add(iTemp396, 1))), 589823))) as usize] };
			let mut iTemp400: i32 = i32::wrapping_add(i32::wrapping_mul(9, iTemp396), iTemp398);
			let mut fTemp401: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp400, 589823))) as usize] };
			let mut fTemp402: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp400, 1), 589823))) as usize] } - fTemp401;
			let mut fTemp403: F64 = fTemp397 - (iTemp398) as F64;
			let mut fTemp404: F64 = 65535.0 * (self.fRec4[1] + fTemp388);
			let mut iTemp405: i32 = (fTemp404) as i32;
			let mut fTemp406: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp381, i32::wrapping_mul(9, i32::wrapping_add(iTemp405, 1))), 589823))) as usize] };
			let mut iTemp407: i32 = i32::wrapping_add(iTemp381, i32::wrapping_mul(9, iTemp405));
			let mut fTemp408: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp407, 589823))) as usize] };
			let mut fTemp409: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp407, 1), 589823))) as usize] } - fTemp408;
			let mut fTemp410: F64 = 65535.0 * (self.fRec4[1] + self.fConst4 * (1.0 / fTemp387 + 1.0 / self.fVec39[1]));
			let mut iTemp411: i32 = (fTemp410) as i32;
			let mut fTemp412: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp381, i32::wrapping_mul(9, i32::wrapping_add(iTemp411, 1))), 589823))) as usize] };
			let mut iTemp413: i32 = i32::wrapping_add(i32::wrapping_mul(9, iTemp411), iTemp381);
			let mut fTemp414: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp413, 589823))) as usize] };
			let mut fTemp415: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp413, 1), 589823))) as usize] } - fTemp414;
			let mut fTemp416: F64 = (fTemp385 * (fTemp415 - fTemp409) + fTemp414 + (fTemp410 - (iTemp411) as F64) * (fTemp412 - (fTemp414 + fTemp385 * (fTemp415 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp413, 10), 589823))) as usize] } - fTemp412)))) - (fTemp408 + (fTemp404 - (iTemp405) as F64) * (fTemp406 - (fTemp408 + fTemp385 * (fTemp409 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp407, 10), 589823))) as usize] } - fTemp406)))))) * self.fVec37[1] / (fTemp377 * (1.0 - (fTemp401 + fTemp403 * fTemp402 + (fTemp395 - (iTemp396) as F64) * (fTemp399 - (fTemp401 + fTemp403 * (fTemp402 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp400, 10), 589823))) as usize] } - fTemp399)))))));
			let mut iTemp417: i32 = (fTemp416 > ((fTemp385 * (fTemp394 - fTemp384) + fTemp393 + (fTemp389 - (iTemp390) as F64) * (fTemp391 - (fTemp393 + fTemp385 * (fTemp394 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp392, 10), 589823))) as usize] } - fTemp391)))) - (fTemp383 + fTemp386)) / (1.0 - (fTemp383 + fTemp385 * fTemp384 + fTemp386)))) as i32;
			let mut fTemp418: F64 = if iTemp417 != 0 {1.0} else {0.5};
			let mut fTemp419: F64 = if iTemp417 != 0 {0.5} else {0.0};
			let mut fTemp420: F64 = fTemp419 + fTemp418;
			let mut fTemp421: F64 = 32767.5 * fTemp420;
			let mut iTemp422: i32 = (fTemp421) as i32;
			let mut fTemp423: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp381, i32::wrapping_mul(9, i32::wrapping_add(iTemp422, 1)))) as usize] };
			let mut iTemp424: i32 = i32::wrapping_add(iTemp381, i32::wrapping_mul(9, iTemp422));
			let mut fTemp425: F64 = unsafe { ftbl0mydspSIG0[iTemp424 as usize] };
			let mut fTemp426: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp424, 1)) as usize] } - fTemp425;
			let mut fTemp427: F64 = (fTemp421 - (iTemp422) as F64) * (fTemp423 - (fTemp425 + fTemp385 * (fTemp426 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp424, 10)) as usize] } - fTemp423))));
			let mut fTemp428: F64 = 0.5 * fTemp420;
			let mut fTemp429: F64 = 65535.0 * (fTemp388 + fTemp428);
			let mut iTemp430: i32 = (fTemp429) as i32;
			let mut fTemp431: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp381, i32::wrapping_mul(9, i32::wrapping_add(iTemp430, 1))), 589823))) as usize] };
			let mut iTemp432: i32 = i32::wrapping_add(iTemp381, i32::wrapping_mul(9, iTemp430));
			let mut fTemp433: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp432, 589823))) as usize] };
			let mut fTemp434: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp432, 1), 589823))) as usize] } - fTemp433;
			let mut iTemp435: i32 = (fTemp416 > ((fTemp385 * (fTemp434 - fTemp426) + fTemp433 + (fTemp429 - (iTemp430) as F64) * (fTemp431 - (fTemp433 + fTemp385 * (fTemp434 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp432, 10), 589823))) as usize] } - fTemp431)))) - (fTemp425 + fTemp427)) / (1.0 - (fTemp425 + fTemp385 * fTemp426 + fTemp427)))) as i32;
			let mut fTemp436: F64 = if iTemp435 != 0 {fTemp418} else {fTemp428};
			let mut fTemp437: F64 = if iTemp435 != 0 {fTemp428} else {fTemp419};
			let mut fTemp438: F64 = fTemp437 + fTemp436;
			let mut fTemp439: F64 = 32767.5 * fTemp438;
			let mut iTemp440: i32 = (fTemp439) as i32;
			let mut fTemp441: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp381, i32::wrapping_mul(9, i32::wrapping_add(iTemp440, 1)))) as usize] };
			let mut iTemp442: i32 = i32::wrapping_add(iTemp381, i32::wrapping_mul(9, iTemp440));
			let mut fTemp443: F64 = unsafe { ftbl0mydspSIG0[iTemp442 as usize] };
			let mut fTemp444: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp442, 1)) as usize] } - fTemp443;
			let mut fTemp445: F64 = (fTemp439 - (iTemp440) as F64) * (fTemp441 - (fTemp443 + fTemp385 * (fTemp444 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp442, 10)) as usize] } - fTemp441))));
			let mut fTemp446: F64 = 0.5 * fTemp438;
			let mut fTemp447: F64 = 65535.0 * (fTemp388 + fTemp446);
			let mut iTemp448: i32 = (fTemp447) as i32;
			let mut fTemp449: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp381, i32::wrapping_mul(9, i32::wrapping_add(iTemp448, 1))), 589823))) as usize] };
			let mut iTemp450: i32 = i32::wrapping_add(iTemp381, i32::wrapping_mul(9, iTemp448));
			let mut fTemp451: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp450, 589823))) as usize] };
			let mut fTemp452: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp450, 1), 589823))) as usize] } - fTemp451;
			let mut iTemp453: i32 = (fTemp416 > ((fTemp385 * (fTemp452 - fTemp444) + fTemp451 + (fTemp447 - (iTemp448) as F64) * (fTemp449 - (fTemp451 + fTemp385 * (fTemp452 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp450, 10), 589823))) as usize] } - fTemp449)))) - (fTemp443 + fTemp445)) / (1.0 - (fTemp443 + fTemp385 * fTemp444 + fTemp445)))) as i32;
			let mut fTemp454: F64 = if iTemp453 != 0 {fTemp436} else {fTemp446};
			let mut fTemp455: F64 = if iTemp453 != 0 {fTemp446} else {fTemp437};
			let mut fTemp456: F64 = fTemp455 + fTemp454;
			let mut fTemp457: F64 = 32767.5 * fTemp456;
			let mut iTemp458: i32 = (fTemp457) as i32;
			let mut fTemp459: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp381, i32::wrapping_mul(9, i32::wrapping_add(iTemp458, 1)))) as usize] };
			let mut iTemp460: i32 = i32::wrapping_add(iTemp381, i32::wrapping_mul(9, iTemp458));
			let mut fTemp461: F64 = unsafe { ftbl0mydspSIG0[iTemp460 as usize] };
			let mut fTemp462: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp460, 1)) as usize] } - fTemp461;
			let mut fTemp463: F64 = (fTemp457 - (iTemp458) as F64) * (fTemp459 - (fTemp461 + fTemp385 * (fTemp462 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp460, 10)) as usize] } - fTemp459))));
			let mut fTemp464: F64 = 0.5 * fTemp456;
			let mut fTemp465: F64 = 65535.0 * (fTemp388 + fTemp464);
			let mut iTemp466: i32 = (fTemp465) as i32;
			let mut fTemp467: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp381, i32::wrapping_mul(9, i32::wrapping_add(iTemp466, 1))), 589823))) as usize] };
			let mut iTemp468: i32 = i32::wrapping_add(iTemp381, i32::wrapping_mul(9, iTemp466));
			let mut fTemp469: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp468, 589823))) as usize] };
			let mut fTemp470: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp468, 1), 589823))) as usize] } - fTemp469;
			let mut iTemp471: i32 = (fTemp416 > ((fTemp385 * (fTemp470 - fTemp462) + fTemp469 + (fTemp465 - (iTemp466) as F64) * (fTemp467 - (fTemp469 + fTemp385 * (fTemp470 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp468, 10), 589823))) as usize] } - fTemp467)))) - (fTemp461 + fTemp463)) / (1.0 - (fTemp461 + fTemp385 * fTemp462 + fTemp463)))) as i32;
			let mut fTemp472: F64 = if iTemp471 != 0 {fTemp454} else {fTemp464};
			let mut fTemp473: F64 = if iTemp471 != 0 {fTemp464} else {fTemp455};
			let mut fTemp474: F64 = fTemp473 + fTemp472;
			let mut fTemp475: F64 = 32767.5 * fTemp474;
			let mut iTemp476: i32 = (fTemp475) as i32;
			let mut fTemp477: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp381, i32::wrapping_mul(9, i32::wrapping_add(iTemp476, 1)))) as usize] };
			let mut iTemp478: i32 = i32::wrapping_add(iTemp381, i32::wrapping_mul(9, iTemp476));
			let mut fTemp479: F64 = unsafe { ftbl0mydspSIG0[iTemp478 as usize] };
			let mut fTemp480: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp478, 1)) as usize] } - fTemp479;
			let mut fTemp481: F64 = (fTemp475 - (iTemp476) as F64) * (fTemp477 - (fTemp479 + fTemp385 * (fTemp480 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp478, 10)) as usize] } - fTemp477))));
			let mut fTemp482: F64 = 0.5 * fTemp474;
			let mut fTemp483: F64 = 65535.0 * (fTemp388 + fTemp482);
			let mut iTemp484: i32 = (fTemp483) as i32;
			let mut fTemp485: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp381, i32::wrapping_mul(9, i32::wrapping_add(iTemp484, 1))), 589823))) as usize] };
			let mut iTemp486: i32 = i32::wrapping_add(iTemp381, i32::wrapping_mul(9, iTemp484));
			let mut fTemp487: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp486, 589823))) as usize] };
			let mut fTemp488: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp486, 1), 589823))) as usize] } - fTemp487;
			let mut iTemp489: i32 = (fTemp416 > ((fTemp385 * (fTemp488 - fTemp480) + fTemp487 + (fTemp483 - (iTemp484) as F64) * (fTemp485 - (fTemp487 + fTemp385 * (fTemp488 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp486, 10), 589823))) as usize] } - fTemp485)))) - (fTemp479 + fTemp481)) / (1.0 - (fTemp479 + fTemp385 * fTemp480 + fTemp481)))) as i32;
			let mut fTemp490: F64 = if iTemp489 != 0 {fTemp472} else {fTemp482};
			let mut fTemp491: F64 = if iTemp489 != 0 {fTemp482} else {fTemp473};
			let mut fTemp492: F64 = fTemp491 + fTemp490;
			let mut fTemp493: F64 = 32767.5 * fTemp492;
			let mut iTemp494: i32 = (fTemp493) as i32;
			let mut fTemp495: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp381, i32::wrapping_mul(9, i32::wrapping_add(iTemp494, 1)))) as usize] };
			let mut iTemp496: i32 = i32::wrapping_add(iTemp381, i32::wrapping_mul(9, iTemp494));
			let mut fTemp497: F64 = unsafe { ftbl0mydspSIG0[iTemp496 as usize] };
			let mut fTemp498: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp496, 1)) as usize] } - fTemp497;
			let mut fTemp499: F64 = (fTemp493 - (iTemp494) as F64) * (fTemp495 - (fTemp497 + fTemp385 * (fTemp498 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp496, 10)) as usize] } - fTemp495))));
			let mut fTemp500: F64 = 0.5 * fTemp492;
			let mut fTemp501: F64 = 65535.0 * (fTemp388 + fTemp500);
			let mut iTemp502: i32 = (fTemp501) as i32;
			let mut fTemp503: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp381, i32::wrapping_mul(9, i32::wrapping_add(iTemp502, 1))), 589823))) as usize] };
			let mut iTemp504: i32 = i32::wrapping_add(iTemp381, i32::wrapping_mul(9, iTemp502));
			let mut fTemp505: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp504, 589823))) as usize] };
			let mut fTemp506: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp504, 1), 589823))) as usize] } - fTemp505;
			let mut iTemp507: i32 = (fTemp416 > ((fTemp385 * (fTemp506 - fTemp498) + fTemp505 + (fTemp501 - (iTemp502) as F64) * (fTemp503 - (fTemp505 + fTemp385 * (fTemp506 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp504, 10), 589823))) as usize] } - fTemp503)))) - (fTemp497 + fTemp499)) / (1.0 - (fTemp497 + fTemp385 * fTemp498 + fTemp499)))) as i32;
			let mut fTemp508: F64 = if iTemp507 != 0 {fTemp490} else {fTemp500};
			let mut fTemp509: F64 = if iTemp507 != 0 {fTemp500} else {fTemp491};
			let mut fTemp510: F64 = fTemp509 + fTemp508;
			let mut fTemp511: F64 = 32767.5 * fTemp510;
			let mut iTemp512: i32 = (fTemp511) as i32;
			let mut fTemp513: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp381, i32::wrapping_mul(9, i32::wrapping_add(iTemp512, 1)))) as usize] };
			let mut iTemp514: i32 = i32::wrapping_add(iTemp381, i32::wrapping_mul(9, iTemp512));
			let mut fTemp515: F64 = unsafe { ftbl0mydspSIG0[iTemp514 as usize] };
			let mut fTemp516: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp514, 1)) as usize] } - fTemp515;
			let mut fTemp517: F64 = (fTemp511 - (iTemp512) as F64) * (fTemp513 - (fTemp515 + fTemp385 * (fTemp516 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp514, 10)) as usize] } - fTemp513))));
			let mut fTemp518: F64 = 0.5 * fTemp510;
			let mut fTemp519: F64 = 65535.0 * (fTemp388 + fTemp518);
			let mut iTemp520: i32 = (fTemp519) as i32;
			let mut fTemp521: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp381, i32::wrapping_mul(9, i32::wrapping_add(iTemp520, 1))), 589823))) as usize] };
			let mut iTemp522: i32 = i32::wrapping_add(iTemp381, i32::wrapping_mul(9, iTemp520));
			let mut fTemp523: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp522, 589823))) as usize] };
			let mut fTemp524: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp522, 1), 589823))) as usize] } - fTemp523;
			let mut iTemp525: i32 = (fTemp416 > ((fTemp385 * (fTemp524 - fTemp516) + fTemp523 + (fTemp519 - (iTemp520) as F64) * (fTemp521 - (fTemp523 + fTemp385 * (fTemp524 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp522, 10), 589823))) as usize] } - fTemp521)))) - (fTemp515 + fTemp517)) / (1.0 - (fTemp515 + fTemp385 * fTemp516 + fTemp517)))) as i32;
			let mut fTemp526: F64 = if iTemp525 != 0 {fTemp508} else {fTemp518};
			let mut fTemp527: F64 = if iTemp525 != 0 {fTemp518} else {fTemp509};
			let mut fTemp528: F64 = fTemp527 + fTemp526;
			let mut fTemp529: F64 = 32767.5 * fTemp528;
			let mut iTemp530: i32 = (fTemp529) as i32;
			let mut fTemp531: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp381, i32::wrapping_mul(9, i32::wrapping_add(iTemp530, 1)))) as usize] };
			let mut iTemp532: i32 = i32::wrapping_add(iTemp381, i32::wrapping_mul(9, iTemp530));
			let mut fTemp533: F64 = unsafe { ftbl0mydspSIG0[iTemp532 as usize] };
			let mut fTemp534: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp532, 1)) as usize] } - fTemp533;
			let mut fTemp535: F64 = (fTemp529 - (iTemp530) as F64) * (fTemp531 - (fTemp533 + fTemp385 * (fTemp534 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp532, 10)) as usize] } - fTemp531))));
			let mut fTemp536: F64 = 0.5 * fTemp528;
			let mut fTemp537: F64 = 65535.0 * (fTemp388 + fTemp536);
			let mut iTemp538: i32 = (fTemp537) as i32;
			let mut fTemp539: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp381, i32::wrapping_mul(9, i32::wrapping_add(iTemp538, 1))), 589823))) as usize] };
			let mut iTemp540: i32 = i32::wrapping_add(iTemp381, i32::wrapping_mul(9, iTemp538));
			let mut fTemp541: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp540, 589823))) as usize] };
			let mut fTemp542: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp540, 1), 589823))) as usize] } - fTemp541;
			let mut iTemp543: i32 = (fTemp416 > ((fTemp385 * (fTemp542 - fTemp534) + fTemp541 + (fTemp537 - (iTemp538) as F64) * (fTemp539 - (fTemp541 + fTemp385 * (fTemp542 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp540, 10), 589823))) as usize] } - fTemp539)))) - (fTemp533 + fTemp535)) / (1.0 - (fTemp533 + fTemp385 * fTemp534 + fTemp535)))) as i32;
			let mut fTemp544: F64 = if iTemp543 != 0 {fTemp526} else {fTemp536};
			let mut fTemp545: F64 = if iTemp543 != 0 {fTemp536} else {fTemp527};
			let mut fTemp546: F64 = fTemp545 + fTemp544;
			let mut fTemp547: F64 = 32767.5 * fTemp546;
			let mut iTemp548: i32 = (fTemp547) as i32;
			let mut fTemp549: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp381, i32::wrapping_mul(9, i32::wrapping_add(iTemp548, 1)))) as usize] };
			let mut iTemp550: i32 = i32::wrapping_add(iTemp381, i32::wrapping_mul(9, iTemp548));
			let mut fTemp551: F64 = unsafe { ftbl0mydspSIG0[iTemp550 as usize] };
			let mut fTemp552: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp550, 1)) as usize] } - fTemp551;
			let mut fTemp553: F64 = (fTemp547 - (iTemp548) as F64) * (fTemp549 - (fTemp551 + fTemp385 * (fTemp552 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp550, 10)) as usize] } - fTemp549))));
			let mut fTemp554: F64 = 0.5 * fTemp546;
			let mut fTemp555: F64 = 65535.0 * (fTemp388 + fTemp554);
			let mut iTemp556: i32 = (fTemp555) as i32;
			let mut fTemp557: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp381, i32::wrapping_mul(9, i32::wrapping_add(iTemp556, 1))), 589823))) as usize] };
			let mut iTemp558: i32 = i32::wrapping_add(iTemp381, i32::wrapping_mul(9, iTemp556));
			let mut fTemp559: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp558, 589823))) as usize] };
			let mut fTemp560: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp558, 1), 589823))) as usize] } - fTemp559;
			let mut iTemp561: i32 = (fTemp416 > ((fTemp385 * (fTemp560 - fTemp552) + fTemp559 + (fTemp555 - (iTemp556) as F64) * (fTemp557 - (fTemp559 + fTemp385 * (fTemp560 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp558, 10), 589823))) as usize] } - fTemp557)))) - (fTemp551 + fTemp553)) / (1.0 - (fTemp551 + fTemp385 * fTemp552 + fTemp553)))) as i32;
			let mut fTemp562: F64 = if iTemp561 != 0 {fTemp544} else {fTemp554};
			let mut fTemp563: F64 = if iTemp561 != 0 {fTemp554} else {fTemp545};
			let mut fTemp564: F64 = fTemp563 + fTemp562;
			let mut fTemp565: F64 = 32767.5 * fTemp564;
			let mut iTemp566: i32 = (fTemp565) as i32;
			let mut fTemp567: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp381, i32::wrapping_mul(9, i32::wrapping_add(iTemp566, 1)))) as usize] };
			let mut iTemp568: i32 = i32::wrapping_add(iTemp381, i32::wrapping_mul(9, iTemp566));
			let mut fTemp569: F64 = unsafe { ftbl0mydspSIG0[iTemp568 as usize] };
			let mut fTemp570: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp568, 1)) as usize] } - fTemp569;
			let mut fTemp571: F64 = (fTemp565 - (iTemp566) as F64) * (fTemp567 - (fTemp569 + fTemp385 * (fTemp570 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp568, 10)) as usize] } - fTemp567))));
			let mut fTemp572: F64 = 0.5 * fTemp564;
			let mut fTemp573: F64 = 65535.0 * (fTemp388 + fTemp572);
			let mut iTemp574: i32 = (fTemp573) as i32;
			let mut fTemp575: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp381, i32::wrapping_mul(9, i32::wrapping_add(iTemp574, 1))), 589823))) as usize] };
			let mut iTemp576: i32 = i32::wrapping_add(iTemp381, i32::wrapping_mul(9, iTemp574));
			let mut fTemp577: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp576, 589823))) as usize] };
			let mut fTemp578: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp576, 1), 589823))) as usize] } - fTemp577;
			let mut iTemp579: i32 = (fTemp416 > ((fTemp385 * (fTemp578 - fTemp570) + fTemp577 + (fTemp573 - (iTemp574) as F64) * (fTemp575 - (fTemp577 + fTemp385 * (fTemp578 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp576, 10), 589823))) as usize] } - fTemp575)))) - (fTemp569 + fTemp571)) / (1.0 - (fTemp569 + fTemp385 * fTemp570 + fTemp571)))) as i32;
			let mut fTemp580: F64 = if iTemp579 != 0 {fTemp562} else {fTemp572};
			let mut fTemp581: F64 = if iTemp579 != 0 {fTemp572} else {fTemp563};
			let mut fTemp582: F64 = fTemp581 + fTemp580;
			let mut fTemp583: F64 = 32767.5 * fTemp582;
			let mut iTemp584: i32 = (fTemp583) as i32;
			let mut fTemp585: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp381, i32::wrapping_mul(9, i32::wrapping_add(iTemp584, 1)))) as usize] };
			let mut iTemp586: i32 = i32::wrapping_add(iTemp381, i32::wrapping_mul(9, iTemp584));
			let mut fTemp587: F64 = unsafe { ftbl0mydspSIG0[iTemp586 as usize] };
			let mut fTemp588: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp586, 1)) as usize] } - fTemp587;
			let mut fTemp589: F64 = (fTemp583 - (iTemp584) as F64) * (fTemp585 - (fTemp587 + fTemp385 * (fTemp588 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp586, 10)) as usize] } - fTemp585))));
			let mut fTemp590: F64 = 0.5 * fTemp582;
			let mut fTemp591: F64 = 65535.0 * (fTemp388 + fTemp590);
			let mut iTemp592: i32 = (fTemp591) as i32;
			let mut fTemp593: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp381, i32::wrapping_mul(9, i32::wrapping_add(iTemp592, 1))), 589823))) as usize] };
			let mut iTemp594: i32 = i32::wrapping_add(iTemp381, i32::wrapping_mul(9, iTemp592));
			let mut fTemp595: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp594, 589823))) as usize] };
			let mut fTemp596: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp594, 1), 589823))) as usize] } - fTemp595;
			let mut iTemp597: i32 = (fTemp416 > ((fTemp385 * (fTemp596 - fTemp588) + fTemp595 + (fTemp591 - (iTemp592) as F64) * (fTemp593 - (fTemp595 + fTemp385 * (fTemp596 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp594, 10), 589823))) as usize] } - fTemp593)))) - (fTemp587 + fTemp589)) / (1.0 - (fTemp587 + fTemp385 * fTemp588 + fTemp589)))) as i32;
			let mut fTemp598: F64 = if iTemp597 != 0 {fTemp580} else {fTemp590};
			let mut fTemp599: F64 = if iTemp597 != 0 {fTemp590} else {fTemp581};
			let mut fTemp600: F64 = fTemp599 + fTemp598;
			let mut fTemp601: F64 = 32767.5 * fTemp600;
			let mut iTemp602: i32 = (fTemp601) as i32;
			let mut fTemp603: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp381, i32::wrapping_mul(9, i32::wrapping_add(iTemp602, 1)))) as usize] };
			let mut iTemp604: i32 = i32::wrapping_add(iTemp381, i32::wrapping_mul(9, iTemp602));
			let mut fTemp605: F64 = unsafe { ftbl0mydspSIG0[iTemp604 as usize] };
			let mut fTemp606: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp604, 1)) as usize] } - fTemp605;
			let mut fTemp607: F64 = (fTemp601 - (iTemp602) as F64) * (fTemp603 - (fTemp605 + fTemp385 * (fTemp606 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp604, 10)) as usize] } - fTemp603))));
			let mut fTemp608: F64 = 0.5 * fTemp600;
			let mut fTemp609: F64 = 65535.0 * (fTemp388 + fTemp608);
			let mut iTemp610: i32 = (fTemp609) as i32;
			let mut fTemp611: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp381, i32::wrapping_mul(9, i32::wrapping_add(iTemp610, 1))), 589823))) as usize] };
			let mut iTemp612: i32 = i32::wrapping_add(iTemp381, i32::wrapping_mul(9, iTemp610));
			let mut fTemp613: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp612, 589823))) as usize] };
			let mut fTemp614: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp612, 1), 589823))) as usize] } - fTemp613;
			let mut iTemp615: i32 = (fTemp416 > ((fTemp385 * (fTemp614 - fTemp606) + fTemp613 + (fTemp609 - (iTemp610) as F64) * (fTemp611 - (fTemp613 + fTemp385 * (fTemp614 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp612, 10), 589823))) as usize] } - fTemp611)))) - (fTemp605 + fTemp607)) / (1.0 - (fTemp605 + fTemp385 * fTemp606 + fTemp607)))) as i32;
			let mut fTemp616: F64 = if iTemp615 != 0 {fTemp598} else {fTemp608};
			let mut fTemp617: F64 = if iTemp615 != 0 {fTemp608} else {fTemp599};
			let mut fTemp618: F64 = fTemp617 + fTemp616;
			let mut fTemp619: F64 = 32767.5 * fTemp618;
			let mut iTemp620: i32 = (fTemp619) as i32;
			let mut fTemp621: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp381, i32::wrapping_mul(9, i32::wrapping_add(iTemp620, 1)))) as usize] };
			let mut iTemp622: i32 = i32::wrapping_add(iTemp381, i32::wrapping_mul(9, iTemp620));
			let mut fTemp623: F64 = unsafe { ftbl0mydspSIG0[iTemp622 as usize] };
			let mut fTemp624: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp622, 1)) as usize] } - fTemp623;
			let mut fTemp625: F64 = (fTemp619 - (iTemp620) as F64) * (fTemp621 - (fTemp623 + fTemp385 * (fTemp624 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp622, 10)) as usize] } - fTemp621))));
			let mut fTemp626: F64 = 0.5 * fTemp618;
			let mut fTemp627: F64 = 65535.0 * (fTemp388 + fTemp626);
			let mut iTemp628: i32 = (fTemp627) as i32;
			let mut fTemp629: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp381, i32::wrapping_mul(9, i32::wrapping_add(iTemp628, 1))), 589823))) as usize] };
			let mut iTemp630: i32 = i32::wrapping_add(iTemp381, i32::wrapping_mul(9, iTemp628));
			let mut fTemp631: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp630, 589823))) as usize] };
			let mut fTemp632: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp630, 1), 589823))) as usize] } - fTemp631;
			let mut iTemp633: i32 = (fTemp416 > ((fTemp385 * (fTemp632 - fTemp624) + fTemp631 + (fTemp627 - (iTemp628) as F64) * (fTemp629 - (fTemp631 + fTemp385 * (fTemp632 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp630, 10), 589823))) as usize] } - fTemp629)))) - (fTemp623 + fTemp625)) / (1.0 - (fTemp623 + fTemp385 * fTemp624 + fTemp625)))) as i32;
			let mut fTemp634: F64 = if iTemp633 != 0 {fTemp616} else {fTemp626};
			let mut fTemp635: F64 = if iTemp633 != 0 {fTemp626} else {fTemp617};
			let mut fTemp636: F64 = fTemp635 + fTemp634;
			let mut fTemp637: F64 = 32767.5 * fTemp636;
			let mut iTemp638: i32 = (fTemp637) as i32;
			let mut fTemp639: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp381, i32::wrapping_mul(9, i32::wrapping_add(iTemp638, 1)))) as usize] };
			let mut iTemp640: i32 = i32::wrapping_add(iTemp381, i32::wrapping_mul(9, iTemp638));
			let mut fTemp641: F64 = unsafe { ftbl0mydspSIG0[iTemp640 as usize] };
			let mut fTemp642: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp640, 1)) as usize] } - fTemp641;
			let mut fTemp643: F64 = (fTemp637 - (iTemp638) as F64) * (fTemp639 - (fTemp641 + fTemp385 * (fTemp642 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp640, 10)) as usize] } - fTemp639))));
			let mut fTemp644: F64 = 0.5 * fTemp636;
			let mut fTemp645: F64 = 65535.0 * (fTemp388 + fTemp644);
			let mut iTemp646: i32 = (fTemp645) as i32;
			let mut fTemp647: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp381, i32::wrapping_mul(9, i32::wrapping_add(iTemp646, 1))), 589823))) as usize] };
			let mut iTemp648: i32 = i32::wrapping_add(iTemp381, i32::wrapping_mul(9, iTemp646));
			let mut fTemp649: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp648, 589823))) as usize] };
			let mut fTemp650: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp648, 1), 589823))) as usize] } - fTemp649;
			let mut iTemp651: i32 = (fTemp416 > ((fTemp385 * (fTemp650 - fTemp642) + fTemp649 + (fTemp645 - (iTemp646) as F64) * (fTemp647 - (fTemp649 + fTemp385 * (fTemp650 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp648, 10), 589823))) as usize] } - fTemp647)))) - (fTemp641 + fTemp643)) / (1.0 - (fTemp641 + fTemp385 * fTemp642 + fTemp643)))) as i32;
			let mut fTemp652: F64 = if iTemp651 != 0 {fTemp634} else {fTemp644};
			let mut fTemp653: F64 = if iTemp651 != 0 {fTemp644} else {fTemp635};
			let mut fTemp654: F64 = fTemp653 + fTemp652;
			let mut fTemp655: F64 = 32767.5 * fTemp654;
			let mut iTemp656: i32 = (fTemp655) as i32;
			let mut fTemp657: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp381, i32::wrapping_mul(9, i32::wrapping_add(iTemp656, 1)))) as usize] };
			let mut iTemp658: i32 = i32::wrapping_add(iTemp381, i32::wrapping_mul(9, iTemp656));
			let mut fTemp659: F64 = unsafe { ftbl0mydspSIG0[iTemp658 as usize] };
			let mut fTemp660: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp658, 1)) as usize] } - fTemp659;
			let mut fTemp661: F64 = (fTemp655 - (iTemp656) as F64) * (fTemp657 - (fTemp659 + fTemp385 * (fTemp660 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp658, 10)) as usize] } - fTemp657))));
			let mut fTemp662: F64 = 0.5 * fTemp654;
			let mut fTemp663: F64 = 65535.0 * (fTemp388 + fTemp662);
			let mut iTemp664: i32 = (fTemp663) as i32;
			let mut fTemp665: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp381, i32::wrapping_mul(9, i32::wrapping_add(iTemp664, 1))), 589823))) as usize] };
			let mut iTemp666: i32 = i32::wrapping_add(iTemp381, i32::wrapping_mul(9, iTemp664));
			let mut fTemp667: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp666, 589823))) as usize] };
			let mut fTemp668: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp666, 1), 589823))) as usize] } - fTemp667;
			let mut iTemp669: i32 = (fTemp416 > ((fTemp385 * (fTemp668 - fTemp660) + fTemp667 + (fTemp663 - (iTemp664) as F64) * (fTemp665 - (fTemp667 + fTemp385 * (fTemp668 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp666, 10), 589823))) as usize] } - fTemp665)))) - (fTemp659 + fTemp661)) / (1.0 - (fTemp659 + fTemp385 * fTemp660 + fTemp661)))) as i32;
			let mut fTemp670: F64 = if iTemp669 != 0 {fTemp652} else {fTemp662};
			let mut fTemp671: F64 = if iTemp669 != 0 {fTemp662} else {fTemp653};
			let mut fTemp672: F64 = fTemp671 + fTemp670;
			let mut fTemp673: F64 = 32767.5 * fTemp672;
			let mut iTemp674: i32 = (fTemp673) as i32;
			let mut fTemp675: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp381, i32::wrapping_mul(9, i32::wrapping_add(iTemp674, 1)))) as usize] };
			let mut iTemp676: i32 = i32::wrapping_add(iTemp381, i32::wrapping_mul(9, iTemp674));
			let mut fTemp677: F64 = unsafe { ftbl0mydspSIG0[iTemp676 as usize] };
			let mut fTemp678: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp676, 1)) as usize] } - fTemp677;
			let mut fTemp679: F64 = (fTemp673 - (iTemp674) as F64) * (fTemp675 - (fTemp677 + fTemp385 * (fTemp678 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp676, 10)) as usize] } - fTemp675))));
			let mut fTemp680: F64 = 0.5 * fTemp672;
			let mut fTemp681: F64 = 65535.0 * (fTemp388 + fTemp680);
			let mut iTemp682: i32 = (fTemp681) as i32;
			let mut fTemp683: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp381, i32::wrapping_mul(9, i32::wrapping_add(iTemp682, 1))), 589823))) as usize] };
			let mut iTemp684: i32 = i32::wrapping_add(iTemp381, i32::wrapping_mul(9, iTemp682));
			let mut fTemp685: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp684, 589823))) as usize] };
			let mut fTemp686: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp684, 1), 589823))) as usize] } - fTemp685;
			let mut iTemp687: i32 = (fTemp416 > ((fTemp385 * (fTemp686 - fTemp678) + fTemp685 + (fTemp681 - (iTemp682) as F64) * (fTemp683 - (fTemp685 + fTemp385 * (fTemp686 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp684, 10), 589823))) as usize] } - fTemp683)))) - (fTemp677 + fTemp679)) / (1.0 - (fTemp677 + fTemp385 * fTemp678 + fTemp679)))) as i32;
			let mut fTemp688: F64 = F64::min(1.0, F64::max(0.0, 0.5 * (if iTemp687 != 0 {fTemp680} else {fTemp671} + if iTemp687 != 0 {fTemp670} else {fTemp680})));
			self.fRec4[0] = fTemp688;
			let mut fTemp689: F64 = 65535.0 * fTemp688;
			let mut iTemp690: i32 = (fTemp689) as i32;
			let mut fTemp691: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp381, i32::wrapping_mul(9, i32::wrapping_add(iTemp690, 1)))) as usize] };
			let mut iTemp692: i32 = i32::wrapping_add(iTemp381, i32::wrapping_mul(9, iTemp690));
			let mut fTemp693: F64 = unsafe { ftbl0mydspSIG0[iTemp692 as usize] };
			let mut fTemp694: F64 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp692, 1)) as usize] } - fTemp693;
			let mut fTemp695: F64 = (fTemp689 - (iTemp690) as F64) * (fTemp691 - (fTemp693 + fTemp385 * (fTemp694 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp692, 10)) as usize] } - fTemp691))));
			let mut fTemp696: F64 = 65535.0 * (fTemp388 + fTemp688);
			let mut iTemp697: i32 = (fTemp696) as i32;
			let mut fTemp698: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp381, i32::wrapping_mul(9, i32::wrapping_add(iTemp697, 1))), 589823))) as usize] };
			let mut iTemp699: i32 = i32::wrapping_add(iTemp381, i32::wrapping_mul(9, iTemp697));
			let mut fTemp700: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp699, 589823))) as usize] };
			let mut fTemp701: F64 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, 1), 589823))) as usize] } - fTemp700;
			let mut fTemp702: F64 = fTemp377 * (fTemp385 * (fTemp701 - fTemp694) + fTemp700 + (fTemp696 - (iTemp697) as F64) * (fTemp698 - (fTemp700 + fTemp385 * (fTemp701 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp699, 10), 589823))) as usize] } - fTemp698)))) - (fTemp693 + fTemp695)) / (1.0 - (fTemp693 + fTemp385 * fTemp694 + fTemp695));
			self.fRec5[0] = F64::min(self.fVec36[((i32::wrapping_sub(self.IOTA0, iSlow43)) & 32767) as usize], self.fRec5[1] + if iTemp378 != 0 {F64::min(fTemp377, fTemp702)} else {F64::max(fTemp377, fTemp702)});
			self.fHbargraph1 = F64::min(0.0, F64::max(-24.0, 2e+01 * F64::log10(F64::max(2.2250738585072014e-308, self.fRec5[0]))));
			*output1 = self.fVec1[((i32::wrapping_sub(self.IOTA0, iSlow43)) & 32767) as usize] * self.fRec5[0];
			self.fRec3[1] = self.fRec3[0];
			self.IOTA0 = i32::wrapping_add(self.IOTA0, 1);
			self.fVec2[1] = self.fVec2[0];
			self.fVec3[2] = self.fVec3[1];
			self.fVec3[1] = self.fVec3[0];
			for j0 in (1..=6).rev() {
				self.fVec4[j0 as usize] = self.fVec4[(i32::wrapping_sub(j0, 1)) as usize];
			}
			for j1 in (1..=14).rev() {
				self.fVec5[j1 as usize] = self.fVec5[(i32::wrapping_sub(j1, 1)) as usize];
			}
			self.fVec18[1] = self.fVec18[0];
			self.fVec19[1] = self.fVec19[0];
			self.fVec20[1] = self.fVec20[0];
			self.fRec0[1] = self.fRec0[0];
			self.fRec1[1] = self.fRec1[0];
			self.fVec21[1] = self.fVec21[0];
			self.fVec22[2] = self.fVec22[1];
			self.fVec22[1] = self.fVec22[0];
			for j2 in (1..=6).rev() {
				self.fVec23[j2 as usize] = self.fVec23[(i32::wrapping_sub(j2, 1)) as usize];
			}
			for j3 in (1..=14).rev() {
				self.fVec24[j3 as usize] = self.fVec24[(i32::wrapping_sub(j3, 1)) as usize];
			}
			self.fVec37[1] = self.fVec37[0];
			self.fVec38[1] = self.fVec38[0];
			self.fVec39[1] = self.fVec39[0];
			self.fRec4[1] = self.fRec4[0];
			self.fRec5[1] = self.fRec5[0];
		}
	}

}


}
pub use dsp::mydsp as Lamb;
