import RingBuffer from "../ringbuffer";

class NesAudioProcessor extends AudioWorkletProcessor {
  private buffer: RingBuffer;

  constructor() {
    super();

    this.buffer = new RingBuffer(4096);

    this.port.onmessage = ({ data }) => {
      if (data) {
        this.buffer.enqueue(data);
      }
    };
  }

  process(_: Float32Array[][], outputs: Float32Array[][]) {
    const channel = outputs[0][0];

    this.buffer.dequeue(channel);

    return true;
  }
}

registerProcessor("nes-audio-processor", NesAudioProcessor);
