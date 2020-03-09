pub type CallList = Vec<CallType>;
pub type TokenList = Vec<String>;

pub enum Operation {
	Add,
	Sub,
	Mult,
	Div,
	Mod,
}

pub enum CallType {
	Return(String),
	Init(String),
	Set(String, TokenList),
	Call(String, Vec<String>),
	Move(String, String),
	Operate(String, String, Operation, String),
}

pub struct Call {
	call: String,
	call_type: CallType,
}
