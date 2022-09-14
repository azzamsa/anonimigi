use regex::{Captures, Regex};

fn main() {
    let input = r#"
❯ ls ~/.local/bin
ls: cannot access '/home/galileo/.local/bin': No such file or directory
ls: cannot access '/home/lovalace/.local/bin': No such file or directory

My Email: me@galileo.com
My Ip: 70.70.254.48

"#;

    let re = Regex::new("(galileo|lovalace)").unwrap();
    let result = re.replace_all(input, |cap: &Captures| {
        match &cap[0] {
            "galileo" => "user",
            "lovalace" => "user",
            _ => unreachable!(),
        }
        .to_string()
    });
    println!("{}", result);
}
