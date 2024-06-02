#![allow(dead_code)]

use common::{
  blitz_data::{PrimitiveType, Statisitic},
  shape_util::ShapeUtil
};

use hlo::hlo_instruction::HloInstruction;

// Different formats that a graph can be packaged as.
pub enum RenderedGraphFormat {
  Dot,
  Html,
  Url,
}

pub struct HloRenderOptions {
  shapw_backend_config: bool,
  show_fusion_subcomputation: bool,
  show_while_subcomputation: bool,
  override_node_colors: bool
}

// Contains color computed according to the numerical diff of an HloInstruction.
pub struct ColorStats {
  color: String,
  stats: String
}

// Used to indicate how we should treat a given HloInstruction in the graph.
#[derive(Debug, Clone, PartialEq)]
enum NodeFilterResult {
  NormalNode,
  HideNode,
  HighlightNode,
  SomeOperandsOmitted,
  OmitNodeOperands,
  SomeUsersOmitted,
}

// NodeFilter is essentially a map from &HloInstructions to NodeFilterResult.
struct NodeFilter {
  filter: Box<dyn Fn(&HloInstruction) -> NodeFilterResult>,
  num_rendered: Option<i64>
}

impl NodeFilter {
  pub fn new(
    filter: Box<dyn Fn(&HloInstruction) -> NodeFilterResult>,
    num_rendered: Option<i64>) -> Self
  {
    NodeFilter {
      filter: filter,
      num_rendered: num_rendered
    }
  }

  pub fn show(&self, instruction: &HloInstruction) -> bool {
    self.filter.as_ref()(instruction) != NodeFilterResult::HideNode
  }

  pub fn highlight(&self, instruction: &HloInstruction) -> bool {
    self.filter.as_ref()(instruction) == NodeFilterResult::HighlightNode
  }

  pub fn omit_operands(&self, instruction: &HloInstruction) -> bool {
    self.filter.as_ref()(instruction) == NodeFilterResult::OmitNodeOperands
  }

  pub fn some_or_all_operands_omitted(&self, instruction: &HloInstruction) -> bool {
    let result = self.filter.as_ref()(instruction);
    result == NodeFilterResult::OmitNodeOperands ||
    result == NodeFilterResult::SomeOperandsOmitted
  }

  pub fn deemphasized(&self, instruction: &HloInstruction) -> bool {
    let result = self.filter.as_ref()(instruction);
    result == NodeFilterResult::OmitNodeOperands ||
    result == NodeFilterResult::SomeOperandsOmitted ||
    result == NodeFilterResult::SomeUsersOmitted
  }

  // Returns a optionally recorded number of nodes which will be rendered.
  pub fn get_num_rendered(&self) -> &Option<i64> {
    &self.num_rendered
  }
}

// We arbitraily set this as the boundary between "large" and "small" insstructions.
fn is_small(instruction: &HloInstruction) -> bool {
  if ShapeUtil::has_primitive_type(
      instruction.shape(), &PrimitiveType::OpaqueType) ||
     ShapeUtil::has_primitive_type(
      instruction.shape(), &PrimitiveType::Token)
  {
    return true;
  }
  ShapeUtil::elements_in_recursive(instruction.shape()) < 4096
}

// Node color schemes, used by NodeColorAttributes.
enum ColorScheme {
  Blue,
  Brown,
  DarkBlue,
  DarkGreen,
  DarkOrange,
  DarkRed,
  Gray,
  Green,
  Orange,
  Purple,
  Red,
  White,
  Yellow,
  DashedBorder,
}

// Graphviz attributes/colors that make up a color scheme.
struct NodeColors {
  style: String,
  fill_color: String,
  stroke_color: String,
  font_color: String,
}

impl NodeColors {
  pub fn new(
    style: &str,
    fill_color: &str,
    stroke_color: &str,
    font_color: &str) -> Self
  {
    NodeColors {
      style: style.to_string(),
      fill_color: fill_color.to_string(),
      stroke_color: stroke_color.to_string(),
      font_color: font_color.to_string()
    }
  }
}

fn node_colors_for_scheme(color: ColorScheme) -> NodeColors {
  match color {
    ColorScheme::Blue => NodeColors::new("filled", "#bbdefb",
      "#8aacc8", "black"),
    ColorScheme::Brown => NodeColors::new("filled", "#bcaaa4",
      "#8c7b75", "black"),
    ColorScheme::DarkBlue => NodeColors::new("filled", "#1565c0",
      "#003c8f", "white"),
    ColorScheme::DarkGreen => NodeColors::new("filled", "#2e7d32",
      "#005005", "white"),
    ColorScheme::DarkOrange => NodeColors::new("filled", "#ffb74d",
      "#7f0000", "white"),
    ColorScheme::DarkRed => NodeColors::new("filled", "#b71c1c",
      "#8aacc8", "black"),
    ColorScheme::Gray => NodeColors::new("filled", "#cfd8dc",
      "#9ea7aa", "black"),
    ColorScheme::Green => NodeColors::new("filled", "#c8e6c9",
      "#97b498", "black"),
    ColorScheme::Orange => NodeColors::new("filled", "#ffe0b2",
      "#cbae82", "black"),
    ColorScheme::Purple => NodeColors::new("filled", "#e1bee7",
      "#af8eb5", "black"),
    ColorScheme::Red => NodeColors::new("filled", "#ffcdd2",
      "#cb9ca1", "black"),
    ColorScheme::White => NodeColors::new("filled", "white",
      "#9e9e9e", "black"),
    ColorScheme::Yellow => NodeColors::new("filled", "#fff9c4",
      "#cbc693", "black"),
    ColorScheme::DashedBorder => NodeColors::new("filled,dashed", "white",
      "#757575", "#757575")
  }
}

// Given a Statistic object, returns a hex string for the fill color of the node
// with that statistic.
fn node_fill_color_for_statistic(statistic: &Statisitic) -> String {
  let stat_val = statistic.stat_val();
  if stat_val == 0 {
    return "#f5f5f5".to_string();
  } else if stat_val < 10 {
    return "#f7d4cc".to_string();
  } else if stat_val < 20 {
    return "#f8b2a3".to_string();
  } else if stat_val < 30 {
    return "#f9a28f".to_string();
  } else if stat_val < 40 {
    return "#fa917b".to_string();
  } else if stat_val < 50 {
    return "#fb8066".to_string();
  } else if stat_val < 60 {
    return "#fc7052".to_string();
  } else if stat_val < 70 {
    return "#fd5f3d".to_string();
  } else if stat_val < 80 {
    return "#fd4e29".to_string();
  } else if stat_val < 90 {
    return "#fe3e14".to_string();
  } else {
    return "#ff2d00".to_string();
  }
}

// Given a Statistic object, returns a hex string for the font color of the node
// with that statistic.
fn node_font_color_for_statistic(statistic: &Statisitic) -> String {
  if statistic.stat_val() < 60 {
    return "black".to_string();
  } else {
    return "white".to_string();
  }
}

pub fn render_graph() {}

pub fn render_all_computations_to_html() {}

pub fn render_all_paths_from_to() {}

pub fn register_fusion_state() {}

pub fn register_graph_to_url_renderer() {}

pub fn warp_fusion_explorer() {}