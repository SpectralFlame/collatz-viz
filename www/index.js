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
  document.querySelectorAll("select").forEach(elem => {
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
  const collatz_kind = document.querySelector("#collatz_kind").value;
  switch (collatz_kind) {
    case "1": kind = CollatzKind.Short; break;
    case "2": kind = CollatzKind.Odd; break;
    case "3": kind = CollatzKind.Compact; break;
    default: kind = CollatzKind.Full;
  }

  console.log("kind", kind);

  const plot_type = document.querySelector("#plot_type").value;

  const start = performance.now();
  switch (plot_type) {
    case '0': chart = viz.orbit_length("canvas", Number(collatz_kind), Number(input_max.value)); break;
    case '1': chart = viz.fraction_above("canvas", Number(collatz_kind), Number(input_max.value)); break;
    case '2': chart = viz.common_ancestor_dist("canvas", Number(collatz_kind), Number(input_max.value)); break;
    default: chart = null;
  }
  const end = performance.now();

  console.log(`Rendered in ${Math.ceil(end - start)}`);
}

main()

