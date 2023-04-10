// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct RemoveLinkIcon {}

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

impl Component for RemoveLinkIcon {
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
                data-icon={ "RemoveLinkIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M960 384q93 0 174 35t142 96 96 142 35 175q0 93-35 174t-96 142-142 96-174 36h-64v-128h64q66 0 124-25t101-68 69-102 26-125q0-66-25-124t-69-101-102-69-124-26H448q-66 0-124 25t-102 69-69 102-25 124q0 66 25 124t68 102 102 69 125 25h64v128h-64q-93 0-174-35t-142-96-96-142T0 832q0-93 35-174t96-142 142-96 175-36h512zm958 809q-5-63-31-117t-69-94-99-63-119-23h-64V768h64q93 0 174 35t142 96 96 142 36 175q0 47-11 96l-119-119zm-1150 23q0 66 25 124t68 102 102 69 125 25h176l128 128h-304q-93 0-174-35t-142-96-96-142-36-175q0-93 35-174t96-142 142-96 175-36h64v128h-64q-66 0-124 25t-102 69-69 102-25 124zm1277 286l-226 226 226 227-90 90-227-226-227 227-90-91 227-227-227-227 90-90 227 227 227-227 90 91z" />
            </svg>
        }
    }
}
