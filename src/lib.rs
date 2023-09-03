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

    let mut url = needs["instance_url"].clone();
    if url.is_empty() {
        url = String::from("https://translate.argosopentech.com/");
        // If URL is not specified, use the default URL
    }

    if !url.starts_with("http") {
        url = format!("https://{}", url);
    }

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
