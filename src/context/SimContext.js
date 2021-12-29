import * as sim from "../../build/lib_simulation_wasm";

import { createContext, useState } from "react";

export const SimContext = createContext();

export const SimProvider = ({ children }) => {
  const [simulation, setSimulation] = useState(() => {
    return new sim.Simulation();
  });
  const [world, setWorld] = useState(() => {
    return simulation.world();
  });
  const [simSpeed, setSimSpeed] = useState(1);
  const [isPaused, setIsPaused] = useState(false);

  return (
    <SimContext.Provider
      value={{
        simulation,
        setSimulation,
        world,
        setWorld,
        simSpeed,
        setSimSpeed,
        isPaused,
        setIsPaused,
      }}
    >
      {children}
    </SimContext.Provider>
  );
};
