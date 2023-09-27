// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct SlackSquare {}

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

impl Component for SlackSquare {
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
                data-icon={ "slack-square" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path fill="#333" d="M880 112H144c-17.7 0-32 14.3-32 32v736c0 17.7 14.3 32 32 32h736c17.7 0 32-14.3 32-32V144c0-17.7-14.3-32-32-32zM529 311.4c0-27.8 22.5-50.4 50.3-50.4 27.8 0 50.3 22.6 50.3 50.4v134.4c0 27.8-22.5 50.4-50.3 50.4-27.8 0-50.3-22.6-50.3-50.4V311.4zM361.5 580.2c0 27.8-22.5 50.4-50.3 50.4-13.3 0-26.1-5.3-35.6-14.8-9.4-9.5-14.7-22.3-14.7-35.6 0-27.8 22.5-50.4 50.3-50.4h50.3v50.4zm134 134.4c0 27.8-22.5 50.4-50.3 50.4-27.8 0-50.3-22.6-50.3-50.4V580.2c0-27.8 22.5-50.4 50.3-50.4 13.3 0 26.1 5.3 35.6 14.8s14.7 22.3 14.7 35.6v134.4zm-50.2-218.4h-134c-27.8 0-50.3-22.6-50.3-50.4 0-27.8 22.5-50.4 50.3-50.4h134c27.8 0 50.3 22.6 50.3 50.4-.1 27.9-22.6 50.4-50.3 50.4zm0-134.4c-13.3 0-26.1-5.3-35.6-14.8S395 324.8 395 311.4c0-27.8 22.5-50.4 50.3-50.4 27.8 0 50.3 22.6 50.3 50.4v50.4h-50.3zm134 403.2c-27.8 0-50.3-22.6-50.3-50.4v-50.4h50.3c27.8 0 50.3 22.6 50.3 50.4 0 27.8-22.5 50.4-50.3 50.4zm134-134.4h-134c-13.3 0-26.1-5.3-35.6-14.8-9.4-9.5-14.7-22.3-14.7-35.6 0-27.8 22.5-50.4 50.3-50.4h134c27.8 0 50.3 22.6 50.3 50.4 0 27.8-22.5 50.4-50.3 50.4zm0-134.4H663v-50.4c0-27.8 22.5-50.4 50.3-50.4s50.3 22.6 50.3 50.4c0 27.8-22.5 50.4-50.3 50.4z"/>
            </svg>
        }
    }
}
