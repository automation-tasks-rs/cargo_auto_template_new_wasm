// src/lib.rs
// This file has just the wasm_bindgen_start() function
// and calls into main_mod.rs.
// So the structure of the project modules can be similar to a binary CLI executable.

// region: auto_md_to_doc_comments include README.md A //!
//! # cargo_auto_template_new_wasm
//!
//! **Template for a minimal wasm project for browser**  
//! ***version: 0.0.2 date: 2024-05-01 author: [web_server_domain](https://web_server_domain) repository: [GitHub](https://github.com/automation-tasks-rs/cargo_auto_template_new_wasm)***
//!
//!  ![maintained](https://img.shields.io/badge/maintained-green)
//!  ![work-in-progress](https://img.shields.io/badge/work_in_progress-yellow)
//!  ![rustlang](https://img.shields.io/badge/rustlang-orange)
//!  ![wasm](https://img.shields.io/badge/wasm-orange)
//!
//!   [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/automation-tasks-rs/cargo_auto_template_new_wasm/blob/master/LICENSE)
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-261-green.svg)]()
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-28-blue.svg)]()
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-71-purple.svg)]()
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)]()
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-16-orange.svg)]()
//!
//! Hashtags: #maintained #ready-for-use #rustlang #automation #workflow  
//! My projects on GitHub are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).  
//! I recommend using the [CRUSTDE - Containerized Rust Development Environment](https://github.com/CRUSTDE-ContainerizedRustDevEnv/crustde_cnt_img_pod) to write Rust projects on Linux, isolated from your system.  
//!
//! ## This template
//!
//! Just like `cargo new` makes a soft and gentle introduction to Rust projects and development, I want to make the same for an in-browser WASM project with
//!
//! ```bash
//! cargo auto new_wasm project_name
//! ```
//!
//! Extremely simple, just the basic moving parts and use cases.  
//! This simplest template does not have a PWA implementation or dedicated web server app.
//!
//! ## Development details
//!
//! Read the development details in a separate md file:
//! [DEVELOPMENT.md](DEVELOPMENT.md)
//!
//! ## Releases changelog
//!
//! Read the releases changelog in a separate md file:
//! [RELEASES.md](RELEASES.md)
//!
//! ## TODO
//!
//! And code happily ever after...
//!
//! ## Open-source and free as a beer
//!
//! My open-source projects are free as a beer (MIT license).  
//! I just love programming.  
//! But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating to my [PayPal](https://paypal.me/LucianoBestia).  
//! You know the price of a beer in your local bar ;-)  
//! So I can drink a free beer for your health :-)  
//! [Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻
//!
//! [//bestia.dev](https://bestia.dev)  
//! [//github.com/bestia-dev](https://github.com/bestia-dev)  
//! [//bestiadev.substack.com](https://bestiadev.substack.com)  
//! [//youtube.com/@bestia-dev-tutorials](https://youtube.com/@bestia-dev-tutorials)  
//!
// endregion: auto_md_to_doc_comments include README.md A //!

use wasm_bindgen::prelude::*;

mod main_mod;
pub use main_mod::wsm;
pub use main_mod::LibraryError;

#[wasm_bindgen(start)]
/// To start the Wasm application, wasm_bindgen runs this functions
pub fn wasm_bindgen_start() -> Result<(), JsValue> {
    // Initialize debugging for when/if something goes wrong.
    console_error_panic_hook::set_once();
    // write the app version just for debug purposes
    wsm::debug_write(&format!("cargo_auto_template_new_wasm v{}", env!("CARGO_PKG_VERSION")));

    main_mod::main();
    // return
    Ok(())
}
