use wasm_bindgen::prelude::wasm_bindgen;

use super::Collatz;

pub mod common_ancestor_dist;
pub mod fraction_above;
pub mod orbit_length;

#[wasm_bindgen]
pub struct CollatzViz {
    data: [Option<Collatz>; 4],
}

#[wasm_bindgen]
impl CollatzViz {
    pub fn new() -> Self {
        Self {
            data: [None, None, None, None],
        }
    }

    pub fn get_length_string(&self) -> String {
        let lens = self
            .data
            .iter()
            .map(|c| {
                if let Some(c) = c {
                    c.len().to_string()
                } else {
                    "0".to_string()
                }
            })
            .collect::<Vec<_>>();
        lens.join(" ")
    }
}
