import LeftSidebar from "./components/LeftSidebar";
import RightSidebar from "./components/RightSidebar";
import Simulation from "./components/Simulation";

export default function App() {
  return (
    <div className="App">
      <LeftSidebar>
        <h2>Controls</h2>
      </LeftSidebar>
      <RightSidebar>
        <h2>Stats</h2>
      </RightSidebar>
      <Simulation />
    </div>
  );
}
