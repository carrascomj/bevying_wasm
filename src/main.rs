use bevy::prelude::*;
use serde::Deserialize;
use std::sync::mpsc::{channel, Receiver, Sender};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;
use web_sys::HtmlInputElement;

/// Data sent from callback through the channel.
#[derive(Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Example {
    pub field1: [f32; 4],
}

fn main() {
    // channel to communicate from WASM frontend to bevy runtime
    let (tx, rx): (Sender<Example>, Receiver<Example>) = channel();
    // the sender is owned by a WASM closure, called from the browser
    let document = web_sys::window().unwrap().document().unwrap();
    let upload_button = document
        .get_element_by_id("upload")
        .unwrap_throw()
        .dyn_into::<web_sys::HtmlLabelElement>()
        .unwrap_throw();
    {
        let target_brows = document
            .get_element_by_id("fileb")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap_throw();
        let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
            event.prevent_default();
            if let Some(file_list) = target_brows.files() {
                if let Some(Ok(example)) = file_list.get(0).map(|val| val.into_serde()) {
                    console::log_1(&"Sending!".into());
                    tx.send(example).unwrap();
                }
            }
        }) as Box<dyn FnMut(_)>);
        console::log_1(&"closure setup done!".into());
        upload_button.set_onclick(Some(closure.as_ref().unchecked_ref()));
    };

    App::new()
        // the receiver is passed to bevy as NonSend Resoruce
        .insert_non_send_resource(rx)
        .add_plugins(DefaultPlugins)
        .add_system(listen_from_javascript)
        .run();
}

fn listen_from_javascript(receiver: NonSend<Receiver<Example>>) {
    if let Ok(val) = receiver.try_recv() {
        console::log_1(&"Receiving!".into());
        info!("State received: {:?}", val);
    }
}
