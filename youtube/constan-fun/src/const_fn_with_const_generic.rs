const fn numbers<const N: usize>() -> [i32; N] {
    let mut numbers: [i32; N] = [0i32; N];

    let mut i: usize = 0;

    while i < N {
        numbers[i] = i as i32 + 1;
        i += 1;
    }

    numbers
}

pub fn test() {
    const TEN_NUMBERS: [i32; 15] = numbers();
    println!("{:?}", TEN_NUMBERS);
}
