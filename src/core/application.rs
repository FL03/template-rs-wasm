/*
    Appellation: application <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use yew::{html, Component, Context, Html};
use wasm_bindgen::prelude::*;


pub enum Msg {
    AddOne,
}

pub struct App {
    value: i64,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let bg = "bg-gradient-to-br from-zinc-900 via-zinc-800 to-zinc-900";
        let color = "text-white";
        html! {
            <div class={format!("{bg} {color} flex flex-col m-0 py-3 z-0 min-h-screen min-w-full max-w-screen")}>
                <main class="container mx-auto h-full">
                    <button 
                        class="bg-gradient-to-br from-cyan-300 via-cyan-400 to-cyan-500 flex flex-auto items-center justify-center mx-auto px-3 py-1 rounded text-white" 
                        onclick={ctx.link().callback(|_| Msg::AddOne)}
                    >
                        { "Add" }
                    </button>
                    <p>{ self.value }</p>
                </main>
                
            </div>
        }
    }
}
