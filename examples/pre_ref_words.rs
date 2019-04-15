// Copyright 2015-2019 KWARC research group. See the LICENSE
// file at the top-level directory of this distribution.
//

//! Given a `CorTeX` corpus of HTML5 documents,
//! extract a frequency report over words preceding a latex \ref macro (or equivalent)
//! such as "Section \ref{sec:intro}"
//! by looking at the created span.ltx_ref or a.ltx_ref elements.

use rayon::prelude::*;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufWriter, Error};
use std::thread;

use libxml::tree::NodeType;
use llamapun::parallel_data::{Corpus, Document};

static BUFFER_CAPACITY: usize = 10_485_760;

pub fn main() -> Result<(), Error> {
  let start = time::get_time();
  // Read input arguments
  let mut document_count = 0;
  let mut input_args = env::args();
  let _ = input_args.next(); // skip process name
  let corpus_path = match input_args.next() {
    Some(path) => path,
    None => "tests/resources/".to_string(),
  };
  let node_statistics_filepath = match input_args.next() {
    Some(path) => path,
    None => "corpus_statistics_ref.csv".to_string(),
  };

  let node_statistics_file = File::create(node_statistics_filepath)?;

  let mut corpus = Corpus::new(corpus_path);

  let catalog = corpus.catalog_with_parallel_walk(|document| {
    let mut catalog = HashMap::new();
    println!(
      "Thread: {:?}, doc: {:?}",
      thread::current().name(),
      document.path
    );
    for ref_node in document.get_ref_nodes() {
      if let Some(previous) = ref_node.get_prev_sibling() {
        if previous.get_type() == Some(NodeType::TextNode) {
          let content_raw = previous.get_content();
          let mut pre_word_vec = Vec::new();
          for c in content_raw.trim_end().chars().rev() {
            if c.is_whitespace() || !c.is_alphanumeric() {
              break;
            }
            pre_word_vec.push(c.to_lowercase().to_string());
          }
          let pre_word: String = pre_word_vec.into_iter().rev().collect();
          if !pre_word.is_empty() {
            let entry = catalog.entry(pre_word).or_insert(0);
            *entry += 1;
          }
        }
      }
    }
    catalog
  });

  let end = time::get_time();
  let duration_sec = (end - start).num_milliseconds() / 1000;
  println!("---");
  println!(".ltx_ref statistics finished in {:?}s", duration_sec);

  let mut catalog_vec: Vec<(&String, &u64)> = catalog.iter().collect();
  catalog_vec.sort_by(|a, b| b.1.cmp(a.1));

  let mut node_statistics_writer = BufWriter::with_capacity(BUFFER_CAPACITY, node_statistics_file);
  node_statistics_writer.write(b"word, frequency\n")?;

  for (key, val) in catalog_vec {
    node_statistics_writer.write(key.as_bytes())?;
    node_statistics_writer.write(b", ")?;
    node_statistics_writer.write(val.to_string().as_bytes())?;
    node_statistics_writer.write(b"\n")?;
  }
  // Close the writer
  node_statistics_writer.flush()
}
