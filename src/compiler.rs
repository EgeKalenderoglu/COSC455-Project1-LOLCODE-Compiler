use crate::lexer::Lexer;
use crate::traits::{Compiler, SyntaxAnalyzer};
use std::collections::HashMap;
use std::fs;
pub struct LolCompiler {
    lexer: Lexer,
    current_token: String,
    scopes: Vec<HashMap<String, String>>,
    html_output: String,
}

impl LolCompiler {

    pub fn save_html(&self, filename: &str) {
        if let Err(error) = fs::write(filename, &self.html_output) {
            eprintln!("Error writing HTML file: {}", error);
            std::process::exit(1);
        }
    }

    fn add_space_before_next_text(&mut self) {
        if !self.current_token.is_empty()
            && !self.current_token.starts_with('#')
        {
            if let Some(first_char) = self.current_token.chars().next() {
                if first_char.is_alphanumeric() {
                    self.html_output.push(' ');
                }
            }
        }
    }

    pub fn new() -> LolCompiler {
        LolCompiler {
            lexer: Lexer::new(""),
            current_token: String::new(),
            scopes: Vec::new(),
            html_output: String::new(),
        }
    }


    fn advance(&mut self) {
        let next = self.next_token();
        self.set_current_token(next);
    }

    fn resolve_variable(&self, name: &str) -> Option<String> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Some(value.clone());
            }
        }
    
        None
    }

    fn match_token(&mut self, expected: &str) {
        if self.current_token.eq_ignore_ascii_case(expected) {
            self.advance();
        } else {
            eprintln!(
                "Syntax error: expected '{}', but found '{}'.",
                expected,
                self.current_token
            );
            std::process::exit(1);
        }
    }
}

impl Compiler for LolCompiler {
    fn compile(&mut self, source: &str) {
        self.lexer = Lexer::new(source);
        self.scopes.clear();
        self.scopes.push(HashMap::new());
        self.html_output.clear();
        self.current_token = self.next_token();
    }

    fn next_token(&mut self) -> String {
        self.lexer.next_token()
    }

    fn parse(&mut self) {
        self.parse_lolcode();
    }

    fn current_token(&self) -> String {
        self.current_token.clone()
    }

    fn set_current_token(&mut self, tok: String) {
        self.current_token = tok;
    }
}

impl SyntaxAnalyzer for LolCompiler {
    fn parse_lolcode(&mut self) {
        self.match_token("#HAI");
        self.html_output.push_str("<html>\n");
        while self.current_token.eq_ignore_ascii_case("#OBTW") {
            self.parse_comment();
        }
    
        let mut body_started = false;
    
        if self.current_token.eq_ignore_ascii_case("#MAEK") {
            self.match_token("#MAEK");
    
            if self.current_token.eq_ignore_ascii_case("HEAD") {
                self.parse_head();
            } else if self.current_token.eq_ignore_ascii_case("PARAGRAF") {
                self.parse_paragraph();
                body_started = true;
            } else if self.current_token.eq_ignore_ascii_case("LIST") {
                self.parse_list();
                body_started = true;
            } else {
                eprintln!(
                    "Syntax error: expected HEAD, PARAGRAF, or LIST after #MAEK, but found '{}'.",
                    self.current_token
                );
                std::process::exit(1);
            }
        }
    
        if !body_started && self.current_token.eq_ignore_ascii_case("#IHAZ") {
            self.parse_variable_define();
        }
    
        self.parse_body();
    
        self.match_token("#KBYE");
        self.html_output.push_str("\n</html>\n");
        if !self.current_token.is_empty() {
            eprintln!(
                "Syntax error: unexpected text after #KBYE: '{}'.",
                self.current_token
            );
            std::process::exit(1);
        }
    }

    fn parse_head(&mut self) {
        self.match_token("HEAD");
    
        self.html_output.push_str("<head>");
    
        self.parse_title();
    
        self.match_token("#MKAY");
    
        self.html_output.push_str("</head>");
    }

    fn parse_title(&mut self) {
        self.match_token("#GIMMEH");
        self.match_token("TITLE");
    
        if self.current_token.is_empty()
            || self.current_token.starts_with('#')
        {
            eprintln!(
                "Syntax error: expected title text, but found '{}'.",
                self.current_token
            );
            std::process::exit(1);
        }
    
        let title = self.current_token.clone();
    
        self.html_output.push_str("<title>");
        self.html_output.push_str(&title);
        self.html_output.push_str("</title>");
    
        self.parse_text();
    
        self.match_token("#OIC");
    }

    fn parse_comment(&mut self) {
        self.match_token("#OBTW");
    
        if self.current_token.is_empty()
            || self.current_token.starts_with('#')
        {
            eprintln!(
                "Syntax error: expected comment text, but found '{}'.",
                self.current_token
            );
            std::process::exit(1);
        }
    
        let comment = self.current_token.clone();
    
        self.html_output.push_str("<!-- ");
        self.html_output.push_str(&comment);
        self.html_output.push_str(" -->");
    
        self.parse_text();
    
        self.match_token("#TLDR");
    }

    fn parse_body(&mut self) {
        if self.current_token.is_empty()
            || self.current_token.eq_ignore_ascii_case("#KBYE")
        {
            return;
        }
    
        if self.current_token.eq_ignore_ascii_case("#MAEK") {
            self.match_token("#MAEK");
        
            if self.current_token.eq_ignore_ascii_case("PARAGRAF") {
                self.parse_paragraph();
            } else if self.current_token.eq_ignore_ascii_case("LIST") {
                self.parse_list();
            } else {
                eprintln!(
                    "Syntax error: expected PARAGRAF or LIST after #MAEK, but found '{}'.",
                    self.current_token
                );
                std::process::exit(1);
            }
        
            self.parse_body();

        } else if self.current_token.eq_ignore_ascii_case("#GIMMEH") {
            self.match_token("#GIMMEH");
    
            if self.current_token.eq_ignore_ascii_case("BOLD") {
                self.parse_bold();
            } else if self.current_token.eq_ignore_ascii_case("LINX") {
                self.parse_link();
            } else if self.current_token.eq_ignore_ascii_case("VAR") {
                self.parse_variable_use();
            } else {
                eprintln!(
                    "Syntax error: expected BOLD, LINX, or VAR after #GIMMEH, but found '{}'.",
                    self.current_token
                );
                std::process::exit(1);
            }
    
            self.parse_body();
    
        } else if self.current_token.eq_ignore_ascii_case("#OBTW") {
            self.parse_comment();
            self.parse_body();
    
        } else if self.current_token.eq_ignore_ascii_case("#NEWLINE") {
            self.parse_newline();
            self.parse_body();
    
        } else if !self.current_token.starts_with('#') {
            let text = self.current_token.clone();
            self.html_output.push_str(&text);
            self.html_output.push(' ');

            self.parse_text();
            self.parse_body();
    
        } else {
            eprintln!(
                "Syntax error: unexpected token '{}' in body.",
                self.current_token
            );
            std::process::exit(1);
        }
    }

    fn parse_paragraph(&mut self) {
        self.match_token("PARAGRAF");
        self.html_output.push_str("<p>");
        self.scopes.push(HashMap::new());
    
        if self.current_token.eq_ignore_ascii_case("#IHAZ") {
            self.parse_variable_define();
        }
    
        self.parse_inner_paragraph();
    
        self.match_token("#MKAY");
        self.html_output.push_str("</p>");
        self.scopes.pop();
    }

    fn parse_inner_paragraph(&mut self) {
        if self.current_token.is_empty()
            || self.current_token.eq_ignore_ascii_case("#MKAY")
        {
            return;
        }
    
        self.parse_inner_text();
        self.parse_inner_paragraph();
    }

    fn parse_inner_text(&mut self) {
        if self.current_token.eq_ignore_ascii_case("#MAEK") {
            self.match_token("#MAEK");
    
            if self.current_token.eq_ignore_ascii_case("LIST") {
                self.parse_list();
            } else {
                eprintln!(
                    "Syntax error: expected LIST inside paragraph, but found '{}'.",
                    self.current_token
                );
                std::process::exit(1);
            }
    
        } else if self.current_token.eq_ignore_ascii_case("#GIMMEH") {
            self.match_token("#GIMMEH");
    
            if self.current_token.eq_ignore_ascii_case("BOLD") {
                self.parse_bold();
            } else if self.current_token.eq_ignore_ascii_case("LINX") {
                self.parse_link();
            } else if self.current_token.eq_ignore_ascii_case("VAR") {
                self.parse_variable_use();
            } else {
                eprintln!(
                    "Syntax error: expected BOLD, LINX, or VAR after #GIMMEH, but found '{}'.",
                    self.current_token
                );
                std::process::exit(1);
            }
    
        } else if self.current_token.eq_ignore_ascii_case("#OBTW") {
            self.parse_comment();
    
        } else if self.current_token.eq_ignore_ascii_case("#NEWLINE") {
            self.parse_newline();
    
        } else if !self.current_token.starts_with('#') {
            let text = self.current_token.clone();
            self.html_output.push_str(&text);
            self.html_output.push(' ');
        
            self.parse_text();
    
        } else {
            eprintln!(
                "Syntax error: unexpected token '{}' inside paragraph.",
                self.current_token
            );
            std::process::exit(1);
        }
    }

    fn parse_variable_define(&mut self) {
        self.match_token("#IHAZ");
    
        if self.current_token.is_empty()
            || self.current_token.starts_with('#')
            || self.current_token.contains(char::is_whitespace)
        {
            eprintln!(
                "Syntax error: expected a single-word variable name, but found '{}'.",
                self.current_token
            );
            std::process::exit(1);
        }
    
        let variable_name = self.current_token.clone();
        self.advance();
    
        self.match_token("#ITIZ");
    
        if self.current_token.is_empty()
            || self.current_token.starts_with('#')
        {
            eprintln!(
                "Syntax error: expected a variable value, but found '{}'.",
                self.current_token
            );
            std::process::exit(1);
        }
    
        let variable_value = self.current_token.clone();
        self.advance();
    
        self.match_token("#MKAY");
    
        if let Some(current_scope) = self.scopes.last_mut() {
            current_scope.insert(variable_name, variable_value);
        }
    }

    fn parse_variable_use(&mut self) {
        self.match_token("VAR");
    
        if self.current_token.is_empty()
            || self.current_token.starts_with('#')
            || self.current_token.contains(char::is_whitespace)
        {
            eprintln!(
                "Syntax error: expected a single-word variable name, but found '{}'.",
                self.current_token
            );
            std::process::exit(1);
        }
    
        let variable_name = self.current_token.clone();
        self.advance();
    
        self.match_token("#OIC");
    
        match self.resolve_variable(&variable_name) {
            Some(value) => {
                self.html_output.push_str(&value);
                self.add_space_before_next_text();
            }
            None => {
                eprintln!(
                    "Static semantic error: variable '{}' was used before it was defined.",
                    variable_name
                );
                std::process::exit(1);
            }
        }
    }

    fn parse_bold(&mut self) {
        self.match_token("BOLD");
    
        self.html_output.push_str("<b>");
    
        if self.current_token.eq_ignore_ascii_case("#GIMMEH") {
            self.match_token("#GIMMEH");
    
            if self.current_token.eq_ignore_ascii_case("VAR") {
                self.parse_variable_use();
            } else {
                eprintln!(
                    "Syntax error: expected VAR inside BOLD, but found '{}'.",
                    self.current_token
                );
                std::process::exit(1);
            }
        } else if !self.current_token.is_empty()
            && !self.current_token.starts_with('#')
        {
            let text = self.current_token.clone();
            self.html_output.push_str(&text);
            self.parse_text();
        } else {
            eprintln!(
                "Syntax error: expected text or variable inside BOLD, but found '{}'.",
                self.current_token
            );
            std::process::exit(1);
        }
    
        self.match_token("#OIC");
    
        self.html_output.push_str("</b>");
        self.add_space_before_next_text();
    }

    fn parse_list(&mut self) {
        self.match_token("LIST");
    
        self.html_output.push_str("<ul>");
    
        if !self.current_token.eq_ignore_ascii_case("#GIMMEH") {
            eprintln!(
                "Syntax error: expected a list item, but found '{}'.",
                self.current_token
            );
            std::process::exit(1);
        }
    
        self.parse_list_items();
    
        self.match_token("#MKAY");
    
        self.html_output.push_str("</ul>");
    }

    fn parse_list_items(&mut self) {
        self.match_token("#GIMMEH");
        self.match_token("ITEM");
    
        self.html_output.push_str("<li>");
    
        self.scopes.push(HashMap::new());
    
        if self.current_token.eq_ignore_ascii_case("#IHAZ") {
            self.parse_variable_define();
        }
    
        self.parse_inner_list();
    
        self.match_token("#OIC");
    
        self.scopes.pop();
    
        self.html_output.push_str("</li>");
    
        if self.current_token.eq_ignore_ascii_case("#GIMMEH") {
            self.parse_list_items();
        }
    }

    fn parse_inner_list(&mut self) {
        if self.current_token.is_empty()
            || self.current_token.eq_ignore_ascii_case("#OIC")
        {
            return;
        }
    
        if self.current_token.eq_ignore_ascii_case("#GIMMEH") {
            self.match_token("#GIMMEH");
    
            if self.current_token.eq_ignore_ascii_case("BOLD") {
                self.parse_bold();
            } else if self.current_token.eq_ignore_ascii_case("LINX") {
                self.parse_link();
            } else if self.current_token.eq_ignore_ascii_case("VAR") {
                self.parse_variable_use();
            } else {
                eprintln!(
                    "Syntax error: expected BOLD, LINX, or VAR inside list item, but found '{}'.",
                    self.current_token
                );
                std::process::exit(1);
            }
        } else if self.current_token.eq_ignore_ascii_case("#OBTW") {
            self.parse_comment();
        } else if self.current_token.eq_ignore_ascii_case("#NEWLINE") {
            self.parse_newline();
        } else if !self.current_token.starts_with('#') {
            let text = self.current_token.clone();
            self.html_output.push_str(&text);
            self.html_output.push(' ');
        
            self.parse_text();
        } else {
            eprintln!(
                "Syntax error: unexpected token '{}' inside list item.",
                self.current_token
            );
            std::process::exit(1);
        }
    
        self.parse_inner_list();
    }

    fn parse_link(&mut self) {
        self.match_token("LINX");
    
        if self.current_token.is_empty()
            || self.current_token.starts_with('#')
            || self.current_token.contains(char::is_whitespace)
        {
            eprintln!(
                "Syntax error: expected an address without spaces, but found '{}'.",
                self.current_token
            );
            std::process::exit(1);
        }
    
        let address = self.current_token.clone();
    
        self.advance();
        self.match_token("#OIC");
    
        self.html_output.push_str("<a href=\"");
        self.html_output.push_str(&address);
        self.html_output.push_str("\">");
        self.html_output.push_str(&address);
        self.html_output.push_str("</a>");
        self.add_space_before_next_text();
    }

    fn parse_newline(&mut self) {
        self.match_token("#NEWLINE");
        self.html_output.push_str("<br>");
    }

    fn parse_text(&mut self) {
        if self.current_token.is_empty()
            || self.current_token.starts_with('#')
        {
            eprintln!(
                "Syntax error: expected text, but found '{}'.",
                self.current_token
            );
            std::process::exit(1);
        }
    
        self.advance();
    }
}