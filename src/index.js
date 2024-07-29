import init, { start_main_loop } from "emulator";

import wasmData from "emulator/emulator_bg.wasm";
await init(wasmData);

document.getElementById("rom-input").addEventListener("change", (event) => {
  const reader = new FileReader();
  reader.onload = (e) => {
    console.log(e);
    console.log(e.target.result);
    start_main_loop(new Uint8Array(e.target.result));
  };
  reader.readAsArrayBuffer(event.target.files[0]);
});
