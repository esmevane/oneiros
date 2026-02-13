use oneiros_outcomes::Outcome;

#[derive(Clone, Outcome)]
pub enum StopServiceOutcomes {
    #[outcome(message("Service stopped."))]
    ServiceStopped,
}
