import init, { start_main_loop } from "emulator";

import wasmData from "emulator/emulator_bg.wasm";
await init(wasmData);

await start_main_loop();
