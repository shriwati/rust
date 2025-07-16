
fn get_list_of_files(dir_name: &str, ext: &str)->Vec<String> {
    let mut list_of_files:Vec<String> = vec![];
    for entry in std::fs::read_dir(&dir_name).unwrap() {
        let file_name= entry.unwrap().path();
        if !file_name.is_dir() && file_name.extension().unwrap() == ext{
            list_of_files.push(file_name.display().to_string());
        }
    }
    list_of_files
}
