use std::char;

pub struct Apple{
    symbol: char
}

impl Apple{
    pub(crate) fn new(symbol: char) -> Self {
        Self {
            symbol,
        }
    }

}
