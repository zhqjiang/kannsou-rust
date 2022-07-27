use crate::a::a_numbers;

pub const fn b_numbers<const N: usize>() -> [i32; N] {
    let mut numbers: [i32; N] = [0i32; N];

    let mut i: usize = 0;

    while i < N {
        numbers[i] = i as i32 + 1;
        i += 1;
    }

    numbers
}

const TEN_NUMBERS: [i32; 2] = a_numbers();

pub fn test() {
    println!("{:?}", TEN_NUMBERS);
}
