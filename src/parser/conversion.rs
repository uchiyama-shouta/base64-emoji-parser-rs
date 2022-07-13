use super::emoji_pattern::emoji_pattern;

pub struct Conversion {}

impl Conversion {
    pub fn to_emoji(text: String) -> String {
        let pattern = emoji_pattern();
        let mut str = text;
        for (key, value) in pattern {
            str = str.split(&key).collect::<Vec<&str>>().join(&value)
        }

        str
    }

    pub fn to_base64(text: String) -> String {
        let pattern = emoji_pattern();
        let mut str = text;
        for (key, value) in pattern {
            str = str.split(&value).collect::<Vec<&str>>().join(&key)
        }

        str
    }
}
