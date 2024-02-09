import { Nes } from "mes";
import { memory } from "mes/mes_bg.wasm";
import { ControllerState } from "./controller";

class EmulatorState {
  private instance: Nes;
  private frameBuffer: Uint8ClampedArray;
  private active: boolean;
  private canvas: CanvasRenderingContext2D;
  public controller: ControllerState;

  constructor(context: CanvasRenderingContext2D) {
    this.instance = new Nes();
    this.frameBuffer = new Uint8ClampedArray();
    this.controller = new ControllerState();
    this.active = false;
    this.canvas = context;
  }

  setCartridge(bytes: Uint8Array) {
    this.instance.setCartridge(bytes);
    this.instance.reset();
    this.active = true;
    this.updateFrameBuffer();
    this.loop();
  }

  stop() {
    this.active = false;
  }

  private loop() {
    if (this.active) {
      this.instance.stepFrame();
      this.instance.stepVblank();
      this.instance.setControllerState(0, this.controller.value);
      this.canvas.putImageData(new ImageData(this.frameBuffer, 256, 240), 0, 0);

      requestAnimationFrame(() => this.loop());
    }
  }

  private updateFrameBuffer() {
    const bufStart = this.instance.getFrameBufferPtr();
    const bufSize = 256 * 240 * 4;
    this.frameBuffer = new Uint8ClampedArray(memory.buffer, bufStart, bufSize);
  }
}

export { EmulatorState };
