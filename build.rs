include!("src/dsp_48k.rs");
include!("src/dsp_96k.rs");
include!("src/dsp_192k.rs");

#[derive(Debug)]
#[allow(unused)]
enum Param {
    Normal {
        label: String,
        param: i32,
        init: f64,
        min: f64,
        max: f64,
        step: f64,
    },
}

fn main() {
    println!("cargo:rerun-if-changed=dsp");

    #[cfg(feature = "faust-rebuild")]
    faust_build::FaustBuilder::new("dsp/lamb-rs-48k.dsp", "src/dsp_48k.rs")
        .set_use_double(true)
        .set_module_name("dsp_48k".to_string())
        .build();

    #[cfg(feature = "faust-rebuild")]
    faust_build::FaustBuilder::new("dsp/lamb-rs-96k.dsp", "src/dsp_96k.rs")
        .set_use_double(true)
        .set_module_name("dsp_96k".to_string())
        .build();

    #[cfg(feature = "faust-rebuild")]
    faust_build::FaustBuilder::new("dsp/lamb-rs-192k.dsp", "src/dsp_192k.rs")
        .set_use_double(true)
        .set_module_name("dsp_192k".to_string())
        .build();
}
