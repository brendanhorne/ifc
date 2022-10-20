use std::fs;
use std::path::Path;

fn main() {
    // read a sample step file from official IFC examples
    let path = Path::new(
        "./../github/IFC/Examples/Basic geometric shape/Examples/Extruded solid/File.ifc",
    )
    .to_str()
    .unwrap();
    println!("In file {}", path);

    let contents = fs::read_to_string(path).expect("Should have been able to read the file");

    println!("{contents}");
}
