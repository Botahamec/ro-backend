/*
 * Copyright (c) 2019 Mike White
 *
 * @Script: result.rs
 * @Author: Mike White
 * @Email: botahamec@outlook.com
 * @Create At: 2019-11-09 20:48:47
 * @Last Modified By: Mike White
 * @Last Modified At: 2019-11-09 21:58:20
 * @Description: Contains the result type used by Ro
 */

use crate::rotype::RoType;
use crate::operation::RoOperation;
use std::collections::HashMap;

enum ResultCall {
	Operation(RoOperation),
	Result
}

pub struct RoResult {
	name: String,
	parameters: HashMap<String, RoType>,
	return_type: RoType,
	call: ResultCall,
	optimized: bool,
	skippable: bool,
	compilable: bool
}