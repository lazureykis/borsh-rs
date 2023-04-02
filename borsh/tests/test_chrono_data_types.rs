#![allow(clippy::float_cmp)]

use borsh::{BorshDeserialize, BorshSerialize};
use chrono::{DateTime, Utc};

#[derive(BorshDeserialize, BorshSerialize, PartialEq, Debug)]
struct StructWithObjectId(DateTime<Utc>);

#[test]
fn test_object_id() {
    let obj = StructWithObjectId(Utc::now());
    let serialized = obj.try_to_vec().unwrap();
    let deserialized: StructWithObjectId = BorshDeserialize::try_from_slice(&serialized).unwrap();
    assert_eq!(obj, deserialized);
}
