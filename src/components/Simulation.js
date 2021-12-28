import * as sim from "../../build/lib_simulation_wasm";

import { useEffect, useState } from "react";

import Canvas from "./Canvas";

const simulation = new sim.Simulation();

export default function Simulation() {
  const [world, setWorld] = useState(simulation.world());
  const [count, setCount] = useState(2500);

  const step = () => {
    if (simulation.step()) {
      setCount(2500);
    } else {
      setCount((c) => c - 1);
    }
    setWorld(simulation.world());
  };

  useEffect(() => {
    const interval = setInterval(() => {
      step();
    }, 1000 / 60);
    return () => clearInterval(interval);
  }, []);

  return (
    <div>
      <Canvas world={world} step={step} />
      <p>{count}</p>
    </div>
  );
}
