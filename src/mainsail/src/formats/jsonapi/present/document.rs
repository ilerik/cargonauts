use serde::{Serialize, Serializer};
use serde::ser::SerializeMap;
use json;

use super::traits::ErrorObject;

pub struct Document<T> {
    pub member: T,
}

impl<T: Serialize> Document<T> {
    pub fn write(&self) -> Result<Vec<u8>, json::Error> {
        let mut serializer = json::Serializer::new(vec![]);
        {
            let mut map = serializer.serialize_map(Some(2))?;
            map.serialize_entry("data", &self.member)?;
            // TODO links
            map.serialize_entry("jsonapi", &JsonApiObject)?;
            map.end()?;
        }
        Ok(serializer.into_inner())
    }
}

struct JsonApiObject;

impl Serialize for JsonApiObject {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry("version", "1.0")?;
        map.end()
    }
}

pub struct ErrorDocument<'a> {
    pub error: ErrorObject<'a>,
}

impl<'a> ErrorDocument<'a> {
    pub fn write(&self) -> Result<Vec<u8>, json::Error> {
        let mut serializer = json::Serializer::new(vec![]);
        {
            let mut map = serializer.serialize_map(Some(2))?;
            map.serialize_entry("error", &self.error)?;
            map.serialize_entry("jsonapi", &JsonApiObject)?;
            map.end()?;
        }
        Ok(serializer.into_inner())
    }
}
