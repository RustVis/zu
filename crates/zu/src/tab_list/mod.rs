// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{function_component, html, Children, Html, Properties};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

#[function_component(TabList)]
pub fn tab_list(_props: &Props) -> Html {
    html! {
      <>
      </>
    }
}
