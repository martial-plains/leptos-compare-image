use std::{cell::RefCell, option::Option, rc::Rc};

use derive_more::derive::Display;
use ev::{MouseEvent, TouchEvent};
use html::{Div, Img};
use leptos::{
    component, create_effect, create_node_ref, create_rw_signal, create_signal, ev, html, view,
    window, window_event_listener, AttributeValue, CollectView, HtmlElement, IntoAttribute,
    IntoView, NodeRef, SignalGet, SignalSet,
};
use leptos_use::{use_event_listener, use_resize_observer};
use style::create_styles;

const WHITE_HEX: &str = "#ffffff";

pub mod style;

#[derive(Debug, Display, Default)]
pub enum AspectRatio {
    #[default]
    #[display("taller")]
    Taller,
    #[display("wider")]
    Wider,
}

#[component]
#[must_use]
pub fn LeptosCompareImage(
    #[prop(default = AspectRatio::Taller)] aspect_ratio: AspectRatio,
    #[prop(default = None)] handle: Option<()>,
    #[prop(default = 40, into)] handle_size: u32,
    #[prop(default = false, into)] hover: bool,
    #[prop(into)] left_image: String,
    #[prop(default = String::new(), into)] left_image_alt: String,
    #[prop(default = None, into)] left_image_css: Option<AttributeValue>,
    #[prop(default = None, into)] left_image_label: Option<String>,
    #[prop(default = None, into)] on_slider_position_change: Option<fn(f64)>,
    #[prop(into)] right_image: String,
    #[prop(default = String::new(), into)] right_image_alt: String,
    #[prop(default = None, into)] right_image_css: Option<AttributeValue>,
    #[prop(default = None, into)] right_image_label: Option<String>,
    #[prop(default = None, into)] skeleton: Option<()>,
    #[prop(default = String::from(WHITE_HEX), into)] slider_line_color: String,
    #[prop(default = 2.0, into)] slider_line_width: f64,
    #[prop(default = 0.5, into)] slider_position_percentage: f64,
    #[prop(default = false)] vertical: bool,
) -> impl IntoView {
    let horizontal = !vertical;

    let left_image_css = create_rw_signal(left_image_css.into_attribute());
    let right_image_css = create_rw_signal(right_image_css.into_attribute());

    // 0 to 1
    let (slider_position, set_slider_position) = create_signal(slider_position_percentage);
    let (container_width, set_container_width) = create_signal(0.0);
    let (container_height, set_container_height) = create_signal(0.0);
    let (left_img_loaded, set_left_img_loaded) = create_signal(false);
    let (right_img_loaded, set_right_img_loaded) = create_signal(false);
    let (is_sliding, set_is_sliding) = create_signal(false);

    let container_ref: NodeRef<Div> = create_node_ref();
    let right_image_ref = create_node_ref();
    let left_image_ref = create_node_ref();

    use_resize_observer(container_ref, move |entry, _| {
        let current_container_width = entry[0].target().get_bounding_client_rect().width();
        set_container_width.set(current_container_width);
    });

    create_effect(move |_| {
        let already_done = left_image_ref.get().is_some();
        if already_done {
            set_left_img_loaded.set(true);
        }

        move || set_left_img_loaded.set(false)
    });

    create_effect(move |_| {
        let already_done = right_image_ref.get().is_some();
        if already_done {
            set_right_img_loaded.set(true);
        }

        move || set_right_img_loaded.set(false)
    });

    let all_images_loaded = move || right_img_loaded.get() && left_img_loaded.get();

    create_effect(move |_| {
        let window_touchmove_listener = Rc::new(RefCell::new(None));
        let window_mousemove_listener = Rc::new(RefCell::new(None));

        let handle_sliding_touch = move |event: TouchEvent| {
            let e = event;

            let cursor_x_from_viewport = if e.touches().length() < 1 {
                e.touches().get(0).unwrap().page_x()
            } else {
                e.page_x()
            };

            let cursor_y_from_viewport = if e.touches().length() < 1 {
                e.touches().get(0).unwrap().page_y()
            } else {
                e.page_y()
            };

            let cursor_x_from_window =
                f64::from(cursor_x_from_viewport) - window().page_x_offset().unwrap();
            let cursor_y_from_window =
                f64::from(cursor_y_from_viewport) - window().page_y_offset().unwrap();

            let image_position =
                (right_image_ref.get().unwrap() as HtmlElement<Img>).get_bounding_client_rect();
            let mut pos = if horizontal {
                cursor_x_from_window - image_position.left()
            } else {
                cursor_y_from_window - image_position.top()
            };

            let min_pos = 0.0 + slider_line_width / 2.0;
            let max_pos = if horizontal {
                container_width.get() - slider_line_width / 2.0
            } else {
                container_height.get() - slider_line_width / 2.0
            };

            if pos < min_pos {
                pos = min_pos;
            }

            if pos > max_pos {
                pos = max_pos;
            }

            if horizontal {
                set_slider_position.set(pos / container_width.get());
            } else {
                set_slider_position.set(pos / container_height.get());
            }

            if let Some(on_slider_position_change) = on_slider_position_change {
                if horizontal {
                    on_slider_position_change(pos / container_width.get());
                } else {
                    on_slider_position_change(pos / container_height.get());
                }
            }
        };

        let handle_sliding_mouse = move |event: MouseEvent| {
            let e = event;

            let cursor_x_from_viewport = e.page_x();

            let cursor_y_from_viewport = e.page_y();

            let cursor_x_from_window =
                f64::from(cursor_x_from_viewport) - window().page_x_offset().unwrap();
            let cursor_y_from_window =
                f64::from(cursor_y_from_viewport) - window().page_y_offset().unwrap();

            let image_position =
                (right_image_ref.get().unwrap() as HtmlElement<Img>).get_bounding_client_rect();
            let mut pos = if horizontal {
                cursor_x_from_window - image_position.left()
            } else {
                cursor_y_from_window - image_position.top()
            };

            let min_pos = 0.0 + slider_line_width / 2.0;
            let max_pos = if horizontal {
                container_width.get() - slider_line_width / 2.0
            } else {
                container_height.get() - slider_line_width / 2.0
            };

            if pos < min_pos {
                pos = min_pos;
            }

            if pos > max_pos {
                pos = max_pos;
            }

            if horizontal {
                set_slider_position.set(pos / container_width.get());
            } else {
                set_slider_position.set(pos / container_height.get());
            }

            if let Some(on_slider_position_change) = on_slider_position_change {
                if horizontal {
                    on_slider_position_change(pos / container_width.get());
                } else {
                    on_slider_position_change(pos / container_height.get());
                }
            }
        };

        let wtl = window_touchmove_listener.clone();
        let start_sliding_touch = move |e: TouchEvent| {
            set_is_sliding.set(true);

            if e.touches().length() == 0 {
                e.prevent_default();
            }

            handle_sliding_touch(e);

            wtl.clone().replace(Some(window_event_listener(
                ev::touchmove,
                handle_sliding_touch,
            )));
        };

        let wml = window_mousemove_listener.clone();
        let start_sliding_mouse = move |_| {
            set_is_sliding.set(true);

            wml.replace(Some(window_event_listener(
                ev::mousemove,
                handle_sliding_mouse,
            )));
        };

        let wtl = window_touchmove_listener;
        let finish_sliding_mobile = move |_| {
            set_is_sliding.set(false);
            if let Some(window_touchmove_listener) = wtl.borrow_mut().take() {
                window_touchmove_listener.remove();
            }
        };

        let wml = window_mousemove_listener;
        let finish_sliding_desktop = move |_| {
            set_is_sliding.set(false);
            if let Some(window_touchmove_listener) = wml.borrow_mut().take() {
                window_touchmove_listener.remove();
            }
        };

        let mut container_touchstart_handle = None;
        let window_touchend_handle = if all_images_loaded() {
            container_touchstart_handle = Some(use_event_listener(
                container_ref,
                ev::touchstart,
                start_sliding_touch,
            ));

            Some(window_event_listener(ev::touchend, finish_sliding_mobile))
        } else {
            None
        };

        let mut container_mousemove_handle = None;
        let mut container_mouseleave_handle = None;
        let mut container_mousedown_handle = None;
        let mut window_mouseup_handle = None;

        if hover {
            container_mousemove_handle = Some(use_event_listener(
                container_ref,
                ev::mousemove,
                handle_sliding_mouse,
            ));
            container_mouseleave_handle = Some(use_event_listener(
                container_ref,
                ev::mouseleave,
                finish_sliding_desktop,
            ));
        } else {
            container_mousedown_handle = Some(use_event_listener(
                container_ref,
                ev::mousedown,
                start_sliding_mouse,
            ));
            window_mouseup_handle =
                Some(window_event_listener(ev::mouseup, finish_sliding_desktop));
        }

        let Some(left_image_ref): Option<HtmlElement<Img>> = left_image_ref.get() else {
            return;
        };

        let left_image_width_height_ratio = f64::from((left_image_ref).natural_height())
            / f64::from((left_image_ref).natural_width());

        let right_image_width_height_ratio =
            f64::from((right_image_ref.get().unwrap() as HtmlElement<Img>).natural_height())
                / f64::from((right_image_ref.get().unwrap() as HtmlElement<Img>).natural_width());

        let ideal_width_height_ratio = if matches!(aspect_ratio, AspectRatio::Taller) {
            left_image_width_height_ratio.max(right_image_width_height_ratio)
        } else {
            left_image_width_height_ratio.min(right_image_width_height_ratio)
        };

        let ideal_container_height = container_width.get() * ideal_width_height_ratio as f64;

        set_container_height.set(ideal_container_height);

        if let (Some(func1), Some(func2)) = (container_touchstart_handle, window_touchend_handle) {
            func1();
            func2.remove();
        }

        if let (Some(func1), Some(func2), Some(func3), Some(func4)) = (
            container_mousemove_handle,
            container_mouseleave_handle,
            container_mousedown_handle,
            window_mouseup_handle,
        ) {
            func1();
            func2();
            func3();
            func4.remove();
        }
    });

    let styles = Rc::new(move || {
        create_styles(
            handle_size,
            container_height.get(),
            container_width.get(),
            slider_position.get(),
            slider_line_width,
            &slider_line_color,
            is_sliding.get(),
            horizontal,
            hover,
            Some(left_image_css.get()),
            Some(right_image_css.into_attribute()),
        )
    });

    view! {
        <>
            {move || {
                skeleton
                    .into_iter()
                    .filter(|()| !all_images_loaded())
                    .map(|skeleton| {
                        view! { <div>{skeleton}</div> }
                    })
                    .collect_view()
            }}
            <div
                style={
                    let style = styles.clone();
                    move || {
                        let mut style = style()["container"].as_css();
                        style
                            .push_str(
                                format!(
                                    "display: {}",
                                    if all_images_loaded() { "block" } else { "none" },
                                )
                                    .as_str(),
                            );
                        style
                    }
                }
                node_ref=container_ref
                data-testid="container"
            >
                <img
                    on:load=move |_| set_right_img_loaded.set(true)
                    alt=right_image_alt.clone()
                    data-testid="right-image"
                    node_ref=right_image_ref
                    src=right_image.clone()
                    style={
                        let style = styles.clone();
                        move || style()["right_image"].as_css()
                    }
                />

                <img
                    on:load=move |_| set_left_img_loaded.set(true)
                    alt=left_image_alt.clone()
                    data-testid="left-image"
                    node_ref=left_image_ref
                    src=left_image.clone()
                    style={
                        let style = styles.clone();
                        move || style()["left_image"].as_css()
                    }
                />
                <div style={
                    let style = styles.clone();
                    move || style()["slider"].as_css()
                }>
                    <div style={
                        let style = styles.clone();
                        move || style()["line"].as_css()
                    } />
                    {
                        let style = styles.clone();
                        move || {
                            if handle.is_some() {
                                view! {
                                    <div style=style()["handle_custom"].as_css()>{handle}</div>
                                }
                            } else {
                                view! {
                                    <div style=style()["handle_default"].as_css()>
                                        <div style=style()["left_arrow"].as_css() />
                                        <div style=style()["right_arrow"].as_css() />
                                    </div>
                                }
                            }
                        }
                    }
                    <div style={
                        let style = styles.clone();
                        move || style()["line"].as_css()
                    } />
                </div>
                // labels
                {
                    let style = styles.clone();
                    move || {
                        left_image_label
                            .iter()
                            .map(|label| {
                                view! {
                                    <div style=style()["left_label_container"].as_css()>
                                        <div style=style()["left_label"].as_css()>{label}</div>
                                    </div>
                                }
                            })
                            .collect_view()
                    }
                }

                {
                    let style = styles.clone();
                    move || {
                        right_image_label
                            .iter()
                            .map(|label| {
                                view! {
                                    <div style=style()["right_label_container"].as_css()>
                                        <div style=style()["right_label"].as_css()>{label}</div>
                                    </div>
                                }
                            })
                            .collect_view()
                    }
                }

            </div>
        </>
    }
}
