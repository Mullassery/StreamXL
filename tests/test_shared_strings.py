from streamxl_core.shared_strings import parse


SST_XML = b"""<?xml version="1.0" encoding="UTF-8"?>
<sst xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" count="3" uniqueCount="3">
  <si><t>Hello</t></si>
  <si><t>World</t></si>
  <si><t>Streamxl</t></si>
</sst>"""


def test_parse_returns_strings():
    result = parse(SST_XML)
    assert result == ["Hello", "World", "Streamxl"]


def test_empty_sst():
    result = parse(b'<sst xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"></sst>')
    assert result == []
