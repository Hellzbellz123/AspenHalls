#![allow(dead_code)]
pub mod scroll_bar {
    use bevy::prelude::{Bundle, Color, Commands, Component, Entity, In, Query};
    use kayak_ui::{
        prelude::*,
        widgets::{BackgroundBundle, ClipBundle},
    };

    use crate::kayak_ui::widgets::slider::map_range;

    use super::scroll_context::ScrollContext;

    /// Props used by the [`ScrollBar`] widget
    #[derive(Component, Default, Debug, PartialEq, Clone)]
    pub struct ScrollBarProps {
        /// If true, disables the ability to drag
        pub disabled: bool,
        /// If true, displays a horizontal scrollbar instead of a vertical one
        pub horizontal: bool,
        /// The thickness of the scrollbar in pixels
        pub thickness: f32,
        /// The color of the scrollbar thumb
        pub thumb_color: Option<Color>,
        /// The styles of the scrollbar thumb
        pub thumb_styles: Option<KStyle>,
        /// The color of the scrollbar track
        pub track_color: Option<Color>,
        /// The styles of the scrollbar track
        pub track_styles: Option<KStyle>,
    }

    impl Widget for ScrollBarProps {}

    #[derive(Bundle)]
    pub struct ScrollBarBundle {
        pub scrollbar_props: ScrollBarProps,
        pub styles: KStyle,
        pub computed_styles: ComputedStyles,
        pub widget_name: WidgetName,
    }

    impl Default for ScrollBarBundle {
        fn default() -> Self {
            Self {
                scrollbar_props: Default::default(),
                styles: Default::default(),
                computed_styles: ComputedStyles::default(),
                widget_name: ScrollBarProps::default().get_name(),
            }
        }
    }

    pub fn scroll_bar_render(
        In((widget_context, entity)): In<(KayakWidgetContext, Entity)>,
        mut commands: Commands,
        mut query: Query<(&ScrollBarProps, &KStyle, &mut ComputedStyles)>,
        context_query: Query<&ScrollContext>,
    ) -> bool {
        if let Ok((scrollbar, styles, mut computed_styles)) = query.get_mut(entity) {
            if let Some(context_entity) = widget_context.get_context_entity::<ScrollContext>(entity)
            {
                if let Ok(scroll_context) = context_query.get(context_entity) {
                    let scroll_x = scroll_context.scroll_x();
                    let scroll_y = scroll_context.scroll_y();
                    let content_width = scroll_context.content_width();
                    let content_height = scroll_context.content_height();
                    let scrollable_width = scroll_context.scrollable_width();
                    let scrollable_height = scroll_context.scrollable_height();

                    let layout = widget_context.get_layout(entity).unwrap_or_default();

                    // === Configuration === //
                    // let disabled = scrollbar.disabled;
                    let horizontal = scrollbar.horizontal;
                    let _thickness = scrollbar.thickness;
                    let thickness = scrollbar.thickness;
                    let thumb_color = scrollbar
                        .thumb_color
                        .unwrap_or_else(|| Color::rgba(0.239, 0.258, 0.337, 1.0));
                    let thumb_styles = scrollbar.thumb_styles.clone();
                    let track_color = scrollbar
                        .track_color
                        .unwrap_or_else(|| Color::rgba(0.1581, 0.1758, 0.191, 0.15));
                    let track_styles = scrollbar.track_styles.clone();
                    // The size of the thumb as a percentage
                    let thumb_size_percent = (if scrollbar.horizontal {
                        layout.width / (content_width - thickness).max(1.0)
                    } else {
                        layout.height / (content_height - thickness).max(1.0)
                    })
                    .clamp(0.1, 1.0);
                    // The size of the thumb in pixels
                    let thumb_size_pixels = thumb_size_percent
                        * if scrollbar.horizontal {
                            layout.width
                        } else {
                            layout.height
                        };
                    let thumb_extents = thumb_size_pixels / 2.0;
                    let percent_scrolled = if scrollbar.horizontal {
                        scroll_context.percent_x()
                    } else {
                        scroll_context.percent_y()
                    };
                    // The thumb's offset as a percentage
                    let thumb_offset = map_range(
                        percent_scrolled * 100.0,
                        (0.0, 100.0),
                        (0.0, 100.0 - thumb_size_percent * 100.0),
                    );

                    // === Styles === //
                    *computed_styles = KStyle::default()
                        .with_style(KStyle {
                            render_command: RenderCommand::Layout.into(),
                            width: if horizontal {
                                Units::Stretch(1.0)
                            } else {
                                Units::Pixels(thickness)
                            }
                            .into(),
                            height: if horizontal {
                                Units::Pixels(thickness)
                            } else {
                                Units::Stretch(1.0)
                            }
                            .into(),
                            ..Default::default()
                        })
                        .with_style(styles)
                        .into();

                    let mut track_style =
                        KStyle::default()
                            .with_style(&track_styles)
                            .with_style(KStyle {
                                background_color: track_color.into(),
                                border_radius: Corner::all(thickness / 2.0).into(),
                                ..Default::default()
                            });

                    let mut border_color = thumb_color;
                    #[allow(clippy::single_match)]
                    match &mut border_color {
                        Color::Rgba {
                            red,
                            green,
                            blue,
                            alpha,
                        } => {
                            *alpha = (*alpha - 0.2).max(0.0);
                            *red = (*red + 0.1).min(1.0);
                            *green = (*green + 0.1).min(1.0);
                            *blue = (*blue + 0.1).min(1.0);
                        }
                        _ => {}
                    }

                    let mut thumb_style = KStyle::default()
                        .with_style(KStyle {
                            position_type: KPositionType::SelfDirected.into(),
                            ..Default::default()
                        })
                        .with_style(&thumb_styles)
                        .with_style(KStyle {
                            background_color: thumb_color.into(),
                            border_radius: Corner::all(thickness / 2.0).into(),
                            border: Edge::all(1.0).into(),
                            border_color: border_color.into(),
                            ..Default::default()
                        });

                    if scrollbar.horizontal {
                        track_style.apply(KStyle {
                            height: Units::Pixels(thickness).into(),
                            width: Units::Stretch(1.0).into(),
                            ..Default::default()
                        });
                        thumb_style.apply(KStyle {
                            height: Units::Pixels(thickness).into(),
                            width: Units::Percentage(thumb_size_percent * 100.0).into(),
                            top: Units::Pixels(0.0).into(),
                            left: Units::Percentage(-thumb_offset).into(),
                            ..Default::default()
                        });
                    } else {
                        track_style.apply(KStyle {
                            width: Units::Pixels(thickness).into(),
                            height: Units::Stretch(1.0).into(),
                            ..Default::default()
                        });
                        thumb_style.apply(KStyle {
                            width: Units::Pixels(thickness).into(),
                            height: Units::Percentage(thumb_size_percent * 100.0).into(),
                            top: Units::Percentage(-thumb_offset).into(),
                            left: Units::Pixels(0.0).into(),
                            ..Default::default()
                        });
                    }

                    // === Events === //
                    let on_event = OnEvent::new(
                        move |In((mut event_dispatcher_context, _, mut event, _entity)): In<(
                            EventDispatcherContext,
                            WidgetState,
                            Event,
                            Entity,
                        )>,
                              mut query: Query<&mut ScrollContext>| {
                            if let Ok(mut scroll_context) = query.get_mut(context_entity) {
                                match event.event_type {
                                    EventType::MouseDown(data) => {
                                        // --- Capture Cursor --- //
                                        event_dispatcher_context
                                            .capture_cursor(event.current_target);
                                        scroll_context.start_pos = data.position.into();
                                        scroll_context.is_dragging = true;

                                        // --- Calculate Start Offsets --- //
                                        // Steps:
                                        // 1. Get position relative to this widget
                                        // 2. Convert relative pos to percentage [0-1]
                                        // 3. Multiply by desired scrollable dimension
                                        // 4. Map value to range padded by half thumb_size (both sides)
                                        // 5. Update scroll
                                        let offset: (f32, f32) = if horizontal {
                                            // 1.
                                            let mut x = data.position.0 - layout.posx;
                                            // 2.
                                            x /= layout.width;
                                            // 3.
                                            x *= -scrollable_width;
                                            // 4.
                                            x = map_range(
                                                x,
                                                (-scrollable_width, 0.0),
                                                (-scrollable_width - thumb_extents, thumb_extents),
                                            );
                                            // 5.
                                            scroll_context.set_scroll_x(x);

                                            (x, scroll_y)
                                        } else {
                                            // 1.
                                            let mut y = data.position.1 - layout.posy;
                                            // 2.
                                            y /= layout.height;
                                            // 3.
                                            y *= -scrollable_height;
                                            // 4.
                                            y = map_range(
                                                y,
                                                (-scrollable_height, 0.0),
                                                (-scrollable_height - thumb_extents, thumb_extents),
                                            );
                                            // 5.
                                            scroll_context.set_scroll_y(y);

                                            (scroll_x, y)
                                        };
                                        scroll_context.start_offset = offset.into();
                                    }
                                    EventType::MouseUp(..) => {
                                        // --- Release Cursor --- //
                                        event_dispatcher_context
                                            .release_cursor(event.current_target);
                                        scroll_context.is_dragging = false;
                                    }
                                    EventType::Hover(data) => {
                                        if scroll_context.is_dragging {
                                            // --- Move Thumb --- //
                                            // Positional difference (scaled by thumb size)
                                            let pos_diff = (
                                                (scroll_context.start_pos.x - data.position.0)
                                                    / thumb_size_percent,
                                                (scroll_context.start_pos.y - data.position.1)
                                                    / thumb_size_percent,
                                            );
                                            let start_offset = scroll_context.start_offset;
                                            if horizontal {
                                                scroll_context
                                                    .set_scroll_x(start_offset.x + pos_diff.0);
                                            } else {
                                                scroll_context
                                                    .set_scroll_y(start_offset.y + pos_diff.1);
                                            }
                                        }
                                    }
                                    EventType::Scroll(..) if scroll_context.is_dragging => {
                                        // Prevent scrolling while dragging
                                        // This is a bit of a hack to prevent issues when scrolling while dragging
                                        event.stop_propagation();
                                    }
                                    _ => {}
                                }
                            }

                            (event_dispatcher_context, event)
                        },
                    );

                    let parent_id = Some(entity);
                    rsx! {
                        <BackgroundBundle on_event={on_event} styles={track_style}>
                            <ClipBundle>
                                <BackgroundBundle styles={thumb_style} />
                            </ClipBundle>
                        </BackgroundBundle>
                    };
                }
            }
        }
        true
    }
}
pub mod scroll_box {
    use bevy::prelude::{Bundle, Color, Commands, Component, Entity, In, ParamSet, Query};
    use kayak_ui::{
        prelude::*,
        widgets::{ClipBundle, ElementBundle},
    };

    use crate::kayak_ui::widgets::slider::{
        scroll_bar::{ScrollBarBundle, ScrollBarProps},
        scroll_content::ScrollContentBundle,
    };

    use super::scroll_context::ScrollContext;

    #[derive(Component, Default, Clone, PartialEq)]
    pub struct ScrollBoxProps {
        /// If true, always shows scrollbars even when there's nothing to scroll
        ///
        /// Individual scrollbars can still be hidden via [`hide_horizontal`](Self::hide_horizontal)
        /// and [`hide_vertical`](Self::hide_vertical).
        pub always_show_scrollbar: bool,
        /// If true, disables horizontal scrolling
        pub disable_horizontal: bool,
        /// If true, disables vertical scrolling
        pub disable_vertical: bool,
        /// If true, hides the horizontal scrollbar
        pub hide_horizontal: bool,
        /// If true, hides the vertical scrollbar
        pub hide_vertical: bool,
        /// The thickness of the scrollbar
        pub scrollbar_thickness: Option<f32>,
        /// The step to scroll by when `ScrollUnit::Line`
        pub scroll_line: Option<f32>,
        /// The color of the scrollbar thumb
        pub thumb_color: Option<Color>,
        /// The styles of the scrollbar thumb
        pub thumb_styles: Option<KStyle>,
        /// The color of the scrollbar track
        pub track_color: Option<Color>,
        /// The styles of the scrollbar track
        pub track_styles: Option<KStyle>,
    }

    impl Widget for ScrollBoxProps {}

    #[derive(Bundle)]
    pub struct ScrollBoxBundle {
        pub scroll_box_props: ScrollBoxProps,
        pub styles: KStyle,
        pub computed_styles: ComputedStyles,
        pub children: KChildren,
        pub on_layout: OnLayout,
        pub widget_name: WidgetName,
    }

    impl Default for ScrollBoxBundle {
        fn default() -> Self {
            Self {
                scroll_box_props: Default::default(),
                styles: Default::default(),
                computed_styles: ComputedStyles::default(),
                children: Default::default(),
                on_layout: Default::default(),
                widget_name: ScrollBoxProps::default().get_name(),
            }
        }
    }

    pub fn scroll_box_render(
        In((widget_context, entity)): In<(KayakWidgetContext, Entity)>,
        mut commands: Commands,
        mut query: Query<(
            &ScrollBoxProps,
            &KStyle,
            &mut ComputedStyles,
            &KChildren,
            &mut OnLayout,
        )>,
        mut context_query: ParamSet<(Query<&ScrollContext>, Query<&mut ScrollContext>)>,
    ) -> bool {
        if let Ok((scroll_box, styles, mut computed_styles, scroll_box_children, mut on_layout)) =
            query.get_mut(entity)
        {
            if let Some(context_entity) = widget_context.get_context_entity::<ScrollContext>(entity)
            {
                if let Ok(scroll_context) = context_query.p0().get(context_entity).cloned() {
                    // === Configuration === //
                    let always_show_scrollbar = scroll_box.always_show_scrollbar;
                    let disable_horizontal = scroll_box.disable_horizontal;
                    let disable_vertical = scroll_box.disable_vertical;
                    let hide_horizontal = scroll_box.hide_horizontal;
                    let hide_vertical = scroll_box.hide_vertical;
                    let scrollbar_thickness = scroll_box.scrollbar_thickness.unwrap_or(10.0);
                    let scroll_line = scroll_box.scroll_line.unwrap_or(16.0);
                    let thumb_color = scroll_box.thumb_color;
                    let thumb_styles = scroll_box.thumb_styles.clone();
                    let track_color = scroll_box.track_color;
                    let track_styles = scroll_box.track_styles.clone();

                    let scroll_x = scroll_context.scroll_x();
                    let scroll_y = scroll_context.scroll_y();
                    let scrollable_width = scroll_context.scrollable_width();
                    let scrollable_height = scroll_context.scrollable_height();

                    let hori_thickness = scrollbar_thickness;
                    let vert_thickness = scrollbar_thickness;

                    let hide_horizontal = hide_horizontal
                        || !always_show_scrollbar && scrollable_width < f32::EPSILON;
                    let hide_vertical =
                        hide_vertical || !always_show_scrollbar && scrollable_height < f32::EPSILON;

                    let pad_x = if hide_vertical { 0.0 } else { vert_thickness };
                    let pad_y = if hide_horizontal { 0.0 } else { hori_thickness };

                    if pad_x != scroll_context.pad_x || pad_y != scroll_context.pad_y {
                        if let Ok(mut scroll_context_mut) =
                            context_query.p1().get_mut(context_entity)
                        {
                            scroll_context_mut.pad_x = pad_x;
                            scroll_context_mut.pad_y = pad_y;
                        }
                    }

                    *on_layout = OnLayout::new(
                        move |In((event, _entity)): In<(LayoutEvent, Entity)>,
                              mut query: Query<&mut ScrollContext>| {
                            if event.flags.intersects(
                                GeometryChanged::WIDTH_CHANGED | GeometryChanged::HEIGHT_CHANGED,
                            ) {
                                if let Ok(mut scroll) = query.get_mut(context_entity) {
                                    scroll.scrollbox_width = event.layout.width;
                                    scroll.scrollbox_height = event.layout.height;
                                }
                            }

                            event
                        },
                    );

                    // === Styles === //
                    *computed_styles = KStyle::default()
                        .with_style(KStyle {
                            render_command: RenderCommand::Layout.into(),
                            ..Default::default()
                        })
                        .with_style(styles)
                        .with_style(KStyle {
                            width: Units::Stretch(1.0).into(),
                            height: Units::Stretch(1.0).into(),
                            ..Default::default()
                        })
                        .into();

                    let hbox_styles = KStyle::default().with_style(KStyle {
                        render_command: RenderCommand::Layout.into(),
                        layout_type: LayoutType::Row.into(),
                        width: Units::Stretch(1.0).into(),
                        ..Default::default()
                    });
                    let vbox_styles = KStyle::default().with_style(KStyle {
                        render_command: RenderCommand::Layout.into(),
                        layout_type: LayoutType::Column.into(),
                        width: Units::Stretch(1.0).into(),
                        ..Default::default()
                    });

                    let content_styles = KStyle::default().with_style(KStyle {
                        position_type: KPositionType::SelfDirected.into(),
                        top: Units::Pixels(scroll_y).into(),
                        left: Units::Pixels(scroll_x).into(),
                        ..Default::default()
                    });

                    let event_handler = OnEvent::new(
                        move |In((event_dispatcher_context, _, mut event, _entity)): In<(
                            EventDispatcherContext,
                            WidgetState,
                            Event,
                            Entity,
                        )>,
                              mut query: Query<&mut ScrollContext>| {
                            if let Ok(mut scroll_context) = query.get_mut(context_entity) {
                                #[allow(clippy::single_match)]
                                match event.event_type {
                                    EventType::Scroll(evt) => {
                                        match evt.delta {
                                            ScrollUnit::Line { x, y } => {
                                                if !disable_horizontal {
                                                    scroll_context
                                                        .set_scroll_x(scroll_x - x * scroll_line);
                                                }
                                                if !disable_vertical {
                                                    scroll_context
                                                        .set_scroll_y(scroll_y + y * scroll_line);
                                                }
                                            }
                                            ScrollUnit::Pixel { x, y } => {
                                                if !disable_horizontal {
                                                    scroll_context.set_scroll_x(scroll_x - x);
                                                }
                                                if !disable_vertical {
                                                    scroll_context.set_scroll_y(scroll_y + y);
                                                }
                                            }
                                        }
                                        event.stop_propagation();
                                    }
                                    _ => {}
                                }
                            }
                            (event_dispatcher_context, event)
                        },
                    );

                    let parent_id = Some(entity);
                    rsx! {
                        <ElementBundle on_event={event_handler} styles={hbox_styles}>
                            <ElementBundle styles={vbox_styles}>
                                <ClipBundle>
                                    <ScrollContentBundle
                                        children={scroll_box_children.clone()}
                                        styles={content_styles}
                                    />
                                </ClipBundle>
                                {if !hide_horizontal {
                                    constructor! {
                                        <ScrollBarBundle
                                            scrollbar_props={ScrollBarProps {
                                                disabled: disable_horizontal,
                                                horizontal: true,
                                                thickness: hori_thickness,
                                                thumb_color,
                                                thumb_styles: thumb_styles.clone(),
                                                track_color,
                                                track_styles: track_styles.clone(),
                                            }}
                                        />
                                    }
                                }}
                            </ElementBundle>
                            {if !hide_vertical {
                                constructor! {
                                    <ScrollBarBundle
                                        scrollbar_props={ScrollBarProps {
                                            disabled: disable_vertical,
                                            thickness: hori_thickness,
                                            thumb_color,
                                            thumb_styles,
                                            track_color,
                                            track_styles,
                                            ..Default::default()
                                        }}
                                    />
                                }
                            }}
                        </ElementBundle>
                    };
                }
            }
        }
        true
    }
}
pub mod scroll_content {
    use bevy::prelude::{Bundle, Component, Entity, In, Query, With};
    use kayak_ui::prelude::*;

    use super::scroll_context::ScrollContext;

    #[derive(Component, Default, PartialEq, Eq, Clone)]
    pub struct ScrollContentProps;

    impl Widget for ScrollContentProps {}

    #[derive(Bundle)]
    pub struct ScrollContentBundle {
        pub scroll_content_props: ScrollContentProps,
        pub styles: KStyle,
        pub computed_styles: ComputedStyles,
        pub children: KChildren,
        pub on_layout: OnLayout,
        pub widget_name: WidgetName,
    }

    impl Default for ScrollContentBundle {
        fn default() -> Self {
            Self {
                scroll_content_props: Default::default(),
                styles: Default::default(),
                computed_styles: ComputedStyles::default(),
                children: Default::default(),
                on_layout: Default::default(),
                widget_name: ScrollContentProps::default().get_name(),
            }
        }
    }

    pub fn scroll_content_render(
        In((widget_context, entity)): In<(KayakWidgetContext, Entity)>,
        mut query: Query<
            (&KStyle, &mut ComputedStyles, &KChildren, &mut OnLayout),
            With<ScrollContentProps>,
        >,
        context_query: Query<&ScrollContext>,
    ) -> bool {
        if let Ok((styles, mut computed_styles, children, mut on_layout)) = query.get_mut(entity) {
            if let Some(context_entity) = widget_context.get_context_entity::<ScrollContext>(entity)
            {
                if let Ok(scroll_context) = context_query.get(context_entity) {
                    // === OnLayout === //
                    *on_layout = OnLayout::new(
                        move |In((event, _entity)): In<(LayoutEvent, Entity)>,
                              mut query: Query<&mut ScrollContext>| {
                            if event.flags.intersects(
                                GeometryChanged::WIDTH_CHANGED | GeometryChanged::HEIGHT_CHANGED,
                            ) {
                                if let Ok(mut scroll) = query.get_mut(context_entity) {
                                    scroll.content_width = event.layout.width;
                                    scroll.content_height = event.layout.height;
                                }
                            }

                            event
                        },
                    );

                    // === Styles === //
                    *computed_styles = KStyle::default()
                        .with_style(KStyle {
                            render_command: RenderCommand::Layout.into(),
                            layout_type: LayoutType::Column.into(),
                            min_width: Units::Pixels(
                                scroll_context.scrollbox_width - scroll_context.pad_x - 10.0,
                            )
                            .into(),
                            min_height: Units::Stretch(
                                scroll_context.scrollbox_height - scroll_context.pad_y,
                            )
                            .into(),
                            width: Units::Auto.into(),
                            height: Units::Auto.into(),
                            ..Default::default()
                        })
                        .with_style(styles)
                        .into();

                    children.process(&widget_context, Some(entity));
                }
            }
        }

        true
    }
}
pub mod scroll_context {
    use bevy::prelude::{Bundle, Commands, Component, Entity, In, Query, Vec2};
    use kayak_ui::prelude::*;

    /// Context data provided by a [`ScrollBox`](crate::ScrollBox) widget
    #[derive(Component, Default, Debug, Copy, Clone, PartialEq)]
    pub struct ScrollContext {
        pub(super) scroll_x: f32,
        pub(super) scroll_y: f32,
        pub(super) content_width: f32,
        pub(super) content_height: f32,
        pub(super) scrollbox_width: f32,
        pub(super) scrollbox_height: f32,
        pub(super) pad_x: f32,
        pub(super) pad_y: f32,
        pub(super) mode: ScrollMode,
        pub(super) is_dragging: bool,
        pub(super) start_pos: Vec2,
        pub(super) start_offset: Vec2,
    }

    #[non_exhaustive]
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum ScrollMode {
        /// Clamps the scroll offset to stay within the scroll range
        Clamped,
        /// Allows infinite scrolling
        Infinite,
    }

    impl Default for ScrollMode {
        fn default() -> Self {
            ScrollMode::Clamped
        }
    }

    impl ScrollContext {
        /// Get the current x-axis scroll offset
        pub fn scroll_x(&self) -> f32 {
            self.scroll_x
        }

        /// Get the current y-axis scroll offset
        pub fn scroll_y(&self) -> f32 {
            self.scroll_y
        }

        /// The width of the content
        pub fn content_width(&self) -> f32 {
            if self.content_width > self.scrollbox_width {
                self.content_width + self.pad_x
            } else {
                self.content_width
            }
        }

        /// The height of the content
        pub fn content_height(&self) -> f32 {
            if self.content_height > self.scrollbox_height {
                self.content_height + self.pad_y
            } else {
                self.content_height
            }
        }

        /// The total amount that can be scrolled along the x-axis
        pub fn scrollable_width(&self) -> f32 {
            (self.content_width() - self.scrollbox_width).max(0.0)
        }

        /// The total amount that can be scrolled along the y-axis
        pub fn scrollable_height(&self) -> f32 {
            (self.content_height() - self.scrollbox_height).max(0.0)
        }

        /// The current scroll mode
        pub fn mode(&self) -> ScrollMode {
            self.mode
        }

        /// Set the scroll offset along the x-axis
        ///
        /// This automatically accounts for the scroll mode
        pub fn set_scroll_x(&mut self, x: f32) {
            let min = -self.scrollable_width();
            self.scroll_x = match self.mode {
                ScrollMode::Clamped => ScrollContext::clamped(x, min, 0.0),
                ScrollMode::Infinite => x,
            }
        }

        /// Set the scroll offset along the y-axis
        ///
        /// This automatically accounts for the scroll mode
        pub fn set_scroll_y(&mut self, y: f32) {
            let min = -self.scrollable_height();
            self.scroll_y = match self.mode {
                ScrollMode::Clamped => ScrollContext::clamped(y, min, 0.0),
                ScrollMode::Infinite => y,
            };
        }

        /// The percent scrolled along the x-axis
        pub fn percent_x(&self) -> f32 {
            let width = self.scrollable_width();
            if width <= f32::EPSILON {
                // Can't divide by zero
                0.0
            } else {
                self.scroll_x / width
            }
        }

        /// The percent scrolled along the y-axis
        pub fn percent_y(&self) -> f32 {
            let height = self.scrollable_height();
            if height <= f32::EPSILON {
                // Can't divide by zero
                0.0
            } else {
                self.scroll_y / height
            }
        }

        /// Clamps a given value between a range
        fn clamped(value: f32, min: f32, max: f32) -> f32 {
            value.clamp(min, max)
        }
    }

    #[derive(Component, Default, PartialEq, Clone)]
    pub struct ScrollContextProvider {
        initial_value: ScrollContext,
    }

    impl Widget for ScrollContextProvider {}

    #[derive(Bundle)]
    pub struct ScrollContextProviderBundle {
        pub scroll_context_provider: ScrollContextProvider,
        pub children: KChildren,
        pub styles: KStyle,
        pub computed_styles: ComputedStyles,
        pub widget_name: WidgetName,
    }

    impl Default for ScrollContextProviderBundle {
        fn default() -> Self {
            Self {
                scroll_context_provider: Default::default(),
                children: KChildren::default(),
                styles: Default::default(),
                computed_styles: ComputedStyles::default(),
                widget_name: ScrollContextProvider::default().get_name(),
            }
        }
    }

    pub fn scroll_context_render(
        In((widget_context, entity)): In<(KayakWidgetContext, Entity)>,
        mut commands: Commands,
        mut query: Query<(
            &ScrollContextProvider,
            &KChildren,
            &KStyle,
            &mut ComputedStyles,
        )>,
    ) -> bool {
        if let Ok((context_provider, children, styles, mut computed_styles)) = query.get_mut(entity)
        {
            let context_entity = commands.spawn(context_provider.initial_value).id();
            widget_context.set_context_entity::<ScrollContext>(Some(entity), context_entity);
            *computed_styles = styles.clone().into();
            children.process(&widget_context, Some(entity));
        }

        true
    }
}

/// Maps a value from one range to another range
fn map_range(value: f32, from_range: (f32, f32), to_range: (f32, f32)) -> f32 {
    let from_diff = from_range.1 - from_range.0;
    if from_diff <= f32::EPSILON {
        value
    } else {
        to_range.0 + (value - from_range.0) * (to_range.1 - to_range.0) / from_diff
    }
}
