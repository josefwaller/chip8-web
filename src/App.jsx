import React, { useEffect, useRef, useState } from "react";

import init, { setup } from "emulator";
import { HexColorPicker } from "react-colorful";
import { Slider } from "rsuite";
import Popup from "reactjs-popup";
import classNames from "classnames";

import wasmData from "emulator/emulator_bg.wasm";
await init(wasmData);

import * as styles from "./App.module.scss";
import "rsuite/Slider/styles/index.css";

export default function App() {
  const inputStates = useRef([...Array(16)].map((_, i) => false));
  const lastKeyUp = useRef(null);
  const fg = useRef("#bc4d47");
  const bg = useRef("#340e0b");
  const clockSpeed = useRef(500);
  const [emuState, setEmuState] = useState(null);
  const frameId = useRef(null);
  const pressInput = (i) => {
    inputStates.current.splice(i, 1, true);
    document.getElementById(`input-${i}`).classList.add(styles.active);
    updateColors();
  };
  const releaseInput = (i) => {
    if (inputStates.current[i]) {
      inputStates.current.splice(i, 1, false);
      lastKeyUp.current = i;
      document.getElementById(`input-${i}`).classList.remove(styles.active);
      updateColors();
    }
  };
  const KEY_MAP = [
    "x",
    "1",
    "2",
    "3",
    "q",
    "w",
    "e",
    "a",
    "s",
    "d",
    "z",
    "c",
    "4",
    "r",
    "f",
    "v",
  ];

  // Setup stuff
  useEffect(() => {
    document.body.onkeydown = (e) => {
      if (e.repeat) return;
      let i = KEY_MAP.indexOf(e.key);
      if (i != -1) {
        pressInput(i);
      }
    };
    document.body.onkeyup = (e) => {
      let i = KEY_MAP.indexOf(e.key);
      if (i != -1) {
        releaseInput(i);
      }
    };
    document.getElementById("rom-input").addEventListener("change", (event) => {
      const reader = new FileReader();
      reader.onload = (e) => {
        setEmuState(setup(new Uint8Array(e.target.result)));
      };
      reader.readAsArrayBuffer(event.target.files[0]);
    });
    updateColors();
  }, []);

  const loopFn = () => {
    emuState.update(inputStates.current, clockSpeed.current, lastKeyUp.current);
    emuState.render(fg.current, bg.current);
    lastKeyUp.current = null;
    frameId.current = requestAnimationFrame(loopFn);
  };

  useEffect(() => {
    if (emuState == null) return;
    if (frameId.current) cancelAnimationFrame(frameId.current);
    frameId.current = requestAnimationFrame(loopFn);
  }, [emuState]);

  const updateColors = () => {
    document.documentElement.style.cssText = `--fg: ${fg.current}; --bg: ${bg.current}; --rs-slider-hover-bar: ${fg.current};`;
  };

  return (
    <div className={styles.container}>
      <canvas id="canvas" width="800" height="400" className={styles.canvas} />
      <div className={styles.inputs}>
        {[
          0x1, 0x2, 0x3, 0xc, 0x4, 0x5, 0x6, 0xd, 0x7, 0x8, 0x9, 0xe, 0xa, 0x0,
          0xb, 0xf,
        ].map((k) => (
          <React.Fragment key={k}>
            <div className={styles.inputButtonContainer} key={k}>
              <button
                className={classNames(styles.button, styles.inputButton, {
                  [styles.active]: inputStates.current[k],
                })}
                id={`input-${k}`}
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
      <input
        id="rom-input"
        type="file"
        style={{ display: "none" }}
        className={styles.romButton}
      />
      <div className={styles.buttonContainer}>
        <button
          id="rom-button"
          onClick={() => document.getElementById("rom-input").click()}
          className={classNames(styles.button, styles.romButton)}
        >
          Upload ROM
        </button>
        <Popup
          trigger={<button className={styles.button}>Settings</button>}
          className="settingsPopup"
          modal
          nested
        >
          <div className={styles.settings}>
            <span className={styles.settingsTitle}>Settings</span>
            <div className={styles.colorSettings}>
              <div>
                Foreground color
                <HexColorPicker
                  color={fg.current}
                  onChange={(c) => {
                    fg.current = c;
                    updateColors();
                  }}
                />
              </div>
              <div>
                Background colour
                <HexColorPicker
                  color={bg.current}
                  onChange={(c) => {
                    bg.current = c;
                    updateColors();
                  }}
                />
              </div>
            </div>
            <div className={styles.clockSpeedSettings}>
              <div>Clock Speed</div>
              <div>
                <Slider
                  min={10}
                  max={2000}
                  defaultValue={clockSpeed.current}
                  onChange={(v) => (clockSpeed.current = v)}
                  barClassName={styles.bar}
                  handleClassName={styles.handle}
                />
              </div>
            </div>
          </div>
        </Popup>
      </div>
    </div>
  );
}
