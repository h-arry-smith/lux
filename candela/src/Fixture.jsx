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
          <p> {name}: {value["Literal"]["value"]} </p>
        )) }
      </div>
    </div>
  );
}