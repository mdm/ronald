import type { Component } from "solid-js";
import { createSignal, onMount } from "solid-js";

import init, { Emulator } from "../ronald-wasm/pkg/ronald_wasm";

import { Harness } from "./Harness";

await init();

const App: Component = () => {
  const [harness, setHarness] = createSignal<Harness>();
  const [running, setRunning] = createSignal(false);

  const pause = () => {
    console.log("pause", running(), harness())
    harness()?.pause();
  };

  const run = async () => {
    console.log("run", running(), harness())
    await harness()?.run();
  };

  let canvas: HTMLCanvasElement | undefined;
  onMount(async () => {
    if (!canvas) {
      return;
    }

    setHarness(new Harness(canvas));

    window.addEventListener("blur", () => {
      if (running()) {
        pause();
      }
    });
    window.addEventListener("focus", async () => {
      if (running()) {
        await run();
      }
    });

    document.addEventListener("keydown", (event: KeyboardEvent) =>
      harness()?.handleKeyDown(event)
    );
    document.addEventListener("keyup", (event: KeyboardEvent) =>
      harness()?.handleKeyUp(event)
    );
  });

  return (
    <div style="display:flex">
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
      <button
        onClick={async () => {
          if (running()) {
            setRunning(false);
            pause();
          } else {
            setRunning(true);
            await run();
          }
        }}
      >
        {running() ? "Pause" : "Run"}
      </button>
    </div>
  );
};

export default App;
