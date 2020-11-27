#[derive(Debug, Copy, Clone)]
pub struct Card {}

impl Card {
    pub fn grant_token(&self, mobile: String, biz_id: String) -> Result<&str, &str> {
        println!("Card granted to the mobile {}, biz_id {}", mobile, biz_id);
        Ok("0000")
    }
}
