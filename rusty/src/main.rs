fn main() {
    let mut x = 4;
    println!("x is: {}", x);

    {
        let y = 2;
        println!("y is: {}", y);
    }

    x += 1;
    println!("x is: {}", x);
    main2()
}

fn main2() {
    const SECONDS_IN_MINUTE: u32 = 60;
    println!("{}", SECONDS_IN_MINUTE);
}