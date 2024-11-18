use super::*;
use bzip2::read::{BzEncoder, MultiBzDecoder};
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};

#[test]
fn test_parse_xml() {
    let sample_xml = r#"
<mediawiki xmlns="http://www.mediawiki.org/xml/export-0.11/" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="http://www.mediawiki.org/xml/export-0.11/ http://www.mediawiki.org/xml/export-0.11.xsd" version="0.11" xml:lang="en">
  <siteinfo>
    <sitename>Wikipedia</sitename>
    <dbname>enwiki</dbname>
    <base>https://en.wikipedia.org/wiki/Main_Page</base>
    <generator>MediaWiki 1.44.0-wmf.1</generator>
    <case>first-letter</case>
    <namespaces>
      <namespace key="-2" case="first-letter">Media</namespace>
    </namespaces>
  </siteinfo>
  <page>
    <title>Sample Text</title>
    <ns>0</ns>
    <id>10</id>
    <redirect title="Computer accessibility" />
    <revision>
      <id>1324354657</id>
      <parentid>2435465768</parentid>
      <timestamp>2024-04-15T14:38:04Z</timestamp>
      <contributor>
        <username>Test</username>
        <id>12345678</id>
      </contributor>
      <comment></comment>
      <origin>987654321</origin>
      <model>wikitext</model>
      <format>text/x-wiki</format>
      <text bytes="111" sha1="123" xml:space="preserve">This is a sample text.
[[TEST]]
This is the [[TEST]] text.
Test text 2 {{TEST}}.
Test text 3 {{TEST|test}}.
Test text 4 {{TEST|test|test}}.
Test text 5 [[TEST5|test|test|test]].
Test text 6 [[TEST6|{{This text does not appear in the output.|This also does not.}}]].
[[File:Test.jpg|thumb|Test image.]]
{| class="wikitable"
|- This text does not appear in the output.
|}</text>
    <sha1>123</sha1>
    </revision>
  </page>
</mediawiki>
"#;
    let test_input_filepath = "sample.xml.bz2";
    let test_output_filepath = "sample.jsonl.bz2";
    let mut writer = BufWriter::new(File::create(Path::new(test_input_filepath)).unwrap());
    let compressor = BzEncoder::new(sample_xml.as_bytes(), bzip2::Compression::best());
    let compressed = compressor.bytes().collect::<Result<Vec<u8>, _>>().unwrap();
    writer.write(&compressed).unwrap();
    writer.flush().unwrap();

    parse_xml("sample.xml.bz2".to_string()).unwrap();

    let decoder = MultiBzDecoder::new(File::open(test_output_filepath).unwrap());
    let mut reader = BufReader::new(decoder);
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).unwrap();
    let output = String::from_utf8(buf).unwrap();
    assert!(output.contains("Sample Text"));

    if Path::new(test_input_filepath).exists() {
        std::fs::remove_file(test_input_filepath).unwrap();
    }
    if Path::new(test_output_filepath).exists() {
        std::fs::remove_file(test_output_filepath).unwrap();
    }
}
