use array_init::array_init;
use clap::Parser;
use fltk::*;
// use serialport::new;
use rand::Rng;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::thread;
use std::time::Duration;

mod app_ui;
/* // To be used
mod serial_ctrl;
mod util;
*/

#[derive(Parser, Default, Debug)]
#[command(
    author = "Ivan Lumbano Vivar",
    version = "0.1.0",
    about = "PDP1 Control frontend"
)]
struct Args {
    /// Serial link [Unix: /dev/tty[]|pts; Windows: COM[]]
    #[arg(short, long)]
    link: String,
    /// Serial link baud rate
    #[arg(short, long)]
    baud_rate: u32,
}

fn main() {
    //
    // Args parse (serial config & thread id handling).
    //

    /* To be used
    let args = Args::parse();
    let (link_name, baud_rate) = (
        args.link,
        args.baud_rate,
    );*/

    //let (serial_th, pwm_th) = (0_usize, 1_usize);

    //
    // No Mutexed I/O direct stream Byte pointer (simulate I/O stream between threads).
    //
    // Since we want to simulate async serial stream there is no need for locking values
    // (reading intermediate writes are also expected).
    //
    // Also unsafe by definition since it can be used by threads that may outlive referenced value
    // owner thread (Not this case).
    //
    let end_flag = Arc::new(AtomicBool::new(false));

    /* // To be used
    let mut byte: [u8; 8] = [0_u8; 8];
    let raw_ptr: [util::raw_ptr::RawPtr<u8>; 8] = array_init(|i| util::raw_ptr::RawPtr {
        ptr: &mut byte[i] as *mut u8,
    });
    */

    let (s_efl0, s_efl1) = (end_flag.clone(), end_flag.clone());

    //
    // UI Indicators
    //

    let indicator: [OnceLock<Arc<Mutex<fltk::misc::Progress>>>; 119] =
        array_init(|_| OnceLock::new());

    let shared_indicator = Arc::new(indicator);
    let s_ind0 = shared_indicator.clone();

    //
    // Mutexed serial (safe control between threads)
    //

    /* // To be used
    let serial = Arc::new(Mutex::new(
        new(link_name, baud_rate)
            .timeout(Duration::from_millis(10))
            .open()
            .expect("Failed to open serial port"),
    ));*/

    //
    // Share duty reference to UI handler functions
    //

    let st_th = thread::spawn(move || {
        let mut rng = rand::rng();
        let mut end_delayed = false;
        loop {
            if end_delayed {
                break;
            }
            if app_ui::ui::PARENT.get().is_none() {
                thread::sleep(Duration::from_millis(100));
                continue;
            }
            thread::sleep(Duration::from_micros(16666));
            for blk in app_ui::ui::BLINKENLIGHTS
                .get()
                .expect("BLINKENLIGHT OnceLock GET Error")
                .iter()
            {
                if s_efl0.load(Ordering::SeqCst) {
                    end_delayed = true;
                    break;
                }
                blk.get()
                    .expect("BLINKENLIGHTS Get Mutex Error")
                    .lock()
                    .expect("BLINKENLIGHT Mutex Error")
                    .set_value(if rng.random_bool(0.5) { 100.0 } else { 0.0 });
            }
        }
        println!("Ended indicator task");
    });

    //
    //
    //
    app_ui::ui::BLINKENLIGHTS.set(s_ind0).ok();

    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let _ = app_ui::ui::UserInterface::make_window();

    //
    // Keep window awake dedicated thread
    //
    let aw_th = thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        loop {
            //
            // Shared End flag = true then end task
            //
            if s_efl1.load(Ordering::SeqCst) {
                break;
            }
            thread::sleep(Duration::from_millis(10));

            //
            // Update window content even while not focused
            //
            app::awake();
        }
        println!("Ended awake task");
    });

    app.run().unwrap();

    //
    // Set end flag true
    //
    end_flag.store(true, Ordering::SeqCst);

    aw_th.join().expect("UI awake handler thread crashed");
    st_th.join().expect("st thread crashed");
}
