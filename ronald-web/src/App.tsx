import type { Component } from 'solid-js';
import { createSignal, onMount } from 'solid-js';
import init, { Emulator } from '../ronald-wasm/pkg/ronald_wasm';

import logo from './logo.svg';
import styles from './App.module.css';

const keymapURL = '/src/assets/default-keymap.json';
const request = new Request(keymapURL);
const response = await fetch(request);
const keymap = await response.json();

await init();
const TARGET_FRAME_MS = 20;

const run = (emulator: Emulator | null) => {
  if (!emulator) {
    return;
  }

  let previousTime = performance.now();
  let timeAvailable = 0;

  const step = (time: DOMHighResTimeStamp) => {
    timeAvailable += time - previousTime;
    previousTime = time;

    while (timeAvailable >= TARGET_FRAME_MS) {
      emulator.step_driver();
      timeAvailable -= TARGET_FRAME_MS;
    }

    requestAnimationFrame(step);
  };

  requestAnimationFrame(step);
};

const handleOnDrop = async (emulator: Emulator | null, event: DragEvent) => {
  console.log(event);
  event.preventDefault();

  let file;
  if (event.dataTransfer?.items) {
      if (event.dataTransfer.items[0].kind === 'file') {
        file = event.dataTransfer.items[0].getAsFile();
      }
  } else {
    file = event.dataTransfer?.files[0];
  }

  if (file && emulator) {
    console.log(file.name, file.size); 
    const buffer = await file.arrayBuffer();
    emulator.load_disk(0, buffer);
  }
}

const handleOnDragOver = (event: DragEvent) => {
  event.preventDefault();
}

const App: Component = () => {
  const [emulator, setEmulator] = createSignal<Emulator | null>(null);

  let canvas: HTMLCanvasElement | undefined;
  onMount(() => {
    setEmulator(new Emulator(canvas));

    document.addEventListener('keydown', ((emulator: Emulator | null, event: KeyboardEvent) => {
      if (emulator && keymap[event.key]) {
        for (const key of keymap[event.key]) {
          emulator.press_key(key);    
        }
      }
    }).bind(null, emulator()));
    document.addEventListener('keyup', ((emulator: Emulator | null, event: KeyboardEvent) => {
      if (emulator && keymap[event.key]) {
        for (const key of keymap[event.key]) {
          emulator.release_key(key);
        }
      }
    }).bind(null, emulator()));
    run(emulator());
  });

  return (
    <canvas
      ref={canvas}
      onDrop={[handleOnDrop, emulator()]}
      onDragOver={handleOnDragOver}
      width="768"
      height="560"
    />
  );
};

export default App;
