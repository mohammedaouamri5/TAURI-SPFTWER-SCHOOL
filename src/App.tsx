import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

import Sidebar from "./sidebare";
import Action from "./action";
import Button from "./button";


function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("cdscdfvsfc");

  const greecct = () => {
    console.log(  invoke("bruh", {  }));
  };


  return (
    <div className="container">
      <div className="sidebare"> <Button onClick={greecct} label="Click Me" /> <Sidebar/></div>
      <div className="action"><Action/></div>
    </div>
  );
} 

export default App;
