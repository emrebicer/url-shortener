use chrono::{DateTime, Utc};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::collections::hash_map::HashMap;
use std::sync::Mutex;

type ShortUrlPath = String;
type FullUrl = String;

pub struct Shortener {
    shortened_urls: Mutex<HashMap<String, ShortenedUrl>>,
}

#[derive(Clone)]
struct ShortenedUrl {
    short_url: ShortUrlPath,
    full_url: FullUrl,
    time_added: DateTime<Utc>, // It will be usefull when
                               // I want to delete very old
                               // shortened urls in the future
}

impl Shortener {
    pub fn new() -> Shortener {
        Shortener {
            shortened_urls: Mutex::new(HashMap::new()),
        }
    }
    pub fn shorten_url(&self, full_url: &mut FullUrl) -> ShortUrlPath {
        // Check if a shortened url exists with the same full_url
        let mut shortened_urls = self.shortened_urls.lock().unwrap();
        if !full_url.starts_with("http://") && !full_url.starts_with("https://") {
            full_url.insert_str(0, "http://");
        }
        let short_url_path = match shortened_urls.values().find(|su| su.full_url == *full_url) {
            Some(su) => su.short_url.to_string(),
            None => {
                // It does not exist, create a short url for given full url
                Shortener::shorten_to_unique_url(&mut shortened_urls, full_url)
            }
        };
        return short_url_path;
    }

    pub fn get_full_url(&self, short_url: &ShortUrlPath) -> Option<FullUrl> {
        let shortened_urls = self.shortened_urls.lock().unwrap();
        let full_url = match shortened_urls.get(short_url) {
            Some(su) => Some(su.full_url.to_string()),
            None => None,
        };
        return full_url;
    }

    fn shorten_to_unique_url(
        shortened_urls: &mut HashMap<String, ShortenedUrl>,
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

            let short_url = Shortener::generate_random_url(random_url_len);

            // Check if the found random short url is already in use
            match shortened_urls.get(&short_url) {
                Some(_) => continue, // Such a short url exists
                None => {
                    let short_url_clone = short_url.clone();
                    // That random url is suitable
                    shortened_urls.insert(
                        short_url,
                        ShortenedUrl {
                            short_url: short_url_clone.to_string(),
                            full_url: full_url.to_string(),
                            time_added: chrono::offset::Utc::now(),
                        },
                    );
                    return short_url_clone;
                }
            }
        }
    }

    fn generate_random_url(str_len: usize) -> String {
        return thread_rng()
            .sample_iter(&Alphanumeric)
            .take(str_len)
            .map(char::from)
            .collect();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn generate_random_url_test() {
        let random_str = super::Shortener::generate_random_url(3);
        assert_eq!(random_str.len(), 3);
        let random_str = super::Shortener::generate_random_url(4);
        assert_eq!(random_str.len(), 4);
        let random_str = super::Shortener::generate_random_url(5);
        assert_eq!(random_str.len(), 5);
    }

    #[test]
    fn shorten_url_test() {
        let shorty = super::Shortener::new();

        let mut full_url = "https://www.rust-lang.org".to_string();
        let short_url = shorty.shorten_url(&mut full_url);
        let found_full_url = shorty.get_full_url(&short_url);
        assert_eq!(found_full_url, Some(full_url));
        assert_eq!(
            shorty.get_full_url(&"https://non_existings.com".to_string()),
            None
        );

        let mut full_url_no_http_prefix = "www.rust-lang.org".to_string();
        let full_url_with_http_prefix = "http://www.rust-lang.org".to_string();
        let short_url = shorty.shorten_url(&mut full_url_no_http_prefix);
        let found_full_url = shorty.get_full_url(&short_url);
        assert_eq!(found_full_url, Some(full_url_with_http_prefix));
    }
}
