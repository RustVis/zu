// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct AddOnlineMeeting {}

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

impl Component for AddOnlineMeeting {
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
                data-icon={ "AddOnlineMeeting" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1024 0q141 0 272 36t245 103 207 160 160 208 103 245 37 272q0 99-19 195t-56 189h-309v-128h218q38-126 38-256 0-133-37-256h-363q8 64 12 127t4 129h-128q0-65-4-128t-13-128H657q-8 64-12 127t-5 129q0 65 4 128t13 128h623v128H679q8 39 23 92t37 110 50 112 65 100 78 71 92 27q30 0 57-11t51-30 44-41 37-46h67v222q-63 17-127 25t-129 9q-141 0-272-36t-245-103-207-160-160-208-103-244-37-273q0-141 36-272t103-245 160-207 208-160T751 37t273-37zM731 177q-85 29-161 75T427 357 308 487t-93 153h333q11-59 27-120t39-121 52-116 65-106zm0 1694q-36-49-65-105t-51-116-39-121-28-121H215q39 81 92 152t120 131 142 105 162 75zm-203-591q-8-64-12-127t-4-129q0-65 4-128t12-128H165q-37 123-37 256t37 256h363zm841-640q-8-39-23-92t-37-110-50-112-65-100-78-71-92-27q-49 0-91 27t-79 71-64 99-51 113-37 110-23 92h690zm464 0q-39-81-92-152t-120-131-142-105-162-75q36 49 65 105t51 116 39 121 28 121h333zm-297 896h384v128h-384v384h-128v-384h-384v-128h384v-384h128v384z" />
            </svg>
        }
    }
}
