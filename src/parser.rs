#[derive(Debug)]
pub struct EnvEntry {
    pub key: String,
    pub value: String,
}

fn strip_quotes(string: &str) -> String {
    if (string.starts_with('"') && string.ends_with('"'))
        || (string.starts_with("'") && string.ends_with("'"))
    {
        string[1..string.len() - 1].to_string()
    } else {
        string.to_string()
    }
}

pub fn parser(content: &str) -> Vec<EnvEntry> {
    content
        .lines()
        .map(|x| x.trim())
        .filter(|x1| !x1.is_empty() && !x1.starts_with("#"))
        .filter_map(|x2| x2.split_once("="))
        .map(|x3| EnvEntry {
            key: x3.0.trim().to_string(),
            value: strip_quotes(x3.1.trim()).to_string(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let input = " KEY = value ";
        let result = parser(input);
        assert_eq!(result[0].key, "KEY");
        assert_eq!(result[0].value, "value");
    }
}
