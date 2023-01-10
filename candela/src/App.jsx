import "./App.css"
import { CodePane } from "./CodePane"
import { InfoPane } from "./InfoPane";
import { StatusBar } from "./StatusBar";

function App() {

  return (
    <div className="flex flex-col w-screen h-screen overflow-none">
      <StatusBar />
      <div className="flex h-[98vh]">
        <div className="w-3/5 border-r border-gray-400">
          <CodePane />
        </div>

        <div className="w-2/5">
          <InfoPane />
        </div>
      </div>
    </div>
  );
}

export default App;
