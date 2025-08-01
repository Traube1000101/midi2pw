use pipewire::{context::Context, main_loop::MainLoop, types::ObjectType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let main_loop = MainLoop::new(None)?;
    let context = Context::new(&main_loop)?;
    let core = context.connect(None)?;
    let registry = core.get_registry()?;

    let _listener = registry
        .add_listener_local()
        .global(|global| {
            if global.type_ == ObjectType::Node {
                if let Some(props) = &global.props {
                    let label_unknown: &str = "<Unknown>";
                    let name = props
                        .get("node.nick")
                        .unwrap_or(props.get("node.name").unwrap_or(label_unknown));
                    let media_class = props.get("media.class").unwrap_or(label_unknown);

                    if media_class.contains("Audio") {
                        println!("{:<4}| {:<18} | {}", global.id, name, media_class);
                    }
                }
            }
        })
        .register();

    main_loop.run();

    Ok(())
}
