/*
    Appellation: template-rs-wasm <lib>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        template-rs-wasm is a complete wasm application template for rust applications
*/
#[doc(inline)]
pub use self::{actors::*, components::*, core::*, data::*};

pub(crate) mod actors;
pub(crate) mod components;
pub(crate) mod core;
pub(crate) mod data;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add_one(data: usize) -> usize {
    data + 1
}
