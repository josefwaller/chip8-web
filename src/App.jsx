import React, { useEffect, useState } from "react";

import init, { start_main_loop } from "emulator";

import wasmData from "emulator/emulator_bg.wasm";
await init(wasmData);

import * as styles from "./App.module.scss";

export default function App() {
  const [inputStates, setInputStates] = useState(
    Object.fromEntries([...Array(16)].map((_, i) => [i, false]))
  );
  const pressInput = (i) =>
    setInputStates({
      ...inputStates,
      [i]: true,
    });
  const releaseInput = (i) =>
    setInputStates({
      ...inputStates,
      [i]: false,
    });

  useEffect(() => {
    document.getElementById("rom-input").addEventListener("change", (event) => {
      const reader = new FileReader();
      reader.onload = (e) => {
        console.log(e);
        console.log(e.target.result);
        start_main_loop(new Uint8Array(e.target.result));
      };
      reader.readAsArrayBuffer(event.target.files[0]);
    });
  });

  return (
    <div className={styles.container}>
      <canvas id="canvas" width="800" height="400" className={styles.canvas} />
      <input id="rom-input" type="file" />
      <input type="range" min="1" max="1000" id="clock-speed" value="500" />
      <button type="button" id="button_0"></button>
      <div className={styles.inputs}>
        {[...Array(16).keys()].map((k) => (
          <React.Fragment key={k}>
            {k % 3 == 0 && <br />}
            <div className={styles.buttonContainer} key={k}>
              <button className={styles.button} id={`input_${k}`}>
                {k.toString(16).toUpperCase()}
              </button>
            </div>
          </React.Fragment>
        ))}
      </div>
      <input type="file" />
    </div>
  );
}
