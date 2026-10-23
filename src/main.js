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

function addEmpleado(tableID, nombre, estado, id){
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

    var botonDescanso = document.createElement("button");

    botonVerDetalles.innerHTML = "Ver Detalles";
    botonVerDetalles.setAttribute("class", "ver_detalles");
    botonVerDetalles.addEventListener("click", () => detallesEmpleado(id));


    botonDescanso.innerHTML = "Ver Horario";
    botonDescanso.setAttribute("class", "ver_descanso");
    botonDescanso.addEventListener("click", () => descansoEmpleado(id, nombre));

    nombreEmpleado.appendChild(nombreTexto);

    estadoEmpleado.appendChild(estadoTexto);

    botonDetalles.appendChild(botonVerDetalles);

    botonDetalles.appendChild(botonDescanso);
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
  if (!document.getElementById("empleados")) return;

  const filtro = document.getElementById("filtrar_empleado").value;

  try {
    let filas = [];

    if (filtro === "conectado") {
      const lista = await invoke("logged_in_list");
      filas = lista.map(e => ({
        id: e.id_empleado,
        nombre: e.nombre,
        estado: "En turno desde " + e.hora_inicio.slice(0, 5),
      }));
    } else if (filtro === "ultima_coneccion") {
      const lista = await invoke("logged_out_list");
      lista.sort((a, b) =>
        (b.fecha + (b.hora_salida || "")).localeCompare(a.fecha + (a.hora_salida || ""))
      );
      filas = lista.map(e => ({
        id: e.id_empleado,
        nombre: e.nombre,
        estado: "Salió el " + e.fecha + " a las " + (e.hora_salida || "").slice(0, 5),
      }));
    } else if (filtro === "nombre") {
      const lista = await invoke("list_employees");
      const empleados = lista
        .filter(e => e.rol_empleado === 2)
        .sort((a, b) => a.nombre.localeCompare(b.nombre, "es"));
      filas = empleados.map(e => ({
        id: e.id_empleado,
        nombre: e.nombre,
        estado: jornadaTexto(e.horario_establecido),
      }));
    } else if (filtro === "descanso") {
      const lista = await invoke("on_break_list");
      const empleados = lista
        .filter(e => e.rol_empleado === 2)
        .sort((a, b) => a.nombre.localeCompare(b.nombre, "es"));
      filas = empleados.map(e => ({
        id: e.id_empleado,
        nombre: e.nombre,
        estado: "En descanso",
      }));
    }

    resetTabla("empleados");
    for (let i = filas.length - 1; i >= 0; i--) {
      addEmpleado("empleados", filas[i].nombre, filas[i].estado, filas[i].id);
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


function fechaLocal(d) {
  const mes = String(d.getMonth() + 1).padStart(2, "0");
  const dia = String(d.getDate()).padStart(2, "0");
  return d.getFullYear() + "-" + mes + "-" + dia;
}

function minutosTexto(m) {
  if (m === null || m === undefined) return "-";
  return Math.floor(m / 60) + "h " + (m % 60) + "m";
}

function abrirHistorial() {
  document.getElementById("modalHistorial").showModal();
  cargarHistorial();
}

async function cargarHistorial() {
  const dias = Number(document.getElementById("hist_rango").value);
  const hoy = new Date();
  const desde = new Date();
  desde.setDate(hoy.getDate() - dias);

  try {
    const turnos = await invoke("my_history", {
      from: fechaLocal(desde),
      to: fechaLocal(hoy),
    });
    mostrarTurnos("tablaHistorial", turnos.filter(t => t.hora_salida !== null));
  } catch (err) {
    alert(err);
  }
}

function mostrarTurnos(tableID, turnos) {
  const tabla = document.getElementById(tableID);
  tabla.innerHTML = "";

  const encabezado = tabla.insertRow();
  ["Fecha", "Entrada", "Salida", "Trabajado", "Descanso", "Extra"].forEach(texto => {
    const th = document.createElement("th");
    th.textContent = texto;
    encabezado.appendChild(th);
  });

  if (turnos.length === 0) {
    const celda = tabla.insertRow().insertCell();
    celda.colSpan = 6;
    celda.textContent = "No hay turnos en este rango";
    return;
  }

  turnos.forEach(t => {
    const fila = tabla.insertRow();
    [
      t.fecha,
      t.hora_inicio.slice(0, 5),
      t.hora_salida.slice(0, 5),
      minutosTexto(t.minutos_trabajadas),
      minutosTexto(t.minutos_descanso),
      minutosTexto(t.minutos_extras),
    ].forEach(valor => {
      fila.insertCell().textContent = valor;
    });
  });
}

function cerrarHistorial() {
  document.getElementById("modalHistorial").close();
}


async function detallesEmpleado(id) {
  try {
    const empleados = await invoke("list_employees");
    const e = empleados.find(x => x.id_empleado === id);

    if (!e) {
      alert("Empleado no encontrado");
      return;
    }

    document.getElementById("det_nombre").textContent = e.nombre;
    document.getElementById("det_cedula").textContent = e.cedula;
    document.getElementById("det_jornada").textContent = jornadaTexto(e.horario_establecido);
    document.getElementById("modalDetalles").showModal();
  } catch (err) {
    alert(err);
  }
}

function cerrarDetalles() {
  document.getElementById("modalDetalles").close();
}


// minutes elapsed since a "HH:MM:SS" time of today
function minutosDesde(horaTexto) {
  const [h, m, s] = horaTexto.split(":").map(Number);
  const ahora = new Date();
  const ahoraSeg = ahora.getHours() * 3600 + ahora.getMinutes() * 60 + ahora.getSeconds();
  return Math.max(0, Math.floor((ahoraSeg - (h * 3600 + m * 60 + (s || 0))) / 60));
}

async function descansoEmpleado(id, nombre) {
  try {
    const s = await invoke("employee_status", { idEmpleado: id });

    // finished breaks, plus the one running right now
    let usado = s.descanso_usado;
    if (s.en_descanso && s.descanso_inicio) usado += minutosDesde(s.descanso_inicio);
    const diferencia = s.descanso_permitido - usado;

    let estado;
    if (!s.turno_abierto) estado = "Sin turno abierto";
    else if (s.en_descanso) estado = "En descanso desde " + s.descanso_inicio.slice(0, 5);
    else estado = "Trabajando";

    document.getElementById("desc_nombre").textContent = nombre;
    document.getElementById("desc_estado").textContent = estado;
    document.getElementById("desc_usado").textContent = minutosTexto(usado);
    document.getElementById("desc_permitido").textContent = minutosTexto(s.descanso_permitido);
    document.getElementById("desc_balance").textContent =
      diferencia >= 0
        ? minutosTexto(diferencia) + " restantes"
        : minutosTexto(-diferencia) + " de exceso";

    // break minutes of the last 30 days of finished shifts
    const hoy = new Date();
    const desde = new Date();
    desde.setDate(hoy.getDate() - 30);
    const turnos = await invoke("employee_history", {
      idEmpleado: id,
      from: fechaLocal(desde),
      to: fechaLocal(hoy),
    });
    mostrarTurnos("tablaDescansoHist", turnos.filter(t => t.hora_salida !== null));

    document.getElementById("modalDescanso").showModal();
  } catch (err) {
    alert(err);
  }
}

function cerrarDescanso() {
  document.getElementById("modalDescanso").close();
}