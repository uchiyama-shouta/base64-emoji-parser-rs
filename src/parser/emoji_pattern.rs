use std::collections::HashMap;

pub fn emoji_pattern() -> HashMap<String, &'static str> {
    let mut pattern = HashMap::new();
    pattern.insert("a".to_string(), "😀");
    pattern.insert("b".to_string(), "😃");
    pattern.insert("c".to_string(), "😄");
    pattern.insert("d".to_string(), "😁");
    pattern.insert("e".to_string(), "😆");
    pattern.insert("f".to_string(), "😅");
    pattern.insert("g".to_string(), "😂");
    pattern.insert("h".to_string(), "🤣");
    pattern.insert("i".to_string(), "🥲");
    pattern.insert("j".to_string(), "😊");
    pattern.insert("k".to_string(), "😇");
    pattern.insert("l".to_string(), "🙂");
    pattern.insert("m".to_string(), "🙃");
    pattern.insert("n".to_string(), "😉");
    pattern.insert("o".to_string(), "😌");
    pattern.insert("p".to_string(), "😍");
    pattern.insert("q".to_string(), "🥰");
    pattern.insert("r".to_string(), "😘");
    pattern.insert("s".to_string(), "😗");
    pattern.insert("t".to_string(), "😙");
    pattern.insert("u".to_string(), "😚");
    pattern.insert("v".to_string(), "😋");
    pattern.insert("w".to_string(), "😛");
    pattern.insert("x".to_string(), "😝");
    pattern.insert("y".to_string(), "😜");
    pattern.insert("z".to_string(), "🤪");

    pattern.insert("A".to_string(), "🤨");
    pattern.insert("B".to_string(), "🧐");
    pattern.insert("C".to_string(), "🤓");
    pattern.insert("D".to_string(), "😎");
    pattern.insert("E".to_string(), "🥸");
    pattern.insert("F".to_string(), "🤩");
    pattern.insert("G".to_string(), "🥳");
    pattern.insert("H".to_string(), "😏");
    pattern.insert("I".to_string(), "😒");
    pattern.insert("J".to_string(), "😞");
    pattern.insert("K".to_string(), "😔");
    pattern.insert("L".to_string(), "😟");
    pattern.insert("M".to_string(), "😕");
    pattern.insert("N".to_string(), "🙁");
    pattern.insert("O".to_string(), "😣");
    pattern.insert("P".to_string(), "😖");
    pattern.insert("Q".to_string(), "😫");
    pattern.insert("R".to_string(), "😩");
    pattern.insert("S".to_string(), "🥺");
    pattern.insert("T".to_string(), "😢");
    pattern.insert("U".to_string(), "😭");
    pattern.insert("V".to_string(), "😤");
    pattern.insert("W".to_string(), "😠");
    pattern.insert("X".to_string(), "😡");
    pattern.insert("Y".to_string(), "🤬");
    pattern.insert("Z".to_string(), "🤯");

    pattern.insert("_".to_string(), "😐");
    pattern.insert("0".to_string(), "😳");
    pattern.insert("1".to_string(), "🥵");
    pattern.insert("2".to_string(), "🥶");
    pattern.insert("3".to_string(), "😱");
    pattern.insert("4".to_string(), "😨");
    pattern.insert("5".to_string(), "😰");
    pattern.insert("6".to_string(), "😥");
    pattern.insert("7".to_string(), "😓");
    pattern.insert("8".to_string(), "🤗");
    pattern.insert("9".to_string(), "🤔");
    pattern.insert("/".to_string(), "🤭");
    pattern.insert("+".to_string(), "🤫");
    pattern.insert("-".to_string(), "😶");
    pattern.insert("=".to_string(), "🤥");

    pattern
}
