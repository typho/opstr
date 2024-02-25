use crate::errors::Errors;
use crate::ops::traits;
use crate::output::{Output, OutputValue};

/*
The following python script was used to generate the combining characters list:

combining_characters_by_wikipedia = {65056, 65057, 65058, 65059, 65060, 65061, 65062, 65063, 65064, 65065, 65066, 65067, 65068, 65069, 65070, 65071, 6832, 6833, 6834, 6835, 6836, 6837, 6838, 6839, 6840, 6841, 6842, 6843, 6844, 6845, 6846, 6847, 6848, 6849, 6850, 6851, 6852, 6853, 6854, 6855, 6856, 6857, 6858, 6859, 6860, 6861, 6862, 8400, 8401, 8402, 8403, 8404, 8405, 8406, 8407, 8408, 8409, 8410, 8411, 8412, 8413, 8414, 8415, 8416, 8417, 8418, 8419, 8420, 8421, 8422, 8423, 8424, 8425, 8426, 8427, 8428, 8429, 8430, 8431, 8432, 768, 769, 770, 771, 772, 773, 774, 775, 776, 777, 778, 779, 780, 781, 782, 783, 784, 785, 786, 787, 788, 789, 790, 791, 792, 793, 794, 795, 796, 797, 798, 799, 800, 801, 802, 803, 804, 805, 806, 807, 808, 809, 810, 811, 812, 813, 814, 815, 816, 817, 818, 819, 820, 821, 822, 823, 824, 825, 826, 827, 828, 829, 830, 831, 832, 833, 834, 835, 836, 837, 838, 839, 840, 841, 842, 843, 844, 845, 846, 847, 848, 849, 850, 851, 852, 853, 854, 855, 856, 857, 858, 859, 860, 861, 862, 863, 864, 865, 866, 867, 868, 869, 870, 871, 872, 873, 874, 875, 876, 877, 878, 879, 7616, 7617, 7618, 7619, 7620, 7621, 7622, 7623, 7624, 7625, 7626, 7627, 7628, 7629, 7630, 7631, 7632, 7633, 7634, 7635, 7636, 7637, 7638, 7639, 7640, 7641, 7642, 7643, 7644, 7645, 7646, 7647, 7648, 7649, 7650, 7651, 7652, 7653, 7654, 7655, 7656, 7657, 7658, 7659, 7660, 7661, 7662, 7663, 7664, 7665, 7666, 7667, 7668, 7669, 7670, 7671, 7672, 7673, 7674, 7675, 7676, 7677, 7678, 7679}
python_unknown_names = {6849: 'COMBINING LEFT PARENTHESIS ABOVE LEFT', 6850: 'COMBINING RIGHT PARENTHESIS ABOVE RIGHT', 6851: 'COMBINING LEFT PARENTHESIS BELOW LEFT', 6852: 'COMBINING RIGHT PARENTHESIS BELOW RIGHT', 6853: 'COMBINING SQUARE BRACKETS ABOVE', 6854: 'COMBINING NUMBER SIGN ABOVE', 6855: 'COMBINING INVERTED DOUBLE ARCH ABOVE', 6856: 'COMBINING PLUS SIGN ABOVE', 6857: 'COMBINING DOUBLE PLUS SIGN ABOVE', 6858: 'COMBINING DOUBLE PLUS SIGN BELOW', 6859: 'COMBINING TRIPLE ACUTE ACCENT', 6860: 'COMBINING LATIN SMALL LETTER INSULAR G', 6861: 'COMBINING LATIN SMALL LETTER INSULAR R', 6862: 'COMBINING LATIN SMALL LETTER INSULAR T', 7674: 'COMBINING DOT BELOW LEFT'}

lookup = lambda cp: python_unknown_names[cp] if cp in python_unknown_names else unicodedata.name(chr(cp))
for i in sorted(combining_characters_by_wikipedia):
  #print('U+{:04X}  {}'.format(i, lookup(i)))
  #print('data.insert("\\u{{{:4X}}}", "{}");'.format(i, lookup(i)))
  print('vec![s("U+{:04X}"), s("{}")],'.format(i, lookup(i)))

*/

pub struct CombiningCodepointList {}

impl traits::OpZero for CombiningCodepointList {
    fn name() -> &'static str { "combining-codepoint-list" }
    fn description() -> &'static str { "list all codepoints with a combining property" }

    fn priority() -> f32 { 0.01 }

    fn run() -> Result<Output, Errors> {
        let s = |s: &str| { OutputValue::SingleLineText(s.to_owned()) };
        let data = vec![
            vec![s("U+0300"), s("COMBINING GRAVE ACCENT")],
            vec![s("U+0301"), s("COMBINING ACUTE ACCENT")],
            vec![s("U+0302"), s("COMBINING CIRCUMFLEX ACCENT")],
            vec![s("U+0303"), s("COMBINING TILDE")],
            vec![s("U+0304"), s("COMBINING MACRON")],
            vec![s("U+0305"), s("COMBINING OVERLINE")],
            vec![s("U+0306"), s("COMBINING BREVE")],
            vec![s("U+0307"), s("COMBINING DOT ABOVE")],
            vec![s("U+0308"), s("COMBINING DIAERESIS")],
            vec![s("U+0309"), s("COMBINING HOOK ABOVE")],
            vec![s("U+030A"), s("COMBINING RING ABOVE")],
            vec![s("U+030B"), s("COMBINING DOUBLE ACUTE ACCENT")],
            vec![s("U+030C"), s("COMBINING CARON")],
            vec![s("U+030D"), s("COMBINING VERTICAL LINE ABOVE")],
            vec![s("U+030E"), s("COMBINING DOUBLE VERTICAL LINE ABOVE")],
            vec![s("U+030F"), s("COMBINING DOUBLE GRAVE ACCENT")],
            vec![s("U+0310"), s("COMBINING CANDRABINDU")],
            vec![s("U+0311"), s("COMBINING INVERTED BREVE")],
            vec![s("U+0312"), s("COMBINING TURNED COMMA ABOVE")],
            vec![s("U+0313"), s("COMBINING COMMA ABOVE")],
            vec![s("U+0314"), s("COMBINING REVERSED COMMA ABOVE")],
            vec![s("U+0315"), s("COMBINING COMMA ABOVE RIGHT")],
            vec![s("U+0316"), s("COMBINING GRAVE ACCENT BELOW")],
            vec![s("U+0317"), s("COMBINING ACUTE ACCENT BELOW")],
            vec![s("U+0318"), s("COMBINING LEFT TACK BELOW")],
            vec![s("U+0319"), s("COMBINING RIGHT TACK BELOW")],
            vec![s("U+031A"), s("COMBINING LEFT ANGLE ABOVE")],
            vec![s("U+031B"), s("COMBINING HORN")],
            vec![s("U+031C"), s("COMBINING LEFT HALF RING BELOW")],
            vec![s("U+031D"), s("COMBINING UP TACK BELOW")],
            vec![s("U+031E"), s("COMBINING DOWN TACK BELOW")],
            vec![s("U+031F"), s("COMBINING PLUS SIGN BELOW")],
            vec![s("U+0320"), s("COMBINING MINUS SIGN BELOW")],
            vec![s("U+0321"), s("COMBINING PALATALIZED HOOK BELOW")],
            vec![s("U+0322"), s("COMBINING RETROFLEX HOOK BELOW")],
            vec![s("U+0323"), s("COMBINING DOT BELOW")],
            vec![s("U+0324"), s("COMBINING DIAERESIS BELOW")],
            vec![s("U+0325"), s("COMBINING RING BELOW")],
            vec![s("U+0326"), s("COMBINING COMMA BELOW")],
            vec![s("U+0327"), s("COMBINING CEDILLA")],
            vec![s("U+0328"), s("COMBINING OGONEK")],
            vec![s("U+0329"), s("COMBINING VERTICAL LINE BELOW")],
            vec![s("U+032A"), s("COMBINING BRIDGE BELOW")],
            vec![s("U+032B"), s("COMBINING INVERTED DOUBLE ARCH BELOW")],
            vec![s("U+032C"), s("COMBINING CARON BELOW")],
            vec![s("U+032D"), s("COMBINING CIRCUMFLEX ACCENT BELOW")],
            vec![s("U+032E"), s("COMBINING BREVE BELOW")],
            vec![s("U+032F"), s("COMBINING INVERTED BREVE BELOW")],
            vec![s("U+0330"), s("COMBINING TILDE BELOW")],
            vec![s("U+0331"), s("COMBINING MACRON BELOW")],
            vec![s("U+0332"), s("COMBINING LOW LINE")],
            vec![s("U+0333"), s("COMBINING DOUBLE LOW LINE")],
            vec![s("U+0334"), s("COMBINING TILDE OVERLAY")],
            vec![s("U+0335"), s("COMBINING SHORT STROKE OVERLAY")],
            vec![s("U+0336"), s("COMBINING LONG STROKE OVERLAY")],
            vec![s("U+0337"), s("COMBINING SHORT SOLIDUS OVERLAY")],
            vec![s("U+0338"), s("COMBINING LONG SOLIDUS OVERLAY")],
            vec![s("U+0339"), s("COMBINING RIGHT HALF RING BELOW")],
            vec![s("U+033A"), s("COMBINING INVERTED BRIDGE BELOW")],
            vec![s("U+033B"), s("COMBINING SQUARE BELOW")],
            vec![s("U+033C"), s("COMBINING SEAGULL BELOW")],
            vec![s("U+033D"), s("COMBINING X ABOVE")],
            vec![s("U+033E"), s("COMBINING VERTICAL TILDE")],
            vec![s("U+033F"), s("COMBINING DOUBLE OVERLINE")],
            vec![s("U+0340"), s("COMBINING GRAVE TONE MARK")],
            vec![s("U+0341"), s("COMBINING ACUTE TONE MARK")],
            vec![s("U+0342"), s("COMBINING GREEK PERISPOMENI")],
            vec![s("U+0343"), s("COMBINING GREEK KORONIS")],
            vec![s("U+0344"), s("COMBINING GREEK DIALYTIKA TONOS")],
            vec![s("U+0345"), s("COMBINING GREEK YPOGEGRAMMENI")],
            vec![s("U+0346"), s("COMBINING BRIDGE ABOVE")],
            vec![s("U+0347"), s("COMBINING EQUALS SIGN BELOW")],
            vec![s("U+0348"), s("COMBINING DOUBLE VERTICAL LINE BELOW")],
            vec![s("U+0349"), s("COMBINING LEFT ANGLE BELOW")],
            vec![s("U+034A"), s("COMBINING NOT TILDE ABOVE")],
            vec![s("U+034B"), s("COMBINING HOMOTHETIC ABOVE")],
            vec![s("U+034C"), s("COMBINING ALMOST EQUAL TO ABOVE")],
            vec![s("U+034D"), s("COMBINING LEFT RIGHT ARROW BELOW")],
            vec![s("U+034E"), s("COMBINING UPWARDS ARROW BELOW")],
            vec![s("U+034F"), s("COMBINING GRAPHEME JOINER")],
            vec![s("U+0350"), s("COMBINING RIGHT ARROWHEAD ABOVE")],
            vec![s("U+0351"), s("COMBINING LEFT HALF RING ABOVE")],
            vec![s("U+0352"), s("COMBINING FERMATA")],
            vec![s("U+0353"), s("COMBINING X BELOW")],
            vec![s("U+0354"), s("COMBINING LEFT ARROWHEAD BELOW")],
            vec![s("U+0355"), s("COMBINING RIGHT ARROWHEAD BELOW")],
            vec![s("U+0356"), s("COMBINING RIGHT ARROWHEAD AND UP ARROWHEAD BELOW")],
            vec![s("U+0357"), s("COMBINING RIGHT HALF RING ABOVE")],
            vec![s("U+0358"), s("COMBINING DOT ABOVE RIGHT")],
            vec![s("U+0359"), s("COMBINING ASTERISK BELOW")],
            vec![s("U+035A"), s("COMBINING DOUBLE RING BELOW")],
            vec![s("U+035B"), s("COMBINING ZIGZAG ABOVE")],
            vec![s("U+035C"), s("COMBINING DOUBLE BREVE BELOW")],
            vec![s("U+035D"), s("COMBINING DOUBLE BREVE")],
            vec![s("U+035E"), s("COMBINING DOUBLE MACRON")],
            vec![s("U+035F"), s("COMBINING DOUBLE MACRON BELOW")],
            vec![s("U+0360"), s("COMBINING DOUBLE TILDE")],
            vec![s("U+0361"), s("COMBINING DOUBLE INVERTED BREVE")],
            vec![s("U+0362"), s("COMBINING DOUBLE RIGHTWARDS ARROW BELOW")],
            vec![s("U+0363"), s("COMBINING LATIN SMALL LETTER A")],
            vec![s("U+0364"), s("COMBINING LATIN SMALL LETTER E")],
            vec![s("U+0365"), s("COMBINING LATIN SMALL LETTER I")],
            vec![s("U+0366"), s("COMBINING LATIN SMALL LETTER O")],
            vec![s("U+0367"), s("COMBINING LATIN SMALL LETTER U")],
            vec![s("U+0368"), s("COMBINING LATIN SMALL LETTER C")],
            vec![s("U+0369"), s("COMBINING LATIN SMALL LETTER D")],
            vec![s("U+036A"), s("COMBINING LATIN SMALL LETTER H")],
            vec![s("U+036B"), s("COMBINING LATIN SMALL LETTER M")],
            vec![s("U+036C"), s("COMBINING LATIN SMALL LETTER R")],
            vec![s("U+036D"), s("COMBINING LATIN SMALL LETTER T")],
            vec![s("U+036E"), s("COMBINING LATIN SMALL LETTER V")],
            vec![s("U+036F"), s("COMBINING LATIN SMALL LETTER X")],
            vec![s("U+1AB0"), s("COMBINING DOUBLED CIRCUMFLEX ACCENT")],
            vec![s("U+1AB1"), s("COMBINING DIAERESIS-RING")],
            vec![s("U+1AB2"), s("COMBINING INFINITY")],
            vec![s("U+1AB3"), s("COMBINING DOWNWARDS ARROW")],
            vec![s("U+1AB4"), s("COMBINING TRIPLE DOT")],
            vec![s("U+1AB5"), s("COMBINING X-X BELOW")],
            vec![s("U+1AB6"), s("COMBINING WIGGLY LINE BELOW")],
            vec![s("U+1AB7"), s("COMBINING OPEN MARK BELOW")],
            vec![s("U+1AB8"), s("COMBINING DOUBLE OPEN MARK BELOW")],
            vec![s("U+1AB9"), s("COMBINING LIGHT CENTRALIZATION STROKE BELOW")],
            vec![s("U+1ABA"), s("COMBINING STRONG CENTRALIZATION STROKE BELOW")],
            vec![s("U+1ABB"), s("COMBINING PARENTHESES ABOVE")],
            vec![s("U+1ABC"), s("COMBINING DOUBLE PARENTHESES ABOVE")],
            vec![s("U+1ABD"), s("COMBINING PARENTHESES BELOW")],
            vec![s("U+1ABE"), s("COMBINING PARENTHESES OVERLAY")],
            vec![s("U+1ABF"), s("COMBINING LATIN SMALL LETTER W BELOW")],
            vec![s("U+1AC0"), s("COMBINING LATIN SMALL LETTER TURNED W BELOW")],
            vec![s("U+1AC1"), s("COMBINING LEFT PARENTHESIS ABOVE LEFT")],
            vec![s("U+1AC2"), s("COMBINING RIGHT PARENTHESIS ABOVE RIGHT")],
            vec![s("U+1AC3"), s("COMBINING LEFT PARENTHESIS BELOW LEFT")],
            vec![s("U+1AC4"), s("COMBINING RIGHT PARENTHESIS BELOW RIGHT")],
            vec![s("U+1AC5"), s("COMBINING SQUARE BRACKETS ABOVE")],
            vec![s("U+1AC6"), s("COMBINING NUMBER SIGN ABOVE")],
            vec![s("U+1AC7"), s("COMBINING INVERTED DOUBLE ARCH ABOVE")],
            vec![s("U+1AC8"), s("COMBINING PLUS SIGN ABOVE")],
            vec![s("U+1AC9"), s("COMBINING DOUBLE PLUS SIGN ABOVE")],
            vec![s("U+1ACA"), s("COMBINING DOUBLE PLUS SIGN BELOW")],
            vec![s("U+1ACB"), s("COMBINING TRIPLE ACUTE ACCENT")],
            vec![s("U+1ACC"), s("COMBINING LATIN SMALL LETTER INSULAR G")],
            vec![s("U+1ACD"), s("COMBINING LATIN SMALL LETTER INSULAR R")],
            vec![s("U+1ACE"), s("COMBINING LATIN SMALL LETTER INSULAR T")],
            vec![s("U+1DC0"), s("COMBINING DOTTED GRAVE ACCENT")],
            vec![s("U+1DC1"), s("COMBINING DOTTED ACUTE ACCENT")],
            vec![s("U+1DC2"), s("COMBINING SNAKE BELOW")],
            vec![s("U+1DC3"), s("COMBINING SUSPENSION MARK")],
            vec![s("U+1DC4"), s("COMBINING MACRON-ACUTE")],
            vec![s("U+1DC5"), s("COMBINING GRAVE-MACRON")],
            vec![s("U+1DC6"), s("COMBINING MACRON-GRAVE")],
            vec![s("U+1DC7"), s("COMBINING ACUTE-MACRON")],
            vec![s("U+1DC8"), s("COMBINING GRAVE-ACUTE-GRAVE")],
            vec![s("U+1DC9"), s("COMBINING ACUTE-GRAVE-ACUTE")],
            vec![s("U+1DCA"), s("COMBINING LATIN SMALL LETTER R BELOW")],
            vec![s("U+1DCB"), s("COMBINING BREVE-MACRON")],
            vec![s("U+1DCC"), s("COMBINING MACRON-BREVE")],
            vec![s("U+1DCD"), s("COMBINING DOUBLE CIRCUMFLEX ABOVE")],
            vec![s("U+1DCE"), s("COMBINING OGONEK ABOVE")],
            vec![s("U+1DCF"), s("COMBINING ZIGZAG BELOW")],
            vec![s("U+1DD0"), s("COMBINING IS BELOW")],
            vec![s("U+1DD1"), s("COMBINING UR ABOVE")],
            vec![s("U+1DD2"), s("COMBINING US ABOVE")],
            vec![s("U+1DD3"), s("COMBINING LATIN SMALL LETTER FLATTENED OPEN A ABOVE")],
            vec![s("U+1DD4"), s("COMBINING LATIN SMALL LETTER AE")],
            vec![s("U+1DD5"), s("COMBINING LATIN SMALL LETTER AO")],
            vec![s("U+1DD6"), s("COMBINING LATIN SMALL LETTER AV")],
            vec![s("U+1DD7"), s("COMBINING LATIN SMALL LETTER C CEDILLA")],
            vec![s("U+1DD8"), s("COMBINING LATIN SMALL LETTER INSULAR D")],
            vec![s("U+1DD9"), s("COMBINING LATIN SMALL LETTER ETH")],
            vec![s("U+1DDA"), s("COMBINING LATIN SMALL LETTER G")],
            vec![s("U+1DDB"), s("COMBINING LATIN LETTER SMALL CAPITAL G")],
            vec![s("U+1DDC"), s("COMBINING LATIN SMALL LETTER K")],
            vec![s("U+1DDD"), s("COMBINING LATIN SMALL LETTER L")],
            vec![s("U+1DDE"), s("COMBINING LATIN LETTER SMALL CAPITAL L")],
            vec![s("U+1DDF"), s("COMBINING LATIN LETTER SMALL CAPITAL M")],
            vec![s("U+1DE0"), s("COMBINING LATIN SMALL LETTER N")],
            vec![s("U+1DE1"), s("COMBINING LATIN LETTER SMALL CAPITAL N")],
            vec![s("U+1DE2"), s("COMBINING LATIN LETTER SMALL CAPITAL R")],
            vec![s("U+1DE3"), s("COMBINING LATIN SMALL LETTER R ROTUNDA")],
            vec![s("U+1DE4"), s("COMBINING LATIN SMALL LETTER S")],
            vec![s("U+1DE5"), s("COMBINING LATIN SMALL LETTER LONG S")],
            vec![s("U+1DE6"), s("COMBINING LATIN SMALL LETTER Z")],
            vec![s("U+1DE7"), s("COMBINING LATIN SMALL LETTER ALPHA")],
            vec![s("U+1DE8"), s("COMBINING LATIN SMALL LETTER B")],
            vec![s("U+1DE9"), s("COMBINING LATIN SMALL LETTER BETA")],
            vec![s("U+1DEA"), s("COMBINING LATIN SMALL LETTER SCHWA")],
            vec![s("U+1DEB"), s("COMBINING LATIN SMALL LETTER F")],
            vec![s("U+1DEC"), s("COMBINING LATIN SMALL LETTER L WITH DOUBLE MIDDLE TILDE")],
            vec![s("U+1DED"), s("COMBINING LATIN SMALL LETTER O WITH LIGHT CENTRALIZATION STROKE")],
            vec![s("U+1DEE"), s("COMBINING LATIN SMALL LETTER P")],
            vec![s("U+1DEF"), s("COMBINING LATIN SMALL LETTER ESH")],
            vec![s("U+1DF0"), s("COMBINING LATIN SMALL LETTER U WITH LIGHT CENTRALIZATION STROKE")],
            vec![s("U+1DF1"), s("COMBINING LATIN SMALL LETTER W")],
            vec![s("U+1DF2"), s("COMBINING LATIN SMALL LETTER A WITH DIAERESIS")],
            vec![s("U+1DF3"), s("COMBINING LATIN SMALL LETTER O WITH DIAERESIS")],
            vec![s("U+1DF4"), s("COMBINING LATIN SMALL LETTER U WITH DIAERESIS")],
            vec![s("U+1DF5"), s("COMBINING UP TACK ABOVE")],
            vec![s("U+1DF6"), s("COMBINING KAVYKA ABOVE RIGHT")],
            vec![s("U+1DF7"), s("COMBINING KAVYKA ABOVE LEFT")],
            vec![s("U+1DF8"), s("COMBINING DOT ABOVE LEFT")],
            vec![s("U+1DF9"), s("COMBINING WIDE INVERTED BRIDGE BELOW")],
            vec![s("U+1DFA"), s("COMBINING DOT BELOW LEFT")],
            vec![s("U+1DFB"), s("COMBINING DELETION MARK")],
            vec![s("U+1DFC"), s("COMBINING DOUBLE INVERTED BREVE BELOW")],
            vec![s("U+1DFD"), s("COMBINING ALMOST EQUAL TO BELOW")],
            vec![s("U+1DFE"), s("COMBINING LEFT ARROWHEAD ABOVE")],
            vec![s("U+1DFF"), s("COMBINING RIGHT ARROWHEAD AND DOWN ARROWHEAD BELOW")],
            vec![s("U+20D0"), s("COMBINING LEFT HARPOON ABOVE")],
            vec![s("U+20D1"), s("COMBINING RIGHT HARPOON ABOVE")],
            vec![s("U+20D2"), s("COMBINING LONG VERTICAL LINE OVERLAY")],
            vec![s("U+20D3"), s("COMBINING SHORT VERTICAL LINE OVERLAY")],
            vec![s("U+20D4"), s("COMBINING ANTICLOCKWISE ARROW ABOVE")],
            vec![s("U+20D5"), s("COMBINING CLOCKWISE ARROW ABOVE")],
            vec![s("U+20D6"), s("COMBINING LEFT ARROW ABOVE")],
            vec![s("U+20D7"), s("COMBINING RIGHT ARROW ABOVE")],
            vec![s("U+20D8"), s("COMBINING RING OVERLAY")],
            vec![s("U+20D9"), s("COMBINING CLOCKWISE RING OVERLAY")],
            vec![s("U+20DA"), s("COMBINING ANTICLOCKWISE RING OVERLAY")],
            vec![s("U+20DB"), s("COMBINING THREE DOTS ABOVE")],
            vec![s("U+20DC"), s("COMBINING FOUR DOTS ABOVE")],
            vec![s("U+20DD"), s("COMBINING ENCLOSING CIRCLE")],
            vec![s("U+20DE"), s("COMBINING ENCLOSING SQUARE")],
            vec![s("U+20DF"), s("COMBINING ENCLOSING DIAMOND")],
            vec![s("U+20E0"), s("COMBINING ENCLOSING CIRCLE BACKSLASH")],
            vec![s("U+20E1"), s("COMBINING LEFT RIGHT ARROW ABOVE")],
            vec![s("U+20E2"), s("COMBINING ENCLOSING SCREEN")],
            vec![s("U+20E3"), s("COMBINING ENCLOSING KEYCAP")],
            vec![s("U+20E4"), s("COMBINING ENCLOSING UPWARD POINTING TRIANGLE")],
            vec![s("U+20E5"), s("COMBINING REVERSE SOLIDUS OVERLAY")],
            vec![s("U+20E6"), s("COMBINING DOUBLE VERTICAL STROKE OVERLAY")],
            vec![s("U+20E7"), s("COMBINING ANNUITY SYMBOL")],
            vec![s("U+20E8"), s("COMBINING TRIPLE UNDERDOT")],
            vec![s("U+20E9"), s("COMBINING WIDE BRIDGE ABOVE")],
            vec![s("U+20EA"), s("COMBINING LEFTWARDS ARROW OVERLAY")],
            vec![s("U+20EB"), s("COMBINING LONG DOUBLE SOLIDUS OVERLAY")],
            vec![s("U+20EC"), s("COMBINING RIGHTWARDS HARPOON WITH BARB DOWNWARDS")],
            vec![s("U+20ED"), s("COMBINING LEFTWARDS HARPOON WITH BARB DOWNWARDS")],
            vec![s("U+20EE"), s("COMBINING LEFT ARROW BELOW")],
            vec![s("U+20EF"), s("COMBINING RIGHT ARROW BELOW")],
            vec![s("U+20F0"), s("COMBINING ASTERISK ABOVE")],
            vec![s("U+FE20"), s("COMBINING LIGATURE LEFT HALF")],
            vec![s("U+FE21"), s("COMBINING LIGATURE RIGHT HALF")],
            vec![s("U+FE22"), s("COMBINING DOUBLE TILDE LEFT HALF")],
            vec![s("U+FE23"), s("COMBINING DOUBLE TILDE RIGHT HALF")],
            vec![s("U+FE24"), s("COMBINING MACRON LEFT HALF")],
            vec![s("U+FE25"), s("COMBINING MACRON RIGHT HALF")],
            vec![s("U+FE26"), s("COMBINING CONJOINING MACRON")],
            vec![s("U+FE27"), s("COMBINING LIGATURE LEFT HALF BELOW")],
            vec![s("U+FE28"), s("COMBINING LIGATURE RIGHT HALF BELOW")],
            vec![s("U+FE29"), s("COMBINING TILDE LEFT HALF BELOW")],
            vec![s("U+FE2A"), s("COMBINING TILDE RIGHT HALF BELOW")],
            vec![s("U+FE2B"), s("COMBINING MACRON LEFT HALF BELOW")],
            vec![s("U+FE2C"), s("COMBINING MACRON RIGHT HALF BELOW")],
            vec![s("U+FE2D"), s("COMBINING CONJOINING MACRON BELOW")],
            vec![s("U+FE2E"), s("COMBINING CYRILLIC TITLO LEFT HALF")],
            vec![s("U+FE2F"), s("COMBINING CYRILLIC TITLO RIGHT HALF")],
        ];

        Ok(Output::Table { data, column_headers: vec!["codepoint".to_owned(), "name".to_owned()], notes: vec![] })
    }
}


