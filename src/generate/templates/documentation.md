# Documentation for {name}

{{ if single_interface }}

# Interface

{{ for interface in interfaces}}
{{ call documentation_interface with interface }}
{{ endfor }}

{{ else }}

{{ for interface in interfaces}}

# interface.name

{{ call documentation_interface with interface }}
{{ endfor }}

{{ endif }}
