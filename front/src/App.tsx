import './App.css';
import { SnackBar } from './components/Snackbar';
import { SnackBarProvider } from './hooks/useSnackBar';
import { Login } from './pages/Login';

function App() {

  return (
    <SnackBarProvider>
      <div className='min-h-screen flex justify-center items-center'>
        <Login />
      </div>
      <SnackBar />
    </SnackBarProvider>
  );
}

export default App
