#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn using_other_iterator_trait_methods() -> u32 {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();

    sum
}

fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v1_iter = v1.iter();

    //assert_eq!(v1_iter.next(), Some(&1)); 
    //assert_eq!(v1_iter.next(), Some(&2)); 
    //assert_eq!(v1_iter.next(), Some(&3)); 
    //assert_eq!(v1_iter.next(), None); 

    for val in v1_iter {
        println!("Got: {}", val);
    }

    //let total: i32 = v1_iter.sum();
    //println!("{}", total);

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    
    println!("{:?}", v2);

    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_size(shoes, 10);
    println!("{:?}", in_my_size);
    println!("{:?}", using_other_iterator_trait_methods())
}
