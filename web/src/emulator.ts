import { Nes } from "mes";
import { memory } from "mes/mes_bg.wasm";
import { Controller, defaultP1 } from "./controller";

class Emulator {
  private instance: Nes;
  private frameBuffer: Uint8ClampedArray;
  private active: boolean;
  private canvas: CanvasRenderingContext2D;
  private controllers: Controller[];
  private frameDuration = 1000 / 60;
  private lastTimestamp = 0;
  private audioCtx: AudioContext;
  private audioWorklet?: AudioWorkletNode;

  constructor(canvas: HTMLCanvasElement) {
    this.instance = new Nes();
    this.frameBuffer = new Uint8ClampedArray();
    this.controllers = [new Controller(defaultP1)];
    this.active = false;
    this.canvas = canvas.getContext("2d")!;
    this.audioCtx = new AudioContext({ sampleRate: 44100 });
    this.initAudioWorklet();
  }

  async initAudioWorklet() {
    await this.audioCtx.audioWorklet.addModule("audio-processor.js");

    this.audioWorklet = new AudioWorkletNode(
      this.audioCtx,
      "nes-audio-processor",
    );

    this.audioWorklet.connect(this.audioCtx.destination);
  }

  setCartridge(bytes: Uint8Array) {
    this.instance.setCartridge(bytes);
    this.instance.reset();
    this.audioCtx.resume();
    this.active = true;
    this.updateFrameBuffer();

    requestAnimationFrame((timestamp) => this.loop(timestamp));
  }

  handleKeyEvent(event: KeyboardEvent, state: boolean) {
    for (let i = 0; i < this.controllers.length; i++) {
      if (this.controllers[i].handleKeyEvent(event, state)) break;
    }
  }

  updateController(id: number, button: number, state: boolean) {
    this.controllers[id].updateButton(button, state);
  }

  stop() {
    this.active = false;
    this.audioCtx.suspend();
  }

  private updateControllers() {
    for (let i = 0; i < this.controllers.length; i++) {
      const state = this.controllers[i].state();
      this.instance.setControllerState(i, state);
    }
  }

  private draw() {
    const imageData = new ImageData(this.frameBuffer, 256, 240);
    this.canvas.putImageData(imageData, 0, 0);
  }

  private loop(timestamp: number) {
    if (!this.active) return;

    const deltaTime = timestamp - this.lastTimestamp;

    if (deltaTime >= this.frameDuration) {
      this.lastTimestamp = timestamp - (deltaTime % this.frameDuration);

      this.instance.stepFrame();
      this.instance.stepVblank();

      const rawAudio = this.instance.drainAudioBuffer();

      if (this.audioWorklet) this.audioWorklet.port.postMessage(rawAudio);

      this.updateControllers();
      this.draw();
    }

    requestAnimationFrame((timestamp) => this.loop(timestamp));
  }

  private updateFrameBuffer() {
    const bufStart = this.instance.getFrameBufferPtr();
    const bufSize = 256 * 240 * 4;
    this.frameBuffer = new Uint8ClampedArray(memory.buffer, bufStart, bufSize);
  }
}

export default Emulator;
