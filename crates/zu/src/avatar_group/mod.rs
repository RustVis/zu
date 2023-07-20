// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

mod spacing;

use std::rc::Rc;
use yew::{function_component, html, AttrValue, ChildrenWithProps, Html, Properties};

use crate::avatar::{Avatar, Props as AvatarProps};
use crate::styles::shape_variant::ShapeVariant;
pub use spacing::Spacing;

pub const DEFAULT_MAX: i32 = 5;
const AVATAR_CLS: &str = "ZuAvatarGroup-avatar";

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: ChildrenWithProps<Avatar>,

    // TODO(Shaohua): Merge classes.
    #[prop_or_default]
    pub classes: AttrValue,

    /// The component used for the root node.
    #[prop_or_default]
    pub component: AttrValue,

    /// Max avatars to show before +x.
    #[prop_or(DEFAULT_MAX)]
    pub max: i32,

    /// Spacing between avatars.
    #[prop_or_default]
    pub spacing: Spacing,

    // TODO(Shaohua): Add style property.
    /// The total number of avatars. Used for calculating the number of extra avatars.
    ///
    /// Default is children.len()
    #[prop_or(-1)]
    pub total: i32,

    #[prop_or_default]
    pub variant: ShapeVariant,
}

#[function_component(AvatarGroup)]
pub fn avatar_group(props: &Props) -> Html {
    #[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
    let children_len = props.children.len() as i32;
    let total_avatars = if props.total <= 0 {
        children_len
    } else {
        props.total
    };
    let mut clamped_max = props.max.max(2);
    if total_avatars == clamped_max {
        clamped_max += 1;
    }
    clamped_max = clamped_max.min(total_avatars + 1);
    let max_avatars = children_len.min(clamped_max - 1);
    let extra_avatars = (total_avatars - clamped_max)
        .max(total_avatars - max_avatars)
        .max(0);

    let margin_left = props.spacing.space();

    let component = if props.component.is_empty() {
        "div"
    } else {
        props.component.as_str()
    };

    let root_cls = "ZuAvatarGroup-root";
    let avatar_style = margin_left.map_or_else(String::new, |margin_left| {
        format!("margin-left: {margin_left}px;")
    });
    #[allow(clippy::cast_sign_loss)]
    let max_avatars = max_avatars as usize;

    let avatar_style: AttrValue = avatar_style.into();
    // Merge styles and classes to avatars.
    let mut children: Vec<_> = props
        .children
        .iter()
        .map(|mut item| {
            let props = Rc::<AvatarProps>::make_mut(&mut item.props);
            if props.classes.is_empty() {
                props.classes = AttrValue::from(AVATAR_CLS);
            } else {
                props.classes = format!("{} {AVATAR_CLS}", props.classes).into();
            }
            if props.style.is_empty() {
                props.style = avatar_style.clone();
            } else {
                props.style = format!("{}; {}", props.style, avatar_style).into();
            }
            item
        })
        .collect();
    children.truncate(max_avatars);
    children.reverse();
    let children = ChildrenWithProps::new(children);

    html! {
        <@{component.to_owned()} class={root_cls}>
            if extra_avatars > 0 {
                <Avatar classes={AVATAR_CLS} style={avatar_style.clone()}>
                    {"+"}{extra_avatars}
                </Avatar>
            }
            { for children }
        </@>
    }
}
