

#[macro_export]
macro_rules! data_from_file {
    ($filename: expr) => {
       {
        let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file_path.push("data");
        file_path.push("commands");
        file_path.push($filename);

        file_path
       } 
    };
}