// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{classes, function_component, html, AttrValue, Html, Properties};
use zu_util::prop::ToAttr;

use crate::styles::color::Color;
use crate::typography::{Typography, Variant};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    /// The action to display in the card header.
    #[prop_or_default]
    pub action: Option<Html>,

    /// The Avatar element to display.
    #[prop_or_default]
    pub avatar: Option<Html>,

    #[prop_or_default]
    pub classes: AttrValue,

    #[prop_or_default]
    pub component: AttrValue,

    /// If true, subheader and title won't be wrapped by a Typography component.
    #[prop_or(false)]
    pub disable_typography: bool,

    /// The content of the component.
    #[prop_or_default]
    pub subheader: Option<Html>,

    #[prop_or_default]
    pub style: AttrValue,

    /// The content of the component.
    #[prop_or_default]
    pub title: Option<Html>,
    // TODO(Shaohua): Add title props.
}

#[function_component(CardHeader)]
pub fn card_header(props: &Props) -> Html {
    let component = if props.component.is_empty() {
        "div"
    } else {
        props.component.as_str()
    };
    let root_cls = classes!("ZuCardHeader-root", props.classes.as_str().to_owned());

    html! {
       <@{component.to_owned()} class={root_cls} style={props.style.to_attr()}>
        if let Some(avatar) = &props.avatar {
            <div class="ZuCardHeader-avatar">
                {avatar.clone()}
            </div>
        }

        <div class="ZuCardHeader-content">
            <Typography classes="ZuCardHeader-title"
                variant={if props.avatar.is_some() {Variant::Body2 } else { Variant::H5 }}
                component="span">
                {props.title.clone()}
            </Typography>

            <Typography classes="ZuCardHeader-subheader"
                variant={if props.avatar.is_some() { Variant::Body2 } else { Variant::Body1 }}
                color={Color::Secondary}
                component="span">
                {props.subheader.clone()}
            </Typography>
        </div>

        if let Some(action) = &props.action {
            <div class="ZuCardHeader-action">
                {action.clone()}
            </div>
        }

        </@>
    }
}
