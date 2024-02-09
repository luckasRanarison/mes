import "./index.css";
import { EmulatorState } from "./emulator";
import { ControllerButton } from "./controller";

const canvas = document.getElementById("canvas") as HTMLCanvasElement;
const context = canvas.getContext("2d")!;
const canvasContainer = document.getElementById("canvas-container")!;
const dragDropArea = document.getElementById("drag-drop-area")!;
const inputRom = document.getElementById("input-rom") as HTMLInputElement;
const stopButton = document.getElementById("button-stop")!;
const fullscreenButton = document.getElementById("button-fullscreen")!;
const keyUp = document.getElementById("key-up")!;
const keyDown = document.getElementById("key-down")!;
const keyLeft = document.getElementById("key-left")!;
const keyRight = document.getElementById("key-right")!;
const keySelect = document.getElementById("key-select")!;
const keyStart = document.getElementById("key-start")!;
const keyB = document.getElementById("key-b")!;
const keyA = document.getElementById("key-a")!;
const emulator = new EmulatorState(context);

async function handleUpload(file?: File | null) {
  if (file) {
    const buffer = await file.arrayBuffer();
    const bytes = new Uint8Array(buffer);

    try {
      emulator.setCartridge(bytes);
      canvasContainer.classList.toggle("hidden");
      dragDropArea.classList.toggle("hidden");
    } catch (error) {
      console.log(error);
    }
  }
}

function handleDragOver(event: DragEvent) {
  event.preventDefault();
  dragDropArea.classList.add("bg-smoke");
}

function handleDragLeave() {
  dragDropArea.classList.remove("bg-smoke");
}

function handleDrop(event: DragEvent) {
  event.preventDefault();
  dragDropArea.classList.remove("bg-smoke");
  handleUpload(event.dataTransfer?.files.item(0));
}

function handleInputChange() {
  handleUpload(inputRom.files?.item(0));
}

function handleStop() {
  emulator.stop();
  canvasContainer.classList.toggle("hidden");
  dragDropArea.classList.toggle("hidden");
}

function handleFullScreen() {
  canvas.requestFullscreen();
}

function handleTouchStart(elemnt: HTMLElement, button: ControllerButton) {
  elemnt.addEventListener("touchstart", (e) => {
    e.preventDefault();
    emulator.controller.updateButton(button, true);
  });
}

function handleTouchEnd(elemnt: HTMLElement, button: ControllerButton) {
  elemnt.addEventListener("touchend", (e) => {
    e.preventDefault();
    emulator.controller.updateButton(button, false);
  });
}

dragDropArea.addEventListener("dragover", handleDragOver);
dragDropArea.addEventListener("dragleave", handleDragLeave);
dragDropArea.addEventListener("drop", handleDrop);
inputRom.addEventListener("change", handleInputChange);
stopButton.addEventListener("click", handleStop);
fullscreenButton.addEventListener("click", handleFullScreen);

window.addEventListener("keydown", (e) =>
  emulator.controller.update(e.code, true)
);
window.addEventListener("keyup", (e) =>
  emulator.controller.update(e.code, false)
);

handleTouchStart(keyUp, ControllerButton.Up);
handleTouchStart(keyDown, ControllerButton.Down);
handleTouchStart(keyLeft, ControllerButton.Left);
handleTouchStart(keyRight, ControllerButton.Right);
handleTouchStart(keySelect, ControllerButton.Select);
handleTouchStart(keyStart, ControllerButton.Start);
handleTouchStart(keyB, ControllerButton.B);
handleTouchStart(keyA, ControllerButton.A);

handleTouchEnd(keyUp, ControllerButton.Up);
handleTouchEnd(keyDown, ControllerButton.Down);
handleTouchEnd(keyLeft, ControllerButton.Left);
handleTouchEnd(keyRight, ControllerButton.Right);
handleTouchEnd(keySelect, ControllerButton.Select);
handleTouchEnd(keyStart, ControllerButton.Start);
handleTouchEnd(keyB, ControllerButton.B);
handleTouchEnd(keyA, ControllerButton.A);
