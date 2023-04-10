// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

use yew::{Html, html};

pub fn default_svg() -> Html {
    html! {
        <svg class="fui-Spinner__Progressbar">
            <circle class="fui-Spinner__Track" />
            <circle class="fui-Spinner__Tail" />
        </svg>
    }
}