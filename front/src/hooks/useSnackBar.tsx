import { PropsWithChildren, useContext } from "react";
import { createContext, useState, useMemo, useCallback, ReactNode, useEffect, useRef, FC } from "react";

export type SnackBarLevel = "info" | "warning" | "error";

export type SnackBarType = {
  content: ReactNode;
  level: SnackBarLevel;
  autoHide?: boolean;
};

export type SnackBarContextType = {
  snackBar: SnackBarType | undefined;
  isOpen?: boolean;
  createSnackBar: (snackbar: SnackBarType) => void;
  closeSnackBar?: () => void;
};

export const SnackBarContext = createContext<SnackBarContextType | undefined>(undefined);

export const SnackBarProvider: FC<PropsWithChildren> = ({ children }) => {
  const [isOpen, setIsOpen] = useState<boolean>(false);
  const [snackBar, setSnackBar] = useState<SnackBarType | undefined>(undefined);
  let timeout = useRef(0);
  const createSnackBar = useCallback((snackbar: SnackBarType) => {
    setSnackBar(snackbar);
    setIsOpen(true);
  }, []);

  const closeSnackBar = useCallback(() => {
    setSnackBar(undefined);
    setIsOpen(false);
  }, []);

  const context = useMemo(
    () => ({
      isOpen,
      snackBar,
      createSnackBar,
      closeSnackBar
    }),
    [isOpen, snackBar, createSnackBar, closeSnackBar]
  );

  useEffect(() => {
    if (snackBar && (snackBar.autoHide==true || typeof snackBar.autoHide == "undefined")) {
      timeout.current = window.setTimeout(() => {
        setIsOpen(false);
        setSnackBar(undefined);
      }, 4000);
    }
  }, [snackBar, timeout]);

  return (
    <SnackBarContext.Provider value={context}>
      {children}
    </SnackBarContext.Provider>
  );
};

export const useSnackBar = (): SnackBarContextType => {
  const context = useContext(SnackBarContext);
  if (context === undefined) {
    throw new Error(
      "useSnackBarContext must be used within a SnackBarProvider"
    );
  }
  return context;
};
