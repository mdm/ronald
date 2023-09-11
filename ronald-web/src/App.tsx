import type { Component } from "solid-js";
import { createSignal, onMount } from "solid-js";
import { createStore } from "solid-js/store";

import init, { Emulator } from "../ronald-wasm/pkg/ronald_wasm";

import { Harness } from "./Harness";
import ControlButton from "./components/ControlButton";
import CpuStateView from "./components/CpuStateView";

await init();

const App: Component = () => {
  const [harness, setHarness] = createSignal<Harness>();
  const [snapshot, setSnapshot] = createStore<any>(); // TODO: makes this typed
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
          <ControlButton
            onClick={async () => {
              if (running()) {
                setRunning(false);
                pause();
              } else {
                setRunning(true);
                await run();
              }
              setSnapshot(harness()?.getSnapshot());
            }}
          >
            {running() ? "Pause" : "Run"}
          </ControlButton>
          <ControlButton>Single Step</ControlButton>
          <ControlButton>Toggle Breakpoint</ControlButton>
        </div>
        <div>
          <CpuStateView state={snapshot.cpu}/>
        </div>
      </div>
    </div>
  );
};

export default App;
