#![allow(dead_code)]

pub struct Entry {}

struct Category {
  category_text: String,
  metric_sum: f64,
  entries: Vec<Entry>,
}

pub struct MetricTableReport {
  entries: Vec<Entry>,
  expected_metric_sum: f64,
  metric_name: String,
  entry_name: String,
  show_category_table: bool,
  show_entry_table: bool,
  max_entries_to_show: i64,
  max_entries_per_category_to_show: i64,
  max_metric_proportion_to_show: f64,
  report: String,
}

impl MetricTableReport {
  const DEFAULT_MAX_METRIC_PROPORTION_TO_SHOW: f64 = 0.99;
  const DEFAULT_MAX_ENTRIES_TO_SHOW: i64 = 100;
  const DEFAULT_MAX_ENTRIES_PER_CATEGORY_TO_SHOW: i64 = 5;

  pub fn new() -> Self {
    MetricTableReport {
      entries: Vec::new(),
      expected_metric_sum: 0.0,
      metric_name: "units".to_string(),
      entry_name: "entries".to_string(),
      show_category_table: false,
      show_entry_table: false,
      max_entries_to_show: MetricTableReport::DEFAULT_MAX_ENTRIES_TO_SHOW,
      max_entries_per_category_to_show: MetricTableReport::DEFAULT_MAX_ENTRIES_PER_CATEGORY_TO_SHOW,
      max_metric_proportion_to_show: MetricTableReport::DEFAULT_MAX_METRIC_PROPORTION_TO_SHOW,
      report: "".to_string(),
    }
  }

  pub fn add_entry(&mut self, entry: Entry) {
    self.entries.push(entry);
  }

  pub fn set_metric_name(&mut self, metric_name: String) {
    self.metric_name = metric_name;
  }

  pub fn set_entry_name(&mut self, entry_name: String) {
    self.entry_name = entry_name;
  }

  pub fn set_show_all_entries(&mut self) {
    self.max_entries_to_show = i64::MAX;
    self.max_entries_per_category_to_show = i64::MAX;
    self.max_metric_proportion_to_show = 1.1;
  }

  pub fn set_show_category_table(&mut self) {
    self.show_category_table = true;
  }

  pub fn set_show_entry_table(&mut self) {
    self.show_entry_table = true;
  }

  pub fn make_report() {}
  pub fn write_report_to_info_log() {}

  fn append_line() {}
  fn make_categories() {}
  fn append_header() {}
  fn append_category_table() {}
  fn append_entry_table() {}
  fn append_table_row() {}
  fn unaccounted_metric() {}
  fn metric_string() {}
  fn metric_percent() {}
}