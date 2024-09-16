/*!
    traffic_light.rs

    Implement a traffic light algorithm.
*/

/// Prints numbers from 1 to `max_num`, with certain exceptions:
/// * For multiples of 3, it prints "Fizz".
/// * For multiples of 5, it prints "Buzz".
/// * For multiples of both 3 and 5, it prints "FizzBuzz".
///
/// # Arguments
///
/// `max_num` - A positive integer (`u32`) that specifies the upper limit of the range to print.
///
#[derive(Debug, PartialEq, Copy, Clone)]

enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

#[derive(Debug, Copy, Clone)]

struct TrafficLightState {
    current_color: TrafficLightColor,
    last_transition_time_ms: u32,
}

// Wait time durations in milliseconds for each color
const RED_DURATION: u32 = 25000;
const YELLOW_DURATION: u32 = 5000;
const GREEN_DURATION: u32 = 30000;
const PEDESTRIAN_GREEN_DURATION: u32 = 20000;

/// Fetch the next color of the traffic light based on the current color
///
/// # Arguments
/// * `state` - The current state of the traffic light
///
/// # Returns
/// * The next color of the traffic light
fn get_next_color(state: TrafficLightState) -> TrafficLightColor {
    match state.current_color {
        TrafficLightColor::Red => TrafficLightColor::Green,
        TrafficLightColor::Yellow => TrafficLightColor::Red,
        TrafficLightColor::Green => TrafficLightColor::Yellow,
    }
}

/// Get the next state of the traffic light, potentially accounting for pedestrian requests.
///
/// # Arguments
/// * `state` - The current traffic light state
/// * `current_time_ms` - The current time in milliseconds
/// * `pedestrian_walk_request` - Boolean flag indicating if there is a pedestrian request
///
/// # Returns
/// * The updated `TrafficLightState` with the next color and last transition time
fn get_next_state(
    state: TrafficLightState,
    current_time_ms: u32,
    pedestrian_walk_request: bool,
) -> TrafficLightColor {
    let time_elapsed = current_time_ms - state.last_transition_time_ms;

    let (required_time, next_color) = match state.current_color {
        TrafficLightColor::Red => (RED_DURATION, TrafficLightColor::Green),
        TrafficLightColor::Yellow => (YELLOW_DURATION, TrafficLightColor::Red),
        TrafficLightColor::Green => {
            let duration = if pedestrian_walk_request {
                PEDESTRIAN_GREEN_DURATION
            } else {
                GREEN_DURATION
            };
            (duration, TrafficLightColor::Yellow)
        }
    };

    if time_elapsed >= required_time {
        // If the required time has passed, switch to the next color
        next_color
    } else {
        // If not enough time has passed, return the same state
        state.current_color
    }
}

// Do not modify below here
#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::traffic_light::{
        get_next_color, get_next_state, TrafficLightColor, TrafficLightState,
    };

    #[test_case(TrafficLightColor::Green, TrafficLightColor::Yellow ; "green -> yellow")]
    #[test_case(TrafficLightColor::Yellow, TrafficLightColor::Red ; "yellow -> red")]
    #[test_case(TrafficLightColor::Red, TrafficLightColor::Green ; "red -> green")]
    fn test_get_next_color(start_color: TrafficLightColor, next_color: TrafficLightColor) {
        let state = TrafficLightState {
            current_color: start_color,
            last_transition_time_ms: 0,
        };

        assert_eq!(get_next_color(state), next_color);
    }

    #[test]
    fn test_get_next_state_no_pedestrians() {
        let mut state = TrafficLightState {
            current_color: TrafficLightColor::Green,
            last_transition_time_ms: 0,
        };

        assert_eq!(get_next_state(state, 0, false), TrafficLightColor::Green);
        assert_eq!(
            get_next_state(state, 19000, false),
            TrafficLightColor::Green
        );
        assert_eq!(
            get_next_state(state, 21000, false),
            TrafficLightColor::Green
        );
        assert_eq!(
            get_next_state(state, 29000, false),
            TrafficLightColor::Green
        );
        assert_eq!(
            get_next_state(state, 31000, false),
            TrafficLightColor::Yellow
        );

        state.current_color = TrafficLightColor::Yellow;
        state.last_transition_time_ms = 30000;

        assert_eq!(
            get_next_state(state, 30000, false),
            TrafficLightColor::Yellow
        );
        assert_eq!(
            get_next_state(state, 34000, false),
            TrafficLightColor::Yellow
        );
        assert_eq!(get_next_state(state, 36000, false), TrafficLightColor::Red);

        state.current_color = TrafficLightColor::Red;
        state.last_transition_time_ms = 35000;

        assert_eq!(get_next_state(state, 35000, false), TrafficLightColor::Red);
        assert_eq!(get_next_state(state, 59000, false), TrafficLightColor::Red);
        assert_eq!(
            get_next_state(state, 61000, false),
            TrafficLightColor::Green
        );
    }

    #[test]
    fn test_get_next_state_pedestrians() {
        let mut state = TrafficLightState {
            current_color: TrafficLightColor::Green,
            last_transition_time_ms: 0,
        };

        assert_eq!(get_next_state(state, 0, true), TrafficLightColor::Green);
        assert_eq!(get_next_state(state, 19000, true), TrafficLightColor::Green);
        assert_eq!(
            get_next_state(state, 21000, true),
            TrafficLightColor::Yellow
        );

        state.current_color = TrafficLightColor::Yellow;
        state.last_transition_time_ms = 20000;

        assert_eq!(
            get_next_state(state, 20000, true),
            TrafficLightColor::Yellow
        );
        assert_eq!(
            get_next_state(state, 24000, true),
            TrafficLightColor::Yellow
        );
        assert_eq!(get_next_state(state, 26000, true), TrafficLightColor::Red);

        state.current_color = TrafficLightColor::Red;
        state.last_transition_time_ms = 25000;

        assert_eq!(get_next_state(state, 25000, true), TrafficLightColor::Red);
        assert_eq!(get_next_state(state, 49000, true), TrafficLightColor::Red);
        assert_eq!(get_next_state(state, 51000, true), TrafficLightColor::Green);
    }
}
