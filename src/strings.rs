pub fn check_palindrome(s: String) -> bool {
    let mut str = s.clone();

    str.make_ascii_lowercase();

    let b = str
        .chars()
        .filter(|ch| ch.is_alphanumeric())
        .collect::<String>()
        .into_bytes();

    let len = b.len();
    let mut start: usize = 0;
    let mut end: usize = len - 1;

    while b[start] == b[end] {
        if start >= end {
            return true;
        }

        start += 1;
        end -= 1;
    }

    false
}
