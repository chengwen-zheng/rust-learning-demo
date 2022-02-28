use std::fs;

fn main() {
    let url = "https://www.rust-lang.org/";
    let output = "rust.md";

    print!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    print!("Converting html to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes()).unwrap();

    print!("Converted markdown has saved in {}", output);
}
