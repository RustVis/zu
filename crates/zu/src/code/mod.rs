// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

use yew::{Children, function_component, Html, html, Properties};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Code)]
pub fn code(props: &Props) -> Html {
    html!{
        <div class="ZuCode-root">
            <pre class="ZuCode-pre">
                <code class="ZuCode-code">
                {for props.children.iter()}
                </code>
            </pre>
        </div>
    }
}