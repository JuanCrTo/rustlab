use std::io;

pub fn ingresar_numero(s: &str) -> f32 {
    println!("{s}");

    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la liena");

    let numero: f32 = input
        .trim()
        .parse()
        .expect("Ingrese un número valido");

    numero
}
