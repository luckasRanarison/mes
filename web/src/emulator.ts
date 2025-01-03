import { Nes } from "mes";
import { Controller } from "./controller";
import audioWorkletUrl from "./workers/audio?worker&url";

class Emulator {
  private instance: Nes;
  private active: boolean;
  private canvas: CanvasRenderingContext2D;
  private controllers: Controller[];
  private frameDuration = 1000 / 60;
  private lastTimestamp = 0;
  private audio?: AudioContext;
  private audioWorklet?: AudioWorkletNode;

  constructor(canvas: HTMLCanvasElement) {
    this.instance = new Nes();
    this.controllers = [Controller.playerOne()];
    this.canvas = canvas.getContext("2d")!;
    this.active = false;
  }

  setCartridge(bytes: Uint8Array) {
    this.instance.setCartridge(bytes);
    this.instance.reset();
    this.active = true;

    requestAnimationFrame((timestamp) => this.loop(timestamp));
  }

  async initAudio() {
    if (!this.audio) {
      const ctx = new AudioContext({ sampleRate: 44100 });
      await ctx.audioWorklet.addModule(audioWorkletUrl);
      this.audioWorklet = new AudioWorkletNode(ctx, "nes-audio-processor");
      this.audioWorklet.connect(ctx.destination);
      this.audio = ctx;
    }

    this.audio.resume();
    this.audioWorklet?.port.postMessage({ reset: true });
  }

  handleKeyEvent(event: KeyboardEvent, state: boolean) {
    for (const controller of this.controllers) {
      if (controller.handleKeyEvent(event, state)) {
        break;
      }
    }
  }

  updateController(id: number, button: number, state: boolean) {
    this.controllers[id].updateButton(button, state);
  }

  stop() {
    this.active = false;
    this.audio?.suspend();
  }

  private updateControllers() {
    for (let i = 0; i < this.controllers.length; i++) {
      const state = this.controllers[i].state();
      this.instance.setControllerState(i, state);
    }
  }

  private draw() {
    const imageData = this.instance.updateImageData();
    this.canvas.putImageData(imageData, 0, 0);
  }

  private loop(timestamp: number) {
    if (!this.active) return;

    const deltaTime = timestamp - this.lastTimestamp;

    if (deltaTime >= this.frameDuration) {
      this.lastTimestamp = timestamp - (deltaTime % this.frameDuration);

      this.instance.stepFrame();

      if (this.audioWorklet) {
        const samples = this.instance.getAudioBuffer();
        this.audioWorklet.port.postMessage({ samples });
        this.instance.clearAudioBuffer();
      }

      this.updateControllers();
      this.draw();

      this.instance.stepVblank();
    }

    requestAnimationFrame((timestamp) => this.loop(timestamp));
  }
}

export default Emulator;
