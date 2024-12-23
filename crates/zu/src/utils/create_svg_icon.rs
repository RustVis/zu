// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use yew::{html, AttrValue, Html};

use crate::svg_icon::SvgIcon;

pub fn create_svg_icon(path: Html, icon: AttrValue) -> Html {
    html! {
        <SvgIcon icon={icon}>
            {path}
        </SvgIcon>
    }
}
