

type Word = fn(Vec<i32>);

struct Entry {
    key: String,
    func: Word,
}

enum CmdSeq {
    None,
    Tail { item: Entry },
    Next { item: Entry, next: Box<CmdSeq> }
}

fn main() {
    println!("Hello, world!");
}
