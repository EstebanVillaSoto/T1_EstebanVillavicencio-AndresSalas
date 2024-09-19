mod tokenizer;

use tokenizer::{Token, TriangleTokenizer};
use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::process;

fn main() {
    // se obtiene los argumento de la línea de comandos
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Uso: tokenize <archivo_entrada.tri> [-o <archivo_salida.tok>]");
        process::exit(1);
    }

    let input_file = &args[1];
    let output_file = if args.len() > 3 && args[2] == "-o" {
        &args[3]
    } else {
        "tokens.out"
    };

    // se lee el archivo de entrada
    let content = match fs::read_to_string(input_file) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error leyendo el archivo '{}': {}", input_file, e);
            process::exit(1);
        }
    };

    // se crea la instancia del tokenizador
    let tokenizer = TriangleTokenizer::new();

    // toma el contenido del archivo y comienza con la tokenización
    let tokens = tokenizer.tokenize(&content);

    if let Err(e) = write_tokens_to_file(&tokens, output_file) {
        eprintln!("Error escribiendo al archivo '{}': {}", output_file, e);
        process::exit(1);
    }

    println!("Tokenización completada");
}


/*
/ Función que se encarga de escribir los tokens generados en un archivo
/ de salida, recibe un vector que contiene los tokens y el nombre del
/ del archivo de salida.
*/
fn write_tokens_to_file(tokens: &Vec<Token>, output_file: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(output_file)?;

    for token in tokens {
        writeln!(file, "{}\t{}\t{}", token.token_type, token.value, format_position(token))?;
    }

    Ok(())
}

/*
/ Función que se encarga de formatear la posición del token en el archivo,
/ recibe el token del cual se quiere obtener la posición y retorna las
/ posiciones X(linea) y Y(Columna) del token.
*/
fn format_position(token: &Token) -> String {
    format!("Linea {}, Columna {}", token.line, token.column)
}
