use nih_plug::prelude::*;

use lowpass_lr4_faust_nih_plug::GainFaustNihPlug;

fn main() {
    nih_export_standalone::<GainFaustNihPlug>();
}
