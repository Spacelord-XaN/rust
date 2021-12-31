pub struct ValueRange<T> {
    min: Option<T>,
    max: Option<T>
}

impl<T> ValueRange<T> {
pub fn fromTo(min: T, max: T) -> ValueRange<T> {
    ValueRange {
        min: Some(min),
        max: Some(max)
    }
}

pub fn from(min: T) -> ValueRange<T> {
    ValueRange {
        min: Some(min),
        max: None
    }
}

pub fn to(max: T) -> ValueRange<T> {
    ValueRange {
        min: None,
        max: Some(max)
    }
}
}