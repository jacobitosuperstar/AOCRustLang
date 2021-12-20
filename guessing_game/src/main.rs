use std::io;

fn main() {
    println!("Adivina el número!!!!");
    println!("Porfavor coloca tu número!!!!");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Error al leer la entrada");

    println!("Tú número es {}",guess);
}
