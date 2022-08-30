pub mod query {
    use crate::msg::ValueResp;

    pub fn value() -> ValueResp {
        ValueResp { value: 0 }
    }
}
