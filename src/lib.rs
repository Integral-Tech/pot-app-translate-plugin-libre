use log;
use serde_json::Value;
use std::{collections::HashMap, error::Error};

#[no_mangle]
pub fn translate(
    text: &str,                     // Text to be translated
    from: &str,                     // Source Language
    to: &str,                       // Target Language
    needs: HashMap<String, String>, // Other arguments defined in info.json
) -> Result<Value, Box<dyn Error>> {
    let client = reqwest::blocking::ClientBuilder::new().build()?;

    let url = match needs.get("instance_url") {
        Some(raw_url) => {
            if !raw_url.starts_with("http") {
                format!("https://{raw_url}")
            } else {
                raw_url.to_string()
            }
        }

        None => {
            log::info!("Instance URL is not specified, use translate.argosopentech.com");
            String::from("https://translate.argosopentech.com/")
        }
    };

    let mut body = HashMap::new();
    body.insert("q", text);
    body.insert("source", from);
    body.insert("target", to);

    if let Some(api_key) = needs.get("api_key") {
        body.insert("api_key", api_key);
    } // Optional API Key

    let res: Value = client
        .post(format!("{url}/translate"))
        .json(&body)
        .send()?
        .json()?;

    fn parse_result(res: Value) -> Option<String> {
        let result = res
            .as_object()?
            .get("translatedText")?
            .as_str()?
            .to_string();

        Some(result)
    }

    match parse_result(res) {
        Some(result) => Ok(Value::String(result)),
        _ => Err("Response Parse Error".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_request() {
        let mut needs = HashMap::new();
        needs.insert(
            "instance_url".to_string(),
            "translate.argosopentech.com".to_string(),
        );

        let result = translate("这是一个自由的翻译 API", "zh", "en", needs).unwrap();
        println!("{result}");
    }
}
