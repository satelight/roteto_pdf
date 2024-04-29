import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import 'bulma/css/bulma.css';

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <div className="container is-max-desktop">
      <h1 className="title">KAITEN PDF</h1>

      <div className="row">
      
      </div>

      <p>Click on the Tauri, Vite, and React logos to learn more.</p>
      <div className="columns">
          
          <div className="column">
            <label htmlFor="greet-input">ファイル場所:</label>
            <input type="text" className="input" id="greet-input"onChange={(e) => setName(e.currentTarget.value)} placeholder="Enter a name..."/>
          </div>

          <div className="column">
            <button className="button is-primary is-light mt-5">Greet</button>
          </div>
        
      </div>

      <p>{greetMsg}</p>
    </div>
  );
}

export default App;
