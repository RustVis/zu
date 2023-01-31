use yew::prelude::{html, Component, Context, Html, Properties};

pub struct HomeTwotone {}

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

impl Component for HomeTwotone {
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
                data-icon={ "home" }
                viewBox={ "0 0 1024 1024" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path fill="#D9D9D9" d="M512.1 172.6l-370 369.7h96V868H392V640c0-22.1 17.9-40 40-40h160c22.1 0 40 17.9 40 40v228h153.9V542.3H882L535.2 195.7l-23.1-23.1zm434.5 422.9c-6 6-13.1 10.8-20.8 13.9 7.7-3.2 14.8-7.9 20.8-13.9zm-887-34.7c5 30.3 31.4 53.5 63.1 53.5h.9c-31.9 0-58.9-23-64-53.5zm-.9-10.5v-1.9 1.9zm.1-2.6c.1-3.1.5-6.1 1-9.1-.6 2.9-.9 6-1 9.1z"/>
  <path d="M951 510c0-.1-.1-.1-.1-.2l-1.8-2.1c-.1-.1-.2-.3-.4-.4-.7-.8-1.5-1.6-2.2-2.4L560.1 118.8l-25.9-25.9a31.5 31.5 0 0 0-44.4 0L77.5 505a63.6 63.6 0 0 0-16 26.6l-.6 2.1-.3 1.1-.3 1.2c-.2.7-.3 1.4-.4 2.1 0 .1 0 .3-.1.4-.6 3-.9 6-1 9.1v3.3c0 .5 0 1 .1 1.5 0 .5 0 .9.1 1.4 0 .5.1 1 .1 1.5 0 .6.1 1.2.2 1.8 0 .3.1.6.1.9l.3 2.5v.1c5.1 30.5 32.2 53.5 64 53.5h42.5V940h691.7V614.3h43.4c8.6 0 16.9-1.7 24.5-4.9s14.7-7.9 20.8-13.9a63.6 63.6 0 0 0 18.7-45.3c0-14.7-5-28.8-14.3-40.2zM568 868H456V664h112v204zm217.9-325.7V868H632V640c0-22.1-17.9-40-40-40H432c-22.1 0-40 17.9-40 40v228H238.1V542.3h-96l370-369.7 23.1 23.1L882 542.3h-96.1z"/>
            </svg>
        }
    }
}
