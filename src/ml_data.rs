use std::collections::HashMap;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MLData {
    pub nodes: Vec<Node>,
    pub tree: Vec<TreeNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Node {
    pub i: String,
    #[serde(default = "default_fnz_id")]
    fnz_id: String,
    pub a: HashMap<String, String>,
}

fn default_fnz_id() -> String {
    String::from("-1")
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TreeNode {
    pub i: String,
    pub c: Option<Vec<TreeNode>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MLDataContainer {
    pub element_statistics: MLData,
}

pub fn read_ml_json(path: &Path) -> MLDataContainer{

    let json_str = fs::read_to_string(path).unwrap();

    let mut deserializer = serde_json::Deserializer::from_str(&json_str);
    deserializer.disable_recursion_limit();
    let deserializer = serde_stacker::Deserializer::new(&mut deserializer);

    MLDataContainer::deserialize(deserializer).unwrap()
}

fn calc_val(v1: f32, v2:f32) -> Option<f32>{
    if v2 == 0.0{
        None
    }else {
        Some(v1/v2)
    }
}

fn sum_rate(v1: f32, v2:f32, val: f32) -> Option<f32>{
    let rate = calc_val(v1, v2)?;
    //match rate {
    //    Some(r) =>{
    //        Some(r + val)
    //    },
    //    None => {
    //        None
    //    }
    //}

    Some(rate + val)
}

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
    height :f32
}

#[cfg(test)]
mod test{
    use std::path::Path;
    use crate::ml_data::{Person, read_ml_json};

    #[test]
    fn json_test(){
        let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

        let p: Person = serde_json::from_str(data).unwrap();

        // Do things just like with any other Rust data structure.
        //println!("Please call {} at the number {}", p.name, p.phones[0]);
    }


    #[test]
    fn load_json_test(){
        let path = Path::new("resources/1645511997141_M8INRNFV6O_curr.json");
        let data = read_ml_json(&path);

        println!("{}", data.element_statistics.nodes.len());
    }
}