import { appWindow } from "@tauri-apps/api/window";
import { useState, useEffect } from "react";

export function StatusBar() {
  let [connectionState, setConnectionState] = useState("disconnected");

  // TODO: Can this be a hook that makes these much easier to register
  useEffect(() => {
    const unlistenConnected = appWindow.listen("network/connected", (_e) => {
      setConnectionState("connected")
    });

    const unlistenDisconnected = appWindow.listen("network/disconnected", (_e) => {
      setConnectionState("disconnected")
    });

    return () => {
      unlistenConnected.then(f => f());
      unlistenDisconnected.then(f => f());
    }
  }, [])
  
  return (
      <div className="flex px-2 space-x-2 justify-end bg-gray-900 border-b border-gray-400 items-center font-mono text-gray-600 text-sm">
        <ConnectionStatus state={connectionState} />
      </div>
  );
}

function ConnectionStatus({state}) {
  const COLORS = {
    "disconnected": "bg-red-500",
    "connected": "bg-green-500",
  }
  
  return (
    <div className="flex items-center">
      <p>
        connection:&nbsp;
      </p>
      <div className={`h-3 w-3 bg-red-500 rounded-full animate-pulse ${COLORS[state]}`}></div>
    </div>
  );
}