use std::{env, fs};
fn main() {
    if let Ok(dir) = env::current_dir() {
        if let Some(dir) = dir.to_str() {
           if  let Ok(files) = fs::read_dir(dir) {
            let _: Vec<_> = files.into_iter().map(|x| {
                if let Ok(dir) = x {
                    if let Ok(_) = fs::read_dir(dir.path()) {
                        println!("{:#?}", dir.file_name());
                    }
                }
            }).collect();
           }
        } }
}
