use crate::traits::LexicalAnalyzer;
// Stores the source characters and the current position used by the
// character-by-character lexical analyzer.
pub struct Lexer {
    source: Vec<char>,
    position: usize,
    lexeme: String,
    expecting_keyword: bool,
}

impl Lexer {
    // Returns the lexeme currently being built by the lexer.
    pub fn get_lexeme(&self) -> String {
        self.lexeme.clone()
    }
    // Clears the previous lexeme before the lexer begins building a new token.
    pub fn clear_lexeme(&mut self) {
        self.lexeme.clear();
    }
    // Reads the source character by character and returns the next LOLCODE token.
    // It handles keywords, annotations, normal text, and reports invalid tokens.
    pub fn next_token(&mut self) -> String {
        self.clear_lexeme();
    
        while self.position < self.source.len()
            && self.source[self.position].is_whitespace()
        {
            self.get_char();
        }
    
        if self.position >= self.source.len() {
            return String::new();
        }
        // After #MAEK or #GIMMEH, the following word is treated as a grammar keyword.
        if self.expecting_keyword {
            while self.position < self.source.len() {
                let c = self.source[self.position];
        
                if c.is_whitespace() || c == '#' {
                    break;
                }
        
                let c = self.get_char();
                self.add_char(c);
            }
        
            self.expecting_keyword = false;
            
            return self.get_lexeme();
        
        
        }
        // Tokens beginning with # are LOLCODE annotations and must be validated.
        if self.source[self.position] == '#' {
            while self.position < self.source.len() {
                let c = self.source[self.position];
    
                if c.is_whitespace() {
                    break;
                }
    
                let c = self.get_char();
                self.add_char(c);
            }
    
            let token = self.get_lexeme();
    
            if !self.lookup(&token) {
                eprintln!("Lexical error: '{}' is not a valid token.", token);
                std::process::exit(1);
            }
    
            if token.eq_ignore_ascii_case("#MAEK")
                || token.eq_ignore_ascii_case("#GIMMEH")
            {
                self.expecting_keyword = true;
            }
    
            return token;
        }
    

        while self.position < self.source.len()
            && self.source[self.position] != '#'
        {
            let c = self.get_char();
            self.add_char(c);
        }
    
        self.get_lexeme().trim().to_string()
    }
    // Creates a new lexer from the source text and starts at the first character.
    pub fn new(source: &str) -> Lexer {
        Lexer {
            source: source.chars().collect(),
            position: 0,
            lexeme: String::new(),
            expecting_keyword: false,
        }
    }
}
// Implements the required character-by-character lexical analyzer operations.
impl LexicalAnalyzer for Lexer {
    // Returns the next character from the source and moves the position forward.
    fn get_char(&mut self) -> char {
        if self.position >= self.source.len() {
            println!("End of input.");
            std::process::exit(0);
        }

        let c = self.source[self.position];
        self.position += 1;
        c
    }
    // Adds one character to the lexeme currently being constructed.
    fn add_char(&mut self, c: char) {
        self.lexeme.push(c);
    }
    // Checks whether a LOLCODE annotation or keyword is part of the valid token set.
    // The comparison is case-insensitive.
    fn lookup(&self, s: &str) -> bool {
        match s.to_uppercase().as_str() {
            "#HAI"
            | "#KBYE"
            | "#OBTW"
            | "#TLDR"
            | "#MAEK"
            | "#OIC"
            | "#GIMMEH"
            | "#MKAY"
            | "#NEWLINE"
            | "#IHAZ"
            | "#ITIZ"
            | "HEAD"
            | "TITLE"
            | "PARAGRAF"
            | "BOLD"
            | "LIST"
            | "ITEM"
            | "LINX"
            | "VAR" => true,

            _ => false,
        }
    }
}