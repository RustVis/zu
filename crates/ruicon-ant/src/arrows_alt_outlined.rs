use yew::prelude::{html, Component, Context, Html, Properties};

pub struct ArrowsAltOutlined {}

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

impl Component for ArrowsAltOutlined {
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
                data-icon={ "arrows-alt" }
                viewBox={ "0 0 1024 1024" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M855 160.1l-189.2 23.5c-6.6.8-9.3 8.8-4.7 13.5l54.7 54.7-153.5 153.5a8.03 8.03 0 0 0 0 11.3l45.1 45.1c3.1 3.1 8.2 3.1 11.3 0l153.6-153.6 54.7 54.7a7.94 7.94 0 0 0 13.5-4.7L863.9 169a7.9 7.9 0 0 0-8.9-8.9zM416.6 562.3a8.03 8.03 0 0 0-11.3 0L251.8 715.9l-54.7-54.7a7.94 7.94 0 0 0-13.5 4.7L160.1 855c-.6 5.2 3.7 9.5 8.9 8.9l189.2-23.5c6.6-.8 9.3-8.8 4.7-13.5l-54.7-54.7 153.6-153.6c3.1-3.1 3.1-8.2 0-11.3l-45.2-45z"/>
            </svg>
        }
    }
}
