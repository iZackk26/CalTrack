#![allow(dead_code, non_snake_case)]
use dioxus::prelude::*;

struct Package {
    tracking: String,
    order_numer: i32,
    platform: String,
    adress: String,
    user: User,
}
#[derive(Debug)]
struct User {
    name: String,
    mail: String,
    id: u32,
    phone: u32,
    password: String,
}

impl User {
    fn change_name(&mut self, name: String) {
        self.name = name
    }
    fn change_mail(&mut self, mail: String) {
        self.mail = mail
    }
    fn change_phone(&mut self, phone: u32) {
        self.phone = phone
    }
}

impl Package {
    fn add_user(&mut self, user: User) {
        self.user = user
    }
    fn new(&mut self, tracking: String, order_numer: i32, platform: String, adress: String) {
        self.tracking = tracking;
        self.order_numer = order_numer;
        self.platform = platform;
        self.adress = adress;
    }
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            justify_content: "center",
            align_items: "center",
            h1 { "CalTrack" }
        },
        div {
            display: "flex",
            flex_direction: "column",

            label { "Product URL: " }
            input {
                placeholder: "https://www.amazon.com",
                id: "url",
                name: "url",
            }
            label { "Order Number: " }
            input {
                placeholder: "1234",
                id: "order",
                name: "order",
            }
            label { "Weight: " }
            input {
                placeholder: "1.2",
                id: "weight",
                name: "weight",
            }
            label { "Price: " }
            input {
                placeholder: "12.34",
                id: "price",
                name: "price",
            }
        },
        div {
            button {
                "Calculate",
            }
        }
    })
}

fn main() {
    dioxus_desktop::launch(App);
}
