; Disable certain features in specific contexts

; Inside strings - disable auto-closing brackets and autocomplete
(string) @string.content
 (#set! disable_all_bracket_pairs true)

; Inside comments - disable autocomplete and bracket matching
(comment) @comment.content
 (#set! completion false)
 (#set! bracket_pairs false)

; Inside line continuation - disable autocomplete
(line_continuation) @continuation.content
 (#set! completion false)
