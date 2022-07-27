import './Extinction.css';

import { useState } from 'react';

import { useSim } from '../../hooks/useSim';
import ConfigForm from './ConfigForm';

export default function Extinction({ handleClose, setIsSimReady }) {
  const { setStartNewSim } = useSim();
  const [showConfig, setShowConfig] = useState(false);

  const handleRepopulate = () => {
    setStartNewSim(true);
    setIsSimReady(true);
  };

  const handleConfigClose = (isSimReady) => {
    setShowConfig(false);
    if (isSimReady) {
      handleClose(false);
    }
  };

  return (
    <div className="extinction">
      { !showConfig ? (
        <>
          <button className="btn close" onClick={ handleClose }>
            X
          </button>
          <h2>Extinction</h2>
          <div className="button-group">
            <button className="btn" onClick={ handleRepopulate }>
              Repopulate
            </button>
            <button className="btn" onClick={ () => setShowConfig(true) }>
              Config
            </button>
          </div>
        </>
      ) : (
        <ConfigForm handleClose={ handleConfigClose } isNestedConfig={ true } />
      ) }
    </div>
  );
}
