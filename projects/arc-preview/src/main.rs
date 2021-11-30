#![recursion_limit = "1024"]

use yew::{
    Component,
    ComponentLink,
    html,
    Html, prelude::*, services::reader::{FileData, ReaderService, ReaderTask}, ShouldRender,
};

use rxnb::cell::NotebookCell;
use rxnb::widgets::{NotebookHeader, NotebookLeftPanel, NotebookTabs};
use rxnb::playground::Playground;
use yew::web_sys::HtmlElement;

mod cell;

pub enum Event {
    Input(String),
    Length(ChangeData),
    Mode(ChangeData),
    Files(ChangeData),
    Loaded(FileData),
}

#[derive(Debug)]
pub struct App {
    link: ComponentLink<Self>,
}

impl Component for App {
    type Message = Event;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }


    fn view(&self) -> Html {
        html! {
        <>
        <NotebookHeader/>
        <main>
            <article class="notebook-body">
                <Playground/>
            </article>
        </main>
        // <footer> </footer>
        </>
        }
    }
}

impl App {}

fn main() {
    yew::start_app::<App>();
}
