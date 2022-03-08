use std::io::Error;
use std::fs;
use std::time::SystemTime;
use std::cmp::Reverse;

fn main() {
    println!("Hello, world!");
    let folder_path = "C:\\Users\\distu\\Downloads\\New folder\\Random 2022\\";
    println!("{:?}", file_namer(folder_path));
}

fn file_namer(folder_path: &str) -> Result<bool, Error> {
    std::env::set_current_dir(folder_path)?;
    let sys_time = SystemTime::now();
    let mut paths: Vec<_> = fs::read_dir(folder_path).unwrap().filter_map(Result::ok).collect();
    println!("{:?}", paths);
    paths.retain(|path| fs::metadata(path.path()).unwrap().is_file());
    let file_names = ["jpg"];//, "gif", "png"];
    //let mut pictures: Vec<_>; 
    for file_name in file_names {
        paths.retain(|path| path.path().extension().unwrap() == file_name);
    }
    paths.sort_by_key(|path| Reverse(sys_time.duration_since(fs::metadata(path.path()).unwrap().modified().unwrap()).unwrap().as_millis()));
    println!("{:?}", paths);
    for (index, path) in paths.iter().enumerate() {
        let file = path.path();
        //println!("{:?}", path);
        //println!("{:?}", file);
        //println!("{:?}", fs::metadata(&file)?.file_type());
        println!("{:?}", file.file_name());
        println!("{:?}", file.extension());
        //fs::rename(file, index.to_string())?;
    }





/*

    for (index, content) in fs::read_dir(folder_path)?.enumerate() {
        let file = content?.path();
        if fs::metadata(&file)?.is_file() {
            println!("{:?}", fs::metadata(&file)?.modified()?);
            println!("{:?}", fs::metadata(&file).unwrap().modified().unwrap());
            //println!("{:?}", file);
            //fs::rename(file, index.to_string())?;
        }

 */       
    //}
    return Ok(true)
}