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
mod random_util;

enum Menu {
    GeraHash,
    Novo,
    Principal,
    Inserir,
    Remover,
    Buscar,
    Random,
}

fn main() {
    let mut h: Hash;

    h = Hash::new(1, 4);

    let mut m = Menu::GeraHash;
    let mut stdout = stdout();

    loop {
        stdout.execute(Clear(ClearType::All)).unwrap();
        let header = "=".to_string().repeat(20);

        // if h.
        println!("{header} HASH TABLE {header}\n",);
        println!("{h}");

        match m {
            Menu::GeraHash => {
                let option =
                    Select::new("Gerar Hash", vec!["Novo", "Carregar", "Aleatorio", "Sair"])
                        .prompt();

                match option {
                    Ok("Novo") => m = Menu::Novo,
                    Ok("Carregar") => {
                        if let Ok(mut file) = File::open("hash_alt1.bin") {
                            h = Hash::deserialize(&mut file)
                        } else {
                            return;
                        }
                        m = Menu::Principal;
                    }
                    Ok("Aleatorio") => m = Menu::Random,
                    Ok(_) => {
                        let encoded = h.serialize();

                        let mut f = File::create("hash_alt1.bin").unwrap();
                        f.write_all(&encoded).unwrap();

                        break;
                    }
                    Err(_) => continue,
                }
            }
            Menu::Novo => {
                let gd = Text::new("Global Depth inicial: ")
                    .with_default("2")
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

                let bs = Text::new("Tamanho do bucket: ")
                    .with_default("4")
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

                h = Hash::new(gd.unwrap().parse().unwrap(), bs.unwrap().parse().unwrap());

                m = Menu::Principal;
            }

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
            Menu::Random => {
                let n = Text::new("Quantidade de registros: ")
                    .with_default("100")
                    .with_help_message("Sera gerado um hash novo com n registros aleatórios")
                    .with_validator(|e: &str| {
                        let parsed: Result<i32, _> = e.parse();
                        if let Ok(_) = parsed {
                            Ok(inquire::validator::Validation::Valid)
                        } else {
                            Ok(inquire::validator::Validation::Invalid(
                                "Tem que ser um inteiro".into(),
                            ))
                        }
                    })
                    .prompt();

                let n: usize = n.unwrap().parse().unwrap();
                let gd: u8;
                let bs: u8;

                let log_n = (n as f64).log2().ceil() as u8;

                if n < 100 {
                    bs = 4;
                } else if n < 1000 {
                    bs = 8;
                } else {
                    bs = 16;
                }

                gd = log_n - (bs as f64).log2() as u8;

                h = Hash::rand_hash_values(gd, bs, n);
                m = Menu::Principal;
            }
        }
    }
}
