#![allow(unused_macros)]
#![allow(dead_code)]
macro_rules! try_parser {
    { $f:ident($input:expr) } => {{
        let fork = &$input.fork();
        match $f(fork) {
            Ok(v) => { $input.advance_to(fork); Some(v) },
            Err(_) => None
        }
    }};
    { $input:expr; . $($t:tt)+ } => {{
        let fork = &$input.fork();
        match fork . $($t)+ {
            Ok(v) => { $input.advance_to(fork); Some(v) },
            Err(_) => None
        }
    }};
    { $input:expr; $($t:tt)+ } => {{
        let fork = &$input.fork();
        match fork.parse::<$($t)+>() {
            Ok(v) => { $input.advance_to(fork); Some(v) },
            Err(_) => None
        }
    }};
}
macro_rules! peek_parser {
    { $f:ident($input:expr) } => {{
        let fork = &$input.fork();
        match $f(fork) {
            Ok(v) => Some(v),
            Err(_) => None
        }
    }};
    { $input:expr; . $($t:tt)+ } => {{
        let fork = &$input.fork();
        match fork . $($t)+ {
            Ok(v) => Some(v),
            Err(_) => None
        }
    }};
    { $input:expr; $($t:tt)+ } => {{
        let fork = &$input.fork();
        match fork.parse::<$($t)+>() {
            Ok(v) => Some(v),
            Err(_) => None
        }
    }};
}
macro_rules! parse_char {
    { $input:expr; $($t:tt)* } => {
        $(
            $input.parse::<Token![$t]>()?;
        )*
    };
}

use syn::{
    parse::{ParseStream, Result},
    Attribute, Token,
};

#[inline]
pub fn parse_attrs(input: ParseStream) -> Result<Vec<Attribute>> {
    input.call(Attribute::parse_outer)
}
/// `---`
#[inline]
pub fn parse_split(input: ParseStream) -> Result<()> {
    parse_char![input;---];
    Ok(())
}
/// `;`
#[inline]
pub fn parse_end(input: ParseStream) -> Result<()> {
    parse_char![input;;];
    Ok(())
}
/// `,`
#[inline]
pub fn parse_comma(input: ParseStream) -> Result<()> {
    parse_char![input;,];
    Ok(())
}
/// `=`
#[inline]
pub fn parse_eq(input: ParseStream) -> Result<()> {
    parse_char![input;=];
    Ok(())
}

#[inline]
pub fn uppercase_first_letter(s: &String) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
