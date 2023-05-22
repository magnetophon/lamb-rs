extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parser, parse_macro_input, DeriveInput};

#[proc_macro]
pub fn nih_params_from_faust(_item: TokenStream) -> TokenStream {
    TokenStream::from(quote! {
        #[derive(Params)]
        struct GainFaustNihPlugParams {
            #[id = "Cutoff"]
            cut_off: FloatParam,
        }

        impl Default for GainFaustNihPlugParams {
            fn default() -> Self {
                Self {
                    // This gain is stored as linear gain. NIH-plug comes with useful conversion functions
                    // to treat these kinds of parameters as if we were dealing with decibels. Storing this
                    // as decibels is easier to work with, but requires a conversion for every sample.
                    cut_off: FloatParam::new("CutOff", 1.0, FloatRange::Linear { min: 0.0, max: 1.0 }),
                }
            }
        }
    })
}
