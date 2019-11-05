use crate::HobbySettings;

#[derive(Debug)]
pub struct Application {

}

impl Application {
	pub fn new(settings: HobbySettings) -> Result<Application, &'static str> {
		
		Ok(Application{})
	}
}