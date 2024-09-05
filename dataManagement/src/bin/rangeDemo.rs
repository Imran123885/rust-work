fn main() {
    let range = 1..=3;
    let range2 = 1..4;

    for num in range {
        println!("{:?}", num);
    }

    for num in range2 {
        println!("{:?}", num);
    }

    for ch in 'a'..='f' {
        println!("{:?}", ch);
    }
}