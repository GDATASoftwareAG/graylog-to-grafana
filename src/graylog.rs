use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContentPack {
    pub name: String,
    pub dashboards: Vec<Dashboard>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Dashboard {
    pub title: String,
    pub description: String,
    pub dashboard_widgets: Vec<DashboardWidget>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DashboardWidgetType {
    #[serde(rename = "SEARCH_RESULT_COUNT")]
    SearchResultCount,
    #[serde(rename = "SEARCH_RESULT_CHART")]
    SearchResultChart,
    #[serde(rename = "QUICKVALUES")]
    QuickValues,
    #[serde(rename = "FIELD_CHART")]
    FieldChart,
    #[serde(rename = "STACKED_CHART")]
    StackedChart,
    #[serde(rename = "QUICKVALUES_HISTOGRAM")]
    QuickValuesHistogram,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DashboardWidget {
    pub description: String,
    pub r#type: DashboardWidgetType,
    pub configuration: DashboardWidgetConfiguration,
    pub row: i64,
    pub col: i64,
    pub height: i64,
    pub width: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DashboardWidgetConfiguration {
    pub query: Option<String>,
    pub valuetype: Option<String>,
    pub interval: Option<DashboardWidgetConfigSearchResultChartInterval>,
    pub renderer: Option<ChartRenderer>,
    pub field: Option<String>,
    pub series: Option<Vec<DashboardWidgetConfigStackedChartSerie>>,
    pub timerange: TimeRange,
    pub trend: Option<bool>,
    pub sort_order: Option<String>,
    pub limit: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ChartRenderer {
    #[serde(rename = "bar")]
    Bar,
    #[serde(rename = "line")]
    Line,
    #[serde(rename = "area")]
    Area,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DashboardWidgetConfigStackedChartSerie {
    pub query: String,
    pub field: String,
    pub statistical_function: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimeRange {
    pub range: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DashboardWidgetConfigSearchResultChartInterval {
    #[serde(rename = "week")]
    Week,
    #[serde(rename = "minute")]
    Minute,
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "hour")]
    Hour,
}

impl DashboardWidgetConfigSearchResultChartInterval {
    pub fn grafana(&self) -> String {
        match self {
            DashboardWidgetConfigSearchResultChartInterval::Week => "7d".to_string(),
            DashboardWidgetConfigSearchResultChartInterval::Minute => "1m".to_string(),
            DashboardWidgetConfigSearchResultChartInterval::Day => "1d".to_string(),
            DashboardWidgetConfigSearchResultChartInterval::Hour => "1h".to_string(),
        }
    }
}
