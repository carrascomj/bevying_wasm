//! Send data from the browser (WASM) to bevy's runtime through a channel.
//! The receiver is owned by bevy's runtime as a resource that systems can listen to.
//! An event (from an <input> button) owns the sender, sending the deserialized file through it.
use bevy::prelude::*;
use serde::Deserialize;
use async_std::channel::{unbounded, Receiver, Sender};
use wasm_bindgen_futures::{JsFuture, spawn_local};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;
use web_sys::HtmlInputElement;

/// Data sent from callback through the channel.
#[derive(Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Example { pub field1: [f32; 4], }

fn main() {
    // channel to communicate from WASM frontend to bevy runtime
    let (sender, receiver): (Sender<Example>, Receiver<Example>) = unbounded();

    // create a <input> HTML element
    let document = web_sys::window().unwrap().document().unwrap();
    let target_brows = document
        // if already present in the index.html, it could be simply queried with
        // get_element_by_id
        .create_element("input")
        .unwrap()
        // html elements have to be casted to the concrete type
        .dyn_into::<HtmlInputElement>()
        .unwrap_throw();
    target_brows.set_type("file");
    target_brows.set_name("fileb");
    target_brows.set_id("fileb");
    // it may be useful to set a class to customize the style so that it shows properly
    // over whatever bevy renders
    target_brows.set_class_name("fileb");
    document.body().expect("where's body!").append_child(&target_brows).unwrap();
    let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
        // the sender is owned by a WASM closure, called from the browser
        let s = sender.clone();
        spawn_local(async move {
            console::log_1(&"checking closure".into());
            // select whatever files where loaded on the target (the HtmlInputElement)
            if let Some(Some(file_list)) = event.target().map(|t| {
                t.dyn_ref::<HtmlInputElement>()
                    .expect("target_brows is an <input>")
                    .files()
            }) {
                // text() returns a Promise that needs to be transformed to a Future
                // and then awaited since the browser thread cannot be blocked
                // this is why we are wrapping the closure in spawn_local
                let text = JsFuture::from(file_list.get(0).unwrap().text())
                    .await
                    .unwrap()
                    .as_string()
                    .unwrap();
                if let Ok(example) = serde_json::from_str(&text) {
                    s.send(example).await.unwrap();
                } else {
                    console::warn_1(&"Provided file does not have right shape".into())
                }
            }
        })
    }) as Box<dyn FnMut(_)>);
    console::log_1(&"closure setup done!".into());
    // add a listener that executes the closure whenever the file is loaded
    target_brows.set_onchange(Some(closure.as_ref().unchecked_ref()));

    App::new()
        // the receiver is passed to bevy as NonSend Resoruce
        .insert_resource(ReceiverResource { rx: receiver })
        .add_plugins(DefaultPlugins)
        .add_system(listen_from_javascript)
        .run();
}

/// Wrapper around Receiver, just to derive [`Resource`].
#[derive(Resource)]
struct ReceiverResource<T> {
    rx: async_std::channel::Receiver<T>,
}


/// Function that listens to the channel, receiving data from Javascript.
fn listen_from_javascript(receiver: Res<ReceiverResource<Example>>) {
    if let Ok(example) = receiver.rx.try_recv() {
        web_sys::console::log_1(&format!("Received example!: {example:?}").into())
        // here one could add the deserialized data directly to a `ResMut<Assets<YourAsset>>`.
    }
}
