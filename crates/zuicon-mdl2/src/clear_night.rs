// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct ClearNight {}

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

impl Component for ClearNight {
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
                data-icon={ "ClearNight" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M128 1536q141 0 272-36t244-104 207-160 161-207 103-245 37-272q0-133-34-261T1019 8q128 16 245 61t217 115 181 161 138 199 89 229 31 251q0 141-36 272t-104 244-160 207-207 161-245 103-272 37q-138 0-269-36t-246-103-212-164T5 1528q31 4 61 6t62 2zm768 384q124 0 238-32t214-90 181-140 140-181 91-214 32-239q0-136-40-263t-112-236-176-194-229-136q45 155 45 317 0 146-35 282t-100 258-157 225-205 182-244 129-277 68q128 128 290 196t344 68z" />
            </svg>
        }
    }
}
