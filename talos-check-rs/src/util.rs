use serde::ser;

pub fn to_yaml<T>(value: &T) -> String
where
    T: ?Sized + ser::Serialize,
{
    serde_yaml::to_string(value).unwrap()
}
