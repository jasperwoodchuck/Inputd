use crate::{
	lang::ast::Binding,
	types::input::RebindDict,
};

pub fn binding_to_dict(bindings: Vec<Binding>) -> RebindDict {
	bindings
		.into_iter()
		.map(|binding| (binding.hotkey, binding.action))
		.collect()
}

pub fn insert_unique<T: PartialEq>(items: &mut Vec<T>, item: T) {
	if !items.contains(&item) {
		items.push(item);
	}
}

pub fn remove_item<T: PartialEq>(items: &mut Vec<T>, item: &T) {
	items.retain(|x| x != item);
}
