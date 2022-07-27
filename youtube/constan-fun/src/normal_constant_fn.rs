// const allow rust run this function in compile time
// for loop are not supported inside contant function
const fn five_numbers() -> [i32; 5] {
    let mut numbers: [i32; 5] = [0i32; 5];

    let mut i: usize = 0;
    while i < 5 {
        numbers[i] = i as i32 + 1;
        i += 1;
    }
    numbers
}

pub fn test() {
    const FIVE_NUMBERS: [i32; 5] = five_numbers();
    println!("{:?}", FIVE_NUMBERS);

    let five_numbers: [i32; 5] = five_numbers();
    println!("{:?}", five_numbers);
}
