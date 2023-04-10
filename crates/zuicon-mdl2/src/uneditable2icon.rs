// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Uneditable2icon {}

#[derive(Properties, Debug, Clone, PartialEq, Eq)]
pub struct Props {
    #[prop_or_default]
    pub class: Option<&'static str>,

    #[prop_or_default]
    pub width: Option<&'static str>,

    #[prop_or_default]
    pub height: Option<&'static str>,

    #[prop_or_default]
    pub color: Option<&'static str>,

    #[prop_or_default]
    pub fill: Option<&'static str>,

    #[prop_or_default]
    pub spin: bool,

    #[prop_or_default]
    pub rotate: i16,
}

impl Component for Uneditable2icon {
    type Properties = Props;
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        // TODO(Shaohua): Do not generate style attribute if it is empty.
        let mut style = String::new();
        if props.rotate != 0 {
            style += &format!("transform: rotate({}deg);", props.rotate);
        }
        html! {
            <svg
                xmlns={ "http://www.w3.org/2000/svg" }
                class={ props.class.unwrap_or("") }
                width={ props.width.unwrap_or("16") }
                height={ props.height.unwrap_or("16") }
                focusable={ "false" }
                data-icon={ "Uneditable2Icon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M633 1890L0 2048l158-633 584-583L19 109l90-90 1920 1920-90 90-723-722-583 583zm-457-18l329-82q-10-46-32-87t-55-73-73-54-87-33l-82 329zm950-656L832 923l-506 505q52 17 98 45t85 66 66 84 45 99l506-506zm294-113l-91-90 373-373-294-293-372 372-91-90 530-531h1q47-48 108-73t129-25q69 0 130 26t106 72 72 107 27 131q0 66-25 127t-73 110l-530 530zm439-621q29-29 45-67t16-79q0-43-16-81t-45-66-66-44-81-17q-38 0-66 10t-53 29-47 41-47 48l293 293 67-67h1-1z" />
            </svg>
        }
    }
}
