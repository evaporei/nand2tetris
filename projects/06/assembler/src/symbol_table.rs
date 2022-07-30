use std::collections::BTreeMap;
use std::str::Chars;

pub type Address = u16;

pub type SymbolTable = BTreeMap<String, Address>;

pub struct Symbol {
    idx: u16,
    s: String,
}

impl Symbol {
    pub fn new(idx: usize, chars: Chars) -> Option<Self> {
        let idx = idx as u16;
        let maybe_symbol: String = chars.skip(1).collect();

        if maybe_symbol.parse::<u16>().is_err() {
            Some(Symbol {
                idx,
                s: maybe_symbol,
            })
        } else {
            None
        }
    }

    pub fn to_tuple(self) -> (String, Address) {
        (self.s, self.idx)
    }
}
