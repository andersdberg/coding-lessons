fn main() {
    // Statements here are executed when the compiled binary is called

    // Print text to the console
    println!("Hello World, from Rust!!! We have to recompile to get this to show");

    #[derive(Show)]
    enum S {
        A,
        B
    }

    fn main() {
        println!("{}", S::A);
    }
}