use std::{cell::Cell, collections::VecDeque};

use gloo_net::http::Request;
use leptos::{
    component, create_effect, create_node_ref, create_resource, create_signal, ev::SubmitEvent,
    html::Input, logging, view, For, IntoView, NodeRef, SignalGet, SignalSet, SignalUpdate,
};

#[component]
fn App() -> impl IntoView {
    let (request, set_request) = create_signal(None);
    let (responses, set_responses) = create_signal(VecDeque::new());

    // Load the response when a request message spawns
    let load_response = create_resource(
        move || request.get(),
        |maybe_msg| async move {
            if let Some(msg) = maybe_msg {
                let response_result = Request::get(&format!("/versus/{msg}")).send().await;
                let text = match response_result {
                    Ok(response) => response
                        .text()
                        .await
                        .unwrap_or_else(|err| format!("Response error: {err}")),
                    Err(err) => format!("Request error: {err}"),
                };

                logging::log!("Response: {text}");
                Some(text)
            } else {
                None
            }
        },
    );

    // Update the responses list when we get a response
    let id_counter = Cell::new(1_usize);
    create_effect(move |_| {
        let maybe_response = load_response.get();
        if let Some(Some(response)) = maybe_response {
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

        let value = input_element.get().expect("<input> must exist").value();
        set_request.set(Some(value));
    };

    view! {
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

    leptos::mount_to_body(|| view! { <App/> })
}
