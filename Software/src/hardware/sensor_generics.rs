pub trait Sensor {
    //    type Input;
    type Output;

    fn read_sensor(&mut self /*, input: Self::Input*/) -> Self::Output;
}
