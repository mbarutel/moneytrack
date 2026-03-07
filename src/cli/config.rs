pub struct Config {
    pub cmd: Process,
}

pub enum Process {
    Summarize,
    List,
}

impl Config {
    pub fn build(args: &[String]) -> Self {
        let command = match args[1].as_ref() {
            "list" => Process::List,
            "summarize" => Process::Summarize,
            _ => panic!("You need to choose a command"),
        };

        Config { cmd: command }
    }
}
