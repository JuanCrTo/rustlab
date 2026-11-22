mod entrada;
mod maths;

use core::f32;
use maths::operaciones;
use std::{io, print, println};

fn main() {
    let a: f32 = entrada::ingresar_numero("Ingrese número");
    let b: f32 = entrada::ingresar_numero("Ingrese número");

    let suma: f32 = operaciones::sumar(a, b);
    let resta: f32 = operaciones::restar(a, b);
    let producto: f32 = operaciones::multiplicar(a, b);
    let cociente: Result<f32, String> = operaciones::dividir(a, b);

    println!("Suma: {suma}");
    println!("Resta: {resta}");
    println!("Producto: {producto}");

    match cociente {
        Ok(v) => println!("Cociente: {v}"),
        Err(m) => println!("Error: {m}"),
    }
}
