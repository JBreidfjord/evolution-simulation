import "./Sidebar.css";

import { useState } from "react";

export default function RightSidebar({ children }) {
  const [open, setOpen] = useState("");

  return (
    <>
      <div className="sidebar-toggle right" onClick={() => setOpen("open")}></div>
      <div className={"sidebar right" + ` ${open}`} onClick={() => setOpen("")}>
        {open && children}
      </div>
    </>
  );
}
