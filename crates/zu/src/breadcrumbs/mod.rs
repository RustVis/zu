// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

mod collapsed;

use yew::{classes, function_component, html, AttrValue, Children, Classes, Html, Properties};

use crate::typography::Typography;

pub const DEFAULT_EXPAND_TEXT: &str = "Show path";
pub const DEFAULT_ITEMS_AFTER_COLLAPSE: i32 = 1;
pub const DEFAULT_ITEMS_BEFORE_COLLAPSE: i32 = 1;
pub const DEFAULT_MAX_ITEMS: i32 = 8;
pub const DEFAULT_SEPARATOR: &str = "/";

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub aria_label: AttrValue,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    /// Default value is `nav`.
    #[prop_or(AttrValue::from("nav"))]
    pub component: AttrValue,

    /// Override the default label for the expand button.
    #[prop_or_default]
    pub expand_text: AttrValue,

    /// If max items is exceeded, the number of items to show after the ellipsis.
    #[prop_or(DEFAULT_ITEMS_AFTER_COLLAPSE)]
    pub items_after_collapse: i32,

    /// If max items is exceeded, the number of items to show before the ellipsis.
    #[prop_or(DEFAULT_ITEMS_BEFORE_COLLAPSE)]
    pub items_before_collapse: i32,

    /// Specifies the maximum number of breadcrumbs to display.
    #[prop_or(DEFAULT_MAX_ITEMS)]
    pub max_items: i32,

    /// Custom separator node.
    ///
    /// Default is "/".
    #[prop_or_default]
    pub separator: Option<Html>,

    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(Breadcrumbs)]
pub fn breadcrumbs(props: &Props) -> Html {
    let root_cls = classes!("ZuBreadcrumb-root", props.classes.clone(),);

    html! {
        <Typography
            classes={root_cls}
            aria_label={&props.aria_label}
            component={&props.component}
            style={&props.style}>
            <ol class="ZuBreadcrumb-ol">
                {for props.children.iter()}
            </ol>
        </Typography>
    }
}
