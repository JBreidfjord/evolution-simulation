import "./Entry.css";

import { useSim } from "../../hooks/useSim";
import { useState } from "react";

export default function Entry({ handleClose, setShowConfigModal }) {
  const { setStartNewSim } = useSim();
  const [showInfo, setShowInfo] = useState(false);

  const handlePopulate = () => {
    setStartNewSim(true);
    handleClose();
  };

  const handleConfig = () => {
    setShowConfigModal(true);
    handleClose();
  };

  return (
    <div className="entry">
      {!showInfo ? (
        <>
          <h2>Flying Microcosmic Societies</h2>
          <h4>Vol. 2</h4>
          <p>An Evolution Simulation</p>
          <div className="button-group">
            <button className="btn" onClick={handlePopulate}>
              Populate
            </button>
            <button className="btn" onClick={handleConfig}>
              Config
            </button>
            <button className="btn" onClick={() => setShowInfo(true)}>
              Info
            </button>
          </div>
        </>
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
