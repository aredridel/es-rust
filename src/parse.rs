use es::Es;
use list::List;
use term::Term;
use list::List::{Cell, Nil};
use term::Term::{Prim, Str};
use combine::{ParseResult, Parser, ParserExt, any, many1, not_followed_by, parser, string, token};
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
    (parser(first)).map(|_| Nil).parse_state(input)
}

fn first<I>(input: State<I>) -> ParseResult<List, I>
    where I: Stream<Item = char>
{
    parser(comword).parse_state(input).map(|(a, b)| (Cell(a), b))
}

fn comword<I>(input: State<I>) -> ParseResult<Term, I>
    where I: Stream<Item = char>
{
    not_followed_by(string("for").or(string("local")).or(string("let")).or(string("fn")))
        .with(many1(any()))
        .map(|t| Str(t))
        .parse_state(input)
}

fn cmdsa<I>(input: State<I>) -> ParseResult<List, I>
    where I: Stream<Item = char>
{
    parser(cmd::<I>).skip(token('&').or(token(';'))).parse_state(input)
}
