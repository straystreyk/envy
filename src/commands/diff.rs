use crate::parser::parser;
use clap::Args;
use std::collections::HashMap;
use std::path::PathBuf;
use tabled::builder::Builder;
use tabled::settings::{Color, Modify, object::Rows};

#[derive(Args)]
pub struct DiffArgs {
    pub paths: Vec<PathBuf>,
}

struct DiffRow {
    key: String,
    values: Vec<String>,
}

fn row_status(values: &[String]) -> &'static str {
    let all_same = values.windows(2).all(|w| w[0] == w[1]);

    if !all_same { "changed" } else { "same" }
}

pub fn diff(args: DiffArgs) {
    let DiffArgs { paths } = args;

    let mut rows: Vec<DiffRow> = Vec::new();
    let mut names: Vec<String> = Vec::new();
    let mut maps: Vec<HashMap<String, String>> = Vec::new();

    for path in paths {
        let content = std::fs::read_to_string(&path).unwrap();
        let filename = path.file_name().unwrap().to_string_lossy().to_string();

        names.push(filename);
        maps.push(
            parser(&content)
                .into_iter()
                .map(|entry| (entry.key, entry.value))
                .collect(),
        )
    }

    let mut all_keys: std::collections::HashSet<String> = std::collections::HashSet::new();

    for map in &maps {
        for key in map.keys() {
            all_keys.insert(key.clone());
        }
    }

    // для каждого ключа строим строку
    for key in &all_keys {
        let values: Vec<String> = maps
            .iter()
            .map(|map| map.get(key).cloned().unwrap_or("-".to_string()))
            .collect();

        rows.push(DiffRow {
            key: key.clone(),
            values,
        });
    }

    let mut builder = Builder::new();
    // строим таблицу
    let mut header = vec!["key".to_string()];
    header.extend(names.clone());
    builder.push_record(&header);

    for row in &rows {
        let mut record = vec![row.key.clone()];
        record.extend(row.values.clone());
        builder.push_record(&record);
    }

    let mut table = builder.build();

    for (i, row) in rows.iter().enumerate() {
        let status = row_status(&row.values);
        let color = match status {
            "changed" => Color::FG_YELLOW,
            _ => Color::FG_WHITE,
        };
        table.with(Modify::new(Rows::new(i + 1..i + 2)).with(color));
    }

    println!("{}", table);
}
