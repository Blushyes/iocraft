use iocraft::prelude::*;
use std::time::Duration;

#[component]
fn TestRunner(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let mut completed_tests = hooks.use_state(|| Vec::<String>::new());
    let mut test_count = hooks.use_state(|| 0);

    // Simulate adding completed tests over time
    hooks.use_future({
        let mut completed_tests = completed_tests.clone();
        let mut test_count = test_count.clone();
        async move {
            for i in 1..=10 {
                smol::Timer::after(Duration::from_millis(300)).await;

                // Add completed test to the static list
                completed_tests
                    .write()
                    .push(format!("✓ Test #{} passed", i));
                test_count.set(i);
            }
        }
    });

    let status_text = if *test_count.read() == 10 {
        "All tests completed! 🎉"
    } else {
        "Running tests..."
    };

    let status_color = if *test_count.read() == 10 {
        Color::Cyan
    } else {
        Color::White
    };

    element! {
        View(flex_direction: FlexDirection::Column) {
            // Static section - shows completed tests that won't be re-rendered
            Static(items: completed_tests.read().clone())

            // Dynamic section - shows current progress
            View(margin_top: 1, padding: 1, border_style: BorderStyle::Round) {
                Text(content: format!("{} ({}/10 completed)", status_text, *test_count.read()), color: status_color, weight: Weight::Bold)
            }
        }
    }
}

fn main() {
    println!("🧪 Static Component Example");
    println!(
        "Watch as tests complete and are permanently displayed above the progress indicator.\n"
    );
    smol::block_on(element!(TestRunner).render_loop()).unwrap();
}
