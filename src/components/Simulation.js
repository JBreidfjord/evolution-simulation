import * as sim from "../../build/lib_simulation_wasm";

import { useEffect, useState } from "react";

import Canvas from "./Canvas";

export default function Simulation() {
  const [simulation, setSimulation] = useState(() => {
    return new sim.Simulation();
  });
  const [world, setWorld] = useState(simulation.world());

  const step = () => {
    simulation.step();
    setWorld(simulation.world());
  };

  useEffect(() => {
    const interval = setInterval(() => {
      step();
    }, 1000 / 60);
    return () => clearInterval(interval);
  }, []);

  return (
    <>
      <Canvas world={world} />
    </>
  );
}
