import { Universe, Cell } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

const CELL_SIZE = 10; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

// Construct the universe, and get its width and height.
const universe = Universe.new();
const width = universe.width();
const height = universe.height();

// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

const pauseButton = document.getElementById('pause');
let animationId = null;

document.getElementById('clear').onclick = () => {
  universe.clear();
  drawGrid();
  drawCells();
};
document.getElementById('random').onclick = () => {
  universe.fill_random();
  drawGrid();
  drawCells();
};

const isPaused = () => {
  return animationId === null;
}

const play = () => {
  pauseButton.textContent = "⏸";
  renderLoop();
};

const pause = () => {
  pauseButton.textContent = "▶";
  cancelAnimationFrame(animationId);
  animationId = null;
};

pauseButton.addEventListener("click", event => {
  if (isPaused()) play();
  else pause();
});

const renderLoop = async () => {
  universe.tick();

  drawGrid();
  drawCells();

  // await new Promise(resolve => setTimeout(resolve, 10));
  animationId = requestAnimationFrame(renderLoop);
};

const draw = (x, y) => {
  const boundingRect = canvas.getBoundingClientRect();

  const scaleX = canvas.width / boundingRect.width;
  const scaleY = canvas.height / boundingRect.height;

  const canvasLeft = (x - boundingRect.left) * scaleX;
  const canvasTop = (y - boundingRect.top) * scaleY;

  const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
  const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

  universe.toggle_cell(row, col);

  drawGrid();
  drawCells();
}

canvas.addEventListener("click", event => draw(event.clientX, event.clientY));

universe.fill_random();
drawGrid();
drawCells();
play();

function drawGrid() {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  // Vertical lines.
  for (let i = 0; i <= width; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
  }

  // Horizontal lines.
  for (let j = 0; j <= height; j++) {
    ctx.moveTo(0,                           j * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
  }

  ctx.stroke();
};

function getIndex(row, column) {
  return row * width + column;
};

function bitIsSet(n, arr) {
  const byte = Math.floor(n / 8);
  const mask = 1 << (n % 8);
  return (arr[byte] & mask) === mask;
};

function drawCells() {
  const cellsPtr = universe.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height / 8);

  ctx.beginPath();

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);

      ctx.fillStyle = bitIsSet(idx, cells)
        ? ALIVE_COLOR
        : DEAD_COLOR;

      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }

  ctx.stroke();
};
