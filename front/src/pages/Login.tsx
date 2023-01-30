import { FC } from "react";
import { api, ApiError, uapi } from "../back/api";
import { SnackBarLevel, useSnackBar } from "../hooks/useSnackBar";

export const Login: FC = () => {

    const { createSnackBar } = useSnackBar();

    const showMessage = (message: string, level: SnackBarLevel) => {
        createSnackBar({ level, content: <span>{message}</span> });
    }

    const login = async () => {

        try {
            if (!window.ethereum) {
                showMessage("You need an ethereum wallet to authenticate", "warning");
                return;
            }

            await window.ethereum.request({ method: 'eth_requestAccounts' })

            const accounts = await window.ethereum.request({
                id: '191',
                method: 'eth_accounts',
                params: [],
            });
            if (accounts.length === 0) {
                showMessage("No account allowed for signin", "warning");
                return;
            };

            const from = accounts[0] as string;

            let nonce = await uapi().getNonce(from);

            const params = [nonce, from]
            let signature = await window.ethereum.request({
                method: 'personal_sign',
                params
            });

            let ok = await uapi().authenticate(from, nonce, signature);
            if (ok) {
                showMessage("You've been succesfully authenticated", "info");
            } else {
                showMessage("Authentication failed", "error");
            }

        } catch (e) {
            console.error(e);
            if (e instanceof ApiError) {
                showMessage(e.message, "error");
            }
        }
    }

    return (
        <div className=''>
            <div className='text-2xl font-bold text-blue-600'>
                Welcome !
            </div>
            <div className='text-xl font-bold text-blue-600'>
                <p>
                    This demo requires a Brave browser with the wallet initialized.
                    <br />
                    The wallet will be used to sign a message (no transaction fees involved).
                </p>
            </div>


            <div className='m-10'>
                <button className="btn btn-blue" onClick={login}>
                    Login
                </button>
            </div>
        </div>
    );
}
