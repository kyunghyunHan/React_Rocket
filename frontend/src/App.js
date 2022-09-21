import "./App.css";
import { useState } from "react";
import axios from "axios";
function App() {
  const [Data, setData] = useState();
  const [Data2, setData2] = useState();

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

  const test3 = async () => {
    await axios
      .post("http://localhost:8000/hello", {
        id: null,
        name: "The Flash",
        identity: "1",
        hometown: "2",
        age: "29"
      })
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
        <button onClick={test}>notice1</button>
        <button onClick={test3}>hellopost</button>
        <div> {Data}</div>
        <div> {Data2}</div>
      </header>
    </div>
  );
}

export default App;
