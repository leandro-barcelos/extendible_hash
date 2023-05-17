// TODO: Nao deixar inserir dois registros com a mesma chave

use std::{
    fs::File,
    io::{stdout, Write},
};

use crossterm::{
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use hash::Hash;
use inquire::{max_length, Select, Text};

mod bucket;
mod hash;

enum Menu {
    Principal,
    Inserir,
    Remover,
    Buscar,
}

fn main() {
    let mut h = Hash::new(2, 4);
    let mut m = Menu::Principal;
    let mut stdout = stdout();

    loop {
        stdout.execute(Clear(ClearType::All)).unwrap();
        let header = "=".to_string().repeat(20);
        println!("{header} HASH TABLE {header}\n",);
        println!("{h}");

        match m {
            Menu::Principal => {
                let option = Select::new(
                    "O que voce quer fazer?",
                    vec!["Inserir", "Remover", "Buscar", "Sair"],
                )
                .prompt();

                match option {
                    Ok("Inserir") => m = Menu::Inserir,
                    Ok("Remover") => m = Menu::Remover,
                    Ok("Buscar") => m = Menu::Buscar,
                    Ok(_) => {
                        let encoded = h.serialize();

                        let mut f = File::create("hash_alt1.bin").unwrap();
                        f.write_all(&encoded).unwrap();

                        break;
                    }
                    Err(_) => continue,
                }
            }
            Menu::Inserir => {
                let nseq = Text::new("Nseq: ")
                    .with_help_message("Digite um valor para o campo nseq do registro")
                    .with_validator(|n: &str| {
                        let parsed: Result<i32, _> = n.parse();
                        if let Ok(_) = parsed {
                            Ok(inquire::validator::Validation::Valid)
                        } else {
                            Ok(inquire::validator::Validation::Invalid(
                                "Tem que ser um inteiro".into(),
                            ))
                        }
                    })
                    .prompt();

                let nseq: i32 = nseq.unwrap().parse().unwrap();

                let text = Text::new("Text: ")
                    .with_help_message("Digite um valor para o campo nseq do registro")
                    .with_validator(max_length!(96, "No maximo 96 caracteres"))
                    .prompt();

                h.insert((nseq, text.unwrap()));

                m = Menu::Principal;
            }
            Menu::Remover => {
                let nseq = Text::new("Nseq: ")
                    .with_help_message("Digite a chave (nseq) para remocao: ")
                    .with_validator(|n: &str| {
                        let parsed: Result<i32, _> = n.parse();
                        if let Ok(_) = parsed {
                            Ok(inquire::validator::Validation::Valid)
                        } else {
                            Ok(inquire::validator::Validation::Invalid(
                                "Tem que ser um inteiro".into(),
                            ))
                        }
                    })
                    .prompt();

                let nseq: i32 = nseq.unwrap().parse().unwrap();

                h.remove(nseq);

                m = Menu::Principal;
            }
            Menu::Buscar => {
                let nseq = Text::new("Nseq: ")
                    .with_help_message("Digite a chave (nseq) para remocao: ")
                    .with_validator(|n: &str| {
                        let parsed: Result<i32, _> = n.parse();
                        if let Ok(_) = parsed {
                            Ok(inquire::validator::Validation::Valid)
                        } else {
                            Ok(inquire::validator::Validation::Invalid(
                                "Tem que ser um inteiro".into(),
                            ))
                        }
                    })
                    .prompt();

                let nseq: i32 = nseq.unwrap().parse().unwrap();

                let f = h.search(nseq);

                match f {
                    Some(t) => println!("{} - {}", t.0, t.1),
                    None => println!("Chave {nseq} nao encontrada"),
                }

                Select::new("", vec!["Voltar"]).prompt().unwrap();

                m = Menu::Principal;
            }
        }
    }
}
