use deluxe::*;

//
// Iter
//

#[derive(Default, ParseMetaItem)]
pub enum Iter {
    #[default]
    None,
    Item,
    #[deluxe(rename = kv)]
    KeyValue,
}
