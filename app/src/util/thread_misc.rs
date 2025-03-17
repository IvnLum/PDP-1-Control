#[cfg(target_os = "linux")]
use libc::{cpu_set_t, pid_t, sched_setaffinity, CPU_SET, CPU_ZERO};

#[cfg(target_os = "windows")]
use windows::Win32::System::Threading::{GetCurrentThread, SetThreadAffinityMask};

use std::time::Duration;

/// Busy wait or spin sleep [To be called from a dedicated pinned thread]
///
/// # Arguments:
///
/// * `duration` - The duration of the busy wait in std::time::Duration.
///
pub fn spin_sleep(duration: Duration) {
    let start = std::time::Instant::now();

    while start.elapsed() < duration {
        std::hint::spin_loop();
    }
}

/// Linux & Windows core pinning
///
/// Arguments:
///
/// * `th_idx` - Thread ID to pin, taken as a value directly in Linux or as a binary shift Windows.
///
pub fn pin_thread_to_core(th_idx: usize) {
    #[cfg(target_os = "linux")]
    unsafe {
        let mut cpuset: cpu_set_t = std::mem::zeroed();
        CPU_ZERO(&mut cpuset);
        CPU_SET(th_idx, &mut cpuset);

        let pid: pid_t = 0;
        if sched_setaffinity(pid, std::mem::size_of::<cpu_set_t>(), &cpuset) != 0 {
            eprintln!("Failed to set CPU affinity on Linux");
        }
    }

    #[cfg(target_os = "windows")]
    unsafe {
        let thread = GetCurrentThread();
        let mask = 1 << th_idx;
        if SetThreadAffinityMask(thread, mask) == 0 {
            eprintln!("Failed to set thread affinity on Windows");
        }
    }
}
