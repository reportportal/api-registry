/// Represents civil time (or occasionally physical time).
///
/// This type can represent a civil time in one of a few possible ways:
///
///   * When utc_offset is set and time_zone is unset: a civil time on a calendar
///     day with a particular offset from UTC.
///   * When time_zone is set and utc_offset is unset: a civil time on a calendar
///     day in a particular time zone.
///   * When neither time_zone nor utc_offset is set: a civil time on a calendar
///     day in local time.
///
/// The date is relative to the Proleptic Gregorian Calendar.
///
/// If year is 0, the DateTime is considered not to have a specific year. month
/// and day must have valid, non-zero values.
///
/// This type may also be used to represent a physical time if all the date and
/// time fields are set and either case of the `time_offset` oneof is set.
/// Consider using `Timestamp` message for physical time instead. If your use
/// case also would like to store the user's timezone, that can be done in
/// another field.
///
/// This type is more flexible than some applications may want. Make sure to
/// document and validate your application's limitations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DateTime {
    /// Optional. Year of date. Must be from 1 to 9999, or 0 if specifying a
    /// datetime without a year.
    #[prost(int32, tag = "1")]
    pub year: i32,
    /// Required. Month of year. Must be from 1 to 12.
    #[prost(int32, tag = "2")]
    pub month: i32,
    /// Required. Day of month. Must be from 1 to 31 and valid for the year and
    /// month.
    #[prost(int32, tag = "3")]
    pub day: i32,
    /// Required. Hours of day in 24 hour format. Should be from 0 to 23. An API
    /// may choose to allow the value "24:00:00" for scenarios like business
    /// closing time.
    #[prost(int32, tag = "4")]
    pub hours: i32,
    /// Required. Minutes of hour of day. Must be from 0 to 59.
    #[prost(int32, tag = "5")]
    pub minutes: i32,
    /// Required. Seconds of minutes of the time. Must normally be from 0 to 59. An
    /// API may allow the value 60 if it allows leap-seconds.
    #[prost(int32, tag = "6")]
    pub seconds: i32,
    /// Required. Fractions of seconds in nanoseconds. Must be from 0 to
    /// 999,999,999.
    #[prost(int32, tag = "7")]
    pub nanos: i32,
    /// Optional. Specifies either the UTC offset or the time zone of the DateTime.
    /// Choose carefully between them, considering that time zone data may change
    /// in the future (for example, a country modifies their DST start/end dates,
    /// and future DateTimes in the affected range had already been stored).
    /// If omitted, the DateTime is considered to be in local time.
    #[prost(oneof = "date_time::TimeOffset", tags = "8, 9")]
    pub time_offset: ::core::option::Option<date_time::TimeOffset>,
}
/// Nested message and enum types in `DateTime`.
pub mod date_time {
    /// Optional. Specifies either the UTC offset or the time zone of the DateTime.
    /// Choose carefully between them, considering that time zone data may change
    /// in the future (for example, a country modifies their DST start/end dates,
    /// and future DateTimes in the affected range had already been stored).
    /// If omitted, the DateTime is considered to be in local time.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TimeOffset {
        /// UTC offset. Must be whole seconds, between -18 hours and +18 hours.
        /// For example, a UTC offset of -4:00 would be represented as
        /// { seconds: -14400 }.
        #[prost(message, tag = "8")]
        UtcOffset(::prost_types::Duration),
        /// Time zone.
        #[prost(message, tag = "9")]
        TimeZone(super::TimeZone),
    }
}
/// Represents a time zone from the
/// [IANA Time Zone Database](<https://www.iana.org/time-zones>).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeZone {
    /// IANA Time Zone Database time zone, e.g. "America/New_York".
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Optional. IANA Time Zone Database version number, e.g. "2019a".
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
}
