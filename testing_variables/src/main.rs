fn multiplication(x: isize, y: isize){
}

fn main() {
    // Cuando usamos "const", hacemos referencia a una constante que NUNCA
    // va a cambiar de valor a lo largo de la ejecución del programa

    const CONSTANTES_EN_MAYUS : u32 = 60*60*3;
    println!("{}", CONSTANTES_EN_MAYUS);
    {
        // Cuando usamos mut, podemos cambiar el valor de una variables, pero
        // no podemos cambiar su tipo.
        let mut x = 5;
        println!("{}",x);
        x = 4;
        println!("{}",x);
    }
    {
        // Cuando declaramos la misma variable con "let", podemos cambiar tanto
        // el valor como el tipo de la variable

        let space = "Jacobo";
        println!("{}", space);
        let space = space.len();
        println!("{}", space);
    }
    {
        // volviendo a imprimir la constante
        println!("{}", CONSTANTES_EN_MAYUS);
    }
    {

    }
}
