// prettier-ignore
enum Button {
  A      = 0b1000_0000,
  B      = 0b0100_0000,
  Select = 0b0010_0000,
  Start  = 0b0001_0000,
  Up     = 0b0000_1000,
  Down   = 0b0000_0100,
  Left   = 0b0000_0010,
  Right  = 0b0000_0001,
}

const defaultP1 = new Map([
  ["KeyA", Button.A],
  ["KeyZ", Button.B],
  ["KeyQ", Button.A],
  ["KeyW", Button.B],
  ["Space", Button.Select],
  ["Enter", Button.Start],
  ["ArrowUp", Button.Up],
  ["ArrowDown", Button.Down],
  ["ArrowLeft", Button.Left],
  ["ArrowRight", Button.Right],
]);

class Controller {
  private value: number;
  private mappings: Map<string, Button>;

  constructor(mappings: Map<string, Button>) {
    this.value = 0b0000_0000;
    this.mappings = mappings;
  }

  handleKeyEvent(event: KeyboardEvent, state: boolean) {
    const button = this.mappings.get(event.code);

    if (button) {
      this.updateButton(button, state);
    }

    return button;
  }

  updateButton(button: Button, state: boolean) {
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

export { Controller, defaultP1 };
