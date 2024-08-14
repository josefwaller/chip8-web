import React, { useEffect, useRef } from "react";

import classNames from "classnames";
import * as styles from "../App.module.scss";
const BAD_ROM_URL = require("../../bad_rom.ch8");
const SPLASH_ROM_URL = require("../../splash_screen.ch8");

export default function RomButton({ loadProgram }) {
  const badRom = useRef(new Uint8Array());
  const splashRom = useRef(new Uint8Array());

  const onFileChoose = (event) => {
    const reader = new FileReader();
    reader.onload = (e) => {
      const program = new Uint8Array(e.target.result);
      if (program.length > 4096) {
        loadProgram(badRom.current);
      } else {
        loadProgram(program);
      }
    };
    reader.readAsArrayBuffer(event.target.files[0]);
  };

  // Fetch the BAD ROM rom
  useEffect(() => {
    loadRom(BAD_ROM_URL, badRom, () => {});
    loadRom(SPLASH_ROM_URL, splashRom, () =>
      loadProgram(splashRom.current, true)
    );
  }, []);

  const loadRom = (url, romRef, callback) => {
    fetch(url).then((res) => {
      if (res.ok) {
        const reader = res.body.getReader();
        const readFn = ({ value, done }) => {
          if (!done) {
            const mergedArray = new Uint8Array(
              romRef.current.length + value.length
            );
            mergedArray.set(romRef.current);
            mergedArray.set(value, romRef.length);
            romRef.current = mergedArray;
            return reader.read().then(readFn);
          } else {
            callback();
          }
        };
        reader.read().then(readFn);
      }
    });
  };

  return (
    <>
      <button
        onClick={() => document.getElementById("rom-input").click()}
        className={classNames(styles.button, styles.romButton)}
      >
        Upload ROM
      </button>
      <input
        id="rom-input"
        type="file"
        style={{ display: "none" }}
        className={styles.romButton}
        onChange={onFileChoose}
      />
    </>
  );
}
