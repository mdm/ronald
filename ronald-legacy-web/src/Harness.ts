import { Emulator } from "../ronald-wasm/pkg/ronald_wasm";
import defaultKeymap from "./assets/default-keymap.json";

const TARGET_FRAME_MS = 20;

export class Harness {
  private emulator: Emulator;
  private running: boolean;
  private previousTime: number;
  private timeAvailable: number;
  private keymap: any;
  private pressedKeys: Set<string>;

  constructor(canvas: HTMLCanvasElement) {
    this.emulator = new Emulator(canvas);
    this.running = false;
    this.previousTime = performance.now();
    this.timeAvailable = 0;
    this.keymap = defaultKeymap;
    this.pressedKeys = new Set();
  }

  async run() {
    console.log("run");

    this.previousTime = performance.now();
    this.timeAvailable = 0;
    this.running = true;
    this.emulator.play_audio();

    requestAnimationFrame(this.step.bind(this));
  }

  pause() {
    console.log("pause");

    for (const pressedKey of this.pressedKeys) {
      for (const key of this.keymap[pressedKey]) {
        this.emulator.release_key(key);
      }
    }

    this.emulator.pause_audio();
    this.running = false;
  }

  stepSingle() {
    this.emulator.step_single();
  }

  getSnapshot() {
    if (this.running) {
      return undefined;
    }

    // TODO: deserialze this properly
    const snapshot = JSON.parse(this.emulator.get_snapshot());
    console.log(snapshot);
    return snapshot;
  }

  getDisassembly() {
    if (this.running) {
      return undefined;
    }

    // TODO: deserialze this properly
    return JSON.parse(this.emulator.disassemble(20));
  }

  handleKeyDown(event: KeyboardEvent) {
    if (!this.running) {
      return;
    }

    if (this.keymap[event.key]) {
      this.pressedKeys.add(event.key);

      for (const key of this.keymap[event.key]) {
        this.emulator.press_key(key);
      }
    }
  }

  handleKeyUp(event: KeyboardEvent) {
    if (!this.running) {
      return;
    }

    if (this.keymap[event.key]) {
      this.pressedKeys.delete(event.key);

      for (const key of this.keymap[event.key]) {
        this.emulator.release_key(key);
      }
    }
  }

  async handleOnDrop(event: DragEvent) {
    console.log(event);
    event.preventDefault();

    let file;
    if (event.dataTransfer?.items) {
      if (event.dataTransfer.items[0].kind === "file") {
        file = event.dataTransfer.items[0].getAsFile();
      }
    } else {
      file = event.dataTransfer?.files[0];
    }

    if (file) {
      console.log(file.name, file.size);
      const buffer = await file.arrayBuffer();
      this.emulator.load_disk(0, buffer, file.name);
    }
  }

  handleOnDragOver(event: DragEvent) {
    event.preventDefault();
  }

  private async setupKeymap() {
    const keymapURL = "src/assets/default-keymap.json";
    const request = new Request(keymapURL);
    const response = await fetch(request);
    this.keymap = await response.json();
  }

  private step(time: DOMHighResTimeStamp) {
    this.timeAvailable += time - this.previousTime;
    this.previousTime = time;

    while (this.timeAvailable >= TARGET_FRAME_MS) {
      this.emulator.step_driver();
      this.timeAvailable -= TARGET_FRAME_MS;
    }

    if (this.running) {
      requestAnimationFrame(this.step.bind(this));
    }
  }
}
