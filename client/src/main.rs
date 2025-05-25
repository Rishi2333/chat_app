use dioxus::prelude::*;
use futures::{SinkExt, StreamExt};
use gloo_net::websocket::{futures::WebSocket, Message};
use dioxus::prelude::Key; // Corrected import for Key enum

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    rsx!(
        document::Stylesheet { href: asset!("/assets/main.css") }
        Home {}
    )
}

#[component]
fn Home() -> Element {
    let mut message_list = use_signal(|| vec![]);
    let mut message_content = use_signal(|| String::new());
    let mut receiver_ws = use_signal(|| None);

    let mut name = use_signal(|| String::new());
    let mut has_name = use_signal(|| false);

    let chat_client = use_coroutine(move |mut rx:UnboundedReceiver<String>| async move {
        let (mut sender, receiver) = WebSocket::open("ws://localhost:3000/chat").unwrap().split();
        receiver_ws.set(Some(receiver));
        while let Some(msg) = rx.next().await {
            let message = format!("{}:{}", name, msg);
            sender.send(Message::Text(message)).await.unwrap();
        }
    });

    let _ = use_future(move || async move {
        if let Some(mut receiver) = receiver_ws.take() {
            while let Some(msg) = receiver.next().await {
                if let Ok(msg) = msg {
                    match msg {
                        Message::Text(content) => {
                            message_list.write().push(content);
                        },
                        _ => ()
                    }
                }
            }
        }
    });

    rsx!(
        if !has_name() {
            div { class: "chat-container",
                div { class: "chat input-name-container",
                    h1 { class: "chat-title", "BAATEIN KARO" }
                    input {
                        r#type: "text",
                        value: name,
                        placeholder: "Enter Your Name ...",
                        oninput: move |e| name.set(e.value()),
                    }
                    button {
                        onclick: move |_| has_name.set(true),
                        disabled: if name().trim() == "" { true },
                        "Join Chat"
                    }
                }
            }
        } else {
            div { class: "chat-container",
                div { class: "chat",
                    div { class: "message-container",
                        {
                            message_list()
                                .iter()
                                .rev()
                                .map(|item| {
                                    let parts: Vec<&str> = item.splitn(2, ':').collect();
                                    let username = if parts.len() > 0 { parts[0] } else { "" };
                                    let message_text = if parts.len() > 1 { parts[1] } else { item };
                                    rsx! {
                                        div {
                                            class: "message-item-wrapper",
                                            class: if username == name() { "user-message-wrapper" },
                                            // Conditionally render the username only if it's not the current user's message
                                            if username != name() {
                                                p {
                                                    class: "message-username",
                                                    "{username}"
                                                }
                                            }
                                            p {
                                                class: "message-item",
                                                class: if username == name() { "user-message" },
                                                "{message_text}"
                                            }
                                        }
                                    }
                                })
                        }
                    }
                    div { class: "input-area",
                        input {
                            r#type: "text",
                            value: message_content,
                            placeholder: "Type a message...",
                            oninput: move |e| message_content.set(e.value()),
                            onkeydown: move |e| {
                                if e.key() == Key::Enter && !message_content().trim().is_empty() {
                                    chat_client.send(message_content());
                                    message_content.set(String::new());
                                }
                            },
                        }
                        button {
                            onclick: move |_| {
                                if !message_content().trim().is_empty() {
                                    chat_client.send(message_content());
                                    message_content.set(String::new());
                                }
                            },
                            disabled: if message_content().trim() == "" { true },
                            "Send"
                        }
                    }
                }
            }
        }
    )
}