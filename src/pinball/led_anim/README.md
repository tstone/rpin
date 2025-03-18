# LED Animations

## Concepts

- "Building an animation" = Turning colors, a duration, and easing(s) into a sequence of frames

## Constructs

- `Animation` - The description of how frames can be generated
- `LedAnimation` - Rendered frames ready to run in Bevy

```rust
AnimationSequence::new()
    .once(EaseBrightness {
        color: Color::from(RED),
        duration: Duration::from_secs(10),
        easing: Easing { from: 0., to: 1., easfn: EaseFunction::QuadraticIn }
    })
    .twice()
    .repeat(5, Anim { ... })
    .infinite(CrossfadeColorTo {
        color: Color::from(BLUE),
        duration: Duration::from_secs(2),
        easing: None
    })
    // can't add more animations after this
    .render_for(entities, fps) // AnimationSequence => LedAnimationPlayback

trait Animation {
    render(led_count: u16, previous_color: Option<Color>, fps: u8) -> RenderedAnimation
    to_led_anim(entities: Vec<Entity>, fps: u8) -> LedAnimation
}



AnimationSequence::new()
    .play(Duration::from_secs(10), Repeated(3, EaseBrightness {
        color: Color::from(RED),
        easing: Easing { from: 0., to: 1., easfn: EaseFunction::QuadraticIn }
    }))
    .play(Duration::from_secs(10), ComboAnimation(
        EaseBrightness {
            color: Color::from(RED),
            easing: Easing { from: 0., to: 1., easfn: EaseFunction::QuadraticIn }
        },
        EasedTranslation {

            easing: Easing { from: 0., to: 1., easfn: EaseFunction::QuadraticIn }
        }
    ))
    .repeat(CrossfadeColorTo {
        color: Color::from(BLUE),
        easing: None
    })
    .to_playback(entities, fps))
```
