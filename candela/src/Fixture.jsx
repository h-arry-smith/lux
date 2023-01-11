export function Fixture({id, parameters}) {
  return (
    <div className="bg-gray-800 border border-gray-700 m-1">
      <div className="bg-gray-700 flex justify-center items-center">
        <h2>
          {id}
        </h2>
      </div>
      <div className="flex flex-col items-center p-2"> 
        { Object.entries(parameters).map(([name, value]) => (
          <p> {name}: {pretty_value(value)} </p>
        )) }
      </div>
    </div>
  );
}

// TODO: Gross!
function pretty_value(value) {
  if ("Literal" in value) {
    return value["Literal"]["value"].toFixed(2);
  }

  if ("Percentage" in value) {
    return value["Percentage"]["percentage"].toFixed(2);
  }
}