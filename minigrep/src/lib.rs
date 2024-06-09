use config::Config;
use std::{env, error::Error, fs};
pub mod config;

pub fn run() -> Result<(), Box<dyn Error>> {
    let cfg = Config::build(env::args())?;
    // Config::build(&arg).unwrap_or_else(|err|{
    //     println!("{err}");
    // })

    let contents = fs::read_to_string(cfg.file_path)?;

    for line in if !cfg.ignore_case {
        search(&cfg.query, &contents)
    } else {
        search_case_insensitve(&cfg.query, &contents)
    } {
        println!("{line}");
    }

    Ok(())
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content.lines().filter(|l| l.contains(query)).collect()
}

fn search_case_insensitve<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = &query.to_lowercase();
    content
        .lines()
        .filter(|l| l.to_lowercase().contains(query))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn two_result() {
        let query = "monk";
        let content = "\
        something about something:
        monkey says this:
        monkey monkey
        Sausage
        Monkey
        ";
        assert_eq!(
            vec!["monkey says this:", "monkey monkey"],
            search(query, content)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "money";
        let content = "\
        MoNey:
        that is mine
        should be mine.
        ";
        assert_eq!(vec!["MoNey:"], search_case_insensitve(query, content))
    }
}
