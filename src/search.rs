use std::fs;

pub fn search_pdf(path: String) -> String {
    let paths = fs::read_dir(path).unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display());
    }
    String::from("To be done")
}
