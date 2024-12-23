// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(StackedBarChartRounded)]
pub fn stacked_bar_chart_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("StackedBarChartRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M6,20L6,20c1.1,0,2-0.9,2-2V9H4v9C4,19.1,4.9,20,6,20z"/><path d="M4,8h4V6c0-1.1-0.9-2-2-2h0C4.9,4,4,4.9,4,6V8z"/><path d="M10,11h4V9c0-1.1-0.9-2-2-2h0c-1.1,0-2,0.9-2,2V11z"/><path d="M16,12v2h4v-2c0-1.1-0.9-2-2-2h0C16.9,10,16,10.9,16,12z"/><path d="M18,20L18,20c1.1,0,2-0.9,2-2v-3h-4v3C16,19.1,16.9,20,18,20z"/><path d="M12,20L12,20c1.1,0,2-0.9,2-2v-6h-4v6C10,19.1,10.9,20,12,20z"/>
        </SvgIcon>
    }
}
