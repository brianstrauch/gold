package tests

// Match
func f1(a string, b string)        {}
func f2(a int, b string, c string) {}
func f3(a, b string, c, d string)  {}

// No Match
func f4(a int, b string)               {}
func f5(a, b string)                   {}
func f6(a string, b *string, c string) {}
