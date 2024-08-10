import React, { useEffect } from "react";
import classNames from "classnames";
import * as styles from "../App.module.scss";

export default function Inputs({ pressInput, releaseInput }) {
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
  const onInputPress = (i) => {
    document.getElementById(`input-${i}`).classList.add(styles.active);
    pressInput(i);
  };
  const onInputRelease = (i) => {
    document.getElementById(`input-${i}`).classList.remove(styles.active);
    releaseInput(i);
  };
  // Setup stuff
  useEffect(() => {
    document.body.onkeydown = (e) => {
      if (e.repeat) return;
      let i = KEY_MAP.indexOf(e.key);
      if (i != -1) {
        onInputPress(i);
      }
    };
    document.body.onkeyup = (e) => {
      let i = KEY_MAP.indexOf(e.key);
      if (i != -1) {
        onInputRelease(i);
      }
    };
  }, []);
  return (
    <div className={styles.inputs}>
      {[
        0x1, 0x2, 0x3, 0xc, 0x4, 0x5, 0x6, 0xd, 0x7, 0x8, 0x9, 0xe, 0xa, 0x0,
        0xb, 0xf,
      ].map((k) => (
        <React.Fragment key={k}>
          <div className={styles.inputButtonContainer} key={k}>
            <button
              className={classNames(styles.button, styles.inputButton)}
              id={`input-${k}`}
              onMouseDown={() => onInputPress(k)}
              onMouseUp={() => onInputRelease(k)}
              onMouseLeave={() => onInputRelease(k)}
              onTouchStart={() => onInputPress(k)}
              onTouchEnd={() => onInputRelease(k)}
              onTouchCancel={() => onInputRelease(k)}
            >
              <div>{k.toString(16).toUpperCase()}</div>
              <div className={styles.keyMapKey}>{KEY_MAP[k]}</div>
            </button>
          </div>
        </React.Fragment>
      ))}
    </div>
  );
}
