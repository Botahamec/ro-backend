use std::collections::HashMap;

pub struct ResultSig {
	pub name: String,
	pub return_type: Option<String>,
	pub parameters: HashMap<String, String>,
}

pub struct RoResult {
	signature: ResultSig,
	functions: Vec<crate::function::Function>,
}
