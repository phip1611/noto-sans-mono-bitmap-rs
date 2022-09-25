/// Return type for the [`UnicodeIter`] iterator.
#[derive(Debug)]
pub enum UnicodeSymbol {
    // Regular chars/symbols.
    Char(char),
    // for example U+00AD (soft hyphen)
    NonDisplayableSymbol(char),
    // Control sequences such as ESC and DEL.
    Control,
}

impl UnicodeSymbol {
    pub const fn is_visible_char(&self) -> bool {
        matches!(self, Self::Char(_))
    }

    pub fn get_char(&self) -> char {
        match self {
            UnicodeSymbol::Char(c) => *c,
            _ => panic!("not a char"),
        }
    }
}

/// Iterator over all valid unicode symbols including
/// BASIC LATIN, LATIN 1 Supplement, and LATIN EXTENDED-A.
/// Returns items of type [`UnicodeSymbol`].
#[derive(Debug)]
pub struct UnicodeIter {
    counter: u32,
    additional_symbols_counter: usize,
}

impl UnicodeIter {
    /// Inclusive unicode limit. Iterates from 0 to this limit through unicode symbols.
    const LIMIT: u32 = 0x17f;
    const ADDITIONAL_SYMBOLS: &'static [u32] = &[0x25A0, 0xFFFD];

    pub const fn new() -> Self {
        Self {
            counter: 0,
            additional_symbols_counter: 0,
        }
    }
}

impl Iterator for UnicodeIter {
    type Item = UnicodeSymbol;

    fn next(&mut self) -> Option<Self::Item> {
        if self.additional_symbols_counter >= UnicodeIter::ADDITIONAL_SYMBOLS.len() {
            None
        } else if self.counter <= Self::LIMIT {
            // https://unicode-table.com/en/blocks/basic-latin/
            let symbol = match self.counter {
                0..=0x1f => UnicodeSymbol::Control,
                0x20..=0x7e => UnicodeSymbol::Char(char::from_u32(self.counter).unwrap()),
                0x7f => UnicodeSymbol::Control,
                0x80..=0x9f => UnicodeSymbol::Control,
                0xa0..=0xac => UnicodeSymbol::Char(char::from_u32(self.counter).unwrap()),
                0xad => UnicodeSymbol::NonDisplayableSymbol(char::from_u32(self.counter).unwrap()),
                0xae..=Self::LIMIT => UnicodeSymbol::Char(char::from_u32(self.counter).unwrap()),
                _ => panic!("Out of range"),
            };

            self.counter += 1;

            Some(symbol)
        } else {
            let symbol = UnicodeSymbol::Char(
                char::from_u32(UnicodeIter::ADDITIONAL_SYMBOLS[self.additional_symbols_counter])
                    .unwrap(),
            );
            self.additional_symbols_counter += 1;
            Some(symbol)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::unicode::UnicodeIter;

    #[test]
    fn print_all_unicode_chars() {
        for symbol in UnicodeIter::new() {
            println!("{:?}", symbol);
        }
    }
}
