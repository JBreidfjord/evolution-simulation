import "./Canvas.css";

import { useEffect, useRef } from "react";

import { useSim } from "../hooks/useSim";

const getPixelRatio = (ctx) => {
  let backingStore =
    ctx.backingStorePixelRatio ||
    ctx.webkitBackingStorePixelRatio ||
    ctx.mozBackingStorePixelRatio ||
    ctx.msBackingStorePixelRatio ||
    ctx.oBackingStorePixelRatio ||
    1;

  return (window.devicePixelRatio || 1) / backingStore;
};

CanvasRenderingContext2D.prototype.drawCreature = function (x, y, size, rotation, fitness) {
  const ctx = this;

  ctx.beginPath();
  ctx.moveTo(x + Math.cos(rotation) * size * 1.5, y + Math.sin(rotation) * size * 1.5);
  ctx.lineTo(
    x + Math.cos(rotation + (2.0 / 3.0) * Math.PI) * size,
    y + Math.sin(rotation + (2.0 / 3.0) * Math.PI) * size
  );
  ctx.lineTo(
    x + Math.cos(rotation - (2.0 / 3.0) * Math.PI) * size,
    y + Math.sin(rotation - (2.0 / 3.0) * Math.PI) * size
  );
  ctx.closePath();

  ctx.fillStyle = `hsl(${fitness * 360}, 100%, 50%)`;
  ctx.fill();
};

CanvasRenderingContext2D.prototype.drawFood = function (x, y, radius) {
  const ctx = this;

  ctx.beginPath();
  ctx.arc(x, y, radius, 0, 2 * Math.PI);

  ctx.fillStyle = "rgb(100, 190, 100)";
  ctx.fill();
};

export default function Canvas() {
  const { world, simConfig, isPaused } = useSim();
  const canvasRef = useRef();

  useEffect(() => {
    const canvas = canvasRef.current;
    const ctx = canvas.getContext("2d");

    const setScale = () => {
      const ratio = getPixelRatio(ctx);
      const width = Math.round(window.innerWidth * 0.95);
      const height = Math.round(window.innerHeight);

      const needResize = canvas.width !== width * ratio || canvas.height !== height * ratio;

      if (needResize) {
        canvas.width = width * ratio;
        canvas.height = height * ratio;
        canvas.style.width = `${width}px`;
        canvas.style.height = `${height}px`;
      }
    };

    const render = () => {
      setScale();
      ctx.clearRect(0, 0, canvas.width, canvas.height);

      for (const food of world.foods) {
        ctx.drawFood(food.x * canvas.width, food.y * canvas.height, (0.01 / 2.0) * canvas.width);
      }

      for (const creature of world.creatures) {
        ctx.drawCreature(
          creature.x * canvas.width,
          creature.y * canvas.height,
          simConfig.creature_size * canvas.width,
          creature.rotation,
          creature.fitness / world.foods.length
        );
      }
    };

    if (isPaused) {
      let requestId;
      const interval = setInterval(() => {
        requestId = render();
      }, 1000 / 60);
      return () => {
        clearInterval(interval);
        cancelAnimationFrame(requestId);
      };
    }

    let requestId = render();
    return () => cancelAnimationFrame(requestId);
  }, [world, simConfig, window.innerWidth, window.innerHeight, isPaused]);

  return <canvas ref={canvasRef} />;
}
