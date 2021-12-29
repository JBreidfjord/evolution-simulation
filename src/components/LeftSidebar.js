import "./Sidebar.css";

import { useState } from "react";

export default function LeftSidebar({ children }) {
  const [open, setOpen] = useState("");

  return (
    <>
      <div className="sidebar-toggle left" onClick={() => setOpen("open")}></div>
      <div className={"sidebar left" + ` ${open}`} onClick={() => setOpen("")}>
        {open && children}
      </div>
    </>
  );
}
