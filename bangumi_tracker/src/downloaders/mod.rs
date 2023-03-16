mod aria2;
pub enum Downloader {
    Aria2,
}

impl Default for Downloader {
    fn default() -> Self {
        Downloader::Aria2
    }
}
