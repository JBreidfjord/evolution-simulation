import ConfigForm from "./components/ConfigForm";
import Controls from "./components/Controls";
import LeftSidebar from "./components/LeftSidebar";
import Modal from "./components/Modal";
import RightSidebar from "./components/RightSidebar";
import { SimProvider } from "./context/SimContext";
import Simulation from "./components/Simulation";
import Statistics from "./components/Statistics";
import { useState } from "react";

export default function App() {
  const [showConfigModal, setShowConfigModal] = useState(false);

  const handleConfigModalClose = () => {
    setShowConfigModal(false);
  };

  return (
    <div className="App">
      <SimProvider>
        <LeftSidebar>
          <Controls setShowConfigModal={setShowConfigModal} />
        </LeftSidebar>
        <RightSidebar>
          <Statistics />
        </RightSidebar>
        <Simulation />
        {showConfigModal && (
          <Modal>
            <ConfigForm handleClose={handleConfigModalClose} />
          </Modal>
        )}
      </SimProvider>
    </div>
  );
}
