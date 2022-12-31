import "./App.css"
import { CodePane } from "./CodePane"
import { InfoPane } from "./InfoPane";

function App() {

  return (
    <div className="flex w-screen h-screen overflow-none">
      <div className="w-3/5 border-r border-gray-400">
        <CodePane />
      </div>

      <div className="w-2/5">
        <InfoPane />
      </div>
    </div>
  );
}

export default App;
