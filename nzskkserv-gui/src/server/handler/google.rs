use log::info;

type GoogleCgiResponse = Vec<(String, Vec<String>)>;

pub(super) async fn google_cgi_convert(query: &str) -> Result<Vec<String>, anyhow::Error> {
    let mut url = "http://www.google.com/transliterate?langpair=ja-Hira|ja&text=".to_string();
    url.push_str(&urlencoding::encode(query));
    url.push(',');
    let mut result = reqwest::get(url).await?.json::<GoogleCgiResponse>().await?;

    info!("Converted by google cgi server: {:?}", result);

    if result.is_empty() {
        return Ok(vec![]);
    }
    let candidates = result.swap_remove(0).1;

    Ok(candidates)
}
