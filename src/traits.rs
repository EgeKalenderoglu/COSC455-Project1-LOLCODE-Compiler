// Defines the main operations required for the LOLCODE compiler.
// The implementation connects lexical analysis and syntax analysis.
pub trait Compiler {
    // Starts compilation using the provided source code.
    fn compile(&mut self, source: &str);
    // Gets the next token from the lexical analyzer.
    fn next_token(&mut self) -> String;
    // Starts the syntax analyzer at the top-level LOLCODE rule.
    fn parse(&mut self);
    // Returns the token currently being processed.
    fn current_token(&self) -> String;
    // Changes the token currently being processed
    fn set_current_token(&mut self, tok: String);
}

// Defines the required character-by-character lexical analyzer operations.
pub trait LexicalAnalyzer {
    // Gets the next character from the source input.
    fn get_char(&mut self) -> char;
    // Adds a character to the lexeme currently being built.
    fn add_char(&mut self, c: char);
    // Checks whether a token or keyword is valid in the LOLCODE language.
    fn lookup(&self, s: &str) -> bool;
}

// Defines the recursive-descent syntax analyzer.
// Each function represents a nonterminal in the LOLCODE grammar.
pub trait SyntaxAnalyzer {
    // Parses the complete LOLCODE program.
    fn parse_lolcode(&mut self);
    // Parses the HEAD section.
    fn parse_head(&mut self);
    // Parses the TITLE section.
    fn parse_title(&mut self);
    // Parses a comment.
    fn parse_comment(&mut self);
    // Parses the main program body.
    fn parse_body(&mut self);
    // Parses a paragraph.
    fn parse_paragraph(&mut self);
    // Parses the contents inside a paragraph.
    fn parse_inner_paragraph(&mut self);
    // Parses one valid type of content inside a paragraph.
    fn parse_inner_text(&mut self);
    // Parses a variable definition.
    fn parse_variable_define(&mut self);
    // Parses a variable use.
    fn parse_variable_use(&mut self);
    // Parses bold content.
    fn parse_bold(&mut self);
    // Parses a list.
    fn parse_list(&mut self);
    // Parses the items inside a list.
    fn parse_list_items(&mut self);
    // Parses the contents inside a list item.
    fn parse_inner_list(&mut self);
    // Parses a hyperlink.
    fn parse_link(&mut self);
    // Parses a newline annotation.
    fn parse_newline(&mut self);
    // Parses normal text.
    fn parse_text(&mut self);
}