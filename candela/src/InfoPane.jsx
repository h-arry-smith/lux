import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from "@tauri-apps/api/window"
import { Timer } from "./Timer";
import { Fixture } from "./Fixture";

export function InfoPane() {
  let [resolvedFixtures, setResolvedFixtures] = useState({});
  
  useEffect(() => {
    invoke("init_tick").then(() => console.log("starting tick loop"));
  }, [])

  useEffect(() => {
    const unlisten  = appWindow.listen("tick", (event) => {
      invoke("resolve").then((resolved) => setResolvedFixtures(resolved))
    })

    return () => {
      unlisten.then(f => f());
    }
  }, [] );

  return (
    <div className="flex flex-col h-full w-full">
      <div className="flex-1">
        { resolvedFixtures && Object.entries(resolvedFixtures).map(([id, parameters]) => {
          return (
            <Fixture
              key={id}
              id={id}
              parameters={parameters["parameters"]} />
          )    
        }) 
      }
      </div>

      <div className="h-1/5 flex flex-col">
        <div className="w-full py-2 px-4 bg-gray-700 flex justify-center items-center">
          <h2 className="text-lg uppercase font-semibold">
            Timer
          </h2>
        </div>
        <div className="w-full bg-gray-800 flex-1 flex flex-col">
          <Timer />
        </div>
      </div>
    </div>
  );
}