use crate::{join_all, reqwest, tokio, Regex};

pub struct Client {}

impl Client {
    pub async fn get_url(&self, url: String) -> reqwest::Result<String> {
        let response = reqwest::get(&url).await?;
        let text = response.text().await?;
        Ok(text)
    }

    #[tokio::main]
    pub async fn get_name(&self, urls: Vec<String>) -> Vec<String> {
        let results = join_all(urls.into_iter().map(|url| self.get_url(url))).await;
        let mut htmls = vec![];
        for result in results {
            match result {
                Ok(html) => htmls.push(html),
                Err(err) => {
                    panic!("Err: {:?}", err);
                }
            }
        }
        let mut names = vec![];
        for html in htmls {
            let re_res = parse_name(html);
            names.extend_from_slice(re_res.as_slice());
        }
        names
    }
}

fn parse_name(html: String) -> Vec<String> {
    let re = Regex::new(r"<title>(.*)</title>").unwrap();
    let mut names = vec![];
    for cap in re.captures_iter(&html) {
        names.push(cap[1].into());
    }
    names
}
