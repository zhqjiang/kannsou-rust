trait HasNumbers {
    // Note that we have a constant here without a value.
    const NUMBERS: [i32; 5];
    // Constants in traits can have default values
    const LAST_NUMBER: i32 = 5;
}

struct IHaveOtherNumbers {}
impl HasNumbers for IHaveOtherNumbers {
    // Here we override the default value from the trait.
    const LAST_NUMBER: i32 = 6;
    const NUMBERS: [i32; 5] = [1, 2, 3, 4, IHaveOtherNumbers::LAST_NUMBER];
}
