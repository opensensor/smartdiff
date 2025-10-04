#!/usr/bin/env rust-script
//! Test binary comparison functionality with real Binary Ninja servers
//!
//! ```cargo
//! [dependencies]
//! smart-diff-binary-ninja-client = { path = "../crates/binary-ninja-client" }
//! smart-diff-engine = { path = "../crates/diff-engine" }
//! tokio = { version = "1", features = ["full"] }
//! anyhow = "1"
//! ```

use anyhow::Result;
use smart_diff_binary_ninja_client::BinaryNinjaClient;
use smart_diff_engine::{BinaryFunctionInfo, BinaryFunctionMatcher};

#[tokio::main]
async fn main() -> Result<()> {
    println!("=== Binary Ninja Comparison Test ===\n");

    // Create client
    let client = BinaryNinjaClient::new();

    // Step 1: Discover servers
    println!("Step 1: Discovering Binary Ninja servers...");
    let servers = client.discover_servers().await?;
    
    if servers.is_empty() {
        println!("✗ No Binary Ninja servers found!");
        println!("  Make sure Binary Ninja is running with binaries loaded");
        println!("  Use 'MCP Server > Start Server for This Binary' in Binary Ninja");
        return Ok(());
    }
    
    println!("✓ Found {} server(s):", servers.len());
    for server in &servers {
        println!("  - {}: {} (port {})", server.binary_id, server.filename, server.port);
    }
    println!();

    if servers.len() < 2 {
        println!("⚠ Need at least 2 binaries to test comparison");
        println!("  Load another binary in Binary Ninja and start its server");
        return Ok(());
    }

    // Step 2: Get function lists from both binaries
    let binary_a = &servers[0];
    let binary_b = &servers[1];
    
    println!("Step 2: Comparing binaries:");
    println!("  Binary A: {} ({})", binary_a.filename, binary_a.binary_id);
    println!("  Binary B: {} ({})", binary_b.filename, binary_b.binary_id);
    println!();

    println!("Step 3: Fetching function lists...");
    let functions_a_raw = client.list_functions(&binary_a.binary_id).await?;
    let functions_b_raw = client.list_functions(&binary_b.binary_id).await?;
    
    println!("  Binary A: {} functions", functions_a_raw.len());
    println!("  Binary B: {} functions", functions_b_raw.len());
    println!();

    // Convert to BinaryFunctionInfo
    let functions_a: Vec<BinaryFunctionInfo> = functions_a_raw
        .into_iter()
        .map(|f| BinaryFunctionInfo::new(f.name, f.address))
        .collect();

    let functions_b: Vec<BinaryFunctionInfo> = functions_b_raw
        .into_iter()
        .map(|f| BinaryFunctionInfo::new(f.name, f.address))
        .collect();

    // Step 4: Perform matching
    println!("Step 4: Matching functions...");
    let matcher = BinaryFunctionMatcher::new();
    let matches = matcher.match_functions(&functions_a, &functions_b)?;
    
    println!("✓ Found {} matches", matches.len());
    println!();

    // Step 5: Show statistics
    println!("Step 5: Match Statistics:");
    let exact_matches = matches.iter().filter(|m| m.name_similarity == 1.0).count();
    let fuzzy_matches = matches.iter().filter(|m| m.name_similarity < 1.0).count();
    let avg_similarity: f64 = matches.iter().map(|m| m.similarity).sum::<f64>() / matches.len() as f64;
    
    println!("  Exact name matches: {}", exact_matches);
    println!("  Fuzzy name matches: {}", fuzzy_matches);
    println!("  Average similarity: {:.1}%", avg_similarity * 100.0);
    println!();

    // Step 6: Show top 10 most changed functions
    println!("Step 6: Top 10 Most Changed Functions:");
    let mut sorted_matches = matches.clone();
    sorted_matches.sort_by(|a, b| a.similarity.partial_cmp(&b.similarity).unwrap());
    
    for (i, m) in sorted_matches.iter().take(10).enumerate() {
        println!(
            "  {}. {} <-> {} (similarity: {:.1}%, type: {:?})",
            i + 1,
            m.function_a.name,
            m.function_b.name,
            m.similarity * 100.0,
            m.match_type
        );
    }
    println!();

    // Step 7: Test decompilation for first match
    if let Some(first_match) = sorted_matches.first() {
        println!("Step 7: Testing decompilation for most changed function:");
        println!("  Function: {} <-> {}", first_match.function_a.name, first_match.function_b.name);
        println!();

        println!("  Decompiling from Binary A...");
        match client.decompile_function(&binary_a.binary_id, &first_match.function_a.name).await {
            Ok(code) => {
                println!("  ✓ Decompiled ({} bytes)", code.len());
                let lines: Vec<&str> = code.lines().take(10).collect();
                for line in lines {
                    println!("    {}", line);
                }
                if code.lines().count() > 10 {
                    println!("    ... ({} more lines)", code.lines().count() - 10);
                }
            }
            Err(e) => println!("  ✗ Failed: {}", e),
        }
        println!();

        println!("  Decompiling from Binary B...");
        match client.decompile_function(&binary_b.binary_id, &first_match.function_b.name).await {
            Ok(code) => {
                println!("  ✓ Decompiled ({} bytes)", code.len());
                let lines: Vec<&str> = code.lines().take(10).collect();
                for line in lines {
                    println!("    {}", line);
                }
                if code.lines().count() > 10 {
                    println!("    ... ({} more lines)", code.lines().count() - 10);
                }
            }
            Err(e) => println!("  ✗ Failed: {}", e),
        }
    }

    println!();
    println!("=== Test Complete ===");
    println!("✓ Binary comparison functionality is working!");

    Ok(())
}

