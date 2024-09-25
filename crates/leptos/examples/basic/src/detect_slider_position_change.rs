use leptos::{create_signal, mount_to_body, view, Callback, SignalSet};

use leptos_compare_image::LeptosCompareImage;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    let img1_src = "images/image1.png";
    let img2_src = "images/image2.png";
    let (position, set_position) = create_signal(0.5);

    mount_to_body(move || {
        view! {
            <div style="max-width: 640px;">
                <LeptosCompareImage
                left_image={img1_src}
                right_image={img2_src}
                on_slider_position_change={Some(Callback::new(move |position| set_position.set(position)))}
                />
                <div>slider position: {position}</div>
        </div>
        }
    })
}
