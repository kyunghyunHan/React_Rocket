import logo from "./logo.svg";
import "./App.css";
import { useEffect } from "react";
import axios from "axios";
function App() {
  const getData = async () => {
    const response = await axios("http://localhost:8000/myrocket");
    console.log(response.data);
  };
  const getData2 = async () => {
    const response = await axios("http://localhost:8000/login");
    console.log(response.data);
  };
  useEffect(() => {
    getData();
    getData2();
  }, []);

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.js</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );
}

export default App;
