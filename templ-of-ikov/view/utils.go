package view

import "strings"

func Classes(args ...interface{}) string {
	var classes []string

	for _, arg := range args {
		switch v := arg.(type) {
		case string:
			if v != "" {
				classes = append(classes, v)
			}
		case []string:
			for _, s := range v {
				if s != "" {
					classes = append(classes, s)
				}
			}
		case map[string]bool:
			for k, cond := range v {
				if cond {
					classes = append(classes, k)
				}
			}
		}
	}

	return strings.Join(classes, " ")
}
