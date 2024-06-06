enum TrafficLight {
    Red,
    Green,
    Yellow
}

impl TrafficLight {
    fn transition_light(&mut self) {
        *self = match self {
            TrafficLight::Green => TrafficLight::Yellow,
            TrafficLight::Yellow => TrafficLight::Red,
            TrafficLight::Red => TrafficLight::Green,
        };
    }

    fn display_light(&self) {
        match self {
            TrafficLight::Green => println!("current traffic signal is GREEN"),
            TrafficLight::Yellow => println!("current traffic signal is YELLOW"),
            TrafficLight::Red => println!("current traffic signal is RED"),
        }
    }
}

fn main() {
    let mut traffic_signal = TrafficLight::Yellow;
    traffic_signal.display_light();

    traffic_signal.transition_light();
    traffic_signal.display_light();

    traffic_signal.transition_light();
    traffic_signal.display_light();
}
