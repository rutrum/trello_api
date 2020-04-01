extern crate reqwest;
extern crate serde_json;

use std::error::Error;

struct Api {
    key: String,
    token: String,
}

impl Api {
    fn new(key: String, token: String) -> Api {
        Api { key, token }
    }

    fn get_boards(&self) -> Result<Vec<String>, Box<Error>> {
        use serde_json::Value;

        let url = format!(
            "https://api.trello.com/1/members/me/boards/?key={key}&token={token}",
            key = self.key,
            token = self.token
        );
        let content = reqwest::get(&url)?.text()?;

        let json: serde_json::Value = serde_json::from_str(&content)?;

        println!("{}", json);

        Ok(Vec::new())

        // match json.as_array() {
        //     None => Ok(Vec::new()),
        //     Some(&list) => {
        //         let strings: Vec<String> = list.iter()
        //             // .filter_map(|x| match x.as_object() {
        //             //     None => None,
        //             //     Some(x) => {
        //             //         x["name"]
        //             //     }
        //             // })
        //             // .filter(|x| if let Value::Array(_) = x { true } else { false })
        //             // .map(|x| match x {

        //             // })
        //             // .collect();
        //         Ok(strings)
        //     },
        // }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_creds_from_file() -> Result<(String, String), Box<Error>> {
        use std::fs::File;
        use std::io::Read;

        let mut f = File::open("keys.json")?;
        let mut content = String::new();
        f.read_to_string(&mut content)?;

        let json: serde_json::Value = serde_json::from_str(&content)?;

        // The verbosity here keeps the quotes off the output string
        let key = format!("{}", json["key"].as_str().unwrap().to_string());
        let token = format!("{}", json["token"].as_str().unwrap().to_string());

        Ok((key, token))
    }

    #[test]
    fn create_api() {
        let (key, token) = get_creds_from_file().unwrap();
        let _ = Api::new(key, token);
    }

    #[test]
    fn get_boards() {
        let (key, token) = get_creds_from_file().unwrap();
        let api = Api::new(key, token);
        api.get_boards();
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
