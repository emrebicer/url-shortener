use chrono::{DateTime, Utc};
use std::collections::VecDeque;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

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
    pub fn shorten_url(&mut self, full_url: &FullUrl) -> ShortUrlPath {
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

       return thread_rng()
            .sample_iter(&Alphanumeric)
            .take(str_len)
            .map(char::from)
            .collect();

        //let mut random_str: String = String::new();
        //for _ in 0..str_len {
            //let ch = rand::random::<char>().to_string();
            //println!("random char: {}", ch);
            //random_str = format!("{}{}", random_str, ch);
        //}
        //return random_str;
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
        let mut shorty = super::Shortener::new();

        let full_url = "https://google.com".to_string();
        let short_url = shorty.shorten_url(&full_url);
        let found_full_url = shorty.get_full_url(&short_url);
        assert_eq!(found_full_url, Some(full_url));
        assert_eq!(shorty.get_full_url(&"non_existings.com".to_string()), None);
    }

}
