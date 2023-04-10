// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct TennisIcon {}

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

impl Component for TennisIcon {
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
                data-icon={ "TennisIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1024 0q141 0 272 36t245 103 207 160 160 208 103 245 37 272q0 141-36 272t-103 245-160 207-208 160-245 103-272 37q-141 0-272-36t-245-103-207-160-160-208-103-244-37-273q0-141 36-272t103-245 160-207 208-160T751 37t273-37zm0 128q-123 0-237 32t-214 90-182 141-140 181-91 214-32 238q0 65 9 128 122-2 219-49t174-123 131-174 95-203q24-66 54-130t68-125 81-116 98-103q-8-1-16-1t-17 0zM165 1279q32 108 90 204t136 174 174 136 204 90q8-140 62-252t141-199 198-151 233-109q66-24 138-60t139-82 123-103 90-121q-30-120-90-225t-146-190-190-145-225-91q-65 34-121 90t-102 123-83 139-60 138q-45 122-109 233t-151 197-199 141-252 63zm859 641q123 0 237-32t214-90 182-141 140-181 91-214 32-238v-16q0-8-1-17-47 53-102 97t-116 82-125 67-131 55q-105 39-203 94t-174 132-123 173-49 220q63 9 128 9z" />
            </svg>
        }
    }
}
