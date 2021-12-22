use std::io;
// paquete con el que vamos a crear los valores aleatorios
use rand::Rng;
// trayendo el ordenamiento/comparación al scope de acción
use std::cmp::Ordering;

fn main() {
    println!("Adivina el número!!!!");

    // hacemos la variable no muteable de "número secreto"
    let secret_number = rand::thread_rng().gen_range(1..1000);

    // mostramos el número secreto para hacer pruebas
    println!("El número secreto es {}",secret_number);

    // para hacer un loop y tener varios intentos en la entrada, usamos loop
    loop {

        // creamos una nueva entidad tipo string que es vacía
        // la string tiene "mut" para resaltar que no es estática, sino que es
        // variable
        // Cuando estamos en el loop la variable se tiene que redefinir
        let mut guess = String::new();

        println!("Porfavor coloca tu número!!!!");

        // hacemos que la referencia a la variable guess con el &mut, ya que
        // queremos que la referencia se modifique con esta operación. De lo
        // contrario, estaríamos haciendo simplemente la referencia a partir de
        // &guess

        io::stdin()
            .read_line(&mut guess)
            // adiconalmente manejamos los errores con el expect. El resultado
            // de la operación io::stdin() puede ser "Ok" o "Err", entonces
            // dependiendo del tipo, podemos manejar el tipo de eror.
            .expect("Error al leer la entrada");

        let guess: u32 = match guess
            .trim() // Elimina el espacio blanco de las strings
            .parse() // convierte la string en algún tipo de número
            {
                Ok(num) => num,
                Err(_) => continue,
            };

        // {} es un placeholder para las variables que siguen después de la
        // ",".
        // Las variables que están después de la "," se reemplazan en orden en
        // el string.
        println!("Tú número es {}",guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Demasiado pequeño"),
            Ordering::Greater => println!("Demasiado grande"),
            Ordering::Equal => {
                println!("Ganaste");
                break;
            }
        }
    }
}
