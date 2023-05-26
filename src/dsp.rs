mod dsp {
    /* ------------------------------------------------------------
name: "gain"
Code generated with Faust 2.59.6 (https://faust.grame.fr)
Compilation options: -a /tmp/.tmpnYdMtk -lang rust -ct 1 -es 1 -mcd 16 -single -ftz 0
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


fn mydsp_faustpower2_f(value: F32) -> F32 {
	return value * value;
}

#[cfg_attr(feature = "default-boxed", derive(default_boxed::DefaultBoxed))]
#[repr(C)]
#[derive(Debug,Clone)]
pub struct mydsp {
	fSampleRate: i32,
	fConst1: F32,
	fConst2: F32,
	fHslider0: F32,
	fRec3: [F32;2],
	fConst3: F32,
	fConst4: F32,
	fRec4: [F32;2],
	fRec5: [F32;2],
	fRec0: [F32;2],
	fRec1: [F32;2],
	fRec10: [F32;2],
	fRec11: [F32;2],
	fRec7: [F32;2],
	fRec8: [F32;2],
}

impl FaustDsp for mydsp {
	type T = F32;
		
	fn new() -> mydsp { 
		mydsp {
			fSampleRate: 0,
			fConst1: 0.0,
			fConst2: 0.0,
			fHslider0: 0.0,
			fRec3: [0.0;2],
			fConst3: 0.0,
			fConst4: 0.0,
			fRec4: [0.0;2],
			fRec5: [0.0;2],
			fRec0: [0.0;2],
			fRec1: [0.0;2],
			fRec10: [0.0;2],
			fRec11: [0.0;2],
			fRec7: [0.0;2],
			fRec8: [0.0;2],
		}
	}
	fn metadata(&self, m: &mut dyn Meta) { 
		m.declare("compile_options", "-a /tmp/.tmpnYdMtk -lang rust -ct 1 -es 1 -mcd 16 -single -ftz 0");
		m.declare("filename", "gain.dsp");
		m.declare("filters.lib/lowpass0_highpass1", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/lowpassLR4:author", "Dario Sanfilippo");
		m.declare("filters.lib/lowpassLR4:copyright", "Copyright (C) 2022 Dario Sanfilippo <sanfilippo.dario@gmail.com>");
		m.declare("filters.lib/lowpassLR4:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/name", "Faust Filters Library");
		m.declare("filters.lib/svf:author", "Oleg Nesterov");
		m.declare("filters.lib/svf:copyright", "Copyright (C) 2020 Oleg Nesterov <oleg@redhat.com>");
		m.declare("filters.lib/svf:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/version", "0.3");
		m.declare("maths.lib/author", "GRAME");
		m.declare("maths.lib/copyright", "GRAME");
		m.declare("maths.lib/license", "LGPL with exception");
		m.declare("maths.lib/name", "Faust Math Library");
		m.declare("maths.lib/version", "2.6");
		m.declare("name", "gain");
		m.declare("platform.lib/name", "Generic Platform Library");
		m.declare("platform.lib/version", "0.3");
		m.declare("routes.lib/name", "Faust Signal Routing Library");
		m.declare("routes.lib/version", "0.2");
		m.declare("signals.lib/name", "Faust Signal Routing Library");
		m.declare("signals.lib/version", "0.3");
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
	}
	fn instance_reset_params(&mut self) {
		self.fHslider0 = 1.0;
	}
	fn instance_clear(&mut self) {
		for l0 in 0..2 {
			self.fRec3[(l0) as usize] = 0.0;
		}
		for l1 in 0..2 {
			self.fRec4[(l1) as usize] = 0.0;
		}
		for l2 in 0..2 {
			self.fRec5[(l2) as usize] = 0.0;
		}
		for l3 in 0..2 {
			self.fRec0[(l3) as usize] = 0.0;
		}
		for l4 in 0..2 {
			self.fRec1[(l4) as usize] = 0.0;
		}
		for l5 in 0..2 {
			self.fRec10[(l5) as usize] = 0.0;
		}
		for l6 in 0..2 {
			self.fRec11[(l6) as usize] = 0.0;
		}
		for l7 in 0..2 {
			self.fRec7[(l7) as usize] = 0.0;
		}
		for l8 in 0..2 {
			self.fRec8[(l8) as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		let mut fConst0: F32 = F32::min(1.92e+05, F32::max(1.0, ((self.fSampleRate) as F32)));
		self.fConst1 = 44.1 / fConst0;
		self.fConst2 = 1.0 - self.fConst1;
		self.fConst3 = 0.5 * fConst0 + -4e+01;
		self.fConst4 = 3.1415927 / fConst0;
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
		ui_interface.open_vertical_box("gain");
		ui_interface.add_horizontal_slider("CutOff", ParamIndex(0), 1.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			0 => Some(self.fHslider0),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			0 => { self.fHslider0 = value }
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
		let mut fSlow0: F32 = self.fConst1 * self.fHslider0;
		let zipped_iterators = inputs0.zip(inputs1).zip(outputs0).zip(outputs1);
		for (((input0, input1), output0), output1) in zipped_iterators {
			self.fRec3[0] = fSlow0 + self.fConst2 * self.fRec3[1];
			let mut fTemp0: F32 = F32::tan(self.fConst4 * (self.fConst3 * mydsp_faustpower2_f(self.fRec3[0]) + 4e+01));
			let mut fTemp1: F32 = fTemp0 * (fTemp0 + 1.4142135) + 1.0;
			let mut fTemp2: F32 = *input0;
			let mut fTemp3: F32 = self.fRec4[1] + fTemp0 * (fTemp2 - self.fRec5[1]);
			let mut fTemp4: F32 = fTemp3 / fTemp1;
			self.fRec4[0] = 2.0 * fTemp4 - self.fRec4[1];
			let mut fTemp5: F32 = self.fRec5[1] + fTemp0 * fTemp3 / fTemp1;
			self.fRec5[0] = 2.0 * fTemp5 - self.fRec5[1];
			let mut fRec6: F32 = fTemp5;
			let mut fTemp6: F32 = self.fRec0[1] + fTemp0 * (fRec6 - self.fRec1[1]);
			let mut fTemp7: F32 = fTemp6 / fTemp1;
			self.fRec0[0] = 2.0 * fTemp7 - self.fRec0[1];
			let mut fTemp8: F32 = self.fRec1[1] + fTemp0 * fTemp6 / fTemp1;
			self.fRec1[0] = 2.0 * fTemp8 - self.fRec1[1];
			let mut fRec2: F32 = fTemp8;
			*output0 = fRec2;
			let mut fTemp9: F32 = *input1;
			let mut fTemp10: F32 = self.fRec10[1] + fTemp0 * (fTemp9 - self.fRec11[1]);
			let mut fTemp11: F32 = fTemp10 / fTemp1;
			self.fRec10[0] = 2.0 * fTemp11 - self.fRec10[1];
			let mut fTemp12: F32 = self.fRec11[1] + fTemp0 * fTemp10 / fTemp1;
			self.fRec11[0] = 2.0 * fTemp12 - self.fRec11[1];
			let mut fRec12: F32 = fTemp12;
			let mut fTemp13: F32 = self.fRec7[1] + fTemp0 * (fRec12 - self.fRec8[1]);
			let mut fTemp14: F32 = fTemp13 / fTemp1;
			self.fRec7[0] = 2.0 * fTemp14 - self.fRec7[1];
			let mut fTemp15: F32 = self.fRec8[1] + fTemp0 * fTemp13 / fTemp1;
			self.fRec8[0] = 2.0 * fTemp15 - self.fRec8[1];
			let mut fRec9: F32 = fTemp15;
			*output1 = fRec9;
			self.fRec3[1] = self.fRec3[0];
			self.fRec4[1] = self.fRec4[0];
			self.fRec5[1] = self.fRec5[0];
			self.fRec0[1] = self.fRec0[0];
			self.fRec1[1] = self.fRec1[0];
			self.fRec10[1] = self.fRec10[0];
			self.fRec11[1] = self.fRec11[0];
			self.fRec7[1] = self.fRec7[0];
			self.fRec8[1] = self.fRec8[0];
		}
	}

}


}
pub use dsp::mydsp as Gain;
