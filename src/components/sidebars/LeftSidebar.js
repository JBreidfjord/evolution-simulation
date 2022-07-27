import './Sidebar.css';

import { useState } from 'react';

export default function LeftSidebar({ children }) {
  const [open, setOpen] = useState('');

  const handleClose = (e) => {
    e.preventDefault();
    if (e.currentTarget === e.target) {
      setOpen('');
    }
  };

  return (
    <>
      <div className="sidebar-toggle left" onClick={() => setOpen('open')}>
        {'>'}
      </div>
      <div className={'sidebar left' + ` ${open}`} onClick={handleClose}>
        {open && children}
      </div>
    </>
  );
}
