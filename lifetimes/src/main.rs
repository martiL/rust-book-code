// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
   if x.len() > y.len() {
      x
   } else {
      y
   }
}

struct ImportantExcerpt<'a> {
   part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
   fn level(&self) -> i32 {
       3
   }
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
   // let r;
   
   // {
   //    let x = 5;
   //    r = &x;
   // }
   
   // println!("r: {}", r);

   // let string1 = String::from("abcd");
   // let string2 = "xyz";

   // let result = longest(string1.as_str(), string2);
   // println!("the longest string is {}", result)

   // compiler should approve, because string1 is valid until the end of the outer scope, string2 is valid until the end of the inner scope, and result references something that is valid until the end of the inner scope
   // let string1 = String::from("abcd");
   
   // {
   //    let string2 = "xyz";
   //    let result = longest(string1.as_str(), string2);
   //    println!("the longest string is {}", result)
   // }

   // compiler should not approve, because We’ll move the declaration of the result variable outside the inner scope but leave the assignment of the value to the result variable inside the scope with string2. Then we’ll move the println! that uses result outside the inner scope, after the inner scope has ended. 
   // let string1 = String::from("long string is long");
   // let result;
   // {
   //     let string2 = String::from("xyz");
   //     result = longest(string1.as_str(), string2.as_str());
   // }
   // println!("The longest string is {}", result);

   let novel = String::from("Call me Ishmael. Some years ago...");
   let first_sentence = novel.split('.').next().expect("Could not find a '.'");
   let i = ImportantExcerpt {
         part: first_sentence,
   };
}
