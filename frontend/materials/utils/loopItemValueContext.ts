import { createContext, useContext } from "react";

const LoopItemValueContext = createContext({});

export default LoopItemValueContext;

export const useLoopItemValueContext = () => {
  const context = useContext(LoopItemValueContext);
  if (!context) {
    return {};
  }
  return context;
};