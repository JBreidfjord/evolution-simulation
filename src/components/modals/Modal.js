import "./Modal.css";

import ReactDOM from "react-dom";

export default function Modal({ children, handleClose }) {
  const handleClick = (e) => {
    e.preventDefault();
    if (e.currentTarget === e.target && handleClose) {
      handleClose();
    }
  };

  return ReactDOM.createPortal(
    <div className="modal-backdrop" onClick={handleClick}>
      <div className="modal">{children}</div>
    </div>,
    document.body
  );
}
