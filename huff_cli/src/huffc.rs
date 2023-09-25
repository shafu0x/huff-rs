use huff_lexer::Lexer;
use huff_utils::prelude::*;
use std::ops::Deref;

fn main() {
    // Instantiate a new lexer
    let source = r#"#define macro HELLO_WORLD()
        #define"#;
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let mut lexer = Lexer::new(flattened_source.source);

    while let Some(tok) = lexer.next() {
        println!("Token: {:?}", tok.unwrap());
    }
}
