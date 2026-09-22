use url::Url;

pub fn normalize_navigation(input: &str, search_template: &str) -> Result<String, String> {
    let value = input.trim();
    if value.is_empty() { return Err("empty navigation target".into()); }
    if let Ok(url) = Url::parse(value) {
        match url.scheme() {
            "https" | "about" => return Ok(url.to_string()),
            "http" => return Err("HTTP navigation is disabled by Voidium's HTTPS-first policy.".into()),
            _ => {}
        }
    }
    let encoded = urlencoding::encode(value).into_owned();
    let template = if search_template.contains("{query}") { search_template } else { "https://www.google.com/search?q={query}" };
    Ok(template.replace("{query}", &encoded))
}
