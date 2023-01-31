use yew::prelude::{html, Component, Context, Html, Properties};

pub struct FileZipTwotone {}

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

impl Component for FileZipTwotone {
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
                data-icon={ "file-zip" }
                viewBox={ "0 0 1024 1024" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path fill="#D9D9D9" d="M344 630h32v2h-32z"/>
  <path fill="#D9D9D9" d="M534 352V136H360v64h64v64h-64v64h64v64h-64v64h64v64h-64v62h64v160H296V520h64v-64h-64v-64h64v-64h-64v-64h64v-64h-64v-64h-64v752h560V394H576a42 42 0 0 1-42-42z"/>
  <path d="M854.6 288.6L639.4 73.4c-6-6-14.1-9.4-22.6-9.4H192c-17.7 0-32 14.3-32 32v832c0 17.7 14.3 32 32 32h640c17.7 0 32-14.3 32-32V311.3c0-8.5-3.4-16.7-9.4-22.7zM602 137.8L790.2 326H602V137.8zM792 888H232V136h64v64h64v-64h174v216a42 42 0 0 0 42 42h216v494z"/>
  <path d="M296 392h64v64h-64zm0-128h64v64h-64zm0 318v160h128V582h-64v-62h-64v62zm48 50v-2h32v64h-32v-62zm16-432h64v64h-64zm0 256h64v64h-64zm0-128h64v64h-64z"/>
            </svg>
        }
    }
}
