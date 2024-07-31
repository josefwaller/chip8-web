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
      <input type="range" min="1" max="2000" id="clock-speed" />
      <button type="button" id="button_0"></button>
      <div className={styles.inputs}>
        {[
          0x1, 0x2, 0x3, 0xc, 0x4, 0x5, 0x6, 0xd, 0x7, 0x8, 0x9, 0xe, 0xa, 0x0,
          0xb, 0xf,
        ].map((k) => (
          <React.Fragment key={k}>
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
