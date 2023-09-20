use std::{fs::{File, self}, io::Write, collections::HashMap};
use substring::Substring;
use serde_json;


pub struct DB {
    pub dir: String,
}
#[allow(dead_code)]
impl DB {
    fn mkdir(&self) {
        if !Self::hasdir(&self) {
            std::fs::create_dir_all(&self.dir).expect("Unable to create directory");
        }
    }
    fn hasdir(&self) -> bool {
        return std::path::Path::new(&self.dir).exists();
    }
    pub fn add_key(&self, key: String, value: String) {
        let mut path = self.dir.clone();
        path.push_str("/");
        Self::mkdir(&self);
        path.push_str(&key);
        let mut file = File::create(path).expect("Unable to create file");
        file.write_all(value.as_bytes()).expect("Unable to write data");
    }

    pub fn read_key(&self, key: String) -> String {
        if Self::hasdir(&self) {
            return std::fs::read_to_string(format!("{}/{}", self.dir, key)).expect("Unable to read file");
        }
        return String::from("No such directory");
    }
    pub fn has_key(&self, key: &String) -> bool {
        if Self::hasdir(&self) {
            return std::path::Path::new(&format!("{}/{}", self.dir, key)).exists();
        }
        return false;

    }
    pub fn remove_key(&self, key: String) {
        if Self::has_key(&self, &key) {
            std::fs::remove_file(format!("{}/{}", self.dir, key)).expect("Unable to remove file");
        }
    }
    pub fn to_json(&self, list: HashMap<&str,&str>) -> String{
        let mut vec: Vec<String> = Vec::new();
        vec.push(String::from("{"));
        for (key, value) in list {
            vec.push(format!("\"{}\":\"{}\"", key, value));
            vec.push(String::from(",\n"));
        }
        vec.pop();

        vec.push(String::from("}"));
        let json = vec.join("");
        return json;
    }
    pub fn add_document(&self,key: String, value: HashMap<&str,&str>) {
        let mut path = self.dir.clone();
        path.push_str("/");
        Self::mkdir(&self);
        path.push_str(&key);
        path.push_str(".json");
        let mut file = File::create(path).expect("Unable to create file");
        file.write_all(Self::to_json(&self,value).as_bytes()).expect("Unable to write data");
    }
    pub fn add_document_owned(&self,key: String, value: HashMap<String,String>) {

        let mut path = self.dir.clone();
        path.push_str("/");
        Self::mkdir(&self);
        path.push_str(&key);
        path.push_str(".json");
        let mut file = File::create(path).expect("Unable to create file");
        file.write_all(Self::to_json_owned(&self,value).as_bytes()).expect("Unable to write data");
    }
    pub fn read_document(&self,key: String) -> HashMap<String,String> {
        if Self::hasdir(&self) {
            let mut path = self.dir.clone();
            path.push_str("/");
            path.push_str(&key);
            path.push_str(".json");
            print!("{}", path);
            let json = std::fs::read_to_string(path).expect("Unable to read file");
            let map: HashMap<String,String> = serde_json::from_str(&json).expect("Unable to parse json");
            return map;
        }
        return HashMap::new();
    }
    pub fn list_documents(&self) -> Vec<String>{
        let paths = fs::read_dir(self.dir.clone() + "/").unwrap();
        let mut vec: Vec<String> = Vec::new();
        for path in paths {
            let mut entry = path.unwrap().path().display().to_string();
            entry = entry.substring(0, entry.len()-5).to_string();
            entry = entry.substring(self.dir.len()+1, entry.len()).to_string();
            vec.push((entry).to_string());
        }
        return vec;
    }
    pub fn to_json_owned(&self, list: HashMap<String,String>) -> String{
        let mut vec: Vec<String> = Vec::new();
        vec.push(String::from("{"));
        for (key, value) in list {
            vec.push(format!("\"{}\":\"{}\"", key, value));
            vec.push(String::from(","));
        }
        vec.pop();
        vec.push(String::from("}"));
        let json = vec.join("");
        return json;
    }

}


