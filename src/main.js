const { invoke } = window.__TAURI__.core;

let greetInputEl;
let greetMsgEl;

let session;
function login(){
  //inicia sesion con las credenciales y envia a una pantalla dependiendo de su roll
    const username = document.getElementById("in_cedula").value;
    const password = document.getElementById("in_contrasenna").value;

    if (username == "employer"){
        location.href = "empleado.html"; 
    }else if (username == "admin"){
        location.href = "admin.html";
    }else{
      console.log("ingresa un username para la prueba");
    }
}

function toggleDescanso(){
  //Inicia o cancela descanso
  if(getComputedStyle(document.body).backgroundColor === "rgb(97, 64, 81)"){
    document.body.style.backgroundColor = "rgb(64, 97, 64)"; 
    document.getElementById("btnDescanso").textContent = "Retomar trabajo"
  }else{
    document.body.style.backgroundColor = "rgb(97, 64, 81)";
    document.getElementById("btnDescanso").textContent = "Tomar descanso"

  }
}

function resetTabla(tableID){
  var tablaRef = document.getElementById(tableID);

  tablaRef.innerHTML = "";
}

function addEmpleado(tableID, nombre, estado){
  //Inserta los empleados dentro de la pantalla del empleado
  //consigue la tabla de empleado
    var tableRef = document.getElementById(tableID);
  //
    var newRow = tableRef.insertRow(0);

    var nombreEmpleado = newRow.insertCell(0);

    var estadoEmpleado = newRow.insertCell(1);

    var botonDetalles = newRow.insertCell(2);

    var nombreTexto = document.createTextNode(nombre);

    var estadoTexto = document.createTextNode(estado);

    var botonVerDetalles = document.createElement("button");

    botonVerDetalles.innerHTML = "Ver Detalles";
    botonVerDetalles.setAttribute("class", "ver_detalles");
    botonVerDetalles.setAttribute("onclick", "detallesEmpleado()");

    nombreEmpleado.appendChild(nombreTexto);

    estadoEmpleado.appendChild(estadoTexto);

    botonDetalles.appendChild(botonVerDetalles);
}

function logout(){
  //Cierra sesion
  location.href = "login.html";
}

function crearEmpleado(){
  //Envia a la pantalla de crear empleado
  location.href = "signup.html";
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
