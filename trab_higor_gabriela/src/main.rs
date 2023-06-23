use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

struct Dicionario {
    palavras: Vec<(String, String)>,  // (palavra, significado)
}

impl Dicionario {
    fn novo() -> Dicionario {
        Dicionario {
            palavras: Vec::new(),
        }
    }

    fn adicionar_palavra(&mut self, palavra: String, significado: String) {
        self.palavras.push((palavra, significado));
    }

    fn buscar(&self, palavra_chave: &str) -> Option<&str> {
        for (palavra, significado) in &self.palavras {
            if palavra == palavra_chave {
                return Some(significado);
            }
        }
        None
    }
}

fn main() {
    let mut dicionario: Dicionario = Dicionario::novo();

    loop {
        println!("Menu:");
        println!("1. Adicionar palavra");
        println!("2. Buscar palavra");
        println!("3. Salvar dicionário em arquivo");
        println!("4. Carregar dicionário do arquivo");
        println!("5. Sair");
        println!("Escolha uma opção:");

        let mut opcao = String::new();
        io::stdin().read_line(&mut opcao).expect("Falha ao ler a opção");

        match opcao.trim() {
            "1" => adicionar_palavra(&mut dicionario),
            "2" => buscar_palavra(&dicionario),
            "3" => salvar_dicionario_em_arquivo(&dicionario),
            "4" => carregar_dicionario_do_arquivo(&mut dicionario),
            "5" => break,
            _ => println!("Opção inválida"),
        }

        println!();
    }
}

fn adicionar_palavra(dicionario: &mut Dicionario) {
    println!("Digite a palavra:");
    let mut palavra = String::new();
    io::stdin().read_line(&mut palavra).expect("Falha ao ler a palavra");

    println!("Digite o significado:");
    let mut significado = String::new();
    io::stdin().read_line(&mut significado).expect("Falha ao ler o significado");

    dicionario.adicionar_palavra(palavra.trim().to_string(), significado.trim().to_string());
    println!("Palavra adicionada com sucesso.");
}

fn buscar_palavra(dicionario: &Dicionario) {
    println!("Digite a palavra que deseja pesquisar:");
    let mut palavra_chave = String::new();
    io::stdin().read_line(&mut palavra_chave).expect("Falha ao ler a palavra");

    let palavra_chave = palavra_chave.trim();

    match dicionario.buscar(&palavra_chave) {
        Some(significado) => println!("Significado de {}: {}", palavra_chave, significado),
        None => println!("Palavra não encontrada no dicionário."),
    }
}

fn salvar_dicionario_em_arquivo(dicionario: &Dicionario) {
    let mut arquivo = File::create("dicionario.txt").expect("Falha ao criar o arquivo");

    for (palavra, significado) in &dicionario.palavras {
        writeln!(&mut arquivo, "Palavra: {}", palavra).expect("Falha ao escrever no arquivo");
        writeln!(&mut arquivo, "Significado: {}", significado).expect("Falha ao escrever no arquivo");
    }

    arquivo.flush().expect("Falha ao escrever no arquivo");

    println!("Dicionário salvo no arquivo.");
}

fn carregar_dicionario_do_arquivo(dicionario: &mut Dicionario) {
    let arquivo = File::open("dicionario.txt");
    match arquivo {
        Ok(file) => {
            let leitor = BufReader::new(file);

            let mut palavra: String = String::new();
            let mut significado: String = String::new();

            for linha in leitor.lines() {
                let linha = linha.expect("Falha ao ler a linha do arquivo");

                if linha.starts_with("Palavra:") {
                    palavra = linha.replace("Palavra: ", "");
                } else if linha.starts_with("Significado:") {
                    significado = linha.replace("Significado: ", "");

                    dicionario.adicionar_palavra(palavra.trim().to_string(), significado.trim().to_string());

                    palavra.clear();
                    significado.clear();
                }
            }

            println!("Dicionário carregado do arquivo.");
        }
        Err(_) => {
            println!("Erro ao abrir o arquivo do dicionário.");
        }
    }
}
