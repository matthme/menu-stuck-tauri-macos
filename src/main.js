const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;

async function openExternal() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  await invoke("open_window", { });
}


async function unminimizeExternal() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  console.log("Unminimizing external window.");
  await invoke("unminimize_window", { });
}

window.addEventListener("DOMContentLoaded", () => {
  document
    .querySelector("#open-external-button")
    .addEventListener("click", () => openExternal());

  document
    .querySelector("#unminimize-external-button")
    .addEventListener("click", () => unminimizeExternal());
});
