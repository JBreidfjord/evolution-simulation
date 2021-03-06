import './Statistics.css';

import { useEffect, useState } from 'react';

import { useSim } from '../../hooks/useSim';

export default function Statistics(): JSX.Element {
  const { world } = useSim();

  const [populationCount, setPopulationCount] = useState(0);
  const [foodCount, setFoodCount] = useState(0);
  const [maxFitness, setMaxFitness] = useState(0);
  const [minFitness, setMinFitness] = useState(0);
  const [avgFitness, setAvgFitness] = useState(0);
  const [minGen, setMinGen] = useState(0);
  const [maxGen, setMaxGen] = useState(0);
  const [bestGen, setBestGen] = useState(0);
  const [bestGenFitness, setBestGenFitness] = useState(0);
  const [avgSize, setAvgSize] = useState(0);

  const calculateGenStats = (): void => {
    const min = Math.min(...world.creatures.map((creature) => creature.generation));
    const max = Math.max(...world.creatures.map((creature) => creature.generation));
    setMinGen(min);
    setMaxGen(max);

    // Create map of each generation's [total fitness, number of individuals]
    const genMap = new Map<number, [number, number]>();
    for (let i = min; i <= max; i++) {
      genMap.set(i, [0, 0]);
    }

    world.creatures.forEach((creature) => {
      if (!genMap.has(creature.generation)) throw new Error('Error parsing generation map');
      (genMap.get(creature.generation) as [number, number])[0] += creature.fitness;
      (genMap.get(creature.generation) as [number, number])[1] += 1;
    });

    if (Object.keys(genMap).length > 0) {
      const gen = Object.keys(genMap).reduce((a, b) => {
        const [aFitness, aCount] = genMap.get(parseInt(a)) as [number, number];
        const [bFitness, bCount] = genMap.get(parseInt(b)) as [number, number];
        return aFitness / aCount > bFitness / bCount ? a : b;
      });
      const intGen = parseInt(gen);
      setBestGen(intGen);
      const [genFitness, genCreatureCount] = genMap.get(intGen) as [number, number];
      setBestGenFitness(genFitness / genCreatureCount);
    }
  };

  useEffect(() => {
    if (world) {
      setPopulationCount(world.creatures.length);
      setFoodCount(world.foods.length);
      const creatureFitness = world.creatures.map((creature) => creature.fitness);
      setMaxFitness(Math.max(...creatureFitness));
      setMinFitness(Math.min(...creatureFitness));
      setAvgFitness(creatureFitness.reduce((a, b) => a + b, 0) / creatureFitness.length);
      setAvgSize(
        world.creatures.map((creature) => creature.size).reduce((a, b) => a + b, 0) /
        world.creatures.length
      );
      calculateGenStats();
    }
  }, [world]);

  return (
    <div className="statistics">
      <h2>Statistics</h2>
      {world && (
        <>
          <p>Population Count: {populationCount}</p>
          <p>Food Count: {foodCount}</p>
          <p>Max Fitness: {maxFitness}</p>
          <p>Min Fitness: {minFitness}</p>
          <p>Avg Fitness: {avgFitness.toFixed(2)}</p>
          <p>Avg Size: {avgSize.toFixed(3)}</p>
          <p>Oldest Gen: {minGen}</p>
          <p>Youngest Gen: {maxGen}</p>
          <p>
            Best Gen: {bestGen} ({bestGenFitness.toPrecision(3)})
          </p>
        </>
      )}
    </div>
  );
}
