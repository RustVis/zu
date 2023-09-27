// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct SaveAs {}

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

impl Component for SaveAs {
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
                data-icon={ "SaveAs" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1848 896q42 0 78 15t64 41 42 63 16 79q0 39-15 76t-43 65l-717 717-377 94 94-377 717-716q29-29 65-43t76-14zm51 249q21-21 21-51 0-31-20-50t-52-20q-14 0-27 4t-23 15l-692 692-34 135 135-34 692-691zM768 1536h128l-128 128H293l-165-165V256q0-27 10-50t27-40 41-28 50-10h1280q27 0 50 10t40 27 28 41 10 50v512l-128 128V256h-128v640H384V256H256v1189l91 91h37v-512h896v128l-128 128v-128H512v384h128v-256h128v256zM512 768h768V256H512v512z" />
            </svg>
        }
    }
}
