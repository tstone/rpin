# LED Animations

## Constructs

- `LedAnimation` - The description of how frames can be generated
- `LedAnimationPlayback` - Rendered frames ready to run in Bevy

```rust
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
