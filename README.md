# SLAM Viewer

![demo](screenshot.png)

Simple [wgpu](https://github.com/gfx-rs/wgpu-rs) based SLAM map viewer.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
slam-cv = "0.1"
slam-viewer = "0.1"
```

If you are new to [slam-cv](https://github.com/podo-os/slam-cv), enter this command simply at your prompt:

```sh
cargo run --example simple
```

If you have your own `World`, add this to your `main.rs`:

```rust
use slam_viewer::Viewer;

fn main() {
    let world = MyWorld(..);

    let viewer = Viewer::default();
    viewer.run(world);
}
```
