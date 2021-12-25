// use std::io;
use std::fs;
use std::cmp::Ordering;

fn main() {
    // let mut input = String::new();
    // println!("Porfavor coloca el nombre del archivo a correr");
    // leer entrada de usuario
    // io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    // eliminar el "/n" y convertir "&str" a "String"
    // input = input.trim().to_string();
    // println!("{}", input);

    // leer archivo
    // let file_contents = fs::read_to_string(input)
    // let file_contents = fs::read_to_string("test.txt")
    let file_contents = fs::read_to_string("input.txt")
    .expect("No se pudo leer el archivo");

    // imprimo el contenido del archivo
    // println!("{}", file_contents);

    let mut input = Vec::new(); // Vector para guardar los valores
    let b = file_contents.lines(); // Iterador sobre "/n"
    for a in b {
        let chars: Vec<char> = a
            .chars() // convierto los elementos del vector en caracteres
            .collect(); // colecciono los valores anteriores en un vector
        input.push(chars); // Agregando los valores al vector muteable
        // println!("{}", a); // imprimiendo los valores de cada elemento
    }
    println!("Valores de entredada del archivo");
    println!("{:?}", input);

    // se van a guardar cada uno de los valores [i] en un vector.
    // vector para guardar los valores de las filas
    let mut eclipse_chars = Vec::new();
    // saco la longitud de uno de los elementos del vector
    // acá no habría problema ya que todos los elementos de los vectores son
    // del mismo largo
    let vector_lenght = &input[1].len() + 0;
    println!("{}", vector_lenght);
    // for para moverme a lo largo de un vector
    for i in 0..vector_lenght {
        let mut eclipse_values = Vec::new();
        // for para moverme entre vectores
        for a in &input{
            // eclipse_chars.push(a);
            eclipse_values.push(
                a[i] // el valor [i] del vector
                .to_string() // volvemos le char una string
                .parse::<u32>() // parseamos el string a i32
                .unwrap() //  unwrap para que el programa pare si hay un error
                );
        }
        eclipse_chars.push(eclipse_values);
    }
    println!("Valores de entrada para los cálculos de eclipse");
    println!("{:?}", eclipse_chars);

    // número al que vamos a comprar la repetición de bits
    // por ejemplo
    // es 12 la suma de la mitad de los elementos
    // si la suma de todos los elementos da más de 12, se repiten más el 1
    // si la suma de todos los elementos da menos de 12, se repiten más el 0
    let mut comparing_value = ((eclipse_chars[1].len() as f32)/(2 as f32)).ceil() as usize;
    if (eclipse_chars[1].len() as f32)%(2 as f32) == 0 as f32 {
        comparing_value = comparing_value + 1;
    }
    println!("Valor al que vamos a comparar");
    println!("{}", comparing_value);

    // haciendo el vector donde vamos a guardar cada uno de los elementos que
    // componen el gamma_rate
    let mut gamma_rate_vector = Vec::new();
    let mut epsilon_rate_vector = Vec::new();
    for a in &eclipse_chars {
        let sum_a: u32 = a.iter().sum(); // sumo elementos en el iterable
        match (sum_a as usize).cmp(&comparing_value){
            Ordering::Less => {
                let gamma_rate_element: String = "0".to_string();
                let epsilon_rate_element: String = "1".to_string();
                gamma_rate_vector.push(gamma_rate_element);
                epsilon_rate_vector.push(epsilon_rate_element);
            },
            Ordering::Greater => {
                let gamma_rate_element: String = "1".to_string();
                let epsilon_rate_element: String = "0".to_string();
                gamma_rate_vector.push(gamma_rate_element);
                epsilon_rate_vector.push(epsilon_rate_element);
            },
            Ordering::Equal => {
                let gamma_rate_element: String = "1".to_string();
                let epsilon_rate_element: String = "0".to_string();
                gamma_rate_vector.push(gamma_rate_element);
                epsilon_rate_vector.push(epsilon_rate_element);
            }
        }
    }
    println!("Gamma rate vector");
    println!("{:?}", gamma_rate_vector);
    println!("Epsilon rate vector");
    println!("{:?}", epsilon_rate_vector);

    let gamma_rate = gamma_rate_vector.join("");
    // hacemos un préstamo dirigido y así pasamos de una String a &str
    let gamma_rate: &str = &gamma_rate;
    // hacemos un préstamo dirigido y así pasamos de una String a &str
    let epsilon_rate = epsilon_rate_vector.join("");
    let epsilon_rate: &str = &epsilon_rate;

    println!("Gamma rate");
    println!("{:?}", gamma_rate);
    println!("Epsilon rate");
    println!("{:?}", epsilon_rate);

    // hacemos el cambio de &str binaria a número
    let gamma_rate = usize::from_str_radix(gamma_rate,2).unwrap();
    println!("Gamma rate");
    println!("{:?}", gamma_rate);
    // hacemos el cambio de &str binaria a número
    let epsilon_rate = usize::from_str_radix(epsilon_rate,2).unwrap();
    println!("Epsilon rate");
    println!("{:?}", epsilon_rate);

    // respuesta del problema
    println!("Respuesta del Problema");
    println!("{}", epsilon_rate*gamma_rate);
}
