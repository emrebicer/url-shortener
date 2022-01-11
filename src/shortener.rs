use chrono::{DateTime, Utc};
use std::collections::VecDeque;

type ShortUrlPath = String;
type FullUrl = String;

pub struct Shortener {
    shortened_urls: VecDeque<ShortenedUrl>,
}

struct ShortenedUrl {
    short_url: ShortUrlPath,
    full_url: FullUrl,
    time_added: DateTime<Utc>,
}

impl Shortener {
    pub fn new() -> Shortener{
        Shortener{
            shortened_urls: VecDeque::new()
        }
    }
    pub fn shorten_url(&mut self, full_url: FullUrl) -> String {
        // Check if a shortened url exists with the same full_url
        match self
            .shortened_urls
            .iter()
            .find(|su| su.full_url == *full_url)
        {
            Some(su) => {
                return su.short_url.to_string();
            }
            None => {
                // It does not exist, create a short url for given full url
                let mut loop_counter = 0;
                let mut random_url_len = 4;
                loop {
                    
                    // If it is taking so long to come up with a unique
                    // url, just increment the url length
                    loop_counter += 1;
                    if loop_counter % 25 == 0 {
                        random_url_len += 1;
                    }

                    let short_url = Shortener::generate_random_url(random_url_len);

                    match self.get_full_url(&short_url) {
                        Some(_) => continue, // Such a short url exists
                        None => {
                            // That random url is suitable
                            self.shortened_urls.push_back(ShortenedUrl {
                                short_url: short_url.to_string(),
                                full_url: full_url.to_string(),
                                time_added: chrono::offset::Utc::now(),
                            });

                            return short_url;
                        }
                    }
                }
            }
        }
    }
    pub fn get_full_url(&self, short_url: &ShortUrlPath) -> Option<FullUrl> {
        match self
            .shortened_urls
            .iter()
            .find(|su| su.short_url == *short_url)
        {
            Some(su) => return Some(su.full_url.to_string()),
            None => return None,
        }
    }

    fn generate_random_url(str_len: usize) -> String {
        let mut random_str: String = String::new();
        for _ in 0..str_len {
            random_str = format!("{}{}", random_str, rand::random::<char>());
        }
        return random_str;
    }
}
