use bson::oid::ObjectId;

pub fn object_id_to_i32(oid: ObjectId) -> i32 {
  // Extract the first four bytes of the ObjectId
  let timestamp_bytes = oid.bytes()[0..4].try_into().unwrap();

  // Convert those bytes into an i32 (assuming big-endian byte order)
  i32::from_be_bytes(timestamp_bytes)
}