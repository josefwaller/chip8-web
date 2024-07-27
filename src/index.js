import init, { add } from "emulator";
import wasmData from "emulator/emulator_bg.wasm";

await init(wasmData);
console.log(add);
let i = add(1, 2);
alert("Hello world!" + i);
