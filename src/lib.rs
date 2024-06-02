use chrono::{DateTime, Duration, Local, TimeDelta};

pub struct PomodoroConfig {
    pub work_time: i64,
    pub short_break: i64,
    pub long_break: i64,
    pub pomodoros_until_long_break: i64,
    pub distraction_level: f64,
}

#[derive(Clone)]
pub struct PomodoroResult {
    pub pomodoros: i64,
    pub short_breaks: i64,
    pub long_breaks: i64,
    pub time_spend: Duration,
    pub end_time: DateTime<Local>,
}

pub fn calculate_pomodoros(
    start_time: DateTime<Local>,
    video_minutes: f64,
    playback_speed: f64,
    config: PomodoroConfig,
) -> PomodoroResult {
    let video_duration_min = video_minutes / playback_speed;
    let calc_video_duration_min = video_duration_min * (1.0 + (config.distraction_level / 100.0));

    let mut loops = 0;
    let mut steps = 0;

    let mut pomodoros = 0;
    let mut short_breaks = 0;
    let mut long_breaks = 0;

    let mut tmp_duration: TimeDelta;
    let mut working_spend = Duration::minutes(0);
    let mut time_spend = Duration::minutes(0);

    let work_duration = Duration::minutes(config.work_time);
    let short_break_duration = Duration::minutes(config.short_break);
    let long_break_duration = Duration::minutes(config.long_break);

    while calc_video_duration_min > working_spend.num_minutes() as f64 {
        steps += 1;

        if steps % 2 == 1 {
            pomodoros += 1;
            tmp_duration = work_duration;
            working_spend += tmp_duration;
        } else {
            if loops != 0 && loops % config.pomodoros_until_long_break == 0 {
                long_breaks += 1;
                tmp_duration = short_break_duration;
            } else {
                short_breaks += 1;
                tmp_duration = long_break_duration;
            }
            loops += 1;
        }

        time_spend += tmp_duration;
    }

    // En caso de que no se complete todo el pomodoro
    time_spend -= working_spend - Duration::minutes(calc_video_duration_min as i64);

    PomodoroResult {
        pomodoros,
        short_breaks,
        long_breaks,
        time_spend,
        end_time: start_time + time_spend,
    }
}
