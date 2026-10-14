use clap::Parser;

#[derive(Debug)]
enum Command {
    Add { title: String },
    Remove { title: String },
    Done { title: String },
    List,
}

fn main() {
    loop {}
}
