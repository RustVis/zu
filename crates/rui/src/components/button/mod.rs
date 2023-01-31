// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use strum_macros::Display;
use yew::{classes, html, Children, Component, Context, Html, Properties};

use crate::components::config_provider::config_context::default_get_prefix_class;
use crate::components::config_provider::size_context::SizeType;
use crate::util::classes_if;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum ButtonType {
    #[strum(serialize = "default")]
    Default,

    #[strum(serialize = "primary")]
    Primary,

    #[strum(serialize = "ghost")]
    Ghost,

    #[strum(serialize = "dashed")]
    Dashed,

    #[strum(serialize = "link")]
    Link,

    #[strum(serialize = "text")]
    Text,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonShape {
    Default,
    Circle,
    Round,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonLoading {
    Load(bool),
    Delay(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum ButtonHTMLType {
    #[strum(serialize = "button")]
    Button,

    #[strum(serialize = "submit")]
    Submit,

    #[strum(serialize = "reset")]
    Reset,
}

#[derive(Properties, Clone, PartialEq)]
pub struct ButtonProps {
    #[prop_or(ButtonType::Default)]
    type_: ButtonType,

    //icon: Option<>,
    #[prop_or(ButtonShape::Default)]
    shape: ButtonShape,

    #[prop_or(SizeType::Middle)]
    size: SizeType,

    #[prop_or(ButtonLoading::Load(false))]
    loading: ButtonLoading,

    #[prop_or_default]
    prefix_class: Option<String>,

    #[prop_or_default]
    class_name: Option<String>,

    #[prop_or(false)]
    ghost: bool,

    #[prop_or(false)]
    danger: bool,

    #[prop_or(false)]
    block: bool,

    #[prop_or(ButtonHTMLType::Button)]
    html_type: ButtonHTMLType,

    pub children: Children,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Msg {}

pub struct Button {}

impl Button {}

impl Component for Button {
    type Message = Msg;
    type Properties = ButtonProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let type_ = props.type_.to_string();
        let html_type = props.html_type.to_string();
        let prefix_class = default_get_prefix_class("btn", props.prefix_class.as_deref());
        // TODO(Shaohua): Read direction from global context.
        let direction = "";
        let block = props.block;
        let danger = props.danger;
        let size_class = props.size.to_class();
        let shape = "";
        let inner_loading = false;

        let class_names = classes!(
            &prefix_class,
            classes_if(vec![
                (format!("{}-{}", &prefix_class, &type_), !type_.is_empty()),
                (
                    format!("{}-{}", &prefix_class, &shape),
                    shape != "default" && !shape.is_empty()
                ),
                (
                    format!("{}-{}", &prefix_class, &size_class),
                    !size_class.is_empty(),
                ),
                //(format!("{}-icon-only", prefix_class), !children && children !== 0 && !!iconType,),
                (format!("{}-loading", &prefix_class), inner_loading,),
                (format!("{}-block", &prefix_class), block,),
                (format!("{}-dangerous", &prefix_class), danger,),
                (format!("{}-rtl", &prefix_class), direction == "rtl",),
            ]),
            &props.class_name,
        );

        log::debug!("button class name: {:?}", class_names);

        let kids: Vec<_> = ctx
            .props()
            .children
            .iter()
            .map(|item| html! { item })
            .collect();
        log::debug!("kids: {:?}", kids);

        html! {
            <button
                class={class_names}
                type={html_type}
                >
                { for kids }
            </button>
        }
    }
}
