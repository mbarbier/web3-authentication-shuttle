import './App.css';
import { api } from './back/api';
import { fromUtf8 } from './back/utils';

function App() {


  const login = async () => {

    let reqAccount = await window.ethereum.request({ method: 'eth_requestAccounts' })
    console.log(reqAccount);

    const accounts = await window.ethereum.request({
      id: '191',
      method: 'eth_accounts',
      params: [],
    })
    if (accounts.length === 0) {
      console.log('No accounts allowed')
      return
    };

    const from = accounts[0] as string;

    let nonce = await api().getNonce(from);
    // let nonceHex = fromUtf8(nonce);
    // console.log("nonceHex: " + nonceHex);

    const params = [nonce, from]
    let signature = await window.ethereum.request({
      method: 'personal_sign',
      params
    });
    console.log("Signature => " + signature);
    
    let response = await api().authenticate(from, nonce, signature);
    console.log(response);
  }

  if (window.ethereum) {
    const isBraveWallet = window.ethereum.isBraveWallet
    console.log('Brave Wallet: ', isBraveWallet)
  } else {
    console.log("No wallet");
  }

  return (
    <div className='min-h-screen flex justify-center items-center'>
      <div className=''>
        <div className='text-2xl font-bold text-blue-600'>
          Welcome !
        </div>
        <div className='text-xl font-bold text-blue-600'>
          This demo needs a Brave browser with the wallet setup.
        </div>
        <div className='m-10'>
          <button className="btn btn-blue" onClick={login}>
            Login
          </button>
        </div>
      </div>
    </div>
  );
}

export default App
