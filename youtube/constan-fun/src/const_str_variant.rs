const fn len(strs: &[&str]) -> usize {
    let mut result = 0;
    let mut remaining = strs;

    while let [current, tail] = remaining {
        result += current.len();
        // remaining = tail
    }

    result
}
