fn main() {
    // TODO: Fix the compiler error by moving the whole definition of this macro.
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
    my_macro!();
}
