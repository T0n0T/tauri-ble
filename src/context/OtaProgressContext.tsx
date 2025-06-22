import React, { createContext, useState, useContext, ReactNode } from 'react';

interface OtaProgressContextType {
  otaProgress: number;
  setOtaProgress: React.Dispatch<React.SetStateAction<number>>;
  otaInProgress: boolean;
  setOtaInProgress: React.Dispatch<React.SetStateAction<boolean>>;
}

const OtaProgressContext = createContext<OtaProgressContextType | undefined>(undefined);

export const OtaProgressProvider = ({ children }: { children: ReactNode }) => {
  const [otaProgress, setOtaProgress] = useState(0);
  const [otaInProgress, setOtaInProgress] = useState(false);

  return (
    <OtaProgressContext.Provider value={{ otaProgress, setOtaProgress, otaInProgress, setOtaInProgress }}>
      {children}
    </OtaProgressContext.Provider>
  );
};

export const useOtaProgress = () => {
  const context = useContext(OtaProgressContext);
  if (context === undefined) {
    throw new Error('useOtaProgress must be used within an OtaProgressProvider');
  }
  return context;
};