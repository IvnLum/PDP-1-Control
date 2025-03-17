use fltk::prelude::*;
use std::sync::{Arc, Mutex, OnceLock};
fl2rust_macro::include_ui! {"src/app_ui/res/ui.fl"}

pub static PARENT: OnceLock<Arc<Mutex<Group>>> = OnceLock::new();
pub static BLINKENLIGHTS: OnceLock<Arc<[OnceLock<Arc<Mutex<Progress>>>; 119]>> = OnceLock::new();

//
// Indicator children within 68-186 range
//
pub static IDXA: i32 = 68_i32;
pub static IDXB: i32 = 186_i32;

fn store_children_references() {
    for (idx, i) in (IDXA..=IDXB).enumerate() {
        if let Some(child) = PARENT
            .get()
            .expect("Get group parent error")
            .lock()
            .expect("Mutex lock error")
            .child(i)
        {
            if let Some(progress) = Progress::from_dyn_widget(&child) {
                BLINKENLIGHTS
                    .get()
                    .expect("BLINKENLIGHTS OnceLock GET Error")[idx]
                    .set(Arc::new(Mutex::new(progress.clone())))
                    .ok();
            }
        }
    }
}

fn from_ui_update_parent(start: &mut Button) {
    if !PARENT.get().is_none() {
        return;
    }
    if let Some(parent) = start.parent() {
        let c_par = parent.clone();
        PARENT.set(Arc::new(Mutex::new(c_par))).ok();
        store_children_references();
    } else {
        println!("Error: No referenced parent");
    }
}
