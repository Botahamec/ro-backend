/*
 * Copyright (c) 2019 Mike White
 *
 * @Script: function.rs
 * @Author: Mike White
 * @Email: botahamec@outlook.com
 * @Create At: 2019-11-09 21:05:31
 * @Last Modified By: Mike White
 * @Last Modified At: 2019-11-14 18:31:47
 * @Description: This is description.
 */

use crate::result::RoResult;

pub struct RoFunction {
	name: String,
	result: String, // the name of the result attached to the function
	execution: RoResult // the result that gets run by the function
}