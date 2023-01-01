import { useState } from "react";
import { CodeEditor } from "./CodeEditor";

export function CodePane() {
  let [consoleText, setConsoleText] = useState("temp console");

  return (
    <div className="flex flex-col h-screen w-full">
      <div className="h-2/3">
        <CodeEditor setConsoleText={setConsoleText} />
      </div>
      <div className="h-1/3 m-1 overflow-scroll border text-sm font-gray-200 border-1 border-white p-1 font-mono bg-black">
        <pre>{ consoleText }</pre>
      </div>
    </div>
  );
}