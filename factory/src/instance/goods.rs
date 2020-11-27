#[derive(Debug, Copy, Clone)]
pub struct Goods {}

impl Goods {
    pub fn deliver(&self, req: DeliverReq) -> Result<&str, &str> {
        println!("Goods delivered {:?}", req);
        Ok("0000")
    }
}

#[derive(Debug, Clone)]
pub struct DeliverReq {
    uid: String,
    commodity_id: String,
    biz_id: String,
    consignee_name: String,
    consignee_mobile: String,
    consignee_address: String,
}

impl DeliverReq {
    pub fn new(
        uid: String,
        commodity_id: String,
        biz_id: String,
        consignee_name: String,
        consignee_mobile: String,
        consignee_address: String,
    ) -> Self {
        Self {
            uid,
            commodity_id,
            biz_id,
            consignee_name,
            consignee_mobile,
            consignee_address,
        }
    }
}
