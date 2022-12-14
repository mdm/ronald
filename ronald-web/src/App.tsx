import type { Component } from 'solid-js';
import { onMount } from 'solid-js';
import init, { Emulator } from '../ronald-wasm/pkg/ronald_wasm';

import logo from './logo.svg';
import styles from './App.module.css';

const keymapURL = '/src/assets/default-keymap.json';
const request = new Request(keymapURL);
const response = await fetch(request);
const keymap = await response.json();

await init();
const TARGET_FRAME_MS = 20;

const step = (emulator: Emulator) => {
  const start = performance.now();
  emulator.step_driver();
  const elapsed = performance.now() - start;
  setTimeout(step.bind(null, emulator), Math.max(0, TARGET_FRAME_MS - elapsed));
};

const App: Component = () => {
  let canvas: HTMLCanvasElement | undefined;
  onMount(() => {
    const emulator = new Emulator(canvas);
    document.addEventListener('keydown', ((emulator: Emulator, event: KeyboardEvent) => {
      if (keymap[event.key]) {
        for (const key of keymap[event.key]) {
          emulator.press_key(key);    
        }
      }
    }).bind(null, emulator));
    document.addEventListener('keyup', ((emulator: Emulator, event: KeyboardEvent) => {
      if (keymap[event.key]) {
        for (const key of keymap[event.key]) {
          emulator.release_key(key);
        }
      }
    }).bind(null, emulator));
    step(emulator);
  });

  return (
    <canvas ref={canvas} width="768" height="560" />
  );
};

export default App;
