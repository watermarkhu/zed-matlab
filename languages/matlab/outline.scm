; Classes
(class_definition
  "classdef" @context
  name: (identifier) @name) @item

; Functions
(function_definition
  "function" @context
  name: (identifier) @name) @item

; Methods within class definitions
(methods
  (function_definition
    "function" @context
    name: (identifier) @name) @item)

; Properties within class definitions
(properties
  (property
    name: (identifier) @name) @item)

; Events within class definitions
(events
  (identifier) @name) @item

; Enumeration values
(enumeration
  (enum
    (identifier) @name) @item)
