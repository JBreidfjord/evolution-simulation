import * as sim from "../build/lib_simulation_wasm";

import React from "react";
import ReactDOM from "react-dom";

const App = () => {
  const simulation = new sim.Simulation();

  const handleClick = () => {
    const world = simulation.world();
    console.log(world.foods.length);
    console.log(world);
  };

  return (
    <>
      <div>
        <button onClick={handleClick}>Click me</button>
      </div>
    </>
  );
};

ReactDOM.render(<App />, document.getElementById("root"));
