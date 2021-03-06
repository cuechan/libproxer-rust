pub mod info;
pub mod user;
pub mod list;

use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::offset::Utc;
use serde::de::{self, Deserializer, Unexpected, Visitor};
use std::fmt;
use std::marker::PhantomData;

pub type Timestamp = DateTime<Utc>;

pub fn stringly_int<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
	D: Deserializer<'de>,
{
	struct IntVisitor(PhantomData<i64>);

	impl<'a> Visitor<'a> for IntVisitor {
		type Value = i64;

		fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
			formatter.write_str("\"int\"")
		}

		fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
			match v.parse::<i64>() {
				Ok(int) => Ok(int),
				Err(_) => Err(de::Error::invalid_value(Unexpected::Str(v), &self)),
			}
		}
	}

	deserializer.deserialize_any(IntVisitor(PhantomData))
}

pub fn stringly_array_spaces<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
	D: Deserializer<'de>,
{
	struct IntVisitor(PhantomData<Vec<String>>);

	impl<'a> Visitor<'a> for IntVisitor {
		type Value = Vec<String>;

		fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
			formatter.write_str("\"string\"")
		}

		fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
			let mut list: Vec<String> = Vec::new();

			// for item in v.to_string().split_whitespace() {
			// 	list.push(item.to_string());
			// }

			v.to_string()
				.split_whitespace()
				.for_each(|x| list.push(x.to_string()));

			Ok(list)
			// Err(_) => Err(de::Error::invalid_value(Unexpected::Str(v), &self))
		}
	}

	deserializer.deserialize_any(IntVisitor(PhantomData))
}

pub fn parse_timestamp<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
	D: Deserializer<'de>,
{
	struct IntVisitor(PhantomData<DateTime<Utc>>);

	impl<'a> Visitor<'a> for IntVisitor {
		type Value = DateTime<Utc>;

		fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
			formatter.write_str("\"unix-timestamp\"")
		}

		fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
			let fmts = vec!["%s", "%F %T"];

			for fmt in fmts {
				if let Ok(r) = NaiveDateTime::parse_from_str(v, fmt) {
					return Ok(DateTime::from_utc(r, Utc));
				}
			}

			Err(de::Error::custom(format!("can't parse time: {}", v)))
		}

		fn visit_i64<E: de::Error>(self, v: i64) -> Result<Self::Value, E> {
			let time = NaiveDateTime::from_timestamp(v, 0);
			let utc = DateTime::from_utc(time, Utc);

			Ok(utc)
		}

		fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
			self.visit_i64(v as i64)

			// let time = NaiveDateTime::from_timestamp(v, 0);
			// let utc = DateTime::from_utc(time, Utc::east(3600));
			//
			// Ok(utc)
		}

		fn visit_i32<E: de::Error>(self, v: i32) -> Result<Self::Value, E> {
			self.visit_i64(v as i64)

			// let time = NaiveDateTime::from_timestamp(v, 0);
			// let utc = DateTime::from_utc(time, Utc::east(3600));
			//
			// Ok(utc)
		}

		fn visit_u32<E: de::Error>(self, v: u32) -> Result<Self::Value, E> {
			self.visit_i64(v as i64)

			// let time = NaiveDateTime::from_timestamp(v, 0);
			// let utc = DateTime::from_utc(time, Utc::east(3600));
			//
			// Ok(utc)
		}
	}

	deserializer.deserialize_any(IntVisitor(PhantomData))
}

#[derive(Debug, Clone, Deserialize, Hash)]
pub enum Kat {
	Anime,
	Manga,
}

#[derive(Debug, Clone, Deserialize, Hash)]
pub enum Medium {
	#[serde(rename = "animeseries")]
	AnimeSeries,
	#[serde(rename = "movie")]
	Movie,
	#[serde(rename = "ova")]
	OVA,
	#[serde(rename = "hentai")]
	Hentai,
	#[serde(rename = "mangaseries")]
	Manga,
	#[serde(rename = "oneshot")]
	OneShot,
	#[serde(rename = "doujin")]
	Doujin,
	#[serde(rename = "hmanga")]
	HManga,
}

#[derive(Debug, Clone, Deserialize, Hash)]
pub enum State {
	#[serde(rename = "0")]
	PreAiring,
	#[serde(rename = "1")]
	Finished,
	#[serde(rename = "2")]
	Airing,
	#[serde(rename = "3")]
	Cancelled,
	#[serde(rename = "4")]
	NoSub,
}

#[derive(Debug, Clone, Deserialize, Hash)]
pub enum Season {
	#[serde(rename = "0")]
	Unknown,
	#[serde(rename = "1")]
	Winter,
	#[serde(rename = "2")]
	Spring,
	#[serde(rename = "3")]
	Summer,
	#[serde(rename = "4")]
	Autumn,
}

/// representing the `watchstate` of an entry
/// only appears in context with an info and user (e.g. comment, rating)
#[derive(Debug, Clone, Deserialize)]
pub enum WatchState {
	#[serde(rename = "0")]
	Watched,
	#[serde(rename = "1")]
	Watching,
	#[serde(rename = "2")]
	WillWatch,
	#[serde(rename = "3")]
	Cancelled,
}
