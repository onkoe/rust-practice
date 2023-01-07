fn main() {
    let test = exclaim("input".to_string());
    println!("{}", test);
}

fn exclaim(input: String) ->  String {
    let mut output = input.to_uppercase();
    output.push('!');
    output
}
