use geo::Point;

use crate::model::{GeoInfo, GeoItemId, TodoId};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Altitude(i32);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Distance(u32);

#[derive(Debug)]
pub struct GeoItem {
    id: GeoItemId,
    todo_id: TodoId,
    point: Point,
    altitude: Altitude,
    distance: Distance,
    info: GeoInfo,
}

impl GeoItem {
    pub fn new(
        id: GeoItemId,
        todo_id: TodoId,
        point: Point,
        altitude: Altitude,
        distance: Distance,
        info: GeoInfo,
    ) -> Self {
        Self {
            id,
            todo_id,
            point,
            altitude,
            distance,
            info,
        }
    }

    pub fn point(&self) -> &Point {
        &self.point
    }

    pub fn altitude(&self) -> &Altitude {
        &self.altitude
    }

    pub fn distance(&self) -> &Distance {
        &self.distance
    }

    pub fn info(&self) -> &GeoInfo {
        &self.info
    }

    pub fn id(&self) -> &GeoItemId {
        &self.id
    }

    pub fn todo_id(&self) -> &TodoId {
        &self.todo_id
    }
}
