
fn main() {
    println!("cargo:rerun-if-changed=dsp");

    #[cfg(feature = "faust-build")]
    faust_build::build_dsp_to_destination("dsp/gain.dsp", "src/dsp.rs");
}
