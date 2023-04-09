import type { Component } from "solid-js";
import { createSignal, onMount } from "solid-js";

import init, { Emulator } from "../ronald-wasm/pkg/ronald_wasm";

import { Harness } from "./Harness";

await init();

const App: Component = () => {
  const [harness, setHarness] = createSignal<Harness>();

  let canvas: HTMLCanvasElement | undefined;
  onMount(async () => {
    if (!canvas) {
      return;
    }

    setHarness(new Harness(canvas));

    window.addEventListener("blur", () => {
      harness()?.pause();
    });
    window.addEventListener("focus", async () => {
      await harness()?.run();
    });

    document.addEventListener("keydown", (event: KeyboardEvent) =>
      harness()?.handleKeyDown(event)
    );
    document.addEventListener("keyup", (event: KeyboardEvent) =>
      harness()?.handleKeyUp(event)
    );

    await harness()?.run();
  });

  return (
    <canvas
      ref={canvas}
      onDrop={(event: DragEvent) => {
        harness()?.handleOnDrop(event);
      }}
      onDragOver={(event: DragEvent) => {
        harness()?.handleOnDragOver(event);
      }}
      width="768"
      height="560"
    />
  );
};

export default App;
