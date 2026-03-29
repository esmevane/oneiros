use crate::*;

pub struct PressurePresenter {
    response: PressureResponse,
}

impl PressurePresenter {
    pub fn new(response: PressureResponse) -> Self {
        Self { response }
    }

    pub fn render(self) -> Rendered<Responses> {
        let prompt = self.render_prompt();
        let text = self.render_text();
        let data = Response::new(Responses::from(self.response));

        Rendered::new(data, prompt, text)
    }

    fn render_prompt(&self) -> String {
        match &self.response {
            PressureResponse::Readings(result) => {
                RelevantPressures::from_pressures(result.pressures.clone()).to_string()
            }
        }
    }

    fn render_text(&self) -> String {
        match &self.response {
            PressureResponse::Readings(result) => {
                format!("Pressure readings for {}.", result.agent)
            }
        }
    }
}
