use std::io;
fn main() {
    println!("Ingresa el total de la cuenta");
    let mut total = String::new();
    io::stdin().read_line(&mut total).unwrap();
    println!("Total: {}", total.trim());

    //Aqui se abrira una caja sorpresa en donde si el usuario escribe texto mostrara error
    let cuenta: u32 = match total.trim().parse() {
        Ok(numero) => numero,
        Err(_) => {
            println!("Error: el total ingresado no es un numero");
            return; //aqui termina la caja
        }
    };

    let propina: u32 = cuenta / 10;
    let total_con_propina: u32 = cuenta + propina;

    if cuenta >= 500 {
        println!("La cuenta es mayor a 500, la propina es del 20%");
        let propina_alta: u32 = cuenta / 5;
        let total_con_propina_alta: u32 = cuenta + propina_alta;
        println!("El total seria: {}", total_con_propina_alta);
    } else {
        println!("La propina es menor a 500, la propina es del 10%");
        println!("El total seria: {}", total_con_propina);
    }
}
