use crate::parser::parser;
use clap::Args;
use std::path::PathBuf;
use tabled::builder::Builder;

#[derive(Args)]
pub struct ListArgs {
    pub path: PathBuf,

    #[arg(long)]
    pub show_values: bool,

    #[arg(long)]
    pub keys: bool,

    #[arg(long)]
    pub count: bool,
}

pub fn list(args: ListArgs) {
    let ListArgs {
        path,
        show_values,
        keys,
        count,
    } = args;

    let content = std::fs::read_to_string(path).unwrap();
    let entries = parser(&content);

    if count {
        println!("{}", entries.len())
    } else if keys {
        let keys: Vec<String> = entries.iter().map(|x| x.key.trim().to_string()).collect();

        for (i, key) in keys.iter().enumerate() {
            println!("{:>3}  {}", i + 1, key);
        }
    } else {
        let mut builder = Builder::new();
        builder.push_record(["KEY", "VALUE"]);

        for entry in &entries {
            let value = if show_values {
                entry.value.clone()
            } else {
                "***".to_string()
            };

            builder.push_record([&entry.key, &value]);
        }

        let table = builder.build();
        println!("{}", table);
    }
}
