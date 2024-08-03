import React, { useEffect, useRef, useState } from "react";

import init, { setup } from "emulator";

import wasmData from "emulator/emulator_bg.wasm";
await init(wasmData);

import * as styles from "./App.module.scss";

export default function App() {
  const inputStates = useRef([...Array(16)].map((_, i) => false));
  const lastKeyUp = useRef(null);
  const [emuState, setEmuState] = useState(null);
  const pressInput = (i) => {
    inputStates.current.splice(i, 1, true);
  };
  const releaseInput = (i) => {
    if (inputStates.current[i]) {
      inputStates.current.splice(i, 1, false);
      lastKeyUp.current = i;
    }
  };

  useEffect(() => {
    document.getElementById("rom-input").addEventListener("change", (event) => {
      const reader = new FileReader();
      reader.onload = (e) => {
        setEmuState(setup(new Uint8Array(e.target.result)));
      };
      reader.readAsArrayBuffer(event.target.files[0]);
    });
  });

  const loopFn = () => {
    emuState.update(
      inputStates.current,
      document.getElementById("clock-speed").value,
      lastKeyUp.current
    );
    emuState.render(
      document.getElementById("foreground-color").value,
      document.getElementById("background-color").value
    );
    lastKeyUp.current = null;
    requestAnimationFrame(loopFn);
  };

  useEffect(() => {
    if (emuState == null) return;
    requestAnimationFrame(loopFn);
  }, [emuState]);

  return (
    <div className={styles.container}>
      <canvas id="canvas" width="800" height="400" className={styles.canvas} />
      <input id="rom-input" type="file" />
      <input type="range" min="1" max="2000" id="clock-speed" />
      <input type="color" id="foreground-color" />
      <input type="color" id="background-color" />
      <div className={styles.inputs}>
        {[
          0x1, 0x2, 0x3, 0xc, 0x4, 0x5, 0x6, 0xd, 0x7, 0x8, 0x9, 0xe, 0xa, 0x0,
          0xb, 0xf,
        ].map((k) => (
          <React.Fragment key={k}>
            <div className={styles.buttonContainer} key={k}>
              <button
                className={styles.button}
                id={`input_${k}`}
                onMouseDown={() => pressInput(k)}
                onMouseUp={() => releaseInput(k)}
                onMouseLeave={() => releaseInput(k)}
              >
                {k.toString(16).toUpperCase()}
              </button>
            </div>
          </React.Fragment>
        ))}
      </div>
    </div>
  );
}
