enum DivisionResult {
    OK(u32, u32),
    DivisionByZero,
}

fn divide_with_remainder(x:u32 , y:u32) -> DivisionResult {
    if y == 0 {return DivisionByZero;}
    else {let quotient = x/y;
        let remainder = x%y;
        return OK(quotient, remainder);}
}

fn main(){let result = divide_with_remainder(10,3);
match {result::OK => pritln!("quotient = {}, remainder = {}", result.0, result.1),
    _=>println!("cannot divide by zero"),
}
}