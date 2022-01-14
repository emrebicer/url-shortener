use chrono::{DateTime, Utc};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::collections::VecDeque;
use std::sync::Mutex;

type ShortUrlPath = String;
type FullUrl = String;

pub struct Shortener {
    shortened_urls: Mutex<VecDeque<ShortenedUrl>>,
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
            shortened_urls: Mutex::new(VecDeque::new()),
        }
    }
    pub fn shorten_url(&self, full_url: &mut FullUrl) -> ShortUrlPath {
        // Check if a shortened url exists with the same full_url
        let mut shortened_urls = self.shortened_urls.lock().unwrap();
        if !full_url.starts_with("http://") && !full_url.starts_with("https://") {
            full_url.insert_str(0, "http://");
        }
        let short_url_path = match shortened_urls.iter().find(|su| su.full_url == *full_url) {
            Some(su) => su.short_url.to_string(),
            None => {
                // It does not exist, create a short url for given full url
                Shortener::shorten_to_unique_url(&mut shortened_urls, full_url)
            }
        };

        // TODO: Not sure if I HAVE to drop the locked mutex, or
        // It unlocks the mutex by itself when the function returns
        drop(shortened_urls);
        return short_url_path;
    }

    pub fn get_full_url(&self, short_url: &ShortUrlPath) -> Option<FullUrl> {
        let shortened_urls = self.shortened_urls.lock().unwrap();
        let full_url = match shortened_urls.iter().find(|su| su.short_url == *short_url) {
            Some(su) => Some(su.full_url.to_string()),
            None => None,
        };

        // TODO: Not sure if I HAVE to drop the locked mutex, or
        // It unlocks the mutex by itself when the function returns
        drop(shortened_urls);
        return full_url;
    }

    fn shorten_to_unique_url(
        shortened_urls: &mut VecDeque<ShortenedUrl>,
        full_url: &FullUrl,
    ) -> ShortUrlPath {
        let mut loop_counter = 0;
        let mut random_url_len = 3;
        loop {
            // If it is taking so long to come up with a unique
            // url, just increment the url length
            loop_counter += 1;
            if loop_counter % 25 == 0 {
                random_url_len += 1;
            }

            let short_url = Shortener::generate_random_url(random_url_len);

            // Check if the found random short url is already in use
            match shortened_urls
                .into_iter()
                .any(|shortened| shortened.short_url == short_url)
            {
                true => continue, // Such a short url exists
                false => {
                    // That random url is suitable
                    shortened_urls.push_back(ShortenedUrl {
                        short_url: short_url.to_string(),
                        full_url: full_url.to_string(),
                        time_added: chrono::offset::Utc::now(),
                    });
                    return short_url;
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
        assert_eq!(shorty.get_full_url(&"https://non_existings.com".to_string()), None);

        let mut full_url_no_http_prefix = "www.rust-lang.org".to_string();
        let full_url_with_http_prefix = "http://www.rust-lang.org".to_string();
        let short_url = shorty.shorten_url(&mut full_url_no_http_prefix);
        let found_full_url = shorty.get_full_url(&short_url);
        assert_eq!(found_full_url, Some(full_url_with_http_prefix));
    }
}
