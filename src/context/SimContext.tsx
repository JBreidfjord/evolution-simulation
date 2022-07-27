import { createContext, useEffect, useState } from 'react';

import * as sim from '../../wasm/simulation';

export const SimContext = createContext();

export const SimProvider = ({ children }) => {
  const [simulation, setSimulation] = useState(() => {
    return new sim.Simulation();
  });
  const [world, setWorld] = useState(() => {
    return simulation.world();
  });
  const [simConfig, setSimConfig] = useState(simulation.config);
  const [simSpeed, setSimSpeed] = useState(1);
  const [isPaused, setIsPaused] = useState(true);
  const [startNewSim, setStartNewSim] = useState(false);

  const newSim = () => {
    setSimulation(() => {
      return new sim.Simulation(new sim.Config(simConfig.intoObject()));
    });
    setWorld(simulation.world());
  };

  useEffect(() => {
    if (startNewSim) {
      newSim();
      setStartNewSim(false);
      setIsPaused(false);
    }
  }, [startNewSim]);

  return (
    <SimContext.Provider
      value={{
        newSim,
        simulation,
        setSimulation,
        world,
        setWorld,
        simConfig,
        setSimConfig,
        simSpeed,
        setSimSpeed,
        isPaused,
        setIsPaused,
        setStartNewSim,
      }}
    >
      {children}
    </SimContext.Provider>
  );
};
