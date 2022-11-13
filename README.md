# QPL - Quigly's Platform Layer

QPL is a platform abstraction library. This library handles windowing, event polling, resource querying, ect.

## Usage

```rust
fn main()
{
    qpl::init();

    let mut window = qpl::create_window(&qpl::WindowCreateInfo
    {
        width: 1280,
        height: 720,
        title: "My Application",
        mode: qpl::WindowMode::Windowed,
        resizable: false,
        ..Default::default()
    });

    while !window.should_close
    {
        window.update_input_state();

        'event_loop: loop
        {
            match window.poll_events()
            {
                Some(event) =>
                {
                    match event
                    {
                        qpl::Event::Quit =>
                        {
                            window.should_close = true;
                            break 'event_loop;
                        },
                        _ => {}
                    }
                },
                None =>
                {
                    break 'event_loop;
                }
            }
        }

        // do your update and rendering
    }
}
```

## Supported platforms

### Currently implemented

- Windows
- Linux

### Planned

- Web assembly
- MacOS
- Android
- iOS