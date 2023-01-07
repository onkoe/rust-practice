fn main() {
    let text = "Hello\nworld\n!";
    let upper = text.to_uppercase();
    let stripped = upper.strip_prefix("HELLO\n").unwrap();
    println!("{}", first_line(stripped));
    println!("{}", first_line(text));
}


pub fn first_line(string: &str) -> &str {
    string.lines().next().unwrap()
}