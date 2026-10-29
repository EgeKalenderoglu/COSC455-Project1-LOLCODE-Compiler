use crate::lexer::Lexer;
use crate::traits::{Compiler, SyntaxAnalyzer};
use std::collections::HashMap;
use std::fs;
// Stores the lexer, current token, variable scopes, and generated HTML
// while the LOLCODE source file is being compiled.
pub struct LolCompiler {
    lexer: Lexer,
    current_token: String,
    scopes: Vec<HashMap<String, String>>,
    html_output: String,
}

impl LolCompiler {
    // Writes the completed HTML output to a file.
    // If the file cannot be written, the compiler prints an error and stops.
    pub fn save_html(&self, filename: &str) {
        if let Err(error) = fs::write(filename, &self.html_output) {
            eprintln!("Error writing HTML file: {}", error);
            std::process::exit(1);
        }
    }
    // Adds a space when the next token is normal text so generated HTML
    // and variable values do not run directly into the next word.
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
    // Creates a new compiler with an empty lexer, no active scopes,
    // no current token, and no generated HTML.
    pub fn new() -> LolCompiler {
        LolCompiler {
            lexer: Lexer::new(""),
            current_token: String::new(),
            scopes: Vec::new(),
            html_output: String::new(),
        }
    }

    // Gets the next token from the lexer and makes it the current token
    // used by the parser.
    fn advance(&mut self) {
        let next = self.next_token();
        self.set_current_token(next);
    }
    // Searches for a variable starting in the innermost scope and then
    // moves outward. This allows local variables to shadow outer variables.
    fn resolve_variable(&self, name: &str) -> Option<String> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Some(value.clone());
            }
        }
    
        None
    }
    // Checks that the current token matches what the grammar expects.
    // If it matches, parsing continues; otherwise a syntax error is reported.
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
// Implements the required Compiler trait and connects the lexer
// with the recursive-descent parser.
impl Compiler for LolCompiler {
    // Resets the compiler for a new source file, creates the global
    // variable scope, and loads the first token.
    fn compile(&mut self, source: &str) {
        self.lexer = Lexer::new(source);
        self.scopes.clear();
        self.scopes.push(HashMap::new());
        self.html_output.clear();
        self.current_token = self.next_token();
    }
    // Requests and returns the next token from the lexical analyzer.
    fn next_token(&mut self) -> String {
        self.lexer.next_token()
    }
    // Starts syntax analysis using the top-level LOLCODE grammar rule.
    fn parse(&mut self) {
        self.parse_lolcode();
    }
    // Returns a copy of the token currently being examined by the parser.
    fn current_token(&self) -> String {
        self.current_token.clone()
    }
    // Changes the parser's current token.
    fn set_current_token(&mut self, tok: String) {
        self.current_token = tok;
    }
}
// Implements the recursive-descent parser. Each parse function
// corresponds to a part of the LOLCODE grammar.
impl SyntaxAnalyzer for LolCompiler {
    // Parses the complete LOLCODE program from #HAI through #KBYE
    // and creates the outer HTML structure.
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
    // Parses the optional HEAD section and its title.
    fn parse_head(&mut self) {
        self.match_token("HEAD");
    
        self.html_output.push_str("<head>");
    
        self.parse_title();
    
        self.match_token("#MKAY");
    
        self.html_output.push_str("</head>");
    }
    // Parses a TITLE annotation and converts the title text to an HTML title.
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
    // Parses an OBTW/TLDR comment and converts it to an HTML comment.
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
    // Parses the main body by checking the current token and deciding
    // which type of LOLCODE construct should be parsed next.
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
    // Parses a paragraph and creates a new local variable scope.
    // The scope is removed when the paragraph ends.
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
    // Continues parsing paragraph contents until the closing #MKAY is reached.
    fn parse_inner_paragraph(&mut self) {
        if self.current_token.is_empty()
            || self.current_token.eq_ignore_ascii_case("#MKAY")
        {
            return;
        }
    
        self.parse_inner_text();
        self.parse_inner_paragraph();
    }
    // Determines what type of content appears inside a paragraph,
    // such as text, variables, bold text, links, lists, or comments.
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
    // Parses a variable definition and stores its name and value
    // in the current active scope.
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
    // Parses a variable use and searches the active scopes for its value.
    // Using an undefined variable causes a static semantic error.  
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
    // Parses bold text or a variable inside BOLD and generates
    // the corresponding HTML <b> element.
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
    // Parses a LOLCODE list and generates an HTML unordered list.
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
    // Parses one list item and gives the item its own local variable scope.
    // Additional list items are parsed recursively.
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
    // Parses the valid content inside a list item until its closing #OIC.
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
    // Parses a link address and generates an HTML anchor element
    // using the address as both the URL and displayed text.
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
    // Converts the LOLCODE #NEWLINE annotation into an HTML line break.
    fn parse_newline(&mut self) {
        self.match_token("#NEWLINE");
        self.html_output.push_str("<br>");
    }
    // Consumes a normal text token and advances the parser to the next token.
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