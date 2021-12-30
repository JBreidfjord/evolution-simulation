import { SimContext } from "../context/SimContext";
import { useContext } from "react";

export const useSim = () => {
  const context = useContext(SimContext);

  if (!context) {
    throw new Error("useSim must be used inside a SimProvider");
  }

  return context;
};
