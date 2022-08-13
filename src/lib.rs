//Creating a module for Lasagna
pub mod lasagna {
    //Constants for calcuations
    const TOTAL_MINUTES: i8 = 40;
    const LAYER_PREP: i8 = 2;

    pub fn expected_minutes_in_oven() -> i8 {
        TOTAL_MINUTES
    }
    pub fn remaining_minutes_in_oven(min: i8) -> i8 {
        TOTAL_MINUTES - min
    }
    pub fn preparation_time_in_minutes(layers: i8) -> i8 {
        layers * LAYER_PREP
    }
    pub fn elapsed_time_in_minutes(layers: i8, mins_in_oven: i8) -> i8 {
        preparation_time_in_minutes(layers) + mins_in_oven
    }
}

//Creating a module for Assemby Line functions -- inclusing tests
pub mod assembly_line {
    //Constant for car production count
    const CAR_PROD_QUANT: u32 = 221;

    pub fn production_rate_per_hour(prod_speed: u32) -> f64 {
        let mut success_rate: f64 = 1.0;
        if prod_speed >= 5 && prod_speed <= 8 {
            success_rate = 0.9;
        } else if prod_speed >= 9 {
            success_rate = 0.77;
        }
        (prod_speed * CAR_PROD_QUANT) as f64 * success_rate
    }

    pub fn working_items_per_minute(prod_speed: u32) -> u32 {
        (production_rate_per_hour(prod_speed) / 60.0) as u32
    }

    #[cfg(test)]
    #[test]
    fn produces_per_hour() {
        assert_eq!(production_rate_per_hour(6), 1193.4);
        assert_eq!(production_rate_per_hour(4), 884.0);
        assert_eq!(production_rate_per_hour(10), 1701.7);
    }
    #[test]
    fn items_per_minutes() {
        assert_eq!(working_items_per_minute(6), 19);
        assert_eq!(working_items_per_minute(4), 14);
        assert_eq!(working_items_per_minute(10), 28);
    }
}

//Created mod for Semi Structured Logs
pub mod semi_structuted_logs {
    #[derive(Debug)]
    pub enum LogLevel {
        Info,
        Warning,
        Error,
    }

    impl LogLevel {
        pub fn log(log: LogLevel, message: &str) -> String {
            match log {
                LogLevel::Info => LogLevel::info(message),
                LogLevel::Warning => String::from("[WARNING]: ") + message,
                LogLevel::Error => String::from("[ERROR]: ") + message,
            }
        }

        pub fn info(message: &str) -> String {
            String::from("[INFO]: ") + message
        }
    }

    #[cfg(test)]
    #[test]
    fn logs_level() {
        assert_eq!(
            LogLevel::log(LogLevel::Error, &"Stack Overflow"),
            "[ERROR]: Stack Overflow"
        );
        assert_eq!(
            LogLevel::log(LogLevel::Info, &"Timezone changed"),
            "[INFO]: Timezone changed"
        );
        assert_eq!(
            LogLevel::log(LogLevel::Warning, &"Unused variables"),
            "[WARNING]: Unused variables"
        );
    }
}
