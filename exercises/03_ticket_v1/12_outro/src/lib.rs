// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

pub struct Order {
    product_name: String,
    quantity: usize,
    unit_price: usize,
}

impl Order {
    pub fn new(product_name: String, quantity: usize, unit_price: usize) -> Self {
        let product_name = Order::validate_name(product_name);
        let quantity = Order::validate_quantity(quantity);
        let unit_price = Order::validate_unit_price(unit_price);

        Order {
            product_name,
            unit_price,
            quantity,
        }
    }

    pub fn product_name(&self) -> &str {
        self.product_name.as_str()
    }

    pub fn quantity(&self) -> &usize {
        &self.quantity
    }

    pub fn unit_price(&self) -> &usize {
        &self.unit_price
    }

    pub fn set_product_name(&mut self, name: String) {
        self.product_name = Order::validate_name(name);
    }

    pub fn set_quantity(&mut self, quantity: usize) {
        self.quantity = Order::validate_quantity(quantity);
    }

    pub fn set_unit_price(&mut self, unit_price: usize) {
        self.unit_price = Order::validate_unit_price(unit_price);
    }

    pub fn total(&self) -> usize {
        self.quantity * self.unit_price
    }

    fn validate_name(name: String) -> String {
        if name.len() == 0 || name.len() > 300 {
            panic!("con not be empty");
        }
        name
    }

    fn validate_quantity(value: usize) -> usize {
        if value <= 0 {
            panic!("can not be 0");
        }
        value
    }

    fn validate_unit_price(value: usize) -> usize {
        if value <= 0 {
            panic!("can not be 0");
        }
        value
    }
}
