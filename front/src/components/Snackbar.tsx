import { FC } from "react";
import { createPortal } from "react-dom";
import { SnackBarLevel, useSnackBar } from "../hooks/useSnackBar";

export const SnackBar: FC = () => {
    const { snackBar, isOpen, closeSnackBar } = useSnackBar();
    if (isOpen && snackBar?.content) {
        
        return createPortal(
            <div className="absolute bottom-2 left-2">
                <div className={`${getColor(snackBar.level)} p-2 rounded-md px-4 items-center text-indigo-100 leading-none lg:rounded-full flex lg:inline-flex`} role="alert">
                    <span className="font-semibold mr-2 text-left flex-auto">
                        {snackBar?.content}
                    </span>
                    <svg className="fill-current h-6 w-6 " role="button" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" onClick={closeSnackBar}>
                        <title>Close</title>
                        <path d="M14.348 14.849a1.2 1.2 0 0 1-1.697 0L10 11.819l-2.651 3.029a1.2 1.2 0 1 1-1.697-1.697l2.758-3.15-2.759-3.152a1.2 1.2 0 1 1 1.697-1.697L10 8.183l2.651-3.031a1.2 1.2 0 1 1 1.697 1.697l-2.758 3.152 2.758 3.15a1.2 1.2 0 0 1 0 1.698z" />
                    </svg>
                </div>
            </div>,
            document.body
        );
    }
    return null;
};

function getColor(level: SnackBarLevel) {
    if(level == "error") return "bg-red-500";
    if(level == "warning") return "bg-orange-500";
    return "bg-blue-500";
}