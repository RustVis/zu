// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

mod person;
mod variant;

use yew::{classes, function_component, html, AttrValue, Children, Html, Properties};
use zu_util::{name, prop::ToAttr};

use crate::styles::shape_variant::ShapeVariant;
pub use person::Person;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    /// Used in combination with src or srcSet to provide an alt attribute for the rendered img element.
    #[prop_or_default]
    pub alt: AttrValue,

    #[prop_or_default]
    pub aria_label: AttrValue,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: AttrValue,

    #[prop_or(false)]
    pub color_default: bool,

    #[prop_or_default]
    pub component: AttrValue,

    /// Setup avatar background color and value based on specified name.
    #[prop_or_default]
    pub name: AttrValue,

    // TODO(Shaohua): Add img_props
    //pub img_props: AttrValue,
    /// The sizes attribute for the img element.
    #[prop_or_default]
    pub sizes: AttrValue,

    /// The src attribute for the img element.
    #[prop_or_default]
    pub src: AttrValue,

    /// The srcSet attribute for the img element.
    #[prop_or_default]
    pub src_set: AttrValue,

    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or_default]
    pub variant: ShapeVariant,
}

#[function_component(Avatar)]
pub fn avatar(props: &Props) -> Html {
    let component = if props.component.is_empty() {
        "div"
    } else {
        props.component.as_str()
    };
    let has_image = !props.src.is_empty() || !props.src_set.is_empty();

    let root_cls = classes!(
        "ZuAvatar-root",
        props.classes.as_str().to_owned(),
        variant::css_class(props.variant),
        if has_image {
            ""
        } else {
            "ZuAvatar-colorDefault"
        },
    );
    // TODO(Shaohua): Load image first.
    // TODO(Shaohua): Support fallback, fallback order:
    // - the provided children
    // - the first letter of the alt text
    // - a generic avatar icon

    let style = [
        props.style.as_str(),
        if props.name.is_empty() {
            String::new()
        } else {
            format!("background-color: {};", name::to_color(props.name.as_str()))
        }
        .as_str(),
    ]
    .join(";");

    let abbr_name: String = if props.name.is_empty() {
        String::new()
    } else {
        let abbr_name = name::abbreviate(props.name.as_str());
        log::info!("abbr_name: {abbr_name}");
        abbr_name
    };

    html! {
        <@{component.to_owned()} class={root_cls} style={style}>
            if !props.children.is_empty() {
                {for props.children.iter()}
            } else if has_image {
                <img class="ZuAvatar-img"
                    aria-label={props.aria_label.to_attr()}
                    src={props.src.to_attr()}
                    src-set={props.src_set.to_attr()}
                    alt={props.alt.to_attr()}
                    sizes={props.sizes.to_attr()} />
            } else if !abbr_name.is_empty() {
                {abbr_name}
            } else {
                <Person classes="ZuAvatar-fallback" />
            }
        </@>
    }
}
