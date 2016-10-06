extern crate rustc_serialize;
use std::collections::BTreeMap;
use std::borrow::Cow;

#[derive(Debug, RustcEncodable, RustcDecodable, Clone)]
/// Measurement's field value.
pub enum Value {
    /// String.
    String(String),
    /// Floating point number.
    Float(f64),
    /// Integer number.
    Integer(i64),
    /// Boolean value.
    Boolean(bool)
}

/// Measurement model.
#[derive(Debug, RustcEncodable, RustcDecodable, Clone)]
pub struct Measurement<'a> {
    /// Key.
    pub key: Cow<'a, str>,

    /// Timestamp.
    pub timestamp: Option<i32>,

    /// Map of fields.
    pub fields: BTreeMap<String, Value>,

    /// Map of tags.
    pub tags: BTreeMap<String, String>
}

impl<'a> Measurement<'a> {
    /// Constructs a new `Measurement`.
    ///
    /// # Examples
    ///
    /// ```
    /// use influent::measurement::Measurement;
    ///
    /// let measurement = Measurement::new("key");
    /// ```
    pub fn new(key: &str) -> Measurement {
        Measurement {
            key: Cow::Borrowed(key),
            timestamp: None,
            fields: BTreeMap::new(),
            tags: BTreeMap::new()
        }
    }

    /// Adds field to the measurement.
    ///
    /// # Examples
    ///
    /// ```
    /// use influent::measurement::{Measurement, Value};
    ///
    /// let mut measurement = Measurement::new("key");
    ///
    /// measurement.add_field("field", Value::String("hello"));
    /// ```
    pub fn add_field(&mut self, field: &'a str, value: Value) {
        self.fields.insert(field.to_owned(), value);
    }

    /// Adds tag to the measurement.
    ///
    /// # Examples
    ///
    /// ```
    /// use influent::measurement::{Measurement, Value};
    ///
    /// let mut measurement = Measurement::new("key");
    ///
    /// measurement.add_tag("tag", "value");
    /// ```
    pub fn add_tag(&mut self, tag: &'a str, value: &'a str) {
        self.tags.insert(tag.to_owned(), value.to_owned());
    }

    /// Sets the timestamp of the measurement.
    ///
    /// # Examples
    ///
    /// ```
    /// use influent::measurement::{Measurement, Value};
    ///
    /// let mut measurement = Measurement::new("key");
    ///
    /// measurement.set_timestamp(1440924047129)
    /// ```
    pub fn set_timestamp(&mut self, timestamp: i32) {
        self.timestamp = Some(timestamp);
    }
}