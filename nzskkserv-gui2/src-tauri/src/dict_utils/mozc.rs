use nzskkserv_core::handler::Entry;

pub(super) fn parse_mozc_dict(dict: &str) -> Vec<(String, Vec<Entry>)> {
    let mut dict_data = vec![];
    for line in dict.lines() {
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        let mut split = line.split("\t");
        let key = split.next();
        let value = split.next();
        let (Some(key), Some(value)) = (key, value) else {
            continue;
        };
        let _part = split.next();
        let comment = split.next();

        dict_data.push((
            key.to_string(),
            vec![Entry {
                candidate: value.to_string(),
                description: comment.map(|s| s.to_string()),
            }],
        ));
    }

    dict_data
}
