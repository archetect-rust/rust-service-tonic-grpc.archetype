PascalCase := "CustomerHistory"
snake_case := "customer_history"
train-case := "customer-history"
pattern := '.*\.rs|.*\.md|.*\.proto|Cargo.toml|.*\.json|.*\.krop|.*\.krpref|.*.krproj'

fix:
  fd -0 -H '{{ pattern }}' . | xargs -0 sd -f c -s "{{ PascalCase }}Service" "{{{{ ProjectName }}"
  fd -0 -H '{{ pattern }}' . | xargs -0 sd -f c -s "{{ snake_case }}_service" "{{{{ project_name }}"
  fd -0 -H '{{ pattern }}' . | xargs -0 sd -f c -s "{{ snake_case }}.service" "{{{{ project_prefix }}.{{{{ project_suffix }}"
  fd -0 -H '{{ pattern }}' . | xargs -0 sd -f c -s "{{ snake_case }}/service" "{{{{ project_prefix }}/{{{{ project_suffix }}"
  fd -0 -H '{{ pattern }}' . | xargs -0 sd -f c -s "{{ train-case }}-service" "{{{{ project-name }}"
  fd -0 -H '{{ pattern }}' . | xargs -0 sd -f c -s "{{ PascalCase }}" "{{{{ ProjectPrefix }}"
  fd -0 -H '{{ pattern }}' . | xargs -0 sd -f c -s "{{ snake_case }}" "{{{{ project_prefix }}"
  fd -0 -H '{{ pattern }}' . | xargs -0 sd -f c -s "{{ train-case }}" "{{{{ project-prefix }}"
  fd -0 -H '{{ pattern }}' . | xargs -0 sd -f c -s {{"{{"}}"{" {{"{{"}}"'{'}}"{{"{{"}}
