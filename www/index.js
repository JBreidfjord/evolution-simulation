import * as sim from "lib-simulation-wasm";

function redraw() {
  ctx.clearRect(0, 0, width, height);

  simulation.step();

  for (const creature of simulation.world().creatures) {
    ctx.drawTriangle(
      creature.x * width,
      creature.y * height,
      0.01 * width,
      creature.rotation
    );
  }

  requestAnimationFrame(redraw);
}

CanvasRenderingContext2D.prototype.drawTriangle = function (
  x,
  y,
  size,
  rotation
) {
  this.beginPath();
  this.moveTo(
    x + Math.cos(rotation) * size * 1.5,
    y + Math.sin(rotation) * size * 1.5
  );
  this.lineTo(
    x + Math.cos(rotation + (2.0 / 3.0) * Math.PI) * size,
    y + Math.sin(rotation + (2.0 / 3.0) * Math.PI) * size
  );
  this.lineTo(
    x + Math.cos(rotation - (2.0 / 3.0) * Math.PI) * size,
    y + Math.sin(rotation - (2.0 / 3.0) * Math.PI) * size
  );
  this.lineTo(
    x + Math.cos(rotation) * size * 1.5,
    y + Math.sin(rotation) * size * 1.5
  );

  this.fillStyle = "rgb(0, 0, 0)";
  this.fill();
};

const simulation = new sim.Simulation();
const world = simulation.world();

console.log(simulation.world().creatures);

const viewport = document.getElementById("viewport");
const width = viewport.width;
const height = viewport.height;
const viewportScale = window.devicePixelRatio || 1;
viewport.width = width * viewportScale;
viewport.height = height * viewportScale;
viewport.style.width = width + "px";
viewport.style.height = height + "px";

const ctx = viewport.getContext("2d");
ctx.scale(viewportScale, viewportScale);

ctx.fillStyle = "rgb(0, 0, 0)";

redraw();
