# Rotating Led Display
No hardware rotating led display.
It's like the regular thing but without buying the hardware.
We cheat a little with several rotating arms otherwise raylib refresh rate break everything.

## Example

```rust
mod rld;

fn main() {
    let screen_width = 640;
    let screen_height = 480;
    let nb_arms = 60;
    let nb_led = 40;
    let mut disp = rld::Rld::new("assets/link.png", screen_width, screen_height, nb_arms, nb_led);
    disp.start();
}
```
![Input image](assets/link.png)

![Result](result.png)
