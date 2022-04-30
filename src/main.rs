use std::env::args;
use std::{fs, fmt};
use std::io::{stdin, Read};
use proc_macro2::{TokenStream, TokenTree, Delimiter};

fn main() {
    match args().nth(1).as_deref().map(str::trim) {
        Some("-") | None => simplify({
            let mut s = String::new();
            stdin().read_to_string(&mut s).expect("reading stdin");
            s
        }),
        Some(path) => simplify(fs::read_to_string(path).expect("path not readable")),
    }
}

fn simplify(s: String) {
    println!("{}", MinTs(s.parse().expect("parsing tokens")));
}

struct MinTs(TokenStream);

impl fmt::Display for MinTs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut it = self.0.clone().into_iter().peekable();
        while let Some(tt) = it.next() {
            match tt {
                TokenTree::Group(g) => {
                    match g.delimiter() {
                        Delimiter::Brace => write!(f, "{{{}}}", MinTs(g.stream()))?,
                        Delimiter::Bracket => write!(f, "[{}]", MinTs(g.stream()))?,
                        Delimiter::None => write!(f, "{}", MinTs(g.stream()))?,
                        Delimiter::Parenthesis => write!(f, "({})", MinTs(g.stream()))?,
                    }
                }
                TokenTree::Ident(i) => {
                    write!(f, "{i}")?;
                    if let Some(TokenTree::Ident(_) | TokenTree::Literal(_)) = it.peek() {
                        f.pad(" ")?;
                    }
                }
                TokenTree::Punct(p) => write!(f, "{p}")?,
                TokenTree::Literal(lit) => write!(f, "{lit}")?,
            }
        }
        Ok(())
    }
}
