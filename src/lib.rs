mod collatz;
mod utils;

use collatz::CollatzKind;
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

#[wasm_bindgen]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen]
impl Chart {
    pub fn common_ancestor_dist(canvas_id: &str, kind: i32, max: i32) -> Result<Chart, JsValue> {
        let kind = match kind {
            0 => CollatzKind::Full,
            1 => CollatzKind::Short,
            2 => CollatzKind::Odd,
            3 => CollatzKind::Compact,
            _ => CollatzKind::Full,
        };
        let max = max as u64;
        let map_coord = collatz::viz::common_ancestor_dist::draw(canvas_id, kind, max)
            .map_err(|err| err.to_string())?;
        Ok(Chart {
            convert: Box::new(move |coord| map_coord(coord).map(|(x, y)| (x.into(), y.into()))),
        })
    }

    pub fn coord(&self, x: i32, y: i32) -> Option<Point> {
        (self.convert)((x, y)).map(|(x, y)| Point { x, y })
    }
}
