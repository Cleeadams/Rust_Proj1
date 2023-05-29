fn main() {
    main5();
}

// fn main1() {
//     let mut x: i8 = 4;
//     println!("x is: {}", x);

//     {
//         let y: i8 = 2;
//         println!("y is: {}", y); 
//     }

//     x += 1;
//     println!("x is: {}", x);
// }

// fn main2() {
//     const SECONDS_IN_MINUTE: u32 = 60;
//     println!("{}", SECONDS_IN_MINUTE);
// }

// fn main3() {
//     let floating_point: f32 = 10.92;
//     let t_or_f = false;

//     let letter: char = '5';

//     println!("{}", floating_point);
//     println!("{}", t_or_f);
//     println!("{}", letter);
// }

// fn main4() {
//     let mut tup = (1, true, 's');
//     println!("{}", tup.2);

//     tup.0 = 101;

//     println!("{}", tup.0);
// }

fn main5() {
    let mut arr: [i8; 5] = [1, 2, 3, 4, 5];
    arr[4] = 3;
    println!("{}", arr[4])
}