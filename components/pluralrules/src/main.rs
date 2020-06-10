use icu_pluralrules::rules::lexer::Lexer;
use icu_pluralrules::rules::parser::Parser;
use icu_pluralrules::rules::resolver::matches;

pub fn main() {
    let s = "n = 0,1 or i = 0 and f = 1";
    let l = Lexer::new(s.as_bytes());
    for token in l {
        println!("{:#?}", token);
    }

    let p = Parser::new(s.as_bytes());
    let ast = p.parse().unwrap();
    println!("{:#?}", &ast);

    println!("{:#?}", matches(&ast, &2u64.into()));
}
