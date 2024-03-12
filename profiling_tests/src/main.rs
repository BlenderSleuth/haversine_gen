mod testing;

use std::env;
use metrics::timing::estimate_cpu_frequency;

const TEST_CPU_FREQ_MILLIS: u64 = 100;

fn main() -> std::io::Result<()> {
    let cpu_freq = estimate_cpu_frequency(TEST_CPU_FREQ_MILLIS);
    
    if env::args().len() == 2 {
        let filename = env::args().nth(1).unwrap();
        let file = std::fs::File::open(&filename)?;
        let size = file.metadata()?.len();

        if size > 0 {
            testing::bandwidth_test_loop(size, cpu_freq, &filename);
        } else {
            eprintln!("ERROR: Test data size must be non-zero.");
        }
    } else {
        eprintln!("Usage: {} [existing filename]", env::current_exe()?.display());
    }

    Ok(())
}
