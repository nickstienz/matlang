mod compiler;

fn main() {
    println!("===[ Matlang by Nicholas Stienz ]===");

    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!(
            "Usage: matlang <filename> <options>\nUse matlang --help or -h for more information."
        );
        return;
    }

    let filename = &args[1];
    let options = &args[2..];

    if filename == "--help" || filename == "-h" {
        println!("Usage: matlang <filename> <options>");
        println!("Options: --help, -h: Display this message.");
        return;
    }
}
