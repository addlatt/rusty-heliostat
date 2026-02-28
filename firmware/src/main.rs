use anyhow::Result;
use chrono::{DateTime, Utc};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::{
        delay::FreeRtos,
        gpio::PinDriver,
        prelude::Peripherals,
    },
    http::server::{Configuration as HttpConfig, EspHttpServer},
    io::Write,
    nvs::EspDefaultNvsPartition,
    ota::EspOta,
    sntp::EspSntp,
    wifi::{AuthMethod, BlockingWifi, ClientConfiguration, Configuration, EspWifi},
};
use log::{info, warn};
use solar_positioning::grena3;
use std::sync::Mutex;

const WIFI_SSID: &str = env!("WIFI_SSID");
const WIFI_PASSWORD: &str = env!("WIFI_PASSWORD");
const HELIOSTAT_LAT: &str = env!("HELIOSTAT_LAT");
const HELIOSTAT_LON: &str = env!("HELIOSTAT_LON");

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;
    let sysloop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    // LED on GPIO 48 (built-in on Arduino Nano ESP32)
    let mut led = PinDriver::output(peripherals.pins.gpio48)?;
    led.set_high()?;
    info!("LED on — board is alive");

    // Connect to WiFi
    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, sysloop.clone(), Some(nvs))?,
        sysloop,
    )?;
    connect_wifi(&mut wifi)?;

    let ip = wifi.wifi().sta_netif().get_ip_info()?.ip;
    info!("WiFi connected! IP: {}", ip);

    // Sync time via NTP (handle stays in scope to keep service running)
    let _sntp = EspSntp::new_default()?;
    info!("SNTP initialised, waiting for time sync...");
    FreeRtos::delay_ms(3000);
    info!("Time sync done (assuming success)");

    // Parse location from env vars
    let lat: f64 = HELIOSTAT_LAT
        .parse()
        .expect("HELIOSTAT_LAT must be a valid f64");
    let lon: f64 = HELIOSTAT_LON
        .parse()
        .expect("HELIOSTAT_LON must be a valid f64");
    info!("Heliostat location: lat={}, lon={}", lat, lon);

    // Mark current OTA slot as valid (rollback protection)
    {
        let mut ota = EspOta::new()?;
        if let Err(e) = ota.mark_running_slot_valid() {
            warn!("Could not mark OTA slot valid (first boot?): {}", e);
        }
    }

    // Start HTTP server
    let server_config = HttpConfig {
        stack_size: 10240,
        ..Default::default()
    };
    let mut server = EspHttpServer::new(&server_config)?;

    // GET / — status page
    server.fn_handler("/", esp_idf_svc::http::Method::Get, move |req| {
        let body = format!(
            "Heliostat firmware OK\nIP: {}\nFree heap: {} bytes\n",
            ip,
            unsafe { esp_idf_svc::sys::esp_get_free_heap_size() }
        );
        req.into_ok_response()?.write_all(body.as_bytes())?;
        Ok::<(), anyhow::Error>(())
    })?;

    // POST /flash — OTA update
    let ota_lock = std::sync::Arc::new(Mutex::new(()));
    server.fn_handler("/flash", esp_idf_svc::http::Method::Post, move |mut req| {
        let _guard = ota_lock.lock().map_err(|_| anyhow::anyhow!("OTA lock poisoned"))?;

        info!("OTA update started");

        let mut ota = EspOta::new()?;
        let mut update = ota.initiate_update()?;

        let mut buf = [0u8; 1024];
        let mut total: usize = 0;

        loop {
            let n = req.read(&mut buf)?;
            if n == 0 {
                break;
            }
            update.write(&buf[..n])?;
            total += n;
            if total % (64 * 1024) == 0 {
                info!("OTA progress: {} KB", total / 1024);
            }
        }

        if total == 0 {
            warn!("OTA: empty payload, aborting");
            update.abort()?;
            return Err(anyhow::anyhow!("Empty OTA payload"));
        }

        info!("OTA complete: {} bytes written, rebooting...", total);
        update.complete()?;

        req.into_ok_response()?.write_all(b"OK, rebooting...\n")?;

        // Give the response time to flush
        FreeRtos::delay_ms(500);
        unsafe { esp_idf_svc::sys::esp_restart() };

        #[allow(unreachable_code)]
        Ok::<(), anyhow::Error>(())
    })?;

    info!("HTTP server running on http://{}:80/", ip);

    // Heartbeat blink loop — also logs sun position every ~60 s
    let mut tick: u32 = 0;
    loop {
        led.toggle()?;
        FreeRtos::delay_ms(1000);
        tick += 1;

        if tick % 60 == 0 {
            let now: DateTime<Utc> = std::time::SystemTime::now().into();
            match grena3::solar_position(now, lat, lon, 69.0, None) {
                Ok(pos) => info!(
                    "Sun — azimuth: {:.1}°, elevation: {:.1}°",
                    pos.azimuth(),
                    pos.elevation_angle()
                ),
                Err(e) => warn!("Solar position error: {}", e),
            }
        }
    }
}

fn connect_wifi(wifi: &mut BlockingWifi<EspWifi<'static>>) -> Result<()> {
    let auth = if WIFI_PASSWORD.is_empty() {
        AuthMethod::None
    } else {
        AuthMethod::WPA2Personal
    };

    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: WIFI_SSID
            .try_into()
            .map_err(|_| anyhow::anyhow!("SSID too long"))?,
        password: WIFI_PASSWORD
            .try_into()
            .map_err(|_| anyhow::anyhow!("Password too long"))?,
        auth_method: auth,
        ..Default::default()
    }))?;

    wifi.start()?;
    info!("WiFi started, scanning...");

    wifi.connect()?;
    info!("WiFi associated");

    wifi.wait_netif_up()?;
    info!("Network interface up");

    Ok(())
}
