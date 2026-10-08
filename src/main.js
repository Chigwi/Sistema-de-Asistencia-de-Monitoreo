const { invoke } = window.__TAURI__.core;

let greetInputEl;
let greetMsgEl;

let session;
function login(){
    const username = document.getElementById("in_cedula").value;
    const password = document.getElementById("in_contrasenna").value;

    if (username.length > 0){
        location.href = "empleado.html"; 
    }else{
        console.log("ingresa un username para la prueba");
    }
}

function toggleDescanso(){
  if(getComputedStyle(document.body).backgroundColor === "rgb(97, 64, 81)"){
    document.body.style.backgroundColor = "rgb(64, 97, 64)"; 
    document.getElementById("btnDescanso").textContent = "retomar trabajo"
  }else{
    document.body.style.backgroundColor = "rgb(97, 64, 81)";
    document.getElementById("btnDescanso").textContent = "tomar descanso"

  }
}

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});
