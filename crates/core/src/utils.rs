pub fn export_last_element<'a>(array: &'a [&'a str]) -> (&'a [&'a str], &'a str) {
    let (e, arr) = array.split_last().unwrap();
    (arr, e)
}

pub fn join_str(args: &[&str]) -> String {
    args.join("")
}

pub fn is_blank(str: &str) -> bool {
    str.chars().all(|c| c.is_whitespace())
}
