use std::collections::HashMap;
use gtk::{Fixed, GestureClick, GestureDrag };
use gtk::prelude::{BoxExt, FixedExt, GestureDragExt, WidgetExt};
use crate::math::geometry::aabb::AABB;
use crate::math::vector::Vector;
use crate::providers::display_provider::DisplayProvider;
use crate::types::GTKBox;
use crate::ui::box_builder::BoxBuilder;
use crate::ui::controls::Control;
use crate::ui::controls::display_element::DisplayElement;
use crate::ui::manager::display_configurator_manager::{DisplayConfiguratorEvent, DisplayConfiguratorManager};
use crate::ui::statable_control::StatableControl;
use crate::ui::states::display_configurator_state::DisplayConfiguratorState;
use crate::ui::states::display_element_state::DisplayElementState;
use crate::ui::updatable_control::UpdatableControl;
use crate::utils::RcMut;

pub struct DisplayConfigurator {
    display_provider: RcMut<DisplayProvider>,
    state: DisplayConfiguratorState,
    display_configurator_box: GTKBox,
    display_elements_fixed: Fixed,
    display_elements: HashMap<String, DisplayElement>
}

impl Control for DisplayConfigurator {
    fn init_events(&self) {}

    fn get_widget(&self) -> &GTKBox {
        &self.display_configurator_box
    }
}

impl UpdatableControl<DisplayConfiguratorState> for DisplayConfigurator {
    fn update_ui(&mut self, state: DisplayConfiguratorState) {
        for (monitor_port, state) in state.display_element_states {
            let display_element = self.display_elements.get_mut(&monitor_port).unwrap();
            display_element.update_ui(state.clone());
            self.display_elements_fixed.move_(
                display_element.get_widget(), state.position.get_x(), state.position.get_y()
            );
        }
    }
}

impl StatableControl<DisplayConfiguratorState> for DisplayConfigurator {
    fn update_state(&mut self, state: DisplayConfiguratorState) {
        self.state = state;
    }
}

impl DisplayConfigurator {
    pub fn new(display_provider: RcMut<DisplayProvider>) -> Self {
        let display_configurator_box = BoxBuilder::new("display-configurator")
            .set_margin_top(10)
            .build();

        let display_elements_fixed = Fixed::new();
        display_elements_fixed.set_height_request(500);
        display_configurator_box.append(&display_elements_fixed);

        let display_elements = HashMap::new();
        let state: DisplayConfiguratorState = Default::default();

        Self {
            display_provider,
            state,
            display_configurator_box,
            display_elements_fixed,
            display_elements
        }
    }

    pub fn insert_display_element(&mut self, port_name: String) {
        let display_element = DisplayElement::new();
        self.display_elements_fixed.put(display_element.get_widget(), 0.0, 0.0);
        self.display_elements.insert(port_name.clone(), display_element);
    }

    pub fn select_display_element(&mut self, port_name: Option<String>) {
        self.state.selected_monitor = port_name.clone();
        if let Some(port_name) = port_name {
            if let Some(display_element) = self.display_elements.get(&port_name) {
                display_element.focus();
            }
        }
    }

    pub fn place_display(&mut self, port_name: String, placed_position: Vector) {
        let mut display_configuration_state = self.state.clone();
        let mut current_display_element_state = display_configuration_state
            .display_element_states
            .get(&port_name)
            .unwrap()
            .clone();
        current_display_element_state.position = placed_position.clone();

        display_configuration_state.display_element_states.insert(
            port_name.clone(), current_display_element_state.clone()
        );
        self.update_state(display_configuration_state.clone());

        self.adjust_current_display_element_to_intersecting_closest_one(
            &port_name, &placed_position, &mut current_display_element_state,
        );
        display_configuration_state.display_element_states.insert(
            port_name.clone(), current_display_element_state.clone()
        );
        self.update_state(display_configuration_state.clone());

        let current_display_element_position = current_display_element_state.position;
        self.move_display_element(port_name.clone(), current_display_element_position.clone());

        let mut display_provider_mut = self.display_provider.borrow_mut();
        display_provider_mut.set_monitor_offset(
            port_name.clone(), current_display_element_position.mul_by(10.0)
        );
    }

    fn adjust_current_display_element_to_intersecting_closest_one(
        &mut self, port_name: &String, placed_position: &Vector,
        current_display_element_state: &mut DisplayElementState
    ) {
        let adjacent_display_element_position = self.get_adjacent_position_to_closest_display_element(
          port_name, placed_position, current_display_element_state
        );

        let mut new_position = placed_position.clone();
        if let Some(position) = adjacent_display_element_position {
            // take previous drag position if adjacent position is out of bounds
            new_position = position.clone();
            if position.get_x() < 0.0 || position.get_y() < 0.0 {
                new_position = current_display_element_state.previous_position.clone();
            }
        }

        current_display_element_state.position = new_position.clone();
        current_display_element_state.previous_position = new_position.clone();
    }

    fn get_adjacent_position_to_closest_display_element(
        &self, port_name: &String, placed_position: &Vector,
        current_display_element_state: &DisplayElementState
    ) -> Option<Vector> {
        let concurrent_display_elements = self.get_other_display_element_states(port_name);
        let intersecting_display_elements = self.get_intersecting_display_elements(
            current_display_element_state, &concurrent_display_elements
        );

        let current_display_element_aabb = AABB::new(
            placed_position.get_x(),
            placed_position.get_x() + current_display_element_state.size.get_x(),
            placed_position.get_y(),
            placed_position.get_y() + current_display_element_state.size.get_y()
        );
        let current_display_element_center_position = current_display_element_aabb.get_center_position();

        let concurrent_display_element_aabbs = self.get_concurrent_display_element_aabbs(
            &port_name, &intersecting_display_elements
        );
        let concurrent_display_elements_center_positions = self.get_concurrent_display_elements_center_positions(
            &concurrent_display_element_aabbs
        );
        let concurrent_display_element_distances = self.get_display_element_distances(
            &current_display_element_center_position, &concurrent_display_elements_center_positions
        );

        let mut sorted_distances = concurrent_display_element_distances.iter()
            .map(|(monitor_port, distance)| {
                (monitor_port.clone(), *distance)
            })
            .collect::<Vec<(String, f64)>>();
        sorted_distances.sort_by(|(_, first_distance), (_, second_distance)| {
            first_distance.partial_cmp(second_distance).unwrap()
        });

        let closest_monitor_port = sorted_distances.first();
        if let Some((monitor_port, _)) = closest_monitor_port {
            let closest_aabb = concurrent_display_element_aabbs.get(monitor_port).unwrap();
            let closest_center = closest_aabb.get_center_position();
            let display_element_track = closest_center.sub(&current_display_element_center_position);

            let xdistance = display_element_track.get_x();
            let ydistance = display_element_track.get_y();

            let mut new_position = placed_position.clone();
            if xdistance.abs() > ydistance.abs() {
                if xdistance > 0.0 {
                    new_position.set_x(
                        closest_aabb.get_start_xposition() - current_display_element_aabb.get_width()
                    );
                } else {
                    new_position.set_x(closest_aabb.get_end_xposition())
                }
            } else {
                if ydistance > 0.0 {
                    new_position.set_y(
                        closest_aabb.get_start_yposition() - current_display_element_aabb.get_height()
                    )
                } else {
                    new_position.set_y(closest_aabb.get_end_yposition())
                }
            }

            Some(new_position)
        } else {
            None
        }
    }

    fn get_concurrent_display_element_aabbs(
        &self, port_name: &String, intersecting_display_elements: &HashMap<String, DisplayElementState>
    ) -> HashMap<String, AABB> {
        intersecting_display_elements.iter()
            .filter(|(monitor_port, _)| {
                *monitor_port != port_name
            })
            .map(|(monitor_port, display_element_state)| {
                (monitor_port.clone(), display_element_state.get_aabb())
            })
            .collect::<HashMap<String, AABB>>()
    }

    fn get_other_display_element_states(&self, port_name: &String) -> HashMap<String, DisplayElementState> {
        self.state.display_element_states.iter()
            .filter(|(monitor_port, _)| {
                *monitor_port != port_name
            })
            .map(|(monitor_port, display_element_state)| {
                (monitor_port.clone(), display_element_state.clone())
            })
            .collect()
    }

    fn get_intersecting_display_elements(
        &self, current_display_element_state: &DisplayElementState,
        concurrent_display_element_states: &HashMap<String, DisplayElementState>
    ) -> HashMap<String, DisplayElementState> {
        let current_display_element_aabb = current_display_element_state.get_aabb();

        let mut intersecting_display_elements: HashMap<String, DisplayElementState> = HashMap::new();
        for (monitor_port, display_element_state) in concurrent_display_element_states {
            let concurrent_display_element_aabb = display_element_state.get_aabb();
            if current_display_element_aabb.intersects_with(&concurrent_display_element_aabb) ||
                concurrent_display_element_aabb.intersects_with(&current_display_element_aabb) {
                intersecting_display_elements.insert(monitor_port.clone(), display_element_state.clone());
            }
        }

        intersecting_display_elements
    }

    fn get_concurrent_display_elements_center_positions(
        &self, concurrent_display_elements: &HashMap<String, AABB>
    ) -> HashMap<String, Vector> {
        concurrent_display_elements.iter()
            .map(|(monitor_port, aabb)| {
                (monitor_port.clone(), aabb.get_center_position())
            })
            .collect::<HashMap<String, Vector>>()
    }

    fn get_display_element_distances(
        &self, current_display_element_center_position: &Vector,
        concurrent_display_elements_center_positions: &HashMap<String, Vector>
    ) -> HashMap<String, f64> {
        concurrent_display_elements_center_positions.iter()
            .map(|(monitor_port, center_positions)| {
                let length = center_positions
                    .sub(&current_display_element_center_position)
                    .length();

                (monitor_port.clone(), length)
            })
            .collect::<HashMap<String, f64>>()
    }

    pub fn move_display_element(&mut self, port_name: String, moved_position: Vector) {
        let display_element_state = self.state.display_element_states.get_mut(&port_name);
        if let Some(display_element) = self.display_elements.get_mut(&port_name) {
            let display_element_state = display_element_state.unwrap();
            display_element_state.position = moved_position.clone();
            display_element.update_ui(display_element_state.clone());

            self.display_elements_fixed.move_(display_element.get_widget(), moved_position.get_x(), moved_position.get_y());
        }
    }

    pub fn get_width(&self) -> f64 {
        self.display_elements_fixed.width() as f64
    }

    pub fn get_height(&self) -> f64 {
        self.display_elements_fixed.height() as f64
    }

    pub fn get_size(&self) -> Vector {
        Vector::new(
            self.display_elements_fixed.width() as f64,
            self.display_elements_fixed.height() as f64
        )
    }

    pub fn get_state(&self) -> DisplayConfiguratorState {
        self.state.clone()
    }

    pub fn init_events_by_manager(&mut self, manager: DisplayConfiguratorManager) {
        for (monitor_port, display_element) in &self.display_elements {
            let state = self.state.display_element_states.get(monitor_port);
            if let None = state {
                continue;
            }

            let controller = GestureClick::new();
            let monitor_port_clone = monitor_port.clone();
            let manager_clone = manager.clone();
            controller.connect_pressed(move |_, _, _, _| {
                manager_clone.send_event(DisplayConfiguratorEvent::DisplaySelected(monitor_port_clone.clone()))
            });
            display_element.set_click_controller(controller);
        }

        let manager_clone = manager.clone();
        let controller = GestureDrag::new();
        controller.connect_drag_update(move |_: &GestureDrag, xoffset, yoffset| {
            let display_configuration_state = manager_clone
                .get_display_configurator()
                .borrow()
                .get_state();

            let selected_port = display_configuration_state.selected_monitor;
            if let Some(selected_port) = selected_port {
                let display_element_state = display_configuration_state.display_element_states
                    .get(&selected_port);

                if let Some(display_element_state) = display_element_state {
                    let previous_position = display_element_state.previous_position.clone();
                    manager_clone.send_event(
                        DisplayConfiguratorEvent::DisplayMoving(selected_port, Vector::new(
                            previous_position.get_x() + xoffset,
                            previous_position.get_y() + yoffset
                        ))
                    );
                }
            }
        });

        let manager_clone = manager.clone();
        controller.connect_drag_end(move |_: &GestureDrag, xoffset, yoffset| {
            let display_configuration_state = manager_clone
                .get_display_configurator()
                .borrow()
                .get_state();

            let selected_port = display_configuration_state.selected_monitor;
            if let Some(selected_port) = selected_port {
                let display_element_state = display_configuration_state.display_element_states
                    .get(&selected_port);

                if let Some(display_element_state) = display_element_state {
                    let previous_position = display_element_state.previous_position.clone();
                    let placed_position = Vector::new(
                        previous_position.get_x() + xoffset,
                        previous_position.get_y() + yoffset
                    );

                    manager_clone.send_event(DisplayConfiguratorEvent::DisplayPlaced(
                        selected_port, placed_position,
                    ));
                }
            }
        });
        self.display_elements_fixed.add_controller(controller);
    }
}