use uuid::Uuid;

pub mod geo;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TodoId(Uuid);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GeoItemId(Uuid);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GeoInfo(u32);
