use yew::prelude::{html, Component, Context, Html, Properties};

pub struct ApartmentOutlined {}

#[derive(Properties, Debug, Clone, PartialEq)]
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

impl Component for ApartmentOutlined {
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
                class={ props.class.unwrap_or("") }
                width={ props.width.unwrap_or("1em") }
                height={ props.height.unwrap_or("1em") }
                focusable={ "false" }
                data-icon={ "apartment" }
                viewBox={ "0 0 1024 1024" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M908 640H804V488c0-4.4-3.6-8-8-8H548v-96h108c8.8 0 16-7.2 16-16V80c0-8.8-7.2-16-16-16H368c-8.8 0-16 7.2-16 16v288c0 8.8 7.2 16 16 16h108v96H228c-4.4 0-8 3.6-8 8v152H116c-8.8 0-16 7.2-16 16v288c0 8.8 7.2 16 16 16h288c8.8 0 16-7.2 16-16V656c0-8.8-7.2-16-16-16H292v-88h440v88H620c-8.8 0-16 7.2-16 16v288c0 8.8 7.2 16 16 16h288c8.8 0 16-7.2 16-16V656c0-8.8-7.2-16-16-16zm-564 76v168H176V716h168zm84-408V140h168v168H428zm420 576H680V716h168v168z"/>
            </svg>
        }
    }
}
