use logos::Logos;

use crate::{
    device::{
        emulated::{
            virtual_dpimouse,
            virtual_keyboard,
        },
        physical::open_device,
    },
    lang::{
        lexer::Token,
        parser::Parser,
    },
    utils::helper::binding_to_dict,
};

mod config;
mod device;
mod lang;
mod rebind;
mod utils;

fn main() {
    let passthrough_inputs = false;

    let keyboard_id = 3;
    let dpimouse_id = 6;

    let virtual_keyboard = virtual_keyboard();
    let virtual_dpimouse = virtual_dpimouse();

    let keyboard = open_device(keyboard_id, !passthrough_inputs);
    let dpimouse = open_device(dpimouse_id, !passthrough_inputs);

    let content = config::load_config_file();

    let tokens = Token::lexer(&content)
        .map(|t| match t {
            Ok(token) => token,
            Err(err) => panic!("{:?}", err),
        })
        .collect::<Vec<Token>>();

    let mut parser = Parser::new(tokens);

    let bindings = parser.parse();

    let remap_dict = binding_to_dict(bindings);

    rebind::start(
        virtual_keyboard,
        virtual_dpimouse,
        keyboard,
        dpimouse,
        remap_dict,
    );
}
