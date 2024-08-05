import React, { useEffect, useRef } from "react";

import classNames from "classnames";
import * as styles from "../App.module.scss";
const BAD_ROM_URL = require("../../bad_rom.rom");

export default function RomButton({ loadProgram }) {
  const badRom = useRef(new Uint8Array());
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
    fetch(BAD_ROM_URL).then((res) => {
      if (res.ok) {
        const reader = res.body.getReader();
        const readFn = ({ value, done }) => {
          if (!done) {
            const mergedArray = new Uint8Array(
              badRom.current.length + value.length
            );
            mergedArray.set(badRom.current);
            mergedArray.set(value, badRom.length);
            badRom.current = mergedArray;
            return reader.read().then(readFn);
          }
        };
        reader.read().then(readFn);
      }
    });
  }, []);

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
