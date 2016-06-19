use es::Es;
use list::List;
use list::List::{Cell,Cons};
use term::Term;
use term::Term::{Str,Prim};
use std::rc::Rc;
use combine::{ParseResult, Parser, ParserExt, any, many1, not_followed_by, parser, string, token, alpha_num};
use combine::primitives::{State, Stream};

impl Es {
    pub fn parse(&self, s: &str) -> Result<List, &'static str> {
        match parser(line).parse(s) {
            Ok((l, _)) => Ok(l),
            Err(_) => Err("parse error"),
        }
    }
}

fn line<I>(input: State<I>) -> ParseResult<List, I>
    where I: Stream<Item = char>
{
    parser(cmd::<I>).or(parser(cmdsa::<I>)).parse_state(input)
}

fn cmd<I>(input: State<I>) -> ParseResult<List, I>
    where I: Stream<Item = char>
{
    parser(simple) // .or(parser(redir::<I>))
        .parse_state(input)
}

fn simple<I>(input: State<I>) -> ParseResult<List, I>
    where I: Stream<Item = char>
{
    (parser(first), parser(simple), parser(word)).map(|e| e.0).parse_state(input)
}

fn first<I>(input: State<I>) -> ParseResult<List, I>
    where I: Stream<Item = char>
{
    (parser(comword), token('^'), parser(sword))
        .map(|e| Cons(e.0, Rc::new(Cell(e.2))))
        .or(parser(comword).map(|e| Cell(e)))
        .parse_state(input)
}

fn comword<I>(input: State<I>) -> ParseResult<Term, I>
    where I: Stream<Item = char>
{
    not_followed_by(string("for").or(string("local")).or(string("let")).or(string("fn")))
        .with(parser(param).or((token('$'), token('&'), many1::<Vec<_>, _>(alpha_num())).map(|e| Str("blah".to_string()))))
        .parse_lazy(input)
}

fn param<I>(input: State<I>) -> ParseResult<Term, I>
    where I: Stream<Item = char>
{
    (token('$'), token('&'), many1(alpha_num())).map(|e| Prim(e.2)).parse_state(input)
}


fn cmdsa<I>(input: State<I>) -> ParseResult<List, I>
    where I: Stream<Item = char>
{
    parser(cmd::<I>).skip(token('&').or(token(';'))).parse_state(input)
}

fn word<I>(input: State<I>) -> ParseResult<List, I>
    where I: Stream<Item = char>
{
    (parser(sword), token('^'), parser(word))
        .map(|e| Cons(e.0, Rc::new(e.2)))
        .or(parser(sword).map(|e| Cell(e)))
        .parse_state(input)
}

fn sword<I>(input: State<I>) -> ParseResult<Term, I>
    where I: Stream<Item = char>
{
    parser(comword).or(parser(keyword)).parse_state(input)
}

fn keyword<I>(input: State<I>) -> ParseResult<Term, I>
    where I: Stream<Item = char>
{
    // FIXME
    not_followed_by(string("for").or(string("local")).or(string("let")).or(string("fn")))
        .with(many1(any())).map(|e| Str(e))
        .parse_state(input)
}
