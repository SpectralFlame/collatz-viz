import { Chart, CollatzViz, CollatzKind } from "collatz-viz"

const canvas = document.querySelector("#canvas");
const input_max = document.querySelector("#max");

let chart = null;
let viz = null;
let init = false;

export function main() {
  viz = CollatzViz.new();
  setupUI();
  setupCanvas();
}

function setupUI() {
  document.querySelectorAll("input").forEach(elem => {
    elem.addEventListener("input", updatePlot);
  });
}

function setupCanvas() {
  // const dpr = window.devicePixelRatio || 1.0;
  const width = canvas.parentNode.offsetWidth;
  const height = canvas.parentNode.offsetHeight;
  canvas.style.width = width + "px";
  canvas.style.height = (height - 50) + "px";
  canvas.width = width;
  canvas.height = height - 50;
  init = true;
  updatePlot();
}

function updatePlot() {
  if (!init) return;
  chart = null;

  let kind = CollatzKind.Full;
  const checked_kind = document.querySelector('input[name="collatz_kind"]:checked').value;
  switch (checked_kind) {
    case "Short": kind = CollatzKind.Short; break;
    case "Odd": kind = CollatzKind.Odd; break;
    case "Compact": kind = CollatzKind.Compact; break;
    default: kind = CollatzKind.Full;
  }

  const checked_plot = document.querySelector('input[name="plot"]:checked').value;

  const start = performance.now();
  switch (checked_plot) {
    case '0': chart = viz.common_ancestor_dist("canvas", Number(checked_kind), Number(input_max.value)); break;
    case '1': chart = viz.fraction_above("canvas", Number(checked_kind), Number(input_max.value)); break;
    case '2': chart = viz.orbit_length("canvas", Number(checked_kind), Number(input_max.value)); break;
    default: chart = null;
  }
  const end = performance.now();

  console.log(`Rendered in ${Math.ceil(end - start)}`);
}

main()

