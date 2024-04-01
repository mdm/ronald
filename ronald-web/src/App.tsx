import type { Component } from "solid-js";
import { createSignal, onMount } from "solid-js";
import { createStore } from "solid-js/store";

import init, { Emulator } from "../ronald-wasm/pkg/ronald_wasm";

import { Harness } from "./Harness";
import ControlButton from "./components/ControlButton";
import CpuStateView from "./components/devtools/CpuStateView";
import DevTools from "./components/devtools";

await init();

const App: Component = () => {
  const [harness, setHarness] = createSignal<Harness>();
  const [running, setRunning] = createSignal(false);
  const [snapshot, setSnapshot] = createSignal<any>();
  const [disassembly, setDisassembly] = createSignal<any>();
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
    setSnapshot(harness()!.getSnapshot());
    setDisassembly(harness()!.getDisassembly());

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
                pause();
                setSnapshot(harness()!.getSnapshot());
                setDisassembly(harness()!.getDisassembly());
                setRunning(false);
              } else {
                await run();
                setRunning(true);
              }
            }}
          >
            {running() ? "Pause" : "Run"}
          </ControlButton>
          <ControlButton
            onClick={() => {
              harness()?.stepSingle();
              setSnapshot(harness()!.getSnapshot());
              setDisassembly(harness()!.getDisassembly());
            }}
          >
            Single Step
          </ControlButton>
          <ControlButton>Toggle Breakpoint</ControlButton>
        </div>
        <div>
          <DevTools
            active={!running()}
            snapshot={snapshot()}
            disassembly={disassembly()}
          />
        </div>
      </div>
    </div>
  );
};

export default App;
