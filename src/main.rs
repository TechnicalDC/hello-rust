use traits::test_traits;

use crate::modules::types::Config;
use crate::generics::test_generics;
use crate::json::play_with_json;
use crate::error_handling::handle_error;

pub mod error_handling;
pub mod json;
pub mod modules;
pub mod extras;
pub mod generics;
pub mod traits;

fn main() {
    let cfg: Config = Config {
        json    : false,
        extra   : false,
        generics: false,
        traits  : false,
        error   : true
    };

    if cfg.extra {}

    if cfg.error {
        handle_error();
    }

    if cfg.json {
        let _ = play_with_json();
    }

    if cfg.generics {
        test_generics();
    }

    if cfg.traits {
        test_traits();
    }
}
