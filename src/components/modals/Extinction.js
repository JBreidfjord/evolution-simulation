import "./Extinction.css";

import { useSim } from "../../hooks/useSim";

export default function Extinction({ handleClose, setShowConfigModal }) {
  const { setStartNewSim } = useSim();

  const handleRepopulate = () => {
    setStartNewSim(true);
    handleClose(false);
  };

  const handleConfig = () => {
    setShowConfigModal(true);
    handleClose(false);
  };

  return (
    <div className="extinction">
      <button className="btn close" onClick={handleClose}>
        X
      </button>
      <h2>Extinction</h2>
      <div className="button-group">
        <button className="btn" onClick={handleRepopulate}>
          Repopulate
        </button>
        <button className="btn" onClick={handleConfig}>
          Config
        </button>
      </div>
    </div>
  );
}
