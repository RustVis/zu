// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct GlobeIcon {}

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

impl Component for GlobeIcon {
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
                data-icon={ "GlobeIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1024 0q141 0 272 36t245 103 207 160 160 208 103 245 37 272q0 141-36 272t-103 245-160 207-208 160-245 103-272 37q-141 0-272-36t-245-103-207-160-160-208-103-244-37-273q0-141 36-272t103-245 160-207 208-160T751 37t273-37zm809 640q-38-81-92-152t-120-131-143-105-161-75q36 50 65 106t51 115 39 121 28 121h333zm87 384q0-133-37-256h-363q8 64 12 127t4 129q0 65-4 128t-12 128h363q37-123 37-256zm-896 896q49 0 91-27t79-71 64-99 51-113 37-110 23-92H679q8 39 23 92t37 110 50 112 65 100 78 71 92 27zm367-640q8-64 12-127t5-129q0-65-4-128t-13-128H657q-8 64-12 127t-5 129q0 65 4 128t13 128h734zM128 1024q0 133 37 256h363q-8-64-12-127t-4-129q0-65 4-128t12-128H165q-37 123-37 256zm896-896q-49 0-91 27t-79 71-64 99-51 113-37 110-23 92h690q-8-39-23-92t-37-110-50-112-65-100-78-71-92-27zm-293 49q-84 29-161 75T427 357 307 487t-92 153h333q12-60 28-121t38-120 52-116 65-106zM215 1408q38 81 92 152t120 131 143 105 161 75q-36-50-65-106t-51-115-39-121-28-121H215zm1102 463q84-29 161-75t143-105 120-130 92-153h-333q-12 60-28 121t-38 120-52 116-65 106z" />
            </svg>
        }
    }
}
