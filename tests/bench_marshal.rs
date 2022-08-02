#![feature(test)]
#![feature(bench_black_box)]

extern crate test;
mod test_data;
use std::hint::black_box;

use json::{self, Marshalable, Unmarshalable};
use test::Bencher;

fn marshal_repeatedly(m: &json::Element) {
    for _ in 0..1000 {
        m.marshal_json();
    }
}

#[bench]
fn bench_glossary(b: &mut Bencher) {
    let marshal = black_box(json::Element::unmarshal_json(test_data::GLOSSARY.chars()).unwrap());
    b.iter(|| marshal_repeatedly(&marshal));
}

#[bench]
fn bench_menu1(b: &mut Bencher) {
    let marshal = black_box(json::Element::unmarshal_json(test_data::MENU1.chars()).unwrap());
    b.iter(|| marshal_repeatedly(&marshal));
}

#[bench]
fn bench_widget(b: &mut Bencher) {
    let marshal = black_box(json::Element::unmarshal_json(test_data::WIDGET.chars()).unwrap());
    b.iter(|| marshal_repeatedly(&marshal));
}

#[bench]
fn bench_web_app(b: &mut Bencher) {
    let marshal = black_box(json::Element::unmarshal_json(test_data::WEB_APP.chars()).unwrap());
    b.iter(|| marshal_repeatedly(&marshal));
}

#[bench]
fn bench_menu2(b: &mut Bencher) {
    let marshal = black_box(json::Element::unmarshal_json(test_data::MENU2.chars()).unwrap());
    b.iter(|| marshal_repeatedly(&marshal));
}
