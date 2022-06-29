const smithy = require(".");

const createHandlers = async () => {
  // init goes here
  return {
    "my operation": () => {
     // Operation handling goes here
    }
  };
};

smithy.createServer(createHandlers);
