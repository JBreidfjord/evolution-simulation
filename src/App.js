import "./App.css";

import ConfigForm from "./components/modals/ConfigForm";
import Controls from "./components/sidebars/Controls";
import Extinction from "./components/modals/Extinction";
import LeftSidebar from "./components/sidebars/LeftSidebar";
import Modal from "./components/modals/Modal";
import RightSidebar from "./components/sidebars/RightSidebar";
import { SimProvider } from "./context/SimContext";
import Simulation from "./components/Simulation";
import Statistics from "./components/sidebars/Statistics";
import { useState } from "react";

export default function App() {
  const [showConfigModal, setShowConfigModal] = useState(false);
  const [showExtinctionModal, setShowExtinctionModal] = useState(false);
  const [clickOpenModal, setClickOpenModal] = useState(false);

  const handleConfigModalClose = () => {
    setShowConfigModal(false);
  };

  const handleExtinctionModalClose = (enableClickOpen = true) => {
    setShowExtinctionModal(false);
    if (enableClickOpen) {
      setClickOpenModal(true);
    }
  };

  const handleExtinctionModalOpen = () => {
    setShowExtinctionModal(true);
    setClickOpenModal(false);
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
        {clickOpenModal && (
          <div className="modal-open-screen" onClick={() => handleExtinctionModalOpen()}></div>
        )}
        <Simulation setShowExtinctionModal={setShowExtinctionModal} />
        {showConfigModal && (
          <Modal handleClose={handleConfigModalClose}>
            <ConfigForm handleClose={handleConfigModalClose} />
          </Modal>
        )}
        {showExtinctionModal && (
          <Modal handleClose={handleExtinctionModalClose}>
            <Extinction
              handleClose={handleExtinctionModalClose}
              setShowConfigModal={setShowConfigModal}
            />
          </Modal>
        )}
      </SimProvider>
    </div>
  );
}
