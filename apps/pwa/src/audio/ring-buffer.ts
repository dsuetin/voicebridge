export class RingBuffer {
  private buffer: Float32Array;
  private writeIndex = 0;
  private size = 0;
  private capacity: number;

  constructor(
    capacity: number
  ) {
    this.capacity = capacity;
    this.buffer = new Float32Array(capacity);
  }


  push(data: Float32Array) {

    for (let i = 0; i < data.length; i++) {

      this.buffer[this.writeIndex] = data[i];

      this.writeIndex =
        (this.writeIndex + 1) % this.capacity;


      if (this.size < this.capacity) {
        this.size++;
      }
    }
  }


  getSize(): number {
    return this.size;
  }


  getSeconds(sampleRate: number): number {
    return this.size / sampleRate;
  }


  getLastSamples(count: number): Float32Array {

    const result =
      new Float32Array(
        Math.min(count, this.size)
      );


    const start =
      (this.writeIndex - result.length + this.capacity)
      % this.capacity;


    for (let i = 0; i < result.length; i++) {

      result[i] =
        this.buffer[
          (start + i) % this.capacity
        ];
    }


    return result;
  }


  clear() {
    this.writeIndex = 0;
    this.size = 0;
  }
}