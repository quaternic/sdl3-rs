extern crate sdl3;
use sdl3::{rect::Rect, render::create_renderer, render::ClippingRect};

#[test]
fn clipping_rect_intersection() {
    // a zero area clipping rect intersecting with anything else gives zero.
    assert_eq!(
        ClippingRect::Zero.intersection(ClippingRect::Zero),
        ClippingRect::Zero
    );
    assert_eq!(
        ClippingRect::Zero.intersection(ClippingRect::None),
        ClippingRect::Zero
    );
    assert_eq!(
        ClippingRect::Zero.intersection(ClippingRect::Some(Rect::new(0, 0, 1, 1))),
        ClippingRect::Zero
    );

    // none gives whatever the arg was
    assert_eq!(
        ClippingRect::None.intersection(ClippingRect::Zero),
        ClippingRect::Zero
    );
    assert_eq!(
        ClippingRect::None.intersection(ClippingRect::None),
        ClippingRect::None
    );
    assert_eq!(
        ClippingRect::None.intersection(ClippingRect::Some(Rect::new(0, 0, 1, 1))),
        ClippingRect::Some(Rect::new(0, 0, 1, 1))
    );

    assert_eq!(
        ClippingRect::Some(Rect::new(0, 0, 1, 1)).intersection(ClippingRect::Zero),
        ClippingRect::Zero
    );
    assert_eq!(
        ClippingRect::Some(Rect::new(0, 0, 1, 1)).intersection(ClippingRect::None),
        ClippingRect::Some(Rect::new(0, 0, 1, 1))
    );
    assert_eq!(
        ClippingRect::Some(Rect::new(0, 0, 10, 10))
            .intersection(ClippingRect::Some(Rect::new(20, 20, 1, 1))),
        ClippingRect::Zero
    );

    assert_eq!(
        ClippingRect::Some(Rect::new(0, 0, 10, 10))
            .intersection(ClippingRect::Some(Rect::new(5, 5, 10, 10))),
        ClippingRect::Some(Rect::new(5, 5, 5, 5))
    );
}

#[test]
fn clipping_rect_intersect_rect() {
    assert_eq!(ClippingRect::Zero.intersect_rect(None), ClippingRect::Zero);
    assert_eq!(
        ClippingRect::Zero.intersect_rect(Rect::new(0, 0, 1, 1)),
        ClippingRect::Zero
    );

    assert_eq!(ClippingRect::None.intersect_rect(None), ClippingRect::Zero);
    assert_eq!(
        ClippingRect::None.intersect_rect(Rect::new(0, 0, 1, 1)),
        ClippingRect::Some(Rect::new(0, 0, 1, 1))
    );

    assert_eq!(
        ClippingRect::Some(Rect::new(0, 0, 1, 1)).intersect_rect(None),
        ClippingRect::Zero
    );
    assert_eq!(
        ClippingRect::Some(Rect::new(0, 0, 10, 10)).intersect_rect(Rect::new(5, 5, 10, 10)),
        ClippingRect::Some(Rect::new(5, 5, 5, 5))
    );
    assert_eq!(
        ClippingRect::Some(Rect::new(0, 0, 10, 10)).intersect_rect(Rect::new(20, 20, 1, 1)),
        ClippingRect::Zero
    );
}

#[test]
fn creating_a_named_renderer() {
    // hidden window
    let window = sdl3::init()
        .unwrap()
        .video()
        .unwrap()
        .window("Hello, World!", 800, 600)
        .hidden()
        .metal_view()
        .build()
        .unwrap();

    // the software renderer should always be available
    create_renderer(window, Some(c"software")).unwrap();
}
