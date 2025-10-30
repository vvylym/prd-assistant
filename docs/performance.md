# Performance Guide

## Overview

This document provides guidance on optimizing PRD Assistant performance, understanding resource usage, and troubleshooting performance issues.

## Performance Characteristics

### Memory Usage

PRD Assistant is designed for minimal memory footprint:

- **Base Memory:** ~5-10 MB at startup
- **Peak Memory:** ~50-100 MB during AI operations
- **Memory Growth:** Linear with PRD size and complexity

### CPU Usage

- **Idle:** <1% CPU usage
- **AI Operations:** 10-30% CPU during content generation
- **File Operations:** <5% CPU for typical operations

### I/O Performance

- **File Reads:** Optimized for sequential access
- **File Writes:** Atomic operations for data integrity
- **Network I/O:** Async operations with proper timeouts

## Performance Optimization

### AI Model Optimization

#### Model Selection

Choose the right model for your needs:

```bash
# Fast, lightweight model
ollama pull gemma2:2b

# Balanced model (default)
ollama pull gemma3n:latest

# High-quality model
ollama pull llama2:13b
```

#### Model Configuration

Optimize model parameters:

```rust
// In src/agent/mod.rs
let request = CompletionRequest {
    temperature: Some(0.7),    // Lower = faster, more deterministic
    max_tokens: Some(2048),    // Limit response length
    // ... other parameters
};
```

### Memory Optimization

#### Streaming Responses

For large PRDs, consider streaming:

```rust
// Example streaming implementation
pub async fn generate_prd_streaming(&self, input: String) -> impl Stream<Item = String> {
    // Implementation for streaming AI responses
}
```

#### Resource Cleanup

Ensure proper resource cleanup:

```rust
// Drop large objects when done
let content = agent.generate_prd_content(input).await?;
// content is automatically dropped here
```

### File System Optimization

#### Batch Operations

Process multiple PRDs efficiently:

```rust
// Batch processing example
async fn batch_generate(projects: Vec<String>) -> Result<()> {
    let agent = PrdAgent::new("batch".to_string())?;
    
    for project in projects {
        // Reuse agent instance
        let content = agent.generate_prd_content(project).await?;
        // Process content...
    }
    
    Ok(())
}
```

#### Caching

Implement caching for repeated operations:

```rust
// Simple caching example
use std::collections::HashMap;

struct CachedAgent {
    agent: PrdAgent,
    cache: HashMap<String, String>,
}

impl CachedAgent {
    async fn generate_cached(&mut self, input: String) -> Result<String> {
        if let Some(cached) = self.cache.get(&input) {
            return Ok(cached.clone());
        }
        
        let result = self.agent.generate_prd_content(input.clone()).await?;
        self.cache.insert(input, result.clone());
        Ok(result)
    }
}
```

## Benchmarking

### Performance Metrics

Key metrics to monitor:

1. **Generation Time:** Time to generate PRD content
2. **Audit Time:** Time to audit existing PRDs
3. **Memory Usage:** Peak memory consumption
4. **File I/O:** Time for file operations

### Benchmarking Tools

#### Built-in Timing

```rust
use std::time::Instant;

let start = Instant::now();
let content = agent.generate_prd_content(input).await?;
let duration = start.elapsed();
println!("Generation took: {:?}", duration);
```

#### External Tools

```bash
# Memory profiling
cargo install cargo-profdata
cargo profdata --bench

# CPU profiling
cargo install cargo-flamegraph
cargo flamegraph --bin prd-assistant

# System monitoring
htop
iostat -x 1
```

### Benchmark Suite

Create performance benchmarks:

```rust
// In benches/performance.rs
use criterion::{criterion_group, criterion_main, Criterion};
use prd_assistant::agent::PrdAgent;

fn benchmark_generation(c: &mut Criterion) {
    let agent = PrdAgent::new("benchmark".to_string()).unwrap();
    
    c.bench_function("prd_generation", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| agent.generate_prd_content("test feature".to_string()))
    });
}

criterion_group!(benches, benchmark_generation);
criterion_main!(benches);
```

## Troubleshooting Performance Issues

### Common Issues

#### Slow AI Responses

**Symptoms:**
- Long wait times for PRD generation
- High CPU usage during AI operations
- Timeout errors

**Solutions:**
1. **Check Ollama Status:**
   ```bash
   ollama ps
   ollama list
   ```

2. **Optimize Model:**
   ```bash
   # Use smaller model
   ollama pull gemma2:2b
   ```

3. **Adjust Parameters:**
   ```rust
   // Reduce max_tokens for faster responses
   max_tokens: Some(1024),
   ```

#### High Memory Usage

**Symptoms:**
- High memory consumption
- System slowdown
- Out of memory errors

**Solutions:**
1. **Process Smaller PRDs:**
   - Break down large features
   - Generate multiple smaller PRDs

2. **Optimize Prompts:**
   - Use more specific descriptions
   - Avoid overly complex requirements

3. **Monitor Memory:**
   ```bash
   # Monitor memory usage
   htop
   free -h
   ```

#### Slow File Operations

**Symptoms:**
- Slow project initialization
- Delayed file saves
- I/O errors

**Solutions:**
1. **Check Disk Space:**
   ```bash
   df -h
   ```

2. **Optimize File System:**
   - Use SSD storage
   - Ensure sufficient free space

3. **Batch Operations:**
   - Group file operations
   - Use atomic writes

### Performance Monitoring

#### Real-time Monitoring

```bash
# Monitor system resources
htop

# Monitor disk I/O
iostat -x 1

# Monitor network
iftop
```

#### Logging

Enable detailed logging:

```bash
RUST_LOG=debug prd-assistant generate-prd project "feature"
```

#### Profiling

Profile the application:

```bash
# CPU profiling
cargo install cargo-flamegraph
cargo flamegraph --bin prd-assistant

# Memory profiling
cargo install cargo-profdata
cargo profdata --bench
```

## Performance Best Practices

### Development

1. **Efficient Algorithms:**
   - Use appropriate data structures
   - Avoid unnecessary allocations
   - Optimize hot paths

2. **Async Programming:**
   - Use async/await properly
   - Avoid blocking operations
   - Handle errors efficiently

3. **Resource Management:**
   - Drop resources when done
   - Use RAII patterns
   - Avoid memory leaks

### Usage

1. **Batch Operations:**
   - Process multiple PRDs together
   - Reuse agent instances
   - Cache frequently used data

2. **Model Selection:**
   - Choose appropriate model size
   - Balance quality vs. speed
   - Consider use case requirements

3. **System Resources:**
   - Ensure sufficient memory
   - Use fast storage
   - Monitor system load

## Performance Tuning

### Configuration Tuning

#### Ollama Configuration

```bash
# Optimize Ollama for performance
export OLLAMA_NUM_PARALLEL=2
export OLLAMA_MAX_LOADED_MODELS=1
export OLLAMA_MAX_QUEUE=512
```

#### Rust Compilation

```bash
# Optimize for performance
RUSTFLAGS="-C target-cpu=native" cargo build --release

# Enable optimizations
cargo build --release --features "optimize"
```

### System Tuning

#### Linux

```bash
# Increase file descriptor limits
ulimit -n 65536

# Optimize memory settings
echo 'vm.swappiness=10' >> /etc/sysctl.conf
```

#### macOS

```bash
# Increase file limits
sudo launchctl limit maxfiles 65536 200000
```

## Performance Testing

### Load Testing

Test under various loads:

```rust
// Load testing example
async fn load_test() -> Result<()> {
    let agent = PrdAgent::new("load-test".to_string())?;
    let tasks = (0..100).map(|i| {
        let agent = agent.clone();
        tokio::spawn(async move {
            agent.generate_prd_content(format!("Feature {}", i)).await
        })
    });
    
    let results = futures::future::join_all(tasks).await;
    // Process results...
    
    Ok(())
}
```

### Stress Testing

Test system limits:

```rust
// Stress testing example
async fn stress_test() -> Result<()> {
    let mut handles = Vec::new();
    
    for i in 0..1000 {
        let handle = tokio::spawn(async move {
            // Generate PRD
            let agent = PrdAgent::new("stress-test".to_string())?;
            agent.generate_prd_content(format!("Stress test {}", i)).await
        });
        handles.push(handle);
    }
    
    // Wait for all tasks
    for handle in handles {
        handle.await??;
    }
    
    Ok(())
}
```

## Performance Metrics

### Key Performance Indicators (KPIs)

1. **Response Time:**
   - PRD generation: <30 seconds
   - PRD auditing: <20 seconds
   - Project initialization: <5 seconds

2. **Throughput:**
   - PRDs per minute: >2
   - Concurrent operations: >5
   - File operations per second: >100

3. **Resource Usage:**
   - Memory usage: <100 MB
   - CPU usage: <50%
   - Disk I/O: <10 MB/s

### Monitoring Dashboard

Create a simple monitoring dashboard:

```rust
// Performance monitoring
use std::time::{Duration, Instant};

struct PerformanceMonitor {
    start_time: Instant,
    operations: Vec<Duration>,
}

impl PerformanceMonitor {
    fn new() -> Self {
        Self {
            start_time: Instant::now(),
            operations: Vec::new(),
        }
    }
    
    fn record_operation(&mut self, duration: Duration) {
        self.operations.push(duration);
    }
    
    fn get_stats(&self) -> (Duration, Duration, Duration) {
        let total = self.start_time.elapsed();
        let avg = self.operations.iter().sum::<Duration>() / self.operations.len() as u32;
        let max = self.operations.iter().max().copied().unwrap_or_default();
        
        (total, avg, max)
    }
}
```

## Conclusion

Performance optimization is an ongoing process. Regular monitoring, profiling, and optimization will ensure PRD Assistant remains fast and efficient as it scales. Use the tools and techniques outlined in this guide to maintain optimal performance.
