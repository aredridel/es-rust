use es::Es;
use list::List;
use list::List::Cell;
use term::Term::Str;
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
        .map(|e| e.0.to_string() + &e.2)
        .or(parser(comword))
        .parse_state(input)
        .map(|(a, b)| (Cell(Str(a)), b))
}

fn comword<I>(input: State<I>) -> ParseResult<String, I>
    where I: Stream<Item = char>
{
    not_followed_by(string("for").or(string("local")).or(string("let")).or(string("fn")))
        .with(parser(param).or((token('$'), token('&'), many1::<Vec<_>, _>(alpha_num())).map(|e| "blah".to_string())))
        .parse_lazy(input)
}

fn param<I>(input: State<I>) -> ParseResult<String, I>
    where I: Stream<Item = char>
{
    many1(alpha_num()).parse_state(input)
}


fn cmdsa<I>(input: State<I>) -> ParseResult<List, I>
    where I: Stream<Item = char>
{
    parser(cmd::<I>).skip(token('&').or(token(';'))).parse_state(input)
}

fn word<I>(input: State<I>) -> ParseResult<String, I>
    where I: Stream<Item = char>
{
    (parser(sword), token('^'), parser(word))
        .map(|e| e.0.to_string() + &e.2)
        .or(parser(sword))
        .parse_state(input)
}

fn sword<I>(input: State<I>) -> ParseResult<String, I>
    where I: Stream<Item = char>
{
    parser(comword).or(parser(keyword)).parse_state(input)
}

fn keyword<I>(input: State<I>) -> ParseResult<String, I>
    where I: Stream<Item = char>
{
    // FIXME
    not_followed_by(string("for").or(string("local")).or(string("let")).or(string("fn")))
        .with(many1(any()))
        .parse_state(input)
}
