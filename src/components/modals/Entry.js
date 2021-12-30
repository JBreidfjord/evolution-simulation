import "./Entry.css";

import ConfigForm from "./ConfigForm";
import { useSim } from "../../hooks/useSim";
import { useState } from "react";

export default function Entry({ setSimReady }) {
  const { setStartNewSim } = useSim();
  const [showInfo, setShowInfo] = useState(false);
  const [showConfig, setShowConfig] = useState(false);

  const handlePopulate = () => {
    setStartNewSim(true);
    setSimReady(true);
  };

  const handleConfigClose = (simReady) => {
    setShowConfig(false);
    if (simReady) {
      setSimReady(true);
    }
  };

  return (
    <div className="entry">
      {!showInfo ? (
        !showConfig ? (
          <>
            <h2>Flying Microcosmic Societies</h2>
            <h4>Vol. 2</h4>
            <p>An Evolution Simulation</p>
            <div className="button-group">
              <button className="btn" onClick={handlePopulate}>
                Populate
              </button>
              <button className="btn" onClick={() => setShowConfig(true)}>
                Config
              </button>
              <button className="btn" onClick={() => setShowInfo(true)}>
                Info
              </button>
            </div>
          </>
        ) : (
          <ConfigForm handleClose={handleConfigClose} isEntryConfig={true} />
        )
      ) : (
        <>
          <p>
            Like in nature, these creatures will learn and adapt to their environment over
            generations of evolution. Each creature is controlled by a neural network, and at the
            end of a generation the best fit creatures will reproduce and populate the next
            generation with their genes.
          </p>
          <button className="btn" onClick={() => setShowInfo(false)}>
            Back
          </button>
        </>
      )}
    </div>
  );
}
