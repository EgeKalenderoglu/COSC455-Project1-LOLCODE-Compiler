use crate::traits::LexicalAnalyzer;

pub struct Lexer {
    source: Vec<char>,
    position: usize,
    lexeme: String,
    expecting_keyword: bool,
}

impl Lexer {
    pub fn get_lexeme(&self) -> String {
        self.lexeme.clone()
    }

    pub fn clear_lexeme(&mut self) {
        self.lexeme.clear();
    }

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

    pub fn new(source: &str) -> Lexer {
        Lexer {
            source: source.chars().collect(),
            position: 0,
            lexeme: String::new(),
            expecting_keyword: false,
        }
    }
}

impl LexicalAnalyzer for Lexer {
    fn get_char(&mut self) -> char {
        if self.position >= self.source.len() {
            println!("End of input.");
            std::process::exit(0);
        }

        let c = self.source[self.position];
        self.position += 1;
        c
    }

    fn add_char(&mut self, c: char) {
        self.lexeme.push(c);
    }

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