use core::str;

use bsc::wifi::wifi;
use embedded_svc::{
    http::{
        client::{Client, Request, RequestWrite, Response},
        Headers, Status,
    },
    io::Read,
};
use esp32_c3_dkc02_bsc as bsc;
use esp_idf_svc::http::client::{EspHttpClient, EspHttpClientConfiguration, EspHttpRequest};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let _wifi = wifi(CONFIG.wifi_ssid, CONFIG.wifi_psk)?;

    // TODO your code here
    //get(...)?;
    let url = String::from("http://neverssl.com/");
    get(url);

    Ok(())
}

fn get(url: impl AsRef<str>) -> anyhow::Result<()> {
    let client = match EspHttpClient::new(&EspHttpClientConfiguration::default()) {
        Ok(http_client) => http_client,
        Err(error) => panic!("Problem ceating EspHttpClient: {:?}", error),
    };
    // 1. Create a new EspHttpClient. (Check documentation)

    // 2. Open a GET request to `url`
    let request = match client.get(url.as_ref()) {
        Ok(request) => request,
        Err(error) => panic!("Problem creating EspHttpRequest: {:?}", error),
    };

    // 3. Requests *may* send data to the server. Turn the request into a writer, specifying 0 bytes as write length
    // (since we don't send anything - but have to do the writer step anyway)
    //
    // https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-reference/protocols/esp_http_client.html
    // If this were a POST request, you'd set a write length > 0 and then writer.do_write(&some_buf);

    // let writer = request...;

    let writer = match request.into_writer(0) {
        Ok(writer) => writer,
        Err(error) => panic!("Problem converting into writer : {:?}", error),
    };

    // 4. Turn the writer into a response and check its status. Successful http status codes are in the 200..=299 range.

    // let response = writer...;
    // let status = ...;
    // println!("response code: {}\n", status);

    let response = match writer.submit() {
        Ok(response) => response,
        Err(error) => panic!("Problem converting into response : {:?}", error),
    };

    match response.status() {
        400 => {
            let mut buf: [u8; 4096];
            let mut string = String::new();
            let reader = response.reader();
            while match reader.read(&mut buf) {
                Ok(size) => size > 0,
                Err(error) => {
                    println!("Problem with reading response : {:?}", error);
                    false
                }
            } {
                print!("{:?}", buf);
                string.push_str(&String::from_utf8_lossy(&buf));
            }
            print!("{}", string)
        }
        _ => (),
    };
    // 5. If the status is OK, read response data chunk by chunk into a buffer and print it until done.
    // 6. Try converting the bytes into a Rust (UTF-8) string and print it.

    Ok(())
}
