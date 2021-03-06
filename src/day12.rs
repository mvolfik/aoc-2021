use {
    crate::utils::{DayResult, Failable},
    std::{
        collections::{HashMap, HashSet},
        str::Lines,
    },
};

pub(crate) fn main(stdin: Lines) -> DayResult {
    let mut connections: HashMap<String, HashSet<String>> = HashMap::new();
    for line in stdin {
        let mut iterator = line.split('-');
        let (a, b) = match (iterator.next(), iterator.next()) {
            (Some(a), Some(b)) => (a, b),
            _ => return Err("Unexpected input format".to_string()),
        };

        connections
            .entry(a.to_string())
            .or_insert_with(HashSet::new)
            .insert(b.to_string());
        connections
            .entry(b.to_string())
            .or_insert_with(HashSet::new)
            .insert(a.to_string());
    }

    let part1 = search(&connections, &["start"], false, false).map(|x| x.to_string());
    let part2 = search(&connections, &["start"], false, true).map(|x| x.to_string());
    Ok((part1, part2))
}

/// Recursive DFS search function
///
/// # Arguments
///
/// * `connections` - Mapping of outgoing connections for each vertex
/// * `path` - Vector of vertices visited so far in this path
/// * `doubled` - If a lowercase vertex was already visited twice (puzzle requirement)
fn search(
    connections: &HashMap<String, HashSet<String>>,
    path: &[&str],
    doubled: bool,
    is_part_2: bool,
) -> Failable<i32> {
    let mut paths = 0;
    for next in match connections.get(*path.last().ok_or("Search called with empty path")?) {
        Some(x) => x,
        None => return Ok(0),
    } {
        if next == "end" {
            paths += 1;
            continue;
        }

        if next == "start" {
            continue;
        }

        let is_upper = next
            .chars()
            .next()
            .ok_or("Unexpected zero length node")?
            .is_uppercase();
        let next_doubled = if is_upper {
            doubled
        } else if doubled {
            if path.contains(&next.as_str()) {
                continue;
            }
            true
        } else {
            match find_at_least(path, next.as_str(), if is_part_2 { 2 } else { 1 }) {
                0 => false,
                1 => {
                    if is_part_2 {
                        true
                    } else {
                        continue;
                    }
                }
                _ => continue,
            }
        };

        let mut new_path = Vec::from(path);
        new_path.push(next.as_str());
        paths += search(connections, &new_path, next_doubled, is_part_2)?;
    }
    Ok(paths)
}

/// Tries to find at least `n` occurences of `searched_value` in `vector`
///
/// Returns the number of occurences found
fn find_at_least<T: PartialEq>(vector: &[T], searched_value: T, n: i32) -> i32 {
    let mut found = 0;
    for item in vector {
        if *item == searched_value {
            found += 1;
            if found >= n {
                return found;
            }
        }
    }
    found
}
