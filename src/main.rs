use std::sync::mpsc::{channel, Receiver, Sender};
use bevy::prelude::*;
use serde::Deserialize;
use wasm_bindgen::prelude::*;

fn main() {
    let (tx, rx): (Sender<Example>, Receiver<Example>) = channel();

    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ReceiverWrapper {rx})
        .insert_resource(Example {
            field1: [1., 1., 1., 1.],
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "shu".to_string(),
                fit_canvas_to_parent: true,
                ..default()
            },
            ..default()
        }))
        .add_system(listen_from_javascript)
        .run();
}

#[derive(Resource)]
struct ReceiverWrapper {
    rx: Receiver<Example>,
}

fn listen_from_javascript(receiver: Res<ReceiverWrapper>) {
    if let Ok(val) = receiver.rx.try_recv() {
        info!("State received: {:?}", val);
    }
}

#[derive(Deserialize, Debug, Default, Clone, PartialEq, Resource)]
pub struct Example {
    pub field1: [f32; 4],
}

#[wasm_bindgen]
pub fn send_to_rust(val: &JsValue, tx: RustSender) {
    let example: Example = val.into_serde().unwrap();
    tx.field.send(example);
}

#[wasm_bindgen]
pub struct RustSender {
    field: Sender<Example>,
}
