use yew::prelude::{html, Component, Context, Html, Properties};

pub struct GitlabFilled {}

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

impl Component for GitlabFilled {
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
                data-icon={ "gitlab" }
                viewBox={ "0 0 1024 1024" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M910.5 553.2l-109-370.8c-6.8-20.4-23.1-34.1-44.9-34.1s-39.5 12.3-46.3 32.7l-72.2 215.4H386.2L314 181.1c-6.8-20.4-24.5-32.7-46.3-32.7s-39.5 13.6-44.9 34.1L113.9 553.2c-4.1 13.6 1.4 28.6 12.3 36.8l385.4 289 386.7-289c10.8-8.1 16.3-23.1 12.2-36.8z"/>
            </svg>
        }
    }
}
