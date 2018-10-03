use std::io;
use std::mem;

struct Room<'a> {
	puerta: Option<Box<&'a Room <'a>>>,	
	descripcion: &'a str,
}
impl <'a> Room <'a> {
	fn to_string(&self) {
		println!("descripcion: {:?}", self.descripcion);
	}
}
struct Dungeon<'a> {
	habitaciones: Vec<&'a Room<'a>>,
}
fn introduccion() {
	println!("Bienvenido al juego de rol de texto experimental.");
	println!("De ahora en adelanta, el motor grafico será tu imaginacion, 
		asi que acomódate y preparate para imaginar.");
}
fn introduccion_turno(accion: &u8, turno: &u8) {
	println!("Acciones restantes este turno = {}, Turno = {}",accion, turno );
	println!("0-Avanzar norte.");
	println!("1-Avanzar sur.");
	println!("2-Avanzar este.");
	println!("3-Avanzar oeste.");
	println!("4-Inspeccionar.");
	println!("5-Pasar turno.");
}
fn main() {
	let mut _description_room1: [&str; 9];
	let mut respuesta = String::new();
	let mut acciones: u8 = 4; //numero de acciones maximas en un turno.
	let mut turno: u8 = 0;
	let habitacion0 = Room{
			descripcion: "Habitacion demo 0.",
			puerta: None,
		};
	let habitacion1 = Room{
			descripcion: "Habitacion demo 1.",
			puerta: Some(Box::new(&habitacion0)),
		};
	let mut mazzmorra = Dungeon{habitaciones: Vec::new()};
	mazzmorra.habitaciones.push(&habitacion0);
	mazzmorra.habitaciones.push(&habitacion1);
	introduccion();
	println!("Tu primera prueba ha sido generada, estas preparado? (Y/N)");
	println!("Se ha generado 1 Dungeon con habitaciones.");
	loop {
		acciones = 4;
		loop {
			let mut accion = String::new();
			if acciones == 0 {
				break;
			}
			introduccion_turno(&acciones, &turno);
			io::stdin().read_line(&mut accion).expect("Respuesta incorrecta.");
			if !mazzmorra.habitaciones.is_empty() {
				mazzmorra.habitaciones.pop().unwrap().to_string();
			}
			acciones -= 1;
		}
		if turno == 4 {
			break;
		}
		turno += 1;
	}
}