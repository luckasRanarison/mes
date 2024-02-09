type ButtonMap = Record<string, ControllerButton | null>;

enum ControllerButton {
  A = 0b1000_0000,
  B = 0b0100_0000,
  Select = 0b0010_0000,
  Start = 0b0001_0000,
  Up = 0b0000_1000,
  Down = 0b0000_0100,
  Left = 0b0000_0010,
  Right = 0b0000_0001,
}

const defaultMapping = {
  KeyA: ControllerButton.A,
  KeyZ: ControllerButton.B,
  KeyQ: ControllerButton.A,
  KeyW: ControllerButton.B,
  Space: ControllerButton.Select,
  Enter: ControllerButton.Start,
  ArrowUp: ControllerButton.Up,
  ArrowDown: ControllerButton.Down,
  ArrowLeft: ControllerButton.Left,
  ArrowRight: ControllerButton.Right,
};

class ControllerState {
  value: number;
  mappings: ButtonMap;

  constructor() {
    this.value = 0;
    this.mappings = defaultMapping;
  }

  update(key: string, state: boolean) {
    const button = this.mappings[key];

    if (button) {
      this.updateButton(button, state);
    }
  }

  updateButton(button: ControllerButton, state: boolean) {
    if (state) {
      this.value |= button;
    } else {
      this.value &= ~button;
    }
  }

  state() {
    return this.value;
  }
}

export { ControllerState, ControllerButton, defaultMapping };
