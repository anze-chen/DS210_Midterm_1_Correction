enum DivisionResult {
    OK(u32, u32),
    DivisionByZero,
}

fn divide_with_remainder(x:u32 , y:u32) -> DivisionResult {
    if y == 0 {return DivisionResult::DivisionByZero;}
    else {let quotient = x/y;
        let remainder = x%y;
        return DivisionResult::OK(quotient, remainder);}
}

fn main(){let result = divide_with_remainder(10,3);
match result {DivisionResult::OK(q,r) => println!("quotient = {}, remainder = {}", q, r),
    _=>println!("cannot divide by zero"),
    }
}