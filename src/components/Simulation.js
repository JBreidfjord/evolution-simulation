import Canvas from "./Canvas";
import { useEffect } from "react";
import { useSim } from "../hooks/useSim";

export default function Simulation({ setShowExtinctionModal }) {
  const { simulation, simSpeed, world, setWorld, isPaused, setIsPaused } = useSim();

  useEffect(() => {
    console.log(simulation);
  }, []);

  const step = () => {
    simulation.step();
    setWorld(simulation.world());
  };

  useEffect(() => {
    if (world.creatures.filter((creature) => creature.alive).length === 0 && simulation.age() > 0) {
      setIsPaused(true);
      setShowExtinctionModal(true);
    }
  }, [world]);

  useEffect(() => {
    if (!isPaused) {
      const interval = setInterval(() => {
        step();
      }, 1000 / (60 * simSpeed));
      return () => clearInterval(interval);
    }
  }, [simSpeed, isPaused]);

  return (
    <>
      <Canvas />
    </>
  );
}
