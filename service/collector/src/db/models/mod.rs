use serde::Deserialize;
use serde::Serialize;
use std::rc::Rc;

pub mod instance;
pub mod log;
pub mod log_type;
pub mod mesh;
pub mod mesh_instance;
pub mod user;

use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct AsJsonError {
	reason: String,
}

impl AsJsonError {
	fn new(reason: &str) -> Self {
		AsJsonError {
			reason: reason.to_string(),
		}
	}
}

impl fmt::Display for AsJsonError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.reason)
	}
}

impl error::Error for AsJsonError {
	fn description(&self) -> &str {
		self.reason.as_str()
	}
}

pub trait ModelAs<'de> {
	type OutputJson: Serialize + Deserialize<'de> + From<Rc<Self::OutputModel>>;
	type OutputModel;

	fn as_model(&self) -> Rc<Self::OutputModel>;

	fn as_json(&self) -> Result<Self::OutputJson, Box<dyn std::error::Error>> {
		Ok(self.as_model().into())
	}
}