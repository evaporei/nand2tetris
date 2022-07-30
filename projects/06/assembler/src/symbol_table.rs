use std::collections::BTreeMap;
use std::str::Chars;

pub type Address = u16;

pub type SymbolTable = BTreeMap<String, Address>;

pub struct Symbol {
    pub idx: u16,
    pub s: String,
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
}

pub fn with_predefined() -> SymbolTable {
    let mut st = SymbolTable::new();

    st.insert("SP".into(), 0);
    st.insert("LCL".into(), 1);
    st.insert("ARG".into(), 2);
    st.insert("THIS".into(), 3);
    st.insert("THAT".into(), 4);

    st.insert("R0".into(), 0);
    st.insert("R1".into(), 1);
    st.insert("R2".into(), 2);
    st.insert("R3".into(), 3);
    st.insert("R4".into(), 4);
    st.insert("R5".into(), 5);
    st.insert("R6".into(), 6);
    st.insert("R7".into(), 7);
    st.insert("R8".into(), 8);
    st.insert("R9".into(), 9);
    st.insert("R10".into(), 10);
    st.insert("R11".into(), 11);
    st.insert("R12".into(), 12);
    st.insert("R13".into(), 13);
    st.insert("R14".into(), 14);
    st.insert("R15".into(), 15);

    st.insert("SCREEN".into(), 16384);
    st.insert("KBD".into(), 24576);

    st
}
