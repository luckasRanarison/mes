import RingBuffer from "../ringbuffer";

class NesAudioProcessor extends AudioWorkletProcessor {
  private buffer: RingBuffer;

  constructor() {
    super();

    this.buffer = new RingBuffer(8192);

    this.port.onmessage = ({ data }) => {
      if (data.reset) this.buffer.clear();
      if (data.samples) this.buffer.enqueue(data.samples);
    };
  }

  process(_: Float32Array[][], outputs: Float32Array[][]) {
    const channel = outputs[0][0];

    this.buffer.dequeue(channel);

    return true;
  }
}

registerProcessor("nes-audio-processor", NesAudioProcessor);
