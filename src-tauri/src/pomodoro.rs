use std::sync::Mutex;
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct TimerState(pub Mutex<Timer>);

#[derive(Clone, Serialize, Deserialize)]
pub struct Timer {
    start_time: Option<u64>,
    duration: u64,
    remaining: u64,
    is_active: bool,
    is_break: bool,
    completed_pomodoros: u32,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            start_time: None,
            duration: 15 * 60, // 25 minutes in seconds
            remaining: 15 * 60,
            is_active: false,
            is_break: false,
            completed_pomodoros: 0,
        }
    }

    fn now() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
    }
}

#[tauri::command]
pub fn start_timer(state: tauri::State<TimerState>) -> Timer {
    let mut timer = state.0.lock().unwrap();
    timer.is_active = true;
    timer.start_time = Some(Timer::now());
    timer.clone()
}

#[tauri::command]
pub fn pause_timer(state: tauri::State<TimerState>) -> Timer {
    let mut timer = state.0.lock().unwrap();
    if timer.is_active {
        timer.is_active = false;
        if let Some(start_time) = timer.start_time {
            let elapsed = Timer::now().saturating_sub(start_time);
            timer.remaining = timer.remaining.saturating_sub(elapsed);
        }
    }
    timer.clone()
}

#[tauri::command]
pub fn reset_timer(state: tauri::State<TimerState>) -> Timer {
    let mut timer = state.0.lock().unwrap();
    *timer = if timer.is_break {
        Timer {
            duration: 5 * 60, // 5 minutes break
            remaining: 5 * 60,
            is_break: true,
            ..*timer
        }
    } else {
        Timer::new()
    };
    timer.clone()
}

#[tauri::command]
pub fn get_timer_state(state: tauri::State<TimerState>) -> Timer {
    let mut timer = state.0.lock().unwrap();
    if timer.is_active {
        if let Some(start_time) = timer.start_time {
            let elapsed = Timer::now().saturating_sub(start_time);
            timer.remaining = timer.remaining.saturating_sub(elapsed);
            timer.start_time = Some(Timer::now());
        }
    }
    if timer.remaining == 0 {
        if !timer.is_break {
            timer.completed_pomodoros += 1;
        }
        *timer = if timer.is_break {
            Timer::new()
        } else {
            Timer {
                duration: 5 * 60, // 5 minutes break
                remaining: 5 * 60,
                is_break: true,
                completed_pomodoros: timer.completed_pomodoros,
                ..Timer::new()
            }
        };
    }
    timer.clone()
}