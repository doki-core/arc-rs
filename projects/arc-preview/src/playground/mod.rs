//! Monaco editor as a [Yew](https://yew.rs) component.
//! Requires the "yew" feature.
use monaco::api::{CodeEditor, CodeEditorOptions, TextModel};
use std::{cell::RefCell, fmt::Write, rc::Rc};
use yew::{html, Component, ComponentLink, Html, NodeRef, Properties, ShouldRender};
use yew::web_sys::HtmlElement;
use monaco::sys::editor::{IModelContentChangedEvent, IDimension};
use yew::services::ConsoleService;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct CodeEditorProps {
    #[prop_or_default]
    pub link: Option<CodeEditorLink>,
    #[prop_or_default]
    pub options: Option<Rc<CodeEditorOptions>>,
}

/// CodeEditor component.
#[derive(Debug)]
pub struct Playground {
    props: CodeEditorProps,
    link: ComponentLink<Self>,
    input: (NodeRef, Option<CodeEditor>),
    output: (NodeRef, Option<CodeEditor>),
}

pub enum Event {
    Edit(IModelContentChangedEvent)
}

impl Component for Playground {
    type Message = Event;
    type Properties = CodeEditorProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        if let Some(editor_link) = &props.link {
            editor_link.connect(link.clone());
        }

        Self {
            props,
            link,
            input: (Default::default(), None),
            output: (Default::default(), None),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Event::Edit(e) => {
                ConsoleService::log(&format!("{:?}", e))
            }
        }
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        todo!("prop changes aren't allowed yet");
    }

    fn view(&self) -> Html {
        html! {
            <div>

            <div ref=self.input.0.clone() />
            <div ref=self.output.0.clone() />
            </div>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        debug_assert!(first_render, "component should never render more than once");

        let opts = CodeEditorOptions {
            dimension: Some(IDimension::new(800, 600)),
            theme: None,
            model: None,
            language: None,
            value: None,
        };

        let el = self
            .input.0
            .cast::<HtmlElement>()
            .expect("failed to resolve editor element");
        let mut model = CodeEditor::create(&el, Some(opts));

        model.on_did_change_model_content(
            |e| {
                ConsoleService::log(&format!("some"));
                ConsoleService::log(&format!("{:?}", e))
            }
        );

        self.input.1 = Some(model);

        let opts = CodeEditorOptions {
            dimension: Some(IDimension::new(800, 600)),
            theme: None,
            model: None,
            language: None,
            value: None,
        };
        let el = self
            .output.0
            .cast::<HtmlElement>()
            .expect("failed to resolve editor element");
        let model = CodeEditor::create(&el, Some(opts));
        self.output.1 = Some(model);
    }
}

/// Link to control a [`CodeEditor`].
#[derive(Clone, Debug, Default)]
pub struct CodeEditorLink(Rc<RefCell<Option<ComponentLink<Playground>>>>);

impl CodeEditorLink {
    fn connect(&self, link: ComponentLink<Playground>) {
        self.0.borrow_mut().replace(link);
    }

    fn with_link<T>(&self, f: impl FnOnce(&ComponentLink<Playground>) -> T) -> Option<T> {
        (*self.0.borrow()).as_ref().map(f)
    }

    fn with_component<T>(&self, f: impl FnOnce(&Playground) -> T) -> Option<T> {
        self.with_link(|link| link.get_component().as_deref().map(f))
            .flatten()
    }

    // fn get_input(&self) -> Option<String> {
    //     self.with_link(|link| link)
    //         .and_then(|cp| cp.get_component().as_deref())
    //         .and_then(|p| p.input.1.as_ref())
    //         .and_then(|e| e.get_model())
    //         .map(|e| e.get_value())
    // }
    //
    // fn set_output(&self, text: &str) -> Option<()> {
    //     self.with_link(|link| link)
    //         .and_then(|cp| cp.get_component().as_deref())
    //         .and_then(|p| p.output.1.as_ref())
    //         .and_then(|e| e.get_model())
    //         .map(|e| e.set_value(text))
    // }

    ///Get access to the underlying [`CodeEditor`].
    ///The return value is `None` if the link isn't connected.
    pub fn with_editor<T>(&self, f: impl FnOnce(&CodeEditor) -> T) -> Option<T> {
        self.with_component(|comp| comp.input.1.as_ref().map(f))
            .flatten()
    }

    fn get_model(&self) -> Option<TextModel> {
        self.with_editor(|editor| editor.get_model()).flatten()
    }

    ///Get the value of the current model.
    pub fn get_value(&self) -> Option<String> {
        self.get_model().map(|model| model.get_value())
    }
}

impl PartialEq for CodeEditorLink {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}
