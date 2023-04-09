mod collatz;
mod utils;

use wasm_bindgen::prelude::*;

extern crate alloc;

use lol_alloc::{AssumeSingleThreaded, FreeListAllocator};

// SAFETY: This application is single threaded, so using AssumeSingleThreaded is allowed.
#[global_allocator]
static ALLOCATOR: AssumeSingleThreaded<FreeListAllocator> =
    unsafe { AssumeSingleThreaded::new(FreeListAllocator::new()) };

pub type DrawResult<T> = Result<T, Box<dyn std::error::Error>>;

#[wasm_bindgen]
pub struct Chart {
    convert: Box<dyn Fn((i32, i32)) -> Option<(f64, f64)>>,
}
