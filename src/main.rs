use std::fs::File;
use std::io::Read;
use cafebabe::ClassFile;

fn main() {
    let mut file = File::open("./zz.class").unwrap();
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).unwrap();
    println!("Parsing zz.class ...");
    match cafebabe::parse_class(&bytes) {
        Ok(class) => parse_class(class),
        Err(e) => println!("Error: {}", e),
    };
}

fn parse_class(class: ClassFile) {
    // println!("Successfully parsed {:?}\n{:#?}", class.this_class, class);
    println!("Class name: {}", class.this_class);
    println!("Fields: {:#?}", class.fields);
}