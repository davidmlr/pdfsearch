use pdf_extract;
use std::fs;

pub fn search_pdf(path: String, search: String) -> Vec<String> {
    let paths = fs::read_dir(path.clone());
    let paths = match paths {
        Ok(val) => val,
        Err(_) => return vec![String::from("Fehler")], // TODO Fehler nicht so dumm bearbeiten.
                                                       // Result zurück geben.
    };

    let mut result = vec![];
    for path in paths {
        let path = path.unwrap().path();
        let out = pdf_extract::extract_text(&path).unwrap();
        if out.contains(&search) {
            let mut name = path.display().to_string();
            name = String::from(name.split("/").last().unwrap());
            name = format!("{}\n", name);
            result.push(name);
        }
        // println!("{}", out);
    }
    result
}
