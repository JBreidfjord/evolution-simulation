import Controls from "./components/Controls";
import LeftSidebar from "./components/LeftSidebar";
import RightSidebar from "./components/RightSidebar";
import Simulation from "./components/Simulation";
import Statistics from "./components/Statistics";
import { useState } from "react";

export default function App() {
  const [world, setWorld] = useState(null);
  const [simSpeed, setSimSpeed] = useState(1);

  return (
    <div className="App">
      <LeftSidebar>
        <Controls simSpeed={simSpeed} setSimSpeed={setSimSpeed} />
      </LeftSidebar>
      <RightSidebar>
        <Statistics world={world} />
      </RightSidebar>
      <Simulation world={world} setWorld={setWorld} simSpeed={simSpeed} />
    </div>
  );
}
