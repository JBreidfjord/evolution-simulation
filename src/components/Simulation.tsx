import { useEffect } from 'react';

import { useSim } from '../hooks/useSim';
import Canvas from './Canvas';

export default function Simulation(): JSX.Element {
  const { simulation, simSpeed, setWorld, isPaused } = useSim();

  const step = (): void => {
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
