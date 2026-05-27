use crate::{
    lang::ast::Binding,
    utils::{
        mappings::str_to_keycode,
        types::{
            KeysVector,
            RebindDict,
        },
    },
};

pub fn binding_to_dict(bindings: Vec<Binding>) -> RebindDict {
    let mut dict = RebindDict::new();

    for binding in bindings {
        let original = binding
            .original
            .iter()
            .map(|key| str_to_keycode(key).unwrap())
            .collect::<KeysVector>();

        let remapped = binding
            .remapped
            .iter()
            .map(|key| str_to_keycode(key).unwrap())
            .collect::<KeysVector>();

        dict.insert(original, remapped);
    }

    dict
}
