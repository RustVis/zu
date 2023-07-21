// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

mod size;

use yew::{classes, function_component, html, AttrValue, Children, Classes, Html, Properties};

use crate::styles::size::MaxWidth;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    /// Default value is `div`.
    #[prop_or(AttrValue::from("div"))]
    pub component: AttrValue,

    /// If true, the left and right padding is removed.
    #[prop_or(false)]
    pub disable_gutters: bool,

    /// Set the max-width to match the min-width of the current breakpoint.
    #[prop_or(false)]
    pub fixed: bool,

    /// Determine the max-width of the container.
    ///
    /// The container width grows with the size of the screen.
    /// Set to None to disable max-width.
    #[prop_or_default]
    pub max_width: Option<MaxWidth>,

    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(Container)]
pub fn container(props: &Props) -> Html {
    let root_cls = classes!(
        "ZuContainer-root",
        size::css_class(&props.max_width),
        if props.disable_gutters {
            "ZuContainer-disableGutters"
        } else {
            "ZuContainer-enableGutters"
        },
        if props.fixed { "ZuContainer-fixed" } else { "" },
        props.classes.clone(),
    );

    let style = if let Some(MaxWidth::Str(s)) = &props.max_width {
        let lst = [props.style.as_str(), &format!("max-width: {s}")];
        lst.join(";")
    } else {
        props.style.as_str().to_owned()
    };

    html! {
        <@{props.component.to_string()} class={root_cls} style={style}>
            {for props.children.iter()}
        </@>
    }
}
