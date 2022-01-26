use crate::url_manager::{FullUrl, ShortUrlPath, UrlManager};
use crate::util::{generate_random_url_path, insert_https_protocol};
use chrono::{DateTime, Utc};
use std::collections::hash_map::HashMap;
use std::sync::Mutex;

pub struct MemoryDatabase {
    shortened_urls: Mutex<HashMap<ShortUrlPath, ShortenedUrl>>,
}

#[derive(Clone)]
pub struct ShortenedUrl {
    short_url_path: ShortUrlPath,
    full_url: FullUrl,
    _time_added: DateTime<Utc>, // It will be usefull when
                                // I want to delete very old
                                // shortened urls in the future
}

impl UrlManager for MemoryDatabase {
    fn shorten_url(&self, full_url: &mut FullUrl) -> ShortUrlPath {
        // Make sure the full url has the http protocol prefix
        insert_https_protocol(full_url);

        // Check if a shortened url exists with the same full_url
        let mut shortened_urls = self.shortened_urls.lock().expect("The mutex is poisened");
        let short_url_path = match shortened_urls.values().find(|su| su.full_url == *full_url) {
            Some(su) => su.short_url_path.to_string(),
            None => {
                // It does not exist, create a unique short url for given full url
                shorten_to_unique_url(&mut shortened_urls, full_url)
            }
        };
        return short_url_path;
    }

    fn get_full_url(&self, short_url_path: &ShortUrlPath) -> Option<FullUrl> {
        let shortened_urls = self.shortened_urls.lock().expect("The mutex is poisened");
        shortened_urls
            .get(short_url_path)
            .map_or(None, |su| Some(su.full_url.to_string()))
    }
}

impl MemoryDatabase {
    pub fn new() -> MemoryDatabase {
        MemoryDatabase {
            shortened_urls: Mutex::new(HashMap::new()),
        }
    }
}

fn shorten_to_unique_url(
    shortened_urls: &mut HashMap<ShortUrlPath, ShortenedUrl>,
    full_url: &FullUrl,
) -> ShortUrlPath {
    let mut loop_counter = 0;
    let mut random_url_len = 2;
    loop {
        // If it is taking so long to come up with a unique
        // url, just increment the url length
        loop_counter += 1;
        if loop_counter % 25 == 0 {
            random_url_len += 1;
        }

        let short_url_path = generate_random_url_path(random_url_len);

        // Check if the found random short url is already in use
        match shortened_urls.get(&short_url_path) {
            Some(_) => continue, // Such a short url exists
            None => {
                // That random url is suitable
                shortened_urls.insert(
                    short_url_path.clone(),
                    ShortenedUrl {
                        short_url_path: short_url_path.to_string(),
                        full_url: full_url.to_string(),
                        _time_added: chrono::offset::Utc::now(),
                    },
                );
                return short_url_path;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::url_manager::UrlManager;

    #[test]
    fn shorten_url_test() {
        let shorty = super::MemoryDatabase::new();

        let mut full_url = "https://www.rust-lang.org".to_string();
        let short_url = shorty.shorten_url(&mut full_url);
        let found_full_url = shorty.get_full_url(&short_url);
        assert_eq!(found_full_url, Some(full_url));
        assert_eq!(
            shorty.get_full_url(&"https://non_existings.com".to_string()),
            None
        );

        let mut full_url_no_http_prefix = "www.rust-lang.org".to_string();
        let full_url_with_https_prefix = "https://www.rust-lang.org".to_string();
        let short_url = shorty.shorten_url(&mut full_url_no_http_prefix);
        let found_full_url = shorty.get_full_url(&short_url);
        assert_eq!(found_full_url, Some(full_url_with_https_prefix));
    }
}
