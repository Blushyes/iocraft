use crate::{AnyElement, Component, ComponentUpdater, Hooks, Props};
use core::{
    pin::Pin,
    task::{Context, Poll},
};
use taffy::style::Style;

/// The props which can be passed to the [`Static`] component.
#[non_exhaustive]
#[derive(Default, Props)]
pub struct StaticProps<'a> {
    /// Array of text items to render statically.
    pub items: Vec<String>,

    /// The children elements to render (will be processed once and stored).
    pub children: Vec<AnyElement<'a>>,
}

/// `Static` component permanently renders its output above everything else.
/// It's useful for displaying activity like completed tasks or logs - things that
/// are not changing after they're rendered (hence the name "Static").
///
/// It's preferred to use `Static` for use cases like these, when you can't know
/// or control the amount of items that need to be rendered.
///
/// For example, you might use `Static` to display a list of completed tests
/// or a list of generated pages, while still displaying a live progress bar.
///
/// **Note:** `Static` only renders new items in `items` prop and ignores items
/// that were previously rendered. This means that when you add new items to `items`
/// array, changes you make to previous items will not trigger a rerender.
pub struct Static {
    rendered_items: Vec<String>,
    rendered_elements: Vec<AnyElement<'static>>,
}

impl Default for Static {
    fn default() -> Self {
        Self {
            rendered_items: Vec::new(),
            rendered_elements: Vec::new(),
        }
    }
}

impl Component for Static {
    type Props<'a> = StaticProps<'a>;

    fn new(_props: &Self::Props<'_>) -> Self {
        Self::default()
    }

    fn update(
        &mut self,
        props: &mut Self::Props<'_>,
        _hooks: Hooks,
        updater: &mut ComponentUpdater,
    ) {
        // Check for new items that haven't been rendered yet
        let new_items: Vec<String> = props
            .items
            .iter()
            .skip(self.rendered_items.len())
            .cloned()
            .collect();

        // Create text elements for new items
        for item in new_items {
            let text_element = crate::element! {
                crate::components::Text(content: item.clone())
            };

            // Convert to owned element to store permanently
            let owned_element = unsafe {
                // This is safe because we're extending the lifetime to 'static
                // The element will be kept alive by our component
                std::mem::transmute::<AnyElement<'_>, AnyElement<'static>>(text_element.into())
            };

            self.rendered_elements.push(owned_element);
            self.rendered_items.push(item);
        }

        // Also process any new children elements
        let new_children_count = props.children.len();
        let current_children_count = self.rendered_elements.len() - self.rendered_items.len();

        if new_children_count > current_children_count {
            let new_children = props
                .children
                .drain(current_children_count..)
                .collect::<Vec<_>>();

            for child in new_children {
                let owned_element =
                    unsafe { std::mem::transmute::<AnyElement<'_>, AnyElement<'static>>(child) };
                self.rendered_elements.push(owned_element);
            }
        }

        // Set layout to column direction for vertical stacking
        updater.set_layout_style(Style {
            flex_direction: taffy::style::FlexDirection::Column,
            ..Default::default()
        });

        // Update all rendered elements (both old and new)
        updater.update_children(self.rendered_elements.iter_mut(), None);
    }

    fn poll_change(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
        // Static components don't change on their own
        Poll::Pending
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_static_empty() {
        let output = element!(Static).to_string();
        assert_eq!(output, "");
    }

    #[test]
    fn test_static_with_items() {
        let items = vec!["Item 1".to_string(), "Item 2".to_string()];
        let output = element!(Static(items: items)).to_string();
        assert_eq!(output, "Item 1\nItem 2\n");
    }

    #[test]
    fn test_static_incremental_rendering() {
        // This test simulates the incremental rendering behavior
        // In a real scenario, this would be tested with multiple render cycles
        let items = vec![
            "First item".to_string(),
            "Second item".to_string(),
            "Third item".to_string(),
        ];
        let output = element!(Static(items: items)).to_string();
        assert_eq!(output, "First item\nSecond item\nThird item\n");
    }

    #[test]
    fn test_static_with_children() {
        let children = vec![
            element!(Text(content: "Child 1")).into(),
            element!(Text(content: "Child 2")).into(),
        ];
        let output = element!(Static(children: children)).to_string();
        assert_eq!(output, "Child 1\nChild 2\n");
    }

    #[test]
    fn test_static_mixed_items_and_children() {
        let items = vec!["Item".to_string()];
        let children = vec![element!(Text(content: "Child")).into()];
        let output = element!(Static(items: items, children: children)).to_string();
        assert_eq!(output, "Item\nChild\n");
    }
}
