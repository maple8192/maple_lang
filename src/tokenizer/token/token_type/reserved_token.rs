pub trait ReservedToken {
    fn to_str(&self) -> &str;
    fn get_len_order_list() -> Vec<Self>;
}