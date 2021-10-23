use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    println!("Adivinhe o número!");

    let numero_secreto = rand::thread_rng().gen_range(1..101); //gera um inteiro entre 1 e 100, pode usar 1..=100
    
    loop {
        println!("Por favor insira o seu chute.");

        let mut chute = String::new(); // chute recebe uma nova instância do tipo String
        
        
        io::stdin() // sem o importação com o use std::io; podemos utilizar std::io::stdin()
        .read_line(&mut chute) // receber a entrada do usuario e inserir na string chute
        .expect("Não foi possível ler a linha"); // fará com que o programa trave e exiba "Failed to read line"

        let chute: u32 = match chute.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Você deve digital um número!");
                continue;
            }
        };

        println!("O seu chute foi: {}", chute); 

        match chute.cmp(&numero_secreto) {
            Ordering::Less => println!("Muito abaixo!"),
            Ordering::Greater => println!("Muito acima!"),
            Ordering::Equal => {
                println!("Você venceu! O número é o: {}", numero_secreto);
                break;
            }
        }
    }
}