import * as sim from "../../build/lib_simulation_wasm";

import { useEffect, useState } from "react";

import Canvas from "./Canvas";

export default function Simulation({ world, setWorld, simSpeed }) {
  const [simulation, setSimulation] = useState(() => {
    return new sim.Simulation();
  });

  const step = () => {
    simulation.step();
    setWorld(simulation.world());
  };

  useEffect(() => {
    const interval = setInterval(() => {
      step();
    }, 1000 / (60 * simSpeed));
    return () => clearInterval(interval);
  }, [simSpeed]);

  return <>{world && <Canvas world={world} />}</>;
}
