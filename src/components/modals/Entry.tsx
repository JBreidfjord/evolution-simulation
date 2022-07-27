import './Entry.css';

import { useState } from 'react';

import { useSim } from '../../hooks/useSim';
import ConfigForm from './ConfigForm';

interface EntryProps {
  setIsSimReady: React.Dispatch<React.SetStateAction<boolean>>;
}

export default function Entry({ setIsSimReady }: EntryProps): JSX.Element {
  const { setStartNewSim } = useSim();
  const [showInfo, setShowInfo] = useState(false);
  const [showConfig, setShowConfig] = useState(false);

  const handlePopulate = (): void => {
    setStartNewSim(true);
    setIsSimReady(true);
  };

  const handleConfigClose = (isSimReady: boolean): void => {
    setShowConfig(false);
    if (isSimReady) {
      setIsSimReady(true);
    }
  };

  return (
    <div className="entry">
      { !showInfo ? (
        !showConfig ? (
          <>
            <h2>Flying Microcosmic Societies</h2>
            <h4>Vol. 2</h4>
            <p>An Evolution Simulation</p>
            <div className="button-group">
              <button className="btn" onClick={ handlePopulate }>
                Populate
              </button>
              <button className="btn" onClick={ () => setShowConfig(true) }>
                Config
              </button>
              <button className="btn" onClick={ () => setShowInfo(true) }>
                Info
              </button>
            </div>
          </>
        ) : (
          <ConfigForm handleClose={ handleConfigClose } isNestedConfig={ true } />
        )
      ) : (
        <>
          <button className="btn close" onClick={ () => setShowInfo(false) }>
            X
          </button>
          <p>
            Like in nature, these creatures will learn and adapt to their environment over
            generations of evolution. Each creature is controlled by a neural network, and their
            goal is to reproduce and pass on their genes to the next generation. To do so they must
            consume enough food to survive, and reproduce when they have the energy to do so.
          </p>
          <button className="btn" onClick={ () => setShowInfo(false) }>
            Back
          </button>
        </>
      ) }
    </div>
  );
}
