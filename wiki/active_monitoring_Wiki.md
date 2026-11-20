# SigmaOS-active-monitoring Component Wiki

## Overview
SigmaOS-active-monitoring provides system monitoring and health checking utilities for the SigmaOS operating system. It demonstrates the system monitoring subsystem and resource tracking within the SigmaOS ecosystem, following the principles of clear abstraction and deterministic monitoring initialization.

## Principles
- Single Responsibility
- Monitoring Abstraction
- Fail-Fast on Monitoring Initialization Errors
- Deterministic Monitoring Setup
- Clear Abstraction Over Monitoring APIs
- Composition over Inheritance for monitoring extensions

## Architecture
SigmaOS-active-monitoring follows a monitoring pattern:
- **Monitoring Core**: Provides resource tracking, metric collection, and health check primitives
- **System Metrics**: Supports CPU, memory, disk, and network metric collection
- **Alert Management**: Manages monitoring alerts and threshold notifications
- **Error Handling**: Fail-fast on invalid monitoring configurations or unsupported metrics

## API
The component provides the following public APIs:
- `monitor::cpu::usage()` - Get current CPU usage percentage
- `monitor::memory::usage()` - Get current memory usage percentage
- `monitor::disk::usage(path)` - Get disk usage for a given path
- `monitor::network::stats()` - Get network interface statistics
- `monitor::alert::threshold(level, callback)` - Set a monitoring alert threshold
- Error types for monitoring failure scenarios or unsupported metrics

## Test Structure (Arrange-Act-ASSERT)
All tests follow the AAA pattern:
1. **Arrange**: Set up test environment with specific monitoring configuration
2. **Act**: Execute the code under test (metric collection, alert setup)
3. **Assert**: Verify the expected monitoring results

Example:
```rust
#[test]
fn test_monitor_cpu_usage_aaa() {
    // Arrange
    
    // Act
    let usage = monitor::cpu::usage();
    
    // Assert
    assert!(usage.is_ok());
    assert!(usage.unwrap() <= 100);
}
```

## Development
- Follows Rust best practices with `#![forbid(unsafe_code)]`
- Monitoring initialization is best-effort; gracefully degrades on unsupported configurations
- Comprehensive error types for all monitoring failure scenarios
- Unit tests provided for metric collection and alert management
- Integration tests for full monitoring system with actual resource tracking

## TODO
- Add support for additional metrics (disk I/O, process counting, temperature sensing)
- Expand alert management capabilities (multiple thresholds, notification channels)
- Add metric persistence and trend analysis features
- Create integration tests with actual system monitoring tools
- Implement metric caching and sampling rate configuration