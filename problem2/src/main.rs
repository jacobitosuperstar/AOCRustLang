use std::io;
use std::fs;

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
    println!("{}", file_contents);

    let mut input = Vec::new(); // Vector para guardar los valores
    let b = file_contents.lines(); // Iterador sobre "/n"
    for a in b {
        let a: Vec<&str> = a.split(' ').collect(); // separado string por ' '
        input.push(a); // Agregando los valores al vector muteable
        // println!("{}", a); // imprimiendo los valores de cada elemento
    }
    // imprimiendo el contenido del vector
    // println!("{:?}", input);

    // Punto 1

    // el vector donde va a estar el movimiento del submarino
    // [horizonal_position, depth]
    let mut sub_position : [isize; 2] = [0,0];

    for instruction in &input {
        let a = instruction[0];
        let b: isize = instruction[1].trim().parse().unwrap();

        if a == "forward" {
            sub_position[0] = sub_position[0] + b;
        } else if a == "up" {
            sub_position[1] = sub_position[1] - b;
        } else if a == "down" {
            sub_position[1] = sub_position[1] + b;
        }
    }

    // mostrando la información referente a la posición del submarin
    println!("{:?}", sub_position);

    // respuesta
    let answer = sub_position[0] * sub_position[1];
    println!("La respuesta del problema es");
    println!("{}", answer);

    // Punto 2

    // el vector donde va a estar el movimiento del submarino
    // [horizonal_position, depth, aim]
    let mut sub_position : [isize; 3] = [0,0,0];

    for instruction in &input {
        let a = instruction[0];
        let b: isize = instruction[1].trim().parse().unwrap();

        if a == "forward" {
            sub_position[0] = sub_position[0] + b;
            sub_position[1] = sub_position[1] + sub_position[2] * b;
        } else if a == "up" {
            sub_position[2] = sub_position[2] - b;
        } else if a == "down" {
            sub_position[2] = sub_position[2] + b;
        }
        // println!("{:?}",sub_position)
    }

    // mostrando la información referente a la posición del submarin
    // println!("{:?}", sub_position);

    // respuesta
    let answer = sub_position[0] * sub_position[1];
    println!("La respuesta del problema es");
    println!("{}", answer);

    // Punto 2
}
