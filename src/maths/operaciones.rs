pub fn sumar(a: f32, b: f32) -> f32 {
    a + b
}

pub fn restar(a: f32, b: f32) -> f32 {
    a - b
}

pub fn multiplicar(a: f32, b: f32) -> f32 {
    a * b
}

pub fn dividir(a: f32, b: f32) -> Result<f32, String> {
    if b == 0.0 {
        return Err(String::from("No se puede dividir en cero"));
    }
    Ok(a / b)
}
