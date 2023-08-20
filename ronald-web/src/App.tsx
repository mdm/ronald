import type { Component } from "solid-js";
import { createSignal, onMount } from "solid-js";

import init, { Emulator } from "../ronald-wasm/pkg/ronald_wasm";

import { Harness } from "./Harness";

await init();

const App: Component = () => {
  const [harness, setHarness] = createSignal<Harness>();
  const [running, setRunning] = createSignal(false);
  let canvas: HTMLCanvasElement | undefined;

  const pause = () => {
    console.log("pause", running(), harness());
    harness()?.pause();
  };

  const run = async () => {
    console.log("run", running(), harness());
    canvas?.focus();
    await harness()?.run();
  };

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
  });

  return (
    <div class="flex bg-slate-900 h-screen">
      <canvas
        ref={canvas}
        width="768"
        height="560"
        tabindex="0"
        onDrop={(event: DragEvent) => {
          harness()?.handleOnDrop(event);
        }}
        onDragOver={(event: DragEvent) => {
          harness()?.handleOnDragOver(event);
        }}
        onKeyDown={(event: KeyboardEvent) => {
          harness()?.handleKeyDown(event);
        }}
        onKeyUp={(event: KeyboardEvent) => {
          harness()?.handleKeyUp(event);
        }}
      />
      <div>
        <div class="flex">
          <button
            class="w-20 h-20 rounded text-slate-900 bg-slate-400 hover:bg-slate-200"
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
      </div>
    </div>
  );
};

export default App;
