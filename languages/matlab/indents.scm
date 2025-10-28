[
  (arguments_statement)
  (if_statement)
  (for_statement)
  (while_statement)
  (switch_statement)
  (try_statement)
  (function_definition)
  (class_definition)
  (enumeration)
  (events)
  (methods)
  (properties)
] @indent

[
  (elseif_clause)
  (else_clause)
  (case_clause)
  (otherwise_clause)
  (catch_clause)
] @indent

[ "end" ] @outdent

((matrix (row) @indent)
 (#set! indent.open_delimiter "[")
 (#set! indent.close_delimiter "]"))

((cell (row) @indent)
 (#set! indent.open_delimiter "{")
 (#set! indent.close_delimiter "}"))

((parenthesis) @indent
 (#set! indent.open_delimiter "(")
 (#set! indent.close_delimiter ")"))
