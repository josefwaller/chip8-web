import React, { useEffect, useRef, useState } from "react";

import init, { setup } from "emulator";

import wasmData from "emulator/emulator_bg.wasm";
await init(wasmData);

import Inputs from "./Inputs";
import SettingsButton from "./SettingsButton";
import RomButton from "./RomButton";

import * as styles from "./App.module.scss";
import "rsuite/Slider/styles/index.css";

export default function App() {
  const inputStates = useRef([...Array(16)].map((_, i) => false));
  const lastKeyUp = useRef(null);
  const fg = useRef("#bc4d47");
  const bg = useRef("#340e0b");
  const clockSpeed = useRef(1000);
  const [emuState, setEmuState] = useState(null);
  const frameId = useRef(null);

  const pressInput = (i) => {
    inputStates.current.splice(i, 1, true);
  };
  const releaseInput = (i) => {
    if (inputStates.current[i]) {
      inputStates.current.splice(i, 1, false);
      lastKeyUp.current = i;
    }
  };

  const loopFn = () => {
    emuState.update(inputStates.current, clockSpeed.current, lastKeyUp.current);
    emuState.render(fg.current, bg.current);
    lastKeyUp.current = null;
    frameId.current = requestAnimationFrame(loopFn);
  };

  const loadProgram = (bytes) => setEmuState(setup(bytes));

  useEffect(() => {
    if (emuState == null) return;
    if (frameId.current) cancelAnimationFrame(frameId.current);
    frameId.current = requestAnimationFrame(loopFn);
  }, [emuState]);

  return (
    <div className={styles.container}>
      <canvas id="canvas" width="800" height="400" className={styles.canvas} />
      <Inputs pressInput={pressInput} releaseInput={releaseInput} />

      <div className={styles.buttonContainer}>
        <RomButton loadProgram={loadProgram} />
        <SettingsButton fg={fg} bg={bg} clockSpeed={clockSpeed} />
      </div>
    </div>
  );
}
