import { Emulator } from "../ronald-wasm/pkg/ronald_wasm";

const TARGET_FRAME_MS = 20;

export class Harness {
  private emulator: Emulator;
  private running: boolean;
  private previousTime: number;
  private timeAvailable: number;
  private keymap?: any;

  constructor(canvas: HTMLCanvasElement) {
    this.emulator = new Emulator(canvas);
    this.running = false;
    this.previousTime = performance.now();
    this.timeAvailable = 0;
  }

  async run() {
    if (!this.keymap) {
      await this.setupKeymap();
    }

    this.previousTime = performance.now();
    this.timeAvailable = 0;
    this.running = true;

    requestAnimationFrame(this.step.bind(this));
  }

  pause() {
    this.running = false;
  }

  handleKeyDown(event: KeyboardEvent) {
    if (this.keymap[event.key]) {
      for (const key of this.keymap[event.key]) {
        this.emulator.press_key(key);
      }
    }
  }

  handleKeyUp(event: KeyboardEvent) {
    if (this.keymap[event.key]) {
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
      this.emulator.load_disk(0, buffer);
    }
  }

  handleOnDragOver(event: DragEvent) {
    event.preventDefault();
  }

  private async setupKeymap() {
    const keymapURL = "/src/assets/default-keymap.json";
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
