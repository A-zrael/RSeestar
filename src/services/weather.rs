use crate::model::{DailyForecast, HourlyForecast, WeatherSnapshot};
use serde_json::Value;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub struct WeatherHandle {
    receiver: mpsc::Receiver<Result<WeatherSnapshot, String>>,
}

impl WeatherHandle {
    pub fn try_recv(&self) -> Result<Result<WeatherSnapshot, String>, mpsc::TryRecvError> {
        self.receiver.try_recv()
    }
}

pub fn fetch() -> WeatherHandle {
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        let result = fetch_weather();
        let _ = sender.send(result);
    });
    WeatherHandle { receiver }
}

fn fetch_weather() -> Result<WeatherSnapshot, String> {
    let agent = ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(12)))
        .build()
        .new_agent();
    let mut latitude = 51.5074_f64;
    let mut longitude = -0.1278_f64;
    let mut location = "London, UK (Fallback)".to_string();
    if let Ok(mut response) = agent
        .get("http://ip-api.com/json/?fields=status,lat,lon,city,country")
        .call()
        && let Ok(geo) = response.body_mut().read_json::<Value>()
        && geo["status"] == "success"
    {
        latitude = geo["lat"].as_f64().unwrap_or(latitude);
        longitude = geo["lon"].as_f64().unwrap_or(longitude);
        location = format!(
            "{}, {}",
            geo["city"].as_str().unwrap_or("Unknown"),
            geo["country"].as_str().unwrap_or("Unknown")
        );
    }
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={latitude}&longitude={longitude}&current=temperature_2m,relative_humidity_2m,cloud_cover,wind_speed_10m,visibility&hourly=temperature_2m,cloud_cover,precipitation_probability&daily=temperature_2m_max,temperature_2m_min,precipitation_probability_max&forecast_days=7&timezone=auto"
    );
    let mut response = agent
        .get(&url)
        .call()
        .map_err(|error| format!("Open-Meteo request failed: {error}"))?;
    let data: Value = response
        .body_mut()
        .read_json()
        .map_err(|error| format!("Open-Meteo response was invalid: {error}"))?;
    let current = &data["current"];
    let mut snapshot = WeatherSnapshot {
        location,
        temperature_c: number_f32(&current["temperature_2m"]),
        humidity_percent: number_u8(&current["relative_humidity_2m"]),
        cloud_percent: number_u8(&current["cloud_cover"]),
        wind_kph: number_f32(&current["wind_speed_10m"]),
        visibility_km: number_f32(&current["visibility"]).map(|metres| metres / 1000.0),
        status: "Live atmospheric telemetry, hourly cloud cover, and 7-day forecast updated successfully.".into(),
        ..Default::default()
    };
    let hourly = &data["hourly"];
    let times = hourly["time"].as_array().cloned().unwrap_or_default();
    let temperatures = hourly["temperature_2m"]
        .as_array()
        .cloned()
        .unwrap_or_default();
    let clouds = hourly["cloud_cover"]
        .as_array()
        .cloned()
        .unwrap_or_default();
    for (index, time) in times.iter().enumerate().take(24) {
        snapshot.hourly.push(HourlyForecast {
            time: time
                .as_str()
                .and_then(|time| time.get(11..16))
                .unwrap_or("--:--")
                .into(),
            temperature_c: temperatures.get(index).and_then(number_f32),
            cloud_percent: clouds.get(index).and_then(number_u8),
        });
    }
    let daily = &data["daily"];
    let dates = daily["time"].as_array().cloned().unwrap_or_default();
    let maximums = daily["temperature_2m_max"]
        .as_array()
        .cloned()
        .unwrap_or_default();
    let minimums = daily["temperature_2m_min"]
        .as_array()
        .cloned()
        .unwrap_or_default();
    let rain = daily["precipitation_probability_max"]
        .as_array()
        .cloned()
        .unwrap_or_default();
    for (index, date) in dates.iter().enumerate().take(7) {
        snapshot.daily.push(DailyForecast {
            date: date.as_str().unwrap_or("----").into(),
            maximum_c: maximums.get(index).and_then(number_f32),
            minimum_c: minimums.get(index).and_then(number_f32),
            rain_percent: rain.get(index).and_then(number_u8),
        });
    }
    Ok(snapshot)
}

fn number_f32(value: &Value) -> Option<f32> {
    value.as_f64().map(|value| value as f32)
}

fn number_u8(value: &Value) -> Option<u8> {
    value.as_u64().and_then(|value| u8::try_from(value).ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numeric_helpers_reject_wrong_types_and_overflow() {
        assert_eq!(number_f32(&Value::from(12.5)), Some(12.5));
        assert_eq!(number_u8(&Value::from(75)), Some(75));
        assert_eq!(number_u8(&Value::from(500)), None);
        assert_eq!(number_f32(&Value::from("12.5")), None);
    }
}
