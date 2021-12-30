import "./Controls.css";

import { useSim } from "../hooks/useSim";

export default function Controls({ setShowConfigModal }) {
  const { simSpeed, setSimSpeed, isPaused, setIsPaused } = useSim();

  return (
    <div className="controls">
      <h2>Controls</h2>
      <label>
        <span>Simulation Speed:</span>
        <input
          type="range"
          onChange={(e) => setSimSpeed(e.target.value)}
          value={simSpeed}
          min="0.5"
          max="10"
          step="0.5"
        />
      </label>
      <p>{simSpeed}x</p>

      <button onClick={() => setIsPaused(!isPaused)} className="btn">
        {isPaused ? "Continue" : "Pause"}
      </button>
      <button onClick={() => setShowConfigModal(true)} className="btn">
        Config
      </button>
    </div>
  );
}
