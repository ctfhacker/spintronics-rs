use spintronics::*;

fn main() {
    let mut circuit = Circuit::new();

    let motor = circuit.motor();
    let r1 = circuit.resistor(1000);
    let r2 = circuit.resistor(200);
    let r3 = circuit.resistor(500);

    circuit.connect(&[motor, r3, r2, r1]);

    circuit.save("/home/user/test.spin");
}
