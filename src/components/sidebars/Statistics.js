import "./Statistics.css";

import { useEffect, useState } from "react";

import { useSim } from "../../hooks/useSim";

export default function Statistics() {
  const { world } = useSim();

  const [populationCount, setPopulationCount] = useState(0);
  const [maxFitness, setMaxFitness] = useState(0);
  const [minFitness, setMinFitness] = useState(0);
  const [avgFitness, setAvgFitness] = useState(0);
  const [minGen, setMinGen] = useState(0);
  const [maxGen, setMaxGen] = useState(0);

  useEffect(() => {
    if (world) {
      setPopulationCount(world.creatures.length);
      const creatureFitness = world.creatures.map((creature) => creature.fitness);
      setMaxFitness(Math.max(...creatureFitness));
      setMinFitness(Math.min(...creatureFitness));
      setAvgFitness(
        (creatureFitness.reduce((a, b) => a + b, 0) / creatureFitness.length).toFixed(2)
      );
      setMinGen(Math.min(...world.creatures.map((creature) => creature.generation)));
      setMaxGen(Math.max(...world.creatures.map((creature) => creature.generation)));
    }
  }, [world]);

  return (
    <div className="statistics">
      <h2>Statistics</h2>
      {world && (
        <>
          <p>Population Count: {populationCount}</p>
          <p>Max Fitness: {maxFitness}</p>
          <p>Min Fitness: {minFitness}</p>
          <p>Avg Fitness: {avgFitness}</p>
          <p>Oldest Gen: {minGen}</p>
          <p>Youngest Gen: {maxGen}</p>
        </>
      )}
    </div>
  );
}
