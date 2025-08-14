use iocraft::prelude::*;
use std::time::Duration;

#[component]
fn Example(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let (stdout, stderr) = hooks.use_output();

    hooks.use_future(async move {
        let mut counter = 0;
        loop {
            smol::Timer::after(Duration::from_secs(1)).await;
            counter += 1;

            // Demonstrate the difference between println and print
            if counter % 3 == 1 {
                stdout.println("Hello from iocraft to stdout!");
                stderr.println("  And hello to stderr too!");
            } else if counter % 3 == 2 {
                stdout.print("Progress: ");
                stdout.print(&format!("{}", counter));
                stdout.println(" (using print + println)");
                stderr.eprint("Error count: ");
                stderr.println("0 (using eprint)");
            } else {
                stdout.print("Loading");
                for _ in 0..3 {
                    smol::Timer::after(Duration::from_millis(200)).await;
                    stdout.print(".");
                }
                stdout.println(" Done!");
            }
        }
    });

    element! {
        View(border_style: BorderStyle::Round, border_color: Color::Green) {
            Text(content: "Hello, use_stdio!")
        }
    }
}

fn main() {
    smol::block_on(element!(Example).render_loop()).unwrap();
}
