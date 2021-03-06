use crate::db::models::user::User;
use rocket_contrib::json::{Json, JsonValue};

use crate::db::models::instance::{Instance, InstanceJson, InstanceModel};

use crate::db::DatabaseConnection;
use diesel::prelude::*;

use rocket::Route;

use crate::db::models::schema::instance::dsl as instance_dsl;
use crate::db::models::ModelAs;

use crate::error::{ErrorKind, Result};

#[get("/", format = "application/json")]
pub fn list(conn: DatabaseConnection, user: User) -> Result<JsonValue> {
	let list = instance_dsl::instance
		.filter(instance_dsl::user_id.eq(user.get_id()))
		.get_results::<InstanceModel>(&*conn)?
		.into_iter()
		.map(std::convert::Into::into)
		.collect::<Vec<InstanceJson>>();

	Ok(json!({
	"status": true,
	"list": list,
	"count": list.len()
	}))
}

#[get("/<id>", format = "application/json")]
pub fn get(conn: DatabaseConnection, user: User, id: i32) -> Result<JsonValue> {
	let item: InstanceJson = instance_dsl::instance
		.filter(instance_dsl::id.eq(id))
		.filter(instance_dsl::user_id.eq(user.get_id()))
		.first::<InstanceModel>(&*conn)?
		.into();

	Ok(json!({
		"status": true,
		"item": item
	}))
}

#[post("/", format = "application/json", data = "<create_request>")]
pub fn create(
	conn: DatabaseConnection,
	user: User,
	create_request: Json<InstanceJson>,
) -> Result<JsonValue> {
	let mut create_request: InstanceJson = create_request.into_inner();

	create_request.user_id.get_or_insert(user.get_id());

	let item: InstanceJson = diesel::insert_into(instance_dsl::instance)
		.values(create_request)
		.get_result::<InstanceModel>(&*conn)?
		.into();

	Ok(json!({
		"status": true,
		"item": item
	}))
}

#[put("/<id>", format = "application/json", data = "<update_request>")]
pub fn update(
	conn: DatabaseConnection,
	user: User,
	update_request: Json<InstanceJson>,
	id: i32,
) -> Result<JsonValue> {
	let update_request: &InstanceJson = &update_request.into_inner();

	let instance_model = Instance::new(&conn, id)?.as_model();

	if instance_model.user_id != user.get_id() {
		return Err(ErrorKind::AccessDenied.into());
	}

	let item: InstanceJson = diesel::update(instance_model.as_ref())
		.set(update_request)
		.get_result::<InstanceModel>(&*conn)?
		.into();

	Ok(json!({
		"status": true,
		"item": item
	}))
}

#[delete("/<id>")]
pub fn delete(conn: DatabaseConnection, user: User, id: i32) -> Result<JsonValue> {
	let instance_model = Instance::new(&conn, id)?.as_model();

	if instance_model.user_id != user.get_id() {
		return Err(ErrorKind::AccessDenied.into());
	}

	diesel::delete(instance_model.as_ref()).execute(&*conn)?;

	Ok(json!({
		"status": true
	}))
}

#[get("/generate-uuid", format = "application/json")]
pub fn generate_uuid(_user: User) -> Result<JsonValue> {
	Ok(json!({
		"status": true,
		"uuid": uuid::Uuid::new_v4().to_hyphenated().to_string()
	}))
}

pub fn get_routes() -> Vec<Route> {
	routes![list, get, create, update, delete, generate_uuid]
}
