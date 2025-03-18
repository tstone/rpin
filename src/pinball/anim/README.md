# LED Animations

## Setup

This plugin requires two things:

1. That `RgbLed`'s have been spawned in the game world
2. That the `LedAnimationPlugin` has been added

```rust
  .add_plugins(LedAnimationPlugin)
```

## Concepts

- An "animation" is a description of an animation, with some configuration (color, ease, etc.)
- An "animation playback" is a rendered animation, associated with the entities it will modify

## LedAnimationPlayback

Starting at the lowest level, an `LedAnimationPlayback` can be built from scratch, manually specifying the frame. Let's start first by building a blue and red police "wig wag".

```rust
fn manual_playback(mut commands: Commands, query: Query<Entity, With<RgbLed>>) {
    // select the entities which will be animated
    let entities = query.iter().take(2).collect::<Vec<_>>();
    // define the frames of the animation
    let frames = vec![
        vec![Color::from(RED), Color::from(BLUE)],
        vec![Color::from(BLUE), Color::from(RED)],
    ];
    // spawn a `LedAnimationPlayback` to make it play
    // 5 = fps
    // None = repeat to infinity
    commands.spawn(LedAnimationPlayback::new(entities, 5, frames, None));
}
```

## LedAnimation

While this low level creation of frames is the simplest, it's not the most convinient. Instead, this plugin introduces the `LedAnimation` trait.

This allows a description of an animation to be built up which can then be rendered into an animation playback. Let's start by making an animation that fades up a single LED over 2 seconds.

```rust
fn single_color(mut commands: Commands, query: Query<Entity, With<RgbLed>>) {
    let entities = query.iter().take(1).collect::<Vec<_>>();
    // creating a pre-built animation
    let anim = BrightnessEaseIn {
        color: Color::from(AQUA),
        from: 0.,
        ease: EasingFunction::Linear
    };
    // rendering it to a playback for a specific set of entities
    // 20 = fps
    let playback = anim.to_infinite_playback(Duration::from_secs(3), entities, 20);
    commands.spawn(playback);
}
```

`LedAnimation` include three ways to render them:

```rust
// plays once then despawns
anim.to_one_shot(duration, entities, fps);
// plays N times then despawns
anim.to_fixed_playback(play_count, duration, entities, fps);
// plays forever; must be manually despawned
anim.to_infinite_playback(duration, entities, fps);
```

## LedAnimationSequence

Animations are useful building blocks, but they really come into their own once they are sequeneced together. Mutliple LedAnimations can be chained and treated as a single animation. For example, let's create a breathing effect that uses custom easing for the in and out states of the "on" phase.

```rust
fn breathing(mut commands: Commands, query: Query<Entity, With<RgbLed>>) {
    let entities = query.iter().take(1).collect::<Vec<_>>();
    let half_second = Duration::from_millis(500);
    let color = Color::from(RED);
    let playback = LedAnimationSequence::new()
        // add animation to the sequence with .add(duration, animation)
        .add(half_second, BrightnessEaseIn {
            color,
            from: 0.,
            ease: EasingFunction::CubicIn
        })
        .add(half_second, Solid { color })
        .add(half_second, BrightnessEaseOut {
            color,
            to: 0.,
            ease: EasingFunction::BackOut
        })
        .add(half_second, Solid { color: Color::from(BLACK) })
        .to_infinite_playback(entities, 24)

    commands.spawn(playback);
}
```

Just like animations, animation sequences also support `to_one_shot`, `to_fixed_playback`, and `to_infinite_playback`.

### Clearing

Sometimes the LED animation should play, then it should clear out. For these cases `.clear()` will insert a 1ms frame of black to turn the LED off.

```rust
fn one_then_off(mut commands: Commands, query: Query<Entity, With<RgbLed>>) {
    let entities = query.iter().take(1).collect::<Vec<_>>();

    // Turn an LED on for 2 seconds as purple then turn it off
    let playback = LedAnimationSequence::new()
        .add(
            Duration::from_secs(2),
            Solid { color: Color::from(PURPLE) }
        )
        .clear()
        .to_one_shot(entities, 1)

    commands.spawn(playback);
}
```

### Repeat Last

With sequences however there is another case that's slightly different. What if we wanted to play a series of animations, then have just the _last_ animation repeat? For these cases `forever` will replace `add`. Once forever will be called, no other animations can be added.

```rust
fn repeat_last(mut commands: Commands, query: Query<Entity, With<RgbLed>>) {
    let entities = query.iter().take(1).collect::<Vec<_>>();

    let color = Color::from(AQUA);
    let playback = LedAnimationSequence::new()
        // "bounce in" the indicator
        .add(Duration::from_millis(500), BrightnessEaseIn {
            color,
            from: 0.,
            ease: EasingFunction::Bounce(20.)
        })
        // then keep it flashing forever
        .forever(Duration::from_millis(750), Flash {
            color,
            hz: 20.,
            ease_in: EasingFunction::BackIn,
            ease_out: EasingFunction::BackOut,
        })
        .to_infinite_playback(entities, 24);

        // here the "BrightnessEaseIn" wil play once then "Flash" will keep playing
        // forever until the playback is despawned

    commands.spawn(playback);
}
```
