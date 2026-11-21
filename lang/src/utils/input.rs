use std::io;

/// Lee un número i32 desde la terminal mostrando un mensaje.
pub fn ingresar_numero(s: &str) -> i32 {
    println!("{}:", s);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la línea");
    input
        .trim()
        .parse()
        .expect("Por favor, ingrese un número válido")
}
