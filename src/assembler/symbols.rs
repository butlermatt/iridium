
#[derive(Debug, PartialEq)]
pub enum SymbolType {
    Label,
    IrString
}

#[derive(Debug)]
pub struct Symbol {
    name: String,
    symbol_type: SymbolType,
    offset: Option<u32>,
}

impl Symbol {
    pub fn new(name: String, symbol_type: SymbolType) -> Self {
        Symbol {
            name,
            symbol_type,
            offset: None,
        }
    }

    pub fn new_with_offset(name: String, symbol_type: SymbolType, offset: u32) -> Self {
        Symbol {
            name,
            symbol_type,
            offset: Some(offset)
        }
    }
}

#[derive(Debug)]
pub struct SymbolTable {
    symbols: Vec<Symbol>
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            symbols: Vec::new()
        }
    }

    pub fn add_symbol(&mut self, s: Symbol) {
        self.symbols.push(s);
    }

    pub fn has_symbol(&mut self, s: &str) -> bool {
        for symbol in &self.symbols {
            if symbol.name == s { return true; }
        }
        false
    }

    pub fn set_symbol_offset(&mut self, s: &str, offset: u32) -> bool {
        for symbol in &mut self.symbols {
            if symbol.name == s {
                symbol.offset = Some(offset);
                return true;
            }
        }
        false
    }

    pub fn symbol_value(&self, s: &str) -> Option<u32> {
        for symbol in &self.symbols {
            if symbol.name == s {
                return symbol.offset;
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_table() {
        let mut sym = SymbolTable::new();
        let new_symbol = Symbol::new_with_offset("test".to_string(), SymbolType::Label, 12);
        sym.add_symbol(new_symbol);
        assert_eq!(sym.symbols.len(), 1);
        let v = sym.symbol_value("test");
        assert!(v.is_some());
        let v = v.unwrap();
        assert_eq!(v, 12);
        let v = sym.symbol_value("does_not_exist");
        assert!(v.is_none());
    }
}