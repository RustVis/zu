// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{html, Children, Component, Context, Html, Properties};

use crate::components::config_provider::size_context::SizeType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonType {
    Default,
    Primary,
    Ghost,
    Dashed,
    Link,
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

#[derive(Properties, Clone, PartialEq)]
pub struct BaseButtonProps {
    #[prop_or_default]
    type_: Option<ButtonType>,

    //icon: Option<>,
    #[prop_or(ButtonShape::Default)]
    shape: ButtonShape,

    #[prop_or(SizeType::Middle)]
    size: SizeType,

    #[prop_or_default]
    loading: Option<ButtonLoading>,

    #[prop_or_default]
    prefix_class: Option<String>,

    #[prop_or_default]
    class_name: Option<String>,

    #[prop_or_default]
    ghost: Option<bool>,

    #[prop_or_default]
    danger: Option<bool>,

    #[prop_or_default]
    block: Option<bool>,

    pub children: Children,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Msg {}

struct Button {}

impl Button {}

impl Component for Button {
    type Message = Msg;
    type Properties = BaseButtonProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let kids: Vec<_> = ctx
            .props()
            .children
            .iter()
            .map(|item| html! { item })
            .collect();

        //{ for ctx.props().children.iter() }
        html! {
            <button>
                { for kids }
            </button>
        }
    }
}
