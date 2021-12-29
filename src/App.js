import Controls from "./components/Controls";
import LeftSidebar from "./components/LeftSidebar";
import RightSidebar from "./components/RightSidebar";
import { SimProvider } from "./context/SimContext";
import Simulation from "./components/Simulation";
import Statistics from "./components/Statistics";

export default function App() {
  return (
    <div className="App">
      <SimProvider>
        <LeftSidebar>
          <Controls />
        </LeftSidebar>
        <RightSidebar>
          <Statistics />
        </RightSidebar>
        <Simulation />
      </SimProvider>
    </div>
  );
}
