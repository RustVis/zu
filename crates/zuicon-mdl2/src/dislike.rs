// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Dislike {}

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

impl Component for Dislike {
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
                data-icon={ "Dislike" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M960 256q178 0 345 69 72 29 144 44t151 15h448v896h-417q-65 0-122 24t-104 70l-622 621q-25 25-50 39t-61 14q-33 0-62-12t-51-35-34-51-13-62q0-81 18-154t53-146q20-43 34-87t19-93H192q-39 0-74-15t-61-41-42-61-15-75q0-32 10-61l256-768q20-59 70-95t112-36h512zm960 256h-320q-179 0-345-69-144-59-295-59H448q-20 0-37 12t-24 32q-5 14-18 54t-33 96-42 124-46 137-44 134-39 118-27 86-10 39q0 26 19 45t45 19h576q0 53-2 98t-10 89-22 86-37 91q-28 58-42 118t-15 126q0 14 9 23t23 9q6 0 10-4t9-9l623-624q32-32 68-56t78-41q80-34 171-34h289V512z" />
            </svg>
        }
    }
}
