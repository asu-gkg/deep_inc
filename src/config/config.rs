#[derive(Debug)]
struct Config {
    standalone: bool,
}

impl Config {
    pub fn new(standalone: bool) -> Self {
        Self {
            standalone
        }
    }
}

pub fn say_hello() {
    let config = Config::new(true);
    println!("{:?}", &config);
    println!("Hello from the config mod!");
}