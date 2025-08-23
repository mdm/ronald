use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::iter::FromIterator;

pub fn serialize<'a, T, K, V, S>(target: T, ser: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: IntoIterator<Item = (&'a K, &'a V)>,
    K: Serialize + 'a,
    V: Serialize + 'a,
{
    let container: Vec<_> = target.into_iter().collect();
    serde::Serialize::serialize(&container, ser)
}

pub fn deserialize<'de, T, K, V, D>(des: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromIterator<(K, V)>,
    K: Deserialize<'de>,
    V: Deserialize<'de>,
{
    let container: Vec<_> = serde::Deserialize::deserialize(des)?;
    Ok(T::from_iter(container))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use std::collections::HashMap;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
    struct TestKey {
        id: u32,
        flag: bool,
    }

    #[test]
    fn test_serialize_empty_map() {
        let map: HashMap<TestKey, i32> = HashMap::new();
        let serialized = serde_json::to_string(&map).unwrap();
        let expected = "[]";
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_round_trip_serialization() {
        let mut original = HashMap::new();
        original.insert(TestKey { id: 10, flag: true }, 999);
        original.insert(
            TestKey {
                id: 20,
                flag: false,
            },
            -50,
        );
        original.insert(TestKey { id: 30, flag: true }, 0);

        let serialized = serde_json::to_string(&original).unwrap();
        let deserialized: HashMap<TestKey, i32> = serde_json::from_str(&serialized).unwrap();

        assert_eq!(original, deserialized);
    }
}
