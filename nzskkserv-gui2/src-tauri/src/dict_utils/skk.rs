use nzskkserv_core::handler::Entry;

pub(super) fn parse_skk_dict(dict: &str) -> Vec<(String, Vec<Entry>)> {
    let mut dict_data = vec![];
    for line in dict.lines() {
        if line.trim().is_empty() || line.starts_with(';') {
            continue;
        }
        let Some((source, entries)) = line.split_once(' ') else {
            continue;
        };
        let entries = entries
            .split("/")
            .filter_map(|entry| {
                if entry.is_empty() {
                    return None;
                }
                let (candidate, description) =
                    if let Some((candidate, description)) = entry.split_once(';') {
                        (candidate, Some(description))
                    } else {
                        (entry, None)
                    };
                Some(Entry {
                    candidate: candidate.to_string(),
                    description: description.map(|s| s.to_string()),
                })
            })
            .collect();
        dict_data.push((source.to_string(), entries));
    }

    dict_data
}
