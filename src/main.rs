use std::str::FromStr;

use chrono::Local;
use leptos::*;

use pomodoro_calculator::PomodoroConfig;
use pomodoro_calculator::{calculate_pomodoros, PomodoroResult};

const MAX_WORK_HOURS: i64 = 1000;
const MAX_POMODORO_TIME: i64 = 6000;
const MAX_BREAK_TIME: i64 = 6000;

fn set_validated_state<T: FromStr + PartialOrd + Copy>(
    event: &web_sys::Event,
    set_state: &impl Fn(T),
    min: T,
    max: T,
) {
    if let Ok(mut value) = event_target_value(event).parse::<T>() {
        if value < min {
            value = min;
        } else if value > max {
            value = max;
        }
        set_state(value);
    }
}

#[component]
fn App() -> impl IntoView {
    // Obtenemos la hora de inicio actual
    let start_time = Local::now();

    // Creamos señales para manejar los inputs del usuario
    let (video_hours, set_video_hours) = create_signal(2i64);
    let (video_minutes, set_video_minutes) = create_signal(0i64);
    let (playback_speed, set_playback_speed) = create_signal(1.0_f64);
    let (work_time, set_work_time) = create_signal(25_i64);
    let (short_break, set_short_break) = create_signal(5_i64);
    let (long_break, set_long_break) = create_signal(15_i64);
    let (pomodoros_until_long_break, set_pomodoros_until_long_break) = create_signal(2_i64);
    let (distraction_level, set_distraction_level) = create_signal(0_i64);
    let (result, set_result) = create_signal(None::<PomodoroResult>);

    // Calculamos los resultados basados en las configuraciones del usuario
    let calculate = move || {
        let config = PomodoroConfig {
            work_time: work_time.get(),
            short_break: short_break.get(),
            long_break: long_break.get(),
            pomodoros_until_long_break: pomodoros_until_long_break.get(),
            distraction_level: distraction_level.get() as f64,
        };

        let total_minutes = ((video_hours.get() * 60) + video_minutes.get()) as f64;

        let new_result =
            calculate_pomodoros(start_time, total_minutes, playback_speed.get(), config);

        set_result(Some(new_result));
    };

    create_effect(move |_| {
        calculate();
    });

    const DAISY_LABEL: &str = "join-item input-bordered w-full max-w-xs";
    const DAISY_SELECT: &str = "select select-bordered select-md w-full max-w-xs";
    const DAISY_INPUT: &str = "input input-bordered input-md w-full max-w-xs";

    view! {
        <div class="mx-auto grid grid-rows-10 max-w-[800px]">
            <div class="row-span-1 flex items-center justify-center">
                <h1 class="text-2xl font-bold max-h-min">"Pomodoro Calculator"</h1>
            </div>
            <div class="row-span-9 grid grid-cols-2">
                <div class=" flex items-center justify-center join join-vertical ">
                    <label class=DAISY_LABEL>
                        <div class="label">
                            <span class="label-text">"Video duration (hours):"</span>
                        </div>

                        <input
                            type="number"
                            class=DAISY_INPUT
                            value=video_hours
                            on:input=move |e| {
                                set_validated_state(&e, &set_video_hours, 0, MAX_WORK_HOURS);
                            }
                        />

                    </label>

                    <label class=DAISY_LABEL>
                        <div class="label">
                            <span class="label-text">"Video duration (minutes):"</span>
                        </div>

                        <input
                            type="number"
                            class=DAISY_INPUT
                            value=video_minutes
                            on:input=move |e| {
                                set_validated_state(&e, &set_video_minutes, 0, MAX_WORK_HOURS);
                            }
                        />

                    </label>

                    <label class=DAISY_LABEL>
                        <div class="label">
                            <span class="label-text">"Playback Speed:"</span>
                        </div>

                        <select
                            class=DAISY_SELECT
                            value=playback_speed
                            on:change=move |e| {
                                f64::from_str(&event_target_value(&e)).map(set_playback_speed).ok();
                            }
                        >

                            <option value="1.0">"1x"</option>
                            <option value="1.25">"1.25x"</option>
                            <option value="1.5">"1.5x"</option>
                            <option value="2.0">"2x"</option>
                        </select>
                    </label>
                    <div class="justify-center divider py-4 px-[10%]"></div>
                    <label class=DAISY_LABEL>
                        <div class="label">
                            <span class="label-text">"Pomodoro Time (minutes):"</span>
                        </div>

                        <input
                            type="number"
                            class=DAISY_INPUT
                            value=work_time
                            on:input=move |e| {
                                set_validated_state(&e, &set_work_time, 0, MAX_POMODORO_TIME);
                            }
                        />

                    </label>

                    <label class=DAISY_LABEL>
                        <div class="label">
                            <span class="label-text">"Short Break (minutes):"</span>
                        </div>
                        <input
                            type="number"
                            class=DAISY_INPUT
                            value=short_break
                            on:input=move |e| {
                                set_validated_state(&e, &set_short_break, 0, MAX_BREAK_TIME);
                            }
                        />

                    </label>

                    <label class=DAISY_LABEL>
                        <div class="label">
                            <span class="label-text">"Long Break (minutes):"</span>
                        </div>
                        <input
                            type="number"
                            class=DAISY_INPUT
                            value=long_break
                            on:input=move |e| {
                                set_validated_state(&e, &set_long_break, 0, MAX_BREAK_TIME);
                            }
                        />

                    </label>

                    <label class=DAISY_LABEL>
                        <div class="label">
                            <span class="label-text">"Pomodoros Until Long Break:"</span>
                        </div>

                        <input
                            type="number"
                            class=DAISY_INPUT
                            value=pomodoros_until_long_break
                            on:input=move |e| {
                                i64::from_str(&event_target_value(&e))
                                    .map(set_pomodoros_until_long_break)
                                    .ok();
                            }
                        />

                    </label>

                    <label class=DAISY_LABEL>
                        <div class="label">
                            <span class="label-text">"Distraction level"</span>
                        </div>
                        <label class="input input-bordered input-md flex items-center gap-4">
                            <input
                                type="range"
                                min="0"
                                max="100"
                                value=distraction_level
                                class="range"
                                step="10"
                                on:input=move |e| {
                                    i64::from_str(&event_target_value(&e))
                                        .map(set_distraction_level)
                                        .ok();
                                }
                            />

                            <p class="text-sm max-w-fit">
                                {move || distraction_level.get().to_string() + "%"}
                            </p>
                        </label>
                    </label>
                </div>
                <div class="row-span-9 grid items-center justify-center">
                    <div class="stats stats-vertical">
                        {move || {
                            if let Some(result) = result.get() {
                                view! {
                                    <div class="stat flex items-center">
                                        <div class="stat-title">Pomodoros:</div>
                                        <div class="stat-value text-2xl">{result.pomodoros}</div>
                                    </div>
                                    <div class="stat flex items-center">
                                        <div class="stat-title">Short Breaks:</div>
                                        <div class="stat-value text-2xl">{result.short_breaks}</div>
                                    </div>
                                    <div class="stat flex items-center">
                                        <div class="stat-title">Long Breaks:</div>
                                        <div class="stat-value text-2xl">{result.long_breaks}</div>
                                    </div>
                                    <div class="stat flex items-center">
                                        <div class="stat-title">Total Time:</div>
                                        <div class="stat-value text-2xl">
                                            {result.time_spend.num_minutes()} " minutes"
                                        </div>
                                    </div>
                                    <div class="stat flex items-center">
                                        <div class="stat-title">Start Time:</div>
                                        <div class="stat-value text-2xl">
                                            {start_time.format("%H:%M:%S").to_string()}
                                        </div>
                                    </div>
                                    <div class="stat flex items-center">
                                        <div class="stat-title">End Time:</div>
                                        <div class="stat-value text-2xl">
                                            {result.end_time.format("%H:%M:%S").to_string()}
                                        </div>
                                    </div>
                                }
                            } else {
                                view! {
                                    <br/>
                                    <br/>
                                }
                            }
                        }}

                    </div>
                </div>
            </div>
        </div>
    }
}

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}
