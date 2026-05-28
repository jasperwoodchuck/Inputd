use crate::{
    lang::ast::Binding,
    utils::{
        mappings::str_to_keycode,
        types::{
            InputMessage,
            InputToken,
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

pub fn insert_token(pressed: &mut KeysVector, input_msg: &InputMessage) {
    if !pressed.contains(&input_msg.token) {
        pressed.push(input_msg.token);
    }
}

pub fn remove_token(pressed: &mut KeysVector, input_msg: &InputMessage) {
    pressed.retain(|k| *k != input_msg.token);
}

pub fn is_modifier(token: InputToken) -> bool {
    matches!(
        token,
        InputToken::Key(evdev::KeyCode::KEY_LEFTMETA)
            | InputToken::Key(evdev::KeyCode::KEY_LEFTALT)
            | InputToken::Key(evdev::KeyCode::KEY_RIGHTALT)
            | InputToken::Key(evdev::KeyCode::KEY_LEFTSHIFT)
            | InputToken::Key(evdev::KeyCode::KEY_RIGHTSHIFT)
            | InputToken::Key(evdev::KeyCode::KEY_LEFTCTRL)
            | InputToken::Key(evdev::KeyCode::KEY_RIGHTCTRL)
    )
}
