fn create_enum() {
    #[derive(Debug)]
    enum CodingLanguage {
        Python,
        Rust,
        JavaScript,
    }
    let rust: CodingLanguage = CodingLanguage::Rust;
    let python: CodingLanguage = CodingLanguage::Python;
    let js: CodingLanguage = CodingLanguage::JavaScript;

    println!("\nWe're coding in {:?}", rust);
    println!("I know a little {:?}", js);
    println!("I'm best at {:?}\n", python);
}

pub fn show_me_enums() {
    println!("\nEnums (enumerated value) is a value that has to be one of the defined values.");
    create_enum();
    println!("\n")
}
