use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

const LINT_FILE: &str = r"D:\renpy-sdk\tmp\College-Kings-2\lint.txt";

const SCENE_RANGE: [&str; 0] = [];

fn main() {
    let mut unique_errors: HashSet<String> = HashSet::new();
    let mut error_map: HashMap<String, Vec<String>> = HashMap::new();

    let file = File::open(LINT_FILE).expect("Unable to open file");
    let lines = BufReader::new(file).lines();

    for (i, line) in lines.into_iter().enumerate() {
        if i == 0 {
            continue;
        }

        let line = line.unwrap();
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if line.contains("game/ep3/") {
            break;
        }

        let mut current_scene = "Other".to_string();
        let scene_re = Regex::new(r"scene([^_.]+)").unwrap();
        if let Some(captures) = scene_re.captures(line) {
            if let Some(scene) = captures.get(1) {
                current_scene = scene.as_str().to_string();
                if !SCENE_RANGE.is_empty() && !SCENE_RANGE.contains(&current_scene.as_str()) {
                    current_scene.push_str(" - MISSING");
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

    let mut sorted_errors: Vec<(String, Vec<String>)> = error_map.into_iter().collect();
    sorted_errors.sort();

    let mut lint_lines = Vec::new();

    for (scene_name, lines) in sorted_errors {
        lint_lines.push("<details>".to_string());
        lint_lines.push(format!(
            "<summary>{}: ({})</summary>\n",
            scene_name,
            lines.len()
        ));

        for line in lines {
            lint_lines.push(format!("- [ ] {}", line));
        }
        lint_lines.push("\n</details>".to_string());
    }

    fs::write(LINT_FILE, lint_lines.join("\n")).expect("Unable to write file");
}
