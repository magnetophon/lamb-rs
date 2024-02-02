mod dsp {
    /* ------------------------------------------------------------
name: "gain"
Code generated with Faust 2.70.3 (https://faust.grame.fr)
Compilation options: -a /run/user/1001/.tmpdzjLEd -lang rust -ct 1 -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0
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
	fHslider1: F32,
	fRec4: [F32;2],
	fRec0: [F32;2],
	fRec1: [F32;2],
	fRec5: [F32;2],
	fRec6: [F32;2],
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
			fHslider1: 0.0,
			fRec4: [0.0;2],
			fRec0: [0.0;2],
			fRec1: [0.0;2],
			fRec5: [0.0;2],
			fRec6: [0.0;2],
		}
	}
	fn metadata(&self, m: &mut dyn Meta) { 
		m.declare("compile_options", r"-a /run/user/1001/.tmpdzjLEd -lang rust -ct 1 -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0");
		m.declare("filename", r"gain.dsp");
		m.declare("filters.lib/lowpass0_highpass1", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/name", r"Faust Filters Library");
		m.declare("filters.lib/svf:author", r"Oleg Nesterov");
		m.declare("filters.lib/svf:copyright", r"Copyright (C) 2020 Oleg Nesterov <oleg@redhat.com>");
		m.declare("filters.lib/svf:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/version", r"1.3.0");
		m.declare("maths.lib/author", r"GRAME");
		m.declare("maths.lib/copyright", r"GRAME");
		m.declare("maths.lib/license", r"LGPL with exception");
		m.declare("maths.lib/name", r"Faust Math Library");
		m.declare("maths.lib/version", r"2.7.0");
		m.declare("name", r"gain");
		m.declare("platform.lib/name", r"Generic Platform Library");
		m.declare("platform.lib/version", r"1.3.0");
		m.declare("routes.lib/name", r"Faust Signal Routing Library");
		m.declare("routes.lib/version", r"1.2.0");
		m.declare("signals.lib/name", r"Faust Signal Routing Library");
		m.declare("signals.lib/version", r"1.5.0");
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
		self.fHslider1 = 1.0;
	}
	fn instance_clear(&mut self) {
		for l0 in 0..2 {
			self.fRec3[l0 as usize] = 0.0;
		}
		for l1 in 0..2 {
			self.fRec4[l1 as usize] = 0.0;
		}
		for l2 in 0..2 {
			self.fRec0[l2 as usize] = 0.0;
		}
		for l3 in 0..2 {
			self.fRec1[l3 as usize] = 0.0;
		}
		for l4 in 0..2 {
			self.fRec5[l4 as usize] = 0.0;
		}
		for l5 in 0..2 {
			self.fRec6[l5 as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		let mut fConst0: F32 = F32::min(1.92e+05, F32::max(1.0, (self.fSampleRate) as F32));
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
		ui_interface.add_horizontal_slider("Resonance", ParamIndex(1), 1.0, 0.0, 2.0, 0.001);
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			0 => Some(self.fHslider0),
			1 => Some(self.fHslider1),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			0 => { self.fHslider0 = value }
			1 => { self.fHslider1 = value }
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
		let mut fSlow1: F32 = self.fConst1 * self.fHslider1;
		let zipped_iterators = inputs0.zip(inputs1).zip(outputs0).zip(outputs1);
		for (((input0, input1), output0), output1) in zipped_iterators {
			self.fRec3[0] = fSlow0 + self.fConst2 * self.fRec3[1];
			let mut fTemp0: F32 = F32::tan(self.fConst4 * (self.fConst3 * mydsp_faustpower2_f(self.fRec3[0]) + 4e+01));
			self.fRec4[0] = fSlow1 + self.fConst2 * self.fRec4[1];
			let mut fTemp1: F32 = fTemp0 * (1.0 / self.fRec4[0] + fTemp0) + 1.0;
			let mut fTemp2: F32 = *input0;
			let mut fTemp3: F32 = self.fRec0[1] + fTemp0 * (fTemp2 - self.fRec1[1]);
			let mut fTemp4: F32 = fTemp3 / fTemp1;
			self.fRec0[0] = 2.0 * fTemp4 - self.fRec0[1];
			let mut fTemp5: F32 = self.fRec1[1] + fTemp0 * fTemp3 / fTemp1;
			self.fRec1[0] = 2.0 * fTemp5 - self.fRec1[1];
			let mut fRec2: F32 = fTemp5;
			*output0 = fRec2;
			let mut fTemp6: F32 = *input1;
			let mut fTemp7: F32 = self.fRec5[1] + fTemp0 * (fTemp6 - self.fRec6[1]);
			let mut fTemp8: F32 = fTemp7 / fTemp1;
			self.fRec5[0] = 2.0 * fTemp8 - self.fRec5[1];
			let mut fTemp9: F32 = self.fRec6[1] + fTemp0 * fTemp7 / fTemp1;
			self.fRec6[0] = 2.0 * fTemp9 - self.fRec6[1];
			let mut fRec7: F32 = fTemp9;
			*output1 = fRec7;
			self.fRec3[1] = self.fRec3[0];
			self.fRec4[1] = self.fRec4[0];
			self.fRec0[1] = self.fRec0[0];
			self.fRec1[1] = self.fRec1[0];
			self.fRec5[1] = self.fRec5[0];
			self.fRec6[1] = self.fRec6[0];
		}
	}

}


}
pub use dsp::mydsp as Gain;
