const { invoke } = window.__TAURI__.tauri;
const { writeTextFile, BaseDirectory } = window.__TAURI__.fs;
//import { writeTextFile, BaseDirectory } from '@tauri-apps/api/fs';

let greetInputEl;
let greetMsgEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
  // Write a text file to the `$APPCONFIG/app.conf` path
  await writeTextFile('testing_noats', 'Hello ' +  greetInputEl.value, { dir: BaseDirectory.AppData });
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document
    .querySelector("#greet-button")
    .addEventListener("click", () => greet());
});
