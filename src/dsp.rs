mod dsp {
    /* ------------------------------------------------------------
author: "Bart Brouns"
license: "AGPLv3"
name: "lamb"
version: "0.1"
Code generated with Faust 2.70.3 (https://faust.grame.fr)
Compilation options: -a /run/user/1001/.tmpcqb2QO -lang rust -ct 1 -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0
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
	fHslider0: F32,
	fSampleRate: i32,
	fConst1: F32,
	fHslider1: F32,
	fHslider2: F32,
	fConst2: F32,
	fConst3: F32,
	fHslider3: F32,
	fRec3: [F32;2],
	IOTA0: i32,
	fVec0: [F32;32768],
	fVec1: [F32;32768],
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
	fVec15: [F32;16384],
	fVec16: [F32;32768],
	fVec17: [F32;2],
	fHslider6: F32,
	fHslider7: F32,
	fVec18: [F32;2],
	fHslider8: F32,
	fVec19: [F32;2],
	fConst4: F32,
	fRec0: [F32;2],
	fRec1: [F32;2],
	fHbargraph0: F32,
	fVec20: [F32;2],
	fVec21: [F32;3],
	fVec22: [F32;7],
	fVec23: [F32;15],
	fVec24: [F32;32],
	fVec25: [F32;64],
	fVec26: [F32;128],
	fVec27: [F32;256],
	fVec28: [F32;512],
	fVec29: [F32;1024],
	fVec30: [F32;2048],
	fVec31: [F32;4096],
	fVec32: [F32;8192],
	fVec33: [F32;16384],
	fVec34: [F32;32768],
	fVec35: [F32;2],
	fVec36: [F32;2],
	fVec37: [F32;2],
	fRec4: [F32;2],
	fRec5: [F32;2],
	fHbargraph1: F32,
}

impl FaustDsp for mydsp {
	type T = F32;
		
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
			fVec17: [0.0;2],
			fHslider6: 0.0,
			fHslider7: 0.0,
			fVec18: [0.0;2],
			fHslider8: 0.0,
			fVec19: [0.0;2],
			fConst4: 0.0,
			fRec0: [0.0;2],
			fRec1: [0.0;2],
			fHbargraph0: 0.0,
			fVec20: [0.0;2],
			fVec21: [0.0;3],
			fVec22: [0.0;7],
			fVec23: [0.0;15],
			fVec24: [0.0;32],
			fVec25: [0.0;64],
			fVec26: [0.0;128],
			fVec27: [0.0;256],
			fVec28: [0.0;512],
			fVec29: [0.0;1024],
			fVec30: [0.0;2048],
			fVec31: [0.0;4096],
			fVec32: [0.0;8192],
			fVec33: [0.0;16384],
			fVec34: [0.0;32768],
			fVec35: [0.0;2],
			fVec36: [0.0;2],
			fVec37: [0.0;2],
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
		m.declare("compile_options", r"-a /run/user/1001/.tmpcqb2QO -lang rust -ct 1 -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0");
		m.declare("filename", r"lamb.dsp");
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
		for l19 in 0..2 {
			self.fVec17[l19 as usize] = 0.0;
		}
		for l20 in 0..2 {
			self.fVec18[l20 as usize] = 0.0;
		}
		for l21 in 0..2 {
			self.fVec19[l21 as usize] = 0.0;
		}
		for l22 in 0..2 {
			self.fRec0[l22 as usize] = 0.0;
		}
		for l23 in 0..2 {
			self.fRec1[l23 as usize] = 0.0;
		}
		for l24 in 0..2 {
			self.fVec20[l24 as usize] = 0.0;
		}
		for l25 in 0..3 {
			self.fVec21[l25 as usize] = 0.0;
		}
		for l26 in 0..7 {
			self.fVec22[l26 as usize] = 0.0;
		}
		for l27 in 0..15 {
			self.fVec23[l27 as usize] = 0.0;
		}
		for l28 in 0..32 {
			self.fVec24[l28 as usize] = 0.0;
		}
		for l29 in 0..64 {
			self.fVec25[l29 as usize] = 0.0;
		}
		for l30 in 0..128 {
			self.fVec26[l30 as usize] = 0.0;
		}
		for l31 in 0..256 {
			self.fVec27[l31 as usize] = 0.0;
		}
		for l32 in 0..512 {
			self.fVec28[l32 as usize] = 0.0;
		}
		for l33 in 0..1024 {
			self.fVec29[l33 as usize] = 0.0;
		}
		for l34 in 0..2048 {
			self.fVec30[l34 as usize] = 0.0;
		}
		for l35 in 0..4096 {
			self.fVec31[l35 as usize] = 0.0;
		}
		for l36 in 0..8192 {
			self.fVec32[l36 as usize] = 0.0;
		}
		for l37 in 0..16384 {
			self.fVec33[l37 as usize] = 0.0;
		}
		for l38 in 0..32768 {
			self.fVec34[l38 as usize] = 0.0;
		}
		for l39 in 0..2 {
			self.fVec35[l39 as usize] = 0.0;
		}
		for l40 in 0..2 {
			self.fVec36[l40 as usize] = 0.0;
		}
		for l41 in 0..2 {
			self.fVec37[l41 as usize] = 0.0;
		}
		for l42 in 0..2 {
			self.fRec4[l42 as usize] = 0.0;
		}
		for l43 in 0..2 {
			self.fRec5[l43 as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		let mut fConst0: F32 = F32::min(1.92e+05, F32::max(1.0, (self.fSampleRate) as F32));
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
		let mut fSlow0: F32 = self.fHslider0;
		let mut fSlow1: F32 = self.fConst1 * fSlow0;
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
		let mut iSlow36: i32 = (F32::floor(0.00012207031 * fSlow2)) as i32 % 2;
		let mut iSlow37: i32 = i32::wrapping_add(iSlow35, i32::wrapping_mul(4096, iSlow34));
		let mut iSlow38: i32 = (F32::floor(6.1035156e-05 * fSlow2)) as i32 % 2;
		let mut iSlow39: i32 = i32::wrapping_add(iSlow37, i32::wrapping_mul(8192, iSlow36));
		let mut fSlow40: F32 = self.fHslider6 + 4.0;
		let mut fSlow41: F32 = self.fHslider7 + 4.0;
		let mut fSlow42: F32 = self.fHslider8;
		let mut iSlow43: i32 = (fSlow1) as i32;
		let zipped_iterators = inputs0.zip(inputs1).zip(outputs0).zip(outputs1);
		for (((input0, input1), output0), output1) in zipped_iterators {
			self.fRec3[0] = fSlow8 + self.fConst3 * self.fRec3[1];
			let mut fTemp9: F32 = F32::powf(1e+01, 0.05 * self.fRec3[0]);
			let mut fTemp10: F32 = *input0 * fTemp9;
			self.fVec0[(self.IOTA0 & 32767) as usize] = fTemp10;
			let mut fTemp11: F32 = F32::abs(fTemp10);
			let mut fTemp12: F32 = *input1 * fTemp9;
			self.fVec1[(self.IOTA0 & 32767) as usize] = fTemp12;
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
			let mut fTemp30: F32 = F32::min(fTemp29, self.fVec13[((i32::wrapping_sub(self.IOTA0, 2048)) & 4095) as usize]);
			self.fVec14[(self.IOTA0 & 8191) as usize] = fTemp30;
			let mut fTemp31: F32 = F32::min(fTemp30, self.fVec14[((i32::wrapping_sub(self.IOTA0, 4096)) & 8191) as usize]);
			self.fVec15[(self.IOTA0 & 16383) as usize] = fTemp31;
			self.fVec16[(self.IOTA0 & 32767) as usize] = F32::min(fTemp31, self.fVec15[((i32::wrapping_sub(self.IOTA0, 8192)) & 16383) as usize]);
			let mut fTemp32: F32 = F32::powf(1e+01, 0.05 * F32::min(F32::min(F32::min(F32::min(F32::min(F32::min(F32::min(F32::min(F32::min(F32::min(F32::min(F32::min(F32::min(F32::min(if iSlow3 != 0 {-fTemp18} else {3.4028235e+38}, if iSlow13 != 0 {self.fVec3[iSlow3 as usize]} else {3.4028235e+38}), if iSlow14 != 0 {self.fVec4[iSlow15 as usize]} else {3.4028235e+38}), if iSlow16 != 0 {self.fVec5[iSlow17 as usize]} else {3.4028235e+38}), if iSlow18 != 0 {self.fVec6[((i32::wrapping_sub(self.IOTA0, iSlow19)) & 31) as usize]} else {3.4028235e+38}), if iSlow20 != 0 {self.fVec7[((i32::wrapping_sub(self.IOTA0, iSlow21)) & 63) as usize]} else {3.4028235e+38}), if iSlow22 != 0 {self.fVec8[((i32::wrapping_sub(self.IOTA0, iSlow23)) & 127) as usize]} else {3.4028235e+38}), if iSlow24 != 0 {self.fVec9[((i32::wrapping_sub(self.IOTA0, iSlow25)) & 255) as usize]} else {3.4028235e+38}), if iSlow26 != 0 {self.fVec10[((i32::wrapping_sub(self.IOTA0, iSlow27)) & 511) as usize]} else {3.4028235e+38}), if iSlow28 != 0 {self.fVec11[((i32::wrapping_sub(self.IOTA0, iSlow29)) & 1023) as usize]} else {3.4028235e+38}), if iSlow30 != 0 {self.fVec12[((i32::wrapping_sub(self.IOTA0, iSlow31)) & 2047) as usize]} else {3.4028235e+38}), if iSlow32 != 0 {self.fVec13[((i32::wrapping_sub(self.IOTA0, iSlow33)) & 4095) as usize]} else {3.4028235e+38}), if iSlow34 != 0 {self.fVec14[((i32::wrapping_sub(self.IOTA0, iSlow35)) & 8191) as usize]} else {3.4028235e+38}), if iSlow36 != 0 {self.fVec15[((i32::wrapping_sub(self.IOTA0, iSlow37)) & 16383) as usize]} else {3.4028235e+38}), if iSlow38 != 0 {self.fVec16[((i32::wrapping_sub(self.IOTA0, iSlow39)) & 32767) as usize]} else {3.4028235e+38})) - self.fRec1[1];
			self.fVec17[0] = fTemp32;
			let mut iTemp33: i32 = (fTemp32 > 0.0) as i32;
			let mut fTemp34: F32 = if iTemp33 != 0 {fSlow41} else {fSlow40};
			self.fVec18[0] = fTemp34;
			let mut fTemp35: F32 = 0.8888889 * fTemp34;
			let mut iTemp36: i32 = (fTemp35) as i32;
			let mut fTemp37: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp36, 294912)) as usize] };
			let mut fTemp38: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp36, 294903)) as usize] };
			let mut fTemp39: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp36, 294904)) as usize] } - fTemp38;
			let mut fTemp40: F32 = fTemp35 - (iTemp36) as F32;
			let mut fTemp41: F32 = 0.5 * (fTemp37 - (fTemp38 + fTemp40 * (fTemp39 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp36, 294913)) as usize] } - fTemp37))));
			let mut fTemp42: F32 = fSlow0 * ((fTemp32 < 0.0) as i32) as u32 as F32 + fSlow42 * (iTemp33) as F32;
			self.fVec19[0] = fTemp42;
			let mut fTemp43: F32 = self.fConst4 / fTemp42;
			let mut fTemp44: F32 = 65535.0 * (fTemp43 + 0.5);
			let mut iTemp45: i32 = (fTemp44) as i32;
			let mut fTemp46: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp36, i32::wrapping_mul(9, i32::wrapping_add(iTemp45, 1))), 589823))) as usize] };
			let mut iTemp47: i32 = i32::wrapping_add(iTemp36, i32::wrapping_mul(9, iTemp45));
			let mut fTemp48: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp47, 589823))) as usize] };
			let mut fTemp49: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, 1), 589823))) as usize] } - fTemp48;
			let mut fTemp50: F32 = 65535.0 * self.fRec0[1];
			let mut iTemp51: i32 = (fTemp50) as i32;
			let mut fTemp52: F32 = 0.8888889 * self.fVec18[1];
			let mut iTemp53: i32 = (fTemp52) as i32;
			let mut fTemp54: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp53, i32::wrapping_mul(9, i32::wrapping_add(iTemp51, 1))), 589823))) as usize] };
			let mut iTemp55: i32 = i32::wrapping_add(i32::wrapping_mul(9, iTemp51), iTemp53);
			let mut fTemp56: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp55, 589823))) as usize] };
			let mut fTemp57: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp55, 1), 589823))) as usize] } - fTemp56;
			let mut fTemp58: F32 = fTemp52 - (iTemp53) as F32;
			let mut fTemp59: F32 = 65535.0 * (self.fRec0[1] + fTemp43);
			let mut iTemp60: i32 = (fTemp59) as i32;
			let mut fTemp61: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp36, i32::wrapping_mul(9, i32::wrapping_add(iTemp60, 1))), 589823))) as usize] };
			let mut iTemp62: i32 = i32::wrapping_add(iTemp36, i32::wrapping_mul(9, iTemp60));
			let mut fTemp63: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp62, 589823))) as usize] };
			let mut fTemp64: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp62, 1), 589823))) as usize] } - fTemp63;
			let mut fTemp65: F32 = 65535.0 * (self.fRec0[1] + self.fConst4 * (1.0 / fTemp42 + 1.0 / self.fVec19[1]));
			let mut iTemp66: i32 = (fTemp65) as i32;
			let mut fTemp67: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp36, i32::wrapping_mul(9, i32::wrapping_add(iTemp66, 1))), 589823))) as usize] };
			let mut iTemp68: i32 = i32::wrapping_add(i32::wrapping_mul(9, iTemp66), iTemp36);
			let mut fTemp69: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp68, 589823))) as usize] };
			let mut fTemp70: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp68, 1), 589823))) as usize] } - fTemp69;
			let mut fTemp71: F32 = (fTemp40 * (fTemp70 - fTemp64) + fTemp69 + (fTemp65 - (iTemp66) as F32) * (fTemp67 - (fTemp69 + fTemp40 * (fTemp70 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp68, 10), 589823))) as usize] } - fTemp67)))) - (fTemp63 + (fTemp59 - (iTemp60) as F32) * (fTemp61 - (fTemp63 + fTemp40 * (fTemp64 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp62, 10), 589823))) as usize] } - fTemp61)))))) * self.fVec17[1] / (fTemp32 * (1.0 - (fTemp56 + fTemp58 * fTemp57 + (fTemp50 - (iTemp51) as F32) * (fTemp54 - (fTemp56 + fTemp58 * (fTemp57 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp55, 10), 589823))) as usize] } - fTemp54)))))));
			let mut iTemp72: i32 = (fTemp71 > ((fTemp40 * (fTemp49 - fTemp39) + fTemp48 + (fTemp44 - (iTemp45) as F32) * (fTemp46 - (fTemp48 + fTemp40 * (fTemp49 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp47, 10), 589823))) as usize] } - fTemp46)))) - (fTemp38 + fTemp41)) / (1.0 - (fTemp38 + fTemp40 * fTemp39 + fTemp41)))) as i32;
			let mut fTemp73: F32 = if iTemp72 != 0 {1.0} else {0.5};
			let mut fTemp74: F32 = if iTemp72 != 0 {0.5} else {0.0};
			let mut fTemp75: F32 = fTemp74 + fTemp73;
			let mut fTemp76: F32 = 32767.5 * fTemp75;
			let mut iTemp77: i32 = (fTemp76) as i32;
			let mut fTemp78: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp36, i32::wrapping_mul(9, i32::wrapping_add(iTemp77, 1)))) as usize] };
			let mut iTemp79: i32 = i32::wrapping_add(iTemp36, i32::wrapping_mul(9, iTemp77));
			let mut fTemp80: F32 = unsafe { ftbl0mydspSIG0[iTemp79 as usize] };
			let mut fTemp81: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp79, 1)) as usize] } - fTemp80;
			let mut fTemp82: F32 = (fTemp76 - (iTemp77) as F32) * (fTemp78 - (fTemp80 + fTemp40 * (fTemp81 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp79, 10)) as usize] } - fTemp78))));
			let mut fTemp83: F32 = 0.5 * fTemp75;
			let mut fTemp84: F32 = 65535.0 * (fTemp43 + fTemp83);
			let mut iTemp85: i32 = (fTemp84) as i32;
			let mut fTemp86: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp36, i32::wrapping_mul(9, i32::wrapping_add(iTemp85, 1))), 589823))) as usize] };
			let mut iTemp87: i32 = i32::wrapping_add(iTemp36, i32::wrapping_mul(9, iTemp85));
			let mut fTemp88: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp87, 589823))) as usize] };
			let mut fTemp89: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp87, 1), 589823))) as usize] } - fTemp88;
			let mut iTemp90: i32 = (fTemp71 > ((fTemp40 * (fTemp89 - fTemp81) + fTemp88 + (fTemp84 - (iTemp85) as F32) * (fTemp86 - (fTemp88 + fTemp40 * (fTemp89 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp87, 10), 589823))) as usize] } - fTemp86)))) - (fTemp80 + fTemp82)) / (1.0 - (fTemp80 + fTemp40 * fTemp81 + fTemp82)))) as i32;
			let mut fTemp91: F32 = if iTemp90 != 0 {fTemp73} else {fTemp83};
			let mut fTemp92: F32 = if iTemp90 != 0 {fTemp83} else {fTemp74};
			let mut fTemp93: F32 = fTemp92 + fTemp91;
			let mut fTemp94: F32 = 32767.5 * fTemp93;
			let mut iTemp95: i32 = (fTemp94) as i32;
			let mut fTemp96: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp36, i32::wrapping_mul(9, i32::wrapping_add(iTemp95, 1)))) as usize] };
			let mut iTemp97: i32 = i32::wrapping_add(iTemp36, i32::wrapping_mul(9, iTemp95));
			let mut fTemp98: F32 = unsafe { ftbl0mydspSIG0[iTemp97 as usize] };
			let mut fTemp99: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp97, 1)) as usize] } - fTemp98;
			let mut fTemp100: F32 = (fTemp94 - (iTemp95) as F32) * (fTemp96 - (fTemp98 + fTemp40 * (fTemp99 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp97, 10)) as usize] } - fTemp96))));
			let mut fTemp101: F32 = 0.5 * fTemp93;
			let mut fTemp102: F32 = 65535.0 * (fTemp43 + fTemp101);
			let mut iTemp103: i32 = (fTemp102) as i32;
			let mut fTemp104: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp36, i32::wrapping_mul(9, i32::wrapping_add(iTemp103, 1))), 589823))) as usize] };
			let mut iTemp105: i32 = i32::wrapping_add(iTemp36, i32::wrapping_mul(9, iTemp103));
			let mut fTemp106: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp105, 589823))) as usize] };
			let mut fTemp107: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp105, 1), 589823))) as usize] } - fTemp106;
			let mut iTemp108: i32 = (fTemp71 > ((fTemp40 * (fTemp107 - fTemp99) + fTemp106 + (fTemp102 - (iTemp103) as F32) * (fTemp104 - (fTemp106 + fTemp40 * (fTemp107 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp105, 10), 589823))) as usize] } - fTemp104)))) - (fTemp98 + fTemp100)) / (1.0 - (fTemp98 + fTemp40 * fTemp99 + fTemp100)))) as i32;
			let mut fTemp109: F32 = if iTemp108 != 0 {fTemp91} else {fTemp101};
			let mut fTemp110: F32 = if iTemp108 != 0 {fTemp101} else {fTemp92};
			let mut fTemp111: F32 = fTemp110 + fTemp109;
			let mut fTemp112: F32 = 32767.5 * fTemp111;
			let mut iTemp113: i32 = (fTemp112) as i32;
			let mut fTemp114: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp36, i32::wrapping_mul(9, i32::wrapping_add(iTemp113, 1)))) as usize] };
			let mut iTemp115: i32 = i32::wrapping_add(iTemp36, i32::wrapping_mul(9, iTemp113));
			let mut fTemp116: F32 = unsafe { ftbl0mydspSIG0[iTemp115 as usize] };
			let mut fTemp117: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp115, 1)) as usize] } - fTemp116;
			let mut fTemp118: F32 = (fTemp112 - (iTemp113) as F32) * (fTemp114 - (fTemp116 + fTemp40 * (fTemp117 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp115, 10)) as usize] } - fTemp114))));
			let mut fTemp119: F32 = 0.5 * fTemp111;
			let mut fTemp120: F32 = 65535.0 * (fTemp43 + fTemp119);
			let mut iTemp121: i32 = (fTemp120) as i32;
			let mut fTemp122: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp36, i32::wrapping_mul(9, i32::wrapping_add(iTemp121, 1))), 589823))) as usize] };
			let mut iTemp123: i32 = i32::wrapping_add(iTemp36, i32::wrapping_mul(9, iTemp121));
			let mut fTemp124: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp123, 589823))) as usize] };
			let mut fTemp125: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp123, 1), 589823))) as usize] } - fTemp124;
			let mut iTemp126: i32 = (fTemp71 > ((fTemp40 * (fTemp125 - fTemp117) + fTemp124 + (fTemp120 - (iTemp121) as F32) * (fTemp122 - (fTemp124 + fTemp40 * (fTemp125 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp123, 10), 589823))) as usize] } - fTemp122)))) - (fTemp116 + fTemp118)) / (1.0 - (fTemp116 + fTemp40 * fTemp117 + fTemp118)))) as i32;
			let mut fTemp127: F32 = if iTemp126 != 0 {fTemp109} else {fTemp119};
			let mut fTemp128: F32 = if iTemp126 != 0 {fTemp119} else {fTemp110};
			let mut fTemp129: F32 = fTemp128 + fTemp127;
			let mut fTemp130: F32 = 32767.5 * fTemp129;
			let mut iTemp131: i32 = (fTemp130) as i32;
			let mut fTemp132: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp36, i32::wrapping_mul(9, i32::wrapping_add(iTemp131, 1)))) as usize] };
			let mut iTemp133: i32 = i32::wrapping_add(iTemp36, i32::wrapping_mul(9, iTemp131));
			let mut fTemp134: F32 = unsafe { ftbl0mydspSIG0[iTemp133 as usize] };
			let mut fTemp135: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp133, 1)) as usize] } - fTemp134;
			let mut fTemp136: F32 = (fTemp130 - (iTemp131) as F32) * (fTemp132 - (fTemp134 + fTemp40 * (fTemp135 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp133, 10)) as usize] } - fTemp132))));
			let mut fTemp137: F32 = 0.5 * fTemp129;
			let mut fTemp138: F32 = 65535.0 * (fTemp43 + fTemp137);
			let mut iTemp139: i32 = (fTemp138) as i32;
			let mut fTemp140: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp36, i32::wrapping_mul(9, i32::wrapping_add(iTemp139, 1))), 589823))) as usize] };
			let mut iTemp141: i32 = i32::wrapping_add(iTemp36, i32::wrapping_mul(9, iTemp139));
			let mut fTemp142: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp141, 589823))) as usize] };
			let mut fTemp143: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp141, 1), 589823))) as usize] } - fTemp142;
			let mut iTemp144: i32 = (fTemp71 > ((fTemp40 * (fTemp143 - fTemp135) + fTemp142 + (fTemp138 - (iTemp139) as F32) * (fTemp140 - (fTemp142 + fTemp40 * (fTemp143 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp141, 10), 589823))) as usize] } - fTemp140)))) - (fTemp134 + fTemp136)) / (1.0 - (fTemp134 + fTemp40 * fTemp135 + fTemp136)))) as i32;
			let mut fTemp145: F32 = if iTemp144 != 0 {fTemp127} else {fTemp137};
			let mut fTemp146: F32 = if iTemp144 != 0 {fTemp137} else {fTemp128};
			let mut fTemp147: F32 = fTemp146 + fTemp145;
			let mut fTemp148: F32 = 32767.5 * fTemp147;
			let mut iTemp149: i32 = (fTemp148) as i32;
			let mut fTemp150: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp36, i32::wrapping_mul(9, i32::wrapping_add(iTemp149, 1)))) as usize] };
			let mut iTemp151: i32 = i32::wrapping_add(iTemp36, i32::wrapping_mul(9, iTemp149));
			let mut fTemp152: F32 = unsafe { ftbl0mydspSIG0[iTemp151 as usize] };
			let mut fTemp153: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp151, 1)) as usize] } - fTemp152;
			let mut fTemp154: F32 = (fTemp148 - (iTemp149) as F32) * (fTemp150 - (fTemp152 + fTemp40 * (fTemp153 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp151, 10)) as usize] } - fTemp150))));
			let mut fTemp155: F32 = 0.5 * fTemp147;
			let mut fTemp156: F32 = 65535.0 * (fTemp43 + fTemp155);
			let mut iTemp157: i32 = (fTemp156) as i32;
			let mut fTemp158: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp36, i32::wrapping_mul(9, i32::wrapping_add(iTemp157, 1))), 589823))) as usize] };
			let mut iTemp159: i32 = i32::wrapping_add(iTemp36, i32::wrapping_mul(9, iTemp157));
			let mut fTemp160: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp159, 589823))) as usize] };
			let mut fTemp161: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp159, 1), 589823))) as usize] } - fTemp160;
			let mut iTemp162: i32 = (fTemp71 > ((fTemp40 * (fTemp161 - fTemp153) + fTemp160 + (fTemp156 - (iTemp157) as F32) * (fTemp158 - (fTemp160 + fTemp40 * (fTemp161 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp159, 10), 589823))) as usize] } - fTemp158)))) - (fTemp152 + fTemp154)) / (1.0 - (fTemp152 + fTemp40 * fTemp153 + fTemp154)))) as i32;
			let mut fTemp163: F32 = if iTemp162 != 0 {fTemp145} else {fTemp155};
			let mut fTemp164: F32 = if iTemp162 != 0 {fTemp155} else {fTemp146};
			let mut fTemp165: F32 = fTemp164 + fTemp163;
			let mut fTemp166: F32 = 32767.5 * fTemp165;
			let mut iTemp167: i32 = (fTemp166) as i32;
			let mut fTemp168: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp36, i32::wrapping_mul(9, i32::wrapping_add(iTemp167, 1)))) as usize] };
			let mut iTemp169: i32 = i32::wrapping_add(iTemp36, i32::wrapping_mul(9, iTemp167));
			let mut fTemp170: F32 = unsafe { ftbl0mydspSIG0[iTemp169 as usize] };
			let mut fTemp171: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp169, 1)) as usize] } - fTemp170;
			let mut fTemp172: F32 = (fTemp166 - (iTemp167) as F32) * (fTemp168 - (fTemp170 + fTemp40 * (fTemp171 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp169, 10)) as usize] } - fTemp168))));
			let mut fTemp173: F32 = 0.5 * fTemp165;
			let mut fTemp174: F32 = 65535.0 * (fTemp43 + fTemp173);
			let mut iTemp175: i32 = (fTemp174) as i32;
			let mut fTemp176: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp36, i32::wrapping_mul(9, i32::wrapping_add(iTemp175, 1))), 589823))) as usize] };
			let mut iTemp177: i32 = i32::wrapping_add(iTemp36, i32::wrapping_mul(9, iTemp175));
			let mut fTemp178: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp177, 589823))) as usize] };
			let mut fTemp179: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp177, 1), 589823))) as usize] } - fTemp178;
			let mut iTemp180: i32 = (fTemp71 > ((fTemp40 * (fTemp179 - fTemp171) + fTemp178 + (fTemp174 - (iTemp175) as F32) * (fTemp176 - (fTemp178 + fTemp40 * (fTemp179 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp177, 10), 589823))) as usize] } - fTemp176)))) - (fTemp170 + fTemp172)) / (1.0 - (fTemp170 + fTemp40 * fTemp171 + fTemp172)))) as i32;
			let mut fTemp181: F32 = if iTemp180 != 0 {fTemp163} else {fTemp173};
			let mut fTemp182: F32 = if iTemp180 != 0 {fTemp173} else {fTemp164};
			let mut fTemp183: F32 = fTemp182 + fTemp181;
			let mut fTemp184: F32 = 32767.5 * fTemp183;
			let mut iTemp185: i32 = (fTemp184) as i32;
			let mut fTemp186: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp36, i32::wrapping_mul(9, i32::wrapping_add(iTemp185, 1)))) as usize] };
			let mut iTemp187: i32 = i32::wrapping_add(iTemp36, i32::wrapping_mul(9, iTemp185));
			let mut fTemp188: F32 = unsafe { ftbl0mydspSIG0[iTemp187 as usize] };
			let mut fTemp189: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp187, 1)) as usize] } - fTemp188;
			let mut fTemp190: F32 = (fTemp184 - (iTemp185) as F32) * (fTemp186 - (fTemp188 + fTemp40 * (fTemp189 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp187, 10)) as usize] } - fTemp186))));
			let mut fTemp191: F32 = 0.5 * fTemp183;
			let mut fTemp192: F32 = 65535.0 * (fTemp43 + fTemp191);
			let mut iTemp193: i32 = (fTemp192) as i32;
			let mut fTemp194: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp36, i32::wrapping_mul(9, i32::wrapping_add(iTemp193, 1))), 589823))) as usize] };
			let mut iTemp195: i32 = i32::wrapping_add(iTemp36, i32::wrapping_mul(9, iTemp193));
			let mut fTemp196: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp195, 589823))) as usize] };
			let mut fTemp197: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp195, 1), 589823))) as usize] } - fTemp196;
			let mut iTemp198: i32 = (fTemp71 > ((fTemp40 * (fTemp197 - fTemp189) + fTemp196 + (fTemp192 - (iTemp193) as F32) * (fTemp194 - (fTemp196 + fTemp40 * (fTemp197 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp195, 10), 589823))) as usize] } - fTemp194)))) - (fTemp188 + fTemp190)) / (1.0 - (fTemp188 + fTemp40 * fTemp189 + fTemp190)))) as i32;
			let mut fTemp199: F32 = if iTemp198 != 0 {fTemp181} else {fTemp191};
			let mut fTemp200: F32 = if iTemp198 != 0 {fTemp191} else {fTemp182};
			let mut fTemp201: F32 = fTemp200 + fTemp199;
			let mut fTemp202: F32 = 32767.5 * fTemp201;
			let mut iTemp203: i32 = (fTemp202) as i32;
			let mut fTemp204: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp36, i32::wrapping_mul(9, i32::wrapping_add(iTemp203, 1)))) as usize] };
			let mut iTemp205: i32 = i32::wrapping_add(iTemp36, i32::wrapping_mul(9, iTemp203));
			let mut fTemp206: F32 = unsafe { ftbl0mydspSIG0[iTemp205 as usize] };
			let mut fTemp207: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp205, 1)) as usize] } - fTemp206;
			let mut fTemp208: F32 = (fTemp202 - (iTemp203) as F32) * (fTemp204 - (fTemp206 + fTemp40 * (fTemp207 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp205, 10)) as usize] } - fTemp204))));
			let mut fTemp209: F32 = 0.5 * fTemp201;
			let mut fTemp210: F32 = 65535.0 * (fTemp43 + fTemp209);
			let mut iTemp211: i32 = (fTemp210) as i32;
			let mut fTemp212: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp36, i32::wrapping_mul(9, i32::wrapping_add(iTemp211, 1))), 589823))) as usize] };
			let mut iTemp213: i32 = i32::wrapping_add(iTemp36, i32::wrapping_mul(9, iTemp211));
			let mut fTemp214: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp213, 589823))) as usize] };
			let mut fTemp215: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp213, 1), 589823))) as usize] } - fTemp214;
			let mut iTemp216: i32 = (fTemp71 > ((fTemp40 * (fTemp215 - fTemp207) + fTemp214 + (fTemp210 - (iTemp211) as F32) * (fTemp212 - (fTemp214 + fTemp40 * (fTemp215 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp213, 10), 589823))) as usize] } - fTemp212)))) - (fTemp206 + fTemp208)) / (1.0 - (fTemp206 + fTemp40 * fTemp207 + fTemp208)))) as i32;
			let mut fTemp217: F32 = if iTemp216 != 0 {fTemp199} else {fTemp209};
			let mut fTemp218: F32 = if iTemp216 != 0 {fTemp209} else {fTemp200};
			let mut fTemp219: F32 = fTemp218 + fTemp217;
			let mut fTemp220: F32 = 32767.5 * fTemp219;
			let mut iTemp221: i32 = (fTemp220) as i32;
			let mut fTemp222: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp36, i32::wrapping_mul(9, i32::wrapping_add(iTemp221, 1)))) as usize] };
			let mut iTemp223: i32 = i32::wrapping_add(iTemp36, i32::wrapping_mul(9, iTemp221));
			let mut fTemp224: F32 = unsafe { ftbl0mydspSIG0[iTemp223 as usize] };
			let mut fTemp225: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp223, 1)) as usize] } - fTemp224;
			let mut fTemp226: F32 = (fTemp220 - (iTemp221) as F32) * (fTemp222 - (fTemp224 + fTemp40 * (fTemp225 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp223, 10)) as usize] } - fTemp222))));
			let mut fTemp227: F32 = 0.5 * fTemp219;
			let mut fTemp228: F32 = 65535.0 * (fTemp43 + fTemp227);
			let mut iTemp229: i32 = (fTemp228) as i32;
			let mut fTemp230: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp36, i32::wrapping_mul(9, i32::wrapping_add(iTemp229, 1))), 589823))) as usize] };
			let mut iTemp231: i32 = i32::wrapping_add(iTemp36, i32::wrapping_mul(9, iTemp229));
			let mut fTemp232: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp231, 589823))) as usize] };
			let mut fTemp233: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp231, 1), 589823))) as usize] } - fTemp232;
			let mut iTemp234: i32 = (fTemp71 > ((fTemp40 * (fTemp233 - fTemp225) + fTemp232 + (fTemp228 - (iTemp229) as F32) * (fTemp230 - (fTemp232 + fTemp40 * (fTemp233 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp231, 10), 589823))) as usize] } - fTemp230)))) - (fTemp224 + fTemp226)) / (1.0 - (fTemp224 + fTemp40 * fTemp225 + fTemp226)))) as i32;
			let mut fTemp235: F32 = if iTemp234 != 0 {fTemp217} else {fTemp227};
			let mut fTemp236: F32 = if iTemp234 != 0 {fTemp227} else {fTemp218};
			let mut fTemp237: F32 = fTemp236 + fTemp235;
			let mut fTemp238: F32 = 32767.5 * fTemp237;
			let mut iTemp239: i32 = (fTemp238) as i32;
			let mut fTemp240: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp36, i32::wrapping_mul(9, i32::wrapping_add(iTemp239, 1)))) as usize] };
			let mut iTemp241: i32 = i32::wrapping_add(iTemp36, i32::wrapping_mul(9, iTemp239));
			let mut fTemp242: F32 = unsafe { ftbl0mydspSIG0[iTemp241 as usize] };
			let mut fTemp243: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp241, 1)) as usize] } - fTemp242;
			let mut fTemp244: F32 = (fTemp238 - (iTemp239) as F32) * (fTemp240 - (fTemp242 + fTemp40 * (fTemp243 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp241, 10)) as usize] } - fTemp240))));
			let mut fTemp245: F32 = 0.5 * fTemp237;
			let mut fTemp246: F32 = 65535.0 * (fTemp43 + fTemp245);
			let mut iTemp247: i32 = (fTemp246) as i32;
			let mut fTemp248: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp36, i32::wrapping_mul(9, i32::wrapping_add(iTemp247, 1))), 589823))) as usize] };
			let mut iTemp249: i32 = i32::wrapping_add(iTemp36, i32::wrapping_mul(9, iTemp247));
			let mut fTemp250: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp249, 589823))) as usize] };
			let mut fTemp251: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp249, 1), 589823))) as usize] } - fTemp250;
			let mut iTemp252: i32 = (fTemp71 > ((fTemp40 * (fTemp251 - fTemp243) + fTemp250 + (fTemp246 - (iTemp247) as F32) * (fTemp248 - (fTemp250 + fTemp40 * (fTemp251 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp249, 10), 589823))) as usize] } - fTemp248)))) - (fTemp242 + fTemp244)) / (1.0 - (fTemp242 + fTemp40 * fTemp243 + fTemp244)))) as i32;
			let mut fTemp253: F32 = if iTemp252 != 0 {fTemp235} else {fTemp245};
			let mut fTemp254: F32 = if iTemp252 != 0 {fTemp245} else {fTemp236};
			let mut fTemp255: F32 = fTemp254 + fTemp253;
			let mut fTemp256: F32 = 32767.5 * fTemp255;
			let mut iTemp257: i32 = (fTemp256) as i32;
			let mut fTemp258: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp36, i32::wrapping_mul(9, i32::wrapping_add(iTemp257, 1)))) as usize] };
			let mut iTemp259: i32 = i32::wrapping_add(iTemp36, i32::wrapping_mul(9, iTemp257));
			let mut fTemp260: F32 = unsafe { ftbl0mydspSIG0[iTemp259 as usize] };
			let mut fTemp261: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp259, 1)) as usize] } - fTemp260;
			let mut fTemp262: F32 = (fTemp256 - (iTemp257) as F32) * (fTemp258 - (fTemp260 + fTemp40 * (fTemp261 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp259, 10)) as usize] } - fTemp258))));
			let mut fTemp263: F32 = 0.5 * fTemp255;
			let mut fTemp264: F32 = 65535.0 * (fTemp43 + fTemp263);
			let mut iTemp265: i32 = (fTemp264) as i32;
			let mut fTemp266: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp36, i32::wrapping_mul(9, i32::wrapping_add(iTemp265, 1))), 589823))) as usize] };
			let mut iTemp267: i32 = i32::wrapping_add(iTemp36, i32::wrapping_mul(9, iTemp265));
			let mut fTemp268: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp267, 589823))) as usize] };
			let mut fTemp269: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp267, 1), 589823))) as usize] } - fTemp268;
			let mut iTemp270: i32 = (fTemp71 > ((fTemp40 * (fTemp269 - fTemp261) + fTemp268 + (fTemp264 - (iTemp265) as F32) * (fTemp266 - (fTemp268 + fTemp40 * (fTemp269 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp267, 10), 589823))) as usize] } - fTemp266)))) - (fTemp260 + fTemp262)) / (1.0 - (fTemp260 + fTemp40 * fTemp261 + fTemp262)))) as i32;
			let mut fTemp271: F32 = if iTemp270 != 0 {fTemp253} else {fTemp263};
			let mut fTemp272: F32 = if iTemp270 != 0 {fTemp263} else {fTemp254};
			let mut fTemp273: F32 = fTemp272 + fTemp271;
			let mut fTemp274: F32 = 32767.5 * fTemp273;
			let mut iTemp275: i32 = (fTemp274) as i32;
			let mut fTemp276: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp36, i32::wrapping_mul(9, i32::wrapping_add(iTemp275, 1)))) as usize] };
			let mut iTemp277: i32 = i32::wrapping_add(iTemp36, i32::wrapping_mul(9, iTemp275));
			let mut fTemp278: F32 = unsafe { ftbl0mydspSIG0[iTemp277 as usize] };
			let mut fTemp279: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp277, 1)) as usize] } - fTemp278;
			let mut fTemp280: F32 = (fTemp274 - (iTemp275) as F32) * (fTemp276 - (fTemp278 + fTemp40 * (fTemp279 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp277, 10)) as usize] } - fTemp276))));
			let mut fTemp281: F32 = 0.5 * fTemp273;
			let mut fTemp282: F32 = 65535.0 * (fTemp43 + fTemp281);
			let mut iTemp283: i32 = (fTemp282) as i32;
			let mut fTemp284: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp36, i32::wrapping_mul(9, i32::wrapping_add(iTemp283, 1))), 589823))) as usize] };
			let mut iTemp285: i32 = i32::wrapping_add(iTemp36, i32::wrapping_mul(9, iTemp283));
			let mut fTemp286: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp285, 589823))) as usize] };
			let mut fTemp287: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp285, 1), 589823))) as usize] } - fTemp286;
			let mut iTemp288: i32 = (fTemp71 > ((fTemp40 * (fTemp287 - fTemp279) + fTemp286 + (fTemp282 - (iTemp283) as F32) * (fTemp284 - (fTemp286 + fTemp40 * (fTemp287 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp285, 10), 589823))) as usize] } - fTemp284)))) - (fTemp278 + fTemp280)) / (1.0 - (fTemp278 + fTemp40 * fTemp279 + fTemp280)))) as i32;
			let mut fTemp289: F32 = if iTemp288 != 0 {fTemp271} else {fTemp281};
			let mut fTemp290: F32 = if iTemp288 != 0 {fTemp281} else {fTemp272};
			let mut fTemp291: F32 = fTemp290 + fTemp289;
			let mut fTemp292: F32 = 32767.5 * fTemp291;
			let mut iTemp293: i32 = (fTemp292) as i32;
			let mut fTemp294: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp36, i32::wrapping_mul(9, i32::wrapping_add(iTemp293, 1)))) as usize] };
			let mut iTemp295: i32 = i32::wrapping_add(iTemp36, i32::wrapping_mul(9, iTemp293));
			let mut fTemp296: F32 = unsafe { ftbl0mydspSIG0[iTemp295 as usize] };
			let mut fTemp297: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp295, 1)) as usize] } - fTemp296;
			let mut fTemp298: F32 = (fTemp292 - (iTemp293) as F32) * (fTemp294 - (fTemp296 + fTemp40 * (fTemp297 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp295, 10)) as usize] } - fTemp294))));
			let mut fTemp299: F32 = 0.5 * fTemp291;
			let mut fTemp300: F32 = 65535.0 * (fTemp43 + fTemp299);
			let mut iTemp301: i32 = (fTemp300) as i32;
			let mut fTemp302: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp36, i32::wrapping_mul(9, i32::wrapping_add(iTemp301, 1))), 589823))) as usize] };
			let mut iTemp303: i32 = i32::wrapping_add(iTemp36, i32::wrapping_mul(9, iTemp301));
			let mut fTemp304: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp303, 589823))) as usize] };
			let mut fTemp305: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp303, 1), 589823))) as usize] } - fTemp304;
			let mut iTemp306: i32 = (fTemp71 > ((fTemp40 * (fTemp305 - fTemp297) + fTemp304 + (fTemp300 - (iTemp301) as F32) * (fTemp302 - (fTemp304 + fTemp40 * (fTemp305 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp303, 10), 589823))) as usize] } - fTemp302)))) - (fTemp296 + fTemp298)) / (1.0 - (fTemp296 + fTemp40 * fTemp297 + fTemp298)))) as i32;
			let mut fTemp307: F32 = if iTemp306 != 0 {fTemp289} else {fTemp299};
			let mut fTemp308: F32 = if iTemp306 != 0 {fTemp299} else {fTemp290};
			let mut fTemp309: F32 = fTemp308 + fTemp307;
			let mut fTemp310: F32 = 32767.5 * fTemp309;
			let mut iTemp311: i32 = (fTemp310) as i32;
			let mut fTemp312: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp36, i32::wrapping_mul(9, i32::wrapping_add(iTemp311, 1)))) as usize] };
			let mut iTemp313: i32 = i32::wrapping_add(iTemp36, i32::wrapping_mul(9, iTemp311));
			let mut fTemp314: F32 = unsafe { ftbl0mydspSIG0[iTemp313 as usize] };
			let mut fTemp315: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp313, 1)) as usize] } - fTemp314;
			let mut fTemp316: F32 = (fTemp310 - (iTemp311) as F32) * (fTemp312 - (fTemp314 + fTemp40 * (fTemp315 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp313, 10)) as usize] } - fTemp312))));
			let mut fTemp317: F32 = 0.5 * fTemp309;
			let mut fTemp318: F32 = 65535.0 * (fTemp43 + fTemp317);
			let mut iTemp319: i32 = (fTemp318) as i32;
			let mut fTemp320: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp36, i32::wrapping_mul(9, i32::wrapping_add(iTemp319, 1))), 589823))) as usize] };
			let mut iTemp321: i32 = i32::wrapping_add(iTemp36, i32::wrapping_mul(9, iTemp319));
			let mut fTemp322: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp321, 589823))) as usize] };
			let mut fTemp323: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp321, 1), 589823))) as usize] } - fTemp322;
			let mut iTemp324: i32 = (fTemp71 > ((fTemp40 * (fTemp323 - fTemp315) + fTemp322 + (fTemp318 - (iTemp319) as F32) * (fTemp320 - (fTemp322 + fTemp40 * (fTemp323 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp321, 10), 589823))) as usize] } - fTemp320)))) - (fTemp314 + fTemp316)) / (1.0 - (fTemp314 + fTemp40 * fTemp315 + fTemp316)))) as i32;
			let mut fTemp325: F32 = if iTemp324 != 0 {fTemp307} else {fTemp317};
			let mut fTemp326: F32 = if iTemp324 != 0 {fTemp317} else {fTemp308};
			let mut fTemp327: F32 = fTemp326 + fTemp325;
			let mut fTemp328: F32 = 32767.5 * fTemp327;
			let mut iTemp329: i32 = (fTemp328) as i32;
			let mut fTemp330: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp36, i32::wrapping_mul(9, i32::wrapping_add(iTemp329, 1)))) as usize] };
			let mut iTemp331: i32 = i32::wrapping_add(iTemp36, i32::wrapping_mul(9, iTemp329));
			let mut fTemp332: F32 = unsafe { ftbl0mydspSIG0[iTemp331 as usize] };
			let mut fTemp333: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp331, 1)) as usize] } - fTemp332;
			let mut fTemp334: F32 = (fTemp328 - (iTemp329) as F32) * (fTemp330 - (fTemp332 + fTemp40 * (fTemp333 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp331, 10)) as usize] } - fTemp330))));
			let mut fTemp335: F32 = 0.5 * fTemp327;
			let mut fTemp336: F32 = 65535.0 * (fTemp43 + fTemp335);
			let mut iTemp337: i32 = (fTemp336) as i32;
			let mut fTemp338: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp36, i32::wrapping_mul(9, i32::wrapping_add(iTemp337, 1))), 589823))) as usize] };
			let mut iTemp339: i32 = i32::wrapping_add(iTemp36, i32::wrapping_mul(9, iTemp337));
			let mut fTemp340: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp339, 589823))) as usize] };
			let mut fTemp341: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp339, 1), 589823))) as usize] } - fTemp340;
			let mut iTemp342: i32 = (fTemp71 > ((fTemp40 * (fTemp341 - fTemp333) + fTemp340 + (fTemp336 - (iTemp337) as F32) * (fTemp338 - (fTemp340 + fTemp40 * (fTemp341 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp339, 10), 589823))) as usize] } - fTemp338)))) - (fTemp332 + fTemp334)) / (1.0 - (fTemp332 + fTemp40 * fTemp333 + fTemp334)))) as i32;
			let mut fTemp343: F32 = F32::min(1.0, F32::max(0.0, 0.5 * (if iTemp342 != 0 {fTemp335} else {fTemp326} + if iTemp342 != 0 {fTemp325} else {fTemp335})));
			self.fRec0[0] = fTemp343;
			let mut fTemp344: F32 = 65535.0 * fTemp343;
			let mut iTemp345: i32 = (fTemp344) as i32;
			let mut fTemp346: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp36, i32::wrapping_mul(9, i32::wrapping_add(iTemp345, 1)))) as usize] };
			let mut iTemp347: i32 = i32::wrapping_add(iTemp36, i32::wrapping_mul(9, iTemp345));
			let mut fTemp348: F32 = unsafe { ftbl0mydspSIG0[iTemp347 as usize] };
			let mut fTemp349: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp347, 1)) as usize] } - fTemp348;
			let mut fTemp350: F32 = (fTemp344 - (iTemp345) as F32) * (fTemp346 - (fTemp348 + fTemp40 * (fTemp349 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp347, 10)) as usize] } - fTemp346))));
			let mut fTemp351: F32 = 65535.0 * (fTemp43 + fTemp343);
			let mut iTemp352: i32 = (fTemp351) as i32;
			let mut fTemp353: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp36, i32::wrapping_mul(9, i32::wrapping_add(iTemp352, 1))), 589823))) as usize] };
			let mut iTemp354: i32 = i32::wrapping_add(iTemp36, i32::wrapping_mul(9, iTemp352));
			let mut fTemp355: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp354, 589823))) as usize] };
			let mut fTemp356: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp354, 1), 589823))) as usize] } - fTemp355;
			let mut fTemp357: F32 = fTemp32 * (fTemp40 * (fTemp356 - fTemp349) + fTemp355 + (fTemp351 - (iTemp352) as F32) * (fTemp353 - (fTemp355 + fTemp40 * (fTemp356 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp354, 10), 589823))) as usize] } - fTemp353)))) - (fTemp348 + fTemp350)) / (1.0 - (fTemp348 + fTemp40 * fTemp349 + fTemp350));
			self.fRec1[0] = self.fRec1[1] + if iTemp33 != 0 {F32::min(fTemp32, fTemp357)} else {F32::max(fTemp32, fTemp357)};
			self.fHbargraph0 = F32::min(0.0, F32::max(-24.0, 2e+01 * F32::log10(F32::max(1.1754944e-38, self.fRec1[0]))));
			*output0 = self.fVec0[((i32::wrapping_sub(self.IOTA0, iSlow43)) & 32767) as usize] * self.fRec1[0];
			let mut fTemp358: F32 = 2e+01 * F32::log10(F32::max(1.1754944e-38, fTemp13 + fSlow9 * (fTemp14 - fTemp13)));
			let mut iTemp359: i32 = ((fTemp358 > fSlow10) as i32) + ((fTemp358 > fSlow7) as i32);
			let mut fTemp360: F32 = fTemp358 - fSlow6;
			let mut fTemp361: F32 = fSlow12 * F32::max(0.0, if (iTemp359 == 0) as i32 != 0 {0.0} else {if (iTemp359 == 1) as i32 != 0 {fSlow11 * mydsp_faustpower2_f(fSlow5 + fTemp360)} else {fTemp360}});
			self.fVec20[0] = fTemp361;
			let mut fTemp362: F32 = F32::min(-fTemp361, -self.fVec20[1]);
			self.fVec21[0] = fTemp362;
			let mut fTemp363: F32 = F32::min(fTemp362, self.fVec21[2]);
			self.fVec22[0] = fTemp363;
			let mut fTemp364: F32 = F32::min(fTemp363, self.fVec22[4]);
			self.fVec23[0] = fTemp364;
			let mut fTemp365: F32 = F32::min(fTemp364, self.fVec23[8]);
			self.fVec24[(self.IOTA0 & 31) as usize] = fTemp365;
			let mut fTemp366: F32 = F32::min(fTemp365, self.fVec24[((i32::wrapping_sub(self.IOTA0, 16)) & 31) as usize]);
			self.fVec25[(self.IOTA0 & 63) as usize] = fTemp366;
			let mut fTemp367: F32 = F32::min(fTemp366, self.fVec25[((i32::wrapping_sub(self.IOTA0, 32)) & 63) as usize]);
			self.fVec26[(self.IOTA0 & 127) as usize] = fTemp367;
			let mut fTemp368: F32 = F32::min(fTemp367, self.fVec26[((i32::wrapping_sub(self.IOTA0, 64)) & 127) as usize]);
			self.fVec27[(self.IOTA0 & 255) as usize] = fTemp368;
			let mut fTemp369: F32 = F32::min(fTemp368, self.fVec27[((i32::wrapping_sub(self.IOTA0, 128)) & 255) as usize]);
			self.fVec28[(self.IOTA0 & 511) as usize] = fTemp369;
			let mut fTemp370: F32 = F32::min(fTemp369, self.fVec28[((i32::wrapping_sub(self.IOTA0, 256)) & 511) as usize]);
			self.fVec29[(self.IOTA0 & 1023) as usize] = fTemp370;
			let mut fTemp371: F32 = F32::min(fTemp370, self.fVec29[((i32::wrapping_sub(self.IOTA0, 512)) & 1023) as usize]);
			self.fVec30[(self.IOTA0 & 2047) as usize] = fTemp371;
			let mut fTemp372: F32 = F32::min(fTemp371, self.fVec30[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize]);
			self.fVec31[(self.IOTA0 & 4095) as usize] = fTemp372;
			let mut fTemp373: F32 = F32::min(fTemp372, self.fVec31[((i32::wrapping_sub(self.IOTA0, 2048)) & 4095) as usize]);
			self.fVec32[(self.IOTA0 & 8191) as usize] = fTemp373;
			let mut fTemp374: F32 = F32::min(fTemp373, self.fVec32[((i32::wrapping_sub(self.IOTA0, 4096)) & 8191) as usize]);
			self.fVec33[(self.IOTA0 & 16383) as usize] = fTemp374;
			self.fVec34[(self.IOTA0 & 32767) as usize] = F32::min(fTemp374, self.fVec33[((i32::wrapping_sub(self.IOTA0, 8192)) & 16383) as usize]);
			let mut fTemp375: F32 = F32::powf(1e+01, 0.05 * F32::min(F32::min(F32::min(F32::min(F32::min(F32::min(F32::min(F32::min(F32::min(F32::min(F32::min(F32::min(F32::min(F32::min(if iSlow3 != 0 {-fTemp361} else {3.4028235e+38}, if iSlow13 != 0 {self.fVec21[iSlow3 as usize]} else {3.4028235e+38}), if iSlow14 != 0 {self.fVec22[iSlow15 as usize]} else {3.4028235e+38}), if iSlow16 != 0 {self.fVec23[iSlow17 as usize]} else {3.4028235e+38}), if iSlow18 != 0 {self.fVec24[((i32::wrapping_sub(self.IOTA0, iSlow19)) & 31) as usize]} else {3.4028235e+38}), if iSlow20 != 0 {self.fVec25[((i32::wrapping_sub(self.IOTA0, iSlow21)) & 63) as usize]} else {3.4028235e+38}), if iSlow22 != 0 {self.fVec26[((i32::wrapping_sub(self.IOTA0, iSlow23)) & 127) as usize]} else {3.4028235e+38}), if iSlow24 != 0 {self.fVec27[((i32::wrapping_sub(self.IOTA0, iSlow25)) & 255) as usize]} else {3.4028235e+38}), if iSlow26 != 0 {self.fVec28[((i32::wrapping_sub(self.IOTA0, iSlow27)) & 511) as usize]} else {3.4028235e+38}), if iSlow28 != 0 {self.fVec29[((i32::wrapping_sub(self.IOTA0, iSlow29)) & 1023) as usize]} else {3.4028235e+38}), if iSlow30 != 0 {self.fVec30[((i32::wrapping_sub(self.IOTA0, iSlow31)) & 2047) as usize]} else {3.4028235e+38}), if iSlow32 != 0 {self.fVec31[((i32::wrapping_sub(self.IOTA0, iSlow33)) & 4095) as usize]} else {3.4028235e+38}), if iSlow34 != 0 {self.fVec32[((i32::wrapping_sub(self.IOTA0, iSlow35)) & 8191) as usize]} else {3.4028235e+38}), if iSlow36 != 0 {self.fVec33[((i32::wrapping_sub(self.IOTA0, iSlow37)) & 16383) as usize]} else {3.4028235e+38}), if iSlow38 != 0 {self.fVec34[((i32::wrapping_sub(self.IOTA0, iSlow39)) & 32767) as usize]} else {3.4028235e+38})) - self.fRec5[1];
			self.fVec35[0] = fTemp375;
			let mut iTemp376: i32 = (fTemp375 > 0.0) as i32;
			let mut fTemp377: F32 = if iTemp376 != 0 {fSlow41} else {fSlow40};
			self.fVec36[0] = fTemp377;
			let mut fTemp378: F32 = 0.8888889 * fTemp377;
			let mut iTemp379: i32 = (fTemp378) as i32;
			let mut fTemp380: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp379, 294912)) as usize] };
			let mut fTemp381: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp379, 294903)) as usize] };
			let mut fTemp382: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp379, 294904)) as usize] } - fTemp381;
			let mut fTemp383: F32 = fTemp378 - (iTemp379) as F32;
			let mut fTemp384: F32 = 0.5 * (fTemp380 - (fTemp381 + fTemp383 * (fTemp382 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp379, 294913)) as usize] } - fTemp380))));
			let mut fTemp385: F32 = fSlow0 * ((fTemp375 < 0.0) as i32) as u32 as F32 + fSlow42 * (iTemp376) as F32;
			self.fVec37[0] = fTemp385;
			let mut fTemp386: F32 = self.fConst4 / fTemp385;
			let mut fTemp387: F32 = 65535.0 * (fTemp386 + 0.5);
			let mut iTemp388: i32 = (fTemp387) as i32;
			let mut fTemp389: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp379, i32::wrapping_mul(9, i32::wrapping_add(iTemp388, 1))), 589823))) as usize] };
			let mut iTemp390: i32 = i32::wrapping_add(iTemp379, i32::wrapping_mul(9, iTemp388));
			let mut fTemp391: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp390, 589823))) as usize] };
			let mut fTemp392: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp390, 1), 589823))) as usize] } - fTemp391;
			let mut fTemp393: F32 = 65535.0 * self.fRec4[1];
			let mut iTemp394: i32 = (fTemp393) as i32;
			let mut fTemp395: F32 = 0.8888889 * self.fVec36[1];
			let mut iTemp396: i32 = (fTemp395) as i32;
			let mut fTemp397: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp396, i32::wrapping_mul(9, i32::wrapping_add(iTemp394, 1))), 589823))) as usize] };
			let mut iTemp398: i32 = i32::wrapping_add(i32::wrapping_mul(9, iTemp394), iTemp396);
			let mut fTemp399: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp398, 589823))) as usize] };
			let mut fTemp400: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp398, 1), 589823))) as usize] } - fTemp399;
			let mut fTemp401: F32 = fTemp395 - (iTemp396) as F32;
			let mut fTemp402: F32 = 65535.0 * (self.fRec4[1] + fTemp386);
			let mut iTemp403: i32 = (fTemp402) as i32;
			let mut fTemp404: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp379, i32::wrapping_mul(9, i32::wrapping_add(iTemp403, 1))), 589823))) as usize] };
			let mut iTemp405: i32 = i32::wrapping_add(iTemp379, i32::wrapping_mul(9, iTemp403));
			let mut fTemp406: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp405, 589823))) as usize] };
			let mut fTemp407: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp405, 1), 589823))) as usize] } - fTemp406;
			let mut fTemp408: F32 = 65535.0 * (self.fRec4[1] + self.fConst4 * (1.0 / fTemp385 + 1.0 / self.fVec37[1]));
			let mut iTemp409: i32 = (fTemp408) as i32;
			let mut fTemp410: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp379, i32::wrapping_mul(9, i32::wrapping_add(iTemp409, 1))), 589823))) as usize] };
			let mut iTemp411: i32 = i32::wrapping_add(i32::wrapping_mul(9, iTemp409), iTemp379);
			let mut fTemp412: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp411, 589823))) as usize] };
			let mut fTemp413: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp411, 1), 589823))) as usize] } - fTemp412;
			let mut fTemp414: F32 = (fTemp383 * (fTemp413 - fTemp407) + fTemp412 + (fTemp408 - (iTemp409) as F32) * (fTemp410 - (fTemp412 + fTemp383 * (fTemp413 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp411, 10), 589823))) as usize] } - fTemp410)))) - (fTemp406 + (fTemp402 - (iTemp403) as F32) * (fTemp404 - (fTemp406 + fTemp383 * (fTemp407 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp405, 10), 589823))) as usize] } - fTemp404)))))) * self.fVec35[1] / (fTemp375 * (1.0 - (fTemp399 + fTemp401 * fTemp400 + (fTemp393 - (iTemp394) as F32) * (fTemp397 - (fTemp399 + fTemp401 * (fTemp400 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp398, 10), 589823))) as usize] } - fTemp397)))))));
			let mut iTemp415: i32 = (fTemp414 > ((fTemp383 * (fTemp392 - fTemp382) + fTemp391 + (fTemp387 - (iTemp388) as F32) * (fTemp389 - (fTemp391 + fTemp383 * (fTemp392 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp390, 10), 589823))) as usize] } - fTemp389)))) - (fTemp381 + fTemp384)) / (1.0 - (fTemp381 + fTemp383 * fTemp382 + fTemp384)))) as i32;
			let mut fTemp416: F32 = if iTemp415 != 0 {1.0} else {0.5};
			let mut fTemp417: F32 = if iTemp415 != 0 {0.5} else {0.0};
			let mut fTemp418: F32 = fTemp417 + fTemp416;
			let mut fTemp419: F32 = 32767.5 * fTemp418;
			let mut iTemp420: i32 = (fTemp419) as i32;
			let mut fTemp421: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp379, i32::wrapping_mul(9, i32::wrapping_add(iTemp420, 1)))) as usize] };
			let mut iTemp422: i32 = i32::wrapping_add(iTemp379, i32::wrapping_mul(9, iTemp420));
			let mut fTemp423: F32 = unsafe { ftbl0mydspSIG0[iTemp422 as usize] };
			let mut fTemp424: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp422, 1)) as usize] } - fTemp423;
			let mut fTemp425: F32 = (fTemp419 - (iTemp420) as F32) * (fTemp421 - (fTemp423 + fTemp383 * (fTemp424 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp422, 10)) as usize] } - fTemp421))));
			let mut fTemp426: F32 = 0.5 * fTemp418;
			let mut fTemp427: F32 = 65535.0 * (fTemp386 + fTemp426);
			let mut iTemp428: i32 = (fTemp427) as i32;
			let mut fTemp429: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp379, i32::wrapping_mul(9, i32::wrapping_add(iTemp428, 1))), 589823))) as usize] };
			let mut iTemp430: i32 = i32::wrapping_add(iTemp379, i32::wrapping_mul(9, iTemp428));
			let mut fTemp431: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp430, 589823))) as usize] };
			let mut fTemp432: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp430, 1), 589823))) as usize] } - fTemp431;
			let mut iTemp433: i32 = (fTemp414 > ((fTemp383 * (fTemp432 - fTemp424) + fTemp431 + (fTemp427 - (iTemp428) as F32) * (fTemp429 - (fTemp431 + fTemp383 * (fTemp432 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp430, 10), 589823))) as usize] } - fTemp429)))) - (fTemp423 + fTemp425)) / (1.0 - (fTemp423 + fTemp383 * fTemp424 + fTemp425)))) as i32;
			let mut fTemp434: F32 = if iTemp433 != 0 {fTemp416} else {fTemp426};
			let mut fTemp435: F32 = if iTemp433 != 0 {fTemp426} else {fTemp417};
			let mut fTemp436: F32 = fTemp435 + fTemp434;
			let mut fTemp437: F32 = 32767.5 * fTemp436;
			let mut iTemp438: i32 = (fTemp437) as i32;
			let mut fTemp439: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp379, i32::wrapping_mul(9, i32::wrapping_add(iTemp438, 1)))) as usize] };
			let mut iTemp440: i32 = i32::wrapping_add(iTemp379, i32::wrapping_mul(9, iTemp438));
			let mut fTemp441: F32 = unsafe { ftbl0mydspSIG0[iTemp440 as usize] };
			let mut fTemp442: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp440, 1)) as usize] } - fTemp441;
			let mut fTemp443: F32 = (fTemp437 - (iTemp438) as F32) * (fTemp439 - (fTemp441 + fTemp383 * (fTemp442 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp440, 10)) as usize] } - fTemp439))));
			let mut fTemp444: F32 = 0.5 * fTemp436;
			let mut fTemp445: F32 = 65535.0 * (fTemp386 + fTemp444);
			let mut iTemp446: i32 = (fTemp445) as i32;
			let mut fTemp447: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp379, i32::wrapping_mul(9, i32::wrapping_add(iTemp446, 1))), 589823))) as usize] };
			let mut iTemp448: i32 = i32::wrapping_add(iTemp379, i32::wrapping_mul(9, iTemp446));
			let mut fTemp449: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp448, 589823))) as usize] };
			let mut fTemp450: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp448, 1), 589823))) as usize] } - fTemp449;
			let mut iTemp451: i32 = (fTemp414 > ((fTemp383 * (fTemp450 - fTemp442) + fTemp449 + (fTemp445 - (iTemp446) as F32) * (fTemp447 - (fTemp449 + fTemp383 * (fTemp450 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp448, 10), 589823))) as usize] } - fTemp447)))) - (fTemp441 + fTemp443)) / (1.0 - (fTemp441 + fTemp383 * fTemp442 + fTemp443)))) as i32;
			let mut fTemp452: F32 = if iTemp451 != 0 {fTemp434} else {fTemp444};
			let mut fTemp453: F32 = if iTemp451 != 0 {fTemp444} else {fTemp435};
			let mut fTemp454: F32 = fTemp453 + fTemp452;
			let mut fTemp455: F32 = 32767.5 * fTemp454;
			let mut iTemp456: i32 = (fTemp455) as i32;
			let mut fTemp457: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp379, i32::wrapping_mul(9, i32::wrapping_add(iTemp456, 1)))) as usize] };
			let mut iTemp458: i32 = i32::wrapping_add(iTemp379, i32::wrapping_mul(9, iTemp456));
			let mut fTemp459: F32 = unsafe { ftbl0mydspSIG0[iTemp458 as usize] };
			let mut fTemp460: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp458, 1)) as usize] } - fTemp459;
			let mut fTemp461: F32 = (fTemp455 - (iTemp456) as F32) * (fTemp457 - (fTemp459 + fTemp383 * (fTemp460 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp458, 10)) as usize] } - fTemp457))));
			let mut fTemp462: F32 = 0.5 * fTemp454;
			let mut fTemp463: F32 = 65535.0 * (fTemp386 + fTemp462);
			let mut iTemp464: i32 = (fTemp463) as i32;
			let mut fTemp465: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp379, i32::wrapping_mul(9, i32::wrapping_add(iTemp464, 1))), 589823))) as usize] };
			let mut iTemp466: i32 = i32::wrapping_add(iTemp379, i32::wrapping_mul(9, iTemp464));
			let mut fTemp467: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp466, 589823))) as usize] };
			let mut fTemp468: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp466, 1), 589823))) as usize] } - fTemp467;
			let mut iTemp469: i32 = (fTemp414 > ((fTemp383 * (fTemp468 - fTemp460) + fTemp467 + (fTemp463 - (iTemp464) as F32) * (fTemp465 - (fTemp467 + fTemp383 * (fTemp468 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp466, 10), 589823))) as usize] } - fTemp465)))) - (fTemp459 + fTemp461)) / (1.0 - (fTemp459 + fTemp383 * fTemp460 + fTemp461)))) as i32;
			let mut fTemp470: F32 = if iTemp469 != 0 {fTemp452} else {fTemp462};
			let mut fTemp471: F32 = if iTemp469 != 0 {fTemp462} else {fTemp453};
			let mut fTemp472: F32 = fTemp471 + fTemp470;
			let mut fTemp473: F32 = 32767.5 * fTemp472;
			let mut iTemp474: i32 = (fTemp473) as i32;
			let mut fTemp475: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp379, i32::wrapping_mul(9, i32::wrapping_add(iTemp474, 1)))) as usize] };
			let mut iTemp476: i32 = i32::wrapping_add(iTemp379, i32::wrapping_mul(9, iTemp474));
			let mut fTemp477: F32 = unsafe { ftbl0mydspSIG0[iTemp476 as usize] };
			let mut fTemp478: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp476, 1)) as usize] } - fTemp477;
			let mut fTemp479: F32 = (fTemp473 - (iTemp474) as F32) * (fTemp475 - (fTemp477 + fTemp383 * (fTemp478 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp476, 10)) as usize] } - fTemp475))));
			let mut fTemp480: F32 = 0.5 * fTemp472;
			let mut fTemp481: F32 = 65535.0 * (fTemp386 + fTemp480);
			let mut iTemp482: i32 = (fTemp481) as i32;
			let mut fTemp483: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp379, i32::wrapping_mul(9, i32::wrapping_add(iTemp482, 1))), 589823))) as usize] };
			let mut iTemp484: i32 = i32::wrapping_add(iTemp379, i32::wrapping_mul(9, iTemp482));
			let mut fTemp485: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp484, 589823))) as usize] };
			let mut fTemp486: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp484, 1), 589823))) as usize] } - fTemp485;
			let mut iTemp487: i32 = (fTemp414 > ((fTemp383 * (fTemp486 - fTemp478) + fTemp485 + (fTemp481 - (iTemp482) as F32) * (fTemp483 - (fTemp485 + fTemp383 * (fTemp486 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp484, 10), 589823))) as usize] } - fTemp483)))) - (fTemp477 + fTemp479)) / (1.0 - (fTemp477 + fTemp383 * fTemp478 + fTemp479)))) as i32;
			let mut fTemp488: F32 = if iTemp487 != 0 {fTemp470} else {fTemp480};
			let mut fTemp489: F32 = if iTemp487 != 0 {fTemp480} else {fTemp471};
			let mut fTemp490: F32 = fTemp489 + fTemp488;
			let mut fTemp491: F32 = 32767.5 * fTemp490;
			let mut iTemp492: i32 = (fTemp491) as i32;
			let mut fTemp493: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp379, i32::wrapping_mul(9, i32::wrapping_add(iTemp492, 1)))) as usize] };
			let mut iTemp494: i32 = i32::wrapping_add(iTemp379, i32::wrapping_mul(9, iTemp492));
			let mut fTemp495: F32 = unsafe { ftbl0mydspSIG0[iTemp494 as usize] };
			let mut fTemp496: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp494, 1)) as usize] } - fTemp495;
			let mut fTemp497: F32 = (fTemp491 - (iTemp492) as F32) * (fTemp493 - (fTemp495 + fTemp383 * (fTemp496 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp494, 10)) as usize] } - fTemp493))));
			let mut fTemp498: F32 = 0.5 * fTemp490;
			let mut fTemp499: F32 = 65535.0 * (fTemp386 + fTemp498);
			let mut iTemp500: i32 = (fTemp499) as i32;
			let mut fTemp501: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp379, i32::wrapping_mul(9, i32::wrapping_add(iTemp500, 1))), 589823))) as usize] };
			let mut iTemp502: i32 = i32::wrapping_add(iTemp379, i32::wrapping_mul(9, iTemp500));
			let mut fTemp503: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp502, 589823))) as usize] };
			let mut fTemp504: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp502, 1), 589823))) as usize] } - fTemp503;
			let mut iTemp505: i32 = (fTemp414 > ((fTemp383 * (fTemp504 - fTemp496) + fTemp503 + (fTemp499 - (iTemp500) as F32) * (fTemp501 - (fTemp503 + fTemp383 * (fTemp504 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp502, 10), 589823))) as usize] } - fTemp501)))) - (fTemp495 + fTemp497)) / (1.0 - (fTemp495 + fTemp383 * fTemp496 + fTemp497)))) as i32;
			let mut fTemp506: F32 = if iTemp505 != 0 {fTemp488} else {fTemp498};
			let mut fTemp507: F32 = if iTemp505 != 0 {fTemp498} else {fTemp489};
			let mut fTemp508: F32 = fTemp507 + fTemp506;
			let mut fTemp509: F32 = 32767.5 * fTemp508;
			let mut iTemp510: i32 = (fTemp509) as i32;
			let mut fTemp511: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp379, i32::wrapping_mul(9, i32::wrapping_add(iTemp510, 1)))) as usize] };
			let mut iTemp512: i32 = i32::wrapping_add(iTemp379, i32::wrapping_mul(9, iTemp510));
			let mut fTemp513: F32 = unsafe { ftbl0mydspSIG0[iTemp512 as usize] };
			let mut fTemp514: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp512, 1)) as usize] } - fTemp513;
			let mut fTemp515: F32 = (fTemp509 - (iTemp510) as F32) * (fTemp511 - (fTemp513 + fTemp383 * (fTemp514 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp512, 10)) as usize] } - fTemp511))));
			let mut fTemp516: F32 = 0.5 * fTemp508;
			let mut fTemp517: F32 = 65535.0 * (fTemp386 + fTemp516);
			let mut iTemp518: i32 = (fTemp517) as i32;
			let mut fTemp519: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp379, i32::wrapping_mul(9, i32::wrapping_add(iTemp518, 1))), 589823))) as usize] };
			let mut iTemp520: i32 = i32::wrapping_add(iTemp379, i32::wrapping_mul(9, iTemp518));
			let mut fTemp521: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp520, 589823))) as usize] };
			let mut fTemp522: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp520, 1), 589823))) as usize] } - fTemp521;
			let mut iTemp523: i32 = (fTemp414 > ((fTemp383 * (fTemp522 - fTemp514) + fTemp521 + (fTemp517 - (iTemp518) as F32) * (fTemp519 - (fTemp521 + fTemp383 * (fTemp522 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp520, 10), 589823))) as usize] } - fTemp519)))) - (fTemp513 + fTemp515)) / (1.0 - (fTemp513 + fTemp383 * fTemp514 + fTemp515)))) as i32;
			let mut fTemp524: F32 = if iTemp523 != 0 {fTemp506} else {fTemp516};
			let mut fTemp525: F32 = if iTemp523 != 0 {fTemp516} else {fTemp507};
			let mut fTemp526: F32 = fTemp525 + fTemp524;
			let mut fTemp527: F32 = 32767.5 * fTemp526;
			let mut iTemp528: i32 = (fTemp527) as i32;
			let mut fTemp529: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp379, i32::wrapping_mul(9, i32::wrapping_add(iTemp528, 1)))) as usize] };
			let mut iTemp530: i32 = i32::wrapping_add(iTemp379, i32::wrapping_mul(9, iTemp528));
			let mut fTemp531: F32 = unsafe { ftbl0mydspSIG0[iTemp530 as usize] };
			let mut fTemp532: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp530, 1)) as usize] } - fTemp531;
			let mut fTemp533: F32 = (fTemp527 - (iTemp528) as F32) * (fTemp529 - (fTemp531 + fTemp383 * (fTemp532 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp530, 10)) as usize] } - fTemp529))));
			let mut fTemp534: F32 = 0.5 * fTemp526;
			let mut fTemp535: F32 = 65535.0 * (fTemp386 + fTemp534);
			let mut iTemp536: i32 = (fTemp535) as i32;
			let mut fTemp537: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp379, i32::wrapping_mul(9, i32::wrapping_add(iTemp536, 1))), 589823))) as usize] };
			let mut iTemp538: i32 = i32::wrapping_add(iTemp379, i32::wrapping_mul(9, iTemp536));
			let mut fTemp539: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp538, 589823))) as usize] };
			let mut fTemp540: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp538, 1), 589823))) as usize] } - fTemp539;
			let mut iTemp541: i32 = (fTemp414 > ((fTemp383 * (fTemp540 - fTemp532) + fTemp539 + (fTemp535 - (iTemp536) as F32) * (fTemp537 - (fTemp539 + fTemp383 * (fTemp540 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp538, 10), 589823))) as usize] } - fTemp537)))) - (fTemp531 + fTemp533)) / (1.0 - (fTemp531 + fTemp383 * fTemp532 + fTemp533)))) as i32;
			let mut fTemp542: F32 = if iTemp541 != 0 {fTemp524} else {fTemp534};
			let mut fTemp543: F32 = if iTemp541 != 0 {fTemp534} else {fTemp525};
			let mut fTemp544: F32 = fTemp543 + fTemp542;
			let mut fTemp545: F32 = 32767.5 * fTemp544;
			let mut iTemp546: i32 = (fTemp545) as i32;
			let mut fTemp547: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp379, i32::wrapping_mul(9, i32::wrapping_add(iTemp546, 1)))) as usize] };
			let mut iTemp548: i32 = i32::wrapping_add(iTemp379, i32::wrapping_mul(9, iTemp546));
			let mut fTemp549: F32 = unsafe { ftbl0mydspSIG0[iTemp548 as usize] };
			let mut fTemp550: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp548, 1)) as usize] } - fTemp549;
			let mut fTemp551: F32 = (fTemp545 - (iTemp546) as F32) * (fTemp547 - (fTemp549 + fTemp383 * (fTemp550 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp548, 10)) as usize] } - fTemp547))));
			let mut fTemp552: F32 = 0.5 * fTemp544;
			let mut fTemp553: F32 = 65535.0 * (fTemp386 + fTemp552);
			let mut iTemp554: i32 = (fTemp553) as i32;
			let mut fTemp555: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp379, i32::wrapping_mul(9, i32::wrapping_add(iTemp554, 1))), 589823))) as usize] };
			let mut iTemp556: i32 = i32::wrapping_add(iTemp379, i32::wrapping_mul(9, iTemp554));
			let mut fTemp557: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp556, 589823))) as usize] };
			let mut fTemp558: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp556, 1), 589823))) as usize] } - fTemp557;
			let mut iTemp559: i32 = (fTemp414 > ((fTemp383 * (fTemp558 - fTemp550) + fTemp557 + (fTemp553 - (iTemp554) as F32) * (fTemp555 - (fTemp557 + fTemp383 * (fTemp558 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp556, 10), 589823))) as usize] } - fTemp555)))) - (fTemp549 + fTemp551)) / (1.0 - (fTemp549 + fTemp383 * fTemp550 + fTemp551)))) as i32;
			let mut fTemp560: F32 = if iTemp559 != 0 {fTemp542} else {fTemp552};
			let mut fTemp561: F32 = if iTemp559 != 0 {fTemp552} else {fTemp543};
			let mut fTemp562: F32 = fTemp561 + fTemp560;
			let mut fTemp563: F32 = 32767.5 * fTemp562;
			let mut iTemp564: i32 = (fTemp563) as i32;
			let mut fTemp565: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp379, i32::wrapping_mul(9, i32::wrapping_add(iTemp564, 1)))) as usize] };
			let mut iTemp566: i32 = i32::wrapping_add(iTemp379, i32::wrapping_mul(9, iTemp564));
			let mut fTemp567: F32 = unsafe { ftbl0mydspSIG0[iTemp566 as usize] };
			let mut fTemp568: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp566, 1)) as usize] } - fTemp567;
			let mut fTemp569: F32 = (fTemp563 - (iTemp564) as F32) * (fTemp565 - (fTemp567 + fTemp383 * (fTemp568 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp566, 10)) as usize] } - fTemp565))));
			let mut fTemp570: F32 = 0.5 * fTemp562;
			let mut fTemp571: F32 = 65535.0 * (fTemp386 + fTemp570);
			let mut iTemp572: i32 = (fTemp571) as i32;
			let mut fTemp573: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp379, i32::wrapping_mul(9, i32::wrapping_add(iTemp572, 1))), 589823))) as usize] };
			let mut iTemp574: i32 = i32::wrapping_add(iTemp379, i32::wrapping_mul(9, iTemp572));
			let mut fTemp575: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp574, 589823))) as usize] };
			let mut fTemp576: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp574, 1), 589823))) as usize] } - fTemp575;
			let mut iTemp577: i32 = (fTemp414 > ((fTemp383 * (fTemp576 - fTemp568) + fTemp575 + (fTemp571 - (iTemp572) as F32) * (fTemp573 - (fTemp575 + fTemp383 * (fTemp576 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp574, 10), 589823))) as usize] } - fTemp573)))) - (fTemp567 + fTemp569)) / (1.0 - (fTemp567 + fTemp383 * fTemp568 + fTemp569)))) as i32;
			let mut fTemp578: F32 = if iTemp577 != 0 {fTemp560} else {fTemp570};
			let mut fTemp579: F32 = if iTemp577 != 0 {fTemp570} else {fTemp561};
			let mut fTemp580: F32 = fTemp579 + fTemp578;
			let mut fTemp581: F32 = 32767.5 * fTemp580;
			let mut iTemp582: i32 = (fTemp581) as i32;
			let mut fTemp583: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp379, i32::wrapping_mul(9, i32::wrapping_add(iTemp582, 1)))) as usize] };
			let mut iTemp584: i32 = i32::wrapping_add(iTemp379, i32::wrapping_mul(9, iTemp582));
			let mut fTemp585: F32 = unsafe { ftbl0mydspSIG0[iTemp584 as usize] };
			let mut fTemp586: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp584, 1)) as usize] } - fTemp585;
			let mut fTemp587: F32 = (fTemp581 - (iTemp582) as F32) * (fTemp583 - (fTemp585 + fTemp383 * (fTemp586 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp584, 10)) as usize] } - fTemp583))));
			let mut fTemp588: F32 = 0.5 * fTemp580;
			let mut fTemp589: F32 = 65535.0 * (fTemp386 + fTemp588);
			let mut iTemp590: i32 = (fTemp589) as i32;
			let mut fTemp591: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp379, i32::wrapping_mul(9, i32::wrapping_add(iTemp590, 1))), 589823))) as usize] };
			let mut iTemp592: i32 = i32::wrapping_add(iTemp379, i32::wrapping_mul(9, iTemp590));
			let mut fTemp593: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp592, 589823))) as usize] };
			let mut fTemp594: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp592, 1), 589823))) as usize] } - fTemp593;
			let mut iTemp595: i32 = (fTemp414 > ((fTemp383 * (fTemp594 - fTemp586) + fTemp593 + (fTemp589 - (iTemp590) as F32) * (fTemp591 - (fTemp593 + fTemp383 * (fTemp594 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp592, 10), 589823))) as usize] } - fTemp591)))) - (fTemp585 + fTemp587)) / (1.0 - (fTemp585 + fTemp383 * fTemp586 + fTemp587)))) as i32;
			let mut fTemp596: F32 = if iTemp595 != 0 {fTemp578} else {fTemp588};
			let mut fTemp597: F32 = if iTemp595 != 0 {fTemp588} else {fTemp579};
			let mut fTemp598: F32 = fTemp597 + fTemp596;
			let mut fTemp599: F32 = 32767.5 * fTemp598;
			let mut iTemp600: i32 = (fTemp599) as i32;
			let mut fTemp601: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp379, i32::wrapping_mul(9, i32::wrapping_add(iTemp600, 1)))) as usize] };
			let mut iTemp602: i32 = i32::wrapping_add(iTemp379, i32::wrapping_mul(9, iTemp600));
			let mut fTemp603: F32 = unsafe { ftbl0mydspSIG0[iTemp602 as usize] };
			let mut fTemp604: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp602, 1)) as usize] } - fTemp603;
			let mut fTemp605: F32 = (fTemp599 - (iTemp600) as F32) * (fTemp601 - (fTemp603 + fTemp383 * (fTemp604 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp602, 10)) as usize] } - fTemp601))));
			let mut fTemp606: F32 = 0.5 * fTemp598;
			let mut fTemp607: F32 = 65535.0 * (fTemp386 + fTemp606);
			let mut iTemp608: i32 = (fTemp607) as i32;
			let mut fTemp609: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp379, i32::wrapping_mul(9, i32::wrapping_add(iTemp608, 1))), 589823))) as usize] };
			let mut iTemp610: i32 = i32::wrapping_add(iTemp379, i32::wrapping_mul(9, iTemp608));
			let mut fTemp611: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp610, 589823))) as usize] };
			let mut fTemp612: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp610, 1), 589823))) as usize] } - fTemp611;
			let mut iTemp613: i32 = (fTemp414 > ((fTemp383 * (fTemp612 - fTemp604) + fTemp611 + (fTemp607 - (iTemp608) as F32) * (fTemp609 - (fTemp611 + fTemp383 * (fTemp612 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp610, 10), 589823))) as usize] } - fTemp609)))) - (fTemp603 + fTemp605)) / (1.0 - (fTemp603 + fTemp383 * fTemp604 + fTemp605)))) as i32;
			let mut fTemp614: F32 = if iTemp613 != 0 {fTemp596} else {fTemp606};
			let mut fTemp615: F32 = if iTemp613 != 0 {fTemp606} else {fTemp597};
			let mut fTemp616: F32 = fTemp615 + fTemp614;
			let mut fTemp617: F32 = 32767.5 * fTemp616;
			let mut iTemp618: i32 = (fTemp617) as i32;
			let mut fTemp619: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp379, i32::wrapping_mul(9, i32::wrapping_add(iTemp618, 1)))) as usize] };
			let mut iTemp620: i32 = i32::wrapping_add(iTemp379, i32::wrapping_mul(9, iTemp618));
			let mut fTemp621: F32 = unsafe { ftbl0mydspSIG0[iTemp620 as usize] };
			let mut fTemp622: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp620, 1)) as usize] } - fTemp621;
			let mut fTemp623: F32 = (fTemp617 - (iTemp618) as F32) * (fTemp619 - (fTemp621 + fTemp383 * (fTemp622 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp620, 10)) as usize] } - fTemp619))));
			let mut fTemp624: F32 = 0.5 * fTemp616;
			let mut fTemp625: F32 = 65535.0 * (fTemp386 + fTemp624);
			let mut iTemp626: i32 = (fTemp625) as i32;
			let mut fTemp627: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp379, i32::wrapping_mul(9, i32::wrapping_add(iTemp626, 1))), 589823))) as usize] };
			let mut iTemp628: i32 = i32::wrapping_add(iTemp379, i32::wrapping_mul(9, iTemp626));
			let mut fTemp629: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp628, 589823))) as usize] };
			let mut fTemp630: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp628, 1), 589823))) as usize] } - fTemp629;
			let mut iTemp631: i32 = (fTemp414 > ((fTemp383 * (fTemp630 - fTemp622) + fTemp629 + (fTemp625 - (iTemp626) as F32) * (fTemp627 - (fTemp629 + fTemp383 * (fTemp630 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp628, 10), 589823))) as usize] } - fTemp627)))) - (fTemp621 + fTemp623)) / (1.0 - (fTemp621 + fTemp383 * fTemp622 + fTemp623)))) as i32;
			let mut fTemp632: F32 = if iTemp631 != 0 {fTemp614} else {fTemp624};
			let mut fTemp633: F32 = if iTemp631 != 0 {fTemp624} else {fTemp615};
			let mut fTemp634: F32 = fTemp633 + fTemp632;
			let mut fTemp635: F32 = 32767.5 * fTemp634;
			let mut iTemp636: i32 = (fTemp635) as i32;
			let mut fTemp637: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp379, i32::wrapping_mul(9, i32::wrapping_add(iTemp636, 1)))) as usize] };
			let mut iTemp638: i32 = i32::wrapping_add(iTemp379, i32::wrapping_mul(9, iTemp636));
			let mut fTemp639: F32 = unsafe { ftbl0mydspSIG0[iTemp638 as usize] };
			let mut fTemp640: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp638, 1)) as usize] } - fTemp639;
			let mut fTemp641: F32 = (fTemp635 - (iTemp636) as F32) * (fTemp637 - (fTemp639 + fTemp383 * (fTemp640 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp638, 10)) as usize] } - fTemp637))));
			let mut fTemp642: F32 = 0.5 * fTemp634;
			let mut fTemp643: F32 = 65535.0 * (fTemp386 + fTemp642);
			let mut iTemp644: i32 = (fTemp643) as i32;
			let mut fTemp645: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp379, i32::wrapping_mul(9, i32::wrapping_add(iTemp644, 1))), 589823))) as usize] };
			let mut iTemp646: i32 = i32::wrapping_add(iTemp379, i32::wrapping_mul(9, iTemp644));
			let mut fTemp647: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp646, 589823))) as usize] };
			let mut fTemp648: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp646, 1), 589823))) as usize] } - fTemp647;
			let mut iTemp649: i32 = (fTemp414 > ((fTemp383 * (fTemp648 - fTemp640) + fTemp647 + (fTemp643 - (iTemp644) as F32) * (fTemp645 - (fTemp647 + fTemp383 * (fTemp648 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp646, 10), 589823))) as usize] } - fTemp645)))) - (fTemp639 + fTemp641)) / (1.0 - (fTemp639 + fTemp383 * fTemp640 + fTemp641)))) as i32;
			let mut fTemp650: F32 = if iTemp649 != 0 {fTemp632} else {fTemp642};
			let mut fTemp651: F32 = if iTemp649 != 0 {fTemp642} else {fTemp633};
			let mut fTemp652: F32 = fTemp651 + fTemp650;
			let mut fTemp653: F32 = 32767.5 * fTemp652;
			let mut iTemp654: i32 = (fTemp653) as i32;
			let mut fTemp655: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp379, i32::wrapping_mul(9, i32::wrapping_add(iTemp654, 1)))) as usize] };
			let mut iTemp656: i32 = i32::wrapping_add(iTemp379, i32::wrapping_mul(9, iTemp654));
			let mut fTemp657: F32 = unsafe { ftbl0mydspSIG0[iTemp656 as usize] };
			let mut fTemp658: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp656, 1)) as usize] } - fTemp657;
			let mut fTemp659: F32 = (fTemp653 - (iTemp654) as F32) * (fTemp655 - (fTemp657 + fTemp383 * (fTemp658 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp656, 10)) as usize] } - fTemp655))));
			let mut fTemp660: F32 = 0.5 * fTemp652;
			let mut fTemp661: F32 = 65535.0 * (fTemp386 + fTemp660);
			let mut iTemp662: i32 = (fTemp661) as i32;
			let mut fTemp663: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp379, i32::wrapping_mul(9, i32::wrapping_add(iTemp662, 1))), 589823))) as usize] };
			let mut iTemp664: i32 = i32::wrapping_add(iTemp379, i32::wrapping_mul(9, iTemp662));
			let mut fTemp665: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp664, 589823))) as usize] };
			let mut fTemp666: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, 1), 589823))) as usize] } - fTemp665;
			let mut iTemp667: i32 = (fTemp414 > ((fTemp383 * (fTemp666 - fTemp658) + fTemp665 + (fTemp661 - (iTemp662) as F32) * (fTemp663 - (fTemp665 + fTemp383 * (fTemp666 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp664, 10), 589823))) as usize] } - fTemp663)))) - (fTemp657 + fTemp659)) / (1.0 - (fTemp657 + fTemp383 * fTemp658 + fTemp659)))) as i32;
			let mut fTemp668: F32 = if iTemp667 != 0 {fTemp650} else {fTemp660};
			let mut fTemp669: F32 = if iTemp667 != 0 {fTemp660} else {fTemp651};
			let mut fTemp670: F32 = fTemp669 + fTemp668;
			let mut fTemp671: F32 = 32767.5 * fTemp670;
			let mut iTemp672: i32 = (fTemp671) as i32;
			let mut fTemp673: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp379, i32::wrapping_mul(9, i32::wrapping_add(iTemp672, 1)))) as usize] };
			let mut iTemp674: i32 = i32::wrapping_add(iTemp379, i32::wrapping_mul(9, iTemp672));
			let mut fTemp675: F32 = unsafe { ftbl0mydspSIG0[iTemp674 as usize] };
			let mut fTemp676: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp674, 1)) as usize] } - fTemp675;
			let mut fTemp677: F32 = (fTemp671 - (iTemp672) as F32) * (fTemp673 - (fTemp675 + fTemp383 * (fTemp676 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp674, 10)) as usize] } - fTemp673))));
			let mut fTemp678: F32 = 0.5 * fTemp670;
			let mut fTemp679: F32 = 65535.0 * (fTemp386 + fTemp678);
			let mut iTemp680: i32 = (fTemp679) as i32;
			let mut fTemp681: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp379, i32::wrapping_mul(9, i32::wrapping_add(iTemp680, 1))), 589823))) as usize] };
			let mut iTemp682: i32 = i32::wrapping_add(iTemp379, i32::wrapping_mul(9, iTemp680));
			let mut fTemp683: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp682, 589823))) as usize] };
			let mut fTemp684: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp682, 1), 589823))) as usize] } - fTemp683;
			let mut iTemp685: i32 = (fTemp414 > ((fTemp383 * (fTemp684 - fTemp676) + fTemp683 + (fTemp679 - (iTemp680) as F32) * (fTemp681 - (fTemp683 + fTemp383 * (fTemp684 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp682, 10), 589823))) as usize] } - fTemp681)))) - (fTemp675 + fTemp677)) / (1.0 - (fTemp675 + fTemp383 * fTemp676 + fTemp677)))) as i32;
			let mut fTemp686: F32 = F32::min(1.0, F32::max(0.0, 0.5 * (if iTemp685 != 0 {fTemp678} else {fTemp669} + if iTemp685 != 0 {fTemp668} else {fTemp678})));
			self.fRec4[0] = fTemp686;
			let mut fTemp687: F32 = 65535.0 * fTemp686;
			let mut iTemp688: i32 = (fTemp687) as i32;
			let mut fTemp689: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp379, i32::wrapping_mul(9, i32::wrapping_add(iTemp688, 1)))) as usize] };
			let mut iTemp690: i32 = i32::wrapping_add(iTemp379, i32::wrapping_mul(9, iTemp688));
			let mut fTemp691: F32 = unsafe { ftbl0mydspSIG0[iTemp690 as usize] };
			let mut fTemp692: F32 = unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp690, 1)) as usize] } - fTemp691;
			let mut fTemp693: F32 = (fTemp687 - (iTemp688) as F32) * (fTemp689 - (fTemp691 + fTemp383 * (fTemp692 - (unsafe { ftbl0mydspSIG0[(i32::wrapping_add(iTemp690, 10)) as usize] } - fTemp689))));
			let mut fTemp694: F32 = 65535.0 * (fTemp386 + fTemp686);
			let mut iTemp695: i32 = (fTemp694) as i32;
			let mut fTemp696: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp379, i32::wrapping_mul(9, i32::wrapping_add(iTemp695, 1))), 589823))) as usize] };
			let mut iTemp697: i32 = i32::wrapping_add(iTemp379, i32::wrapping_mul(9, iTemp695));
			let mut fTemp698: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(iTemp697, 589823))) as usize] };
			let mut fTemp699: F32 = unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp697, 1), 589823))) as usize] } - fTemp698;
			let mut fTemp700: F32 = fTemp375 * (fTemp383 * (fTemp699 - fTemp692) + fTemp698 + (fTemp694 - (iTemp695) as F32) * (fTemp696 - (fTemp698 + fTemp383 * (fTemp699 - (unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(i32::wrapping_add(iTemp697, 10), 589823))) as usize] } - fTemp696)))) - (fTemp691 + fTemp693)) / (1.0 - (fTemp691 + fTemp383 * fTemp692 + fTemp693));
			self.fRec5[0] = self.fRec5[1] + if iTemp376 != 0 {F32::min(fTemp375, fTemp700)} else {F32::max(fTemp375, fTemp700)};
			self.fHbargraph1 = F32::min(0.0, F32::max(-24.0, 2e+01 * F32::log10(F32::max(1.1754944e-38, self.fRec5[0]))));
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
			self.fVec17[1] = self.fVec17[0];
			self.fVec18[1] = self.fVec18[0];
			self.fVec19[1] = self.fVec19[0];
			self.fRec0[1] = self.fRec0[0];
			self.fRec1[1] = self.fRec1[0];
			self.fVec20[1] = self.fVec20[0];
			self.fVec21[2] = self.fVec21[1];
			self.fVec21[1] = self.fVec21[0];
			for j2 in (1..=6).rev() {
				self.fVec22[j2 as usize] = self.fVec22[(i32::wrapping_sub(j2, 1)) as usize];
			}
			for j3 in (1..=14).rev() {
				self.fVec23[j3 as usize] = self.fVec23[(i32::wrapping_sub(j3, 1)) as usize];
			}
			self.fVec35[1] = self.fVec35[0];
			self.fVec36[1] = self.fVec36[0];
			self.fVec37[1] = self.fVec37[0];
			self.fRec4[1] = self.fRec4[0];
			self.fRec5[1] = self.fRec5[0];
		}
	}

}


}
pub use dsp::mydsp as Lamb;
