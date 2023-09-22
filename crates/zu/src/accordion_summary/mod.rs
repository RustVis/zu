// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use yew::{classes, function_component, html, AttrValue, Children, Classes, Html, Properties};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or(false)]
    pub disabled: bool,

    #[prop_or(false)]
    pub disable_gutters: bool,

    // TODO(Shaohua): Remove expanded property.
    #[prop_or(false)]
    pub expanded: bool,

    /// The icon to display as the expand indicator.
    #[prop_or_default]
    pub expand_icon: Option<Html>,

    // pub focus_visible_class_name: AttrValue,
    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(AccordionSummary)]
pub fn accordion_summary(props: &Props) -> Html {
    let root_cls = classes!(
        "ZuAccordionSummary-root",
        if props.expanded {
            "ZuAccordionSummary-expanded"
        } else {
            ""
        },
        if props.disabled {
            "ZuAccordionSummary-disabled"
        } else {
            ""
        },
        if props.disable_gutters {
            ""
        } else {
            "ZuAccordionSummary-gutters"
        },
        props.classes.clone(),
    );
    // let focus_visible_cls = "ZuAccordion-focusVisible";
    let content_cls = classes!(
        "ZuAccordionSummary-content",
        if props.expanded {
            "ZuAccordionSummary-expanded"
        } else {
            ""
        },
        if props.disable_gutters {
            ""
        } else {
            "ZuAccordionSummary-gutters"
        }
    );
    let expand_icon_cls = classes!(
        "ZuAccordionSummary-expandIconWrapper",
        if props.expanded {
            "ZuAccordionSummary-expanded"
        } else {
            ""
        },
    );

    // TODO(Shaohua): Replace root div with ButtonBase.
    // TODO(Shaohua): Add onClick callback.

    html! {
        <div class={root_cls}>
            <div class={content_cls}>
                {for props.children.iter()}
            </div>

            if let Some(expand_icon) = &props.expand_icon {
                <div class={expand_icon_cls}>
                    {expand_icon.clone()}
                </div>
            }
        </div>
    }
}
