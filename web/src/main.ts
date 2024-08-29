import "./index.css";

import ErrorSvg from "./assets/error.svg";
import Emulator from "./emulator";

const romInput = document.getElementById("input-rom") as HTMLInputElement;
const canvasElement = document.getElementById("canvas") as HTMLCanvasElement;
const canvasContainer = document.getElementById("canvas-container")!;
const dragDropElement = document.getElementById("drag-drop-area")!;
const stopButton = document.getElementById("button-stop")!;
const fullscreenButton = document.getElementById("button-fullscreen")!;

const emulator = new Emulator(canvasElement);

async function handleUpload(file?: File | null) {
  if (!file) return;

  const buffer = await file.arrayBuffer();
  const bytes = new Uint8Array(buffer);

  try {
    emulator.setCartridge(bytes);
    dragDropElement.classList.toggle("hidden");
    canvasContainer.classList.toggle("hidden");
  } catch (error: any) {
    showErrorPopup(error.toString().replace("Error: ", ""));
  }
}

function setupControllerHandlers() {
  window.onkeydown = (event) => {
    emulator.handleKeyEvent(event, true);
  };

  window.onkeyup = (event) => {
    emulator.handleKeyEvent(event, false);
  };

  document.querySelectorAll(".button").forEach((element) => {
    const value = element.getAttribute("value")!;
    const buttonValue = parseInt(value, 2);
    const htmlElement = element as HTMLElement;

    htmlElement.ontouchstart = (event) => {
      event.preventDefault();
      emulator.updateController(0, buttonValue, true);
    };

    htmlElement.ontouchend = (event) => {
      event.preventDefault();
      emulator.updateController(0, buttonValue, false);
    };
  });
}

function setupDragAndDrop() {
  dragDropElement.ondragover = (event) => {
    event.preventDefault();
    dragDropElement.classList.add("bg-smoke");
  };

  dragDropElement.ondragleave = () => {
    dragDropElement.classList.remove("bg-smoke");
  };

  dragDropElement.ondrop = (event) => {
    event.preventDefault();
    dragDropElement.classList.remove("bg-smoke");
    handleUpload(event.dataTransfer?.files.item(0));
  };

  romInput.onchange = () => {
    handleUpload(romInput.files?.item(0));
  };
}

function setupEmulatorButtons() {
  stopButton.onclick = () => {
    emulator.stop();
    canvasContainer.classList.toggle("flex");
    canvasContainer.classList.toggle("hidden");
    dragDropElement.classList.toggle("hidden");
  };

  fullscreenButton.onclick = () => {
    canvasElement.requestFullscreen();
  };
}

function showErrorPopup(message: string) {
  const popupContainer = document.createElement("div");

  popupContainer.className =
    "z-50 inset-0 fixed flex h-screen w-screen items-center justify-center bg-smoke p-4";

  popupContainer.innerHTML = `
    <div
      class="flex min-w-72 max-w-96 flex-col items-center rounded-md bg-white px-8 py-6"
    >
      <img src=${ErrorSvg} alt="error" width="50" class="mb-2" />
      <div class="mb-4 text-xl font-semibold text-primary">Error</div>
      <div class="mb-6 text-center">${message}</div>
      <button class="rounded-md bg-primary px-5 py-2 text-white">Close</button>
    </div>
  `;

  popupContainer.onclick = () => document.body.removeChild(popupContainer);

  document.body.append(popupContainer);
}

setupDragAndDrop();
setupControllerHandlers();
setupEmulatorButtons();
