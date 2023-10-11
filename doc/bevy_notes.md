# Bevy notes

## Main doc with short tutorials

<https://bevy-cheatbook.github.io>

## Camera control

I was able to use the followin to move the camera around

```rust
let mut camera = Camera2dBundle {
    ..Default::default()
};
// This can be adjusted to move the camera
camera.projection.viewport_origin += Vec2::new(0., 0.);
commands.spawn(camera);
```
