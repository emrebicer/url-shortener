use crate::url_manager::{FullUrl, ShortUrlPath};

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub fn verify_http_protocol(url: &mut FullUrl) {
    if !url.starts_with("http://") && !url.starts_with("https://") {
        url.insert_str(0, "http://");
    };
}

pub fn generate_random_url_path(str_len: usize) -> ShortUrlPath {
    return thread_rng()
        .sample_iter(&Alphanumeric)
        .take(str_len)
        .map(char::from)
        .collect();
}

#[cfg(test)]
mod tests {
    #[test]
    fn generate_random_url_test() {
        let random_str = super::generate_random_url_path(3);
        assert_eq!(random_str.len(), 3);
        let random_str = super::generate_random_url_path(4);
        assert_eq!(random_str.len(), 4);
        let random_str = super::generate_random_url_path(5);
        assert_eq!(random_str.len(), 5);
    }

    #[test]
    fn verify_http_protocol_test() {
        let mut url = "https://www.rust-lang.org/".to_string();
        super::verify_http_protocol(&mut url);
        assert_eq!(url, "https://www.rust-lang.org/");

        url = "http://www.rust-lang.org/".to_string();
        super::verify_http_protocol(&mut url);
        assert_eq!(url, "http://www.rust-lang.org/");

        url = "www.rust-lang.org/".to_string();
        super::verify_http_protocol(&mut url);
        assert_eq!(url, "http://www.rust-lang.org/");

        url = "httprust-lang.org/".to_string();
        super::verify_http_protocol(&mut url);
        assert_eq!(url, "http://httprust-lang.org/");

        url = "httpsrust-lang.org/".to_string();
        super::verify_http_protocol(&mut url);
        assert_eq!(url, "http://httpsrust-lang.org/");

        // Short url test
        url = "u.me".to_string();
        super::verify_http_protocol(&mut url);
        assert_eq!(url, "http://u.me");
    }
}
