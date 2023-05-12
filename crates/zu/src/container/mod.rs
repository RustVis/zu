// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::styles::size::SizeVariant;
use yew::{classes, function_component, html, Children, Html, Properties};

#[must_use]
pub const fn max_width_cls(size: Option<SizeVariant>) -> &'static str {
    match size {
        Some(SizeVariant::XSmall) => "ZuContainer-maxWidthXs",
        Some(SizeVariant::Small) => "ZuContainer-maxWidthSm",
        Some(SizeVariant::Middle) => "ZuContainer-maxWidthMd",
        Some(SizeVariant::Large) => "ZuContainer-maxWidthLg",
        Some(SizeVariant::XLarge) => "ZuContainer-maxWidthXl",
        Some(SizeVariant::Num(_)) | None => "",
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: String,

    #[prop_or_default]
    pub component: String,

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
    pub max_width: Option<SizeVariant>,

    #[prop_or_default]
    pub style: String,
}

#[function_component(Container)]
pub fn container(props: &Props) -> Html {
    let component = if props.component.is_empty() {
        "div".to_owned()
    } else {
        props.component.clone()
    };

    let mut cls_list = vec!["ZuContainer-root", max_width_cls(props.max_width)];
    if props.disable_gutters {
        cls_list.push("ZuContainer-disableGutters");
    } else {
        cls_list.push("ZuContainer-enableGutters");
    }
    if props.fixed {
        cls_list.push("ZuContainer-fixed");
    }
    let cls = classes!(cls_list, &props.classes);

    let style = if let Some(SizeVariant::Num(num)) = props.max_width {
        let lst = [props.style.as_str(), &format!("max-width: {num}px")];
        lst.join(";")
    } else {
        props.style.clone()
    };

    html! {
        <@{component} class={cls} style={style}>
            {props.children.clone()}
        </@>
    }
}
