use crate::utils::input::ingresar_numero;

pub fn run() {
    let a = ingresar_numero("Ingrese el primer número");
    let b = ingresar_numero("Ingrese el segundo número");

    println!("Suma: {}", sumar(a, b));
    println!("Resta: {}", restar(a, b));
    println!("Multiplicación: {}", multiplicar(a, b));

    match dividir(a, b) {
        Ok(resultado) => println!("División: {}", resultado),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn sumar(a: i32, b: i32) -> i32 {
    a + b
}

pub fn restar(a: i32, b: i32) -> i32 {
    a - b
}

pub fn multiplicar(a: i32, b: i32) -> i32 {
    a * b
}

pub fn dividir(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err(String::from("Cannot divide by zero"));
    }
    Ok(a / b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operaciones() {
        assert_eq!(sumar(2, 3), 5);
        assert_eq!(restar(2, 3), -1);
        assert_eq!(multiplicar(2, 3), 6);
        assert_eq!(dividir(6, 3), Ok(2));
        assert!(dividir(1, 0).is_err());
    }
}
