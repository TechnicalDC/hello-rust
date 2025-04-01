//use crate::extras::typed_example;
use crate::modules::types::Config;
use crate::generics::test_generics;

pub mod json;
pub mod modules;
pub mod extras;
pub mod generics;

fn main() {
    let cfg: Config = Config {
        json: false,
        extra: false,
        generics: true
    };

    if cfg.generics {
        test_generics(&vec![1,2,3]);
    }
}

