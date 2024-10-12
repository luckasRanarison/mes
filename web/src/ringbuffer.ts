class RingBuffer {
  private buffer: Float32Array;
  private writeIndex: number;
  private readIndex: number;
  private capacity: number;
  private size: number;

  constructor(capacity: number) {
    this.buffer = new Float32Array(capacity);
    this.capacity = capacity;
    this.writeIndex = 0;
    this.readIndex = 0;
    this.size = 0;
  }

  enqueue(data: Float32Array) {
    for (let i = 0; i < data.length; i++) {
      if (this.size < this.capacity) {
        this.buffer[this.writeIndex] = data[i];
        this.writeIndex = (this.writeIndex + 1) % this.capacity;
        this.size += 1;
      } else {
        throw new Error("Ring buffer overflow");
      }
    }
  }

  dequeue(channel: Float32Array) {
    for (let i = 0; i < channel.length; i++) {
      if (this.size > 0 && this.readIndex !== this.writeIndex) {
        channel[i] = this.buffer[this.readIndex];
        this.readIndex = (this.readIndex + 1) % this.capacity;
        this.size -= 1;
      } else {
        break;
      }
    }
  }

  clear() {
    this.size = 0;
    this.writeIndex = 0;
    this.readIndex = 0;
  }
}

export default RingBuffer;
