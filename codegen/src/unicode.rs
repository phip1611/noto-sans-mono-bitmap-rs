/// Return type for the [`UnicodeIter`] iterator.
#[derive(Debug)]
pub enum UnicodeSymbol {
    Char(char),
    Control,
}

impl UnicodeSymbol {
    pub fn is_char(&self) -> bool {
        matches!(self, Self::Char(_))
    }

    pub fn get_char(&self) -> char {
        match self {
            UnicodeSymbol::Char(c) => *c,
            UnicodeSymbol::Control => panic!("not a char")
        }
    }
}

/// Iterator over all valid unicode symbols including
/// BASIC LATIN, LATIN 1 Supplement, and LATIN EXTENDED-A.
/// Returns items of type [`UnicodeSymbol`].
#[derive(Debug)]
pub struct UnicodeIter {
    counter: u32,
}

impl UnicodeIter {
    /// Inclusive unicode limit.
    const LIMIT: u32 = 0x17f;

    pub const fn new() -> Self {
        Self { counter: 0 }
    }
}

impl Iterator for UnicodeIter {
    type Item = (UnicodeSymbol);

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter > Self::LIMIT  {
            return None;
        }

        // https://unicode-table.com/en/blocks/basic-latin/
        let symbol = match self.counter {
            0..=0x1f => UnicodeSymbol::Control,
            0x20..=0x7e => UnicodeSymbol::Char(char::from_u32(self.counter).unwrap()),
            0x7f => UnicodeSymbol::Control,
            0x80..=0x9f => UnicodeSymbol::Control,
            0xa0..=Self::LIMIT => UnicodeSymbol::Char(char::from_u32(self.counter).unwrap()),
            _ => panic!("Out of range"),
        };

        self.counter += 1;

        Some(symbol)
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
