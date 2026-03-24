fn main() {
    let start = 0;
    let end = 12;

    for i in start..=end {
        if i <= 3{
            println!("{}",i);
        }
        else if i%2 != 0 | i%3 != 0 | i%5 != 0 | i%7 != 0{
            println!("{}",i);
        }
        else if i<10 {
            if i == 7 {
                println!("7");
            }
            else if i==5{println!("5")};
        }
    }
}