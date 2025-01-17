#[derive(Debug, PartialEq)]
struct TemperatureReading {
    day: u32,
    temperature: f64
}

struct TemperatureAnalyzer {
    readings: Vec<TemperatureReading>,
}

impl TemperatureReading {
    fn new(day: u32, temperature: f64) -> Self {
        Self { day, temperature }
    }
}

impl TemperatureAnalyzer {
    fn new() -> Self {
        Self { readings: Vec::new() }
    }

    fn add_reading(&mut self, reading: TemperatureReading) -> Result<(), String> {
        if self.readings.iter().find(|tr| tr.day == reading.day).is_some() {
            Err("Already has a reading".to_string())
        } else {
            self.readings.push(reading);
            Ok(())
        }
    }

    fn get_average_temperature(&self) -> Option<f64> {
        if self.readings.is_empty() {
            None
        } else {
            let sum = self.readings.iter().map(|tr| tr.temperature).sum::<f64>();
            Some(sum / self.readings.len() as f64)
        }
    }

    fn get_temperature_range(&self) -> Option<(f64, f64)> {
        if self.readings.is_empty() {
            None
        } else {
            let mut min = f64::INFINITY;
            let mut max = f64::NEG_INFINITY;
            self.readings.iter().for_each(|tr| {
                min = min.min(tr.temperature);
                max = max.max(tr.temperature);
            });
            Some((min, max))
        }
    }

    fn get_days_above(&self, threshold: f64) -> Vec<u32> {
        let mut days: Vec<u32> = self.readings.iter()
            .filter(|tr| tr.temperature > threshold)
            .map(|tr| tr.day)
            .collect();

        days.sort();
        days
    }

    fn get_temperature_for_day(&self, day: u32) -> Option<f64> {
        self.readings.iter()
            .find(|tr| tr.day == day)
            .map_or(None, |tr| Some(tr.temperature))
    }

    fn update_temperature(&mut self, day: u32, new_temp: f64) -> Result<(), String> {
        self.readings.iter_mut()
            .find(|tr| tr.day == day)
            .map_or(
                Err("Day not found".to_string()),
                |reading| {
                    reading.temperature = new_temp;
                    Ok(())
                }
            )
    }

    fn remove_outliers(&mut self) {
        if self.readings.is_empty() {
            return;
        }

        let mean = self.get_average_temperature().unwrap();

        let variance = self.readings.iter()
            .map(|tr| {
                let diff = tr.temperature - mean;
                diff * diff
            })
            .sum::<f64>() / self.readings.len() as f64;

        let std_dev = variance.sqrt();
        let threshold = 2.0 * std_dev;

        self.readings.retain(|tr| {
            let diff = (tr.temperature - mean).abs();
            diff <= threshold
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create test data
    fn setup_test_data() -> TemperatureAnalyzer {
        let mut analyzer = TemperatureAnalyzer::new();
        analyzer.add_reading(TemperatureReading::new(1, 20.0)).unwrap();
        analyzer.add_reading(TemperatureReading::new(2, 25.0)).unwrap();
        analyzer.add_reading(TemperatureReading::new(3, 15.0)).unwrap();
        analyzer
    }

    #[test]
    fn test_add_reading() {
        let mut analyzer = TemperatureAnalyzer::new();

        // Test successful addition
        assert!(analyzer.add_reading(TemperatureReading::new(1, 20.5)).is_ok());

        // Test duplicate day
        assert!(analyzer.add_reading(TemperatureReading::new(1, 21.0)).is_err());
    }

    #[test]
    fn test_get_average_temperature() {
        let mut analyzer = TemperatureAnalyzer::new();

        // Test empty analyzer
        assert_eq!(analyzer.get_average_temperature(), None);

        // Test with one reading
        analyzer.add_reading(TemperatureReading::new(1, 20.0)).unwrap();
        assert_eq!(analyzer.get_average_temperature(), Some(20.0));

        // Test with multiple readings
        analyzer.add_reading(TemperatureReading::new(2, 30.0)).unwrap();
        assert_eq!(analyzer.get_average_temperature(), Some(25.0));
    }

    #[test]
    fn test_get_temperature_range() {
        let analyzer = setup_test_data();

        // Test with multiple readings
        assert_eq!(analyzer.get_temperature_range(), Some((15.0, 25.0)));

        // Test empty analyzer
        let empty_analyzer = TemperatureAnalyzer::new();
        assert_eq!(empty_analyzer.get_temperature_range(), None);
    }

    #[test]
    fn test_get_days_above() {
        let analyzer = setup_test_data();

        // Test threshold below all temperatures
        assert_eq!(analyzer.get_days_above(10.0), vec![1, 2, 3]);

        // Test threshold in middle
        assert_eq!(analyzer.get_days_above(20.0), vec![2]);

        // Test threshold above all temperatures
        assert!(analyzer.get_days_above(30.0).is_empty());
    }

    #[test]
    fn test_get_temperature_for_day() {
        let analyzer = setup_test_data();

        // Test existing day
        assert_eq!(analyzer.get_temperature_for_day(1), Some(20.0));

        // Test non-existent day
        assert_eq!(analyzer.get_temperature_for_day(5), None);
    }

    #[test]
    fn test_update_temperature() {
        let mut analyzer = setup_test_data();

        // Test successful update
        assert!(analyzer.update_temperature(1, 22.0).is_ok());
        assert_eq!(analyzer.get_temperature_for_day(1), Some(22.0));

        // Test update non-existent day
        assert!(analyzer.update_temperature(5, 20.0).is_err());
    }

    #[test]
    fn test_remove_outliers() {

    }
}

fn main() {
    println!("Hello, world!");
}
