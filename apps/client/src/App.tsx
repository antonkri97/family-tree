import { useEffect, useState } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'

function App() {
  const [message, setMessage] = useState("Loading...");

  useEffect(() => {
    fetch(import.meta.env.VITE_API_URL || "http://localhost:8000")
      .then((res) => res.text())
      .then(setMessage)
      .catch(() => setMessage("Error fetching data"));
  }, []);

  return <h1>{message}</h1>;
}

export default App
