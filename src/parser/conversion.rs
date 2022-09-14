use super::emoji_pattern::emoji_pattern;
use base64;

pub struct Conversion {}

impl Conversion {
    pub fn to_emoji(text: String) -> String {
        let pattern = emoji_pattern();
        let mut str = base64::encode(text);
        println!("{}", str);
        for (key, value) in pattern {
            str = str.split(&key).collect::<Vec<&str>>().join(&value)
        }

        str
    }

    pub fn to_base64(text: String) -> String {
        let pattern = emoji_pattern();
        let mut str: String = text;
        for (key, value) in pattern {
            str = str.split(&value).collect::<Vec<&str>>().join(&key)
        }

        let bytes = base64::decode(&str).unwrap();
        let content = String::from_utf8(bytes.to_vec()).unwrap();

        content
    }
}


#[cfg(test)]
mod tests {
    use crate::parser::conversion::Conversion;

    #[test]
    fn exploration() {
        let text = "こんにちは、世界".to_string();
        assert_eq!(Conversion::to_base64(Conversion::to_emoji(text.clone())), text);
    }
}
