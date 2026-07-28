export class Microphone {

  private context?: AudioContext;
  private stream?: MediaStream;


  async start(
    callback: (data: Float32Array) => void
  ) {

    this.stream =
      await navigator.mediaDevices.getUserMedia({
        audio: {
          channelCount: 1,
          echoCancellation: false,
          noiseSuppression: false,
          autoGainControl: false,
        }
      });


    this.context =
      new AudioContext({
        sampleRate: 16000,
      });


    await this.context.audioWorklet.addModule(
      new URL("./processor.js", import.meta.url)
    );


    const source =
      this.context.createMediaStreamSource(
        this.stream
      );


    const processor =
      new AudioWorkletNode(
        this.context,
        "pcm-processor"
      );


    processor.port.onmessage =
      (event) => {
        callback(event.data);
      };


    source.connect(processor);

    processor.connect(
      this.context.destination
    );
  }


  stop() {

    this.stream?.getTracks()
      .forEach(t => t.stop());

    this.context?.close();

  }
}