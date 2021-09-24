use std::io;

fn main() {
    println!("Devinez le nombre !");

    println!("Veuillez entrer un nombre.");

    let mut supposition = String::new();

    io::stdin()
        .read_line(&mut supposition)
        .expect("Échec de la lecture de l'entrée utilisateur");

    println!("Votre nombre : {}", supposition);
   // let x: Result<u32, &str> = Ok(2);
    //assert_eq!(x.unwrap(), 2);

   let result = rand::thread_rng().gen_range(1..101);

   
}
