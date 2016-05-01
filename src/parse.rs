
use es::Es;
use list::List;

pub trait Parse {
    fn parse(&self) -> Result<List, &'static str>;
}

impl Parse for Es {
    fn parse(&self) -> Result<List, &'static str> {
        Ok::<List, &'static str>(List::Nil)
    }
}
