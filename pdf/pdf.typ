#set page(width: 3in, height: 2in)

#let name = if ("name" in sys.inputs) {
  sys.inputs.name
} else { "" }

#let company = if ("company" in sys.inputs) {
  sys.inputs.company
} else { "" }

#align(center + horizon)[
  *#name* \
  #company
]
