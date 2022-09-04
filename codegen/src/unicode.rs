/// The unicode ranges that this library can generate.
pub const SUPPORTED_UNICODE_RANGES: &[UnicodeRange] = &[
    // ASCII
    UnicodeRange {
        feature_name: "unicode-basic-latin",
        begin: 0,
        end: 0x7f,
        default_feature: true,
    },
    // ÄöüË
    UnicodeRange {
        feature_name: "unicode-latin-1-supplement",
        begin: 0x80,
        end: 0xff,
        default_feature: false,
    },
    /* I'll might add more later step by step when I exactly trace their effect on the whole
       crate size.
    // ĈĻŜ
    UnicodeRange {
        feature_name: "unicode-latin-extended-a",
        begin: 0x100,
        end: 0x17f,
        default_feature: false,
    },
    UnicodeRange {
        feature_name: "unicode-latin-extended-b",
        begin: 0x180,
        end: 0x24f,
        default_feature: false,
    },
    UnicodeRange {
        feature_name: "unicode-currency-symbols",
        begin: 0x20a0,
        end: 0x20cf,
        default_feature: false,
    },
    UnicodeRange {
        feature_name: "unicode-number-forms",
        begin: 0x2150,
        end: 0x218f,
        default_feature: false,
    },
    UnicodeRange {
        feature_name: "unicode-arrows",
        begin: 0x2190,
        end: 0x21ff,
        default_feature: false,
    },
    UnicodeRange {
        feature_name: "unicode-mathematical-operators",
        begin: 0x2200,
        end: 0x22ff,
        default_feature: false,
    },
    UnicodeRange {
        feature_name: "unicode-box-drawing",
        begin: 0x2500,
        end: 0x257f,
        default_feature: false,
    },
    UnicodeRange {
        feature_name: "unicode-block-element",
        begin: 0x2580,
        end: 0x259f,
        default_feature: false,
    },
    UnicodeRange {
        feature_name: "unicode-geometric-shapes",
        begin: 0x25a0,
        end: 0x25ff,
        default_feature: false,
    },*/
];

#[derive(Debug, PartialEq, Eq)]
pub struct UnicodeRange {
    // lower cash case, such as "basic-latin"
    pub feature_name: &'static str,
    // inclusive end
    pub begin: u32,
    // inclusive end
    pub end: u32,
    // whether this unicode range is a default features
    pub default_feature: bool,
}

impl UnicodeRange {
    pub fn iter(&self) -> UnicodeRangeIter {
        UnicodeRangeIter::<'_>::new(self)
    }

    pub fn feature_name(&self) -> &'static str {
        self.feature_name
    }
}

/// Iterator over all characters of a unicode range.
#[derive(Debug)]
pub struct UnicodeRangeIter<'a> {
    range: &'a UnicodeRange,
    // the incrementing counter between range.begin and range.end
    counter: u32,
}

impl<'a> UnicodeRangeIter<'a> {
    fn new(range: &'a UnicodeRange) -> Self {
        Self {
            range,
            counter: range.begin,
        }
    }
}

impl<'a> Iterator for UnicodeRangeIter<'a> {
    type Item = UnicodeSymbol;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter > self.range.end {
            None
        } else {
            // The basic-latin block contains some non-displayable symbols. I take care of them
            // here. Background: https://unicode-table.com/en/blocks/basic-latin/
            let symbol = match self.counter {
                0..=0x1f => UnicodeSymbol::Control,
                0x20..=0x7e => UnicodeSymbol::Char(char::from_u32(self.counter).unwrap()),
                0x7f => UnicodeSymbol::Control,
                0x80..=0x9f => UnicodeSymbol::Control,
                0xa0..=0xac => UnicodeSymbol::Char(char::from_u32(self.counter).unwrap()),
                // soft hyphen
                0xad => UnicodeSymbol::NonDisplayableSymbol(char::from_u32(self.counter).unwrap()),
                0xae..=u32::MAX => UnicodeSymbol::Char(char::from_u32(self.counter).unwrap()),
            };
            self.counter += 1;
            Some(symbol)
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    // this test makes sure that there are no accidentally overlappings in the
    // unicode ranges array SUPPORTED_UNICODE_RANGES
    #[test]
    fn check_ranges_non_overlapping() {
        fn range_overlaps(range_left: &UnicodeRange, range_right: &UnicodeRange) -> bool {
            let right_bigger_than_left = range_right.begin > range_left.end;
            let right_smaller_than_left = range_right.end < range_left.begin;

            !(right_bigger_than_left || right_smaller_than_left)
        }

        for range in SUPPORTED_UNICODE_RANGES {
            assert!(
                range.begin < range.end,
                "range.begin < range.end failed for range {}",
                range.feature_name
            );
            let overlaps_with_any = SUPPORTED_UNICODE_RANGES
                .iter()
                .filter(|r| r.feature_name != range.feature_name)
                .any(|r| {
                    let overlaps = range_overlaps(range, r);
                    if overlaps {
                        println!("{} and {} overlap", range.feature_name, r.feature_name)
                    }
                    overlaps
                });

            assert!(!overlaps_with_any);
        }
    }

    // tests that the iterator starts at the range begin and not always at zero
    #[test]
    fn test_unicode_iter_begin_end() {
        let mut iter1 = SUPPORTED_UNICODE_RANGES[0].iter();
        assert_eq!(iter1.counter, 0);
        for _ in 0..('a' as usize) {
            iter1.next().unwrap();
        }
        match iter1.next().unwrap() {
            UnicodeSymbol::Char(c) => {
                assert_eq!(c, 'a')
            }
            _ => panic!(),
        };

        let mut iter2 = SUPPORTED_UNICODE_RANGES[1].iter();
        // skip cr1 control characters
        for _ in 0..32 {
            iter2.next().unwrap();
        }
        // skip symbols until copyright symbol
        for _ in 0..9 {
            iter2.next().unwrap();
        }
        match iter2.next().unwrap() {
            UnicodeSymbol::Char(c) => {
                assert_eq!(c, '©')
            }
            _ => panic!(),
        };
    }

    // Ensure that the SUPPORTED_UNICODE_RANGES field does not contain a field multiple times
    // (e.g., due to copy & paste issues)
    #[allow(non_snake_case)]
    #[test]
    fn test_SUPPORTED_UNICODE_RANGES_unique() {
        SUPPORTED_UNICODE_RANGES.iter().enumerate().for_each(|r1| {
            SUPPORTED_UNICODE_RANGES
                .iter()
                .enumerate()
                .filter(|r2| r1.0 != r2.0)
                .for_each(|r2| assert_ne!(r1.1, r2.1))
        })
    }
}
