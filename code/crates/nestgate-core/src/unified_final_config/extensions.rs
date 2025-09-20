
/// Placeholder for automation extensions - will be replaced with actual import
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnifiedAutomationExtensions {
    pub workflow_enabled: bool,
    pub ml_prediction_enabled: bool,
    pub scheduling_enabled: bool,
}
/// Placeholder for fs monitor extensions - will be replaced with actual import  
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnifiedFsMonitorExtensions {
    pub real_time_monitoring: bool,
    pub event_filtering: bool,
    pub notification_enabled: bool,
}
