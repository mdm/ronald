import type { Component } from 'solid-js';
import { onMount } from 'solid-js';
import init, { Emulator } from '../ronald-wasm/pkg/ronald_wasm';

import logo from './logo.svg';
import styles from './App.module.css';

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
      emulator.press_key(event.key);
    }).bind(null, emulator));
    document.addEventListener('keyup', ((emulator: Emulator, event: KeyboardEvent) => {
      emulator.release_key(event.key);
    }).bind(null, emulator));
    step(emulator);
  });

  return (
    <canvas ref={canvas} width="768" height="560" />
  );
};

export default App;
