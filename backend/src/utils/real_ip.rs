use crate::services::env::EnvConfig;
use actix_governor::{KeyExtractor, SimpleKeyExtractionError};
use actix_web::dev::ServiceRequest;
use std::net::IpAddr;

#[derive(Debug, Clone, Copy)]
pub struct RealIpKeyExtractor;

impl RealIpKeyExtractor {
    pub fn extract_ip(req: &ServiceRequest) -> Option<IpAddr> {
        if EnvConfig::get().trusted_proxy {
            if let Some(ip) = req
                .headers()
                .get("CF-Connecting-IP")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.parse().ok())
            {
                return Some(ip);
            }

            if let Some(ip) = req
                .headers()
                .get("X-Forwarded-For")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.split(',').next())
                .and_then(|s| s.trim().parse().ok())
            {
                return Some(ip);
            }
        }

        req.peer_addr().map(|addr| addr.ip())
    }
}

impl KeyExtractor for RealIpKeyExtractor {
    type Key = IpAddr;
    type KeyExtractionError = SimpleKeyExtractionError<&'static str>;

    fn extract(&self, req: &ServiceRequest) -> Result<Self::Key, Self::KeyExtractionError> {
        let ip = Self::extract_ip(req).ok_or_else(|| {
            SimpleKeyExtractionError::new("Could not extract IP address from request")
        })?;

        // For IPv6, rate-limit per /56 prefix (matches PeerIpKeyExtractor behaviour)
        Ok(match ip {
            IpAddr::V6(ipv6) => {
                let mut octets = ipv6.octets();
                octets[7..16].fill(0);
                IpAddr::V6(octets.into())
            }
            other => other,
        })
    }
}
