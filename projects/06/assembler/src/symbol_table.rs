use std::collections::BTreeMap;

pub type Address = u16;

#[derive(Debug, PartialEq)]
pub enum SymbolKind {
    Variable,
    Label,
}

#[derive(Debug, PartialEq)]
pub struct Symbol {
    pub idx: u16,
    pub s: String,
    pub k: SymbolKind,
}

impl Symbol {
    pub fn variable(idx: usize, s: String) -> Option<Self> {
        let idx = idx as u16;

        // dbg!("variable", &s, idx);

        if s.parse::<u16>().is_err() {
            Some(Symbol {
                idx,
                s,
                k: SymbolKind::Variable,
            })
        } else {
            None
        }
    }

    pub fn label(idx: usize, mut s: String) -> Option<Self> {
        let idx = idx as u16;

        s.remove(s.len() - 1); // closing paren )

        // dbg!("label", &s, idx);

        Some(Symbol {
            idx,
            s: s.to_owned(),
            k: SymbolKind::Label,
        })
    }
}

#[derive(Default, Debug)]
pub struct SymbolTable {
    map: BTreeMap<String, Address>,
    next_address: u16,
}

impl SymbolTable {
    pub fn with_predefined() -> Self {
        let mut map = BTreeMap::new();

        map.insert("SP".into(), 0);
        map.insert("LCL".into(), 1);
        map.insert("ARG".into(), 2);
        map.insert("THIS".into(), 3);
        map.insert("THAT".into(), 4);

        map.insert("R0".into(), 0);
        map.insert("R1".into(), 1);
        map.insert("R2".into(), 2);
        map.insert("R3".into(), 3);
        map.insert("R4".into(), 4);
        map.insert("R5".into(), 5);
        map.insert("R6".into(), 6);
        map.insert("R7".into(), 7);
        map.insert("R8".into(), 8);
        map.insert("R9".into(), 9);
        map.insert("R10".into(), 10);
        map.insert("R11".into(), 11);
        map.insert("R12".into(), 12);
        map.insert("R13".into(), 13);
        map.insert("R14".into(), 14);
        map.insert("R15".into(), 15);

        map.insert("SCREEN".into(), 16384);
        map.insert("KBD".into(), 24576);

        Self {
            map,
            next_address: 16,
        }
    }

    pub fn insert(&mut self, symbol: Symbol) {
        match symbol.k {
            SymbolKind::Variable => {
                // Here we only add the first occurence of the variable.
                // It will not be overwritten.
                self.map.entry(symbol.s).or_insert_with(|| {
                    // We only update the `next_address` if a new variable
                    // was inserted.
                    let addr = self.next_address;
                    self.next_address += 1;
                    addr
                });
            }
            SymbolKind::Label => {
                self.map.insert(symbol.s, symbol.idx);
            }
        }
    }

    pub fn get(&self, symbol_s: &String) -> Option<Address> {
        self.map.get(symbol_s).copied()
    }
}
