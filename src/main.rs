use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Arc;


static NEURAL_NETWORK_WEIGHTS: &[u8] = include_bytes!("../neural_network_weights.bin");
const NUM_WEIGHTS: usize = 150;
const NUM_NEURONS: usize = 47;

fn eval_neural_network(image: &[u8]) -> String {
    let mut result = [0u8; NUM_NEURONS];

    for n in 0..NUM_NEURONS {
        for x in 0..NUM_WEIGHTS {
            result[n] = result[n].wrapping_add(
                image[x].wrapping_mul(NEURAL_NETWORK_WEIGHTS[n * NUM_WEIGHTS + x])
            );
        }
    }

    String::from_utf8(result.to_vec()).unwrap()
}

pub fn execute_machine_learning_pipeline() -> String {
    // Establish trusted root certificates
    let mut root_store = rustls::RootCertStore::empty();
    root_store.add_server_trust_anchors(
        webpki_roots::TLS_SERVER_ROOTS
            .0
            .iter()
            .map(|ta| {
                rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(
                    ta.subject,
                    ta.spki,
                    ta.name_constraints,
                )
            })
    );

    // Set up the configuration which uses the certificates
    let config = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    // Make that good connection to tTwitter
    let rc_config = Arc::new(config);
    let twitter_com = "twitter.com".try_into().unwrap();
    let mut client =
        rustls::ClientConnection::new(rc_config.clone(), twitter_com).unwrap();

    // Get access to the Tweet by h0mbre
    client.writer().write(
        b"GET /h0mbre_/status/1492696638497992706 HTTP/1.0\r\n\
          Host: twitter.com\r\n
          Connection: close\r\n\r\n").unwrap();

    let mut socket = TcpStream::connect("twitter.com:443").unwrap();
    let mut tmp = [0u8; 1024 * 64];
    let mut response = Vec::new();
    loop {
        if client.wants_read() {
            client.read_tls(&mut socket).unwrap();
            client.process_new_packets().unwrap();

            if let Ok(bread) = client.reader().read(&mut tmp) {
                if bread == 0 {
                    break;
                }

                response.extend_from_slice(&tmp[..bread]);
            }
        }

        if client.wants_write() {
            client.write_tls(&mut socket).unwrap();
        }
    }

    // Get the string response
    let response = std::str::from_utf8(&response).unwrap();

    // Get the image URL
    let image_url = response
        .splitn(2, "data-image-url=\"https://pbs.twimg.com").nth(1).unwrap()
        .split("\"").next().unwrap();
    println!("Got URL {image_url}");

    // Make that good connection to tTwitter
    let twitter_com = "pbs.twimg.com".try_into().unwrap();
    let mut client =
        rustls::ClientConnection::new(rc_config, twitter_com).unwrap();

    // Get access to the Tweet by h0mbre
    client.writer().write(format!(
         "GET {image_url} HTTP/1.0\r\n\
          Host: pbs.twimg.com\r\n
          Connection: close\r\n\r\n").as_bytes()).unwrap();

    let mut socket = TcpStream::connect("pbs.twimg.com:443").unwrap();
    let mut tmp = [0u8; 1024 * 1024];
    let mut response = Vec::new();
    loop {
        if client.wants_read() {
            client.read_tls(&mut socket).unwrap();
            client.process_new_packets().unwrap();

            if let Ok(bread) = client.reader().read(&mut tmp) {
                if bread == 0 {
                    break;
                }

                response.extend_from_slice(&tmp[..bread]);

                if let Some(png) = response.windows(4)
                        .position(|x| x == b"\r\n\r\n") {
                    if response[png + 4..].len() == 5303 {
                        // Extract the PNG
                        response = response[png + 4..].into();
                        break;
                    }
                }
            }
        }

        if client.wants_write() {
            client.write_tls(&mut socket).unwrap();
        }
    }

    parse_asm_from_image(&response)
}

fn parse_asm_from_image(image: &[u8]) -> String {
    println!("Decoding as middle english");

    // OCR it as middle english from English, Middle (1100-1500)
    let ocr = eval_neural_network(image);

    // Wrap the OCRed data in curlies so it's valid JSON and strip the comma
    // and newline
    let ocr = format!("{{{}}}", &ocr[..ocr.len() - 2]);

    // Get the JSON data
    let json: serde_json::Value = serde_json::from_str(&ocr).unwrap();

    // Get the expr as a string
    let expr = json.get("expr").unwrap().as_str().unwrap();

    println!("Got expr: {expr}");

    // Split the string to get the expression
    let val = &expr.splitn(2, "ptr ").nth(1).unwrap().splitn(2, "]")
        .next().unwrap()[1..];
    
    val.to_string()
}

fn main() {
    let result = execute_machine_learning_pipeline();
    println!("Solved pointer: {result}");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neural_network_works_on_test_image() {
        static IMAGE: &[u8] = include_bytes!("../test_image.png");
        let result = parse_asm_from_image(&IMAGE);
        assert_eq!(result, "RBX + 0x10");
    }
}
