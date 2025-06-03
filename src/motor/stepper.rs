//use crate::{Steps, Degrees, MotorPosition, Position};
use crate::{Steps, MotorPosition, Position};

pub struct TelescopeConfig {
    pub steps_per_degree_azimuth: f32,
    pub steps_per_degree_altitude: f32,
    pub azimuth_gear_ratio: f32,
    pub altitude_gear_ratio: f32,
}

impl Default for TelescopeConfig {
    fn default() -> Self {
        Self {
            steps_per_degree_azimuth: 200.0 * 16.0 / 360.0,  // 200 steps/rev, 16x microstepping
            steps_per_degree_altitude: 200.0 * 16.0 / 360.0,
            azimuth_gear_ratio: 100.0,    // 100:1 gearing
            altitude_gear_ratio: 100.0,
        }
    }
}

pub fn position_to_steps(pos: Position, config: &TelescopeConfig) -> MotorPosition {
    let azimuth_steps = (pos.azimuth * config.steps_per_degree_azimuth * config.azimuth_gear_ratio) as Steps;
    let altitude_steps = (pos.altitude * config.steps_per_degree_altitude * config.altitude_gear_ratio) as Steps;
    
    MotorPosition { azimuth_steps, altitude_steps }
}

pub fn steps_to_position(motor_pos: MotorPosition, config: &TelescopeConfig) -> Position {
    let azimuth = motor_pos.azimuth_steps as f32 / (config.steps_per_degree_azimuth * config.azimuth_gear_ratio);
    let altitude = motor_pos.altitude_steps as f32 / (config.steps_per_degree_altitude * config.altitude_gear_ratio);
    
    Position { azimuth, altitude }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_position_conversion_roundtrip() {
        let config = TelescopeConfig::default();
        let original_pos = Position { azimuth: 45.0, altitude: 30.0 };
        
        let motor_pos = position_to_steps(original_pos, &config);
        let converted_back = steps_to_position(motor_pos, &config);
        
        assert_relative_eq!(converted_back.azimuth, original_pos.azimuth, epsilon = 0.1);
        assert_relative_eq!(converted_back.altitude, original_pos.altitude, epsilon = 0.1);
    }

    #[test]
    fn test_zero_position() {
        let config = TelescopeConfig::default();
        let zero_pos = Position { azimuth: 0.0, altitude: 0.0 };
        
        let motor_pos = position_to_steps(zero_pos, &config);
        
        assert_eq!(motor_pos.azimuth_steps, 0);
        assert_eq!(motor_pos.altitude_steps, 0);
    }
}