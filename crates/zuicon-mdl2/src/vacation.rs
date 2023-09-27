// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Vacation {}

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

impl Component for Vacation {
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
                data-icon={ "Vacation" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1728 1664q26 0 45 19t19 45q0 26-19 45t-45 19H960q-26 0-45-19t-19-45q0-26 19-45t45-19h768zm-128 256q26 0 45 19t19 45q0 26-19 45t-45 19h-512q-26 0-45-19t-19-45q0-26 19-45t45-19h512zm238-512h210v128H537q-10 64-14 128t-7 127-3 128-1 129H128v-193q0-81 6-160t17-159H0v-128h172q50-245 151-471t250-427q-146-5-268-61T82 289l-45-46 47-44q104-97 228-147T579 1q156 0 286 58t239 168q77 2 149 24t134 60 113 90 87 115 57 135 20 150v64h-64q-132 0-247-56t-199-159q-76 138-191 227t-269 125q-43 99-76 200t-57 206h289q22-84 69-154t112-122 146-79 167-29q87 0 167 28t145 80 113 121 69 155zM1238 380q-6 65-26 132 51 88 134 146t185 74q-10-61-35-115t-63-101-88-80-107-56zm-126 7q-102 8-192 50T759 547 642 704t-61 189q85-7 161-36t139-78 112-112 81-144q14-33 23-67t15-69zM224 247q80 66 177 101t201 35q97 0 187-30t168-88q-80-66-177-101t-201-35q-97 0-187 30t-168 88zm318 775q-8 2-16 2t-16 0h-32q-16 0-32-2v-47q-53 118-89 232t-59 230-32 235-10 248h128q0-121 8-234t27-223 49-219 74-222zm442 386h720q-20-57-56-104t-83-81-104-52-117-19q-61 0-117 18t-103 52-84 81-56 105z" />
            </svg>
        }
    }
}
