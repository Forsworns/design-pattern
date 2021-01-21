#[derive(Clone)]
pub struct Video {}

pub trait DownloadVideo {
    fn download(&mut self) -> Video;
}

pub struct RealVideoDownloader {}

impl DownloadVideo for RealVideoDownloader {
    fn download(&mut self) -> Video {
        std::thread::sleep(std::time::Duration::new(3, 0));
        Video {}
    }
}
pub struct CachedVideoDownloader {
    video: Option<Video>,
    downloader: RealVideoDownloader,
}

impl DownloadVideo for CachedVideoDownloader {
    fn download(&mut self) -> Video {
        println!("Downloading....");
        if self.video.is_none() {
            self.video = Some(self.downloader.download());
        }
        self.video.clone().unwrap()
    }
}

impl CachedVideoDownloader {
    pub fn new() -> Self {
        Self {
            video: None,
            downloader: RealVideoDownloader {},
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    // cargo test -- --nocapture

    #[test]
    fn it_works() {
        let mut proxy = CachedVideoDownloader::new();
        for _ in 1..5 {
            proxy.download();
        }
    }
}
