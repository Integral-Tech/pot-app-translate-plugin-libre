use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

#[no_mangle]
pub fn translate(
    text: &str,                     // Text to be translated
    from: &str,                     // Source Language
    to: &str,                       // Target Language
    needs: HashMap<String, String>, // Other arguments defined in info.json
) -> Result<String, Box<dyn Error>> {
    let client = reqwest::blocking::ClientBuilder::new().build()?;

    let url = match needs.get("instance_url") {
        Some(_) => {
            let raw_url = needs.get("instance_url").unwrap();
            if !raw_url.starts_with("http") {
                format!("https://{raw_url}")
            } else {
                raw_url.to_string()
            }
        }

        None => {
            println!("Instance URL is not specified, use translate.argosopentech.com");
            String::from("https://translate.argosopentech.com/")
        }
    };

    let mut body = HashMap::new();
    body.insert("q", text);
    body.insert("source", from);
    body.insert("target", to);

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

        Some(result.replace("@@", "/"))
    }

    match parse_result(res) {
        Some(result) => return Ok(result),
        _ => return Err("Response Parse Error".into()),
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
