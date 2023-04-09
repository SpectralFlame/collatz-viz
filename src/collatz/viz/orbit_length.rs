use crate::collatz::{Collatz, CollatzKind};
use crate::{Chart, DrawResult};
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use super::CollatzViz;

#[wasm_bindgen]
impl CollatzViz {
    pub fn orbit_length(
        &mut self,
        canvas_id: &str,
        kind: i32,
        max: i32,
    ) -> Result<Chart, JsValue> {
        let kind = CollatzKind::from(kind);
        let max = max as u64;
        Ok(self
            .draw_orbit_length(canvas_id, kind, max)
            .map_err(|err| err.to_string())?)
    }
}

impl CollatzViz {
    pub fn draw_orbit_length(
        &mut self,
        canvas_id: &str,
        kind: CollatzKind,
        max: u64,
    ) -> DrawResult<Chart> {
        if self.data[kind as usize].is_none() {
            self.data[kind as usize] = Some(Collatz::new(kind));
        }
        let collatz = self.data[kind as usize].as_mut().unwrap();
        collatz.generate_fill_down(max);

        let backend = CanvasBackend::new(canvas_id).expect("cannot find canvas");
        let root = backend.into_drawing_area();

        root.fill(&WHITE)?;

        let mut points = Vec::with_capacity(max as usize - 1);
        let mut max_height = 0.;
        for n in 2..=max {
            if kind == CollatzKind::Odd {
                if n % 2 == 0 {
                    continue;
                }
            }
            if kind == CollatzKind::Compact {
                if n % 2 == 0 || n % 3 == 0 {
                    continue;
                }
            }
            let orbit_length = collatz.get_depth(n);
            let x = n as f64;
            let y = orbit_length as f64;
            max_height = y.max(max_height);
            points.push((x, y));
        }

        let mut chart = ChartBuilder::on(&root)
            .margin(20u32)
            .build_cartesian_2d(0f64..max as f64, 0f64..max_height)?;

        chart
            .draw_series(
                // LineSeries::new(points, &RED)
                points.iter().enumerate().map(|(i, p)| {
                    Circle::new(*p, 1, &HSLColor(i as f64 / points.len() as f64, 1., 0.5))
                }),
            )
            .unwrap();

        root.present()?;
        let map_coord = chart.into_coord_trans();

        Ok(Chart {
            convert: Box::new(move |coord| map_coord(coord).map(|(x, y)| (x.into(), y.into()))),
        })
    }
}

