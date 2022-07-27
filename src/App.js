import "./App.css";

import { useEffect, useState } from "react";

import ConfigForm from "./components/modals/ConfigForm";
import Entry from "./components/modals/Entry";
import Extinction from "./components/modals/Extinction";
import Modal from "./components/modals/Modal";
import Controls from "./components/sidebars/Controls";
import LeftSidebar from "./components/sidebars/LeftSidebar";
import RightSidebar from "./components/sidebars/RightSidebar";
import Statistics from "./components/sidebars/Statistics";
import Simulation from "./components/Simulation";
import { useSim } from "./hooks/useSim";

export default function App() {
  const { simulation, world, setIsPaused } = useSim();
  const [showConfigModal, setShowConfigModal] = useState(false);
  const [showExtinctionModal, setShowExtinctionModal] = useState(false);
  const [clickOpenModal, setClickOpenModal] = useState(false);
  const [showEntryModal, setShowEntryModal] = useState(true);
  const [simReady, setSimReady] = useState(false);

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

  // Check for extinction
  useEffect(() => {
    if (world.creatures.length === 0 && simulation.age() > 0) {
      setIsPaused(true);
      setSimReady(false);
      handleExtinctionModalOpen();
    }
  }, [world]);

  useEffect(() => {
    if (simReady) {
      setShowEntryModal(false);
      handleExtinctionModalClose(false);
    }
  }, [simReady]);

  return (
    <div className="App">
      <LeftSidebar>
        <Controls setShowConfigModal={ setShowConfigModal } />
      </LeftSidebar>
      <RightSidebar>
        <Statistics />
      </RightSidebar>
      { clickOpenModal && (
        <div className="modal-open-screen" onClick={ () => handleExtinctionModalOpen() }></div>
      ) }
      <Simulation />
      { showConfigModal && (
        <Modal handleClose={ () => setShowConfigModal(false) }>
          <ConfigForm handleClose={ () => setShowConfigModal(false) } />
        </Modal>
      ) }
      { showExtinctionModal && (
        <Modal handleClose={ handleExtinctionModalClose }>
          <Extinction handleClose={ handleExtinctionModalClose } setSimReady={ setSimReady } />
        </Modal>
      ) }
      { showEntryModal && (
        <Modal>
          <Entry setSimReady={ setSimReady } />
        </Modal>
      ) }
    </div>
  );
}
