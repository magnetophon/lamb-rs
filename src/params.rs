#[derive(Params)]
struct GainFaustNihPlugParams {
    #[id = "CutOff"]
    cutoff: FloatParam,
    #[id = "Resonance"]
    resonance: FloatParam
}

impl Default for GainFaustNihPlugParams {
    fn default() -> Self {
        Self {
            cutoff: FloatParam::new("CutOff", 1.0, FloatRange::Linear { min: 0.0, max: 1.0}),
            resonance: FloatParam::new("Resonance", 1.0, FloatRange::Linear { min: 0.0, max: 2.0})
        }
    }
}
