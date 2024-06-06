pub fn path_of_file(file_name: String) -> String
{
    let mut base_path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    base_path.push_str("/conf/");
    base_path.push_str(file_name.as_str());
    return base_path;
}