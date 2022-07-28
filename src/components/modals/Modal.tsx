import './Modal.css';

import ReactDOM from 'react-dom';

interface ModalProps {
  children: React.ReactNode;
  handleClose?(): void
}

export default function Modal({ children, handleClose }: ModalProps): JSX.Element {
  const handleClick = (e: React.MouseEvent<HTMLDivElement>): void => {
    e.preventDefault();
    if (e.currentTarget === e.target && handleClose) {
      handleClose();
    }
  };

  return ReactDOM.createPortal(
    <div className="modal-backdrop" onClick={ handleClick }>
      <div className="modal">{ children }</div>
    </div>,
    document.body
  );
}
