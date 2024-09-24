fn main() {
    let p1 = Gorilla {
        name: String::from("Tony"),
    };
    let p2 = SpiderMonkey {
        name: String::from("Joel"),
        tail_size: 75,
    };

    p1.eat_banana();
    p2.eat_banana();
}

pub trait Primate {
    fn eat_banana(&self) {
        println!("some primate is eating a banana");
    }
}

struct Gorilla {
    name: String,
}

impl Primate for Gorilla {
    fn eat_banana(&self) {
        println!("A gorilla is eating banana")
    }
}

struct SpiderMonkey {
    name: String,
    tail_size: u8,
}

impl Primate for SpiderMonkey {}

trait Feline {}

impl dyn Primate {
    fn a() -> String {
        String::from("A")
    }
}

fn cute(a: impl Primate) {}
