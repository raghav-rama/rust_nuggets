fn main() {
    println!("Hello, this is collection of rust stuff!");
    println!("Feel free to look in the bin folder and explore all the cool stuff");
    println!("If you want to run a particular binary, you can use the following command:");
    println!("cargo run --bin <binary_name>");
    let c = (
        "\033[0m",  // End of color
        "\033[36m", // Cyan
        "\033[91m", // Red
        "\033[35m", // Magenta
    );
    println!(
        "{}hi{} there, {}this{} is a {}cool{} message",
        c.1, c.0, c.2, c.0, c.3, c.0
    );
}
