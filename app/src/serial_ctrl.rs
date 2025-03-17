use serialport::SerialPort;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::util::raw_ptr::RawPtr;
use crate::util::thread_misc::spin_sleep;

/// Write serial main task to be executed from a dedicated pinned thread
///
/// Arguments:
///
/// * `byte` - Raw-pointer with thread borrowing traits as an output target.
/// * `cycle_period` - The full cycle period or 1/freq,  ie: 50hz -> 1/50hz (20ms).
///
pub fn write_serial(
    serial: Arc<Mutex<Box<dyn SerialPort>>>,
    byte: [RawPtr<u8>; 8],
    end: Arc<AtomicBool>,
) {
    let mut now: std::time::Instant;
    let duration = Duration::from_micros(50);

    loop {
        now = std::time::Instant::now();
        if end.load(Ordering::SeqCst) {
            //
            // end flag TRUE then break loop
            //

            break;
        }
        unsafe {
            if let Err(e) =
                serial
                    .lock()
                    .expect("Mutex write error!")
                    .write_all(&[(*byte[0].ptr) //<< 0
                    | (*byte[1].ptr) << 1
                    | (*byte[2].ptr) << 2
                    | (*byte[3].ptr) << 3
                    | (*byte[4].ptr) << 4
                    | (*byte[5].ptr) << 5
                    | (*byte[6].ptr) << 6
                    | (*byte[7].ptr) << 7])
            {
                eprintln!("Failed to write: {}", e);
            }
        }
        if now.elapsed() < duration {
            spin_sleep(duration.saturating_sub(now.elapsed()));
        } else {
            eprintln!(
                "Write serial took too long {:?} / {:?} (expected)",
                now.elapsed(),
                duration
            );
        }
    }
    println!("Successfully ended write_serial thread main task");
}
