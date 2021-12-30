import * as sim from "../../build/lib_simulation_wasm";

import { createContext, useEffect, useState } from "react";

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
  const [isPaused, setIsPaused] = useState(false);
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
