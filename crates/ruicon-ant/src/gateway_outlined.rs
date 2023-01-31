use yew::prelude::{html, Component, Context, Html, Properties};

pub struct GatewayOutlined {}

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

impl Component for GatewayOutlined {
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
                data-icon={ "gateway" }
                viewBox={ "0 0 1024 1024" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M928 392c8.8 0 16-7.2 16-16V192c0-8.8-7.2-16-16-16H744c-8.8 0-16 7.2-16 16v56H296v-56c0-8.8-7.2-16-16-16H96c-8.8 0-16 7.2-16 16v184c0 8.8 7.2 16 16 16h56v240H96c-8.8 0-16 7.2-16 16v184c0 8.8 7.2 16 16 16h184c8.8 0 16-7.2 16-16v-56h432v56c0 8.8 7.2 16 16 16h184c8.8 0 16-7.2 16-16V648c0-8.8-7.2-16-16-16h-56V392h56zM792 240h88v88h-88v-88zm-648 88v-88h88v88h-88zm88 456h-88v-88h88v88zm648-88v88h-88v-88h88zm-80-64h-56c-8.8 0-16 7.2-16 16v56H296v-56c0-8.8-7.2-16-16-16h-56V392h56c8.8 0 16-7.2 16-16v-56h432v56c0 8.8 7.2 16 16 16h56v240z"/>
            </svg>
        }
    }
}
