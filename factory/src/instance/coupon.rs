#[derive(Debug, Copy, Clone)]
pub struct Coupon {}

impl Coupon {
    pub fn send(&self, uid: String, commodity_id: String, biz_id: String) -> Result<&str, &str> {
        println!("模拟发放优惠券一张：{}, {}, {}", uid, commodity_id, biz_id);
        Ok("0000")
    }
}
