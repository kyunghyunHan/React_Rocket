import logo from "./logo.svg";
import "./App.css";
import { useEffect, useState } from "react";
import axios from "axios";
function App() {
  const [Data, setData] = useState();
  const getData = async () => {
    const response = await axios("http://localhost:8000/myrocket");
    console.log(response.data);
  };

  useEffect(() => {
    getData();
  }, []);
  const test = async () => {
    await axios
      .get("http://localhost:8000/hello", {})
      .then((res) => {
        console.log(res.data);
        setData(res.data.title);
      })
      .catch((err) => {
        console.log(err);
      });
  };

  return (
    <div className="App">
      <header className="App-header">
        <button onClick={test}>dd</button>
        {Data}
      </header>
    </div>
  );
}

export default App;
