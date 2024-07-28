use spintronics::*;

fn main() {
    let mut circuit = Circuit::new();

    let motor = circuit.motor();
    let r1 = circuit.resistor(1000);
    let r2 = circuit.resistor(500);
    let r3 = circuit.resistor(200);

    circuit.connect(&[motor, r1]);
    circuit.connect(&[motor, r2, r3]);

    circuit.save("/home/user/test.spin");
}
