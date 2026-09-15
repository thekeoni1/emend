//! T1 P2 tests (`specs/0004-xlsx-diff.md`): content comparison. Every archive
//! is generated from fictional content in a temporary directory; nothing is
//! committed.

mod common;

use std::fs;
use std::io::Write;

use common::*;
use zip::unstable::write::FileOptionsExt;
use zip::write::SimpleFileOptions;
use zip::{DateTime, HasZipMetadata, ZipArchive, ZipWriter};

#[test]
fn short_raw_stream_is_an_error() {
    let dir = TempDir::new("short");
    let (input, output) = (dir.path("in.zip"), dir.path("out.zip"));
    write_zip(&input, &[stored("a.bin", &pattern(5000))]);
    let bytes = fs::read(&input).expect("read");
    fs::write(&output, cut_entry_tail(&bytes, 1000)).expect("write");
    assert_eq!(
        meta(&input),
        meta(&output),
        "precondition: the output opens and its central record keeps the declared sizes"
    );

    let result = run(&input, &output);
    assert_eq!(result.status, 2);
    assert_eq!(result.stdout, "");
    assert_eq!(
        result.stderr,
        "error: cannot read entry a.bin in the output archive\n"
    );
}

#[test]
fn recompressed_entry() {
    let dir = TempDir::new("recompressed");
    let data = text(2000);
    let fast = Spec {
        level: Some(1),
        ..deflated("level.xml", &data)
    };
    let best = Spec {
        level: Some(9),
        ..deflated("level.xml", &data)
    };
    let (input, output) = (dir.path("in.zip"), dir.path("out.zip"));
    write_zip(&input, &[fast, deflated("method.xml", &data)]);
    write_zip(&output, &[best, stored("method.xml", &data)]);
    let (mi, mo) = (meta(&input), meta(&output));
    assert_ne!(
        mi[0][2], mo[0][2],
        "precondition: different compressed sizes for the level entry"
    );

    let result = run(&input, &output);
    assert_eq!(result.status, 1);
    assert_eq!(result.stderr, "");
    let entry_rows = rows(&result.stdout, "entry");
    for (i, row) in entry_rows.iter().enumerate() {
        assert_eq!(row[2], "recompressed");
        assert_columns(row, Some((i, &mi[i])), Some((i, &mo[i])));
        assert_tail(row, ["", "", "", ""]);
    }
    assert_eq!(entry_rows.len(), 2);
    assert_eq!(
        archive_rows(&result.stdout),
        expected_archive([2, 2, 0, 2, 0, 0, 0])
    );
}

#[test]
fn changed_entry_with_equal_length() {
    let dir = TempDir::new("equal-length");
    let data = pattern(1000);
    let (input, output) = (dir.path("in.zip"), dir.path("out.zip"));
    write_zip(
        &input,
        &[
            deflated("first-byte.txt", &data),
            deflated("last-byte.txt", &data),
            deflated("three.txt", &data),
        ],
    );
    write_zip(
        &output,
        &[
            deflated("first-byte.txt", &flipped(&data, &[0])),
            deflated("last-byte.txt", &flipped(&data, &[999])),
            deflated("three.txt", &flipped(&data, &[0, 500, 999])),
        ],
    );

    let result = run(&input, &output);
    assert_eq!(result.status, 1);
    assert_eq!(result.stderr, "");
    let (mi, mo) = (meta(&input), meta(&output));
    let entry_rows = rows(&result.stdout, "entry");
    let expected = [
        ["0", "1", "0", ""],
        ["999", "1", "999", ""],
        ["0", "3", "999", ""],
    ];
    assert_eq!(entry_rows.len(), 3);
    for (i, row) in entry_rows.iter().enumerate() {
        assert_eq!(row[2], "changed");
        assert_columns(row, Some((i, &mi[i])), Some((i, &mo[i])));
        assert_tail(row, expected[i]);
    }
    assert_eq!(
        archive_rows(&result.stdout),
        expected_archive([3, 3, 0, 0, 3, 0, 0])
    );
}

#[test]
fn changed_entry_with_different_lengths() {
    let dir = TempDir::new("lengths");
    let data = pattern(1000);
    let (input, output) = (dir.path("in.zip"), dir.path("out.zip"));
    write_zip(
        &input,
        &[
            deflated("shorter-out.txt", &data),
            deflated("longer-out.txt", &data[..600]),
        ],
    );
    write_zip(
        &output,
        &[
            deflated("shorter-out.txt", &data[..600]),
            deflated("longer-out.txt", &data),
        ],
    );

    let result = run(&input, &output);
    assert_eq!(result.status, 1);
    assert_eq!(result.stderr, "");
    let entry_rows = rows(&result.stdout, "entry");
    assert_eq!(entry_rows.len(), 2);
    for row in &entry_rows {
        assert_eq!(row[2], "changed");
        assert_tail(row, ["600", "", "", ""]);
    }
    assert_eq!(
        (entry_rows[0][7], entry_rows[0][8]),
        ("1000", "600"),
        "uncompressed sizes"
    );
}

#[test]
fn large_entry_with_differences_in_first_middle_and_last_chunk() {
    let dir = TempDir::new("large");
    let data = pattern(300_000);
    let offsets = [10, 65_535, 65_536, 131_077, 299_999];
    let (input, output) = (dir.path("in.zip"), dir.path("out.zip"));
    write_zip(&input, &[deflated("large.bin", &data)]);
    write_zip(&output, &[deflated("large.bin", &flipped(&data, &offsets))]);

    let result = run(&input, &output);
    assert_eq!(result.status, 1);
    assert_eq!(result.stderr, "");
    let entry_rows = rows(&result.stdout, "entry");
    assert_eq!(entry_rows.len(), 1);
    assert_eq!(entry_rows[0][2], "changed");
    assert_tail(&entry_rows[0], ["10", "5", "299999", ""]);
    assert_eq!(
        archive_rows(&result.stdout),
        expected_archive([1, 1, 0, 0, 1, 0, 0])
    );
}

#[test]
fn wrong_crc_is_an_error() {
    let dir = TempDir::new("crc");

    // The CRC-32 field changed in both headers; the data is unchanged.
    let (input, output) = (dir.path("in.zip"), dir.path("out.zip"));
    write_zip(&input, &[deflated("a.txt", &text(40))]);
    let mut bytes = fs::read(&input).expect("read");
    let crc = crc_of(&bytes, b"a.txt");
    set_crc(&mut bytes, b"a.txt", crc ^ 0x0000_00ff);
    fs::write(&output, bytes).expect("write");
    let result = run(&input, &output);
    assert_eq!(result.status, 2);
    assert_eq!(result.stdout, "");
    assert_eq!(
        result.stderr,
        "error: entry a.txt in the output archive failed to decompress or its CRC-32 does not match\n"
    );

    // One byte flipped inside a stored entry past the second 64 KiB buffer;
    // the recorded CRC-32 and sizes are unchanged, so the raw bytes differ
    // and decompression finds the mismatch.
    let marker = b"zq-marker-t1p2";
    let mut big = pattern(200_000);
    big[150_000..150_000 + marker.len()].copy_from_slice(marker);
    let (input, output) = (dir.path("big-in.zip"), dir.path("big-out.zip"));
    write_zip(&input, &[stored("big.bin", &big)]);
    let mut bytes = fs::read(&input).expect("read");
    let at = bytes
        .windows(marker.len())
        .position(|w| w == &marker[..])
        .expect("the marker is in the stored data");
    assert!(
        at > 2 * 64 * 1024,
        "precondition: the flipped byte is past the second buffer"
    );
    bytes[at] ^= 0x01;
    fs::write(&output, bytes).expect("write");
    assert_eq!(
        meta(&input),
        meta(&output),
        "precondition: same method, sizes and CRC-32"
    );
    let result = run(&input, &output);
    assert_eq!(result.status, 2);
    assert_eq!(result.stdout, "");
    assert_eq!(
        result.stderr,
        "error: entry big.bin in the output archive failed to decompress or its CRC-32 does not match\n"
    );
}

#[test]
fn encrypted_entry_is_an_error() {
    // The fixture is made with the crate's ZipCrypto writer, which zip 8.6.0
    // offers with the deflate-flate2-zlib-rs feature alone, through
    // zip::unstable::write::FileOptionsExt. Nothing is set by hand.
    let dir = TempDir::new("encrypted");
    let (plain, encrypted) = (dir.path("plain.zip"), dir.path("encrypted.zip"));
    write_zip(
        &plain,
        &[
            deflated("a.txt", &text(5)),
            deflated("secret.txt", &text(6)),
        ],
    );
    let options = SimpleFileOptions::default().last_modified_time(DateTime::default());
    let zipcrypto = options
        .with_deprecated_encryption(b"fictional-password")
        .expect("ZipCrypto options");
    let mut writer = ZipWriter::new(fs::File::create(&encrypted).expect("create the archive"));
    writer.start_file("a.txt", options).expect("start an entry");
    writer.write_all(&text(5)).expect("write an entry");
    writer
        .start_file("secret.txt", zipcrypto)
        .expect("start an entry");
    writer.write_all(&text(6)).expect("write an entry");
    writer.finish().expect("finish the archive");
    let mut archive = ZipArchive::new(fs::File::open(&encrypted).expect("open")).expect("read");
    let flags: Vec<bool> = (0..2)
        .map(|i| {
            archive
                .by_index_raw(i)
                .expect("entry")
                .get_metadata()
                .encrypted
        })
        .collect();
    assert_eq!(
        flags,
        [false, true],
        "precondition: the zip crate sees only secret.txt as encrypted"
    );

    let result = run(&plain, &encrypted);
    assert_eq!(result.status, 2);
    assert_eq!(result.stdout, "");
    assert_eq!(
        result.stderr,
        "error: entry secret.txt in the output archive is encrypted\n"
    );

    let result = run(&encrypted, &plain);
    assert_eq!(result.status, 2);
    assert_eq!(
        result.stderr,
        "error: entry secret.txt in the input archive is encrypted\n"
    );
}

#[test]
fn unsupported_method_is_an_error() {
    let dir = TempDir::new("method");
    let (input, output) = (dir.path("in.zip"), dir.path("out.zip"));
    write_zip(&input, &[stored("m.txt", &text(8))]);
    let mut bytes = fs::read(&input).expect("read");
    set_method(&mut bytes, b"m.txt", 12);
    fs::write(&output, bytes).expect("write");
    let result = run(&input, &output);
    assert_eq!(result.status, 2);
    assert_eq!(result.stdout, "");
    assert_eq!(
        result.stderr,
        "error: entry m.txt in the output archive uses unsupported compression method 12\n"
    );
}
