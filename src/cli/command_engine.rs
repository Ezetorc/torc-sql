use clap::Parser;

#[derive(Parser)]
#[command(
    name = "torc-sql",
    version,
    author = "Ezetorc",
    about = "Mini relational database engine"
)]
pub struct CommandEngine {
    #[arg(long)]
    name: String,
}

impl CommandEngine {
    pub fn start() {
        let cli: CommandEngine = CommandEngine::parse();

        println!("Hello {}", cli.name);
    }
}
