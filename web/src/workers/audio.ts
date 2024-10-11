class NesAudioProcessor extends AudioWorkletProcessor {
  private queue: Float32Array[];
  private bufferIndex: number;

  constructor() {
    super();

    this.queue = [];
    this.bufferIndex = 0;

    this.port.onmessage = (event) => {
      if (event.data) {
        this.queue.push(event.data);
      }
    };
  }

  process(_: Float32Array[][], outputs: Float32Array[][]) {
    const output = outputs[0];
    const channel = output[0];

    let buffer = this.queue[0];

    if (!buffer) return true;

    // FIXME: What if the audio goes out of sync with the output?
    for (let i = 0; i < channel.length; i++) {
      channel[i] = buffer[this.bufferIndex];

      if (this.bufferIndex < buffer.length) {
        this.bufferIndex += 1;
      } else {
        this.queue.shift();
        this.bufferIndex = 0;
        buffer = this.queue[0];

        if (!buffer) {
          break;
        }
      }
    }

    return true;
  }
}

registerProcessor("nes-audio-processor", NesAudioProcessor);
