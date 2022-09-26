import "./App.css";
import { useState } from "react";
import axios from "axios";
function App() {
  const [Data, setData] = useState();
  const [Data2, setData2] = useState();

  const test = async () => {
    await axios
      .post("http://localhost:8000/read", {})
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
        name: "The Flash",
        identity: "1",
        hometown: "2",
        age: 29
      })
      .then((res) => {
        console.log(res.data);
        setData(res.data.title);
      })
      .catch((err) => {
        console.log(err);
      });
  };

  const test4 = async () => {
    await axios
      .delete("http://localhost:8000/hello/4", {
        name: "The Flash",
        identity: "1",
        hometown: "2",
        age: 29
      })
      .then((res) => {
        console.log(res.data);
        setData(res.data.title);
      })
      .catch((err) => {
        console.log(err);
      });
  };
  const test5 = async () => {
    await axios
      .post("http://localhost:8000/page_view/", {
        name: "The Flash",
        identity: "1",
        hometown: "2",
        age: 29
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
        <button onClick={test}>read</button>
        <button onClick={test3}>post</button>
        <button onClick={test4}>delete</button>
        <button onClick={test5}>newpost</button>
        <div> {Data}</div>
        <div> {Data2}</div>
      </header>
    </div>
  );
}

export default App;
