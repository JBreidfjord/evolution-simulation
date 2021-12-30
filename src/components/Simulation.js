import Canvas from "./Canvas";
import { useEffect } from "react";
import { useSim } from "../hooks/useSim";

export default function Simulation() {
  const { simulation, simSpeed, setWorld, isPaused } = useSim();

  const step = () => {
    simulation.step();
    setWorld(simulation.world());
  };

  useEffect(() => {
    if (!isPaused) {
      const interval = setInterval(() => {
        step();
      }, 1000 / (60 * simSpeed));
      return () => clearInterval(interval);
    }
  }, [simSpeed, isPaused]);

  return <Canvas />;
}
