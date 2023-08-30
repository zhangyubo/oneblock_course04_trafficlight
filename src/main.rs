enum TrafficLight {
    Red,
    Yellow,
    Green,
}

trait Timeable {
    fn duration(&self) -> u32;
}

impl Timeable for TrafficLight {
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 50,
        }
    }
}

fn main() {
    let red_light = TrafficLight::Red;
    let yellow_light = TrafficLight::Yellow;
    let green_light = TrafficLight::Green;

    println!("红色交通灯持续时间: {} 秒", red_light.duration());
    println!("黄色交通灯持续时间: {} 秒", yellow_light.duration());
    println!("绿色交通灯持续时间: {} 秒", green_light.duration());
}
