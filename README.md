# bevy_intro_screen

This is a versatile Bevy library designed to create engaging and customizable introductory screens for your game. Initially conceived for splash screens, its flexibility allows for adaptation as loading screens between game states.

### Features

* **Customizable:** Tailor the appearance and behavior of your intro screen to perfectly match your game's style.
* **Flexible Duration:** Control the display time of your intro screen, whether fixed or dynamic based on loading progress.
* **Robust Failure Handling:** Implement custom error management for unexpected situations.
* **Extensible:** Easily add custom components and systems to expand functionality.

## Getting Started

1. Add the dependency to your `Cargo.toml`:
```rust
[dependencies]
bevy_intro_screen = "0.1.0" 
```

2. Import the necessary stuff:
```rust,ignore
use bevy::prelude::*;
use bevy_intro_screen::prelude::*;

fn main() {
    let run_at = ..;
    let transition_to = ..;
    // Included options include using egui;
    let ui = ..;
    let preferences = IntroPreferences::builder()
        .run_at(run_at)
        .transition_to(transition_to)
        .skip_on_input(true)
        .duration(FixedDuration::new(transition_to))
        .ui(ui)
        .build();

    let splash_plugin = IntroScreenPlugin::builder()
        .preferences(preferences)
        .failure_manager(OnFailureContinue)
        .build();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(splash_plugin)
        .run()
}
```

Please note that the examples provided here are simplified and serve as a starting point. For comprehensive documentation of the crate, please visit the [crate documentation](https://docs.rs/bevy_intro_screen) for a better understanding of the crate's functionalities and APIs.

For more examples, please refer to the examples directory.

## Usage Beyond Intro Screens

While primarily designed for splash screens, this library can be adapted to function as a loading screen between game states

## Contributing

Contributions are welcome! Feel free to open issues or pull requests.
