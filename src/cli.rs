use super::parse::Config;

pub struct Cli {
    pub config: Config,
}

impl Cli {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn list(self) {}

    pub fn add(self) {}

    pub fn view(self) {}
}
