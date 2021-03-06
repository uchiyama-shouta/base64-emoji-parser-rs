use std::collections::HashMap;

pub fn emoji_pattern() -> HashMap<String, &'static str> {
    let mut pattern = HashMap::new();
    pattern.insert("a".to_string(), "๐");
    pattern.insert("b".to_string(), "๐");
    pattern.insert("c".to_string(), "๐");
    pattern.insert("d".to_string(), "๐");
    pattern.insert("e".to_string(), "๐");
    pattern.insert("f".to_string(), "๐");
    pattern.insert("g".to_string(), "๐");
    pattern.insert("h".to_string(), "๐คฃ");
    pattern.insert("i".to_string(), "๐ฅฒ");
    pattern.insert("j".to_string(), "๐");
    pattern.insert("k".to_string(), "๐");
    pattern.insert("l".to_string(), "๐");
    pattern.insert("m".to_string(), "๐");
    pattern.insert("n".to_string(), "๐");
    pattern.insert("o".to_string(), "๐");
    pattern.insert("p".to_string(), "๐");
    pattern.insert("q".to_string(), "๐ฅฐ");
    pattern.insert("r".to_string(), "๐");
    pattern.insert("s".to_string(), "๐");
    pattern.insert("t".to_string(), "๐");
    pattern.insert("u".to_string(), "๐");
    pattern.insert("v".to_string(), "๐");
    pattern.insert("w".to_string(), "๐");
    pattern.insert("x".to_string(), "๐");
    pattern.insert("y".to_string(), "๐");
    pattern.insert("z".to_string(), "๐คช");

    pattern.insert("A".to_string(), "๐คจ");
    pattern.insert("B".to_string(), "๐ง");
    pattern.insert("C".to_string(), "๐ค");
    pattern.insert("D".to_string(), "๐");
    pattern.insert("E".to_string(), "๐ฅธ");
    pattern.insert("F".to_string(), "๐คฉ");
    pattern.insert("G".to_string(), "๐ฅณ");
    pattern.insert("H".to_string(), "๐");
    pattern.insert("I".to_string(), "๐");
    pattern.insert("J".to_string(), "๐");
    pattern.insert("K".to_string(), "๐");
    pattern.insert("L".to_string(), "๐");
    pattern.insert("M".to_string(), "๐");
    pattern.insert("N".to_string(), "๐");
    pattern.insert("O".to_string(), "๐ฃ");
    pattern.insert("P".to_string(), "๐");
    pattern.insert("Q".to_string(), "๐ซ");
    pattern.insert("R".to_string(), "๐ฉ");
    pattern.insert("S".to_string(), "๐ฅบ");
    pattern.insert("T".to_string(), "๐ข");
    pattern.insert("U".to_string(), "๐ญ");
    pattern.insert("V".to_string(), "๐ค");
    pattern.insert("W".to_string(), "๐?");
    pattern.insert("X".to_string(), "๐ก");
    pattern.insert("Y".to_string(), "๐คฌ");
    pattern.insert("Z".to_string(), "๐คฏ");

    pattern.insert("_".to_string(), "๐");
    pattern.insert("0".to_string(), "๐ณ");
    pattern.insert("1".to_string(), "๐ฅต");
    pattern.insert("2".to_string(), "๐ฅถ");
    pattern.insert("3".to_string(), "๐ฑ");
    pattern.insert("4".to_string(), "๐จ");
    pattern.insert("5".to_string(), "๐ฐ");
    pattern.insert("6".to_string(), "๐ฅ");
    pattern.insert("7".to_string(), "๐");
    pattern.insert("8".to_string(), "๐ค");
    pattern.insert("9".to_string(), "๐ค");
    pattern.insert("/".to_string(), "๐คญ");
    pattern.insert("+".to_string(), "๐คซ");
    pattern.insert("-".to_string(), "๐ถ");
    pattern.insert("=".to_string(), "๐คฅ");

    pattern
}
