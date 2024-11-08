// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FiberPinRounded)]
pub fn fiber_pin_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FiberPinRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20 4H4c-1.11 0-1.99.89-1.99 2L2 18c0 1.11.89 2 2 2h16c1.11 0 2-.89 2-2V6c0-1.11-.89-2-2-2zM9 11.5c0 .83-.67 1.5-1.5 1.5h-2v1.25c0 .41-.34.75-.75.75S4 14.66 4 14.25V10c0-.55.45-1 1-1h2.5c.83 0 1.5.67 1.5 1.5v1zm3.5 2.75c0 .41-.34.75-.75.75s-.75-.34-.75-.75v-4.5c0-.41.34-.75.75-.75s.75.34.75.75v4.5zm7.5-.04c0 .44-.35.79-.79.79-.25 0-.49-.12-.64-.33l-2.31-3.17v2.88c0 .34-.28.62-.62.62h-.01c-.35 0-.63-.28-.63-.62V9.83c0-.46.37-.83.83-.83.27 0 .52.13.67.35l2.25 3.15V9.62c0-.34.28-.62.62-.62h.01c.34 0 .62.28.62.62v4.59zM5.5 10.5h2v1h-2z"/>
        </SvgIcon>
    }
}
