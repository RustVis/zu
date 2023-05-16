// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

mod spacing;
mod variant;

use yew::{function_component, html, AttrValue, Children, Html, Properties};

pub use spacing::Spacing;
pub use variant::Variant;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    // TODO(Shaohua): Merge classes.
    #[prop_or_default]
    pub classes: AttrValue,

    /// The component used for the root node.
    #[prop_or_default]
    pub component: AttrValue,

    /// Max avatars to show before +x.
    #[prop_or(5)]
    pub max: i32,

    /// Spacing between avatars.
    #[prop_or_default]
    pub spacing: Spacing,

    /// The total number of avatars. Used for calculating the number of extra avatars.
    ///
    /// Default is children.len()
    #[prop_or(-1)]
    pub total: i32,

    #[prop_or_default]
    pub variant: Variant,
}

#[function_component(AvatarGroup)]
pub fn avatar_group(props: &Props) -> Html {
    #[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
    let children_len = props.children.len() as i32;
    let total_avatars = if props.total < 0 {
        children_len
    } else {
        props.total
    };
    let mut clamped_max = props.max.min(2);
    if total_avatars == clamped_max {
        clamped_max += 1;
    }
    clamped_max = clamped_max.min(total_avatars + 1);
    let max_avatars = children_len.min(clamped_max - 1);
    let _extra_avatars = (total_avatars - clamped_max)
        .max(total_avatars - max_avatars)
        .max(0);

    let component = if props.component.is_empty() {
        "div"
    } else {
        props.component.as_str()
    };

    let root_cls = "ZuAvatarGroup-root";

    // TODO(Shaohua): Include Avatar and update styles.

    html! {
        <@{component.to_owned()} class={root_cls}>
            {for props.children.iter()}
        </@>
    }
}
