#[derive(Debug)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn next_state(&self) -> Self {
        match self {
            TrafficLight::Red => TrafficLight::Green,
            TrafficLight::Yellow => TrafficLight::Red,
            TrafficLight::Green => TrafficLight::Yellow,
        }
    }
}

fn main() {
    let mut light = TrafficLight::Red;
    for _ in 0..5 {
        light = light.next_state();
        println!("Traffic light is now: {:?}", light); // This should now work with the Debug trait implemented
    }
}
