use crate::get_random_u32;

/// Returns a random integer between u64::MIN and u64::MAX.
pub fn get_random_u64() -> u64 { (get_random_u32() as u64) << 32 | get_random_u32() as u64 }

/// Returns a random uniform f32 between 0.0 and 1.0 (exclusive).
pub fn get_random_f32() -> f32 { (get_random_u32() >> 8) as f32 * f32::EPSILON / 2.0 }

/// Returns a random uniform f64 between 0.0 and 1.0 (exclusive).
pub fn get_random_f64() -> f64 { (get_random_u64() >> 11) as f64 * f64::EPSILON / 2.0 }
