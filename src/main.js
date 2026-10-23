const { invoke } = window.__TAURI__.core;


async function login() {
  const cedula = document.getElementById("in_cedula").value.trim();
  const password = document.getElementById("in_contrasenna").value;

  try {
    const session = await invoke("login", { cedula, password });
    sessionStorage.setItem("session", JSON.stringify(session));

    if (session.rol === "Admin") {
      location.href = "admin.html";
    } else {
      location.href = "empleado.html";
    }
  } catch (err) {
    console.log(err);
    alert(err);
  }
}

async function toggleDescanso() {
  // pink = working, so pressing the button starts a break (same colors as before)
  const iniciaDescanso = getComputedStyle(document.body).backgroundColor === "rgb(97, 64, 81)";

  try {
    await invoke(iniciaDescanso ? "start_break" : "end_break");

    if (iniciaDescanso) {
      document.body.style.backgroundColor = "rgb(64, 97, 64)";
      document.getElementById("btnDescanso").textContent = "Retomar trabajo";
    } else {
      document.body.style.backgroundColor = "rgb(97, 64, 81)";
      document.getElementById("btnDescanso").textContent = "Tomar descanso";
    }
  } catch (err) {
    alert(err);
  }
}

function resetTabla(tableID){
  //limpia el contenido de la tabla que le envies 
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

async function logout() {
  try {
    await invoke("logout");
    sessionStorage.removeItem("session");
    location.href = "login.html";
  } catch (err) {
    alert(err); // e.g. "Debe terminar el descanso antes de salir"
  }
}

function crearEmpleado(){
  //Envia a la pantalla de crear empleado
  location.href = "signup.html";
}

//crea un nuevo usuario
async function crearCuenta() {
  const nombre = document.getElementById("in_nombre").value.trim();
  const cedula = document.getElementById("in_cedula").value.trim();
  const contrasena = document.getElementById("in_contrasenna").value;
  const confirmar = document.getElementById("in_contrasenna_again").value;

  if (contrasena !== confirmar) {
    alert("Las contraseñas no coinciden");
    return;
  }

  try {
    await invoke("register_employee", {
      data: {
        rol_empleado: Number(document.getElementById("rol").value),
        nombre,
        cedula,
        contrasenna: contrasena,
        horario_establecido: Number(document.getElementById("jornada").value),
      },
    });
    alert("Empleado creado");
    location.href = "admin.html";
  } catch (err) {
    alert(err);
  }
}

