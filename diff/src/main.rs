use similar::{TextDiff, ChangeTag};

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
        value: &'a str,
        tag: &'a str,
        old_index: Option<usize>,
        new_index: Option<usize>,
    }

    // let d = JsonData {
    //     value: all_changes[0].value(),
    //     tag: match all_changes[0].tag() {
    //         ChangeTag::Delete => "delete",
    //         ChangeTag::Insert => "add",
    //         ChangeTag::Equal => "equal", 
    //     },
    //     old_index: all_changes[0].old_index(),
    //     new_index: all_changes[0].new_index()
    // };

    let json_array: Vec<_> = all_changes.iter().map(|v| JsonData {
        value: v.value(),
        tag: match v.tag() {
            ChangeTag::Delete => "delete",
            ChangeTag::Insert => "add",
            ChangeTag::Equal => "equal", 
        },
        old_index: v.old_index(),
        new_index: v.new_index()
    }).collect();

    println!("{:?}", serde_json::to_string(&json_array));
    //println!("{:?}", all_changes);

}
