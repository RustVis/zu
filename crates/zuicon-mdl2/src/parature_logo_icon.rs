// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct ParatureLogoIcon {}

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

impl Component for ParatureLogoIcon {
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
                data-icon={ "ParatureLogoIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1339 0q94 14 169 55t129 103 83 144 29 176q0 115-38 227t-106 211-158 184-195 146-218 96-227 35l-158 671-296-920q-21-65-38-132t-17-136q0-45 9-86t22-83v-1h1q1-10 9-31t18-45 20-45 16-32q-8 31-12 62t-4 64q0 89 30 160t85 122 128 77 161 27q89 0 179-27t174-77 155-116 125-147 84-167 31-181q0-106-47-194T1339 0zM785 685q-49 0-90-15t-72-43-48-68-18-91q0-77 37-149t98-129 136-90 151-34q49 0 91 15t72 43 48 69 17 91q0 51-17 101t-48 94-70 82-88 65-98 43-101 16z" />
            </svg>
        }
    }
}
