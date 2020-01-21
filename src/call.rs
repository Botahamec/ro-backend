#[derive(PartialEq, Clone)]
pub enum CallType {
	Function,
	Result,
	Operation
}

#[derive(Default, PartialEq, Clone)]
pub struct Call {
	call: String,
	call_type: CallType
}

impl Default for CallType {
	fn default() -> Self {
		CallType::Result
	}
}