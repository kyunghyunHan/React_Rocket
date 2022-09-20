import "./App.css";
import { useState } from "react";
import axios from "axios";
function App() {
  const [Data, setData] = useState();
  const [Data2, setData2] = useState();

  const getData = async () => {
    await axios
      .get("http://localhost:8000/myrocket")
      .then((res) => {
        console.log(res.data);
        setData2(res.data);
      })
      .catch(() => {});
  };
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
        <button onClick={getData}>dd</button>
        <div> {Data}</div>
        <div> {Data2}</div>
      </header>
    </div>
  );
}

export default App;
