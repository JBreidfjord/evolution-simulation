import './Sidebar.css';

import { useState } from 'react';

interface LeftSidebarProps {
  children: React.ReactNode
}

export default function LeftSidebar({ children }: LeftSidebarProps): JSX.Element {
  const [open, setOpen] = useState('');

  const handleClose = (e: React.MouseEvent<HTMLDivElement>): void => {
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
