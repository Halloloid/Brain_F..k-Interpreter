use std::{io::stdin};


struct LoopIndex {
    l_start : u8,
    l_end : u8
}

fn main() {
    let mut encoded = String::new();
    stdin().read_line(&mut encoded).expect("Failed to Read the Line");

    let mut ptr : u8 = 0;
    let mut encodedvect :Vec<char> = Vec::new();
    
    let mut indexes = LoopIndex{
        l_start : 0,
        l_end : 0
    };
    
    for i in encoded.chars(){
        encodedvect.push(i);
        ptr+=1;
        if i == '[' {
            indexes.l_start = ptr;
        }
        if i == ']' {
            indexes.l_end = ptr;
        }
    }

    
    encodedvect.pop();
    encodedvect.pop();

    // print!("{:#?}",encodedvect);
    
    let mut ptr = 0;
    let mut tape : Vec<u32> = Vec::new();
    tape.push(0);
    let mut i = 0;
    
    while i != encodedvect.len() {
        match encodedvect[i] {
            '>' => {
                ptr+=1;
                tape.push(0);
                i+=1;
            },
            '<' => { ptr-=1;i+=1 },
            '+' => { tape[ptr] += 1;i+=1 },
            '-' => { tape[ptr] -= 1;i+=1 },
            '.' => { print!("\n{}",std::char::from_u32(tape[ptr]).unwrap());i+=1 },
            '[' => {
                if tape[ptr] == 0 {
                    i = indexes.l_end as usize;
                } else {
                    indexes.l_start = i as u8;
                    i+=1;
                }
            },
            ']' => {
                if tape[ptr] != 0 {
                    i = indexes.l_start as usize;
                } else {
                    indexes.l_end = i as u8;
                    i+=1;
                }
            }
            _ => (),
        } 
    }
}
