

fn main() {
    println!("rust 101!\n");
    
    // throw away value;
    let _ = 42;

    //dont warn about unused - TODO;
    let _x = 11;

    let _pair = ('a', 15);

    let (letter, number) = ('a', 13);
  
    let block = {
        let letter = 'b';
        print!("another letter: {}\n\n", letter);
        let l1 = 13;
        let l2 = 10;
        l1 + l2
    };
    print!(" the block evaluation {}\n", block);
  
    print!("{} and {}\n", sum(_pair.1,number), letter);

    let evaluation = checkIf13(number);
    print!("{}",evaluation);

}

fn sum(value1: i32, value2: i32) -> i32{
    value1 + value2
}

fn checkIf13(value: i32) -> bool{
    if value == 13 {
        true
    }else{
        false
    }
}
