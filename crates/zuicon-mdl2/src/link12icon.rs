// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Link12icon {}

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

impl Component for Link12icon {
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
                data-icon={ "Link12Icon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1707 715q76 27 139 75t108 111 69 138 25 156q0 106-40 199t-110 162-163 110-199 41h-512q-106 0-199-40t-162-110-110-163-41-199q0-106 40-199t110-162 163-110 199-41h171q0 35-13 66t-37 54-55 36-66 14q-71 0-133 27t-108 73-73 109-27 133q0 71 27 133t73 108 108 73 133 27h512q70 0 132-27t109-73 73-108 27-133q0-92-46-168t-124-123V715zM171 683q0 91 46 167t124 124v189q-76-27-139-75T94 977 25 839 0 683q0-106 40-199t110-162 163-110 199-41h512q106 0 199 40t162 110 110 163 41 199q0 106-40 199t-110 162-163 110-199 41H853q0-35 13-66t37-54 54-37 67-14q70 0 132-27t109-73 73-108 27-133q0-70-26-132t-73-109-109-74-133-27H512q-71 0-133 27t-108 73-73 109-27 133z" />
            </svg>
        }
    }
}
