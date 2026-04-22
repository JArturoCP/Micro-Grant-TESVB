# 🚚 FleetChain Solana - Explicación del Código

Este programa está desarrollado en **Rust** utilizando el framework **Anchor** para la blockchain de **Solana**. Su objetivo es gestionar el historial de mantenimientos de una flotilla de vehículos mediante operaciones tipo **CRUD** (Crear, Leer, Actualizar y Eliminar).

---

## 🔑 1. Configuración Inicial

use anchor_lang::prelude::*;

// ID del programa
declare_id!("2nZDmTBcKALtWFbiHqME2nP5THcagqddtrJKVmBuANTY");

* Se importa la librería principal de Anchor.
* `declare_id!` define el identificador único del programa en la red de Solana.

---

## ⚙️ 2. Módulo del Programa

#[program]
pub mod fleetchain_solana {

Aquí se definen todas las funciones públicas del smart contract.

---

## 🟢 3. Operaciones CRUD

### 3.1 CREATE - Inicializar Flotilla

pub fn inicializar_flotilla(ctx: Context<CrearFlotilla>, nombre_empresa: String) -> Result<()>

* Crea una cuenta en la blockchain que representa la flotilla.
* Guarda:

  * El propietario (`owner`)
  * El nombre de la empresa
  * Un arreglo vacío de servicios

👉 Usa un **PDA (Program Derived Address)** para generar una dirección única.

---

### 3.2 CREATE - Registrar Mantenimiento

pub fn registrar_mantenimiento(...)

* Agrega un nuevo registro de mantenimiento.
* Valida que quien ejecuta la función sea el dueño:

require!(gestor.owner == ctx.accounts.owner.key(), Errores::NoEresElOwner);

* Inserta un nuevo objeto `Mantenimiento` en el vector.

---

### 3.3 UPDATE - Editar Mantenimiento

pub fn editar_mantenimiento(...)

* Busca un mantenimiento por la **placa del vehículo**.
* Si lo encuentra:

  * Actualiza tipo, kilometraje y costo.
* Si no:

  * Lanza error `VehiculoNoEncontrado`.

---

### 3.4 DELETE - Eliminar Mantenimiento

pub fn eliminar_mantenimiento(...)

* Busca el índice del mantenimiento.
* Si existe:

  * Lo elimina con `remove()`.
* Si no:

  * Retorna error.

---

### 3.5 READ - Ver Historial

pub fn ver_historial(...)

* Muestra en consola:

  * Nombre de la empresa
  * Lista completa de mantenimientos

---

## 📦 4. Estructuras de Datos

### 🔧 Mantenimiento

pub struct Mantenimiento {
pub placa_vehiculo: String,
pub tipo_servicio: String,
pub kilometraje_actual: u32,
pub costo_servicio: u32,
}

Representa un registro individual de mantenimiento.

📌 Incluye restricciones:

* `placa_vehiculo`: máximo 10 caracteres
* `tipo_servicio`: máximo 30 caracteres

---

### 🏢 GestorFlotilla

pub struct GestorFlotilla {
pub owner: Pubkey,
pub nombre_empresa: String,
pub servicios: Vec<Mantenimiento>,
}

* Es la cuenta principal almacenada en la blockchain.
* Contiene:

  * Propietario
  * Nombre de empresa
  * Lista de mantenimientos

📌 Límite:

* Máximo 10 servicios en el vector

---

## 🔐 5. Contextos (Accounts)

### CrearFlotilla

#[derive(Accounts)]
pub struct CrearFlotilla<'info>

* Define las cuentas necesarias para crear la flotilla:

  * `owner`: quien paga y firma
  * `gestor`: cuenta que se inicializa
  * `system_program`: requerido por Solana

📌 Usa:

* `seeds` → para generar el PDA
* `bump` → evitar colisiones

---

### GestionarFlotilla

#[derive(Accounts)]
pub struct GestionarFlotilla<'info>

* Se usa para todas las operaciones CRUD.
* Requiere:

  * `owner` (firmante)
  * `gestor` (cuenta mutable)

---

## ⚠️ 6. Manejo de Errores

#[error_code]
pub enum Errores {

Define errores personalizados:

* `NoEresElOwner` → cuando alguien no autorizado intenta modificar datos
* `VehiculoNoEncontrado` → cuando no existe un registro con esa placa

---

## 🧠 Conclusión

Este programa implementa un sistema completo de gestión de mantenimientos en blockchain usando:

* 🔑 Control de acceso mediante `owner`
* 📦 Almacenamiento estructurado en cuentas de Solana
* ⚡ Uso de PDAs para direcciones determinísticas
* 🔄 Operaciones CRUD sobre un vector interno

Es un ejemplo claro de cómo llevar lógica de negocio tradicional (gestión de flotillas) a un entorno descentralizado, garantizando **integridad, transparencia y seguridad** en los datos.
