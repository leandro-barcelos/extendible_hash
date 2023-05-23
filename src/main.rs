// TODO: Nao deixar inserir dois registros com a mesma chave

use std::{
    fmt::format,
    fs::File,
    io::{stdout, Write},
};

use crossterm::{
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use hash_alt1::HashAlt1;
use hash_alt2::HashAlt2;
use inquire::{max_length, Select, Text};
use random_util::{random_string, unique_random_numbers};

use crate::record::Record;

mod bucket_alt1;
mod bucket_alt2;
mod hash_alt1;
mod hash_alt2;
mod random_util;
mod record;

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
    let mut h_alt1: HashAlt1;
    let mut h_alt2: HashAlt2;

    h_alt1 = HashAlt1::new(1, 4);
    h_alt2 = HashAlt2::new(1, 4);

    let mut m = Menu::GeraHash;
    let mut stdout = stdout();

    loop {
        stdout.execute(Clear(ClearType::All)).unwrap();
        let header = "=".to_string().repeat(20);
        println!("{header} HASH TABLE {header}\n",);
        print!("{h_alt1}");

        match m {
            Menu::GeraHash => {
                let option =
                    Select::new("Gerar Hash", vec!["Novo", "Carregar", "Aleatorio", "Sair"])
                        .prompt();

                match option {
                    Ok("Novo") => m = Menu::Novo,
                    Ok("Carregar") => {
                        if let Ok(mut file) = File::open("hash_alt1.bin") {
                            h_alt1 = HashAlt1::deserialize(&mut file)
                        } else {
                            return;
                        }

                        if let Ok(mut file) = File::open("hash_alt2.bin") {
                            h_alt2 = HashAlt2::deserialize(&mut file)
                        } else {
                            return;
                        }
                        m = Menu::Principal;
                    }
                    Ok("Aleatorio") => m = Menu::Random,
                    Ok(_) => {
                        save_quit(&h_alt1, &h_alt2);
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

                let gd: u8 = gd.unwrap().parse().unwrap();
                let bs: u8 = bs.unwrap().parse().unwrap();

                h_alt1 = HashAlt1::new(gd, bs);
                h_alt2 = HashAlt2::new(gd, bs);

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
                        save_quit(&h_alt1, &h_alt2);
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
                    .prompt()
                    .unwrap();

                h_alt1.insert(Record {
                    nseq,
                    text: text.clone(),
                });

                h_alt2.insert(h_alt1.search(nseq).unwrap(), (text, nseq));

                m = Menu::Principal;
            }
            Menu::Remover => {
                let alt = Select::new(
                    "Qual tipo de chave: ",
                    vec!["Primaria (nseq)", "Secundaria (text + nseq)"],
                )
                .prompt();

                match alt {
                    Ok("Primaria (nseq)") => {
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

                        if let Some(r) = h_alt1.remove(nseq) {
                            h_alt2.remove((r.text, r.nseq));
                        }
                    }
                    Ok("Secundaria (text + nseq)") => {
                        let text = Text::new("Text: ")
                            .with_help_message("Digite a chave (nseq) para remocao: ")
                            .prompt()
                            .unwrap();

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

                        if h_alt2.remove((text, nseq)) {
                            h_alt1.remove(nseq);
                        }
                    }
                    Ok(_) => todo!(),
                    Err(_) => todo!(),
                }

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

                let f = h_alt1.search(nseq);

                match f {
                    Some(t) => println!(
                        "{} - {}",
                        h_alt1.buckets[t.0].data[t.1].nseq, h_alt1.buckets[t.0].data[t.1].text
                    ),
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

                h_alt1 = HashAlt1::new(gd, bs);
                h_alt2 = HashAlt2::new(gd, bs);

                rand_hash_values(&mut h_alt1, &mut h_alt2, n);
                m = Menu::Principal;
            }
        }
    }
}

fn save_quit(h1: &HashAlt1, h2: &HashAlt2) {
    let encoded1 = h1.serialize();
    let encoded2 = h2.serialize();

    let mut f = File::create("hash_alt1.bin").unwrap();
    f.write_all(&encoded1).unwrap();

    let mut f = File::create("hash_alt2.bin").unwrap();
    f.write_all(&encoded2).unwrap();
}

fn rand_hash_values(h1: &mut HashAlt1, h2: &mut HashAlt2, n: usize) {
    let mut nseq: i32;
    let mut text: String;

    let random_nseq = unique_random_numbers(0, n as i32);

    for i in 0..n {
        nseq = random_nseq[i];
        text = random_string(95);

        h1.insert(Record {
            nseq,
            text: text.clone(),
        });
        let rid = h1.search(nseq).unwrap();

        h2.insert(rid, (text, nseq));
    }
}
