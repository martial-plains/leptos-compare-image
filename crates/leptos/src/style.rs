use std::{
    collections::{BTreeMap, HashMap},
    option::Option,
    string::String,
};

use derive_more::derive::{Deref, DerefMut};
use leptos::{Attribute, IntoAttribute};

use crate::style;

#[derive(Debug, Default, Clone, Deref, DerefMut)]
pub struct Style(BTreeMap<String, Option<Attribute>>);

impl Style {
    #[must_use]
    pub fn as_css(&self) -> String {
        let mut buf = String::new();

        for (index, (key, value)) in <BTreeMap<String, Option<Attribute>> as Clone>::clone(self)
            .into_iter()
            .enumerate()
        {
            if index != 0 {
                buf.push(' ');
            }

            if let Some(value) = value {
                buf.push_str(
                    format!(
                        "{key}: {value};",
                        value = match value {
                            Attribute::String(value) => value.to_string(),
                            Attribute::Option(value) => match value {
                                Some(value) => value.to_string(),
                                None => "null".to_string(),
                            },
                            Attribute::Bool(value) => value.to_string(),
                            // Attribute::Number(value) => value.to_string(),
                            Attribute::Fn(_) =>
                                value.as_nameless_value_string().unwrap().to_string(),
                        }
                    )
                    .as_str(),
                );
            }
        }

        buf
    }
}

#[must_use]
#[allow(clippy::too_many_arguments)]
pub fn create_styles<'a>(
    handle_size: u32,
    container_height: f64,
    container_width: f64,
    slider_position: f64,
    slider_line_width: f64,
    slider_line_color: &str,
    is_sliding: bool,
    horizontal: bool,
    hover: bool,
    left_image_css: Option<Attribute>,
    right_image_css: Option<Attribute>,
) -> HashMap<&'a str, style::Style> {
    let mut styles = HashMap::<&'a str, style::Style>::new();

    let left_image_css = left_image_css.map(|css| {
        css.as_nameless_value_string()
            .unwrap_or_default()
            .split(';')
            .collect::<Vec<&str>>()
            .iter()
            .filter_map(|s| s.split_once(':'))
            .map(|(s1, s2)| (s1.trim(), s2.trim()))
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect::<HashMap<String, String>>()
    });

    let right_image_css = right_image_css.map(|css| {
        css.as_nameless_value_string()
            .unwrap_or_default()
            .split(';')
            .collect::<Vec<&str>>()
            .iter()
            .filter_map(|s| s.split_once(':'))
            .map(|(s1, s2)| (s1.trim(), s2.trim()))
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect::<HashMap<String, String>>()
    });

    let container = container(container_height);

    styles.insert("container", container);

    let right_image = right_image(
        horizontal,
        container_width,
        slider_position,
        container_height,
        right_image_css,
    );

    styles.insert("right_image", right_image);

    let left_image = left_image(
        horizontal,
        container_width,
        slider_position,
        container_height,
        left_image_css,
    );

    styles.insert("left_image", left_image);

    let slider = slider(
        horizontal,
        hover,
        container_width,
        slider_position,
        handle_size,
        container_height,
    );

    styles.insert("slider", slider);

    let line = line(slider_line_color, horizontal, slider_line_width);

    styles.insert("line", line);

    let handle_custom = handle_custon();

    styles.insert("handle_custom", handle_custom);

    let handle_default = handle_default(
        horizontal,
        slider_line_width,
        slider_line_color,
        handle_size,
    );

    styles.insert("handle_default", handle_default);

    let left_arrow = left_arrow(handle_size, slider_line_color);

    styles.insert("left_arrow", left_arrow);

    let right_arrow = right_arrow(handle_size, slider_line_color);

    styles.insert("right_arrow", right_arrow);

    let left_label = left_label(horizontal, is_sliding);

    styles.insert("left_label", left_label);

    let right_label = right_label(horizontal, is_sliding);

    styles.insert("right_label", right_label);

    let left_label_container = left_label_container(
        horizontal,
        container_width,
        slider_position,
        container_height,
    );

    styles.insert("left_label_container", left_label_container);

    let right_label_container = right_label_container(
        horizontal,
        container_width,
        slider_position,
        container_height,
    );

    styles.insert("right_label_container", right_label_container);

    styles
}

fn right_label_container(
    horizontal: bool,
    container_width: f64,
    slider_position: f64,
    container_height: f64,
) -> Style {
    let mut right_label_container = [
        ("height", "100%"),
        ("position", "absolute"),
        ("width", "100%"),
    ]
    .iter()
    .fold(style::Style::default(), |mut acc, (key, value)| {
        acc.insert(
            (*key).to_string(),
            Some(Attribute::String((*value).to_string().into())),
        );
        acc
    });

    right_label_container.insert(
        "clip".to_string(),
        if horizontal {
            Some(Attribute::String(
                format!(
                    "rect(auto, auto, auto, {:.0}px)",
                    container_width * slider_position
                )
                .into(),
            ))
        } else {
            Some(Attribute::String(
                format!(
                    "rect({:.0}px, auto, auto, auto)",
                    container_height * slider_position
                )
                .into(),
            ))
        },
    );
    right_label_container
}

fn left_label_container(
    horizontal: bool,
    container_width: f64,
    slider_position: f64,
    container_height: f64,
) -> Style {
    let mut left_label_container = [
        ("height", "100%"),
        ("position", "absolute"),
        ("width", "100%"),
    ]
    .iter()
    .fold(style::Style::default(), |mut acc, (key, value)| {
        acc.insert(
            (*key).to_string(),
            Some(Attribute::String((*value).to_string().into())),
        );
        acc
    });

    left_label_container.insert(
        "clip".to_string(),
        if horizontal {
            Some(Attribute::String(
                format!(
                    "rect(auto, {:.0}px, auto, auto)",
                    container_width * slider_position
                )
                .into(),
            ))
        } else {
            Some(Attribute::String(
                format!(
                    "rect({:.0}px, auto, auto, auto)",
                    container_height * slider_position
                )
                .into(),
            ))
        },
    );
    left_label_container
}

fn right_label(horizontal: bool, is_sliding: bool) -> Style {
    let mut right_label = [
        ("background", "rgba(0, 0, 0, 0.5)"),
        ("color", "white"),
        ("padding", "10px 20px"),
        ("position", "absolute"),
        (
            "transform",
            if horizontal {
                "translate(0, -50%)"
            } else {
                "translate(-50%, 0)"
            },
        ),
        ("transition", "opacity 0.1s ease-out"),
    ]
    .iter()
    .fold(style::Style::default(), |mut acc, (key, value)| {
        acc.insert(
            (*key).to_string(),
            Some(Attribute::String((*value).to_string().into())),
        );
        acc
    });

    right_label.extend(
        <BTreeMap<std::string::String, Option<Attribute>> as Clone>::clone(
            &<style::Style as Clone>::clone(
                &[
                    (
                        "left",
                        if horizontal {
                            None
                        } else {
                            Some(Attribute::String("50%".to_string().into()))
                        },
                    ),
                    (
                        "right",
                        if horizontal {
                            Some(Attribute::String("5%".to_string().into()))
                        } else {
                            None
                        },
                    ),
                    (
                        "top",
                        if horizontal {
                            Some(Attribute::String("50%".to_string().into()))
                        } else {
                            None
                        },
                    ),
                    (
                        "bottom",
                        if horizontal {
                            None
                        } else {
                            Some(Attribute::String("3%".to_string().into()))
                        },
                    ),
                ]
                .into_iter()
                .fold(style::Style::default(), |mut acc, (key, value)| {
                    acc.insert(key.to_string(), value);
                    acc
                }),
            ),
        ),
    );

    right_label.insert(
        "opacity".to_string(),
        if is_sliding {
            Some("0".into_attribute())
        } else {
            Some("1".into_attribute())
        },
    );
    right_label
}

fn left_label(horizontal: bool, is_sliding: bool) -> Style {
    let mut left_label = [
        ("background", "rgba(0, 0, 0, 0.5)"),
        ("color", "white"),
        ("left", if horizontal { "5%" } else { "50%" }),
        ("padding", "10px 20px"),
        ("position", "absolute"),
        ("top", if horizontal { "50%" } else { "3%" }),
        (
            "transform",
            if horizontal {
                "translate(0, -50%)"
            } else {
                "translate(-50%, 0)"
            },
        ),
        ("transition", "opacity 0.1s ease-out"),
    ]
    .iter()
    .fold(style::Style::default(), |mut acc, (key, value)| {
        acc.insert(
            (*key).to_string(),
            Some(Attribute::String((*value).to_string().into())),
        );
        acc
    });

    left_label.insert(
        "opacity".to_string(),
        if is_sliding {
            Some(0.into_attribute())
        } else {
            Some(1.into_attribute())
        },
    );
    left_label
}

fn right_arrow(handle_size: u32, slider_line_color: &str) -> Style {
    let right_arrow = [
        (
            "border",
            format!("inset {:.0}px rgba(0,0,0,0)", f64::from(handle_size) * 0.15).as_str(),
        ),
        (
            "border-left",
            format!(
                "{:.0}px solid {}",
                f64::from(handle_size) * 0.15,
                slider_line_color
            )
            .as_str(),
        ),
        ("height", "0px"),
        (
            "margin-right",
            format!("-{:.0}px", f64::from(handle_size) * 0.25).as_str(),
        ),
        ("width", "0px"),
    ]
    .iter()
    .fold(style::Style::default(), |mut acc, (key, value)| {
        acc.insert(
            (*key).to_string(),
            Some(Attribute::String((*value).to_string().into())),
        );
        acc
    });
    right_arrow
}

fn left_arrow(handle_size: u32, slider_line_color: &str) -> Style {
    [
        (
            "border",
            format!("inset {:.0}px rgba(0,0,0,0)", f64::from(handle_size) * 0.15).as_str(),
        ),
        (
            "border-right",
            format!(
                "{:.0}px solid {}",
                f64::from(handle_size) * 0.15,
                slider_line_color
            )
            .as_str(),
        ),
        ("height", "0px"),
        (
            "margin-left",
            format!("-{:.0}px", f64::from(handle_size) * 0.25).as_str(),
        ),
        (
            "margin-right",
            format!("{:.0}px", f64::from(handle_size) * 0.25).as_str(),
        ),
        ("width", "0px"),
    ]
    .iter()
    .fold(style::Style::default(), |mut acc, (key, value)| {
        acc.insert(
            (*key).to_string(),
            Some(Attribute::String((*value).to_string().into())),
        );
        acc
    })
}

fn handle_default(
    horizontal: bool,
    slider_line_width: f64,
    slider_line_color: &str,
    handle_size: u32,
) -> Style {
    let handle_default = [
        ("align-items", "center"),
        (
            "border",
            format!("{slider_line_width:.0}px solid {slider_line_color}").as_str(),
        ),
        ("border-radius", "100%"),
        ("box-shadow", "0px 3px 1px -2px rgba(0, 0, 0, 0.2), 0px 2px 2px 0px rgba(0, 0, 0, 0.14), 0px 1px 5px 0px rgba(0, 0, 0, 0.12)"),
        ("box-sizing", "border-box"),
        ("display", "flex"),
        ("flex", "1 0 auto"),
        ("height", format!("{handle_size}px").as_str()),
        ("justify-content", "center"),
        ("width", format!("{handle_size}px").as_str()),
        ("transform", if horizontal {
            "none"
        } else {
            "rotate(90deg)"
        })
    ].iter().fold(style::Style::default(), |mut acc, (key, value)| {
        acc.insert((*key).to_string(), Some(Attribute::String((*value).to_string().into())));
        acc
    });
    handle_default
}

fn handle_custon() -> Style {
    let handle_custom = [
        ("align-items", "center"),
        ("box-sizing", "border-box"),
        ("display", "flex"),
        ("flex", "1 0 auto"),
        ("height", "auto"),
        ("justify-content", "center"),
        ("width", "auto"),
    ]
    .iter()
    .fold(style::Style::default(), |mut acc, (key, value)| {
        acc.insert(
            (*key).to_string(),
            Some(Attribute::String((*value).to_string().into())),
        );
        acc
    });
    handle_custom
}

fn line(slider_line_color: &str, horizontal: bool, slider_line_width: f64) -> Style {
    let mut line = [
       ( "background", slider_line_color),
       ("box-shadow", "0px 3px 1px -2px rgba(0, 0, 0, 0.2), 0px 2px 2px 0px rgba(0, 0, 0, 0.14), 0px 1px 5px 0px rgba(0, 0, 0, 0.12)"),
       ("flex", "0 1 auto"),
    ].iter()
    .fold(style::Style::default(), |mut acc, (key, value)| {
        acc.insert((*key).to_string(), Some(Attribute::String((*value).to_string().into())));
        acc
    });

    line.extend(
        <BTreeMap<std::string::String, Option<Attribute>> as Clone>::clone(
            &<style::Style as Clone>::clone(
                &[
                    (
                        "height",
                        if horizontal {
                            "100%".to_string()
                        } else {
                            format!("{slider_line_width:.0}px")
                        },
                    ),
                    (
                        "width",
                        if horizontal {
                            format!("{slider_line_width:.0}px")
                        } else {
                            "100%".to_string()
                        },
                    ),
                ]
                .iter()
                .fold(style::Style::default(), |mut acc, (key, value)| {
                    acc.insert(
                        (*key).to_string(),
                        Some(Attribute::String(value.clone().into())),
                    );
                    acc
                }),
            ),
        ),
    );
    line
}

fn slider(
    horizontal: bool,
    hover: bool,
    container_width: f64,
    slider_position: f64,
    handle_size: u32,
    container_height: f64,
) -> Style {
    let mut slider = [
        ("align-items", "center"),
        ("display", "flex"),
        ("justify-content", "center"),
        ("position", "absolute"),
    ]
    .iter()
    .fold(style::Style::default(), |mut acc, (key, value)| {
        acc.insert(
            (*key).to_string(),
            Some(Attribute::String((*value).to_string().into())),
        );
        acc
    });

    slider.extend(
        <BTreeMap<std::string::String, Option<Attribute>> as Clone>::clone(
            &<style::Style as Clone>::clone(
                &[
                    (
                        "flex-direction",
                        if horizontal {
                            "column".to_string()
                        } else {
                            "row".to_string()
                        },
                    ),
                    (
                        "height",
                        if horizontal {
                            "100%".to_string()
                        } else {
                            format!("{handle_size:.0}px")
                        },
                    ),
                    (
                        "width",
                        if horizontal {
                            format!("{handle_size:.0}px")
                        } else {
                            "100%".to_string()
                        },
                    ),
                ]
                .iter()
                .fold(style::Style::default(), |mut acc, (key, value)| {
                    acc.insert(
                        (*key).to_string(),
                        Some(Attribute::String(value.clone().into())),
                    );
                    acc
                }),
            ),
        ),
    );

    slider.insert(
        "cursor".to_string(),
        if !hover && horizontal {
            Some(Attribute::String("ew-resize".to_string().into()))
        } else if !hover && !horizontal {
            Some(Attribute::String("ns-resize".to_string().into()))
        } else {
            Some(Attribute::Bool(false))
        },
    );

    slider.insert(
        "left".to_string(),
        if horizontal {
            Some(Attribute::String(
                format!(
                    "{:.0}px",
                    container_width.mul_add(slider_position, -(f64::from(handle_size) / 2.0))
                )
                .into(),
            ))
        } else {
            Some(0.into_attribute())
        },
    );

    slider.insert(
        "top".to_string(),
        if horizontal {
            Some(Attribute::String(leptos::Oco::Borrowed("0")))
        } else {
            Some(Attribute::String(
                format!(
                    "{:.0}px",
                    container_height.mul_add(slider_position, -(f64::from(handle_size) / 2.0))
                )
                .into(),
            ))
        },
    );
    slider
}

fn left_image(
    horizontal: bool,
    container_width: f64,
    slider_position: f64,
    container_height: f64,
    left_image_css: Option<HashMap<String, String>>,
) -> Style {
    let mut left_image = [
        ("display", "block"),
        ("height", "100%"),
        ("object-fit", "cover"),
        ("position", "absolute"),
        ("width", "100%"),
    ]
    .iter()
    .fold(style::Style::default(), |mut acc, (key, value)| {
        acc.insert(
            (*key).to_string(),
            Some(Attribute::String((*value).to_string().into())),
        );
        acc
    });

    left_image.insert(
        "clip".to_string(),
        if horizontal {
            Some(Attribute::String(
                format!(
                    "rect(auto, {:.0}px, auto, auto)",
                    container_width * slider_position
                )
                .into(),
            ))
        } else {
            Some(Attribute::String(
                format!(
                    "rect(auto, auto, {:.0}px, auto)",
                    container_height * slider_position
                )
                .into(),
            ))
        },
    );

    if let Some(mut left_image_css) = left_image_css {
        left_image.keys().for_each(|key| {
            left_image_css.remove(key);
        });

        for (key, value) in left_image_css {
            left_image.insert(key, Some(value.into_attribute()));
        }
    }

    left_image
}

fn right_image(
    horizontal: bool,
    container_width: f64,
    slider_position: f64,
    container_height: f64,
    right_image_css: Option<HashMap<String, String>>,
) -> Style {
    let mut right_image = [
        ("display", "block"),
        ("height", "100%"),
        ("object-fit", "cover"),
        ("position", "absolute"),
        ("width", "100%"),
    ]
    .iter()
    .fold(style::Style::default(), |mut acc, (key, value)| {
        acc.insert(
            (*key).to_string(),
            Some(Attribute::String((*value).to_string().into())),
        );
        acc
    });

    right_image.insert(
        "clip".to_string(),
        if horizontal {
            Some(Attribute::String(
                format!(
                    "rect(auto, auto, auto, {:.0}px)",
                    container_width * slider_position
                )
                .into(),
            ))
        } else {
            Some(Attribute::String(
                format!(
                    "rect({:.0}px, auto, auto, auto)",
                    container_height * slider_position
                )
                .into(),
            ))
        },
    );

    if let Some(mut right_image_css) = right_image_css {
        right_image.keys().for_each(|key| {
            right_image_css.remove(key);
        });

        for (key, value) in right_image_css {
            right_image.insert(key, Some(Attribute::String(value.into())));
        }
    }
    right_image
}

fn container(container_height: f64) -> Style {
    [
        ("box-sizing", "border-box"),
        ("position", "relative"),
        ("width", "100%"),
        ("height", format!("{container_height:.0}px").as_str()),
        ("overflow", "hidden"),
    ]
    .iter()
    .fold(style::Style::default(), |mut acc, (key, value)| {
        acc.insert(
            (*key).to_string(),
            Some(Attribute::String((*value).to_string().into())),
        );
        acc
    })
}
