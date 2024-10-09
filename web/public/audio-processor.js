class NesAudioProcessor extends AudioWorkletProcessor {
  constructor() {
    super();

    this.buffer = new Float32Array(1470);

    this.port.onmessage = (event) => {
      if (event.data) this.buffer.set(event.data);
    };
  }

  process(_, outputs) {
    const output = outputs[0];
    const outputChannel = output[0];

    for (let i = 0; i < this.buffer.length; i++) {
      // TODO: Queue samples
      // What if the audio goes out of sync with the video?
      // outputChannel[i] = this.buffer[i];
    }

    return true;
  }
}

registerProcessor("nes-audio-processor", NesAudioProcessor);
