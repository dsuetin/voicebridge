import { useRef, useState } from "react";
import { Microphone } from "./audio/microphone";
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


const startMic = async () => {

    await init();


    const engine =
        new WakeWordEngine();


    engineRef.current = engine;


    const microphone =
        new Microphone();


    await microphone.start(
        (data) => {

            const score =
                engine.process(data);


            console.log(
                "rms:",
                engine.rms().toFixed(4),
                "speech:",
                engine.speech_detected(),
                "buffer:",
                engine.buffer_seconds().toFixed(2)
            );


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