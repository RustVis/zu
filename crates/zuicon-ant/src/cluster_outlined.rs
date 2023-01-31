// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct ClusterOutlined {}

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

impl Component for ClusterOutlined {
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
                data-icon={ "cluster" }
                viewBox={ "0 0 1024 1024" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M888 680h-54V540H546v-92h238c8.8 0 16-7.2 16-16V168c0-8.8-7.2-16-16-16H240c-8.8 0-16 7.2-16 16v264c0 8.8 7.2 16 16 16h238v92H190v140h-54c-4.4 0-8 3.6-8 8v176c0 4.4 3.6 8 8 8h176c4.4 0 8-3.6 8-8V688c0-4.4-3.6-8-8-8h-54v-72h220v72h-54c-4.4 0-8 3.6-8 8v176c0 4.4 3.6 8 8 8h176c4.4 0 8-3.6 8-8V688c0-4.4-3.6-8-8-8h-54v-72h220v72h-54c-4.4 0-8 3.6-8 8v176c0 4.4 3.6 8 8 8h176c4.4 0 8-3.6 8-8V688c0-4.4-3.6-8-8-8zM256 805.3c0 1.5-1.2 2.7-2.7 2.7h-58.7c-1.5 0-2.7-1.2-2.7-2.7v-58.7c0-1.5 1.2-2.7 2.7-2.7h58.7c1.5 0 2.7 1.2 2.7 2.7v58.7zm288 0c0 1.5-1.2 2.7-2.7 2.7h-58.7c-1.5 0-2.7-1.2-2.7-2.7v-58.7c0-1.5 1.2-2.7 2.7-2.7h58.7c1.5 0 2.7 1.2 2.7 2.7v58.7zM288 384V216h448v168H288zm544 421.3c0 1.5-1.2 2.7-2.7 2.7h-58.7c-1.5 0-2.7-1.2-2.7-2.7v-58.7c0-1.5 1.2-2.7 2.7-2.7h58.7c1.5 0 2.7 1.2 2.7 2.7v58.7zM360 300a40 40 0 1 0 80 0 40 40 0 1 0-80 0z"/>
            </svg>
        }
    }
}
