#[derive(Clone)]
pub struct User {
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
    pub membership_number: Option<String>,
    pub role: String,
    pub join_date: String,
    pub is_active: bool,
    pub rfid_card: Option<RfidCard>,
}

#[derive(Clone)]
pub struct Event {
    pub id: String,
    pub title: String,
    pub description: String,
    pub slug: String,
    pub date: String,
    pub end_date: Option<String>,
    pub location: String,
    pub max_attendees: Option<u32>,
    pub current_attendees: u32,
    pub is_public: bool,
    pub requires_registration: bool,
    pub registration_deadline: Option<String>,
    pub price: Option<u32>,
    pub organizer: User,
    pub status: String,
    pub tags: Vec<String>,
    pub image: Option<String>,
    pub registrations: Vec<EventRegistration>,
    pub check_ins: Vec<EventCheckIn>,
}

#[derive(Clone)]
pub struct RfidCard {
    pub id: String,
    pub card_number: String,
    pub status: String,
    pub issued_date: String,
    pub user_id: String,
}

#[derive(Clone)]
pub struct ShippingAddress {
    pub street: String,
    pub city: String,
    pub state: String,
    pub zip_code: String,
    pub country: String,
}

#[derive(Clone)]
pub struct RfidOrder {
    pub id: String,
    pub user_id: String,
    pub user: User,
    pub request_date: String,
    pub status: String,
    pub reason: String,
    pub shipping_address: ShippingAddress,
    pub notes: Option<String>,
    pub processed_by: Option<String>,
    pub processed_date: Option<String>,
}

#[derive(Clone)]
pub struct EventRegistration {
    pub id: String,
    pub event_id: String,
    pub user_id: String,
    pub user: User,
    pub registration_date: String,
    pub status: String,
}

#[derive(Clone)]
pub struct EventCheckIn {
    pub id: String,
    pub event_id: String,
    pub user_id: String,
    pub user: User,
    pub check_in_time: String,
    pub method: String,
    pub rfid_card_number: Option<String>,
    pub checked_in_by: String,
}
