use std::thread;
use std::time::Duration;
use rodio::{OutputStream, source::SineWave};
use rodio::Source;


fn play_tone(stream_handle: &rodio::OutputStreamHandle, mut frequency: f32, mut duration_ms: u64) {
    let source = SineWave::new(frequency).take_duration(Duration::from_millis(duration_ms)).amplify(0.05);
    stream_handle.play_raw(source.convert_samples()).unwrap();

    let source = SineWave::new(frequency+100.0).take_duration(Duration::from_millis(duration_ms-0));
    stream_handle.play_raw(source.convert_samples()).unwrap();

}


pub fn new_level_sound() {
	let builder = thread::Builder::new(); 
	let handle = builder.spawn( move || {

	 let (_stream, stream_handle) = OutputStream::try_default().unwrap();
	    let melody = [
	        (392.0, 150), // G4
	        (494.0, 150), // B4
	        (587.0, 200), // D5
	        (784.0, 200), // G5
	        (698.0, 150), // F5
	        (659.0, 150), // E5
	    ];


	    for &(freq, duration) in &melody {
	        play_tone(&stream_handle, freq, duration);
	        std::thread::sleep(Duration::from_millis(250)); 
     }
   }).unwrap();
}




pub fn new_level_sound2() {
	let builder = thread::Builder::new(); 
	let handle = builder.spawn( move || {

 let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let melody = [
        (261.63, 150),  // C4
        (329.63, 150),  // E4
        (392.00, 200),  // G4
        (440.00, 150),  // A4
        (349.23, 150),  // F4
        (293.66, 200),  // D4
        (261.63, 150),  // C4
        (329.63, 200),  // E4
        (392.00, 250),  // G4
        (523.25, 200),  // C5
        (392.00, 300),  // G4
    ];

    for &(freq, duration) in &melody {
        play_tone(&stream_handle, freq as f32, duration);
        std::thread::sleep(Duration::from_millis(150)); 
    }
   }).unwrap();

}


pub fn step_sound() {
	let builder = thread::Builder::new(); 
	let handle = builder.spawn( move || {

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let source = SineWave::new(400.0).take_duration(Duration::from_millis(70)).amplify(0.1);;
    stream_handle.play_raw(source.convert_samples()).unwrap();
    let source = SineWave::new(300.0).take_duration(Duration::from_millis(50)).amplify(0.1);;
    stream_handle.play_raw(source.convert_samples()).unwrap();
    let source = SineWave::new(550.0).take_duration(Duration::from_millis(70)).amplify(0.1);;
    stream_handle.play_raw(source.convert_samples()).unwrap();
    let source = SineWave::new(450.0).take_duration(Duration::from_millis(50)).amplify(0.1);;
    stream_handle.play_raw(source.convert_samples()).unwrap();
    let source = SineWave::new(400.0).take_duration(Duration::from_millis(70)).amplify(0.1);;
    stream_handle.play_raw(source.convert_samples()).unwrap();
    let source = SineWave::new(300.0).take_duration(Duration::from_millis(50)).amplify(0.1);;
    stream_handle.play_raw(source.convert_samples()).unwrap();
    std::thread::sleep(Duration::from_millis(150));
   }).unwrap();
}

pub fn bell_sound() {
	let builder = thread::Builder::new(); 
	let handle = builder.spawn( move || {

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let source = SineWave::new(1200.0).take_duration(Duration::from_millis(100)).amplify(0.1);;
    stream_handle.play_raw(source.convert_samples()).unwrap();
    let source = SineWave::new(800.0).take_duration(Duration::from_millis(100)).amplify(0.1);;
    stream_handle.play_raw(source.convert_samples()).unwrap();
    let source = SineWave::new(1500.0).take_duration(Duration::from_millis(80)).amplify(0.1);;
    stream_handle.play_raw(source.convert_samples()).unwrap();
    let source = SineWave::new(900.0).take_duration(Duration::from_millis(120));
    stream_handle.play_raw(source.convert_samples()).unwrap();
    let source = SineWave::new(1300.0).take_duration(Duration::from_millis(150));
    stream_handle.play_raw(source.convert_samples()).unwrap();
    let source = SineWave::new(600.0).take_duration(Duration::from_millis(150));
    stream_handle.play_raw(source.convert_samples()).unwrap();
    std::thread::sleep(Duration::from_millis(250));
   }).unwrap();
}
