use core::num::flt2dec::strategy::grisu::format_exact;

trait HTML {
    fn html_render(&self) -> String;
}

struct UpcomingEvent {
    date: String,
    title: String,
    content: String,
}

impl UpcomingEvent {

    fn parse(input: &String) -> UpcomingEvent {
        let data: Vec<&str> = input.trim().split("\n").collect();

        if data.len() != 3 { panic!("there should be 3 parts in parser input") }

        UpcomingEvent {
            date: data[1].to_string(),
            title: data[0].to_string(),
            content: data[2].to_string()
        }
    }
}

impl HTML for UpcomingEvent {
    fn html_render(&self) -> String {
        let html = format!("
        <div class=\"col s6 m4\">
            <div class=\"card grey lighten-2 z-depth-3\">
                <div class=\"card-content\">
                    <span class=\"card-title\">{}</span>
                    <h6 style=\"font-weight: bold;\">{}</h6>
                    <p>{}</p>
                </div>
            </div>
        </div>",
        self.date,
        self.title,
        basic_formatting_converter(self.content)
        );
    }
}

struct MeetingNote {
    title: String,
    content: header_body_pair
}

struct HeaderBodyPair {
    header: String,
    body: String
}

struct FileData {
    title: String,
    uploaded: String,
    content: String,
    path: String
}

struct Contact {
    name: String,
    email: String
}

fn basic_formatting_converter(mut input: String) -> String {
    /*
    ^^ => italics
    ** => bold
    __ => underline
    */
    let mut italics_acive = false;
    let mut bold_active = false;
    let mut underline_active = false;
    for i in 0..input.len()-1 {
        let curr = format!("{}{}",input[i],input[i+1]);
        if curr == "^^".to_string() {
            if italics_acive {
                input.replace_range(i..i+2, "</i>");
            } else {
                input.replace_range(i..i+2, "<i>")
            }
            italics_acive = !italics_acive;
        }
        else if curr == "**".to_string() {
            if bold_active {
                input.replace_range(i..i+2, "</b>");
            } else {
                input.replace_range(i..i+2, "</b>");
            }
            bold_active = !bold_active;
        }
        else if curr == "__".to_string() {
            if underline_active {
                input.replace_range(i..i+2, "</u>");
            } else {
                input.replace_range(i..i+2, "</u>");
            }
            underline_active = !underline_active;
        }
    }
    input
}