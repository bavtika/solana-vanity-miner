use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
use clap::Parser;
use rand::rngs::OsRng;

#[derive(Parser, Debug)]
#[command(author, version, about = "Solana Vanity Address Miner (Standalone CLI)", long_about = None)]
struct Cli {
    /// The pattern to search for
    #[arg(short, long)]
    pattern: String,

    /// Whether the pattern must be at the start (prefix) or end (suffix)
    #[arg(long, default_value_t = true)]
    prefix: bool,

    /// Whether the pattern search is case-sensitive
    #[arg(short, long, default_value_t = true)]
    strict: bool,
    
    /// Number of threads to use (defaults to all logical cores)
    #[arg(short, long)]
    threads: Option<usize>,
}

fn main() {
    let cli = Cli::parse();
    
    let pattern = if cli.strict {
        cli.pattern.clone()
    } else {
        cli.pattern.to_lowercase()
    };
    
    let is_prefix = cli.prefix;
    let is_strict = cli.strict;
    
    let num_threads = cli.threads.unwrap_or_else(|| num_cpus::get());
    
    println!("🚀 Solana Vanity Miner Standalone");
    println!("=============================");
    println!("Pattern: {}", pattern);
    println!("Position: {}", if is_prefix { "Prefix" } else { "Suffix" });
    println!("Case Sensitive: {}", is_strict);
    println!("Threads: {}", num_threads);
    println!("=============================\n");
    println!("Mining started. Press Ctrl+C to abort...\n");

    let found = Arc::new(AtomicBool::new(false));
    let hashes_checked = Arc::new(AtomicUsize::new(0));
    
    let (tx, rx) = std::sync::mpsc::channel::<(String, Vec<u8>)>();

    // Spawn miner threads
    for _ in 0..num_threads {
        let tx_clone = tx.clone();
        let pattern = pattern.clone();
        let hashes = hashes_checked.clone();
        let found = found.clone();
        
        std::thread::spawn(move || {
            let mut rng = OsRng;
            let mut local_count = 0;
            
            while !found.load(Ordering::Relaxed) {
                let signing_key = ed25519_dalek::SigningKey::generate(&mut rng);
                let verifying_key = signing_key.verifying_key();
                let pubkey_bytes = verifying_key.as_bytes();
                
                let pubkey_b58 = bs58::encode(pubkey_bytes).into_string();
                
                let check_str = if is_strict {
                    pubkey_b58.clone()
                } else {
                    pubkey_b58.to_lowercase()
                };

                let is_match = if is_prefix {
                    check_str.starts_with(&pattern)
                } else {
                    check_str.ends_with(&pattern)
                };

                if is_match {
                    found.store(true, Ordering::Relaxed);
                    let mut secret_bytes = signing_key.to_bytes().to_vec();
                    secret_bytes.extend_from_slice(pubkey_bytes); // Solana format: [secret, public]
                    let _ = tx_clone.send((pubkey_b58, secret_bytes));
                    break;
                }

                local_count += 1;
                if local_count >= 10000 {
                    hashes.fetch_add(local_count, Ordering::Relaxed);
                    local_count = 0;
                }
            }
        });
    }

    // Hashrate reporter thread
    let hashes_reporter = hashes_checked.clone();
    let found_reporter = found.clone();
    std::thread::spawn(move || {
        let mut last_hashes = 0;
        while !found_reporter.load(Ordering::Relaxed) {
            std::thread::sleep(Duration::from_secs(2));
            let current = hashes_reporter.load(Ordering::Relaxed);
            let diff = current - last_hashes;
            last_hashes = current;
            
            let rate = diff / 2;
            if rate > 0 {
                // Print carriage return to overwrite line
                print!("\rHashrate: {} H/s", rate);
                use std::io::Write;
                let _ = std::io::stdout().flush();
            }
        }
    });

    // Wait for match
    if let Ok((pubkey, secret_bytes)) = rx.recv() {
        println!("\n\n🎉 MATCH FOUND!");
        println!("=============================");
        println!("Public Key (Address): {}", pubkey);
        
        let secret_key_bs58 = bs58::encode(&secret_bytes).into_string();
        println!("Private Key (Base58): {}", secret_key_bs58);
        println!("=============================\n");
        println!("⚠️  KEEP YOUR PRIVATE KEY SAFE. NEVER SHARE IT WITH ANYONE.");
    }
}
