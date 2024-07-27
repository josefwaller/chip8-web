import React from 'react';

import * as styles from './App.module.scss';

import init, { add } from "emulator";
import wasmData from "emulator/emulator_bg.wasm";

await init(wasmData);

export default function App() {
    let i = add(1, 2);

    return <h1 className={styles.text}>hello world {i}</h1>
}