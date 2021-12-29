import "./Controls.css";

export default function Controls({ simSpeed, setSimSpeed }) {
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
    </div>
  );
}
