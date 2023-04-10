// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct CertifiedDatabaseIcon {}

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

impl Component for CertifiedDatabaseIcon {
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
                data-icon={ "CertifiedDatabaseIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M384 1600q0 19 13 35t27 28q46 38 114 63t143 39 148 21 131 6q81 0 161-8t159-28v130q-79 18-159 26t-161 8q-61 0-121-4t-121-15q-38-6-86-17t-100-28-101-40-88-55-63-72-24-89V448q0-49 24-89t62-71 88-55 101-41 100-28 87-17q60-10 120-14t122-5q61 0 121 4t121 15q37 6 86 17t100 28 101 40 88 55 63 72 24 89v320h-128V637q-58 37-130 62t-148 40-153 22-145 7q-68 0-144-6t-153-22-149-41-130-62v963zM960 256q-57 0-130 6t-148 20-143 40-115 63q-14 11-27 27t-13 36q0 19 13 35t27 28q46 38 114 63t143 39 148 21 131 6q57 0 130-6t148-20 143-40 114-63q14-11 27-27t14-36q0-19-13-35t-28-28q-46-38-114-63t-142-39-148-21-131-6zm1088 1024q0 82-34 156t-94 130v457l-256-128-256 128v-457q-60-55-94-129t-34-157q0-79 30-149t82-122 122-83 150-30q79 0 149 30t122 82 83 123 30 149zm-256 362q-64 22-128 22t-128-22v174l128-64 128 64v-174zm-128-106q53 0 99-20t82-55 55-81 20-100q0-53-20-99t-55-82-81-55-100-20q-53 0-99 20t-82 55-55 81-20 100q0 53 20 99t55 82 81 55 100 20z" />
            </svg>
        }
    }
}
