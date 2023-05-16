mod solution;

const USAGE: &str = "Usage: [COMMAND]\ncommands:\n  list: see problems\n  number: run problem by number";
const LIST: &str = r#"
Arrays & Hashing:
  219. Contains Duplicate II
"#;

fn main() {
    let args = std::env::args();
    let last_arg = args.last().unwrap();

    let time = std::time::Instant::now();
    match &*last_arg {
        "list" => println!("{LIST}"),
        _ => println!("{USAGE}"),
    }
    println!("{:?}", time.elapsed());
}
