use crate::{Degrees, Radians, Position};
//use libm::{sinf, cosf, atan2, asin, sqrt};
use libm::{sinf, cosf, atan2f, asinf};

pub const PI: f32 = 3.14159265359;

pub fn degrees_to_radians(degrees: Degrees) -> Radians {
    degrees * PI / 180.0
}

pub fn radians_to_degrees(radians: Radians) -> Degrees {
    radians * 180.0 / PI
}

/// Convert equatorial coordinates (RA, Dec) to horizontal (Alt, Az)
pub fn equatorial_to_horizontal(
    ra: Degrees,           // Right Ascension
    dec: Degrees,          // Declination  
    lat: Degrees,          // Observer latitude
    lst: Degrees,          // Local Sidereal Time
) -> Position {
    let ra_rad: f32  = degrees_to_radians(ra);
    let dec_rad: f32 = degrees_to_radians(dec);
    let lat_rad: f32 = degrees_to_radians(lat);
    let lst_rad: f32 = degrees_to_radians(lst);
    
    // Hour angle
    let ha: f32 = lst_rad - ra_rad;
    
    // Calculate altitude
    let sin_alt: f32 = sinf(dec_rad) * sinf(lat_rad) + cosf(dec_rad) * cosf(lat_rad) * cosf(ha);
    let altitude: f32 = radians_to_degrees(asinf(sin_alt));
    
    // Calculate azimuth
    let y:f32 = -cosf(dec_rad) * cosf(lat_rad) * sinf(ha);
    let x:f32 = sinf(dec_rad) - sinf(lat_rad) * sin_alt;
    let azimuth = radians_to_degrees(atan2f(y, x)) + 180.0;
    
    Position { azimuth, altitude }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_degree_radian_conversion() {
        assert_relative_eq!(degrees_to_radians(180.0), PI, epsilon = 1e-6);
        assert_relative_eq!(radians_to_degrees(PI), 180.0, epsilon = 1e-6);
        assert_relative_eq!(degrees_to_radians(90.0), PI / 2.0, epsilon = 1e-6);
    }

    #[test]
    fn test_equatorial_to_horizontal_polaris() {
        // Polaris (roughly at celestial north pole)
        // Should have altitude ≈ observer latitude
        let ra = 37.95;     // Polaris RA (degrees)
        let dec = 89.26;    // Polaris Dec (degrees)  
        let lat = 40.0;     // Observer at 40°N
        let lst = 37.95;    // LST = RA for culmination
        
        let pos = equatorial_to_horizontal(ra, dec, lat, lst);
        
        // At culmination, Polaris altitude should be close to latitude
        assert_relative_eq!(pos.altitude, lat, epsilon = 2.0);
    }

    #[test]
    fn test_equatorial_to_horizontal_zenith() {
        // Object at zenith should have altitude = 90°
        let lat = 40.0;
        let dec = lat;      // Declination = latitude
        let ra = 100.0;     
        let lst = ra;       // At culmination (LST = RA)
        
        let pos = equatorial_to_horizontal(ra, dec, lat, lst);
        
        assert_relative_eq!(pos.altitude, 90.0, epsilon = 1.0);
    }
}