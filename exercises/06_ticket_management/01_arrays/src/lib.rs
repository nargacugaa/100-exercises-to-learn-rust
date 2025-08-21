// TODO: Flesh out the `WeekTemperatures` struct and its method implementations to pass the tests.

pub struct WeekTemperatures {
    // TODO
    daily_temperature_readings: [DayTemperature; 7],
}

struct DayTemperature {
    day: Weekday,
    temperature: Option<i32>,
}

#[derive(Debug,PartialEq)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl WeekTemperatures {
    pub fn new() -> Self {
        Self {
            daily_temperature_readings: [
                DayTemperature {
                    day: Weekday::Sunday,
                    temperature: None,
                },
                DayTemperature {
                    day: Weekday::Monday,
                    temperature: None,
                },
                DayTemperature {
                    day: Weekday::Tuesday,
                    temperature: None,
                },
                DayTemperature {
                    day: Weekday::Wednesday,
                    temperature: None,
                },
                DayTemperature {
                    day: Weekday::Thursday,
                    temperature: None,
                },
                DayTemperature {
                    day: Weekday::Friday,
                    temperature: None,
                },
                DayTemperature {
                    day: Weekday::Saturday,
                    temperature: None,
                },
            ],
        }
    }

    pub fn get_temperature(&self, day: Weekday) -> Option<i32> {
        for item in self.daily_temperature_readings.iter() {
            if item.day.eq(&day) {
               return item.temperature;
            }
        }
        None
    }

    pub fn set_temperature(&mut self, day: Weekday, temperature: i32) {
        for item in  &mut self.daily_temperature_readings {
            if item.day.eq(&day) {
                item.temperature = Some(temperature)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_temperature() {
        let mut week_temperatures = WeekTemperatures::new();

        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Tuesday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Wednesday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Thursday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Friday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Saturday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Sunday), None);

        week_temperatures.set_temperature(Weekday::Monday, 20);
        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), Some(20));

        week_temperatures.set_temperature(Weekday::Monday, 25);
        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), Some(25));

        week_temperatures.set_temperature(Weekday::Tuesday, 30);
        week_temperatures.set_temperature(Weekday::Wednesday, 35);
        week_temperatures.set_temperature(Weekday::Thursday, 40);
        week_temperatures.set_temperature(Weekday::Friday, 45);
        week_temperatures.set_temperature(Weekday::Saturday, 50);
        week_temperatures.set_temperature(Weekday::Sunday, 55);

        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), Some(25));
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Tuesday),
            Some(30)
        );
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Wednesday),
            Some(35)
        );
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Thursday),
            Some(40)
        );
        assert_eq!(week_temperatures.get_temperature(Weekday::Friday), Some(45));
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Saturday),
            Some(50)
        );
        assert_eq!(week_temperatures.get_temperature(Weekday::Sunday), Some(55));
    }
}
