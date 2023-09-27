// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct RewindEightX {}

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

impl Component for RewindEightX {
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
                data-icon={ "RewindEightX" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1024 507L1664-5v1035l-640-517v517L380 510 1024-5v512zM896 762V261L584 511l312 251zm640 0V261l-312 250 312 251zm-524 1030q0 61-22 108t-61 81-89 50-109 17q-57 0-108-17t-90-50-60-80-23-109q0-38 12-73t33-66 50-53 65-37q-54-30-88-78t-35-112q0-51 20-92t53-69 78-44 93-16q48 0 92 15t78 44 54 70 20 92q0 63-34 111t-88 79q34 14 63 37t51 54 33 65 12 73zm-280-266q30 0 56-11t45-32 31-46 12-56q0-30-11-56t-30-46-46-31-57-12q-32 0-58 11t-45 30-30 46-11 58q0 30 11 56t30 46 46 31 57 12zm174 258q0-38-13-70t-36-56-55-37-70-14q-38 0-70 13t-55 37-37 56-13 71q0 81 47 128t128 48q39 0 71-13t55-36 35-55 13-72zm626-376h113l-215 324 211 316h-119q-38-64-77-127t-77-129h-2q-8 12-14 24t-15 25l-129 207h-118l218-314-208-326h119q38 67 76 133t75 136h2l160-269z" />
            </svg>
        }
    }
}
