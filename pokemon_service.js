const smithy = require(".");

const createHandlers = async () => {
  const input = smithy.GetPokemonSpeciesInput("Shit")
  console.log(input)
  // init goes here
  return {
    "my operation": () => {
     // Operation handling goes here
    }
  };
};

smithy.createServer(createHandlers);
