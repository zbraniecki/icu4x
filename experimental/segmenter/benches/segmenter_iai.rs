use icu_segmenter::LineSegmenter;

const TEST_STR_TH: &str =
    "ภาษาไทยภาษาไทย ภาษาไทยภาษาไทย ภาษาไทยภาษาไทย ภาษาไทยภาษาไทย ภาษาไทยภาษาไทย ภาษาไทยภาษาไทย";

fn iai_segment_iter_utf8() {
    let segmenter_lstm =
        LineSegmenter::try_new_lstm_unstable(&icu_testdata::unstable()).expect("Data exists");

    segmenter_lstm.segment_str(TEST_STR_TH).count();
}

fn iai_segment_iter_utf16() {
    let utf16_th: Vec<u16> = TEST_STR_TH.encode_utf16().collect();
    let segmenter_lstm =
        LineSegmenter::try_new_lstm_unstable(&icu_testdata::unstable()).expect("Data exists");

    segmenter_lstm.segment_utf16(&utf16_th).count();
}

iai::main!(iai_segment_iter_utf8, iai_segment_iter_utf16);
