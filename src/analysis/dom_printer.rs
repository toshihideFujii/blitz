#![allow(dead_code)]

// This file defines external functions that can be called to
// explicitly instantiate the dominance tree printer.

struct DomViewer {}

struct DomOnlyViewer {}

struct PostDomViewer {}

struct PostDomOnlyViewer {}

struct DomPrinter {}

struct DomOnlyPrinter {}

struct PostDomPrinter {}

struct PostDomOnlyPrinter {}

pub fn create_dom_printer_wrapper_pass_pass() {}
pub fn create_dom_only_printer_wrapper_psss_pass() {}
pub fn create_dom_viewer_wrapper_pass_pass() {}
pub fn create_dom_only_viewer_wrapper_pass_pass() {}
pub fn create_post_dom_printer_wrapper_pass_pass() {}
pub fn create_post_dom_only_printer_wrapper_pass_pass() {}
pub fn create_post_dom_viewer_wrapper_pass_pass() {}
pub fn create_post_dom_only_viewer_wrapper_pass_pass() {}