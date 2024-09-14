#![allow(dead_code)]

use std::{cmp::Ordering, collections::HashMap};

// Class for creating a text format table showing entries with a metric
// (e.g. cycles) and a text (e.g. name of function taking that many
// cycles). Entries are grouped by a category and sorted in decreasing order of
// the metric.
//
// Example of a categories table generated using this class:
//
// ********** microseconds report **********
// There are 3,912,517 microseconds in total.
// There are 123 microseconds ( 0.00%) not accounted for by the data.
// There are 3002 ops.
//
// ********** categories table **********
// The left hand side numbers are microseconds.
// 1,749,414 (44.71% Σ44.72%)   convolution (206 ops)
//                             * 10.51% %convolution.202
//                             * 10.51% %convolution.204
//                             * 10.51% %convolution.203
//                             * ... (203 more ops)
//   884,939 (22.62% Σ67.33%)   convolution window-dilated (7 ops)
//                             *  7.50% %convolution-window-dilated.7
// [...]
//
// The entry table is similar, it just has the entries directly as the entries
// instead of grouping by categories first.
#[derive(Debug, Clone)]
pub struct Entry {
  text: String,
  short_text: String,
  category_text: String,
  metric: f64,
}

// Represents a set of entries with the same category_text.
#[derive(Debug, Clone)]
struct Category {
  category_text: String,
  metric_sum: f64,
  entries: Vec<Entry>,
}

impl Category {
  pub fn new(category_text: String, metric_sum: f64) -> Self {
    Category {
      category_text: category_text,
      metric_sum: metric_sum,
      entries: Vec::new()
    }
  }
}

pub struct MetricTableReport {
  entries: Vec<Entry>,
  expected_metric_sum: f64,
  metric_name: String,
  entry_name: String,
  show_category_table: bool,
  show_entry_table: bool,
  max_entries_to_show: usize,
  max_entries_per_category_to_show: i64,
  max_metric_proportion_to_show: f64,
  report: String,
}

impl MetricTableReport {
  const DEFAULT_MAX_METRIC_PROPORTION_TO_SHOW: f64 = 0.99;
  const DEFAULT_MAX_ENTRIES_TO_SHOW: usize = 100;
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

  // The default name for the metric is "units", this function allows setting a
  // more meaningful name.
  pub fn set_metric_name(&mut self, metric_name: String) {
    self.metric_name = metric_name;
  }

  // The default name for referring to entries is "entries", this functions
  // allows setting a more meaningful name.
  pub fn set_entry_name(&mut self, entry_name: String) {
    self.entry_name = entry_name;
  }

  // By default the size of the table is limited. Calling this function forces
  // all entries to be shown no matter how many there are.
  pub fn set_show_all_entries(&mut self) {
    self.max_entries_to_show = usize::MAX;
    self.max_entries_per_category_to_show = i64::MAX;
    self.max_metric_proportion_to_show = 1.1;
  }

  // Set option to show a table with data on the categories of entries.
  pub fn set_show_category_table(&mut self) {
    self.show_category_table = true;
  }

  // Set option to show a table with data on the entries.
  pub fn set_show_entry_table(&mut self) {
    self.show_entry_table = true;
  }

  // Returns the report as a string. expected_metric_sum is the expected sum of
  // the metric across the entries. It is not an error for the actual sum to be
  // different from the expectation - the report will include the
  // discrepancy. All metric percentages are for the ratio with respect to the
  // expected sum, not the actual sum.
  pub fn make_report(&mut self, expected_metric_sum: f64) -> String {
    self.expected_metric_sum = expected_metric_sum;
    self.report.clear();

    // Sort the entries.
    self.entries.sort_by(|a, b| {
      if a.metric > b.metric {
        return Ordering::Greater;
      } else if a.metric == b.metric {
        return Ordering::Equal;  
      } else {
        return Ordering::Less;
      }
    });

    // Create the report
    self.append_line(None);
    self.append_header();

    if self.show_category_table {
      self.append_line(None);
      self.append_category_table();
    }
    if self.show_entry_table {
      self.append_line(None);
      self.append_entry_table();
    }
    self.append_line(None);

    self.report.clone()
  }

  // As MakeReport(), but writes the report to the INFO log in a way that avoids
  // cutting the report off if it is longer than the maximum line length for a
  // logged line. Individual lines in the report may still be cut off, but they
  // would have to be very long for that to happen.
  pub fn write_report_to_info_log(&mut self, expected_metric_sum: f64) {
    // Write something to the log normally to get the date-time and file prefix.
    println!("Writing report to log.");

    let mut pos = 0;
    let report = self.make_report(expected_metric_sum);
    let report_size = report.len();

    while pos < report_size {
      let mut end_of_line = report.find('\n');
      if end_of_line.is_none() {
        end_of_line = Some(report.len());
      }
      pos = end_of_line.unwrap() + 1;
    }
  }

  // Append all parameters to the report.
  fn append_line(&mut self, param: Option<&String>) {
    if param.is_some() {
      self.report += param.unwrap();  
    }
    self.report += "\n";
  }

  // Returns a vector of categories of entries with the same category_text. The
  // vector is sorted in order of decreasing metric sum.
  //
  // The returned categories contain pointers into the entries parameter. The
  // style guide requires parameters to which references/pointers are retained
  // to be taken by pointer, even for const parameters, so that is why entries
  // is taken by pointer.
  fn make_categories(entries: &Vec<Entry>) -> Vec<Category> {
    // Create the categories using a category_text -> category map.
    let mut category_map: HashMap<String, Category> = HashMap::new();
    for entry in entries {
      let mut category =
        Category::new(entry.category_text.clone(), entry.metric);
      category.entries.push(entry.clone());
      category_map.insert(entry.category_text.clone(), category);
    }

    // Move the categories to a vector.
    let mut categories: Vec<Category> = Vec::new();
    categories.reserve(category_map.len());
    for key_value_pair in &category_map {
      categories.push(key_value_pair.1.clone());
      categories.last_mut().unwrap().category_text = key_value_pair.0.clone();
    }

    // Sort the categories.
    categories.sort_by(|a, b| {
      if a.metric_sum > b.metric_sum {
        return Ordering::Greater;
      } else if a.metric_sum == b.metric_sum {
        return Ordering::Equal;
      } else {
        return Ordering::Less;
      }
    });

    categories
  }

  // Append a header to the report.
  fn append_header(&mut self) {
    let mut str = "********** ".to_string() +  &self.metric_name
      + " report **********";
    self.append_line(Some(&str));

    str = "There are ".to_string() + &self.metric_string(self.expected_metric_sum)
      + " " + &self.metric_name + " in total.";
    self.append_line(Some(&str));

    str = "There are ".to_string() + &self.metric_string(self.unaccounted_metric()) + " "
      + &self.metric_name + " (" + &self.metric_percent(self.unaccounted_metric());
    self.append_line(Some(&str));

    str = "There are ".to_string() + &self.entries.len().to_string() + " "
      + &self.entry_name + ".";
    self.append_line(Some(&str));
  }

  // Append a table of categories to the report.
  fn append_category_table(&mut self) {
    let categories = MetricTableReport::make_categories(&self.entries);

    let str = "********** categories table for ".to_string()
      + &self.metric_name + " **********";
    self.append_line(Some(&str));
    self.append_line(None);

    let mut metric_sum = self.unaccounted_metric();
    let mut categories_shown = 0;
    for category in &categories {
      if categories_shown >= self.max_entries_to_show ||
        metric_sum / self.expected_metric_sum > self.max_metric_proportion_to_show
      {
        break;
      }
      categories_shown += 1;
      metric_sum += category.metric_sum;

      // Show the category.
      let mut text = category.category_text.clone();
      if text.is_empty() {
        text = "[no category]".to_string();
      }
      text += " (";
      text += &category.entries.len().to_string();
      text += " ";
      text += &self.entry_name;
      text += ")";
      self.append_table_row(&text, category.metric_sum, metric_sum);

      // Show the top entries in the category.
      let mut entries_to_show = self.max_entries_per_category_to_show as usize;
      if entries_to_show  < category.entries.len() {
        entries_to_show = category.entries.len();
      }
      if entries_to_show + 1 == category.entries.len() {
        // May as well show the last entry on the line that would otherwise say
        // that there is a single entry not shown.
        entries_to_show += 1;
      }
      for i in 0..entries_to_show {
        let str = "                              * ".to_string()
          + &self.metric_percent(category.entries[i].metric).to_string() + " "
          + &category.entries[i].short_text;
        self.append_line(Some(&str));
      }
      let remaining_entries = category.entries.len() - entries_to_show;
      if remaining_entries > 0 {
        let str = "                              * ".to_string()
          + "... (" + &remaining_entries.to_string() + " more " + &self.entry_name + ")";
        self.append_line(Some(&str));
      }
    }

    let remaining_categories = categories.len() - categories_shown;
    if remaining_categories > 0 {
      let str = "... (".to_string() + &remaining_categories.to_string()
        + "more categories)";
      self.append_table_row(&str,
        self.expected_metric_sum - metric_sum,
        self.expected_metric_sum)
    }
  }

  // Append a table of entries to the report.
  fn append_entry_table(&mut self) {
    let str = "********** ".to_string() + &self.entry_name + " table for "
      + &self.metric_name + " **********";
    self.append_line(Some(&str));
    self.append_line(None);

    let mut metric_sum = self.unaccounted_metric();
    let mut entries_shown = 0;
    let mut entries = vec![];
    entries.clone_from(&self.entries);
    for entry in &entries {
      if entries_shown >= self.max_entries_to_show ||
        metric_sum / self.expected_metric_sum > self.max_metric_proportion_to_show
      {
        break;
      }
      entries_shown += 1;
      metric_sum += entry.metric;

      let mut text = entry.text.clone();
      if text.is_empty() {
        text = "[no entry text]".to_string();
      }
      self.append_table_row(&text, entry.metric, metric_sum);
    }
    let remaining_entries = self.entries.len() - entries_shown;
    if remaining_entries > 0 {
      let str = "... (".to_string() + &remaining_entries.to_string() + " more "
        + &self.entry_name + ")";
      self.append_table_row(&str,
        self.expected_metric_sum - metric_sum,
        self.expected_metric_sum);
    }
  }

  // Appends a row of a table to the report.
  fn append_table_row(&mut self, text: &String, metric: f64, running_metric_sum: f64) {
    // This is the widest metric number possible, assuming non-negative metrics,
    // so align to that width.
    let max_metric_string_size =
      self.metric_string(self.expected_metric_sum).len();
    let metric_string = self.metric_string(metric);

    // Don't try to make a gigantic string and crash if expected_metric_sum_ is
    // wrong somehow.
    let mut padding_len = 1;
    let metric_string_size = metric_string.len();
    if max_metric_string_size >= metric_string_size {
      padding_len += max_metric_string_size - metric_string.len();
    }

    let str = padding_len.to_string() + " " + &metric_string + " ("
      + &self.metric_percent(metric) + " Σ" + &self.metric_percent(running_metric_sum)
      + ")   " + text;
    self.append_line(Some(&str));
  }

  // Returns the discrepancy between the expected sum of the metric of the
  // entries and the actual sum.
  fn unaccounted_metric(&self) -> f64 {
    let mut metric_sum = 0.0;
    for entry in &self.entries {
      metric_sum += entry.metric;
    }
    self.expected_metric_sum - metric_sum
  }

  // Formats the metric value as a string.
  fn metric_string(&self, _metric: f64) -> String {
    unimplemented!()
  }

  // Returns a string representing the metric value as a proportion of the
  // expected metric sum.
  fn metric_percent(&self, metric: f64) -> String {
    let num = metric / self.expected_metric_sum * 100.0;
    num.to_string()
  }
}