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

function mostrarNombre() {
  const label = document.getElementById("nombre_usuario");
  if (!label) return; // login.html and signup.html don't have this label

  const session = JSON.parse(sessionStorage.getItem("session") || "null");
  if (!session) {
    location.href = "login.html"; // nobody logged in
    return;
  }
  label.textContent = session.nombre;
}

window.addEventListener("DOMContentLoaded", mostrarNombre);


function jornadaTexto(id) {
  if (id === 1) return "Diurna";
  if (id === 2) return "Nocturna";
  return "Sin horario";
}

async function cargarEmpleados() {
  if (!document.getElementById("empleados")) return; // only admin.html has the table

  const filtro = document.getElementById("filtrar_empleado").value;

  try {
    let filas = [];

    if (filtro === "conectado") {
      const lista = await invoke("logged_in_list");
      filas = lista.map(e => ({
        nombre: e.nombre,
        estado: "En turno desde " + e.hora_inicio.slice(0, 5),
      }));
    } else if (filtro === "ultima_coneccion") {
      const lista = await invoke("logged_out_list");
      lista.sort((a, b) =>
        (b.fecha + (b.hora_salida || "")).localeCompare(a.fecha + (a.hora_salida || ""))
      );
      filas = lista.map(e => ({
        nombre: e.nombre,
        estado: "Salió el " + e.fecha + " a las " + (e.hora_salida || "").slice(0, 5),
      }));
    } else if (filtro === "nombre") {
      const lista = await invoke("list_employees");
      const empleados = lista
        .filter(e => e.rol_empleado === 2)
        .sort((a, b) => a.nombre.localeCompare(b.nombre, "es"));
      filas = empleados.map(e => ({
        nombre: e.nombre,
        estado: jornadaTexto(e.horario_establecido),
      }));
    } else if (filtro === "descanso") {
      const lista = await invoke("on_break_list");
      const empleados = lista
        .filter(e => e.rol_empleado === 2)
        .sort((a, b) => a.nombre.localeCompare(b.nombre, "es"));
      filas = empleados.map(e => ({
        nombre: e.nombre,
        estado: "En descanso",
      }));
    }

    resetTabla("empleados");
    // addEmpleado inserts each row at the top, so go backwards to keep the order
    for (let i = filas.length - 1; i >= 0; i--) {
      addEmpleado("empleados", filas[i].nombre, filas[i].estado);
    }
  } catch (err) {
    alert(err);
  }
}

window.addEventListener("DOMContentLoaded", () => {
  const filtro = document.getElementById("filtrar_empleado");
  if (filtro) {
    filtro.addEventListener("change", cargarEmpleados);
    cargarEmpleados();
  }
});