use clap::{Parser, Subcommand};
use debug_print::debug_println;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use webbrowser::Browser;

#[derive(Debug, Parser)]
#[clap(author = "Joseph Sagiv", name = "flow", about = "Smart task manager", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// adds things
    #[clap(arg_required_else_help = true)]
    Add {
        /// Stuff to add
        #[clap(required = true)]
        content: String,

        /// Save to scope
        #[clap(short = 's', long, required = false, value_name = "NAME")]
        scope: Option<String>,
    },

    /// view things
    #[clap(arg_required_else_help = false)]
    View {
        /// scope to view
        #[clap(short = 's', long, value_name = "NAME")]
        scope: Option<String>,
    },

    /// scope things
    #[clap(arg_required_else_help = true)]
    Scope {
        /// Stuff to add
        #[clap(required = true)]
        name: String,
    },
}

fn print_to_file(content: String, scope: String) {
    let mut file = File::options()
        .create(true)
        .append(true)
        .open(format!("{scope}.md"))
        .expect("cannot open file");
    let output_format = format!("\n\n - [ ] {} ", content);
    file.write_all(output_format.as_bytes())
        .expect("write failed");
}

const DEFAULT_FLOW: &str = "flow";

fn process_cli_input(args: Cli) -> Result<(), Box<dyn Error>> {
    match args.command {
        Commands::Add { content, scope } => add_flow(content, scope),
        Commands::Scope { name } => todo!(),
        Commands::View { scope } => view_scope(scope),
    };
    Ok(())
}

fn add_flow(content: String, scope: Option<String>) {
    println!("Adding {:?}", content);
    let _scope = match scope {
        Some(s) => s,
        None => DEFAULT_FLOW.to_string(),
    };
    print_to_file(content, _scope);
}

fn view_scope(scope: Option<String>) {
    match scope {
        Some(_scope) => {
            if webbrowser::open_browser(Browser::Chrome, format!("./{_scope}.md").as_str()).is_ok()
            {
                println!("Success")
            } else {
                print!("Failed!")
            }
        }
        None => if webbrowser::open_browser(Browser::Chrome, "flow.md").is_ok() {},
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    debug_println!("{:?}", args);
    process_cli_input(args)
}

fn main() {
    run().expect("blew up");
}
