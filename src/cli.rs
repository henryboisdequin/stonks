use super::parse::Config;

pub struct Cli {
    pub config: Vec<Config>,
}

impl Cli {
    pub fn new(config: Vec<Config>) -> Self {
        Self { config }
    }

    pub fn list(self) {
        let symbols = self.config;

        println!("All the symbols of the companies you are watching:");

        for symbol in symbols {
            println!("{:?}", symbol);
        }
    }

    pub fn add(self) {}

    pub fn view(self) {}
}
