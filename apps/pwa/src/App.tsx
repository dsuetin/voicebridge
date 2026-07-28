import { useRef, useState } from "react";
import { Microphone } from "./audio/microphone";
import { RingBuffer } from "./audio/ring-buffer";
import "./index.css";

import init, {
  WakeWordEngine,
} from "@voicebridge/wakeword";


function App() {

  console.log("APP VERSION WITH WASM");

  const [mic, setMic] =
    useState(false);

  const engineRef =
    useRef<WakeWordEngine | null>(null);


  const microphoneRef =
    useRef<Microphone | null>(null);


  const ringBufferRef =
    useRef<RingBuffer | null>(null);



  const startMic = async () => {

    console.log(
      "Loading WASM..."
    );
    console.log("START");

    await init();
    console.log("WASM INIT OK");

    const engine =
      new WakeWordEngine();
    console.log(
      "ENGINE CREATED",
      engine
    );

    engineRef.current =
      engine;


    console.log(
      "WASM initialized"
    );


    const microphone =
      new Microphone();


    microphoneRef.current =
      microphone;


    const ringBuffer =
      new RingBuffer(
        16000 * 5
      );


    ringBufferRef.current =
      ringBuffer;



    await microphone.start(
      (data) => {


        //
        // сохраняем последние 5 секунд
        //
        ringBuffer.push(data);



        //
        // отправляем в Rust WASM
        //
        const score =
          engine.process(data);



        console.log(
          "score:",
          score,
          "buffer:",
          ringBuffer.getSeconds(16000)
        );


        // console.log(
        //   "processed:",
        //   engine.processed_seconds(),
        //   "sec"
        // );

      }
    );


    setMic(true);
  };



  return (
    <div className="container">

      <h1>
        🎙 VoiceBridge
      </h1>


      <div className="status">

        {mic
          ? "🎤 Listening"
          : "🟢 Ready"}

      </div>



      <button
        onClick={startMic}
        disabled={mic}
      >
        Start microphone
      </button>



      <div className="info">

        <p>
          Microphone:
          {mic ? " ✅" : " ❌"}
        </p>


        <p>
          Wake word:
          ⏳ waiting
        </p>


        <p>
          Buffer:
          0 / 5 sec
        </p>


        <p>
          WebSocket:
          disconnected
        </p>

      </div>

    </div>
  );
}


export default App;