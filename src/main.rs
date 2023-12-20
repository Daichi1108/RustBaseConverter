fn main() {
    println!("{}", todecimal("A"));
}

fn fromdecimal(inputnum:u128, inputbase:u8) -> String {
    let mut input = u128::from(inputnum);
    let base = u128::from(inputbase);

    let mut output = String::new();

    while input > 0 {
        output.insert(0, to_char((input%base) as u8));
        input /= base;
    }

    return output;
}

fn todecimal(inputnum:String, inputbase:u8) -> u128 {
    let input = String::from(inputnum);
    let base = u128::from(inputbase);
    
    let mut output = 0;

    let charnum = input.len() - 1;
    
    for i in input {
        output += to_digit(i) * base.pow(charnum);
        charnum--;
    }
}

fn to_char(num:u8) -> char {
    if num <= 9{
        return (num+48) as char;
    }
    return (num+55) as char;
}

fn to_digit(charinput:char) -> u128 {
    let num = u128::from(charinput as u128);

    if num <= 57 {
        return num - 48
    }
    return num - 55
}