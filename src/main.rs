#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::LevelFilter;
//use dioxus_elements::textfield::TextField;

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");

    launch(App);
}



fn App() -> Element {
    // Build cool things ✌️
    //let mut sometext = "New".to_string();

    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        img { src: "header.svg", id: "header" }
                div { id: "links",
                    a { href: "https://dioxuslabs.com/learn/0.5/", "📚 Learn Dioxus" }
                    a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                    a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                    a { href: "https://github.com/DioxusLabs/dioxus-std", "⚙️ Dioxus Standard Library" }
                    a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus",
                        "💫 VSCode Extension"
                    }
                    a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }

                    //textfield { value: sometext }
                }
                div {id: "links",
                    
                    button {
                        onclick: move |_| handleButtonClick(),
                        "Click me!"
                    }
            
                }
    }
}

async fn handleButtonClick(){
    // Handle button click event
    println!("Button clicked!");
    log::info!("Button clicked!");
    match get_server_data().await {
        Ok(data) => {
            println!("Server response: {}", data);
            log::info!("Server response: {}", data);
        }
        Err(err) => {
            println!("Server error: {}", err);
            log::error!("Server error: {}", err);
        }
        
    }
    //dioxus_logger .info("Button clicked!");
    //sometext = "Clicked".to_string();
}



#[server(PostServerData)]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    println!("Server received: {}", data);
    Ok(())
}

#[server(GetServerData)]
async fn get_server_data() -> Result<String, ServerFnError> {
    Ok("Hello from the server!".to_string())
}
