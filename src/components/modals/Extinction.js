import "./Extinction.css";

import ConfigForm from "./ConfigForm";
import { useSim } from "../../hooks/useSim";
import { useState } from "react";

export default function Extinction({ handleClose, setSimReady }) {
  const { setStartNewSim } = useSim();
  const [showConfig, setShowConfig] = useState(false);

  const handleRepopulate = () => {
    setStartNewSim(true);
    setSimReady(true);
  };

  const handleConfigClose = (simReady) => {
    setShowConfig(false);
    if (simReady) {
      handleClose(false);
    }
  };

  return (
    <div className="extinction">
      {!showConfig ? (
        <>
          <button className="btn close" onClick={handleClose}>
            X
          </button>
          <h2>Extinction</h2>
          <div className="button-group">
            <button className="btn" onClick={handleRepopulate}>
              Repopulate
            </button>
            <button className="btn" onClick={() => setShowConfig(true)}>
              Config
            </button>
          </div>
        </>
      ) : (
        <ConfigForm handleClose={handleConfigClose} isNestedConfig={true} />
      )}
    </div>
  );
}
