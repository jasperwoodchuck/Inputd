use crate::{
	lang::ast::Binding,
	types::input::{
		InputMessage,
		InputToken,
		RebindDict,
	},
};

pub fn binding_to_dict(bindings: Vec<Binding>) -> RebindDict {
	bindings
		.into_iter()
		.map(|binding| (binding.hotkey, binding.action))
		.collect()
}

pub fn insert_token(tokens: &mut Vec<InputToken>, input_msg: &InputMessage) {
	if !tokens.contains(&input_msg.token) {
		tokens.push(input_msg.token);
	}
}

pub fn remove_token(tokens: &mut Vec<InputToken>, input_msg: &InputMessage) {
	tokens.retain(|k| *k != input_msg.token);
}
