use std::collections::HashMap;

pub struct FuncSig {
	pub name: Option<String>,
	pub parameters: Option<HashMap<String, String>>,
	pub return_type: Option<String>,
	pub result: Option<String>,
}

pub struct Function {
	signature: FuncSig,
	calls: crate::call::CallList,
}
