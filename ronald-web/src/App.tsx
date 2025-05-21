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
            title="Play/Pause"
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
            {running() ? (
              <svg
                xmlns="http://www.w3.org/2000/svg"
                height="24px"
                viewBox="0 -960 960 960"
                width="24px"
              >
                <path d="M520-200v-560h240v560H520Zm-320 0v-560h240v560H200Zm400-80h80v-400h-80v400Zm-320 0h80v-400h-80v400Zm0-400v400-400Zm320 0v400-400Z" />
              </svg>
            ) : (
              <svg
                xmlns="http://www.w3.org/2000/svg"
                height="24px"
                viewBox="0 -960 960 960"
                width="24px"
              >
                <path d="M320-200v-560l440 280-440 280Zm80-280Zm0 134 210-134-210-134v268Z" />
              </svg>
            )}
          </ControlButton>
          <ControlButton
            title="Step"
            onClick={() => {
              harness()?.stepSingle();
              setSnapshot(harness()!.getSnapshot());
              setDisassembly(harness()!.getDisassembly());
            }}
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              height="24px"
              viewBox="0 -960 960 960"
              width="24px"
            >
              <path d="M480-80q-50 0-85-35t-35-85q0-50 35-85t85-35q50 0 85 35t35 85q0 50-35 85t-85 35Zm0-320L280-600l56-56 104 103v-327h80v327l103-103 57 56-200 200Z" />
            </svg>{" "}
          </ControlButton>
          <ControlButton title="Step over">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              height="24px"
              viewBox="0 -960 960 960"
              width="24px"
            >
              <path d="M480-200q-50 0-85-35t-35-85q0-50 35-85t85-35q50 0 85 35t35 85q0 50-35 85t-85 35ZM163-480q14-119 104-199.5T479-760q73 0 135 29.5T720-650v-110h80v280H520v-80h168q-32-54-86.5-87T480-680q-88 0-155 57t-81 143h-81Z" />
            </svg>{" "}
          </ControlButton>
          <ControlButton title="Step out">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              height="24px"
              viewBox="0 -960 960 960"
              width="24px"
            >
              <path d="M480-80q-50 0-85-35t-35-85q0-50 35-85t85-35q50 0 85 35t35 85q0 50-35 85t-85 35Zm-40-320v-327L336-624l-56-56 200-200 200 200-57 56-103-103v327h-80Z" />
            </svg>{" "}
          </ControlButton>
          <ControlButton title="Add breakpoint">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              height="24px"
              viewBox="0 -960 960 960"
              width="24px"
            >
              <path d="M440-42v-80q-125-14-214.5-103.5T122-440H42v-80h80q14-125 103.5-214.5T440-838v-80h80v80q125 14 214.5 103.5T838-520h80v80h-80q-14 125-103.5 214.5T520-122v80h-80Zm40-158q116 0 198-82t82-198q0-116-82-198t-198-82q-116 0-198 82t-82 198q0 116 82 198t198 82Zm0-120q-66 0-113-47t-47-113q0-66 47-113t113-47q66 0 113 47t47 113q0 66-47 113t-113 47Zm0-80q33 0 56.5-23.5T560-480q0-33-23.5-56.5T480-560q-33 0-56.5 23.5T400-480q0 33 23.5 56.5T480-400Zm0-80Z" />
            </svg>{" "}
          </ControlButton>
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
