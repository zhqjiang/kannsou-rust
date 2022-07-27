struct WillSayGoodbye<'a>(&'a str);
impl<'a> Drop for WillSayGoodbye<'a> {
    fn drop(&mut self) {
        println!("{}", self.0)
    }
}

// constants are default copy
// constants can have constructors
// constants in Rust are not just constant values,
// but they contain codes that executed in compile time
const GOODBYE_IN_GERMAN: WillSayGoodbye = WillSayGoodbye("Auf Wisdersehen");

fn main() {
    {
        let _goodbye_sayer: WillSayGoodbye = GOODBYE_IN_GERMAN;
    }
}
