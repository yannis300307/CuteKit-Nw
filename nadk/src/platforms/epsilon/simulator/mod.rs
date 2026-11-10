use crate::{CalculatorModel, epsilon::Epsilon};

pub mod fs;

impl Epsilon {
	pub fn init() {}

	pub fn get_model() -> CalculatorModel { CalculatorModel::EpsilonSimulator }
}

#[macro_export]
macro_rules! configure_app {
	($app_name: expr, $icon_path: expr) => {};
}
