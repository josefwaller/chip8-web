import init, { init_state, loop_forever } from "emulator";

import wasmData from "emulator/emulator_bg.wasm";
await init(wasmData);

let state = await init_state();

onmessage = function (e) {
  if (e.data?.type == "set_inputs") {
    const inputs = [...Array(16)].map((_, i) => (e.data?.value[i] ? 1 : 0));
    state.set_inputs(inputs);
  }
  state = loop_forever(state);
};
