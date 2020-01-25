
pub struct Pair {
    pub title: String,
    pub content: String
}

pub fn read_pairs(input: &String) -> Vec<Pair> {
    let string_pairs = input.split("\n\n").map(|s| {s.to_string()});
    let mut pairs = Vec::new();
    for string in string_pairs {
        let mut pair = Pair{title:"".to_string(),content:"".to_string()};
        let parts = string.split("\n");
        for (i,part) in parts.enumerate() {
            if i == 0 {
                pair.title = String::from(part);
            }
            else {
                pair.content += part;
                pair.content += "<br>";
            }
        }
        pairs.push(pair);
    }
    pairs
}