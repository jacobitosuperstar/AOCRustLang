/// Measurement of the deep of the keys in the ocean.
/// We make a vector that contains all the values of the entry. Then compares
/// the current value with the next one, if the value increases we add a number
/// to a counter.

use std::io;
use std::fs;
use std::cmp::Ordering;

fn main() {
    let mut input = String::new();
    println!("Porfavor coloca el nombre del archivo a correr");
    // leer entrada de usuario
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    // eliminar el "/n" y convertir "&str" a "String"
    input = input.trim().to_string();
    // println!("{}", input);

    // leer archivo
    let file_contents = fs::read_to_string(input)
    // let file_contents = fs::read_to_string("test.txt")
    .expect("No se pudo leer el archivo");

    // imprimo el contenido del archivo
    // println!("{}", file_contents);

    let mut input = Vec::new(); // Vector para guardar los valores
    let b = file_contents.lines(); // Iterador sobre "/n"
    for a in b {
        let a: isize = match a.trim().parse(){
            Ok(num) => num,
            Err(_) => {continue},
        };
        input.push(a); // Agregando los valores al vector muteable
        // println!("{}", a); // imprimiendo los valores de cada elemento
    }

    let mut counter: isize = 0;
    let len_vector = input.len() - 1;
    for i in 0..len_vector {
        if i == len_vector{
            break;
        }
        else {
            let a = input[i];
            let b = input[i+1];
            match b.cmp(&a){
               Ordering::Less => continue,
               Ordering::Greater => {
                   counter = counter + 1;
               },
               Ordering::Equal => continue,
           };
        }
    }
    println!("La cantidad de veces que la profundidad aumenta es {}",counter);
}
