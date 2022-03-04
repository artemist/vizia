use crate::Color;
use crate::Data;
use crate::Units;

/// A stop in a gradient, defined by a position and a color
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GradientStop {
    // Position of the gradient stop
    // TODO - it doesn't make sense for this to be in Units
    pub position: Units,
    // Colour of the gradient stop
    pub color: Color,
}

impl GradientStop {
    pub fn new(position: Units, color: Color) -> Self {
        Self { position, color }
    }
}

/// The direction of a linear gadient.
#[derive(Debug, Clone, PartialEq, Data)]
pub enum GradientDirection {
    LeftToRight,
    TopToBottom,
}

impl Default for GradientDirection {
    fn default() -> Self {
        GradientDirection::LeftToRight
    }
}

/// Describes a linear gradient
#[derive(Default, Debug, Clone, PartialEq, Data)]
pub struct LinearGradient {
    // Direction of the gradient
    pub direction: GradientDirection,
    // Stops of the gradient
    pub stops: Vec<(f32, Color)>,
}

impl LinearGradient {
    pub fn new(direction: GradientDirection) -> Self {
        Self { direction, stops: Vec::new() }
    }

    pub fn with_stop(mut self, stop: (f32, Color)) -> Self {
        self.stops.push(stop);

        self
    }

    pub fn get_stops(&self, _parent_length: f32) -> &Vec<(f32, Color)> {
        &self.stops
    }
}
