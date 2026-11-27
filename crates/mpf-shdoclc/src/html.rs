/// The ERROR.HTM HTML resource of shdoclc.rc, byte for byte.
pub const ERROR_HTM: &str = r#"<!DOCTYPE HTML>
<html>
<head>
  <meta http-equiv="Content-Type" content="text/html; charset=UTF-8">
  <title>Page Load Error</title>
</head>
<body onload="javascript: if(window.location.search) document.getElementById('err').innerHTML = unescape(window.location.search.substr(1))">
<div style="margin-left:45px; width:720px;">
  <h2>The page cannot be loaded because of an error</h2>
  <p>Error code: <b id="err">Unknown</b></p>
</div>
</body>
</html>
"#;

/// The HTML resource with the given name, or None when shdoclc.rc defines no such resource.
/// Wine matches resource names exactly and case-sensitively: find_entry_by_name in
/// dlls/ntdll/resource.c compares them with wcsncmp and then checks the length.
pub fn html_resource(name: &str) -> Option<&'static str> {
    (name == "ERROR.HTM").then_some(ERROR_HTM)
}
