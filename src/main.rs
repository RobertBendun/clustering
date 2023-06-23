pub mod dot;

#[derive(clap::Parser)]
struct CommandLine {
    #[arg(short, long)]
    input: String,
}

fn main() {
    let args = <CommandLine as clap::Parser>::parse();

    let graph = dot::Dot::from_file(args.input);
    println!("{graph}");
}
