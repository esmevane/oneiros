use crate::*;

pub struct PressurePresenter<'a> {
    response: PressureResponse,
    request: &'a PressureRequest,
}

impl<'a> PressurePresenter<'a> {
    pub fn new(response: PressureResponse, request: &'a PressureRequest) -> Self {
        Self { response, request }
    }

    pub fn mcp(&self) -> McpResponse {
        match &self.response {
            PressureResponse::Readings(ReadingsResponse::V1(result)) => {
                let title = match self.request {
                    PressureRequest::GetPressure(get) => match get.current() {
                        Ok(details) => format!("# Pressure — {}\n\n", details.agent),
                        Err(_) => "# Pressure\n\n".to_string(),
                    },
                    _ => "# Pressure\n\n".to_string(),
                };
                let mut md = title;
                for pressure in &result.pressures {
                    md.push_str(&format!("## {}\n\n", pressure.urge));
                    md.push_str(&format!(
                        "**urgency:** {:.0}%\n\n",
                        pressure.urgency() * 100.0
                    ));
                }
                McpResponse::new(md)
            }
            PressureResponse::AllReadings(AllReadingsResponse::V1(result)) => {
                let mut md = String::from("# Pressure — All Agents\n\n");
                for pressure in &result.pressures {
                    md.push_str(&format!(
                        "- **{}** ({}): {:.0}%\n",
                        pressure.urge,
                        pressure.agent_id,
                        pressure.urgency() * 100.0
                    ));
                }
                McpResponse::new(md)
            }
        }
    }

    pub fn render(self) -> Rendered<PressureResponse> {
        let prompt = self.render_prompt();
        let text = self.render_text();

        Rendered::new(self.response, prompt, text)
    }

    fn render_prompt(&self) -> String {
        match &self.response {
            PressureResponse::Readings(ReadingsResponse::V1(result)) => {
                RelevantPressures::from_pressures(result.pressures.clone()).to_string()
            }
            PressureResponse::AllReadings(AllReadingsResponse::V1(result)) => {
                RelevantPressures::from_pressures(result.pressures.clone()).to_string()
            }
        }
    }

    fn render_text(&self) -> String {
        match &self.response {
            PressureResponse::Readings(ReadingsResponse::V1(result)) => {
                format!("Pressure readings for {}.", result.agent)
            }
            PressureResponse::AllReadings(_) => "Pressure readings for all agents.".to_string(),
        }
    }
}
