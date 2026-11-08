// ADDED FROM `commander.js` REPO

use std::collections::HashSet;

const MAX_DISTANCE: usize = 3;

fn edit_distance(a: &str, b: &str) -> usize {
    // https://en.wikipedia.org/wiki/Damerau–Levenshtein_distance
    // Calculating optimal string alignment distance, no substring is edited more than once.
    // (Simple implementation.)

    // Quick early exit, return worst case.
    if a.len().abs_diff(b.len()) > MAX_DISTANCE {
        return a.len().max(b.len());
    }

    // distance between prefix substrings of a and b
    let mut d = vec![vec![0; b.len() + 1]; a.len() + 1];

    // pure deletions turn a into empty string
    for i in 0..=a.len() {
        d[i][0] = i;
    }
    // pure insertions turn empty string into b
    for j in 0..=b.len() {
        d[0][j] = j;
    }

    // fill matrix
    for j in 1..=b.len() {
        for i in 1..=a.len() {
            let cost = if a.chars().nth(i - 1).unwrap() == b.chars().nth(j - 1).unwrap() {
                0
            } else {
                1
            };

            let deletion = d[i - 1][j] + 1;
            let insertion = d[i][j - 1] + 1;
            let substitution = d[i - 1][j - 1] + cost;

            d[i][j] = deletion.min(insertion).min(substitution);

            // transposition
            if i > 1
                && j > 1
                && a.chars().nth(i - 1).unwrap() == b.chars().nth(j - 2).unwrap()
                && a.chars().nth(i - 2).unwrap() == b.chars().nth(j - 1).unwrap()
            {
                d[i][j] = d[i][j].min(d[i - 2][j - 2] + 1);
            }
        }
    }

    d[a.len()][b.len()]
}

///
/// Find close matches, restricted to same number of edits.
///
/// # Arguments
///
/// * `word` - The word to suggest similar words for.
/// * `candidates` - The candidates to suggest similar words from.
///
/// # Returns
///
/// The suggested similar words.
pub fn suggest_similar(word: String, candidates: Vec<String>) -> String {
    if candidates.is_empty() {
        return String::new();
    }

    let mut word = word.clone();

    // remove possible duplicates
    let mut candidates: Vec<String> = candidates
        .into_iter()
        .map(|candidate| candidate.to_string())
        .collect::<HashSet<String>>()
        .into_iter()
        .collect();

    let searching_options = word.starts_with("--");
    if searching_options {
        word = word[2..].to_string();
        candidates = candidates
            .into_iter()
            .map(|candidate| candidate[2..].to_string())
            .collect();
    }

    let mut similar = Vec::new();
    let mut best_distance = MAX_DISTANCE;
    let min_similarity = 0.4;
    for candidate in candidates {
        if candidate.len() <= 1 {
            continue; // no one character guesses
        }

        let distance = edit_distance(&word, &candidate);
        let length = word.len().max(candidate.len());
        let similarity = (length - distance) as f64 / length as f64;
        if similarity > min_similarity {
            if distance < best_distance {
                // better edit distance, throw away previous worse matches
                best_distance = distance;
                similar = vec![candidate];
            } else if distance == best_distance {
                similar.push(candidate);
            }
        }
    }

    similar.sort();
    if searching_options {
        similar = similar
            .into_iter()
            .map(|candidate| format!("--{}", candidate))
            .collect();
    }

    if similar.len() > 1 {
        format!("Did you mean one of {}?", similar.join(", "))
    } else if similar.len() == 1 {
        format!("Did you mean {}?", similar[0])
    } else {
        String::new()
    }
}
