use serde::{Deserialize, Deserializer};

fn deserialize_array<'de, D, T, const N: usize>(deserializer: D) -> Result<[T; N], D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>
{
    let data = Vec::<T>::deserialize(deserializer)?;

    let len = data.len();

    data.into_iter()
        .array_chunks()
        .next()
        .ok_or_else(|| serde::de::Error::invalid_length(len, &stringify!(N)))
}