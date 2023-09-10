use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
use std::sync::Arc;

type KfnFn = fn(&mut Vec<Number>);

#[derive (Clone)]
enum Number {
    Int(i32),
    Float(f64),
}

enum Storage {
    DictWord(Arc<[Token]>),
    DictVar(Variable),
}

enum Token {
    Word(Arc<[Token]>),
    Func(KfnFn),
    Var(Variable),
    Num(Number),
    Err,
}

struct DictEntry{
    name: String,
    data: Storage
}

struct Memory {
    mem: Vec<u8>,
    new_memory: usize,
}

struct Variable {
    addr: usize,
    size: u32
}

impl Memory {
    fn create_mem() -> Memory {
        Memory {
            mem: Vec::with_capacity(200),
            new_memory: 0
        }
    }

    fn alloc(&mut self, a: i32) -> usize {
        let s = a.to_ne_bytes();
        self.mem.extend_from_slice(&s);
        let temp = self.new_memory;
        self.new_memory += s.len();
        temp
    }

    fn read(&self, var: &Variable) -> i32 {
        i32::from_ne_bytes(self.mem[var.addr..(var.addr+4)].try_into().unwrap())
    }
}

fn find_word(dict: &Vec<DictEntry>, n: String) -> Option<Storage> {
    for i in dict {
        let DictEntry { name, data } = i;
        if name == &n {
            return Some(*data);
        }
    }
    None
}


fn is_number(s: &String) -> bool {
    s.parse::<i32>().is_ok() || s.parse::<f64>().is_ok()
}

fn parse_to_number(s: String) -> Number {
    match s.parse::<i32>() {
        Ok(i) => Number::Int(i),
        Err(_err) => match s.parse::<f64>() {
            Ok(f) => Number::Float(f),
            Err(_err) => Number::Int(0i32),
        }
    }
}

fn tokenize(s: String, dict: &Vec<DictEntry>) -> Token {
    if is_number(&s) {
        return Token::Num(parse_to_number(s));
    } else {
        let nnum = find_word(dict, s);
        match nnum {
            Some(data) => {
                match data {
                    Storage::DictWord(token_lst) => { return Token::Word(token_lst); },
                    Storage::DictVar(var) => {return Token::Var(var);},
                }
            },
            None => {return Token::Err;}
        }
    }
}

fn init_dict() -> Vec<DictEntry> {
    vec![
        DictEntry { name: String::from("."), data: Storage::DictWord(Arc::new([Token::Func(kfn_pop)]))}
    ]
}

fn kfn_pop(st: &mut Vec<Number>) {
    let num = st.pop();
    if let None = num {
        println!("Stack underflow.");
    } else {
        match num {
            Some(Number::Int(i)) => {print!("{i}");}
            Some(Number::Float(f)) => {print!("{f}");}
            None => {println!("Stack underflow.")}
        }
    }
}

fn main() -> Result<()> {
    let mut mem = Memory::create_mem();
    let mut stack: Vec<Number> = Vec::new();
    let mut dict: Vec<DictEntry> = init_dict();
    let mut rl = DefaultEditor::new()?;
    #[cfg(feature = "with-file-history")]
    if let _ = rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    loop {
        let readline = rl.readline("? ");
        match readline {
                     rustyline::Result::Ok(line) => {
                         let _ = rl.add_history_entry(line.as_str());
                         let args_str: Vec<&str> = line.split_whitespace().collect::<Vec<&str>>().iter().map(| i | { tokenize(i.to_string(), &dict) } ).collect();
                         for arg in &args {
                             match arg {
                                 Token::Num(num) => {stack.push(*num);},
                             }
                         }
                     },
                     rustyline::Result::Err(ReadlineError::Interrupted) => {
                         println!("CTRL-C");
                         break
                     },
                     rustyline::Result::Err(ReadlineError::Eof) => {
                         println!("CTRL-D");
                         break
                     },
                     rustyline::Result::Err(err) => {
                         println!("Error: {:?}", err);
                         break
                     }

                 }
    }
    if let Some(num) = stack.pop() {
        match num {
            Number::Int(i) => println!("{}", i),
            Number::Float(f) => println!("{}", f),
        }
    }
    if let Some(num) = stack.pop() {
        match num {
            Number::Int(i) => println!("{}", i),
            Number::Float(f) => println!("{}", f),
        }
    }
    //let var1 = Variable { addr: mem.alloc(879), size: 4 };
    //for i in 0..4 {
    //    println!("{:b}", mem.mem[i]);
    //}
    //println!("{}", mem.read(&var1));
    Ok(())
}
