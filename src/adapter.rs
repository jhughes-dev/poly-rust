/**
 * So, adapter pattern doesn't lend itself to the type of runtime polymorphism that this repo is about.
 * Don't worry, we'll still use the adapter pattern, but please excuse the shoehorning of polymorphism.
 */

pub trait ExpectedInterface {
    fn send(&self, data: &str);
    fn receive(&self) -> String;
}

#[derive(Default)]
pub struct DeviceInterface {}

impl DeviceInterface {
    pub fn new() -> DeviceInterface {
        DeviceInterface {}
    }

    fn read(&self) -> &[u8] {
        // Hex values for "Hello, world!"
        &[
            0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x2C, 0x20, 0x77, 0x6F, 0x72, 0x6C, 0x64, 0x21,
        ]
    }

    fn write(&self, data: &[u8]) {
        // I don't like using the lossy conversion, but it illustrates the point.
        println!("Writing data: {}", String::from_utf8_lossy(data));
    }
}

// This device is using a Strategy Pattern, and it expects a certain interface, defined by the ExpectedInterface trait.
// This is the only real instance of polymorphism in this example, but it will work for our purposes.
pub struct ClientDriver {
    port_device: Box<dyn ExpectedInterface>,
}

impl ClientDriver {
    pub fn new(port_device: Box<dyn ExpectedInterface>) -> Self {
        Self { port_device }
    }

    pub fn send(&self, data: &str) {
        // Notice this code is written in terms of the ExpectedInterface trait
        self.port_device.send(data);
    }

    pub fn receive(&self) -> String {
        //same thing here,
        self.port_device.receive()
    }
}

// This is the adapter. It adapts the DeviceInterface to the ExpectedInterface.
pub struct Adapter {
    device: DeviceInterface,
}

impl Adapter {
    pub fn new(device: DeviceInterface) -> Adapter {
        Adapter { device }
    }
}

impl ExpectedInterface for Adapter {
    fn send(&self, data: &str) {
        // Does the conversion to expected type as well as the calling the other methods
        self.device.write(data.as_bytes());
    }

    fn receive(&self) -> String {
        // Wouldn't really do this in production code, but works for this example.
        String::from_utf8_lossy(self.device.read()).to_string()
    }
}
