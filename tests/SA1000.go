package main

import (
	"regexp"
	r "regexp"
)

func main() {
	const a = `(`
	const b = "("
	const c = a

	var d = `(`
	var e = "("
	var f = a

	g := `(`
	h := "("
	i := a

	// Match
	regexp.Compile("(")
	regexp.Match("(", nil)
	regexp.MatchReader("(", nil)
	regexp.MatchString("(", "")
	regexp.MustCompile("(")
	regexp.MustCompile(`(`)
	r.MustCompile("(")
	regexp.MustCompile(a)
	regexp.MustCompile(b)
	regexp.MustCompile(c)
	regexp.MustCompile(d)
	regexp.MustCompile(e)
	regexp.MustCompile(f)
	regexp.MustCompile(g)
	regexp.MustCompile(h)
	regexp.MustCompile(i)

	// No Match
	regexp.MustCompile("")
	regexp.DoNotCompile("(")
	regexp.MatchString("", "(")
	other.MustCompile("(")
}
