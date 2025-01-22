mod constant;
use std::fs::{self, File};
use std::path::Path;
use clap::*;
use dialoguer::*;
use dialoguer::theme::ColorfulTheme;
use constant::TEST;

#[derive(Parser, Debug)]
struct CLI {
    #[command(subcommand)]
    options : Options,
}

#[derive(Subcommand, Debug)]
enum Options {
    Init {
        /// Initialize Prisma ORM
        #[clap(short, long)]
        option_one : bool,
        /// Initialize Drizzle ORM
        #[clap(short, long)]
        option_two : bool,
    },
    List,
}

fn main(){
    let cli = CLI::parse();
    let a = vec!["prisma".to_string(), "drizzle".to_string()];

    match cli.options {
        Options::Init { option_one, option_two } => {
            if option_one && option_two {
                println!("You cant select both");
                std::process::exit(1);
            }
            if option_one {
                println!("Prisma")
            }
            else if option_two {
                println!("Drizzle")
            } else {
                let selection = Select::with_theme(&ColorfulTheme::default())
                            .with_prompt("What do you choose?")
                            .items(&a)
                            .default(0)
                            .interact()
                            .unwrap();
                println!("You selected: {}", a[selection]);
            }
        },
        Options::List => {
            let paths = fs::read_dir(".").unwrap();
            for path in paths {
                println!("Name: {}", path.unwrap().path().display());
            }
        },
    }
}