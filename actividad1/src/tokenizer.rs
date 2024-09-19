#[derive(Debug)]
pub struct Token {
    pub token_type: String,
    pub value: String,
    pub line: usize,
    pub column: usize,
}

pub struct TriangleTokenizer;

impl TriangleTokenizer {
    /*
    / Función que se encarga de crear una instancia del tokenizador.
    */
    pub fn new() -> Self {
        TriangleTokenizer
    }
    /*
    / Función que se encarga de tokenizar un string, donde divide el código
    / en tokens, recibe una cadena que contiene el código fuente y retorna
    / un vector de token con todos los tokens generados.
    */
    pub fn tokenize(&self, input: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut line_num = 1;

        for line in input.lines() {
            let mut column_num = 1;
            let words = line.split_whitespace();

            for word in words {
                let token_type = self.identify_token(word);
                tokens.push(Token {
                    token_type,
                    value: word.to_string(),
                    line: line_num,
                    column: column_num,
                });

                column_num += word.len() + 1;
            }

            line_num += 1;
        }

        tokens
    }

    /* 
    / Función que identifica el tipo del token, recibe un token y dermina si
    / si es una palabra clave, operador entero o identificador, y retorna
    / un string que representa el tipo de token .  
    */
    fn identify_token(&self, token: &str) -> String {
        match token {
            "const" | "var" | "if" | "then" | "else" | "while" => "Keyword".to_string(),
            "=" | ":=" | "+" | "-" | "*" | "/" => "Operator".to_string(),
            _ if token.parse::<i64>().is_ok() => "Integer-Literal".to_string(),
            _ => "Identifier".to_string(),
        }
    }
}
