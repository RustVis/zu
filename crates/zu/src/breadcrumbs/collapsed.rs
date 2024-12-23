// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use yew::{function_component, html, AttrValue, Html, Properties};

use crate::button_base::ButtonBase;
use crate::internal::svg_icons::MoreHorizontal as MoreHorizontalIcon;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub collapsed_icon: Html,

    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(BreadcrumbsCollapsed)]
pub fn breadcrumbs_collapsed(props: &Props) -> Html {
    // TODO(Shaohua): Check value of focus_ripple.
    html! {
        <li>
            <ButtonBase
                classes="ZuBreadcrumbCollapsed-button"
                style={&props.style}
                focus_ripple={true}>
                <MoreHorizontalIcon>
                    {props.collapsed_icon.clone()}
                </MoreHorizontalIcon>
            </ButtonBase>
        </li>
    }
}
