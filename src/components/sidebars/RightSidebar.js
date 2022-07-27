import './Sidebar.css';

import { useState } from 'react';

export default function RightSidebar({ children }) {
  const [open, setOpen] = useState('');

  const handleClose = (e) => {
    e.preventDefault();
    if (e.currentTarget === e.target) {
      setOpen('');
    }
  };

  return (
    <>
      <div className="sidebar-toggle right" onClick={() => setOpen('open')}>
        {'<'}
      </div>
      <div className={'sidebar right' + ` ${open}`} onClick={handleClose}>
        {open && children}
      </div>
    </>
  );
}
