use crate::{graylog, ApplicationArguments};
use log::warn;
use serde::{Deserialize, Serialize};
use url::form_urlencoded;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Dashboard {
    pub title: String,
    panels: Vec<Panel>,
    time: TimeRange,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TimeRange {
    from: String,
    to: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiDashboard {
    pub dashboard: Dashboard,
    #[serde(rename = "folderId")]
    pub folder_id: i64,
    pub overwrite: bool,
}

impl Dashboard {
    pub fn create_dashboard_from_graylog(
        dash: graylog::Dashboard,
        opt: &ApplicationArguments,
    ) -> Dashboard {
        Dashboard {
            title: dash.title,
            panels: dash
                .dashboard_widgets
                .into_iter()
                .filter_map(|t| Panel::create_panel(t, opt))
                .collect(),
            time: TimeRange {
                from: "now-2d".to_string(),
                to: "now".to_string(),
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PanelType {
    #[serde(rename = "graph")]
    Graph,
    #[serde(rename = "singlestat")]
    SingleStat,
    #[serde(rename = "grafana-piechart-panel")]
    PieChart,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Link {
    title: String,
    r#type: String,
    url: String,
    #[serde(rename = "targetBlank")]
    target_blank: bool,
}

impl Link {
    fn new(url: &str, query: &str, seconds: i64) -> Link {
        let encoded: String = form_urlencoded::Serializer::new(String::new())
            .append_pair("rangetype", "relative")
            .append_pair("fields", "message,source")
            .append_pair("width", "1920")
            .append_pair("highlightMessage", "")
            .append_pair("relative", &seconds.to_string())
            .append_pair("q", query)
            .finish();
        Link {
            title: "Go to Graylog".to_string(),
            r#type: "absolute".to_string(),
            url: format!("{}/search?{}", url, encoded),
            target_blank: true,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Panel {
    r#type: PanelType,
    title: String,
    links: Vec<Link>,
    datasource: String,
    targets: Vec<PanelTarget>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bars: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lines: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    points: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sparkline: Option<Sparkline>,
    #[serde(rename = "gridPos")]
    grid_pos: GridPos,
    #[serde(rename = "valueName", skip_serializing_if = "Option::is_none")]
    value_name: Option<String>,
    #[serde(rename = "timeFrom", skip_serializing_if = "Option::is_none")]
    time_from: Option<String>,
}

impl Panel {
    fn new_graph(
        title: String,
        targets: Vec<PanelTarget>,
        renderer: graylog::ChartRenderer,
        grid_pos: GridPos,
        opt: &ApplicationArguments,
    ) -> Panel {
        Panel {
            title,
            r#type: PanelType::Graph,
            datasource: opt.datasource.clone(),
            targets,
            bars: Some(renderer == graylog::ChartRenderer::Bar),
            lines: Some(
                renderer == graylog::ChartRenderer::Line
                    || renderer == graylog::ChartRenderer::Area,
            ),
            points: Some(renderer == graylog::ChartRenderer::Area),
            sparkline: None,
            grid_pos,
            value_name: None,
            time_from: None,
            links: vec![],
        }
    }

    fn new(
        title: String,
        r#type: PanelType,
        query: &str,
        sparkline: Option<Sparkline>,
        targets: Vec<PanelTarget>,
        grid_pos: GridPos,
        range: i64,
        opt: &ApplicationArguments,
    ) -> Panel {
        Panel {
            title,
            r#type,
            value_name: Some("total".to_string()),
            datasource: opt.datasource.clone(),
            targets,
            bars: None,
            lines: None,
            points: None,
            sparkline,
            grid_pos,
            time_from: Some(format!("{}h", range / 3600)),
            links: vec![Link::new(&opt.graylog_url, query, range)],
        }
    }

    pub fn create_panel(
        widget: graylog::DashboardWidget,
        opt: &ApplicationArguments,
    ) -> Option<Panel> {
        let grid_pos = GridPos::new_with_widget(&widget);

        let panel = match widget.r#type {
            graylog::DashboardWidgetType::FieldChart => {
                let configuration = widget.configuration;
                Panel::new_graph(
                    widget.description,
                    vec![PanelTarget::new(
                        &configuration.query.unwrap(),
                        configuration.interval.unwrap().grafana(),
                        "A",
                        configuration.field.unwrap(),
                        configuration.valuetype.unwrap(),
                    )],
                    configuration.renderer.unwrap(),
                    grid_pos,
                    opt,
                )
            }
            graylog::DashboardWidgetType::StackedChart => {
                let configuration = widget.configuration;
                let interval = configuration.interval.unwrap().grafana();
                Panel::new_graph(
                    widget.description,
                    configuration
                        .series
                        .unwrap()
                        .iter()
                        .map(|s| {
                            PanelTarget::new(
                                &s.query,
                                interval.to_string(),
                                "A",
                                s.field.clone(),
                                s.statistical_function.clone(),
                            )
                        })
                        .collect(),
                    configuration.renderer.unwrap(),
                    grid_pos,
                    opt,
                )
            }
            graylog::DashboardWidgetType::SearchResultCount => {
                let configuration = widget.configuration;
                let query = &configuration.query.unwrap();
                Panel::new(
                    widget.description,
                    PanelType::SingleStat,
                    query,
                    Some(Sparkline::new(configuration.trend.unwrap())),
                    vec![PanelTarget::new(
                        query,
                        "1m",
                        "A",
                        "select field".to_string(),
                        "count".to_string(),
                    )],
                    grid_pos,
                    configuration.timerange.range,
                    opt,
                )
            }
            graylog::DashboardWidgetType::SearchResultChart => {
                let configuration = widget.configuration;
                Panel::new_graph(
                    widget.description,
                    vec![PanelTarget::new(
                        &configuration.query.unwrap(),
                        configuration.interval.unwrap().grafana(),
                        "A",
                        "select field".to_string(),
                        "count".to_string(),
                    )],
                    graylog::ChartRenderer::Bar,
                    grid_pos,
                    opt,
                )
            }
            graylog::DashboardWidgetType::QuickValues => {
                let configuration = widget.configuration;
                let query = &configuration.query.unwrap();
                Panel::new(
                    widget.description,
                    PanelType::PieChart,
                    query,
                    None,
                    vec![PanelTarget::new_buckets(
                        query,
                        &configuration.field.unwrap(),
                        configuration.sort_order,
                        configuration.limit,
                    )],
                    grid_pos,
                    configuration.timerange.range,
                    opt,
                )
            }
            graylog::DashboardWidgetType::QuickValuesHistogram => {
                warn!(
                    "Not Supported {:?} graph: {}",
                    graylog::DashboardWidgetType::QuickValuesHistogram,
                    widget.description
                );
                return None;
            }
        };
        Some(panel)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sparkline {
    show: bool,
    full: bool,
    #[serde(rename = "lineColor")]
    line_color: String,
    #[serde(rename = "fillColor")]
    fill_color: String,
}

impl Sparkline {
    fn new(trend: bool) -> Sparkline {
        Sparkline {
            show: trend,
            full: false,
            line_color: "rgb(31, 120, 193)".to_string(),
            fill_color: "rgba(31, 118, 189, 0.18)".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GridPos {
    h: i64,
    w: i64,
    y: i64,
    x: i64,
}

impl GridPos {
    fn new_with_widget(widget: &graylog::DashboardWidget) -> GridPos {
        GridPos::new(widget.row, widget.col, widget.width, widget.height)
    }
    fn new(row: i64, col: i64, width: i64, height: i64) -> GridPos {
        GridPos {
            h: height * 6,
            w: width * 5,
            y: row * 6,
            x: (col - 1) * 5,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PanelTarget {
    #[serde(rename = "refId")]
    ref_id: String,
    metrics: Vec<PanelTargetMetric>,
    #[serde(rename = "bucketAggs")]
    bucket_aggs: Vec<PanelBucketAgg>,
    #[serde(rename = "timeField")]
    time_field: String,
    query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    alias: Option<String>,
}

impl PanelTarget {
    fn new<T1, T2>(
        query: &str,
        interval: T1,
        ref_id: T2,
        field: String,
        valuetype: String,
    ) -> PanelTarget
    where
        T1: Into<String>,
        T2: Into<String>,
    {
        PanelTarget {
            ref_id: ref_id.into(),
            metrics: vec![PanelTargetMetric {
                r#type: if valuetype == "count" {
                    "count".to_string()
                } else {
                    "sum".to_string()
                },
                id: "1".to_string(),
                field,
            }],
            bucket_aggs: vec![PanelBucketAgg::new_date_histogram(interval.into(), None)],
            time_field: "timestamp".to_string(),
            query: query.to_string(),
            alias: Some(query.to_string()),
        }
    }

    fn new_buckets(
        query: &str,
        field: &str,
        sort_order: Option<String>,
        limit: Option<i64>,
    ) -> PanelTarget {
        PanelTarget {
            ref_id: "A".to_string(),
            metrics: vec![PanelTargetMetric {
                r#type: "count".to_string(),
                id: "1".to_string(),
                field: "select field".to_string(),
            }],
            bucket_aggs: vec![
                PanelBucketAgg::new_terms(
                    field,
                    sort_order.or_else(|| Some("desc".to_string())),
                    limit.unwrap_or(5),
                ),
                PanelBucketAgg::new_date_histogram("1h".to_string(), true),
            ],
            time_field: "timestamp".to_string(),
            query: query.to_string(),
            alias: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PanelTargetMetric {
    r#type: String,
    id: String,
    field: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PanelBucketAgg {
    r#type: String,
    id: String,
    settings: PanelBucketAggSettings,
    field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    fake: Option<bool>,
}
impl PanelBucketAgg {
    fn new_date_histogram<T1, T2>(interval: T1, fake: T2) -> PanelBucketAgg
    where
        T1: Into<Option<String>>,
        T2: Into<Option<bool>>,
    {
        PanelBucketAgg {
            r#type: "date_histogram".to_string(),
            id: "2".to_string(),
            settings: PanelBucketAggSettings {
                interval: interval.into(),
                order: None,
                size: None,
                min_doc_count: 0,
                trim_edges: 0,
                order_by: None,
            },
            field: "timestamp".to_string(),
            fake: fake.into(),
        }
    }
    fn new_terms(field: &str, order: Option<String>, limit: i64) -> PanelBucketAgg {
        PanelBucketAgg {
            r#type: "terms".to_string(),
            field: field.to_string(),
            id: "1".to_string(),
            settings: PanelBucketAggSettings {
                interval: None,
                order,
                size: Some("0".to_string()),
                min_doc_count: limit,
                trim_edges: 0,
                order_by: Some("_term".to_string()),
            },
            fake: Some(true),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PanelBucketAggSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    interval: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<String>,
    min_doc_count: i64,
    #[serde(rename = "trimEdges")]
    trim_edges: i64,
    #[serde(rename = "orderBy", skip_serializing_if = "Option::is_none")]
    order_by: Option<String>,
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn panel_bucket_agg_new_date_histogram_with_fake_interval() {
        let bucket = PanelBucketAgg::new_date_histogram("".to_string(), true);

        assert_eq!(Some(true), bucket.fake);
        assert_eq!(Some("".to_string()), bucket.settings.interval);
    }

    #[test]
    fn panel_bucket_agg_new_date_histogram_without_fake_interval() {
        let bucket = PanelBucketAgg::new_date_histogram(None, None);

        assert_eq!(None, bucket.fake);
        assert_eq!(None, bucket.settings.interval);
    }
}
