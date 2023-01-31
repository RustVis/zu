use yew::prelude::{html, Component, Context, Html, Properties};

pub struct SlackOutlined {}

#[derive(Properties, Debug, Clone, PartialEq)]
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

impl Component for SlackOutlined {
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
                class={ props.class.unwrap_or("") }
                width={ props.width.unwrap_or("1em") }
                height={ props.height.unwrap_or("1em") }
                focusable={ "false" }
                data-icon={ "slack" }
                viewBox={ "0 0 1024 1024" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path fill="#333" d="M409.4 128c-42.4 0-76.7 34.4-76.7 76.8 0 20.3 8.1 39.9 22.4 54.3 14.4 14.4 33.9 22.5 54.3 22.5h76.7v-76.8c0-42.3-34.3-76.7-76.7-76.8zm0 204.8H204.7c-42.4 0-76.7 34.4-76.7 76.8s34.4 76.8 76.7 76.8h204.6c42.4 0 76.7-34.4 76.7-76.8.1-42.4-34.3-76.8-76.6-76.8zM614 486.4c42.4 0 76.8-34.4 76.7-76.8V204.8c0-42.4-34.3-76.8-76.7-76.8-42.4 0-76.7 34.4-76.7 76.8v204.8c0 42.5 34.3 76.8 76.7 76.8zm281.4-76.8c0-42.4-34.4-76.8-76.7-76.8S742 367.2 742 409.6v76.8h76.7c42.3 0 76.7-34.4 76.7-76.8zm-76.8 128H614c-42.4 0-76.7 34.4-76.7 76.8 0 20.3 8.1 39.9 22.4 54.3 14.4 14.4 33.9 22.5 54.3 22.5h204.6c42.4 0 76.7-34.4 76.7-76.8.1-42.4-34.3-76.7-76.7-76.8zM614 742.4h-76.7v76.8c0 42.4 34.4 76.8 76.7 76.8 42.4 0 76.8-34.4 76.7-76.8.1-42.4-34.3-76.7-76.7-76.8zM409.4 537.6c-42.4 0-76.7 34.4-76.7 76.8v204.8c0 42.4 34.4 76.8 76.7 76.8 42.4 0 76.8-34.4 76.7-76.8V614.4c0-20.3-8.1-39.9-22.4-54.3-14.4-14.4-34-22.5-54.3-22.5zM128 614.4c0 20.3 8.1 39.9 22.4 54.3 14.4 14.4 33.9 22.5 54.3 22.5 42.4 0 76.8-34.4 76.7-76.8v-76.8h-76.7c-42.3 0-76.7 34.4-76.7 76.8z"/>
            </svg>
        }
    }
}
