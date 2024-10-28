// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use yew::{function_component, html, Children, Html, Properties};

use crate::button::Button;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Code)]
pub fn code(props: &Props) -> Html {
    // TODO(Shaohua): Copy text to clipboard on copy button clicked.

    html! {
        <div class="ZuCode-root">
            <pre class="ZuCode-pre">
                <code class="ZuCode-code">
                {for props.children.iter()}
                </code>
            </pre>
            <Button classes="ZuCode-copy">
            {"Copy"}
            </Button>
        </div>
    }
}
