import * as sim from "lib-simulation-wasm";

function setScale() {
  const canvas = document.getElementById("canvas");
  const canvasContainer = document.getElementById("canvas-container");

  const viewportScale = window.devicePixelRatio || 1;
  const { width, height } = canvasContainer.getBoundingClientRect();
  const displayWidth = Math.round(width * viewportScale);
  const displayHeight = Math.round(height * viewportScale);

  const ctx = canvas.getContext("2d");

  const needResize = canvas.width !== displayWidth || canvas.height !== displayHeight;

  if (needResize) {
    canvas.width = displayWidth;
    canvas.height = displayHeight;

    canvas.style.width = canvas.width + "px";
    canvas.style.height = canvas.height + "px";

    ctx.scale(viewportScale, viewportScale);
  }

  return [ctx, canvas.width, canvas.height];
}

function redraw() {
  let [ctx, width, height] = setScale();
  ctx.clearRect(0, 0, width, height);

  for (let i = 0; i < simSpeed; i++) {
    simulation.step();
  }

  const world = simulation.world();
  const foodCount = world.foods.length;

  for (const food of world.foods) {
    ctx.drawCircle(food.x * width, food.y * height, (0.01 / 2.0) * width);
  }

  for (const creature of world.creatures) {
    ctx.drawTriangle(
      creature.x * width,
      creature.y * height,
      0.01 * width,
      creature.rotation,
      creature.fitness / foodCount
    );
  }

  updateStats();

  requestAnimationFrame(redraw);
}

function updateStats() {
  const world = simulation.world();
  const table = document.getElementById("stats");
  let old_tbody = table.childNodes[2];
  let new_tbody = document.createElement("tbody");

  for (const creature of world.creatures) {
    let row = new_tbody.insertRow();
    row.insertCell().innerText = creature.fitness;
    row.insertCell().innerText = creature.x.toFixed(2);
    row.insertCell().innerText = creature.y.toFixed(2);
  }

  old_tbody.parentNode.replaceChild(new_tbody, old_tbody);

  let creatureFitness = world.creatures.map((creature) => creature.fitness);
  document.getElementById("fitness-min").innerText = Math.min(...creatureFitness);
  document.getElementById("fitness-max").innerText = Math.max(...creatureFitness);
  document.getElementById("fitness-avg").innerText = (
    creatureFitness.reduce((a, b) => a + b, 0) / creatureFitness.length
  ).toFixed(2);
}

CanvasRenderingContext2D.prototype.drawTriangle = function (x, y, size, rotation, fitness) {
  this.beginPath();
  this.moveTo(x + Math.cos(rotation) * size * 1.5, y + Math.sin(rotation) * size * 1.5);
  this.lineTo(
    x + Math.cos(rotation + (2.0 / 3.0) * Math.PI) * size,
    y + Math.sin(rotation + (2.0 / 3.0) * Math.PI) * size
  );
  this.lineTo(
    x + Math.cos(rotation - (2.0 / 3.0) * Math.PI) * size,
    y + Math.sin(rotation - (2.0 / 3.0) * Math.PI) * size
  );
  this.lineTo(x + Math.cos(rotation) * size * 1.5, y + Math.sin(rotation) * size * 1.5);

  this.fillStyle = `rgb(150, 120, ${Math.floor(fitness * 255)})`;
  this.fill();
};

CanvasRenderingContext2D.prototype.drawCircle = function (x, y, radius) {
  this.beginPath();
  this.arc(x, y, radius, 0, 2 * Math.PI);

  this.fillStyle = "rgb(100, 190, 100)";
  this.fill();
};

document.getElementById("train").onclick = function () {
  document.getElementById("train-stats").innerText = simulation.train();
};

document.getElementById("sim-speed").oninput = function () {
  simSpeed = this.value;
  document.getElementById("sim-speed-value").innerText = this.value;
};

const simulation = new sim.Simulation();
let simSpeed = 1;

redraw();
