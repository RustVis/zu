use yew::prelude::{html, Component, Context, Html, Properties};

pub struct AliwangwangFilled {}

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

impl Component for AliwangwangFilled {
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
                data-icon={ "aliwangwang" }
                viewBox={ "0 0 1024 1024" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M868.2 377.4c-18.9-45.1-46.3-85.6-81.2-120.6a377.26 377.26 0 0 0-120.5-81.2A375.65 375.65 0 0 0 519 145.8c-41.9 0-82.9 6.7-121.9 20C306 123.3 200.8 120 170.6 120c-2.2 0-7.4 0-9.4.2-11.9.4-22.8 6.5-29.2 16.4-6.5 9.9-7.7 22.4-3.4 33.5l64.3 161.6a378.59 378.59 0 0 0-52.8 193.2c0 51.4 10 101 29.8 147.6 18.9 45 46.2 85.6 81.2 120.5 34.7 34.8 75.4 62.1 120.5 81.2C418.3 894 467.9 904 519 904c51.3 0 100.9-10 147.7-29.8 44.9-18.9 85.5-46.3 120.4-81.2 34.7-34.8 62.1-75.4 81.2-120.6a376.5 376.5 0 0 0 29.8-147.6c-.2-51.2-10.1-100.8-29.9-147.4zm-325.2 79c0 20.4-16.6 37.1-37.1 37.1-20.4 0-37.1-16.7-37.1-37.1v-55.1c0-20.4 16.6-37.1 37.1-37.1 20.4 0 37.1 16.6 37.1 37.1v55.1zm175.2 0c0 20.4-16.6 37.1-37.1 37.1S644 476.8 644 456.4v-55.1c0-20.4 16.7-37.1 37.1-37.1 20.4 0 37.1 16.6 37.1 37.1v55.1z"/>
            </svg>
        }
    }
}
