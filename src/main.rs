mod parser;

fn main() {
    let input = String::from("Title1\nbody1para1\npara2\npara3\n\nTitle2\nbody2para1\npara2\npara3");
    let output = parser::read_pairs(&input);
    for out in output {
        println!("{}\n{}\n\n", out.title, out.content);
    }
}
