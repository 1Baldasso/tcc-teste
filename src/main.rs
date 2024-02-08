use clap::Parser;
use arrow::util::string_writer::StringWriter;
use crate::transpiler::visitor::NodeVisitor;

mod transpiler;
#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
    #[arg(short='o', long="output", default_value="./")]
    output: std::path::PathBuf,
    #[arg(short='n', long="no-keys", default_value="false")]
    no_keys: bool,
    #[arg(short='k', long="key-type", default_value="uniqueidentifier")]
    key_type: String,
    #[arg(short='s', long="schema", default_value="sql-server")]
    schema: String,
}

fn main(){
    let args: Cli = Cli::parse();
    match std::fs::read_to_string(&args.path) { 
        Ok(value) => {
            match json::parse(&value) {
                Ok(value) => {
                    let mut mut_writer = StringWriter::new();
                    value.to_owned().visit_node(&mut mut_writer, &args);
                    println!("{}", mut_writer.to_string());
                },
                Err(error) => {
                    eprintln!("{}", error)
                }
            }
        },
        Err(error) => {
            eprintln!("{}", error);
        }
        
    }
}