/// Split arg with "=" to unify arg notations.
/// --flag=value => ["--flag", "value"]
/// --flag value => ["--flag", "value"]
/// -abe => ["-a", "-b", "-e"]
/// -abef=32 => ["-a", "-b", "-e", "-f", "32"]
pub fn normalized_args(raw_args: Vec<String>) -> Vec<String> {
    raw_args.iter().fold(Vec::<String>::new(), |mut acc, cur| {
        if cur.starts_with('-') && !cur.starts_with("--") {
            if cur.contains('=') {
                let splitted_flag: Vec<String> = cur.splitn(2, '=').map(|s| s.to_owned()).collect();
                let short_named = splitted_flag[0].chars().skip(1).map(|c| format!("-{}", c));
                acc.append(&mut short_named.collect());
                acc.append(&mut splitted_flag[1..].to_vec());
            } else {
                let short_named = cur.chars().skip(1).map(|c| format!("-{}", c));
                acc.append(&mut short_named.collect());
            }
        } else if cur.starts_with('-') && cur.contains('=') {
            let mut splitted_flag: Vec<String> = cur.splitn(2, '=').map(|s| s.to_owned()).collect();
            acc.append(&mut splitted_flag);
        } else {
            acc.push(cur.to_owned());
        }
        acc
    })
}
