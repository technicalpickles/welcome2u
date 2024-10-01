use std::env;
use anyhow::Result;
use anyhow::Context;

use display::MotdSegment;

fn main() -> Result<()> {
    env::set_var("BASE_DIR", ".");
    env::set_var("CONFIG_PATH", "./config.sh");

    let mut segments : Vec<Box<dyn MotdSegment>> = vec![
        Box::<heading::HeadingSegment>::default(),
        Box::<quote::FortuneHeaderSegment>::default(),
        Box::new(<user::UserSegment>::default()),
        Box::new(<os::OsSegment>::default()),
        Box::new(<uptime::UptimeSegment>::default()),
        // Box::new(CommandSegment::new("modules/30-load")),
        Box::new(<memory::MemorySegment>::default()),
        Box::<disk::DiskSegment>::default(),
        // Box::new(<docker::DockerSegment>::default())
    ];

    for segment in segments.iter_mut() {
        segment.prepare().with_context(|| format!("Failed to prepare segment: {:?}", segment))?;
    }

    for segment in segments.iter_mut() {
        segment.render().with_context(|| format!("Failed to render segment: {:?}", segment))?;
    }

    Ok(())
}
