//! Network speed test module

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::Instant;

/// Speed test results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeedTestResult {
    /// Download speed in Mbps
    pub download_mbps: f64,
    /// Upload speed in Mbps
    pub upload_mbps: f64,
    /// Ping latency in milliseconds
    pub ping_ms: f64,
    /// Jitter in milliseconds
    pub jitter_ms: f64,
    /// Test server location
    pub server_location: String,
    /// Test timestamp
    pub timestamp: std::time::SystemTime,
}

/// Speed test progress callback
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpeedTestPhase {
    /// Testing ping/latency
    Ping,
    /// Testing download speed
    Download(u8), // Progress 0-100
    /// Testing upload speed
    Upload(u8), // Progress 0-100
    /// Test complete
    Complete,
}

impl SpeedTestPhase {
    /// Get the phase name
    pub fn name(&self) -> &str {
        match self {
            Self::Ping => "Measuring Latency",
            Self::Download(_) => "Testing Download Speed",
            Self::Upload(_) => "Testing Upload Speed",
            Self::Complete => "Test Complete",
        }
    }

    /// Get the progress percentage (0-100)
    pub fn progress(&self) -> u8 {
        match self {
            Self::Ping => 10,
            Self::Download(p) => 10 + (p / 2),  // 10-60
            Self::Upload(p) => 60 + (p / 2),    // 60-100
            Self::Complete => 100,
        }
    }
}

/// Speed test configuration
#[derive(Debug, Clone)]
pub struct SpeedTestConfig {
    /// Test server URL (default: a reliable speed test server)
    pub server_url: String,
    /// Number of ping tests to perform
    pub ping_count: usize,
    /// Download test duration in seconds
    pub download_duration_secs: u64,
    /// Upload test duration in seconds
    pub upload_duration_secs: u64,
    /// Download test file size in bytes
    pub download_size: usize,
    /// Upload test file size in bytes
    pub upload_size: usize,
}

impl Default for SpeedTestConfig {
    fn default() -> Self {
        Self {
            server_url: "https://speed.cloudflare.com".to_string(),
            ping_count: 10,
            download_duration_secs: 10,
            upload_duration_secs: 10,
            download_size: 10 * 1024 * 1024, // 10 MB
            upload_size: 1024 * 1024,    // 1 MB
        }
    }
}

/// Speed test manager
pub struct SpeedTestManager {
    config: SpeedTestConfig,
    last_result: Option<SpeedTestResult>,
    is_running: bool,
}

impl SpeedTestManager {
    /// Create a new speed test manager
    pub fn new() -> Self {
        Self {
            config: SpeedTestConfig::default(),
            last_result: None,
            is_running: false,
        }
    }

    /// Create with custom configuration
    pub fn with_config(config: SpeedTestConfig) -> Self {
        Self {
            config,
            last_result: None,
            is_running: false,
        }
    }

    /// Check if a speed test is currently running
    pub fn is_running(&self) -> bool {
        self.is_running
    }

    /// Get the last speed test result
    pub fn last_result(&self) -> Option<&SpeedTestResult> {
        self.last_result.as_ref()
    }

    /// Run a speed test
    pub async fn run_test<F>(&mut self, mut progress_callback: F) -> Result<SpeedTestResult>
    where
        F: FnMut(SpeedTestPhase),
    {
        if self.is_running {
            return Err(anyhow::anyhow!("Speed test already running"));
        }

        self.is_running = true;
        tracing::info!("Starting speed test...");

        // Phase 1: Ping test
        progress_callback(SpeedTestPhase::Ping);
        let ping_ms = self.test_ping().await?;
        let jitter_ms = self.calculate_jitter().await?;

        // Phase 2: Download test
        let mut download_mbps: f64 = 0.0;
        for i in 0..10 {
            progress_callback(SpeedTestPhase::Download((i * 10) as u8));
            let speed = self.test_download().await?;
            download_mbps = download_mbps.max(speed);
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }

        // Phase 3: Upload test
        let mut upload_mbps: f64 = 0.0;
        for i in 0..10 {
            progress_callback(SpeedTestPhase::Upload((i * 10) as u8));
            let speed = self.test_upload().await?;
            upload_mbps = upload_mbps.max(speed);
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }

        progress_callback(SpeedTestPhase::Complete);

        let result = SpeedTestResult {
            download_mbps,
            upload_mbps,
            ping_ms,
            jitter_ms,
            server_location: self.get_server_location().await?,
            timestamp: std::time::SystemTime::now(),
        };

        self.last_result = Some(result.clone());
        self.is_running = false;

        tracing::info!(
            "Speed test complete: Down={:.2} Mbps, Up={:.2} Mbps, Ping={:.2} ms",
            result.download_mbps,
            result.upload_mbps,
            result.ping_ms
        );

        Ok(result)
    }

    /// Test ping/latency
    async fn test_ping(&self) -> Result<f64> {
        tracing::debug!("Testing ping...");
        
        // Note: In a full implementation, this would:
        // 1. Make multiple HTTP HEAD requests to a test server
        // 2. Measure round-trip time for each request
        // 3. Calculate average latency
        
        // Simulate ping test
        let mut total_ms = 0.0;
        for _ in 0..self.config.ping_count {
            let start = Instant::now();
            // Simulate network request
            tokio::time::sleep(tokio::time::Duration::from_millis(20)).await;
            let elapsed = start.elapsed().as_secs_f64() * 1000.0;
            total_ms += elapsed;
        }
        
        let avg_ms = total_ms / self.config.ping_count as f64;
        Ok(avg_ms)
    }

    /// Calculate jitter
    async fn calculate_jitter(&self) -> Result<f64> {
        // Note: In a full implementation, this would calculate
        // the variance in ping times
        Ok(5.0) // Simulated jitter
    }

    /// Test download speed
    async fn test_download(&self) -> Result<f64> {
        // Note: In a full implementation, this would:
        // 1. Download a file of known size from test server
        // 2. Measure time taken
        // 3. Calculate speed in Mbps
        
        // Simulate download speed test
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        // Simulate varying download speeds (50-150 Mbps)
        let speed = 50.0 + (rand::random::<f64>() * 100.0);
        Ok(speed)
    }

    /// Test upload speed
    async fn test_upload(&self) -> Result<f64> {
        // Note: In a full implementation, this would:
        // 1. Upload data to test server
        // 2. Measure time taken
        // 3. Calculate speed in Mbps
        
        // Simulate upload speed test
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        // Simulate varying upload speeds (20-80 Mbps)
        let speed = 20.0 + (rand::random::<f64>() * 60.0);
        Ok(speed)
    }

    /// Get the test server location
    async fn get_server_location(&self) -> Result<String> {
        // Note: In a full implementation, this would query the server
        // for its geographic location
        Ok("Cloudflare Network".to_string())
    }

    /// Get current network statistics (real-time)
    pub fn get_current_stats(&self) -> NetworkStats {
        // Note: In a full implementation, this would monitor actual
        // network traffic and calculate real-time speeds
        NetworkStats {
            download_kbps: 0.0,
            upload_kbps: 0.0,
            total_downloaded: 0,
            total_uploaded: 0,
        }
    }
}

/// Real-time network statistics
#[derive(Debug, Clone, Default)]
pub struct NetworkStats {
    /// Current download speed in KB/s
    pub download_kbps: f64,
    /// Current upload speed in KB/s
    pub upload_kbps: f64,
    /// Total bytes downloaded in session
    pub total_downloaded: u64,
    /// Total bytes uploaded in session
    pub total_uploaded: u64,
}

impl Default for SpeedTestManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_speed_test_manager_creation() {
        let manager = SpeedTestManager::new();
        assert!(!manager.is_running());
        assert!(manager.last_result().is_none());
    }

    #[test]
    fn test_speed_test_phase_progress() {
        assert_eq!(SpeedTestPhase::Ping.progress(), 10);
        assert_eq!(SpeedTestPhase::Download(50).progress(), 35);
        assert_eq!(SpeedTestPhase::Upload(50).progress(), 85);
        assert_eq!(SpeedTestPhase::Complete.progress(), 100);
    }

    #[test]
    fn test_speed_test_config_default() {
        let config = SpeedTestConfig::default();
        assert_eq!(config.ping_count, 10);
        assert_eq!(config.download_duration_secs, 10);
        assert_eq!(config.upload_duration_secs, 10);
    }

    #[tokio::test]
    async fn test_speed_test_run() {
        let mut manager = SpeedTestManager::new();
        let result = manager.run_test(|phase| {
            tracing::debug!("Phase: {:?}", phase);
        }).await;
        
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.download_mbps > 0.0);
        assert!(result.upload_mbps > 0.0);
        assert!(result.ping_ms > 0.0);
    }
}
