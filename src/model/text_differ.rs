// text difference

use similar::{ChangeTag, TextDiff};

pub struct TextDifference {}

impl TextDifference {
    pub fn differ(text1: &str, text2: &str) -> Vec<(String, String)> {
        let diff = TextDiff::from_words(text1, text2);

        diff.iter_all_changes()
            .map(|change| {
                let sign = match change.tag() {
                    ChangeTag::Delete => String::from("-"),
                    ChangeTag::Insert => String::from("+"),
                    ChangeTag::Equal => String::from("="),
                };
                (String::from(change.as_str().unwrap()), sign)
            })
            .collect()
    }
}

#[cfg(test)]
mod test {

    use similar::{ChangeTag, TextDiff};

    #[test]
    fn test_diff() {
        let diff = TextDiff::from_lines(
            "Hello World\nThis is the second line.\nThis is the third.",
            "Hello World\nThis is the second line.\nThis is life.\nMoar and more",
        );

        // let all_changes = diff
        //     .ops()
        //     .iter()
        //     .flat_map(|op| diff.iter_inline_changes(op))
        //     .collect::<Vec<_>>();
        // println!("{}", serde_json::to_string_pretty(&all_changes).unwrap());

        for change in diff.iter_all_changes() {
            let sign = match change.tag() {
                ChangeTag::Delete => "-",
                ChangeTag::Insert => "+",
                ChangeTag::Equal => "=",
            };
            print!("{}{}", sign, change);
        }
    }
}
