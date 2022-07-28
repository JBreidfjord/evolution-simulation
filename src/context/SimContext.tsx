import { createContext, useEffect, useState } from 'react';

import { Config, Simulation } from '../../wasm/simulation';
import { World } from '../interfaces';

export interface SimContextProps {
  newSim(): void;
  simulation: Simulation;
  setSimulation: React.Dispatch<React.SetStateAction<Simulation>>;
  world: World;
  setWorld: React.Dispatch<React.SetStateAction<World>>;
  simConfig: Config;
  setSimConfig: React.Dispatch<React.SetStateAction<Config>>;
  simSpeed: number;
  setSimSpeed: React.Dispatch<React.SetStateAction<number>>;
  isPaused: boolean;
  setIsPaused: React.Dispatch<React.SetStateAction<boolean>>;
  setStartNewSim: React.Dispatch<React.SetStateAction<boolean>>;
}

const notImplemented = (): void => {
  throw new Error('Function not implemented');
};
const initialConfig = new Config({});
const initialSim = new Simulation(initialConfig);
const initialState: SimContextProps = {
  newSim: notImplemented,
  simulation: initialSim,
  setSimulation: notImplemented,
  world: initialSim.world(),
  setWorld: notImplemented,
  simConfig: initialConfig,
  setSimConfig: notImplemented,
  simSpeed: 1,
  setSimSpeed: notImplemented,
  isPaused: true,
  setIsPaused: notImplemented,
  setStartNewSim: notImplemented
};

export const SimContext = createContext<SimContextProps>(initialState);

interface SimContextProviderProps {
  children: React.ReactNode
}

export const SimContextProvider = ({ children }: SimContextProviderProps): JSX.Element => {
  const [simulation, setSimulation] = useState(() => {
    return new Simulation();
  });
  const [world, setWorld] = useState(() => {
    return simulation.world();
  });
  const [simConfig, setSimConfig] = useState(simulation.config);
  const [simSpeed, setSimSpeed] = useState(1);
  const [isPaused, setIsPaused] = useState(true);
  const [startNewSim, setStartNewSim] = useState(false);

  const newSim = (): void => {
    setSimulation(() => {
      return new Simulation(new Config(simConfig.toJSON()));
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
