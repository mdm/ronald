import type { Component } from 'solid-js';
import { onMount } from 'solid-js';
import init, { run_cpc } from '../ronald-wasm/pkg/ronald_wasm';

import logo from './logo.svg';
import styles from './App.module.css';

await init();

const App: Component = () => {
  let canvas: HTMLCanvasElement | undefined;
  onMount(() => {
    run_cpc(canvas);
  });

  return (
    <canvas ref={canvas} width="768" height="560" />
  );
};

export default App;
