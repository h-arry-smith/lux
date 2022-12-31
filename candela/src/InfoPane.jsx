import { Timer } from "./Timer";

export function InfoPane() {
  return (
    <div className="flex flex-col h-full w-full">
      <div className="h-full">
        <p>Top</p>
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