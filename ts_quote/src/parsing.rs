use std::fmt::format;
use std::iter::Peekable;

use proc_macro2::*;
use quote::*;
use syn::Result;
use syn::{parse_str, Lit};

use std::collections::HashSet;

struct ExprSubstitution {
    name: String,
    expr: TokenStream,
}

impl ToTokens for ExprSubstitution {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = Ident::new(self.name.as_str(), Span::call_site());
        let expr = &self.expr;
        tokens.extend(quote! {
            #name = #expr
        })
    }
}

struct ParseContext {
    id: String,
    subgroup_index: u32,
    token_iter: Peekable<token_stream::IntoIter>,
    string_val: String,
    ident_substituions: HashSet<Ident>,
    expr_substituions: Vec<ExprSubstitution>,
}

pub fn ts_str_impl(input: TokenStream) -> Result<TokenStream> {
    let mut parse_context = ParseContext::new("0".to_string(), input);
    parse_context.parse();

    // let raw_string = format!(r##"r#"{}"#"##, parse_context.string_val);
    let raw_string = &parse_context.string_val;
    let substitution_mappings = parse_context.substitution_mappings();

    println!("Raw string: {}", raw_string);

    Ok(quote! {
        format!(#raw_string, #substitution_mappings)
    })
}

impl ParseContext {
    fn new(id: String, tokens: TokenStream) -> Self {
        Self {
            id,
            subgroup_index: 0,
            token_iter: tokens.into_iter().peekable(),
            string_val: String::new(),
            ident_substituions: HashSet::new(),
            expr_substituions: vec![],
        }
    }

    fn substitution_mappings(&self) -> TokenStream {
        let subsitutions = &self.expr_substituions;
        quote! {
            #(#subsitutions,)*
        }
    }

    fn create_expr_substitution(&mut self, expr: TokenStream) -> String {
        let name = format!("_expr_sub_{}_{}", self.id, self.expr_substituions.len());
        self.expr_substituions.push(ExprSubstitution {
            name: name.clone(),
            expr,
        });
        name
    }

    fn merge_args(&mut self, other: ParseContext) {
        self.ident_substituions.extend(other.ident_substituions);
        self.expr_substituions.extend(other.expr_substituions);
    }

    fn parse(&mut self) {
        while self.consume_next() {}
    }

    fn push_back_str(&mut self, s: &str) {
        self.string_val.push_str(format!("{}", s).as_str());
    }

    fn push_back_token(&mut self, token: TokenTree) {
        self.string_val.push_str(format!("{}", token).as_str());
    }

    fn push_back_literal(&mut self, lit: Literal) {
        let Ok(Lit::Str(_)) = parse_str::<Lit>(lit.to_string().as_str()) else {
            self.string_val.push_str(lit.to_string().as_str());
            return;
        };
        let str = lit.to_string();
        let str = str.replace("{", "{{").replace("}", "}}");
        self.string_val.push_str(str.as_str());
    }

    fn next(&mut self) -> Option<TokenTree> {
        self.token_iter.next()
    }

    fn peek_next(&mut self) -> Option<&TokenTree> {
        self.token_iter.peek()
    }

    fn consume_next(&mut self) -> bool {
        let Some(token) = self.next() else {
            return false;
        };
        match token {
            TokenTree::Group(group) => self.consume_group(group),
            TokenTree::Punct(punct) => self.consume_punct(punct),
            TokenTree::Literal(lit) => self.push_back_literal(lit),
            token => self.push_back_token(token),
        };
        self.push_back_str(" ");
        true
    }

    fn create_subcontext(&mut self, tokens: TokenStream) -> ParseContext {
        let context = ParseContext::new(format!("{}{}", self.id, self.subgroup_index), tokens);
        self.subgroup_index += 1;
        context
    }

    fn consume_group(&mut self, group: Group) {
        let inner = group.stream();

        let mut contens = self.create_subcontext(inner);
        contens.parse();

        match group.delimiter() {
            Delimiter::Parenthesis => {
                self.push_back_str(format!("({})", contens.string_val).as_str())
            }
            Delimiter::Brace => {
                self.push_back_str(format!("{{{{ {} }}}}", contens.string_val).as_str())
            }
            Delimiter::Bracket => self.push_back_str(format!("[{}]", contens.string_val).as_str()),
            Delimiter::None => self.push_back_str(contens.string_val.as_str()),
        }

        self.merge_args(contens);
    }

    fn consume_punct(&mut self, punct: Punct) {
        match punct.as_char() {
            '#' => self.consume_hash_substitution(punct),
            _ => self.consume_punct_series(punct),
        }
    }

    fn consume_punct_series(&mut self, inital: Punct) {
        let mut result: String = format!("{}", inital.as_char());
        let mut current = inital;
        while let Spacing::Joint = current.spacing() {
            if let Some(TokenTree::Punct(punct)) = self.peek_next() {
                if punct.as_char() == '#' {
                    break;
                }
            }

            let Some(TokenTree::Punct(punct)) = self.next() else {
                panic!("Spacing::Joint should guarantee a Punct here");
            };
            result.push(punct.as_char());
            current = punct;
        }
        self.push_back_str(result.as_str());
    }

    fn consume_hash_substitution(&mut self, hash: Punct) {
        let Some(token) = self.peek_next() else {
            self.consume_punct_series(hash);
            return;
        };
        let token = token.clone();
        match token {
            TokenTree::Group(group) => {
                let _ = self.next();
                self.consume_group_hash_substitution(group)
            }
            TokenTree::Ident(ident) => {
                let _ = self.next();
                self.consume_ident_hash_substitution(ident)
            }
            TokenTree::Literal(lit) => self.consume_literal_hash_substitution(lit),
            _ => self.consume_punct_series(hash),
        }
    }

    fn consume_group_hash_substitution(&mut self, group: Group) {
        let name = self.create_expr_substitution(group.stream());
        self.push_back_str(format!("{{{}}}", name).as_str());
    }

    fn consume_ident_hash_substitution(&mut self, ident: Ident) {
        self.push_back_str(format!("{{{}}}", ident).as_str());
        self.ident_substituions.insert(ident.clone());
    }

    fn consume_literal_hash_substitution(&mut self, lit: Literal) {
        // Parse a raw string literal
        let Ok(Lit::Str(lit_str)) = parse_str::<Lit>(lit.to_string().as_str()) else {
            return;
        };
        // Consume the string literal
        let consumed = self.next();
        println!("Consumed string literal: {:?}", consumed);
        let literal = lit_str.value();

        let mut components: Vec<&str> = literal.split('#').collect();

        if let Some(first) = components.first() {
            if !first.is_empty() {
                self.push_back_literal_component(first.to_string());
            }
            components.remove(0);
        }

        while !components.is_empty() {
            let lit = components.remove(0);
            self.consume_hashed_literal_component(lit.to_string())
        }

        // let literal = literal.replace("{", "{{");
        // let literal = literal.replace("}", "}}");
        // self.push_back_str(literal.as_str());
    }

    fn push_back_literal_component(&mut self, lit: String) {
        let lit = lit.replace("{", "{{").replace("}", "}}");
        self.push_back_str(lit.as_str());
    }

    fn consume_hashed_literal_component(&mut self, lit: String) {
        // If there are no characters in the literal string, we just
        // push back the # character
        let Some(first) = lit.chars().next() else {
            self.push_back_str("#");
            return;
        };

        let lit = match first {
            '{' => self.consume_hashed_literal_group(Delimiter::Brace, lit),
            '[' => self.consume_hashed_literal_group(Delimiter::Bracket, lit),
            '(' => self.consume_hashed_literal_group(Delimiter::Parenthesis, lit),
            c if is_ident_start_character(c) => self.consume_hashed_ident_group(lit),
            _ => lit,
        };

        let lit = lit.replace("{", "{{").replace("}", "}}");
        println!("pushing back hased literal: {}", lit);
        self.push_back_str(lit.as_str());
    }

    fn consume_hashed_ident_group(&mut self, lit: String) -> String {
        println!("consume_hashed_ident_group");
        for (index, char) in lit.chars().enumerate() {
            if is_ident_character(char) {
                continue;
            }
            let Some((first, second)) = split_at_index(lit.as_str(), index) else {
                // TODO: should this be an error?
                self.push_back_str("#");
                return lit;
            };
            let ident = match parse_str::<Ident>(first) {
                Ok(ident) => ident,
                Err(err) => {
                    // TODO: should this error propogate?
                    eprintln!("Error parsing Ident: {:?}", err);
                    self.push_back_str("#");
                    return lit;
                }
            };
            self.consume_ident_hash_substitution(ident);
            return second.to_string();
        }
        match parse_str::<Ident>(lit.as_str()) {
            Ok(ident) => {
                self.consume_ident_hash_substitution(ident);
                return String::new();
            }
            Err(_) => {
                self.push_back_str("#");
            }
        };
        lit
    }

    fn consume_hashed_literal_group(&mut self, delimiter: Delimiter, lit: String) -> String {
        println!("consume_hashed_literal_group");
        let mut depth: u32 = 0;
        for (index, char) in lit.chars().enumerate() {
            match delimiter.matches(char) {
                Some(DelimiterType::Open) => depth += 1,
                Some(DelimiterType::Close) => depth -= 1,
                None => {}
            }
            if depth == 0 {
                let Some((first, second)) = split_at_index(lit.as_str(), index + 1) else {
                    // TODO: should this be an error?
                    self.push_back_str("#");
                    return lit;
                };

                let group = match parse_str::<Group>(first) {
                    Ok(group) => group,
                    Err(err) => {
                        // TODO: should this error propogate?
                        eprintln!("Error parsing group: {:?}", err);
                        self.push_back_str("#");
                        return lit;
                    }
                };

                self.consume_group_hash_substitution(group);
                return second.to_string();
            }
        }
        lit
    }
}

enum DelimiterType {
    Open,
    Close,
}

trait MatchesDelimiter {
    fn matches(&self, c: char) -> Option<DelimiterType>;
}

impl MatchesDelimiter for Delimiter {
    fn matches(&self, c: char) -> Option<DelimiterType> {
        match self {
            Delimiter::Parenthesis => match c {
                '(' => Some(DelimiterType::Open),
                ')' => Some(DelimiterType::Close),
                _ => None,
            },
            Delimiter::Brace => match c {
                '{' => Some(DelimiterType::Open),
                '}' => Some(DelimiterType::Close),
                _ => None,
            },
            Delimiter::Bracket => match c {
                '[' => Some(DelimiterType::Open),
                ']' => Some(DelimiterType::Close),
                _ => None,
            },
            Delimiter::None => None,
        }
    }
}

fn split_at_index(s: &str, index: usize) -> Option<(&str, &str)> {
    if index > s.len() || !s.is_char_boundary(index) {
        // Index out of bounds or not at a character boundary
        return None;
    }

    let (first_part, second_part) = s.split_at(index);
    Some((first_part, second_part))
}

fn is_ident_character(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn is_ident_start_character(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}
