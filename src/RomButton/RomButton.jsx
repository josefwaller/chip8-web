import React from "react";

import classNames from "classnames";
import * as styles from "../App.module.scss";

export default function RomButton({ loadProgram }) {
  const onFileChoose = (event) => {
    const reader = new FileReader();
    reader.onload = (e) => {
      loadProgram(new Uint8Array(e.target.result));
    };
    reader.readAsArrayBuffer(event.target.files[0]);
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
