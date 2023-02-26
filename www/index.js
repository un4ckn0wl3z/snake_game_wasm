import init, { greeting } from 'snake_game_wasm';

init().then(_ => {
  greeting("Anuwat");
  console.log("ok");
});