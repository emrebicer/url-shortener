pub type ShortUrlPath = String;
pub type FullUrl = String;

pub trait UrlManager {
    fn get_full_url(&self, short_url_path: &ShortUrlPath) -> Option<FullUrl>;
    fn shorten_url(&self, full_url: &mut FullUrl) -> ShortUrlPath;
}
