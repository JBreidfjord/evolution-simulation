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
    if (simulation.step()) {
      updateGeneration();
    }
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

  let highlightedCreatures = updateStats();
  drawHighlights(ctx, width, height, highlightedCreatures);

  requestAnimationFrame(redraw);
}

function createStats() {
  const world = simulation.world();

  const table = document.getElementById("stats");
  let old_tbody = table.childNodes[2];
  let new_tbody = document.createElement("tbody");
  new_tbody.id = "stats-body";

  world.creatures.sort((a, b) => b.fitness - a.fitness);

  for (const creature of world.creatures) {
    let row = new_tbody.insertRow();

    let input = document.createElement("input");
    input.type = "checkbox";
    input.id = creature.id;

    row.insertCell().appendChild(input);
    row.insertCell().innerText = creature.id;
    row.insertCell().innerText = creature.fitness;
    // row.insertCell().innerText = creature.x.toFixed(2);
    // row.insertCell().innerText = creature.y.toFixed(2);
  }

  old_tbody.parentNode.replaceChild(new_tbody, old_tbody);
}

function updateStats() {
  const world = simulation.world();

  let highlightedCreatures = [];

  const tbody = document.getElementById("stats-body");
  let rows = tbody.childNodes;
  world.creatures.sort((a, b) => a.id - b.id);
  rows.forEach((row, i) => {
    let cols = row.childNodes;
    if (cols[0].childNodes[0].checked) {
      highlightedCreatures.push(world.creatures[i].id);
    }
    cols[1].innerText = world.creatures[i].id;
    cols[2].innerText = world.creatures[i].fitness;
    // cols[3].innerText = world.creatures[i].x.toFixed(2);
    // cols[4].innerText = world.creatures[i].y.toFixed(2);
  });

  let creatureFitness = world.creatures.map((creature) => creature.fitness);
  document.getElementById("fitness-min").innerText = Math.min(...creatureFitness);
  document.getElementById("fitness-max").innerText = Math.max(...creatureFitness);
  document.getElementById("fitness-avg").innerText = (
    creatureFitness.reduce((a, b) => a + b, 0) / creatureFitness.length
  ).toPrecision(3);

  let bestCreature = world.creatures.find(
    (creature) => creature.fitness === Math.max(...creatureFitness)
  );
  if (!highlightedCreatures.includes(bestCreature.id)) {
    highlightedCreatures.push(bestCreature.id);
  }

  return highlightedCreatures;
}

function updateGeneration(minFitness = null, maxFitness = null, avgFitness = null) {
  document.getElementById("gen-stats-table").removeAttribute("hidden");

  if (minFitness === null) {
    minFitness = parseFloat(document.getElementById("fitness-min").innerText);
  }
  if (maxFitness === null) {
    maxFitness = parseFloat(document.getElementById("fitness-max").innerText);
  }
  if (avgFitness === null) {
    avgFitness = parseFloat(document.getElementById("fitness-avg").innerText);
  }

  document.getElementById("gen-count").innerText = simulation.generation;

  let tbody = document.getElementById("gen-stats-body");
  let row = tbody.insertRow(0);
  row.insertCell().innerText = simulation.generation - 1;
  row.insertCell().innerText = minFitness;
  row.insertCell().innerText = maxFitness;
  row.insertCell().innerText = avgFitness.toPrecision(3);

  if (tbody.childElementCount > 5) {
    tbody.removeChild(tbody.lastChild);
  }
}

function drawHighlights(ctx, width, height, highlightedCreatures) {
  const world = simulation.world();

  for (const creature of world.creatures) {
    if (highlightedCreatures.includes(creature.id)) {
      ctx.drawHighlight(creature.x * width, creature.y * height, 0.05 * width, creature.rotation);
    }
  }
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

CanvasRenderingContext2D.prototype.drawHighlight = function (x, y, radius, rotation) {
  this.beginPath();
  let arc = ((5 / 4) * Math.PI) / 2;
  this.arc(x, y, radius, rotation, rotation + arc);
  this.arc(x, y, radius, rotation, rotation - arc, true);

  this.fillStyle = "rgba(255, 255, 255, 0.1)";
  this.fill();
};

document.getElementById("train").onclick = function () {
  updateGeneration(...simulation.train());
};

document.getElementById("sim-speed").oninput = function () {
  simSpeed = this.value;
  document.getElementById("sim-speed-value").innerText = this.value;
};

const simulation = new sim.Simulation();
let simSpeed = 1;

createStats();
redraw();
