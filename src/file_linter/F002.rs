use crate::{error::Error, file_linter::tree_sitter_go};
use regex::Regex;
use std::collections::HashSet;
use tree_sitter::{Query, QueryCursor};
use tree_sitter_edit::{NodeId, Replace};

use super::FileLinter;

lazy_static! {
    static ref QUERY: Query =
        tree_sitter::Query::new(unsafe { tree_sitter_go() }, "(import_spec_list) @list").unwrap();
    static ref STANDARD_IMPORTS: HashSet<&'static str> = HashSet::from([
        "archive/tar",
        "archive/zip",
        "arena",
        "bufio",
        "bytes",
        "compress/bzip2",
        "compress/flate",
        "compress/gzip",
        "compress/lzw",
        "compress/zlib",
        "container/heap",
        "container/list",
        "container/ring",
        "context",
        "crypto",
        "crypto/aes",
        "crypto/cipher",
        "crypto/des",
        "crypto/dsa",
        "crypto/ecdh",
        "crypto/ecdsa",
        "crypto/ed25519",
        "crypto/elliptic",
        "crypto/hmac",
        "crypto/md5",
        "crypto/rand",
        "crypto/rc4",
        "crypto/rsa",
        "crypto/sha1",
        "crypto/sha256",
        "crypto/sha512",
        "crypto/subtle",
        "crypto/tls",
        "crypto/x509",
        "crypto/x509/pkix",
        "database/sql",
        "database/sql/driver",
        "debug/buildinfo",
        "debug/dwarf",
        "debug/elf",
        "debug/gosym",
        "debug/macho",
        "debug/pe",
        "debug/plan9obj",
        "embed",
        "encoding",
        "encoding/ascii85",
        "encoding/asn1",
        "encoding/base32",
        "encoding/base64",
        "encoding/binary",
        "encoding/csv",
        "encoding/gob",
        "encoding/hex",
        "encoding/json",
        "encoding/pem",
        "encoding/xml",
        "errors",
        "expvar",
        "flag",
        "fmt",
        "go/ast",
        "go/build",
        "go/build/constraint",
        "go/constant",
        "go/doc",
        "go/doc/comment",
        "go/format",
        "go/importer",
        "go/parser",
        "go/printer",
        "go/scanner",
        "go/token",
        "go/types",
        "hash",
        "hash/adler32",
        "hash/crc32",
        "hash/crc64",
        "hash/fnv",
        "hash/maphash",
        "html",
        "html/template",
        "image",
        "image/color",
        "image/color/palette",
        "image/draw",
        "image/gif",
        "image/jpeg",
        "image/png",
        "index/suffixarray",
        "io",
        "io/fs",
        "io/ioutil",
        "log",
        "log/syslog",
        "math",
        "math/big",
        "math/bits",
        "math/cmplx",
        "math/rand",
        "mime",
        "mime/multipart",
        "mime/quotedprintable",
        "net",
        "net/http",
        "net/http/cgi",
        "net/http/cookiejar",
        "net/http/fcgi",
        "net/http/httptest",
        "net/http/httptrace",
        "net/http/httputil",
        "net/http/pprof",
        "net/mail",
        "net/netip",
        "net/rpc",
        "net/rpc/jsonrpc",
        "net/smtp",
        "net/textproto",
        "net/url",
        "os",
        "os/exec",
        "os/signal",
        "os/user",
        "path",
        "path/filepath",
        "plugin",
        "reflect",
        "regexp",
        "regexp/syntax",
        "runtime",
        "runtime/cgo",
        "runtime/coverage",
        "runtime/debug",
        "runtime/metrics",
        "runtime/pprof",
        "runtime/race",
        "runtime/trace",
        "sort",
        "strconv",
        "strings",
        "sync",
        "sync/atomic",
        "syscall",
        "testing",
        "testing/fstest",
        "testing/iotest",
        "testing/quick",
        "text/scanner",
        "text/tabwriter",
        "text/template",
        "text/template/parse",
        "time",
        "time/tzdata",
        "unicode",
        "unicode/utf16",
        "unicode/utf8",
        "unsafe",
    ]);
    static ref PREFIX_PATTERN: Regex = Regex::new(r"prefix\((.*)\)").unwrap();
}

// F002 - Unsorted imports
pub fn run(linter: &mut FileLinter) -> (Vec<Error>, Vec<Replace>) {
    if !linter.configuration.is_enabled(String::from("F002")) {
        return (vec![], vec![]);
    }

    if let Some(settings) = &linter.configuration.settings {
        let groups = &settings.F002;
        let mut errors = vec![];

        let mut sorted_imports: Vec<Vec<String>> = vec![Vec::new(); groups.len()];
        let mut curr = 0;

        let mut cursor = QueryCursor::new();
        for m in cursor.matches(&QUERY, linter.tree.root_node(), linter.source.as_bytes()) {
            let list = m.captures[0].node;
            for import_spec in list.children(&mut list.walk()) {
                let text = linter.text(import_spec);

                if text == "(" || text == ")" || text == "\n" {
                    continue;
                }

                if text == "\n\n" {
                    curr += 1;
                    continue;
                }

                let import = text.split_whitespace().last().unwrap().trim_matches('"');

                if let Some(group) = index(groups, import) {
                    sorted_imports[group].push(format!("\t{}", text));
                    if group < curr && errors.is_empty() {
                        errors.push(Error {
                            filename: linter.path.clone(),
                            position: import_spec.start_position(),
                            rule: String::from("F002"),
                            message: format!(r#"unsorted import "{import}""#),
                        });
                    }
                    curr = group;
                } else if errors.is_empty() {
                    errors.push(Error {
                        filename: linter.path.clone(),
                        position: import_spec.start_position(),
                        rule: String::from("F002"),
                        message: format!(r#"unclassified import "{import}""#),
                    });
                }
            }

            let mut editors = vec![];

            if !errors.is_empty() {
                let sections: Vec<String> = sorted_imports
                    .iter()
                    .filter(|v| !v.is_empty())
                    .map(|v| v.join("\n"))
                    .collect();
                let out = format!("(\n{}\n)", sections.join("\n\n"));

                editors.push(Replace {
                    id: NodeId::new(&list),
                    bytes: out.as_bytes().to_vec(),
                });

                return (errors, editors);
            }
        }
    }

    (vec![], vec![])
}

fn index(groups: &[String], import: &str) -> Option<usize> {
    let mut default_group = None;
    let mut prefix_group = None;

    let mut longest_prefix = 0;

    for (i, rule) in groups.iter().enumerate() {
        if rule == "standard" && STANDARD_IMPORTS.contains(import) {
            return Some(i);
        }

        if rule == "default" {
            default_group = Some(i);
            continue;
        }

        if let Some(captures) = PREFIX_PATTERN.captures(rule) {
            if let Some(prefix) = captures.get(1) {
                if import.starts_with(prefix.as_str()) && prefix.len() > longest_prefix {
                    prefix_group = Some(i);
                    longest_prefix = prefix.len();
                    continue;
                }
            }
        }
    }

    prefix_group.or(default_group)
}
