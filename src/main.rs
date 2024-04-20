use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// File extensions to include (comma-separated)
    #[arg(long, value_delimiter = ',', short = 'e')]
    extensions: Vec<String>,

    /// Exclude files/folders matching the pattern (comma-separated)
    #[arg(long, value_delimiter = ',', short = 'x')]
    exclude: Vec<String>,
}

fn main() {
    let args = Args::parse();
    let current_dir = ".";

    let extensions: Vec<&str> = args.extensions.iter().map(|s| s.as_str()).collect();
    let exclude_patterns: Vec<&str> = args.exclude.iter().map(|s| s.as_str()).collect();

    let mut extension_map = HashMap::new();
    extension_map.insert("go", "golang");
    extension_map.insert("rs", "rust");
    extension_map.insert("cs", "csharp");
    extension_map.insert("py", "python");
    extension_map.insert("js", "javascript");
    extension_map.insert("ts", "typescript");
    extension_map.insert("html", "html");
    extension_map.insert("css", "css");
    extension_map.insert("scss", "scss");
    extension_map.insert("json", "json");
    extension_map.insert("yaml", "yaml");
    extension_map.insert("toml", "toml");
    extension_map.insert("md", "markdown");
    extension_map.insert("sh", "bash");
    extension_map.insert("sql", "sql");
    extension_map.insert("java", "java");
    extension_map.insert("kt", "kotlin");
    extension_map.insert("cpp", "cpp");
    extension_map.insert("c", "c");
    extension_map.insert("rb", "ruby");
    extension_map.insert("php", "php");

    println!("Project tree:");
    print_directory_tree(current_dir, 0, &exclude_patterns);

    println!("\nFiles:");
    print_file_contents(current_dir, &extensions, &extension_map, &exclude_patterns);
}

fn print_directory_tree(dir: &str, level: usize, exclude_patterns: &[&str]) {
    let entries = fs::read_dir(dir).unwrap();

    println!("```");

    for (i, entry) in entries.enumerate() {
        let entry = entry.unwrap();
        let path = entry.path();

        if is_excluded(&path, exclude_patterns) {
            continue;
        }

        if i == 0 && level > 0 {
            print!("{}", "└── ".repeat(level));
        } else if level > 0 {
            print!("{}", "│   ".repeat(level));
        }

        if path.is_dir() {
            println!("{}/", path.file_name().unwrap().to_str().unwrap());
            print_directory_tree(path.to_str().unwrap(), level + 1, exclude_patterns);
        } else {
            println!("{}", path.file_name().unwrap().to_str().unwrap());
        }
    }

    print!("```");
}

fn print_file_contents(
    dir: &str,
    extensions: &[&str],
    extension_map: &HashMap<&str, &str>,
    exclude_patterns: &[&str],
) {
    let entries = fs::read_dir(dir).unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();

        if is_excluded(&path, exclude_patterns) {
            continue;
        }

        if path.is_file() {
            if let Some(ext) = path.extension() {
                if extensions.is_empty() || extensions.contains(&ext.to_str().unwrap()) {
                    let file_name = path.file_name().unwrap().to_str().unwrap();
                    let mut file = File::open(&path).unwrap();
                    let mut contents = String::new();
                    file.read_to_string(&mut contents).unwrap();

                    let code_block_name = extension_map.get(ext.to_str().unwrap()).unwrap_or(&"");

                    println!("{}:", file_name);
                    println!("```{}", code_block_name);
                    println!("{}", contents);
                    println!("```");
                    println!();
                }
            }
        } else if path.is_dir() {
            print_file_contents(
                path.to_str().unwrap(),
                extensions,
                extension_map,
                exclude_patterns,
            );
        }
    }
}

fn is_excluded(path: &Path, exclude_patterns: &[&str]) -> bool {
    exclude_patterns
        .iter()
        .any(|pattern| path.to_str().unwrap().contains(pattern))
}