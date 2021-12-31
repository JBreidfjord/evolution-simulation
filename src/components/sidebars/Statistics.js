import "./Statistics.css";

import { useEffect, useState } from "react";

import { useSim } from "../../hooks/useSim";

export default function Statistics() {
  const { world } = useSim();

  const [remainingPopulation, setRemainingPopulation] = useState(0);
  const [maxFitness, setMaxFitness] = useState(0);
  const [minFitness, setMinFitness] = useState(0);
  const [avgFitness, setAvgFitness] = useState(0);
  const [avgSurvivingFitness, setAvgSurvivingFitness] = useState(0);

  useEffect(() => {
    if (world) {
      setRemainingPopulation(world.creatures.filter((creature) => creature.alive).length);
      const creatureFitness = world.creatures.map((creature) => creature.fitness);
      setMaxFitness(Math.max(...creatureFitness));
      setMinFitness(Math.min(...creatureFitness));
      setAvgFitness(
        (creatureFitness.reduce((a, b) => a + b, 0) / creatureFitness.length).toFixed(2)
      );
      setAvgSurvivingFitness(
        ((avgFitness * creatureFitness.length) / remainingPopulation).toFixed(2)
      );
    }
  }, [world]);

  return (
    <div className="statistics">
      <h2>Statistics</h2>
      {world && (
        <>
          <p>Population Count: {remainingPopulation}</p>
          <p>Max Fitness: {maxFitness}</p>
          <p>Min Fitness: {minFitness}</p>
          <p>Avg Fitness: {avgFitness}</p>
        </>
      )}
    </div>
  );
}
