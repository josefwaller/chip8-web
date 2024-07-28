import React, { useEffect, useState } from "react";

import * as styles from "./App.module.scss";

export default function App() {
  const [worker, setWorker] = useState(null);
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
    if (worker) worker.postMessage({ type: "set_inputs", value: inputStates });
  }, [inputStates]);

  window.onload = () => {
    console.log(document);
    const worker = new Worker(new URL("./runner.js", import.meta.url));
    console.log("Loaded worker");
    worker.postMessage("Posting here");
    setWorker(worker);
  };
  return (
    <div className={styles.container}>
      <canvas id="canvas" className={styles.canvas} />
      <div className={styles.inputs}>
        {[...Array(16).keys()].map((k) => (
          <React.Fragment key={k}>
            {k % 3 == 0 && <br />}
            <div className={styles.buttonContainer} key={k}>
              <button
                className={styles.button}
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
      <input type="file" />
    </div>
  );
}
