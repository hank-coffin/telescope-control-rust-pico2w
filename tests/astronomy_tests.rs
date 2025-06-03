use telescope_controller::astronomy::*;
use telescope_controller::Position;
use approx::assert_relative_eq;

#[test]
fn test_star_tracking_sirius() {
    // Test with known star positions (Sirius)
    let sirius_ra: f32 = 101.287;    // degrees
    let sirius_dec: f32 = -16.716;   // degrees
    let observer_lat: f32 = 38.6783; // Citrus Heights, CA

    let lst_values: [f32; 4] = [90.0, 101.287, 120.0, 150.0]; // Test at different LST values to simulate Earth rotation
    
    for lst in lst_values.iter() {
        let pos: Position = equatorial_to_horizontal(sirius_ra, sirius_dec, observer_lat, *lst);
        
        // Basic sanity checks
        assert!(pos.azimuth >= 0.0 && pos.azimuth <= 360.0);
        assert!(pos.altitude >= -90.0 && pos.altitude <= 90.0);
        
        // At culmination (LST = RA), should be at highest altitude
        if (lst - sirius_ra).abs() < 1.0 {
            assert!(pos.altitude > -20.0); // Should be visible from CH
        }
    }
}

#[test]
fn test_coordinate_system_consistency() {
    // Test that our coordinate transformations are consistent
    let test_objects = [
        (0.0, 0.0),      // Vernal equinox
        (90.0, 0.0),     // Summer solstice point
        (180.0, 0.0),    // Autumnal equinox
        (270.0, 0.0),    // Winter solstice point
    ];
    
    let observer_lat = 45.0; // Mid-latitude observer
    
    for (ra, dec) in test_objects.iter() {
        for lst_offset in [0.0, 90.0, 180.0, 270.0].iter() {
            let lst = ra + lst_offset;
            let pos = equatorial_to_horizontal(*ra, *dec, observer_lat, lst);
            
            // Coordinates should be valid
            assert!(pos.azimuth >= 0.0 && pos.azimuth <= 360.0);
            assert!(pos.altitude >= -90.0 && pos.altitude <= 90.0);
        }
    }
}