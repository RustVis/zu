// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use yew::{function_component, html, Children, Html, Properties};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(TabList)]
pub fn tab_list(_props: &Props) -> Html {
    html! {
      <>
      </>
    }
}
