use std::fs::File;
use std::io::Read;
use serde::de::DeserializeOwned;

pub trait BaseWorker {
    fn new(config: &str) -> Self;
    fn execute(&self) -> Result<(), &'static str>;

    fn read_json_into_string<T>(json_path: &str) -> T
        where
            T: DeserializeOwned, // Specify that T must implement DeserializeOwned
    {
        let mut file = File::open(json_path).expect("Failed to open the file");
        let mut json_data = String::new();
        file.read_to_string(&mut json_data).expect("Failed to read the file");
        let config: T = serde_json::from_str(&json_data).expect("Failed to deserialize JSON");
        config
    }
}
