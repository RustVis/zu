// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct FiletypeWoff {}

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

impl Component for FiletypeWoff {
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
                data-icon={ "filetype-woff" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path fill-rule="evenodd" d="M14 4.5V11h-1V4.5h-2A1.5 1.5 0 0 1 9.5 3V1H4a1 1 0 0 0-1 1v9H2V2a2 2 0 0 1 2-2h5.5L14 4.5Zm-5.464 9.688v-.522c0-.257-.04-.471-.117-.641a.861.861 0 0 0-.323-.387.862.862 0 0 0-.468-.129.868.868 0 0 0-.472.13.868.868 0 0 0-.32.386c-.077.17-.116.384-.116.641v.522c0 .256.039.47.117.641a.866.866 0 0 0 .319.387.883.883 0 0 0 .472.126.877.877 0 0 0 .468-.126.861.861 0 0 0 .323-.386 1.55 1.55 0 0 0 .117-.642Zm.803-.516v.513c0 .375-.069.7-.205.973-.137.271-.333.48-.59.627-.253.144-.559.216-.916.216-.356 0-.662-.072-.92-.216a1.463 1.463 0 0 1-.59-.627 2.151 2.151 0 0 1-.204-.973v-.513c0-.379.068-.704.205-.975.137-.274.333-.483.589-.627.258-.147.564-.22.92-.22.357 0 .663.073.917.22.256.146.452.356.589.63.136.271.205.595.205.972Zm-6.064-.536-.74 2.79h-.73l-1.055-4h.855l.601 2.903h.038l.706-2.903h.683l.706 2.903h.04l.596-2.903h.858l-1.055 4h-.73l-.74-2.79h-.033Zm7.398 2.79v-1.592h1.606v-.638h-1.606v-1.117h1.758v-.653H9.882v4h.791Zm2.988-1.592v1.591h-.791v-3.999h2.548v.653h-1.757v1.117h1.605v.638h-1.605Z"/>
            </svg>
        }
    }
}
