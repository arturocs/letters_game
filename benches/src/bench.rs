#[path = "../../src/game_core.rs"]
mod game_core;

#[path = "../../src/game_data.rs"]
mod game_data;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use game_core::{Game, Language};
use game_data::ENGLISH_DICTIONARY;

pub fn bench_parse_dictionary(c: &mut Criterion) {
    c.bench_function("parse_dictionary", |b| {
        b.iter(|| {
            black_box(Game::parse_dictionary(ENGLISH_DICTIONARY));
        })
    });
}

pub fn bench_generate_available_letters(c: &mut Criterion) {
    let mut game = Game::new(Language::English, 20);
    c.bench_function("generate_available_letters", |b| {
        b.iter(|| {
            game.generate_available_letters(100);
            black_box(&game);
        })
    });
}

pub fn bench_exist(c: &mut Criterion) {
    let game = Game::new(Language::English, 20);
    c.bench_function("exist", |b| {
        b.iter(|| {
            black_box(game.exist("nummulitidae"));
        })
    });
}

pub fn bench_is_formable(c: &mut Criterion) {
    let mut game = Game::new(Language::English, 20);
    game.available_letters = vec![
        'a', 'a', 'a', 'a', 'f', 'h', 'h', 'i', 'i', 'i', 'l', 'm', 'm', 'n', 'r', 's', 's', 't',
        'u', 'u', 'y',
    ];
    c.bench_function("is_formable", |b| {
        b.iter(|| {
            black_box(game.is_formable("nummulitidae"));
        })
    });
}

pub fn bench_find_longest_word(c: &mut Criterion) {
    let mut game = Game::new(Language::English, 20);
    game.available_letters = vec![
        'a', 'a', 'a', 'a', 'f', 'h', 'h', 'i', 'i', 'i', 'l', 'm', 'm', 'n', 'r', 's', 's', 't',
        'u', 'u', 'y',
    ];
    c.bench_function("find_longest_word", |b| {
        b.iter(|| black_box(game.find_longest_word()))
    });
}

pub fn bench_play(c: &mut Criterion) {
    c.bench_function("play", |b| {
        b.iter(|| {
            let mut game = Game::new(Language::English, 20);
            game.available_letters = vec![
                'a', 'a', 'a', 'a', 'd', 'e', 'f', 'h', 'h', 'i', 'i', 'i', 'l', 'm', 'm', 'n',
                'r', 's', 's', 't', 'u', 'u', 'y',
            ];

            black_box(game.play("nummulitidae"))
        })
    });
}

criterion_group!(
    benches,
    bench_parse_dictionary,
    bench_is_formable,
    bench_generate_available_letters,
    bench_exist,
    bench_find_longest_word,
    bench_play
);
criterion_main!(benches);
