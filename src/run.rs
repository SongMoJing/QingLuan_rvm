use std::fs::File;
use std::io;
use std::io::Read;
use zip::read::ZipArchive;

pub fn main(path: String) {
    let qls = File::open(path).unwrap();
    let qls = io::BufReader::new(qls);
    let mut qls: ZipArchive<io::BufReader<File>> = ZipArchive::new(qls).unwrap();

    let ls = find_file(&mut qls, "class_map").expect("Can't find Class Map");
    let parts: Vec<&str> = ls.split(' ').collect();
    println!("Class list: {:?}", parts);
    // for i in 0..qls.len() {
    //     let file = qls.by_index(i).unwrap();
    //     println!("{}", file.name().green());
    // }
}

fn find_file(archive: &mut ZipArchive<io::BufReader<File>>, file_name: &str) -> Option<String> {
    if let Ok(mut file) = archive.by_name(file_name) {
        let mut content = String::new();
        file.read_to_string(&mut content).expect("Failed to read file content");
        Option::from(content)
    } else {
        None
    }
}