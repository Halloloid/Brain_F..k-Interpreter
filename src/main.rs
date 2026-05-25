use std::io::{stdin};

// enum commands {
//     Rightptr, // >
//     Leftptr,  // <
//     Inc,      // +
//     Dec,      // -
//     Output,   // .
//     Input,    // ,
//     LoopStart,// [
//     LoopEnd,  // ]
// }

fn main() {
    let mut encoded = String::new();
    // encoded.push_str("+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.");
    stdin().read_line(&mut encoded).expect("Failed to Read the Line");

    let mut ptr = 0;
    let mut tape : Vec<u32> = Vec::new();
    tape.push(0);
    for i in encoded.chars(){
        match i {
            '>' => {
                ptr+=1;
                tape.push(0);
            },
            '<' => ptr-=1,
            '+' => tape[ptr] += 1,
            '-' => tape[ptr] -= 1,
            '.' => print!("{}",std::char::from_u32(tape[ptr]).unwrap()),
            _ => (),
        } 
    }
}
