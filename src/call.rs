
pub enum CallType {
	Function,
	Result,
	Operation
}

pub struct Call {
	call: String,
	call_type: CallType
}