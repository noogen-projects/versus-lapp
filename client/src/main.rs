use std::cell::Cell;
use std::collections::VecDeque;

use gloo_net::http::Request;
use leptos::ev::SubmitEvent;
use leptos::html::Input;
use leptos::{
    component, create_action, create_effect, create_node_ref, create_signal, logging, view, For, IntoView, NodeRef,
    SignalGet, SignalUpdate,
};

#[component]
fn App() -> impl IntoView {
    let (responses, set_responses) = create_signal(VecDeque::new());

    // Load the response when a request message spawns
    let send_request = create_action(|msg: &String| {
        let url = format!("/versus/{msg}");
        let request = Request::get(&url);
        logging::log!("Request: {url}");

        async move {
            let text = match request.send().await {
                Ok(response) => response
                    .text()
                    .await
                    .unwrap_or_else(|err| format!("Response error: {err}")),
                Err(err) => format!("Request error: {err}"),
            };

            logging::log!("Response: {text}");
            text
        }
    });
    let load_response = send_request.value();

    // Update the responses list when we get a response
    let id_counter = Cell::new(1_usize);
    create_effect(move |_| {
        if let Some(response) = load_response.get() {
            let id = id_counter.replace(id_counter.get() + 1);

            set_responses.update(move |responses| {
                responses.push_front((id, create_signal(response)));
            });
        }
    });

    // Spawn a request message when the form is submitted
    let input_element: NodeRef<Input> = create_node_ref();
    let on_send = move |ev: SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        let input = input_element.get().expect("<input> must exist");
        send_request.dispatch(input.value());
    };

    view! {
        <label>
            Material 3
            <md-checkbox checked></md-checkbox>
        </label>

        <md-outlined-button>Back</md-outlined-button>
        <md-filled-button>Next</md-filled-button>

        <div>
            <h1>"versus"</h1>
            <form on:submit = on_send>
                <input type = "text"
                    placeholder = "Message"
                    node_ref = input_element
                />
                <input type = "submit" value = "Send"/>
            </form>
        </div>

        <For
            each = move || { responses.get() }
            key = |(id, _)| *id
            children = move |(_, (message, _))| {
                view! {
                    <p>{ message }</p>
                }
            }
        />
    }
}

fn main() {
    console_error_panic_hook::set_once();

    leptos::mount_to_body(App)
}
