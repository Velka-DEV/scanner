use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;
use clap::Parser;
use serde::Deserialize;
use serde_json::from_str;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the directory
    #[arg(long, default_value = ".", short = 'p')]
    path: String,

    /// File extensions to include (comma-separated)
    #[arg(long, value_delimiter = ',', short = 'e')]
    extensions: Vec<String>,

    /// Exclude files/folders matching the pattern (comma-separated)
    #[arg(long, value_delimiter = ',', short = 'x')]
    exclude: Vec<String>,

    /// Use a preset (e.g., --preset nuxt, --preset rust)
    #[arg(long, short = 'r')]
    preset: Option<String>,

    /// List all available presets
    #[arg(long, short = 'l')]
    list_presets: bool,
}

#[derive(Deserialize)]
struct Preset {
    extensions: Vec<String>,
    exclude: Vec<String>,
}

fn main() {
    let args = Args::parse();

    // Load presets from the embedded JSON file
    let presets: HashMap<String, Preset> = load_presets();

    // If --list-presets is set, print the preset names and exit
    if args.list_presets {
        println!("Available presets:");
        for preset_name in presets.keys() {
            println!(" - {}", preset_name);
        }
        return;
    }

    let path = &args.path;

    let mut extensions = args.extensions.clone();
    let mut exclude_patterns = args.exclude.clone();

    // Apply preset if provided
    if let Some(preset_name) = &args.preset {
        if let Some(preset) = presets.get(preset_name) {
            if extensions.is_empty() {
                extensions = preset.extensions.clone();
            }
            if exclude_patterns.is_empty() {
                exclude_patterns = preset.exclude.clone();
            }
        } else {
            eprintln!("Preset '{}' not found", preset_name);
        }
    }

    let extensions: Vec<&str> = extensions.iter().map(|s| s.as_str()).collect();
    let exclude_patterns: Vec<&str> = exclude_patterns.iter().map(|s| s.as_str()).collect();

    let mut extension_map = HashMap::new();
    extension_map.insert("go", "go");
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
    extension_map.insert("yml", "yaml");
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
    extension_map.insert("swift", "swift");
    extension_map.insert("hs", "haskell");
    extension_map.insert("scala", "scala");
    extension_map.insert("ex", "elixir");
    extension_map.insert("erl", "erlang");
    extension_map.insert("lua", "lua");
    extension_map.insert("dockerfile", "dockerfile");
    extension_map.insert("Dockerfile", "dockerfile");

    println!("Project tree:");
    println!();
    println!("```");
    print_directory_tree(&path, 0, &exclude_patterns);
    println!("```");

    println!();

    println!("\nFiles:");
    print_file_contents(&path, &extensions, &extension_map, &exclude_patterns);
}

fn load_presets() -> HashMap<String, Preset> {
    // Embed the `presets.json` file
    let preset_data = include_str!("presets.json");
    from_str(preset_data).expect("Failed to parse presets.json")
}

fn print_directory_tree(dir: &str, level: usize, exclude_patterns: &[&str]) {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };
        let path = entry.path();

        if is_excluded(&path, exclude_patterns) {
            continue;
        }

        let indent = if level > 0 {
            "│   ".repeat(level - 1)
        } else {
            String::new()
        };

        let prefix = if level > 0 { "├── " } else { "" };

        if path.is_dir() {
            println!("{}{}{}/", indent, prefix, path.file_name().unwrap().to_str().unwrap());
            print_directory_tree(path.to_str().unwrap(), level + 1, exclude_patterns);
        } else {
            println!("{}{}{}", indent, prefix, path.file_name().unwrap().to_str().unwrap());
        }
    }
}

fn print_file_contents(
    dir: &str,
    extensions: &[&str],
    extension_map: &HashMap<&str, &str>,
    exclude_patterns: &[&str],
) {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };
        let path = entry.path();

        if is_excluded(&path, exclude_patterns) {
            continue;
        }

        if path.is_file() {
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_str().unwrap_or("");
                if extensions.is_empty() || extensions.contains(&ext_str) {
                    let file_name = path.file_name().unwrap().to_str().unwrap();
                    let mut file = match File::open(&path) {
                        Ok(f) => f,
                        Err(_) => continue,
                    };
                    let mut contents = String::new();

                    if let Ok(_) = file.read_to_string(&mut contents) {
                        let code_block_name = extension_map.get(ext_str).unwrap_or(&"");

                        println!();
                        println!("{}:", file_name);
                        println!("```{}", code_block_name);
                        println!("{}", contents);
                        println!("```");
                        println!();
                    } else {
                        eprintln!("Failed to read file: {}", file_name);
                    }
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
        .any(|pattern| path.to_str().unwrap_or("").contains(pattern))
}
