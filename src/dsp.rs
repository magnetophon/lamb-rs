mod dsp {
    /* ------------------------------------------------------------
author: "Bart Brouns"
license: "AGPLv3"
name: "lamb"
version: "0.1"
Code generated with Faust 2.70.3 (https://faust.grame.fr)
Compilation options: -a /run/user/1001/.tmpmasGfv -lang rust -ct 1 -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0
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


fn mydspSIG0_faustpower2_f(value: F32) -> F32 {
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
	
	fn fillmydspSIG0(&mut self, count: i32, table: &mut[F32]) {
		for i1 in 0..count {
			self.iRec2[0] = i32::wrapping_add(self.iRec2[1], 1);
			let mut iTemp0: i32 = i32::wrapping_add(self.iRec2[0], -1);
			let mut fTemp1: F32 = 0.05625 * (iTemp0 % 9) as F32 as i32 as F32;
			let mut fTemp2: F32 = fTemp1 + 0.3;
			let mut fTemp3: F32 = 1.0 / fTemp2;
			let mut fTemp4: F32 = F32::min(2.0 * fTemp2, 2.0 * (1.0 - fTemp2));
			let mut fTemp5: F32 = 0.5 * fTemp4;
			let mut fTemp6: F32 = F32::min(1.0, F32::max(0.0, 1.5259022e-05 * (0.11111111 * (iTemp0 % 589824) as F32) as i32 as F32));
			let mut iTemp7: i32 = ((fTemp6 > (fTemp1 + (0.3 - fTemp5))) as i32) + ((fTemp6 > (fTemp1 + fTemp5 + 0.3)) as i32);
			let mut fTemp8: F32 = -0.3 - fTemp1;
			table[i1 as usize] = F32::powf(0.5 * (F32::sin(6.2831855 * (0.25 * ((fTemp6 - (fTemp3 + -2.0) * if (iTemp7 == 0) as i32 != 0 {0.0} else {if (iTemp7 == 1) as i32 != 0 {0.5 * (mydspSIG0_faustpower2_f(fTemp6 + fTemp5 + fTemp8) / fTemp4)} else {fTemp6 + fTemp8}} / (fTemp3 + -1.0)) / fTemp2) + 0.75)) + 1.0), 1.3333334 * fTemp2 + 0.33333334);
			self.iRec2[1] = self.iRec2[0];
		}
	}

}


pub fn newmydspSIG0() -> mydspSIG0 { 
	mydspSIG0 {
		iRec2: [0;2],
	}
}
static mut ftbl0mydspSIG0: [F32;589824] = [0.0;589824];
fn mydsp_faustpower2_f(value: F32) -> F32 {
	return value * value;
}

#[cfg_attr(feature = "default-boxed", derive(default_boxed::DefaultBoxed))]
#[repr(C)]
#[derive(Debug,Clone)]
pub struct mydsp {
	fSampleRate: i32,
	fConst0: F32,
	fConst1: F32,
	fHslider0: F32,
	fHslider1: F32,
	fHslider2: F32,
	fConst2: F32,
	fConst3: F32,
	fHslider3: F32,
	fRec3: [F32;2],
	IOTA0: i32,
	fVec0: [F32;262144],
	fVec1: [F32;262144],
	fHslider4: F32,
	fHslider5: F32,
	fVec2: [F32;2],
	fVec3: [F32;3],
	fVec4: [F32;7],
	fVec5: [F32;15],
	fVec6: [F32;32],
	fVec7: [F32;64],
	fVec8: [F32;128],
	fVec9: [F32;256],
	fVec10: [F32;512],
	fVec11: [F32;1024],
	fVec12: [F32;2048],
	fVec13: [F32;4096],
	fVec14: [F32;8192],
	fVec15: [F32;2],
	fHslider6: F32,
	fHslider7: F32,
	fVec16: [F32;2],
	fHslider8: F32,
	fVec17: [F32;2],
	fRec0: [F32;2],
	fRec1: [F32;2],
	fHbargraph0: F32,
	fVec18: [F32;2],
	fVec19: [F32;3],
	fVec20: [F32;7],
	fVec21: [F32;15],
	fVec22: [F32;32],
	fVec23: [F32;64],
	fVec24: [F32;128],
	fVec25: [F32;256],
	fVec26: [F32;512],
	fVec27: [F32;1024],
	fVec28: [F32;2048],
	fVec29: [F32;4096],
	fVec30: [F32;8192],
	fVec31: [F32;2],
	fVec32: [F32;2],
	fVec33: [F32;2],
	fRec4: [F32;2],
	fRec5: [F32;2],
	fHbargraph1: F32,
}

impl FaustDsp for mydsp {
	type T = F32;
		
	fn new() -> mydsp { 
		mydsp {
			fSampleRate: 0,
			fConst0: 0.0,
			fConst1: 0.0,
			fHslider0: 0.0,
			fHslider1: 0.0,
			fHslider2: 0.0,
			fConst2: 0.0,
			fConst3: 0.0,
			fHslider3: 0.0,
			fRec3: [0.0;2],
			IOTA0: 0,
			fVec0: [0.0;262144],
			fVec1: [0.0;262144],
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
			fVec15: [0.0;2],
			fHslider6: 0.0,
			fHslider7: 0.0,
			fVec16: [0.0;2],
			fHslider8: 0.0,
			fVec17: [0.0;2],
			fRec0: [0.0;2],
			fRec1: [0.0;2],
			fHbargraph0: 0.0,
			fVec18: [0.0;2],
			fVec19: [0.0;3],
			fVec20: [0.0;7],
			fVec21: [0.0;15],
			fVec22: [0.0;32],
			fVec23: [0.0;64],
			fVec24: [0.0;128],
			fVec25: [0.0;256],
			fVec26: [0.0;512],
			fVec27: [0.0;1024],
			fVec28: [0.0;2048],
			fVec29: [0.0;4096],
			fVec30: [0.0;8192],
			fVec31: [0.0;2],
			fVec32: [0.0;2],
			fVec33: [0.0;2],
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
		m.declare("basics.lib/version", r"1.12.0");
		m.declare("compile_options", r"-a /run/user/1001/.tmpmasGfv -lang rust -ct 1 -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0");
		m.declare("filename", r"gain.dsp");
		m.declare("interpolators.lib/interpolate_linear:author", r"Stéphane Letz");
		m.declare("interpolators.lib/interpolate_linear:licence", r"MIT");
		m.declare("interpolators.lib/name", r"Faust Interpolator Library");
		m.declare("interpolators.lib/version", r"1.3.0");
		m.declare("license", r"AGPLv3");
		m.declare("maths.lib/author", r"GRAME");
		m.declare("maths.lib/copyright", r"GRAME");
		m.declare("maths.lib/license", r"LGPL with exception");
		m.declare("maths.lib/name", r"Faust Math Library");
		m.declare("maths.lib/version", r"2.7.0");
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
		for l2 in 0..262144 {
			self.fVec0[l2 as usize] = 0.0;
		}
		for l3 in 0..262144 {
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
		for l17 in 0..2 {
			self.fVec15[l17 as usize] = 0.0;
		}
		for l18 in 0..2 {
			self.fVec16[l18 as usize] = 0.0;
		}
		for l19 in 0..2 {
			self.fVec17[l19 as usize] = 0.0;
		}
		for l20 in 0..2 {
			self.fRec0[l20 as usize] = 0.0;
		}
		for l21 in 0..2 {
			self.fRec1[l21 as usize] = 0.0;
		}
		for l22 in 0..2 {
			self.fVec18[l22 as usize] = 0.0;
		}
		for l23 in 0..3 {
			self.fVec19[l23 as usize] = 0.0;
		}
		for l24 in 0..7 {
			self.fVec20[l24 as usize] = 0.0;
		}
		for l25 in 0..15 {
			self.fVec21[l25 as usize] = 0.0;
		}
		for l26 in 0..32 {
			self.fVec22[l26 as usize] = 0.0;
		}
		for l27 in 0..64 {
			self.fVec23[l27 as usize] = 0.0;
		}
		for l28 in 0..128 {
			self.fVec24[l28 as usize] = 0.0;
		}
		for l29 in 0..256 {
			self.fVec25[l29 as usize] = 0.0;
		}
		for l30 in 0..512 {
			self.fVec26[l30 as usize] = 0.0;
		}
		for l31 in 0..1024 {
			self.fVec27[l31 as usize] = 0.0;
		}
		for l32 in 0..2048 {
			self.fVec28[l32 as usize] = 0.0;
		}
		for l33 in 0..4096 {
			self.fVec29[l33 as usize] = 0.0;
		}
		for l34 in 0..8192 {
			self.fVec30[l34 as usize] = 0.0;
		}
		for l35 in 0..2 {
			self.fVec31[l35 as usize] = 0.0;
		}
		for l36 in 0..2 {
			self.fVec32[l36 as usize] = 0.0;
		}
		for l37 in 0..2 {
			self.fVec33[l37 as usize] = 0.0;
		}
		for l38 in 0..2 {
			self.fRec4[l38 as usize] = 0.0;
		}
		for l39 in 0..2 {
			self.fRec5[l39 as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		self.fConst0 = F32::min(1.92e+05, F32::max(1.0, (self.fSampleRate) as F32));
		self.fConst1 = 1.0 / self.fConst0;
		self.fConst2 = 44.1 / self.fConst0;
		self.fConst3 = 1.0 - self.fConst2;
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
		ui_interface.add_horizontal_slider("input_gain", ParamIndex(0), 0.0, -24.0, 24.0, 1.0);
		ui_interface.declare(Some(ParamIndex(1)), "02", "");
		ui_interface.add_horizontal_slider("strength", ParamIndex(1), 1e+02, 0.0, 1e+02, 1.0);
		ui_interface.declare(Some(ParamIndex(2)), "03", "");
		ui_interface.add_horizontal_slider("thresh", ParamIndex(2), -1.0, -3e+01, 0.0, 1.0);
		ui_interface.declare(Some(ParamIndex(3)), "04", "");
		ui_interface.declare(Some(ParamIndex(3)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(3)), "unit", "ms");
		ui_interface.add_horizontal_slider("attack", ParamIndex(3), 3e+01, 0.0, 1e+02, 1.0);
		ui_interface.declare(Some(ParamIndex(4)), "05", "");
		ui_interface.add_horizontal_slider("attack_shape", ParamIndex(4), 2.0, -4.0, 4.0, 0.1);
		ui_interface.declare(Some(ParamIndex(5)), "06", "");
		ui_interface.declare(Some(ParamIndex(5)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(5)), "unit", "ms");
		ui_interface.add_horizontal_slider("release", ParamIndex(5), 42.0, 0.0, 1e+03, 1.0);
		ui_interface.declare(Some(ParamIndex(6)), "07", "");
		ui_interface.add_horizontal_slider("release_shape", ParamIndex(6), -3.0, -4.0, 4.0, 0.1);
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
		let mut fSlow0: F32 = F32::max(self.fConst1, 0.001 * self.fHslider0);
		let mut fSlow1: F32 = self.fConst0 * fSlow0;
		let mut fSlow2: F32 = fSlow1 + 1.0;
		let mut iSlow3: i32 = (F32::floor(fSlow2)) as i32 % 2;
		let mut fSlow4: F32 = self.fHslider1;
		let mut fSlow5: F32 = 0.5 * fSlow4;
		let mut fSlow6: F32 = self.fHslider2;
		let mut fSlow7: F32 = fSlow6 + fSlow5;
		let mut fSlow8: F32 = self.fConst2 * self.fHslider3;
		let mut fSlow9: F32 = 0.01 * self.fHslider4;
		let mut fSlow10: F32 = fSlow6 - fSlow5;
		let mut fSlow11: F32 = 0.5 / F32::max(1.1920929e-07, fSlow4);
		let mut fSlow12: F32 = 0.01 * self.fHslider5;
		let mut iSlow13: i32 = (F32::floor(0.5 * fSlow2)) as i32 % 2;
		let mut iSlow14: i32 = (F32::floor(0.25 * fSlow2)) as i32 % 2;
		let mut iSlow15: i32 = i32::wrapping_add(iSlow3, i32::wrapping_mul(2, iSlow13));
		let mut iSlow16: i32 = (F32::floor(0.125 * fSlow2)) as i32 % 2;
		let mut iSlow17: i32 = i32::wrapping_add(iSlow15, i32::wrapping_mul(4, iSlow14));
		let mut iSlow18: i32 = (F32::floor(0.0625 * fSlow2)) as i32 % 2;
		let mut iSlow19: i32 = i32::wrapping_add(iSlow17, i32::wrapping_mul(8, iSlow16));
		let mut iSlow20: i32 = (F32::floor(0.03125 * fSlow2)) as i32 % 2;
		let mut iSlow21: i32 = i32::wrapping_add(iSlow19, i32::wrapping_mul(16, iSlow18));
		let mut iSlow22: i32 = (F32::floor(0.015625 * fSlow2)) as i32 % 2;
		let mut iSlow23: i32 = i32::wrapping_add(iSlow21, i32::wrapping_mul(32, iSlow20));
		let mut iSlow24: i32 = (F32::floor(0.0078125 * fSlow2)) as i32 % 2;
		let mut iSlow25: i32 = i32::wrapping_add(iSlow23, i32::wrapping_mul(64, iSlow22));
		let mut iSlow26: i32 = (F32::floor(0.00390625 * fSlow2)) as i32 % 2;
		let mut iSlow27: i32 = i32::wrapping_add(iSlow25, i32::wrapping_mul(128, iSlow24));
		let mut iSlow28: i32 = (F32::floor(0.001953125 * fSlow2)) as i32 % 2;
		let mut iSlow29: i32 = i32::wrapping_add(iSlow27, i32::wrapping_mul(256, iSlow26));
		let mut iSlow30: i32 = (F32::floor(0.0009765625 * fSlow2)) as i32 % 2;
		let mut iSlow31: i32 = i32::wrapping_add(iSlow29, i32::wrapping_mul(512, iSlow28));
		let mut iSlow32: i32 = (F32::floor(0.00048828125 * fSlow2)) as i32 % 2;
		let mut iSlow33: i32 = i32::wrapping_add(iSlow31, i32::wrapping_mul(1024, iSlow30));
		let mut iSlow34: i32 = (F32::floor(0.00024414062 * fSlow2)) as i32 % 2;
		let mut iSlow35: i32 = i32::wrapping_add(iSlow33, i32::wrapping_mul(2048, iSlow32));
		let mut fSlow36: F32 = self.fHslider6 + 4.0;
		let mut fSlow37: F32 = self.fHslider7 + 4.0;
		let mut fSlow38: F32 = F32::max(self.fConst1, 0.001 * self.fHslider8);
		let mut iSlow39: i32 = (fSlow1) as i32;
		let zipped_iterators = inputs0.zip(inputs1).zip(outputs0).zip(outputs1);
		for (((input0, input1), output0), output1) in zipped_iterators {
			self.fRec3[0] = fSlow8 + self.fConst3 * self.fRec3[1];
			let mut fTemp9: F32 = F32::powf(1e+01, 0.05 * self.fRec3[0]);
			let mut fTemp10: F32 = *input0 * fTemp9;
			self.fVec0[(self.IOTA0 & 262143) as usize] = fTemp10;
			let mut fTemp11: F32 = F32::abs(fTemp10);
			let mut fTemp12: F32 = *input1 * fTemp9;
			self.fVec1[(self.IOTA0 & 262143) as usize] = fTemp12;
			let mut fTemp13: F32 = F32::abs(fTemp12);
			let mut fTemp14: F32 = F32::max(fTemp11, fTemp13);
			let mut fTemp15: F32 = 2e+01 * F32::log10(F32::max(1.1754944e-38, fTemp11 + fSlow9 * (fTemp14 - fTemp11)));
			let mut iTemp16: i32 = ((fTemp15 > fSlow10) as i32) + ((fTemp15 > fSlow7) as i32);
			let mut fTemp17: F32 = fTemp15 - fSlow6;
			let mut fTemp18: F32 = fSlow12 * F32::max(0.0, if (iTemp16 == 0) as i32 != 0 {0.0} else {if (iTemp16 == 1) as i32 != 0 {fSlow11 * mydsp_faustpower2_f(fSlow5 + fTemp17)} else {fTemp17}});
			self.fVec2[0] = fTemp18;
			let mut fTemp19: F32 = F32::min(-fTemp18, -self.fVec2[1]);
			self.fVec3[0] = fTemp19;
			let mut fTemp20: F32 = F32::min(fTemp19, self.fVec3[2]);
			self.fVec4[0] = fTemp20;
			let mut fTemp21: F32 = F32::min(fTemp20, self.fVec4[4]);
			self.fVec5[0] = fTemp21;
			let mut fTemp22: F32 = F32::min(fTemp21, self.fVec5[8]);
			self.fVec6[(self.IOTA0 & 31) as usize] = fTemp22;
			let mut fTemp23: F32 = F32::min(fTemp22, self.fVec6[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize]);
			self.fVec7[(self.IOTA0 & 63) as usize] = fTemp23;
			let mut fTemp24: F32 = F32::min(fTemp23, self.fVec7[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize]);
			self.fVec8[(self.IOTA0 & 127) as usize] = fTemp24;
			let mut fTemp25: F32 = F32::min(fTemp24, self.fVec8[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize]);
			self.fVec9[(self.IOTA0 & 255) as usize] = fTemp25;
			let mut fTemp26: F32 = F32::min(fTemp25, self.fVec9[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize]);
			self.fVec10[(self.IOTA0 & 511) as usize] = fTemp26;
			let mut fTemp27: F32 = F32::min(fTemp26, self.fVec10[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize]);
			self.fVec11[(self.IOTA0 & 1023) as usize] = fTemp27;
			let mut fTemp28: F32 = F32::min(fTemp27, self.fVec11[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize]);
			self.fVec12[(self.IOTA0 & 2047) as usize] = fTemp28;
			let mut fTemp29: F32 = F32::min(fTemp28, self.fVec12[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			self.fVec13[(self.IOTA0 & 4095) as usize] = fTemp29;
			self.fVec14[(self.IOTA0 & 8191) as usize] = F32::min(fTemp29, self.fVec13[((i32::wrapping_sub(self.IOTA0, 2048)) & 4095) as usize]);
			let mut fTemp30: F32 = F32::powf(1e+01, 0.05 * F32::min(F32::min(F32::min(F32::min(F32::min(F32::min(F32::min(F32::min(F32::min(F32::min(F32::min(F32::min(if iSlow3 != 0 {-fTemp18} else {3.4028235e+38}, if iSlow13 != 0 {self.fVec3[iSlow3 as usize]} else {3.4028235e+38}), if iSlow14 != 0 {self.fVec4[iSlow15 as usize]} else {3.4028235e+38}), if iSlow16 != 0 {self.fVec5[iSlow17 as usize]} else {3.4028235e+38}), if iSlow18 != 0 {self.fVec6[((i32::wrapping_sub(self.IOTA0, iSlow19)) & 31) as usize]} else {3.4028235e+38}), if iSlow20 != 0 {self.fVec7[((i32::wrapping_sub(self.IOTA0, iSlow21)) & 63) as usize]} else {3.4028235e+38}), if iSlow22 != 0 {self.fVec8[((i32::wrapping_sub(self.IOTA0, iSlow23)) & 127) as usize]} else {3.4028235e+38}), if iSlow24 != 0 {self.fVec9[((i32::wrapping_sub(self.IOTA0, iSlow25)) & 255) as usize]} else {3.4028235e+38}), if iSlow26 != 0 {self.fVec10[((i32::wrapping_sub(self.IOTA0, iSlow27)) & 511) as usize]} else {3.4028235e+38}), if iSlow28 != 0 {self.fVec11[((i32::wrapping_sub(self.IOTA0, iSlow29)) & 1023) as usize]} else {3.4028235e+38}), if iSlow30 != 0 {self.fVec12[((i32::wrapping_sub(self.IOTA0, iSlow31)) & 2047) as usize]} else {3.4028235e+38}), if iSlow32 != 0 {self.fVec13[((i32::wrapping_sub(self.IOTA0, iSlow33)) & 4095) as usize]} else {3.4028235e+38}), if iSlow34 != 0 {self.fVec14[((i32::wrapping_sub(self.IOTA0, iSlow35)) & 8191) as usize]} else {3.4028235e+38})) - self.fRec1[1];
			self.fVec15[0] = fTemp30;
			let mut iTemp31: i32 = (fTemp30 > 0.0) as i32;
			let mut fTemp32: F32 = if iTemp31 != 0 {fSlow37} else {fSlow36};
			self.fVec16[0] = fTemp32;
			let mut fTemp33: F32 = 0.8888889 * fTemp32;
			let mut iTemp34: i32 = (fTemp33) as i32;
			let mut fTemp35: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp34, 294912)) as usize] };
			let mut fTemp36: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp34, 294903)) as usize] };
			let mut fTemp37: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp34, 294904)) as usize] } - fTemp36;
			let mut fTemp38: F32 = fTemp33 - (iTemp34) as F32;
			let mut fTemp39: F32 = 0.5 * (fTemp35 - (fTemp36 + fTemp38 * (fTemp37 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp34, 294913)) as usize] } - fTemp35))));
			let mut fTemp40: F32 = fSlow0 * ((fTemp30 < 0.0) as i32) as u32 as F32 + fSlow38 * (iTemp31) as F32;
			self.fVec17[0] = fTemp40;
			let mut fTemp41: F32 = self.fConst1 / fTemp40;
			let mut fTemp42: F32 = 65535.0 * (fTemp41 + 0.5);
			let mut iTemp43: i32 = (fTemp42) as i32;
			let mut fTemp44: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp34, i32::wrapping_mul(9, i32::wrapping_add(iTemp43, 1))), 589823))) as usize] };
			let mut iTemp45: i32 = i32::wrapping_add(iTemp34, i32::wrapping_mul(9, iTemp43));
			let mut fTemp46: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp45, 589823))) as usize] };
			let mut fTemp47: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp45, 1), 589823))) as usize] } - fTemp46;
			let mut fTemp48: F32 = 65535.0 * self.fRec0[1];
			let mut iTemp49: i32 = (fTemp48) as i32;
			let mut fTemp50: F32 = 0.8888889 * self.fVec16[1];
			let mut iTemp51: i32 = (fTemp50) as i32;
			let mut fTemp52: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp51, i32::wrapping_mul(9, i32::wrapping_add(iTemp49, 1))), 589823))) as usize] };
			let mut iTemp53: i32 = i32::wrapping_add(i32::wrapping_mul(9, iTemp49), iTemp51);
			let mut fTemp54: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp53, 589823))) as usize] };
			let mut fTemp55: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp53, 1), 589823))) as usize] } - fTemp54;
			let mut fTemp56: F32 = fTemp50 - (iTemp51) as F32;
			let mut fTemp57: F32 = 65535.0 * (self.fRec0[1] + fTemp41);
			let mut iTemp58: i32 = (fTemp57) as i32;
			let mut fTemp59: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp34, i32::wrapping_mul(9, i32::wrapping_add(iTemp58, 1))), 589823))) as usize] };
			let mut iTemp60: i32 = i32::wrapping_add(iTemp34, i32::wrapping_mul(9, iTemp58));
			let mut fTemp61: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp60, 589823))) as usize] };
			let mut fTemp62: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp60, 1), 589823))) as usize] } - fTemp61;
			let mut fTemp63: F32 = 65535.0 * (self.fRec0[1] + self.fConst1 * (1.0 / fTemp40 + 1.0 / self.fVec17[1]));
			let mut iTemp64: i32 = (fTemp63) as i32;
			let mut fTemp65: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp34, i32::wrapping_mul(9, i32::wrapping_add(iTemp64, 1))), 589823))) as usize] };
			let mut iTemp66: i32 = i32::wrapping_add(i32::wrapping_mul(9, iTemp64), iTemp34);
			let mut fTemp67: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp66, 589823))) as usize] };
			let mut fTemp68: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp66, 1), 589823))) as usize] } - fTemp67;
			let mut fTemp69: F32 = (fTemp38 * (fTemp68 - fTemp62) + fTemp67 + (fTemp63 - (iTemp64) as F32) * (fTemp65 - (fTemp67 + fTemp38 * (fTemp68 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp66, 10), 589823))) as usize] } - fTemp65)))) - (fTemp61 + (fTemp57 - (iTemp58) as F32) * (fTemp59 - (fTemp61 + fTemp38 * (fTemp62 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp60, 10), 589823))) as usize] } - fTemp59)))))) * self.fVec15[1] / (fTemp30 * (1.0 - (fTemp54 + fTemp56 * fTemp55 + (fTemp48 - (iTemp49) as F32) * (fTemp52 - (fTemp54 + fTemp56 * (fTemp55 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp53, 10), 589823))) as usize] } - fTemp52)))))));
			let mut iTemp70: i32 = (fTemp69 > ((fTemp38 * (fTemp47 - fTemp37) + fTemp46 + (fTemp42 - (iTemp43) as F32) * (fTemp44 - (fTemp46 + fTemp38 * (fTemp47 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp45, 10), 589823))) as usize] } - fTemp44)))) - (fTemp36 + fTemp39)) / (1.0 - (fTemp36 + fTemp38 * fTemp37 + fTemp39)))) as i32;
			let mut fTemp71: F32 = if iTemp70 != 0 {1.0} else {0.5};
			let mut fTemp72: F32 = if iTemp70 != 0 {0.5} else {0.0};
			let mut fTemp73: F32 = fTemp72 + fTemp71;
			let mut fTemp74: F32 = 32767.5 * fTemp73;
			let mut iTemp75: i32 = (fTemp74) as i32;
			let mut fTemp76: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp34, i32::wrapping_mul(9, i32::wrapping_add(iTemp75, 1)))) as usize] };
			let mut iTemp77: i32 = i32::wrapping_add(iTemp34, i32::wrapping_mul(9, iTemp75));
			let mut fTemp78: F32 = unsafe { ftbl0mydspSIG0[iTemp77 as usize] };
			let mut fTemp79: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp77, 1)) as usize] } - fTemp78;
			let mut fTemp80: F32 = (fTemp74 - (iTemp75) as F32) * (fTemp76 - (fTemp78 + fTemp38 * (fTemp79 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp77, 10)) as usize] } - fTemp76))));
			let mut fTemp81: F32 = 0.5 * fTemp73;
			let mut fTemp82: F32 = 65535.0 * (fTemp41 + fTemp81);
			let mut iTemp83: i32 = (fTemp82) as i32;
			let mut fTemp84: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp34, i32::wrapping_mul(9, i32::wrapping_add(iTemp83, 1))), 589823))) as usize] };
			let mut iTemp85: i32 = i32::wrapping_add(iTemp34, i32::wrapping_mul(9, iTemp83));
			let mut fTemp86: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp85, 589823))) as usize] };
			let mut fTemp87: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp85, 1), 589823))) as usize] } - fTemp86;
			let mut iTemp88: i32 = (fTemp69 > ((fTemp38 * (fTemp87 - fTemp79) + fTemp86 + (fTemp82 - (iTemp83) as F32) * (fTemp84 - (fTemp86 + fTemp38 * (fTemp87 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp85, 10), 589823))) as usize] } - fTemp84)))) - (fTemp78 + fTemp80)) / (1.0 - (fTemp78 + fTemp38 * fTemp79 + fTemp80)))) as i32;
			let mut fTemp89: F32 = if iTemp88 != 0 {fTemp71} else {fTemp81};
			let mut fTemp90: F32 = if iTemp88 != 0 {fTemp81} else {fTemp72};
			let mut fTemp91: F32 = fTemp90 + fTemp89;
			let mut fTemp92: F32 = 32767.5 * fTemp91;
			let mut iTemp93: i32 = (fTemp92) as i32;
			let mut fTemp94: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp34, i32::wrapping_mul(9, i32::wrapping_add(iTemp93, 1)))) as usize] };
			let mut iTemp95: i32 = i32::wrapping_add(iTemp34, i32::wrapping_mul(9, iTemp93));
			let mut fTemp96: F32 = unsafe { ftbl0mydspSIG0[iTemp95 as usize] };
			let mut fTemp97: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp95, 1)) as usize] } - fTemp96;
			let mut fTemp98: F32 = (fTemp92 - (iTemp93) as F32) * (fTemp94 - (fTemp96 + fTemp38 * (fTemp97 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp95, 10)) as usize] } - fTemp94))));
			let mut fTemp99: F32 = 0.5 * fTemp91;
			let mut fTemp100: F32 = 65535.0 * (fTemp41 + fTemp99);
			let mut iTemp101: i32 = (fTemp100) as i32;
			let mut fTemp102: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp34, i32::wrapping_mul(9, i32::wrapping_add(iTemp101, 1))), 589823))) as usize] };
			let mut iTemp103: i32 = i32::wrapping_add(iTemp34, i32::wrapping_mul(9, iTemp101));
			let mut fTemp104: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp103, 589823))) as usize] };
			let mut fTemp105: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp103, 1), 589823))) as usize] } - fTemp104;
			let mut iTemp106: i32 = (fTemp69 > ((fTemp38 * (fTemp105 - fTemp97) + fTemp104 + (fTemp100 - (iTemp101) as F32) * (fTemp102 - (fTemp104 + fTemp38 * (fTemp105 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp103, 10), 589823))) as usize] } - fTemp102)))) - (fTemp96 + fTemp98)) / (1.0 - (fTemp96 + fTemp38 * fTemp97 + fTemp98)))) as i32;
			let mut fTemp107: F32 = if iTemp106 != 0 {fTemp89} else {fTemp99};
			let mut fTemp108: F32 = if iTemp106 != 0 {fTemp99} else {fTemp90};
			let mut fTemp109: F32 = fTemp108 + fTemp107;
			let mut fTemp110: F32 = 32767.5 * fTemp109;
			let mut iTemp111: i32 = (fTemp110) as i32;
			let mut fTemp112: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp34, i32::wrapping_mul(9, i32::wrapping_add(iTemp111, 1)))) as usize] };
			let mut iTemp113: i32 = i32::wrapping_add(iTemp34, i32::wrapping_mul(9, iTemp111));
			let mut fTemp114: F32 = unsafe { ftbl0mydspSIG0[iTemp113 as usize] };
			let mut fTemp115: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp113, 1)) as usize] } - fTemp114;
			let mut fTemp116: F32 = (fTemp110 - (iTemp111) as F32) * (fTemp112 - (fTemp114 + fTemp38 * (fTemp115 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp113, 10)) as usize] } - fTemp112))));
			let mut fTemp117: F32 = 0.5 * fTemp109;
			let mut fTemp118: F32 = 65535.0 * (fTemp41 + fTemp117);
			let mut iTemp119: i32 = (fTemp118) as i32;
			let mut fTemp120: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp34, i32::wrapping_mul(9, i32::wrapping_add(iTemp119, 1))), 589823))) as usize] };
			let mut iTemp121: i32 = i32::wrapping_add(iTemp34, i32::wrapping_mul(9, iTemp119));
			let mut fTemp122: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp121, 589823))) as usize] };
			let mut fTemp123: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp121, 1), 589823))) as usize] } - fTemp122;
			let mut iTemp124: i32 = (fTemp69 > ((fTemp38 * (fTemp123 - fTemp115) + fTemp122 + (fTemp118 - (iTemp119) as F32) * (fTemp120 - (fTemp122 + fTemp38 * (fTemp123 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp121, 10), 589823))) as usize] } - fTemp120)))) - (fTemp114 + fTemp116)) / (1.0 - (fTemp114 + fTemp38 * fTemp115 + fTemp116)))) as i32;
			let mut fTemp125: F32 = if iTemp124 != 0 {fTemp107} else {fTemp117};
			let mut fTemp126: F32 = if iTemp124 != 0 {fTemp117} else {fTemp108};
			let mut fTemp127: F32 = fTemp126 + fTemp125;
			let mut fTemp128: F32 = 32767.5 * fTemp127;
			let mut iTemp129: i32 = (fTemp128) as i32;
			let mut fTemp130: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp34, i32::wrapping_mul(9, i32::wrapping_add(iTemp129, 1)))) as usize] };
			let mut iTemp131: i32 = i32::wrapping_add(iTemp34, i32::wrapping_mul(9, iTemp129));
			let mut fTemp132: F32 = unsafe { ftbl0mydspSIG0[iTemp131 as usize] };
			let mut fTemp133: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp131, 1)) as usize] } - fTemp132;
			let mut fTemp134: F32 = (fTemp128 - (iTemp129) as F32) * (fTemp130 - (fTemp132 + fTemp38 * (fTemp133 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp131, 10)) as usize] } - fTemp130))));
			let mut fTemp135: F32 = 0.5 * fTemp127;
			let mut fTemp136: F32 = 65535.0 * (fTemp41 + fTemp135);
			let mut iTemp137: i32 = (fTemp136) as i32;
			let mut fTemp138: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp34, i32::wrapping_mul(9, i32::wrapping_add(iTemp137, 1))), 589823))) as usize] };
			let mut iTemp139: i32 = i32::wrapping_add(iTemp34, i32::wrapping_mul(9, iTemp137));
			let mut fTemp140: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp139, 589823))) as usize] };
			let mut fTemp141: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp139, 1), 589823))) as usize] } - fTemp140;
			let mut iTemp142: i32 = (fTemp69 > ((fTemp38 * (fTemp141 - fTemp133) + fTemp140 + (fTemp136 - (iTemp137) as F32) * (fTemp138 - (fTemp140 + fTemp38 * (fTemp141 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp139, 10), 589823))) as usize] } - fTemp138)))) - (fTemp132 + fTemp134)) / (1.0 - (fTemp132 + fTemp38 * fTemp133 + fTemp134)))) as i32;
			let mut fTemp143: F32 = if iTemp142 != 0 {fTemp125} else {fTemp135};
			let mut fTemp144: F32 = if iTemp142 != 0 {fTemp135} else {fTemp126};
			let mut fTemp145: F32 = fTemp144 + fTemp143;
			let mut fTemp146: F32 = 32767.5 * fTemp145;
			let mut iTemp147: i32 = (fTemp146) as i32;
			let mut fTemp148: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp34, i32::wrapping_mul(9, i32::wrapping_add(iTemp147, 1)))) as usize] };
			let mut iTemp149: i32 = i32::wrapping_add(iTemp34, i32::wrapping_mul(9, iTemp147));
			let mut fTemp150: F32 = unsafe { ftbl0mydspSIG0[iTemp149 as usize] };
			let mut fTemp151: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp149, 1)) as usize] } - fTemp150;
			let mut fTemp152: F32 = (fTemp146 - (iTemp147) as F32) * (fTemp148 - (fTemp150 + fTemp38 * (fTemp151 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp149, 10)) as usize] } - fTemp148))));
			let mut fTemp153: F32 = 0.5 * fTemp145;
			let mut fTemp154: F32 = 65535.0 * (fTemp41 + fTemp153);
			let mut iTemp155: i32 = (fTemp154) as i32;
			let mut fTemp156: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp34, i32::wrapping_mul(9, i32::wrapping_add(iTemp155, 1))), 589823))) as usize] };
			let mut iTemp157: i32 = i32::wrapping_add(iTemp34, i32::wrapping_mul(9, iTemp155));
			let mut fTemp158: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp157, 589823))) as usize] };
			let mut fTemp159: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp157, 1), 589823))) as usize] } - fTemp158;
			let mut iTemp160: i32 = (fTemp69 > ((fTemp38 * (fTemp159 - fTemp151) + fTemp158 + (fTemp154 - (iTemp155) as F32) * (fTemp156 - (fTemp158 + fTemp38 * (fTemp159 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp157, 10), 589823))) as usize] } - fTemp156)))) - (fTemp150 + fTemp152)) / (1.0 - (fTemp150 + fTemp38 * fTemp151 + fTemp152)))) as i32;
			let mut fTemp161: F32 = if iTemp160 != 0 {fTemp143} else {fTemp153};
			let mut fTemp162: F32 = if iTemp160 != 0 {fTemp153} else {fTemp144};
			let mut fTemp163: F32 = fTemp162 + fTemp161;
			let mut fTemp164: F32 = 32767.5 * fTemp163;
			let mut iTemp165: i32 = (fTemp164) as i32;
			let mut fTemp166: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp34, i32::wrapping_mul(9, i32::wrapping_add(iTemp165, 1)))) as usize] };
			let mut iTemp167: i32 = i32::wrapping_add(iTemp34, i32::wrapping_mul(9, iTemp165));
			let mut fTemp168: F32 = unsafe { ftbl0mydspSIG0[iTemp167 as usize] };
			let mut fTemp169: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp167, 1)) as usize] } - fTemp168;
			let mut fTemp170: F32 = (fTemp164 - (iTemp165) as F32) * (fTemp166 - (fTemp168 + fTemp38 * (fTemp169 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp167, 10)) as usize] } - fTemp166))));
			let mut fTemp171: F32 = 0.5 * fTemp163;
			let mut fTemp172: F32 = 65535.0 * (fTemp41 + fTemp171);
			let mut iTemp173: i32 = (fTemp172) as i32;
			let mut fTemp174: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp34, i32::wrapping_mul(9, i32::wrapping_add(iTemp173, 1))), 589823))) as usize] };
			let mut iTemp175: i32 = i32::wrapping_add(iTemp34, i32::wrapping_mul(9, iTemp173));
			let mut fTemp176: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp175, 589823))) as usize] };
			let mut fTemp177: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp175, 1), 589823))) as usize] } - fTemp176;
			let mut iTemp178: i32 = (fTemp69 > ((fTemp38 * (fTemp177 - fTemp169) + fTemp176 + (fTemp172 - (iTemp173) as F32) * (fTemp174 - (fTemp176 + fTemp38 * (fTemp177 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp175, 10), 589823))) as usize] } - fTemp174)))) - (fTemp168 + fTemp170)) / (1.0 - (fTemp168 + fTemp38 * fTemp169 + fTemp170)))) as i32;
			let mut fTemp179: F32 = if iTemp178 != 0 {fTemp161} else {fTemp171};
			let mut fTemp180: F32 = if iTemp178 != 0 {fTemp171} else {fTemp162};
			let mut fTemp181: F32 = fTemp180 + fTemp179;
			let mut fTemp182: F32 = 32767.5 * fTemp181;
			let mut iTemp183: i32 = (fTemp182) as i32;
			let mut fTemp184: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp34, i32::wrapping_mul(9, i32::wrapping_add(iTemp183, 1)))) as usize] };
			let mut iTemp185: i32 = i32::wrapping_add(iTemp34, i32::wrapping_mul(9, iTemp183));
			let mut fTemp186: F32 = unsafe { ftbl0mydspSIG0[iTemp185 as usize] };
			let mut fTemp187: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp185, 1)) as usize] } - fTemp186;
			let mut fTemp188: F32 = (fTemp182 - (iTemp183) as F32) * (fTemp184 - (fTemp186 + fTemp38 * (fTemp187 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp185, 10)) as usize] } - fTemp184))));
			let mut fTemp189: F32 = 0.5 * fTemp181;
			let mut fTemp190: F32 = 65535.0 * (fTemp41 + fTemp189);
			let mut iTemp191: i32 = (fTemp190) as i32;
			let mut fTemp192: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp34, i32::wrapping_mul(9, i32::wrapping_add(iTemp191, 1))), 589823))) as usize] };
			let mut iTemp193: i32 = i32::wrapping_add(iTemp34, i32::wrapping_mul(9, iTemp191));
			let mut fTemp194: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp193, 589823))) as usize] };
			let mut fTemp195: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp193, 1), 589823))) as usize] } - fTemp194;
			let mut iTemp196: i32 = (fTemp69 > ((fTemp38 * (fTemp195 - fTemp187) + fTemp194 + (fTemp190 - (iTemp191) as F32) * (fTemp192 - (fTemp194 + fTemp38 * (fTemp195 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp193, 10), 589823))) as usize] } - fTemp192)))) - (fTemp186 + fTemp188)) / (1.0 - (fTemp186 + fTemp38 * fTemp187 + fTemp188)))) as i32;
			let mut fTemp197: F32 = if iTemp196 != 0 {fTemp179} else {fTemp189};
			let mut fTemp198: F32 = if iTemp196 != 0 {fTemp189} else {fTemp180};
			let mut fTemp199: F32 = fTemp198 + fTemp197;
			let mut fTemp200: F32 = 32767.5 * fTemp199;
			let mut iTemp201: i32 = (fTemp200) as i32;
			let mut fTemp202: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp34, i32::wrapping_mul(9, i32::wrapping_add(iTemp201, 1)))) as usize] };
			let mut iTemp203: i32 = i32::wrapping_add(iTemp34, i32::wrapping_mul(9, iTemp201));
			let mut fTemp204: F32 = unsafe { ftbl0mydspSIG0[iTemp203 as usize] };
			let mut fTemp205: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp203, 1)) as usize] } - fTemp204;
			let mut fTemp206: F32 = (fTemp200 - (iTemp201) as F32) * (fTemp202 - (fTemp204 + fTemp38 * (fTemp205 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp203, 10)) as usize] } - fTemp202))));
			let mut fTemp207: F32 = 0.5 * fTemp199;
			let mut fTemp208: F32 = 65535.0 * (fTemp41 + fTemp207);
			let mut iTemp209: i32 = (fTemp208) as i32;
			let mut fTemp210: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp34, i32::wrapping_mul(9, i32::wrapping_add(iTemp209, 1))), 589823))) as usize] };
			let mut iTemp211: i32 = i32::wrapping_add(iTemp34, i32::wrapping_mul(9, iTemp209));
			let mut fTemp212: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp211, 589823))) as usize] };
			let mut fTemp213: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp211, 1), 589823))) as usize] } - fTemp212;
			let mut iTemp214: i32 = (fTemp69 > ((fTemp38 * (fTemp213 - fTemp205) + fTemp212 + (fTemp208 - (iTemp209) as F32) * (fTemp210 - (fTemp212 + fTemp38 * (fTemp213 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp211, 10), 589823))) as usize] } - fTemp210)))) - (fTemp204 + fTemp206)) / (1.0 - (fTemp204 + fTemp38 * fTemp205 + fTemp206)))) as i32;
			let mut fTemp215: F32 = if iTemp214 != 0 {fTemp197} else {fTemp207};
			let mut fTemp216: F32 = if iTemp214 != 0 {fTemp207} else {fTemp198};
			let mut fTemp217: F32 = fTemp216 + fTemp215;
			let mut fTemp218: F32 = 32767.5 * fTemp217;
			let mut iTemp219: i32 = (fTemp218) as i32;
			let mut fTemp220: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp34, i32::wrapping_mul(9, i32::wrapping_add(iTemp219, 1)))) as usize] };
			let mut iTemp221: i32 = i32::wrapping_add(iTemp34, i32::wrapping_mul(9, iTemp219));
			let mut fTemp222: F32 = unsafe { ftbl0mydspSIG0[iTemp221 as usize] };
			let mut fTemp223: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp221, 1)) as usize] } - fTemp222;
			let mut fTemp224: F32 = (fTemp218 - (iTemp219) as F32) * (fTemp220 - (fTemp222 + fTemp38 * (fTemp223 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp221, 10)) as usize] } - fTemp220))));
			let mut fTemp225: F32 = 0.5 * fTemp217;
			let mut fTemp226: F32 = 65535.0 * (fTemp41 + fTemp225);
			let mut iTemp227: i32 = (fTemp226) as i32;
			let mut fTemp228: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp34, i32::wrapping_mul(9, i32::wrapping_add(iTemp227, 1))), 589823))) as usize] };
			let mut iTemp229: i32 = i32::wrapping_add(iTemp34, i32::wrapping_mul(9, iTemp227));
			let mut fTemp230: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp229, 589823))) as usize] };
			let mut fTemp231: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp229, 1), 589823))) as usize] } - fTemp230;
			let mut iTemp232: i32 = (fTemp69 > ((fTemp38 * (fTemp231 - fTemp223) + fTemp230 + (fTemp226 - (iTemp227) as F32) * (fTemp228 - (fTemp230 + fTemp38 * (fTemp231 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp229, 10), 589823))) as usize] } - fTemp228)))) - (fTemp222 + fTemp224)) / (1.0 - (fTemp222 + fTemp38 * fTemp223 + fTemp224)))) as i32;
			let mut fTemp233: F32 = if iTemp232 != 0 {fTemp215} else {fTemp225};
			let mut fTemp234: F32 = if iTemp232 != 0 {fTemp225} else {fTemp216};
			let mut fTemp235: F32 = fTemp234 + fTemp233;
			let mut fTemp236: F32 = 32767.5 * fTemp235;
			let mut iTemp237: i32 = (fTemp236) as i32;
			let mut fTemp238: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp34, i32::wrapping_mul(9, i32::wrapping_add(iTemp237, 1)))) as usize] };
			let mut iTemp239: i32 = i32::wrapping_add(iTemp34, i32::wrapping_mul(9, iTemp237));
			let mut fTemp240: F32 = unsafe { ftbl0mydspSIG0[iTemp239 as usize] };
			let mut fTemp241: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp239, 1)) as usize] } - fTemp240;
			let mut fTemp242: F32 = (fTemp236 - (iTemp237) as F32) * (fTemp238 - (fTemp240 + fTemp38 * (fTemp241 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp239, 10)) as usize] } - fTemp238))));
			let mut fTemp243: F32 = 0.5 * fTemp235;
			let mut fTemp244: F32 = 65535.0 * (fTemp41 + fTemp243);
			let mut iTemp245: i32 = (fTemp244) as i32;
			let mut fTemp246: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp34, i32::wrapping_mul(9, i32::wrapping_add(iTemp245, 1))), 589823))) as usize] };
			let mut iTemp247: i32 = i32::wrapping_add(iTemp34, i32::wrapping_mul(9, iTemp245));
			let mut fTemp248: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp247, 589823))) as usize] };
			let mut fTemp249: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp247, 1), 589823))) as usize] } - fTemp248;
			let mut iTemp250: i32 = (fTemp69 > ((fTemp38 * (fTemp249 - fTemp241) + fTemp248 + (fTemp244 - (iTemp245) as F32) * (fTemp246 - (fTemp248 + fTemp38 * (fTemp249 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp247, 10), 589823))) as usize] } - fTemp246)))) - (fTemp240 + fTemp242)) / (1.0 - (fTemp240 + fTemp38 * fTemp241 + fTemp242)))) as i32;
			let mut fTemp251: F32 = if iTemp250 != 0 {fTemp233} else {fTemp243};
			let mut fTemp252: F32 = if iTemp250 != 0 {fTemp243} else {fTemp234};
			let mut fTemp253: F32 = fTemp252 + fTemp251;
			let mut fTemp254: F32 = 32767.5 * fTemp253;
			let mut iTemp255: i32 = (fTemp254) as i32;
			let mut fTemp256: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp34, i32::wrapping_mul(9, i32::wrapping_add(iTemp255, 1)))) as usize] };
			let mut iTemp257: i32 = i32::wrapping_add(iTemp34, i32::wrapping_mul(9, iTemp255));
			let mut fTemp258: F32 = unsafe { ftbl0mydspSIG0[iTemp257 as usize] };
			let mut fTemp259: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp257, 1)) as usize] } - fTemp258;
			let mut fTemp260: F32 = (fTemp254 - (iTemp255) as F32) * (fTemp256 - (fTemp258 + fTemp38 * (fTemp259 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp257, 10)) as usize] } - fTemp256))));
			let mut fTemp261: F32 = 0.5 * fTemp253;
			let mut fTemp262: F32 = 65535.0 * (fTemp41 + fTemp261);
			let mut iTemp263: i32 = (fTemp262) as i32;
			let mut fTemp264: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp34, i32::wrapping_mul(9, i32::wrapping_add(iTemp263, 1))), 589823))) as usize] };
			let mut iTemp265: i32 = i32::wrapping_add(iTemp34, i32::wrapping_mul(9, iTemp263));
			let mut fTemp266: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp265, 589823))) as usize] };
			let mut fTemp267: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp265, 1), 589823))) as usize] } - fTemp266;
			let mut iTemp268: i32 = (fTemp69 > ((fTemp38 * (fTemp267 - fTemp259) + fTemp266 + (fTemp262 - (iTemp263) as F32) * (fTemp264 - (fTemp266 + fTemp38 * (fTemp267 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp265, 10), 589823))) as usize] } - fTemp264)))) - (fTemp258 + fTemp260)) / (1.0 - (fTemp258 + fTemp38 * fTemp259 + fTemp260)))) as i32;
			let mut fTemp269: F32 = if iTemp268 != 0 {fTemp251} else {fTemp261};
			let mut fTemp270: F32 = if iTemp268 != 0 {fTemp261} else {fTemp252};
			let mut fTemp271: F32 = fTemp270 + fTemp269;
			let mut fTemp272: F32 = 32767.5 * fTemp271;
			let mut iTemp273: i32 = (fTemp272) as i32;
			let mut fTemp274: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp34, i32::wrapping_mul(9, i32::wrapping_add(iTemp273, 1)))) as usize] };
			let mut iTemp275: i32 = i32::wrapping_add(iTemp34, i32::wrapping_mul(9, iTemp273));
			let mut fTemp276: F32 = unsafe { ftbl0mydspSIG0[iTemp275 as usize] };
			let mut fTemp277: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp275, 1)) as usize] } - fTemp276;
			let mut fTemp278: F32 = (fTemp272 - (iTemp273) as F32) * (fTemp274 - (fTemp276 + fTemp38 * (fTemp277 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp275, 10)) as usize] } - fTemp274))));
			let mut fTemp279: F32 = 0.5 * fTemp271;
			let mut fTemp280: F32 = 65535.0 * (fTemp41 + fTemp279);
			let mut iTemp281: i32 = (fTemp280) as i32;
			let mut fTemp282: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp34, i32::wrapping_mul(9, i32::wrapping_add(iTemp281, 1))), 589823))) as usize] };
			let mut iTemp283: i32 = i32::wrapping_add(iTemp34, i32::wrapping_mul(9, iTemp281));
			let mut fTemp284: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp283, 589823))) as usize] };
			let mut fTemp285: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp283, 1), 589823))) as usize] } - fTemp284;
			let mut iTemp286: i32 = (fTemp69 > ((fTemp38 * (fTemp285 - fTemp277) + fTemp284 + (fTemp280 - (iTemp281) as F32) * (fTemp282 - (fTemp284 + fTemp38 * (fTemp285 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp283, 10), 589823))) as usize] } - fTemp282)))) - (fTemp276 + fTemp278)) / (1.0 - (fTemp276 + fTemp38 * fTemp277 + fTemp278)))) as i32;
			let mut fTemp287: F32 = if iTemp286 != 0 {fTemp269} else {fTemp279};
			let mut fTemp288: F32 = if iTemp286 != 0 {fTemp279} else {fTemp270};
			let mut fTemp289: F32 = fTemp288 + fTemp287;
			let mut fTemp290: F32 = 32767.5 * fTemp289;
			let mut iTemp291: i32 = (fTemp290) as i32;
			let mut fTemp292: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp34, i32::wrapping_mul(9, i32::wrapping_add(iTemp291, 1)))) as usize] };
			let mut iTemp293: i32 = i32::wrapping_add(iTemp34, i32::wrapping_mul(9, iTemp291));
			let mut fTemp294: F32 = unsafe { ftbl0mydspSIG0[iTemp293 as usize] };
			let mut fTemp295: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp293, 1)) as usize] } - fTemp294;
			let mut fTemp296: F32 = (fTemp290 - (iTemp291) as F32) * (fTemp292 - (fTemp294 + fTemp38 * (fTemp295 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp293, 10)) as usize] } - fTemp292))));
			let mut fTemp297: F32 = 0.5 * fTemp289;
			let mut fTemp298: F32 = 65535.0 * (fTemp41 + fTemp297);
			let mut iTemp299: i32 = (fTemp298) as i32;
			let mut fTemp300: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp34, i32::wrapping_mul(9, i32::wrapping_add(iTemp299, 1))), 589823))) as usize] };
			let mut iTemp301: i32 = i32::wrapping_add(iTemp34, i32::wrapping_mul(9, iTemp299));
			let mut fTemp302: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp301, 589823))) as usize] };
			let mut fTemp303: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp301, 1), 589823))) as usize] } - fTemp302;
			let mut iTemp304: i32 = (fTemp69 > ((fTemp38 * (fTemp303 - fTemp295) + fTemp302 + (fTemp298 - (iTemp299) as F32) * (fTemp300 - (fTemp302 + fTemp38 * (fTemp303 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp301, 10), 589823))) as usize] } - fTemp300)))) - (fTemp294 + fTemp296)) / (1.0 - (fTemp294 + fTemp38 * fTemp295 + fTemp296)))) as i32;
			let mut fTemp305: F32 = if iTemp304 != 0 {fTemp287} else {fTemp297};
			let mut fTemp306: F32 = if iTemp304 != 0 {fTemp297} else {fTemp288};
			let mut fTemp307: F32 = fTemp306 + fTemp305;
			let mut fTemp308: F32 = 32767.5 * fTemp307;
			let mut iTemp309: i32 = (fTemp308) as i32;
			let mut fTemp310: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp34, i32::wrapping_mul(9, i32::wrapping_add(iTemp309, 1)))) as usize] };
			let mut iTemp311: i32 = i32::wrapping_add(iTemp34, i32::wrapping_mul(9, iTemp309));
			let mut fTemp312: F32 = unsafe { ftbl0mydspSIG0[iTemp311 as usize] };
			let mut fTemp313: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp311, 1)) as usize] } - fTemp312;
			let mut fTemp314: F32 = (fTemp308 - (iTemp309) as F32) * (fTemp310 - (fTemp312 + fTemp38 * (fTemp313 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp311, 10)) as usize] } - fTemp310))));
			let mut fTemp315: F32 = 0.5 * fTemp307;
			let mut fTemp316: F32 = 65535.0 * (fTemp41 + fTemp315);
			let mut iTemp317: i32 = (fTemp316) as i32;
			let mut fTemp318: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp34, i32::wrapping_mul(9, i32::wrapping_add(iTemp317, 1))), 589823))) as usize] };
			let mut iTemp319: i32 = i32::wrapping_add(iTemp34, i32::wrapping_mul(9, iTemp317));
			let mut fTemp320: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp319, 589823))) as usize] };
			let mut fTemp321: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp319, 1), 589823))) as usize] } - fTemp320;
			let mut iTemp322: i32 = (fTemp69 > ((fTemp38 * (fTemp321 - fTemp313) + fTemp320 + (fTemp316 - (iTemp317) as F32) * (fTemp318 - (fTemp320 + fTemp38 * (fTemp321 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp319, 10), 589823))) as usize] } - fTemp318)))) - (fTemp312 + fTemp314)) / (1.0 - (fTemp312 + fTemp38 * fTemp313 + fTemp314)))) as i32;
			let mut fTemp323: F32 = if iTemp322 != 0 {fTemp305} else {fTemp315};
			let mut fTemp324: F32 = if iTemp322 != 0 {fTemp315} else {fTemp306};
			let mut fTemp325: F32 = fTemp324 + fTemp323;
			let mut fTemp326: F32 = 32767.5 * fTemp325;
			let mut iTemp327: i32 = (fTemp326) as i32;
			let mut fTemp328: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp34, i32::wrapping_mul(9, i32::wrapping_add(iTemp327, 1)))) as usize] };
			let mut iTemp329: i32 = i32::wrapping_add(iTemp34, i32::wrapping_mul(9, iTemp327));
			let mut fTemp330: F32 = unsafe { ftbl0mydspSIG0[iTemp329 as usize] };
			let mut fTemp331: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp329, 1)) as usize] } - fTemp330;
			let mut fTemp332: F32 = (fTemp326 - (iTemp327) as F32) * (fTemp328 - (fTemp330 + fTemp38 * (fTemp331 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp329, 10)) as usize] } - fTemp328))));
			let mut fTemp333: F32 = 0.5 * fTemp325;
			let mut fTemp334: F32 = 65535.0 * (fTemp41 + fTemp333);
			let mut iTemp335: i32 = (fTemp334) as i32;
			let mut fTemp336: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp34, i32::wrapping_mul(9, i32::wrapping_add(iTemp335, 1))), 589823))) as usize] };
			let mut iTemp337: i32 = i32::wrapping_add(iTemp34, i32::wrapping_mul(9, iTemp335));
			let mut fTemp338: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp337, 589823))) as usize] };
			let mut fTemp339: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp337, 1), 589823))) as usize] } - fTemp338;
			let mut iTemp340: i32 = (fTemp69 > ((fTemp38 * (fTemp339 - fTemp331) + fTemp338 + (fTemp334 - (iTemp335) as F32) * (fTemp336 - (fTemp338 + fTemp38 * (fTemp339 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp337, 10), 589823))) as usize] } - fTemp336)))) - (fTemp330 + fTemp332)) / (1.0 - (fTemp330 + fTemp38 * fTemp331 + fTemp332)))) as i32;
			let mut fTemp341: F32 = F32::min(1.0, F32::max(0.0, 0.5 * (if iTemp340 != 0 {fTemp333} else {fTemp324} + if iTemp340 != 0 {fTemp323} else {fTemp333})));
			self.fRec0[0] = fTemp341;
			let mut fTemp342: F32 = 65535.0 * fTemp341;
			let mut iTemp343: i32 = (fTemp342) as i32;
			let mut fTemp344: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp34, i32::wrapping_mul(9, i32::wrapping_add(iTemp343, 1)))) as usize] };
			let mut iTemp345: i32 = i32::wrapping_add(iTemp34, i32::wrapping_mul(9, iTemp343));
			let mut fTemp346: F32 = unsafe { ftbl0mydspSIG0[iTemp345 as usize] };
			let mut fTemp347: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp345, 1)) as usize] } - fTemp346;
			let mut fTemp348: F32 = (fTemp342 - (iTemp343) as F32) * (fTemp344 - (fTemp346 + fTemp38 * (fTemp347 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp345, 10)) as usize] } - fTemp344))));
			let mut fTemp349: F32 = 65535.0 * (fTemp41 + fTemp341);
			let mut iTemp350: i32 = (fTemp349) as i32;
			let mut fTemp351: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp34, i32::wrapping_mul(9, i32::wrapping_add(iTemp350, 1))), 589823))) as usize] };
			let mut iTemp352: i32 = i32::wrapping_add(iTemp34, i32::wrapping_mul(9, iTemp350));
			let mut fTemp353: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp352, 589823))) as usize] };
			let mut fTemp354: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp352, 1), 589823))) as usize] } - fTemp353;
			let mut fTemp355: F32 = fTemp30 * (fTemp38 * (fTemp354 - fTemp347) + fTemp353 + (fTemp349 - (iTemp350) as F32) * (fTemp351 - (fTemp353 + fTemp38 * (fTemp354 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp352, 10), 589823))) as usize] } - fTemp351)))) - (fTemp346 + fTemp348)) / (1.0 - (fTemp346 + fTemp38 * fTemp347 + fTemp348));
			self.fRec1[0] = self.fRec1[1] + if iTemp31 != 0 {F32::min(fTemp30, fTemp355)} else {F32::max(fTemp30, fTemp355)};
			self.fHbargraph0 = F32::min(0.0, F32::max(-24.0, 2e+01 * F32::log10(F32::max(1.1754944e-38, self.fRec1[0]))));
			*output0 = self.fVec0[((i32::wrapping_sub(self.IOTA0, iSlow39)) & 262143) as usize] * self.fRec1[0];
			let mut fTemp356: F32 = 2e+01 * F32::log10(F32::max(1.1754944e-38, fTemp13 + fSlow9 * (fTemp14 - fTemp13)));
			let mut iTemp357: i32 = ((fTemp356 > fSlow10) as i32) + ((fTemp356 > fSlow7) as i32);
			let mut fTemp358: F32 = fTemp356 - fSlow6;
			let mut fTemp359: F32 = fSlow12 * F32::max(0.0, if (iTemp357 == 0) as i32 != 0 {0.0} else {if (iTemp357 == 1) as i32 != 0 {fSlow11 * mydsp_faustpower2_f(fSlow5 + fTemp358)} else {fTemp358}});
			self.fVec18[0] = fTemp359;
			let mut fTemp360: F32 = F32::min(-fTemp359, -self.fVec18[1]);
			self.fVec19[0] = fTemp360;
			let mut fTemp361: F32 = F32::min(fTemp360, self.fVec19[2]);
			self.fVec20[0] = fTemp361;
			let mut fTemp362: F32 = F32::min(fTemp361, self.fVec20[4]);
			self.fVec21[0] = fTemp362;
			let mut fTemp363: F32 = F32::min(fTemp362, self.fVec21[8]);
			self.fVec22[(self.IOTA0 & 31) as usize] = fTemp363;
			let mut fTemp364: F32 = F32::min(fTemp363, self.fVec22[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize]);
			self.fVec23[(self.IOTA0 & 63) as usize] = fTemp364;
			let mut fTemp365: F32 = F32::min(fTemp364, self.fVec23[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize]);
			self.fVec24[(self.IOTA0 & 127) as usize] = fTemp365;
			let mut fTemp366: F32 = F32::min(fTemp365, self.fVec24[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize]);
			self.fVec25[(self.IOTA0 & 255) as usize] = fTemp366;
			let mut fTemp367: F32 = F32::min(fTemp366, self.fVec25[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize]);
			self.fVec26[(self.IOTA0 & 511) as usize] = fTemp367;
			let mut fTemp368: F32 = F32::min(fTemp367, self.fVec26[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize]);
			self.fVec27[(self.IOTA0 & 1023) as usize] = fTemp368;
			let mut fTemp369: F32 = F32::min(fTemp368, self.fVec27[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize]);
			self.fVec28[(self.IOTA0 & 2047) as usize] = fTemp369;
			let mut fTemp370: F32 = F32::min(fTemp369, self.fVec28[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			self.fVec29[(self.IOTA0 & 4095) as usize] = fTemp370;
			self.fVec30[(self.IOTA0 & 8191) as usize] = F32::min(fTemp370, self.fVec29[((i32::wrapping_sub(self.IOTA0, 2048)) & 4095) as usize]);
			let mut fTemp371: F32 = F32::powf(1e+01, 0.05 * F32::min(F32::min(F32::min(F32::min(F32::min(F32::min(F32::min(F32::min(F32::min(F32::min(F32::min(F32::min(if iSlow3 != 0 {-fTemp359} else {3.4028235e+38}, if iSlow13 != 0 {self.fVec19[iSlow3 as usize]} else {3.4028235e+38}), if iSlow14 != 0 {self.fVec20[iSlow15 as usize]} else {3.4028235e+38}), if iSlow16 != 0 {self.fVec21[iSlow17 as usize]} else {3.4028235e+38}), if iSlow18 != 0 {self.fVec22[((i32::wrapping_sub(self.IOTA0, iSlow19)) & 31) as usize]} else {3.4028235e+38}), if iSlow20 != 0 {self.fVec23[((i32::wrapping_sub(self.IOTA0, iSlow21)) & 63) as usize]} else {3.4028235e+38}), if iSlow22 != 0 {self.fVec24[((i32::wrapping_sub(self.IOTA0, iSlow23)) & 127) as usize]} else {3.4028235e+38}), if iSlow24 != 0 {self.fVec25[((i32::wrapping_sub(self.IOTA0, iSlow25)) & 255) as usize]} else {3.4028235e+38}), if iSlow26 != 0 {self.fVec26[((i32::wrapping_sub(self.IOTA0, iSlow27)) & 511) as usize]} else {3.4028235e+38}), if iSlow28 != 0 {self.fVec27[((i32::wrapping_sub(self.IOTA0, iSlow29)) & 1023) as usize]} else {3.4028235e+38}), if iSlow30 != 0 {self.fVec28[((i32::wrapping_sub(self.IOTA0, iSlow31)) & 2047) as usize]} else {3.4028235e+38}), if iSlow32 != 0 {self.fVec29[((i32::wrapping_sub(self.IOTA0, iSlow33)) & 4095) as usize]} else {3.4028235e+38}), if iSlow34 != 0 {self.fVec30[((i32::wrapping_sub(self.IOTA0, iSlow35)) & 8191) as usize]} else {3.4028235e+38})) - self.fRec5[1];
			self.fVec31[0] = fTemp371;
			let mut iTemp372: i32 = (fTemp371 > 0.0) as i32;
			let mut fTemp373: F32 = if iTemp372 != 0 {fSlow37} else {fSlow36};
			self.fVec32[0] = fTemp373;
			let mut fTemp374: F32 = 0.8888889 * fTemp373;
			let mut iTemp375: i32 = (fTemp374) as i32;
			let mut fTemp376: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp375, 294912)) as usize] };
			let mut fTemp377: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp375, 294903)) as usize] };
			let mut fTemp378: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp375, 294904)) as usize] } - fTemp377;
			let mut fTemp379: F32 = fTemp374 - (iTemp375) as F32;
			let mut fTemp380: F32 = 0.5 * (fTemp376 - (fTemp377 + fTemp379 * (fTemp378 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp375, 294913)) as usize] } - fTemp376))));
			let mut fTemp381: F32 = fSlow0 * ((fTemp371 < 0.0) as i32) as u32 as F32 + fSlow38 * (iTemp372) as F32;
			self.fVec33[0] = fTemp381;
			let mut fTemp382: F32 = self.fConst1 / fTemp381;
			let mut fTemp383: F32 = 65535.0 * (fTemp382 + 0.5);
			let mut iTemp384: i32 = (fTemp383) as i32;
			let mut fTemp385: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp375, i32::wrapping_mul(9, i32::wrapping_add(iTemp384, 1))), 589823))) as usize] };
			let mut iTemp386: i32 = i32::wrapping_add(iTemp375, i32::wrapping_mul(9, iTemp384));
			let mut fTemp387: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp386, 589823))) as usize] };
			let mut fTemp388: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp386, 1), 589823))) as usize] } - fTemp387;
			let mut fTemp389: F32 = 65535.0 * self.fRec4[1];
			let mut iTemp390: i32 = (fTemp389) as i32;
			let mut fTemp391: F32 = 0.8888889 * self.fVec32[1];
			let mut iTemp392: i32 = (fTemp391) as i32;
			let mut fTemp393: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp392, i32::wrapping_mul(9, i32::wrapping_add(iTemp390, 1))), 589823))) as usize] };
			let mut iTemp394: i32 = i32::wrapping_add(i32::wrapping_mul(9, iTemp390), iTemp392);
			let mut fTemp395: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp394, 589823))) as usize] };
			let mut fTemp396: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp394, 1), 589823))) as usize] } - fTemp395;
			let mut fTemp397: F32 = fTemp391 - (iTemp392) as F32;
			let mut fTemp398: F32 = 65535.0 * (self.fRec4[1] + fTemp382);
			let mut iTemp399: i32 = (fTemp398) as i32;
			let mut fTemp400: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp375, i32::wrapping_mul(9, i32::wrapping_add(iTemp399, 1))), 589823))) as usize] };
			let mut iTemp401: i32 = i32::wrapping_add(iTemp375, i32::wrapping_mul(9, iTemp399));
			let mut fTemp402: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp401, 589823))) as usize] };
			let mut fTemp403: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp401, 1), 589823))) as usize] } - fTemp402;
			let mut fTemp404: F32 = 65535.0 * (self.fRec4[1] + self.fConst1 * (1.0 / fTemp381 + 1.0 / self.fVec33[1]));
			let mut iTemp405: i32 = (fTemp404) as i32;
			let mut fTemp406: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp375, i32::wrapping_mul(9, i32::wrapping_add(iTemp405, 1))), 589823))) as usize] };
			let mut iTemp407: i32 = i32::wrapping_add(i32::wrapping_mul(9, iTemp405), iTemp375);
			let mut fTemp408: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp407, 589823))) as usize] };
			let mut fTemp409: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp407, 1), 589823))) as usize] } - fTemp408;
			let mut fTemp410: F32 = (fTemp379 * (fTemp409 - fTemp403) + fTemp408 + (fTemp404 - (iTemp405) as F32) * (fTemp406 - (fTemp408 + fTemp379 * (fTemp409 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp407, 10), 589823))) as usize] } - fTemp406)))) - (fTemp402 + (fTemp398 - (iTemp399) as F32) * (fTemp400 - (fTemp402 + fTemp379 * (fTemp403 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp401, 10), 589823))) as usize] } - fTemp400)))))) * self.fVec31[1] / (fTemp371 * (1.0 - (fTemp395 + fTemp397 * fTemp396 + (fTemp389 - (iTemp390) as F32) * (fTemp393 - (fTemp395 + fTemp397 * (fTemp396 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp394, 10), 589823))) as usize] } - fTemp393)))))));
			let mut iTemp411: i32 = (fTemp410 > ((fTemp379 * (fTemp388 - fTemp378) + fTemp387 + (fTemp383 - (iTemp384) as F32) * (fTemp385 - (fTemp387 + fTemp379 * (fTemp388 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp386, 10), 589823))) as usize] } - fTemp385)))) - (fTemp377 + fTemp380)) / (1.0 - (fTemp377 + fTemp379 * fTemp378 + fTemp380)))) as i32;
			let mut fTemp412: F32 = if iTemp411 != 0 {1.0} else {0.5};
			let mut fTemp413: F32 = if iTemp411 != 0 {0.5} else {0.0};
			let mut fTemp414: F32 = fTemp413 + fTemp412;
			let mut fTemp415: F32 = 32767.5 * fTemp414;
			let mut iTemp416: i32 = (fTemp415) as i32;
			let mut fTemp417: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp375, i32::wrapping_mul(9, i32::wrapping_add(iTemp416, 1)))) as usize] };
			let mut iTemp418: i32 = i32::wrapping_add(iTemp375, i32::wrapping_mul(9, iTemp416));
			let mut fTemp419: F32 = unsafe { ftbl0mydspSIG0[iTemp418 as usize] };
			let mut fTemp420: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp418, 1)) as usize] } - fTemp419;
			let mut fTemp421: F32 = (fTemp415 - (iTemp416) as F32) * (fTemp417 - (fTemp419 + fTemp379 * (fTemp420 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp418, 10)) as usize] } - fTemp417))));
			let mut fTemp422: F32 = 0.5 * fTemp414;
			let mut fTemp423: F32 = 65535.0 * (fTemp382 + fTemp422);
			let mut iTemp424: i32 = (fTemp423) as i32;
			let mut fTemp425: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp375, i32::wrapping_mul(9, i32::wrapping_add(iTemp424, 1))), 589823))) as usize] };
			let mut iTemp426: i32 = i32::wrapping_add(iTemp375, i32::wrapping_mul(9, iTemp424));
			let mut fTemp427: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp426, 589823))) as usize] };
			let mut fTemp428: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp426, 1), 589823))) as usize] } - fTemp427;
			let mut iTemp429: i32 = (fTemp410 > ((fTemp379 * (fTemp428 - fTemp420) + fTemp427 + (fTemp423 - (iTemp424) as F32) * (fTemp425 - (fTemp427 + fTemp379 * (fTemp428 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp426, 10), 589823))) as usize] } - fTemp425)))) - (fTemp419 + fTemp421)) / (1.0 - (fTemp419 + fTemp379 * fTemp420 + fTemp421)))) as i32;
			let mut fTemp430: F32 = if iTemp429 != 0 {fTemp412} else {fTemp422};
			let mut fTemp431: F32 = if iTemp429 != 0 {fTemp422} else {fTemp413};
			let mut fTemp432: F32 = fTemp431 + fTemp430;
			let mut fTemp433: F32 = 32767.5 * fTemp432;
			let mut iTemp434: i32 = (fTemp433) as i32;
			let mut fTemp435: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp375, i32::wrapping_mul(9, i32::wrapping_add(iTemp434, 1)))) as usize] };
			let mut iTemp436: i32 = i32::wrapping_add(iTemp375, i32::wrapping_mul(9, iTemp434));
			let mut fTemp437: F32 = unsafe { ftbl0mydspSIG0[iTemp436 as usize] };
			let mut fTemp438: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp436, 1)) as usize] } - fTemp437;
			let mut fTemp439: F32 = (fTemp433 - (iTemp434) as F32) * (fTemp435 - (fTemp437 + fTemp379 * (fTemp438 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp436, 10)) as usize] } - fTemp435))));
			let mut fTemp440: F32 = 0.5 * fTemp432;
			let mut fTemp441: F32 = 65535.0 * (fTemp382 + fTemp440);
			let mut iTemp442: i32 = (fTemp441) as i32;
			let mut fTemp443: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp375, i32::wrapping_mul(9, i32::wrapping_add(iTemp442, 1))), 589823))) as usize] };
			let mut iTemp444: i32 = i32::wrapping_add(iTemp375, i32::wrapping_mul(9, iTemp442));
			let mut fTemp445: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp444, 589823))) as usize] };
			let mut fTemp446: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp444, 1), 589823))) as usize] } - fTemp445;
			let mut iTemp447: i32 = (fTemp410 > ((fTemp379 * (fTemp446 - fTemp438) + fTemp445 + (fTemp441 - (iTemp442) as F32) * (fTemp443 - (fTemp445 + fTemp379 * (fTemp446 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp444, 10), 589823))) as usize] } - fTemp443)))) - (fTemp437 + fTemp439)) / (1.0 - (fTemp437 + fTemp379 * fTemp438 + fTemp439)))) as i32;
			let mut fTemp448: F32 = if iTemp447 != 0 {fTemp430} else {fTemp440};
			let mut fTemp449: F32 = if iTemp447 != 0 {fTemp440} else {fTemp431};
			let mut fTemp450: F32 = fTemp449 + fTemp448;
			let mut fTemp451: F32 = 32767.5 * fTemp450;
			let mut iTemp452: i32 = (fTemp451) as i32;
			let mut fTemp453: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp375, i32::wrapping_mul(9, i32::wrapping_add(iTemp452, 1)))) as usize] };
			let mut iTemp454: i32 = i32::wrapping_add(iTemp375, i32::wrapping_mul(9, iTemp452));
			let mut fTemp455: F32 = unsafe { ftbl0mydspSIG0[iTemp454 as usize] };
			let mut fTemp456: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp454, 1)) as usize] } - fTemp455;
			let mut fTemp457: F32 = (fTemp451 - (iTemp452) as F32) * (fTemp453 - (fTemp455 + fTemp379 * (fTemp456 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp454, 10)) as usize] } - fTemp453))));
			let mut fTemp458: F32 = 0.5 * fTemp450;
			let mut fTemp459: F32 = 65535.0 * (fTemp382 + fTemp458);
			let mut iTemp460: i32 = (fTemp459) as i32;
			let mut fTemp461: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp375, i32::wrapping_mul(9, i32::wrapping_add(iTemp460, 1))), 589823))) as usize] };
			let mut iTemp462: i32 = i32::wrapping_add(iTemp375, i32::wrapping_mul(9, iTemp460));
			let mut fTemp463: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp462, 589823))) as usize] };
			let mut fTemp464: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp462, 1), 589823))) as usize] } - fTemp463;
			let mut iTemp465: i32 = (fTemp410 > ((fTemp379 * (fTemp464 - fTemp456) + fTemp463 + (fTemp459 - (iTemp460) as F32) * (fTemp461 - (fTemp463 + fTemp379 * (fTemp464 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp462, 10), 589823))) as usize] } - fTemp461)))) - (fTemp455 + fTemp457)) / (1.0 - (fTemp455 + fTemp379 * fTemp456 + fTemp457)))) as i32;
			let mut fTemp466: F32 = if iTemp465 != 0 {fTemp448} else {fTemp458};
			let mut fTemp467: F32 = if iTemp465 != 0 {fTemp458} else {fTemp449};
			let mut fTemp468: F32 = fTemp467 + fTemp466;
			let mut fTemp469: F32 = 32767.5 * fTemp468;
			let mut iTemp470: i32 = (fTemp469) as i32;
			let mut fTemp471: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp375, i32::wrapping_mul(9, i32::wrapping_add(iTemp470, 1)))) as usize] };
			let mut iTemp472: i32 = i32::wrapping_add(iTemp375, i32::wrapping_mul(9, iTemp470));
			let mut fTemp473: F32 = unsafe { ftbl0mydspSIG0[iTemp472 as usize] };
			let mut fTemp474: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp472, 1)) as usize] } - fTemp473;
			let mut fTemp475: F32 = (fTemp469 - (iTemp470) as F32) * (fTemp471 - (fTemp473 + fTemp379 * (fTemp474 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp472, 10)) as usize] } - fTemp471))));
			let mut fTemp476: F32 = 0.5 * fTemp468;
			let mut fTemp477: F32 = 65535.0 * (fTemp382 + fTemp476);
			let mut iTemp478: i32 = (fTemp477) as i32;
			let mut fTemp479: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp375, i32::wrapping_mul(9, i32::wrapping_add(iTemp478, 1))), 589823))) as usize] };
			let mut iTemp480: i32 = i32::wrapping_add(iTemp375, i32::wrapping_mul(9, iTemp478));
			let mut fTemp481: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp480, 589823))) as usize] };
			let mut fTemp482: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp480, 1), 589823))) as usize] } - fTemp481;
			let mut iTemp483: i32 = (fTemp410 > ((fTemp379 * (fTemp482 - fTemp474) + fTemp481 + (fTemp477 - (iTemp478) as F32) * (fTemp479 - (fTemp481 + fTemp379 * (fTemp482 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp480, 10), 589823))) as usize] } - fTemp479)))) - (fTemp473 + fTemp475)) / (1.0 - (fTemp473 + fTemp379 * fTemp474 + fTemp475)))) as i32;
			let mut fTemp484: F32 = if iTemp483 != 0 {fTemp466} else {fTemp476};
			let mut fTemp485: F32 = if iTemp483 != 0 {fTemp476} else {fTemp467};
			let mut fTemp486: F32 = fTemp485 + fTemp484;
			let mut fTemp487: F32 = 32767.5 * fTemp486;
			let mut iTemp488: i32 = (fTemp487) as i32;
			let mut fTemp489: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp375, i32::wrapping_mul(9, i32::wrapping_add(iTemp488, 1)))) as usize] };
			let mut iTemp490: i32 = i32::wrapping_add(iTemp375, i32::wrapping_mul(9, iTemp488));
			let mut fTemp491: F32 = unsafe { ftbl0mydspSIG0[iTemp490 as usize] };
			let mut fTemp492: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp490, 1)) as usize] } - fTemp491;
			let mut fTemp493: F32 = (fTemp487 - (iTemp488) as F32) * (fTemp489 - (fTemp491 + fTemp379 * (fTemp492 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp490, 10)) as usize] } - fTemp489))));
			let mut fTemp494: F32 = 0.5 * fTemp486;
			let mut fTemp495: F32 = 65535.0 * (fTemp382 + fTemp494);
			let mut iTemp496: i32 = (fTemp495) as i32;
			let mut fTemp497: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp375, i32::wrapping_mul(9, i32::wrapping_add(iTemp496, 1))), 589823))) as usize] };
			let mut iTemp498: i32 = i32::wrapping_add(iTemp375, i32::wrapping_mul(9, iTemp496));
			let mut fTemp499: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp498, 589823))) as usize] };
			let mut fTemp500: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp498, 1), 589823))) as usize] } - fTemp499;
			let mut iTemp501: i32 = (fTemp410 > ((fTemp379 * (fTemp500 - fTemp492) + fTemp499 + (fTemp495 - (iTemp496) as F32) * (fTemp497 - (fTemp499 + fTemp379 * (fTemp500 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp498, 10), 589823))) as usize] } - fTemp497)))) - (fTemp491 + fTemp493)) / (1.0 - (fTemp491 + fTemp379 * fTemp492 + fTemp493)))) as i32;
			let mut fTemp502: F32 = if iTemp501 != 0 {fTemp484} else {fTemp494};
			let mut fTemp503: F32 = if iTemp501 != 0 {fTemp494} else {fTemp485};
			let mut fTemp504: F32 = fTemp503 + fTemp502;
			let mut fTemp505: F32 = 32767.5 * fTemp504;
			let mut iTemp506: i32 = (fTemp505) as i32;
			let mut fTemp507: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp375, i32::wrapping_mul(9, i32::wrapping_add(iTemp506, 1)))) as usize] };
			let mut iTemp508: i32 = i32::wrapping_add(iTemp375, i32::wrapping_mul(9, iTemp506));
			let mut fTemp509: F32 = unsafe { ftbl0mydspSIG0[iTemp508 as usize] };
			let mut fTemp510: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp508, 1)) as usize] } - fTemp509;
			let mut fTemp511: F32 = (fTemp505 - (iTemp506) as F32) * (fTemp507 - (fTemp509 + fTemp379 * (fTemp510 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp508, 10)) as usize] } - fTemp507))));
			let mut fTemp512: F32 = 0.5 * fTemp504;
			let mut fTemp513: F32 = 65535.0 * (fTemp382 + fTemp512);
			let mut iTemp514: i32 = (fTemp513) as i32;
			let mut fTemp515: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp375, i32::wrapping_mul(9, i32::wrapping_add(iTemp514, 1))), 589823))) as usize] };
			let mut iTemp516: i32 = i32::wrapping_add(iTemp375, i32::wrapping_mul(9, iTemp514));
			let mut fTemp517: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp516, 589823))) as usize] };
			let mut fTemp518: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp516, 1), 589823))) as usize] } - fTemp517;
			let mut iTemp519: i32 = (fTemp410 > ((fTemp379 * (fTemp518 - fTemp510) + fTemp517 + (fTemp513 - (iTemp514) as F32) * (fTemp515 - (fTemp517 + fTemp379 * (fTemp518 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp516, 10), 589823))) as usize] } - fTemp515)))) - (fTemp509 + fTemp511)) / (1.0 - (fTemp509 + fTemp379 * fTemp510 + fTemp511)))) as i32;
			let mut fTemp520: F32 = if iTemp519 != 0 {fTemp502} else {fTemp512};
			let mut fTemp521: F32 = if iTemp519 != 0 {fTemp512} else {fTemp503};
			let mut fTemp522: F32 = fTemp521 + fTemp520;
			let mut fTemp523: F32 = 32767.5 * fTemp522;
			let mut iTemp524: i32 = (fTemp523) as i32;
			let mut fTemp525: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp375, i32::wrapping_mul(9, i32::wrapping_add(iTemp524, 1)))) as usize] };
			let mut iTemp526: i32 = i32::wrapping_add(iTemp375, i32::wrapping_mul(9, iTemp524));
			let mut fTemp527: F32 = unsafe { ftbl0mydspSIG0[iTemp526 as usize] };
			let mut fTemp528: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp526, 1)) as usize] } - fTemp527;
			let mut fTemp529: F32 = (fTemp523 - (iTemp524) as F32) * (fTemp525 - (fTemp527 + fTemp379 * (fTemp528 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp526, 10)) as usize] } - fTemp525))));
			let mut fTemp530: F32 = 0.5 * fTemp522;
			let mut fTemp531: F32 = 65535.0 * (fTemp382 + fTemp530);
			let mut iTemp532: i32 = (fTemp531) as i32;
			let mut fTemp533: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp375, i32::wrapping_mul(9, i32::wrapping_add(iTemp532, 1))), 589823))) as usize] };
			let mut iTemp534: i32 = i32::wrapping_add(iTemp375, i32::wrapping_mul(9, iTemp532));
			let mut fTemp535: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp534, 589823))) as usize] };
			let mut fTemp536: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp534, 1), 589823))) as usize] } - fTemp535;
			let mut iTemp537: i32 = (fTemp410 > ((fTemp379 * (fTemp536 - fTemp528) + fTemp535 + (fTemp531 - (iTemp532) as F32) * (fTemp533 - (fTemp535 + fTemp379 * (fTemp536 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp534, 10), 589823))) as usize] } - fTemp533)))) - (fTemp527 + fTemp529)) / (1.0 - (fTemp527 + fTemp379 * fTemp528 + fTemp529)))) as i32;
			let mut fTemp538: F32 = if iTemp537 != 0 {fTemp520} else {fTemp530};
			let mut fTemp539: F32 = if iTemp537 != 0 {fTemp530} else {fTemp521};
			let mut fTemp540: F32 = fTemp539 + fTemp538;
			let mut fTemp541: F32 = 32767.5 * fTemp540;
			let mut iTemp542: i32 = (fTemp541) as i32;
			let mut fTemp543: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp375, i32::wrapping_mul(9, i32::wrapping_add(iTemp542, 1)))) as usize] };
			let mut iTemp544: i32 = i32::wrapping_add(iTemp375, i32::wrapping_mul(9, iTemp542));
			let mut fTemp545: F32 = unsafe { ftbl0mydspSIG0[iTemp544 as usize] };
			let mut fTemp546: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp544, 1)) as usize] } - fTemp545;
			let mut fTemp547: F32 = (fTemp541 - (iTemp542) as F32) * (fTemp543 - (fTemp545 + fTemp379 * (fTemp546 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp544, 10)) as usize] } - fTemp543))));
			let mut fTemp548: F32 = 0.5 * fTemp540;
			let mut fTemp549: F32 = 65535.0 * (fTemp382 + fTemp548);
			let mut iTemp550: i32 = (fTemp549) as i32;
			let mut fTemp551: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp375, i32::wrapping_mul(9, i32::wrapping_add(iTemp550, 1))), 589823))) as usize] };
			let mut iTemp552: i32 = i32::wrapping_add(iTemp375, i32::wrapping_mul(9, iTemp550));
			let mut fTemp553: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp552, 589823))) as usize] };
			let mut fTemp554: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp552, 1), 589823))) as usize] } - fTemp553;
			let mut iTemp555: i32 = (fTemp410 > ((fTemp379 * (fTemp554 - fTemp546) + fTemp553 + (fTemp549 - (iTemp550) as F32) * (fTemp551 - (fTemp553 + fTemp379 * (fTemp554 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp552, 10), 589823))) as usize] } - fTemp551)))) - (fTemp545 + fTemp547)) / (1.0 - (fTemp545 + fTemp379 * fTemp546 + fTemp547)))) as i32;
			let mut fTemp556: F32 = if iTemp555 != 0 {fTemp538} else {fTemp548};
			let mut fTemp557: F32 = if iTemp555 != 0 {fTemp548} else {fTemp539};
			let mut fTemp558: F32 = fTemp557 + fTemp556;
			let mut fTemp559: F32 = 32767.5 * fTemp558;
			let mut iTemp560: i32 = (fTemp559) as i32;
			let mut fTemp561: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp375, i32::wrapping_mul(9, i32::wrapping_add(iTemp560, 1)))) as usize] };
			let mut iTemp562: i32 = i32::wrapping_add(iTemp375, i32::wrapping_mul(9, iTemp560));
			let mut fTemp563: F32 = unsafe { ftbl0mydspSIG0[iTemp562 as usize] };
			let mut fTemp564: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp562, 1)) as usize] } - fTemp563;
			let mut fTemp565: F32 = (fTemp559 - (iTemp560) as F32) * (fTemp561 - (fTemp563 + fTemp379 * (fTemp564 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp562, 10)) as usize] } - fTemp561))));
			let mut fTemp566: F32 = 0.5 * fTemp558;
			let mut fTemp567: F32 = 65535.0 * (fTemp382 + fTemp566);
			let mut iTemp568: i32 = (fTemp567) as i32;
			let mut fTemp569: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp375, i32::wrapping_mul(9, i32::wrapping_add(iTemp568, 1))), 589823))) as usize] };
			let mut iTemp570: i32 = i32::wrapping_add(iTemp375, i32::wrapping_mul(9, iTemp568));
			let mut fTemp571: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp570, 589823))) as usize] };
			let mut fTemp572: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp570, 1), 589823))) as usize] } - fTemp571;
			let mut iTemp573: i32 = (fTemp410 > ((fTemp379 * (fTemp572 - fTemp564) + fTemp571 + (fTemp567 - (iTemp568) as F32) * (fTemp569 - (fTemp571 + fTemp379 * (fTemp572 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp570, 10), 589823))) as usize] } - fTemp569)))) - (fTemp563 + fTemp565)) / (1.0 - (fTemp563 + fTemp379 * fTemp564 + fTemp565)))) as i32;
			let mut fTemp574: F32 = if iTemp573 != 0 {fTemp556} else {fTemp566};
			let mut fTemp575: F32 = if iTemp573 != 0 {fTemp566} else {fTemp557};
			let mut fTemp576: F32 = fTemp575 + fTemp574;
			let mut fTemp577: F32 = 32767.5 * fTemp576;
			let mut iTemp578: i32 = (fTemp577) as i32;
			let mut fTemp579: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp375, i32::wrapping_mul(9, i32::wrapping_add(iTemp578, 1)))) as usize] };
			let mut iTemp580: i32 = i32::wrapping_add(iTemp375, i32::wrapping_mul(9, iTemp578));
			let mut fTemp581: F32 = unsafe { ftbl0mydspSIG0[iTemp580 as usize] };
			let mut fTemp582: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp580, 1)) as usize] } - fTemp581;
			let mut fTemp583: F32 = (fTemp577 - (iTemp578) as F32) * (fTemp579 - (fTemp581 + fTemp379 * (fTemp582 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp580, 10)) as usize] } - fTemp579))));
			let mut fTemp584: F32 = 0.5 * fTemp576;
			let mut fTemp585: F32 = 65535.0 * (fTemp382 + fTemp584);
			let mut iTemp586: i32 = (fTemp585) as i32;
			let mut fTemp587: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp375, i32::wrapping_mul(9, i32::wrapping_add(iTemp586, 1))), 589823))) as usize] };
			let mut iTemp588: i32 = i32::wrapping_add(iTemp375, i32::wrapping_mul(9, iTemp586));
			let mut fTemp589: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp588, 589823))) as usize] };
			let mut fTemp590: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp588, 1), 589823))) as usize] } - fTemp589;
			let mut iTemp591: i32 = (fTemp410 > ((fTemp379 * (fTemp590 - fTemp582) + fTemp589 + (fTemp585 - (iTemp586) as F32) * (fTemp587 - (fTemp589 + fTemp379 * (fTemp590 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp588, 10), 589823))) as usize] } - fTemp587)))) - (fTemp581 + fTemp583)) / (1.0 - (fTemp581 + fTemp379 * fTemp582 + fTemp583)))) as i32;
			let mut fTemp592: F32 = if iTemp591 != 0 {fTemp574} else {fTemp584};
			let mut fTemp593: F32 = if iTemp591 != 0 {fTemp584} else {fTemp575};
			let mut fTemp594: F32 = fTemp593 + fTemp592;
			let mut fTemp595: F32 = 32767.5 * fTemp594;
			let mut iTemp596: i32 = (fTemp595) as i32;
			let mut fTemp597: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp375, i32::wrapping_mul(9, i32::wrapping_add(iTemp596, 1)))) as usize] };
			let mut iTemp598: i32 = i32::wrapping_add(iTemp375, i32::wrapping_mul(9, iTemp596));
			let mut fTemp599: F32 = unsafe { ftbl0mydspSIG0[iTemp598 as usize] };
			let mut fTemp600: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp598, 1)) as usize] } - fTemp599;
			let mut fTemp601: F32 = (fTemp595 - (iTemp596) as F32) * (fTemp597 - (fTemp599 + fTemp379 * (fTemp600 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp598, 10)) as usize] } - fTemp597))));
			let mut fTemp602: F32 = 0.5 * fTemp594;
			let mut fTemp603: F32 = 65535.0 * (fTemp382 + fTemp602);
			let mut iTemp604: i32 = (fTemp603) as i32;
			let mut fTemp605: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp375, i32::wrapping_mul(9, i32::wrapping_add(iTemp604, 1))), 589823))) as usize] };
			let mut iTemp606: i32 = i32::wrapping_add(iTemp375, i32::wrapping_mul(9, iTemp604));
			let mut fTemp607: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp606, 589823))) as usize] };
			let mut fTemp608: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp606, 1), 589823))) as usize] } - fTemp607;
			let mut iTemp609: i32 = (fTemp410 > ((fTemp379 * (fTemp608 - fTemp600) + fTemp607 + (fTemp603 - (iTemp604) as F32) * (fTemp605 - (fTemp607 + fTemp379 * (fTemp608 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp606, 10), 589823))) as usize] } - fTemp605)))) - (fTemp599 + fTemp601)) / (1.0 - (fTemp599 + fTemp379 * fTemp600 + fTemp601)))) as i32;
			let mut fTemp610: F32 = if iTemp609 != 0 {fTemp592} else {fTemp602};
			let mut fTemp611: F32 = if iTemp609 != 0 {fTemp602} else {fTemp593};
			let mut fTemp612: F32 = fTemp611 + fTemp610;
			let mut fTemp613: F32 = 32767.5 * fTemp612;
			let mut iTemp614: i32 = (fTemp613) as i32;
			let mut fTemp615: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp375, i32::wrapping_mul(9, i32::wrapping_add(iTemp614, 1)))) as usize] };
			let mut iTemp616: i32 = i32::wrapping_add(iTemp375, i32::wrapping_mul(9, iTemp614));
			let mut fTemp617: F32 = unsafe { ftbl0mydspSIG0[iTemp616 as usize] };
			let mut fTemp618: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp616, 1)) as usize] } - fTemp617;
			let mut fTemp619: F32 = (fTemp613 - (iTemp614) as F32) * (fTemp615 - (fTemp617 + fTemp379 * (fTemp618 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp616, 10)) as usize] } - fTemp615))));
			let mut fTemp620: F32 = 0.5 * fTemp612;
			let mut fTemp621: F32 = 65535.0 * (fTemp382 + fTemp620);
			let mut iTemp622: i32 = (fTemp621) as i32;
			let mut fTemp623: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp375, i32::wrapping_mul(9, i32::wrapping_add(iTemp622, 1))), 589823))) as usize] };
			let mut iTemp624: i32 = i32::wrapping_add(iTemp375, i32::wrapping_mul(9, iTemp622));
			let mut fTemp625: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp624, 589823))) as usize] };
			let mut fTemp626: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp624, 1), 589823))) as usize] } - fTemp625;
			let mut iTemp627: i32 = (fTemp410 > ((fTemp379 * (fTemp626 - fTemp618) + fTemp625 + (fTemp621 - (iTemp622) as F32) * (fTemp623 - (fTemp625 + fTemp379 * (fTemp626 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp624, 10), 589823))) as usize] } - fTemp623)))) - (fTemp617 + fTemp619)) / (1.0 - (fTemp617 + fTemp379 * fTemp618 + fTemp619)))) as i32;
			let mut fTemp628: F32 = if iTemp627 != 0 {fTemp610} else {fTemp620};
			let mut fTemp629: F32 = if iTemp627 != 0 {fTemp620} else {fTemp611};
			let mut fTemp630: F32 = fTemp629 + fTemp628;
			let mut fTemp631: F32 = 32767.5 * fTemp630;
			let mut iTemp632: i32 = (fTemp631) as i32;
			let mut fTemp633: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp375, i32::wrapping_mul(9, i32::wrapping_add(iTemp632, 1)))) as usize] };
			let mut iTemp634: i32 = i32::wrapping_add(iTemp375, i32::wrapping_mul(9, iTemp632));
			let mut fTemp635: F32 = unsafe { ftbl0mydspSIG0[iTemp634 as usize] };
			let mut fTemp636: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp634, 1)) as usize] } - fTemp635;
			let mut fTemp637: F32 = (fTemp631 - (iTemp632) as F32) * (fTemp633 - (fTemp635 + fTemp379 * (fTemp636 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp634, 10)) as usize] } - fTemp633))));
			let mut fTemp638: F32 = 0.5 * fTemp630;
			let mut fTemp639: F32 = 65535.0 * (fTemp382 + fTemp638);
			let mut iTemp640: i32 = (fTemp639) as i32;
			let mut fTemp641: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp375, i32::wrapping_mul(9, i32::wrapping_add(iTemp640, 1))), 589823))) as usize] };
			let mut iTemp642: i32 = i32::wrapping_add(iTemp375, i32::wrapping_mul(9, iTemp640));
			let mut fTemp643: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp642, 589823))) as usize] };
			let mut fTemp644: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, 1), 589823))) as usize] } - fTemp643;
			let mut iTemp645: i32 = (fTemp410 > ((fTemp379 * (fTemp644 - fTemp636) + fTemp643 + (fTemp639 - (iTemp640) as F32) * (fTemp641 - (fTemp643 + fTemp379 * (fTemp644 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp642, 10), 589823))) as usize] } - fTemp641)))) - (fTemp635 + fTemp637)) / (1.0 - (fTemp635 + fTemp379 * fTemp636 + fTemp637)))) as i32;
			let mut fTemp646: F32 = if iTemp645 != 0 {fTemp628} else {fTemp638};
			let mut fTemp647: F32 = if iTemp645 != 0 {fTemp638} else {fTemp629};
			let mut fTemp648: F32 = fTemp647 + fTemp646;
			let mut fTemp649: F32 = 32767.5 * fTemp648;
			let mut iTemp650: i32 = (fTemp649) as i32;
			let mut fTemp651: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp375, i32::wrapping_mul(9, i32::wrapping_add(iTemp650, 1)))) as usize] };
			let mut iTemp652: i32 = i32::wrapping_add(iTemp375, i32::wrapping_mul(9, iTemp650));
			let mut fTemp653: F32 = unsafe { ftbl0mydspSIG0[iTemp652 as usize] };
			let mut fTemp654: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp652, 1)) as usize] } - fTemp653;
			let mut fTemp655: F32 = (fTemp649 - (iTemp650) as F32) * (fTemp651 - (fTemp653 + fTemp379 * (fTemp654 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp652, 10)) as usize] } - fTemp651))));
			let mut fTemp656: F32 = 0.5 * fTemp648;
			let mut fTemp657: F32 = 65535.0 * (fTemp382 + fTemp656);
			let mut iTemp658: i32 = (fTemp657) as i32;
			let mut fTemp659: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp375, i32::wrapping_mul(9, i32::wrapping_add(iTemp658, 1))), 589823))) as usize] };
			let mut iTemp660: i32 = i32::wrapping_add(iTemp375, i32::wrapping_mul(9, iTemp658));
			let mut fTemp661: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp660, 589823))) as usize] };
			let mut fTemp662: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp660, 1), 589823))) as usize] } - fTemp661;
			let mut iTemp663: i32 = (fTemp410 > ((fTemp379 * (fTemp662 - fTemp654) + fTemp661 + (fTemp657 - (iTemp658) as F32) * (fTemp659 - (fTemp661 + fTemp379 * (fTemp662 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp660, 10), 589823))) as usize] } - fTemp659)))) - (fTemp653 + fTemp655)) / (1.0 - (fTemp653 + fTemp379 * fTemp654 + fTemp655)))) as i32;
			let mut fTemp664: F32 = if iTemp663 != 0 {fTemp646} else {fTemp656};
			let mut fTemp665: F32 = if iTemp663 != 0 {fTemp656} else {fTemp647};
			let mut fTemp666: F32 = fTemp665 + fTemp664;
			let mut fTemp667: F32 = 32767.5 * fTemp666;
			let mut iTemp668: i32 = (fTemp667) as i32;
			let mut fTemp669: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp375, i32::wrapping_mul(9, i32::wrapping_add(iTemp668, 1)))) as usize] };
			let mut iTemp670: i32 = i32::wrapping_add(iTemp375, i32::wrapping_mul(9, iTemp668));
			let mut fTemp671: F32 = unsafe { ftbl0mydspSIG0[iTemp670 as usize] };
			let mut fTemp672: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp670, 1)) as usize] } - fTemp671;
			let mut fTemp673: F32 = (fTemp667 - (iTemp668) as F32) * (fTemp669 - (fTemp671 + fTemp379 * (fTemp672 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp670, 10)) as usize] } - fTemp669))));
			let mut fTemp674: F32 = 0.5 * fTemp666;
			let mut fTemp675: F32 = 65535.0 * (fTemp382 + fTemp674);
			let mut iTemp676: i32 = (fTemp675) as i32;
			let mut fTemp677: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp375, i32::wrapping_mul(9, i32::wrapping_add(iTemp676, 1))), 589823))) as usize] };
			let mut iTemp678: i32 = i32::wrapping_add(iTemp375, i32::wrapping_mul(9, iTemp676));
			let mut fTemp679: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp678, 589823))) as usize] };
			let mut fTemp680: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp678, 1), 589823))) as usize] } - fTemp679;
			let mut iTemp681: i32 = (fTemp410 > ((fTemp379 * (fTemp680 - fTemp672) + fTemp679 + (fTemp675 - (iTemp676) as F32) * (fTemp677 - (fTemp679 + fTemp379 * (fTemp680 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp678, 10), 589823))) as usize] } - fTemp677)))) - (fTemp671 + fTemp673)) / (1.0 - (fTemp671 + fTemp379 * fTemp672 + fTemp673)))) as i32;
			let mut fTemp682: F32 = F32::min(1.0, F32::max(0.0, 0.5 * (if iTemp681 != 0 {fTemp674} else {fTemp665} + if iTemp681 != 0 {fTemp664} else {fTemp674})));
			self.fRec4[0] = fTemp682;
			let mut fTemp683: F32 = 65535.0 * fTemp682;
			let mut iTemp684: i32 = (fTemp683) as i32;
			let mut fTemp685: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp375, i32::wrapping_mul(9, i32::wrapping_add(iTemp684, 1)))) as usize] };
			let mut iTemp686: i32 = i32::wrapping_add(iTemp375, i32::wrapping_mul(9, iTemp684));
			let mut fTemp687: F32 = unsafe { ftbl0mydspSIG0[iTemp686 as usize] };
			let mut fTemp688: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp686, 1)) as usize] } - fTemp687;
			let mut fTemp689: F32 = (fTemp683 - (iTemp684) as F32) * (fTemp685 - (fTemp687 + fTemp379 * (fTemp688 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp686, 10)) as usize] } - fTemp685))));
			let mut fTemp690: F32 = 65535.0 * (fTemp382 + fTemp682);
			let mut iTemp691: i32 = (fTemp690) as i32;
			let mut fTemp692: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp375, i32::wrapping_mul(9, i32::wrapping_add(iTemp691, 1))), 589823))) as usize] };
			let mut iTemp693: i32 = i32::wrapping_add(iTemp375, i32::wrapping_mul(9, iTemp691));
			let mut fTemp694: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp693, 589823))) as usize] };
			let mut fTemp695: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp693, 1), 589823))) as usize] } - fTemp694;
			let mut fTemp696: F32 = fTemp371 * (fTemp379 * (fTemp695 - fTemp688) + fTemp694 + (fTemp690 - (iTemp691) as F32) * (fTemp692 - (fTemp694 + fTemp379 * (fTemp695 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp693, 10), 589823))) as usize] } - fTemp692)))) - (fTemp687 + fTemp689)) / (1.0 - (fTemp687 + fTemp379 * fTemp688 + fTemp689));
			self.fRec5[0] = self.fRec5[1] + if iTemp372 != 0 {F32::min(fTemp371, fTemp696)} else {F32::max(fTemp371, fTemp696)};
			self.fHbargraph1 = F32::min(0.0, F32::max(-24.0, 2e+01 * F32::log10(F32::max(1.1754944e-38, self.fRec5[0]))));
			*output1 = self.fVec1[((i32::wrapping_sub(self.IOTA0, iSlow39)) & 262143) as usize] * self.fRec5[0];
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
			self.fVec15[1] = self.fVec15[0];
			self.fVec16[1] = self.fVec16[0];
			self.fVec17[1] = self.fVec17[0];
			self.fRec0[1] = self.fRec0[0];
			self.fRec1[1] = self.fRec1[0];
			self.fVec18[1] = self.fVec18[0];
			self.fVec19[2] = self.fVec19[1];
			self.fVec19[1] = self.fVec19[0];
			for j2 in (1..=6).rev() {
				self.fVec20[j2 as usize] = self.fVec20[(i32::wrapping_sub(j2, 1)) as usize];
			}
			for j3 in (1..=14).rev() {
				self.fVec21[j3 as usize] = self.fVec21[(i32::wrapping_sub(j3, 1)) as usize];
			}
			self.fVec31[1] = self.fVec31[0];
			self.fVec32[1] = self.fVec32[0];
			self.fVec33[1] = self.fVec33[0];
			self.fRec4[1] = self.fRec4[0];
			self.fRec5[1] = self.fRec5[0];
		}
	}

}


}
pub use dsp::mydsp as Gain;
