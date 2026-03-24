fn main() {
    let start = 0;
    let end = 45;

    for i in start..=end {
        if i <= 1{
            continue;
        }
        let mut is_prime =true;
        let limit = (i as f64).sqrt() as i32;

        for f in 2..=limit {
            if i%f == 0{
                is_prime = false;
                break;
            }
        }
        
    if is_prime {
        println!("{}",i);
            }
        }   
    }