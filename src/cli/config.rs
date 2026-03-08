pub struct Config {
    pub cmd: Command,
}

pub enum Command {
    Summarize,
    Import,
    List,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        }

        let command = match args[1].as_ref() {
            "list" => Command::List,
            "import" => Command::Import,
            "summarize" => Command::Summarize,
            _ => panic!("You need to choose a command"),
        };

        Ok(Config { cmd: command })
    }
}
