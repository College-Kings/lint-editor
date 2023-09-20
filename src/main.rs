use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

const LINT_FILE: &str = r"D:\renpy-sdk\tmp\College-Kings-2\lint.txt";

const SCENE_RANGE: [&str; 133] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "13a", "14", "15", "15a",
    "16", "17", "18", "18a", "18b", "18c", "19", "20", "21", "21a", "21b", "22", "22a", "22b",
    "23", "24", "25", "26", "27", "28", "28a", "29", "30", "31", "32", "32a", "33", "33a", "34",
    "35", "35a", "36", "36a", // "37",
    "38", "39", "39a", "40", "41", "42", "42a", "42b", "42c", "43", "43a", "43b", "43c", "44",
    "45", "46", "46a", "46b", "47", "47a", "48", "49", "50", "51", "52", "53", "54", "54a", "55",
    "55a", "55b", "55c", "56", "57", // "58",
    // "59",
    // "59a",
    "60", "61", "62", "62a", "63", "64", "64a", "65", "65a", "65b", "66", // "67",
    "68", // "69",
    // "69a",
    "69b", "70", "71", "72", // "72a",
    "73", "74", "74a", "75", "76", "76a", "76b", "77", // "78",
    "78a", "79", "80", "81", "82", "82a",
    // "82b",
    // "82c",
    // "82d",
    // "82e",
    "82f", // "82g",
    // "82h",
    "83", "83a", "83b", // "83c",
    "83d", // "83e",
    // "83f",
    // "83g",
    "83h", "83i", //"84",
    "84a", "84b", // "84c",
    "84d", // "84e",
    // "84f",
    // "84g",
    // "84h",
    "84i", "85",
    // "85a",
    // "85b",
    // "85c",
    // "85d",
    // "85e",
    // "85f",
    // "85g",
    // "85h",
    "85i",
    // "86",
    // "87",
    // "88",
    // "89",
    // "90",
    // "90a",
    "91", // "92",
    "93", // "94",
    // "95",
    // "95a",
    // "96",
    // "97",
    //"98",
    "99", "100", // "101",
    "101a", "102",
    // "103",
    // "104",
];

fn main() {
    let mut current_scene = String::new();
    let mut unique_errors: HashSet<String> = HashSet::new();
    let mut unique_lines: Vec<String> = Vec::new();

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

        let scene_re = Regex::new(r"scene([^_.]+)").unwrap();
        if let Some(captures) = scene_re.captures(line) {
            if let Some(scene) = captures.get(1) {
                if !SCENE_RANGE.contains(&scene.as_str()) {
                    continue;
                }

                if current_scene != scene.as_str() {
                    unique_lines.push(format!("\n{}:", scene.as_str()));
                    current_scene = scene.as_str().to_string();
                }
            }
        }

        let error_re = Regex::new(r"'([^']+)'").unwrap();
        if let Some(captures) = error_re.captures(line) {
            if let Some(error) = captures.get(1) {
                if !unique_errors.contains(error.as_str()) {
                    unique_errors.insert(error.as_str().to_string());
                    unique_lines.push(format!("- [ ] {}", line));
                }
            }
        } else {
            unique_lines.push(format!("- [ ] {}", line));
        }
    }

    let joined_lines = unique_lines.join("\n");
    fs::write(LINT_FILE, joined_lines).expect("Unable to write file");
}
