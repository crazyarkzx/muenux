use dialoguer::Select;
use dialoguer::{theme::ColorfulTheme, Input};

mod ue {
    pub fn ue() {
        // julgue meus metodos, mas não meus resultados 😭
        println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
        print!("                        {}",
        "Urna Eletrônica\n"
        );
        println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n");
    }
    
    pub fn clear()
    {
        print!("\x1Bc");
    }
}

fn cli() -> Result<(), Box<dyn std::error::Error>> {
    crate::ue::ue();
    
    let op = Select::with_theme(&ColorfulTheme::default())
    .with_prompt("Selecione uma Opção: ")
    .default(0)
    .item("Votar")
    .item("Sair")
    .interact()?;
    
    let _ = match op {
        0 => votar(),
        1 => Ok(crate::ue::clear()),
        _ => std::process::exit(0)
    };
    Ok(())
 }
 
 fn votar() -> Result<(), Box<dyn std::error::Error>> {
     crate::ue::clear();
     crate::ue::ue();
     
     let input: i32 = Input::with_theme(&ColorfulTheme::default())
     .with_prompt("Digite seu Voto: ")
     .interact_text()
     .unwrap();
     
     let op = Select::with_theme(&ColorfulTheme::default())
     .with_prompt("Confirmar Voto: ")
     .default(0)
     .item("Sim")
     .item("Não")
     .interact()?;
    
    let _ = match op {
        0 => verify(),
        1 => votar()?,
        _ => std::process::exit(0)
    };
     
     fn verify(){}
     
     if input == 22
     {
         println!("\nVocê Votou no Flavio");
     }
     else if input == 14
     {
         println!("\nVocê Votou no Renan");
     }
     else if input == 13
     {
         println!("\nVocê Votou no Lula");
         panic!();
     }
     else
     {
         let _ = votar();
     }
     Ok(())
 }

fn main() {
    ue::clear();
    let _ = cli();
}
