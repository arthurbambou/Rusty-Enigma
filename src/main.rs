use std::fs::File;
use std::io::{Error, Read};
use noak::error::DecodeError;
use noak::reader::Class;

fn main() {
    let mut file = File::open("./zz.class").unwrap();
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).unwrap();
    println!("Parsing zz.class ...");
    // match cafebabe::parse_class(&bytes) {
    //     Ok(class) => parse_class(class),
    //     Err(e) => println!("Error: {}", e),
    // };
    match Class::new(&bytes) {
        Ok(class) => match parse_class(class) {
            Ok(_) => println!("It went ok"),
            Err(e) => println!("Error: {}", e)
        },
        Err(e) => println!("Error: {}", e)
    };
}

fn parse_class(mut class: Class) -> Result<(), DecodeError> {

    println!("Fields");
    for field in class.fields()? {
        let field = field?;
        let pool = class.pool()?;
        println!("  - {}:", pool.retrieve(field.name())?.display());
        println!("    - Access Flags: {:?}", field.access_flags());
        println!("    - Descriptor: {}", pool.retrieve(field.descriptor())?.display());
    };

    println!("Methods");
    for method in class.methods()? {
        let method = method?;
        let pool = class.pool()?;
        println!("  - {}:", pool.retrieve(method.name())?.display());
        println!("    - Access Flags: {:?}", method.access_flags());
        println!("    - Descriptor: {}", pool.retrieve(method.descriptor())?.display());

        for attribute in method.attributes() {
            let attribute = attribute?;
            println!("\t\t\t- {}", pool.retrieve(attribute.name())?.display());
            println!("\t\t\t\t- Content {:#?}", attribute.content());
        }
    }


    Ok(())
}