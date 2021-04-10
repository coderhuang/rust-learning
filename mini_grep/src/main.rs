use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    println!("Hello, mini grep!");

    let args: Vec<String> = env::args().collect();

    // println!("path:{}\nsearch string:{}", file_path, search_string);
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("parsing problem:{}", err);
        process::exit(1);
    });
    println!("config struct:{:#?}", config);

    run(config);
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{}", contents);
    Ok(())
}

#[derive(Debug)]
struct Config {
    file_path: String,
    search_string: String,
}

impl Config {
    fn new(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("mini grep need 3 arguments");
        }

        let file_path = args[1].clone();
        let search_string = args[2].clone();

        Ok(Config {
            file_path,
            search_string,
        })
    }
}
