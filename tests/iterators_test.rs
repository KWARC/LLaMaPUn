extern crate llamapun;
extern crate senna;
use llamapun::data::{Corpus, Document};
use senna::pos::POS;

#[test]
fn can_iterate_corpus() {
  let mut corpus = Corpus::new("tests".to_string());
  let mut word_count = 0;
  let mut doc_count = 0;
  for mut document in corpus.iter() {
    doc_count += 1;
    for mut paragraph in document.paragraph_iter() {
      for mut sentence in paragraph.iter() {
        for word in sentence.simple_iter() {
          word_count += 1;
          assert!(!word.range.is_empty());
          assert!(word.pos == POS::NOT_SET);
        }
      }
    }
  }
  println!("Words iterated on: {:?}", word_count);
  assert_eq!(doc_count, 2, "expected 2 documents, found {:?}", doc_count);
  assert!(
    word_count > 8400,
    "expected more than 8400 words, found {:?}",
    word_count
  );
}

#[test]
fn can_load_document_directly() {
  let corpus = Corpus::new(".".to_string());
  let mut word_count = 0;
  let mut document = Document::new("tests/resources/0903.1000.html".to_string(), &corpus).unwrap();
  for mut paragraph in document.paragraph_iter() {
    for mut sentence in paragraph.iter() {
      for word in sentence.simple_iter() {
        word_count += 1;
        assert!(!word.range.is_empty());
        assert!(word.pos == POS::NOT_SET);
      }
    }
  }
  assert!(
    word_count > 1500,
    "expected more than 1500 words, found {}",
    word_count
  );
}

#[test]
fn can_iterate_sentences_directly() {
  let corpus = Corpus::new(".".to_string());
  let mut word_count = 0;
  let mut document = Document::new("tests/resources/0903.1000.html".to_string(), &corpus).unwrap();
  for mut sentence in document.sentence_iter() {
    for word in sentence.simple_iter() {
      word_count += 1;
      assert!(!word.range.is_empty());
      assert!(word.pos == POS::NOT_SET);
    }
  }
  assert!(
    word_count > 1500,
    "expected more than 1500 words, found {}",
    word_count
  );
}

#[test]
fn can_senna_iterate_corpus() {
  let mut corpus = Corpus::new("tests".to_string());
  let mut word_count = 0;
  let mut doc_count = 0;
  for mut document in corpus.iter() {
    doc_count += 1;
    for mut paragraph in document.paragraph_iter() {
      for mut sentence in paragraph.iter() {
        for word in sentence.senna_iter() {
          word_count += 1;
          assert!(!word.range.is_empty());
          assert!(word.pos != POS::NOT_SET);
        }
      }
    }
  }
  assert_eq!(doc_count, 2, "expected 2 documents, found {:?}", doc_count);
  assert!(
    word_count > 9700,
    "expected more than 9700 words, found {:?}",
    word_count
  );
}
