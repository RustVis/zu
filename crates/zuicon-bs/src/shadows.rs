// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Shadows {}

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

impl Component for Shadows {
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
                data-icon={ "shadows" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M16 8A8 8 0 1 1 0 8a8 8 0 0 1 16 0Zm-8 7a.5.5 0 0 1 0-1h3.5c.032 0 .063.003.093.009A7.032 7.032 0 0 0 12.9 13H8a.5.5 0 0 1 0-1h5.745c.22-.315.415-.65.581-1H8a.5.5 0 0 1 0-1h6.71a7.03 7.03 0 0 0 .22-1H8a.5.5 0 0 1 0-1h7c0-.34-.024-.673-.07-1H8a.5.5 0 0 1 0-1h6.71a6.949 6.949 0 0 0-.384-1H8a.5.5 0 0 1 0-1h5.745a7.035 7.035 0 0 0-.846-1H8a.5.5 0 0 1 0-1h3.608A7 7 0 1 0 8 15Z"/>
            </svg>
        }
    }
}
