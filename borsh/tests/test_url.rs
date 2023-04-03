use borsh::{BorshDeserialize, BorshSerialize};
use url::Url;

#[derive(BorshDeserialize, BorshSerialize, PartialEq, Debug)]
struct StructWithUrl(Url);

#[test]
fn test_object_id() {
    let obj = StructWithUrl(Url::parse("https://google.com/path?param=1#section1").unwrap());
    let serialized = obj.try_to_vec().unwrap();
    let deserialized: StructWithUrl = BorshDeserialize::try_from_slice(&serialized).unwrap();
    assert_eq!(obj, deserialized);
}
