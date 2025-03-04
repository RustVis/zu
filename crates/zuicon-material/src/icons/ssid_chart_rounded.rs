// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SsidChartRounded)]
pub fn ssid_chart_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SsidChartRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M3,9.03c0-0.32,0.15-0.62,0.41-0.81L7.14,5.5c0.4-0.29,0.95-0.25,1.3,0.1l3.78,3.78l7.2-5.23C20.07,3.67,21,4.14,21,4.96 c0,0.32-0.15,0.62-0.41,0.81l-7.9,5.73c-0.4,0.29-0.95,0.25-1.29-0.1L7.62,7.62L4.59,9.84C3.93,10.32,3,9.85,3,9.03z M21,16 c0-0.55-0.45-1-1-1h-3.35c-0.23,0-0.45,0.08-0.62,0.22l-3.9,3.12L6.6,12.99c-0.35-0.34-0.88-0.38-1.27-0.1l-1.9,1.35 C3.16,14.43,3,14.74,3,15.06c0,0.81,0.92,1.29,1.58,0.81L5.8,15l5.57,5.39c0.36,0.35,0.93,0.38,1.32,0.06L17,17h3 C20.55,17,21,16.55,21,16z"/>
        </SvgIcon>
    }
}
