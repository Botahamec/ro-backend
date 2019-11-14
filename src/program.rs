/*
 * Copyright (c) 2019 Mike White
 *
 * @Script: program.rs
 * @Author: Mike White
 * @Email: botahamec@outlook.com
 * @Create At: 2019-11-14 18:22:57
 * @Last Modified By: Mike White
 * @Last Modified At: 2019-11-14 18:28:06
 * @Description: Container for all the required objects of a Ro program
 */

use std::collections::HashMap;
use crate::result::RoResult;
use crate::function::RoFunction;
use crate::rotype::RoStruct;

pub struct RoProgram {
	results: HashMap<String, RoResult>,
	functions: HashMap<String, RoFunction>,
	structs: HashMap<String, RoStruct>
}