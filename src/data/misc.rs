fn about_null() {
    println!("\nYou can define nothing types to replace null types which are used to type stuff that you don't want to type or as a null");
    println!("They don't take up memory at compliation.");
    println!("Also known as unit.");
    let _null: () = ();
    println!("Doesn't do anything else.");
}

fn about_aliases() {
    println!("\nAliases can rename things");

    #[derive(Debug)]
    enum CodingLanguage {
        Python,
        Rust,
    }
    type MyLanguages = CodingLanguage;
    let python: CodingLanguage = MyLanguages::Python;
    let rust: CodingLanguage = MyLanguages::Rust;

    println!("Aliases are just a way of renaming in scope: {:?} + {:?} are instances of CodingLanguage but are called using an alias.", python, rust);
}

pub fn about_misc_data_types() {
    println!("\nOther data types");
    about_null();
    about_aliases();
}
