use std::error::Error;
use std::io::{Error as IOError, ErrorKind};

use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GithubRepo {
    pub title: String,
    pub link: String,
    pub description: String,
}

fn selector_parse(s: &str) -> Result<Selector, Box<dyn Error>> {
    match Selector::parse(s) {
        Ok(selector) => Ok(selector),
        Err(_) => Err(Box::new(IOError::new(ErrorKind::Other,
                                            format!("parse selector error: {}", s))))
    }
}

pub async fn get_github_trending(lang: &str, since: &str) -> Result<Vec<GithubRepo>, Box<dyn Error>> {
    let url = format!("https://github.com/trending/{}?since={}&spoken_language_code=en", lang, since);
    let html = reqwest::get(url).await?.text().await?;

    let doc = Html::parse_fragment(&html);
    let post_select = selector_parse("article[class=Box-row]")?;
    let res = doc.select(&post_select);
    let mut items: Vec<GithubRepo> = Vec::new();
    for element in res {
        let titles: Option<Vec<String>> = match element.select(
            &selector_parse("a[data-view-component=true]")?
        ).into_iter().last() {
            Some(data) => {
                match data.value().attr("href") {
                    Some(data) => {
                        let titles: Vec<String> = data.split("/").map(|x| x.to_string()).collect();
                        if titles.len() < 3 {
                            None
                        } else {
                            // titles
                            Some(titles)
                        }
                    }
                    None => None
                }
            }
            None => None
        };
        let titles = match titles {
            Some(data) => data,
            None => continue
        };
        let title = format!("{} / {}", titles[1], titles[2]);
        let language = match element.select(
            &selector_parse("span[itemprop=programmingLanguage]")?
        ).next() {
            Some(data) => data.inner_html().trim().to_string(),
            None => "".to_string()
        };
        let desc = match element.select(
            &selector_parse("p")?)
            .next() {
            Some(data) => data.inner_html().trim().to_string(),
            None => "".to_string()
        };
        let link = format!("https://github.com/{}/{}", titles[1], titles[2]);
        let star = match element.select(
            &selector_parse(&format!("a[href='/{}/{}/stargazers']", titles[1], titles[2]))?)
            .next() {
            Some(data) => match data.last_child() {
                Some(data) => match data.value().as_text() {
                    Some(data) => data.trim().replace(",", "").parse::<i32>()?,
                    None => 0
                }
                None => 0
            }
            None => 0
        };

        items.push(GithubRepo {
            title,
            link,
            description: format!("\n<p>Desc: {}</p>\n<p>Lang: {}</p>\n<p>Star: {}</p>", desc, language, star),
        });
    }

    Ok(items)
}