import './Sidebar.css';

import { useState } from 'react';

interface RightSidebarProps {
  children: React.ReactNode
}

export default function RightSidebar({ children }: RightSidebarProps): JSX.Element {
  const [open, setOpen] = useState('');

  const handleClose = (e: React.MouseEvent<HTMLDivElement>): void => {
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
