// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{classes, function_component, html, AttrValue, Children, Classes, Html, Properties};
use zu_util::prop::ToAttr;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub alt: AttrValue,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub component: AttrValue,

    /// Image to be displayed as a background image.
    #[prop_or_default]
    pub image: AttrValue,

    /// An alias for image property.
    #[prop_or_default]
    pub src: AttrValue,

    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or_default]
    pub title: AttrValue,

    #[prop_or_default]
    pub width: i32,

    #[prop_or_default]
    pub height: i32,
}

#[function_component(CardMedia)]
pub fn card_media(props: &Props) -> Html {
    const MEDIA_COMPONENTS: &[&str] = &["video", "audio", "picture", "iframe", "img"];
    const IMAGE_COMPONENTS: &[&str] = &["picture", "img"];
    let is_media_component = MEDIA_COMPONENTS.contains(&props.component.as_str());
    let is_image_component = IMAGE_COMPONENTS.contains(&props.component.as_str());

    let component = if props.component.is_empty() {
        "div"
    } else {
        props.component.as_str()
    };

    let root_cls = classes!(
        "ZuCardMedia-root",
        if is_media_component {
            "ZuCardMedia-media"
        } else {
            ""
        },
        if is_image_component {
            "ZuCardMedia-image"
        } else {
            ""
        },
        props.classes.clone(),
    );

    let style = if !is_media_component && !props.image.is_empty() {
        [
            &format!(r#"background-image: url("{}")"#, props.image.as_str()),
            props.style.as_str(),
        ]
        .join(";")
    } else {
        props.style.as_str().to_owned()
    };

    let img_src = if is_media_component {
        if !props.image.is_empty() {
            Some(props.image.as_str().to_owned())
        } else if !props.src.is_empty() {
            Some(props.src.as_str().to_owned())
        } else {
            None
        }
    } else {
        None
    };

    // TODO(Shaohua): Add role attribute.
    html! {
        <@{component.to_owned()}
            class={root_cls}
            alt={props.alt.to_attr()}
            title={props.title.to_attr()}
            width={if props.width > 0 {Some(props.width.to_string()) } else { None }}
            height={if props.height > 0 {Some(props.height.to_string()) } else { None }}
            style={style}
            src={img_src}>
            {for props.children.iter()}
        </@>
    }
}
