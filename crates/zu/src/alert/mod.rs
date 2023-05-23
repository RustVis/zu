// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

mod severity;
mod variant;

use yew::{
    classes, function_component, html, virtual_dom::VNode, AttrValue, Children, Html, Properties,
};

use crate::paper::Paper;
use crate::styles::{
    severity::{Severity, SeverityColor},
    CssClass,
};

// Re-export
pub use variant::Variant;

pub const DEFAULT_CLOSE: &str = "Close";

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    /// The action to display.
    ///
    ///  It renders after the message, at the end of the alert.
    #[prop_or_default]
    pub action: VNode,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: AttrValue,

    // TODO(Shaohua): Set default value here.
    #[prop_or_default]
    pub close_text: AttrValue,

    #[prop_or_default]
    pub color: SeverityColor,

    /// Override the icon displayed before the children.
    #[prop_or_default]
    pub icon: VNode,

    // TODO(Shaohua): Add onClose callback.
    /// The ARIA role attribute of the element.
    #[prop_or_default]
    pub role: AttrValue,

    /// The severity of the alert.
    #[prop_or_default]
    pub severity: Severity,

    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or_default]
    pub variant: Variant,
}

#[function_component(Alert)]
pub fn alert(props: &Props) -> Html {
    let root_cls = classes! {
        "ZuAlert-root",
        severity::severity_class(props.severity),
        props.variant.css_class(),
    };
    let icon_cls = "ZuAlert-icon";
    let message_cls = "ZuAlert-message";
    let action_cls = "ZuAlert-action";

    html! {
        <Paper classes={root_cls} style={&props.style}>
            <div class={icon_cls}>
            </div>
            <div class={message_cls}>
            </div>
            <div class={action_cls}>
            </div>
        </Paper>
    }
}
