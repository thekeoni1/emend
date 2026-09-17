//! Tests for the token stream (`specs/0009-package-layer.md`, "P1").
//!
//! Every part here is built byte by byte in the test, as the archive tests
//! build their archives: no fixture files, and nothing that reads a part
//! through the same code the stream uses.
//!
//! The property under test throughout is P1's acceptance: the mark when
//! present, followed by every token's `raw` in order, is the input byte for
//! byte.

use std::io::{self, Cursor, Read};

use quick_xml::events::Event;

use super::{BYTE_ORDER_MARK, Stream};
use crate::archive::Error;

/// The entry index the tests pass in, to check it comes back in errors.
const INDEX: usize = 7;

/// Longer than the read-ahead, so a single token spans several reads.
const LONG: usize = 70_000;

/// Reads a part to the end, returning what the tokens reconstruct, how many
/// there were, and the event kinds in order.
///
/// Every token's `raw` is checked to begin exactly where the previous one
/// ended, by position: `position()` after each token must have advanced by
/// that token's own length and no more. The token is dropped before
/// `position()` is read, which is why the bytes are copied out first.
fn read_part(part: &[u8]) -> Result<(Vec<u8>, usize, Vec<&'static str>), Error> {
    let mut stream = Stream::new(Cursor::new(part.to_vec()), INDEX)?;
    let mut out = Vec::new();
    if stream.bom() {
        out.extend_from_slice(&BYTE_ORDER_MARK);
    }
    // The first token begins at the first counted byte: after the mark.
    let mut expected = if stream.bom() {
        BYTE_ORDER_MARK.len() as u64
    } else {
        0
    };
    assert_eq!(
        stream.position(),
        expected,
        "the stream starts after the mark"
    );

    let mut kinds = Vec::new();
    let mut count = 0;
    loop {
        let (bytes, kind) = match stream.next()? {
            None => break,
            Some(token) => (token.raw.to_vec(), kind_of(&token.event)),
        };
        assert!(!bytes.is_empty(), "a token covered no bytes");
        let after = stream.position();
        assert_eq!(
            after,
            expected + bytes.len() as u64,
            "token {count} ({kind}) did not begin where the previous ended"
        );
        expected = after;
        out.extend_from_slice(&bytes);
        kinds.push(kind);
        count += 1;
    }
    // Nothing is left over: the tokens reached the end of the input.
    assert_eq!(
        expected,
        part.len() as u64,
        "the tokens stopped short of the end of the part"
    );
    Ok((out, count, kinds))
}

fn kind_of(event: &Event<'_>) -> &'static str {
    match event {
        Event::Start(_) => "Start",
        Event::End(_) => "End",
        Event::Empty(_) => "Empty",
        Event::Text(_) => "Text",
        Event::CData(_) => "CData",
        Event::Comment(_) => "Comment",
        Event::Decl(_) => "Decl",
        Event::PI(_) => "PI",
        Event::DocType(_) => "DocType",
        Event::GeneralRef(_) => "Ref",
        Event::Eof => "Eof",
    }
}

/// Asserts the reconstruction property and returns the token count.
fn reconstructs(part: &[u8]) -> usize {
    match read_part(part) {
        Ok((out, count, _)) => {
            assert_eq!(out, part, "the tokens did not reconstruct the part");
            count
        }
        Err(err) => panic!("a well-formed part was refused: {err}"),
    }
}

/// The error a part is refused with, or a panic naming what came instead.
fn refusal(part: &[u8]) -> Error {
    match read_part(part) {
        Err(err) => err,
        Ok((_, count, kinds)) => {
            panic!("a malformed part was accepted as {count} tokens: {kinds:?}")
        }
    }
}

/// A body holding every form P1's list names, without a declaration, a mark
/// or trailing whitespace: those are varied around it.
fn every_form() -> Vec<u8> {
    let long_text = "t".repeat(LONG);
    let long_value = "v".repeat(LONG);
    let mut part = Vec::new();
    part.extend_from_slice(b"<!--a comment-->");
    part.extend_from_slice(b"<?instruction with data?>");
    part.extend_from_slice(b"<!DOCTYPE root>");
    part.extend_from_slice(b"<root>");
    // Whitespace-only text, including a carriage return and a tab.
    part.extend_from_slice(b"  \t\r\n  ");
    part.extend_from_slice(b"<empty/>");
    part.extend_from_slice(b"<spaced />");
    // Attribute values in both quote kinds, each holding a reference.
    part.extend_from_slice(b"<quoted single='&amp;' double=\"&lt;\"/>");
    // The five predefined references, then a decimal and a hexadecimal
    // character reference.
    part.extend_from_slice(b"<refs>&amp;&lt;&gt;&quot;&apos;&#65;&#x42;</refs>");
    part.extend_from_slice(b"<![CDATA[a section with < & > and ]] inside]]>");
    part.extend_from_slice("<utf8>\u{e9} \u{4e2d}\u{6587} \u{1f600}</utf8>".as_bytes());
    part.extend_from_slice(b"<nested><inner><deep>x</deep></inner></nested>\r\n");
    part.extend_from_slice(format!("<long>{long_text}</long>").as_bytes());
    part.extend_from_slice(format!("<longattr value=\"{long_value}\"/>").as_bytes());
    part.extend_from_slice(b"</root>");
    part
}

#[test]
fn every_form_reconstructs_in_all_eight_framings() {
    // P1's list: each with and without a byte order mark, with and without a
    // declaration, with and without whitespace after the root.
    let body = every_form();
    for mark in [false, true] {
        for declaration in [false, true] {
            for trailing in [false, true] {
                let mut part = Vec::new();
                if mark {
                    part.extend_from_slice(&BYTE_ORDER_MARK);
                }
                if declaration {
                    part.extend_from_slice(b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
                }
                part.extend_from_slice(&body);
                if trailing {
                    part.extend_from_slice(b"\r\n  \n");
                }
                let count = reconstructs(&part);
                assert!(
                    count >= 20,
                    "framing mark={mark} decl={declaration} trailing={trailing} \
                     produced only {count} tokens"
                );
            }
        }
    }
}

#[test]
fn a_mark_is_reported_and_is_in_no_token() {
    let part = [&BYTE_ORDER_MARK[..], b"<a/>"].concat();
    let mut stream = Stream::new(Cursor::new(part.clone()), INDEX).expect("new");
    assert!(stream.bom(), "the mark was not reported");
    let raw = stream
        .next()
        .expect("a token")
        .expect("a token")
        .raw
        .to_vec();
    // The mark is not part of the first token: its bytes are the element's.
    assert_eq!(raw, b"<a/>");
    reconstructs(&part);
}

#[test]
fn no_mark_is_reported_when_the_part_has_none() {
    let stream = Stream::new(Cursor::new(b"<a/>".to_vec()), INDEX).expect("new");
    assert!(!stream.bom(), "a mark was reported for a part with none");
}

#[test]
fn a_part_starting_with_two_marks_keeps_the_second_in_a_text_token() {
    // P1 ruling 1: the crate strips the leading mark once, uncounted, so the
    // second mark is content and must come back inside a token. This is the
    // case the draft's mechanism lost three bytes on.
    let part = [&BYTE_ORDER_MARK[..], &BYTE_ORDER_MARK[..], b"<a/>"].concat();
    let (out, _, kinds) = read_part(&part).expect("two marks are not an error");
    assert_eq!(out, part, "the second mark was lost");
    assert_eq!(kinds, vec!["Text", "Empty"], "the second mark was not text");

    // And the mark is reported once, not twice.
    let stream = Stream::new(Cursor::new(part.clone()), INDEX).expect("new");
    assert!(stream.bom());
}

#[test]
fn an_empty_part_has_no_tokens_and_reconstructs_empty() {
    let mut stream = Stream::new(Cursor::new(Vec::new()), INDEX).expect("new");
    assert!(!stream.bom());
    assert!(
        stream.next().expect("no error").is_none(),
        "a token from nothing"
    );
    // Asking again is still the end, and still not an error.
    assert!(stream.next().expect("no error").is_none());
    assert_eq!(reconstructs(b""), 0);
}

#[test]
fn a_part_of_only_a_mark_has_no_tokens() {
    // The mark is the whole part: it is reported, and there is nothing else.
    let mut stream = Stream::new(Cursor::new(BYTE_ORDER_MARK.to_vec()), INDEX).expect("new");
    assert!(stream.bom());
    assert!(stream.next().expect("no error").is_none());
    assert_eq!(reconstructs(&BYTE_ORDER_MARK), 0);
}

#[test]
fn a_part_shorter_than_a_mark_reconstructs() {
    // One and two bytes: `new` reads what it can and must not treat a short
    // read as a mark. Neither is well-formed XML, so both are refused, but
    // the refusal must be the parser's and not a panic in `new`.
    for part in [&b"<"[..], b"<a"] {
        assert!(matches!(refusal(part), Error::Xml(INDEX)));
    }
    // Whitespace alone is accepted, and is short enough to exercise the same
    // path in `new`.
    assert_eq!(reconstructs(b" "), 1);
    assert_eq!(reconstructs(b"\n\t"), 1);
}

#[test]
fn a_token_longer_than_the_read_ahead_reconstructs() {
    // The tee's bound is the largest token, so a token several read-aheads
    // long must still come back whole and in one piece.
    let text = "x".repeat(LONG);
    let part = format!("<a>{text}</a>").into_bytes();
    let (out, _, kinds) = read_part(&part).expect("well-formed");
    assert_eq!(out, part);
    assert_eq!(kinds, vec!["Start", "Text", "End"]);

    let value = "y".repeat(LONG);
    let part = format!("<a value=\"{value}\"/>").into_bytes();
    let (out, _, kinds) = read_part(&part).expect("well-formed");
    assert_eq!(out, part);
    assert_eq!(kinds, vec!["Empty"]);
}

#[test]
fn many_tokens_reconstruct_a_part_larger_than_the_read_ahead() {
    // Exercises the tee's compaction: the part is far longer than the
    // read-ahead but every token is short, so the front is released
    // repeatedly and the retained bytes must stay correct across a compact.
    const ELEMENTS: usize = 40_000;
    let mut part = Vec::from(&b"<root>"[..]);
    for n in 0..ELEMENTS {
        part.extend_from_slice(format!("<c n=\"{n}\">{n}</c>").as_bytes());
    }
    part.extend_from_slice(b"</root>");
    // Several compactions must actually happen, or the test proves nothing
    // about them: the front is released once per token and compacted once per
    // COMPACT_AT bytes released.
    assert!(
        part.len() > 4 * super::COMPACT_AT,
        "the part is {} bytes, too small to compact more than once",
        part.len()
    );
    let (out, count, _) = read_part(&part).expect("well-formed");
    assert_eq!(out, part);
    assert_eq!(count, 2 + ELEMENTS * 3, "a token went missing");
}

#[test]
fn whitespace_before_and_after_the_root_reconstructs() {
    reconstructs(b"\n  <a/>\n  ");
    reconstructs(b"<?xml version=\"1.0\"?>\n\n<a/>\n");
}

#[test]
fn crlf_line_ends_are_kept_as_they_are() {
    // The bytes are copied, so a CRLF is neither normalised nor trimmed.
    let part = b"<a>\r\n1\r\n</a>\r\n";
    let (out, _, kinds) = read_part(part).expect("well-formed");
    assert_eq!(out, part);
    assert_eq!(kinds, vec!["Start", "Text", "End", "Text"]);
}

// --------------------------------------------------------------- refusals

#[test]
fn a_tag_with_no_closing_angle_bracket_is_refused() {
    // P1 ruling 2's reading of "an unclosed tag": a tag with no `>`.
    for part in [&b"<a"[..], b"<a b=\"1\"", b"<root><a"] {
        assert!(
            matches!(refusal(part), Error::Xml(INDEX)),
            "not refused as Xml: {:?}",
            String::from_utf8_lossy(part)
        );
    }
}

#[test]
fn a_mismatched_end_tag_is_refused() {
    assert!(matches!(refusal(b"<a></b>"), Error::Xml(INDEX)));
    assert!(matches!(refusal(b"<a><b></a></b>"), Error::Xml(INDEX)));
    // An end tag with no start at all is refused too.
    assert!(matches!(refusal(b"</a>"), Error::Xml(INDEX)));
}

#[test]
fn a_bare_angle_bracket_in_text_with_no_later_one_is_refused() {
    // P1 ruling 2's reading of "a bare `<`": one with no later `>`, which the
    // parser reaches the end of the input looking for.
    assert!(matches!(refusal(b"a < b"), Error::Xml(INDEX)));
    assert!(matches!(refusal(b"<a>x < y"), Error::Xml(INDEX)));
}

#[test]
fn a_dangling_ampersand_is_refused() {
    assert!(matches!(refusal(b"<a>&</a>"), Error::Xml(INDEX)));
    assert!(matches!(refusal(b"<a>&"), Error::Xml(INDEX)));
}

#[test]
fn a_byte_that_is_not_utf8_in_text_is_refused() {
    // P1 ruling 2: the refusal is not limited to values the layer
    // interprets, because 0.42 decodes text as `str`. The part is otherwise
    // well-formed, and the bad byte is inside a text run.
    let part = b"<a>ok\xc3</a>";
    assert!(matches!(refusal(part), Error::Xml(INDEX)));
    // The same byte inside an attribute value, and inside a name.
    assert!(matches!(refusal(b"<a v=\"\xff\"/>"), Error::Xml(INDEX)));
    assert!(matches!(refusal(b"<a\xff/>"), Error::Xml(INDEX)));
}

#[test]
fn a_binary_part_is_refused() {
    // What makes a real binary part refusable is that it is not valid UTF-8,
    // which is the only validity this parser checks (measured; see the
    // module's Limits). The first bytes of a PNG, a blob of high bytes as a
    // printer settings part begins, a lone surrogate and an overlong
    // encoding all fail that way.
    for part in [
        &b"\x89PNG\r\n\x1a\n\x00\x00\x00\rIHDR"[..],
        b"\x01\x00\x00\x00\xff\xfe\x80\x81",
        b"<a>\xed\xa0\x80</a>",
        b"<a>\xc0\x80</a>",
    ] {
        assert!(
            matches!(refusal(part), Error::Xml(INDEX)),
            "not refused as Xml: {:?}",
            String::from_utf8_lossy(part)
        );
    }
}

#[test]
fn the_index_in_an_error_is_the_one_the_stream_was_given() {
    // The index is carried through, and it is the entry's, not a position.
    for index in [0usize, 1, 42, 780] {
        let mut stream = Stream::new(Cursor::new(b"<a></b>".to_vec()), index).expect("new");
        let err = loop {
            match stream.next() {
                Ok(Some(_)) => continue,
                Ok(None) => panic!("the malformed part was accepted"),
                Err(err) => break err,
            }
        };
        match err {
            Error::Xml(carried) => assert_eq!(carried, index),
            other => panic!("expected Xml, got {other:?}"),
        }
    }
}

#[test]
fn a_refused_part_stays_refused_and_does_not_restart() {
    // A parse error is final: asking again returns the end of the input
    // rather than restarting the parser mid-document.
    let mut stream = Stream::new(Cursor::new(b"<a>&</a>".to_vec()), INDEX).expect("new");
    let mut saw_error = false;
    loop {
        match stream.next() {
            Ok(Some(_)) => continue,
            Ok(None) => break,
            Err(_) => {
                saw_error = true;
                break;
            }
        }
    }
    assert!(saw_error, "the dangling ampersand was accepted");
    assert!(stream.next().expect("no second error").is_none());
}

#[test]
fn the_error_message_is_the_catalogued_one() {
    // `specs/0009-package-layer.md`, "Errors": every message P1 introduces is
    // in the module catalogue with its test. This is that test, word for
    // word, and it also pins that no position and no content are carried.
    let message = Error::Xml(7).to_string();
    assert_eq!(message, "not well-formed XML or not UTF-8 (entry 7)");
    // The catalogue in this module's header holds the same words.
    let source = include_str!("../xml.rs");
    let doc = source
        .lines()
        .filter_map(|line| line.trim_start().strip_prefix("//!"))
        .map(str::trim)
        .collect::<Vec<_>>()
        .join(" ");
    assert!(
        doc.contains("not well-formed XML or not UTF-8 (entry N)"),
        "the xml module's catalogue does not hold the message"
    );
}

#[test]
fn a_reader_failure_is_io_and_not_xml() {
    // Only the part's own bytes make an `Xml`: a failing reader stays `Io`,
    // so a disk fault is never reported as a malformed part.
    struct Failing {
        before: usize,
    }
    impl Read for Failing {
        fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
            if self.before == 0 {
                return Err(io::Error::new(io::ErrorKind::PermissionDenied, "denied"));
            }
            let n = self.before.min(out.len());
            out[..n].fill(b' ');
            self.before -= n;
            Ok(n)
        }
    }

    // Failing inside `new`, while the first three bytes are read. Matched
    // rather than unwrapped: `Stream` is not `Debug`, and a reader is not
    // something to print.
    match Stream::new(Failing { before: 0 }, INDEX) {
        Err(err) => assert!(matches!(err, Error::Io(_)), "got {err:?}"),
        Ok(_) => panic!("a failing reader opened"),
    }

    // Failing later, once the parser is reading.
    let mut stream = Stream::new(Failing { before: 8 }, INDEX).expect("new");
    let err = loop {
        match stream.next() {
            Ok(Some(_)) => continue,
            Ok(None) => panic!("the failure was reported as the end of the input"),
            Err(err) => break err,
        }
    };
    assert!(matches!(err, Error::Io(_)), "got {err:?}");
}

// ------------------------------------------------- the accepted limits

#[test]
fn the_three_accepted_classes_are_accepted_and_reconstruct() {
    // The module's Limits, from P1 ruling 2. Each is not well-formed XML, and
    // each must still come back byte for byte, because this layer's contract
    // is fidelity and not validation.
    assert_eq!(reconstructs(b"<a>"), 1);
    assert_eq!(reconstructs(b"<a>hi"), 2);
    assert_eq!(reconstructs(b"<a>&foo;</a>"), 3);
    assert_eq!(reconstructs(b"<a/><b/>"), 2);
    assert_eq!(reconstructs(b"text before <a/> and after"), 3);
    // A `<` in text followed by a later `>` is read as a tag, not refused.
    assert_eq!(reconstructs(b"<a>x < y</a>"), 3);
}

#[test]
fn a_character_xml_forbids_is_accepted_when_it_is_valid_utf8() {
    // The fourth Limits entry (P1 ruling 4): the parser checks the encoding,
    // not the character range. NUL and one other C0 control, in text,
    // accepted and reconstructed. XML 1.0 forbids both outright.
    let with_nul = b"<a>ok\x00ok</a>";
    let (out, _, kinds) = read_part(with_nul).expect("NUL is not an encoding error");
    assert_eq!(out, with_nul, "the NUL was not reproduced");
    assert_eq!(kinds, vec!["Start", "Text", "End"]);

    let with_escape = b"<a>ok\x1bok</a>";
    let (out, _, kinds) = read_part(with_escape).expect("ESC is not an encoding error");
    assert_eq!(out, with_escape, "the control byte was not reproduced");
    assert_eq!(kinds, vec!["Start", "Text", "End"]);

    // The contrast that makes the limit precise: the same position holding a
    // byte that is not valid UTF-8 IS refused.
    assert!(matches!(refusal(b"<a>ok\xffok</a>"), Error::Xml(INDEX)));
}
