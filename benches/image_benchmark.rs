use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ppm::image::Image;
use ppm::image::extern_ppm_lib;
use ppm::pixel::Pixel;
use std::path::Path;
use std::time::Duration;

pub fn get_image() -> Image {

    let image_data = (0..4).map(|_i| Pixel::new(128, 255, 128)).collect();

    Image{buffer: image_data, height: 2, width: 2, max_color: 255, ppm_type: String::from("P3")}
}

fn bench_image_write(c: &mut Criterion) {

    let image = get_image();
    let path = Path::new("image_bench.ppm");

    let mut group = c.benchmark_group("benches write");
    group.measurement_time(Duration::new(20, 0));
    group.sample_size(50);
    group.bench_function("bench save image struct", |b| b.iter(|| image.save(black_box(path))));
    group.finish();

    std::fs::remove_file(path).expect("Finally try to delete the file");
}

fn bench_image_read(c: &mut Criterion) {

    let image = get_image();
    let path = Path::new("image_bench.ppm");
    image.save(path).expect("Try close the file");

    let mut group = c.benchmark_group("benches read");
    group.measurement_time(Duration::new(20, 0));
    group.bench_function("bench read and get image struct", |b| b.iter_with_large_drop(|| Image::new_with_file(black_box(path))));
    group.finish();

    std::fs::remove_file(path).expect("Finally try to delete the file");
}

fn bench_external_lib_image_write(c: &mut Criterion) {

    let image = get_image();
    let path = Path::new("image_bench.ppm");

    let mut group = c.benchmark_group("benches write with external lib");
    group.measurement_time(Duration::new(20, 0));
    group.sample_size(50);
    group.bench_function("bench save image struct with external lib", |b| b.iter(|| extern_ppm_lib::write(black_box(path.to_str().unwrap()), black_box(&image))));
    group.finish();

    std::fs::remove_file(path).expect("Finally try to delete the file");
}

fn bench_external_lib_image_read(c: &mut Criterion) {

    let image = get_image();
    let path = Path::new("image_bench.ppm");
    extern_ppm_lib::write(path.to_str().unwrap(), &image);

    let mut group = c.benchmark_group("benches read");
    group.measurement_time(Duration::new(20, 0));
    group.bench_function("bench read and get image struct with external lib", |b| b.iter_with_large_drop(|| extern_ppm_lib::read(black_box(path.to_str().unwrap()))));
    group.finish();

    std::fs::remove_file(path).expect("Finally try to delete the file");
}

criterion_group!(benches, bench_image_write, bench_image_read, bench_external_lib_image_write, bench_external_lib_image_read);

criterion_main!(benches);