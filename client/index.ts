import init, { World, Direction, GameStatus } from "snake_game";
import { rnd } from "./utils/rnd";

init().then((wasm) => {
  const CELL_SIZE = 20;
  const WORLD_WIDTH = 8;
  let snakeIdx = rnd(WORLD_WIDTH * WORLD_WIDTH);

  const world = World.new(WORLD_WIDTH, snakeIdx);
  const worldWidth = world.width();

  const gameControlBtn = document.getElementById("game-control-btn");
  const gameStatus = document.getElementById("status");
  const gameScore = document.getElementById("score");
  const canvas = <HTMLCanvasElement>document.getElementById("snake-canvas");
  const ctx = canvas.getContext("2d");

  canvas.height = CELL_SIZE * worldWidth;
  canvas.width = CELL_SIZE * worldWidth;

  gameControlBtn.addEventListener("click", () => {
    const status = world.status();

    if (status === undefined) {
      gameControlBtn.textContent = "Restart";
      world.start_game();
      play();
    } else {
      location.reload();
    }
  });

  addEventListener("keydown", (e) => {
    switch (e.code) {
      case "ArrowUp":
        world.change_snake_dir(Direction.Up);
        break;
      case "ArrowDown":
        world.change_snake_dir(Direction.Down);
        break;
      case "ArrowLeft":
        world.change_snake_dir(Direction.Left);
        break;
      case "ArrowRight":
        world.change_snake_dir(Direction.Right);
        break;
    }
  });

  function drawWorld() {
    ctx.beginPath();

    for (let x = 0; x < worldWidth + 1; x++) {
      ctx.moveTo(x * CELL_SIZE, 0);
      ctx.lineTo(x * CELL_SIZE, worldWidth * CELL_SIZE);
    }

    for (let y = 0; y < worldWidth + 1; y++) {
      ctx.moveTo(0, y * CELL_SIZE);
      ctx.lineTo(worldWidth * CELL_SIZE, y * CELL_SIZE);
    }

    ctx.stroke();
  }

  function drawSnake() {
    const snakeBody = new Uint32Array(
      wasm.memory.buffer,
      world.snake_body(),
      world.snake_length()
    );

    snakeBody
      .slice()
      .reverse()
      .forEach((idx, i) => {
        const col = idx % worldWidth;

        ctx.fillStyle = i === world.snake_length() - 1 ? "red" : "black";

        const row = Math.floor(idx / worldWidth);

        ctx.beginPath();
        ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
      });

    ctx.stroke();
  }

  function drawReword() {
    const idx = world.reward_cell();

    if (idx < worldWidth * worldWidth) {
      const col = idx % worldWidth;
      const row = Math.floor(idx / worldWidth);

      ctx.fillStyle = "green";
      ctx.beginPath();
      ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
      ctx.stroke();
    }
  }

  function drawGameStatus() {
    gameStatus.textContent = world.game_status_text();
  }

  function drawGameScore() {
    gameScore.textContent = world.score().toString();
  }

  function paint() {
    drawWorld();
    drawSnake();
    drawReword();
    drawGameStatus();
    drawGameScore();
  }

  function play() {
    console.log("playing...");
    const status = world.status();

    if (status == GameStatus.Won || status == GameStatus.Lost) {
      return;
    }

    const fps = 10;
    setTimeout(() => {
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      world.step();
      paint();
      requestAnimationFrame(play);
    }, 1000 / fps);
  }

  paint();
});
