use std::env;
use std::fs;
use std::process;

#[derive(Debug)]
pub struct Token {
    pub token_type: String,
    pub value: String,
    pub line: usize,
    pub column: usize,
}

/*
/ Función que se encarga de leer el archivo con los tokens y devuelve un 
/ vector de token
*/
fn read_tokens(file_path: &str) -> Result<Vec<Token>, std::io::Error> {
    let content = fs::read_to_string(file_path)?;
    let mut tokens = Vec::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() >= 5 {
            let token_type = parts[0].to_string();
            let value = parts[1].to_string();
            let line_info: Vec<&str> = parts[3].split(',').collect();
            let line_num = line_info[0].parse::<usize>().unwrap_or(0);
            let column_num = parts[4].replace("Columna", "").parse::<usize>().unwrap_or(0);

            tokens.push(Token {
                token_type,
                value,
                line: line_num,
                column: column_num,
            });
        }
    }

    Ok(tokens)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = if args.len() > 1 {
        &args[1]
    } else {
        "tokens.out"
    };

    match read_tokens(file_path) {
        Ok(tokens) => {
            println!("Listado de tokens del archivo.");
            for token in tokens {
                println!("{:?}", token);
            }
        }
        Err(e) => {
            eprintln!("Error leyendo el archivo. '{}': {}", file_path, e);
            process::exit(1);
        }
    }
}
