import React, { useEffect, useRef, useState } from "react";

import init, { EmulatorInfo, setup } from "emulator";

import wasmData from "emulator/emulator_bg.wasm";
await init(wasmData);

import Inputs from "./Inputs";
import SettingsButton from "./SettingsButton";
import RomButton from "./RomButton";

import * as styles from "./App.module.scss";
import "rsuite/Slider/styles/index.css";

const DEFAULT_VOLUME = 0.1;

export default function App() {
  const inputStates = useRef([...Array(16)].map((_, i) => false));
  const fg = useRef("#aa5656");
  const bg = useRef("#481818");
  // Gain used to control whether or not the beep should play
  // Set to 0 when ST = 0 and 1 otherwise
  const controlGain = useRef(null);
  // Gain used to control the user defined volume
  const volumeGain = useRef(null);
  // The actual sound, just used as a flag to check if it has been initialized or not
  const sound = useRef(null);
  const clockSpeed = useRef(1000);
  const [emuState, setEmuState] = useState(null);
  const frameId = useRef(null);

  const pressInput = (i) => {
    inputStates.current.splice(i, 1, true);
  };
  const releaseInput = (i) => {
    if (inputStates.current[i]) {
      inputStates.current.splice(i, 1, false);
    }
  };

  const loopFn = () => {
    emuState.update(inputStates.current, clockSpeed.current);
    emuState.render(fg.current, bg.current);
    controlGain.current.gain.value = emuState.getSound() ? 1.0 : 0.0;
    frameId.current = requestAnimationFrame(loopFn);
  };

  const loadProgram = (bytes) => {
    setEmuState(new EmulatorInfo(bytes));
    if (sound.current === null) {
      initSound();
    }
  };

  useEffect(() => {
    if (emuState == null) return;
    if (frameId.current) cancelAnimationFrame(frameId.current);
    frameId.current = requestAnimationFrame(loopFn);
  }, [emuState]);

  // Why do we have initSound here?
  // Because you can't automatically start a sound
  // So we just wait for the user to click upload, at which point we "start" the sound and set the volume to 0
  const initSound = () => {
    const context = controlGain.current.context;
    const o = context.createOscillator();
    o.type = "sin";
    o.frequency.setValueAtTime(440, context.currentTime); // value in hertz
    o.connect(controlGain.current);
    o.start(0);
    sound.current = o;
  };

  useEffect(() => {
    const context = new AudioContext();

    const vG = context.createGain();
    vG.gain.value = DEFAULT_VOLUME;
    vG.connect(context.destination);
    volumeGain.current = vG;

    const cG = context.createGain();
    cG.gain.value = 0.0;
    cG.connect(vG);
    controlGain.current = cG;
  }, []);

  return (
    <div className={styles.container}>
      <canvas id="canvas" width="800" height="400" className={styles.canvas} />

      <Inputs pressInput={pressInput} releaseInput={releaseInput} />

      <div className={styles.buttonContainer}>
        <RomButton loadProgram={loadProgram} />
        <SettingsButton
          fg={fg}
          bg={bg}
          clockSpeed={clockSpeed}
          volume={
            volumeGain.current === null
              ? DEFAULT_VOLUME
              : volumeGain.current.gain.value
          }
          setVolume={(v) =>
            volumeGain.current === null
              ? null
              : (volumeGain.current.gain.value = v)
          }
        />
      </div>
    </div>
  );
}
