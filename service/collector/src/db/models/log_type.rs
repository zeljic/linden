use crate::db::models::schema::log_type;
use crate::db::models::ModelAs;
use crate::db::DatabaseConnection;
use std::error;
use std::rc::Rc;

use crate::db::models::schema::log_type::dsl as log_type_dsl;
use crate::db::models::AsJsonError;
use diesel::prelude::*;

use crate::db::models::user::{User, UserModel};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Queryable, Identifiable, Associations, PartialEq, Clone)]
#[table_name = "log_type"]
#[belongs_to(UserModel, foreign_key = "user_id")]
pub struct LogTypeModel {
	pub id: i32,
	pub user_id: i32,
	pub name: String,
	pub description: Option<String>,
	pub enabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, Insertable, AsChangeset)]
#[table_name = "log_type"]
pub struct LogTypeJson {
	pub id: Option<i32>,
	pub user_id: Option<i32>,
	pub name: Option<String>,
	pub description: Option<String>,
	pub enabled: Option<bool>,
}

#[derive(Debug)]
pub struct LogType {
	model: Rc<LogTypeModel>,
}

impl LogType {
	pub fn new(conn: &DatabaseConnection, id: i32) -> Result<Self> {
		if let Ok(model) = log_type_dsl::log_type
			.find(id)
			.first::<LogTypeModel>(&conn.0)
		{
			return Ok(LogType {
				model: Rc::new(model),
			});
		}

		Err(Box::new(AsJsonError::new("Unable to read log_type by id")))
	}

	pub fn is_user_id(&self, user: &User) -> bool {
		self.model.user_id == user.as_model().id
	}

	pub fn from_json(conn: &DatabaseConnection, json: &LogTypeJson) -> Result<Self> {
		match json.id {
			None => Err(Box::new(AsJsonError::new(
				"Unable to read log_type_json id",
			))),
			Some(id) => LogType::new(conn, id),
		}
	}
}

impl<'de> ModelAs<'de> for LogType {
	type OutputJson = LogTypeJson;
	type OutputModel = LogTypeModel;

	fn as_model(&self) -> Rc<Self::OutputModel> {
		Rc::clone(&self.model)
	}
}

impl From<Rc<LogTypeJson>> for LogTypeJson {
	fn from(rc_json: Rc<LogTypeJson>) -> Self {
		rc_json.as_ref().clone()
	}
}

impl From<Rc<LogTypeModel>> for LogTypeJson {
	fn from(rc_model: Rc<LogTypeModel>) -> Self {
		rc_model.as_ref().clone().into()
	}
}

impl From<LogTypeModel> for LogTypeJson {
	fn from(model: LogTypeModel) -> Self {
		Self {
			id: Option::from(model.id),
			user_id: Option::from(model.user_id),
			name: Option::from(model.name),
			description: model.description,
			enabled: Option::from(model.enabled),
		}
	}
}
