use crate::adapter::*;

#[test]
fn test_adapter() {
    // Adapter Presents ExpectedInterface
    let adaptor = Adapter::new( DeviceInterface::new());
    let client = ClientDriver::new(Box::new(adaptor));

    client.send("Hello, world!");

    assert_eq!(client.receive(), "Hello, world!");
}
