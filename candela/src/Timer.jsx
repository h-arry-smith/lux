import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api"

import { PlayIcon, PauseIcon, StopIcon } from "@heroicons/react/24/outline";

export function Timer() {
  let [timerStatus, setTimerStatus] = useState("stopped");
  let [time, setTime] = useState("00:00:00:00");

  let COLORS = {
    "running": "text-green-500",
    "paused": "text-yellow-500",
    "stopped": "text-gray-500",
  }

  useEffect(() => {
    let interval = setInterval(() => getCurrentTime(), 50);

    return () => {
      clearInterval(interval);
    };
  }, [time])

  function getCurrentTime() {
    invoke("get_current_time").then((t) => setTime(t));
  }

  function startTime() {
    invoke("start_time").then((t) => {
      setTime(t);
      setTimerStatus("running");
    })
  }

  function pauseTime() {
    invoke("pause_time").then((t) => {
      setTime(t);
      setTimerStatus("paused");
    })
  }

  function stopTime() {
    invoke("stop_time").then((t) => {
      setTime(t);
      setTimerStatus("stopped");
    })
  }
  
  return (
    <div class="flex h-full">
      <div className="flex flex-col flex-0">
        <div className="flex h-1/2 w-24 mt-1 ml-1 pb-1">
          <button 
            onClick={startTime}
            className="w-1/2 flex justify-center items-center h-full border rounded bg-gray-600 hover:border-yellow-500 hover:text-yellow-500">
            <PlayIcon className="w-6 h-6" />
          </button>
          <button 
            onClick={pauseTime}
            className="w-1/2 flex justify-center items-center h-full border rounded bg-gray-600 ml-1 hover:border-yellow-500 hover:text-yellow-500">
            <PauseIcon className="w-6 h-6" />
          </button>
        </div>
        <div className="flex h-1/2 ml-1 pb-1">
          <button 
            onClick={stopTime}
            className="w-1/2 flex justify-center items-center h-full border rounded bg-gray-600 hover:border-yellow-500 hover:text-yellow-500">
            <StopIcon className="w-6 h-6" />
          </button>
          <button className="w-1/2 flex justify-center items-center h-full border rounded bg-gray-600 ml-1 hover:border-yellow-500">
            D
          </button>
        </div>
      </div>

      <div className="flex justify-center flex-1 items-center m-1 bg-black rounded border p-2">
        <p className={`font-mono text-5xl font-bold ${COLORS[timerStatus]}`}>
          {time}
        </p>
    </div>
  </div>
  )
}