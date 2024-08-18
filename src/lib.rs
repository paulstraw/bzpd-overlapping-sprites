#![no_std]
extern crate alloc;

#[macro_use]
extern crate playdate as pd;

use core::ffi::*;
use core::ptr::NonNull;
use pd::display::Display;
use pd::fs::Path;
use pd::graphics::bitmap::*;
use pd::graphics::text::*;
use pd::graphics::*;
use pd::sound::prelude::*;
use pd::sprite;
use pd::sprite::Sprite;
use pd::sys::ffi::LCDSolidColor;
use pd::sys::ffi::PDRect;
use pd::sys::ffi::PlaydateAPI;
use pd::sys::EventLoopCtrl;
use pd::system::prelude::*;
use pd::system::update::UpdateCtrl;

/// Game state
struct State {
    bmp: Bitmap,
    sprite: Sprite,
    sprite2: Sprite,
    sprite3: Sprite,
}

impl State {
    fn new() -> Self {
        let bmp = Bitmap::new(20, 20, Color::Solid(LCDSolidColor::kColorBlack))
            .expect("Oh no, Mr. Bill!");

        let sprite = Sprite::new();
        sprite.set_image(&bmp, BitmapFlip::kBitmapUnflipped);
        sprite.move_to(50.0, 50.0);
        sprite.set_collide_rect(PDRect {
            x: 0.0,
            y: 0.0,
            width: 20.0,
            height: 20.0,
        });
        sprite.add();

        let sprite2 = Sprite::new();
        sprite2.set_image(&bmp, BitmapFlip::kBitmapUnflipped);
        sprite2.move_to(60.0, 60.0);
        sprite2.set_collide_rect(PDRect {
            x: 0.0,
            y: 0.0,
            width: 20.0,
            height: 20.0,
        });
        sprite2.add();

        let sprite3 = Sprite::new();
        sprite3.set_image(&bmp, BitmapFlip::kBitmapUnflipped);
        sprite3.move_to(40.0, 40.0);
        sprite3.set_collide_rect(PDRect {
            x: 0.0,
            y: 0.0,
            width: 20.0,
            height: 20.0,
        });
        sprite3.add();

        Self {
            bmp,
            sprite,
            sprite2,
            sprite3,
        }
    }

    /// System event handler
    fn event(&'static mut self, event: SystemEvent) -> EventLoopCtrl {
        match event {
            // Initial setup
            SystemEvent::Init => {
                // Set FPS to 30
                Display::Default().set_refresh_rate(30.0);

                // Register our update handler that defined below
                self.set_update_handler();

                println!("Game init complete");
            }
            _ => {}
        }
        EventLoopCtrl::Continue
    }
}

impl Update for State {
    /// Updates the state
    fn update(&mut self) -> UpdateCtrl {
        clear(Color::WHITE);

        System::Default().draw_fps(0, 0);

        sprite::update_and_draw_sprites();

        // NOTE: Panics if no sprites overlap (or if self.sprite has no collide rect)
        // NOTE: Leaks when sprites overlap (8 bytes per overlapping sprite?)
        self.sprite.overlapping_sprites();

        UpdateCtrl::Continue
    }
}

/// Entry point
#[no_mangle]
pub fn event_handler(
    _api: NonNull<PlaydateAPI>,
    event: SystemEvent,
    _sim_key_code: u32,
) -> EventLoopCtrl {
    // Unsafe static storage for our state.
    // Usually it's safe because there's only one thread.
    pub static mut STATE: Option<State> = None;
    if unsafe { STATE.is_none() } {
        let state = State::new();
        unsafe { STATE = Some(state) }
    }

    // Call state.event
    unsafe { STATE.as_mut().expect("impossible") }.event(event)
}

// Needed for debug build, absolutely optional
ll_symbols!();
