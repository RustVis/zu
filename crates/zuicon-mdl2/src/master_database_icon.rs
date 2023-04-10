// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct MasterDatabaseIcon {}

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

impl Component for MasterDatabaseIcon {
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
                data-icon={ "MasterDatabaseIcon" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M960 768q-68 0-144-6t-153-22-149-41-130-62v963q0 23 19 43t48 39 66 32 71 25 64 18 46 10q49 9 98 14t100 8v130q-44-2-108-9t-136-22-142-39-127-59-92-82-35-108V448q0-47 22-86t58-69 83-55 95-41 95-28 84-18q66-12 133-17t134-6q67 0 134 5t133 18q36 7 83 18t95 28 95 41 83 54 59 70 22 86v448h-128V637q-59 37-130 62t-148 40-153 22-145 7zm0-512q-57 0-130 6t-148 20-143 40-115 63q-14 11-27 27t-13 36q0 19 13 35t27 28q46 38 114 63t143 39 148 21 131 6q57 0 130-6t148-20 143-40 114-63q14-11 27-27t14-36q0-19-13-35t-28-28q-46-38-114-63t-142-39-148-21-131-6zm1088 739v1053H1024V999l304 401 210-380 211 380 299-405zm-128 389l-186 253-196-353-194 350-192-254v540h768v-536z" />
            </svg>
        }
    }
}
