struct Package {
    sender_country: String,
    recipient_country: String,
    weight_in_grams: i32,
}

impl Package {
    fn new(sender_country: String, recipient_country: String, weight_in_grams: i32) -> Package {
        Package {
            sender_country,
            recipient_country,
            weight_in_grams,
        }
    }

    fn is_international(&self) -> bool {
        self.sender_country != self.recipient_country
    }

    fn get_fees(&self) -> i32 {
        let arbitrary_number = 42;
        self.weight_in_grams * arbitrary_number
    }
}

pub fn create_international_package() {
    let sender_country = String::from("Spain");
    let recipient_country = String::from("Russia");
    let package = Package::new(sender_country, recipient_country, 1200);
    assert!(package.is_international());
}

pub fn calculate_transport_fees() {
    let sender_country = String::from("Spain");
    let recipient_country = String::from("Spain");
    let package = Package::new(sender_country, recipient_country, 1000);
    assert_eq!(package.get_fees(), 42000);
}
