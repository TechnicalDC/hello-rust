use crate::modules::types::Config;
use crate::generics::test_generics;
use crate::json::play_with_json;

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

    if cfg.extra {}

    if cfg.json {
        let _ = play_with_json();
    }

    if cfg.generics {
        test_generics();
        //test_generics(&vec![1,2,3]);
    }
}
