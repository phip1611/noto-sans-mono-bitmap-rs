//! Module for letters with the font weight bold and size 20.
//!
//! # Font License
//! * Downloaded from: <https://fonts.google.com/noto/specimen/Noto+Sans+Mono>
//! * License: SIL Open Font License (OFL) <https://scripts.sil.org/cms/scripts/page.php?site_id=nrsi&id=OFL>

/// The constant height of each rasterized character for the given font weight
/// and size. This size corresponds to the size of the module name. The font
/// size will be a few percent smaller, as each raster contains a small vertical
/// to ensure vertical alignment of multiple characters.
#[allow(dead_code)]
pub const RASTER_HEIGHT: usize = 20;

/// The constant width of each rasterized character for the given font weight and
/// size. This is less than [`RASTER_HEIGHT`], so that multiple letters can be print
/// next to each other and look "nice" out-of-the-box, hence, library users do not
/// have to perform additional alignment of characters or fill in padding.
pub const RASTER_WIDTH: usize = 9;

/// Returns the raster of the given character for font weight bold and font size 14px.
/// Wide characters, such as '�', will be truncated in their width in order to fulfill
/// the mono font guarantee. All characters are centered in their raster.
#[inline]
pub const fn get_char(c: char) -> Option<&'static [&'static [u8]]> {
    match c {
        // letter: ' ' / 0x20
        #[cfg(feature = "unicode-basic-latin")]
        ' ' => Some(include!("../res_rasterized_characters/0x20_h20_wBold.txt")),
        // letter: '!' / 0x21
        #[cfg(feature = "unicode-basic-latin")]
        '!' => Some(include!("../res_rasterized_characters/0x21_h20_wBold.txt")),
        // letter: '"' / 0x22
        #[cfg(feature = "unicode-basic-latin")]
        '"' => Some(include!("../res_rasterized_characters/0x22_h20_wBold.txt")),
        // letter: '#' / 0x23
        #[cfg(feature = "unicode-basic-latin")]
        '#' => Some(include!("../res_rasterized_characters/0x23_h20_wBold.txt")),
        // letter: '$' / 0x24
        #[cfg(feature = "unicode-basic-latin")]
        '$' => Some(include!("../res_rasterized_characters/0x24_h20_wBold.txt")),
        // letter: '%' / 0x25
        #[cfg(feature = "unicode-basic-latin")]
        '%' => Some(include!("../res_rasterized_characters/0x25_h20_wBold.txt")),
        // letter: '&' / 0x26
        #[cfg(feature = "unicode-basic-latin")]
        '&' => Some(include!("../res_rasterized_characters/0x26_h20_wBold.txt")),
        // letter: ''' / 0x27
        #[cfg(feature = "unicode-basic-latin")]
        '\'' => Some(include!("../res_rasterized_characters/0x27_h20_wBold.txt")),
        // letter: '(' / 0x28
        #[cfg(feature = "unicode-basic-latin")]
        '(' => Some(include!("../res_rasterized_characters/0x28_h20_wBold.txt")),
        // letter: ')' / 0x29
        #[cfg(feature = "unicode-basic-latin")]
        ')' => Some(include!("../res_rasterized_characters/0x29_h20_wBold.txt")),
        // letter: '*' / 0x2a
        #[cfg(feature = "unicode-basic-latin")]
        '*' => Some(include!("../res_rasterized_characters/0x2a_h20_wBold.txt")),
        // letter: '+' / 0x2b
        #[cfg(feature = "unicode-basic-latin")]
        '+' => Some(include!("../res_rasterized_characters/0x2b_h20_wBold.txt")),
        // letter: ',' / 0x2c
        #[cfg(feature = "unicode-basic-latin")]
        ',' => Some(include!("../res_rasterized_characters/0x2c_h20_wBold.txt")),
        // letter: '-' / 0x2d
        #[cfg(feature = "unicode-basic-latin")]
        '-' => Some(include!("../res_rasterized_characters/0x2d_h20_wBold.txt")),
        // letter: '.' / 0x2e
        #[cfg(feature = "unicode-basic-latin")]
        '.' => Some(include!("../res_rasterized_characters/0x2e_h20_wBold.txt")),
        // letter: '/' / 0x2f
        #[cfg(feature = "unicode-basic-latin")]
        '/' => Some(include!("../res_rasterized_characters/0x2f_h20_wBold.txt")),
        // letter: '0' / 0x30
        #[cfg(feature = "unicode-basic-latin")]
        '0' => Some(include!("../res_rasterized_characters/0x30_h20_wBold.txt")),
        // letter: '1' / 0x31
        #[cfg(feature = "unicode-basic-latin")]
        '1' => Some(include!("../res_rasterized_characters/0x31_h20_wBold.txt")),
        // letter: '2' / 0x32
        #[cfg(feature = "unicode-basic-latin")]
        '2' => Some(include!("../res_rasterized_characters/0x32_h20_wBold.txt")),
        // letter: '3' / 0x33
        #[cfg(feature = "unicode-basic-latin")]
        '3' => Some(include!("../res_rasterized_characters/0x33_h20_wBold.txt")),
        // letter: '4' / 0x34
        #[cfg(feature = "unicode-basic-latin")]
        '4' => Some(include!("../res_rasterized_characters/0x34_h20_wBold.txt")),
        // letter: '5' / 0x35
        #[cfg(feature = "unicode-basic-latin")]
        '5' => Some(include!("../res_rasterized_characters/0x35_h20_wBold.txt")),
        // letter: '6' / 0x36
        #[cfg(feature = "unicode-basic-latin")]
        '6' => Some(include!("../res_rasterized_characters/0x36_h20_wBold.txt")),
        // letter: '7' / 0x37
        #[cfg(feature = "unicode-basic-latin")]
        '7' => Some(include!("../res_rasterized_characters/0x37_h20_wBold.txt")),
        // letter: '8' / 0x38
        #[cfg(feature = "unicode-basic-latin")]
        '8' => Some(include!("../res_rasterized_characters/0x38_h20_wBold.txt")),
        // letter: '9' / 0x39
        #[cfg(feature = "unicode-basic-latin")]
        '9' => Some(include!("../res_rasterized_characters/0x39_h20_wBold.txt")),
        // letter: ':' / 0x3a
        #[cfg(feature = "unicode-basic-latin")]
        ':' => Some(include!("../res_rasterized_characters/0x3a_h20_wBold.txt")),
        // letter: ';' / 0x3b
        #[cfg(feature = "unicode-basic-latin")]
        ';' => Some(include!("../res_rasterized_characters/0x3b_h20_wBold.txt")),
        // letter: '<' / 0x3c
        #[cfg(feature = "unicode-basic-latin")]
        '<' => Some(include!("../res_rasterized_characters/0x3c_h20_wBold.txt")),
        // letter: '=' / 0x3d
        #[cfg(feature = "unicode-basic-latin")]
        '=' => Some(include!("../res_rasterized_characters/0x3d_h20_wBold.txt")),
        // letter: '>' / 0x3e
        #[cfg(feature = "unicode-basic-latin")]
        '>' => Some(include!("../res_rasterized_characters/0x3e_h20_wBold.txt")),
        // letter: '?' / 0x3f
        #[cfg(feature = "unicode-basic-latin")]
        '?' => Some(include!("../res_rasterized_characters/0x3f_h20_wBold.txt")),
        // letter: '@' / 0x40
        #[cfg(feature = "unicode-basic-latin")]
        '@' => Some(include!("../res_rasterized_characters/0x40_h20_wBold.txt")),
        // letter: 'A' / 0x41
        #[cfg(feature = "unicode-basic-latin")]
        'A' => Some(include!("../res_rasterized_characters/0x41_h20_wBold.txt")),
        // letter: 'B' / 0x42
        #[cfg(feature = "unicode-basic-latin")]
        'B' => Some(include!("../res_rasterized_characters/0x42_h20_wBold.txt")),
        // letter: 'C' / 0x43
        #[cfg(feature = "unicode-basic-latin")]
        'C' => Some(include!("../res_rasterized_characters/0x43_h20_wBold.txt")),
        // letter: 'D' / 0x44
        #[cfg(feature = "unicode-basic-latin")]
        'D' => Some(include!("../res_rasterized_characters/0x44_h20_wBold.txt")),
        // letter: 'E' / 0x45
        #[cfg(feature = "unicode-basic-latin")]
        'E' => Some(include!("../res_rasterized_characters/0x45_h20_wBold.txt")),
        // letter: 'F' / 0x46
        #[cfg(feature = "unicode-basic-latin")]
        'F' => Some(include!("../res_rasterized_characters/0x46_h20_wBold.txt")),
        // letter: 'G' / 0x47
        #[cfg(feature = "unicode-basic-latin")]
        'G' => Some(include!("../res_rasterized_characters/0x47_h20_wBold.txt")),
        // letter: 'H' / 0x48
        #[cfg(feature = "unicode-basic-latin")]
        'H' => Some(include!("../res_rasterized_characters/0x48_h20_wBold.txt")),
        // letter: 'I' / 0x49
        #[cfg(feature = "unicode-basic-latin")]
        'I' => Some(include!("../res_rasterized_characters/0x49_h20_wBold.txt")),
        // letter: 'J' / 0x4a
        #[cfg(feature = "unicode-basic-latin")]
        'J' => Some(include!("../res_rasterized_characters/0x4a_h20_wBold.txt")),
        // letter: 'K' / 0x4b
        #[cfg(feature = "unicode-basic-latin")]
        'K' => Some(include!("../res_rasterized_characters/0x4b_h20_wBold.txt")),
        // letter: 'L' / 0x4c
        #[cfg(feature = "unicode-basic-latin")]
        'L' => Some(include!("../res_rasterized_characters/0x4c_h20_wBold.txt")),
        // letter: 'M' / 0x4d
        #[cfg(feature = "unicode-basic-latin")]
        'M' => Some(include!("../res_rasterized_characters/0x4d_h20_wBold.txt")),
        // letter: 'N' / 0x4e
        #[cfg(feature = "unicode-basic-latin")]
        'N' => Some(include!("../res_rasterized_characters/0x4e_h20_wBold.txt")),
        // letter: 'O' / 0x4f
        #[cfg(feature = "unicode-basic-latin")]
        'O' => Some(include!("../res_rasterized_characters/0x4f_h20_wBold.txt")),
        // letter: 'P' / 0x50
        #[cfg(feature = "unicode-basic-latin")]
        'P' => Some(include!("../res_rasterized_characters/0x50_h20_wBold.txt")),
        // letter: 'Q' / 0x51
        #[cfg(feature = "unicode-basic-latin")]
        'Q' => Some(include!("../res_rasterized_characters/0x51_h20_wBold.txt")),
        // letter: 'R' / 0x52
        #[cfg(feature = "unicode-basic-latin")]
        'R' => Some(include!("../res_rasterized_characters/0x52_h20_wBold.txt")),
        // letter: 'S' / 0x53
        #[cfg(feature = "unicode-basic-latin")]
        'S' => Some(include!("../res_rasterized_characters/0x53_h20_wBold.txt")),
        // letter: 'T' / 0x54
        #[cfg(feature = "unicode-basic-latin")]
        'T' => Some(include!("../res_rasterized_characters/0x54_h20_wBold.txt")),
        // letter: 'U' / 0x55
        #[cfg(feature = "unicode-basic-latin")]
        'U' => Some(include!("../res_rasterized_characters/0x55_h20_wBold.txt")),
        // letter: 'V' / 0x56
        #[cfg(feature = "unicode-basic-latin")]
        'V' => Some(include!("../res_rasterized_characters/0x56_h20_wBold.txt")),
        // letter: 'W' / 0x57
        #[cfg(feature = "unicode-basic-latin")]
        'W' => Some(include!("../res_rasterized_characters/0x57_h20_wBold.txt")),
        // letter: 'X' / 0x58
        #[cfg(feature = "unicode-basic-latin")]
        'X' => Some(include!("../res_rasterized_characters/0x58_h20_wBold.txt")),
        // letter: 'Y' / 0x59
        #[cfg(feature = "unicode-basic-latin")]
        'Y' => Some(include!("../res_rasterized_characters/0x59_h20_wBold.txt")),
        // letter: 'Z' / 0x5a
        #[cfg(feature = "unicode-basic-latin")]
        'Z' => Some(include!("../res_rasterized_characters/0x5a_h20_wBold.txt")),
        // letter: '[' / 0x5b
        #[cfg(feature = "unicode-basic-latin")]
        '[' => Some(include!("../res_rasterized_characters/0x5b_h20_wBold.txt")),
        // letter: '\' / 0x5c
        #[cfg(feature = "unicode-basic-latin")]
        '\\' => Some(include!("../res_rasterized_characters/0x5c_h20_wBold.txt")),
        // letter: ']' / 0x5d
        #[cfg(feature = "unicode-basic-latin")]
        ']' => Some(include!("../res_rasterized_characters/0x5d_h20_wBold.txt")),
        // letter: '^' / 0x5e
        #[cfg(feature = "unicode-basic-latin")]
        '^' => Some(include!("../res_rasterized_characters/0x5e_h20_wBold.txt")),
        // letter: '_' / 0x5f
        #[cfg(feature = "unicode-basic-latin")]
        '_' => Some(include!("../res_rasterized_characters/0x5f_h20_wBold.txt")),
        // letter: '`' / 0x60
        #[cfg(feature = "unicode-basic-latin")]
        '`' => Some(include!("../res_rasterized_characters/0x60_h20_wBold.txt")),
        // letter: 'a' / 0x61
        #[cfg(feature = "unicode-basic-latin")]
        'a' => Some(include!("../res_rasterized_characters/0x61_h20_wBold.txt")),
        // letter: 'b' / 0x62
        #[cfg(feature = "unicode-basic-latin")]
        'b' => Some(include!("../res_rasterized_characters/0x62_h20_wBold.txt")),
        // letter: 'c' / 0x63
        #[cfg(feature = "unicode-basic-latin")]
        'c' => Some(include!("../res_rasterized_characters/0x63_h20_wBold.txt")),
        // letter: 'd' / 0x64
        #[cfg(feature = "unicode-basic-latin")]
        'd' => Some(include!("../res_rasterized_characters/0x64_h20_wBold.txt")),
        // letter: 'e' / 0x65
        #[cfg(feature = "unicode-basic-latin")]
        'e' => Some(include!("../res_rasterized_characters/0x65_h20_wBold.txt")),
        // letter: 'f' / 0x66
        #[cfg(feature = "unicode-basic-latin")]
        'f' => Some(include!("../res_rasterized_characters/0x66_h20_wBold.txt")),
        // letter: 'g' / 0x67
        #[cfg(feature = "unicode-basic-latin")]
        'g' => Some(include!("../res_rasterized_characters/0x67_h20_wBold.txt")),
        // letter: 'h' / 0x68
        #[cfg(feature = "unicode-basic-latin")]
        'h' => Some(include!("../res_rasterized_characters/0x68_h20_wBold.txt")),
        // letter: 'i' / 0x69
        #[cfg(feature = "unicode-basic-latin")]
        'i' => Some(include!("../res_rasterized_characters/0x69_h20_wBold.txt")),
        // letter: 'j' / 0x6a
        #[cfg(feature = "unicode-basic-latin")]
        'j' => Some(include!("../res_rasterized_characters/0x6a_h20_wBold.txt")),
        // letter: 'k' / 0x6b
        #[cfg(feature = "unicode-basic-latin")]
        'k' => Some(include!("../res_rasterized_characters/0x6b_h20_wBold.txt")),
        // letter: 'l' / 0x6c
        #[cfg(feature = "unicode-basic-latin")]
        'l' => Some(include!("../res_rasterized_characters/0x6c_h20_wBold.txt")),
        // letter: 'm' / 0x6d
        #[cfg(feature = "unicode-basic-latin")]
        'm' => Some(include!("../res_rasterized_characters/0x6d_h20_wBold.txt")),
        // letter: 'n' / 0x6e
        #[cfg(feature = "unicode-basic-latin")]
        'n' => Some(include!("../res_rasterized_characters/0x6e_h20_wBold.txt")),
        // letter: 'o' / 0x6f
        #[cfg(feature = "unicode-basic-latin")]
        'o' => Some(include!("../res_rasterized_characters/0x6f_h20_wBold.txt")),
        // letter: 'p' / 0x70
        #[cfg(feature = "unicode-basic-latin")]
        'p' => Some(include!("../res_rasterized_characters/0x70_h20_wBold.txt")),
        // letter: 'q' / 0x71
        #[cfg(feature = "unicode-basic-latin")]
        'q' => Some(include!("../res_rasterized_characters/0x71_h20_wBold.txt")),
        // letter: 'r' / 0x72
        #[cfg(feature = "unicode-basic-latin")]
        'r' => Some(include!("../res_rasterized_characters/0x72_h20_wBold.txt")),
        // letter: 's' / 0x73
        #[cfg(feature = "unicode-basic-latin")]
        's' => Some(include!("../res_rasterized_characters/0x73_h20_wBold.txt")),
        // letter: 't' / 0x74
        #[cfg(feature = "unicode-basic-latin")]
        't' => Some(include!("../res_rasterized_characters/0x74_h20_wBold.txt")),
        // letter: 'u' / 0x75
        #[cfg(feature = "unicode-basic-latin")]
        'u' => Some(include!("../res_rasterized_characters/0x75_h20_wBold.txt")),
        // letter: 'v' / 0x76
        #[cfg(feature = "unicode-basic-latin")]
        'v' => Some(include!("../res_rasterized_characters/0x76_h20_wBold.txt")),
        // letter: 'w' / 0x77
        #[cfg(feature = "unicode-basic-latin")]
        'w' => Some(include!("../res_rasterized_characters/0x77_h20_wBold.txt")),
        // letter: 'x' / 0x78
        #[cfg(feature = "unicode-basic-latin")]
        'x' => Some(include!("../res_rasterized_characters/0x78_h20_wBold.txt")),
        // letter: 'y' / 0x79
        #[cfg(feature = "unicode-basic-latin")]
        'y' => Some(include!("../res_rasterized_characters/0x79_h20_wBold.txt")),
        // letter: 'z' / 0x7a
        #[cfg(feature = "unicode-basic-latin")]
        'z' => Some(include!("../res_rasterized_characters/0x7a_h20_wBold.txt")),
        // letter: '{' / 0x7b
        #[cfg(feature = "unicode-basic-latin")]
        '{' => Some(include!("../res_rasterized_characters/0x7b_h20_wBold.txt")),
        // letter: '|' / 0x7c
        #[cfg(feature = "unicode-basic-latin")]
        '|' => Some(include!("../res_rasterized_characters/0x7c_h20_wBold.txt")),
        // letter: '}' / 0x7d
        #[cfg(feature = "unicode-basic-latin")]
        '}' => Some(include!("../res_rasterized_characters/0x7d_h20_wBold.txt")),
        // letter: '~' / 0x7e
        #[cfg(feature = "unicode-basic-latin")]
        '~' => Some(include!("../res_rasterized_characters/0x7e_h20_wBold.txt")),
        // letter: '¡' / 0xa1
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¡' => Some(include!("../res_rasterized_characters/0xa1_h20_wBold.txt")),
        // letter: '¢' / 0xa2
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¢' => Some(include!("../res_rasterized_characters/0xa2_h20_wBold.txt")),
        // letter: '£' / 0xa3
        #[cfg(feature = "unicode-latin-1-supplement")]
        '£' => Some(include!("../res_rasterized_characters/0xa3_h20_wBold.txt")),
        // letter: '¤' / 0xa4
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¤' => Some(include!("../res_rasterized_characters/0xa4_h20_wBold.txt")),
        // letter: '¥' / 0xa5
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¥' => Some(include!("../res_rasterized_characters/0xa5_h20_wBold.txt")),
        // letter: '¦' / 0xa6
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¦' => Some(include!("../res_rasterized_characters/0xa6_h20_wBold.txt")),
        // letter: '§' / 0xa7
        #[cfg(feature = "unicode-latin-1-supplement")]
        '§' => Some(include!("../res_rasterized_characters/0xa7_h20_wBold.txt")),
        // letter: '¨' / 0xa8
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¨' => Some(include!("../res_rasterized_characters/0xa8_h20_wBold.txt")),
        // letter: '©' / 0xa9
        #[cfg(feature = "unicode-latin-1-supplement")]
        '©' => Some(include!("../res_rasterized_characters/0xa9_h20_wBold.txt")),
        // letter: 'ª' / 0xaa
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ª' => Some(include!("../res_rasterized_characters/0xaa_h20_wBold.txt")),
        // letter: '«' / 0xab
        #[cfg(feature = "unicode-latin-1-supplement")]
        '«' => Some(include!("../res_rasterized_characters/0xab_h20_wBold.txt")),
        // letter: '¬' / 0xac
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¬' => Some(include!("../res_rasterized_characters/0xac_h20_wBold.txt")),
        // letter: '®' / 0xae
        #[cfg(feature = "unicode-latin-1-supplement")]
        '®' => Some(include!("../res_rasterized_characters/0xae_h20_wBold.txt")),
        // letter: '¯' / 0xaf
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¯' => Some(include!("../res_rasterized_characters/0xaf_h20_wBold.txt")),
        // letter: '°' / 0xb0
        #[cfg(feature = "unicode-latin-1-supplement")]
        '°' => Some(include!("../res_rasterized_characters/0xb0_h20_wBold.txt")),
        // letter: '±' / 0xb1
        #[cfg(feature = "unicode-latin-1-supplement")]
        '±' => Some(include!("../res_rasterized_characters/0xb1_h20_wBold.txt")),
        // letter: '²' / 0xb2
        #[cfg(feature = "unicode-latin-1-supplement")]
        '²' => Some(include!("../res_rasterized_characters/0xb2_h20_wBold.txt")),
        // letter: '³' / 0xb3
        #[cfg(feature = "unicode-latin-1-supplement")]
        '³' => Some(include!("../res_rasterized_characters/0xb3_h20_wBold.txt")),
        // letter: '´' / 0xb4
        #[cfg(feature = "unicode-latin-1-supplement")]
        '´' => Some(include!("../res_rasterized_characters/0xb4_h20_wBold.txt")),
        // letter: 'µ' / 0xb5
        #[cfg(feature = "unicode-latin-1-supplement")]
        'µ' => Some(include!("../res_rasterized_characters/0xb5_h20_wBold.txt")),
        // letter: '¶' / 0xb6
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¶' => Some(include!("../res_rasterized_characters/0xb6_h20_wBold.txt")),
        // letter: '·' / 0xb7
        #[cfg(feature = "unicode-latin-1-supplement")]
        '·' => Some(include!("../res_rasterized_characters/0xb7_h20_wBold.txt")),
        // letter: '¸' / 0xb8
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¸' => Some(include!("../res_rasterized_characters/0xb8_h20_wBold.txt")),
        // letter: '¹' / 0xb9
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¹' => Some(include!("../res_rasterized_characters/0xb9_h20_wBold.txt")),
        // letter: 'º' / 0xba
        #[cfg(feature = "unicode-latin-1-supplement")]
        'º' => Some(include!("../res_rasterized_characters/0xba_h20_wBold.txt")),
        // letter: '»' / 0xbb
        #[cfg(feature = "unicode-latin-1-supplement")]
        '»' => Some(include!("../res_rasterized_characters/0xbb_h20_wBold.txt")),
        // letter: '¼' / 0xbc
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¼' => Some(include!("../res_rasterized_characters/0xbc_h20_wBold.txt")),
        // letter: '½' / 0xbd
        #[cfg(feature = "unicode-latin-1-supplement")]
        '½' => Some(include!("../res_rasterized_characters/0xbd_h20_wBold.txt")),
        // letter: '¾' / 0xbe
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¾' => Some(include!("../res_rasterized_characters/0xbe_h20_wBold.txt")),
        // letter: '¿' / 0xbf
        #[cfg(feature = "unicode-latin-1-supplement")]
        '¿' => Some(include!("../res_rasterized_characters/0xbf_h20_wBold.txt")),
        // letter: 'À' / 0xc0
        #[cfg(feature = "unicode-latin-1-supplement")]
        'À' => Some(include!("../res_rasterized_characters/0xc0_h20_wBold.txt")),
        // letter: 'Á' / 0xc1
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Á' => Some(include!("../res_rasterized_characters/0xc1_h20_wBold.txt")),
        // letter: 'Â' / 0xc2
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Â' => Some(include!("../res_rasterized_characters/0xc2_h20_wBold.txt")),
        // letter: 'Ã' / 0xc3
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ã' => Some(include!("../res_rasterized_characters/0xc3_h20_wBold.txt")),
        // letter: 'Ä' / 0xc4
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ä' => Some(include!("../res_rasterized_characters/0xc4_h20_wBold.txt")),
        // letter: 'Å' / 0xc5
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Å' => Some(include!("../res_rasterized_characters/0xc5_h20_wBold.txt")),
        // letter: 'Æ' / 0xc6
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Æ' => Some(include!("../res_rasterized_characters/0xc6_h20_wBold.txt")),
        // letter: 'Ç' / 0xc7
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ç' => Some(include!("../res_rasterized_characters/0xc7_h20_wBold.txt")),
        // letter: 'È' / 0xc8
        #[cfg(feature = "unicode-latin-1-supplement")]
        'È' => Some(include!("../res_rasterized_characters/0xc8_h20_wBold.txt")),
        // letter: 'É' / 0xc9
        #[cfg(feature = "unicode-latin-1-supplement")]
        'É' => Some(include!("../res_rasterized_characters/0xc9_h20_wBold.txt")),
        // letter: 'Ê' / 0xca
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ê' => Some(include!("../res_rasterized_characters/0xca_h20_wBold.txt")),
        // letter: 'Ë' / 0xcb
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ë' => Some(include!("../res_rasterized_characters/0xcb_h20_wBold.txt")),
        // letter: 'Ì' / 0xcc
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ì' => Some(include!("../res_rasterized_characters/0xcc_h20_wBold.txt")),
        // letter: 'Í' / 0xcd
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Í' => Some(include!("../res_rasterized_characters/0xcd_h20_wBold.txt")),
        // letter: 'Î' / 0xce
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Î' => Some(include!("../res_rasterized_characters/0xce_h20_wBold.txt")),
        // letter: 'Ï' / 0xcf
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ï' => Some(include!("../res_rasterized_characters/0xcf_h20_wBold.txt")),
        // letter: 'Ð' / 0xd0
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ð' => Some(include!("../res_rasterized_characters/0xd0_h20_wBold.txt")),
        // letter: 'Ñ' / 0xd1
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ñ' => Some(include!("../res_rasterized_characters/0xd1_h20_wBold.txt")),
        // letter: 'Ò' / 0xd2
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ò' => Some(include!("../res_rasterized_characters/0xd2_h20_wBold.txt")),
        // letter: 'Ó' / 0xd3
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ó' => Some(include!("../res_rasterized_characters/0xd3_h20_wBold.txt")),
        // letter: 'Ô' / 0xd4
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ô' => Some(include!("../res_rasterized_characters/0xd4_h20_wBold.txt")),
        // letter: 'Õ' / 0xd5
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Õ' => Some(include!("../res_rasterized_characters/0xd5_h20_wBold.txt")),
        // letter: 'Ö' / 0xd6
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ö' => Some(include!("../res_rasterized_characters/0xd6_h20_wBold.txt")),
        // letter: '×' / 0xd7
        #[cfg(feature = "unicode-latin-1-supplement")]
        '×' => Some(include!("../res_rasterized_characters/0xd7_h20_wBold.txt")),
        // letter: 'Ø' / 0xd8
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ø' => Some(include!("../res_rasterized_characters/0xd8_h20_wBold.txt")),
        // letter: 'Ù' / 0xd9
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ù' => Some(include!("../res_rasterized_characters/0xd9_h20_wBold.txt")),
        // letter: 'Ú' / 0xda
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ú' => Some(include!("../res_rasterized_characters/0xda_h20_wBold.txt")),
        // letter: 'Û' / 0xdb
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Û' => Some(include!("../res_rasterized_characters/0xdb_h20_wBold.txt")),
        // letter: 'Ü' / 0xdc
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ü' => Some(include!("../res_rasterized_characters/0xdc_h20_wBold.txt")),
        // letter: 'Ý' / 0xdd
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Ý' => Some(include!("../res_rasterized_characters/0xdd_h20_wBold.txt")),
        // letter: 'Þ' / 0xde
        #[cfg(feature = "unicode-latin-1-supplement")]
        'Þ' => Some(include!("../res_rasterized_characters/0xde_h20_wBold.txt")),
        // letter: 'ß' / 0xdf
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ß' => Some(include!("../res_rasterized_characters/0xdf_h20_wBold.txt")),
        // letter: 'à' / 0xe0
        #[cfg(feature = "unicode-latin-1-supplement")]
        'à' => Some(include!("../res_rasterized_characters/0xe0_h20_wBold.txt")),
        // letter: 'á' / 0xe1
        #[cfg(feature = "unicode-latin-1-supplement")]
        'á' => Some(include!("../res_rasterized_characters/0xe1_h20_wBold.txt")),
        // letter: 'â' / 0xe2
        #[cfg(feature = "unicode-latin-1-supplement")]
        'â' => Some(include!("../res_rasterized_characters/0xe2_h20_wBold.txt")),
        // letter: 'ã' / 0xe3
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ã' => Some(include!("../res_rasterized_characters/0xe3_h20_wBold.txt")),
        // letter: 'ä' / 0xe4
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ä' => Some(include!("../res_rasterized_characters/0xe4_h20_wBold.txt")),
        // letter: 'å' / 0xe5
        #[cfg(feature = "unicode-latin-1-supplement")]
        'å' => Some(include!("../res_rasterized_characters/0xe5_h20_wBold.txt")),
        // letter: 'æ' / 0xe6
        #[cfg(feature = "unicode-latin-1-supplement")]
        'æ' => Some(include!("../res_rasterized_characters/0xe6_h20_wBold.txt")),
        // letter: 'ç' / 0xe7
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ç' => Some(include!("../res_rasterized_characters/0xe7_h20_wBold.txt")),
        // letter: 'è' / 0xe8
        #[cfg(feature = "unicode-latin-1-supplement")]
        'è' => Some(include!("../res_rasterized_characters/0xe8_h20_wBold.txt")),
        // letter: 'é' / 0xe9
        #[cfg(feature = "unicode-latin-1-supplement")]
        'é' => Some(include!("../res_rasterized_characters/0xe9_h20_wBold.txt")),
        // letter: 'ê' / 0xea
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ê' => Some(include!("../res_rasterized_characters/0xea_h20_wBold.txt")),
        // letter: 'ë' / 0xeb
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ë' => Some(include!("../res_rasterized_characters/0xeb_h20_wBold.txt")),
        // letter: 'ì' / 0xec
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ì' => Some(include!("../res_rasterized_characters/0xec_h20_wBold.txt")),
        // letter: 'í' / 0xed
        #[cfg(feature = "unicode-latin-1-supplement")]
        'í' => Some(include!("../res_rasterized_characters/0xed_h20_wBold.txt")),
        // letter: 'î' / 0xee
        #[cfg(feature = "unicode-latin-1-supplement")]
        'î' => Some(include!("../res_rasterized_characters/0xee_h20_wBold.txt")),
        // letter: 'ï' / 0xef
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ï' => Some(include!("../res_rasterized_characters/0xef_h20_wBold.txt")),
        // letter: 'ð' / 0xf0
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ð' => Some(include!("../res_rasterized_characters/0xf0_h20_wBold.txt")),
        // letter: 'ñ' / 0xf1
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ñ' => Some(include!("../res_rasterized_characters/0xf1_h20_wBold.txt")),
        // letter: 'ò' / 0xf2
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ò' => Some(include!("../res_rasterized_characters/0xf2_h20_wBold.txt")),
        // letter: 'ó' / 0xf3
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ó' => Some(include!("../res_rasterized_characters/0xf3_h20_wBold.txt")),
        // letter: 'ô' / 0xf4
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ô' => Some(include!("../res_rasterized_characters/0xf4_h20_wBold.txt")),
        // letter: 'õ' / 0xf5
        #[cfg(feature = "unicode-latin-1-supplement")]
        'õ' => Some(include!("../res_rasterized_characters/0xf5_h20_wBold.txt")),
        // letter: 'ö' / 0xf6
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ö' => Some(include!("../res_rasterized_characters/0xf6_h20_wBold.txt")),
        // letter: '÷' / 0xf7
        #[cfg(feature = "unicode-latin-1-supplement")]
        '÷' => Some(include!("../res_rasterized_characters/0xf7_h20_wBold.txt")),
        // letter: 'ø' / 0xf8
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ø' => Some(include!("../res_rasterized_characters/0xf8_h20_wBold.txt")),
        // letter: 'ù' / 0xf9
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ù' => Some(include!("../res_rasterized_characters/0xf9_h20_wBold.txt")),
        // letter: 'ú' / 0xfa
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ú' => Some(include!("../res_rasterized_characters/0xfa_h20_wBold.txt")),
        // letter: 'û' / 0xfb
        #[cfg(feature = "unicode-latin-1-supplement")]
        'û' => Some(include!("../res_rasterized_characters/0xfb_h20_wBold.txt")),
        // letter: 'ü' / 0xfc
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ü' => Some(include!("../res_rasterized_characters/0xfc_h20_wBold.txt")),
        // letter: 'ý' / 0xfd
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ý' => Some(include!("../res_rasterized_characters/0xfd_h20_wBold.txt")),
        // letter: 'þ' / 0xfe
        #[cfg(feature = "unicode-latin-1-supplement")]
        'þ' => Some(include!("../res_rasterized_characters/0xfe_h20_wBold.txt")),
        // letter: 'ÿ' / 0xff
        #[cfg(feature = "unicode-latin-1-supplement")]
        'ÿ' => Some(include!("../res_rasterized_characters/0xff_h20_wBold.txt")),
        // letter: 'Ā' / 0x100
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ā' => Some(include!("../res_rasterized_characters/0x100_h20_wBold.txt")),
        // letter: 'ā' / 0x101
        #[cfg(feature = "unicode-latin-extended-a")]
        'ā' => Some(include!("../res_rasterized_characters/0x101_h20_wBold.txt")),
        // letter: 'Ă' / 0x102
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ă' => Some(include!("../res_rasterized_characters/0x102_h20_wBold.txt")),
        // letter: 'ă' / 0x103
        #[cfg(feature = "unicode-latin-extended-a")]
        'ă' => Some(include!("../res_rasterized_characters/0x103_h20_wBold.txt")),
        // letter: 'Ą' / 0x104
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ą' => Some(include!("../res_rasterized_characters/0x104_h20_wBold.txt")),
        // letter: 'ą' / 0x105
        #[cfg(feature = "unicode-latin-extended-a")]
        'ą' => Some(include!("../res_rasterized_characters/0x105_h20_wBold.txt")),
        // letter: 'Ć' / 0x106
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ć' => Some(include!("../res_rasterized_characters/0x106_h20_wBold.txt")),
        // letter: 'ć' / 0x107
        #[cfg(feature = "unicode-latin-extended-a")]
        'ć' => Some(include!("../res_rasterized_characters/0x107_h20_wBold.txt")),
        // letter: 'Ĉ' / 0x108
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ĉ' => Some(include!("../res_rasterized_characters/0x108_h20_wBold.txt")),
        // letter: 'ĉ' / 0x109
        #[cfg(feature = "unicode-latin-extended-a")]
        'ĉ' => Some(include!("../res_rasterized_characters/0x109_h20_wBold.txt")),
        // letter: 'Ċ' / 0x10a
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ċ' => Some(include!("../res_rasterized_characters/0x10a_h20_wBold.txt")),
        // letter: 'ċ' / 0x10b
        #[cfg(feature = "unicode-latin-extended-a")]
        'ċ' => Some(include!("../res_rasterized_characters/0x10b_h20_wBold.txt")),
        // letter: 'Č' / 0x10c
        #[cfg(feature = "unicode-latin-extended-a")]
        'Č' => Some(include!("../res_rasterized_characters/0x10c_h20_wBold.txt")),
        // letter: 'č' / 0x10d
        #[cfg(feature = "unicode-latin-extended-a")]
        'č' => Some(include!("../res_rasterized_characters/0x10d_h20_wBold.txt")),
        // letter: 'Ď' / 0x10e
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ď' => Some(include!("../res_rasterized_characters/0x10e_h20_wBold.txt")),
        // letter: 'ď' / 0x10f
        #[cfg(feature = "unicode-latin-extended-a")]
        'ď' => Some(include!("../res_rasterized_characters/0x10f_h20_wBold.txt")),
        // letter: 'Đ' / 0x110
        #[cfg(feature = "unicode-latin-extended-a")]
        'Đ' => Some(include!("../res_rasterized_characters/0x110_h20_wBold.txt")),
        // letter: 'đ' / 0x111
        #[cfg(feature = "unicode-latin-extended-a")]
        'đ' => Some(include!("../res_rasterized_characters/0x111_h20_wBold.txt")),
        // letter: 'Ē' / 0x112
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ē' => Some(include!("../res_rasterized_characters/0x112_h20_wBold.txt")),
        // letter: 'ē' / 0x113
        #[cfg(feature = "unicode-latin-extended-a")]
        'ē' => Some(include!("../res_rasterized_characters/0x113_h20_wBold.txt")),
        // letter: 'Ĕ' / 0x114
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ĕ' => Some(include!("../res_rasterized_characters/0x114_h20_wBold.txt")),
        // letter: 'ĕ' / 0x115
        #[cfg(feature = "unicode-latin-extended-a")]
        'ĕ' => Some(include!("../res_rasterized_characters/0x115_h20_wBold.txt")),
        // letter: 'Ė' / 0x116
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ė' => Some(include!("../res_rasterized_characters/0x116_h20_wBold.txt")),
        // letter: 'ė' / 0x117
        #[cfg(feature = "unicode-latin-extended-a")]
        'ė' => Some(include!("../res_rasterized_characters/0x117_h20_wBold.txt")),
        // letter: 'Ę' / 0x118
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ę' => Some(include!("../res_rasterized_characters/0x118_h20_wBold.txt")),
        // letter: 'ę' / 0x119
        #[cfg(feature = "unicode-latin-extended-a")]
        'ę' => Some(include!("../res_rasterized_characters/0x119_h20_wBold.txt")),
        // letter: 'Ě' / 0x11a
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ě' => Some(include!("../res_rasterized_characters/0x11a_h20_wBold.txt")),
        // letter: 'ě' / 0x11b
        #[cfg(feature = "unicode-latin-extended-a")]
        'ě' => Some(include!("../res_rasterized_characters/0x11b_h20_wBold.txt")),
        // letter: 'Ĝ' / 0x11c
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ĝ' => Some(include!("../res_rasterized_characters/0x11c_h20_wBold.txt")),
        // letter: 'ĝ' / 0x11d
        #[cfg(feature = "unicode-latin-extended-a")]
        'ĝ' => Some(include!("../res_rasterized_characters/0x11d_h20_wBold.txt")),
        // letter: 'Ğ' / 0x11e
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ğ' => Some(include!("../res_rasterized_characters/0x11e_h20_wBold.txt")),
        // letter: 'ğ' / 0x11f
        #[cfg(feature = "unicode-latin-extended-a")]
        'ğ' => Some(include!("../res_rasterized_characters/0x11f_h20_wBold.txt")),
        // letter: 'Ġ' / 0x120
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ġ' => Some(include!("../res_rasterized_characters/0x120_h20_wBold.txt")),
        // letter: 'ġ' / 0x121
        #[cfg(feature = "unicode-latin-extended-a")]
        'ġ' => Some(include!("../res_rasterized_characters/0x121_h20_wBold.txt")),
        // letter: 'Ģ' / 0x122
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ģ' => Some(include!("../res_rasterized_characters/0x122_h20_wBold.txt")),
        // letter: 'ģ' / 0x123
        #[cfg(feature = "unicode-latin-extended-a")]
        'ģ' => Some(include!("../res_rasterized_characters/0x123_h20_wBold.txt")),
        // letter: 'Ĥ' / 0x124
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ĥ' => Some(include!("../res_rasterized_characters/0x124_h20_wBold.txt")),
        // letter: 'ĥ' / 0x125
        #[cfg(feature = "unicode-latin-extended-a")]
        'ĥ' => Some(include!("../res_rasterized_characters/0x125_h20_wBold.txt")),
        // letter: 'Ħ' / 0x126
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ħ' => Some(include!("../res_rasterized_characters/0x126_h20_wBold.txt")),
        // letter: 'ħ' / 0x127
        #[cfg(feature = "unicode-latin-extended-a")]
        'ħ' => Some(include!("../res_rasterized_characters/0x127_h20_wBold.txt")),
        // letter: 'Ĩ' / 0x128
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ĩ' => Some(include!("../res_rasterized_characters/0x128_h20_wBold.txt")),
        // letter: 'ĩ' / 0x129
        #[cfg(feature = "unicode-latin-extended-a")]
        'ĩ' => Some(include!("../res_rasterized_characters/0x129_h20_wBold.txt")),
        // letter: 'Ī' / 0x12a
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ī' => Some(include!("../res_rasterized_characters/0x12a_h20_wBold.txt")),
        // letter: 'ī' / 0x12b
        #[cfg(feature = "unicode-latin-extended-a")]
        'ī' => Some(include!("../res_rasterized_characters/0x12b_h20_wBold.txt")),
        // letter: 'Ĭ' / 0x12c
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ĭ' => Some(include!("../res_rasterized_characters/0x12c_h20_wBold.txt")),
        // letter: 'ĭ' / 0x12d
        #[cfg(feature = "unicode-latin-extended-a")]
        'ĭ' => Some(include!("../res_rasterized_characters/0x12d_h20_wBold.txt")),
        // letter: 'Į' / 0x12e
        #[cfg(feature = "unicode-latin-extended-a")]
        'Į' => Some(include!("../res_rasterized_characters/0x12e_h20_wBold.txt")),
        // letter: 'į' / 0x12f
        #[cfg(feature = "unicode-latin-extended-a")]
        'į' => Some(include!("../res_rasterized_characters/0x12f_h20_wBold.txt")),
        // letter: 'İ' / 0x130
        #[cfg(feature = "unicode-latin-extended-a")]
        'İ' => Some(include!("../res_rasterized_characters/0x130_h20_wBold.txt")),
        // letter: 'ı' / 0x131
        #[cfg(feature = "unicode-latin-extended-a")]
        'ı' => Some(include!("../res_rasterized_characters/0x131_h20_wBold.txt")),
        // letter: 'Ĳ' / 0x132
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ĳ' => Some(include!("../res_rasterized_characters/0x132_h20_wBold.txt")),
        // letter: 'ĳ' / 0x133
        #[cfg(feature = "unicode-latin-extended-a")]
        'ĳ' => Some(include!("../res_rasterized_characters/0x133_h20_wBold.txt")),
        // letter: 'Ĵ' / 0x134
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ĵ' => Some(include!("../res_rasterized_characters/0x134_h20_wBold.txt")),
        // letter: 'ĵ' / 0x135
        #[cfg(feature = "unicode-latin-extended-a")]
        'ĵ' => Some(include!("../res_rasterized_characters/0x135_h20_wBold.txt")),
        // letter: 'Ķ' / 0x136
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ķ' => Some(include!("../res_rasterized_characters/0x136_h20_wBold.txt")),
        // letter: 'ķ' / 0x137
        #[cfg(feature = "unicode-latin-extended-a")]
        'ķ' => Some(include!("../res_rasterized_characters/0x137_h20_wBold.txt")),
        // letter: 'ĸ' / 0x138
        #[cfg(feature = "unicode-latin-extended-a")]
        'ĸ' => Some(include!("../res_rasterized_characters/0x138_h20_wBold.txt")),
        // letter: 'Ĺ' / 0x139
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ĺ' => Some(include!("../res_rasterized_characters/0x139_h20_wBold.txt")),
        // letter: 'ĺ' / 0x13a
        #[cfg(feature = "unicode-latin-extended-a")]
        'ĺ' => Some(include!("../res_rasterized_characters/0x13a_h20_wBold.txt")),
        // letter: 'Ļ' / 0x13b
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ļ' => Some(include!("../res_rasterized_characters/0x13b_h20_wBold.txt")),
        // letter: 'ļ' / 0x13c
        #[cfg(feature = "unicode-latin-extended-a")]
        'ļ' => Some(include!("../res_rasterized_characters/0x13c_h20_wBold.txt")),
        // letter: 'Ľ' / 0x13d
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ľ' => Some(include!("../res_rasterized_characters/0x13d_h20_wBold.txt")),
        // letter: 'ľ' / 0x13e
        #[cfg(feature = "unicode-latin-extended-a")]
        'ľ' => Some(include!("../res_rasterized_characters/0x13e_h20_wBold.txt")),
        // letter: 'Ŀ' / 0x13f
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ŀ' => Some(include!("../res_rasterized_characters/0x13f_h20_wBold.txt")),
        // letter: 'ŀ' / 0x140
        #[cfg(feature = "unicode-latin-extended-a")]
        'ŀ' => Some(include!("../res_rasterized_characters/0x140_h20_wBold.txt")),
        // letter: 'Ł' / 0x141
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ł' => Some(include!("../res_rasterized_characters/0x141_h20_wBold.txt")),
        // letter: 'ł' / 0x142
        #[cfg(feature = "unicode-latin-extended-a")]
        'ł' => Some(include!("../res_rasterized_characters/0x142_h20_wBold.txt")),
        // letter: 'Ń' / 0x143
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ń' => Some(include!("../res_rasterized_characters/0x143_h20_wBold.txt")),
        // letter: 'ń' / 0x144
        #[cfg(feature = "unicode-latin-extended-a")]
        'ń' => Some(include!("../res_rasterized_characters/0x144_h20_wBold.txt")),
        // letter: 'Ņ' / 0x145
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ņ' => Some(include!("../res_rasterized_characters/0x145_h20_wBold.txt")),
        // letter: 'ņ' / 0x146
        #[cfg(feature = "unicode-latin-extended-a")]
        'ņ' => Some(include!("../res_rasterized_characters/0x146_h20_wBold.txt")),
        // letter: 'Ň' / 0x147
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ň' => Some(include!("../res_rasterized_characters/0x147_h20_wBold.txt")),
        // letter: 'ň' / 0x148
        #[cfg(feature = "unicode-latin-extended-a")]
        'ň' => Some(include!("../res_rasterized_characters/0x148_h20_wBold.txt")),
        // letter: 'ŉ' / 0x149
        #[cfg(feature = "unicode-latin-extended-a")]
        'ŉ' => Some(include!("../res_rasterized_characters/0x149_h20_wBold.txt")),
        // letter: 'Ŋ' / 0x14a
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ŋ' => Some(include!("../res_rasterized_characters/0x14a_h20_wBold.txt")),
        // letter: 'ŋ' / 0x14b
        #[cfg(feature = "unicode-latin-extended-a")]
        'ŋ' => Some(include!("../res_rasterized_characters/0x14b_h20_wBold.txt")),
        // letter: 'Ō' / 0x14c
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ō' => Some(include!("../res_rasterized_characters/0x14c_h20_wBold.txt")),
        // letter: 'ō' / 0x14d
        #[cfg(feature = "unicode-latin-extended-a")]
        'ō' => Some(include!("../res_rasterized_characters/0x14d_h20_wBold.txt")),
        // letter: 'Ŏ' / 0x14e
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ŏ' => Some(include!("../res_rasterized_characters/0x14e_h20_wBold.txt")),
        // letter: 'ŏ' / 0x14f
        #[cfg(feature = "unicode-latin-extended-a")]
        'ŏ' => Some(include!("../res_rasterized_characters/0x14f_h20_wBold.txt")),
        // letter: 'Ő' / 0x150
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ő' => Some(include!("../res_rasterized_characters/0x150_h20_wBold.txt")),
        // letter: 'ő' / 0x151
        #[cfg(feature = "unicode-latin-extended-a")]
        'ő' => Some(include!("../res_rasterized_characters/0x151_h20_wBold.txt")),
        // letter: 'Œ' / 0x152
        #[cfg(feature = "unicode-latin-extended-a")]
        'Œ' => Some(include!("../res_rasterized_characters/0x152_h20_wBold.txt")),
        // letter: 'œ' / 0x153
        #[cfg(feature = "unicode-latin-extended-a")]
        'œ' => Some(include!("../res_rasterized_characters/0x153_h20_wBold.txt")),
        // letter: 'Ŕ' / 0x154
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ŕ' => Some(include!("../res_rasterized_characters/0x154_h20_wBold.txt")),
        // letter: 'ŕ' / 0x155
        #[cfg(feature = "unicode-latin-extended-a")]
        'ŕ' => Some(include!("../res_rasterized_characters/0x155_h20_wBold.txt")),
        // letter: 'Ŗ' / 0x156
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ŗ' => Some(include!("../res_rasterized_characters/0x156_h20_wBold.txt")),
        // letter: 'ŗ' / 0x157
        #[cfg(feature = "unicode-latin-extended-a")]
        'ŗ' => Some(include!("../res_rasterized_characters/0x157_h20_wBold.txt")),
        // letter: 'Ř' / 0x158
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ř' => Some(include!("../res_rasterized_characters/0x158_h20_wBold.txt")),
        // letter: 'ř' / 0x159
        #[cfg(feature = "unicode-latin-extended-a")]
        'ř' => Some(include!("../res_rasterized_characters/0x159_h20_wBold.txt")),
        // letter: 'Ś' / 0x15a
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ś' => Some(include!("../res_rasterized_characters/0x15a_h20_wBold.txt")),
        // letter: 'ś' / 0x15b
        #[cfg(feature = "unicode-latin-extended-a")]
        'ś' => Some(include!("../res_rasterized_characters/0x15b_h20_wBold.txt")),
        // letter: 'Ŝ' / 0x15c
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ŝ' => Some(include!("../res_rasterized_characters/0x15c_h20_wBold.txt")),
        // letter: 'ŝ' / 0x15d
        #[cfg(feature = "unicode-latin-extended-a")]
        'ŝ' => Some(include!("../res_rasterized_characters/0x15d_h20_wBold.txt")),
        // letter: 'Ş' / 0x15e
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ş' => Some(include!("../res_rasterized_characters/0x15e_h20_wBold.txt")),
        // letter: 'ş' / 0x15f
        #[cfg(feature = "unicode-latin-extended-a")]
        'ş' => Some(include!("../res_rasterized_characters/0x15f_h20_wBold.txt")),
        // letter: 'Š' / 0x160
        #[cfg(feature = "unicode-latin-extended-a")]
        'Š' => Some(include!("../res_rasterized_characters/0x160_h20_wBold.txt")),
        // letter: 'š' / 0x161
        #[cfg(feature = "unicode-latin-extended-a")]
        'š' => Some(include!("../res_rasterized_characters/0x161_h20_wBold.txt")),
        // letter: 'Ţ' / 0x162
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ţ' => Some(include!("../res_rasterized_characters/0x162_h20_wBold.txt")),
        // letter: 'ţ' / 0x163
        #[cfg(feature = "unicode-latin-extended-a")]
        'ţ' => Some(include!("../res_rasterized_characters/0x163_h20_wBold.txt")),
        // letter: 'Ť' / 0x164
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ť' => Some(include!("../res_rasterized_characters/0x164_h20_wBold.txt")),
        // letter: 'ť' / 0x165
        #[cfg(feature = "unicode-latin-extended-a")]
        'ť' => Some(include!("../res_rasterized_characters/0x165_h20_wBold.txt")),
        // letter: 'Ŧ' / 0x166
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ŧ' => Some(include!("../res_rasterized_characters/0x166_h20_wBold.txt")),
        // letter: 'ŧ' / 0x167
        #[cfg(feature = "unicode-latin-extended-a")]
        'ŧ' => Some(include!("../res_rasterized_characters/0x167_h20_wBold.txt")),
        // letter: 'Ũ' / 0x168
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ũ' => Some(include!("../res_rasterized_characters/0x168_h20_wBold.txt")),
        // letter: 'ũ' / 0x169
        #[cfg(feature = "unicode-latin-extended-a")]
        'ũ' => Some(include!("../res_rasterized_characters/0x169_h20_wBold.txt")),
        // letter: 'Ū' / 0x16a
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ū' => Some(include!("../res_rasterized_characters/0x16a_h20_wBold.txt")),
        // letter: 'ū' / 0x16b
        #[cfg(feature = "unicode-latin-extended-a")]
        'ū' => Some(include!("../res_rasterized_characters/0x16b_h20_wBold.txt")),
        // letter: 'Ŭ' / 0x16c
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ŭ' => Some(include!("../res_rasterized_characters/0x16c_h20_wBold.txt")),
        // letter: 'ŭ' / 0x16d
        #[cfg(feature = "unicode-latin-extended-a")]
        'ŭ' => Some(include!("../res_rasterized_characters/0x16d_h20_wBold.txt")),
        // letter: 'Ů' / 0x16e
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ů' => Some(include!("../res_rasterized_characters/0x16e_h20_wBold.txt")),
        // letter: 'ů' / 0x16f
        #[cfg(feature = "unicode-latin-extended-a")]
        'ů' => Some(include!("../res_rasterized_characters/0x16f_h20_wBold.txt")),
        // letter: 'Ű' / 0x170
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ű' => Some(include!("../res_rasterized_characters/0x170_h20_wBold.txt")),
        // letter: 'ű' / 0x171
        #[cfg(feature = "unicode-latin-extended-a")]
        'ű' => Some(include!("../res_rasterized_characters/0x171_h20_wBold.txt")),
        // letter: 'Ų' / 0x172
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ų' => Some(include!("../res_rasterized_characters/0x172_h20_wBold.txt")),
        // letter: 'ų' / 0x173
        #[cfg(feature = "unicode-latin-extended-a")]
        'ų' => Some(include!("../res_rasterized_characters/0x173_h20_wBold.txt")),
        // letter: 'Ŵ' / 0x174
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ŵ' => Some(include!("../res_rasterized_characters/0x174_h20_wBold.txt")),
        // letter: 'ŵ' / 0x175
        #[cfg(feature = "unicode-latin-extended-a")]
        'ŵ' => Some(include!("../res_rasterized_characters/0x175_h20_wBold.txt")),
        // letter: 'Ŷ' / 0x176
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ŷ' => Some(include!("../res_rasterized_characters/0x176_h20_wBold.txt")),
        // letter: 'ŷ' / 0x177
        #[cfg(feature = "unicode-latin-extended-a")]
        'ŷ' => Some(include!("../res_rasterized_characters/0x177_h20_wBold.txt")),
        // letter: 'Ÿ' / 0x178
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ÿ' => Some(include!("../res_rasterized_characters/0x178_h20_wBold.txt")),
        // letter: 'Ź' / 0x179
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ź' => Some(include!("../res_rasterized_characters/0x179_h20_wBold.txt")),
        // letter: 'ź' / 0x17a
        #[cfg(feature = "unicode-latin-extended-a")]
        'ź' => Some(include!("../res_rasterized_characters/0x17a_h20_wBold.txt")),
        // letter: 'Ż' / 0x17b
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ż' => Some(include!("../res_rasterized_characters/0x17b_h20_wBold.txt")),
        // letter: 'ż' / 0x17c
        #[cfg(feature = "unicode-latin-extended-a")]
        'ż' => Some(include!("../res_rasterized_characters/0x17c_h20_wBold.txt")),
        // letter: 'Ž' / 0x17d
        #[cfg(feature = "unicode-latin-extended-a")]
        'Ž' => Some(include!("../res_rasterized_characters/0x17d_h20_wBold.txt")),
        // letter: 'ž' / 0x17e
        #[cfg(feature = "unicode-latin-extended-a")]
        'ž' => Some(include!("../res_rasterized_characters/0x17e_h20_wBold.txt")),
        // letter: 'ſ' / 0x17f
        #[cfg(feature = "unicode-latin-extended-a")]
        'ſ' => Some(include!("../res_rasterized_characters/0x17f_h20_wBold.txt")),
        // letter: '�' / 0xfffd
        #[cfg(feature = "unicode-specials")]
        '�' => Some(include!(
            "../res_rasterized_characters/0xfffd_h20_wBold.txt"
        )),
        _ => None,
    }
}
