//! Writes wav files for every first order filter for analysis

extern crate oxcable;

#[cfg(not(test))]
fn main() {
    use std::vec::Vec;
    use oxcable::core::AudioDevice;
    use oxcable::io::wav;
    use oxcable::filters::second_order;
    use oxcable::filters::second_order::Filter;
    println!("Writing second order filters to wav files...");

    // Initialize objects
    let mut wav_in = wav::WavReader::new("wav/delta.wav");

    let cutoff = 1000.0;
    let mut filters = Vec::<second_order::Filter>::new();
    let mut wavs = Vec::<wav::WavWriter>::new();
    filters.push(Filter::new(second_order::LowPass(cutoff), 1));
    wavs.push(wav::WavWriter::new("wav/test_second_order_low_pass.wav", 1));
    filters.push(Filter::new(second_order::HighPass(cutoff), 1));
    wavs.push(wav::WavWriter::new("wav/test_second_order_high_pass.wav", 1));
    filters.push(Filter::new(second_order::LowShelf(cutoff, -6.0), 1));
    wavs.push(wav::WavWriter::new("wav/test_second_order_low_shelf.wav", 1));
    filters.push(Filter::new(second_order::HighShelf(cutoff, -6.0), 1));
    wavs.push(wav::WavWriter::new("wav/test_second_order_high_shelf.wav", 1));
    filters.push(Filter::new(second_order::Peak(cutoff, 6.0, 1.0), 1));
    wavs.push(wav::WavWriter::new("wav/test_second_order_peak.wav", 1));

    // Link oscillators to wav outs
    for i in range(0u, filters.len()) {
        filters[i].inputs.set_channel(0, wav_in.outputs.get_channel(0));
        wavs[i].inputs.set_channel(0, filters[i].outputs.get_channel(0));
    }

    // Write files
    for t in range(0, 44100) {
        wav_in.tick(t);
        for i in range(0, filters.len()) {
            filters[i].tick(t);
            wavs[i].tick(t);
        }
    }

    // Finish the wav files
    for i in range(0, wavs.len()) {
        wavs[i].update_data_size();
    }
    println!("Done");
}
