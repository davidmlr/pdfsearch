mod app;
pub use app::Search;
use pdf_extract;
use std::fs;
use std::path::Path;

pub fn search_pdf(path: String, search: String) -> String {
    let paths = fs::read_dir(Path::new(&path));
    let paths = match paths {
        Ok(val) => val,
        Err(_) => return String::from("Pfad ungültig"), // TODO Fehler nicht so bearbeiten.
                                                        // Result zurück geben.
    };

    let mut result = String::new();
    for path in paths {
        let path = path.unwrap().path();
        let out = match pdf_extract::extract_text(&path) {
            Ok(val) => val,
            Err(_) => return String::from("Ordner darf nur PDF Dateien enthalten!"),
        };
        if out.contains(&search) {
            let mut name = path.display().to_string();
            name = String::from(name.split("/").last().unwrap());
            name = format!("{}\n", name);
            result.push_str(&name);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::search_pdf;

    #[test]
    fn it_works() {
        let path = String::from("example/");
        let search = String::from("Lorem");
        assert_eq!(
            String::from("test_file_1.pdf\ntest_file_2.pdf\ntest_file_3.pdf\ntest_file_4.pdf\ntest_file_5.pdf\n"),
            search_pdf(path, search)
        );
    }

    #[test]
    fn not_works() {
        let path = String::from("example/");
        let search = String::from("Fedora");
        assert_eq!(String::from("test_file_5.pdf\n"), search_pdf(path, search));
    }
}
