use clap::*;
use dialoguer::*;
use dialoguer::theme::ColorfulTheme;

#[derive(Parser, Debug)]
struct CLI {
    #[command(subcommand)]
    options : Options,
}

#[derive(Subcommand, Debug)]
enum Options {
    Init {
        #[clap(short, long)]
        option_one : bool,
        #[clap(short, long)]
        option_two : bool,
    }
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
                match selection {
                    0 => {
                        println!("You selected prisma")
                    }
                    1 => {
                        println!("You selected drizzle")
                    }
                    _ => {}
                }
            }
        }
    }
}