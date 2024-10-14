class AudioRingBuffer {
  private buffer: Float32Array;
  private writeIndex: number;
  private readIndex: number;
  private capacity: number;

  constructor(capacity: number) {
    this.buffer = new Float32Array(capacity);
    this.capacity = capacity;
    this.writeIndex = 0;
    this.readIndex = 0;
  }

  enqueue(samples: Float32Array) {
    if (this.writeLength() < samples.length) {
      throw new Error("Ring buffer overflow");
    }

    for (let i = 0; i < samples.length; i++) {
      this.buffer[this.writeIndex] = samples[i];
      this.writeIndex = (this.writeIndex + 1) % this.capacity;
    }
  }

  dequeue(channel: Float32Array) {
    const length = Math.min(channel.length, this.readLength());

    for (let i = 0; i < length; i++) {
      channel[i] = this.buffer[this.readIndex];
      this.readIndex = (this.readIndex + 1) % this.capacity;
    }
  }

  readLength() {
    return (this.writeIndex - this.readIndex + this.capacity) % this.capacity;
  }

  writeLength() {
    return this.capacity - this.readLength();
  }

  clear() {
    this.writeIndex = 0;
    this.readIndex = 0;
  }
}

export default AudioRingBuffer;
