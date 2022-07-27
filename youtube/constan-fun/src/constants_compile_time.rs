trait HasNumbers {
    const NUMBERS: [i32; 5];
    const LAST_NUMBER: i32 = 5;
}

struct IHaveOtherNumbers {}
impl HasNumbers for IHaveOtherNumbers {
    const LAST_NUMBER: i32 = 6;
    const NUMBERS: [i32; 5] = [1, 2, 3, 4, IHaveOtherNumbers::LAST_NUMBER];
}

// create a constant without a name
// this type () is called unit
// inner code of { } runs at compile time
const _: () = {
    use std::marker::PhantomData;
    struct ImplementsMyTrait<T: HasNumbers>(PhantomData<T>);
    let _ = ImplementsMyTrait(PhantomData::<IHaveOtherNumbers>);
};
