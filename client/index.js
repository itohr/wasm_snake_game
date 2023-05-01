import init, { greet } from "snake_game";

init().then((_) => {
  greet("Bob");
  console.log("OK");
});
