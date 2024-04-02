use std::ops::RangeInclusive;

pub use ranges::BASIC_LATIN;

/// The unicode ranges that the library will support. Some unicode ranges
/// do not include all symbols, as some are for example reserver for control
/// sequences or have other special meaning.
pub const SUPPORTED_UNICODE_RANGES: &[UnicodeRange] = &[
    ranges::BASIC_LATIN,
    ranges::LATIN_1_SUPPLEMENT,
    ranges::LATIN_EXTENDED_A,
    // careful: adding more results in much more file size
    /*
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
    ranges::SPECIALS,
];

/// Relevant information about unicode ranges.
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
    /// Symbols that should be ignored. For example, the "basic-latin" unicode range
    /// contains ASCII, which contains many control characters. We want these to be skipped.
    pub ignored_symbols: &'static [RangeInclusive<u32>],
}

impl UnicodeRange {
    /// Returns an iterator over the range.
    pub const fn iter(&self) -> UnicodeRangeIter {
        UnicodeRangeIter::<'_>::new(self)
    }

    /// Returns the name of the Cargo feature for this unicode range.
    pub const fn feature_name(&self) -> &'static str {
        self.feature_name
    }

    /// Returns true if the symbol is in one of the specified ranges of ignored symbols.
    fn symbol_is_ignored(&self, symbol: u32) -> bool {
        self.ignored_symbols
            .iter()
            .any(|ignored_symbols| ignored_symbols.contains(&symbol))
    }
}

/// Iterator over all characters of a [`UnicodeRange`] with respect to ignored
/// symbols.
#[derive(Debug)]
pub struct UnicodeRangeIter<'a> {
    range: &'a UnicodeRange,
    // the incrementing counter between range.begin and range.end
    counter: u32,
}

impl<'a> UnicodeRangeIter<'a> {
    const fn new(range: &'a UnicodeRange) -> Self {
        Self {
            range,
            counter: range.begin,
        }
    }
}

impl<'a> Iterator for UnicodeRangeIter<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter > self.range.end {
            return None;
        }

        // skip ignored ranges, such as control characters in between
        while self.range.symbol_is_ignored(self.counter) {
            self.counter += 1;
        }

        // Without this check, and if the last symbol is ignored, it might happen that
        // this iterator returns a value out of its range.
        if self.counter > self.range.end {
            None
        } else {
            let char = self.counter;
            self.counter += 1;
            Some(char::from_u32(char).unwrap())
        }
    }
}

mod ranges {
    use super::*;

    // ASCII etc
    pub const BASIC_LATIN: UnicodeRange = UnicodeRange {
        feature_name: "unicode-basic-latin",
        begin: 0,
        end: 0x7f,
        default_feature: true,
        ignored_symbols: &[
            // control characters
            0..=0x1f,
            // delete control character
            0x7f..=0x7f,
        ],
    };

    // ÄöüË
    pub const LATIN_1_SUPPLEMENT: UnicodeRange = UnicodeRange {
        feature_name: "unicode-latin-1-supplement",
        begin: 0x80,
        end: 0xff,
        default_feature: false,
        ignored_symbols: &[
            // control characters
            0x80..=0x9f,
            // protected whitespace
            0xa0..=0xa0,
            // soft hyphen
            0xad..=0xad,
        ],
    };

    // ĈĻŜ
    pub const LATIN_EXTENDED_A: UnicodeRange = UnicodeRange {
        feature_name: "unicode-latin-extended-a",
        begin: 0x100,
        end: 0x17f,
        default_feature: false,
        ignored_symbols: &[],
    };

    // includes "�", i.e., the generic replacement character
    pub const SPECIALS: UnicodeRange = UnicodeRange {
        feature_name: "unicode-specials",
        begin: 0xfff0,
        end: 0xffff,
        default_feature: false,
        ignored_symbols: &[
            // in this range, we excldue everything except "�"
            // as this is the only relevant symbol from this range
            // that people ever will use (in the context & scope of this library)
            0xfff0..=0xfffc,
            0xfffe..=0xffff,
        ],
    };
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

    // Tests that the iterator starts at the range begin and not always at zero.
    // Also verify that the iterator stops at it's designated end.
    #[test]
    fn test_unicode_iter_begin_end() {
        let mut iter = ranges::LATIN_1_SUPPLEMENT.iter();
        assert!(iter.next().unwrap() as u32 >= iter.range.begin);
        assert!((iter.next().unwrap() as u32) < ranges::LATIN_EXTENDED_A.begin);

        let iter = ranges::BASIC_LATIN.iter();
        assert_eq!('~', iter.last().unwrap());

        let iter = ranges::SPECIALS.iter();
        // this is currently a special case, where the last valid symbol is this one
        // (also see the ignored symbols of this range)
        assert_eq!(iter.last().unwrap(), '�');
    }

    #[test]
    fn test_unicode_range_iter_skip_control_sequences() {
        // ASCII code range
        let mut iter = ranges::BASIC_LATIN.iter();
        // first one must be space
        assert_eq!(iter.next().unwrap(), '\x20');

        // ASCII code range
        let mut iter = ranges::SPECIALS.iter();
        // the other symbols in this range are currently omitted
        assert_eq!(iter.next().unwrap(), '�');
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

    // Ensure that the SUPPORTED_UNICODE_RANGES are in a total order.
    #[allow(non_snake_case)]
    #[test]
    fn test_SUPPORTED_UNICODE_RANGES_in_order() {
        SUPPORTED_UNICODE_RANGES
            .iter()
            .zip(SUPPORTED_UNICODE_RANGES.iter().skip(1))
            .for_each(|(a, b)| {
                assert!(a.end < b.begin);
            })
    }

    // Ensure that the SUPPORTED_UNICODE_RANGES ignored symbols are within the boundaries
    // of the unicode range.
    #[allow(non_snake_case)]
    #[test]
    fn test_SUPPORTED_UNICODE_RANGES_ignored_symbols_in_range() {
        SUPPORTED_UNICODE_RANGES.iter().for_each(|range| {
            for x in range.ignored_symbols {
                assert!(x.start() <= x.end());
                assert!(*x.start() >= range.begin);
                assert!(*x.end() <= range.end);
            }
        })
    }
}
