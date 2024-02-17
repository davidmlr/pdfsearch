use crate::search::search_pdf;

mod search;

fn main() {
    let path = String::from("example");
    search_pdf(path);
}
