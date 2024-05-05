use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read};

const LINT_FILE: &str = r"D:\renpy-8.2.0-sdk\tmp\college-kings-2-dev\lint.txt";

const IGNORE_SCENES: [&str; 0] = [];
const IGNORE_FILES: [&str; 1] = ["game/config.rpy"];

fn main() -> std::io::Result<()> {
    let mut unique_errors: HashSet<String> = HashSet::new();
    let mut error_map: HashMap<String, Vec<String>> = HashMap::new();

    let file = File::open(LINT_FILE).expect("Unable to open file");
    let mut contents = String::new();
    BufReader::new(file)
        .read_to_string(&mut contents)
        .expect("Unable to read file");
    let chunks: Vec<&str> = contents.split("\r\n\r\n").collect();

    for (i, chunk) in chunks.into_iter().enumerate() {
        if i == 0 {
            continue;
        }

        let line = chunk.trim();
        if line.is_empty() {
            continue;
        }

        if IGNORE_FILES.contains(&line.split(':').next().unwrap()) {
            continue;
        }

        if line.ends_with("is not an image.") || line.ends_with(".webp', which is not loadable.") {
            continue;
        }

        if line.starts_with("Statistics:") {
            break;
        }

        let mut current_scene = "Other".to_string();
        let scene_re = Regex::new(r"scene([^_.]+)").unwrap();
        if let Some(captures) = scene_re.captures(line) {
            if let Some(scene) = captures.get(1) {
                current_scene = scene.as_str().to_string();
                if IGNORE_SCENES.contains(&current_scene.as_str()) {
                    continue;
                }
            }
        }

        let error_re = Regex::new(r"'([^']+)'").unwrap();
        if let Some(captures) = error_re.captures(line) {
            if let Some(error) = captures.get(1) {
                if unique_errors.contains(error.as_str()) {
                    continue;
                }
                unique_errors.insert(error.as_str().to_string());
            }
        }

        error_map
            .entry(current_scene.to_string())
            .and_modify(|e| e.push(line.to_string()))
            .or_insert(vec![line.to_string()]);
    }

    let sort_regex = Regex::new(r"(\d+)([a-z])*").unwrap();

    let mut sorted_errors: Vec<(String, Vec<String>)> = error_map.into_iter().collect();
    sorted_errors.sort_by(|a, b| {
        let a = &a.0;
        let b = &b.0;

        match (a == "Other", b == "Other") {
            (true, true) => return std::cmp::Ordering::Equal,
            (true, false) => return std::cmp::Ordering::Greater,
            (false, true) => return std::cmp::Ordering::Less,
            (false, false) => {}
        }

        let a_captures = sort_regex.captures(a).unwrap();
        let a_num = a_captures
            .get(1)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap_or(0);
        let a_char = a_captures.get(2).map(|m| m.as_str()).unwrap_or("");

        let b_captures = sort_regex.captures(b).unwrap();
        let b_num = b_captures
            .get(1)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap_or(0);
        let b_char = b_captures.get(2).map(|m| m.as_str()).unwrap_or("");

        (a_num, a_char).cmp(&(b_num, b_char))
    });

    let mut lint_lines = Vec::new();

    for (scene_name, lines) in sorted_errors {
        // lint_lines.push("<details>".to_string());
        // lint_lines.push(format!(
        //     "<summary>{}: ({})</summary>\n",
        //     scene_name,
        //     lines.len()
        // ));

        for line in lines {
            lint_lines.push(format!("- [ ] {}", line));
        }
        // lint_lines.push("\n</details>".to_string());
    }

    fs::write(LINT_FILE, lint_lines.join("\n"))?;

    Ok(())
}
