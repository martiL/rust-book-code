use similar::{TextDiff};

extern crate serde;

use serde::{Serialize, Deserialize};

fn main() {
    // let diff = TextDiff::from_words(
    //     "Hello World\nThis is the second line.\nThis is the third.",
    //     "Hallo Welt\nThis is the second line.\nThis is life.\nMoar and more",
    // );

    // for change in diff.iter_all_changes() {
    //     let sign = match change.tag() {
    //         ChangeTag::Delete => "-",
    //         ChangeTag::Insert => "+",
    //         ChangeTag::Equal => "=",
    //     };
    //     print!("{}{}", sign, change);
    // }

    let diff = TextDiff::from_words(
        "Hello World\nThis is the second line.\nThis is the third.",
        "Hallo Welt\nThis is the second line.\nThis is life.\nMoar and more",
    );

    let all_changes = diff
        .ops()
        .iter()
        .flat_map(|op| diff.iter_changes(op))
        .collect::<Vec<_>>();

    #[derive(Serialize, Deserialize, Debug)]
    struct JsonData<'a> {
        value: &'a str
    }
    
    let d = JsonData {
        value: all_changes[0].value()
    };

    println!("{:?}", serde_json::to_string(&d));

}
